//! **#136 — THE SMELT, EXTENDED TO ELEMENT FILLS** (operator ruling B, 2026-08-20).
//!
//! Every conserved world-physics ledger declares its grain at its fill site.
//! Element aperture fills bypassed the SMELT that resource fills obey, so element
//! stocks drifted to ~700-bit denominators and stalled `mass_of`. These witnesses
//! pin the restored law: a delivered element fill lands on the 1/64 SMELT grain
//! (its denominator divides 64), it equals the SMELTED iceberg exactly, and no
//! matter is created — the sender debits precisely what it dispatched, the void is
//! a lawful non-negative loss.

use cfe::dop::{positive, unit, ClosureEvent, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{element_aperture_transfer, smelt, World, FIRST_ORB};
use num_bigint::BigInt;

/// The whole board's element mass — Σ every stock on every orb, exact.
fn element_total(world: &World) -> Ratio {
    let mut total = ratio(0, 1);
    for orb in world.orbs() {
        for (_, amount) in &orb.elements {
            total += amount.clone();
        }
    }
    total
}

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

/// `denom | 1024` — the element-ledger grain invariant: the amount is some k/1024 in
/// lowest terms (denominator a divisor of 1024), so exact-rational replay stays
/// affordable. The 1/64 haul lands on a coarser sub-grid (64 | 1024), also on-grain.
fn on_grain(x: &Ratio) -> bool {
    (BigInt::from(1024) % x.denom()) == BigInt::from(0)
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

/// **#136 D2 — THE SMELT-SWEEP FLOORS AND CONSERVES WITH DUST.** The migration
/// floors every off-grain element stock to the 1/64 grain, records the total
/// shaving in the deed, and is bit-exact: pre-sweep mass = post-sweep mass + dust.
/// It appends the deed only when something was off-grain, and it is idempotent —
/// a board already on the grain sweeps to nothing.
#[test]
fn the_smelt_sweep_floors_and_conserves_with_dust() {
    let mut world = sunlit("sweep");
    let element = bank_element(&mut world);
    // Force a legacy off-grain stock: bank a third on top of the assayed amount
    // (k/64 + 1/3 has denominator 192 — off the grain), the shape a pre-ruling-B
    // un-smelted haul would have left behind.
    world.orbs_mut()[0].bank_element(element, ratio(1, 3));
    let banked = world.orb().expect("orb").element_amount(&element);
    assert!(!on_grain(&banked), "the fixture is genuinely off-grain: {banked}");

    let pre = element_total(&world);
    let dust = world.smelt_sweep().expect("an off-grain board sweeps");

    // Every stock now lands on the grain, and mass is conserved WITH the dust.
    for orb in world.orbs() {
        for (_, amount) in &orb.elements {
            assert!(on_grain(amount), "post-sweep stock off-grain: {amount}");
        }
    }
    let post = element_total(&world);
    assert_eq!(pre, post + dust.clone(), "pre = post + dust, bit-for-bit");
    assert!(dust >= ratio(0, 1), "the dust is a shaving, never a mint");

    // The sweep is a receipted deed carrying that same dust.
    match world.chronicle().receipts().last().expect("a deed landed").event() {
        ClosureEvent::ElementStocksSwept { dust: recorded } => {
            assert_eq!(*recorded, dust, "the deed records the swept dust")
        }
        other => panic!("the last deed is the sweep, not {other:?}"),
    }

    // Idempotent + self-terminating: a board already on the grain adds no deed.
    let before = world.chronicle().receipts().len();
    assert!(world.smelt_sweep().is_none(), "an on-grain board sweeps to nothing");
    assert_eq!(world.chronicle().receipts().len(), before, "no second deed");
}

/// **#136 D2 — THE SWEEP MIGRATES A RESUMED CHRONICLE, REPLAY-EXACT.** A pre-ruling-B
/// chronicle carries an un-smelted element fill; on resume the whole history replays
/// (reproducing the off-grain stock) and the sweep floors it as a committed deed, so
/// `replay == committed` holds across the migration. Re-resuming that chronicle is a
/// fixed point: the deed floors during replay and no new deed is minted.
#[test]
fn the_smelt_sweep_migrates_a_resumed_chronicle() {
    let mut world = sunlit("resume-sweep");
    let element = bank_element(&mut world);
    let orb_id = world.orb().expect("orb").orb_id;
    let stamp = world.now() + 1;
    // A raw legacy fill: an un-smelted third banked straight onto the ledger, the
    // exact shape `haul_element` left before ruling B extended the SMELT.
    world.chronicle_mut().append(
        ClosureEvent::ElementApertureReceived { deal: 7, to_orb: orb_id, element, amount: ratio(1, 3) },
        stamp,
        vec![1],
    );

    // Resume replays the off-grain history, then the sweep floors it.
    let resumed = World::resume(world.chronicle().clone());
    let migrated = resumed.orb().expect("orb").element_amount(&element);
    assert!(on_grain(&migrated), "the resumed stock is migrated onto the grain: {migrated}");
    assert!(
        matches!(
            resumed.chronicle().receipts().last().expect("a deed").event(),
            ClosureEvent::ElementStocksSwept { .. }
        ),
        "resume minted the migration deed"
    );

    // A fixed point: re-resuming floors during replay and mints NO new deed, and the
    // committed stock is byte-identical — the migration is a one-time deed, forever.
    let again = World::resume(resumed.chronicle().clone());
    assert_eq!(
        again.orb().expect("orb").element_amount(&element),
        migrated,
        "re-resume is a fixed point (replay == committed across the sweep)"
    );
    assert_eq!(
        again.chronicle().receipts().len(),
        resumed.chronicle().receipts().len(),
        "no second sweep deed on re-resume (self-terminating)"
    );
}
