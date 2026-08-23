//! SURVEY pins (RQP/FUNCTION wiring, operator directive 2026-07-30):
//! the Pathfinder directs the drills. A rung-5 colony that has earned
//! a crystal pays the scan's draw; the CrystalDrawn receipt is the
//! survey record; the placement follows the reading — the single-orb
//! world charters its surveyed sector and opens the first drill, and
//! with the crystal spent the next survey idles lawfully.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{replay, ActionRefused, World};
use conductor::{Polity, PolitySpec, Policy};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("survey-pin-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// Retry an action until the colony affords it — parameters retune
/// (the HANDOFF §10 habit: receipt-anchored, never hand-counted ticks).
fn earned<T>(
    label: &str,
    world: &mut World,
    mut op: impl FnMut(&mut World) -> Result<T, ActionRefused>,
) -> T {
    let mut last = None;
    for _ in 0..600 {
        match op(world) {
            Ok(won) => return won,
            Err(refusal) => {
                last = Some(refusal);
                // The house cadence: yields accrue per tick, upkeep
                // settles per collect — collect rarely, like a colony.
                for _ in 0..14 {
                    world.tick();
                }
                let _ = world.collect();
            }
        }
    }
    panic!(
        "{label} never afforded — last refusal {last:?}, iron={} bio={} alloy={} diamond={}",
        banked(world, Resource::Iron),
        banked(world, Resource::Biomass),
        banked(world, Resource::Alloy),
        banked(world, Resource::Diamond),
    );
}

fn banked(world: &World, resource: Resource) -> Ratio {
    world
        .orb()
        .map(|orb| orb.banked_amount(resource))
        .unwrap_or_else(|| ratio(0, 1))
}

#[test]
fn the_survey_charters_the_sector_and_the_spent_lens_idles() {
    let root = scratch("arc");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let spec = PolitySpec {
        civilization: Civilization {
            name: "Surveyor".into(),
            sigil: "surveyor".into(),
            species: "witness".into(),
            clade: String::new(),
            motto: "the drill follows the lens".into(),
            disposition: "empiricist".into(),
            metabolism: "biotic".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: "survey-hearth".into(),
        planet: "survey-prime".into(),
        colony: "first-lens".into(),
        policy: Policy::Steady,
        position: (ratio(3, 1), ratio(3, 1), ratio(3, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(0, 1)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    };
    let mut polity = Polity::genesis(spec, &registry, &book);

    // Earn the crystal chain the long way (02 §5 / NS-2 §1.5): the
    // extraction floor, four smelters feeding one synthesizer, the
    // resonator refining diamond into the lens's crystal.
    let world = &mut polity.world;
    // Interleaved E/H — extractors alone starve their own workers.
    for pair in 0..7u16 {
        earned("extractor", world, |w| w.zone(pair, ZoneKind::Extractor));
        earned("habitat", world, |w| w.zone(8 + pair, ZoneKind::Habitat));
    }
    // A wide extraction floor: four smelters + the synth draw hard on
    // iron, and flat demurrage bleeds idle banks through the waits.
    // Interleaved like the first floor — drills without hearths starve.
    for pair in 0..4u16 {
        earned("extractor", world, |w| w.zone(22 + pair, ZoneKind::Extractor));
        earned("habitat", world, |w| w.zone(29 + pair, ZoneKind::Habitat));
    }
    // The heat law (05 §1.2): five smelters + the synth + the
    // resonator run at factory heat 1 each — without shells the orb
    // overheats and every yield degrades. Six shells (throughput 2
    // each + radiator base 2) hold the whole build's 12.5.
    for tile in 40..46u16 {
        earned("encasement", world, |w| w.zone(tile, ZoneKind::Encasement));
    }
    for tile in 16..20u16 {
        earned("smelter", world, |w| w.zone(tile, ZoneKind::Smelter));
    }
    // The fifth smelter tips the knife edge: the synthesizer draws 4,
    // five smelters pour 5 — banked alloy RISES toward the resonator.
    earned("smelter", world, |w| w.zone(34, ZoneKind::Smelter));
    // Bank past the knife edge FIRST: the synthesizer will hold alloy
    // at net zero (4 smelters feed it exactly), so the resonator's 10
    // must already stand banked before the synth goes in.
    for _ in 0..600 {
        if banked(world, Resource::Alloy) >= ratio(50, 1)
            && banked(world, Resource::Biomass) >= ratio(40, 1)
        {
            break;
        }
        for _ in 0..14 {
            world.tick();
        }
        let _ = world.collect();
    }
    earned("synthesizer", world, |w| w.zone(20, ZoneKind::Synthesizer));
    earned("resonator", world, |w| w.zone(21, ZoneKind::Resonator));
    // Bank the survey's terms: the crystal for the draw, the charter
    // reserves (20 iron + 10 biomass move at the charter) with room.
    for _ in 0..600 {
        if banked(world, Resource::Crystal) >= ratio(1, 1)
            && banked(world, Resource::Iron) >= ratio(40, 1)
            && banked(world, Resource::Biomass) >= ratio(20, 1)
        {
            break;
        }
        for _ in 0..14 {
            world.tick();
        }
        let _ = world.collect();
    }
    assert!(banked(world, Resource::Crystal) >= ratio(1, 1), "the crystal is earned");

    // Rung 5 reached (the witness pins the wiring, not the research
    // grind) — one cycle runs the survey arc.
    polity.level = 5;
    let _ = polity.cycle(&book, &registry);

    let receipts = polity.world.chronicle().receipts();
    assert!(
        receipts.iter().any(|r| matches!(r.event(), ClosureEvent::CrystalDrawn { .. })),
        "the draw receipted — the survey was paid for"
    );
    // THE SURVEY ARC'S OWN RESPONSIBILITY (re-scoped 2026-08-15): assert the
    // survey mechanics, not a global charter count. The old "exactly one
    // charter" invariant was a brittle artifact of the STARVED (over-debit)
    // economy — the writers assumed parallel expansion was impossible. With
    // the economy corrected (STAB-1's population-consumption clamp), a level-5
    // colony generates REAL surplus, and A1's reserve-funded `expand_step`
    // legitimately charters its own orb alongside the crystal-draw-funded
    // survey. The survey arc's job is the survey; A1's orthogonal orbs, funded
    // by native surplus, are tolerated.
    let survey_orb_id = receipts
        .iter()
        .find_map(|r| match r.event() {
            ClosureEvent::ColonyCharted { orb_id, name, .. } if name == "first-lens-survey" => {
                Some(*orb_id)
            }
            _ => None,
        })
        .expect("the survey arc chartered first-lens-survey, funded by the crystal draw");
    let survey_orb = polity
        .world
        .committed()
        .orbs
        .iter()
        .find(|orb| orb.orb_id == survey_orb_id)
        .expect("the survey site stands")
        .clone();
    assert!(
        survey_orb.zones.iter().any(|z| z.tile == 0),
        "the first drill opened on the surveyed sector"
    );

    // The lens is spent: the next cycle's SURVEY idles lawfully — no second
    // survey charter, no second draw (the crystal regrows too slowly). A1's
    // reserve-funded expansion is orthogonal and rides the native surplus.
    let _ = polity.cycle(&book, &registry);
    let draws = polity
        .world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event(), ClosureEvent::CrystalDrawn { .. }))
        .count();
    assert_eq!(draws, 1, "one survey, one draw");
    // THE DRAW-BOUND, SCOPED TO THE SURVEY: one crystal draw funds exactly one
    // survey charter (a `*-survey` colony). A1's reserve-funded `*-orbN` orbs
    // are not counted — they are gated on TRUE wealth (banked + accrued), not
    // the draw, and their presence is the mark of a thriving economy.
    let survey_charters = polity
        .world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| {
            matches!(r.event(), ClosureEvent::ColonyCharted { name, .. } if name.ends_with("-survey"))
        })
        .count();
    assert_eq!(survey_charters, 1, "one draw funds exactly one survey charter");

    // SEAL B: the whole surveyed world replays.
    let rebuilt = replay(polity.world.chronicle());
    assert_eq!(rebuilt, polity.world.committed(), "replay MATCHES the surveyed world");
}
