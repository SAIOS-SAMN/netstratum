//! The TIERS emerge (ruling of 2026-10-21: ancients near the core;
//! progenitors; stewards as realms emerge, the realms emerging rather than
//! being drawn; sovereigns and captains; and the rest). No line is seated a tier
//! at founding; each is reached by what its lineage does:
//!
//! - **ancient**: arises at a core (`ancients`), the council above every seat;
//! - **progenitor**: the builders' line of a gate its lineage folded
//!   (`ascension`, at the fold);
//! - **steward**: a realm's steward, read from the term's passage over the
//!   realm's network (`realms`);
//! - **sovereign**: the eldest line of a lineage that holds two or more worlds
//!   of one star system, no other lineage holding a world there (`sovereigns`);
//! - **captain**: a civilization's most-travelled line over a realm's network
//!   in a term (`realms`);
//! - **the rest**: no seat, or an office its economy earned (`office_step`).
//!
//! A line takes an emergent seat only when it rises in this order, and keeps a
//! seat once taken.
use crate::Polity;
use ledger::registry::Registry;
use std::collections::BTreeMap;
use std::path::Path;

/// A station's place in the emergent order (lower is higher); the watchers
/// and observers stand beside the stewards, and every office and the common
/// below the captains.
pub fn rank(station: Option<&str>) -> u8 {
    match station {
        Some("progenitor") => 1,
        Some("steward") | Some("watcher") | Some("observer") => 2,
        Some("sovereign") => 3,
        Some("captain") => 4,
        _ => 5,
    }
}

/// Whether taking `want` raises a line holding `held`.
pub fn rises(held: Option<&str>, want: &str) -> bool {
    rank(Some(want)) < rank(held)
}

/// Seat `want` on the line if it rises there and is not an ancient.
pub fn raise(p: &mut Polity, registry: &Registry, want: &str, ancients: &[String]) -> bool {
    !ancients.contains(&p.spec.civilization.sigil) && rises(p.spec.civilization.station.as_deref(), want) && p.take_station(registry, want)
}

/// The living ancients' lines.
pub fn ancients(var_root: &Path) -> Vec<String> {
    crate::ancients::arisen(var_root).into_iter().map(|(_, s)| s).collect()
}

fn depth(sigil: &str, parents: &BTreeMap<String, Vec<String>>) -> usize {
    let mut at = sigil.to_string();
    let mut n = 0;
    while let Some(up) = parents.get(&at).and_then(|p| p.first()) {
        n += 1;
        if n > parents.len() {
            break;
        }
        at = up.clone();
    }
    n
}

/// The sovereigns: every star system whose era-1 worlds are held by lines of
/// one lineage only, two or more of them, seats the lineage's eldest line
/// there (fewest generations from its root, then the first seated).
pub fn sovereigns(worlds: &mut [Polity], registry: &Registry, var_root: &Path) -> Vec<String> {
    let parents: BTreeMap<String, Vec<String>> = registry.list().into_iter().map(|c| (c.sigil, c.lineage)).collect();
    let ancients = ancients(var_root);
    let mut systems: BTreeMap<(u64, u64), Vec<(usize, String)>> = BTreeMap::new();
    for (i, p) in worlds.iter().enumerate() {
        if p.penned {
            continue;
        }
        let Some(path) = p.spec.place.as_deref() else { continue };
        let (Some(&g), Some(&s)) = (path.first(), path.get(1)) else { continue };
        systems.entry((g, s)).or_default().push((i, crate::ancients::root_of(&p.spec.civilization.sigil, &parents)));
    }
    let mut said = Vec::new();
    for ((g, s), lines) in systems {
        if lines.len() < 2 || lines.iter().any(|(_, r)| *r != lines[0].1) {
            continue;
        }
        let Some(&(eldest, _)) = lines.iter().min_by_key(|(i, _)| (depth(&worlds[*i].spec.civilization.sigil, &parents), *i)) else { continue };
        if raise(&mut worlds[eldest], registry, "sovereign", &ancients) {
            said.push(format!(
                "A SOVEREIGN EMERGES — {} holds the system {g}/{s}: {} worlds of the lineage {}",
                worlds[eldest].spec.civilization.sigil,
                lines.len(),
                lines[0].1
            ));
        }
    }
    said
}

#[cfg(test)]
mod tests {
    use super::{rank, rises};

    #[test]
    fn a_seat_is_taken_only_by_rising() {
        assert!(rises(None, "captain"));
        assert!(rises(Some("caravan"), "sovereign"));
        assert!(rises(Some("sovereign"), "steward"));
        assert!(rises(Some("steward"), "progenitor"));
        assert!(!rises(Some("progenitor"), "steward"));
        assert!(!rises(Some("sovereign"), "sovereign"));
        assert!(!rises(Some("watcher"), "sovereign"));
        assert_eq!(rank(Some("exchange")), rank(None));
    }
}
