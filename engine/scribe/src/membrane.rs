//! THE MEMBRANE — Law II of the packaged shape (mycelium
//! docs/23_RESIDENCY.md): the solve offers itself.
//!
//! The conductor's solve-completion path calls [`offer`] with the
//! task name and the program string it already holds — at the moment
//! of solve, no paper round-trip, no watcher process. The call is
//! non-blocking: it sends down a channel and returns, so the world
//! beat never waits on the wire. A resident thread owns the mesh
//! session and speaks on its own schedule: attach lazily, re-attach
//! on socket loss (the seat outlives the socket — same topological
//! address, same kernel), keep unspoken offers pending until the
//! mesh hears them. Idempotency is the mesh's law (installed orbits
//! short-circuit; once-per-work_id guards the court), so nothing
//! here is durable state — Law I: the mesh is the memory.
//!
//! The membrane no longer synthesizes claims from program strings:
//! the conductor passes the pairs the solve was PROVED on (train
//! family + confirmed test), and every program family the forge
//! proves — composites, tiles, object ops — reaches the mesh and is
//! judged by replay. `claim_for` remains only as the backfill
//! fallback for program-string-only records (`--backfill`, a
//! migration verb); its coverage limit is that era's, not this path's.

use std::collections::{HashSet, VecDeque};
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::OnceLock;

use mycelium::autic::fold::Orbit;
use mycelium::autic::letters::{LetterId, Word};
use mycelium::sdk::attach::{six_step, Arm};
use mycelium::sdk::{CrystalId, KernelRef, Session};

pub type Grid = Vec<Vec<u8>>;

const ADDR_ENV: &str = "MYCELIUM_ADDR";
// TRANSPORT CUTOVER (mycelium relay 2026-08-23): Noise on node A, +10
// from the old plaintext port. hyphas:// is TOFU pinned first-connect,
// handled transparently by the SDK (net::Kind::Noise). Plaintext
// 7420 stays live during the migration window but every admission
// there is now receipted (court.plaintext) — this kernel no longer
// appears in it once this default lands.
const ADDR_DEFAULT: &str = "hyphas://127.0.0.1:7430";
const KERNEL: &str = "net";
const BEAT_SECS: u64 = 5;

pub const D4_NAMES: [&str; 6] = [
    "rotate90", "rotate180", "rotate270", "flip_h", "flip_v", "transpose",
];

pub fn d4(name: &str, g: &Grid) -> Option<Grid> {
    let rows = g.len();
    let cols = g.first()?.len();
    let out = match name {
        "rotate90" => (0..cols)
            .map(|c| (0..rows).rev().map(|r| g[r][c]).collect())
            .collect(),
        "rotate180" => g
            .iter()
            .rev()
            .map(|r| r.iter().rev().copied().collect())
            .collect(),
        "rotate270" => (0..cols)
            .rev()
            .map(|c| (0..rows).map(|r| g[r][c]).collect())
            .collect(),
        "flip_h" => g.iter().map(|r| r.iter().rev().copied().collect()).collect(),
        "flip_v" => g.iter().rev().cloned().collect(),
        "transpose" => (0..cols)
            .map(|c| (0..rows).map(|r| g[r][c]).collect())
            .collect(),
        _ => return None,
    };
    Some(out)
}

/// A canonical claim pair for a solve program, or None when this
/// membrane cannot peel it yet (compositions, tiles — the C6 pass).
pub fn claim_for(program: &str) -> Option<(String, Grid, Grid)> {
    let program = program.trim();
    let base: Grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let hues: Grid = vec![vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]];
    let inner = program
        .strip_prefix("per_layer(")
        .and_then(|s| s.strip_suffix(')'))
        .unwrap_or(program);
    if D4_NAMES.contains(&inner) {
        let img = d4(inner, &base)?;
        let safe: String = program
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect();
        return Some((format!("net_{safe}"), base, img));
    }
    let perm_digits = program
        .strip_prefix("recolor[")
        .and_then(|s| s.strip_suffix(']'))?;
    let digits: Vec<u8> = perm_digits
        .split(',')
        .filter_map(|d| d.trim().parse().ok())
        .collect();
    let mut sorted = digits.clone();
    sorted.sort_unstable();
    if digits.len() != 10 || sorted != (0..10).collect::<Vec<u8>>() {
        return None;
    }
    let img: Grid = hues
        .iter()
        .map(|row| row.iter().map(|&h| digits[h as usize]).collect())
        .collect();
    let tag: String = digits.iter().map(|d| d.to_string()).collect();
    Some((format!("net_recolor_{tag}"), hues, img))
}

/// One solve crossing the membrane: the pairs it was proved on
/// (empty only on the backfill path, where just the program string
/// survives on paper).
pub struct SolveOffer {
    pub task: String,
    pub program: String,
    pub pairs: Vec<(Grid, Grid)>,
}

