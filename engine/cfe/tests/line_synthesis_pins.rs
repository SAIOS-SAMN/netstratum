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
            "clade within the ratified ceiling — Umbrid's 19 and Darlok's 20 are lawful draws"
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
    let before = world.chronicle().receipts().last().expect("founded").own().0;

    let first = world.mint_line().expect("a people enters");
    let ClosureEvent::LineMinted { seed, ceilings } =
        world.chronicle().receipts().last().expect("minted").event()
    else {
        panic!("the last receipt is the mint");
    };
    assert_eq!(seed, &before, "the seed IS the pre-mint terminal digest");
    // FOLD-AWARE (2026-08-06): the receipt DECLARES the ceilings its
    // draw ran under, and on an unminted chain they equal the seed's —
    // so the declared derivation and the compiled one agree here.
    let (class_c, clade_c) = ceilings.expect("a new mint declares its ceilings");
    assert_eq!(
        (class_c, clade_c),
        (class_ceiling(), clade_ceiling()),
        "mint declares open class/clade ceilings"
    );
    assert_eq!(
        cfe::dop::line_synthesis_at(seed, class_c, clade_c),
        first,
        "every reader derives the same people from the receipt"
    );

    let second = world.mint_line().expect("another people");
    assert_ne!(first, second, "each receipt moves the digest — each mint is a new line");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

/// **THE FOLD IS DRAWABLE** (operator ruling 2026-08-06): a minted
/// class grows the draw space the moment its receipt stands — the
/// mint's receipt declares the fold's ceilings, and a digest landing
/// on the new residue wears the minted code. The sealed compiled rule
/// (ceilings: None) never could.
#[test]
fn a_minted_class_is_drawable_and_the_receipt_declares_it() {
    use cfe::dop::ClosureEvent;
    use cfe::world::World;
    let mut world = World::new();
    world.found("fold-cradle").expect("founds");
    world
        .seat_station(
            cfe::dop::name("tunborn"),
            cfe::dop::Station::from_wire(1).expect("a band office"),
        )
        .expect("seats the progenitor");
    let code = world
        .mint_vocabulary(
            "tunborn",
            cfe::vocabulary::Axis::Class,
            "✶",
            cfe::dop::name("voidborne"),
            cfe::vocabulary::trading(vec![
                (cfe::vocabulary::CHANNEL_YIELD, cfe::ratio::ratio(7, 4)),
                (cfe::vocabulary::CHANNEL_UPKEEP, cfe::ratio::ratio(7, 4)),
            ]),
        )
        .expect("the progenitor mints a class");
    assert_eq!(code, class_ceiling() + 1, "the next code, derived");

    world.mint_line().expect("a people enters the grown space");
    let ClosureEvent::LineMinted { seed, ceilings } =
        world.chronicle().receipts().last().expect("minted").event()
    else {
        panic!("the last receipt is the mint");
    };
    let declared = ceilings.expect("a new mint declares its ceilings");
    assert_eq!(declared.0, code, "the receipt carries the grown class ceiling");

    // The pure law: a digest whose class byte lands on the new residue
    // wears the minted code — the space genuinely grew...
    let mut probe = *seed;
    probe[3] = code - 1; // (code-1) % code == code-1 → class = code
    assert_eq!(
        cfe::dop::line_synthesis_at(&probe, declared.0, declared.1).class,
        code,
        "the minted class is drawable"
    );
    // ...and the sealed compiled rule never reaches it.
    assert!(cfe::dop::line_synthesis(&probe).class <= class_ceiling());

    // The draw is derived, never committed — replay is untouched.
    assert_eq!(cfe::world::replay(world.chronicle()), world.committed());
}

/// Tag 101 rides the wire byte-faithful in BOTH forms: the sealed
/// 32-byte pre-amendment receipt and the declaring 34-byte one.
#[test]
fn the_mint_receipt_round_trips_both_forms() {
    use cfe::dop::ClosureEvent;
    for event in [
        ClosureEvent::LineMinted { seed: [7u8; 32], ceilings: None },
        ClosureEvent::LineMinted { seed: [7u8; 32], ceilings: Some((9, 21)) },
    ] {
        let bytes = event.wire_bytes();
        let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
        assert_eq!(tag, 101);
        let decoded = ClosureEvent::from_wire(tag, value).expect("decodes");
        assert_eq!(decoded, event);
        assert_eq!(decoded.wire_bytes(), bytes, "byte-faithful both ways");
    }
}
