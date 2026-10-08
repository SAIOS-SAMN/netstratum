//! P2-PERSIST — the chronicle on disk (NS-3 §2.4). The file IS the
//! save: an append-only stream of framed receipts. Loading goes
//! through the verification read (a tampered file refuses with its
//! break named); syncing appends ONLY the tail (the file is literally
//! append-only); a divergent file refuses rather than being clobbered.
//! The workspace (accrued, un-stamped ticks) is honestly lost across
//! the boundary: what has no receipt does not persist.

use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::tlv;
use crate::vrp::{ChainRead, Chronicle, Receipt};

/// A typed persistence refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistRefused {
    /// The filesystem said no (the message carried whole).
    Io { message: String },
    /// A frame did not parse as a receipt.
    Malformed { frame: usize },
    /// The stored chain does not re-verify; the first break named.
    Broken { at: u64 },
    /// The file holds history this chronicle does not — refusing to
    /// clobber a longer or different past.
    HistoryDiverged { at: u64 },
}

/// Load a chronicle from its file — through the one door:
/// `Chronicle::from_receipts` re-verifies every link. Unknown tags in
/// the stream are skipped (SEAL A: future frames are future content).
pub fn load(path: &Path) -> Result<Chronicle, PersistRefused> {
    let bytes = fs::read(path).map_err(|error| PersistRefused::Io {
        message: error.to_string(),
    })?;
    let mut receipts = Vec::new();
    for (frame, (tag, value)) in tlv::records(&bytes).enumerate() {
        if tag == crate::vrp::RECEIPT_FRAME_TAG || tag == crate::vrp::RECEIPT_FRAME_TAG_V2 {
            let receipt =
                Receipt::from_wire(tag, value).ok_or(PersistRefused::Malformed { frame })?;
            receipts.push(receipt);
        }
    }
    Chronicle::from_receipts(receipts).map_err(|read| match read {
        ChainRead::Broken { at } => PersistRefused::Broken { at },
        ChainRead::Intact => unreachable!("from_receipts errs only on breaks"),
    })
}

/// Sync a chronicle to its file, appending ONLY the receipts the file
/// does not yet hold. Returns how many were appended. If the file
/// holds receipts this chronicle does not (longer, or a different
/// history at any index), the sync REFUSES — a divergent past is never
/// clobbered silently.
pub fn sync(chronicle: &Chronicle, path: &Path) -> Result<usize, PersistRefused> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| PersistRefused::Io {
            message: error.to_string(),
        })?;
    }
    let on_disk = match path.exists() {
        true => load(path)?.receipts().to_vec(),
        false => Vec::new(),
    };
    // The held receipts start at absolute position `base` (the ones before
    // were drained to the annals); the file must hold that prefix, agree
    // where the two overlap, and end where the drained prefix ended.
    let held = chronicle.receipts();
    let base = chronicle.base() as usize;
    let total = base + held.len();
    if on_disk.len() > total {
        return Err(PersistRefused::HistoryDiverged { at: total as u64 });
    }
    if on_disk.len() < base {
        return Err(PersistRefused::HistoryDiverged { at: on_disk.len() as u64 });
    }
    if base > 0 && on_disk[base - 1].own() != chronicle.base_parent() {
        return Err(PersistRefused::HistoryDiverged { at: (base - 1) as u64 });
    }
    for (at, stored) in on_disk.iter().enumerate().skip(base) {
        if stored != &held[at - base] {
            return Err(PersistRefused::HistoryDiverged { at: at as u64 });
        }
    }
    let tail = &held[on_disk.len() - base..];
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| PersistRefused::Io {
            message: error.to_string(),
        })?;
    for receipt in tail {
        file.write_all(&receipt.wire_bytes())
            .map_err(|error| PersistRefused::Io {
                message: error.to_string(),
            })?;
    }
    file.flush().map_err(|error| PersistRefused::Io {
        message: error.to_string(),
    })?;
    Ok(tail.len())
}

/// **THE SYNC CURSOR** — what THIS PROCESS already proved about a
/// chronicle file, so a later sync never re-reads or re-verifies the
/// bytes it already walked. `sync` re-parses and re-digest-chains the
/// WHOLE on-disk file every call — fine once, ruinous called every
/// beat over a file that only grows (measured: #157, ~3s/civ/beat by
/// beat 11, the dominant share of the per-beat floor). The carrier is
/// append-only and single-writer (the two-writer split NS-5 refuses
/// elsewhere) — a prefix this process itself wrote and verified in an
/// earlier beat cannot have changed under it, so there is nothing to
/// re-prove there. Default is the cold-start cursor (nothing proved
/// yet), which makes the first `sync_from` call identical to `sync`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SyncCursor {
    receipts: usize,
    bytes: u64,
}

/// A READER'S place in a chronicle file another process writes: the bytes
/// and receipts already proved, and the own-hash of the last of them.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TailCursor {
    pub bytes: u64,
    pub receipts: u64,
    pub head: Option<crate::vrp::ReceiptHash>,
}

