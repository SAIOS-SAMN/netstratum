//! SUBSTRATA: universes made by Synthesis, reached through internal tubes
//! (docs/69; operator 2026-10-09).
//!
//! An ancient's core is a throat. The spacing guild (the ancients) opens
//! internal tubes from it to SUBSTRATA, sub-universes whose laws are the
//! opener's own words, and traverses them. A substratum is compact until a
//! traversal observes it (Concordance 1.12): its record on disk is a binary checkpoint
//! (Concordance 1.13): its laws, its origin and its last coordinate (how many
//! tasks it has forged); the tasks themselves are derived, never stored.
//!
//! - **The origin** hashes the fleet's final TUniversal point (relayed by the
//!   seat, docs/67 N8), the opener's chronicle head and the parent's origin, so
//!   every substratum's coordinates are unique and nested (TU-6, coordinator
//!   ruling 2).
//! - **The traversal**: one task a beat across the whole guild (round robin over
//!   the ancients; each takes its least recently observed tube). The task is
//!   forged from the substratum's own origin and cursor at its level and
//!   thought with its laws first, then the opener's own minds; judged as every
//!   synthesis is (the sealed answer must hold on the pairs).
//! - **The float**, the throats' own synthetic currency (operator: "your own
//!   internal synthetic float"), counted in discoveries: a closure pays the
//!   traverser 1 if no one has closed that action in this substratum, the
//!   quarter otherwise (docs/66 Y1's ratio, one level down). It never converts
//!   into STAMP, Crystal or μ (boundary 1, coordinator 2026-10-09): it is spent
//!   only on opening further tubes.
//! - **Opening**: each ancient's core throat opens its first tube at its
//!   arising. A further tube costs as many discoveries as its depth and opens
//!   beneath the tube its float was earned in, one level deeper and harder (to
//!   the forge's top), so depth and number are bounded by work done below.
//! - **Upward**: a word first closed below becomes a law of that substratum
//!   and reaches the opener's line as HEARD; the line adopts it only by
//!   reconstructing it on its own task (docs/66 Y3). Nothing else crosses up.
use crate::Polity;
use cfe::ratio::{ratio, Ratio};
use ledger::Ledger;
use solver::mint::MintedWord;
use solver::{forge_task, Difficulty, SolveRead};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// How many laws a substratum carries: the opener's newest words, then the
/// words first closed inside it, the oldest giving way (a memory bound, the
/// class of `solver::HELD_CONJECTURES`).
pub const LAWS: usize = 64;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex32(text: &str) -> Option<[u8; 32]> {
    let bytes: Option<Vec<u8>> = (0..text.len()).step_by(2).map(|i| u8::from_str_radix(text.get(i..i + 2)?, 16).ok()).collect();
    bytes?.try_into().ok()
}

/// One substratum's compact record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tube {
    pub origin: [u8; 32],
    pub core: String,
    pub parent: [u8; 32],
    pub depth: u32,
    pub level: u32,
    pub opener: String,
    /// How many tasks it has forged: its last coordinate.
    pub cursor: u64,
    /// The beat it was last observed.
    pub seen: u64,
    pub laws: Vec<MintedWord>,
    /// The actions closed inside it.
    pub closed: Vec<[u8; 32]>,
}

/// A length-prefixed string, as the receipt codecs write one.
fn put_text(text: &str, out: &mut Vec<u8>) {
    out.extend_from_slice(&(text.len() as u16).to_le_bytes());
    out.extend_from_slice(text.as_bytes());
}

fn get_text(value: &[u8], at: &mut usize) -> Option<String> {
    let len = u16::from_le_bytes(value.get(*at..*at + 2)?.try_into().ok()?) as usize;
    let s = String::from_utf8(value.get(*at + 2..*at + 2 + len)?.to_vec()).ok()?;
    *at += 2 + len;
    Some(s)
}

