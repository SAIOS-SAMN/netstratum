//! THE STANDING SURFACE — which row a structure occupies
//! (`19_LENS` §12, task 2).
//!
//! The kernel refuses a work-anchored mint unless the prior state reads
//! `Vacuity::Entangled`, and until now **no surface showed an operator
//! which row anything stood in**. A refusal arrived with a verdict
//! nobody could have anticipated by looking.
//!
//! The four rows, and none of them is a score:
//!
//! ```text
//!     VOID       neither pole stands — no structure, no flow
//!     UNFRAMED   flow with nothing to anchor it
//!     STASIS     structure with no flow: pure circulation, doing no
//!                work. To be RE-VOLUMIZED, never merely refused.
//!     ENTANGLED  both poles stand. The only admitted row.
//! ```
//!
//! **This reports; it never gates.** The kernel reaches its own verdict
//! from its own read — this surface computes the same reading for
//! display, and if the lens were deleted the judge would be unchanged.
//! That is Rule 5, and it is why this module may exist at all.
//!
//! **STASIS IS NOT A FAILING.** `18_TORSION` LAW 4 is explicit that the
//! refusal is thermodynamic rather than moral: *"nothing is being
//! punished for being idle; there is simply no work to be had from a
//! flat field."* A surface that rendered `Stasis` as an error would be
//! reintroducing the penal frame Article III struck whole.

use crate::peel::Depth;
use rfa_core::{boundary_form, dual_pole, holonomies, Delta, Support, Vacuity};

/// Where a structure stands, and what the reading was taken from.
///
/// Both poles are carried so a reader can see WHY the row is what it is
/// — a verdict alone would be a bare judgement, and this lane does not
/// hand those out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Standing {
    /// The row.
    pub row: Vacuity,
    /// Independent circulation — the structural pole. Zero here is
    /// lawful and meaningful: it means nothing circulates above the
    /// floor.
    pub circulation: usize,
    /// Positions carrying net flow — the boundary pole.
    pub flowing: usize,
}

impl Standing {
    /// Read where this structure stands.
    ///
    /// Pure and total. Derives at call from the relation and its
    /// observation record; retains nothing.
    pub fn of(delta: &Delta, support: &Support) -> Self {
        let circulation = holonomies(delta, support).len();
        let form = boundary_form(delta, support);
        Standing {
            row: dual_pole(circulation, &form),
            circulation,
            flowing: form.rank(),
        }
    }

    /// Is this the admitted row — structure AND flow, both standing?
    ///
    /// A READING of the row, never a gate. The judge asks this question
    /// of its own read; this answers it for a display.
    pub fn admitted(&self) -> bool {
        self.row.is_admitted()
    }

    /// **What this row NEEDS**, in one line, for an operator surface.
    ///
    /// Stated as a next step rather than a fault, because none of these
    /// rows is a wrongdoing — three of them are simply states from which
    /// no work is available yet.
    pub fn wants(&self) -> &'static str {
        match self.row {
            Vacuity::Void => "a first relation — nothing stands to move or be moved",
            Vacuity::Unframed => "structure to anchor the flow it already carries",
            Vacuity::Stasis => "re-volumizing — the wheel turns against nothing",
            Vacuity::Entangled => "nothing; both poles stand",
        }
    }

    /// The shallowest depth at which this standing is legible.
    ///
    /// The row is a property of the WHOLE structure, so the far view
    /// suffices — an operator does not need to approach to learn where
    /// something stands, only to learn what it is made of.
    pub fn legible_at(&self) -> Depth {
        Depth::Far
    }
}
