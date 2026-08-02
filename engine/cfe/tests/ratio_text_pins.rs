//! THE CANONICAL TEXT FORM OF AN EXACT RATIONAL.
//!
//! This rule lived in two byte-identical copies (`ledger/src/json.rs`
//! and `lens/src/json.rs`) with nothing holding them equal. It is the
//! rule that keeps floats out of an exact system, so it now has one
//! home — beside the type it governs — and these pins.

use cfe::ratio::{ratio, ratio_str};

#[test]
fn integers_render_plain() {
    assert_eq!(ratio_str(&ratio(7, 1)), "7");
    assert_eq!(ratio_str(&ratio(0, 1)), "0");
    assert_eq!(ratio_str(&ratio(-7, 1)), "-7");
    // Reduction happens in the type, so this IS an integer.
    assert_eq!(ratio_str(&ratio(8, 4)), "2");
}

#[test]
fn non_integers_render_as_p_over_q() {
    assert_eq!(ratio_str(&ratio(1, 3)), "1/3");
    assert_eq!(ratio_str(&ratio(-1, 3)), "-1/3");
    assert_eq!(ratio_str(&ratio(22, 7)), "22/7");
}

/// **NEVER A DECIMAL, NEVER A FLOAT.** The whole point of the rule: a
/// third that cannot be written exactly in decimal must never be
/// approximated into one on the way to a display.
#[test]
fn no_rendering_ever_contains_a_decimal_point() {
    for (n, d) in [(1, 3), (2, 7), (1, 999_983), (355, 113), (-1, 3)] {
        let text = ratio_str(&ratio(n, d));
        assert!(
            !text.contains('.') && !text.contains('e') && !text.contains('E'),
            "{n}/{d} rendered as {text}"
        );
    }
}

/// Exactness survives the round trip: the text is the pair, so nothing
/// is lost in rendering it.
#[test]
fn the_text_carries_the_whole_value() {
    let big = ratio(123_456_789, 1_000_000_007);
    assert_eq!(ratio_str(&big), "123456789/1000000007");
}
