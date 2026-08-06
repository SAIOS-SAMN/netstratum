//! **THE TORSION DEFINITION, PINNED** (#44, operator condition on the
//! no-new-physics agreement 2026-08-06).
//!
//! Netstratum's torsion IS the key-fiber commutator (23_TRAVERSAL
//! §2k.1: `[a, b] = a ∘ b ∘ a⁻¹ ∘ b⁻¹` in the free group over moves)
//! — and it is NOT the homological torsion the xylarium constellation
//! reads (their `torsion_chain` over T_k; the name-collision rule
//! stands). These pins hold the algebra so no merge or relay can
//! silently swap the reading into a gate:
//!
//! - additive moves commute — their commutator is the identity;
//! - the inverse law holds exactly on the axes where inversion is
//!   lawful (`a ∘ a⁻¹ = e` on banked/gauge/books);
//! - replacement axes (seats, records) do not invert — a commutator
//!   over them CARRIES torsion, which is the whole point of §2k.1;
//! - the lens's `torsion_order` is a separate reading of the gauge
//!   planes (h[4..10]) and shares nothing but the word.

use cfe::dop::Resource;
use cfe::ratio::ratio;
use kernel::motion::Move;

#[test]
fn additive_moves_commute_and_invert() {
    let a = Move::banked_of(1, Resource::Iron, ratio(3, 2));
    let b = Move::banked_of(2, Resource::Biomass, ratio(5, 1));
    assert!(
        a.commutator(&b).is_identity(),
        "additive axes are abelian — no torsion between bank moves"
    );
    assert!(
        a.then_free(&a.inverse()).is_identity(),
        "a ∘ a⁻¹ = e on the axes where inversion is lawful"
    );
    assert!(a.commutes_with(&b));
}

#[test]
fn replacement_axes_carry_torsion() {
    // A seat move: replacement, not addition — it has no inverse, so
    // the commutator with anything seat-bearing is NOT the identity.
    let seat = Move { seats: vec![("tunborn".into(), 1)], ..Move::identity() };
    let bank = Move::banked_of(1, Resource::Iron, ratio(1, 1));
    let torsion = seat.commutator(&bank);
    assert!(
        !torsion.is_identity(),
        "a replacement axis does not invert — the pair carries torsion"
    );
    // And the torsion is exactly the uninvertible remainder: the seats
    // survive; the additive components cancel.
    assert!(torsion.banked.is_empty(), "the bank halves cancelled");
    assert!(!torsion.seats.is_empty(), "the seat is the torsion");
}
