//! **THE CARRIED ANNALS** (memory bound, slice 2; the coordinator's history
//! rule: in memory, only the receipts after each world's snapshot — the whole
//! chronicle stays on disk as the annals).
//!
//! When a chronicle DRAINS the receipts a snapshot already carried, what the
//! live world still reads of them is kept here, nothing more:
//!
//!   * every receipt NOT of a [`bulk`] kind, verbatim — the vocabulary, the
//!     solver words and recipes, surveys, parameters, practices, techs,
//!     unions, foundings, famines, yields, vents, upkeep… the kinds history
//!     readers fold over (measured in G2: ~15% of all receipts);
//!   * the own-hash of every drained receipt, in order — a literature claim
//!     may cite ANY receipt the world sealed (claims are rare; the list is
//!     scanned, 32 bytes a receipt, no hashed index). On disk they are an
//!     APPEND-ONLY file beside the chronicle (the snapshot names how many
//!     it holds), never rewritten;
//!   * the latest aperture stamp per orb (the crystal sheet's `last_link`)
//!     and the count of drained assays (the office evidence) — the only two
//!     things any reader takes from a bulk kind.
//!
//! The bulk kinds are the market's per-beat traffic: the four aperture legs,
//! the leisure and population-accrual settles, assays, element wants and STAMP
//! mints — measured at ~85% of G2's 4.0M receipts, read by no history reader
//! beyond the two values above.

use crate::dop::ClosureEvent;
use crate::vrp::Receipt;
use std::collections::BTreeMap;

/// Is this a BULK kind — dropped from memory once drained, its record on disk?
pub fn bulk(event: &ClosureEvent) -> bool {
    matches!(
        event,
        ClosureEvent::ApertureSent { .. }
            | ClosureEvent::ApertureReceived { .. }
            | ClosureEvent::StampApertureSent { .. }
            | ClosureEvent::StampApertureReceived { .. }
            | ClosureEvent::LeisureSettled { .. }
            | ClosureEvent::PopulationAccrued { .. }
            | ClosureEvent::AssayStruck { .. }
            | ClosureEvent::ElementWantStruck { .. }
            | ClosureEvent::ElementWantRelieved { .. }
            | ClosureEvent::StampMinted { .. }
            // docs/64 P2b: read by replay alone (the population settle, the
            // upkeep toll) or by the crystal sheet's last link, which the
            // annals keep per orb.
            | ClosureEvent::PopulationSettled { .. }
            | ClosureEvent::UpkeepSettled { .. }
            | ClosureEvent::BondSettled { .. }
            | ClosureEvent::BondSealed { .. }
    )
}

/// THE WINDOWED KINDS (docs/64 P2): drained receipts every reader wants only
/// within a recent window — the collected yields and the vents, read by the
/// crystal deposit sheet over its deposit window and by the elevation's year.
/// The annals keep them only within [`WINDOW`] stamps of the newest drained
/// receipt; older ones are on disk.
pub fn windowed(event: &ClosureEvent) -> bool {
    matches!(event, ClosureEvent::YieldCollected { .. } | ClosureEvent::VentSettled { .. })
}

/// The longest window any reader of a windowed kind takes: the crystal sheet's
/// deposit window at its widest (`World::deposit_window`).
pub const WINDOW: u64 = 240;

/// What a chronicle still carries of the receipts it drained.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Annals {
    /// The kept receipts as their wire frames, concatenated (the snapshot's
    /// own encoding), and where each frame starts: a lifetime of receipts is
    /// never held as objects; a reader decodes the ones it walks.
    kept_wire: Vec<u8>,
    kept_at: Vec<u32>,
    /// The last handoff seal drained (the hand it named, its sequence): the
    /// mesh's walk starts from it without decoding the kept history.
    last_seal: Option<(u64, u64)>,
    /// Drained own-hashes NOT YET FILED in the digest file — the only ones
    /// memory holds (the memory bound: the file is the record).
    digests: Vec<[u8; 32]>,
    last_link: BTreeMap<u64, u64>,
    assays: u64,
    /// How many own-hashes the digest file already holds, and where it is.
    filed: u64,
    file: Option<std::path::PathBuf>,
}

impl Annals {
    /// Absorb receipts drained from the chronicle, oldest first; then the
    /// windowed kinds older than the window let go.
    pub fn absorb(&mut self, drained: Vec<Receipt>) {
        let newest = drained.last().map(|r| r.stamp());
        self.absorb_all(drained);
        if let Some(newest) = newest {
            self.prune_windowed(newest.saturating_sub(WINDOW));
        }
    }

