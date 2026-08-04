//! **SPATIAL SITING — (r, θ, ψ), wrapped L1, and the union proved
//! invertible.**
//!
//! `θ` gives an orb a longitude; `ψ` lifts it out of the plane. With
//! both, a place is a point in a VOLUME rather than on a disc, which is
//! what lets `Ω` range over a galaxy instead of one orbital sheet — and
//! it is the triple `arbor::Coordinate` has addressed all along
//! (`s: [Ratio; 3]`) with nothing inhabiting it.

use cfe::dop::{on_slot, separation, slot_angle, slot_of, turn_gap, ClosureEvent, ThermalSite, TURN_SLOTS};
use cfe::ratio::ratio;
use cfe::world::{ActionRefused, World};

fn site(r: (i64, i64), turn: Option<i64>, tilt: Option<i64>) -> ThermalSite {
    ThermalSite {
        albedo: ratio(0, 1),
        orbit_r: ratio(r.0, r.1),
        theta: turn.map(slot_angle),
        psi: tilt.map(slot_angle),
    }
}

// ── THE TURNS · exact, wrapped, short way round ──────────────────────

#[test]
fn a_turn_wraps_the_short_way_and_it_is_exact() {
    // 9/10 and 1/10 are 1/5 apart, not 4/5 — the axis is a CIRCLE. In
    // radians a full turn is 2π and no rational could close it; in
    // turns the seam is exact.
    assert_eq!(turn_gap(&ratio(9, 10), &ratio(1, 10)), ratio(1, 5));
    assert_eq!(turn_gap(&ratio(0, 1), &ratio(1, 2)), ratio(1, 2), "antipodal is the maximum");
    assert_eq!(turn_gap(&ratio(1, 3), &ratio(1, 3)), ratio(0, 1));
}

#[test]
fn every_common_subdivision_lands_on_a_slot_and_that_is_why_sixty() {
    for parts in [1i64, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60] {
        for k in 0..parts {
            assert!(on_slot(&ratio(k, parts)), "{k}/{parts} must land on a slot");
        }
    }
    assert!(!on_slot(&ratio(1, 7)), "sevenths do not divide sixty");
    assert!(!on_slot(&ratio(1, 8)), "eighths do not divide sixty");
}

#[test]
fn the_slot_index_wraps_and_round_trips() {
    assert_eq!(slot_angle(0), slot_angle(TURN_SLOTS), "the ring closes");
    assert_eq!(slot_angle(61), slot_angle(1));
    assert_eq!(slot_angle(-1), slot_angle(59), "and it closes both ways");
    for k in 0..TURN_SLOTS {
        assert_eq!(slot_of(&slot_angle(k)), Some(k));
    }
}

// ── ψ · the volume, and why the disc was not enough ──────────────────

/// **ψ SEPARATES WHAT θ CANNOT.** Two orbs on one ring at one longitude
/// are the SAME PLACE without an inclination and different places with
/// one. This is the test that fails if `ψ` is dropped from the metric.
#[test]
fn psi_separates_orbs_that_theta_alone_would_collide() {
    let ring = (2, 1);
    let equatorial = site(ring, Some(12), Some(0));
    let inclined = site(ring, Some(12), Some(15));

    assert_eq!(
        separation(&equatorial, &inclined),
        Some(ratio(15, 60)),
        "same ring, same longitude, a quarter turn of inclination apart"
    );
    // And without ψ the same pair is one place.
    let flat_a = site(ring, Some(12), None);
    let flat_b = site(ring, Some(12), None);
    assert_eq!(separation(&flat_a, &flat_b), Some(ratio(0, 1)), "a disc collapses them");
}

#[test]
fn separation_adds_three_wrapped_axes_and_squares_nothing() {
    let a = site((3, 1), Some(1), Some(2));
    let b = site((5, 1), Some(4), Some(50));
    // radial 2 + θ gap 3/60 + ψ gap min(48/60, 12/60) = 12/60
    assert_eq!(separation(&a, &b), Some(ratio(2, 1) + ratio(3, 60) + ratio(12, 60)));
}

#[test]
fn an_orb_without_a_longitude_has_no_separation_at_all() {
    let placed = site((1, 1), Some(0), Some(0));
    let unplaced = site((1, 1), None, None);
    assert_eq!(separation(&placed, &unplaced), None, "unplaced, not distant");
}

// ── THE PLACEMENT LAW · nothing collides ─────────────────────────────

#[test]
fn an_orbit_between_the_teeth_refuses_on_either_turn() {
    let mut world = World::new();
    world.found("placed").expect("founds");
    world.commission_star(cfe::thermal::StarClass::G).ok();
    let orb = world.committed().orbs[0].orb_id;

    let bad_theta =
        world.site_planet(orb, ratio(0, 1), ratio(1, 1), Some(ratio(1, 7)), None);
    assert!(matches!(bad_theta, Err(ActionRefused::OffSlot { .. })), "{bad_theta:?}");

    let bad_psi = world.site_planet(
        orb,
        ratio(0, 1),
        ratio(1, 1),
        Some(slot_angle(0)),
        Some(ratio(1, 7)),
    );
    assert!(matches!(bad_psi, Err(ActionRefused::OffSlot { .. })), "{bad_psi:?}");
}

// ── THE WIRE · additive, and legacy untouched ────────────────────────

#[test]
fn a_siting_without_turns_is_byte_identical_to_the_old_frame() {
    let legacy = ClosureEvent::PlanetSited {
        orb_id: 7,
        albedo: ratio(1, 4),
        orbit_r: ratio(3, 2),
        theta: None,
        psi: None,
    };
    let bytes = legacy.wire_bytes();
    assert_eq!(bytes[0], 25, "still tag 25 — 0x41 is RETIRED and no new tag was minted");
    assert_eq!(ClosureEvent::from_wire(25, &bytes[5..]), Some(legacy), "round-trips absent");
}

#[test]
fn both_turns_survive_the_wire_and_lengthen_the_frame() {
    let placed = ClosureEvent::PlanetSited {
        orb_id: 7,
        albedo: ratio(1, 4),
        orbit_r: ratio(3, 2),
        theta: Some(slot_angle(5)),
        psi: Some(slot_angle(50)),
    };
    let bytes = placed.wire_bytes();
    assert_eq!(ClosureEvent::from_wire(25, &bytes[5..]), Some(placed), "both turns decode");

    let bare = ClosureEvent::PlanetSited {
        orb_id: 7,
        albedo: ratio(1, 4),
        orbit_r: ratio(3, 2),
        theta: None,
        psi: None,
    };
    assert!(bytes.len() > bare.wire_bytes().len(), "the trailing turns are real bytes");
}

// ── THE UNION, AND ITS INVERSE · Ω = V ∪ D, proved both ways ─────────


// THE D/V BIJECTION PINS WERE STRUCK 2026-08-04 with the vocabulary
// they tested: `dual_pole` and `Vacuity` went with the capacity law
// (the vacuity removal). The SPATIAL pins above are untouched — they
// were never about the verdict, and this lane's siting work stands.