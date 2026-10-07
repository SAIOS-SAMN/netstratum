//! A TIER'S LINE — the canonical, hash-chained run of everything a tier has
//! folded (mycelium 78.123, ahead of 2(c)'s challenge window).
//!
//!   head₀ = 32 zero bytes
//!   headₙ = payload_hash("mycelium tier line v1" ‖ headₙ₋₁ ‖ entryₙ)
//!
//! where `entryₙ` is ONE UNDERLYING BATCH — for the `colonies` tier, one
//! chronicle receipt's net change per holding and its declared emission —
//! written canonically (`put_nets(rows) ‖ put_nets(declared)`, children
//! never included). A fold is the sum of every entry since the tier's last
//! admitted fold and its prefix is the head after the last of them, so a
//! challenge re-derives the fold from what it nets: the tier HOLDS its
//! entries, here. A fold nets at most `census::MAX_SPAN` entries, so an
//! availability demand never asks for more than ~1.1 MB.
//!
//! The colonies line's history, as ruled by the coordinator 2026-10-07:
//!   * entries 0 and 1 (deepmat-veil h 243590, rivergate-hearth h 243591)
//!     are COARSE, PRE-CORRECTION: whole-window batches, each equal to its
//!     fold. Grandfathered, never re-folded (there is no supersede act); a
//!     challenge on them is weak, not refused.
//!   * entries 2..23626 (alignment h 243597, understory h 243599) are
//!     per-receipt, but understory's fold nets 23614 of them — it predates
//!     the span bound. Left as admitted.
//!   * the two folds before the line existed (bloomcall-hearth h 203172,
//!     glyph-sky h 203174) are not on it at all: grandfathered "unchallenged,
//!     pre-line".
//!
//! OWED: prune by finality — entries netted only by folds that read FINAL
//! (challenge window passed, nothing open) are archived out of the hot file,
//! keeping the head chain and cursors. Waits on mycelium 2(c), which is what
//! makes a fold's finality readable.
//!
//! On disk (`home/line-<tier>`): one record per entry, appended only after
//! the court admits the fold —
//!   `[head 32][cite 32][len u32 LE][entry len]`
//! — where `cite` is the hash of the netstratum receipt the entry was read
//! from. The cite is not hashed into the line (v1 names only the rows; a
//! source commitment would come as a law for every domain, never a
//! netstratum field); it is the tier's pointer back to the receipt.

use mycelium::anastome::dialogue::uplink::{Net, TierFold};
use std::collections::BTreeMap;
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

    /// The head the line WOULD have after every entry in `batch`.
    pub fn peek_all(&self, batch: &[(Vec<u8>, [u8; 32])]) -> [u8; 32] {
        batch.iter().fold(self.head(), |head, (entry, _)| next_head(&head, entry))
    }

    /// Append every entry in `batch`, in order, as ONE write and one sync —
    /// a fold's entries land together or not at all as far as one write can
    /// promise; returns the final head.
    pub fn append_all(&mut self, batch: Vec<(Vec<u8>, [u8; 32])>) -> io::Result<[u8; 32]> {
        let mut head = self.head();
        let mut bytes = Vec::new();
        let mut landed = Vec::with_capacity(batch.len());
        for (entry, cite) in batch {
            head = next_head(&head, &entry);
            let len = u32::try_from(entry.len()).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "an entry past 4 GiB"))?;
            bytes.extend_from_slice(&head);
            bytes.extend_from_slice(&cite);
            bytes.extend_from_slice(&len.to_le_bytes());
            bytes.extend_from_slice(&entry);
            landed.push(Entry { head, cite, entry });
        }
        let mut f = OpenOptions::new().create(true).append(true).open(&self.path)?;
        f.write_all(&bytes)?;
        f.sync_data()?;
        self.entries.extend(landed);
        Ok(head)
    }

    /// Append `entry` (cited locally by `cite`) and return its head.
    pub fn append(&mut self, entry: Vec<u8>, cite: [u8; 32]) -> io::Result<[u8; 32]> {
        self.append_all(vec![(entry, cite)])
    }
}

