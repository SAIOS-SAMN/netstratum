//! **THE ENDOWMENT IS JUDGED + THE FOUNDING BOOTSTRAP** (task #7,
//! 2026-08-12 — the live judge tally resolved to named receipts).
//!
//! STAR-ENDOW-1 (c2242de) landed the commissioning kit and the siting
//! band gift WITHOUT kernel arms, so every non-G commissioning and
//! every cryo/hyperthermal siting on the live board read as
//! `ConservationBroken` — the judge disagreeing with the law. And the
//! chronicle's opening deed — the sovereign registering ITSELF — read
//! `NoSuchEntity` on every world ever founded, because containment
//! demanded a registrar precede the first identity.
//!
//! Both laws land here as the probe measured them: a real world's own
//! receipts admit whole; a forged top-up refuses by VALUE.

use cfe::dop::ClosureEvent;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{World, FIRST_ORB, SOVEREIGN};
use kernel::{admits, Refused, Step};

/// Judge every receipt of a world's chronicle pairwise (the
/// conductor's per-deed walk, verbatim) and return the refusals.
fn judge_whole(world: &World) -> Vec<(u64, Refused)> {
    let receipts = world.chronicle().receipts().to_vec();
    let mut refused = Vec::new();
    let mut prior = cfe::world::replay(
        &cfe::vrp::Chronicle::from_receipts(Vec::new()).expect("empty"),
    );
    for at in 0..receipts.len() {
        let next = cfe::world::replay(
            &cfe::vrp::Chronicle::from_receipts(receipts[..=at].to_vec()).expect("prefix"),
        );
        let receipt = &receipts[at];
        let step = Step {
            event: receipt.event().clone(),
            actor: receipt.provenance().first().copied().unwrap_or(SOVEREIGN),
            stamp: receipt.stamp(),
        };
        if let Err(unlawful) = admits(&prior, &next, &step) {
            refused.push((receipt.sequence(), unlawful));
        }
        prior = next;
    }
    refused
}

/// The founding arc — registration, founding, a NON-G commissioning
/// (kit differs from the founding packet) and a CRYOGENIC siting (band
/// gift) — admits whole. This was 3 refusals per world before the
/// arms landed.
#[test]
fn the_founding_arc_admits_whole() {
    let mut w = World::new();
    w.found("endow-judged").expect("founds");
    w.commission_star(StarClass::M).expect("commissions");
    w.site_planet(
        FIRST_ORB,
        cfe::dop::unit(0, 1),
        cfe::dop::positive(100, 1),
        None,
        None,
    )
    .expect("sites the vault band");
    let refused = judge_whole(&w);
    assert!(
        refused.is_empty(),
        "the founding arc must admit whole; refused: {refused:?}"
    );
}

/// The negative: a commissioning whose `next` credits MORE than the
/// kit's top-up refuses by VALUE — the arm is a bound, not a waiver.
#[test]
fn a_forged_top_up_refuses_by_value() {
    let mut w = World::new();
    w.found("endow-forged").expect("founds");
    let prior = w.committed();
    w.commission_star(StarClass::M).expect("commissions");
    let mut next = w.committed();
    // Forge: an extra iron rides the commissioning.
    if let Some(orb) = next.orbs.first_mut() {
        orb.bank(cfe::dop::Resource::Iron, ratio(1, 1));
    }
    let receipt = w
        .chronicle()
        .receipts()
        .iter()
        .rev()
        .find(|r| matches!(r.event(), ClosureEvent::StarCommissioned { .. }))
        .expect("the commissioning receipted")
        .clone();
    let step = Step {
        event: receipt.event().clone(),
        actor: SOVEREIGN,
        stamp: receipt.stamp(),
    };
    assert!(
        matches!(
            admits(&prior, &next, &step),
            Err(Refused::ConservationBroken { .. })
        ),
        "an over-credited kit must refuse"
    );
}

/// THE FOUNDING BOOTSTRAP is exactly one deed wide: the first identity
/// enters by its own hand; a SECOND self-registration (prior already
/// holds an entity) still refuses `NoSuchEntity` for an unregistered
/// registrar.
#[test]
fn the_bootstrap_is_one_deed_wide() {
    let mut w = World::new();
    w.found("boot-narrow").expect("founds");
    let prior = w.committed();
    assert!(!prior.entities.is_empty(), "the sovereign registered");
    let step = Step {
        event: ClosureEvent::EntityRegistered {
            entity_id: 99,
            principal: 99,
            handle: "usurper".to_string(),
        },
        actor: 99,
        stamp: w.now(),
    };
    // next: the usurper appears (as a lawless writer would have it).
    let mut next = prior.clone();
    next.entities.push(cfe::ipsal::Entity {
        entity_id: 99,
        handle: "usurper".to_string(),
        principal: Some(99),
        grants: Vec::new(),
    });
    assert!(
        matches!(
            admits(&prior, &next, &step),
            Err(Refused::NoSuchEntity { actor: 99 })
        ),
        "self-registration past the first identity must refuse"
    );
}
