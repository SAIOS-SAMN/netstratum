//! NS-3 VRP — the receipt chain and the attested world clock.
//!
//! The append-only law is structural: `Chronicle::append` is the one
//! write, and it computes both the parent link and the receipt's own
//! digest internally — callers cannot supply either. Time is a separate
//! organ: the clock walks ONLY by 3-of-3 unanimous attestation of the
//! exact successor (NS-3 §1.3); `append` has no clock access, so a
//! receipt is stamped with the value it is GIVEN and can never move time.

use crate::dop::ClosureEvent;

/// One timekeeper's answer, carried whole in an open quorum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attestation {
    Attests { next: u64 },
    Refuses,
}

/// A timekeeper reads the clock and attests its exact successor.
///
/// **THE SEATED QUORUM IS UNANIMOUS BY CONSTRUCTION — a placeholder,
/// declared 2026-08-03, not a law being enforced.** A world seats three
/// keepers and its tick asks all three (`world/tick.rs`, the one
/// production caller). They share ONE clock in ONE process, `attest`
/// consults nothing but that clock, and so the three answers are always
/// identical: `TickRead::QuorumOpen` is unreachable through this
/// method, and the live clock always walks. `id` is carried and never
/// read for the same reason — there is nothing yet for keepers to
/// differ ABOUT.
///
/// This is an absent SOURCE, not an absent judgment. The tick law lives
/// in [`WorldClock::tick`], which refuses 2-of-3, a skip and a replay on
/// the attestations it is handed, exactly as NS-3 §3.1 vectors it —
/// witnessed by `the_clock_refuses_partial_skip_and_replay`
/// (NS-3 §4 conformance clause c) and pinned as a placeholder by
/// `the_seated_keepers_cannot_disagree_today`, both in
/// `tests/ns3_conformance.rs`.
///
/// Keepers become real parties when identity does — NS-5's separate
/// processes, NS-7's cryptographic identity. Until then: do not read a
/// live quorum here, and do not invent one. A rule picking which keeper
/// refuses would be authored physics, which is exactly what the
/// scalar/scored provenance refusal bars.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timekeeper {
    pub id: u8,
}

impl Timekeeper {
    /// Reads the clock, attests its exact successor. Deliberately total
    /// and deliberately blind to `self.id` — see the type's note: one
    /// process gives three keepers one input, so the honest answer is
    /// the same answer.
    pub fn attest(&self, clock: &WorldClock) -> Attestation {
        Attestation::Attests {
            next: clock.now() + 1,
        }
    }
}

/// The tick's typed read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickRead {
    /// 3-of-3 unanimous on the exact successor — the clock walked.
    Ticked { now: u64 },
    /// The quorum did not close; the clock did not move. (2-of-3, a
    /// refusal, or a unanimous non-successor — no jump, no replay.)
    QuorumOpen { attestations: [Attestation; 3] },
}

/// The world clock (NS-3 §1.3): constructible at zero, walked only by
/// quorum.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorldClock {
    value: u64,
}

impl WorldClock {
    pub fn new() -> Self {
        Self::default()
    }

    /// Boot-time reinstatement (P2-PERSIST): the clock resumes at the
    /// highest STAMPED value the chronicle proves — every tick up to it
    /// was attested live in the era that stamped it. Ticks after the
    /// last receipt were workspace-time and are honestly lost (what has
    /// no receipt does not persist). This is not a walk: no attestation
    /// is forged; a proven past is re-read.
    pub fn resume(value: u64) -> Self {
        Self { value }
    }

    #[inline]
    pub fn now(&self) -> u64 {
        self.value
    }

    pub fn tick(&mut self, attestations: &[Attestation; 3]) -> TickRead {
        let successor = self.value + 1;
        let agreeing = attestations
            .iter()
            .filter(|attestation| matches!(attestation, Attestation::Attests { next } if *next == successor))
            .count();
        match agreeing {
            3 => {
                self.value = successor;
                TickRead::Ticked { now: self.value }
            }
            _ => TickRead::QuorumOpen {
                attestations: *attestations,
            },
        }
    }
}

/// The chain identity — a BLAKE3 digest, lawful at this layer only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReceiptHash(pub [u8; 32]);

impl ReceiptHash {
    /// The genesis parent — the named zero.
    pub const GENESIS: Self = Self([0u8; 32]);
}

