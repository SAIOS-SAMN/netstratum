//! # iae — the thought engine (vendored into netstratum)
//!
//! The cognition layer of ipsal-autic-v0.4, brought into netstratum to
//! replace brute-force enumeration with exact algebraic reduction. It sits
//! ABOVE `substrate` (the exact-ℚ Clifford ground) and, in later slices,
//! carries the chain reduction, the Toroid condensation, and the mint.
//!
//! **IAE-1 — the transport pair (Φ / Φ⁻¹).** The geometry-first foundation
//! (operator ruling): every upward/downward movement in the engine is the
//! ONE transport pair applied at a surface (LAW XV). Φ carries its exact
//! inverse Φ⁻¹ from birth (`Φ·Φ⁻¹ = 1` verified exact-rational on entry),
//! composed by the geometric product — non-abelian, the real geometry, not
//! the abelianised chain shadow. The chain carrier, its reduction, and the
//! bridges (`letters`, `equivalence`) land in the following IAE slices.

pub mod transport;

// IAE-2 — the stratified carrier: a cellular sheaf with heterogeneous stalks
// and exact restriction morphisms, and the commutation-gated transduction
// between complexes. The reduction (cohomology by exact SNF) lands next.
mod mat;
pub mod sheaf;
pub mod transduction;