    /// Keep the windowed kinds from `floor` on; every other kept frame stays.
    fn prune_windowed(&mut self, floor: u64) {
        let mut wire = Vec::with_capacity(self.kept_wire.len());
        let mut at = Vec::with_capacity(self.kept_at.len());
        for i in 0..self.kept_at.len() {
            let start = self.kept_at[i] as usize;
            let end = self.kept_at.get(i + 1).map_or(self.kept_wire.len(), |n| *n as usize);
            let keep = match self.kept_receipt(i) {
                // A kind now bulk leaves the kept frames (older snapshots
                // carried it); a windowed kind leaves once outside the window.
                Some(r) => !(bulk(r.event()) || (windowed(r.event()) && r.stamp() < floor)),
                None => true,
            };
            if keep {
                at.push(wire.len() as u32);
                wire.extend_from_slice(&self.kept_wire[start..end]);
            }
        }
        self.kept_wire = wire;
        self.kept_at = at;
    }

    fn absorb_all(&mut self, drained: Vec<Receipt>) {
        for r in drained {
            self.digests.push(r.own().0);
            match r.event() {
                ClosureEvent::ApertureSent { from_orb: orb, .. }
                | ClosureEvent::ApertureReceived { to_orb: orb, .. }
                | ClosureEvent::BondSealed { source_orb: orb, .. }
                | ClosureEvent::BondSettled { source_orb: orb, .. } => {
                    self.last_link.insert(*orb, r.stamp());
                }
                ClosureEvent::AssayStruck { .. } => self.assays += 1,
                _ => {}
            }
            if let ClosureEvent::HandoffSealed { to, .. } = r.event() {
                self.last_seal = Some((*to, r.sequence()));
            }
            if !bulk(r.event()) {
                self.kept_at.push(self.kept_wire.len() as u32);
                self.kept_wire.extend_from_slice(&r.wire_bytes());
            }
        }
    }

    /// How many drained receipts of non-bulk kinds are kept.
    pub fn kept_len(&self) -> usize {
        self.kept_at.len()
    }

    /// The `i`-th kept receipt, decoded from its frame.
    pub fn kept_receipt(&self, i: usize) -> Option<Receipt> {
        let at = *self.kept_at.get(i)? as usize;
        let (tag, value) = crate::tlv::records(self.kept_wire.get(at..)?).next()?;
        Receipt::from_wire(tag, value)
    }

