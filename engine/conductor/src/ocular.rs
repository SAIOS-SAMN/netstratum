//! **THE OCULAR ECONOMY** (OCULAR-1, the no-scalar re-founding, doc 54).
//!
//! The civic layer read its economy through the wrong organ: `substrate::snf`
//! (a Z-module) fed by `quantize` (an exact-rational rate rounded onto a fixed
//! `LATTICE = 64` grain), then collapsed to scalars (`residual_extent`,
//! `caste_convergence`). Each of those loses interpretation. The rest of the
//! engine already reads the world the right way -- as an exact-rational cochain
//! over a cover (`rfa_core::Delta` + `Support`), with the reading a graded field,
//! never a scalar. This module brings the economy onto that organ.
//!
//! The economy is a CONNECTION. Node 0 is the environment (the reservoir the
//! population draws from and the world supplies to); nodes `1..=n` are the
//! resource axes. The relation `Delta[0][i]` is the exact-rational NET FLOW
//! between the environment and axis `i`: `supply_i - demand_i`. What the society
//! SEES is then the `divergence` of that connection -- at each axis node,
//! `div_i = demand_i - supply_i`, the SIGNED deficit, exact, per axis, with the
//! sign and the direction both intact (a positive divergence is an unmet need,
//! an egress; a negative one is a surplus, an ingress). The books close by
//! antisymmetry (`sum_i div_i = 0`), the economy's conservation law.
//!
//! No `quantize`, no `LATTICE`, no `sum |g_i|`. A sub-1/64 deficit that the old
//! lattice rounded to "covered" survives here as an exact nonzero divergence.
//! This slice is ADDITIVE -- it changes no live behavior; it stands the new
//! organ up and witnesses it names the same deficit the old path found, without
//! the loss. OCULAR-2..4 migrate the consumers onto it and delete the scalars.

use cfe::ratio::{ratio, Ratio};
use rfa_core::{divergence, Delta, Support};

/// Build the economy as an exact-rational connection over the resource axes.
/// `demand[i]` and `supply[i]` are the exact per-axis loads (NOT quantized).
/// Node 0 is the environment; axis `i` is node `i + 1`. The single fiber carries
/// the one flow coordinate. `Delta[0][i+1] = supply[i] - demand[i]`.
pub fn economy_connection(demand: &[Ratio], supply: &[Ratio]) -> (Delta, Support) {
    let axes = demand.len().max(supply.len());
    let dim = axes + 1; // +1 for the environment reservoir at node 0
    let fiber = 1;
    let mut delta = Delta::zero(dim, fiber);
    let mut support = Support::new(dim, fiber);
    let zero = ratio(0, 1);
    for a in 0..axes {
        let d = demand.get(a).cloned().unwrap_or_else(|| zero.clone());
        let s = supply.get(a).cloned().unwrap_or_else(|| zero.clone());
        // Net flow from the environment (node 0) INTO the axis (node a + 1).
        // set_antisym writes the (0, a+1) edge and its negated twin in one stroke.
        delta.set_antisym(0, a + 1, vec![s - d]);
        support.connect(0, a + 1, 0);
    }
    (delta, support)
}

/// **WHAT THE SOCIETY SEES** -- the signed deficit as a graded field, exact and
/// per-axis, never a norm. `seen[i] = demand_i - supply_i` (the axis node's
/// divergence): positive is an unmet need, negative a surplus, zero balanced.
/// This is the loss-free replacement for the old quantized `deficit` vector and
/// its `residual_extent` collapse -- the direction and the exact magnitude both
/// survive, and a deficit finer than the retired 1/64 grain is still seen.
pub fn seen_deficit(demand: &[Ratio], supply: &[Ratio]) -> Vec<Ratio> {
    let (delta, _support) = economy_connection(demand, supply);
    let div = divergence(&delta); // [node][fiber]
    // The axis nodes are 1..=axes; their divergence on the single fiber is the
    // signed deficit. (Node 0, the environment, carries the closing balance.)
    let axes = demand.len().max(supply.len());
    (0..axes).map(|a| div[a + 1][0].clone()).collect()
}
