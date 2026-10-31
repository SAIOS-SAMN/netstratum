//! Realms (docs/75; operator 2026-10-13): realms form only where an
//! ancient's progenitors have deployed a passage network and passage crossed
//! it. Space no network reached belongs to no realm; the known universe is
//! never partitioned (Concordance 3.3: a realm is never laid in advance).
//!
//! - A line's CIVILIZATION is its lineage root (`ancients::root_of`).
//! - A realm is named by its core: the gates leading to one core (built by
//!   its parliament for its ancients, docs/72) and the passage over them.
//! - Passage in a term, per line and core: a hub leg that rode a mapped gate
//!   to that core (`Passages`, from the hub step), and an envoy standing at
//!   that core at the term's close (`ascension.bin`).
//! - A realm exists when passage to its core was counted. Its members are the
//!   civilizations whose lines used its network the most, and the progenitors
//!   operating it (the builders' lineages of its mapped gates). A tie between
//!   networks leaves a civilization contested.
//! - A boundary is a civilization reached by more than one realm's network.
//! - The steward is the most-contacted member line (contacts: the term's
//!   completed exchanges, `Contacts`), else the line with the most passage;
//!   it takes the steward's seat (`tiers`).
//! - A captain is a member civilization's most-travelled line over the realm's
//!   network in the term; it takes the captain's seat (`tiers`).
//!
//! The term is the core period (one bounce). Placeholders for physics not yet
//! defined: civilization = lineage root, passage = a gated leg or an envoy,
//! membership = the network used most, steward = the most-contacted member.
//! Checkpoint `<var>/realms.bin` (Concordance 1.13): tag 1 a realm (core,
//! steward's world, then each member civilization), tag 2 a boundary (the
//! civilization, then each core whose network reached it).
use crate::Polity;
use ledger::registry::Registry;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The term's completed exchanges between lines, by the pair of their worlds
/// (ordered), counted.
#[derive(Debug, Clone, Default)]
pub struct Contacts {
    pairs: BTreeMap<(String, String), u64>,
}

impl Contacts {
    /// One completed exchange between the lines on worlds `a` and `b`.
    pub fn add(&mut self, a: &str, b: &str) {
        if a == b {
            return;
        }
        let key = if a < b {
            (a.to_string(), b.to_string())
        } else {
            (b.to_string(), a.to_string())
        };
        *self.pairs.entry(key).or_insert(0) += 1;
    }

