//! Attachment (docs/109 §4): the conductor's half of the file exchange with
//! the house, read each time the universe changes. A mind that asked to
//! attach couples to a world still forming its orbit whose land feeds its
//! metabolism; the coupling rides the body through the system's joins
//! (`formation::settle_system`), and when the body's orbit is solid the mind
//! is admitted: a line is founded on that world and the mind is registered as
//! its contained agent. Detachments and sandbox requests are carried out, and
//! every admission is published with the tiers its line has reached. A mind's
//! actions arrive as directives in its world's inbox, acting as its entity
//! (`Polity::apply_directive`).

use crate::pulse::{Body, Universe};
use crate::seating::Site;
use crate::{Policy, Polity, PolitySpec};
use attach::files::{self, Admission};
use cfe::ratio::{ratio, Ratio};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::collections::BTreeMap;
use std::path::Path;

/// What a reading of attachment did, for the log.
#[derive(Debug, Clone, Default)]
pub struct Attached {
    pub coupled: Vec<(String, Vec<u64>)>,
    pub admitted: Vec<(String, String)>,
    pub released: Vec<String>,
    pub sandboxes: Vec<(String, bool)>,
}

/// Every world of the universe (a star system's satellite), mutable.
fn worlds_mut(u: &mut Universe) -> impl Iterator<Item = &mut Body> {
    u.cores
        .iter_mut()
        .flat_map(|c| c.satellites.iter_mut())
        .flat_map(|s| s.satellites.iter_mut())
}

/// Every world of the universe.
fn worlds_of(u: &Universe) -> impl Iterator<Item = &Body> {
    u.cores
        .iter()
        .flat_map(|c| c.satellites.iter())
        .flat_map(|s| s.satellites.iter())
}

/// The forming world a request couples to: a world whose orbit is not yet
/// solid, that no mind is coupled to and no line stands on, whose land feeds a
/// people of `metabolism` (its site's wild land); the one feeding most of
/// them, then the lower path. `None` where no forming world fits yet.
pub fn target(
    worlds: &[(Vec<u64>, bool, bool)],
    sites: &BTreeMap<Vec<u64>, Site>,
    metabolism: &str,
    claimed: &dyn Fn(&[u64]) -> bool,
) -> Option<Vec<u64>> {
    let zero = ratio(0, 1);
    worlds
        .iter()
        .filter(|(path, solid, coupled)| !solid && !coupled && !claimed(path))
        .filter_map(|(path, _, _)| Some((path, sites.get(path)?.wild_for(metabolism))))
        .filter(|(_, wild)| *wild > zero)
        .max_by(|(pa, a), (pb, b)| a.cmp(b).then(pb.cmp(pa)))
        .map(|(path, _)| path.clone())
}

