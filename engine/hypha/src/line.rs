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
//! THE HOT LINE AND ITS ARCHIVE. The hot file holds only entries some fold
//! may still be asked to publish. It starts from a BASE (`line-<tier>.base`:
//! the head before its first entry, and that entry's index on the whole
//! line), so pruning never breaks the chain:
//!   * PRUNE BY FINALITY ([`TierLine::prune_through`]): entries netted only
//!     by folds the court reads `Final` or `PreLine` are appended to
//!     `line-<tier>.archive` and leave the hot file; the base moves to the
//!     head after the last of them. Never on Provisional, Blocked,
//!     Falsified, Unwound or Unrecorded.
//!   * AN EPOCH ([`TierLine::restart`]): under mycelium 78.125 a tier's first
//!     fold starts its line at the zero head, and every fold sealed before it
//!     reads PreLine. The colonies line's first 31732 entries were archived
//!     whole as `line-colonies.pre-78.125` (coordinator ruling 2026-10-07,
//!     the same rule for every domain) and the hot line restarted at zero.
//!
//! [`FoldLog`] (`folds-<tier>`) keeps each admitted fold's full stamp and the
//! span of the line it nets — finality is read by stamp.
//!
//! On disk (`home/line-<tier>`, and the archives alike): one record per
//! entry, appended only after the court admits the fold —
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

fn put_record(out: &mut Vec<u8>, e: &Entry) -> io::Result<()> {
    let len = u32::try_from(e.entry.len()).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "an entry past 4 GiB"))?;
    out.extend_from_slice(&e.head);
    out.extend_from_slice(&e.cite);
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(&e.entry);
    Ok(())
}

fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    let mut f = fs::File::create(&tmp)?;
    f.write_all(bytes)?;
    f.sync_data()?;
    fs::rename(&tmp, path)
}

/// A tier's hot line, read back whole and re-verified from its base.
pub struct TierLine {
    path: PathBuf,
    base_path: PathBuf,
    /// The head before the first hot entry (zero for a line never pruned).
    pub base: [u8; 32],
    /// The first hot entry's index on the whole line.
    pub offset: u64,
    pub entries: Vec<Entry>,
}

impl TierLine {
    /// Open `home/line-<tier>` from its base, re-deriving every head. A
    /// record whose head does not follow is refused, with its index named: a
    /// line that does not replay cannot answer a challenge.
    pub fn open(home: &Path, tier: &str) -> io::Result<TierLine> {
        let path = home.join(format!("line-{tier}"));
        let base_path = home.join(format!("line-{tier}.base"));
        let (base, offset) = match fs::read(&base_path) {
            Ok(b) if b.len() == 40 => {
                let mut head = [0u8; 32];
                head.copy_from_slice(&b[..32]);
                let mut off = [0u8; 8];
                off.copy_from_slice(&b[32..]);
                (head, u64::from_le_bytes(off))
            }
            Ok(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{}: not a base", base_path.display()))),
            Err(e) if e.kind() == io::ErrorKind::NotFound => ([0; 32], 0),
            Err(e) => return Err(e),
        };
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(e),
        };
        let bad = |at: usize, why: &str| io::Error::new(io::ErrorKind::InvalidData, format!("{}: record {at}: {why}", path.display()));
        let mut entries = Vec::new();
        let mut prev = base;
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
        Ok(TierLine { path, base_path, base, offset, entries })
    }

    /// The head after the last entry (the base for an empty hot line).
    pub fn head(&self) -> [u8; 32] {
        self.entries.last().map_or(self.base, |e| e.head)
    }

    /// Every entry ever appended: archived and hot.
    pub fn total(&self) -> u64 {
        self.offset + self.entries.len() as u64
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
            let e = Entry { head, cite, entry };
            put_record(&mut bytes, &e)?;
            landed.push(e);
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

    fn write_base(&self, head: [u8; 32], offset: u64) -> io::Result<()> {
        let mut b = head.to_vec();
        b.extend_from_slice(&offset.to_le_bytes());
        write_atomic(&self.base_path, &b)
    }

    /// Move the first `n` hot entries to `line-<tier>.archive`; the base
    /// becomes the head after the last of them. The archive is written
    /// first, so a crash leaves an entry in both files, never in neither.
    pub fn prune_through(&mut self, n: usize) -> io::Result<()> {
        if n == 0 {
            return Ok(());
        }
        if n > self.entries.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "pruning past the hot line's end"));
        }
        let mut out = Vec::new();
        for e in &self.entries[..n] {
            put_record(&mut out, e)?;
        }
        let archive = self.path.with_extension("archive");
        let mut f = OpenOptions::new().create(true).append(true).open(&archive)?;
        f.write_all(&out)?;
        f.sync_data()?;
        let (base, offset) = (self.entries[n - 1].head, self.offset + n as u64);
        let mut rest = Vec::new();
        for e in &self.entries[n..] {
            put_record(&mut rest, e)?;
        }
        write_atomic(&self.path, &rest)?;
        self.write_base(base, offset)?;
        self.entries.drain(..n);
        self.base = base;
        self.offset = offset;
        Ok(())
    }

    /// START AN EPOCH: the whole hot line set aside as `line-<tier>.<epoch>`
    /// (kept whole, its own chain from its own base) and the hot line
    /// restarted at the zero head, its indices continuing. Refuses to
    /// overwrite an epoch already set aside.
    pub fn restart(&mut self, epoch: &str) -> io::Result<PathBuf> {
        let kept = self.path.with_extension(epoch);
        if kept.exists() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("{} already holds an epoch", kept.display())));
        }
        if self.path.exists() {
            fs::rename(&self.path, &kept)?;
        }
        let offset = self.total();
        self.write_base([0; 32], offset)?;
        self.entries.clear();
        self.base = [0; 32];
        self.offset = offset;
        Ok(kept)
    }
}

