//! **E3.9 — TREATY ELEMENT ALLOW-LISTS** (31_ELEMENT_COMMERCE §4).
//!
//! An allow-list ANNEXES a standing trade pact: while any live list
//! stands on a pair, element crossings between them refuse ids outside
//! the union of every live list. No list ⇒ open matter flows unfenced
//! (the pre-E3.9 law, unchanged). The list dies with the pact it rides
//! — strike, renounce, or expiry — and an empty list refuses at the
//! door, because barring all matter is war's job.

use cfe::dop::{positive, unit, ClosureEvent, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::treaty::{TreatyKind, TreatyTerms, PAIR_MELT};
use cfe::world::{
    charter_element_route, element_aperture_transfer, replay, ActionRefused, World, FIRST_ORB,
};

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

/// Prospect + assay one workable element into the bank (the deposit
/// law guarantees stratum 1 is line 1 and refines at its own depth).
fn bank_element(world: &mut World) -> [u8; 32] {
    let struck = world.prospect(FIRST_ORB, 1).expect("surface opens");
    for _ in 0..8 {
        world.tick();
    }
    world.assay(FIRST_ORB, struck.element).expect("surface banks");
    struck.element
}

fn trade_terms() -> TreatyTerms {
    TreatyTerms::new(TreatyKind::Trade, vec![(PAIR_MELT, ratio(1, 2))]).expect("concord eases")
}

fn haul(
    src: &mut World,
    dst: &mut World,
    element: [u8; 32],
    deal: u64,
) -> Result<(), ActionRefused> {
    element_aperture_transfer(
        src,
        FIRST_ORB,
        dst,
        FIRST_ORB,
        element,
        cfe::dop::Positive::new(ratio(1, 4)).expect("positive"),
        deal,
        &ratio(0, 1),
    )
}

#[test]
fn no_list_means_open_matter() {
    let mut a = sunlit("open-a");
    let mut b = sunlit("open-b");
    let element = bank_element(&mut a);
    // A trade pact WITHOUT a fence changes nothing about which ids
    // cross — the fence exists only where a pair co-signed one.
    a.treaty(7, "open-b", trade_terms(), 0).expect("pact seals");
    haul(&mut a, &mut b, element, 100).expect("open matter flows unfenced");
    assert_eq!(replay(a.chronicle()), a.committed(), "SEAL B src");
    assert_eq!(replay(b.chronicle()), b.committed(), "SEAL B dst");
}

#[test]
fn the_fence_passes_listed_and_refuses_unlisted() {
    let mut a = sunlit("fence-a");
    let mut b = sunlit("fence-b");
    let listed = bank_element(&mut a);
    // A second identity the fence does not name.
    let unlisted = {
        let struck = a.prospect(FIRST_ORB, 2).expect("stratum 2 opens");
        for _ in 0..8 {
            a.tick();
        }
        let _ = a.assay(FIRST_ORB, struck.element);
        struck.element
    };
    assert_ne!(listed, unlisted);
    a.treaty(7, "fence-b", trade_terms(), 0).expect("pact seals");
    a.treaty_elements(7, "fence-b", vec![listed]).expect("fence seals");

    haul(&mut a, &mut b, listed, 100).expect("enumerated matter crosses");
    match haul(&mut a, &mut b, unlisted, 101) {
        Err(ActionRefused::ElementNotAllowed { element, .. }) => {
            assert_eq!(element, unlisted);
        }
        other => panic!("unlisted id must refuse ElementNotAllowed, got {other:?}"),
    }
    // The refusal wrote NOTHING — no receipt on either book.
    assert!(!a.chronicle().receipts().iter().any(|r| {
        matches!(r.event(), ClosureEvent::ElementApertureSent { deal: 101, .. })
    }));
    assert_eq!(replay(a.chronicle()), a.committed(), "SEAL B src");
    assert_eq!(replay(b.chronicle()), b.committed(), "SEAL B dst");
}

/// The two-book law: the RECEIVER's fence bars the id even when the
/// sender's book is unfenced — each world's fence is its own law.
#[test]
fn the_receivers_fence_also_bars() {
    let mut a = sunlit("recv-a");
    let mut b = sunlit("recv-b");
    let element = bank_element(&mut a);
    let decoy = [9u8; 32];
    b.treaty(7, "recv-a", trade_terms(), 0).expect("pact seals");
    b.treaty_elements(7, "recv-a", vec![decoy]).expect("fence seals");
    match haul(&mut a, &mut b, element, 100) {
        Err(ActionRefused::ElementNotAllowed { .. }) => {}
        other => panic!("receiver's fence must bar, got {other:?}"),
    }
}

#[test]
fn lists_compose_by_union() {
    let mut a = sunlit("union-a");
    let mut b = sunlit("union-b");
    let first = bank_element(&mut a);
    let second = {
        let struck = a.prospect(FIRST_ORB, 2).expect("opens");
        for _ in 0..8 {
            a.tick();
        }
        let _ = a.assay(FIRST_ORB, struck.element);
        struck.element
    };
    a.treaty(7, "union-b", trade_terms(), 0).expect("pact seals");
    a.treaty_elements(7, "union-b", vec![first]).expect("first list");
    a.treaty_elements(7, "union-b", vec![second]).expect("second list");
    haul(&mut a, &mut b, first, 100).expect("first list bites");
    haul(&mut a, &mut b, second, 101).expect("second list unions in");
}

#[test]
fn the_empty_list_refuses() {
    let mut a = sunlit("empty-a");
    a.treaty(7, "empty-b", trade_terms(), 0).expect("pact seals");
    assert_eq!(
        a.treaty_elements(7, "empty-b", Vec::new()),
        Err(ActionRefused::AllowListEmpty),
        "barring everything is war's job"
    );
}

#[test]
fn the_list_needs_a_live_trade_pact() {
    let mut a = sunlit("ride-a");
    let id = [3u8; 32];
    // No pact at all.
    assert_eq!(
        a.treaty_elements(7, "ride-b", vec![id]),
        Err(ActionRefused::AllowListNeedsTrade { deal: 7 }),
    );
    // A peace is not a trade — it binds no goods.
    a.treaty(
        8,
        "ride-b",
        TreatyTerms::new(TreatyKind::Peace, Vec::new()).expect("peace"),
        0,
    )
    .expect("peace seals");
    assert_eq!(
        a.treaty_elements(8, "ride-b", vec![id]),
        Err(ActionRefused::AllowListNeedsTrade { deal: 8 }),
    );
    // The right deal with the right kind takes it.
    a.treaty(9, "ride-b", trade_terms(), 0).expect("trade seals");
    a.treaty_elements(9, "ride-b", vec![id]).expect("annex rides");
}

/// The annex dies with the pact: renounce prunes it, and the fence
/// stops biting the moment the treaty it rode is gone.
#[test]
fn renounce_kills_the_fence() {
    let mut a = sunlit("dies-a");
    let mut b = sunlit("dies-b");
    let listed = bank_element(&mut a);
    let unlisted = {
        let struck = a.prospect(FIRST_ORB, 2).expect("opens");
        for _ in 0..8 {
            a.tick();
        }
        let _ = a.assay(FIRST_ORB, struck.element);
        struck.element
    };
    a.treaty(7, "dies-b", trade_terms(), 0).expect("pact seals");
    a.treaty_elements(7, "dies-b", vec![listed]).expect("fence seals");
    assert!(matches!(
        haul(&mut a, &mut b, unlisted, 100),
        Err(ActionRefused::ElementNotAllowed { .. })
    ));
    a.renounce(7, "dies-b").expect("own leg ends");
    assert!(
        a.committed().element_allowlists.is_empty(),
        "the annex is pruned with the pact"
    );
    haul(&mut a, &mut b, unlisted, 101).expect("no pact, no fence");
    assert_eq!(replay(a.chronicle()), a.committed(), "SEAL B after prune");
}

/// Expiry is a READING: a lapsed pact stops its fence biting with no
/// receipt at all (the bond's own shape).
#[test]
fn expiry_silences_the_fence() {
    let mut a = sunlit("lapse-a");
    let mut b = sunlit("lapse-b");
    let listed = bank_element(&mut a);
    let unlisted = {
        let struck = a.prospect(FIRST_ORB, 2).expect("opens");
        for _ in 0..8 {
            a.tick();
        }
        let _ = a.assay(FIRST_ORB, struck.element);
        struck.element
    };
    a.treaty(7, "lapse-b", trade_terms(), 4).expect("term pact seals");
    a.treaty_elements(7, "lapse-b", vec![listed]).expect("fence seals");
    assert!(matches!(
        haul(&mut a, &mut b, unlisted, 100),
        Err(ActionRefused::ElementNotAllowed { .. })
    ));
    for _ in 0..8 {
        a.tick();
        b.tick();
    }
    haul(&mut a, &mut b, unlisted, 101).expect("lapsed pact, silent fence");
    assert_eq!(replay(a.chronicle()), a.committed(), "SEAL B after lapse");
}

/// A lane specialized to a barred element cannot be chartered.
#[test]
fn the_fence_reads_at_the_charter() {
    let mut a = sunlit("lane-a");
    let mut b = sunlit("lane-b");
    // Alloy for the charter bill on both books.
    for world in [&mut a, &mut b] {
        if let Some(orb) = world.committed().orbs.first().map(|o| o.orb_id) {
            let _ = orb;
        }
    }
    let listed = bank_element(&mut a);
    let barred = [5u8; 32];
    a.treaty(7, "lane-b", trade_terms(), 0).expect("pact seals");
    a.treaty_elements(7, "lane-b", vec![listed]).expect("fence seals");
    match charter_element_route(&mut a, FIRST_ORB, &mut b, FIRST_ORB, barred, unit(1, 2), 1, 40) {
        Err(ActionRefused::ElementNotAllowed { .. }) => {}
        other => panic!("a barred lane must refuse at the charter, got {other:?}"),
    }
}

/// SEAL B + resume: the fence replays and resumes byte-for-byte, and
/// the committed state round-trips the codec whole.
#[test]
fn the_fence_survives_replay_resume_and_codec() {
    let mut a = sunlit("carry-a");
    let listed = bank_element(&mut a);
    a.treaty(7, "carry-b", trade_terms(), 0).expect("pact seals");
    a.treaty_elements(7, "carry-b", vec![listed]).expect("fence seals");
    assert_eq!(
        a.committed().element_allowlists,
        vec![(7, "carry-b".to_string(), vec![listed])]
    );
    assert_eq!(replay(a.chronicle()), a.committed(), "SEAL B");
    let resumed = World::resume(a.chronicle().clone());
    assert_eq!(resumed.committed(), a.committed(), "resume carries the fence");
    let bytes = codec::state_bytes(&a.committed());
    let decoded = codec::from_state_bytes(&bytes).expect("v19 decodes");
    assert_eq!(decoded, a.committed(), "codec round-trips the fence");
    // The deed's own wire round-trips byte-exact.
    let deed = ClosureEvent::TreatyElementsAllowed {
        deal: 7,
        partner: "carry-b".into(),
        elements: vec![listed, [8u8; 32]],
    };
    let bytes = deed.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(tag, 165);
    let decoded = ClosureEvent::from_wire(tag, value).expect("decodes");
    assert_eq!(decoded, deed);
    assert_eq!(decoded.wire_bytes(), bytes, "equal values, equal bytes");
}
