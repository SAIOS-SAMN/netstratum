//! **CARRIED STATE** (operator directive 2026-10-08: "byte-exactness replay
//! is the antithesis of what we are building"; judge each deed live once and
//! carry state forward instead of replaying from genesis).
//!
//! A world's SNAPSHOT is its committed state as the live world held it at a
//! chronicle boundary — `<world>/state.snap`:
//!
//! ```text
//! "netstratum snapshot v1" ‖ receipts u64 LE ‖ head[32] ‖ codec::state_bytes
//! ```
//!
//! where `receipts` is how many receipts the state stands after and `head`
//! the own-hash of the last of them (the commitment: a snapshot belongs to
//! the chronicle whose receipt at that position hashes to it). A world
//! resumes from its snapshot plus the receipts written after it, applied
//! forward once — never from genesis. A world with no snapshot (its first
//! boot under this law) is read the old way, once, and snapshots from then
//! on. The chronicle itself stays whole on disk: it is the civilizations'
//! annals, not the means of resuming.

use cfe::vrp::Chronicle;
use cfe::world::state::Committed;
use cfe::world::{Replayer, World};
use std::io;
use std::path::Path;

const MAGIC: &[u8] = b"netstratum snapshot v1";

/// A carried state and the chronicle position it stands at.
pub struct Snapshot {
    pub receipts: usize,
    pub head: [u8; 32],
    pub state: Committed,
}

/// Write `world`'s committed state at its chronicle's head, atomically.
pub fn write(dir: &Path, world: &World) -> io::Result<()> {
    let receipts = world.chronicle().receipts();
    let Some(last) = receipts.last() else {
        return Ok(()); // an empty chronicle carries nothing
    };
    let mut bytes = MAGIC.to_vec();
    bytes.extend_from_slice(&world.chronicle().total().to_le_bytes());
    bytes.extend_from_slice(&last.own().0);
    bytes.extend_from_slice(&codec::state_bytes(&world.committed()));
    let path = dir.join("state.snap");
    let tmp = dir.join("state.snap.tmp");
    std::fs::write(&tmp, &bytes)?;
    std::fs::rename(&tmp, &path)
}

/// Read `dir`'s snapshot, or `None` when there is none or it does not read.
pub fn read(dir: &Path) -> Option<Snapshot> {
    let bytes = std::fs::read(dir.join("state.snap")).ok()?;
    let rest = bytes.strip_prefix(MAGIC)?;
    let receipts = u64::from_le_bytes(rest.get(..8)?.try_into().ok()?) as usize;
    let head: [u8; 32] = rest.get(8..40)?.try_into().ok()?;
    let state = codec::from_state_bytes(rest.get(40..)?)?;
    Some(Snapshot { receipts, head, state })
}

/// How a world was brought back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resumed {
    /// From its snapshot, applying this many receipts written after it.
    Carried { tail: usize },
    /// No usable snapshot (none yet, or one for another chronicle): read from
    /// the start, once.
    FromStart,
}

/// Bring a world back from its chronicle: from its snapshot when the
/// snapshot belongs to this chronicle (its receipt at the snapshot's position
/// hashes to the snapshot's head), applying only the receipts after it.
pub fn resume(dir: &Path, chronicle: Chronicle) -> (World, Resumed) {
    if let Some(snap) = read(dir) {
        // Positions are ABSOLUTE; the chronicle may hold only a tail from `base`.
        let receipts = chronicle.receipts();
        let base = chronicle.base() as usize;
        let total = base + receipts.len();
        let belongs = snap.receipts > base
            && snap.receipts <= total
            && receipts[snap.receipts - 1 - base].own().0 == snap.head;
        if belongs {
            let tail = total - snap.receipts;
            let mut stepper = Replayer::from_state(snap.state);
            for receipt in &receipts[snap.receipts - base..] {
                stepper.apply(receipt);
            }
            return (World::resume_from(stepper.into_committed(), chronicle), Resumed::Carried { tail });
        }
    }
    (World::resume(chronicle), Resumed::FromStart)
}