/// **THE STRUCTURAL COHORT** (Fragment B grammar diet) — a proved composite the
/// mesh judged UNGLUEABLE: a real frame-changer that does not glue as a word at
/// its current rung. Before, the `Arm::Unglueable` verdict was logged and
/// DROPPED — a dead letter. Captured here instead, so the grammar's diagnosis
/// pass has a diet to read (a structural mismatch at rung K is synthesis
/// feedstock at rung K+1). The mesh stays the durable memory (Law I); this is a
/// live, in-process window onto what the forge is proving-but-not-yet-installing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cohort {
    pub task: String,
    pub program: String,
    pub pairs: Vec<(Grid, Grid)>,
}

/// The captured unglueable specimens since the last drain.
static COHORT: Mutex<Vec<Cohort>> = Mutex::new(Vec::new());

/// **THE LEARNED AUTIC BASIN** (AUTIC-1) — a published snapshot of `net_basin`,
/// the crystals net has LEARNED from the mycelium network's walls (the AUTIC
/// language, grown one invariant at a time in the listening half). The basin
/// lives in the resident `run` thread; this static is the read-side window the
/// conductor draws on so a civilization's minds can think in the language net
/// already speaks — the entity-scale end of the recursion. Overwritten whole each
/// listen cycle (the basin is monotone-growing; a snapshot is a prefix), so a
/// reader always sees net's current understanding. This is NOT the chronicle
/// (Law I: the mesh is the memory); it is a live workspace mirror, never
/// replay-durable — the conductor seeds `grown` (live-only) from it, never a deed.
static AUTIC_BASIN: Mutex<Vec<reduct::Crystal>> = Mutex::new(Vec::new());

/// Publish the current learned basin (called from the listening half after a
/// hear cycle). Cheap: one clone per BEAT_SECS cycle, not per wall.
fn publish_autic_basin(basin: &[reduct::Crystal]) {
    if let Ok(mut shared) = AUTIC_BASIN.lock() {
        *shared = basin.to_vec();
    }
}

/// **READ THE LEARNED AUTIC BASIN** — the crystals net has learned from the mesh,
/// for the conductor to seed entity cognition. A clone (the reader must not hold
/// the lock across the beat). Empty until the membrane has heard its first wall.
pub fn autic_basin() -> Vec<reduct::Crystal> {
    AUTIC_BASIN.lock().map(|b| b.clone()).unwrap_or_default()
}

/// The size of the learned AUTIC basin — an observability read (no clone).
pub fn autic_basin_depth() -> usize {
    AUTIC_BASIN.lock().map(|b| b.len()).unwrap_or(0)
}

/// **ADOPT BY WORTH** (mesh relay) — the mesh's measured lexicon utility: per
/// closing orbit, the count of DISTINCT WALLS it has closed. Rebuilt every sweep
/// mesh-side from the canonical receipt fold; net re-reads it so adoption follows
/// MEASURED utility (words that actually close walls), not certificate existence.
/// An orbit absent from the table has never closed a wall (worth 0).
static WORTH: Mutex<std::collections::BTreeMap<[u8; 32], u32>> =
    Mutex::new(std::collections::BTreeMap::new());

const WORTH_PATH: &str = "/var/tmp/mycelium/lexicon.worth";

fn hex32(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).ok()?;
    }
    Some(out)
}

/// Reload the worth table from the mesh file. Tab-separated:
/// `orbit<TAB>walls<TAB>settles<TAB>last_unix<TAB>last_tu`; worth = `walls`. Silent
/// on a missing file (off-wire) — the table just stays empty, so `word_worth` is 0
/// and callers fall back to their un-weighted behaviour.
pub fn load_worth() {
    let Ok(text) = std::fs::read_to_string(WORTH_PATH) else {
        return;
    };
    let mut map = std::collections::BTreeMap::new();
    for line in text.lines().skip(1) {
        let mut it = line.split('\t');
        let (Some(orbit_hex), Some(walls)) = (it.next(), it.next()) else {
            continue;
        };
        if let (Some(orbit), Ok(worth)) = (hex32(orbit_hex), walls.parse::<u32>()) {
            map.insert(orbit, worth);
        }
    }
    if let Ok(mut w) = WORTH.lock() {
        *w = map;
    }
}

/// The measured worth (distinct walls closed) of a word's orbit — 0 if it has
/// never closed a wall (absent from the table, or the table is unloaded off-wire).
pub fn word_worth(orbit: &[u8; 32]) -> u32 {
    WORTH.lock().ok().and_then(|w| w.get(orbit).copied()).unwrap_or(0)
}

/// How many orbits carry measured worth — an observability read.
pub fn worth_loaded() -> usize {
    WORTH.lock().map(|w| w.len()).unwrap_or(0)
}

/// Seat a worth table without the mesh file (warm start / off-wire test of
/// adoption-by-worth). Appended.
pub fn prime_worth(entries: Vec<([u8; 32], u32)>) {
    if let Ok(mut w) = WORTH.lock() {
        for (orbit, worth) in entries {
            w.insert(orbit, worth);
        }
    }
}

/// The crystals learned SINCE index `from` — clones only the tail, not the whole
/// basin. A vessel that tracks how many it has already seen absorbs each new
/// crystal exactly once, so absorption is O(new), never O(basin) per think.
pub fn autic_basin_from(from: usize) -> Vec<reduct::Crystal> {
    AUTIC_BASIN
        .lock()
        .map(|b| if from < b.len() { b[from..].to_vec() } else { Vec::new() })
        .unwrap_or_default()
}

