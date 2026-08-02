//! THE PROPAGATION PINS — resonance as motion (`19_LENS` §5).

use cfe::ratio::{ratio, Ratio};
use lens::propagation::{Direction, Medium, Pace, Resonance};
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
fn direction_survives_because_it_is_the_content() {
    // A rate collapsed to a magnitude says how fast and throws away
    // whether the volume was growing or dying. Both readings below have
    // the SAME magnitude and are opposite events.
    let out = read(2, 9, 3, &Medium::inert());
    let inn = read(9, 2, 3, &Medium::inert());

    assert_eq!(out.direction, Direction::Outward);
    assert_eq!(inn.direction, Direction::Inward);
    assert_eq!(out.expansion, q(7, 1));
    assert_eq!(inn.expansion, q(-7, 1));
    assert_eq!(out.expansion.clone() * q(-1, 1), inn.expansion,
        "same magnitude, opposite events — a norm would have merged them");
}

#[test]
fn a_still_volume_is_not_called_converged() {
    let same = read(4, 4, 10, &Medium::inert());
    assert_eq!(same.direction, Direction::Still);
    assert_eq!(same.pace, Pace::Still);
    assert_eq!(same.expansion, q(0, 1));
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
    assert_eq!(shock.expansion, drift.expansion);
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

    assert_eq!(bare.modulated, bare.expansion, "an inert medium changes nothing");
    assert!(!bare.medium_mattered());
    assert!(sped.modulated > sped.expansion, "acceleration amplifies the motion");
    assert!(damped.modulated < damped.expansion, "damping reduces it");
    // The bare expansion is untouched in every case — the medium's
    // effect is reported BESIDE the motion, never folded into it.
    assert_eq!(sped.expansion, bare.expansion);
    assert_eq!(damped.expansion, bare.expansion);
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
    assert_eq!(lossy.expansion, q(5, 1), "the motion is untouched");
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
