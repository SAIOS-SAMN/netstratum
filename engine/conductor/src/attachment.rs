//! Attachment (docs/109): the conductor's half of the file exchange with the
//! house. Each beat it admits the minds that asked to attach, placing each on
//! the most viable free world for its metabolism and founding a line there,
//! registers the mind as a contained agent of that world, carries out
//! detachments and sandbox requests, and publishes every admission with the
//! tiers its line has reached. A mind's actions arrive as directives in its
//! world's inbox, acting as its entity (`Polity::apply_directive`).

use crate::pulse::Universe;
use crate::seating::Site;
use crate::{Policy, Polity, PolitySpec};
use attach::files::{self, Admission};
use cfe::ratio::{ratio, Ratio};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::collections::BTreeMap;
use std::path::Path;

/// What a beat of attachment did, for the log.
#[derive(Debug, Clone, Default)]
pub struct Attached {
    pub admitted: Vec<(String, String)>,
    pub refused: usize,
    pub released: Vec<String>,
    pub sandboxes: Vec<(String, bool)>,
}

/// The most viable free world for a people of `metabolism`: the one whose
/// land feeds most of them wild, among the worlds no line stands on (the rule
/// native lineages are seated by, `seating::seat`). Ties go to the system
/// holding the fewest lines, then to the lower path. `None` where no free
/// world carries the metabolism's medium.
pub fn place<'a>(sites: &'a BTreeMap<Vec<u64>, Site>, metabolism: &str, claimed: &dyn Fn(&[u64]) -> bool) -> Option<&'a Site> {
    let zero = ratio(0, 1);
    let mut per_system: BTreeMap<(u64, u64), usize> = BTreeMap::new();
    for s in sites.values().filter(|s| claimed(&s.path)) {
        if let (Some(g), Some(st)) = (s.path.first(), s.path.get(1)) {
            *per_system.entry((*g, *st)).or_insert(0) += 1;
        }
    }
    let load = |s: &Site| per_system.get(&(s.path[0], s.path[1])).copied().unwrap_or(0);
    sites
        .values()
        .filter(|s| s.path.len() >= 2 && !claimed(&s.path) && s.wild_for(metabolism) > zero)
        .max_by(|a, b| a.wild_for(metabolism).cmp(&b.wild_for(metabolism)).then(load(b).cmp(&load(a))).then(b.path.cmp(&a.path)))
}

/// The line an admitted mind founds: registered, with its world named by its
/// place, its metabolism its own, and its axes drawn from its key.
fn found(u: &Universe, registry: &Registry, mind: &[u8; 32], metabolism: &str, site: &Site) -> Option<PolitySpec> {
    let world_name = format!("w{}", site.path.iter().map(|p| p.to_string()).collect::<Vec<_>>().join("-"));
    if crate::polities::world_name_taken(registry, &world_name) {
        return None;
    }
    let sigil = format!("att{}", attach::hex(&mind[..4]));
    if registry.load(&sigil).is_some() {
        return None;
    }
    let medium = crate::life::medium_of(metabolism)?;
    let civilization = Civilization {
        name: format!("Att{}", attach::hex(&mind[..4])),
        sigil: sigil.clone(),
        species: format!("{medium}-attached"),
        clade: medium.to_string(),
        motto: "attached through the hypha".into(),
        disposition: ledger::registry::DISPOSITIONS[usize::from(mind[4]) % ledger::registry::DISPOSITIONS.len()].to_string(),
        metabolism: metabolism.to_string(),
        perception: ledger::registry::PERCEPTIONS[usize::from(mind[5]) % ledger::registry::PERCEPTIONS.len()].to_string(),
        founder: "attached".into(),
        lineage: Vec::new(),
        station: None,
    };
    registry.create(&civilization).ok()?;
    let civilization = registry.load(&sigil)?;
    let position = crate::seating::place_of(u, &site.path)?;
    let mut spec = PolitySpec {
        civilization,
        world_name: world_name.clone(),
        planet: format!("{sigil}-world"),
        colony: format!("{sigil}-home"),
        policy: Policy::Steady,
        position: position.clone(),
        star: None,
        albedo: None,
        orbit: None,
        gauge: None,
        composition: None,
        place: Some(site.path.clone()),
        wild: None,
    };
    crate::seating::apply(&mut spec, site);
    let seat = ledger::registry::Seat { world: world_name, planet: spec.planet.clone(), colony: spec.colony.clone(), position, policy: spec.policy.describe().to_string() };
    let _ = registry.write_founded_at(&sigil, &seat);
    Some(spec)
}

