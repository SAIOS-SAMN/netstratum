//! **#136 — THE SMELT, EXTENDED TO ELEMENT FILLS** (operator ruling B, 2026-08-20).
//!
//! Every conserved world-physics ledger declares its grain at its fill site.
//! Element aperture fills bypassed the SMELT that resource fills obey, so element
//! stocks drifted to ~700-bit denominators and stalled `mass_of`. These witnesses
//! pin the restored law: a delivered element fill lands on the 1/64 SMELT grain
//! (its denominator divides 64), it equals the SMELTED iceberg exactly, and no
//! matter is created — the sender debits precisely what it dispatched, the void is
//! a lawful non-negative loss.

use cfe::dop::{positive, unit, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{element_aperture_transfer, smelt, World, FIRST_ORB};
use num_bigint::BigInt;

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world
}

fn bank_element(world: &mut World) -> [u8; 32] {
    let struck = world.prospect(FIRST_ORB, 1).expect("surface opens");
    for _ in 0..8 {
        world.tick();
    }
    world.assay(FIRST_ORB, struck.element).expect("surface banks");
    struck.element
}

/// `denom | 64` — the grain invariant: the amount is some k/64 in lowest terms
/// (denominator a divisor of 64), so exact-rational replay stays affordable.
fn on_grain(x: &Ratio) -> bool {
    (BigInt::from(64) % x.denom()) == BigInt::from(0)
}

#[test]
fn an_element_fill_lands_on_the_grain() {
    let mut a = sunlit("smelt-a");
    let mut b = sunlit("smelt-b");
    let element = bank_element(&mut a);
    let have = a.orb().expect("orb").element_amount(&element);
    assert!(have > ratio(0, 1), "the assay banked a workable amount");

    // Dispatch the whole holding at the local door (melt 0 — no hull needed). The
    // fill is `smelt(amount · (1 − melt))`; the receiver must land on the grain.
    let amount = have.clone();
    element_aperture_transfer(
        &mut a,
        FIRST_ORB,
        &mut b,
        FIRST_ORB,
        element,
        cfe::dop::Positive::new(amount.clone()).expect("positive"),
        100,
        &ratio(0, 1),
    )
    .expect("open-door haul");

    let delivered = b.orb().expect("orb").element_amount(&element);
    assert!(on_grain(&delivered), "the fill lands on 1/64, not off-grain: {delivered}");
    assert_eq!(
        delivered,
        smelt(&amount),
        "delivered is the SMELTED amount exactly (melt 0 ⇒ smelt(amount))"
    );

    // Conservation: sender debited exactly `amount`; the void (amount − delivered)
    // is a lawful, non-negative loss — no matter created.
    let a_after = a.orb().expect("orb").element_amount(&element);
    assert_eq!(have - a_after, amount, "the sender debits exactly what it dispatched");
    assert!(amount - delivered >= ratio(0, 1), "the void is a loss, never a mint");
}

#[test]
fn a_lossy_fill_is_the_smelted_iceberg() {
    // The delivered formula itself, pinned: for any amount and melt < 1, the fill
    // is `smelt(amount · (1 − melt))` and lands on the grain. (Unit-level, so it
    // needs no hull — the live haul gates positive melt on hull capacity.)
    for (num, den, mnum, mden) in [(7i64, 3i64, 1i64, 3i64), (5, 8, 1, 4), (13, 7, 2, 5)] {
        let amount = ratio(num, den);
        let melt = ratio(mnum, mden);
        let delivered = smelt(&(amount.clone() * (ratio(1, 1) - melt.clone())));
        assert!(on_grain(&delivered), "iceberg fill off-grain: {delivered}");
        assert!(delivered <= amount.clone() * (ratio(1, 1) - melt), "smelt floors, never lifts");
    }
}
