//! BOOK-1 deeds — mint, post, archive (`19_LENS` §16).
//!
//! **A civilization does not merely use books. It mints them and it
//! archives them.** The charter keeps four (`09 §2.1`–`§2.4`); until
//! BOOK-1 that was the entire vocabulary of account an orb could hold,
//! so a genuinely new kind of relation had nowhere to be kept and no
//! reading layer could see it without a recompile.
//!
//! Three deeds, and no fourth:
//!
//! ```text
//!     MINT      declare a kind of account         BookMinted
//!     POST      a DUAL entry, both sides at once  BookPosted
//!     ARCHIVE   close it to WRITING only          BookArchived
//! ```
//!
//! # The novelty gate is IDENTITY, and it is exact
//!
//! A kind is content-addressed over its normalized name
//! ([`crate::books::kind_of`]), so two declarations that normalize alike
//! are one kind and the second refuses against the first by NAME. There
//! is no threshold anywhere in this file.
//!
//! **The specification asked for `distance(schema_new, schemas) ≥ θ` and
//! that is refused here, deliberately.** `θ` is a dialed constant of
//! exactly the shape `19_LENS` §5c and Article III strike, and a mint
//! gate is the last place a tunable belongs — whoever tunes it decides
//! who may keep books. More decisively, **there is no structural schema
//! to measure a distance over**: a book declares a name and accrues two
//! sides, so a "schema distance" could only be string distance, which is
//! a linguistic judgment wearing a numeric coat and would refuse
//! genuinely distinct books as readily as duplicates.
//!
//! **What the exact gate catches, stated rather than implied.** Case,
//! padding and inner-whitespace variants — the whole family of
//! accidental and lazy duplicates. It does not catch a deliberate
//! near-miss such as `valor-2`, and no honest test here can, because
//! that is a different name and may well be a different book.
//!
//! **So deliberate spam is not answered by a gate, and must not be.**
//! It is answered by refusing to reward width: `Regard::width` is a
//! reading, and `19_LENS` §1 rules that nothing optimizes a reading. An
//! orb that mints a thousand empty books has a thousand empty books and
//! has bought nothing with them, because an unposted book opens no
//! channel at all. A mint cost would bound it further; setting that
//! magnitude is a charter decision and is not taken here.

use super::*;
use crate::books::{kind_of, BookAccount, BookKind};

impl World {
    /// **Mint A BOOK** — declare a kind of account this orb will keep.
    ///
    /// Returns the kind, which is the caller's handle for posting. Idem-
    /// potence is deliberately not offered: re-minting refuses loudly
    /// rather than silently succeeding, because a civilization that
    /// thinks it created a second book should learn that it did not.
    pub fn mint_book(
        &mut self,
        orb: u64,
        name: &crate::books::BookName,
    ) -> Result<BookKind, ActionRefused> {
        self.mint_book_as(SOVEREIGN, orb, name)
    }

    /// The contained form. Build scope: minting adds a standing account.
    pub fn mint_book_as(
        &mut self,
        actor: u64,
        orb: u64,
        name: &crate::books::BookName,
    ) -> Result<BookKind, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        // The empty normal form and the charter collision died at the
        // `BookName` door (29_CONSTRUCTION) — both were decidable from
        // the text alone. Only the facts of this orb's shelf remain.
        let name = name.as_str();
        if self.orb_by_id(orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        let kind = kind_of(name);
        // The novelty gate: same normal form, same kind, same book.
        if let Some(standing) = self.orb_by_id(orb).and_then(|held| held.book(kind)) {
            return Err(ActionRefused::BookAlreadyMinted { name: standing.name.clone() });
        }
        let held = peel(&mut self.orbs, orb).expect("judged above");
        held.books.push(BookAccount {
            kind,
            name: name.to_string(),
            minted_by: orb,
            kept: ratio(0, 1),
            lost: ratio(0, 1),
            archived: false,
        });
        // Kind-ascending: one state, one encoding.
        held.books.sort_by_key(|held| held.kind);
        self.close(ClosureEvent::BookMinted { orb_id: orb, kind, name: name.to_string() }, actor);
        Ok(kind)
    }

