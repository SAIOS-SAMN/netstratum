//! The lens (#37) — read-only projections from the chain.
//!
//! §11 rule 2: exports are write-only display, deletable at any
//! moment. #37's law is that they must be RE-servable from the one
//! truth before the display half deletes (the observation-continuity
//! rule). This crate is the physical split's first resident: it reads
//! the arbor (and nothing else — no ledger file, no export), decodes
//! through the codec, and projects. A lens output is disposable and
//! never read back.
//!
//! First projections: the committed head state, and the orb's house
//! wire (`state.tlv`'s bytes) — proving the display's committed half
//! re-derives from the chain byte-for-byte.

pub mod books;
pub mod kinematic;
pub mod luminal;
pub mod observation;
pub mod observer;
pub mod peel;
pub mod propagation;
pub mod render;
pub mod revocation;
// The standing surface is gone (operator ruling 2026-08-04, the vacuity
// removal). It rendered which of four rows a structure occupied, and it
// existed by its own header because the kernel gated a mint on that row.
// With the gate struck it would have been a reading nothing acts on,
// which is the law 8 failure the removal was correcting — so it went
// with the law rather than outliving it.
//
// `revocation` above is not its replacement and does not revive it: it
// reads D4's window (deeds emitted while nothing grew) off `holonomies`
// alone, never off the struck dual-pole verdict.

use cfe::world::Committed;
use mesh::arbor::{Arbor, ArborRefused};
use std::path::Path;

/// The typed lens refusal — a projection names what it could not read.
#[derive(Debug)]
pub enum LensRefused {
    /// The mainline has no records — nothing to project.
    Unborn,
    /// The arbor refused (open, fetch, or a forged blob).
    Arbor(ArborRefused),
    /// The head blob did not decode under the codec.
    Undecodable,
    /// The chronicle refused on load (a broken link names itself).
    Chronicle(cfe::persist::PersistRefused),
}

impl From<ArborRefused> for LensRefused {
    fn from(refused: ArborRefused) -> Self {
        LensRefused::Arbor(refused)
    }
}

/// The head projection: where the mainline stands, decoded whole.
pub fn committed(arbor: &Arbor) -> Result<Committed, LensRefused> {
    let head = arbor.head("").ok_or(LensRefused::Unborn)?;
    let bytes = arbor.fetch(&head.state)?;
    codec::from_state_bytes(&bytes).ok_or(LensRefused::Undecodable)
}

/// The replay-grounded projection: rebuild the live World from its
/// chronicle (`chronicle.tlv` — chain-verified bytes, every link
/// recomputed on load; the lawful read, never an export). Journals and
/// the wall project from this World — the whole display re-derives from
/// the chain (no JSON export).
pub fn world(chronicle: &Path) -> Result<cfe::world::World, LensRefused> {
    let chronicle = cfe::persist::load(chronicle).map_err(LensRefused::Chronicle)?;
    Ok(cfe::world::World::resume(chronicle))
}
