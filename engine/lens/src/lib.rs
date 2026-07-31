//! THE LENS (#37) — read-only projections FROM the chain.
//!
//! §11 rule 2: exports are write-only display, deletable at any
//! moment. #37's law is that they must be RE-SERVABLE from the one
//! truth before the display half deletes (the observation-continuity
//! rule). This crate is the physical split's first resident: it reads
//! the spine (and nothing else — no ledger file, no export), decodes
//! through the codec, and projects. A lens output is disposable and
//! never read back.
//!
//! First projections: the committed head state, and the orb's house
//! wire (`state.tlv`'s bytes) — proving the display's committed half
//! re-derives from the chain byte-for-byte.

use cfe::world::Committed;
use mesh::spine::{Spine, SpineRefused};

/// The typed lens refusal — a projection names what it could not read.
#[derive(Debug)]
pub enum LensRefused {
    /// The mainline has no records — nothing to project.
    Unborn,
    /// The spine refused (open, fetch, or a forged blob).
    Spine(SpineRefused),
    /// The head blob did not decode under the codec.
    Undecodable,
}

impl From<SpineRefused> for LensRefused {
    fn from(refused: SpineRefused) -> Self {
        LensRefused::Spine(refused)
    }
}

/// The head projection: where the mainline stands, decoded whole.
pub fn committed(spine: &Spine) -> Result<Committed, LensRefused> {
    let head = spine.head("").ok_or(LensRefused::Unborn)?;
    let bytes = spine.fetch(&head.state)?;
    codec::from_state_bytes(&bytes).ok_or(LensRefused::Undecodable)
}

/// The orb's house wire — the same bytes the display's `state.tlv`
/// carries, re-served from the chain instead of a file. `None` when
/// the world holds no orb yet (the display writes nothing then too).
pub fn state_tlv(spine: &Spine) -> Result<Option<Vec<u8>>, LensRefused> {
    let state = committed(spine)?;
    Ok(state.orbs.first().map(|orb| orb.wire_bytes()))
}
