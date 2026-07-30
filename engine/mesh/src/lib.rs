//! NS-5 MESH — proof-of-convergence consilience (MESH-1a).
//!
//! Two independent engine instances converge on one world state by
//! exact measurement: each replays the shared receipt spine for
//! itself, projects the committed state onto the v1 manifold (NS-5
//! §1.1), and the consilience triangle closes at C(Δ) = 0 exactly —
//! or refuses typed with the disagreement carried whole (§1.4).
//! Unanimity is the geometry, never a vote.
//!
//! Consilience is a READ: nothing here appends to any world
//! chronicle. The mesh records (tags 64–66) live in the mesh's own
//! framing, beside the world, never inside it — the lens law in mesh
//! form.
//!
//! The rulings of 2026-07-30 (docs/N6_DIRECTIVES.md): identity is
//! blake3 and provenance only at N6 — independent recomputation IS
//! the claim; transport is bytes between directories — the engine
//! never opens a socket (SEAL C: the adapter is a file copy).

use cfe::dop::Resource;
use cfe::ratio::{self, ratio, Ratio};
use cfe::tlv;
use cfe::vrp::{ChainRead, Chronicle, Receipt, RECEIPT_FRAME_TAG};
use cfe::world::{replay, Committed};
use rfa_core::{coherence_functional, Delta};

/// Tag 64 — the mesh head announcement (NS-5 §2.1).
pub const MESH_HEAD_TAG: u8 = 64;
/// Tag 65 — the consilience receipt (NS-5 §2.2).
pub const CONSILIENCE_RECEIPT_TAG: u8 = 65;
/// Tag 66 — the mesh refusal (NS-5 §2.3).
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

/// The head announcement (NS-5 §2.1): the chain's identity (receipt
/// 0's digest), its length, and where it stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshHead {
    pub genesis: [u8; 32],
    pub length: u64,
    pub stamp: u64,
    pub head: [u8; 32],
}

/// The consilience receipt (NS-5 §2.2): the measured agreement —
/// exactly zero by issuance law (§1.3.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsilienceReceipt {
    pub length: u64,
    pub stamp: u64,
    pub head: [u8; 32],
    pub arity: u8,
    pub dimensions: u32,
    pub coherence: Ratio,
}

/// The typed mesh refusal (NS-5 §1.4): disagreement preserved whole,
/// never voted away.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeshRefused {
    /// A receipt frame that does not parse.
    Malformed { side: Side, frame: u64 },
    /// A chain that does not re-verify from its own content.
    Forged { side: Side, at: u64 },
    /// Same history, less of it — the missing range named, half-open.
    Drift { behind: Side, from: u64, to: u64 },
    /// Forked history: the first differing sequence, the residual
    /// list whole (dimension, B − A), and C(Δ) the summary scalar.
    Diverged { at: u64, residuals: Vec<(u32, Ratio)>, coherence: Ratio },
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

impl ConsilienceReceipt {
    /// One TLV record, tag 65: `LE64(length) ‖ LE64(stamp) ‖ head[32]
    /// ‖ arity u8 ‖ LE32(dimensions) ‖ ratio(coherence)`.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut value = self.length.to_le_bytes().to_vec();
        value.extend_from_slice(&self.stamp.to_le_bytes());
        value.extend_from_slice(&self.head);
        value.push(self.arity);
        value.extend_from_slice(&self.dimensions.to_le_bytes());
        ratio::encode(&self.coherence, &mut value);
        let mut out = Vec::new();
        tlv::put(CONSILIENCE_RECEIPT_TAG, &value, &mut out);
        out
    }

    pub fn from_wire(tag: u8, value: &[u8]) -> Option<Self> {
        if tag != CONSILIENCE_RECEIPT_TAG {
            return None;
        }
        let length = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
        let stamp = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
        let head: [u8; 32] = value.get(16..48)?.try_into().ok()?;
        let arity = *value.get(48)?;
        let dimensions = u32::from_le_bytes(value.get(49..53)?.try_into().ok()?);
        let (coherence, _) = ratio::decode(value.get(53..)?)?;
        Some(Self { length, stamp, head, arity, dimensions, coherence })
    }
}

