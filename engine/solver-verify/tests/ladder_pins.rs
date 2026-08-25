//! THE LADDER READS ITSELF — `Difficulty::climbs` (operator ruling
//! 2026-08-04, replacing `level < 5` at its call sites).
//!
//! The load-bearing pin is `climbs_is_exactly_what_the_constant_said`:
//! the refactor must be provably behaviour-preserving, or it is a
//! change wearing a cleanup's name.

use solver_verify::{Difficulty, VolDifficulty};

/// THE EQUIVALENCE. `climbs(n)` and the struck `n < 5` agree on every
/// rung, so replacing one with the other cannot move a verdict.
#[test]
fn climbs_is_exactly_what_the_constant_said() {
    for level in 0..12u32 {
        assert_eq!(
            Difficulty::climbs(level),
            level < 8,
            "rung {level}: the derived ceiling must agree with saturation (word_len climbs through 6)"
        );
    }
}

/// A PLATEAU IS NOT A CEILING — the pin that caught the first
/// derivation. Rungs 0 and 1 are the same opening rung, so asking only
/// `level(n+1) != level(n)` reported "at the top" at the BOTTOM.
/// `climbs` compares against saturation instead.
#[test]
fn the_doubled_opening_rung_is_not_mistaken_for_the_top() {
    assert_eq!(Difficulty::level(0), Difficulty::level(1), "one opening rung, twice");
    assert!(Difficulty::climbs(0), "the ladder rises above 0 — just not at 1");
    assert!(Difficulty::climbs(1));
}

/// AND IT IS DERIVED, not restated. The ladder saturates because
/// `level`'s catch-all arm makes every rung at or above the top
/// identical — `climbs` reads that, it does not remember it.
#[test]
fn the_ladder_saturates_and_that_is_where_the_climb_stops() {
    assert_eq!(
        Difficulty::level(8),
        Difficulty::level(9),
        "the catch-all arm makes 9 structurally identical to 8"
    );
    assert_eq!(Difficulty::level(9), Difficulty::level(60), "and 60, and every rung after");
    assert!(!Difficulty::climbs(8), "so nothing rises above 8");
    assert!(Difficulty::climbs(7), "and 8 does rise above 7");
}

/// EVERY RUNG BELOW THE TOP GENUINELY DIFFERS — otherwise `climbs`
/// would stop early and a line would freeze mid-ladder. This is the
/// property the derivation depends on, so it is pinned rather than
/// assumed.
#[test]
fn no_two_adjacent_rungs_below_the_top_are_equal() {
    for level in 0..8u32 {
        if level == 0 {
            // 0 and 1 are deliberately the same opening rung.
            assert_eq!(Difficulty::level(0), Difficulty::level(1));
            continue;
        }
        assert_ne!(
            Difficulty::level(level),
            Difficulty::level(level + 1),
            "rung {level} must differ from {} or the climb stalls",
            level + 1
        );
    }
}

/// The volumetric ladder saturates too. It has no `climbs` yet because
/// nothing reads a volumetric ceiling — recorded so the asymmetry is
/// deliberate rather than an oversight.
#[test]
fn the_volumetric_ladder_saturates_on_the_same_shape() {
    assert_eq!(VolDifficulty::level(8), VolDifficulty::level(9));
    assert_ne!(VolDifficulty::level(7), VolDifficulty::level(8));
}

/// **THE RUNG IS THE PRICING KEY, AND THE MATRIX IS NOT.**
///
/// Every ladder shape reports its position; anything off the ladder
/// reports none. That `None` is the anti-self-pricing defence — a
/// payload that invents a convenient shape is off-ladder and prices at
/// zero, exactly as when the whole struct was matched.
#[test]
fn every_ladder_shape_knows_its_rung_and_nothing_else_does() {
    for level in 0..=Difficulty::top() {
        assert_eq!(
            Difficulty::level(level).rung(),
            Some(if level == 1 { 0 } else { level }),
            "rung {level} reports its own position (0 and 1 are one opening rung)"
        );
    }
    // A shape the ladder never issued: off-ladder, so it prices zero.
    let invented = Difficulty { rows: 9, cols: 9, ..Difficulty::level(5) };
    assert_eq!(invented.rung(), None, "an invented shape is on no rung");

    // And the sharp case: the ATTACKER shape — a level-5 tier and word
    // length with a trivial grid, which is what naming your own price
    // would look like.
    let cheap = Difficulty { rows: 1, cols: 1, ..Difficulty::level(5) };
    assert_eq!(cheap.rung(), None, "a cheap shape wearing a hard rung's tier is off-ladder");
}

/// The volumetric ladder answers the same question with no field in
/// common — which is the whole reason a POSITION is the right key and
/// a shape is not.
#[test]
fn the_volumetric_ladder_reports_positions_too() {
    for level in 0..=VolDifficulty::top() {
        assert!(
            VolDifficulty::level(level).rung().is_some(),
            "volumetric rung {level} reports a position"
        );
    }
    let invented = VolDifficulty { layers: 9, ..VolDifficulty::level(4) };
    assert_eq!(invented.rung(), None, "off-ladder in three dimensions too");
}
