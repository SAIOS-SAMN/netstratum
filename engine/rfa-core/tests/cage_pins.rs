//! THE CAGE — containment witnesses. What is pinned here is mostly what
//! CANNOT be written; the compile-fail cases are recorded as prose
//! because a test that does not compile cannot be a test.

use cfe::ratio::ratio;
use rfa_core::{coherence_functional, Delta, Lens};

fn incoherent() -> Delta {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![ratio(2, 1)]);
    d.set_antisym(1, 2, vec![ratio(3, 1)]);
    d.set_antisym(0, 2, vec![ratio(7, 1)]);
    d
}

#[test]
fn a_caged_reading_states_what_it_is() {
    // IDENTITY survives: a witness can say exactly what a state measures.
    assert_eq!(coherence_functional(&incoherent()), Lens::of(ratio(12, 1)));
    assert_eq!(coherence_functional(&Delta::zero(3, 1)), Lens::of(ratio(0, 1)));
}

#[test]
fn the_only_exit_is_a_rendering() {
    // The door OUT lands in a String — nothing can numerically branch on
    // it, so the boundary holds without anyone remembering that it must.
    let shown = coherence_functional(&incoherent()).show();
    assert_eq!(shown, "12");
    assert_eq!(format!("{}", Lens::of(ratio(1, 2))), "1/2");
}

#[test]
fn the_cage_has_no_return_door() {
    // THE CONTAINMENT PROPERTY, stated as the API's shape: `Lens` offers
    // `of` (in) and `show`/`Display` (out to a String) and NOTHING else.
    // There is no `into_inner`, no `Deref`, no `as_ratio`. One such
    // method would reopen every path this module closes, because a
    // caller could escape, compare, and gate exactly as before.
    //
    // WHAT DOES NOT COMPILE, and is the real content of this file:
    //
    //     let a = coherence_functional(&x);
    //     let b = coherence_functional(&y);
    //     if a < b { .. }          // no PartialOrd — descent
    //     if a > Lens::of(eps) { } // no PartialOrd — thresholding
    //     [a, b].iter().min()      // no Ord — ranking
    //     let r: Ratio = a.into(); // no conversion — escape
    //
    // Each line above was a real pattern in this engine today. Each is
    // now rejected by the compiler rather than by a docstring.
    let a = coherence_functional(&incoherent());
    let b = a.clone();
    assert_eq!(a, b, "identity is permitted");
    assert!(!a.show().is_empty(), "rendering is permitted");
}

#[test]
fn the_conservation_distance_is_caged_too() {
    // Not only C. Every scalar that claimed to summarise the universe
    // lives behind the same door — the distance-to-surface reading
    // included. The engine asks `first_conservation_break` instead,
    // which names a POSITION and can be acted on.
    let d = incoherent();
    assert_eq!(d.conservation_measurement(), Lens::of(ratio(0, 1)));
    assert_eq!(d.first_conservation_break(), None);
}
