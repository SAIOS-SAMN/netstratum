//! # substrate — the exact-rational algebra
//!
//! The mathematical ground of IPSAL/AUTIC v0.4: smooth-prime rational
//! fields, polynomial-per-plane carriers, the {S,G,T} block-diagonal
//! volume, horizon bounds, and the singularity mirror. Nothing here
//! cognizes and nothing here names a modality (LAW III — the kernel is
//! general).
//!
//! B1 — THE SUBSTRATE LIFT (allowlist record, each module named; 22 B1):
//! * `rational` — Field trait, Space/Temporal/Gauge, `Rational<F>`,
//!   `OverflowError`, `lcm_smooth_primes`
//! * `vector_module` — `VectorModule<F>` polynomial-per-plane carrier + the
//!   F[x] ring surface (div_rem, ring gcd, pseudo-remainder, primitive
//!   part, Cauchy bound)
//! * `multivector` — `PlaneIndex`/`PairPlane`, `Multivector<F>` (CGA
//!   signature (p+1,1), geometric product, reverse, the e₀-split unipotent
//!   foundation)
//! * `volume` — `Volume` {S,G,T} triple + `BlockDiagonalOperator`
//! * `horizon` — `HorizonContext` + Δ_min/Λ_max thresholds
//! * `mirror` — `SingularityMirror<F>` bounded inversion
//! * `wire` — BigInt varint bounded-binary helpers (06 §5)
//!
//! LEFT BEHIND by allowlist (named per the landing record): every store
//! (`SortedKeyedVec`, censuses, bases), every digest surface (`sigil_ref`),
//! the evaluated-transcendental concessions (`color` cyclotomic cosines,
//! locked Taylor exp — REFUSED by LAW XXII, not carried), `torsion`/
//! `inverse_pi` (their consumer organs land later with their own briefs),
//! `coboundary`/`cochain`/`cocycle`/`cohomology` (reduction is an iae
//! organ in v0.4, built to 03/18, never ported), `glome`, `conformance`,
//! `routing`, `chronometry`, `observability`, `multiset`, and the
//! `consilience`/`reversibility`/`holonomy` carriers (Phase-2 organs own
//! their v0.4 forms).

pub mod horizon;
pub mod mirror;
pub mod multivector;
pub mod rational;
// IAE-2 (homological ground): exact integer Smith Normal Form with full
// unimodular witnesses — the substrate the sheaf cohomology stands on. Pure
// exact-integer linear algebra, no cognition, so it belongs here.
pub mod snf;
pub mod vector_module;
pub mod volume;
pub mod wire;
