//! **CARRIED STATE** (operator directive 2026-10-08: "byte-exactness replay
//! is the antithesis of what we are building"; judge each deed live once and
//! carry state forward instead of replaying from genesis).
//!
//! A world's SNAPSHOT is its committed state as the live world held it at a
//! chronicle boundary — `<world>/state.snap`:
//!
//! ```text
//! "netstratum snapshot v3" ‖ receipts u64 ‖ head[32]
//!   ‖ base u64 ‖ base_bytes u64 ‖ base_parent[32]
//!   ‖ annals_len u64 ‖ annals::Annals::to_carried_bytes ‖ codec::state_bytes
//! ```
//!
//! (integers little-endian) where `receipts` is how many receipts the state
//! stands after and `head` the own-hash of the last of them (the commitment:
//! a snapshot belongs to the chronicle whose receipt at that position hashes
//! to it). The MEMORY BOUND rides beside it: the chronicle held in memory
//! starts at absolute receipt `base`, which begins at byte `base_bytes` of
//! `chronicle.tlv` and chains from `base_parent`; what the live world still
//! reads of the receipts before it is the carried annals. The annals' drained
//! own-hashes are `annals.dig` beside it — 32 bytes each, APPEND-ONLY, never
//! rewritten; the snapshot names how many belong to it. A resume reads the
//! file from `base_bytes` only. v2 (digests inline) and v1 (no tail, no
//! annals) snapshots still read. A world
//! resumes from its snapshot plus the receipts written after it, applied
//! forward once — never from genesis. A world with no snapshot (its first
//! boot under this law) is read the old way, once, and snapshots from then
//! on. The chronicle itself stays whole on disk: it is the civilizations'
//! annals, not the means of resuming.

use cfe::annals::Annals;
use cfe::persist::{SyncCursor, TailCursor};
use cfe::vrp::{Chronicle, ReceiptHash};
use cfe::world::state::Committed;
use cfe::world::{Replayer, World};
use std::io;
use std::path::Path;

const MAGIC_V1: &[u8] = b"netstratum snapshot v1";
const MAGIC_V2: &[u8] = b"netstratum snapshot v2";
const MAGIC: &[u8] = b"netstratum snapshot v3";
const DIGESTS: &str = "annals.dig";

/// Where a snapshot's chronicle tail begins.
pub struct Tail {
    pub base: u64,
    pub base_bytes: u64,
    pub base_parent: ReceiptHash,
    pub annals: Annals,
}

/// A carried state and the chronicle position it stands at.
pub struct Snapshot {
    pub receipts: usize,
    pub head: [u8; 32],
    pub tail: Option<Tail>,
    pub state: Committed,
}

/// Write `world`'s committed state at its chronicle's head, atomically.
/// `file_end` is the byte length of `chronicle.tlv` holding every receipt
/// through that head (the sync cursor just after a sync), from which the
/// held tail's starting byte is counted back.
pub fn write(dir: &Path, world: &World, file_end: u64) -> io::Result<()> {
    let chronicle = world.chronicle();
    let receipts = chronicle.receipts();
    let Some(last) = receipts.last() else {
        return Ok(()); // an empty chronicle carries nothing
    };
    let held_bytes: u64 = receipts.iter().map(|r| r.wire_bytes().len() as u64).sum();
    let base_bytes = file_end.checked_sub(held_bytes).ok_or_else(|| io::Error::other("file shorter than the held tail"))?;
    append_digests(dir, chronicle.annals().digests())?;
    let annals = chronicle.annals().to_carried_bytes();
    let mut bytes = MAGIC.to_vec();
    bytes.extend_from_slice(&chronicle.total().to_le_bytes());
    bytes.extend_from_slice(&last.own().0);
    bytes.extend_from_slice(&chronicle.base().to_le_bytes());
    bytes.extend_from_slice(&base_bytes.to_le_bytes());
    bytes.extend_from_slice(&chronicle.base_parent().0);
    bytes.extend_from_slice(&(annals.len() as u64).to_le_bytes());
    bytes.extend_from_slice(&annals);
    bytes.extend_from_slice(&codec::state_bytes(&world.committed()));
    let path = dir.join("state.snap");
    let tmp = dir.join("state.snap.tmp");
    std::fs::write(&tmp, &bytes)?;
    std::fs::rename(&tmp, &path)
}

/// Bring `annals.dig` to exactly `digests`, appending only what it lacks. A
/// file holding more than the annals (written, then the snapshot naming it
/// never landed) is cut back to them first; nothing already written moves.
fn append_digests(dir: &Path, digests: &[[u8; 32]]) -> io::Result<()> {
    use std::io::Write;
    let path = dir.join(DIGESTS);
    let file = std::fs::OpenOptions::new().create(true).append(true).open(&path)?;
    let held = (file.metadata()?.len() / 32) as usize;
    if held > digests.len() || file.metadata()?.len() % 32 != 0 {
        file.set_len((held.min(digests.len()) * 32) as u64)?;
    }
    let held = held.min(digests.len());
    let mut file = file;
    let mut out = Vec::with_capacity((digests.len() - held) * 32);
    for d in &digests[held..] {
        out.extend_from_slice(d);
    }
    file.write_all(&out)?;
    file.flush()
}

