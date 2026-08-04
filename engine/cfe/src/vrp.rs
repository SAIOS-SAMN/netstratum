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

/// One receipt (NS-3 §1.1). Fields are readable; the chronicle is the
/// only constructor path that yields a chained receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub sequence: u64,
    pub stamp: u64,
    pub parent: ReceiptHash,
    pub own: ReceiptHash,
    pub event: ClosureEvent,
    pub provenance: Vec<u64>,
}

/// The chain's typed verification read (NS-3 §1.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainRead {
    Intact,
    Broken { at: u64 },
}

/// The receipt FRAME tag (NS-3 §2.4, P2): the on-disk framing of one
/// whole receipt. Permanent, like every tag.
pub const RECEIPT_FRAME_TAG: u8 = 20;

impl Receipt {
    /// The receipt's on-disk frame (NS-3 §2.4): one TLV record, tag 20,
    /// value = `LE64(sequence) ‖ LE64(stamp) ‖ parent[32] ‖ own[32] ‖
    /// LE32(provenance count) ‖ LE64(k)… ‖ event TLV record`.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut value = self.sequence.to_le_bytes().to_vec();
        value.extend_from_slice(&self.stamp.to_le_bytes());
        value.extend_from_slice(&self.parent.0);
        value.extend_from_slice(&self.own.0);
        value.extend_from_slice(&(self.provenance.len() as u32).to_le_bytes());
        for k in &self.provenance {
            value.extend_from_slice(&k.to_le_bytes());
        }
        value.extend_from_slice(&self.event.wire_bytes());
        let mut out = Vec::new();
        crate::tlv::put(RECEIPT_FRAME_TAG, &value, &mut out);
        out
    }

    /// Decode one framed receipt; `None` on an unknown tag or a
    /// malformed frame (the caller skips unknown tags — SEAL A).
    pub fn from_wire(tag: u8, value: &[u8]) -> Option<Self> {
        if tag != RECEIPT_FRAME_TAG {
            return None;
        }
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
        Some(Receipt {
            sequence,
            stamp,
            parent,
            own,
            event,
            provenance,
        })
    }
}

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
            ChainRead::Intact => Ok(Self { receipts, hand: None }),
            broken => Err(broken),
        }
    }

    pub fn receipts(&self) -> &[Receipt] {
        &self.receipts
    }

    /// MESH-1b: hold (or release) the pen. While held, every append
    /// carries the hand — see `append`. The walk in `mesh` stays the
    /// judge; this is the stamp, not the gate.
    pub fn set_hand(&mut self, hand: Option<u64>) {
        self.hand = hand;
    }

    pub fn hand(&self) -> Option<u64> {
        self.hand
    }

    /// The one write. Parent link and own digest are computed here.
    /// A held pen (MESH-1b) stamps the hand at the provenance head
    /// and cites a seal standing last (NS-5 §7.1); pre-stamped
    /// provenance (the mesh doors) and unheld chronicles pass
    /// through untouched.
    pub fn append(&mut self, event: ClosureEvent, stamp: u64, provenance: Vec<u64>) -> &Receipt {
        let provenance = match self.hand {
            Some(hand) if provenance.first() != Some(&hand) => {
                let mut carried = vec![hand];
                if let Some(last) = self.receipts.last() {
                    if matches!(last.event, ClosureEvent::HandoffSealed { .. }) {
                        carried.push(last.sequence);
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
            .map(|receipt| receipt.own)
            .unwrap_or(ReceiptHash::GENESIS);
        let own = receipt_digest(sequence, stamp, &parent, &event.wire_bytes(), &provenance);
        self.receipts.push(Receipt {
            sequence,
            stamp,
            parent,
            own,
            event,
            provenance,
        });
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
        let expected = receipt_digest(
            receipt.sequence,
            receipt.stamp,
            &parent,
            &receipt.event.wire_bytes(),
            &receipt.provenance,
        );
        match receipt.parent == parent && receipt.own == expected {
            true => parent = receipt.own,
            false => {
                return ChainRead::Broken {
                    at: receipt.sequence,
                }
            }
        }
    }
    ChainRead::Intact
}

/// The normative digest layout (NS-3 §2.1):
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