/// ONE ADMITTED FOLD, as its tier keeps it: the court's stamp (its key, the
/// coordinates finality is read by) and the span of the line it nets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldMark {
    pub at: Vec<u8>,
    /// The index on the whole line of the fold's first entry.
    pub first: u64,
    pub count: u64,
}

/// `home/folds-<tier>`: every fold the tier has had admitted, in order —
/// `[key len u16 LE][stamp key][first u64 LE][count u64 LE]` per fold.
pub struct FoldLog {
    path: PathBuf,
    pub folds: Vec<FoldMark>,
}

impl FoldLog {
    pub fn open(home: &Path, tier: &str) -> io::Result<FoldLog> {
        let path = home.join(format!("folds-{tier}"));
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(e),
        };
        let bad = || io::Error::new(io::ErrorKind::InvalidData, format!("{}: truncated", path.display()));
        let mut folds = Vec::new();
        let mut p = 0;
        while p < bytes.len() {
            let mut take = |n: usize| -> io::Result<&[u8]> {
                let s = bytes.get(p..p + n).ok_or_else(bad)?;
                p += n;
                Ok(s)
            };
            let klen = u16::from_le_bytes(take(2)?.try_into().map_err(|_| bad())?) as usize;
            let at = take(klen)?.to_vec();
            let first = u64::from_le_bytes(take(8)?.try_into().map_err(|_| bad())?);
            let count = u64::from_le_bytes(take(8)?.try_into().map_err(|_| bad())?);
            folds.push(FoldMark { at, first, count });
        }
        Ok(FoldLog { path, folds })
    }

    pub fn push(&mut self, mark: FoldMark) -> io::Result<()> {
        let klen = u16::try_from(mark.at.len()).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "a stamp key past 64 KiB"))?;
        let mut b = klen.to_le_bytes().to_vec();
        b.extend_from_slice(&mark.at);
        b.extend_from_slice(&mark.first.to_le_bytes());
        b.extend_from_slice(&mark.count.to_le_bytes());
        let mut f = OpenOptions::new().create(true).append(true).open(&self.path)?;
        f.write_all(&b)?;
        f.sync_data()?;
        self.folds.push(mark);
        Ok(())
    }

    /// The hot entries that may be pruned: walking the folds over the hot
    /// line in order, every one `settled` says yes to, stopping at the first
    /// it does not. Returns how many hot entries those folds' spans cover.
    pub fn prunable(&self, line: &TierLine, mut settled: impl FnMut(&FoldMark) -> bool) -> usize {
        let mut through = line.offset;
        for f in self.folds.iter().filter(|f| f.first + f.count > line.offset) {
            if f.first != through || f.first + f.count > line.total() || !settled(f) {
                break;
            }
            through = f.first + f.count;
        }
        (through - line.offset) as usize
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
    fn pruning_and_an_epoch_keep_the_chain_and_finality_gates_the_prune() {
        let home = scratch("prune");
        let mut line = TierLine::open(&home, "colonies").unwrap();
        let mut log = FoldLog::open(&home, "colonies").unwrap();
        for (i, n) in [2usize, 3, 1].into_iter().enumerate() {
            let first = line.total();
            line.append_all((0..n).map(|j| (vec![i as u8, j as u8], [0; 32])).collect()).unwrap();
            log.push(FoldMark { at: vec![i as u8], first, count: n as u64 }).unwrap();
        }
        let head = line.head();
        // the first two folds settled, the third provisional
        let n = log.prunable(&line, |f| f.at != vec![2]);
        assert_eq!(n, 5);
        line.prune_through(n).unwrap();
        let back = TierLine::open(&home, "colonies").unwrap();
        assert_eq!((back.offset, back.entries.len(), back.head()), (5, 1, head));
        assert_eq!(FoldLog::open(&home, "colonies").unwrap().prunable(&back, |_| true), 1);

        let mut back = back;
        back.restart("pre-test").unwrap();
        assert!(back.restart("pre-test").is_err(), "an epoch is never overwritten");
        let fresh = TierLine::open(&home, "colonies").unwrap();
        assert_eq!((fresh.base, fresh.offset, fresh.entries.len()), ([0; 32], 6, 0));
        assert_eq!(fresh.peek(b"x"), next_head(&[0; 32], b"x"));
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