/// READ ONLY THE NEW TAIL of a chronicle file, verified against what was
/// already proved: the receipts appended since `cursor`, each chained from
/// the cursor's head (`vrp::verify_from`). A frame still being written (a
/// truncated tail) is left for the next read, never half-taken. A file
/// shorter than the cursor, or a tail that does not chain, REFUSES — a
/// reader never follows a rewritten past. A missing file reads empty.
pub fn read_tail(path: &Path, cursor: &mut TailCursor) -> Result<Vec<Receipt>, PersistRefused> {
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(PersistRefused::Io { message: error.to_string() }),
    };
    let len = file.metadata().map(|m| m.len()).map_err(|error| PersistRefused::Io { message: error.to_string() })?;
    if len < cursor.bytes {
        return Err(PersistRefused::HistoryDiverged { at: cursor.receipts });
    }
    file.seek(SeekFrom::Start(cursor.bytes)).map_err(|error| PersistRefused::Io { message: error.to_string() })?;
    let mut tail = Vec::new();
    file.read_to_end(&mut tail).map_err(|error| PersistRefused::Io { message: error.to_string() })?;
    let mut receipts = Vec::new();
    let mut consumed = 0usize;
    for (frame, (tag, value)) in tlv::records(&tail).enumerate() {
        consumed += 5 + value.len();
        if tag == crate::vrp::RECEIPT_FRAME_TAG || tag == crate::vrp::RECEIPT_FRAME_TAG_V2 {
            receipts.push(Receipt::from_wire(tag, value).ok_or(PersistRefused::Malformed { frame })?);
        }
    }
    let parent = cursor.head.unwrap_or(crate::vrp::ReceiptHash::GENESIS);
    if let ChainRead::Broken { at } = crate::vrp::verify_from(parent, &receipts) {
        return Err(PersistRefused::Broken { at });
    }
    cursor.bytes += consumed as u64;
    cursor.receipts += receipts.len() as u64;
    if let Some(last) = receipts.last() {
        cursor.head = Some(last.own());
    }
    Ok(receipts)
}

/// Sync a chronicle to its file, ADDITIONALLY caching how much of it
/// this process has already proved (`cursor`) so a later call in the
/// SAME process skips straight to the unread tail — never reloads or
/// re-verifies the proved prefix. Semantics match `sync` exactly
/// (same refusals, same append-only law, same divergence safety
/// against a shortened or tampered TAIL); only the WORK differs. A
/// cold cursor (`Default`, or a file shorter than what was last
/// proved — rolled back or replaced under this process) falls back to
/// the full `sync` once and seeds the cursor from its result.
pub fn sync_from(
    chronicle: &Chronicle,
    path: &Path,
    cursor: &mut SyncCursor,
) -> Result<usize, PersistRefused> {
    let held = chronicle.receipts();
    let base = chronicle.base() as usize;
    let total = base + held.len();
    let cold = |cursor: &mut SyncCursor| -> Result<usize, PersistRefused> {
        let appended = sync(chronicle, path)?;
        let bytes = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        *cursor = SyncCursor { receipts: total, bytes };
        Ok(appended)
    };
    if cursor.receipts == 0 || !path.exists() {
        return cold(cursor);
    }
    let mut file = fs::File::open(path).map_err(|error| PersistRefused::Io {
        message: error.to_string(),
    })?;
    let file_len = file.metadata().map(|m| m.len()).map_err(|error| PersistRefused::Io {
        message: error.to_string(),
    })?;
    if file_len < cursor.bytes {
        // Shorter than what this process already proved — never trust
        // a shrunk carrier; re-derive the cursor from a full check.
        return cold(cursor);
    }
    file.seek(SeekFrom::Start(cursor.bytes)).map_err(|error| PersistRefused::Io {
        message: error.to_string(),
    })?;
    let mut tail_bytes = Vec::new();
    file.read_to_end(&mut tail_bytes).map_err(|error| PersistRefused::Io {
        message: error.to_string(),
    })?;
    let mut on_disk_tail = Vec::new();
    for (frame, (tag, value)) in tlv::records(&tail_bytes).enumerate() {
        if tag == crate::vrp::RECEIPT_FRAME_TAG || tag == crate::vrp::RECEIPT_FRAME_TAG_V2 {
            let receipt =
                Receipt::from_wire(tag, value).ok_or(PersistRefused::Malformed { frame })?;
            on_disk_tail.push(receipt);
        }
    }
    // The cursor counts ABSOLUTE receipts proved; a cursor behind the
    // drained base (it never proved what was drained) re-derives cold.
    let proved = cursor.receipts.min(total);
    if proved < base {
        return cold(cursor);
    }
    let held_from_cursor = &held[proved - base..];
    if on_disk_tail.len() > held_from_cursor.len() {
        return Err(PersistRefused::HistoryDiverged { at: proved as u64 });
    }
    for (i, stored) in on_disk_tail.iter().enumerate() {
        if stored != &held_from_cursor[i] {
            return Err(PersistRefused::HistoryDiverged { at: (proved + i) as u64 });
        }
    }
    let to_write = &held_from_cursor[on_disk_tail.len()..];
    let mut appended_bytes: u64 = 0;
    if !to_write.is_empty() {
        let mut file = fs::OpenOptions::new().append(true).open(path).map_err(|error| {
            PersistRefused::Io { message: error.to_string() }
        })?;
        for receipt in to_write {
            let wire = receipt.wire_bytes();
            appended_bytes += wire.len() as u64;
            file.write_all(&wire).map_err(|error| PersistRefused::Io {
                message: error.to_string(),
            })?;
        }
        file.flush().map_err(|error| PersistRefused::Io {
            message: error.to_string(),
        })?;
    }
    *cursor = SyncCursor { receipts: total, bytes: file_len + appended_bytes };
    Ok(to_write.len())
}
