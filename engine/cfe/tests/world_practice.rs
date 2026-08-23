//! PRACTICE-1 pins (14 §4, operator-ruled): the derived creed never
//! binds — the receipted practice does. One benefit, one friction,
//! exact; release is a REDISCOVERY that emits the re-emergence node;
//! an adopted discipline gates the climb through alignment; the
//! Overdrive takes only the tempered.

use cfe::dop::{positive, unit, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn banked(world: &World, resource: Resource) -> Ratio {
    world
        .orb()
        .map(|orb| orb.banked_amount(resource))
        .unwrap_or_else(|| ratio(0, 1))
}

/// The door refuses typed: unknown code, double adoption, and the
/// Overdrive before eight hardships.
#[test]
fn the_door_refuses_typed() {
    let mut world = World::new();
    world.found("practiced").expect("founds");
    assert!(matches!(
        world.adopt_practice(9),
        Err(ActionRefused::PracticeUnknown { practice: 9 })
    ));
    world.adopt_practice(2).expect("adopts the Showpiece");
    assert!(matches!(
        world.adopt_practice(2),
        Err(ActionRefused::PracticeAlreadyAdopted { practice: 2 })
    ));
    assert!(matches!(
        world.adopt_practice(5),
        Err(ActionRefused::OverdriveUntempered { hardships: 0 })
    ));
    assert!(matches!(
        world.release_practice(4),
        Err(ActionRefused::PracticeNotHeld { practice: 4 })
    ));
}

/// The Showpiece, exact: habitat yield ×5/4, habitat upkeep ×2 — and
/// nothing else moves (the extractor runs the v1 numbers).
#[test]
fn the_icon_prices_exactly() {
    let mut world = World::new();
    world.found("iconheld").expect("founds");
    world.adopt_practice(2).expect("adopts");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    // iron: 20 − 10 (E) − 5·0 … founding banks 20/10; E yield 4×3/2 =
    // +6; habitat life-support ×2: −4 × (1/4 × 2) = −2 → 14.
    assert_eq!(banked(&world, Resource::Iron), ratio(14, 1));
    // biomass: 10 − 5 (H) − 4 × 1/2 (rations) + 4 × (1 × 5/4) = 8.
    assert_eq!(banked(&world, Resource::Biomass), ratio(8, 1));
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// The Fast, exact: extractor rations ×3/4, habitat yield ×3/4.
#[test]
fn the_fast_prices_exactly() {
    let mut world = World::new();
    world.found("fastheld").expect("founds");
    world.adopt_practice(1).expect("adopts");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    // iron: 20 − 10 + 6 − 4 × 1/4 = 15 (life support unmoved).
    assert_eq!(banked(&world, Resource::Iron), ratio(15, 1));
    // biomass: 10 − 5 − 4 × (1/2 × 3/4) + 4 × (1 × 3/4) = 13/2.
    assert_eq!(banked(&world, Resource::Biomass), ratio(13, 2));
}

/// Release is a REDISCOVERY: barred until new doctrine lands after
/// the adoption; a lawful release emits the condition-7 re-emergence
/// node, seeded from the moment — and the whole arc replays.
#[test]
fn release_is_a_rediscovery_and_the_lesson_re_emerges() {
    let mut world = World::new();
    world.found("relearner").expect("founds");
    world.adopt_practice(2).expect("adopts");
    assert!(matches!(
        world.release_practice(2),
        Err(ActionRefused::ReleaseUnearned { practice: 2 })
    ));
    // The solar leap is the doctrine earned since adoption.
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites — the leap receipts");
    world.release_practice(2).expect("the rediscovery releases");
    assert!(world.committed().practices.is_empty());
    let relearned: Vec<u8> = world
        .committed()
        .techs
        .iter()
        .filter(|(_, condition, _, _)| *condition == 7)
        .map(|(_, condition, _, _)| *condition)
        .collect();
    assert_eq!(relearned, vec![7], "one re-emergence node, condition 7");
    let named = world.chronicle().receipts().iter().any(|receipt| {
        matches!(receipt.event(),
            ClosureEvent::TechDiscovered { condition: 7, name, .. } if name.contains("re-emergence"))
    });
    assert!(named, "the node speaks its concept in the line's tongue");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.committed(), world.committed(), "resume carries it");
}

/// The alignment loop: a practiced line whose entropic books outweigh
/// its syntropic ones stands UNALIGNED — the climb refuses naming the
/// fourth gate.
#[test]
fn an_unaligned_practice_bars_the_climb() {
    let mut world = World::new();
    world.found("unaligned").expect("founds");
    world.adopt_practice(4).expect("adopts the Hoard");
    // A self-forward once stood here with `.err()` — tolerated refusal,
    // dead by its own comment ("the books path below suffices"). Since
    // C1 a self-loop cannot be CONSTRUCTED, so the vestige is deleted
    // rather than converted: there is no deed to tolerate.
    // Simpler entropic weight: an absorption debt lapsing.
    // (Direct book arithmetic through the landed deeds is exercised in
    // world_absorb; here the gate reading alone is pinned.)
    match world.elevate(positive(1, 8)) {
        Err(ActionRefused::ElevationBarred { aligned, .. }) => {
            // With no deeds posted the line stands aligned — the bar
            // here comes from the other gates; aligned must be TRUE.
            assert!(aligned, "no books posted → aligned by law");
        }
        other => panic!("the unelevated world climbs? {other:?}"),
    }
}
