//! The BOOKS — an open set that civilizations grow
//! (`19_LENS` §16, operator correction 2026-08-02).
//!
//! **A civilization does not merely use books. It mints them and it
//! archives them.** The first cut of this lane read an observer's
//! sensitivity out of eight named struct fields, which made the
//! instrument's width a constant of the source code. Under that shape a
//! civilization could invent an entirely new kind of account and the
//! lens would be structurally unable to see it until someone edited
//! this crate. That is content growth gated on a compile, and it is the
//! wrong scope for a reading layer.
//!
//! So the set is open here. The lens iterates whatever books stand and
//! names none of them in its instrument.
//!
//! # The layer error that came first
//!
//! The first cut also welded books onto the eleven harmonic channels
//! and then reported, as a limit, that *"eight books cannot honestly
//! fill eleven channels."* That framing was wrong at the root. The
//! eleven channels are substrate — spectral, spatial, adjacency,
//! torsion — fixed because the physics is fixed. Books are content,
//! open because civilizations make them. **Joining a growing content
//! set to a fixed substrate axis was the same layer confusion this tree
//! has caught repeatedly**, and the honest repair is not a better
//! mapping but no mapping: [`crate::luminal::Aperture`] keeps the
//! eleven and knows nothing of books; [`Regard`] holds the books and
//! knows nothing of harmonics.
//!
//! It also carried a plainer defect. The old channel 0 summed
//! `given_in_crisis + returned_across` against `extracted_across`,
//! which welded the compassion pair to the morality pair and orphaned
//! `drawn_in_crisis` entirely. **Each book is a dual and its two sides
//! belong to each other**; mixing pairs loses the very structure the
//! books are keeping.
//!
//! # What A BOOK is
//!
//! A dual ledger: what was kept against what was lost, on one named
//! account. The four the charter keeps today are all of that shape
//! (`09 §2.1`–`§2.4`), which is why an open set costs no translation —
//! the shape was already uniform, only the ENUMERATION was frozen.

use cfe::books::CHARTER_BOOKS;
use cfe::dop::ColonyOrb;
use cfe::ratio::{ratio, Ratio};

/// Who minted A BOOK — and therefore who may close it.
///
/// The distinction is not bookkeeping. It is the anti-laundering rule
/// (§16b): a civilization may archive what it minted, and may never
/// archive what the charter keeps. Without that, closing the covenant
/// book is the cheapest way to stop being measured by it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BookId {
    /// Kept by the charter, named in law, closeable by nobody
    /// (`09 §2.1`–`§2.4`).
    Chartered(&'static str),
    /// Minted by a civilization, content-addressed the way every other
    /// open ledger in this tree is (`14 §2`, the element stores).
    Minted([u8; 32]),
}

impl BookId {}

/// One BOOK — a dual account: what stood, against what did not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Book {
    pub id: BookId,
    /// The syntropic side — value that held.
    pub kept: Ratio,
    /// The entropic side of the same pair. Never a separate book: the
    /// two sides belong to each other.
    pub lost: Ratio,
    /// CLOSED to writing, never to reading (§16b). An archived book
    /// takes no new entries and keeps every entry it took.
    pub archived: bool,
}

impl Book {
    /// Has anything ever been posted here? A book that never posted
    /// teaches nothing — and that is not the same as teaching zero.
    pub fn posted(&self) -> bool {
        self.kept != ratio(0, 1) || self.lost != ratio(0, 1)
    }

    /// What stands, against the whole of what this account saw. Exact
    /// rational in `[0, 1]`. `None` where nothing was ever posted,
    /// because silence is not a signal.
    pub fn standing(&self) -> Option<Ratio> {
        match self.posted() {
            true => Some(self.kept.clone() / (self.kept.clone() + self.lost.clone())),
            false => None,
        }
    }
}

