//! N-I8 — consilience: a word installs only when TWO distinct vessels close the
//! same orbit (Pin C); one vessel confirming itself never mints (C−). This
//! witnesses the pure deposit/consilience law on a two-key fixture; the live
//! two-office wiring is exercised by the era invariants.

use conductor::consilience_deposit;
use solver::Op;

#[test]
fn pin_c_two_vessels_mint_and_pin_c_minus_one_vessel_does_not() {
    let mut pending = Vec::new();
    let orbit = [1u8; 32];
    let expansion = vec![Op::Rotate90, Op::FlipV];
    let (v, w) = (7u64, 9u64); // two distinct vessels (offices)
    let (d1, d2, d3) = ([10u8; 32], [11u8; 32], [12u8; 32]);

    // C−: the SAME vessel closes the orbit three times — three digests, ONE
    // vantage. It never reaches two distinct vessel_ids, so it never mints.
    assert!(
        consilience_deposit(&mut pending, orbit, &expansion, v, d1).is_none(),
        "one deposit is not consilience",
    );
    assert!(consilience_deposit(&mut pending, orbit, &expansion, v, d2).is_none());
    assert!(
        consilience_deposit(&mut pending, orbit, &expansion, v, d3).is_none(),
        "one vessel, three digests — still no mint (C−, no one-vantage fallback)",
    );

    // C: a SECOND distinct vessel closes the SAME orbit → cited returned to mint.
    // Deliberately reuses `d1` (the SAME digest `v` already deposited) — the
    // shape `research_step` produces every beat (one chronicle digest stamped
    // on both the automaton's and the sibling's deposit). `cited` must carry
    // ONE entry PER DISTINCT VESSEL, never collapse on shared digest content,
    // or `mint_solver_word_as`'s `cited.len() < 2` gate silently starves every
    // same-beat consilience forever.
    let cited = consilience_deposit(&mut pending, orbit, &expansion, w, d1);
    assert_eq!(
        cited.as_ref().map(Vec::len),
        Some(2),
        "two distinct vessels sharing the SAME digest still cite 2 — identity, not content, gates the mint",
    );
    // The helper does NOT remove — the caller clears pending on a successful mint
    // (so a consilient-but-unaffordable word stays pending, fasting).
    assert_eq!(pending.len(), 1, "consilience_deposit reads the wait; the mint clears it");
}

#[test]
fn a_vessel_cannot_manufacture_a_second_vantage_from_itself() {
    // Idempotent per (vessel, digest): the same vessel depositing the same digest
    // twice is ONE deposit. A mind cannot become two by repeating itself.
    let mut pending = Vec::new();
    let orbit = [2u8; 32];
    let expansion = vec![Op::Transpose];
    let v = 3u64;
    let d = [20u8; 32];
    assert!(consilience_deposit(&mut pending, orbit, &expansion, v, d).is_none());
    assert!(
        consilience_deposit(&mut pending, orbit, &expansion, v, d).is_none(),
        "same vessel + same digest twice is still one vantage",
    );
    assert_eq!(pending[0].2.len(), 1, "the duplicate deposit was ignored");
}

#[test]
fn one_vessels_repeated_deposits_stay_bounded_by_vessel_not_by_attempt() {
    // MEASURED LEAK, FIXED 2026-08-24: research_step stamps every deposit
    // with the BEAT'S OWN receipt digest, so a vessel re-confirming an
    // orbit that never reaches consilience carries a NEW digest every beat
    // -- (vessel, digest) never repeats even though it's the same vessel.
    // Appending one entry per attempt grew this WITHOUT BOUND for any
    // orbit only one vessel ever visits. Only vessel diversity gates the
    // mint, so one vessel's repeated attempts must occupy exactly one slot
    // -- bounded by vessel count, not by how many beats it kept trying.
    let mut pending = Vec::new();
    let orbit = [5u8; 32];
    let expansion = vec![Op::Rotate90];
    let v = 11u64;
    for beat in 0..500u32 {
        let digest = *blake3::hash(&beat.to_le_bytes()).as_bytes();
        assert!(consilience_deposit(&mut pending, orbit, &expansion, v, digest).is_none());
    }
    assert_eq!(
        pending[0].2.len(),
        1,
        "500 beats of one vessel re-confirming still occupies exactly one slot"
    );
}