/// The pre-TU-1 receipt shape (NS-3 §1.1) — kept byte-identical so the
/// historical chronicle (every receipt written before the {S,G,T}
/// amendment) replays exactly. Never constructed by new code; `V1` reads
/// old chronicles, `Chronicle::append` only ever writes `V2` going
/// forward (TU-1a ruling 2026-08-22: a direct restructure, not additive
/// optional fields — the ledger trunk carries no structural ambiguity).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyReceipt {
    pub sequence: u64,
    pub stamp: u64,
    pub parent: ReceiptHash,
    pub own: ReceiptHash,
    pub event: ClosureEvent,
    pub provenance: Vec<u64>,
}

/// The {S,G,T} receipt (doc 55, TU-1). `stamp` stays T's local-clock
/// half (unchanged). `region_seed` is S — the COARSE galactic cell
/// (GALACTIC-1's `dop::region_seed_of`), never exact `(x,y,z)`: Čech
/// cohomology is about which cover a civilization overlaps, not point
/// masses (TU-1b ruling). `gauge` is G — a beat-level snapshot (never
/// per-deed: `reduct::GaugeDirection` is a live, freshly-recomputed
/// cohomology peel with no stable per-world identity, and a beat is
/// this engine's indivisible quantum of macro-time, so gauge formally
/// settles at the beat boundary, not mid-beat — TU-1c ruling). Carried
/// as raw index bytes (never the `reduct::GaugeDirection` type itself)
/// because `cfe` does not and should not depend on `reduct` — the
/// cognition crate stays downstream of the world crate, not beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifoldReceipt {
    pub sequence: u64,
    pub stamp: u64,
    pub parent: ReceiptHash,
    pub own: ReceiptHash,
    pub event: ClosureEvent,
    pub provenance: Vec<u64>,
    pub region_seed: [u8; 32],
    pub gauge: Vec<u8>,
}

/// One receipt (NS-3 §1.1). A direct restructure (TU-1a), not additive
/// versioning: `V1` is every receipt ever written before this
/// amendment, byte-identical on the wire and in the digest; `V2` is
/// every receipt `Chronicle::append` writes from here forward. Use the
/// accessor methods (`.stamp()`, `.event()`, …) for the fields both
/// variants share — they read as fields did before this change, just
/// through one extra dispatch. `.region_seed()`/`.gauge()` are `None`
/// for `V1` (a world that predates the regional/gauge axes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Receipt {
    V1(LegacyReceipt),
    V2(ManifoldReceipt),
}

impl Receipt {
    pub fn sequence(&self) -> u64 {
        match self {
            Receipt::V1(r) => r.sequence,
            Receipt::V2(r) => r.sequence,
        }
    }
    pub fn stamp(&self) -> u64 {
        match self {
            Receipt::V1(r) => r.stamp,
            Receipt::V2(r) => r.stamp,
        }
    }
    pub fn parent(&self) -> ReceiptHash {
        match self {
            Receipt::V1(r) => r.parent,
            Receipt::V2(r) => r.parent,
        }
    }
    pub fn own(&self) -> ReceiptHash {
        match self {
            Receipt::V1(r) => r.own,
            Receipt::V2(r) => r.own,
        }
    }
    pub fn event(&self) -> &ClosureEvent {
        match self {
            Receipt::V1(r) => &r.event,
            Receipt::V2(r) => &r.event,
        }
    }
    pub fn provenance(&self) -> &[u64] {
        match self {
            Receipt::V1(r) => &r.provenance,
            Receipt::V2(r) => &r.provenance,
        }
    }
    /// S — the coarse galactic cell, or `None` for a `V1` (pre-TU-1)
    /// receipt.
    pub fn region_seed(&self) -> Option<[u8; 32]> {
        match self {
            Receipt::V1(_) => None,
            Receipt::V2(r) => Some(r.region_seed),
        }
    }
    /// G — the beat's cached gauge-direction sample, or `None` for a
    /// `V1` receipt.
    pub fn gauge(&self) -> Option<&[u8]> {
        match self {
            Receipt::V1(_) => None,
            Receipt::V2(r) => Some(&r.gauge),
        }
    }