    /// **Post A dual entry** — both sides of one account, together.
    ///
    /// There is no single-sided posting deed, by construction. The
    /// cross-welding fault this design exists to prevent — one book's
    /// kept side read against another's lost side — cannot be expressed
    /// through this surface.
    pub fn post_book(
        &mut self,
        orb: u64,
        kind: BookKind,
        kept: crate::dop::NonNegative,
        lost: crate::dop::NonNegative,
    ) -> Result<(), ActionRefused> {
        self.post_book_as(SOVEREIGN, orb, kind, kept, lost)
    }

    /// The contained form. Collect scope: a posting moves book value.
    pub fn post_book_as(
        &mut self,
        actor: u64,
        orb: u64,
        kind: BookKind,
        kept: crate::dop::NonNegative,
        lost: crate::dop::NonNegative,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        // A book accrues — a negative posting would let an orb unwrite
        // its own history. The shape died at the `NonNegative` doors
        // (zero is lawful: posting nothing to both sides is a deed that
        // records a stand of nothing, and the chronicle keeps it).
        let kept = kept.value().clone();
        let lost = lost.value().clone();
        if self.orb_by_id(orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        match self.orb_by_id(orb).and_then(|held| held.book(kind)) {
            None => return Err(ActionRefused::NoSuchBook),
            // §16c: closed to writing. This is the only place that bites.
            Some(standing) if standing.archived => {
                return Err(ActionRefused::BookClosedToWriting { name: standing.name.clone() })
            }
            Some(_) => {}
        }
        let held = peel(&mut self.orbs, orb).expect("judged above");
        let account = held
            .books
            .iter_mut()
            .find(|held| held.kind == kind)
            .expect("judged above");
        account.kept += kept.clone();
        account.lost += lost.clone();
        self.close(ClosureEvent::BookPosted { orb_id: orb, kind, kept, lost }, actor);
        Ok(())
    }

    /// **Archive A BOOK — closed to writing, never to reading** (§16c).
    ///
    /// The account keeps every entry it took and stays readable forever.
    /// If archiving dropped a book from the reading, the cheapest way to
    /// stop being measured by one's breaches would be to close the book,
    /// and the worst-behaved civilization would present the cleanest
    /// instrument.
    ///
    /// **Provenance bounds who may close what.** A civilization archives
    /// what it minted. The charter's four are closeable by nobody —
    /// otherwise the covenant book is closeable by exactly whoever it
    /// convicts.
    pub fn archive_book(&mut self, orb: u64, kind: BookKind) -> Result<(), ActionRefused> {
        self.archive_book_as(SOVEREIGN, orb, kind)
    }

    /// The contained form. Build scope: archiving alters standing state.
    pub fn archive_book_as(
        &mut self,
        actor: u64,
        orb: u64,
        kind: BookKind,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orb_by_id(orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        // A charter book has no BookAccount to find, so it would fall
        // out as NoSuchBook — which would be a true refusal for a
        // misleading reason. It refuses by name instead, because the
        // reason is the whole point of the rule.
        if let Some(name) = crate::books::CHARTER_BOOKS
            .iter()
            .find(|held| kind_of(held) == kind)
        {
            return Err(ActionRefused::CharterBookNotArchivable { name: (*name).to_string() });
        }
        match self.orb_by_id(orb).and_then(|held| held.book(kind)) {
            None => return Err(ActionRefused::NoSuchBook),
            Some(standing) if standing.archived => {
                return Err(ActionRefused::BookAlreadyArchived { name: standing.name.clone() })
            }
            Some(_) => {}
        }
        let held = peel(&mut self.orbs, orb).expect("judged above");
        held.books
            .iter_mut()
            .find(|held| held.kind == kind)
            .expect("judged above")
            .archived = true;
        self.close(ClosureEvent::BookArchived { orb_id: orb, kind }, actor);
        Ok(())
    }
}
