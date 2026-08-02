//! LIT-1 DEEDS — literacy, aspiring, and the witnessed claim.
//!
//! Two deeds and a derived predicate:
//!
//! ```text
//!     literate(orb)          DERIVED — the line keeps its own books
//!     aspire <work>          declare a reach          WorkAspired
//!     claim <work> <digests> assert the deed          WorkClaimed
//! ```
//!
//! **The law verifies the CITATION, never the virtue.** A claim must
//! cite chain receipts, and every cited digest is checked against this
//! world's chronicle. Nothing here scores whether the deeds amount to
//! valor — a formula that did would be the struck C-gate under a new
//! name, and `18_TORSION` bars exactly that.

use super::*;
use crate::literature::{is_work, work_named, work_of, Aspiration, WorkId, WorkView};

impl World {
    /// **LITERACY — DERIVED, NEVER GRANTED.** A line is literate when it
    /// keeps its own books: it has MINTED one and POSTED to it. Writing
    /// is the act; this is the reading of that act.
    ///
    /// Stores nothing, so it cannot go stale and there is no second
    /// record to disagree with the books. A line that mints a book and
    /// never writes in it has not learned to write.
    pub fn literate(&self, orb: u64) -> bool {
        self.orb_by_id(orb).is_some_and(|held| {
            held.books
                .iter()
                .any(|book| book.kept != ratio(0, 1) || book.lost != ratio(0, 1))
        })
    }

    /// Every chartered work, with what this line has done about it.
    /// Ordered by the charter, so a display never reshuffles.
    pub fn literature(&self, orb: u64) -> Vec<WorkView> {
        let held = self.orb_by_id(orb);
        crate::literature::WORKS
            .iter()
            .map(|name| {
                let work = work_of(name);
                let standing = held.and_then(|orb| {
                    orb.aspirations.iter().find(|held| held.work == work)
                });
                WorkView {
                    work,
                    name: (*name).to_string(),
                    claimed: standing.is_some_and(|held| held.claimed()),
                    witnesses: standing.map(|held| held.witnesses.len()).unwrap_or(0),
                }
            })
            .collect()
    }

    /// **ASPIRE** — a literate line declares a work it reaches for.
    ///
    /// Proves nothing and is meant to prove nothing. The gate is
    /// literacy alone: a line that cannot keep its own books has no
    /// business reaching for the literature.
    pub fn aspire(&mut self, orb: u64, name: &str) -> Result<WorkId, ActionRefused> {
        self.aspire_as(SOVEREIGN, orb, name)
    }

    /// The contained form. Build scope: aspiring adds standing state.
    pub fn aspire_as(
        &mut self,
        actor: u64,
        orb: u64,
        name: &str,
    ) -> Result<WorkId, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orb_by_id(orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        // The works are CHARTERED. A line invents accounts (BOOK-1); it
        // does not invent the literature it reaches for.
        if !is_work(name) {
            return Err(ActionRefused::NoSuchWork { name: name.to_string() });
        }
        if !self.literate(orb) {
            return Err(ActionRefused::NotLiterate);
        }
        let work = work_of(name);
        if self
            .orb_by_id(orb)
            .is_some_and(|held| held.aspirations.iter().any(|held| held.work == work))
        {
            return Err(ActionRefused::WorkAlreadyAspired { name: name.to_string() });
        }
        let declared = work_named(work).unwrap_or(name).to_string();
        let held = peel(&mut self.orbs, orb).expect("judged above");
        held.aspirations.push(Aspiration {
            work,
            name: declared.clone(),
            witnesses: Vec::new(),
        });
        held.aspirations.sort_by_key(|held| held.work);
        self.close(ClosureEvent::WorkAspired { orb_id: orb, work, name: declared }, actor);
        Ok(work)
    }

    /// **CLAIM** — the line asserts it has DONE the thing, and cites
    /// chain receipts for it.
    ///
    /// Every witness is checked against this world's chronicle. An
    /// unwitnessed claim refuses: a claim without receipts is a boast,
    /// and the chronicle is what separates the two.
    pub fn claim_work(
        &mut self,
        orb: u64,
        name: &str,
        witnesses: &[[u8; 32]],
    ) -> Result<(), ActionRefused> {
        self.claim_work_as(SOVEREIGN, orb, name, witnesses)
    }

    /// The contained form. Build scope: a claim alters standing state.
    pub fn claim_work_as(
        &mut self,
        actor: u64,
        orb: u64,
        name: &str,
        witnesses: &[[u8; 32]],
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orb_by_id(orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        if !is_work(name) {
            return Err(ActionRefused::NoSuchWork { name: name.to_string() });
        }
        if witnesses.is_empty() {
            return Err(ActionRefused::ClaimWithoutWitness { name: name.to_string() });
        }
        let work = work_of(name);
        match self
            .orb_by_id(orb)
            .and_then(|held| held.aspirations.iter().find(|held| held.work == work))
        {
            // A work is reached for before it is claimed. Skipping that
            // would let a line claim what it never set out to do.
            None => return Err(ActionRefused::WorkNotAspired { name: name.to_string() }),
            Some(held) if held.claimed() => {
                return Err(ActionRefused::WorkAlreadyClaimed { name: name.to_string() })
            }
            Some(_) => {}
        }
        // THE CITATION IS WHAT IS VERIFIED. Every digest must be a
        // receipt this world actually sealed — the law checks that the
        // evidence EXISTS, and judges nothing about what it shows.
        for witness in witnesses {
            if !self.chronicle.receipts().iter().any(|receipt| receipt.own.0 == *witness) {
                return Err(ActionRefused::WitnessNotInChronicle { digest: *witness });
            }
        }
        let mut cited = witnesses.to_vec();
        cited.sort();
        cited.dedup();
        let held = peel(&mut self.orbs, orb).expect("judged above");
        held.aspirations
            .iter_mut()
            .find(|held| held.work == work)
            .expect("judged above")
            .witnesses = cited.clone();
        self.close(
            ClosureEvent::WorkClaimed { orb_id: orb, work, witnesses: cited },
            actor,
        );
        Ok(())
    }
}