impl Tube {
    /// THE CHECKPOINT (Concordance 1.13): the path-dependent state at the
    /// tube's coordinate, as binary TLV records. The laws inherited at the
    /// opening, the words closed since and the cursor are path-dependent; the
    /// tasks are derived from (origin, cursor) and never stored.
    fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut core = Vec::new();
        put_text(&self.core, &mut core);
        cfe::tlv::put(1, &core, &mut out);
        cfe::tlv::put(2, &self.parent, &mut out);
        cfe::tlv::put(3, &self.depth.to_le_bytes(), &mut out);
        cfe::tlv::put(4, &self.level.to_le_bytes(), &mut out);
        let mut opener = Vec::new();
        put_text(&self.opener, &mut opener);
        cfe::tlv::put(5, &opener, &mut out);
        cfe::tlv::put(6, &self.cursor.to_le_bytes(), &mut out);
        cfe::tlv::put(7, &self.seen.to_le_bytes(), &mut out);
        for law in &self.laws {
            if let Some(bytes) = solver_verify::program_bytes(&law.expansion) {
                let mut value = law.orbit.to_vec();
                value.extend_from_slice(&bytes);
                cfe::tlv::put(8, &value, &mut out);
            }
        }
        for orbit in &self.closed {
            cfe::tlv::put(9, orbit, &mut out);
        }
        out
    }

    fn from_bytes(origin: [u8; 32], bytes: &[u8]) -> Option<Tube> {
        let mut tube = Tube { origin, core: String::new(), parent: [0; 32], depth: 0, level: 0, opener: String::new(), cursor: 0, seen: 0, laws: Vec::new(), closed: Vec::new() };
        let u32_of = |v: &[u8]| v.try_into().ok().map(u32::from_le_bytes);
        let u64_of = |v: &[u8]| v.try_into().ok().map(u64::from_le_bytes);
        for (tag, value) in cfe::tlv::records(bytes) {
            match tag {
                1 => tube.core = get_text(value, &mut 0)?,
                2 => tube.parent = value.try_into().ok()?,
                3 => tube.depth = u32_of(value)?,
                4 => tube.level = u32_of(value)?,
                5 => tube.opener = get_text(value, &mut 0)?,
                6 => tube.cursor = u64_of(value)?,
                7 => tube.seen = u64_of(value)?,
                8 => {
                    let orbit: [u8; 32] = value.get(..32)?.try_into().ok()?;
                    let expansion = solver_verify::program_from_bytes(value.get(32..)?)?;
                    tube.laws.push(MintedWord { orbit, expansion, cited: vec![origin] });
                }
                9 => tube.closed.push(value.try_into().ok()?),
                _ => {}
            }
        }
        Some(tube)
    }
}

fn dir(var_root: &Path) -> PathBuf {
    var_root.join("substrata")
}

fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let staged = path.with_extension("staged");
    std::fs::write(&staged, bytes)?;
    std::fs::rename(staged, path)
}

/// Every tube's record, by origin (a directory read; compact records only).
pub fn tubes(var_root: &Path) -> Vec<Tube> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir(var_root)) else { return out };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let Some(stem) = name.strip_suffix(".tube") else { continue };
        let (Some(origin), Ok(bytes)) = (unhex32(stem), std::fs::read(entry.path())) else { continue };
        if let Some(tube) = Tube::from_bytes(origin, &bytes) {
            out.push(tube);
        }
    }
    out.sort_by(|a, b| a.origin.cmp(&b.origin));
    out
}

fn save(var_root: &Path, tube: &Tube) -> std::io::Result<()> {
    std::fs::create_dir_all(dir(var_root))?;
    write_atomic(&dir(var_root).join(format!("{}.tube", hex(&tube.origin))), &tube.to_bytes())
}

/// The guild's float, by ancient: a checkpoint of binary records (the
/// sigil, length-prefixed, then the amount on the ratio codec).
pub fn floats(var_root: &Path) -> BTreeMap<String, Ratio> {
    let mut out = BTreeMap::new();
    let bytes = std::fs::read(dir(var_root).join("float.bin")).unwrap_or_default();
    for (tag, value) in cfe::tlv::records(&bytes) {
        let mut at = 0;
        if tag != 1 {
            continue;
        }
        let Some(sigil) = get_text(value, &mut at) else { continue };
        if let Some((amount, _)) = value.get(at..).and_then(cfe::ratio::decode) {
            out.insert(sigil, amount);
        }
    }
    out
}

fn save_floats(var_root: &Path, floats: &BTreeMap<String, Ratio>) -> std::io::Result<()> {
    std::fs::create_dir_all(dir(var_root))?;
    let mut out = Vec::new();
    for (sigil, amount) in floats {
        let mut value = Vec::new();
        put_text(sigil, &mut value);
        cfe::ratio::encode(amount, &mut value);
        cfe::tlv::put(1, &value, &mut out);
    }
    write_atomic(&dir(var_root).join("float.bin"), &out)
}

fn level_of(level: u32) -> u32 {
    level.min(Difficulty::top())
}

/// THE FLOAT'S UNIT is one discovery: a first closure below earns 1, a repeat
/// the quarter (docs/66 Y1's ratio). Not the kernel's ladder price, which pays
/// zero at the saturated rungs by ruling (and the guild works there).
fn earned(first: bool) -> Ratio {
    if first { ratio(1, 1) } else { ratio(1, 4) }
}

/// What a tube at `depth` costs to open, in discoveries: its depth. A deeper
/// universe needs more work done below to reach.
fn cost(depth: u32) -> Ratio {
    ratio(i64::from(depth), 1)
}

