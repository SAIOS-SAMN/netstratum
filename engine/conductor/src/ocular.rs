//! **The ocular economy** (OCULAR-1, the no-scalar re-founding, doc 54).
//!
//! The civic layer read its economy through the wrong organ: `substrate::snf`
//! (a Z-module) fed by `quantize` (an exact-rational rate rounded onto a fixed
//! `LATTICE = 64` grain), then collapsed to scalars (`residual_extent`,
//! `caste_convergence`). Each of those loses interpretation. The rest of the
//! engine already reads the world the right way -- as an exact-rational cochain
//! over a cover (`rfa_core::Delta` + `Support`), with the reading a graded field,
//! never a scalar. This module brings the economy onto that organ.
//!
//! The economy is a connection. Node 0 is the environment (the reservoir the
//! population draws from and the world supplies to); nodes `1..=n` are the
//! resource axes. The relation `Delta[0][i]` is the exact-rational net flow
//! between the environment and axis `i`: `supply_i - demand_i`. What the society
//! sees is then the `divergence` of that connection -- at each axis node,
//! `div_i = demand_i - supply_i`, the signed deficit, exact, per axis, with the
//! sign and the direction both intact (a positive divergence is an unmet need,
//! an egress; a negative one is a surplus, an ingress). The books close by
//! antisymmetry (`sum_i div_i = 0`), the economy's conservation law.
//!
//! No `quantize`, no `LATTICE`, no `sum |g_i|`. A sub-1/64 deficit that the old
//! lattice rounded to "covered" survives here as an exact nonzero divergence.
//! This slice is additive -- it changes no live behavior; it stands the new
//! organ up and witnesses it names the same deficit the old path found, without
//! the loss. OCULAR-2..4 migrate the consumers onto it and delete the scalars.
