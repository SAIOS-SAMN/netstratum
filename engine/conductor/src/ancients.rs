//! THE ANCIENTS ARISE (Concordance 3.1; docs/58 §6, docs/61 G3, docs/68).
//!
//! An ancient is PRODUCED at a core by netstratum's own physics, never by
//! appointment or lineage alone (docs/58 §6.6):
//!
//! - **G3, the throat's keeper**, one law across the two kernels (barren's
//!   `gateways.hub_contest`): lines are grouped by their lineage's root (the
//!   progenitor line and the peoples it fathered); a group BEARS the hub when one
//!   of its peoples stands inside the core's bulge; of the bearing groups, the
//!   one whose peoples hold the most systems at that core keeps the throat, ties
//!   by size (population) and then by name. Depth (the nearest to the core)
//!   picks only the rider, the people the group keeps it through. Recomputed
//!   every bounce; no tenure is owed to hold it.
//! - **By birth**: a line inside a core's bulge that woke within one core
//!   assembly time of that core's first mind (the oldest line in the bulge).
//! - **By deed**, where no line qualifies by birth: the keeper's rider arises
//!   once its group has kept the throat for one full core period (from one
//!   bounce to the next) without losing it.
//!
//! **The bulge** is the core's own cell: torus L1 distance under one cell. It is
//! derived, not dialled: a cell is 1/32 Mpc (docs/58 §8), so any galactic bulge
//! (the Milky Way's is 2 kpc, barren's `R_BULGE_KPC`) lies inside the core's own
//! cell; and the core's bell, one cell a beat, fills that cell in its first beat.
//! **The core assembly time** is therefore one beat: the bell's crossing of the
//! bulge. **The core period** is the core's own bounce period: `core_bounce`
//! re-emits every core's holdings once a year.
//!
//! What arises is recorded on the universe ledger (`AncientArose`, tag 192:
//! the line's sigil, the systems its group holds at the core as `tree`, the
//! living lines at the core as `living`) and in `<var>/ancients.bin` (binary
//! records, Concordance 1.13), so a core's ancient arises once. The first to
//! arise ends the trust (docs/58 §6.7): `<var>/trust.bin` names it, the core,
//! and its reserve basket.
use crate::Polity;
use cfe::ratio::{ratio, Ratio};
use ledger::registry::Registry;
use ledger::Ledger;
use std::collections::BTreeMap;
use std::path::Path;

/// The core assembly time, in beats: the bell's crossing of the bulge.
pub const ASSEMBLY_BEATS: u64 = 1;

/// Is `position` inside the bulge of the core at `centre`?
pub fn in_bulge(position: &(Ratio, Ratio, Ratio), centre: &(Ratio, Ratio, Ratio), span: &Ratio) -> bool {
    ledger::torus_distance(position, centre, span) < ratio(1, 1)
}

/// The root of a line's lineage: its first parent followed to the end.
pub(crate) fn root_of(sigil: &str, parents: &BTreeMap<String, Vec<String>>) -> String {
    let mut at = sigil.to_string();
    let mut seen = std::collections::BTreeSet::new();
    while let Some(up) = parents.get(&at).and_then(|p| p.first()) {
        if !seen.insert(at.clone()) {
            break;
        }
        at = up.clone();
    }
    at
}

/// The throat's keeper of one core (G3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keeper {
    /// The lineage root whose group keeps the throat.
    pub line: String,
    /// The systems the group holds at this core.
    pub systems: u64,
    /// The people it keeps it through: the group's line nearest the core.
    pub rider: String,
}

/// Every core's keeper, read from where the living lines stand now (G3).
pub fn keepers(worlds: &[Polity], book: &Ledger, registry: &Registry) -> BTreeMap<String, Keeper> {
    let parents: BTreeMap<String, Vec<String>> = registry.list().into_iter().map(|c| (c.sigil, c.lineage)).collect();
    let span = book.universe_span();
    let mut out = BTreeMap::new();
    for galaxy in book.galaxies() {
        // (systems, population, riders as (distance, sigil)) per lineage root.
        let mut groups: BTreeMap<String, (u64, u64, Vec<(Ratio, String)>)> = BTreeMap::new();
        for polity in worlds.iter().filter(|p| !p.penned) {
            if book.core_of(&polity.spec.position).as_deref() != Some(galaxy.name.as_str()) {
                continue;
            }
            let sigil = &polity.spec.civilization.sigil;
            let entry = groups.entry(root_of(sigil, &parents)).or_insert_with(|| (0, 0, Vec::new()));
            entry.0 += polity.world.orbs().len() as u64;
            entry.1 += polity.world.population_of(cfe::world::FIRST_ORB);
            if in_bulge(&polity.spec.position, &galaxy.center, &span) {
                entry.2.push((ledger::torus_distance(&polity.spec.position, &galaxy.center, &span), sigil.clone()));
            }
        }
        let best = groups
            .into_iter()
            .filter_map(|(line, (systems, size, mut riders))| {
                riders.sort();
                let rider = riders.into_iter().next()?.1;
                Some((systems, size, line, rider))
            })
            .max_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(b.2.cmp(&a.2)));
        if let Some((systems, _, line, rider)) = best {
            out.insert(galaxy.name.clone(), Keeper { line, systems, rider });
        }
    }
    out
}

