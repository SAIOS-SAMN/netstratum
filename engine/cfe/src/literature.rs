//! LIT-1 — literature: what a literate line may aspire to
//! (operator directive 2026-08-02).
//!
//! BOOK-1 gave a civilization accounts of its own making. Literature is
//! the tier above: **works a line aspires to, and may claim only with
//! chain receipts to show for it.**
//!
//! ```text
//!     BOOK         an account a line KEEPS          BOOK-1
//!     LITERACY     the line learned to read/write   derived, never stored
//!     LITERATURE   a work it may ASPIRE to          LIT-1
//! ```
//!
//! # Literacy is derived, not granted
//!
//! **A line is literate when it keeps its own books.** Not when a flag
//! is set, not when a condition is awarded — when it has minted a book
//! and posted to it. Writing is the act; literacy is the reading of
//! that act.
//!
//! This stores nothing. Literacy is a pure function of committed state
//! and is recomputed whenever asked, so it can never go stale and there
//! is no second record of it to disagree with the books themselves.
//!
//! **Why it is not a base-7 condition.** The crystallized stack is full
//! — conditions 0..=6, with the crown gated on the stellar leap and all
//! five (`10 §3`, `§3b`). An eighth condition would change the base and
//! silently move the crown's gate. Literacy needs no slot: it is
//! already implied by receipts that exist.
//!
//! # Aspiring is free; claiming costs evidence
//!
//! ```text
//!     ASPIRE   a literate line declares a work it reaches for.
//!              Cheap, revocable by nothing, proves nothing.
//!
//!     CLAIM    the line asserts it has DONE the thing — and must cite
//!              CHAIN RECEIPTS. The law verifies every cited digest is
//!              a real receipt in this world's chronicle.
//! ```
//!
//! **The law verifies the citation, never the virtue.** It checks that
//! the receipts exist and are this world's; it does not score whether
//! the deeds amount to valor. Any formula that scored virtue would be
//! the struck C-gate wearing a new name — a number standing in for a
//! judgement, which `27_EVIDENCE` bars and this tree has removed once
//! already.
//!
//! An unwitnessed claim refuses. That is the whole of "deeds if claimed
//! need chain receipts": a claim without receipts is a boast, and the
//! chronicle is what separates the two.
//!
//! # The four chartered WORKS
//!
//! Named by operator directive. Reached *through union and otherwise
//! towards stewardship* — the union deeds (`12_UNION`) and the care of
//! what a line holds are where the receipts come from.
//!
//! **A note on `COMMITMENT`, recorded rather than resolved silently.**
//! `commitment` is already load-bearing in the kernel: `26_ARTICLES`
//! article IV clause 2 rules that this manifold is *one commitment per
//! step*, and `Commitment.parent` is the chain's backward reference.
//! That is the governing layer. A work a civilization aspires to is
//! content — the same distinction that kept a civilization's and a
//! clade's own names from being renamed when a seat name was struck. Two correct uses in two layers is not
//! a defect; the defect would be a third that conflates them.

/// The works the charter names. Content vocabulary — a line aspires to
/// these; it does not invent them (that is what BOOK-1 is for).
pub const WORKS: [&str; 4] = ["VALOR", "HONOR", "COURAGE", "COMMITMENT"];

/// The identity of a work — content-addressed over its normalized name,
/// exactly as a book kind is, so the two vocabularies behave alike.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WorkId(pub [u8; 32]);

/// The kind a declared name addresses. Pure and total.
pub fn work_of(name: &str) -> WorkId {
    WorkId(blake3::hash(crate::books::normalize(name).as_bytes()).into())
}

/// Is this one of the chartered works? Compared in normal form, so
/// padding and case cannot smuggle a name past.
pub fn is_work(name: &str) -> bool {
    let normal = crate::books::normalize(name);
    WORKS
        .iter()
        .any(|held| crate::books::normalize(held) == normal)
}

/// The chartered work a name addresses, if any.
pub fn work_named(id: WorkId) -> Option<&'static str> {
    WORKS.iter().copied().find(|name| work_of(name) == id)
}

/// One aspiration — committed state, rebuilt from receipts on replay.
/// Work-ascending on the orb (canonical order).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Aspiration {
    pub work: WorkId,
    /// As declared. Presentation only — [`WorkId`] is the identity.
    pub name: String,
    /// The chain receipts cited when the work was claimed. Empty while
    /// the line is still only reaching for it.
    pub witnesses: Vec<[u8; 32]>,
}

impl Aspiration {
    /// A work is claimed once it carries witnesses. There is no separate
    /// flag: the evidence is the claim, so the two cannot disagree.
    pub fn claimed(&self) -> bool {
        !self.witnesses.is_empty()
    }
}

/// **The reading of A line's literature** — what it reaches for, what
/// it has shown receipts for, and nothing scored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkView {
    pub work: WorkId,
    pub name: String,
    pub claimed: bool,
    /// How many chain receipts back the claim. A count of cited
    /// evidence, never a measure of the virtue.
    pub witnesses: usize,
}