    /// The worlds the line on world `world` exchanged with this term.
    pub fn partners(&self, world: &str) -> Vec<String> {
        self.pairs
            .keys()
            .filter_map(|(a, b)| {
                if a == world {
                    Some(b.clone())
                } else if b == world {
                    Some(a.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// The exchanges counted this term.
    pub fn total(&self) -> u64 {
        self.pairs.values().sum()
    }

    /// A new term.
    pub fn clear(&mut self) {
        self.pairs.clear();
    }
}

/// The term's passage over progenitor networks, by (line's world, core),
/// counted.
#[derive(Debug, Clone, Default)]
pub struct Passages {
    by: BTreeMap<(String, String), u64>,
}

impl Passages {
    /// One passage by the line on world `line` over the network to `core`.
    pub fn add(&mut self, line: &str, core: &str) {
        *self
            .by
            .entry((line.to_string(), core.to_string()))
            .or_insert(0) += 1;
    }

    /// A new term.
    pub fn clear(&mut self) {
        self.by.clear();
    }
}

fn put_text(text: &str, out: &mut Vec<u8>) {
    out.extend_from_slice(&(text.len() as u16).to_le_bytes());
    out.extend_from_slice(text.as_bytes());
}

/// The realms' reading at a term's close: realms by core, their members,
/// boundaries, contested civilizations and stewards, checkpointed, and what
/// happened for the log, and the stewards and captains it seats.
pub fn bounce(
    worlds: &mut [Polity],
    registry: &Registry,
    var_root: &Path,
    contacts: &Contacts,
    passages: &Passages,
    bounce: u64,
) -> Vec<String> {
    let parents: BTreeMap<String, Vec<String>> = registry
        .list()
        .into_iter()
        .map(|c| (c.sigil, c.lineage))
        .collect();
    let living: Vec<usize> = (0..worlds.len()).filter(|&i| !worlds[i].penned).collect();
    let civ_of_world: BTreeMap<String, String> = living
        .iter()
        .map(|&i| {
            (
                worlds[i].spec.world_name.clone(),
                crate::ancients::root_of(&worlds[i].spec.civilization.sigil, &parents),
            )
        })
        .collect();
    let world_of_sigil: BTreeMap<String, String> = living
        .iter()
        .map(|&i| {
            (
                worlds[i].spec.civilization.sigil.clone(),
                worlds[i].spec.world_name.clone(),
            )
        })
        .collect();
    // Passage this term per (line, core): the hub's gated legs, and the
    // envoys standing at their cores.
    let mut by_line: BTreeMap<(String, String), u64> = passages.by.clone();
    for (sigil, core) in crate::ascension::envoys(var_root) {
        if let Some(world) = world_of_sigil.get(&sigil) {
            *by_line.entry((world.clone(), core)).or_insert(0) += 1;
        }
    }
    // Each civilization's use of each core's network.
    let mut use_of: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    let mut line_passage: BTreeMap<String, u64> = BTreeMap::new();
    for ((world, core), n) in &by_line {
        let Some(civ) = civ_of_world.get(world) else {
            continue;
        };
        *use_of
            .entry(civ.clone())
            .or_default()
            .entry(core.clone())
            .or_insert(0) += n;
        *line_passage.entry(world.clone()).or_insert(0) += n;
    }
    // Membership: the network a civilization used most; a tie, contested.
    let mut members: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut contested: Vec<String> = Vec::new();
    let mut boundaries: Vec<(String, Vec<String>)> = Vec::new();
    for (civ, uses) in &use_of {
        let most = uses.values().copied().max().unwrap_or(0);
        let top: Vec<&String> = uses
            .iter()
            .filter(|(_, n)| **n == most)
            .map(|(c, _)| c)
            .collect();
        if top.len() == 1 {
            members
                .entry(top[0].clone())
                .or_default()
                .insert(civ.clone());
        } else {
            contested.push(civ.clone());
        }
        if uses.len() > 1 {
            boundaries.push((civ.clone(), uses.keys().cloned().collect()));
        }
    }
    // The progenitors operating each realm's network: its mapped gates'
    // builders.
    let (gates, _) = crate::ascension::read(var_root);
    for g in gates.iter().filter(|g| g.mapped) {
        if let Some(set) = members.get_mut(&g.core) {
            set.insert(g.builder.clone());
        }
    }
    // Each realm's steward: its most-contacted member line, else the most
    // passage; ties by name.
    let mut line_contacts: BTreeMap<String, u64> = BTreeMap::new();
    for ((a, b), n) in &contacts.pairs {
        *line_contacts.entry(a.clone()).or_insert(0) += n;
        *line_contacts.entry(b.clone()).or_insert(0) += n;
    }
    let mut said = Vec::new();
    let mut out = Vec::new();
    let mut stewards: Vec<(String, String)> = Vec::new();
    for (core, set) in &members {
        let in_realm = |w: &String| civ_of_world.get(w).is_some_and(|c| set.contains(c));
        let pick = |scores: &BTreeMap<String, u64>| {
            scores
                .iter()
                .filter(|(w, n)| **n > 0 && in_realm(*w))
                .max_by(|(wa, na), (wb, nb)| na.cmp(nb).then(wb.cmp(wa)))
                .map(|(w, _)| w.clone())
        };
        let steward = pick(&line_contacts).or_else(|| pick(&line_passage));
        if let Some(w) = &steward {
            stewards.push((core.clone(), w.clone()));
        }
        let passed: u64 = by_line
            .iter()
            .filter(|((_, c), _)| c == core)
            .map(|(_, n)| n)
            .sum();
        let mut v = Vec::new();
        put_text(core, &mut v);
        put_text(steward.as_deref().unwrap_or(""), &mut v);
        for c in set {
            put_text(c, &mut v);
        }
        cfe::tlv::put(1, &v, &mut out);
        said.push(format!(
            "REALM {core} — steward {}; {} civilizations ({}); {passed} passages over its network this term",
            steward.as_deref().unwrap_or("none"),
            set.len(),
            set.iter().cloned().collect::<Vec<_>>().join(", ")
        ));
    }
    for (civ, cores) in &boundaries {
        let mut v = Vec::new();
        put_text(civ, &mut v);
        for c in cores {
            put_text(c, &mut v);
        }
        cfe::tlv::put(2, &v, &mut out);
        said.push(format!(
            "BOUNDARY — {civ} is reached by the networks of {}",
            cores.join(", ")
        ));
    }
    said.insert(
        0,
        format!(
            "REALMS (term {bounce}) — {} realms where progenitor networks carried passage; {} boundaries; {} contested; no realm elsewhere",
            members.len(),
            boundaries.len(),
            contested.len()
        ),
    );
    // The TIERS emerge from the realms (`tiers`): each realm's steward takes
    // the steward's seat, and each member civilization's most-travelled line
    // over the realm's network this term (ties by world name) its captain's.
    let ancients = crate::tiers::ancients(var_root);
    let index_of: BTreeMap<String, usize> = living
        .iter()
        .map(|&i| (worlds[i].spec.world_name.clone(), i))
        .collect();
    for (core, w) in &stewards {
        let Some(&i) = index_of.get(w) else { continue };
        if crate::tiers::raise(&mut worlds[i], registry, "steward", &ancients) {
            said.push(format!(
                "A STEWARD EMERGES — {} stewards the realm {core}",
                worlds[i].spec.civilization.sigil
            ));
        }
    }
    let mut captains: BTreeMap<(String, String), (u64, String)> = BTreeMap::new();
    for ((world, core), n) in &by_line {
        let Some(civ) = civ_of_world.get(world) else {
            continue;
        };
        if *n == 0 || !members.get(core).is_some_and(|set| set.contains(civ)) {
            continue;
        }
        let best = captains
            .entry((core.clone(), civ.clone()))
            .or_insert((0, String::new()));
        if *n > best.0 {
            *best = (*n, world.clone());
        }
    }
    for ((core, civ), (n, w)) in &captains {
        let Some(&i) = index_of.get(w) else { continue };
        if crate::tiers::raise(&mut worlds[i], registry, "captain", &ancients) {
            said.push(format!(
                "A CAPTAIN EMERGES — {} of {civ}, {n} passages over the realm {core}'s roads",
                worlds[i].spec.civilization.sigil
            ));
        }
    }
    let path = var_root.join("realms.bin");
    let staged = path.with_extension("staged");
    if std::fs::write(&staged, out).is_ok() {
        let _ = std::fs::rename(staged, path);
    }
    said
}