/// What a core's book holds: its ancient once arisen, and its keeper's tenure.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CoreBook {
    pub arisen: Option<String>,
    /// The keeping lineage root and the bounce it has kept the throat since.
    pub kept: Option<(String, u64)>,
}

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

/// The cores' books, a CHECKPOINT of binary records (Concordance 1.13): tag 1
/// an ancient arisen (core, sigil), tag 2 a keeper's tenure (core, root, the
/// bounce since). A text book written before 1.13 is read once, then rewritten.
fn read_books(var_root: &Path) -> BTreeMap<String, CoreBook> {
    let mut books: BTreeMap<String, CoreBook> = BTreeMap::new();
    let Ok(bytes) = std::fs::read(var_root.join("ancients.bin")) else {
        for line in std::fs::read_to_string(var_root.join("ancients")).unwrap_or_default().lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.as_slice() {
                ["arose", core, sigil] => books.entry(core.to_string()).or_default().arisen = Some(sigil.to_string()),
                ["kept", core, root, since] => {
                    if let Ok(since) = since.parse() {
                        books.entry(core.to_string()).or_default().kept = Some((root.to_string(), since));
                    }
                }
                _ => {}
            }
        }
        return books;
    };
    for (tag, value) in cfe::tlv::records(&bytes) {
        let mut at = 0;
        let Some(core) = get_text(value, &mut at) else { continue };
        match tag {
            1 => books.entry(core).or_default().arisen = get_text(value, &mut at),
            2 => {
                let Some(root) = get_text(value, &mut at) else { continue };
                let Some(since) = value.get(at..at + 8).and_then(|b| b.try_into().ok()).map(u64::from_le_bytes) else { continue };
                books.entry(core).or_default().kept = Some((root, since));
            }
            _ => {}
        }
    }
    books
}

fn write_books(var_root: &Path, books: &BTreeMap<String, CoreBook>) -> std::io::Result<()> {
    let mut out = Vec::new();
    for (core, book) in books {
        if let Some(sigil) = &book.arisen {
            let mut value = Vec::new();
            put_text(core, &mut value);
            put_text(sigil, &mut value);
            cfe::tlv::put(1, &value, &mut out);
        }
        if let Some((root, since)) = &book.kept {
            let mut value = Vec::new();
            put_text(core, &mut value);
            put_text(root, &mut value);
            value.extend_from_slice(&since.to_le_bytes());
            cfe::tlv::put(2, &value, &mut out);
        }
    }
    let path = var_root.join("ancients.bin");
    let staged = path.with_extension("staged");
    std::fs::write(&staged, out)?;
    std::fs::rename(staged, path)
}

/// The arisen ancients, `(core, sigil)`, in core order.
pub fn arisen(var_root: &Path) -> Vec<(String, String)> {
    read_books(var_root).into_iter().filter_map(|(core, book)| book.arisen.map(|sigil| (core, sigil))).collect()
}

/// An ancient as it arose: the core, the line, and by which rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arisen {
    pub core: String,
    pub sigil: String,
    pub world: String,
    pub by_birth: bool,
}

