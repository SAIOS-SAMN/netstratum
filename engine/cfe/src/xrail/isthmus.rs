//! X7.2 — Isthmus opaque frame for court claims (no kernel payload inspect).

/// Magic for netstratum-local isthmus carrier frames.
pub const ISTHMUS_MAGIC: &[u8] = b"NS.X7.ISTH.v1";

/// Opaque court claim frame — carrier never inspects inner authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsthmusFrame {
    /// Opaque payload (datum-shaped claim bytes).
    pub payload: Vec<u8>,
    /// Outer integrity hash of payload.
    pub envelope_hash: [u8; 32],
}

pub fn encode_isthmus(payload: &[u8]) -> Vec<u8> {
    let envelope_hash = *blake3::hash(payload).as_bytes();
    let mut out = ISTHMUS_MAGIC.to_vec();
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(&envelope_hash);
    out.extend_from_slice(payload);
    out
}

pub fn decode_isthmus(bytes: &[u8]) -> Option<IsthmusFrame> {
    if !bytes.starts_with(ISTHMUS_MAGIC) {
        return None;
    }
    let rest = &bytes[ISTHMUS_MAGIC.len()..];
    if rest.len() < 4 + 32 {
        return None;
    }
    let len = u32::from_le_bytes(rest[..4].try_into().ok()?) as usize;
    let envelope_hash: [u8; 32] = rest[4..36].try_into().ok()?;
    let payload = rest.get(36..36 + len)?.to_vec();
    if payload.len() != len {
        return None;
    }
    if *blake3::hash(&payload).as_bytes() != envelope_hash {
        return None;
    }
    Some(IsthmusFrame {
        payload,
        envelope_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opaque_round_trip() {
        let payload = b"court-claim-opaque";
        let f = encode_isthmus(payload);
        let d = decode_isthmus(&f).expect("ok");
        assert_eq!(d.payload, payload);
    }

    #[test]
    fn bad_envelope_refuses() {
        let mut f = encode_isthmus(b"x");
        f[ISTHMUS_MAGIC.len() + 4] ^= 0xff;
        assert!(decode_isthmus(&f).is_none());
    }
}
