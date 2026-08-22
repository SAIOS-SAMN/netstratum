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

use mycelium::autic::letters::{LetterId, Word};
use mycelium::sdk::attach::{six_step, Arm};
use mycelium::sdk::{CrystalId, KernelRef, Session};

pub type Grid = Vec<Vec<u8>>;

const ADDR_ENV: &str = "MYCELIUM_ADDR";
const ADDR_DEFAULT: &str = "hypha://127.0.0.1:7420";
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
        Ok(s) => {
            eprintln!("membrane: kernel '{KERNEL}' seated at {addr}");
            Some(s)
        }
        Err(_) => None,
    }
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
                },
                GaugeDirection { index: h },
            ));
        }
    }
    FullMeasurement::try_new(rank, CellFrame { rows, cols }, occ).ok()
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
        .map(|(s, d)| (Site { row: s.row, col: s.col }, GaugeDirection { index: d.index }))
        .collect();
    FullMeasurement::try_new(10, CellFrame { rows: frame.rows, cols: frame.cols }, occ).ok()
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
                    match mycelium::sdk::attach::enroll(s, KERNEL) {
                        Ok((n, missed)) => eprintln!(
                            "membrane: kernel '{KERNEL}' enrolled {n} certificates ({missed} missed)"
                        ),
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
            if let Ok(walls) = s.walls(me) {
                for (orbit, wall) in walls {
                    if !heard.insert(orbit) {
                        continue;
                    }
                    let Some((a, b)) = wall.pair.as_ref() else { continue };
                    let (Some(ra), Some(rb)) = (to_reduct_fm(a), to_reduct_fm(b)) else {
                        continue;
                    };
                    match solver::hear(&ra, &rb, &mut net_basin) {
                        solver::Heard::Understood => {
                            tag = tag.wrapping_add(1);
                            let pair = vec![(a.clone(), b.clone())];
                            let _ = six_step(s, KERNEL, &pair, tag, &mut basis);
                            eprintln!("membrane: wall understood → offered (net derived it)");
                        }
                        solver::Heard::Learned => {
                            eprintln!(
                                "membrane: wall learned → basin grew to {} (census when E-1 lands)",
                                net_basin.len()
                            );
                        }
                    }
                }
                // AUTIC-1 — publish net's current understanding so the conductor
                // can seed entity cognition. Once per listen cycle, after the
                // walls are heard: the read-side sees the whole grown basin.
                publish_autic_basin(&net_basin);
            }
        }
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
