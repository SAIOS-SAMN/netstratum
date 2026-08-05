//! LINE SYNTHESIS — a civilization drawn from a digest, pure (the
//! standing ruling: *deed creation dynamic, built into the system
//! itself* — the engine mints its peoples; nobody authors them).
//!
//! The theorems:
//!   T1 determinism — same digest, same line, forever (CHARTER §9).
//!   T2 lawfulness  — every draw lands inside the closed vocabularies
//!                    BY CONSTRUCTION: axes in range, class and clade
//!                    within the ratified ceilings, never checked after.
//!   T3 totality    — every class has words, every disposition ×
//!                    perception has a motto: no digest yields an
//!                    empty name. Exhaustive over the small domains.
//!   T4 spread      — different digests yield different lines; the
//!                    space is a draw, not a constant.
//!   T5 unstationed — a minted line holds NO office: seats are space,
//!                    entered by the receipted seat deed (tag 89),
//!                    never manufactured by a draw.

use cfe::dop::{class_ceiling, clade_ceiling, line_synthesis};

fn digest(fill: u8) -> [u8; 32] {
    // A REAL digest, not an arithmetic pattern: the first fixture
    // correlated bytes linearly and the spread pin caught it — 24
    // distinct names where a decorrelated draw yields near the space's
    // size. The pin was right; the fixture was the defect.
    *blake3::hash(&[fill]).as_bytes()
}

#[test]
fn the_same_digest_draws_the_same_line_forever() {
    let a = line_synthesis(&digest(7));
    let b = line_synthesis(&digest(7));
    assert_eq!(a, b, "the draw is the digest's, not the moment's");
}

#[test]
fn every_draw_is_lawful_by_construction() {
    for fill in 0..=255u8 {
        let line = line_synthesis(&digest(fill));
        assert!((1..=3).contains(&line.disposition), "disposition in the closed vocabulary");
        assert!((1..=3).contains(&line.metabolism), "metabolism in the closed vocabulary");
        assert!((1..=4).contains(&line.perception), "perception in the closed vocabulary");
        assert!(
            (1..=class_ceiling()).contains(&line.class),
            "class within the ratified ceiling"
        );
        assert!(
            (1..=clade_ceiling()).contains(&line.clade),
            "clade within the ratified ceiling — Draco's 19 and Darlok's 20 are lawful draws"
        );
        assert!(!line.name.is_empty() && !line.species.is_empty() && !line.motto.is_empty());
    }
}

#[test]
fn different_digests_draw_different_lines() {
    let names: std::collections::HashSet<String> =
        (0..=255u8).map(|fill| line_synthesis(&digest(fill)).name).collect();
    assert!(
        names.len() > 32,
        "the space is a draw, not a constant — {} distinct names over 256 digests",
        names.len()
    );
}

#[test]
fn a_minted_line_holds_no_office() {
    // T5 — structural: `LineDraw` carries NO station field at all, so a
    // draw CANNOT seat a line. This pin exists to fail loudly if anyone
    // ever adds one: seats are space, entered by tag 89's deed, and a
    // draw that manufactured office would mint authority from a hash.
    let line = line_synthesis(&digest(1));
    let debug = format!("{line:?}");
    assert!(
        !debug.contains("station"),
        "the draw mints a PEOPLE — office comes by deed: {debug}"
    );
}

/// THE DEED HALF (tag 101): the mint receipts the seed alone; the same
/// people derives from the receipt wherever it is read; successive
/// mints draw different lines because the terminal digest moves; and
/// replay MATCHES because identity is never matter.
#[test]
fn the_deed_mints_by_seed_and_everything_derives() {
    use cfe::dop::ClosureEvent;
    use cfe::world::{replay, World};
    let mut world = World::new();
    world.found("cradle").expect("founds");
    let before = world.chronicle().receipts().last().expect("founded").own.0;

    let first = world.mint_line().expect("a people enters");
    let ClosureEvent::LineMinted { seed } =
        &world.chronicle().receipts().last().expect("minted").event
    else {
        panic!("the last receipt is the mint");
    };
    assert_eq!(seed, &before, "the seed IS the pre-mint terminal digest");
    assert_eq!(
        line_synthesis(seed),
        first,
        "every reader derives the same people from the receipt"
    );

    let second = world.mint_line().expect("another people");
    assert_ne!(first, second, "each receipt moves the digest — each mint is a new line");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
