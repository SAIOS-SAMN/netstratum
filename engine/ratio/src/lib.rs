//! The RFA carrier (NS-1 §1): exact rationals p/q. `BigRational`
//! construction reduces to canonical coprime form with a positive
//! denominator — there is exactly one representation of every value,
//! which is what makes the wire encoding byte-exact.

use num_bigint::{BigInt, BigUint, Sign};
use num_rational::BigRational;
use num_traits::One;

pub type Ratio = BigRational;

/// Build a ratio from small integers (test and rate literals).
pub fn ratio(numer: i64, denom: i64) -> Ratio {
    BigRational::new(BigInt::from(numer), BigInt::from(denom))
}

/// THE SEALED DEFINITION of canonical form (kernel I3 calls this; it never
/// defines canonicity itself — the equations are shared, never copied,
/// KERNEL CHARTER §0b). A value is canonical iff reducing it changes
/// nothing: `BigRational::new` normalizes to coprime terms with a positive
/// denominator, so `is_canonical(r)` holds exactly when `r` already equals
/// its own normalization. Every value CONSTRUCTED through this type is
/// canonical by construction — so this returns true for any well-formed
/// `Ratio`, and its role is the sealed law + defense-in-depth at the byte
/// boundary (a decode that produced a non-canonical value would fail here).
pub fn is_canonical(r: &Ratio) -> bool {
    BigRational::new(r.numer().clone(), r.denom().clone()) == *r
}

/// The canonical wire encoding (NS-1 §2.2):
/// `sign u8 (0 = non-negative, 1 = negative)`
/// `‖ LE32(numer_len) ‖ numer magnitude bytes (big-endian, no leading zeros)`
/// `‖ LE32(denom_len) ‖ denom magnitude bytes (big-endian, no leading zeros)`.
/// Because the carrier is canonical (coprime, positive denominator),
/// equal values encode to equal bytes — always.
pub fn encode(value: &Ratio, out: &mut Vec<u8>) {
    let sign: u8 = match value.numer().sign() {
        Sign::Minus => 1,
        _ => 0,
    };
    out.push(sign);
    let numer = value.numer().magnitude().to_bytes_be();
    let denom = value.denom().magnitude().to_bytes_be();
    for part in [numer, denom] {
        let bytes = trimmed(part);
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&bytes);
    }
}

/// Decode one ratio from the front of `bytes`; returns the value and
/// the number of bytes consumed, or `None` on a malformed carrier.
pub fn decode(bytes: &[u8]) -> Option<(Ratio, usize)> {
    let sign = *bytes.first()?;
    let mut at = 1usize;
    let mut parts: [BigUint; 2] = [BigUint::one(), BigUint::one()];
    for part in &mut parts {
        let len = u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?) as usize;
        at += 4;
        *part = BigUint::from_bytes_be(bytes.get(at..at + len)?);
        at += len;
    }
    let [numer_mag, denom_mag] = parts;
    let numer_sign = match (sign, numer_mag.bits()) {
        (1, _) => Sign::Minus,
        (_, 0) => Sign::NoSign,
        _ => Sign::Plus,
    };
    let numer = BigInt::from_biguint(numer_sign, numer_mag);
    let denom = BigInt::from_biguint(Sign::Plus, denom_mag);
    match denom.bits() {
        0 => None,
        _ => Some((BigRational::new(numer, denom), at)),
    }
}

/// The magnitude with leading zeros trimmed; zero encodes as the empty
/// byte string (`to_bytes_be` of zero is `[0]` — one canonical form
/// requires one canonical choice, and empty is it).
fn trimmed(bytes: Vec<u8>) -> Vec<u8> {
    let lead = bytes.iter().take_while(|byte| **byte == 0).count();
    bytes[lead..].to_vec()
}
