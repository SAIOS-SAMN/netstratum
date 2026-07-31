//! Rational Field Algebra core.
//!
//! Clean-room port of the convergence-engine math core onto the CFE
//! `Ratio` carrier. The objects:
//!
//! - [`Delta`] — an antisymmetric relational tensor over exact rational:
//!   `Δ_ij = −Δ_ji`, zero diagonal, one exact-rational-vector of fiber coordinates
//!   per entry. It encodes RELATIONSHIPS between entities, never the
//!   entities themselves.
//! - [`for_each_residual`] — the ONE computation of the cocycle
//!   condition `R_ijk = Δ_ik − Δ_ij − Δ_jk` over all ordered distinct
//!   triples. Everything downstream (coherence, gates) composes it.
//! - [`coherence_functional`] — `C(Δ) = Σ |R|` over the exact rationals
//!   (THE PROPORTIONAL LAW: squares are barred as aggregators of difference).
//!   `C(Δ) = 0` iff the cocycle condition holds; otherwise the residuals
//!   name exactly what has not converged, and where.
//! - [`coboundary_reduce`] — projection onto the canonical H¹
//!   representative (strips coordinate distortion), then integer
//!   normalization (lcm of denominators, gcd of numerators): the
//!   irreducible form of the relational state.
//! - [`gates`] — the three validation gates every proposed operator
//!   must pass: productivity, conservation, bounded magnitude.

pub mod delta;
pub mod fold;
pub mod gates;
pub mod rcf;
pub mod reduce;

pub use delta::{coherence_functional, for_each_residual, magnitude, Delta};
pub use fold::{f_fold_star, orbit_lock, FoldVerdict, Operator, OrbitWitness, QMatrix};
pub use gates::{validate_step, GateVerdict, StepBound};
pub use rcf::{invariant_factors, rcf_hash, rcf_signature, QPoly, RcfSignature};
pub use reduce::coboundary_reduce;
