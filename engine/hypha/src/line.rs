//! A TIER'S LINE — the canonical, hash-chained run of everything a tier has
//! folded (mycelium 78.123, ahead of 2(c)'s challenge window).
//!
//!   head₀ = 32 zero bytes
//!   headₙ = payload_hash("mycelium tier line v1" ‖ headₙ₋₁ ‖ entryₙ)
//!
//! where `entryₙ` is the fold's rows and declared written canonically
//! (`TierFold::rows_bytes`). A fold's prefix is the head after its last
//! entry, so the court's seal commits to the whole line, and an availability
//! challenge is answered by replaying it: the tier HOLDS its rows, here.
//!
//! On disk (`home/line-<tier>`): one record per entry, appended only after
//! the court admits the fold —
//!   `[head 32][cite 32][len u32 LE][entry len]`
//! — where `cite` is netstratum's own chronicle head the entry was read at.
//! The cite is not hashed into the line (the court's line format names only
//! the rows); it is the tier's pointer back to the receipts that ground them.

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const DOMAIN: &[u8] = b"mycelium tier line v1";

/// The head after `entry`, from the head before it.
pub fn next_head(prev: &[u8; 32], entry: &[u8]) -> [u8; 32] {
    let mut bytes = Vec::with_capacity(DOMAIN.len() + 32 + entry.len());
    bytes.extend_from_slice(DOMAIN);
    bytes.extend_from_slice(prev);
    bytes.extend_from_slice(entry);
    mycelium::chronometry::payload_hash(&bytes)
}

/// One record of a tier's line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub head: [u8; 32],
    pub cite: [u8; 32],
    pub entry: Vec<u8>,
}

/// A tier's line, read back whole and re-verified.
pub struct TierLine {
    path: PathBuf,
    pub entries: Vec<Entry>,
}

impl TierLine {
    /// Open `home/line-<tier>`, re-deriving every head from the entries. A
    /// record whose head does not follow is refused, with its index named: a
    /// line that does not replay cannot answer a challenge.
    pub fn open(home: &Path, tier: &str) -> io::Result<TierLine> {
        let path = home.join(format!("line-{tier}"));
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(e),
        };
        let bad = |at: usize, why: &str| io::Error::new(io::ErrorKind::InvalidData, format!("{}: record {at}: {why}", path.display()));
        let mut entries = Vec::new();
        let mut prev = [0u8; 32];
        let mut p = 0;
        while p < bytes.len() {
            let at = entries.len();
            let take = |p: &mut usize, n: usize| -> io::Result<&[u8]> {
                let s = bytes.get(*p..*p + n).ok_or_else(|| bad(at, "truncated"))?;
                *p += n;
                Ok(s)
            };
            let head: [u8; 32] = take(&mut p, 32)?.try_into().map_err(|_| bad(at, "head"))?;
            let cite: [u8; 32] = take(&mut p, 32)?.try_into().map_err(|_| bad(at, "cite"))?;
            let len = u32::from_le_bytes(take(&mut p, 4)?.try_into().map_err(|_| bad(at, "length"))?) as usize;
            let entry = take(&mut p, len)?.to_vec();
            if next_head(&prev, &entry) != head {
                return Err(bad(at, "the head does not follow from the line"));
            }
            prev = head;
            entries.push(Entry { head, cite, entry });
        }
        Ok(TierLine { path, entries })
    }

    /// The head after the last entry (zero for an empty line).
    pub fn head(&self) -> [u8; 32] {
        self.entries.last().map_or([0; 32], |e| e.head)
    }

    /// The head the line WOULD have after `entry` — the prefix a fold of it
    /// signs. Nothing is written.
    pub fn peek(&self, entry: &[u8]) -> [u8; 32] {
        next_head(&self.head(), entry)
    }

    /// Append `entry` (read at chronicle head `cite`) and return its head.
    pub fn append(&mut self, entry: Vec<u8>, cite: [u8; 32]) -> io::Result<[u8; 32]> {
        let head = self.peek(&entry);
        let len = u32::try_from(entry.len()).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "an entry past 4 GiB"))?;
        let mut record = Vec::with_capacity(68 + entry.len());
        record.extend_from_slice(&head);
        record.extend_from_slice(&cite);
        record.extend_from_slice(&len.to_le_bytes());
        record.extend_from_slice(&entry);
        let mut f = OpenOptions::new().create(true).append(true).open(&self.path)?;
        f.write_all(&record)?;
        f.sync_data()?;
        self.entries.push(Entry { head, cite, entry });
        Ok(head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("ns-hypha-line-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).ok();
        d
    }

    #[test]
    fn the_line_chains_persists_and_refuses_a_tampered_record() {
        let home = scratch("chain");
        let mut line = TierLine::open(&home, "colonies").unwrap();
        assert_eq!(line.head(), [0; 32]);
        let h1 = line.append(b"first".to_vec(), [1; 32]).unwrap();
        let h2 = line.append(b"second".to_vec(), [2; 32]).unwrap();
        assert_eq!(h1, next_head(&[0; 32], b"first"));
        assert_eq!(h2, next_head(&h1, b"second"));
        let back = TierLine::open(&home, "colonies").unwrap();
        assert_eq!(back.head(), h2);
        assert_eq!(back.entries.len(), 2);

        let path = home.join("line-colonies");
        let mut bytes = fs::read(&path).unwrap();
        let last = bytes.len() - 1;
        bytes[last] ^= 1;
        fs::write(&path, bytes).unwrap();
        assert!(TierLine::open(&home, "colonies").is_err());
        let _ = fs::remove_dir_all(&home);
    }
}
