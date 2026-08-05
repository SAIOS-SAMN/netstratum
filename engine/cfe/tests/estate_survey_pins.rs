//! THE ESTATE SURVEY — the three theorems, witnessed (E5b, operator
//! rulings 2026-08-05: *survey is a derivation of observation;
//! observations must be witnessed; surveys must be metered*).
//!
//! Each pin is one theorem's proof at the emitter; the gate's half
//! lives in `kernel/tests/estate_survey_gated.rs`. Replay MATCHES
//! closes every arc — SEAL B is the connectivity proof.

use cfe::dop::{parcels_of, survey_meter, ClosureEvent};
use cfe::ratio::ratio;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn founded_pair() -> World {
    let mut world = World::new();
    world.found("origin").expect("founds");
    world.zone(0, cfe::dop::ZoneKind::Extractor).expect("zones the mine");
    world.zone(8, cfe::dop::ZoneKind::Habitat).expect("zones the farm");
    for _ in 0..30 {
        world.tick();
    }
    world.collect().expect("banks the yield");
    let _ = world.chart("haven", FIRST_ORB).expect("charts the second orb");
    world
}

/// THEOREM 1 (witnessed) — an observation of nothing cannot enter the
/// record, and a lawful survey carries EXACTLY the founding receipts'
/// digests as its witnesses, one per orb, gathered from the chronicle.
#[test]
fn the_survey_is_witnessed_by_the_founding_receipts() {
    let mut empty = World::new();
    // The unfounded world refuses at CONTAINMENT, before the estate is
    // even consulted — no sovereign is registered before the founding,
    // so the actor itself does not stand. `NoColony` guards the deeper
    // (normally unreachable) empty-estate state behind it. Either way:
    // an estate of nothing cannot be observed, and nothing is written.
    assert!(empty.survey_estate(FIRST_ORB).is_err(), "an estate of nothing cannot be observed");
    assert!(empty.chronicle().receipts().is_empty(), "and the refusal orphaned nothing");

    let mut world = founded_pair();
    world.survey_estate(FIRST_ORB).expect("the survey closes");

    // Gather the expected witnesses INDEPENDENTLY of the deed's path.
    let expected: Vec<(u64, [u8; 32])> = world
        .committed()
        .orbs
        .iter()
        .map(|orb| {
            let digest = world
                .chronicle()
                .receipts()
                .iter()
                .find(|receipt| match &receipt.event {
                    ClosureEvent::ColonyFounded { orb_id, .. }
                    | ClosureEvent::ColonyCharted { orb_id, .. } => *orb_id == orb.orb_id,
                    _ => false,
                })
                .map(|receipt| receipt.own.0)
                .expect("every orb was founded by receipt");
            (orb.orb_id, digest)
        })
        .collect();

    let receipt = world.chronicle().receipts().last().expect("the survey receipt");
    let ClosureEvent::EstateSurveyed { covered, .. } = &receipt.event else {
        panic!("the last receipt is the survey");
    };
    assert_eq!(covered, &expected, "one witness per orb: its own founding receipt");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// THEOREM 2 (metered) — cost = rate × volume, drawn exactly; a survey
/// that cannot pay refuses PURE: no receipt, no debit, nothing orphaned.
#[test]
fn the_meter_charges_the_volume_and_an_unpayable_survey_leaves_nothing() {
    let mut world = founded_pair();
    let (resource, rate) = survey_meter();
    let before = world.orbs()[0].banked_amount(resource);
    world.survey_estate(FIRST_ORB).expect("payable");
    let cost = rate * ratio(2, 1);
    assert_eq!(
        world.orbs()[0].banked_amount(resource),
        before - cost.clone(),
        "the meter drew rate × volume, exactly"
    );
    let ClosureEvent::EstateSurveyed { cost: declared, .. } =
        &world.chronicle().receipts().last().expect("receipt").event
    else {
        panic!("survey receipt");
    };
    assert_eq!(declared, &cost, "the receipt declares what the meter drew");

    // The unpayable case: found-then-chart drains the origin's iron to
    // the founding reserve's last unit, then a transfer empties it.
    let mut poor = World::new();
    poor.found("origin").expect("founds");
    let haven = poor.chart("haven", FIRST_ORB).expect("charts");
    let left = poor.orbs()[0].banked_amount(resource);
    if let Some(amount) = cfe::dop::Positive::new(left.clone()) {
        poor.transfer(cfe::dop::between(FIRST_ORB, haven), resource, amount)
            .expect("drains");
    }
    let receipts_before = poor.chronicle().receipts().len();
    let banked_before = poor.orbs()[0].banked_amount(resource);
    match poor.survey_estate(FIRST_ORB) {
        Err(ActionRefused::CannotAfford { .. }) => {}
        other => panic!("an unpayable survey must refuse CannotAfford, got {other:?}"),
    }
    assert_eq!(
        poor.chronicle().receipts().len(),
        receipts_before,
        "a refusal orphans nothing — no receipt"
    );
    assert_eq!(
        poor.orbs()[0].banked_amount(resource),
        banked_before,
        "and no debit"
    );
    assert_eq!(replay(poor.chronicle()), poor.committed(), "replay MATCHES");
}

/// THEOREM 3 (derivation) — parcels derive from the receipt and ONLY
/// from it: none before any survey, identical on every read after, and
/// re-keyed by the LATEST survey when the estate is re-metered.
#[test]
fn parcels_derive_from_the_receipt_and_only_from_it() {
    let mut world = founded_pair();
    assert!(world.parcels().is_none(), "an unsurveyed estate has no parcels");

    let seed = world.survey_estate(FIRST_ORB).expect("surveys");
    let first = world.parcels().expect("parcels derive from the receipt");
    let again = world.parcels().expect("still derive");
    assert_eq!(first, again, "the same receipt yields the same parcels forever");

    let orbs: Vec<u64> = world.committed().orbs.iter().map(|orb| orb.orb_id).collect();
    assert_eq!(first, parcels_of(&seed, &orbs), "the derivation IS the public rule");

    // Re-survey: metered again, and the parcels re-key to the latest.
    let seed2 = world.survey_estate(FIRST_ORB).expect("re-surveys, re-metered");
    assert_ne!(seed, seed2, "a new survey stands on a new terminal digest");
    assert_eq!(
        world.parcels().expect("derive"),
        parcels_of(&seed2, &orbs),
        "the LATEST survey is the sole source"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// THEOREM 4 (the exact break) — every parcel's free and monetary
/// portions sum to exactly one: no residue truncated, none invented.
/// What a break leaves over is the carry the interest law accrues on,
/// and it is exact by construction.
#[test]
fn the_break_is_exact_for_every_parcel() {
    let mut world = founded_pair();
    world.survey_estate(FIRST_ORB).expect("surveys");
    let parcels = world.parcels().expect("derive");
    assert_eq!(parcels.len(), 2, "one parcel per covered orb");
    for parcel in &parcels {
        assert_eq!(
            parcel.free.clone() + parcel.monetary.clone(),
            ratio(1, 1),
            "free + monetary is the whole, exactly"
        );
        assert!(parcel.free >= ratio(0, 1) && parcel.free <= ratio(1, 1));
        assert!(parcel.monetary >= ratio(0, 1) && parcel.monetary <= ratio(1, 1));
    }
}
