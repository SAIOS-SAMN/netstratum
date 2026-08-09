//! THE PEN — **retired**.
//!
//! JSON emission is gone from the production write path
//! (`docs/JSON_DISPLAY_ONLY.md`). Colony carriers are TLV / binary cards;
//! journals are framed `.jrn`; the agent speaks a line wire.
//!
//! This crate remains as an empty placeholder so historical path
//! references and docs can name the retirement. Do not reintroduce a
//! JSON writer here.

/// Marker that the pen crate is intentionally empty.
pub const RETIRED: &str = "pen::JsonWriter struck — no JSON on the write path";
