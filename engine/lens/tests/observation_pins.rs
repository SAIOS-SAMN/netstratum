//! THE OBSERVATION PINS — reading is not uni-directional (`19_LENS` §15).

use cfe::ratio::{ratio, Ratio};
use lens::luminal::{Aperture, Phase};
use lens::observation::Observation;
use lens::observer::Capacity;
use lens::peel::Depth;
use lens::propagation::{Medium, Motion};
use rfa_core::{Delta, Support};

fn q(n: i64, d: i64) -> Ratio { ratio(n, d) }

fn built(a: Ratio, b: Ratio, c: Ratio) -> (Delta, Support) {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![a]);
    d.set_antisym(1, 2, vec![b]);
    d.set_antisym(0, 2, vec![c]);
    let mut s = Support::new(3, 1);
    for (i, j) in [(0, 1), (1, 2), (0, 2)] { s.connect(i, j, 0); }
    (d, s)
}

#[test]
fn every_arm_is_reachable_from_one_position_and_none_is_the_entrance() {
    // THE LOAD-BEARING PIN. A journal is read front to back; a position
    // is read from any side. From ONE Observation every arm answers,
    // and no arm had to be asked first.
    let (d, s) = built(q(2, 1), q(3, 1), q(9, 1));
    let here = Observation::at(&d, &s).depth(Depth::Mid);

    // Entered in an arbitrary order — each derives independently.
    // (The STANDING arm was struck with the four-row classification,
    // operator ruling 2026-08-04; the circulation it reported is read
    // off the degree arm, which is where it always came from.)
    let flux = here.flux();
    let luminal = here.luminal();
    let view = here.view();
    let degree = here.degree();

    assert!(degree.above() > 0, "structure stands above the ground");
    assert_eq!(
        degree.above(),
        view.materialised(),
        "the arms agree: what the peel materialises is what stands above the floor"
    );
    assert_eq!(view.depth(), Depth::Mid);
    assert!(!luminal.amplitude.shown().is_empty());
    assert_eq!(flux.sources + flux.sinks + flux.balanced, 3, "every vertex censused");

    // And asking in the REVERSE order gives identical answers — there
    // is no order-dependence, because there is no accumulated state.
    let again = Observation::at(&d, &s).depth(Depth::Mid);
    assert_eq!(again.degree(), degree);
    assert_eq!(again.view(), view);
    assert_eq!(again.flux(), flux);
}

#[test]
fn a_position_holds_no_reading_so_it_cannot_go_stale() {
    // The type carries borrowed inputs and four scalars. There is
    // nowhere in it to put a cached reading, which is what makes §1a
    // and §4 structural here rather than promised.
    let (d, s) = built(q(2, 1), q(3, 1), q(9, 1));
    let here = Observation::at(&d, &s).depth(Depth::Near);

    let first = here.view();
    let _elsewhere = here.flux();
    let _more = here.luminal();
    let again = here.view();
    assert_eq!(first, again, "no arm disturbs another");
}

#[test]
fn moving_the_position_moves_the_readings_that_depend_on_it() {
    let (d, s) = built(q(1, 2), q(1, 3), q(7, 4));
    let base = Observation::at(&d, &s).depth(Depth::Mid);

    // PHASE moves the luminal and nothing structural.
    let turned = base.clone().phase(Phase::at(q(3, 1)));
    assert_ne!(base.luminal().wavelength, turned.luminal().wavelength);
    assert_eq!(base.flux(), turned.flux(), "the structure is not an angle");
    assert_eq!(base.degree(), turned.degree());

    // APERTURE moves what is manifest and nothing geometric.
    let keen = base.clone().through(Aperture::flat().tuned(0, q(9, 1)));
    assert_ne!(base.luminal().amplitude.shown(), keen.luminal().amplitude.shown());
    assert_eq!(base.luminal().torsion_order, keen.luminal().torsion_order);

    // DEPTH moves what is materialised; standing is depth-free.
    // (Compared against NEAR: this structure carries exactly one charge,
    // so Mid and Far both materialise 1 — a real coincidence, not a
    // failure, and worth not asserting against.)
    let far = base.clone().depth(Depth::Far);
    let near = base.clone().depth(Depth::Near);
    assert!(near.view().materialised() > far.view().materialised(),
        "the near view carries its own size: {} vs {}",
        near.view().materialised(), far.view().materialised());
    assert_eq!(base.flux(), far.flux(), "the structure is not a distance");
    assert_eq!(near.flux(), far.flux());
}

#[test]
fn held_capacity_bounds_the_position_it_does_not_refuse_it() {
    let (d, s) = built(q(2, 1), q(3, 1), q(9, 1));
    let shallow = Observation::at(&d, &s)
        .depth(Depth::Near)
        .holding(Capacity::GROUND);

    assert_eq!(shallow.granted(), Depth::Far, "answered at what is held");
    assert_eq!(shallow.view().materialised(), 1, "and only that was built");
    // The other arms still answer — a shallow observer is not a blind
    // one, it simply resolves less.
    assert!(shallow.degree().get() >= 1);
    assert_eq!(shallow.degree().above(), 1, "one charge stands above the floor");
}

#[test]
fn propagation_reads_across_two_positions_and_holds_neither() {
    // The caller holds both committed points; the reading holds none.
    let (before, s1) = built(q(1, 1), q(1, 2), q(3, 2));   // composes — on the ground
    let (after, s2) = built(q(2, 1), q(3, 1), q(9, 1));    // circulates
    let prior = Observation::at(&before, &s1);
    let now = Observation::at(&after, &s2);

    let moved = now.propagation_from(&prior, 4, &Medium::inert(), q(0, 1), q(0, 1));
    assert_eq!(moved.motion, Motion::Diverging { by: q(1, 1) },
        "the volume rose off the ground — divergence, named");

    // And the inverse reading names convergence in its own arm.
    let back = prior.propagation_from(&now, 4, &Medium::inert(), q(0, 1), q(0, 1));
    assert_eq!(back.motion, Motion::Converging { by: q(1, 1) });
}
