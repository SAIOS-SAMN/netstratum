//! A tiny hand-rolled JSON emitter/extractor — the mirror's needs are
//! flat and known; no serialization framework enters the workspace.

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

/// Extract `"key":"value"` from flat JSON (directive files are written
/// by our own steward tool; this is a reader of known shapes, not a
/// general parser — the game loop re-validates everything).
pub fn extract_string(text: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let after = &text[text.find(&needle)? + needle.len()..];
    let after = after.trim_start().strip_prefix(':')?.trim_start();
    let mut out = String::new();
    let mut chars = after.strip_prefix('"')?.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next()? {
                'n' => out.push('\n'),
                't' => out.push('\t'),
                'r' => out.push('\r'),
                other => out.push(other),
            },
            '"' => return Some(out),
            c => out.push(c),
        }
    }
    None
}

/// Extract `"key":123` — an unquoted JSON number (the writer's
/// `field_u64` form). Same known-shape discipline as `extract_string`.
pub fn extract_u64(text: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{key}\"");
    let after = &text[text.find(&needle)? + needle.len()..];
    let after = after.trim_start().strip_prefix(':')?.trim_start();
    let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

/// Extract `"key":["a","b",...]` as strings.
pub fn extract_string_array(text: &str, key: &str) -> Option<Vec<String>> {
    let needle = format!("\"{key}\"");
    let after = &text[text.find(&needle)? + needle.len()..];
    let after = after.trim_start().strip_prefix(':')?.trim_start();
    let end = after.find(']')?;
    let inner = &after.strip_prefix('[')?[..end - 1];
    let mut out = Vec::new();
    let mut rest = inner.trim_start();
    while let Some(stripped) = rest.strip_prefix('"') {
        let close = stripped.find('"')?;
        out.push(stripped[..close].to_string());
        rest = stripped[close + 1..].trim_start();
        rest = rest.strip_prefix(',').unwrap_or(rest).trim_start();
    }
    Some(out)
}
