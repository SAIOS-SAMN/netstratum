//! Display helpers for the lens — **no JSON**.
//!
//! `render_state` / `render_receipt` JSON emitters are struck
//! (docs/JSON_DISPLAY_ONLY.md). Colony truth is `chronicle.tlv` +
//! `state.tlv`. Journals use framed TLV (`journal.jrn`); the wall is HTML.
//!
//! What remains: the universal clock odometer and hex digests, shared
//! with the scribe's journal writers.

/// Hex of a digest — display form of the chain's bytes.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The six-field base-60 odometer: T of TUniversal, a pure function
/// of the tick count — never of any wall clock.
pub fn universal_clock(ticks: u64) -> String {
    let mut fields = [0u64; 6];
    let mut rest = ticks;
    for slot in (0..6).rev() {
        fields[slot] = rest % 60;
        rest /= 60;
    }
    fields
        .iter()
        .map(|f| format!("{f:02}"))
        .collect::<Vec<_>>()
        .join(":")
}
