//! 20_FOUNDING §5 — THE PURSE.
//!
//! A line's economy read whole, as a PROJECTION and never a settling
//! touch. These pins guard the property that distinguishes the two: a
//! purse read deducts what is owed and WRITES NOTHING. If reading a
//! purse ever begins settling, a status render becomes a chain event
//! and every UI refresh writes receipts.
//!
//! Each pin fails if the mechanic changes. None of them assert a
//! definition back at itself.

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{World, FIRST_ORB};

/// The fixture from the landed demurrage witness: a G sun at half
/// orbit banks 15/8 joules, then four ticks pass UNCOLLECTED so the
/// due stands unsettled.
fn warm_world_with_unsettled_due() -> (World, u64) {
    let mut world = World::new();
    world.found("purse").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    world.zone(16, ZoneKind::Encasement).expect("zones");
    world.tick();
    world.collect().expect("collects");
    let touch = world.orb().expect("orb").last_touch;
    for _ in 0..4 {
        world.tick();
    }
    (world, touch)
}

fn joules(purse: &[(Resource, Ratio)]) -> Ratio {
    purse
        .iter()
        .find(|(r, _)| *r == Resource::Joule)
        .map(|(_, a)| a.clone())
        .unwrap_or_else(|| ratio(0, 1))
}

/// THE PROJECTION LAW: reading a purse writes NO receipts. This is the
/// pin the whole design rests on — if it fails, looking at your money
/// has become a chain event.
#[test]
fn reading_a_purse_writes_nothing() {
    let (world, touch) = warm_world_with_unsettled_due();
    let before = world.chronicle().receipts().len();
    let committed_before = world.committed();

    for _ in 0..8 {
        let _ = world.purse(touch + 4);
    }

    assert_eq!(
        world.chronicle().receipts().len(),
        before,
        "a purse read receipted something — it is settling, not projecting"
    );
    assert_eq!(
        world.committed(),
        committed_before,
        "a purse read moved committed state"
    );
}

/// The reading DEDUCTS the standing due rather than showing the gross
/// balance. Banked 15/8, flat decay 1/32 × 4 elapsed = 1/8, so the
/// purse reads 7/4 while the orb still holds 15/8 un-settled.
#[test]
fn the_purse_deducts_the_unsettled_due() {
    let (world, touch) = warm_world_with_unsettled_due();
    let gross = world.orb().expect("orb").banked_amount(Resource::Joule);
    assert_eq!(gross, ratio(15, 8), "fixture drifted");

    let read = joules(&world.purse(touch + 4));
    assert_eq!(
        read,
        ratio(7, 4),
        "the purse showed the gross balance — value is hiding from decay"
    );
    assert!(read < gross, "the purse did not deduct anything");
}

/// The due GROWS with elapsed time, so the same stores read poorer the
/// longer they sit. Drives the reading through `demurrage_due` rather
/// than proving it inert.
#[test]
fn the_purse_falls_as_the_due_accrues() {
    let (world, touch) = warm_world_with_unsettled_due();
    let near = joules(&world.purse(touch + 1));
    let far = joules(&world.purse(touch + 4));
    assert!(
        far < near,
        "a longer wait read no poorer — the due is not load-bearing in the purse"
    );
    assert_eq!(near - far, ratio(3, 32), "the flat rate is not being applied");
}

/// A purse never reads negative: the due is capped at what is held.
#[test]
fn a_purse_never_reads_negative() {
    let (world, touch) = warm_world_with_unsettled_due();
    for elapsed in [0u64, 4, 40, 4_000, 400_000] {
        for (resource, amount) in world.purse(touch + elapsed) {
            assert!(
                amount >= ratio(0, 1),
                "{resource:?} read negative at elapsed {elapsed} — the cap is not holding"
            );
        }
    }
}

/// Ordered by resource, so two nodes reading the same state produce the
/// same sequence.
#[test]
fn the_purse_reads_in_a_stable_order() {
    let (world, touch) = warm_world_with_unsettled_due();
    let purse = world.purse(touch + 4);
    let mut codes: Vec<u8> = purse.iter().map(|(r, _)| *r as u8).collect();
    let seen = codes.clone();
    codes.sort_unstable();
    assert_eq!(seen, codes, "purse order is not stable — the read is not replayable");
    codes.dedup();
    assert_eq!(codes.len(), purse.len(), "a resource appeared twice — orbs were not summed");
}
