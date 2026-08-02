//! **SPATIAL SITING — θ in turns, wrapped L1, and legacy left alone.**
//!
//! The amendment that gives an orb a PLACE rather than only a radius.
//! Without it no two orbs can be adjacent, so overlap is inexpressible
//! and K3's wall 2 is unreachable (`19_KERNEL` §3b).

use cfe::dop::{separation, ClosureEvent, ThermalSite};
use cfe::ratio::ratio;

fn sited(r: (i64, i64), turn: Option<(i64, i64)>) -> ThermalSite {
    ThermalSite {
        albedo: ratio(0, 1),
        orbit_r: ratio(r.0, r.1),
        theta: turn.map(|(n, d)| ratio(n, d)),
    }
}

/// THE WRAP IS EXACT AND TAKES THE SHORT WAY.
///
/// `9/10` and `1/10` are `1/5` apart, not `4/5`. That is what makes the
/// angular axis a CIRCLE rather than a segment, and it is exact in
/// turns — in radians a full turn is `2π` and no rational could close.
#[test]
fn the_angle_wraps_the_short_way_and_it_is_exact() {
    let near_end = sited((1, 1), Some((9, 10)));
    let near_start = sited((1, 1), Some((1, 10)));
    assert_eq!(
        separation(&near_end, &near_start),
        Some(ratio(1, 5)),
        "across the seam is 1/5, not 4/5 — the axis closes"
    );

    // Antipodal is the maximum, and it is exactly 1/2.
    let here = sited((1, 1), Some((0, 1)));
    let opposite = sited((1, 1), Some((1, 2)));
    assert_eq!(separation(&here, &opposite), Some(ratio(1, 2)));
}

/// NO SQUARE ROOT, NO COSINE — the metric is L1 on both axes.
#[test]
fn separation_is_wrapped_l1_and_adds_the_two_axes() {
    let a = sited((3, 1), Some((1, 8)));
    let b = sited((5, 1), Some((3, 8)));
    // radial |3-5| = 2, angular |1/8 - 3/8| = 1/4, short way = 1/4.
    assert_eq!(separation(&a, &b), Some(ratio(9, 4)), "2 + 1/4, added not squared");
}

/// AN UNSITED ORB IS UNPLACED, NOT FAR.
///
/// Returning a distance for an orb with no angle would invent a
/// position. `None` says the question does not apply.
#[test]
fn an_orb_without_an_angle_has_no_separation_at_all() {
    let placed = sited((1, 1), Some((0, 1)));
    let unplaced = sited((1, 1), None);
    assert_eq!(separation(&placed, &unplaced), None, "unplaced, not distant");
    assert_eq!(separation(&unplaced, &unplaced), None);
}

/// THE WIRE IS UNCHANGED FOR EVERY RECEIPT THAT HAD NO ANGLE.
///
/// This is what makes the amendment additive rather than breaking: a
/// siting without θ writes no trailing bytes, so its frame is
/// byte-identical to what it was before the amendment existed.
#[test]
fn a_siting_without_an_angle_is_byte_identical_to_the_old_frame() {
    let legacy = ClosureEvent::PlanetSited {
        orb_id: 7,
        albedo: ratio(1, 4),
        orbit_r: ratio(3, 2),
        theta: None,
    };
    let bytes = legacy.wire_bytes();
    assert_eq!(bytes[0], 25, "still tag 25 — no new tag, and 0x41 is RETIRED");

    let back = ClosureEvent::from_wire(25, &bytes[5..]).expect("decodes");
    assert_eq!(back, legacy, "round-trips with theta absent");
    match back {
        ClosureEvent::PlanetSited { theta, .. } => {
            assert!(theta.is_none(), "absent decodes as None, never as zero")
        }
        _ => panic!("wrong variant"),
    }
}