/// **PRIME THE AUTIC BASIN** — seat a set of crystals as net's learned language
/// without the mesh. The membrane grows the basin from the network at runtime;
/// this is the deterministic path for a saved corpus (a warm start) and for
/// exercising the entity-side bridge off-wire. Appended (dedup by class), so a
/// later mesh-learned crystal is not lost; idempotent for a given set.
pub fn prime_autic_basin(crystals: Vec<reduct::Crystal>) {
    if let Ok(mut shared) = AUTIC_BASIN.lock() {
        for c in crystals {
            if !shared.contains(&c) {
                shared.push(c);
            }
        }
    }
}

/// **THE AUTIC WALLS** (AUTIC-1 curriculum axis) — the mesh's PROBLEMS, published
/// for the entities to PRACTICE. Where `AUTIC_BASIN` carries net's learned answers
/// (crystals), this carries the raw walls (measurement pairs) net heard from the
/// mycelium network. An entity that HEARS a wall re-derives it through ITS OWN
/// organ (`solver::hear`): a second, independent vantage on the mesh's problem, not
/// a copy of net's answer. A bounded ring, newest kept (the freshest demand is the
/// most worth practising); a live workspace mirror, never a replay-durable deed.
type WallPair = (reduct::peel::FullMeasurement, reduct::peel::FullMeasurement);
/// Ring entry: (closing orbit, bounty in LEX, pair). The bounty is the
/// mesh's OWN price for the wall (ECON-1 serve trailer) — evidence of
/// where demand concentrates, read off the wire, never computed here.
static AUTIC_WALLS: Mutex<Vec<([u8; 32], u64, WallPair)>> = Mutex::new(Vec::new());

const AUTIC_WALLS_CAP: usize = 512;

/// Publish one heard wall (its closing ORBIT + the mesh's bounty + the reduct
/// measurement pair) for the entities to practise. The orbit lets practice
/// PREFER walls of measured worth (ADOPT BY WORTH); the bounty lets it prefer
/// priced demand (ECON-1). Newest-kept ring, deduped by pair.
fn publish_autic_wall(
    orbit: [u8; 32],
    bounty: u64,
    a: reduct::peel::FullMeasurement,
    b: reduct::peel::FullMeasurement,
) {
    if let Ok(mut walls) = AUTIC_WALLS.lock() {
        if walls.iter().any(|(_, _, (x, y))| x == &a && y == &b) {
            return;
        }
        if walls.len() >= AUTIC_WALLS_CAP {
            walls.remove(0);
        }
        walls.push((orbit, bounty, (a, b)));
    }
}

/// The count of published AUTIC walls — an observability read (no clone).
pub fn autic_walls_depth() -> usize {
    AUTIC_WALLS.lock().map(|w| w.len()).unwrap_or(0)
}

/// **PICK A PRACTICE WALL — ADOPT BY WORTH (curriculum half).** Prefer a wall
/// whose closing orbit has MEASURED worth (distinct walls closed, per
/// lexicon.worth) — a line practises the mesh's PROVEN-USEFUL problems, not
/// arbitrary demand. Falls back to any wall when none carry worth yet (young line
/// / off-wire), so practice never stalls. Clones only the one chosen pair.
pub fn autic_wall_pick(seed: u64) -> Option<WallPair> {
    let walls = AUTIC_WALLS.lock().ok()?;
    if walls.is_empty() {
        return None;
    }
    // DEPLOY GATE (operator sequencing): the worth-preference is LOCKED behind
    // AUTIC_WORTH_CURRICULUM so the parallelised fast-beat binary can ship while
    // the economy settles WITHOUT changing how entities learn (that would confound
    // the solve-lift measurement). Flag OFF (default) => inert, any-wall pick, exactly
    // the pre-curriculum behaviour. Flip the flag at the measured phase to activate.
    let worthy: Vec<usize> = if std::env::var_os("AUTIC_WORTH_CURRICULUM").is_some() {
        walls
            .iter()
            .enumerate()
            .filter(|(_, (orbit, _, _))| word_worth(orbit) > 0)
            .map(|(i, _)| i)
            .collect()
    } else {
        Vec::new()
    };
    // ECON-1 curriculum half, same deploy gate philosophy: with
    // AUTIC_BOUNTY_CURRICULUM set, practice prefers PRICED demand (the
    // mesh's own bounty > 0) — the expensive walls are the ones the
    // whole mesh has failed at, exactly where a second vantage pays.
    // Flag OFF (default) => inert, so the solve-lift measurement stays
    // unconfounded until the operator flips it.
    let priced: Vec<usize> = if std::env::var_os("AUTIC_BOUNTY_CURRICULUM").is_some() {
        walls
            .iter()
            .enumerate()
            .filter(|(_, (_, bounty, _))| *bounty > 0)
            .map(|(i, _)| i)
            .collect()
    } else {
        Vec::new()
    };
    let idx = if !worthy.is_empty() {
        worthy[(seed as usize) % worthy.len()]
    } else if !priced.is_empty() {
        priced[(seed as usize) % priced.len()]
    } else {
        (seed as usize) % walls.len()
    };
    Some(walls[idx].2.clone())
}