impl MeshRefused {
    /// One TLV record, tag 66: `kind u8` then the kind's own layout
    /// (NS-5 §2.3). The residual list is the payload, whole.
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
            Self::Drift { behind, from, to } => {
                value.push(3);
                value.push(behind.code());
                value.extend_from_slice(&from.to_le_bytes());
                value.extend_from_slice(&to.to_le_bytes());
            }
            Self::Diverged { at, residuals, coherence } => {
                value.push(4);
                value.extend_from_slice(&at.to_le_bytes());
                value.extend_from_slice(&(residuals.len() as u32).to_le_bytes());
                for (dimension, residual) in residuals {
                    value.extend_from_slice(&dimension.to_le_bytes());
                    ratio::encode(residual, &mut value);
                }
                ratio::encode(coherence, &mut value);
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
            3 => {
                let behind = Side::from_code(*value.get(1)?)?;
                let from = u64::from_le_bytes(value.get(2..10)?.try_into().ok()?);
                let to = u64::from_le_bytes(value.get(10..18)?.try_into().ok()?);
                Some(Self::Drift { behind, from, to })
            }
            4 => {
                let at = u64::from_le_bytes(value.get(1..9)?.try_into().ok()?);
                let count = u32::from_le_bytes(value.get(9..13)?.try_into().ok()?) as usize;
                let mut cursor = 13usize;
                let mut residuals = Vec::with_capacity(count);
                for _ in 0..count {
                    let dimension =
                        u32::from_le_bytes(value.get(cursor..cursor + 4)?.try_into().ok()?);
                    let (residual, used) = ratio::decode(value.get(cursor + 4..)?)?;
                    residuals.push((dimension, residual));
                    cursor += 4 + used;
                }
                let (coherence, _) = ratio::decode(value.get(cursor..)?)?;
                Some(Self::Diverged { at, residuals, coherence })
            }
            _ => None,
        }
    }
}

/// The announcement read: where a chain stands, in 80 bytes.
pub fn head(chronicle: &Chronicle) -> MeshHead {
    let receipts = chronicle.receipts();
    MeshHead {
        genesis: receipts.first().map(|receipt| receipt.own.0).unwrap_or([0u8; 32]),
        length: receipts.len() as u64,
        stamp: receipts.last().map(|receipt| receipt.stamp).unwrap_or(0),
        head: receipts.last().map(|receipt| receipt.own.0).unwrap_or([0u8; 32]),
    }
}

/// The ingestion door: parse the framed receipts out of a peer's
/// bytes (unknown tags skip — SEAL A), then re-verify the chain from
/// its own content. A frame that does not parse refuses MALFORMED; a
/// chain that does not recompute refuses FORGED with its break named.
/// A `Chronicle` in hand is intact by construction — forgery never
/// gets past this door.
pub fn receive(bytes: &[u8], side: Side) -> Result<Chronicle, MeshRefused> {
    let mut receipts = Vec::new();
    for (frame, (tag, value)) in tlv::records(bytes).enumerate() {
        if tag == RECEIPT_FRAME_TAG {
            let receipt = Receipt::from_wire(tag, value)
                .ok_or(MeshRefused::Malformed { side, frame: frame as u64 })?;
            receipts.push(receipt);
        }
    }
    Chronicle::from_receipts(receipts).map_err(|read| match read {
        ChainRead::Broken { at } => MeshRefused::Forged { side, at },
        ChainRead::Intact => unreachable!("from_receipts errs only on breaks"),
    })
}

