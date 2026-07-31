//! The JSON EMITTER (#37 — moved whole from the ledger at the split):
//! rendering primitives for the write-only displays. The extractor
//! half stays ledger-side — reading flat JSON back is Tier-A
//! read-back law, never a lens concern. No serialization framework
//! enters the workspace.

use cfe::ratio::Ratio;
use num_traits::One;

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

/// Exact rendering: integers plain, everything else `p/q` — never a
/// decimal, never a float.
pub fn ratio_str(r: &Ratio) -> String {
    if r.denom().is_one() {
        r.numer().to_string()
    } else {
        format!("{}/{}", r.numer(), r.denom())
    }
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