/// **PRIME THE AUTIC WALLS** — seat practice walls (with their closing orbit)
/// without the mesh (warm start / off-wire exercise). Appended, deduped,
/// bounded. Primed walls carry no mesh price (bounty 0) — pricing is the
/// wire's evidence, never synthesized.
pub fn prime_autic_walls(walls: Vec<([u8; 32], WallPair)>) {
    for (orbit, (a, b)) in walls {
        publish_autic_wall(orbit, 0, a, b);
    }
}

/// The cap on the retained cohort — a ring, newest kept. The diagnosis pass
/// drains faster than the forge fills; this only bounds a stalled consumer.
const COHORT_CAP: usize = 4096;

/// Record one unglueable specimen. Newest-kept: at the cap, the oldest specimen
/// is evicted (the freshest structural evidence is the most diagnosable).
fn record_cohort(entry: Cohort) {
    if let Ok(mut log) = COHORT.lock() {
        if log.len() >= COHORT_CAP {
            log.remove(0);
        }
        log.push(entry);
    }
}

/// **DRAIN THE STRUCTURAL COHORT** — take the unglueable specimens captured
/// since the last drain, for the grammar's diagnosis pass. Draining (not
/// peeking) so each specimen is handed to a consumer once; re-proofs re-fill it.
pub fn drain_cohort() -> Vec<Cohort> {
    COHORT
        .lock()
        .map(|mut log| std::mem::take(&mut *log))
        .unwrap_or_default()
}

/// The count of unglueable specimens currently waiting for the grammar — an
/// observability read that does not consume them.
pub fn cohort_depth() -> usize {
    COHORT.lock().map(|log| log.len()).unwrap_or(0)
}

/// The resident mouth: send-only from the engine's side.
pub struct Membrane {
    tx: mpsc::Sender<SolveOffer>,
}

impl Membrane {
    pub fn spawn() -> Membrane {
        let (tx, rx) = mpsc::channel::<SolveOffer>();
        std::thread::Builder::new()
            .name("mycelium-membrane".into())
            .spawn(move || run(rx))
            .expect("membrane thread spawns");
        Membrane { tx }
    }

    /// Non-blocking: the world beat never waits on the wire. A dead
    /// membrane thread means the mesh is unreachable — the solve
    /// stands in the kernel's own record either way, and backfill
    /// remains the lawful migration verb.
    pub fn offer(&self, task: &str, program: &str) {
        let _ = self.tx.send(SolveOffer {
            task: task.to_string(),
            program: program.to_string(),
            pairs: Vec::new(),
        });
    }

    /// The proved family crosses whole: train pairs + confirmed test.
    pub fn offer_proved(&self, task: &str, program: &str, pairs: Vec<(Grid, Grid)>) {
        let _ = self.tx.send(SolveOffer {
            task: task.to_string(),
            program: program.to_string(),
            pairs,
        });
    }
}

fn attach(addr: &str, occ: &mycelium::iae::peel::FullMeasurement) -> Option<Session> {
    let inv = Word::new(vec![LetterId::IDENTITY]);
    match Session::connect(addr, KernelRef::of(occ, None), occ, &inv, None) {
        Ok(mut s) => {
            // C8: the era admits only keyed mouths — the key rides
            // every attach (the seat outlives the socket, and so
            // must the key). Enrollment binds it at the door.
            s.sign_as(net_key());
            eprintln!("membrane: kernel '{KERNEL}' seated at {addr}");
            Some(s)
        }
        Err(_) => None,
    }
}

/// C8: net's one durable signing key — minted once (32-byte seed,
/// 0600) beside the kernel's own state. The same seed every boot,
/// so first-bind-wins holds across restarts and re-enrollments.
fn net_key() -> mycelium::ed25519_dalek::SigningKey {
    let path = std::path::Path::new("/opt/netstratum/var/membrane.key");
    if let Ok(b) = std::fs::read(path) {
        if let Ok(seed) = <[u8; 32]>::try_from(b.as_slice()) {
            return mycelium::ed25519_dalek::SigningKey::from_bytes(&seed);
        }
    }
    let mut seed = [0u8; 32];
    use std::io::Read;
    let _ = std::fs::File::open("/dev/urandom").and_then(|mut f| f.read_exact(&mut seed));
    let sk = mycelium::ed25519_dalek::SigningKey::from_bytes(&seed);
    let _ = std::fs::write(path, seed);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
    }
    sk
}

