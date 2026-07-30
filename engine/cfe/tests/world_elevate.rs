//! ELEVATE-1 — THE THREE-GATE GAUGE CLIMB (12_UNION §6, Q3 resolved
//! as discrete gauge morphodynamics). Elevation is an earned,
//! receipted ascent in G, executed as `GaugeShifted` and gated on
//! three exact readings of the world's own chronicle: thermodynamic
//! settlement (S), loop coherence within the year voice (T), and the
//! crossing (G). The climb is not a ladder — a refusal names every
//! gate's reading, and each false is escapable. The one monotone
//! thing is G itself: a climb moves strictly upward.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{aperture_transfer, replay, ActionRefused, World, FIRST_ORB};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1)).expect("sites");
    world
}

/// The refusal names every gate: a fresh world has not crossed; after
/// the crossing but with no loop closed this year, coherence bars;
/// after a collect, the climb opens. A non-positive delta refuses
/// typed — the fiber never retreats.
#[test]
fn the_gates_refuse_typed_and_each_is_escapable() {
    let mut world = sunlit("ascendant");
    let mut partner = sunlit("witness");

    // A negative or zero climb is not a climb.
    assert_eq!(world.elevate(ratio(0, 1)), Err(ActionRefused::BadClimb));
    assert_eq!(world.elevate(ratio(-1, 6)), Err(ActionRefused::BadClimb));

    // Not crossed, no loop closed: both gates named in one refusal.
    assert_eq!(
        world.elevate(ratio(1, 6)),
        Err(ActionRefused::ElevationBarred {
            settled: true,
            coherent: false,
            crossed: false,
            aligned: true,
        })
    );

    // Cross the void (THRESHOLD-1) — one gate opens.
    aperture_transfer(
        &mut world, FIRST_ORB, &mut partner, FIRST_ORB,
        Resource::Iron, ratio(1, 1), 1,
    )
    .expect("the crossing");
    assert_eq!(
        world.elevate(ratio(1, 6)),
        Err(ActionRefused::ElevationBarred {
            settled: true,
            coherent: false,
            crossed: true,
            aligned: true,
        })
    );

    // Close a loop within the year voice — the climb opens.
    world.zone(0, ZoneKind::Extractor).expect("zones");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    world.elevate(ratio(1, 6)).expect("the climb opens");
    assert_eq!(world.gauge(), &ratio(1, 6));
}

/// The lawful climb is one GaugeShifted receipt; replay reconstructs
/// the ascent from the chronicle alone (SEAL B), and the climb is
/// monotone — each step rises from the last.
#[test]
fn a_lawful_climb_is_one_receipt_and_replays() {
    let mut world = sunlit("ascendant");
    let mut partner = sunlit("witness");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    aperture_transfer(
        &mut world, FIRST_ORB, &mut partner, FIRST_ORB,
        Resource::Iron, ratio(1, 1), 2,
    )
    .expect("the crossing");

    let shifts_before = shifts(&world);
    world.elevate(ratio(1, 4)).expect("climbs");
    world.elevate(ratio(1, 4)).expect("climbs again");
    assert_eq!(shifts(&world), shifts_before + 2, "each step is ONE receipt");
    assert_eq!(world.gauge(), &ratio(1, 2));
    assert_eq!(replay(world.chronicle()).gauge[0], ratio(1, 2));
}

/// The settlement gate: a famine-struck world is barred — and the bar
/// names settled=false, never a verdict. Relief reopens the climb
/// (stagnation is a region, escapable).
#[test]
fn famine_bars_the_climb_until_relieved() {
    // The D1 arc: 3E+1H nets biomass −1/2/tick — famine on schedule.
    let mut world = sunlit("overreach");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(1, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("banks the opening yield");
    world.zone(2, ZoneKind::Extractor).expect("the third extractor");
    let mut famished = false;
    for _ in 0..20 {
        if !world.tick().struck.is_empty() {
            famished = true;
            break;
        }
    }
    assert!(famished, "the overdriven colony strikes famine");
    match world.elevate(ratio(1, 6)) {
        Err(ActionRefused::ElevationBarred { settled: false, .. }) => {}
        other => panic!("famine must bar the settlement gate, got {other:?}"),
    }
}

fn shifts(world: &World) -> usize {
    world
        .chronicle()
        .receipts()
        .iter()
        .filter(|receipt| matches!(receipt.event, ClosureEvent::GaugeShifted { .. }))
        .count()
}