/// THE ANCIENTS' STEP, once a bounce (`bounce` counts the core periods): read
/// the keepers, advance their tenure, raise every ancient now due, record each
/// on the universe ledger, and end the trust at the first. Returns the keepers
/// and what arose.
pub fn step(
    worlds: &[Polity],
    book: &Ledger,
    registry: &Registry,
    var_root: &Path,
    bounce: u64,
    stamp: u64,
) -> (BTreeMap<String, Keeper>, Vec<Arisen>) {
    let span = book.universe_span();
    let mut books = read_books(var_root);
    let keepers = keepers(worlds, book, registry);
    let parents: BTreeMap<String, Vec<String>> = registry.list().into_iter().map(|c| (c.sigil, c.lineage)).collect();
    let mut arisen = Vec::new();
    for galaxy in book.galaxies() {
        let core = galaxy.name.clone();
        let entry = books.entry(core.clone()).or_default();
        // G3 tenure: a keeper unchanged keeps its since; a change restarts it.
        entry.kept = match (keepers.get(&core), entry.kept.take()) {
            (Some(k), Some((root, since))) if root == k.line => Some((root, since)),
            (Some(k), _) => Some((k.line.clone(), bounce)),
            (None, _) => None,
        };
        if entry.arisen.is_some() {
            continue;
        }
        let living: Vec<&Polity> = worlds
            .iter()
            .filter(|p| !p.penned && book.core_of(&p.spec.position).as_deref() == Some(core.as_str()))
            .collect();
        // BY BIRTH: the bulge's first mind and every line that woke within one
        // assembly time of it (a line's clock is its age).
        let bulge: Vec<&&Polity> = living.iter().filter(|p| in_bulge(&p.spec.position, &galaxy.center, &span)).collect();
        let first = bulge.iter().map(|p| p.world.now()).max();
        let mut born: Vec<&&Polity> = bulge
            .iter()
            .copied()
            .filter(|p| first.is_some_and(|f| f.saturating_sub(p.world.now()) <= ASSEMBLY_BEATS))
            .collect();
        born.sort_by(|a, b| b.world.now().cmp(&a.world.now()).then(a.spec.civilization.sigil.cmp(&b.spec.civilization.sigil)));
        let (rising, by_birth) = match born.first() {
            Some(p) => (Some(**p), true),
            // BY DEED: the keeper's rider, once a full core period is kept.
            None => match (keepers.get(&core), &entry.kept) {
                (Some(k), Some((_, since))) if bounce >= since + 1 => {
                    (living.iter().copied().find(|p| p.spec.civilization.sigil == k.rider), false)
                }
                _ => (None, false),
            },
        };
        let Some(polity) = rising else { continue };
        let sigil = polity.spec.civilization.sigil.clone();
        let root = root_of(&sigil, &parents);
        let tree: u64 = living
            .iter()
            .filter(|p| root_of(&p.spec.civilization.sigil, &parents) == root)
            .map(|p| p.world.orbs().len() as u64)
            .sum();
        let event = cfe::dop::ClosureEvent::AncientArose { sigil: sigil.clone(), tree, living: living.len() as u64 };
        if book.append_universe(vec![event], stamp).is_err() {
            continue;
        }
        entry.arisen = Some(sigil.clone());
        arisen.push(Arisen { core: core.clone(), sigil, world: polity.spec.world_name.clone(), by_birth });
    }
    let _ = write_books(var_root, &books);
    // THE TRUST ENDS at the first ancient (docs/58 §6.7-6.8): the oldest of
    // those arising now, then by name. Its reserve basket is stated by mass:
    // what its phenotype eats, and how much a beat its working mouths need.
    // The trust's passing is an INPUT (Concordance 1.13), stored once as binary
    // records in `<var>/trust.bin`: tag 1 the ancient, 2 its world, 3 its core,
    // 4 the rule (1 birth, 2 deed), 5 the stamp, 6 the basket (its ration on
    // the receipt codec's carrier: 0 ‖ resource code, or 1 ‖ element id), 7 its
    // working mouths' need a beat (ratio codec). A text trust written before
    // 1.13 stands as the same passing.
    let trust = var_root.join("trust.bin");
    if !trust.exists() && !var_root.join("trust").exists() {
        let first = arisen
            .iter()
            .filter_map(|a| worlds.iter().find(|p| p.spec.world_name == a.world).map(|p| (a, p)))
            .max_by(|(a, p), (b, q)| p.world.now().cmp(&q.world.now()).then(b.sigil.cmp(&a.sigil)));
        if let Some((a, p)) = first {
            let orb = cfe::world::FIRST_ORB;
            let mut out = Vec::new();
            for (tag, text) in [(1u8, &a.sigil), (2, &a.world), (3, &a.core)] {
                let mut value = Vec::new();
                put_text(text, &mut value);
                cfe::tlv::put(tag, &value, &mut out);
            }
            cfe::tlv::put(4, &[if a.by_birth { 1 } else { 2 }], &mut out);
            cfe::tlv::put(5, &stamp.to_le_bytes(), &mut out);
            let basket = match p.world.ration_of(orb) {
                cfe::dop::Ration::Resource(r) => vec![0, r as u8],
                cfe::dop::Ration::Element(id) => [&[1u8][..], &id[..]].concat(),
            };
            cfe::tlv::put(6, &basket, &mut out);
            let mut need = Vec::new();
            cfe::ratio::encode(&(p.world.per_capita_ration() * ratio(p.world.labor_force(orb) as i64, 1)), &mut need);
            cfe::tlv::put(7, &need, &mut out);
            let _ = std::fs::write(&trust, out);
        }
    }
    (keepers, arisen)
}

/// THE COUNCILS (Concordance 3.9): each arisen ancient's house seats a council
/// of the mycelium client minds, and client minds seat there and nowhere
/// else. The house (ns-hypha) lists council seats on exactly the worlds named
/// in `<var>/council.bin`: binary records, tag 1 an ancient's world.
pub fn write_council(worlds: &[Polity], var_root: &Path) -> std::io::Result<()> {
    let mut out = Vec::new();
    for (_, sigil) in arisen(var_root) {
        if let Some(p) = worlds.iter().find(|p| !p.penned && p.spec.civilization.sigil == sigil) {
            cfe::tlv::put(1, p.spec.world_name.as_bytes(), &mut out);
        }
    }
    let path = var_root.join("council.bin");
    let staged = path.with_extension("staged");
    std::fs::write(&staged, out)?;
    std::fs::rename(staged, path)
}

