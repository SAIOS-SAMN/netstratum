//! P2-PERSIST — the chronicle on disk (NS-3 §2.4). The file IS the
//! save: an append-only stream of framed receipts. Loading goes
//! through the verification read (a tampered file refuses with its
//! break named); syncing appends ONLY the tail (the file is literally
//! append-only); a divergent file refuses rather than being clobbered.
//! The workspace (accrued, un-stamped ticks) is honestly lost across
//! the boundary: what has no receipt does not persist.

use std::fs;
use std::io::Write;
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
        if tag == crate::vrp::RECEIPT_FRAME_TAG {
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
    let held = chronicle.receipts();
    if on_disk.len() > held.len() {
        return Err(PersistRefused::HistoryDiverged {
            at: held.len() as u64,
        });
    }
    for (at, stored) in on_disk.iter().enumerate() {
        if stored != &held[at] {
            return Err(PersistRefused::HistoryDiverged { at: at as u64 });
        }
    }
    let tail = &held[on_disk.len()..];
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