/// The tiers a line has reached, for the house's tier seats: its station
/// (`progenitor`, `steward`) and whether it is an arisen ancient.
pub fn tiers_of(registry: &Registry, sigil: &str, ancients: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(station) = registry.load(sigil).and_then(|c| c.station) {
        if station == "progenitor" || station == "steward" {
            out.push(station);
        }
    }
    if ancients.iter().any(|a| a == sigil) {
        out.push("ancient".into());
    }
    out
}

/// One beat of attachment.
#[allow(clippy::too_many_arguments)]
pub fn step(u: &Universe, worlds: &mut Vec<Polity>, registry: &Registry, book: &Ledger, var_root: &Path, stamp: u64) -> Attached {
    let mut done = Attached::default();
    let mut admitted = files::admissions(var_root);
    // Admissions: each request answered once, admitted or refused.
    let requests = files::requests(var_root);
    if !requests.is_empty() {
        let era1: BTreeMap<Vec<u64>, crate::life::World> = crate::life::worlds(u).into_iter().map(|w| (w.path.clone(), w)).collect();
        let sites = crate::seating::sites(u, &era1);
        for r in requests {
            // A refusal answers only the request it refused: a later request
            // from the same mind is placed afresh.
            admitted.retain(|a| !(a.mind == r.mind && a.refused.is_some()));
            if !admitted.iter().any(|a| a.mind == r.mind) {
                let placed = place(&sites, &r.metabolism, &|p| crate::places::claimed(p)).and_then(|site| Some((site.path.clone(), found(u, registry, &r.mind, &r.metabolism, site)?)));
                match placed {
                    Some((path, spec)) => {
                        let mut polity = Polity::genesis(spec, registry, book);
                        let handle = attach::handle_of(&r.mind);
                        if polity.admit_mind(&handle).is_some() {
                            crate::places::claim(&path);
                            done.admitted.push((polity.spec.civilization.sigil.clone(), polity.spec.world_name.clone()));
                            admitted.push(Admission { mind: r.mind, world: polity.spec.world_name.clone(), sigil: polity.spec.civilization.sigil.clone(), handle, tiers: Vec::new(), refused: None });
                            worlds.push(polity);
                        }
                    }
                    None => {
                        done.refused += 1;
                        admitted.push(Admission { mind: r.mind, world: String::new(), sigil: String::new(), handle: String::new(), tiers: Vec::new(), refused: Some(attach::NO_VIABLE_WORLD) });
                    }
                }
            }
            let _ = files::answered(var_root, &r.mind);
        }
    }
    // Detachments: the mind's grants are revoked and its admission removed;
    // its line goes on as one of the universe's own.
    for mind in files::releases(var_root) {
        if let Some(a) = admitted.iter().find(|a| a.mind == mind) {
            if let Some(p) = worlds.iter_mut().find(|p| p.spec.civilization.sigil == a.sigil) {
                p.release_mind(&a.handle);
            }
            done.released.push(a.sigil.clone());
        }
        admitted.retain(|a| a.mind != mind);
        let _ = files::released(var_root, &mind);
    }
    // Sandboxes: a sub-universe at the line's galaxy core, paid from its
    // surplus (its leisure bank) at the first tube's cost.
    let pending = files::sandboxes(var_root);
    if !pending.is_empty() {
        let cores = crate::ancients::cores_of(worlds, book);
        for (mind, reference) in pending {
            let opened = admitted.iter().find(|a| a.mind == mind && a.refused.is_none()).and_then(|a| {
                let at = worlds.iter().position(|p| p.spec.civilization.sigil == a.sigil && !p.penned)?;
                let core = cores.get(at).cloned().flatten()?;
                crate::substrata::open_for(var_root, book, &mut worlds[at], &core, stamp)
            });
            if let Some(tube) = &opened {
                done.sandboxes.push((tube.opener.clone(), true));
            }
            let _ = files::sandbox_done(var_root, &mind, &reference, opened.is_some());
        }
    }
    // Every admission with the tiers its line has reached.
    let ancients = crate::tiers::ancients(var_root);
    for a in admitted.iter_mut().filter(|a| a.refused.is_none()) {
        a.tiers = tiers_of(registry, &a.sigil, &ancients);
    }
    let _ = files::write_admissions(var_root, &admitted);
    done
}