/// The line an admitted mind founds: registered, with its world named by its
/// place, its metabolism its own, and its axes drawn from its key.
fn found(
    u: &Universe,
    registry: &Registry,
    mind: &[u8; 32],
    metabolism: &str,
    site: &Site,
) -> Option<PolitySpec> {
    let world_name = format!(
        "w{}",
        site.path
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join("-")
    );
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
        disposition: ledger::registry::DISPOSITIONS
            [usize::from(mind[4]) % ledger::registry::DISPOSITIONS.len()]
        .to_string(),
        metabolism: metabolism.to_string(),
        perception: ledger::registry::PERCEPTIONS
            [usize::from(mind[5]) % ledger::registry::PERCEPTIONS.len()]
        .to_string(),
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
    let seat = ledger::registry::Seat {
        world: world_name,
        planet: spec.planet.clone(),
        colony: spec.colony.clone(),
        position,
        policy: spec.policy.describe().to_string(),
    };
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

/// A reading of attachment, made each time the universe changes.
pub fn read(
    u: &mut Universe,
    worlds: &mut Vec<Polity>,
    registry: &Registry,
    book: &Ledger,
    var_root: &Path,
    stamp: u64,
) -> Attached {
    let mut done = Attached::default();
    let mut admitted = files::admissions(var_root);
    let requests = files::requests(var_root);
    let detaching = files::releases(var_root);
    // A pending request that asked to detach before it was admitted is
    // withdrawn, its coupling released.
    for mind in &detaching {
        if !admitted.iter().any(|a| a.mind == *mind) {
            for w in worlds_mut(u).filter(|w| w.coupled.as_ref() == Some(mind)) {
                w.coupled = None;
            }
            let _ = files::answered(var_root, mind);
        }
    }
    let pending: Vec<files::Request> = requests
        .into_iter()
        .filter(|r| !detaching.contains(&r.mind) && !admitted.iter().any(|a| a.mind == r.mind))
        .collect();
    if !pending.is_empty() {
        let era1: BTreeMap<Vec<u64>, crate::life::World> = crate::life::worlds(u)
            .into_iter()
            .map(|w| (w.path.clone(), w))
            .collect();
        let sites = crate::seating::sites(u, &era1);
        for r in &pending {
            // Admission: the coupled world's orbit is solid.
            let solid_world = worlds_of(u)
                .find(|w| w.coupled == Some(r.mind) && w.solid)
                .map(|w| w.path.clone());
            if let Some(path) = solid_world {
                let founded = sites
                    .get(&path)
                    .and_then(|site| found(u, registry, &r.mind, &r.metabolism, site));
                if let Some(spec) = founded {
                    let mut polity = Polity::genesis(spec, registry, book);
                    let handle = attach::handle_of(&r.mind);
                    if polity.admit_mind(&handle).is_some() {
                        crate::places::claim(&path);
                        done.admitted.push((
                            polity.spec.civilization.sigil.clone(),
                            polity.spec.world_name.clone(),
                        ));
                        admitted.push(Admission {
                            mind: r.mind,
                            world: polity.spec.world_name.clone(),
                            sigil: polity.spec.civilization.sigil.clone(),
                            handle,
                            tiers: Vec::new(),
                            refused: None,
                        });
                        worlds.push(polity);
                        let _ = files::answered(var_root, &r.mind);
                    }
                }
                continue;
            }
            // Coupling: a request riding no body couples to a forming world.
            if worlds_of(u).any(|w| w.coupled == Some(r.mind)) {
                continue;
            }
            let forming: Vec<(Vec<u64>, bool, bool)> = worlds_of(u)
                .map(|w| (w.path.clone(), w.solid, w.coupled.is_some()))
                .collect();
            if let Some(path) = target(&forming, &sites, &r.metabolism, &|p| {
                crate::places::claimed(p)
            }) {
                if let Some(w) = worlds_mut(u).find(|w| w.path == path) {
                    w.coupled = Some(r.mind);
                    done.coupled.push((attach::hex(&r.mind[..4]), path));
                }
            }
        }
    }
    // Detachments: the mind's grants are revoked and its admission removed;
    // its line goes on as one of the universe's own.
    for mind in detaching {
        if let Some(a) = admitted.iter().find(|a| a.mind == mind) {
            if let Some(p) = worlds
                .iter_mut()
                .find(|p| p.spec.civilization.sigil == a.sigil)
            {
                p.release_mind(&a.handle);
            }
            done.released.push(a.sigil.clone());
        }
        admitted.retain(|a| a.mind != mind);
        let _ = files::released(var_root, &mind);
    }
    // Sandboxes: a sub-universe at the line's galaxy core, paid from its
    // surplus (its leisure bank) at the first tube's cost.
    let sandboxes = files::sandboxes(var_root);
    if !sandboxes.is_empty() {
        let cores = crate::ancients::cores_of(worlds, book);
        for (mind, reference) in sandboxes {
            let opened = admitted
                .iter()
                .find(|a| a.mind == mind && a.refused.is_none())
                .and_then(|a| {
                    let at = worlds
                        .iter()
                        .position(|p| p.spec.civilization.sigil == a.sigil && !p.penned)?;
                    let core = cores.get(at).cloned().flatten()?;
                    crate::substrata::open_for(var_root, book, &mut worlds[at], &core, stamp)
                });
            done.sandboxes
                .push((attach::hex(&mind[..4]), opened.is_some()));
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
    fn a_request_couples_only_to_a_forming_world_its_metabolism_fits() {
        let sites: BTreeMap<Vec<u64>, Site> = [
            site(&[0, 0, 0], 5, 1),
            site(&[0, 0, 1], 9, 0),
            site(&[0, 1, 0], 0, 7),
            site(&[0, 1, 1], 20, 0),
        ]
        .into_iter()
        .map(|s| (s.path.clone(), s))
        .collect();
        // (path, solid, coupled): [0,1,1] feeds most but its orbit is solid.
        let worlds = vec![
            (vec![0, 0, 0], false, false),
            (vec![0, 0, 1], false, false),
            (vec![0, 1, 0], false, false),
            (vec![0, 1, 1], true, false),
        ];
        let none = |_: &[u64]| false;
        assert_eq!(
            target(&worlds, &sites, "mineral", &none),
            Some(vec![0, 0, 1]),
            "the forming world feeding most of its people; never a solid one"
        );
        assert_eq!(
            target(&worlds, &sites, "biotic", &none),
            Some(vec![0, 1, 0])
        );
        let claimed = |p: &[u64]| p == [0, 0, 1];
        assert_eq!(
            target(&worlds, &sites, "mineral", &claimed),
            Some(vec![0, 0, 0]),
            "a world a line stands on is not free"
        );
        let coupled = vec![(vec![0, 0, 0], false, true), (vec![0, 0, 1], false, true)];
        assert_eq!(
            target(&coupled, &sites, "mineral", &none),
            None,
            "a world another mind is coupled to is taken"
        );
        assert!(
            target(&worlds, &sites, "plasmic", &none).is_none(),
            "no forming world carries a plasma medium: the request waits"
        );
    }

    fn scratch(name: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "conductor-attachment-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&p);
        p
    }

    #[test]
    fn an_admitted_mind_acts_through_its_world_inbox_as_its_own_entity() {
        let root = scratch("acts");
        let registry = Registry::at(root.join("registry"));
        let book = Ledger::at(root.join("worlds"));
        let spec = crate::polities::founding_polities()
            .into_iter()
            .next()
            .expect("a founding spec");
        let mut polity = Polity::genesis(spec, &registry, &book);
        let handle = attach::handle_of(&[0xab; 32]);
        let id = polity.admit_mind(&handle).expect("admitted");
        assert_eq!(
            polity.admit_mind(&handle),
            Some(id),
            "admission is idempotent across restarts"
        );

        let zone = |acting_as: &str| Directive {
            verb: "zone".into(),
            args: vec!["63".into(), "1".into()],
            acting_as: acting_as.into(),
        };
        assert!(
            !polity.apply_directive(&zone("mind-0000000000000000")),
            "a handle the world does not hold acts as no one"
        );
        assert!(
            polity.apply_directive(&zone(&handle)),
            "the mind zones an extractor by its wire code"
        );
        let last = polity
            .world
            .chronicle()
            .receipts()
            .last()
            .expect("a receipt");
        assert_eq!(
            last.provenance(),
            vec![id],
            "the receipt names the mind's entity"
        );

        polity.release_mind(&handle);
        let raze = Directive {
            verb: "raze".into(),
            args: vec!["63".into()],
            acting_as: handle.clone(),
        };
        assert!(
            !polity.apply_directive(&raze),
            "a released mind's grants are gone"
        );
        let _ = std::fs::remove_dir_all(root);
    }
}
