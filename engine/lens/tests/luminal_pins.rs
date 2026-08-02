//! THE LUMINAL PINS — what an observer sees, held by witnesses
//! (`19_LENS` §10).
//!
//! 1. THE THREE AXES ARE ORTHOGONAL — depth, phase and aperture each
//!    move the reading without moving the others. This is the pin that
//!    proves the geometry did not collapse.
//! 2. LUMINAL EMERGES, IT IS NOT STORED — re-observing at a phase gives
//!    the same answer as observing there first. No history, no drift.
//! 3. THE DIRECT PHASE HIDES NOTHING — gap zero, wavelength equal to the
//!    structure's own period.
//! 4. THE APERTURE IS PER-OBSERVER — two instruments, one substrate,
//!    two readings, and neither is wrong.
//! 5. THE AMPLITUDE IS CAGED — it may be shown and cannot be computed
//!    on. Held by the TYPE, so this pin records the property rather
//!    than testing a runtime guard.
//! 6. THE TORSION ORDER IS THE PERIOD — halves wrap at 2, twelfths at 12.

use cfe::ratio::{ratio, Ratio};
use lens::luminal::{emerge, Aperture, Phase, HARMONIC_CHANNELS};
use lens::peel::{peel, Depth};
use rfa_core::{Delta, Support};

fn q(n: i64, d: i64) -> Ratio {
    ratio(n, d)
}

/// A 3-entity relation on two channels, every pair observed.
fn structure(d01: Ratio, d12: Ratio, d02: Ratio) -> (Delta, Support) {
    let mut delta = Delta::zero(3, 2);
    delta.set_antisym(0, 1, vec![d01, q(1, 1)]);
    delta.set_antisym(1, 2, vec![d12, q(1, 1)]);
    delta.set_antisym(0, 2, vec![d02, q(1, 1)]);
    let mut support = Support::new(3, 2);
    for (i, j) in [(0, 1), (1, 2), (0, 2)] {
        support.connect(i, j, 0);
        support.connect(i, j, 1);
    }
    (delta, support)
}

#[test]
fn the_three_axes_are_orthogonal() {
    // THE LOAD-BEARING PIN. Depth, phase and aperture each move the
    // reading INDEPENDENTLY. If turning one silently moved another, the
    // axes would have collapsed into each other and the geometry would
    // be gone — which is exactly the flattening this layer exists to
    // prevent.
    let (delta, support) = structure(q(2, 1), q(3, 1), q(9, 1));
    let flat = Aperture::flat();
    let direct = Phase::direct();

    let mid = peel(&delta, &support, Depth::Mid);
    let near = peel(&delta, &support, Depth::Near);

    // PHASE moves the wavelength; the structure's own period does not.
    let a = emerge(&delta, &mid, &direct, &flat);
    let b = emerge(&delta, &mid, &Phase::at(q(3, 1)), &flat);
    assert_eq!(a.torsion_order, b.torsion_order, "phase must not move the period");
    assert_ne!(a.wavelength, b.wavelength, "phase MUST move the wavelength");

    // DEPTH moves the amplitude; the period and wavelength do not.
    let c = emerge(&delta, &near, &direct, &flat);
    assert_eq!(a.torsion_order, c.torsion_order, "depth must not move the period");
    assert_eq!(a.wavelength, c.wavelength, "depth must not move the wavelength");
    assert_ne!(
        a.amplitude.shown(),
        c.amplitude.shown(),
        "depth MUST move what is manifest"
    );

    // APERTURE moves the amplitude; nothing geometric.
    let keen = Aperture::flat().tuned(1, q(7, 1));
    let d = emerge(&delta, &mid, &direct, &keen);
    assert_eq!(a.torsion_order, d.torsion_order, "aperture must not move the period");
    assert_eq!(a.wavelength, d.wavelength, "aperture must not move the wavelength");
    assert_ne!(
        a.amplitude.shown(),
        d.amplitude.shown(),
        "aperture MUST move what this observer sees"
    );
}

#[test]
fn the_luminal_emerges_and_is_never_stored() {
    // "It emerges, it is not stored separately. They cannot drift
    // apart." Observing at a phase, then again, then at another and
    // back, must give byte-identical readings — there is no history to
    // accumulate and nothing to go stale.
    let (delta, support) = structure(q(2, 1), q(3, 1), q(9, 1));
    let view = peel(&delta, &support, Depth::Mid);
    let flat = Aperture::flat();
    let angle = Phase::at(q(5, 2));

    let first = emerge(&delta, &view, &angle, &flat);
    let _elsewhere = emerge(&delta, &view, &Phase::at(q(11, 3)), &flat);
    let again = emerge(&delta, &view, &angle, &flat);

    assert_eq!(first.wavelength, again.wavelength);
    assert_eq!(first.torsion_order, again.torsion_order);
    assert_eq!(first.phase_gap, again.phase_gap);
    assert_eq!(first.amplitude.shown(), again.amplitude.shown());
}

