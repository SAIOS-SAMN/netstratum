//! LIT-1 — LITERATURE (operator directive 2026-08-02).
//!
//! The tier above BOOK-1: works a LITERATE line may aspire to, and may
//! claim only with chain receipts to show for it.

use cfe::books::normalize;
use cfe::dop::{ClosureEvent, ColonyOrb};
use cfe::literature::{claimed_share, is_work, work_of, WORKS};
use cfe::ratio::ratio;
use cfe::world::{replay, ActionRefused, World};

fn founded() -> World {
    let mut world = World::default();
    world.found("home").expect("founds");
    world
}

fn orb_id(world: &World) -> u64 {
    world.orb().expect("founded").orb_id
}

/// A line that keeps its own books — the shortest path to literacy.
fn literate_world() -> (World, u64) {
    let mut world = founded();
    let orb = orb_id(&world);
    let kind = world.mint_book(orb, "Ledger of Deeds").expect("mints");
    world
        .post_book(orb, kind, ratio(1, 1), ratio(0, 1))
        .expect("posts");
    (world, orb)
}

fn a_receipt(world: &World) -> [u8; 32] {
    world.chronicle().receipts().first().expect("a receipt").own.0
}

// ── LITERACY IS DERIVED ───────────────────────────────────────────────

/// **THE GATE.** Literacy is not granted, awarded or stored — it is the
/// reading of an act. Minting alone is not writing; posting is.
#[test]
fn literacy_is_the_reading_of_having_written() {
    let mut world = founded();
    let orb = orb_id(&world);
    assert!(!world.literate(orb), "a fresh line keeps no books");

    let kind = world.mint_book(orb, "Ledger of Deeds").expect("mints");
    assert!(
        !world.literate(orb),
        "an empty book is not writing — the page is blank"
    );

    world
        .post_book(orb, kind, ratio(1, 1), ratio(0, 1))
        .expect("posts");
    assert!(world.literate(orb), "a line that has written can read");
}

/// Derived means derived: nothing stores it, so it is a pure function of
/// the books and reproduces exactly.
#[test]
fn literacy_stores_nothing_and_reproduces() {
    let (world, orb) = literate_world();
    assert_eq!(world.literate(orb), world.literate(orb));
    let rebuilt = replay(world.chronicle());
    assert!(
        rebuilt
            .orbs
            .first()
            .expect("orb")
            .books
            .iter()
            .any(|book| book.kept != ratio(0, 1)),
        "the books that MAKE it literate rebuild from receipts"
    );
}

// ── ASPIRING ──────────────────────────────────────────────────────────

/// An illiterate line may not reach for the literature.
#[test]
fn an_illiterate_line_cannot_aspire() {
    let mut world = founded();
    let orb = orb_id(&world);
    assert!(matches!(
        world.aspire(orb, "VALOR"),
        Err(ActionRefused::NotLiterate)
    ));
}

/// The four works are CHARTERED. A line invents accounts (BOOK-1); it
/// does not invent the literature it reaches for.
#[test]
fn the_works_are_chartered_and_cannot_be_invented() {
    let (mut world, orb) = literate_world();
    for name in WORKS {
        assert!(is_work(name));
        assert!(world.aspire(orb, name).is_ok(), "{name} is chartered");
    }
    assert!(matches!(
        world.aspire(orb, "Glory"),
        Err(ActionRefused::NoSuchWork { .. })
    ));
    // Presentation cannot smuggle a name past, in either direction.
    assert!(is_work("  valor "));
    assert_eq!(work_of("VALOR"), work_of(" valor "));
    assert_eq!(normalize("  HONOR  "), "honor");
}

#[test]
fn a_work_is_reached_for_once() {
    let (mut world, orb) = literate_world();
    world.aspire(orb, "VALOR").expect("aspires");
    assert!(matches!(
        world.aspire(orb, "valor"),
        Err(ActionRefused::WorkAlreadyAspired { .. })
    ));
}

// ── CLAIMING NEEDS CHAIN RECEIPTS ─────────────────────────────────────

/// **THE HEART OF THE DIRECTIVE: deeds if claimed need CHAIN RECEIPTS.**
/// A claim without evidence is a boast, and the chronicle is what
/// separates the two.
#[test]
fn a_claim_without_receipts_is_refused() {
    let (mut world, orb) = literate_world();
    world.aspire(orb, "COURAGE").expect("aspires");
    assert!(matches!(
        world.claim_work(orb, "COURAGE", &[]),
        Err(ActionRefused::ClaimWithoutWitness { .. })
    ));
}

/// The law verifies the CITATION. A digest this world never sealed is
/// refused, so evidence cannot be fabricated.
#[test]
fn a_fabricated_witness_is_refused() {
    let (mut world, orb) = literate_world();
    world.aspire(orb, "HONOR").expect("aspires");
    assert!(matches!(
        world.claim_work(orb, "HONOR", &[[0xAB; 32]]),
        Err(ActionRefused::WitnessNotInChronicle { .. })
    ));
}