    /// The receipt's on-disk frame (NS-3 §2.4). `V1` is tag 20, byte-
    /// identical to every receipt ever written: `LE64(sequence) ‖
    /// LE64(stamp) ‖ parent[32] ‖ own[32] ‖ LE32(provenance count) ‖
    /// LE64(k)… ‖ event TLV record`. `V2` is tag 21 (TU-1, new,
    /// permanent — additive at the FRAME level even though the receipt
    /// itself is a direct restructure, so a `V1`-only reader skips `V2`
    /// frames under SEAL A rather than refusing the whole file): the
    /// same layout with `region_seed[32] ‖ LE32(gauge count) ‖ gauge
    /// bytes…` inserted after the provenance list, before the event.
    pub fn wire_bytes(&self) -> Vec<u8> {
        match self {
            Receipt::V1(r) => {
                let mut value = r.sequence.to_le_bytes().to_vec();
                value.extend_from_slice(&r.stamp.to_le_bytes());
                value.extend_from_slice(&r.parent.0);
                value.extend_from_slice(&r.own.0);
                value.extend_from_slice(&(r.provenance.len() as u32).to_le_bytes());
                for k in &r.provenance {
                    value.extend_from_slice(&k.to_le_bytes());
                }
                value.extend_from_slice(&r.event.wire_bytes());
                let mut out = Vec::new();
                crate::tlv::put(RECEIPT_FRAME_TAG, &value, &mut out);
                out
            }
            Receipt::V2(r) => {
                let mut value = r.sequence.to_le_bytes().to_vec();
                value.extend_from_slice(&r.stamp.to_le_bytes());
                value.extend_from_slice(&r.parent.0);
                value.extend_from_slice(&r.own.0);
                value.extend_from_slice(&(r.provenance.len() as u32).to_le_bytes());
                for k in &r.provenance {
                    value.extend_from_slice(&k.to_le_bytes());
                }
                value.extend_from_slice(&r.region_seed);
                value.extend_from_slice(&(r.gauge.len() as u32).to_le_bytes());
                value.extend_from_slice(&r.gauge);
                value.extend_from_slice(&r.event.wire_bytes());
                let mut out = Vec::new();
                crate::tlv::put(RECEIPT_FRAME_TAG_V2, &value, &mut out);
                out
            }
        }
    }

    /// Decode one framed receipt; `None` on an unknown tag or a
    /// malformed frame (the caller skips unknown tags — SEAL A).
    pub fn from_wire(tag: u8, value: &[u8]) -> Option<Self> {
        match tag {
            RECEIPT_FRAME_TAG => {
                let sequence = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let stamp = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let parent = ReceiptHash(value.get(16..48)?.try_into().ok()?);
                let own = ReceiptHash(value.get(48..80)?.try_into().ok()?);
                let count = u32::from_le_bytes(value.get(80..84)?.try_into().ok()?) as usize;
                let mut at = 84usize;
                let mut provenance = Vec::with_capacity(count);
                for _ in 0..count {
                    provenance.push(u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?));
                    at += 8;
                }
                let (event_tag, event_value) = crate::tlv::records(value.get(at..)?).next()?;
                let event = ClosureEvent::from_wire(event_tag, event_value)?;
                Some(Receipt::V1(LegacyReceipt {
                    sequence,
                    stamp,
                    parent,
                    own,
                    event,
                    provenance,
                }))
            }
            RECEIPT_FRAME_TAG_V2 => {
                let sequence = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let stamp = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let parent = ReceiptHash(value.get(16..48)?.try_into().ok()?);
                let own = ReceiptHash(value.get(48..80)?.try_into().ok()?);
                let count = u32::from_le_bytes(value.get(80..84)?.try_into().ok()?) as usize;
                let mut at = 84usize;
                let mut provenance = Vec::with_capacity(count);
                for _ in 0..count {
                    provenance.push(u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?));
                    at += 8;
                }
                let region_seed: [u8; 32] = value.get(at..at + 32)?.try_into().ok()?;
                at += 32;
                let gauge_len = u32::from_le_bytes(value.get(at..at + 4)?.try_into().ok()?) as usize;
                at += 4;
                let gauge = value.get(at..at + gauge_len)?.to_vec();
                at += gauge_len;
                let (event_tag, event_value) = crate::tlv::records(value.get(at..)?).next()?;
                let event = ClosureEvent::from_wire(event_tag, event_value)?;
                Some(Receipt::V2(ManifoldReceipt {
                    sequence,
                    stamp,
                    parent,
                    own,
                    event,
                    provenance,
                    region_seed,
                    gauge,
                }))
            }
            _ => None,
        }
    }
}

/// The chain's typed verification read (NS-3 §1.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainRead {
    Intact,
    Broken { at: u64 },
}

