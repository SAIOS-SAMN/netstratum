//! BOOK-1 — THE OPEN BOOK REGISTRY: civilizations mint and archive
//! (`19_LENS` §16, operator specification 2026-08-02).
//!
//! The charter keeps four books (`09 §2.1`–`§2.4`), and until now that
//! was the whole vocabulary of account an orb could hold. A civilization
//! that invented a genuinely new kind of relation had nowhere to keep
//! it, and no reading layer could see it without a recompile. This
//! module opens the set.
//!
//! # WHAT A BOOK IS
//!
//! A DUAL ledger: what was KEPT against what was LOST, on one named
//! account. The charter's four are already exactly that shape, which is
//! why opening the set needed no translation of what landed — only the
//! ENUMERATION was closed, never the form.
//!
//! **DUAL-ENTRY IS STRUCTURAL, NOT A CONVENTION.** Both sides live in
//! one [`BookAccount`], so there is no way to hold one side of a book
//! without the other and no way to post to one side alone. The
//! cross-welding fault that the lens lane found — one book's kept side
//! summed against another's lost side — is unrepresentable here rather
//! than merely forbidden.
//!
//! # THE CHARTER FOUR ARE NOT MOVED
//!
//! They stay where they are: fixed fields on `ColonyOrb`, already
//! sealed on the wire at orb entry tag 41 (IDEO-1). **Rewriting landed
//! committed state to make a uniform container would break every
//! existing chronicle for a tidiness that buys nothing.** Instead
//! [`ColonyOrb::books`] presents charter and minted books through ONE
//! read view, so a reader never learns which storage a book came from.
//!
//! # IDENTITY IS THE NOVELTY GATE, AND IT IS EXACT
//!
//! A book kind is CONTENT-ADDRESSED over its normalized name, the way
//! every other open ledger in this tree is addressed (`14 §2`). Two
//! declarations that normalize alike ARE the same kind, so a duplicate
//! is not caught by a test — it is the same book, and minting it twice
//! refuses.
//!
//! **This deliberately does NOT implement a schema-distance threshold.**
//! The specification asked for `distance(schema_new, schemas) ≥ θ`, and
//! that is refused here for two reasons, the second more important than
//! the first:
//!
//! 1. `θ` is a dialed constant. `19_LENS` §5c and Article III both
//!    strike exactly this shape — *the bounded dynamic parameters a
//!    descent needs to be tuned* — and a mint gate is the last place a
//!    tunable belongs, since whoever tunes it decides who may keep
//!    books.
//! 2. **There is no structural schema to measure a distance over.** A
//!    book declares a name and accrues two sides; nothing machine-
//!    checkable distinguishes "Valor" from "Valour" except the strings.
//!    A threshold over string distance is a LINGUISTIC judgment wearing
//!    a numeric coat, and it would refuse genuinely distinct books as
//!    readily as duplicates.
//!
//! **What the exact gate does and does not catch is stated plainly.**
//! It catches case, whitespace and padding variants — the whole family
//! of accidental and lazy duplicates. It does not catch a deliberate
//! near-miss like `valor-2`, and no honest test in this tree can,
//! because that IS a different name and might be a different book.
//!
//! The lawful limiter on deliberate spam is COST and the refusal to
//! reward width — see `19_LENS` §16 and the note in `world::mint_book`.

use crate::ratio::{ratio, Ratio};

/// The four the charter keeps, in canonical order. Named here once so
/// no other module has to know them by heart.
pub const CHARTER_BOOKS: [&str; 4] = ["TRADE-1", "ABSORB-1", "BOND-1", "COVENANT-1"];

/// THE IDENTITY OF A BOOK KIND — content-addressed over the normalized
/// name, so a rename cannot mint a duplicate and a re-declaration of the
/// same account is the same account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BookKind(pub [u8; 32]);

/// **THE NORMAL FORM OF A BOOK NAME.** Case-folded, trimmed, and inner
/// runs of whitespace collapsed to one space. Everything that differs
/// only in presentation collapses to one identity here.
pub fn normalize(name: &str) -> String {
    name.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// The kind a declared name addresses. Pure and total.
pub fn kind_of(name: &str) -> BookKind {
    BookKind(blake3::hash(normalize(name).as_bytes()).into())
}

/// Is this name one the charter keeps? Compared in normal form, so
/// `" trade-1 "` cannot slip past as a mintable name.
pub fn is_charter(name: &str) -> bool {
    let normal = normalize(name);
    CHARTER_BOOKS.iter().any(|held| normalize(held) == normal)
}

/// **A MINTABLE BOOK NAME, proven at construction** (P1,
/// 29_CONSTRUCTION). Two arms died at this door, both decidable from
/// the text alone: a name whose normal form is empty addresses
/// nothing, and the charter's four are law, not inventory. The verb
/// keeps only the facts of the orb's own shelf.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookName(String);

/// The named fault a refused book name carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookNameFault {
    /// A name that normalizes to nothing addresses nothing.
    NormalizesToNothing,
    /// The charter's four cannot be minted over; the fault names the
    /// standing book, because the reason is the whole point.
    Charter { name: String },
}

impl BookName {
    pub fn new(text: &str) -> Result<BookName, BookNameFault> {
        let normal = normalize(text);
        if normal.is_empty() {
            return Err(BookNameFault::NormalizesToNothing);
        }
        if let Some(held) = CHARTER_BOOKS.iter().find(|held| normalize(held) == normal) {
            return Err(BookNameFault::Charter { name: (*held).to_string() });
        }
        Ok(BookName(text.to_string()))
    }
    /// The name as declared — presentation; [`BookKind`] is identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    /// The kind this name addresses.
    pub fn kind(&self) -> BookKind {
        kind_of(&self.0)
    }
}

/// The literal door — a malformed literal is a build error in a
/// runtime coat.
pub fn book_name(text: &str) -> BookName {
    BookName::new(text)
        .expect("a book-name literal must normalize to something and must not be chartered")
}

/// ONE MINTED ACCOUNT — committed state, rebuilt from receipts on
/// replay. Kind-ascending on the orb (canonical order).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BookAccount {
    pub kind: BookKind,
    /// As declared. Presentation only — [`BookKind`] is the identity.
    pub name: String,
    /// The orb that minted it. Provenance decides who may close it.
    pub minted_by: u64,
    /// The syntropic side.
    pub kept: Ratio,
    /// The entropic side of the SAME account, never a separate book.
    pub lost: Ratio,
    /// CLOSED TO WRITING, never to reading.
    pub archived: bool,
}

/// THE UNIFORM READ VIEW over charter and minted books alike.
///
/// A reader never learns which storage a book came from, which is the
/// whole point: the charter four and a book minted this morning are the
/// same kind of thing to anyone reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookView {
    pub kind: BookKind,
    pub name: String,
    /// Kept by the charter — closeable by nobody.
    pub chartered: bool,
    pub kept: Ratio,
    pub lost: Ratio,
    pub archived: bool,
}

impl BookView {
    /// Has anything ever been posted here? A book that never posted
    /// teaches nothing, which is not the same as teaching zero.
    pub fn posted(&self) -> bool {
        self.kept != ratio(0, 1) || self.lost != ratio(0, 1)
    }

    /// What stands, against the whole this account saw. Exact rational
    /// in `[0, 1]`; `None` where nothing was ever posted, because
    /// silence is not a signal.
    pub fn standing(&self) -> Option<Ratio> {
        match self.posted() {
            true => Some(self.kept.clone() / (self.kept.clone() + self.lost.clone())),
            false => None,
        }
    }
}
