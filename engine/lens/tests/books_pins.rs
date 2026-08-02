//! THE BOOK SET IS OPEN, AND THE INSTRUMENT FOLLOWS IT
//! (`19_LENS` §16, operator correction 2026-08-02).
//!
//! **Civilizations MINT books and ARCHIVE them.** A reading layer whose
//! instrument is a fixed array of named accounts cannot see a kind of
//! account that did not exist when the crate was compiled, which makes
//! content growth wait on an edit to this lane. These pins hold the
//! instrument open.
//!
//! Two of them pin faults the first cut actually had, so they fail if it
//! is ever rebuilt that way: the welding of a growing content set onto
//! the fixed harmonic substrate, and the archive-as-laundering vector.

use cfe::dop::ColonyOrb;
use cfe::ratio::ratio;
use lens::books::{books_of, Book, BookId, Regard};
use lens::luminal::Aperture;
use lens::observation::Observation;
use rfa_core::{Delta, Support};

fn orb(id: u64) -> ColonyOrb {
    ColonyOrb {
        orb_id: id,
        name: format!("orb-{id}"),
        ..ColonyOrb::default()
    }
}

fn minted(tag: u8) -> BookId {
    BookId::Minted([tag; 32])
}

fn book(id: BookId, kept: i64, lost: i64) -> Book {
    Book {
        id,
        kept: ratio(kept, 1),
        lost: ratio(lost, 1),
        archived: false,
    }
}

// ── THE SET IS OPEN ───────────────────────────────────────────────────

/// **THE POINT OF THE CORRECTION.** A book nobody in this crate has
/// heard of tunes the instrument anyway. Nothing about the reading path
/// names an account, so a civilization can invent one and be seen.
#[test]
fn a_book_this_crate_never_heard_of_still_grinds_a_channel() {
    let invented = minted(0xAB);
    let regard = Regard::of_books(&[book(invented.clone(), 3, 1)]);

    assert!(regard.keeps(&invented));
    assert_eq!(regard.weight(&invented), ratio(3, 4));
    assert_eq!(regard.width(), 1);
}

/// The width is a fact about the civilization, not a constant of the
/// source. Mint more kinds of account, get a wider instrument — with no
/// ceiling in this crate.
#[test]
fn the_instrument_widens_with_every_book_minted() {
    let mut books = Vec::new();
    for tag in 0..40u8 {
        books.push(book(minted(tag), 1, 1));
        assert_eq!(Regard::of_books(&books).width(), books.len());
    }
    assert_eq!(Regard::of_books(&books).width(), 40);
}

/// Order is canonical and provenance-stable regardless of the order the
/// books arrive in — the same discipline every open ledger here follows.
#[test]
fn the_regard_is_id_ascending_however_the_books_arrive() {
    let scrambled = [
        book(minted(9), 1, 1),
        book(BookId::Chartered("TRADE-1"), 1, 1),
        book(minted(2), 1, 1),
        book(BookId::Chartered("BOND-1"), 1, 1),
    ];
    let seen: Vec<_> = Regard::of_books(&scrambled).books().cloned().collect();
    let mut sorted = seen.clone();
    sorted.sort();
    assert_eq!(seen, sorted);
    // Chartered books sort ahead of minted ones: the law's set first.
    assert!(seen[0].is_chartered() && seen[1].is_chartered());
    assert!(!seen[2].is_chartered() && !seen[3].is_chartered());
}

// ── THE LAYER SEPARATION ──────────────────────────────────────────────

/// **THE LAYER ERROR, PINNED SHUT.** The eleven harmonic channels are
/// SUBSTRATE and the books are CONTENT. However loud an observer's books
/// are, the aperture must not move — the first cut joined them, and this
/// fails if anyone rejoins them.
#[test]
fn books_never_move_the_harmonic_aperture() {
    let mut loud = orb(1);
    loud.given_in_crisis = ratio(99, 1);
    loud.bond_flow = ratio(99, 1);
    loud.covenants_fulfilled = ratio(99, 1);

    let delta = Delta::zero(3, 1);
    let support = Support::new(3, 1);
    let plain = Observation::at(&delta, &support);
    let observed = Observation::at(&delta, &support).seen_by(&loud);

    // The books ARE read — the regard widened...
    assert!(observed.regard().width() > 0);
    assert_eq!(plain.regard().width(), 0);
    // ...and the substrate did not move by so much as one channel.
    assert_eq!(
        observed.instrument(),
        plain.instrument(),
        "content must not reach the fixed harmonic substrate"
    );
    assert_eq!(observed.instrument(), &Aperture::flat());
}

