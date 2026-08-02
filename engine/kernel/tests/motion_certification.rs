//! CERTIFICATION OF THE MOVE ALGEBRA (`19_KERNEL` §3a.2).
//!
//! Each law is proved by a witness that FAILS if the law breaks — not
//! by restating a definition. The non-commutativity witness in
//! particular exhibits a concrete pair whose two orderings land on
//! different standings, so it cannot pass vacuously.

use cfe::dop::Resource;
use cfe::ratio::ratio;
use kernel::motion::{compose_scheduled, envelope, Move, Slot, Standing};

const ORB: u64 = 1;

fn earn(amount: i64) -> Move {
    Move::banked_of(ORB, Resource::Iron, ratio(amount, 1))
}
fn spend(amount: i64) -> Move {
    Move::banked_of(ORB, Resource::Iron, ratio(-amount, 1))
}
fn holding(amount: i64) -> Standing {
    Standing { banked: vec![((ORB, Resource::Iron), ratio(amount, 1))], seats: Vec::new() }
}

// ── AXIOM 1 · IDENTITY ────────────────────────────────────────────────

#[test]
fn identity_is_neutral_on_both_sides() {
    let a = earn(7);
    let e = Move::identity();
    assert_eq!(e.then_free(&a), a, "e ∘ a = a");
    assert_eq!(a.then_free(&e), a, "a ∘ e = a");
    assert!(e.is_identity(), "and the identity reads as one");

    // Applied to a standing, it moves nothing.
    let held = holding(5);
    assert_eq!(held.then(&e), held, "the identity move changes no standing");
}

// ── AXIOM 2 · ASSOCIATIVITY ───────────────────────────────────────────

#[test]
fn free_composition_is_associative() {
    let (a, b, c) = (earn(3), spend(1), earn(11));
    assert_eq!(
        a.then_free(&b).then_free(&c),
        a.then_free(&b.then_free(&c)),
        "(a ∘ b) ∘ c = a ∘ (b ∘ c)"
    );
}

#[test]
fn application_does_not_distribute_over_free_composition_when_the_floor_bites() {
    // A DRAFT OF THIS TEST COMPARED AN EXPRESSION TO ITSELF and passed
    // vacuously. Replaced with the claim that actually matters, which
    // turns out to be a CONSTRAINT rather than a law:
    //
    //     s.then(a).then(b)   is NOT  s.then(a ∘ b)   in general
    //
    // Free composition is unclamped; application is clamped. Pre-
    // composing two moves and applying once skips the intermediate
    // floor, so batching CHANGES SEMANTICS.
    let held = holding(5);
    let (a, b) = (spend(10), earn(5));

    let stepwise = held.then(&a).then(&b);
    let batched = held.then(&a.then_free(&b));

    assert_eq!(stepwise.held(ORB, Resource::Iron), ratio(5, 1), "floors at 0, then earns 5");
    assert_eq!(batched.held(ORB, Resource::Iron), ratio(0, 1), "net −5 applied once, floored");
    assert_ne!(stepwise, batched, "batching is not free — the intermediate floor is real");
}

#[test]
fn application_and_free_composition_agree_when_no_floor_is_reached() {
    // The constraint above is not a defect: away from the floor the two
    // agree exactly, so batching is safe precisely when nothing bottoms
    // out. That boundary is what an implementation must respect.
    let held = holding(100);
    let (a, b) = (spend(10), earn(5));
    assert_eq!(
        held.then(&a).then(&b),
        held.then(&a.then_free(&b)),
        "no clamp, no difference"
    );
}

// ── AXIOM 3 · INVERSION ───────────────────────────────────────────────

#[test]
fn every_move_has_a_formal_inverse() {
    let a = earn(9).then_free(&Move { gauge: vec![ratio(2, 1)], ..Move::default() });
    let undone = a.then_free(&a.inverse());
    assert!(undone.is_identity(), "a ∘ a⁻¹ = e on the additive axes, got {undone:?}");
    assert!(a.inverse().then_free(&a).is_identity(), "a⁻¹ ∘ a = e");
}

#[test]
fn the_inverse_is_algebra_and_not_permission() {
    // The inverse EXISTS so the commutator can be written (§2k.1: the
    // commutator IS the torsion). It is not thereby a lawful deed —
    // ARTICLE IV forbids retreating rank and nothing un-mints.
    //
    // The floor is where that shows up mechanically: undoing a spend
    // that was CLAMPED does not restore what was lost, so `a⁻¹` is not
    // an undo of `a` on a real standing.
    let held = holding(5);
    let overspend = spend(10);
    let bottomed = held.then(&overspend);
    assert_eq!(bottomed.held(ORB, Resource::Iron), ratio(0, 1), "the floor held");

    let attempted_undo = bottomed.then(&overspend.inverse());
    assert_eq!(
        attempted_undo.held(ORB, Resource::Iron),
        ratio(10, 1),
        "applying the inverse does NOT return to 5 — it adds 10 to a floored 0"
    );
    assert_ne!(attempted_undo, held, "a ∘ a⁻¹ is not the identity on a CLAMPED standing");
}

// ── AXIOM 4 · NON-COMMUTATIVITY, with a concrete witness ──────────────

#[test]
fn composition_does_not_commute_the_floor_is_the_reason() {
    // THE WITNESS. Holding 5: spend 10 then earn 5 bottoms out at 0 and
    // ends at 5. Earn 5 then spend 10 reaches 10 and ends at 0.
    // Same two moves, two orders, two standings.
    let held = holding(5);
    let (a, b) = (spend(10), earn(5));

    let a_then_b = held.then(&a).then(&b);
    let b_then_a = held.then(&b).then(&a);

    assert_eq!(a_then_b.held(ORB, Resource::Iron), ratio(5, 1), "spend-first floors, then earns");
    assert_eq!(b_then_a.held(ORB, Resource::Iron), ratio(0, 1), "earn-first affords the spend");
    assert_ne!(a_then_b, b_then_a, "a ∘ b ≠ b ∘ a — the floor is the source");
}

