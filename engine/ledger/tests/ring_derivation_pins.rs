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
//!
//! **RESOLVED (ruling 2026-08-23).** `STATIONS` grew 4 → 10 (Wave-1,
//! 2026-08-07, `RING_ARITIES` updated to match) then 10 → 16 (T10-T15,
//! 2026-08-08, `RING_ARITIES` NOT updated — the drift these tests exist
//! to catch) then 16 → 18 (elder/ancient, 2026-08-13). This file went
//! red naming the fork: bump `TURN_SLOTS` 60 → 180 (cross-cutting —
//! `solver-verify/src/shell.rs`'s orbital shell geometry and
//! `shell_pins.rs` hard-assert `TURN_SLOTS == 60`), or drop the
//! requirement that was never load-bearing for stations. `STATIONS` is
//! an append-only OFFICE/ROLE taxonomy, not a population partition any
//! live code lays out on the turn ring (`STATIONS.len()` appeared only
//! in this file's own arrays, never in a ring-placement call) — unlike
//! dispositions/metabolisms/perceptions, which the incursion generator
//! genuinely draws uniformly. Sixty stands on `lcm(3,4,5)` alone now,
//! the base-60 orbital ring `ratio::TURN_SLOTS` already documents
//! (twelve exact subdivisions — the reason base 60 has always been
//! used), unmoved by however many offices the game later names.

use ledger::registry::{DISPOSITIONS, METABOLISMS, PERCEPTIONS};

/// **EVERY CLOSED VOCABULARY'S ARITY IS ONE THE RING ADMITS.**
///
/// This is the requirement itself: a population must be layoutable
/// symmetrically by any partition it actually has. If a vocabulary grows
/// to an arity the ring cannot divide, this fails and names it — which
/// is the day `RING_ARITIES` needs the new number.
#[test]
fn every_live_vocabulary_divides_the_ring() {
    let live: [(&str, i64); 4] = [
        ("dispositions", DISPOSITIONS.len() as i64),
        ("metabolisms", METABOLISMS.len() as i64),
        ("perceptions", PERCEPTIONS.len() as i64),
        // PRACTICE-1's five receipted disciplines (in-value codes 1..=5).
        ("practices", 5),
        // STATIONS is deliberately absent — see the file header: an
        // append-only office/role taxonomy, never a population
        // partition any live code lays out on the turn ring.
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
/// `RING_ARITIES` is `{3, 4, 5}`; the vocabularies are 3, 3, 4, 5
/// (stations is deliberately excluded — see the file header). Duplicates
/// do not change an lcm, so the set is what matters — and this is the
/// witness that the set is a reading of the tree rather than a guess
/// that happens to agree.
#[test]
fn the_stated_arities_are_the_live_ones() {
    let mut live: Vec<i64> = vec![
        DISPOSITIONS.len() as i64,
        METABOLISMS.len() as i64,
        PERCEPTIONS.len() as i64,
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