/// The two sides of a book belong to each other. The first cut welded
/// the compassion pair to the morality pair and orphaned
/// `drawn_in_crisis` entirely; each pair must read only its own duals.
#[test]
fn each_book_reads_only_its_own_dual() {
    let mut o = orb(1);
    o.given_in_crisis = ratio(1, 1);
    o.drawn_in_crisis = ratio(1, 1); // TRADE-1 balanced at 1/2
    o.returned_across = ratio(3, 1);
    o.extracted_across = ratio(1, 1); // ABSORB-1 at 3/4

    let regard = Regard::of_orb(&o);
    assert_eq!(regard.weight(&BookId::Chartered("TRADE-1")), ratio(1, 2));
    assert_eq!(regard.weight(&BookId::Chartered("ABSORB-1")), ratio(3, 4));
    // And drawn_in_crisis is not orphaned: it is what made TRADE-1 1/2.
    assert!(regard.keeps(&BookId::Chartered("TRADE-1")));
}

// ── ARCHIVING ─────────────────────────────────────────────────────────

/// **THE LAUNDERING VECTOR, CLOSED.** If archiving dropped a book from
/// the reading, the cheapest way to stop being measured by your breaches
/// would be to close the book. Archiving closes an account to WRITING
/// and never to READING, so the weight is identical either way.
#[test]
fn archiving_a_book_does_not_change_what_it_reads() {
    let id = minted(0x7F);
    let open = book(id.clone(), 1, 3);
    let closed = Book {
        archived: true,
        ..open.clone()
    };

    assert_eq!(
        Regard::of_books(&[open]).weight(&id),
        Regard::of_books(&[closed]).weight(&id),
        "a closed book keeps every entry it took"
    );
}

/// A civilization may close what it minted. It may never close what the
/// charter keeps — otherwise the covenant book is closeable by whoever
/// it convicts.
#[test]
fn the_chartered_books_are_closeable_by_nobody() {
    for book in books_of(&orb(1)) {
        assert!(
            book.id.is_chartered(),
            "the four the charter keeps are chartered"
        );
        assert!(!book.archived, "and none of them is ever closed");
    }
}

// ── SILENCE ───────────────────────────────────────────────────────────

/// An observer with no posted books keeps an EMPTY instrument, not a
/// zeroed one, and an unkept book reads FLAT. No history is not no
/// sensitivity — the same rule that governs the aperture.
#[test]
fn silence_is_not_a_signal_of_zero() {
    let regard = Regard::of_orb(&orb(1));
    assert_eq!(regard.width(), 0, "nothing posted, nothing opened");
    assert_eq!(regard, Regard::flat());
    assert_eq!(
        regard.weight(&BookId::Chartered("BOND-1")),
        ratio(1, 1),
        "a book not kept reads flat, never zero"
    );
}

/// The derivation is a function of committed state, so it reproduces —
/// and two observers with different histories read differently, which is
/// the per-observer half of LAW 9.
#[test]
fn different_histories_grind_different_instruments() {
    let mut giver = orb(1);
    giver.given_in_crisis = ratio(4, 1);
    let mut taker = orb(2);
    taker.drawn_in_crisis = ratio(4, 1);

    let (g, t) = (Regard::of_orb(&giver), Regard::of_orb(&taker));
    assert_ne!(g, t);
    assert_eq!(g.weight(&BookId::Chartered("TRADE-1")), ratio(1, 1));
    assert_eq!(t.weight(&BookId::Chartered("TRADE-1")), ratio(0, 1));
    assert_eq!(g, Regard::of_orb(&giver), "reproducible");
}