/// The receipt FRAME tag (NS-3 §2.4, P2): the on-disk framing of one
/// whole `V1` receipt. Permanent, like every tag — untouched by TU-1,
/// so every historical chronicle decodes byte-identically.
pub const RECEIPT_FRAME_TAG: u8 = 20;
/// The `V2` ({S,G,T}) receipt frame tag (TU-1, new 2026-08-22).
/// Permanent from here forward.
pub const RECEIPT_FRAME_TAG_V2: u8 = 21;

/// MESH-1b (NS-5 §7.5): the conductor's hand. Every other seat
/// derives its hand from its name via `hand_of`; 0 stays unclaimed.
pub const CONDUCTOR_HAND: u64 = 1;

/// A seat's hand — the first 8 bytes of its name's digest, LE. The
/// reserved values (0, the conductor's 1) remap to 2; a natural
/// collision there is astronomically unlikely and lawful if it lands
/// (two seats sharing a hand share a pen, nothing forks).
pub fn hand_of(name: &str) -> u64 {
    let digest = blake3::hash(name.as_bytes());
    let hand = u64::from_le_bytes(digest.as_bytes()[..8].try_into().expect("8 bytes"));
    if hand <= CONDUCTOR_HAND { 2 } else { hand }
}

/// The chronicle: append-only — no removal, no mutation, no reorder.
#[derive(Debug, Clone, Default)]
pub struct Chronicle {
    receipts: Vec<Receipt>,
    /// MESH-1b consumer funnel: the held pen. Workspace only — never
    /// serialized; the receipts' provenance is the record.
    hand: Option<u64>,
    /// TU-1 / S: the world's coarse galactic cell, set once (mirrors
    /// `World::region_seed`, GALACTIC-1's `chart_region`). `None` until
    /// charted — every `append` before that point still writes `V1`
    /// (a world with no region is a world TU-1 does not yet describe,
    /// not an error).
    region_seed: Option<[u8; 32]>,
    /// TU-1 / G: this beat's cached gauge-direction sample (raw index
    /// bytes — `cfe` does not depend on `reduct`). Set once per beat by
    /// the caller (mirrors `set_hand`); every `append` within that beat
    /// reads the same snapshot, never recomputing mid-beat (TU-1c).
    gauge_cache: Vec<u8>,
}

impl Chronicle {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reconstitute a chronicle from stored receipts — ONLY through the
    /// verification read: a chain that does not re-verify from its own
    /// content refuses with its first break named. Append-only holds
    /// across the process boundary because this is the one door back in.
    pub fn from_receipts(receipts: Vec<Receipt>) -> Result<Self, ChainRead> {
        match verify_receipts(&receipts) {
            ChainRead::Intact => Ok(Self { receipts, hand: None, region_seed: None, gauge_cache: Vec::new() }),
            broken => Err(broken),
        }
    }

    pub fn receipts(&self) -> &[Receipt] {
        &self.receipts
    }

    /// MESH-1b: hold (or release) the pen. While held, every append
    /// carries the hand — see `append`. The walk in `mesh` stays the
    /// gate; this is the stamp, not the gate.
    pub fn set_hand(&mut self, hand: Option<u64>) {
        self.hand = hand;
    }

    pub fn hand(&self) -> Option<u64> {
        self.hand
    }

    /// TU-1 / S: seal the world's coarse galactic cell — once (mirrors
    /// `chart_region`'s sealed-once law at the `World` layer). Every
    /// `append` from this point forward writes `V2`.
    pub fn set_region_seed(&mut self, seed: [u8; 32]) {
        self.region_seed = Some(seed);
    }

    pub fn region_seed(&self) -> Option<[u8; 32]> {
        self.region_seed
    }

    /// TU-1 / G: cache this beat's gauge-direction sample. Called once
    /// per beat (TU-1c) — every `append` within the beat reads this same
    /// snapshot.
    pub fn set_gauge_sample(&mut self, sample: Vec<u8>) {
        self.gauge_cache = sample;
    }

