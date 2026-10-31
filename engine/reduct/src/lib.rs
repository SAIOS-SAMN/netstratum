//! # reduct — Netstratum's exact-reduction organ (formerly the local `iae`)
//!
//! **Naming (2026-08-19, operator ruling).** The IAE/AUTIC thought engine was
//! ported to Mycelium, which is now the authority on IAE. This crate — the copy
//! vendored into Netstratum — is renamed `reduct` so the two never collide in
//! one workspace. It keeps net's live reduce organ (the D₄×recolor ARC seat +
//! the survival seat); frame-morphism recognition (the asymmetric coboundary /
//! `FrameMorph` vocabulary) is Mycelium's IAE domain and is coordinated with it,
//! not re-forked here. `mycelium::iae` (used only in `scribe`) is the other,
//! authoritative crate — never conflate the two.
//!
//! The cognition layer of ipsal-autic-v0.4, brought into Netstratum to
//! replace brute-force enumeration with exact algebraic reduction. It sits
//! above `substrate` (the exact-ℚ Clifford ground) and, in later slices,
//! carries the chain reduction, the Toroid condensation, and the mint.
//!
//! **IAE-1 — the transport pair (Φ / Φ⁻¹).** The geometry-first foundation
//! (operator ruling): every upward/downward movement in the engine is the
//! one transport pair applied at a surface (law XV). Φ carries its exact
//! inverse Φ⁻¹ from birth (`Φ·Φ⁻¹ = 1` verified exact-rational on entry),
//! composed by the geometric product — non-abelian, the real geometry, not
//! the abelianised chain shadow. The chain carrier, its reduction, and the
//! bridges (`letters`, `equivalence`) land in the following IAE slices.

pub mod congruence;
pub mod mint;
pub mod observation;
pub mod peel;
pub mod reduction;
pub mod sentence;
pub mod think;
pub mod transport;

// The port's reduce organ, re-exported for the vessel layer (`cfe` grows a
// basin of `reduct::Crystal`; the seat renders `observe`/`think`).
pub use congruence::ChainDelta;
pub use observation::{adopt, observe, Observed};
pub use reduction::Crystal;
pub use sentence::{assemble, probe_class, AssemblyRead, AuticSentence};
pub use think::{think, PairClosure, TaskThought};

// IAE-2 — the stratified carrier: a cellular sheaf with heterogeneous stalks
// and exact restriction morphisms, and the commutation-gated transduction
// between complexes. The reduction (cohomology by exact SNF) lands next.
pub mod atlas;
pub mod bridge;
pub mod cohomology;
mod mat;
pub mod mind;
pub mod proposer;
pub mod resolve;
pub mod sheaf;
pub mod transduction;
