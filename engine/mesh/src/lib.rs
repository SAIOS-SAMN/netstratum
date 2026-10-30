//! The mesh, after Charter §11 — the lawful remainder.
//!
//! The reconciliation instruments (consilience, the manifold, the
//! census, the triangle — tag 65 and refusal kinds 3–4) were excised
//! under the one-truth amendment: what cannot diverge is not
//! reconciled, and agreement between copies was never a measurement.
//! What stands is what was always in-chain and single-copy:
//!
//! - `receive` — One chain's integrity door (frames parse, digests
//!   recompute, the first break named). No second copy is consulted.
//! - `head` — the 80-byte announcement of where a chain stands.
//! - the authority walk and its doors (NS-5 §7) — the receipted
//!   handoff, one writer per clock, deeds on the chain.
//!
//! The arbor at TUniversal{S,G,T} replaces replication whole (the
//! constitution ladder): state is position, the kernel is motion, the
//! chain is the path. This crate becomes the arbor's carrier when the
//! codec gate lands; nothing here reads or writes a second truth.
//!
//! The sandboxed nursery (NS-5 §7.6, operator ruling 2026-07-31):
//! open divergence is lawful; covert divergence is slashed. A
//! declared fork is a nursery under its own branch coordinates — the
//! mainline cannot be written from inside it (one value per address,
//! always), followers never decohere (a branch is addressable, never
//! a second claim on one address), and re-convergence has one door:
//! the kernel gates the merge, or the nursery keeps its toys. The
//! lie is penalized, never the disagreement.
//!
//! The uplink law (operator order 2026-07-31 — mesh authenticity):
//! nothing file-borne ever rides the mesh. Only chain-verified bytes
//! ascend — `chronicle.tlv` through the `receive` door today, arbor
//! commitments at the ladder's #36. Ledger files are pre-chain debts
//! awaiting adoption; lens exports are for eyes; the mesh carries
//! neither, and a peer that offers them is offering nothing.

pub mod arbor;

use cfe::dop::ClosureEvent;
use cfe::tlv;
use cfe::vrp::Chronicle;

/// Tag 64 — the mesh head announcement (NS-5 §3.1).
pub const MESH_HEAD_TAG: u8 = 64;
/// Tag 65 is RETIRED (Charter §11) — the consilience receipt; retired
/// codes are never reused.
/// Tag 66 — the mesh refusal (kinds 3–4 RETIRED under §11; kinds
/// 1, 2, 5, 6 stand — single-chain refusals all).
pub const MESH_REFUSAL_TAG: u8 = 66;

/// Which peer of the pair (wire codes: A = 1, B = 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    A,
    B,
}

impl Side {
    fn code(self) -> u8 {
        match self {
            Self::A => 1,
            Self::B => 2,
        }
    }

    fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::A),
            2 => Some(Self::B),
            _ => None,
        }
    }
}

/// The head announcement (NS-5 §3.1): the chain's identity (receipt
/// 0's digest), its length, and where it stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshHead {
    pub genesis: [u8; 32],
    pub length: u64,
    pub stamp: u64,
    pub head: [u8; 32],
}

/// The typed mesh refusal — every kind gates one chain against its
/// own content or its own law; no copy is reconciled with another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeshRefused {
    /// A receipt frame that does not parse.
    Malformed { side: Side, frame: u64 },
    /// A chain that does not re-verify from its own content.
    Forged { side: Side, at: u64 },
    /// NS-5 §7.2 — the receipt at `at` does not carry the lawful
    /// hand: one writer per clock, broken.
    Usurped { at: u64, lawful: u64 },
    /// NS-5 §7.2 — the taking hand's first append does not cite the
    /// seal it took the pen from.
    Uncited { at: u64, handoff: u64 },
}

impl MeshHead {
    /// One TLV record, tag 64: `genesis[32] ‖ LE64(length) ‖
    /// LE64(stamp) ‖ head[32]`.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut value = self.genesis.to_vec();
        value.extend_from_slice(&self.length.to_le_bytes());
        value.extend_from_slice(&self.stamp.to_le_bytes());
        value.extend_from_slice(&self.head);
        let mut out = Vec::new();
        tlv::put(MESH_HEAD_TAG, &value, &mut out);
        out
    }

    pub fn from_wire(tag: u8, value: &[u8]) -> Option<Self> {
        if tag != MESH_HEAD_TAG {
            return None;
        }
        let genesis: [u8; 32] = value.get(..32)?.try_into().ok()?;
        let length = u64::from_le_bytes(value.get(32..40)?.try_into().ok()?);
        let stamp = u64::from_le_bytes(value.get(40..48)?.try_into().ok()?);
        let head: [u8; 32] = value.get(48..80)?.try_into().ok()?;
        Some(Self { genesis, length, stamp, head })
    }
}

