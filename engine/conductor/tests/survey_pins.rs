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
use conductor::{Egregore, EgregoreSpec, Policy};
use mirror::registry::{Civilization, Registry};
use mirror::Mirror;
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
    let mirror = Mirror::at(root.join("worlds"));
    let spec = EgregoreSpec {
        civilization: Civilization {
            name: "Surveyor".into(),
            slug: "surveyor".into(),
            species: "witness".into(),
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
    let mut egregore = Egregore::genesis(spec, &registry, &mirror);

    // Earn the crystal chain the long way (02 §5 / NS-2 §1.5): the
    // extraction floor, four smelters feeding one synthesizer, the
    // resonator refining diamond into the lens's crystal.
    let world = &mut egregore.world;
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
    egregore.level = 5;
    let _ = egregore.cycle(&mirror, &registry);

    let receipts = egregore.world.chronicle().receipts();
    assert!(
        receipts.iter().any(|r| matches!(r.event, ClosureEvent::CrystalDrawn { .. })),
        "the draw receipted — the survey was paid for"
    );
    let chartered: Vec<String> = receipts
        .iter()
        .filter_map(|r| match &r.event {
            ClosureEvent::ColonyCharted { name, .. } => Some(name.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(chartered, vec!["first-lens-survey".to_string()], "the sector chartered once");
    let survey_orb = egregore
        .world
        .committed()
        .orbs
        .iter()
        .find(|orb| orb.orb_id != cfe::world::FIRST_ORB)
        .expect("the survey site stands")
        .clone();
    assert!(
        survey_orb.zones.iter().any(|z| z.tile == 0),
        "the first drill opened on the surveyed sector"
    );

    // The lens is spent: the next cycle's survey idles lawfully — no
    // second charter, no second draw (1/8 a tick regrows too slowly).
    let _ = egregore.cycle(&mirror, &registry);
    let draws = egregore
        .world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::CrystalDrawn { .. }))
        .count();
    assert_eq!(draws, 1, "one survey, one draw");
    let charters = egregore
        .world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::ColonyCharted { .. }))
        .count();
    assert_eq!(charters, 1, "the expansion is bounded by the draw");

    // SEAL B: the whole surveyed world replays.
    let rebuilt = replay(egregore.world.chronicle());
    assert_eq!(rebuilt, egregore.world.committed(), "replay MATCHES the surveyed world");
}