/// **The BOOKS an orb keeps — the single enumeration point.**
///
/// **Now fed by the real registry** (`BOOK-1`, cfe `world::books`).
/// `ColonyOrb::books` presents the charter's four and everything the
/// civilization minted through one view, so this function names no book
/// and counts nothing: the lane's width follows the civilization rather
/// than the source code, which is the whole point of the shape.
///
/// The charter list comes from `cfe` rather than being repeated here —
/// cfe is the authority on which books are law, and a second copy of
/// that list in a reading layer is exactly the kind of drift `19_LENS`
/// §1a exists to prevent.
///
/// Returned in the registry's own canonical order: charter books first,
/// then minted books kind-ascending.
pub fn books_of(orb: &ColonyOrb) -> Vec<Book> {
    orb.books()
        .into_iter()
        .map(|held| Book {
            id: match held.chartered {
                // Unreachable in practice — cfe builds the view and the
                // list from one array — so this falls back rather than
                // panicking. A reading never loses a book it cannot
                // classify; at worst it reads as minted.
                true => CHARTER_BOOKS
                    .iter()
                    .find(|name| **name == held.name)
                    .copied()
                    .map_or(BookId::Minted(held.kind.0), BookId::Chartered),
                false => BookId::Minted(held.kind.0),
            },
            kept: held.kept,
            lost: held.lost,
            archived: held.archived,
        })
        .collect()
}

/// **The regard — an observer's sensitivity across the books it keeps.**
///
/// Open by construction: one channel per book that posted, however many
/// that becomes. Nothing here is an array, a constant, or a name.
///
/// Distinct from [`crate::luminal::Aperture`] and deliberately so — the
/// aperture weights the eleven fixed harmonic channels of the substrate,
/// this weights the open content set of a civilization. Different
/// layers, never joined (see the module header).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Regard {
    /// id-ascending; `(book, weight, observations)`.
    channels: Vec<(BookId, Ratio, u32)>,
}

impl Regard {
    /// The observer that keeps no books — width zero. An empty regard,
    /// not a zeroed one.
    pub fn flat() -> Self {
        Regard {
            channels: Vec::new(),
        }
    }

    /// The METABOLISM, unchanged from the aperture's:
    /// `h' = (h·n + signal) / (n + 1)`, exact throughout. A channel
    /// unseen before opens at the signal itself.
    pub fn observe(&mut self, id: &BookId, signal: Ratio) {
        match self.channels.iter_mut().find(|(book, _, _)| book == id) {
            Some((_, weight, count)) => {
                let n = ratio(*count as i64, 1);
                *weight = (weight.clone() * n + signal) / ratio(*count as i64 + 1, 1);
                *count += 1;
            }
            None => {
                let at = self.channels.partition_point(|(book, _, _)| book < id);
                self.channels.insert(at, (id.clone(), signal, 1));
            }
        }
    }

    /// **Derived from the books, never assigned.** Every book that
    /// posted opens a channel; a book that never posted opens none,
    /// because silence is not a signal of zero.
    pub fn of_books(books: &[Book]) -> Self {
        let mut regard = Regard::flat();
        for book in books {
            if let Some(standing) = book.standing() {
                regard.observe(&book.id, standing);
            }
        }
        regard
    }

    /// The instrument this observer has earned, from its own committed
    /// books and nothing else.
    pub fn of_orb(orb: &ColonyOrb) -> Self {
        Regard::of_books(&books_of(orb))
    }

    /// The weight given one book. A book this observer does not keep
    /// reads flat, never zero — no history is not no sensitivity.
    pub fn weight(&self, id: &BookId) -> Ratio {
        self.channels
            .iter()
            .find(|(book, _, _)| book == id)
            .map(|(_, weight, _)| weight.clone())
            .unwrap_or_else(|| ratio(1, 1))
    }

    /// Does this observer keep this book at all?
    pub fn keeps(&self, id: &BookId) -> bool {
        self.channels.iter().any(|(book, _, _)| book == id)
    }

    /// **How wide the instrument is — and it grows with the
    /// civilization.** This number is a fact about how many kinds of
    /// account an observer has actually kept, and it is bounded by
    /// nothing in this crate.
    pub fn width(&self) -> usize {
        self.channels.len()
    }

    /// The books in view, id-ascending.
    pub fn books(&self) -> impl Iterator<Item = &BookId> {
        self.channels.iter().map(|(book, _, _)| book)
    }
}
