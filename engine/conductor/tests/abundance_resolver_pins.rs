//! **COG-EMERGENCE-2a — THE ABUNDANCE RESOLVER.** Above the survival floor
//! (`express_survival`, the sole zone-builder — N-I6), a line with surplus
//! chooses ONE act — deepen, expand, synthesize, or hoard — by a single fold
//! draw, NOT three authored `_step` gates. These witnesses pin the two
//! properties the mandate demands of that draw: it leans with TENSION (the
//! deficit γ), and it DIVERGES on HISTORY (the seed) — two lines in the same
//! surplus can choose differently. And they pin the demotion of affordability:
//! the resolver takes no budget, so the will is topological, physics judges it
//! after. (In `tests/`, off the tuned-constant ratchet.)

use conductor::soc::{investment_weights, resolve_investment, AlgebraicRank, Investment};

fn seed(n: u8) -> [u8; 32] {
    *blake3::hash(&[n]).as_bytes()
}

/// A line with some reach to spend (β₀=2 capability components + β₁=1 recycling
/// cycle → dimension 3). The shared reach cancels in the draw, so its exact
/// value only scales the weights, never the choice.
fn reach() -> AlgebraicRank {
    AlgebraicRank { components: 2, cycles: 1, relations: vec![] }
}

const DEEPEN: usize = 0;
const EXPAND: usize = 1;
const SYNTHESIZE: usize = 2;
const HOARD: usize = 3;

#[test]
fn at_ease_the_menu_is_uniform_the_line_explores() {
    // No tension anywhere ⇒ every act (including Hoard, the untilted floor)
    // weighs the same. The draw is a uniform explore, not a fixed default.
    let w = investment_weights(&reach(), 0, 0, 0, 0);
    assert_eq!(w[DEEPEN], w[HOARD]);
    assert_eq!(w[EXPAND], w[HOARD]);
    assert_eq!(w[SYNTHESIZE], w[HOARD]);
    assert!(w[HOARD] > 0, "a line always has reach to spend");
}

#[test]
fn a_uniform_menu_reaches_every_act_it_does_not_collapse_to_one() {
    // EXPLORE, witnessed: at ease, sweeping the line's histories reaches all
    // four acts — the surplus choice is a genuine draw over the manifold, not
    // an authored winner. (Two civs, same ease, different builds.)
    let mut seen = [false; 4];
    for n in 0..64u8 {
        let act = resolve_investment(&reach(), 0, 0, 0, 0, seed(n));
        seen[Investment::MENU.iter().position(|a| *a == act).unwrap()] = true;
    }
    assert!(seen.iter().all(|&s| s), "a uniform menu must reach every act, {seen:?}");
}

#[test]
fn a_cognitive_tension_leans_the_menu_toward_deepening() {
    // The cognitive γ tilts ONLY Deepen; the others stay at the floor.
    let w = investment_weights(&reach(), 40, 0, 0, 0);
    assert!(w[DEEPEN] > w[HOARD], "the void pulls the mind's act above the floor");
    assert_eq!(w[EXPAND], w[HOARD], "an untilted act stays at the floor");
    assert_eq!(w[SYNTHESIZE], w[HOARD]);
}

#[test]
fn the_lean_rises_monotonically_with_the_tension() {
    // Deeper void ⇒ heavier pull toward Deepen — a continuous tension, not a
    // threshold. (Monotone in the γ, exactly like the zone tilt.)
    let mild = investment_weights(&reach(), 10, 0, 0, 0)[DEEPEN];
    let sharp = investment_weights(&reach(), 100, 0, 0, 0)[DEEPEN];
    assert!(sharp > mild, "a sharper void leans harder toward deepening");
}

#[test]
fn each_tension_leans_its_own_act() {
    // The three tensions are independent axes — each raises only its act.
    let expand = investment_weights(&reach(), 0, 40, 0, 0);
    assert!(expand[EXPAND] > expand[HOARD]);
    assert_eq!(expand[DEEPEN], expand[HOARD]);

    let synth = investment_weights(&reach(), 0, 0, 40, 0);
    assert!(synth[SYNTHESIZE] > synth[HOARD]);
    assert_eq!(synth[DEEPEN], synth[HOARD]);
}

#[test]
fn two_histories_diverge_on_the_same_surplus() {
    // DESIGN, NOT CONVERGENCE: identical reach, identical tensions — the only
    // difference is the line's chronicle digest (its history). Sweeping seeds,
    // the outcome is NOT a pure function of the situation: different lines make
    // different choices. If a single winner always dominated, this would be a
    // gate, not a draw.
    let mut seen = [false; 4];
    for n in 0..32u8 {
        // A non-degenerate menu (Deepen tilted, but the floor still competes).
        let act = resolve_investment(&reach(), 20, 0, 0, 0, seed(n));
        seen[Investment::MENU.iter().position(|a| *a == act).unwrap()] = true;
    }
    let distinct = seen.iter().filter(|&&s| s).count();
    assert!(distinct >= 2, "the same surplus must admit divergent choices, got {seen:?}");
}

#[test]
fn the_resolver_consults_no_budget_the_will_is_topological() {
    // AFFORDABILITY DEMOTED: the resolver's inputs are reach + tensions + seed —
    // no resource stock, no leisure, no iron. A line under a dominating cognitive
    // void chooses to Deepen whether or not it can pay; physics rejects the
    // attempt AFTER the fact. This test can only pass because the choice is
    // decoupled from any budget — the signature itself is the proof, and here we
    // show the tension carries the draw with nothing else supplied.
    let mut deepen = 0;
    for n in 0..64u8 {
        // γ far above the lattice floor ⇒ Deepen dominates the mass.
        if resolve_investment(&reach(), 10_000, 0, 0, 0, seed(n)) == Investment::Deepen {
            deepen += 1;
        }
    }
    assert!(deepen > 60, "a dominating void carries the will, no budget consulted ({deepen}/64)");
}