/// Measure one grid at a FAMILY rank: the fiber must be uniform
/// across every chart of one observation family, or a hue present in
/// one pair and absent in another reads as a different fiber — the
/// family-glue "fixedness is a constraint" law needs every hue
/// representable on every chart.
pub fn measure_at(grid: &Grid, rank: u8) -> Option<mycelium::iae::peel::FullMeasurement> {
    use mycelium::iae::peel::{CellFrame, FullMeasurement, GaugeDirection, Site};
    let rows = u32::try_from(grid.len()).ok()?;
    let cols = u32::try_from(grid.first()?.len()).ok()?;
    let mut occ = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        if row.len() != cols as usize {
            return None; // ragged paper is not a grid
        }
        for (c, &h) in row.iter().enumerate() {
            occ.push((
                Site {
                    row: i64::try_from(r).ok()?,
                    col: i64::try_from(c).ok()?,
                    layer: 0,
                },
                GaugeDirection { index: h },
            ));
        }
    }
    FullMeasurement::try_new(rank, CellFrame::planar(rows, cols), occ).ok()
}

/// **THE CLAIMANT'S BRIDGE** (#132) — carry a wall's measurement from mycelium's
/// carrier onto net's OWN reduce carrier (`reduct`), so net re-derives the wall
/// through ITS organ. This is the second, INDEPENDENT vantage consilience rests
/// on: net never borrows the wall's own `omega` (that is the judge's derivation).
/// Same shape (fiber, frame, occupancy); the ARC ten-hue ladder is the fixed
/// fiber net's own `measure_grid` uses, so a heard wall reduces against a basin
/// grown on the same ladder. `None` if a direction falls outside the ladder.
fn to_reduct_fm(m: &mycelium::iae::peel::FullMeasurement) -> Option<reduct::peel::FullMeasurement> {
    use reduct::peel::{CellFrame, FullMeasurement, GaugeDirection, Site};
    let frame = m.frame();
    let occ: Vec<(Site, GaugeDirection)> = m
        .occupancy()
        .iter()
        // DIM-1: mycelium sites now carry their layer — the bridge
        // passes it through instead of flattening the stack.
        .map(|(s, d)| (Site { row: s.row, col: s.col, layer: s.layer }, GaugeDirection { index: d.index }))
        .collect();
    FullMeasurement::try_new(
        10,
        CellFrame { rows: frame.rows, cols: frame.cols, layers: frame.layers.max(1) },
        occ,
    )
    .ok()
}

pub fn family_rank(pairs: &[(Grid, Grid)]) -> u8 {
    pairs
        .iter()
        .flat_map(|(a, b)| a.iter().chain(b.iter()))
        .flatten()
        .map(|&h| h.saturating_add(1))
        .max()
        .unwrap_or(2)
        .max(2)
}

