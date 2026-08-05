//! THE LEDGER'S READ-BACK — flat-JSON extractors, and nothing else.
//!
//! Tier-A law: directive files and the adopted registry/galaxy reads.
//! **No NEW read-back path ever** — the §11 standing test. Pub for the
//! SCRIBE's two adopted read-backs (the chronicle-line and wall-card
//! reads).
//!
//! # WHY THIS FILE IS NAMED FOR ITS JOB
//!
//! It was `json.rs`, and it held the extractors AND a `JsonWriter` that
//! duplicated the lens's writer symbol for symbol. The duplication was
//! deliberate and its reasoning was recorded here:
//!
//! > *two small pens, one per layer, neither borrowing the other's —
//! > the duplication is the price of the one-way arrows, paid knowingly
//! > (the shim it replaces was the cycle).*
//!
//! The constraint was real — this crate may never depend on a lens —
//! but the option it missed was a crate BELOW both rather than a shim
//! BETWEEN them. Nothing with zero dependencies can close a cycle. The
//! writer now lives in `pen` and the exact-rational text form in
//! `cfe::ratio`, beside the type it governs.
//!
//! **What remains is genuinely the ledger's own.** Reading JSON BACK is
//! a persistence concern, and this crate is the only one that has it: a
//! lens writes JSON that is never read as state (`19_LENS` §15a).
//! Naming this file after the FORMAT is what let a writer hide in it;
//! naming it after the JOB is what keeps one from coming back.

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
