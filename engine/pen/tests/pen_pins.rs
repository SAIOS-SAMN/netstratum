//! THE PEN'S OWN PINS.
//!
//! Neither of the two writers this replaces had a single test. That was
//! survivable while each served one crate; it is not now, because one
//! bug here reaches every layer that emits. Comma placement is the part
//! that actually breaks — it is the only stateful thing in the writer.

use pen::JsonWriter;

#[test]
fn an_empty_object_closes_clean() {
    let mut w = JsonWriter::new();
    w.open();
    w.close();
    assert_eq!(w.finish(), "{}");
}

/// The whole reason the writer holds state: separators go BETWEEN
/// members and never before the first or after the last.
#[test]
fn commas_seat_between_members_and_nowhere_else() {
    let mut w = JsonWriter::new();
    w.open();
    w.field_str("a", "1");
    w.field_u64("b", 2);
    w.field_raw("c", "3");
    w.close();
    assert_eq!(w.finish(), r#"{"a":"1","b":2,"c":3}"#);
}

/// A key arms a pending value, so the value must NOT emit a second
/// separator of its own. This is the case the `seat`/`pending_key`
/// dance exists for.
#[test]
fn a_keyed_container_does_not_double_its_separator() {
    let mut w = JsonWriter::new();
    w.open();
    w.field_u64("first", 1);
    w.key("nested");
    w.open();
    w.field_u64("inner", 2);
    w.close();
    w.close();
    assert_eq!(w.finish(), r#"{"first":1,"nested":{"inner":2}}"#);
}

#[test]
fn nested_arrays_track_their_own_level() {
    let mut w = JsonWriter::new();
    w.open();
    w.key("xs");
    w.open_array();
    w.open();
    w.field_u64("n", 1);
    w.close();
    w.open();
    w.field_u64("n", 2);
    w.close();
    w.close_array();
    w.close();
    assert_eq!(w.finish(), r#"{"xs":[{"n":1},{"n":2}]}"#);
}

#[test]
fn the_array_helper_quotes_and_the_raw_one_does_not() {
    let mut w = JsonWriter::new();
    w.open();
    w.key("quoted");
    w.array(["a".to_string(), "b".to_string()].into_iter());
    w.key("raw");
    w.raw_array([String::from("{\"n\":1}")].into_iter());
    w.close();
    assert_eq!(w.finish(), r#"{"quoted":["a","b"],"raw":[{"n":1}]}"#);
}

#[test]
fn an_empty_iterator_still_yields_an_array() {
    let mut w = JsonWriter::new();
    w.open();
    w.key("none");
    w.array(std::iter::empty());
    w.close();
    assert_eq!(w.finish(), r#"{"none":[]}"#);
}

/// Escaping covers the literal-breaking characters and the control
/// range. A quote reaching output unescaped is a malformed document.
#[test]
fn literal_breaking_characters_are_escaped() {
    let mut w = JsonWriter::new();
    w.open();
    w.field_str("k", "a\"b\\c\nd\te\rf\u{1}");
    w.close();
    // The control character becomes a \u escape rather than a raw
    // byte: the named escapes cover the common four, the \u form
    // covers the rest of the control range.
    assert_eq!(w.finish(), r#"{"k":"a\"b\\c\nd\te\rf\u0001"}"#);
}

/// Keys are escaped too — a key is a JSON literal like any other.
#[test]
fn keys_are_escaped_as_well_as_values() {
    let mut w = JsonWriter::new();
    w.open();
    w.field_u64("a\"b", 1);
    w.close();
    assert_eq!(w.finish(), r#"{"a\"b":1}"#);
}