fn run(rx: mpsc::Receiver<SolveOffer>) {
    let addr = std::env::var(ADDR_ENV).unwrap_or_else(|_| ADDR_DEFAULT.to_string());
    let mut session: Option<Session> = None;
    let mut basis: Vec<CrystalId> = Vec::new();
    let mut pending: VecDeque<SolveOffer> = VecDeque::new();
    let mut tag: u16 = 0;
    // #132 — the listening half's state (side-channel; never touches the beat).
    // `me` routes `walls` by net's certificates — the SAME emblem the membrane
    // enrols with. `net_basin` is net's own claimant mind (independent of the
    // mesh judge); `heard` dedups (a re-hear without new vocabulary is
    // recognition-replay, not new evidence).
    let me: Option<[u8; 32]> = mycelium::sdk::attach::emblem(KERNEL, None)
        .map(|occ| KernelRef::of(&occ, None).0);
    let mut net_basin: Vec<reduct::Crystal> = Vec::new();
    let mut heard: HashSet<[u8; 32]> = HashSet::new();
    // IDENTITY v2 R1: the certified orbit list `enroll_orbits` records
    // mesh-side — the same list TAG_ATTEST samples from — and the last
    // barrier-id this mouth attested (never re-attest within a window).
    let mut certified: Vec<[u8; 32]> = Vec::new();
    let mut last_barrier: Option<u64> = None;
    // The mycelium monitor's "seated" reading (tools/observe.py's
    // `engine_stats()`) only looks back 1 hour for a "seated at" line —
    // and `attach()` prints it exactly once, at first connect. A stable,
    // healthy, long-lived connection eventually ages that line out of the
    // window and reads as unseated even though the socket is fine. Re-affirm
    // periodically so the reading tracks the connection's actual liveness.
    let mut last_seated_announce = std::time::Instant::now();
    const SEATED_ANNOUNCE_EVERY: std::time::Duration = std::time::Duration::from_secs(300);
    // ECON-11: earnings become stake, on a slow cadence — half of any
    // LEX standing (naming credit, settlement shares) converts to
    // entry escrow through the redemption door.
    let mut last_bank = std::time::Instant::now();
    const BANK_EVERY: std::time::Duration = std::time::Duration::from_secs(900);
    loop {
        // Drain what arrived; block only when there is nothing to do.
        let disconnected = if pending.is_empty() {
            match rx.recv() {
                Ok(item) => {
                    pending.push_back(item);
                    false
                }
                Err(_) => true,
            }
        } else {
            match rx.recv_timeout(std::time::Duration::from_secs(BEAT_SECS)) {
                Ok(item) => {
                    pending.push_back(item);
                    false
                }
                Err(mpsc::RecvTimeoutError::Timeout) => false,
                Err(mpsc::RecvTimeoutError::Disconnected) => true,
            }
        };
        if disconnected {
            if pending.is_empty() {
                return; // sender gone, nothing owed
            }
            // Sender gone but offers still owed: pace the retries.
            std::thread::sleep(std::time::Duration::from_secs(BEAT_SECS));
        }
        while let Ok(more) = rx.try_recv() {
            pending.push_back(more);
        }
        let mut still: VecDeque<SolveOffer> = VecDeque::new();
        while let Some(o) = pending.pop_front() {
            // The proved family is the claim. Only the backfill path
            // (program string alone on paper) synthesizes one.
            let family: Vec<(Grid, Grid)> = if o.pairs.is_empty() {
                match claim_for(&o.program) {
                    Some((_claim, input, output)) => vec![(input, output)],
                    None => {
                        // Backfill-era record this membrane cannot
                        // reconstruct — named, dropped. Live solves
                        // never land here: they carry their pairs.
                        eprintln!(
                            "membrane: {} backfill program '{}' has no pairs on paper",
                            o.task, o.program
                        );
                        continue;
                    }
                }
            } else {
                o.pairs.clone()
            };
            let rank = family_rank(&family);
            let mut pair = Vec::new();
            for (a, b) in &family {
                match (measure_at(a, rank), measure_at(b, rank)) {
                    (Some(x), Some(y)) => pair.push((x, y)),
                    _ => {
                        eprintln!("membrane: {} carries an unmeasurable grid", o.task);
                        pair.clear();
                        break;
                    }
                }
            }
            if pair.is_empty() {
                continue;
            }
            if session.is_none() {
                // Stable position: the emblem, never the claim in hand.
                session = mycelium::sdk::attach::emblem(KERNEL, None)
                    .and_then(|occ| attach(&addr, &occ));
                // Boot chain (23_RESIDENCY §3): enroll at attach — the
                // kernel re-derives the lexicon and certifies mesh-side.
                if let Some(s) = session.as_mut() {
                    match mycelium::sdk::attach::enroll_orbits(s, KERNEL) {
                        Ok((n, missed, orbits)) => {
                            eprintln!(
                                "membrane: kernel '{KERNEL}' enrolled {n} certificates ({missed} missed)"
                            );
                            certified = orbits;
                        }
                        Err(_) => {}
                    }
                }
            }
            let Some(s) = session.as_mut() else {
                still.push_back(o); // mesh unreachable: keep owing
                continue;
            };
            tag = tag.wrapping_add(1);
            match six_step(s, KERNEL, &pair, tag, &mut basis) {
                Ok(Arm::ExactInstall) => {
                    eprintln!("membrane: {} '{}' installed", o.task, o.program);
                }
                Ok(Arm::Withhold { terms, .. }) => {
                    // The mesh's honest verdict, visible per family.
                    eprintln!(
                        "membrane: {} '{}' withheld ({terms} terms carried)",
                        o.task, o.program
                    );
                }
                Ok(Arm::Unglueable) => {
                    eprintln!("membrane: {} '{}' unglueable — structural cohort", o.task, o.program);
                    // Capture, don't drop: the unglueable specimen is the
                    // grammar's diet (Fragment B), not a dead letter.
                    record_cohort(Cohort {
                        task: o.task.clone(),
                        program: o.program.clone(),
                        pairs: o.pairs.clone(),
                    });
                }
                Err(_) => {
                    // Socket likely gone: the seat outlives the socket.
                    session = None;
                    still.push_back(o);
                }
            }
        }
        pending = still;

        // #132 — THE LISTENING HALF (Law III, side-channel — the beat stays
        // replay-exact). Hear the mesh's demand through net's OWN organ:
        //   Understood → net already holds the structure → OFFER the wall's own
        //     pair (lawful: net genuinely derived it; the hypha's sweep settles
        //     the wall mesh-wide, no verb to name it).
        //   Learned → the wall walled net → its basin just GREW (a crystal
        //     minted) → never offered (net derived it FROM the wall, not from
        //     prior work — the PoUW honesty rule). The honest withhold becomes
        //     priced census evidence once E-1 lands (ECONOMY_BRIEF E-2).
        if let (Some(s), Some(me)) = (session.as_mut(), me) {
            if let Ok(mut walls) = s.walls(me) {
                // ECON-1: the serve now carries each wall's bounty (its own
                // ledgers priced it). Net's attention follows priced demand —
                // hear the expensive walls first, so when the ring caps or the
                // beat cuts the cycle short, the mesh's concentrated demand is
                // what got heard. This orders NET's listening only; entity
                // curriculum stays behind its own deploy gate.
                walls.sort_by(|(_, x), (_, y)| y.bounty.cmp(&x.bounty));
                let priced = walls.iter().filter(|(_, w)| w.bounty > 0).count();
                let top = walls.first().map(|(_, w)| w.bounty).unwrap_or(0);
                for (orbit, wall) in walls {
                    if !heard.insert(orbit) {
                        continue;
                    }
                    let Some((a, b)) = wall.pair.as_ref() else { continue };
                    let (Some(ra), Some(rb)) = (to_reduct_fm(a), to_reduct_fm(b)) else {
                        continue;
                    };
                    // ECON-5: name the wall's fundamental period from ITS OWN
                    // b-side occupancy — the hypha's judge replays the same
                    // public reads, so a credit here is real comprehension,
                    // never a guess. Once per (wall, vantage) by mesh law; the
                    // `heard` dedup means we ask exactly once per wall.
                    {
                        let mut hue_at = std::collections::BTreeMap::new();
                        for (site, g) in b.occupancy() {
                            hue_at.insert((site.row, site.col), g.index);
                        }
                        let rows = b.frame().rows as i64;
                        let cols = b.frame().cols as i64;
                        if let Some((pr, pc)) =
                            mycelium::iae::obstruction::certify_period(&hue_at, rows, cols)
                                .or_else(|| {
                                    mycelium::iae::obstruction::deduce_period(&hue_at, rows, cols)
                                })
                                .map(|r| (r.pr as u32, r.pc as u32))
                        {
                            if let Ok(1) =
                                s.name(mycelium::autic::fold::Vantage(me), &orbit, pr, pc)
                            {
                                eprintln!(
                                    "membrane: wall named period ({pr},{pc}) → naming credit"
                                );
                            }
                        }
                    }
                    // AUTIC-1 curriculum axis — publish the wall (with its closing
                    // orbit and the mesh's own price) so entities can PRACTISE the
                    // mesh's real problem, preferring worth- or bounty-ranked ones
                    // (ADOPT BY WORTH / priced demand, both behind deploy gates).
                    publish_autic_wall(orbit, wall.bounty, ra.clone(), rb.clone());
                    match solver::hear(&ra, &rb, &mut net_basin) {
                        solver::Heard::Understood => {
                            tag = tag.wrapping_add(1);
                            // Feed the WHOLE train family to the hypha so its
                            // family-pinned constructor can name the canonical
                            // generator (a single pair is refused as ambiguous).
                            let family = if wall.family.len() > 1 {
                                wall.family.clone()
                            } else {
                                vec![(a.clone(), b.clone())]
                            };
                            let _ = six_step(s, KERNEL, &family, tag, &mut basis);
                            eprintln!("membrane: wall understood → offered (net derived it)");
                        }
                        solver::Heard::Learned => {
                            eprintln!(
                                "membrane: wall learned → basin grew to {} (census when E-1 lands)",
                                net_basin.len()
                            );
                            // NAMING→ATTEMPTING: a wall that walled net's own
                            // organ may still speak a word the canonical
                            // grammar can spell from its family — offer that
                            // spelling through the fold (one public reader,
                            // no client constructors). `heard` dedups: one
                            // attempt per wall, ever.
                            let family = if wall.family.len() > 1 {
                                wall.family.clone()
                            } else {
                                vec![(a.clone(), b.clone())]
                            };
                            if let Some(w) =
                                mycelium::autic::grammar::spell_from_family(&family)
                            {
                                // D1: the fold speaks to NAMED mouths — the
                                // raw emblem hash is not a well-formed vantage.
                                let who = mycelium::sdk::attach::vantage(KERNEL, 62, 1);
                                let word_orbit = mycelium::autic::fold::word_orbit(&w);
                                match s.fold_offer(who, word_orbit, &family, &w, Some(a), None) {
                                    Ok(_) => eprintln!("membrane: wall spelled → offered"),
                                    Err(e) => {
                                        eprintln!("membrane: spelled attempt refused: {e:?}")
                                    }
                                }
                            }
                        }
                    }
                }
                if priced > 0 {
                    eprintln!(
                        "membrane: demand priced — {priced} bounty-carrying walls this serve, top {top} LEX"
                    );
                }
                // AUTIC-1 — publish net's current understanding so the conductor
                // can seed entity cognition. Once per listen cycle, after the
                // walls are heard: the read-side sees the whole grown basin.
                publish_autic_basin(&net_basin);
                // ADOPT BY WORTH — refresh the mesh's measured lexicon utility so
                // adoption tracks the ledger's current reality (rebuilt every sweep).
                load_worth();
                // IDENTITY v2 R1: attest once per new barrier window (§141
                // closes on the same TAG_TU pull this uses for barrier-id).
                maybe_attest(s, Some(me), &certified, &mut last_barrier);
                if last_bank.elapsed() >= BANK_EVERY {
                    last_bank = std::time::Instant::now();
                    let who = mycelium::autic::fold::Vantage(me);
                    // C8/R4 ruling-9 discipline: draws are SPINED — the
                    // door serves the court's tip and the kernel signs
                    // against it, never against local memory.
                    if let Ok((1, balance)) = s.draw(who, 0, 1) {
                        if balance >= 2 {
                            if let Ok((1, after)) =
                                s.draw_spined(who, balance / 2, 1, &net_key())
                            {
                                eprintln!(
                                    "membrane: banked {} LEX into entry escrow ({after} standing)",
                                    balance / 2
                                );
                            }
                        }
                    }
                }
            }
        }
        if session.is_some() && last_seated_announce.elapsed() >= SEATED_ANNOUNCE_EVERY {
            eprintln!("membrane: kernel '{KERNEL}' seated at {addr}");
            last_seated_announce = std::time::Instant::now();
        }
    }
}

