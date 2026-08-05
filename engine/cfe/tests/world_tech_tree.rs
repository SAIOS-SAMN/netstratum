//! TECH-1b — THE CRYSTALLIZED TREE (10 §2–§3). No static ladder:
//! nodes CRYSTALLIZE at the year voice when a condition holds over the
//! world's own receipts — sustained pressure or surplus, measured
//! exactly, once per condition per orb. The node id is the seeded
//! synthesis (BLAKE3 of the chronicle root at the moment of
//! crystallization ‖ orb ‖ condition) so replay re-derives the
//! identical node at the identical tick; the modifiers touch only
//! landed heat-law parameters, multiplicatively.

use cfe::dop::{positive, unit, ClosureEvent, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB};

/// A hot world of paired zones (extractors feed habitats feed
/// extractors — self-sustaining) venting its whole surplus, no shell
/// standing. `pairs` scales the factory heat: one pair runs heat_net
/// 1/2 (curable by the doctrine), two pairs run 1 (the doctrine only
/// halves it — the overheat holds).
fn burning(pairs: u16) -> World {
    let mut world = World::new();
    world.found("cinder-reach").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, unit(1, 2), positive(1, 2), None, None).expect("sites hot");
    for pair in 0..pairs {
        world.zone(pair, ZoneKind::Extractor).expect("zones");
        world.zone(8 + pair, ZoneKind::Habitat).expect("zones");
    }
    world
}

fn nodes(world: &World) -> Vec<(u8, String)> {
    world
        .chronicle()
        .receipts()
        .iter()
        .filter_map(|receipt| match &receipt.event {
            ClosureEvent::TechDiscovered { condition, name, .. } if *condition > 0 => {
                Some((*condition, name.clone()))
            }
            _ => None,
        })
        .collect()
}

/// The hotter Furnace (two pairs, heat_net 1): the doctrine
/// crystallizes at the first vented year but only HALVES the fire —
/// the overheat holds through four years and OVERFLOW DISCIPLINE
/// crystallizes beside it. No ladder ordered this: the tree grew from
/// this world's own pressure. Idempotent; replays; resumes (SEAL B).
#[test]
fn held_pressure_crystallizes_discipline_beside_the_doctrine() {
    let mut world = burning(2);
    for tick in 1..=301u32 {
        world.tick();
        if tick % 60 == 0 {
            let _ = world.collect();
        }
    }
    let held = nodes(&world);
    assert!(
        held.iter().any(|(condition, name)| *condition == 4 && name.ends_with(" doctrine")),
        "the vented years crystallize a doctrine first, got {held:?}"
    );
    assert!(
        held.iter().any(|(condition, name)| *condition == 2 && name.ends_with(" discipline")),
        "the overheat the doctrine could not cure crystallizes a discipline, got {held:?}"
    );
    assert!(world.orb().expect("orb").overheat, "half-cured is still burning");

    // Idempotent: more years mint no duplicates.
    for tick in 1..=120u32 {
        world.tick();
        if tick % 60 == 0 {
            let _ = world.collect();
        }
    }
    assert_eq!(
        nodes(&world).iter().filter(|(condition, _)| *condition == 2).count(),
        1,
        "one node per condition per orb, forever"
    );

    // The tree is a replay fact and survives resume.
    assert_eq!(replay(world.chronicle()).techs, world.techs().to_vec());
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.techs(), world.techs());
}

/// The doctrine's modifier is REAL, measured as survival: on the
/// one-pair world (heat_net 1/2) the crystallized radiator (5/4 × 2 =
/// 5/2) closes the budget exactly (2 + 1/2 − 5/2 = 0) — the venting
/// stops and the overheat CLEARS. The same technology that only
/// half-cures the hotter world cures this one whole: crystallized
/// nodes meet each world where it burns.
#[test]
fn the_doctrine_cools_the_curable_world() {
    let mut world = burning(1);
    let mut cleared = false;
    for tick in 1..=70u32 {
        cleared = cleared || !world.tick().overheat_cleared.is_empty();
        if tick % 60 == 0 {
            let _ = world.collect();
        }
    }
    assert!(
        nodes(&world).iter().any(|(condition, _)| *condition == 4),
        "the vented year crystallizes the doctrine"
    );
    assert!(cleared, "the boosted radiator closes the budget; overheat clears");
    assert!(!world.orb().expect("orb").overheat, "the world stopped burning");
}

/// The Frost under famine: a heated colony with no fuel strikes joule
/// famine and survives it eight years — CRYO-RESONANT INSULATION
/// crystallizes, receipted with the table's exact magnitude (3/4).
#[test]
fn survived_cold_crystallizes_insulation() {
    let mut world = World::new();
    world.found("dimlight-hold").expect("founds");
    world.commission_star(StarClass::M).expect("commissions");
    world.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None).expect("sites cold");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    let mut struck = false;
    for _ in 1..=541u32 {
        let outcome = world.tick();
        struck = struck || !outcome.struck.is_empty();
    }
    assert!(struck, "the cold bites — joule famine strikes");
    let insulated = world.chronicle().receipts().iter().any(|receipt| {
        matches!(
            &receipt.event,
            ClosureEvent::TechDiscovered { condition: 1, magnitude, name, .. }
                if *magnitude == ratio(3, 4) && name.ends_with(" insulation")
        )
    });
    assert!(insulated, "eight survived famine-years crystallize an insulation of this line's own");
}

/// **RECEIPT AUTHORITY** (operator ruling 2026-08-05): the state
/// carries what the receipts asserted — entry for entry, the tech
/// list's magnitude IS the `TechDiscovered` magnitude, and it is what
/// the tick's boon factors read. No rule reads the table at tick time;
/// the inline dials are dead. Resume rebuilds the same magnitudes from
/// the receipts alone.
#[test]
fn the_state_carries_what_the_receipts_asserted() {
    let mut world = burning(1);
    for tick in 1..=70u32 {
        world.tick();
        if tick % 60 == 0 {
            let _ = world.collect();
        }
    }
    assert!(!world.techs().is_empty(), "the arc crystallizes");
    for (orb, condition, node, magnitude) in world.techs() {
        let receipted = world
            .chronicle()
            .receipts()
            .iter()
            .find_map(|receipt| match &receipt.event {
                ClosureEvent::TechDiscovered {
                    orb_id, condition: held, node: id, magnitude, ..
                } if orb_id == orb && held == condition && id == node => Some(magnitude.clone()),
                _ => None,
            })
            .expect("every held node has its receipt");
        assert_eq!(
            *magnitude, receipted,
            "condition {condition}: the state must carry the receipt's own magnitude"
        );
    }
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.techs(), world.techs(), "resume rebuilds the receipted magnitudes");
}
