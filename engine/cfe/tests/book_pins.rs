//! BOOK-1 — THE OPEN REGISTRY (`19_LENS` §16).
//!
//! The verification checklist the specification set, plus the two
//! invariants that carry the actual security of the design: archiving
//! must not launder, and the charter's books must be closeable by
//! nobody.

use cfe::books::{is_charter, kind_of, normalize, CHARTER_BOOKS};
use cfe::dop::{ClosureEvent, ColonyOrb};
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

// ── THE NOVELTY GATE ──────────────────────────────────────────────────

/// **THE CHECKLIST'S FIRST ITEM.** A duplicate does not fail a distance
/// test — it is the SAME book, because a kind is content-addressed over
/// the normalized name. The refusal names what already stands.
#[test]
fn a_duplicate_book_refuses_by_identity_not_by_threshold() {
    let mut world = founded();
    let orb = orb_id(&world);
    world.mint_book(orb, "Valor").expect("first mint stands");

    // Exact, case, padding, and inner-whitespace variants are ONE book.
    for variant in ["Valor", "valor", "  VALOR  ", "va lor"] {
        let again = world.mint_book(orb, variant);
        match variant {
            // "va lor" normalizes differently — it is a different name
            // and therefore honestly a different book.
            "va lor" => assert!(again.is_ok(), "a genuinely different name is a new book"),
            _ => assert!(
                matches!(again, Err(ActionRefused::BookAlreadyMinted { .. })),
                "{variant} is the same book"
            ),
        }
    }
}

/// The normal form is what identity is built on, so it is pinned
/// directly rather than only through the gate.
#[test]
fn the_normal_form_collapses_presentation_only() {
    assert_eq!(normalize("  Deep   Valor "), "deep valor");
    assert_eq!(kind_of("Deep Valor"), kind_of("  deep   VALOR  "));
    assert_ne!(kind_of("valor"), kind_of("valor-2"));
}

/// The charter's four are law, not inventory — and the check is made in
/// normal form so padding cannot smuggle one through.
#[test]
fn a_charter_book_cannot_be_minted_over() {
    let mut world = founded();
    let orb = orb_id(&world);
    for name in CHARTER_BOOKS {
        assert!(is_charter(name));
        assert!(matches!(
            world.mint_book(orb, name),
            Err(ActionRefused::CharterBookNotMintable { .. })
        ));
        assert!(matches!(
            world.mint_book(orb, &format!("  {} ", name.to_lowercase())),
            Err(ActionRefused::CharterBookNotMintable { .. })
        ));
    }
}

/// A name that normalizes to nothing addresses nothing.
#[test]
fn an_empty_name_addresses_no_book() {
    let mut world = founded();
    let orb = orb_id(&world);
    assert!(matches!(
        world.mint_book(orb, "   "),
        Err(ActionRefused::BookNameEmpty)
    ));
}

// ── ARCHIVING ─────────────────────────────────────────────────────────

/// **THE CHECKLIST'S SECOND ITEM.** Archiving stops further writing and
/// preserves every entry for reading — the anti-laundering rule (§16c).
#[test]
fn archiving_closes_writing_and_preserves_reading() {
    let mut world = founded();
    let orb = orb_id(&world);
    let kind = world.mint_book(orb, "Valor").expect("mints");
    world
        .post_book(orb, kind, ratio(3, 1), ratio(1, 1))
        .expect("posts");
    world.archive_book(orb, kind).expect("archives");

    // Closed to WRITING.
    assert!(matches!(
        world.post_book(orb, kind, ratio(1, 1), ratio(0, 1)),
        Err(ActionRefused::BookClosedToWriting { .. })
    ));
    // And every entry it took is still there, unchanged.
    let held = world.orb().expect("orb").book(kind).expect("still readable");
    assert_eq!(held.kept, ratio(3, 1));
    assert_eq!(held.lost, ratio(1, 1));
    assert!(held.archived);
    // It is still in the uniform view — archiving is not deletion.
    assert!(world
        .orb()
        .expect("orb")
        .books()
        .iter()
        .any(|view| view.kind == kind));
}

/// **THE CHECKLIST'S THIRD ITEM.** The charter's books are closeable by
/// nobody, and the refusal says WHY rather than falling out as a
/// misleading "no such book".
#[test]
fn a_charter_book_is_closeable_by_nobody() {
    let mut world = founded();
    let orb = orb_id(&world);
    for name in CHARTER_BOOKS {
        assert!(matches!(
            world.archive_book(orb, kind_of(name)),
            Err(ActionRefused::CharterBookNotArchivable { .. })
        ));
    }
}

#[test]
fn a_closed_book_cannot_be_closed_twice() {
    let mut world = founded();
    let orb = orb_id(&world);
    let kind = world.mint_book(orb, "Valor").expect("mints");
    world.archive_book(orb, kind).expect("archives");
    assert!(matches!(
        world.archive_book(orb, kind),
        Err(ActionRefused::BookAlreadyArchived { .. })
    ));
}

// ── DUAL ENTRY ────────────────────────────────────────────────────────

/// A book ACCRUES. A negative posting would let an orb unwrite its own
/// history, which is what the chronicle exists to prevent.
#[test]
fn a_book_cannot_be_unwritten() {
    let mut world = founded();
    let orb = orb_id(&world);
    let kind = world.mint_book(orb, "Valor").expect("mints");
    assert!(matches!(
        world.post_book(orb, kind, ratio(-1, 1), ratio(0, 1)),
        Err(ActionRefused::BookPostingNegative)
    ));
    assert!(matches!(
        world.post_book(orb, kind, ratio(0, 1), ratio(-1, 1)),
        Err(ActionRefused::BookPostingNegative)
    ));
}

