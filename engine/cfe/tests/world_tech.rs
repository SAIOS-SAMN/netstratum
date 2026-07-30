//! TECH-1a — THE DISCOVERY CLOSURE AND THE SOLAR LEAP (10 §2.2,
//! 12_UNION §5a). Knowing one's sun is a technology, not a birthright:
//! a world's FIRST thermal opt-in receipts the kernel solar leap
//! (condition 0) — the opt-in IS the leap's receipt, once per world,
//! never per orb. Pre-solar is a lawful state: a world that never
//! sites simply never knows, and nothing in its own life breaks.

use cfe::dop::ClosureEvent;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB};

fn leaps(world: &World) -> usize {
    world
        .chronicle()
        .receipts()
        .iter()
        .filter(|receipt| {
            matches!(
                &receipt.event,
                ClosureEvent::TechDiscovered { condition: 0, .. }
            )
        })
        .count()
}

/// The first siting receipts the leap exactly once; a second orb's
/// siting rides the world's existing knowledge — no second leap.
#[test]
fn the_first_siting_receipts_the_solar_leap_once() {
    let mut world = World::new();
    world.found("heliopolis").expect("founds");
    assert!(!world.knows_its_sun(), "a founded world is pre-solar");
    assert_eq!(leaps(&world), 0);

    world.commission_star(StarClass::G).expect("commissions");
    assert!(
        !world.knows_its_sun(),
        "a commissioned sky is not yet a KNOWN sky — the leap is the opt-in"
    );

    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1)).expect("sites");
    assert!(world.knows_its_sun(), "the opt-in IS the leap's receipt");
    assert_eq!(leaps(&world), 1);

    // A second orb charted and sited: the world already knows its sun.
    let second = world.chart("penumbra", FIRST_ORB).expect("charts");
    world.site_planet(second, ratio(1, 10), ratio(1, 1)).expect("sites");
    assert_eq!(leaps(&world), 1, "the leap is once per world, never per orb");
}

/// Replay carries the leap: the committed projection holds the node,
/// and a resumed world still knows its sun (SEAL B).
#[test]
fn the_leap_replays_and_resumes() {
    let mut world = World::new();
    world.found("heliopolis").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1)).expect("sites");

    let committed = replay(world.chronicle());
    assert_eq!(committed.techs.len(), 1);
    assert_eq!(committed.techs[0].0, FIRST_ORB);
    assert_eq!(committed.techs[0].1, 0, "condition 0 — the kernel leap");

    let resumed = World::resume(world.chronicle().clone());
    assert!(resumed.knows_its_sun(), "a resumed world remembers its sky");
    assert_eq!(resumed.techs(), world.techs());
}


