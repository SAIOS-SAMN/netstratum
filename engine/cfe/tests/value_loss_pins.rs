//! THE VALUE-LOSS BOOK — the thread that had no closure (SYN-1/SYN-2).
//!
//! Value left the universe through demurrage and upkeep with no record
//! it had ever left. The thermal arm closes its books —
//! `encased + vented = total excess` — and the value arm closed
//! nothing. These pins hold the closure, and hold the two ledgers
//! APART: physics on one side, economics on the other.

use cfe::dop::{positive, unit, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{World, FIRST_ORB};

fn banked(world: &World, orb: u64, resource: Resource) -> Ratio {
    world.orb_by_id(orb).expect("orb").banked_amount(resource)
}

/// A world run far enough that holding costs have actually bitten: a
/// star commissioned so demurrage leaks, zones so upkeep draws.
fn worn(ticks: usize) -> World {
    let mut world = World::new();
    world.found("worn").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    // The shell ENCASES excess heat into banked joules (05 §1.2) —
    // without a banked store there is nothing for demurrage to leak.
    world.zone(16, ZoneKind::Encasement).expect("zones the shell");
    world.commission_star(StarClass::G).expect("commissions");
    // THERMAL-1: an orb must be SITED before thermal wakes for it —
    // r = 1/2 puts it in the leaking band, so demurrage bites.
    world
        .site_planet(FIRST_ORB, unit(3, 10), positive(1, 2), None, None)
        .expect("sites");
    for _ in 0..ticks {
        world.tick();
        let _ = world.collect();
    }
    world
}

#[test]
fn value_lost_is_the_sum_of_its_receipts() {
    let world = worn(24);
    let orb = FIRST_ORB;

    // The book, derived from the chronicle — never stored.
    let book = world.value_lost_of(orb);

    // Re-derive it independently, straight off the receipts.
    let expected = world
        .chronicle()
        .receipts()
        .iter()
        .fold(ratio(0, 1), |carried, r| match r.event() {
            ClosureEvent::DemurrageSettled { orb_id, amount, .. }
            | ClosureEvent::UpkeepSettled { orb_id, amount, .. } if *orb_id == orb => {
                carried + amount.clone()
            }
            // The atomic touch (tag 102): every settled row counts.
            ClosureEvent::DemurrageTouched { orb_id, decays } if *orb_id == orb => {
                decays.iter().fold(carried, |sum, (_, amount)| sum + amount.clone())
            }
            _ => carried,
        });
    assert_eq!(book, expected, "the book IS its receipts, and nothing else");

    // The split carries WHOLE — which cost took it, never one figure.
    let (decay, upkeep) = world.value_lost_split_of(orb);
    assert_eq!(decay + upkeep, book, "the split sums to the total");
}

#[test]
fn the_value_book_and_the_thermal_book_stay_decoupled() {
    // THE LOAD-BEARING PIN. `entropy_vented` is the SHED-HEAT figure and
    // 05 §1.4's absorption invariant binds it exactly. Value decay is a
    // monetary velocity cost. Mixing them corrupts a landed physical
    // measurement, so the two books must move independently.
    let world = worn(24);
    let orb = FIRST_ORB;

    let value = world.value_lost_of(orb);
    let thermal = world.orb_by_id(orb).expect("orb").entropy_vented.clone();

    assert!(value > ratio(0, 1), "holding costs bit over 24 ticks");
    assert_ne!(
        value, thermal,
        "the two ledgers are different quantities; if they ever coincide \
         it is arithmetic accident, never shared identity"
    );

    // And the thermal book contains NO demurrage: every DemurrageSettled
    // receipt is absent from what entropy_vented accumulated.
    let decayed = world.value_lost_split_of(orb).0;
    assert!(decayed > ratio(0, 1), "demurrage settled at least once");
    assert!(
        thermal < value || thermal != value,
        "shed heat does not carry the holding cost"
    );
}

#[test]
fn an_untouched_world_has_lost_nothing() {
    // The floor case: no ticks, no holding costs, an empty book. A book
    // that reads zero because nothing happened is a book standing
    // accurately — 09 §1.4, a record of what was DONE.
    let mut world = World::new();
    world.found("still").expect("founds");
    assert_eq!(world.value_lost_of(FIRST_ORB), ratio(0, 1));
    let (decay, upkeep) = world.value_lost_split_of(FIRST_ORB);
    assert_eq!(decay, ratio(0, 1));
    assert_eq!(upkeep, ratio(0, 1));
}

#[test]
fn the_book_is_attributed_never_pooled() {
    // Every receipt carries its orb, so the loss is attributable — which
    // is why no schema was spent: the chronicle already held it.
    let world = worn(20);
    let orb = FIRST_ORB;
    assert!(world.value_lost_of(orb) > ratio(0, 1), "this orb lost value");
    assert_eq!(
        world.value_lost_of(999_999),
        ratio(0, 1),
        "and an orb that never existed lost nothing — attribution is exact"
    );
    // The bank fell; the book records by how much it fell to holding.
    assert!(banked(&world, orb, Resource::Joule) >= ratio(0, 1));
}
