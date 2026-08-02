//! THE PEN — JSON emitter mechanics, and nothing else.
//!
//! One writer, shared by every layer that emits. No serialization
//! framework enters the workspace.
//!
//! # WHY THIS CRATE EXISTS
//!
//! There were two of these — `ledger/src/json.rs` and
//! `lens/src/json.rs` — with eleven duplicated public symbols and 58%
//! identical text. **The duplication was deliberate, and the reasoning
//! behind it was sound.** The ledger's own header recorded it:
//!
//! > *two small pens, one per layer, neither borrowing the other's —
//! > the duplication is the price of the one-way arrows, paid knowingly
//! > (the shim it replaces was the cycle).*
//!
//! The constraint is real. The ledger may never depend on a lens — held
//! by `deletability::the_ledger_never_depends_on_a_lens_at_runtime` —
//! and a shim BETWEEN two peers is exactly the cycle that was excised.
//!
//! **What that reasoning missed is a third option: not between them,
//! but BELOW them.** A crate with no dependencies at all cannot
//! participate in a cycle, because a cycle needs an arrow leaving it and
//! none leaves here. So the arrows stay one-way and the price no longer
//! has to be paid.
//!
//! # WHAT IS NOT HERE, AND WHY
//!
//! **The exact-rational text form is NOT here.** It lives beside the
//! type it governs, at `cfe::ratio::ratio_str` — the rule that says
//! *integers plain, everything else `p/q`, never a decimal, never a
//! float* is a fact about `Ratio`, not about JSON. It had drifted into
//! two copies, which was the one genuine correctness risk in the
//! duplication: two copies of the rule that keeps floats out of an exact
//! system.
//!
//! **The extractors are NOT here.** Reading flat JSON back is the
//! ledger's own Tier-A law and stays `ledger::extract`. That seam is
//! real: for the ledger, JSON is a PERSISTENCE format it reads back; for
//! a lens it is a write-only projection never read as state
//! (`19_LENS` §15a). Naming both halves after the format they share is
//! what let two implementations of one writer look normal.

/// Escape a string for a JSON literal.
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// A writer that tracks comma placement per nesting level. `key()`
/// arms a pending value: the next value writer attaches to that key
/// without emitting a fresh comma.
pub struct JsonWriter {
    out: String,
    populated: Vec<bool>,
    pending_key: bool,
}

impl Default for JsonWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonWriter {
    pub fn new() -> Self {
        Self {
            out: String::new(),
            populated: Vec::new(),
            pending_key: false,
        }
    }

    /// Place a separator if this container already has a member, and
    /// mark it populated. A pending key already placed the separator.
    fn seat(&mut self) {
        if self.pending_key {
            self.pending_key = false;
            return;
        }
        if let Some(populated) = self.populated.last_mut() {
            if *populated {
                self.out.push(',');
            }
            *populated = true;
        }
    }

    pub fn open(&mut self) {
        self.seat();
        self.out.push('{');
        self.populated.push(false);
    }

    pub fn close(&mut self) {
        self.out.push('}');
        self.populated.pop();
    }

    pub fn open_array(&mut self) {
        self.seat();
        self.out.push('[');
        self.populated.push(false);
    }

    pub fn close_array(&mut self) {
        self.out.push(']');
        self.populated.pop();
    }

    /// Emit `"key":` and arm the pending value.
    pub fn key(&mut self, key: &str) {
        self.seat();
        self.out.push_str(&format!("\"{}\":", escape(key)));
        self.pending_key = true;
    }

    pub fn field_str(&mut self, key: &str, value: &str) {
        self.key(key);
        self.seat();
        self.out.push_str(&format!("\"{}\"", escape(value)));
    }

    pub fn field_u64(&mut self, key: &str, value: u64) {
        self.key(key);
        self.seat();
        self.out.push_str(&value.to_string());
    }

    pub fn field_raw(&mut self, key: &str, raw: &str) {
        self.key(key);
        self.seat();
        self.out.push_str(raw);
    }

    /// A whole string array in one stroke.
    pub fn array(&mut self, items: impl Iterator<Item = String>) {
        self.seat();
        self.out.push('[');
        for (i, item) in items.enumerate() {
            if i > 0 {
                self.out.push(',');
            }
            self.out.push_str(&format!("\"{}\"", escape(&item)));
        }
        self.out.push(']');
    }

    /// A whole array of pre-rendered JSON objects in one stroke —
    /// the items are already lawful JSON, seated verbatim.
    pub fn raw_array(&mut self, items: impl Iterator<Item = String>) {
        self.seat();
        self.out.push('[');
        for (i, item) in items.enumerate() {
            if i > 0 {
                self.out.push(',');
            }
            self.out.push_str(&item);
        }
        self.out.push(']');
    }

    pub fn finish(self) -> String {
        self.out
    }
}