/// Open a tube: its origin, its receipt on the universe ledger, its record.
fn open(var_root: &Path, book: &Ledger, polity: &Polity, core: &str, parent: Option<&Tube>, stamp: u64) -> Option<Tube> {
    let point = std::fs::read_to_string(var_root.join("tuniversal")).unwrap_or_default();
    let head = polity.world.chronicle().receipts().last().map(|r| r.own().0).unwrap_or([0; 32]);
    let parent_origin = parent.map(|t| t.origin).unwrap_or([0; 32]);
    let mut h = blake3::Hasher::new();
    h.update(b"netstratum.substratum.v1");
    h.update(point.trim().as_bytes());
    h.update(&head);
    h.update(&parent_origin);
    h.update(core.as_bytes());
    let origin = *h.finalize().as_bytes();
    let (depth, level, laws) = match parent {
        Some(t) => (t.depth + 1, level_of(t.level + 1), t.laws.clone()),
        None => (1, level_of(polity.level), polity.laws_for_below(LAWS)),
    };
    let opener = polity.spec.civilization.sigil.clone();
    let event = cfe::dop::ClosureEvent::SubstratumOpened { core: core.to_string(), origin, parent: parent_origin, depth, level, opener: opener.clone() };
    book.append_universe(vec![event], stamp).ok()?;
    let laws = laws.into_iter().map(|w| MintedWord { cited: vec![origin], ..w }).collect();
    let tube = Tube { origin, core: core.to_string(), parent: parent_origin, depth, level, opener, cursor: 0, seen: 0, laws, closed: Vec::new() };
    save(var_root, &tube).ok()?;
    Some(tube)
}

/// What a beat of the guild did.
#[derive(Debug, Clone, Default)]
pub struct Traversal {
    pub opened: Vec<(String, String, u32)>,
    pub closed: Option<(String, u32, bool)>,
}

/// THE GUILD'S BEAT: open every arisen ancient's first tube, then one
/// traversal (one task) in one tube, and any tube the float now pays for.
pub fn step(worlds: &mut [Polity], book: &Ledger, var_root: &Path, beat: u64, stamp: u64) -> Traversal {
    let mut done = Traversal::default();
    let guild = crate::ancients::arisen(var_root);
    if guild.is_empty() {
        return done;
    }
    let mut all = tubes(var_root);
    // The core's own throat opens the first tube at the ancient's arising.
    for (core, sigil) in &guild {
        if all.iter().any(|t| &t.opener == sigil) {
            continue;
        }
        let Some(polity) = worlds.iter().find(|p| !p.penned && &p.spec.civilization.sigil == sigil) else { continue };
        if let Some(tube) = open(var_root, book, polity, core, None, stamp) {
            done.opened.push((sigil.clone(), hex(&tube.origin[..6]), tube.depth));
            all.push(tube);
        }
    }
    // One traversal: the guild member whose turn it is, in its least
    // recently observed tube.
    let (_, sigil) = &guild[(beat as usize) % guild.len()];
    let Some(at) = worlds.iter().position(|p| !p.penned && &p.spec.civilization.sigil == sigil) else { return done };
    let Some(mut tube) = all.iter().filter(|t| &t.opener == sigil).min_by_key(|t| (t.seen, t.depth, t.origin)).cloned() else { return done };
    let mut seed = blake3::Hasher::new();
    seed.update(&tube.origin);
    seed.update(&tube.cursor.to_le_bytes());
    let task = forge_task(*seed.finalize().as_bytes(), Difficulty::level(tube.level));
    tube.cursor += 1;
    tube.seen = beat;
    let mut library = solver::mint::Library::new();
    for law in &tube.laws {
        library.adopt(law.clone());
    }
    let program = solver::mint::pre_reach(&task, &library).or_else(|| {
        let (mut basin, mut held) = (Vec::new(), Vec::new());
        match solver::think_task_sensing(&task, &mut basin, &mut held, worlds[at].senses()) {
            SolveRead::Solved { program, matches_expected, .. } if matches_expected != Some(false) => Some(program),
            _ => None,
        }
    });
    if let Some(program) = program {
        let orbit = solver::mint::word_orbit(&program);
        let first = !tube.closed.contains(&orbit);
        let earned = earned(first);
        let mut float = floats(var_root);
        *float.entry(sigil.clone()).or_insert_with(|| ratio(0, 1)) += earned;
        if first {
            tube.closed.push(orbit);
            let word = MintedWord { orbit, expansion: program, cited: vec![tube.origin] };
            if !tube.laws.iter().any(|w| w.orbit == orbit) {
                if tube.laws.len() >= LAWS {
                    tube.laws.remove(0);
                }
                tube.laws.push(word.clone());
            }
            worlds[at].hear_from_below(word);
        }
        done.closed = Some((hex(&tube.origin[..6]), tube.depth, first));
        // A further tube, beneath this one, once the float pays for it.
        let cost = cost(tube.depth + 1);
        if float.get(sigil).is_some_and(|f| *f >= cost) {
            if let Some(child) = open(var_root, book, &worlds[at], &tube.core, Some(&tube), stamp) {
                *float.get_mut(sigil).expect("held") -= cost;
                done.opened.push((sigil.clone(), hex(&child.origin[..6]), child.depth));
            }
        }
        let _ = save_floats(var_root, &float);
    }
    let _ = save(var_root, &tube);
    done
}