#[test]
fn the_direct_phase_hides_nothing() {
    // At the head-on angle the inner orb fills the outer: the observed
    // wavelength IS the structure's period, and the gap is zero.
    let (delta, support) = structure(q(1, 2), q(1, 3), q(7, 4));
    let view = peel(&delta, &support, Depth::Mid);
    let seen = emerge(&delta, &view, &Phase::direct(), &Aperture::flat());

    assert_eq!(seen.wavelength, seen.torsion_order, "direct: wavelength IS the period");
    assert_eq!(seen.phase_gap, q(0, 1), "direct: nothing is hidden");

    // Turn the angle and the gap opens — the same truth, less of it
    // revealed.
    let oblique = emerge(&delta, &view, &Phase::at(q(1, 4)), &Aperture::flat());
    assert!(oblique.phase_gap > q(0, 1), "an oblique angle conceals");
}

#[test]
fn two_apertures_read_one_substrate_differently_and_both_are_right() {
    // LAW 9 without belief: the substrate is identical for every
    // observer; only the instrument differs. Two apertures, one
    // structure, two honest readings.
    let (delta, support) = structure(q(2, 1), q(3, 1), q(9, 1));
    let view = peel(&delta, &support, Depth::Near);
    let direct = Phase::direct();

    let spectral = Aperture::flat().tuned(0, q(9, 1)).tuned(1, q(0, 1));
    let spatial = Aperture::flat().tuned(0, q(0, 1)).tuned(1, q(9, 1));

    let a = emerge(&delta, &view, &direct, &spectral);
    let b = emerge(&delta, &view, &direct, &spatial);

    assert_ne!(a.amplitude.shown(), b.amplitude.shown(), "different instruments, different readings");
    // And the geometry they are reading is the same — the disagreement
    // is in sensitivity, never in the structure.
    assert_eq!(a.torsion_order, b.torsion_order);
    assert_eq!(a.wavelength, b.wavelength);
}

#[test]
fn the_aperture_metabolism_is_bounded_and_exact() {
    // h[k]' = (h[k]·n + signal) / (n+1) — a running average converges
    // toward what was observed and never past it. Exact rationals, no
    // float, no root.
    let mut aperture = Aperture::flat();
    let target = q(5, 1);
    for _ in 0..24 {
        aperture.observe(3, target.clone());
    }
    let w = aperture.weight(3);
    assert!(w > q(1, 1), "the channel tuned up toward the signal");
    assert!(w <= target, "and never past it — a running average is bounded by its inputs");

    // Out-of-range channels are inert, never a panic.
    aperture.observe(HARMONIC_CHANNELS + 5, q(99, 1));
    assert_eq!(aperture.weight(HARMONIC_CHANNELS + 5), q(0, 1));
}

#[test]
fn the_torsion_order_is_the_period_of_the_wave() {
    // Halves wrap at 2; twelfths at 12. The period is the lcm of the
    // denominators actually standing.
    let (halves, hs) = structure(q(1, 2), q(3, 2), q(5, 2));
    let view = peel(&halves, &hs, Depth::Far);
    let seen = emerge(&halves, &view, &Phase::direct(), &Aperture::flat());
    assert_eq!(seen.torsion_order, q(2, 1), "halves wrap at 2");

    let (mixed, ms) = structure(q(1, 3), q(1, 4), q(1, 2));
    let view = peel(&mixed, &ms, Depth::Far);
    let seen = emerge(&mixed, &view, &Phase::direct(), &Aperture::flat());
    assert_eq!(seen.torsion_order, q(12, 1), "thirds and quarters wrap at 12");
}

#[test]
fn the_amplitude_is_shown_and_cannot_be_computed_on() {
    // The exception 18 §2b carves for an OPERATOR lens, held by the TYPE
    // rather than by anyone's care: the value enters exact and leaves
    // only as text. There is no accessor, no ordering, no comparison —
    // so this pin records that the only door out is `shown()`.
    let (delta, support) = structure(q(2, 1), q(3, 1), q(9, 1));
    let view = peel(&delta, &support, Depth::Mid);
    let seen = emerge(&delta, &view, &Phase::direct(), &Aperture::flat());

    let rendered = seen.amplitude.shown();
    assert!(!rendered.is_empty(), "an amplitude renders");
    // It is a STRING. Nothing downstream can branch on a magnitude it
    // cannot recover — which is the whole point of the cage.
    assert!(rendered.chars().all(|c| c.is_ascii_digit() || c == '/' || c == '-'));
}