/// The consilience judgment (NS-5 §1.3): walk the spine to the first
/// fork (DIVERGED, residuals whole); no fork but unequal lengths is
/// DRIFT (the missing range named); one spine and both sides replay
/// INDEPENDENTLY — the triangle closes at C(Δ) = 0 exactly, or the
/// measurement itself refuses. Nothing is assumed, nothing is voted.
pub fn consile(a: &Chronicle, b: &Chronicle) -> Result<ConsilienceReceipt, MeshRefused> {
    let ours = a.receipts();
    let theirs = b.receipts();
    let common = ours.len().min(theirs.len());
    for at in 0..common {
        if ours[at].own != theirs[at].own {
            return Err(diverged(a, b, at as u64));
        }
    }
    if ours.len() < theirs.len() {
        return Err(MeshRefused::Drift {
            behind: Side::A,
            from: ours.len() as u64,
            to: theirs.len() as u64,
        });
    }
    if theirs.len() < ours.len() {
        return Err(MeshRefused::Drift {
            behind: Side::B,
            from: theirs.len() as u64,
            to: ours.len() as u64,
        });
    }
    let (position_a, position_b) = positions(a, b);
    let dimensions = position_a.len() as u32;
    let coherence = coherence_functional(&triangle(&position_a, &position_b));
    if coherence != ratio(0, 1) {
        // One spine, two positions: SEAL B is broken somewhere. The
        // measurement judges — this is divergence at the head, not a
        // receipt with a caveat.
        return Err(diverged(a, b, ours.len() as u64));
    }
    Ok(ConsilienceReceipt {
        length: ours.len() as u64,
        stamp: ours.last().map(|receipt| receipt.stamp).unwrap_or(0),
        head: ours.last().map(|receipt| receipt.own.0).unwrap_or([0u8; 32]),
        arity: 2,
        dimensions,
        coherence,
    })
}

/// The v1 manifold (NS-5 §1.1): both committed states projected onto
/// the union span — gauge grades first (grade-ascending), then per
/// orb ascending by orb id the banked ledger over resource codes 1–7.
/// An absent coordinate reads exactly 0. Total, deterministic,
/// canonical.
fn positions(a: &Chronicle, b: &Chronicle) -> (Vec<Ratio>, Vec<Ratio>) {
    let ours = replay(a);
    let theirs = replay(b);
    let grades = ours.gauge.len().max(theirs.gauge.len());
    let mut orb_ids: Vec<u64> = ours
        .orbs
        .iter()
        .chain(theirs.orbs.iter())
        .map(|orb| orb.orb_id)
        .collect();
    orb_ids.sort_unstable();
    orb_ids.dedup();
    let mut position_a = Vec::new();
    let mut position_b = Vec::new();
    for grade in 0..grades {
        position_a.push(gauge_at(&ours, grade));
        position_b.push(gauge_at(&theirs, grade));
    }
    for orb_id in orb_ids {
        for code in 1u8..=7 {
            let resource = Resource::from_wire(code).expect("codes 1-7 are the roster");
            position_a.push(banked_of(&ours, orb_id, resource));
            position_b.push(banked_of(&theirs, orb_id, resource));
        }
    }
    (position_a, position_b)
}

fn gauge_at(committed: &Committed, grade: usize) -> Ratio {
    committed.gauge.get(grade).cloned().unwrap_or_else(|| ratio(0, 1))
}

fn banked_of(committed: &Committed, orb_id: u64, resource: Resource) -> Ratio {
    committed
        .orbs
        .iter()
        .find(|orb| orb.orb_id == orb_id)
        .map(|orb| orb.banked_amount(resource))
        .unwrap_or_else(|| ratio(0, 1))
}

/// The consilience triangle (NS-5 §1.2): Δ(origin→A) = a's position,
/// Δ(origin→B) = b's position, Δ(A→B) = 0 — the shared-position
/// claim. C(Δ) = 6·Σ R² over the six ordered triples; zero iff every
/// residual is zero.
fn triangle(position_a: &[Ratio], position_b: &[Ratio]) -> Delta {
    let mut delta = Delta::zero(3, position_a.len());
    delta.set_antisym(0, 1, position_a.to_vec());
    delta.set_antisym(0, 2, position_b.to_vec());
    delta
}

/// The DIVERGED refusal, measured: every nonzero residual (B − A)
/// carried whole in dimension order, C(Δ) the trailing summary.
fn diverged(a: &Chronicle, b: &Chronicle, at: u64) -> MeshRefused {
    let (position_a, position_b) = positions(a, b);
    let mut residuals = Vec::new();
    for (dimension, (ours, theirs)) in position_a.iter().zip(&position_b).enumerate() {
        let residual = theirs - ours;
        if residual != ratio(0, 1) {
            residuals.push((dimension as u32, residual));
        }
    }
    let coherence = coherence_functional(&triangle(&position_a, &position_b));
    MeshRefused::Diverged { at, residuals, coherence }
}
