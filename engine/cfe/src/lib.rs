//! CFE — the Canonical Field Engine (CHARTER §8): the runtime that
//! evaluates RFA state transitions, runs the TLV codecs, and drives the
//! Netstratum simulation ticks.
//!
//! Module map (one module per protocol seam):
//! - `ratio` — the RFA carrier: exact rationals in canonical coprime
//!   form, with their wire encoding.
//! - `tlv`   — the additive wire format (CHARTER SEAL A: readers skip
//!   unknown tags, never crash on them).
//! - `vrp`   — NS-3: the receipt chain and the attested world clock.
//! - `dop`   — NS-1: dimensional orbs (the colony orb is the first).
//! - `query` — NS-2: the resonance query — the lens beside the ledger
//!   (sigil addressing, O(bucket), pure; the draw closes in `world`).
//! - `world` — the tick loop binding them: actions close as receipts;
//!   unreceipted accrual is workspace and never persists.

pub mod almanac;
pub mod anchor;
pub mod annals;
pub mod parameter;
pub mod books;
pub mod growth;
pub mod literature;
pub mod dop;
pub mod epoch;
mod grain;
// ONT-1 (operator directive 2026-08-16): the morphodynamic field — materials as
// vectors, environment as M ⊕ Φ (matter + massless flux), a species as a
// projection T_metab into the 5 universal roles; viability is a topological
// intersection. Lives in cfe (pure physics) so both siting and production read
// one viability. See morphogen.rs.
pub mod morphogen;
pub mod ipsal;
pub mod persist;
pub mod query;
/// Re-exported from the `ratio` LEAF crate (K4): moving the module
/// out is what let `physics` stop depending on `cfe`, which is what
/// let `cfe` reach the verifier. Callers keep writing `cfe::ratio::*`.
pub use ratio;
pub mod thermal;
pub mod tlv;
pub mod treaty;
pub mod vocabulary;
pub mod vrp;
pub mod world;
pub mod xrail;