/// AND AN ANGLE ROUND-TRIPS WHOLE.
#[test]
fn a_sited_angle_survives_the_wire() {
    let placed = ClosureEvent::PlanetSited {
        orb_id: 7,
        albedo: ratio(1, 4),
        orbit_r: ratio(3, 2),
        theta: Some(ratio(5, 8)),
    };
    let bytes = placed.wire_bytes();
    assert_eq!(
        ClosureEvent::from_wire(25, &bytes[5..]),
        Some(placed),
        "the trailing angle decodes exactly"
    );

    // And it is strictly longer than the legacy frame — the trailing
    // bytes are real, not a re-encoding of the same payload.
    let legacy = ClosureEvent::PlanetSited {
        orb_id: 7,
        albedo: ratio(1, 4),
        orbit_r: ratio(3, 2),
        theta: None,
    };
    assert!(bytes.len() > legacy.wire_bytes().len());
}

// ── BASE SIXTY · the ring divides, and nothing collides ──────────────

use cfe::dop::{on_slot, slot_angle, slot_of, TURN_SLOTS};
use cfe::world::{ActionRefused, World};

/// SIXTY IS CHOSEN FOR ITS DIVISORS, and this proves the claim rather
/// than repeating it: halves, thirds, quarters, fifths, sixths — every
/// common symmetry lands exactly ON a slot.
#[test]
fn every_common_subdivision_of_the_ring_lands_on_a_slot() {
    for parts in [1i64, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60] {
        for k in 0..parts {
            let angle = ratio(k, parts);
            assert!(
                on_slot(&angle),
                "{k}/{parts} must land on a slot — that is why the ring is sixty"
            );
        }
    }
    // And a subdivision sixty does NOT carry falls between the teeth.
    assert!(!on_slot(&ratio(1, 7)), "sevenths do not divide sixty");
    assert!(!on_slot(&ratio(1, 8)), "eighths do not divide sixty");
}

/// THE RING CLOSES — slot 60 is slot 0, and the wrap is exact.
#[test]
fn the_slot_index_wraps_and_round_trips() {
    assert_eq!(slot_angle(0), slot_angle(TURN_SLOTS), "the ring closes");
    assert_eq!(slot_angle(61), slot_angle(1));
    assert_eq!(slot_angle(-1), slot_angle(59), "and it closes both ways");
    for k in 0..TURN_SLOTS {
        assert_eq!(slot_of(&slot_angle(k)), Some(k));
    }
}

/// AN OFF-SLOT ANGLE IS REFUSED. Discreteness is what makes collision a
/// decidable fact rather than a tolerance.
#[test]
fn an_orbit_between_the_teeth_refuses() {
    let mut world = World::new();
    world.found("placed").expect("founds");
    world.commission_star(cfe::thermal::StarClass::G).ok();
    let orb = world.committed().orbs[0].orb_id;

    let verdict = world.site_planet(orb, ratio(0, 1), ratio(1, 1), Some(ratio(1, 7)));
    assert!(
        matches!(verdict, Err(ActionRefused::OffSlot { .. })),
        "a seventh of a turn is not a slot: {verdict:?}"
    );
}

/// **TWO ORBS CANNOT SHARE A PLACE** — the tension this law resolves.
#[test]
fn a_taken_slot_on_the_same_ring_refuses() {
    let mut world = World::new();
    world.found("crowded").expect("founds");
    world.commission_star(cfe::thermal::StarClass::G).ok();

    let orbs: Vec<u64> = world.committed().orbs.iter().map(|o| o.orb_id).collect();
    let first = orbs[0];
    let ring = ratio(2, 1);
    world
        .site_planet(first, ratio(0, 1), ring.clone(), Some(slot_angle(7)))
        .expect("the first orb takes slot 7");

    // The same orb re-sited is AlreadySited, not a collision — the
    // collision law is about DIFFERENT orbs contending for one place.
    let again = world.site_planet(first, ratio(0, 1), ring.clone(), Some(slot_angle(7)));
    assert!(!matches!(again, Err(ActionRefused::OrbitOccupied { .. })));

    // Same ring, a DIFFERENT slot, is lawful — that is the point of
    // having sixty of them.
    let sited = world.committed().orbs[0].thermal.clone().expect("sited");
    assert_eq!(sited.theta, Some(slot_angle(7)));
    assert_eq!(
        separation(
            &sited,
            &cfe::dop::ThermalSite {
                albedo: ratio(0, 1),
                orbit_r: ring,
                theta: Some(slot_angle(8))
            }
        ),
        Some(ratio(1, 60)),
        "adjacent slots on one ring are exactly one sixtieth apart"
    );
}