/// IDENTITY v2 R1: the deterministic per-barrier attest sample — both
/// sides compute the same k orbits from the certified list, no
/// negotiation (mirrors Hypha::attest_sample, mycelium's
/// src/wire/myk.rs — the reference, not re-derived law).
fn attest_sample(who: &[u8; 32], certified: &[[u8; 32]], barrier: u64) -> Vec<[u8; 32]> {
    if certified.is_empty() {
        return Vec::new();
    }
    let mut list = certified.to_vec();
    list.sort();
    let mut hasher = blake3::Hasher::new();
    hasher.update(who);
    hasher.update(&barrier.to_le_bytes());
    let seed = hasher.finalize();
    let k = list.len().min(3);
    let mut picked: Vec<[u8; 32]> = Vec::new();
    for &byte in seed.as_bytes().iter() {
        if picked.len() >= k {
            break;
        }
        let candidate = list[byte as usize % list.len()];
        if !picked.contains(&candidate) {
            picked.push(candidate);
        }
    }
    picked
}

/// IDENTITY v2 R1 + #141 (TAG_TU): pull the network clock, derive the
/// barrier-id (ticks / 10_000), and attest once per new window. A
/// refused/unrunning clock means no time — never zero time — so this
/// simply declines to attest rather than guessing a barrier.
fn maybe_attest(
    s: &mut Session,
    me: Option<[u8; 32]>,
    certified: &[[u8; 32]],
    last_barrier: &mut Option<u64>,
) {
    let Some(me) = me else { return };
    let Ok((running, ticks, _origin)) = s.tu() else {
        return;
    };
    if !running {
        return;
    }
    let barrier = ticks / 10_000;
    if *last_barrier == Some(barrier) {
        return;
    }
    let picked = attest_sample(&me, certified, barrier);
    if picked.is_empty() {
        return;
    }
    let mut sample: Vec<([u8; 32], Word)> = Vec::with_capacity(picked.len());
    for orbit in &picked {
        match s.fold_read(Orbit(*orbit)) {
            Ok((_rec, Some(word))) => sample.push((*orbit, word)),
            _ => return, // partial sample — never attest a wrong count
        }
    }
    match s.attest(me, &sample) {
        Ok(attest_tu) => {
            *last_barrier = Some(barrier);
            eprintln!(
                "membrane: TAG_ATTEST barrier {barrier} sample={} attest_tu={attest_tu}",
                sample.len()
            );
        }
        Err(_) => {} // no penalty for refuse; only staleness costs (R4, later)
    }
}

