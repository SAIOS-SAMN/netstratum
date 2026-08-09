//! ONE-SHOT migrate extractors for legacy JSON on disk.
//!
//! Production write path emits **no** JSON. These readers exist only so
//! `load` / chart / universe can convert residual files once and delete
//! them. **No new JSON read-back path** — when migrate is complete these
//! helpers become dead code safe to delete.
//!
//! Exact-rational text form lives at `cfe::ratio::ratio_str`.

/// Extract `"key":"value"` from flat legacy JSON (migrate-only).
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
