//! Rational Field Algebra core.
//!
//! Clean-room port of the convergence-engine math core onto the CFE
//! `Ratio` carrier. The objects:
//!
//! - [`Delta`] — an antisymmetric relational tensor over exact rational:
//!   `Δ_ij = −Δ_ji`, zero diagonal, one exact-rational-vector of fiber coordinates
//!   per entry. It encodes relationships between entities, never the
//!   entities themselves.
//! - [`for_each_residual`] — the one computation of the cocycle
//!   condition `R_ijk = Δ_ik − Δ_ij − Δ_jk` over all ordered distinct
//!   triples. Everything downstream (coherence, gates) composes it.
//! - [`coherence_functional`] — `C(Δ) = Σ |R|` over the exact rationals
//!   (The proportional law: squares are barred as aggregators of difference).
//!   `C(Δ) = 0` iff the cocycle condition holds — that and nothing more.
//!   **C has no operational function** (operator ruling 2026-08-01): it
//!   is not a score, not a gauge, not a gate, and no engine path
//!   consults it. It is an L1 collapse for an operator lens only. The word
//!   "converged" is struck from this crate — zero says the relations
//!   compose, never that anything was finished or emptied.
//! - [`coboundary_reduce`] — projection onto the canonical H¹
//!   representative (strips coordinate distortion), then integer
//!   normalization (lcm of denominators, gcd of numerators): the
//!   irreducible form of the relational state.
//! - [`gates`] — the three validation gates every proposed operator
//!   must pass: productivity, conservation, bounded magnitude.

pub mod boundary;
pub mod cage;
pub mod capacity;
pub mod delta;
pub mod fiber;
pub mod fold;
pub mod gates;
pub mod rcf;
pub mod reduce;

pub use cage::{consult, Exact, Lens, Proposal};
pub use capacity::{
    capacity, circulation, divergence, ext_x, flux, ground, preserves_hodge_split, Degree, Extent,
    Flux,
};
pub use delta::{coherence_functional, for_each_residual, magnitude, Delta};
pub use fiber::{loop_holonomy, plaquette_curvature, MatrixFiber2x2};
pub use fold::{
    f_fold_star, gated_fold, orbit_lock, verify_orbit_witness, FoldReading, Operator, OrbitWitness,
    RatMatrix, TorsionCause,
};
pub use gates::{validate_step, GateReading, StepBound};
// The four-row classification is struck (operator ruling 2026-08-04):
// `Vacuity` and `dual_pole` are gone with the capacity law that was
// their only gate. The boundary form stays — the divergence is a
// first-class reading for the sign law and is what `capacity.rs` splits
// against.
pub use boundary::{boundary_form, BoundaryForm, Flow};
pub use rcf::{
    invariant_factors, rcf_hash, rcf_hash_skew_bipartite, rcf_signature, RatPoly, RcfSignature,
};
pub use reduce::{coboundary_reduce, forest_potentials, holonomies, Holonomy, Support};
// The carrier, re-exported. A crate that builds a Delta through this
// algebra must name its exact-rational entries; re-exporting `Ratio`
// here lets a crate above rfa-core (and below `cfe` in the arrow law,
// like `solver`) do so through the algebra it already depends on,
// without a direct edge to `cfe`.
pub use cfe::ratio::{ratio, Ratio};
