//! **THE RING IS DERIVED FROM THE ENGINE'S OWN VOCABULARIES** (operator
//! ruling 2026-08-05: *all of the constants here need to be derived
//! dynamically*).
//!
//! `ratio::TURN_SLOTS` is `lcm(RING_ARITIES)`, and `RING_ARITIES` states
//! the arities a population must be layoutable at. The vocabularies live
//! ABOVE `ratio` — `ledger` and `cfe` both depend on it, never the
//! reverse — so the requirement is stated down there and PROVED here,
//! against the live lists. Copying the numbers down would be the
//! two-truths defect; stating and proving is not.

use ledger::registry::{DISPOSITIONS, METABOLISMS, PERCEPTIONS, STATIONS};

/// **EVERY CLOSED VOCABULARY'S ARITY IS ONE THE RING ADMITS.**
///
/// This is the requirement itself: a population must be layoutable
/// symmetrically by any partition it actually has. If a vocabulary grows
/// to an arity the ring cannot divide, this fails and names it — which
/// is the day `RING_ARITIES` needs the new number.
#[test]
fn every_live_vocabulary_divides_the_ring() {
    let live: [(&str, i64); 5] = [
        ("dispositions", DISPOSITIONS.len() as i64),
        ("metabolisms", METABOLISMS.len() as i64),
        ("perceptions", PERCEPTIONS.len() as i64),
        ("stations", STATIONS.len() as i64),
        // PRACTICE-1's five receipted disciplines (in-value codes 1..=5).
        ("practices", 5),
    ];
    for (name, arity) in live {
        let proof = ratio::divide_ring(ratio::TURN_SLOTS, arity);
        assert!(
            proof.holds(),
            "the {name} vocabulary has arity {arity} and the ring cannot divide by it — \
             {} — so RING_ARITIES is out of date",
            proof.because()
        );
    }
}

/// **AND THE RING IS EXACTLY THE LCM OF THEM — no larger.**
///
/// A ring that merely admits the vocabularies could be any multiple.
/// This pins that it is the SMALLEST such ring, so sixty is derived
/// rather than chosen and cannot quietly become a hundred and twenty.
#[test]
fn the_ring_is_the_least_that_admits_them_all() {
    let live: Vec<i64> = vec![
        DISPOSITIONS.len() as i64,
        METABOLISMS.len() as i64,
        PERCEPTIONS.len() as i64,
        STATIONS.len() as i64,
        5,
    ];
    assert_eq!(
        ratio::lcm_of(&live),
        ratio::TURN_SLOTS,
        "the live vocabularies' lcm is not the ring — RING_ARITIES has drifted \
         from what the engine actually carries"
    );

    // And nothing smaller does it.
    for candidate in 1..ratio::TURN_SLOTS {
        assert!(
            !live.iter().all(|arity| candidate % arity == 0),
            "{candidate} also admits every live vocabulary — the ring is not minimal"
        );
    }
}

/// **THE STATED ARITIES ARE THE LIVE ONES, as a SET.**
///
/// `RING_ARITIES` is `{3, 4, 5}`; the vocabularies are 3, 3, 4, 4, 5.
/// Duplicates do not change an lcm, so the set is what matters — and
/// this is the witness that the set is a reading of the tree rather
/// than a guess that happens to agree.
#[test]
fn the_stated_arities_are_the_live_ones() {
    let mut live: Vec<i64> = vec![
        DISPOSITIONS.len() as i64,
        METABOLISMS.len() as i64,
        PERCEPTIONS.len() as i64,
        STATIONS.len() as i64,
        5,
    ];
    live.sort_unstable();
    live.dedup();
    let mut stated = ratio::RING_ARITIES.to_vec();
    stated.sort_unstable();
    assert_eq!(
        live, stated,
        "the arities the ring was derived from are not the arities the engine carries"
    );
}