    /// The drained receipts of every non-bulk kind, oldest first, each
    /// decoded as it is reached (walk backward from the end to read only the
    /// recent ones).
    pub fn kept_iter(&self) -> impl DoubleEndedIterator<Item = Receipt> + Clone + '_ {
        (0..self.kept_at.len()).filter_map(move |i| self.kept_receipt(i))
    }

    /// The last handoff seal among the drained receipts: (hand, sequence).
    pub fn last_seal(&self) -> Option<(u64, u64)> {
        self.last_seal
    }

    /// The kept frames' bytes, held.
    pub fn kept_bytes(&self) -> usize {
        self.kept_wire.len()
    }

    /// Did the world ever seal a drained receipt with this own-hash? The
    /// unfiled ones in memory first, then the digest file, streamed — claims
    /// are rare, and the file is never held whole.
    pub fn sealed(&self, digest: &[u8; 32]) -> bool {
        if self.digests.contains(digest) {
            return true;
        }
        let Some(path) = &self.file else { return false };
        let Ok(file) = std::fs::File::open(path) else { return false };
        use std::io::Read;
        let mut reader = std::io::BufReader::new(file).take(self.filed * 32);
        let mut one = [0u8; 32];
        while reader.read_exact(&mut one).is_ok() {
            if one == *digest {
                return true;
            }
        }
        false
    }

    /// Every own-hash drained so far, filed or not.
    pub fn digest_count(&self) -> u64 {
        self.filed + self.digests.len() as u64
    }

    /// The own-hashes not yet filed, oldest first.
    pub fn unfiled(&self) -> &[[u8; 32]] {
        &self.digests
    }

    /// Where the digest file stands and how much of it is this annals'.
    pub fn filed(&self) -> (Option<&std::path::Path>, u64) {
        (self.file.as_deref(), self.filed)
    }

    /// The unfiled own-hashes have reached `path` (which now holds
    /// `digest_count()` of them): memory lets them go.
    pub fn mark_filed(&mut self, path: std::path::PathBuf) {
        self.filed += self.digests.len() as u64;
        self.digests = Vec::new();
        self.file = Some(path);
    }

    /// The own-hash of the first receipt drained (the chronicle's genesis
    /// once anything drained).
    pub fn first_digest(&self) -> Option<[u8; 32]> {
        if self.filed > 0 {
            let mut one = [0u8; 32];
            use std::io::Read;
            let path = self.file.as_ref()?;
            std::fs::File::open(path).ok()?.read_exact(&mut one).ok()?;
            return Some(one);
        }
        self.digests.first().copied()
    }

    /// The latest drained aperture stamp per orb.
    pub fn last_links(&self) -> &BTreeMap<u64, u64> {
        &self.last_link
    }

    /// How many assays were drained.
    pub fn assays(&self) -> u64 {
        self.assays
    }

    /// Plain bytes for the snapshot, the digests LEFT OUT (they live in their
    /// append-only file): `kept count u32 ‖ frames ‖ digest count u64 ‖
    /// links u32 ‖ (orb,stamp)·n ‖ assays u64`, little-endian.
    pub fn to_carried_bytes(&self) -> Vec<u8> {
        let mut out = (self.kept_at.len() as u32).to_le_bytes().to_vec();
        out.extend_from_slice(&self.kept_wire);
        out.extend_from_slice(&self.digest_count().to_le_bytes());
        self.put_tail(&mut out);
        out
    }

    /// Read [`Annals::to_carried_bytes`] back, its digests left IN THE FILE at
    /// `file` (which must hold at least the count the bytes name), or `None`.
    pub fn from_carried_bytes(b: &[u8], file: std::path::PathBuf) -> Option<Annals> {
        let ((kept_wire, kept_at, last_seal), mut p) = Self::read_kept(b)?;
        let count = u64::from_le_bytes(b.get(p..p + 8)?.try_into().ok()?);
        p += 8;
        if std::fs::metadata(&file).map(|m| m.len()).unwrap_or(0) < count * 32 {
            return None;
        }
        let (last_link, assays) = Self::read_tail(b, &mut p)?;
        (p == b.len()).then_some(Annals { kept_wire, kept_at, last_seal, digests: Vec::new(), last_link, assays, filed: count, file: Some(file) })
    }

    /// How many digests a [`Annals::to_carried_bytes`] payload names (so its
    /// reader knows how much of the digest file to take).
    pub fn carried_digest_count(b: &[u8]) -> Option<usize> {
        let (_, p) = Self::read_kept(b)?;
        usize::try_from(u64::from_le_bytes(b.get(p..p + 8)?.try_into().ok()?)).ok()
    }

    fn put_tail(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.last_link.len() as u32).to_le_bytes());
        for (orb, stamp) in &self.last_link {
            out.extend_from_slice(&orb.to_le_bytes());
            out.extend_from_slice(&stamp.to_le_bytes());
        }
        out.extend_from_slice(&self.assays.to_le_bytes());
    }

    /// The kept frames, each verified to decode (the object is dropped at
    /// once: memory keeps only the bytes), and where they end.
    fn read_kept(b: &[u8]) -> Option<((Vec<u8>, Vec<u32>, Option<(u64, u64)>), usize)> {
        let n = u32::from_le_bytes(b.get(..4)?.try_into().ok()?) as usize;
        let mut p = 4usize;
        let start = p;
        let mut at = Vec::with_capacity(n.min(1 << 20));
        let mut seal = None;
        for _ in 0..n {
            let (tag, value) = crate::tlv::records(b.get(p..)?).next()?;
            let r = Receipt::from_wire(tag, value)?;
            if let ClosureEvent::HandoffSealed { to, .. } = r.event() {
                seal = Some((*to, r.sequence()));
            }
            at.push((p - start) as u32);
            p += 5 + value.len();
        }
        Some(((b.get(start..p)?.to_vec(), at, seal), p))
    }

    fn read_tail(b: &[u8], p: &mut usize) -> Option<(BTreeMap<u64, u64>, u64)> {
        let u64_at = |p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
        *p += 4;
        let mut last_link = BTreeMap::new();
        for _ in 0..n {
            let orb = u64_at(p)?;
            last_link.insert(orb, u64_at(p)?);
        }
        Some((last_link, u64_at(p)?))
    }

    /// Plain bytes for the snapshot beside it:
    /// `kept count u32 ‖ frames ‖ digests u32 ‖ 32·n ‖ links u32 ‖ (orb,stamp)·n ‖ assays u64`,
    /// every integer little-endian, each kept receipt one TLV frame.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = (self.kept_at.len() as u32).to_le_bytes().to_vec();
        out.extend_from_slice(&self.kept_wire);
        out.extend_from_slice(&(self.digests.len() as u32).to_le_bytes());
        for d in &self.digests {
            out.extend_from_slice(d);
        }
        out.extend_from_slice(&(self.last_link.len() as u32).to_le_bytes());
        for (orb, stamp) in &self.last_link {
            out.extend_from_slice(&orb.to_le_bytes());
            out.extend_from_slice(&stamp.to_le_bytes());
        }
        out.extend_from_slice(&self.assays.to_le_bytes());
        out
    }

    /// Read [`Annals::to_bytes`] back whole, or `None`.
    pub fn from_bytes(b: &[u8]) -> Option<Annals> {
        let u32_at = |p: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v)
        };
        let u64_at = |p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let ((kept_wire, kept_at, last_seal), mut p) = Self::read_kept(b)?;
        let n = u32_at(&mut p)? as usize;
        let mut digests = Vec::with_capacity(n.min(1 << 24));
        for _ in 0..n {
            digests.push(b.get(p..p + 32)?.try_into().ok()?);
            p += 32;
        }
        let n = u32_at(&mut p)? as usize;
        let mut last_link = BTreeMap::new();
        for _ in 0..n {
            let orb = u64_at(&mut p)?;
            last_link.insert(orb, u64_at(&mut p)?);
        }
        let assays = u64_at(&mut p)?;
        (p == b.len()).then_some(Annals { kept_wire, kept_at, last_seal, digests, last_link, assays, filed: 0, file: None })
    }
}
