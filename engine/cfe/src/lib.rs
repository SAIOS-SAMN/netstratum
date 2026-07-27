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
//! - `world` — the tick loop binding them: actions close as receipts;
//!   unreceipted accrual is workspace and never persists.

pub mod dop;
pub mod ipsal;
pub mod persist;
pub mod ratio;
pub mod tlv;
pub mod vrp;
pub mod world;