    /// The one write. Parent link and own digest are computed here.
    /// A held pen (MESH-1b) stamps the hand at the provenance head
    /// and cites a seal standing last (NS-5 §7.1); pre-stamped
    /// provenance (the mesh doors) and unheld chronicles pass
    /// through untouched. TU-1: once a region is charted, every append
    /// writes `V2` (S = the sealed cell, G = the beat's cached sample);
    /// before that, `V1` — byte-identical to every receipt this engine
    /// has ever written.
    pub fn append(&mut self, event: ClosureEvent, stamp: u64, provenance: Vec<u64>) -> &Receipt {
        let provenance = match self.hand {
            Some(hand) if provenance.first() != Some(&hand) => {
                let mut carried = vec![hand];
                if let Some(last) = self.receipts.last() {
                    if matches!(last.event(), ClosureEvent::HandoffSealed { .. }) {
                        carried.push(last.sequence());
                    }
                }
                carried.extend(provenance);
                carried
            }
            _ => provenance,
        };
        let sequence = self.receipts.len() as u64;
        let parent = self
            .receipts
            .last()
            .map(|receipt| receipt.own())
            .unwrap_or(ReceiptHash::GENESIS);
        let event_bytes = event.wire_bytes();
        let receipt = match self.region_seed {
            Some(region_seed) => {
                let gauge = self.gauge_cache.clone();
                let own = receipt_digest_v2(sequence, stamp, &parent, &event_bytes, &provenance, &region_seed, &gauge);
                Receipt::V2(ManifoldReceipt {
                    sequence,
                    stamp,
                    parent,
                    own,
                    event,
                    provenance,
                    region_seed,
                    gauge,
                })
            }
            None => {
                let own = receipt_digest(sequence, stamp, &parent, &event_bytes, &provenance);
                Receipt::V1(LegacyReceipt {
                    sequence,
                    stamp,
                    parent,
                    own,
                    event,
                    provenance,
                })
            }
        };
        self.receipts.push(receipt);
        self.receipts.last().expect("just appended")
    }

    pub fn verify_chain(&self) -> ChainRead {
        verify_receipts(&self.receipts)
    }
}

/// Re-verify a receipt slice from its own content; the first broken
/// link is named. Public so a replayer or a peer can verify receipts it
/// received over the wire, outside any chronicle.
pub fn verify_receipts(receipts: &[Receipt]) -> ChainRead {
    let mut parent = ReceiptHash::GENESIS;
    for receipt in receipts {
        let expected = match receipt {
            Receipt::V1(r) => receipt_digest(r.sequence, r.stamp, &parent, &r.event.wire_bytes(), &r.provenance),
            Receipt::V2(r) => receipt_digest_v2(
                r.sequence,
                r.stamp,
                &parent,
                &r.event.wire_bytes(),
                &r.provenance,
                &r.region_seed,
                &r.gauge,
            ),
        };
        match receipt.parent() == parent && receipt.own() == expected {
            true => parent = receipt.own(),
            false => {
                return ChainRead::Broken {
                    at: receipt.sequence(),
                }
            }
        }
    }
    ChainRead::Intact
}

/// The normative digest layout (NS-3 §2.1), `V1` — UNCHANGED by TU-1:
/// `BLAKE3( LE64(sequence) ‖ LE64(stamp) ‖ parent[32] ‖ event_bytes ‖ LE64(k)… )`.
/// Public over raw event bytes so the NS-3 §3 vectors — whose event
/// bytes come from the v0.4 ancestor — verify against this exact
/// function.
pub fn receipt_digest(
    sequence: u64,
    stamp: u64,
    parent: &ReceiptHash,
    event_bytes: &[u8],
    provenance: &[u64],
) -> ReceiptHash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&sequence.to_le_bytes());
    hasher.update(&stamp.to_le_bytes());
    hasher.update(&parent.0);
    hasher.update(event_bytes);
    for k in provenance {
        hasher.update(&k.to_le_bytes());
    }
    ReceiptHash(*hasher.finalize().as_bytes())
}

/// The `V2` digest layout (TU-1): `receipt_digest`'s exact formula, with
/// `region_seed` and `gauge` folded in AFTER provenance and BEFORE
/// nothing else changes — S and G are attested facts, so a tampered
/// region or gauge sample breaks the chain exactly like a tampered event
/// would (the whole point of stamping them: an untrusted claim about a
/// civilization's position or orientation must be checkable, not just
/// carried).
pub fn receipt_digest_v2(
    sequence: u64,
    stamp: u64,
    parent: &ReceiptHash,
    event_bytes: &[u8],
    provenance: &[u64],
    region_seed: &[u8; 32],
    gauge: &[u8],
) -> ReceiptHash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&sequence.to_le_bytes());
    hasher.update(&stamp.to_le_bytes());
    hasher.update(&parent.0);
    hasher.update(event_bytes);
    for k in provenance {
        hasher.update(&k.to_le_bytes());
    }
    hasher.update(region_seed);
    hasher.update(gauge);
    ReceiptHash(*hasher.finalize().as_bytes())
}