/// A work is reached for before it is claimed — otherwise a line could
/// claim what it never set out to do.
#[test]
fn a_work_never_reached_for_cannot_be_claimed() {
    let (mut world, orb) = literate_world();
    let witness = a_receipt(&world);
    assert!(matches!(
        world.claim_work(orb, "VALOR", &[witness]),
        Err(ActionRefused::WorkNotAspired { .. })
    ));
}

#[test]
fn a_claim_stands_once_and_evidence_is_never_swapped() {
    let (mut world, orb) = literate_world();
    let witness = a_receipt(&world);
    world.aspire(orb, "VALOR").expect("aspires");
    world.claim_work(orb, "VALOR", &[witness]).expect("claims");
    assert!(matches!(
        world.claim_work(orb, "VALOR", &[witness]),
        Err(ActionRefused::WorkAlreadyClaimed { .. })
    ));
}

/// The whole journey: write, reach, do, cite.
#[test]
fn a_literate_line_reaches_and_claims_on_evidence() {
    let (mut world, orb) = literate_world();
    let witness = a_receipt(&world);
    world.aspire(orb, "VALOR").expect("aspires");

    let before = world.literature(orb);
    let valor = before.iter().find(|held| held.name == "VALOR").expect("listed");
    assert!(!valor.claimed, "reaching is not having");
    assert_eq!(valor.witnesses, 0);

    world.claim_work(orb, "VALOR", &[witness]).expect("claims");
    let after = world.literature(orb);
    let valor = after.iter().find(|held| held.name == "VALOR").expect("listed");
    assert!(valor.claimed);
    assert_eq!(valor.witnesses, 1);
    // The other three are untouched — a claim is per work.
    assert_eq!(after.iter().filter(|held| held.claimed).count(), 1);
    assert_eq!(claimed_share(&after), ratio(1, 4));
}

/// Cited evidence is deduplicated and ordered, so one state has one
/// encoding however the operator typed it.
#[test]
fn cited_evidence_is_canonical() {
    let (mut world, orb) = literate_world();
    let receipts: Vec<[u8; 32]> = world
        .chronicle()
        .receipts()
        .iter()
        .map(|receipt| receipt.own.0)
        .collect();
    assert!(receipts.len() >= 2, "the fixture sealed several receipts");
    world.aspire(orb, "COMMITMENT").expect("aspires");
    // Same two receipts, out of order, one repeated.
    let cited = [receipts[1], receipts[0], receipts[1]];
    world.claim_work(orb, "COMMITMENT", &cited).expect("claims");

    let held = world.orb().expect("orb");
    let work = held
        .aspirations
        .iter()
        .find(|held| held.work == work_of("COMMITMENT"))
        .expect("claimed");
    assert_eq!(work.witnesses.len(), 2, "the repeat collapsed");
    let mut sorted = work.witnesses.clone();
    sorted.sort();
    assert_eq!(work.witnesses, sorted, "stored in canonical order");
}

// ── REPLAY AND THE WIRE ───────────────────────────────────────────────

#[test]
fn the_literature_rebuilds_from_receipts_alone() {
    let (mut world, orb) = literate_world();
    let witness = a_receipt(&world);
    world.aspire(orb, "VALOR").expect("aspires");
    world.aspire(orb, "HONOR").expect("aspires");
    world.claim_work(orb, "VALOR", &[witness]).expect("claims");

    let rebuilt = replay(world.chronicle());
    assert_eq!(
        rebuilt.orbs.first().expect("orb").aspirations,
        world.orb().expect("orb").aspirations,
        "replay reproduces the literature exactly"
    );
}

#[test]
fn an_aspiration_survives_the_wire() {
    let (mut world, orb) = literate_world();
    let witness = a_receipt(&world);
    world.aspire(orb, "VALOR").expect("aspires");
    world.aspire(orb, "COURAGE").expect("aspires");
    world.claim_work(orb, "VALOR", &[witness]).expect("claims");

    let held = world.orb().expect("orb");
    let round = ColonyOrb::from_wire(&held.wire_bytes()).expect("decodes");
    assert_eq!(&round, held);
}

/// A line that reaches for nothing emits no entry and stays
/// byte-identical to a pre-LIT-1 orb.
#[test]
fn a_line_that_reaches_for_nothing_encodes_as_before() {
    let plain = ColonyOrb {
        orb_id: 1,
        name: "home".to_string(),
        ..ColonyOrb::default()
    };
    let bytes = plain.wire_bytes();
    assert!(
        !cfe::tlv::records(&bytes).any(|(tag, _)| tag == 91),
        "no literature entry for a line that reaches for nothing"
    );
    assert_eq!(ColonyOrb::from_wire(&bytes).expect("decodes"), plain);
}

#[test]
fn the_literature_deeds_survive_the_event_codec() {
    let work = work_of("VALOR");
    for event in [
        ClosureEvent::WorkAspired { orb_id: 1, work, name: "VALOR".to_string() },
        ClosureEvent::WorkClaimed { orb_id: 1, work, witnesses: vec![[7u8; 32], [9u8; 32]] },
    ] {
        let bytes = event.wire_bytes();
        let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
        assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), event);
    }
}
