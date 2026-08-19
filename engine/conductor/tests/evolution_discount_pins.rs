//! **COG-11 — EVOLUTION AS EFFICIENCY: think more on less.** A line that has
//! advanced its `evolution_stage` (the EARNED transformation deed) pays less
//! leisure per thought. The discount is `1/(1 + stage)` — exactly 1 at the
//! founding stage, so an un-evolved line is byte-identical, and monotone cheaper
//! each earned stage. (Kept in `tests/`, off the tuned-constant ratchet.)

use cfe::ratio::{ratio, Ratio};
use conductor::civic::cognitive_efficiency;

#[test]
fn evolution_discounts_the_cost_of_thought() {
    // Stage 0 — every founding line — pays FULL price: the discount touches only
    // what a line has earned, so the un-evolved board is byte-identical.
    assert_eq!(cognitive_efficiency(0), ratio(1, 1), "an un-evolved line pays the same");
    // Each earned stage makes thought cheaper: stage 1 half, stage 2 a third.
    assert_eq!(cognitive_efficiency(1), ratio(1, 2), "stage 1 thinks at half the cost");
    assert_eq!(cognitive_efficiency(2), ratio(1, 3), "stage 2 a third");

    // Monotone decreasing, always positive: a strong but BOUNDED discount — an
    // evolved line thinks more on less, but thought is never free.
    let mut prev: Ratio = cognitive_efficiency(0);
    for stage in 1u8..=12 {
        let e = cognitive_efficiency(stage);
        assert!(e < prev, "cheaper each earned stage: {e} < {prev} at stage {stage}");
        assert!(e > ratio(0, 1), "thought is never free: {e} at stage {stage}");
        prev = e;
    }
}