#[test]
fn replacement_is_the_second_source_of_non_commutativity() {
    // A seat is not a delta; the last seating stands. Two orderings of
    // one line give two different offices.
    let held = Standing::default();
    let a = Move { seats: vec![("abyssal-vocalis".into(), 1)], ..Move::default() };
    let b = Move { seats: vec![("abyssal-vocalis".into(), 3)], ..Move::default() };

    assert_eq!(held.then(&a).then(&b).seats, vec![("abyssal-vocalis".to_string(), 3)]);
    assert_eq!(held.then(&b).then(&a).seats, vec![("abyssal-vocalis".to_string(), 1)]);
    assert_ne!(held.then(&a).then(&b), held.then(&b).then(&a), "replacement does not commute");
}

#[test]
fn unclamped_additive_axes_do_commute_and_the_commutator_says_so() {
    // Non-commutativity is a property of PARTICULAR AXES, not a
    // decoration on all of them. The free group over additive axes is
    // abelian, and the commutator must report exactly that — otherwise
    // it would flag torsion where none stands.
    let (a, b) = (earn(4), spend(9));
    assert!(a.commutes_with(&b), "additive unclamped axes commute");
    assert!(a.commutator(&b).is_identity(), "[a, b] = e where they commute");
}

#[test]
fn the_commutator_needs_the_inverse_to_exist_at_all() {
    // The point of AXIOM 3: [a,b] = a∘b∘a⁻¹∘b⁻¹ is unwritable without
    // inversion, and §2k.1 established the commutator IS the torsion.
    // No inverses, no torsion measurement.
    let (a, b) = (earn(3), earn(5));
    let by_hand = a.then_free(&b).then_free(&a.inverse()).then_free(&b.inverse());
    assert_eq!(a.commutator(&b), by_hand, "the commutator is exactly that word");
}

// ── THE SCHEDULE · what makes non-commutativity SAFE ──────────────────

#[test]
fn the_schedule_makes_the_input_order_irrelevant() {
    // THE CONSENSUS CLAIM. If a ∘ b ≠ b ∘ a, two nodes holding the same
    // moves reach different states unless the order is canonical. The
    // schedule sorts by (stamp, sequence) — both already in the receipt
    // and inside its hash — so arrangement cannot change the outcome.
    let held = holding(5);
    let moves = vec![
        (Slot { stamp: 10, sequence: 2 }, earn(5)),
        (Slot { stamp: 10, sequence: 1 }, spend(10)),
    ];
    let shuffled = vec![moves[1].clone(), moves[0].clone()];

    let one = compose_scheduled(&held, moves);
    let other = compose_scheduled(&held, shuffled);
    assert_eq!(one, other, "the schedule fixes the order; the input arrangement does not matter");

    // And it composes in SEQUENCE order, not arrival order: sequence 1
    // (the spend) floors first, so the standing ends at 5.
    assert_eq!(one.held(ORB, Resource::Iron), ratio(5, 1), "sequence 1 before sequence 2");
}

#[test]
fn stamp_alone_is_insufficient_which_is_why_sequence_is_the_tiebreak() {
    // Several deeds share a tick. If the schedule keyed on stamp only,
    // two deeds at one stamp would have no defined order and a
    // non-commutative pair would diverge between nodes.
    let a = Slot { stamp: 10, sequence: 1 };
    let b = Slot { stamp: 10, sequence: 2 };
    assert!(a < b, "same stamp, and the order is still total");

    let c = Slot { stamp: 11, sequence: 0 };
    assert!(b < c, "a later stamp outranks a higher sequence");
}

// ── THE ENVELOPE · O(n^D), replacing the flat bound ───────────────────

#[test]
fn the_envelope_is_the_volume_and_it_breathes_with_rank() {
    // A flat O(1) budget was a scalar collapse — fixed regardless of
    // how much structure stands. The volume of D independent axes of
    // side n is n^D, and it WIDENS as rank is earned.
    assert_eq!(envelope(4, 1), Some(4), "one axis: the line");
    assert_eq!(envelope(4, 2), Some(16), "two axes: the square");
    assert_eq!(envelope(4, 3), Some(64), "three axes: the volume");

    // Monotone in rank, which is what makes it a re-volumizer rather
    // than a ceiling: ARTICLE IV holds rank non-decreasing, so the
    // envelope never narrows.
    let mut previous = 0u128;
    for rank in 1..=8 {
        let volume = envelope(6, rank).expect("no overflow at this scale");
        assert!(volume > previous, "rank {rank}: the envelope must widen, got {volume}");
        previous = volume;
    }
}

#[test]
fn rank_zero_is_the_degenerate_point_and_that_is_the_vacuity_case() {
    // n^0 = 1 — a single point, no volume. That is exactly the standing
    // capacity_law refuses to mint on (Vacuity, D = 0): the envelope
    // agrees with the constitution rather than contradicting it.
    assert_eq!(envelope(1_000_000, 0), Some(1), "no rank, no room");
}

#[test]
fn the_envelope_refuses_to_wrap() {
    // An envelope that silently wrapped would report an enormous space
    // as a tiny one, which is worse than refusing to answer.
    assert_eq!(envelope(u64::MAX, 64), None, "overflow refuses rather than lying");
}