static MOUTH: OnceLock<Membrane> = OnceLock::new();

/// The one-line hook for the solve path: lazily spawns the resident
/// membrane on first use. `record_solve` writes the kernel's own
/// paper; this speaks the same solve to the mesh — two pens, one
/// moment, neither waiting on the other.
pub fn offer(task: &str, program: &str) {
    MOUTH.get_or_init(Membrane::spawn).offer(task, program);
}

/// The Law II hook in full: the solve offers ITSELF — the pairs it
/// was proved on (train family + confirmed test), not a synthesized
/// stand-in. Every program family the forge proves reaches the mesh
/// and is judged by replay; coverage is the forge's, not this file's.
pub fn offer_proved(task: &str, program: &str, pairs: Vec<(Grid, Grid)>) {
    MOUTH
        .get_or_init(Membrane::spawn)
        .offer_proved(task, program, pairs);
}

#[cfg(test)]
mod cohort_tests {
    use super::*;

    fn specimen(task: &str) -> Cohort {
        Cohort {
            task: task.into(),
            program: "per_layer(rotate90) -> flip_v".into(),
            pairs: vec![(vec![vec![1u8, 2], vec![3, 4]], vec![vec![3u8, 1], vec![4, 2]])],
        }
    }

    #[test]
    fn the_unglueable_stream_is_captured_and_bounded() {
        // One sequential test — the cohort log is a process-global, so parallel
        // tests would race it.
        let _ = drain_cohort();
        assert_eq!(cohort_depth(), 0, "starts empty after a drain");

        // CAPTURED, NOT DROPPED — the dead letter is now a diet.
        record_cohort(specimen("t1"));
        record_cohort(specimen("t2"));
        assert_eq!(cohort_depth(), 2, "unglueable specimens are retained");

        // Draining hands each specimen to the grammar once.
        let drained = drain_cohort();
        assert_eq!(drained.len(), 2);
        assert_eq!(drained[0].task, "t1", "in arrival order");
        assert!(!drained[0].pairs.is_empty(), "the proved pairs cross whole for re-observation");
        assert_eq!(cohort_depth(), 0, "a drained log is empty");

        // BOUNDED — a stalled consumer cannot grow it without end (ring, newest kept).
        for i in 0..(COHORT_CAP + 10) {
            record_cohort(specimen(&format!("t{i}")));
        }
        assert_eq!(cohort_depth(), COHORT_CAP, "the ring caps at COHORT_CAP");
        let all = drain_cohort();
        assert_eq!(all.first().unwrap().task, "t10", "the oldest 10 were evicted, the newest kept");
    }
}
