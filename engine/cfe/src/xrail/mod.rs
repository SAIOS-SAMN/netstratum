//! X7 cross-repo settlement adapters (30 / 32 / TASKS_TIER).
//!
//! Pure packet schemas live here always — **no path dependency** on external
//! strand/isthmus/datum crates. Emit/import world deeds that burn/credit STAMP
//! sit in `world::xrail` and require cargo features + T9 Anchor.
//!
//! | Feature | Rail | Adapter |
//! |---------|------|---------|
//! | `deep-rail-strand` / `stamp-rail-strand` | R3 | strand packets |
//! | `deep-rail-court` / `stamp-rail-court` | R4 | isthmus frame + datum verify |

pub mod strand;
pub mod isthmus;
pub mod datum;

pub use strand::{
    decode_strand, encode_note_packet, encode_share_packet, encode_stamp_packet, StrandKind,
    StrandPacket, STRAND_MAGIC,
};
pub use isthmus::{decode_isthmus, encode_isthmus, IsthmusFrame, ISTHMUS_MAGIC};
pub use datum::{encode_court_claim, verify_court_claim, CourtClaim, DATUM_MAGIC};