impl MeshRefused {
    /// One TLV record, tag 66: `kind u8` then the kind's own layout.
    /// Kinds 3–4 are RETIRED (§11): this encoder never emits them and
    /// this decoder refuses them like any unknown kind — a reader
    /// meeting a retired record skips it whole (SEAL A).
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut value = Vec::new();
        match self {
            Self::Malformed { side, frame } => {
                value.push(1);
                value.push(side.code());
                value.extend_from_slice(&frame.to_le_bytes());
            }
            Self::Forged { side, at } => {
                value.push(2);
                value.push(side.code());
                value.extend_from_slice(&at.to_le_bytes());
            }
            Self::Usurped { at, lawful } => {
                value.push(5);
                value.extend_from_slice(&at.to_le_bytes());
                value.extend_from_slice(&lawful.to_le_bytes());
            }
            Self::Uncited { at, handoff } => {
                value.push(6);
                value.extend_from_slice(&at.to_le_bytes());
                value.extend_from_slice(&handoff.to_le_bytes());
            }
        }
        let mut out = Vec::new();
        tlv::put(MESH_REFUSAL_TAG, &value, &mut out);
        out
    }

    pub fn from_wire(tag: u8, value: &[u8]) -> Option<Self> {
        if tag != MESH_REFUSAL_TAG {
            return None;
        }
        match *value.first()? {
            1 => {
                let side = Side::from_code(*value.get(1)?)?;
                let frame = u64::from_le_bytes(value.get(2..10)?.try_into().ok()?);
                Some(Self::Malformed { side, frame })
            }
            2 => {
                let side = Side::from_code(*value.get(1)?)?;
                let at = u64::from_le_bytes(value.get(2..10)?.try_into().ok()?);
                Some(Self::Forged { side, at })
            }
            5 => {
                let at = u64::from_le_bytes(value.get(1..9)?.try_into().ok()?);
                let lawful = u64::from_le_bytes(value.get(9..17)?.try_into().ok()?);
                Some(Self::Usurped { at, lawful })
            }
            6 => {
                let at = u64::from_le_bytes(value.get(1..9)?.try_into().ok()?);
                let handoff = u64::from_le_bytes(value.get(9..17)?.try_into().ok()?);
                Some(Self::Uncited { at, handoff })
            }
            _ => None,
        }
    }
}

/// The announcement read: where a chain stands, in 80 bytes.
pub fn head(chronicle: &Chronicle) -> MeshHead {
    let receipts = chronicle.receipts();
    MeshHead {
        genesis: chronicle
            .annals()
            .first_digest()
            .or_else(|| receipts.first().map(|receipt| receipt.own().0))
            .unwrap_or([0u8; 32]),
        length: chronicle.total(),
        stamp: receipts.last().map(|receipt| receipt.stamp()).unwrap_or(0),
        head: receipts.last().map(|receipt| receipt.own().0).unwrap_or([0u8; 32]),
    }
}

/// The walk's resting state: the hand that holds the pen (None in the
/// pre-mesh era) and the seal awaiting its citation, if any.
struct WalkState {
    hand: Option<u64>,
    pending_cite: Option<u64>,
}

/// The authority walk (NS-5 §7.1), the one computation of it: gate
/// each receipt under the authority before it (carry the hand, cite a
/// pending seal — authorship before citation), then apply its effect
/// (a seal moves the pen and opens a citation). Before the first seal
/// every append is lawful; a seal standing last is lawful with its
/// citation outstanding.
fn walk(chronicle: &Chronicle) -> Result<WalkState, MeshRefused> {
    // The hand carried forward: the drained prefix was walked live when it
    // was written, so the walk starts from what it left — the last seal the
    // annals keep names the hand, and a seal that closed the drained prefix
    // still owes its citation to the first held receipt.
    let mut state = WalkState { hand: None, pending_cite: None };
    let base = chronicle.base();
    if let Some((to, sequence)) = chronicle.annals().last_seal() {
        state.hand = Some(to);
        if sequence + 1 == base {
            state.pending_cite = Some(sequence);
        }
    }
    for receipt in chronicle.receipts() {
        if let Some(lawful) = state.hand {
            if !receipt.provenance().contains(&lawful) {
                return Err(MeshRefused::Usurped { at: receipt.sequence(), lawful });
            }
            if let Some(handoff) = state.pending_cite {
                if !receipt.provenance().contains(&handoff) {
                    return Err(MeshRefused::Uncited { at: receipt.sequence(), handoff });
                }
                state.pending_cite = None;
            }
        }
        if let ClosureEvent::HandoffSealed { from, to } = receipt.event() {
            if let Some(lawful) = state.hand {
                if *from != lawful {
                    return Err(MeshRefused::Usurped { at: receipt.sequence(), lawful });
                }
            }
            // The first seal self-declares its incumbent (nothing
            // earlier could attest it — the provenance law).
            state.hand = Some(*to);
            state.pending_cite = Some(receipt.sequence());
        }
    }
    Ok(state)
}

/// The authority reading (NS-5 §7.3): `None` pre-mesh, the hand after
/// seals, or the walk's first violation refused typed. A pure read —
/// never a flag, never consulted by any game rule.
pub fn authority(chronicle: &Chronicle) -> Result<Option<u64>, MeshRefused> {
    walk(chronicle).map(|state| state.hand)
}

/// The seal door (NS-5 §7.3): the yielding hand names the successor.
/// A stale hand refuses before anything appends; the seal carries its
/// sealer and, when the seal is itself the taking hand's first
/// append, the citation it owes.
pub fn yield_authority(
    chronicle: &mut Chronicle,
    from: u64,
    to: u64,
    stamp: u64,
) -> Result<u64, MeshRefused> {
    let state = walk(chronicle)?;
    if let Some(lawful) = state.hand {
        if from != lawful {
            return Err(MeshRefused::Usurped {
                at: chronicle.total(),
                lawful,
            });
        }
    }
    let mut provenance = vec![from];
    if let Some(handoff) = state.pending_cite {
        provenance.push(handoff);
    }
    let receipt = chronicle.append(ClosureEvent::HandoffSealed { from, to }, stamp, provenance);
    Ok(receipt.sequence())
}
