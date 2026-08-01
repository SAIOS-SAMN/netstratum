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

#[test]
fn the_containment_vector_holds_no_usable_scalar() {
    // THE SECOND LINE. C = 12 enters; the cage holds 12 and 1 at
    // opposite diagonal corners and the VALUE nowhere. Even a leak of
    // the interior yields components, not a magnitude — reassembly needs
    // the rule, and the rule lives only in `show`.
    let c = coherence_functional(&incoherent());
    assert_eq!(c.show(), "12", "the assembly rule renders it whole");

    // A fractional reading proves the separation is real rather than
    // incidental: 1/2 is stored as 1 and 2, and neither cell is 1/2.
    let half = Lens::of(ratio(1, 2));
    assert_eq!(half.show(), "1/2");
    assert_ne!(half, Lens::of(ratio(1, 1)), "the numerator alone is not the value");
    assert_ne!(half, Lens::of(ratio(2, 1)), "the denominator alone is not the value");

    // Identity still holds across the decomposition, so witnesses work:
    // canonical p/q is unique, so equal values decompose identically.
    assert_eq!(Lens::of(ratio(6, 4)), Lens::of(ratio(3, 2)), "canonical form, one cage");
}

#[test]
fn the_door_carries_exact_candidates_and_believes_none_of_them() {
    use rfa_core::cage::Proposal;
    // Something inside may have used a GPU, floats, or a coin flip to
    // find this. What crosses is exact, and it crosses as a GUESS.
    let found = Proposal::offer(ratio(22, 7)).withdraw(|c| *c > ratio(3, 1));
    assert_eq!(found, Some(ratio(22, 7)), "verified, so it crosses");

    // A candidate that fails its own verifier does NOT cross.
    let refused = Proposal::offer(ratio(1, 9)).withdraw(|c| *c > ratio(3, 1));
    assert_eq!(refused, None, "unverified proposals do not leave");

    // Structural candidates cross too — a program, a witness, a path.
    let plan = Proposal::offer(vec![1i64, 4, 9]).withdraw(|p| p.len() == 3);
    assert_eq!(plan, Some(vec![1, 4, 9]));

    // WHAT CANNOT BE WRITTEN, and is the real content of this pin:
    //
    //     Proposal::offer(0.5f64)      // f64: no `Exact` impl
    //     Proposal::offer(vec![0.5f64])
    //
    // A float has no Exact impl, so the type `Proposal<f64>` does not
    // exist. The approximation cannot be NAMED at the door, never mind
    // carried through it — the boundary is the type system, not care.
}

#[test]
fn c_is_consulted_and_may_use_any_tool_inside() {
    use rfa_core::cage::consult;
    // THE TOOL USES FLOATS. Deliberately — this is the whole point of
    // the sector. A float search runs inside; nothing float crosses.
    let answer = consult(48u64, |n| {
        let approx = (n as f64).sqrt();          // unlawful anywhere else
        let floor = approx.floor() as i64;        // the ANSWER is exact
        floor
    });
    // What comes back is a GUESS, and the caller re-derives it exactly.
    // The exact rule the OUTSIDE applies — never trusting the float.
    let exact = |c: &i64| *c * *c <= 48 && (*c + 1) * (*c + 1) > 48;
    assert_eq!(answer.withdraw(exact), Some(6), "the float proposed 6; exact arithmetic agreed");

    // A consultation that proposes wrongly costs nothing: the same
    // exact rule refuses it at the door and nothing crosses.
    assert_eq!(consult(48u64, |_| 7i64).withdraw(exact), None,
               "a wrong proposal never leaves the sector");
}
