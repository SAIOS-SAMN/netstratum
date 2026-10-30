//! X7.1 — Strand schema for STAMP / Note / Share packets (round-trip pure).

use crate::ratio::{self, Ratio};

/// Magic prefix — netstratum-local strand wire (compatible shape for Xylarium later).
pub const STRAND_MAGIC: &[u8] = b"NS.X7.STRAND.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StrandKind {
    Stamp = 1,
    Note = 2,
    Share = 3,
}

impl StrandKind {
    pub fn from_wire(b: u8) -> Option<Self> {
        match b {
            1 => Some(Self::Stamp),
            2 => Some(Self::Note),
            3 => Some(Self::Share),
            _ => None,
        }
    }
    pub fn word(self) -> &'static str {
        match self {
            Self::Stamp => "stamp",
            Self::Note => "note",
            Self::Share => "share",
        }
    }
}

/// Decoded strand tollway packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrandPacket {
    pub kind: StrandKind,
    pub amount: Ratio,
    /// Content-addressed claim id (BLAKE3 of body without magic).
    pub claim_hash: [u8; 32],
    /// Optional instrument id (note_id / share_id); 0 for bare STAMP.
    pub instrument_id: u64,
}

fn body_hash(body: &[u8]) -> [u8; 32] {
    *blake3::hash(body).as_bytes()
}

fn put_packet(kind: StrandKind, amount: &Ratio, instrument_id: u64) -> Vec<u8> {
    let mut body = Vec::new();
    body.push(kind as u8);
    body.extend_from_slice(&instrument_id.to_le_bytes());
    ratio::encode(amount, &mut body);
    let hash = body_hash(&body);
    let mut out = STRAND_MAGIC.to_vec();
    out.extend_from_slice(&hash);
    out.extend_from_slice(&body);
    out
}

pub fn encode_stamp_packet(amount: &Ratio) -> Vec<u8> {
    put_packet(StrandKind::Stamp, amount, 0)
}

pub fn encode_note_packet(note_id: u64, notional: &Ratio) -> Vec<u8> {
    put_packet(StrandKind::Note, notional, note_id)
}

pub fn encode_share_packet(share_id: u64, units: &Ratio) -> Vec<u8> {
    put_packet(StrandKind::Share, units, share_id)
}

/// Decode + integrity check (hash over body). Refuse = None.
pub fn decode_strand(bytes: &[u8]) -> Option<StrandPacket> {
    if !bytes.starts_with(STRAND_MAGIC) {
        return None;
    }
    let rest = &bytes[STRAND_MAGIC.len()..];
    if rest.len() < 32 + 1 + 8 {
        return None;
    }
    let claim_hash: [u8; 32] = rest[..32].try_into().ok()?;
    let body = &rest[32..];
    if body_hash(body) != claim_hash {
        return None; // tampered
    }
    let kind = StrandKind::from_wire(*body.first()?)?;
    let instrument_id = u64::from_le_bytes(body.get(1..9)?.try_into().ok()?);
    let (amount, _) = ratio::decode(body.get(9..)?)?;
    if amount <= Ratio::new(0.into(), 1.into()) {
        return None;
    }
    Some(StrandPacket {
        kind,
        amount,
        claim_hash,
        instrument_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ratio::ratio;

    #[test]
    fn stamp_packet_round_trips() {
        let p = encode_stamp_packet(&ratio(3, 2));
        let d = decode_strand(&p).expect("decode");
        assert_eq!(d.kind, StrandKind::Stamp);
        assert_eq!(d.amount, ratio(3, 2));
        assert_eq!(d.instrument_id, 0);
    }

    #[test]
    fn tampered_packet_refuses() {
        let mut p = encode_note_packet(7, &ratio(1, 1));
        let last = p.len() - 1;
        p[last] ^= 1;
        assert!(decode_strand(&p).is_none());
    }
}