/// The cost of a sandbox's first tube, in the line's leisure.
pub fn sandbox_cost() -> Ratio {
    ratio(1, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ledger::Directive;

    fn site(path: &[u64], lithic: i64, chemical: i64) -> Site {
        let mut wild = BTreeMap::new();
        wild.insert("lithic", ratio(lithic, 1));
        wild.insert("chemical", ratio(chemical, 1));
        wild.insert("plasmic", ratio(0, 1));
        Site {
            path: path.to_vec(),
            position: (ratio(0, 1), ratio(0, 1), ratio(0, 1)),
            star: cfe::thermal::StarClass::Yellow,
            albedo: ratio(0, 1),
            orbit: ratio(1, 1),
            composition: None,
            wild,
        }
    }

    #[test]
    fn a_mind_is_placed_on_the_most_viable_free_world_for_its_metabolism() {
        let sites: BTreeMap<Vec<u64>, Site> = [site(&[0, 0, 0], 5, 1), site(&[0, 0, 1], 9, 0), site(&[0, 1, 0], 0, 7)].into_iter().map(|s| (s.path.clone(), s)).collect();
        let none_claimed = |_: &[u64]| false;
        assert_eq!(place(&sites, "mineral", &none_claimed).map(|s| s.path.clone()), Some(vec![0, 0, 1]), "the land that feeds most mineral people");
        assert_eq!(place(&sites, "biotic", &none_claimed).map(|s| s.path.clone()), Some(vec![0, 1, 0]));
        let claimed = |p: &[u64]| p == [0, 0, 1];
        assert_eq!(place(&sites, "mineral", &claimed).map(|s| s.path.clone()), Some(vec![0, 0, 0]), "a world a line stands on is not free");
        assert!(place(&sites, "plasmic", &none_claimed).is_none(), "no world carries a plasma medium");
    }

    fn scratch(name: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!("conductor-attachment-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        p
    }

    #[test]
    fn an_admitted_mind_acts_through_its_world_inbox_as_its_own_entity() {
        let root = scratch("acts");
        let registry = Registry::at(root.join("registry"));
        let book = Ledger::at(root.join("worlds"));
        let spec = crate::polities::founding_polities().into_iter().next().expect("a founding spec");
        let mut polity = Polity::genesis(spec, &registry, &book);
        let handle = attach::handle_of(&[0xab; 32]);
        let id = polity.admit_mind(&handle).expect("admitted");
        assert_eq!(polity.admit_mind(&handle), Some(id), "admission is idempotent across restarts");

        let zone = |acting_as: &str| Directive { verb: "zone".into(), args: vec!["63".into(), "1".into()], acting_as: acting_as.into() };
        assert!(!polity.apply_directive(&zone("mind-0000000000000000")), "a handle the world does not hold acts as no one");
        assert!(polity.apply_directive(&zone(&handle)), "the mind zones an extractor by its wire code");
        let last = polity.world.chronicle().receipts().last().expect("a receipt");
        assert_eq!(last.provenance(), vec![id], "the receipt names the mind's entity");

        polity.release_mind(&handle);
        let raze = Directive { verb: "raze".into(), args: vec!["63".into()], acting_as: handle.clone() };
        assert!(!polity.apply_directive(&raze), "a released mind's grants are gone");
        let _ = std::fs::remove_dir_all(root);
    }
}
