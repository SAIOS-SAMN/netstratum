//! The elasticity witness (substrate contract §5): the full loop —
//! a coherent colony, a shock injecting torsion, the stress gauge
//! reading non-zero AND naming where, gated recovery steps strictly
//! reducing stress, and convergence back to the fixed point.
//!
//! This is the stochastic-elasticity gameplay mechanic run end-to-end
//! on the bare substrate: no game vocabulary, just the math the
//! dashboard will render.

use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;
use rfa_core::{
    coherence_functional, for_each_residual, magnitude, validate_step, Delta, GateVerdict,
    StepBound,
};

/// A 3-entity colony in exact equilibrium (the cocycle condition
/// holds): Δ01 = 1, Δ12 = 1/2, Δ02 = 3/2.
fn equilibrium() -> Delta {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![ratio(1, 1)]);
    d.set_antisym(1, 2, vec![ratio(1, 2)]);
    d.set_antisym(0, 2, vec![ratio(3, 2)]);
    d
}

/// A colony with the 0↔2 relation shocked to `d02`.
fn shocked(d02: Ratio) -> Delta {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![ratio(1, 1)]);
    d.set_antisym(1, 2, vec![ratio(1, 2)]);
    d.set_antisym(0, 2, vec![d02]);
    d
}

#[test]
fn shock_stress_gated_recovery_convergence() {
    // 1. EQUILIBRIUM — the stress gauge reads exactly zero.
    let colony = equilibrium();
    assert!(coherence_functional(&colony).is_zero());

    // 2. THE SHOCK — a meteor severs the 0↔2 trade relation: Δ02
    //    jumps 3/2 → 7/2 (torsion +2). The gauge reads non-zero…
    let hit = shocked(ratio(7, 2));
    let stress = coherence_functional(&hit);
    assert_eq!(stress, ratio(12, 1), "six ordered triples × |±2| — the proportional law");

    //    …and the residuals NAME the damaged relation: every firing
    //    triple involves all three entities around the broken 0↔2 leg.
    let mut fired = 0;
    for_each_residual(&hit, |i, j, k, _l, r| {
        fired += 1;
        assert!(r == &ratio(2, 1) || r == &ratio(-2, 1));
        let mut idx = [i, j, k];
        idx.sort_unstable();
        assert_eq!(idx, [0, 1, 2]);
    });
    assert_eq!(fired, 6);

    // 3. GATED RECOVERY — the colony re-equilibrates tick by tick.
    //    Each step passes the TWO gates: conservation, and the
    //    two-sided magnitude bound. (The productivity gate is STRUCK —
    //    see step 5.)
    let bound = StepBound {
        reference: magnitude(&hit),
        epsilon: ratio(1, 1),
    };
    let path = [ratio(5, 2), ratio(2, 1), ratio(3, 2)]; // Δ02 healing home
    let mut current = hit;
    let mut last_stress = stress;
    for d02 in path {
        let next = shocked(d02);
        assert_eq!(
            validate_step(&current, &next, &bound),
            GateVerdict::Accepted,
            "a lawful recovery step refused"
        );
        // C is READ here, and governs nothing. That this path happens to
        // fall is an OBSERVATION about this path, never a requirement
        // the gate imposed.
        let now = coherence_functional(&next);
        assert!(now < last_stress, "this particular path does fall — observed, not required");
        last_stress = now;
        current = next;
    }

    // 4. CONVERGENCE — home again, exactly: the gauge reads zero and
    //    the recovered colony IS the equilibrium, byte-for-byte.
    assert!(last_stress.is_zero());
    assert_eq!(current, equilibrium());

    // 5. THE LINEAGE IS NOT BOUND TO C's LANE (operator ruling
    //    2026-08-01, the struck productivity gate). A step moving AWAY
    //    from equilibrium (Δ02: 3/2 → 4) is now ACCEPTED. The struck
    //    gate refused exactly this as `Unproductive`, which confined
    //    every lineage to one direction along one scalar axis and
    //    foreclosed every other branch it could open.
    //
    //    A scalar being uni-directional is fine FOR THE SCALAR. Binding
    //    the lineage to it is the fault.
    let overshoot = shocked(ratio(4, 1));
    assert_eq!(
        validate_step(&current, &overshoot, &bound),
        GateVerdict::Accepted,
        "motion away from C's minimum is lawful — the lineage is n-dimensional"
    );
    // C still reads the rise. It simply governs nothing.
    assert!(
        coherence_functional(&overshoot) > coherence_functional(&current),
        "C observes the excursion — an inert reading with no agency"
    );
}