/// Both sides move together through one deed. There is no single-sided
/// posting surface, so the cross-welding fault is unrepresentable.
#[test]
fn a_posting_carries_both_sides_at_once() {
    let mut world = founded();
    let orb = orb_id(&world);
    let kind = world.mint_book(orb, "Valor").expect("mints");
    world
        .post_book(orb, kind, ratio(2, 1), ratio(1, 1))
        .expect("posts");
    world
        .post_book(orb, kind, ratio(1, 1), ratio(2, 1))
        .expect("posts again");
    let held = world.orb().expect("orb").book(kind).expect("kept");
    assert_eq!(held.kept, ratio(3, 1), "kept accrued");
    assert_eq!(held.lost, ratio(3, 1), "lost accrued on the SAME account");
}

#[test]
fn posting_to_a_book_that_was_never_minted_refuses() {
    let mut world = founded();
    let orb = orb_id(&world);
    assert!(matches!(
        world.post_book(orb, kind_of("never minted"), ratio(1, 1), ratio(0, 1)),
        Err(ActionRefused::NoSuchBook)
    ));
}

// ── REPLAY AND THE WIRE ───────────────────────────────────────────────

/// **THE INVARIANT THE WHOLE LEDGER RESTS ON.** The registry rebuilds
/// from receipts alone: mint, both postings, and the close.
#[test]
fn the_registry_rebuilds_from_receipts_alone() {
    let mut world = founded();
    let orb = orb_id(&world);
    let kept = world.mint_book(orb, "Valor").expect("mints");
    let closed = world.mint_book(orb, "Ardor").expect("mints");
    world
        .post_book(orb, kept, ratio(5, 1), ratio(2, 1))
        .expect("posts");
    world
        .post_book(orb, closed, ratio(1, 1), ratio(1, 1))
        .expect("posts");
    world.archive_book(orb, closed).expect("archives");

    let rebuilt = replay(world.chronicle());
    assert_eq!(
        rebuilt.orbs.first().expect("orb").books,
        world.orb().expect("orb").books,
        "replay reproduces the registry exactly"
    );
    let held = rebuilt.orbs.first().expect("orb");
    assert_eq!(held.book(kept).expect("kept").kept, ratio(5, 1));
    assert!(held.book(closed).expect("closed").archived);
    assert!(!held.book(kept).expect("kept").archived);
}

/// One state, one encoding — through the orb codec and back.
#[test]
fn a_minted_book_survives_the_wire() {
    let mut world = founded();
    let orb = orb_id(&world);
    let kind = world.mint_book(orb, "Valor").expect("mints");
    world
        .post_book(orb, kind, ratio(7, 3), ratio(1, 5))
        .expect("posts");
    world.archive_book(orb, kind).expect("archives");

    let held = world.orb().expect("orb");
    let round = ColonyOrb::from_wire(&held.wire_bytes()).expect("decodes");
    assert_eq!(&round, held);
    assert_eq!(round.book(kind).expect("book").kept, ratio(7, 3));
    assert!(round.book(kind).expect("book").archived);
}

/// An orb that minted nothing stays byte-identical to a pre-BOOK-1 one.
#[test]
fn a_bookless_orb_encodes_exactly_as_before() {
    let plain = ColonyOrb {
        orb_id: 1,
        name: "home".to_string(),
        ..ColonyOrb::default()
    };
    let bytes = plain.wire_bytes();
    assert_eq!(ColonyOrb::from_wire(&bytes).expect("decodes"), plain);
    assert!(plain.books.is_empty());
    // The BOOK-1 entry is emitted only when a book stands, so no tag 90
    // appears at all — a bookless orb is byte-identical to a pre-BOOK-1
    // one and old readers see nothing new.
    assert!(
        !cfe::tlv::records(&bytes).any(|(tag, _)| tag == 90),
        "no book entry is emitted for an orb that minted none"
    );
}

/// The three deeds survive the event codec.
#[test]
fn the_book_deeds_survive_the_event_codec() {
    let kind = kind_of("Valor");
    for event in [
        ClosureEvent::BookMinted { orb_id: 1, kind, name: "Valor".to_string() },
        ClosureEvent::BookPosted { orb_id: 1, kind, kept: ratio(3, 4), lost: ratio(1, 8) },
        ClosureEvent::BookArchived { orb_id: 1, kind },
    ] {
        let bytes = event.wire_bytes();
        let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
        assert_eq!(
            ClosureEvent::from_wire(tag, value).expect("decodes"),
            event,
            "round trip"
        );
    }
}

// ── THE UNIFORM VIEW ──────────────────────────────────────────────────

/// A reader never learns which storage a book came from: the charter's
/// four and one minted this morning arrive through one view.
#[test]
fn charter_and_minted_books_read_through_one_view() {
    let mut world = founded();
    let orb = orb_id(&world);
    assert_eq!(
        world.orb().expect("orb").books().len(),
        4,
        "the charter four stand before anything is minted"
    );
    world.mint_book(orb, "Valor").expect("mints");
    let view = world.orb().expect("orb").books();
    assert_eq!(view.len(), 5);
    assert_eq!(view.iter().filter(|held| held.chartered).count(), 4);
    assert_eq!(view.iter().filter(|held| !held.chartered).count(), 1);
}

/// An unposted book teaches nothing — that is not the same as teaching
/// zero, and `standing` says so.
#[test]
fn an_unposted_book_has_no_standing() {
    let mut world = founded();
    let orb = orb_id(&world);
    let kind = world.mint_book(orb, "Valor").expect("mints");
    let view = world.orb().expect("orb").books();
    let held = view.iter().find(|held| held.kind == kind).expect("minted");
    assert!(!held.posted());
    assert_eq!(held.standing(), None);
}
