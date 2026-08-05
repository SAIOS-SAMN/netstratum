//! THE INTERLEAVING STRESS WITNESS (HANDOFF §9.1, dated 2026-07-27):
//! the two claims that shipped proven-by-argument, now measured.
//!
//! 1. Two IDENTICAL-REMAINDER absorption debts on one lane lapsing at
//!    DIFFERENT ticks: the replay removal is first-match by
//!    pair+remainder — measured here to take the OLDEST, exactly as
//!    live emission does, including at the mid-flight point where
//!    only the surviving debt's `opened` stamp can tell them apart.
//! 2. Absorb lapse + bond settle + forward resolution landing on ONE
//!    lane at ONE stamp: three world-driven resolutions in a single
//!    tick prologue — replay and resume rebuild the same state from
//!    the same receipt order.

use cfe::dop::{positive, unit, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// The furnace↔coldstore fixture, verbatim from `world_absorb.rs`
/// (tests cannot share modules; the twin is deliberate): ends at
/// stamp 19 — source holds 1 un-sealed pending overflow (struck),
/// the charted banks 59997/10000 sink.
fn furnace_and_coldstore() -> (World, u64) {
    let mut world = World::new();
    world.found("furnace").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects"); // 30 iron, 13 biomass @16
    let charted = world.chart("coldstore", FIRST_ORB).expect("charts");
    world.zone(16, ZoneKind::Encasement).expect("zones the shell");
    world
        .zone_as(SOVEREIGN, charted, 0, ZoneKind::CryoArray)
        .expect("zones the array");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 2), None, None)
        .expect("sites hot");
    world
        .site_planet(charted, unit(0, 1), positive(100, 1), None, None)
        .expect("sites cold");
    world.tick();
    world.collect().expect("banks the joules"); // @17: seals vent 1/2
    world
        .transfer(cfe::dop::between(FIRST_ORB, charted), Resource::Joule, cfe::dop::positive(1, 1))
        .expect("relieves the freeze");
    world.tick(); // @17: thawed — captures, burns 1/4
    world.tick(); // @18
    world
        .collect_as(SOVEREIGN, charted)
        .expect("banks the sink"); // @19
    (world, charted)
}

fn lapse_stamps(world: &World) -> Vec<(u64, Ratio, u64)> {
    world
        .chronicle()
        .receipts()
        .iter()
        .filter_map(|r| match &r.event {
            ClosureEvent::AbsorptionLapsed { debtor_orb, amount, .. } => {
                Some((*debtor_orb, amount.clone(), r.stamp))
            }
            _ => None,
        })
        .collect()
}

/// HANDOFF §9.1, first claim. Two debts, one lane, IDENTICAL
/// remainders, different opened stamps. At the mid-flight point (the
/// older lapsed, the younger standing) the only distinguishing byte
/// is the survivor's `opened` — if replay's first-match removal ever
/// took the younger, this equality breaks. Measured to the end and
/// carried through resume.
#[test]
fn identical_remainders_lapse_in_emission_order() {
    let (mut world, charted) = furnace_and_coldstore();
    world
        .absorb(cfe::dop::between(FIRST_ORB, charted), cfe::dop::positive(1, 2))
        .expect("absorbs half"); // debt A @19 → lapses 27
    world.tick(); // @20 — the standing strike keeps venting
    world.tick(); // @21
    world
        .absorb(cfe::dop::between(FIRST_ORB, charted), cfe::dop::positive(1, 2))
        .expect("absorbs the same again"); // debt B @21 → lapses 29
    let debts = world.committed().debts;
    assert_eq!(debts.len(), 2, "two windows stand open");
    assert_eq!(debts[0].remainder, debts[1].remainder, "identical remainders");
    assert_eq!((debts[0].opened, debts[1].opened), (19, 21));

    // Walk to the 27 prologue: the OLDER lapses alone (the existing
    // lapse witness counts the same way — prologues read `now` before
    // the walk advances, 21..=27 is seven ticks).
    for _ in 0..7 {
        world.tick();
    }
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").extracted_across,
        ratio(1, 2),
        "one remainder posted"
    );
    let standing = world.committed().debts;
    assert_eq!(standing.len(), 1);
    assert_eq!(standing[0].opened, 21, "the YOUNGER survives live");
    // THE MEASUREMENT: replay's first-match removal must also have
    // taken the older — the survivor's opened stamp is the witness.
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt.debts.len(), 1);
    assert_eq!(rebuilt.debts[0].opened, 21, "replay removed the OLDEST match");
    assert_eq!(rebuilt, world.committed(), "replay MATCHES mid-flight");

    // Walk to 29: the younger follows; the books sum the whole.
    world.tick();
    world.tick();
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").extracted_across,
        ratio(1, 1)
    );
    assert!(world.committed().debts.is_empty());
    let lapses = lapse_stamps(&world);
    assert_eq!(
        lapses,
        vec![
            (FIRST_ORB, ratio(1, 2), 27),
            (FIRST_ORB, ratio(1, 2), 29),
        ],
        "two identical lapses, distinct stamps, emission order"
    );
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES at rest");
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.committed(), world.committed(), "resume carries it");
}

/// HANDOFF §9.1, second claim. One lane, one stamp: the reciprocity
/// lapse, the term bond's orderly expiry, and the forward slash all
/// resolve inside a single tick prologue at 27 — three world-driven
/// receipts in one breath. Replay and resume rebuild the identical
/// state from the identical order.
#[test]
fn three_resolutions_share_one_lane_and_one_stamp() {
    let (mut world, charted) = furnace_and_coldstore();
    world
        .absorb(cfe::dop::between(FIRST_ORB, charted), cfe::dop::positive(1, 1))
        .expect("absorbs"); // debt @19 → lapses 27
    world
        .seal_bond(cfe::dop::between(FIRST_ORB, charted), Resource::Biomass, cfe::dop::positive(1, 8), 8)
        .expect("seals the term bond"); // @19 → expires orderly at 27
    world
        .seal_forward(cfe::dop::between(FIRST_ORB, charted), Resource::Iron, cfe::dop::positive(1, 1), cfe::dop::window(8))
        .expect("seals the pact"); // @19, undelivered → slashes at 27

    for _ in 0..8 {
        world.tick(); // 19 → 26: everything still standing
    }
    assert_eq!(world.committed().debts.len(), 1);
    assert_eq!(world.committed().bonds.len(), 1);
    assert_eq!(world.committed().forwards.len(), 1);

    world.tick(); // 27: lapse + expiry + slash, one prologue
    assert!(world.committed().debts.is_empty());
    assert!(world.committed().bonds.is_empty());
    assert!(world.committed().forwards.is_empty());

    // All three receipts stand at stamp 27, in the prologue's fixed
    // emission order: lapse → bond settle → forward slash.
    let at_27: Vec<&'static str> = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| r.stamp == 27)
        .filter_map(|r| match &r.event {
            ClosureEvent::AbsorptionLapsed { .. } => Some("lapse"),
            ClosureEvent::BondSettled { .. } => Some("bond"),
            ClosureEvent::ForwardSlashed { .. } => Some("slash"),
            _ => None,
        })
        .collect();
    assert_eq!(at_27, vec!["lapse", "bond", "slash"], "one stamp, fixed order");

    // The extraction book holds the lapse; the breach book holds the
    // slash; the collateral moved to the aggrieved side.
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").extracted_across,
        ratio(1, 1)
    );
    assert!(
        world.orb_by_id(FIRST_ORB).expect("orb").forwards_breached > ratio(0, 1),
        "the breach posted"
    );

    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES the one-stamp pileup");
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.committed(), world.committed(), "resume carries it");
}