/// The first `count` own-hashes of `annals.dig`.
fn read_digests(dir: &Path, count: usize) -> Option<Vec<[u8; 32]>> {
    use std::io::Read;
    let mut file = std::fs::File::open(dir.join(DIGESTS)).ok()?;
    let mut bytes = vec![0u8; count.checked_mul(32)?];
    file.read_exact(&mut bytes).ok()?;
    Some(bytes.chunks_exact(32).map(|c| c.try_into().expect("32 bytes")).collect())
}

/// Read `dir`'s snapshot (v3, v2 with inline digests, or v1 without a tail),
/// or `None` when there is none or it does not read.
pub fn read(dir: &Path) -> Option<Snapshot> {
    let bytes = std::fs::read(dir.join("state.snap")).ok()?;
    let u64_at = |b: &[u8], at: usize| -> Option<u64> { Some(u64::from_le_bytes(b.get(at..at + 8)?.try_into().ok()?)) };
    if let Some(rest) = bytes.strip_prefix(MAGIC_V1) {
        let receipts = u64_at(rest, 0)? as usize;
        let head: [u8; 32] = rest.get(8..40)?.try_into().ok()?;
        let state = codec::from_state_bytes(rest.get(40..)?)?;
        return Some(Snapshot { receipts, head, tail: None, state });
    }
    let (rest, carried) = match bytes.strip_prefix(MAGIC) {
        Some(rest) => (rest, true),
        None => (bytes.strip_prefix(MAGIC_V2)?, false),
    };
    let receipts = u64_at(rest, 0)? as usize;
    let head: [u8; 32] = rest.get(8..40)?.try_into().ok()?;
    let base = u64_at(rest, 40)?;
    let base_bytes = u64_at(rest, 48)?;
    let base_parent = ReceiptHash(rest.get(56..88)?.try_into().ok()?);
    let annals_len = u64_at(rest, 88)? as usize;
    let annals_bytes = rest.get(96..96usize.checked_add(annals_len)?)?;
    let annals = match carried {
        true => {
            let count = Annals::carried_digest_count(annals_bytes)?;
            Annals::from_carried_bytes(annals_bytes, read_digests(dir, count)?)?
        }
        false => Annals::from_bytes(annals_bytes)?,
    };
    let state = codec::from_state_bytes(rest.get(96 + annals_len..)?)?;
    Some(Snapshot { receipts, head, tail: Some(Tail { base, base_bytes, base_parent, annals }), state })
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
        if let Some(carried) = carry_forward(snap, &chronicle) {
            return (World::resume_from(carried.0, chronicle), carried.1);
        }
    }
    (World::resume(chronicle), Resumed::FromStart)
}

/// The snapshot's state stepped forward over the receipts after it, when the
/// snapshot belongs to this chronicle.
fn carry_forward(snap: Snapshot, chronicle: &Chronicle) -> Option<(Committed, Resumed)> {
    // Positions are ABSOLUTE; the chronicle may hold only a tail from `base`.
    let receipts = chronicle.receipts();
    let base = chronicle.base() as usize;
    let total = base + receipts.len();
    let belongs = snap.receipts > base
        && snap.receipts <= total
        && receipts[snap.receipts - 1 - base].own().0 == snap.head;
    if !belongs {
        return None;
    }
    let mut stepper = Replayer::from_state(snap.state);
    for receipt in &receipts[snap.receipts - base..] {
        stepper.apply(receipt);
    }
    Some((stepper.into_committed(), Resumed::Carried { tail: total - snap.receipts }))
}

/// **THE BOUNDED BOOT** — bring a world back from its directory reading only
/// the chronicle tail its snapshot names: the receipts from `base_bytes` on,
/// verified from `base_parent`, the annals carried for the rest. Returns the
/// world, how it came back, and the sync cursor standing at the file's end
/// (so the first sync never re-reads the drained prefix). With no v2
/// snapshot, or one whose tail does not read and verify, the whole file is
/// read once, the old way. `None` when the world has no readable chronicle.
pub fn boot(dir: &Path) -> Option<(World, Resumed, SyncCursor)> {
    let carrier = dir.join("chronicle.tlv");
    if let Some(snap) = read(dir) {
        if let Some(tail) = &snap.tail {
            let mut cursor = TailCursor {
                bytes: tail.base_bytes,
                receipts: tail.base,
                head: (tail.base > 0).then_some(tail.base_parent),
            };
            if let Ok(held) = cfe::persist::read_tail(&carrier, &mut cursor) {
                if let Ok(mut chronicle) = Chronicle::from_tail(tail.base, tail.base_parent, held) {
                    chronicle.set_annals(tail.annals.clone());
                    let total = chronicle.total() as usize;
                    if let Some((state, resumed)) = carry_forward(snap, &chronicle) {
                        let sync = SyncCursor::proved(total, cursor.bytes);
                        return Some((World::resume_from(state, chronicle), resumed, sync));
                    }
                }
            }
        }
    }
    let chronicle = cfe::persist::load(&carrier).ok()?;
    let (world, resumed) = resume(dir, chronicle);
    Some((world, resumed, SyncCursor::default()))
}
