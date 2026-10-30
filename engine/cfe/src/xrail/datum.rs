//! X7.3 — Datum-style multi-axial court claim verify (local pure stand-in).
//!
//! Full PoW++ lives in the datum repo; this module verifies a **multi-axial close
//! envelope**: magic · axes seal · work digest · amount. Reward only when
//! all axes present and amount positive — no free mint on thin claims.

use crate::ratio::{self, Ratio};

pub const DATUM_MAGIC: &[u8] = b"NS.X7.DATUM.v1";

/// Court claim after multi-axial close (netstratum-local verify shape).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtClaim {
    /// Disposition · metabolism · perception codes (1..=3 / 1..=3 / 1..=4).
    pub axes: [u8; 3],
    /// Work digest (BLAKE3) that the claim is bound to.
    pub work: [u8; 32],
    pub amount: Ratio,
}

/// Encode a court claim for isthmus payload.
pub fn encode_court_claim(claim: &CourtClaim) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&claim.axes);
    body.extend_from_slice(&claim.work);
    ratio::encode(&claim.amount, &mut body);
    let mut out = DATUM_MAGIC.to_vec();
    out.extend_from_slice(&body);
    out
}

/// Verify multi-axial close: all axes non-zero, work non-zero, amount > 0.
pub fn verify_court_claim(bytes: &[u8]) -> Option<CourtClaim> {
    if !bytes.starts_with(DATUM_MAGIC) {
        return None;
    }
    let body = &bytes[DATUM_MAGIC.len()..];
    if body.len() < 3 + 32 {
        return None;
    }
    let axes = [body[0], body[1], body[2]];
    if axes.contains(&0) {
        return None; // multi-axial: every axis must seal
    }
    // Closed vocab floors: disposition/metabolism 1..=3, perception 1..=4.
    if axes[0] > 3 || axes[1] > 3 || axes[2] > 4 {
        return None;
    }
    let work: [u8; 32] = body[3..35].try_into().ok()?;
    if work == [0u8; 32] {
        return None;
    }
    let (amount, _) = ratio::decode(&body[35..])?;
    if amount <= Ratio::new(0.into(), 1.into()) {
        return None;
    }
    Some(CourtClaim { axes, work, amount })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ratio::ratio;

    #[test]
    fn multi_axial_close_rewards() {
        let c = CourtClaim {
            axes: [1, 2, 3],
            work: [7u8; 32],
            amount: ratio(5, 1),
        };
        let b = encode_court_claim(&c);
        let v = verify_court_claim(&b).expect("verify");
        assert_eq!(v.amount, ratio(5, 1));
    }

    #[test]
    fn missing_axis_refuses() {
        let c = CourtClaim {
            axes: [1, 0, 3],
            work: [1u8; 32],
            amount: ratio(1, 1),
        };
        assert!(verify_court_claim(&encode_court_claim(&c)).is_none());
    }
}
