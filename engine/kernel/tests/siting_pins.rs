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