/// An entry's canonical bytes: `put_nets(rows) ‖ put_nets(declared)`, each
/// list in its identities' ascending order — the court's own writing
/// (`TierFold::rows_bytes`), so the line and the court can never disagree on
/// an encoding.
pub fn entry_bytes(mut rows: Vec<Net>, mut declared: Vec<Net>) -> Vec<u8> {
    rows.sort_by_key(|(who, _)| *who);
    declared.sort_by_key(|(who, _)| *who);
    TierFold { tier: [0; 32], prefix: [0; 32], rows, declared, children: Vec::new() }.rows_bytes()
}

/// HOW FAR EACH PEOPLE'S CHRONICLE HAS BEEN FOLDED on a tier — so no
/// receipt is netted twice. `home/cursor-<tier>`, one line per people:
/// `people<TAB>receipts folded<TAB>hex hash of the last receipt folded`.
/// The hash is checked against the chronicle before a census resumes from
/// it: a chronicle that no longer holds that receipt there is refused.
pub struct Cursors {
    path: PathBuf,
    pub at: BTreeMap<String, (usize, [u8; 32])>,
}

impl Cursors {
    pub fn open(home: &Path, tier: &str) -> io::Result<Cursors> {
        let path = home.join(format!("cursor-{tier}"));
        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(e),
        };
        let bad = |line: &str| io::Error::new(io::ErrorKind::InvalidData, format!("{}: unreadable cursor {line:?}", path.display()));
        let mut at = BTreeMap::new();
        for line in text.lines().filter(|l| !l.is_empty()) {
            let mut f = line.split('\t');
            let (Some(people), Some(n), Some(hex)) = (f.next(), f.next(), f.next()) else { return Err(bad(line)) };
            let n: usize = n.parse().map_err(|_| bad(line))?;
            let mut hash = [0u8; 32];
            if hex.len() != 64 {
                return Err(bad(line));
            }
            for (i, b) in hash.iter_mut().enumerate() {
                *b = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).map_err(|_| bad(line))?;
            }
            at.insert(people.to_string(), (n, hash));
        }
        Ok(Cursors { path, at })
    }

    /// Record `people` as folded through `n` receipts, the last hashing to
    /// `last`, and write the whole file (small: one line per people).
    pub fn set(&mut self, people: &str, n: usize, last: [u8; 32]) -> io::Result<()> {
        self.at.insert(people.to_string(), (n, last));
        let mut text = String::new();
        for (p, (n, h)) in &self.at {
            let hex: String = h.iter().map(|b| format!("{b:02x}")).collect();
            text.push_str(&format!("{p}\t{n}\t{hex}\n"));
        }
        let tmp = self.path.with_extension("tmp");
        fs::write(&tmp, text)?;
        fs::rename(&tmp, &self.path)
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

    #[test]
    fn a_batch_peeks_the_head_it_appends_and_cursors_round_trip() {
        let home = scratch("batch");
        let mut line = TierLine::open(&home, "colonies").unwrap();
        let batch = vec![(b"a".to_vec(), [1; 32]), (b"b".to_vec(), [2; 32])];
        let peeked = line.peek_all(&batch);
        assert_eq!(line.append_all(batch).unwrap(), peeked);
        assert_eq!(line.entries.len(), 2);

        let mut c = Cursors::open(&home, "colonies").unwrap();
        c.set("amber-veil", 81, [9; 32]).unwrap();
        c.set("glyph-sky", 16597, [10; 32]).unwrap();
        let back = Cursors::open(&home, "colonies").unwrap();
        assert_eq!(back.at.get("amber-veil"), Some(&(81, [9; 32])));
        assert_eq!(back.at.len(), 2);
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn an_entry_writes_as_the_court_writes_a_fold_rows() {
        let rows = vec![([2; 32], [5, 0, 0, 0]), ([1; 32], [-5, 0, 0, 0])];
        let sorted = vec![([1; 32], [-5, 0, 0, 0]), ([2; 32], [5, 0, 0, 0])];
        let court = TierFold { tier: [7; 32], prefix: [8; 32], rows: sorted, declared: vec![], children: vec![] };
        assert_eq!(entry_bytes(rows, vec![]), court.rows_bytes());
    }
}
