//! **THE PRACTICE MECHANISMS ARE DATA** (operator ruling 2026-08-05).
//!
//! Five disciplines, five bindings, zero bespoke branches. These pins
//! hold three things: that every seed practice binding TRADES (§4a is
//! now PROVABLE on the practices, not claimed of them), that the data
//! carries exactly the factors the dead branches applied (SEAL B: a
//! world that adopted a practice before this landing evolves
//! identically after it), and that the gates are explicit entries —
//! silence is no gate.
//!
//! The behavioral half of the equivalence lives where it always did:
//! `world_practice.rs` pins the exact post-adoption prices, and it
//! passed unchanged across the landing.

use cfe::ratio::ratio;
use cfe::vocabulary::{
    binding_trades, canonical, factor_on, gate_of, seed, Axis, CHANNEL_ALIGN,
    CHANNEL_CONVERTER_YIELD, CHANNEL_DEMURRAGE, CHANNEL_EXTRACTOR_UPKEEP, CHANNEL_HABITAT_UPKEEP,
    CHANNEL_HABITAT_YIELD, CHANNEL_SURVEY, CHANNEL_TEMPER, CHANNEL_UPKEEP, CHANNEL_YIELD,
};

fn practice(code: u8) -> Vec<(u8, cfe::ratio::Ratio)> {
    seed(Axis::Practice)
        .into_iter()
        .find(|entry| entry.code == code)
        .expect("a seed practice")
        .binding
}

/// **EVERY DISCIPLINE TRADES** — one benefit, one friction, in bounds,
/// on landed channels. The module doc claimed this of the five; now
/// the door proves it of the data.
#[test]
fn every_seed_practice_binding_trades() {
    for code in 1u8..=5 {
        assert!(
            binding_trades(&practice(code)),
            "practice {code} carries a binding that does not trade"
        );
    }
}

/// **THE DATA IS THE DEAD BRANCHES, EXACTLY** — factor for factor.
/// This is the SEAL B half of the landing: same adoption, same world.
#[test]
fn the_bindings_carry_the_legacy_factors_exactly() {
    // austerity: habitat yield 3/4, extractor upkeep 3/4.
    let austerity = practice(1);
    assert_eq!(factor_on(&austerity, CHANNEL_HABITAT_YIELD), ratio(3, 4));
    assert_eq!(factor_on(&austerity, CHANNEL_EXTRACTOR_UPKEEP), ratio(3, 4));
    // showpiece: habitat yield 5/4, habitat upkeep 2.
    let showpiece = practice(2);
    assert_eq!(factor_on(&showpiece, CHANNEL_HABITAT_YIELD), ratio(5, 4));
    assert_eq!(factor_on(&showpiece, CHANNEL_HABITAT_UPKEEP), ratio(2, 1));
    // restraint: the survey draw halves.
    assert_eq!(factor_on(&practice(3), CHANNEL_SURVEY), ratio(1, 2));
    // hoard: converter yield 3/4, demurrage decay halves.
    let hoard = practice(4);
    assert_eq!(factor_on(&hoard, CHANNEL_CONVERTER_YIELD), ratio(3, 4));
    assert_eq!(factor_on(&hoard, CHANNEL_DEMURRAGE), ratio(1, 2));
    // overdrive: everything runs hot — global 5/4 on both.
    let overdrive = practice(5);
    assert_eq!(factor_on(&overdrive, CHANNEL_YIELD), ratio(5, 4));
    assert_eq!(factor_on(&overdrive, CHANNEL_UPKEEP), ratio(5, 4));
}

/// **THE GATES ARE EXPLICIT ENTRIES.** Every discipline binds the
/// alignment floor at 1/2 (any adopted practice holds the climb to
/// the syntropy floor — the friction the whole family shares); the
/// Overdrive alone binds the tempering threshold at 8; and silence is
/// NO gate, never a gate of one.
#[test]
fn the_gates_read_from_the_data_alone() {
    for code in 1u8..=5 {
        assert_eq!(
            gate_of(&practice(code), CHANNEL_ALIGN),
            Some(ratio(1, 2)),
            "practice {code} must hold the climb to the floor"
        );
    }
    assert_eq!(gate_of(&practice(5), CHANNEL_TEMPER), Some(ratio(8, 1)));
    for code in 1u8..=4 {
        assert_eq!(
            gate_of(&practice(code), CHANNEL_TEMPER),
            None,
            "practice {code} demands no tempering — silence is no gate"
        );
    }
}

/// **NO TWO DISCIPLINES ARE THE SAME MECHANISM** — the novelty law
/// (§6), asserted on the practice seed exactly as it is asserted on
/// the archetype seed, before anything is allowed to mint against it.
#[test]
fn no_two_practices_share_a_mechanism() {
    let shapes: Vec<Vec<(u8, cfe::ratio::Ratio)>> =
        (1u8..=5).map(|code| canonical(&practice(code))).collect();
    for (at, shape) in shapes.iter().enumerate() {
        assert!(
            !shapes[at + 1..].contains(shape),
            "practice {} computes as a later practice — a partition \
             that distinguishes nothing",
            at + 1
        );
    }
}
