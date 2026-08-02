//! THE LENS (#37) — read-only projections FROM the chain.
//!
//! §11 rule 2: exports are write-only display, deletable at any
//! moment. #37's law is that they must be RE-SERVABLE from the one
//! truth before the display half deletes (the observation-continuity
//! rule). This crate is the physical split's first resident: it reads
//! the arbor (and nothing else — no ledger file, no export), decodes
//! through the codec, and projects. A lens output is disposable and
//! never read back.
//!
//! First projections: the committed head state, and the orb's house
//! wire (`state.tlv`'s bytes) — proving the display's committed half
//! re-derives from the chain byte-for-byte.

pub mod json;
pub mod kinematic;
pub mod luminal;
pub mod peel;
pub mod render;

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

/// The orb's house wire — the same bytes the display's `state.tlv`
/// carries, re-served from the chain instead of a file. `None` when
/// the world holds no orb yet (the display writes nothing then too).
pub fn state_tlv(arbor: &Arbor) -> Result<Option<Vec<u8>>, LensRefused> {
    let state = committed(arbor)?;
    Ok(state.orbs.first().map(|orb| orb.wire_bytes()))
}

/// The replay-grounded projection: rebuild the live World from its
/// chronicle (`chronicle.tlv` — chain-verified bytes, every link
/// recomputed on load; the lawful read, never an export). The rich
/// renderings ([`render::render_state`], journals) project from this
/// World — the whole display re-derives from the chain.
pub fn world(chronicle: &Path) -> Result<cfe::world::World, LensRefused> {
    let chronicle = cfe::persist::load(chronicle).map_err(LensRefused::Chronicle)?;
    Ok(cfe::world::World::resume(chronicle))
}
