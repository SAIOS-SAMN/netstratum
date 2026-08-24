//! ENTITY-ALIGN-2 (docs/56 §4, piece a) — the sibling occasionally chooses
//! its OWN problem instead of confirming the automaton's. Two pure pieces:
//! `basin_digest` (the seed a self-selecting vessel derives from, its own
//! basin, never the civ digest) and `sibling_self_select_beat` (the
//! hash-seeded gate deciding WHICH beats are self-select beats).

use conductor::{basin_digest, sibling_self_select_beat};
use reduct::peel::CellFrame;
use reduct::ChainDelta;
use solver::Crystal;

fn frame(rows: u32) -> CellFrame {
    CellFrame { rows, cols: 1, layers: 1 }
}

#[test]
fn an_empty_basin_has_no_digest_to_derive_from() {
    assert_eq!(basin_digest(None), None, "no basin at all — nothing grown yet");
    assert_eq!(
        basin_digest(Some(&Vec::new())),
        None,
        "an empty basin is the same as none — autonomy is earned, not free"
    );
}

#[test]
fn a_grown_basin_derives_a_deterministic_digest_from_its_latest_crystal() {
    let basin = vec![Crystal::seal(ChainDelta::empty(frame(1)))];
    let a = basin_digest(Some(&basin));
    let b = basin_digest(Some(&basin));
    assert!(a.is_some(), "a non-empty basin always derives a digest");
    assert_eq!(a, b, "the same basin state always derives the same digest — replay-exact");
}

#[test]
fn distinct_basin_states_derive_distinct_digests() {
    let basin_a = vec![Crystal::seal(ChainDelta::empty(frame(1)))];
    let basin_b = vec![Crystal::seal(ChainDelta::empty(frame(2)))];
    assert_ne!(
        basin_digest(Some(&basin_a)),
        basin_digest(Some(&basin_b)),
        "different knowledge derives a different problem seed"
    );
}

#[test]
fn a_grown_basin_never_derives_the_civ_chronicle_digest() {
    // The whole point of ENTITY-ALIGN-2: the seed comes from the vessel's OWN
    // basin, not any externally-supplied civ digest — so a basin_digest never
    // needs (and cannot be handed) the civ's chronicle hash to compute.
    let basin = vec![Crystal::seal(ChainDelta::empty(frame(1)))];
    let civ_digest = [42u8; 32];
    assert_ne!(
        basin_digest(Some(&basin)),
        Some(civ_digest),
        "the derived seed is never coincidentally the civ digest"
    );
}

#[test]
fn self_select_beats_are_deterministic_and_replay_exact() {
    let (sibling, beat) = (7u64, 1000u64);
    assert_eq!(
        sibling_self_select_beat(sibling, beat, 4),
        sibling_self_select_beat(sibling, beat, 4),
        "the same (vessel, beat, stride) always reads the same verdict"
    );
}

#[test]
fn self_select_beats_land_at_roughly_one_in_stride() {
    let stride = 4u64;
    let sibling = 11u64;
    let hits = (0u64..4000)
        .filter(|&beat| sibling_self_select_beat(sibling, beat, stride))
        .count();
    // A hash-seeded gate, not an exact fraction — a wide tolerance band
    // around the expected 1/4 (1000 of 4000) proves it fires "occasionally,"
    // not "never" and not "usually."
    assert!(
        (700..1300).contains(&hits),
        "expected roughly 1/{stride} of beats to self-select, got {hits}/4000"
    );
}

#[test]
fn different_vessels_land_on_different_self_select_beats() {
    // Emergent, not a synchronized global tick: two siblings should NOT
    // self-select on identical beat sets (unlike AUTIC_STRIDE's
    // `is_multiple_of`, which fires the same beats for every civ).
    let stride = 4u64;
    let (a, b) = (3u64, 5u64);
    let beats_a: Vec<u64> = (0u64..200).filter(|&beat| sibling_self_select_beat(a, beat, stride)).collect();
    let beats_b: Vec<u64> = (0u64..200).filter(|&beat| sibling_self_select_beat(b, beat, stride)).collect();
    assert_ne!(beats_a, beats_b, "distinct vessels self-select on distinct beats");
}
