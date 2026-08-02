//! THE PROPAGATION PINS — resonance as motion (`19_LENS` §5).

use cfe::ratio::{ratio, Ratio};
use lens::propagation::{Medium, Motion, Pace, Resonance};
use rfa_core::capacity::Degree;

fn q(n: i64, d: i64) -> Ratio { ratio(n, d) }

fn read(from: usize, to: usize, steps: u64, m: &Medium) -> Resonance {
    Resonance::across(
        Degree::above_ground(from),
        Degree::above_ground(to),
        steps,
        m,
        q(0, 1),
        q(0, 1),
    )
}

#[test]
fn both_poles_are_named_and_neither_is_the_others_absence() {
    // A rate collapsed to a magnitude says how fast and throws away
    // WHICH WAY. These two have the same size and are opposite events.
    let out = read(2, 9, 3, &Medium::inert());
    let inn = read(9, 2, 3, &Medium::inert());

    // The pole lives in the ARM, not in a sign. Convergence is its own
    // reading — never an expansion that went negative.
    assert_eq!(out.motion, Motion::Diverging { by: q(7, 1) });
    assert_eq!(inn.motion, Motion::Converging { by: q(7, 1) });

    // Same magnitude, opposite poles — a norm would have merged them.
    assert_eq!(out.motion.magnitude(), inn.motion.magnitude());
    assert_ne!(out.motion, inn.motion);
    // And the signed form is a convenience at the edge, never the store.
    assert_eq!(out.motion.signed(), q(7, 1));
    assert_eq!(inn.motion.signed(), q(-7, 1));
}

#[test]
fn a_still_volume_is_not_called_converged() {
    let same = read(4, 4, 10, &Medium::inert());
    assert_eq!(same.motion, Motion::Still);
    assert_eq!(same.pace, Pace::Still);
    assert_eq!(same.motion.magnitude(), q(0, 1));
}

#[test]
fn pace_is_read_against_the_span_not_against_a_cut() {
    // The SAME change is a shock over one step and a drift over a
    // hundred. Pace is relative to the span by construction.
    let shock = read(0, 20, 2, &Medium::inert());
    let drift = read(0, 20, 200, &Medium::inert());
    assert_eq!(shock.pace, Pace::Fast, "20 over 2 steps is fast");
    assert_eq!(drift.pace, Pace::Long, "the same 20 over 200 steps is long");
    // And the underlying motion is identical — only the pace differs.
    assert_eq!(shock.motion, drift.motion);
}

#[test]
fn the_medium_accelerates_or_damps_and_says_which() {
    let fast = Medium::from_surround(q(4, 1), q(1, 1));   // retains 4, sheds 1
    let slow = Medium::from_surround(q(1, 1), q(4, 1));   // bleeding
    assert!(fast.accelerates());
    assert!(slow.decelerates());
    assert!(!Medium::inert().accelerates() && !Medium::inert().decelerates());

    let bare = read(0, 3, 3, &Medium::inert());
    let sped = read(0, 3, 3, &fast);
    let damped = read(0, 3, 3, &slow);

    assert_eq!(bare.modulated, bare.motion.signed(), "an inert medium changes nothing");
    assert!(!bare.medium_mattered());
    assert!(sped.modulated > sped.motion.signed(), "acceleration amplifies the motion");
    assert!(damped.modulated < damped.motion.signed(), "damping reduces it");
    // The bare motion is untouched in every case — the medium's effect
    // is reported BESIDE it, never folded in.
    assert_eq!(sped.motion, bare.motion);
    assert_eq!(damped.motion, bare.motion);
}

#[test]
fn a_medium_that_sheds_nothing_is_inert_never_a_division_by_zero() {
    let m = Medium::from_surround(q(9, 1), q(0, 1));
    assert_eq!(*m.eta(), q(1, 1));
    assert!(!m.accelerates() && !m.decelerates());
}

#[test]
fn degradation_is_carried_beside_the_motion_never_subtracted_from_it() {
    // A reader that wants the net can take it; one that wants both has
    // both. Folding loss into the expansion would destroy the ability
    // to tell a small growth through a lossy medium from a large growth
    // through a clean one.
    let lossy = Resonance::across(
        Degree::above_ground(0),
        Degree::above_ground(5),
        5,
        &Medium::inert(),
        q(3, 1),   // decay consumed
        q(4, 1),   // distance from origin
    );
    assert_eq!(lossy.motion, Motion::Diverging { by: q(5, 1) }, "the motion is untouched");
    assert_eq!(lossy.degradation, q(12, 1), "decay x distance, carried whole");
}

#[test]
fn the_reading_derives_and_holds_nothing() {
    let m = Medium::from_surround(q(3, 1), q(2, 1));
    let first = read(1, 8, 4, &m);
    let _elsewhere = read(9, 2, 40, &m);
    let again = read(1, 8, 4, &m);
    assert_eq!(first, again, "same question, same answer — no history");
}

#[test]
fn convergence_is_a_capability_not_a_deficit() {
    // 05 §1.4 — a dim-star system holds the rarest resource in a
    // high-flux universe, radiator headroom, and SELLS it. The Frost
    // trades convergence. So a converging volume is not a failing
    // volume, and the reading must not encode it as one.
    let inward = read(12, 3, 9, &Medium::inert());

    // It is its own arm, carrying its own positive size.
    match &inward.motion {
        Motion::Converging { by } => assert_eq!(*by, q(9, 1),
            "convergence carries a POSITIVE size in its own arm"),
        other => panic!("expected Converging, got {other:?}"),
    }

    // And it is not reachable by asking for divergence — there is no
    // arm in which convergence is a diverging volume with a minus sign.
    assert_ne!(inward.motion, Motion::Diverging { by: q(9, 1) });

    // A converging volume paces exactly as a diverging one does: the
    // pole does not privilege either side of the reading.
    let outward = read(3, 12, 9, &Medium::inert());
    assert_eq!(inward.pace, outward.pace);
    assert_eq!(inward.motion.magnitude(), outward.motion.magnitude());
}
