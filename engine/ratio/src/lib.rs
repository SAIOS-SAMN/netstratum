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

/// **THE CANONICAL TEXT FORM OF AN EXACT RATIONAL.** Integers plain,
/// everything else `p/q` — never a decimal, never a float.
///
/// It lives here, beside the type it governs, because that is what it
/// is a fact about. It previously existed as two byte-identical copies
/// in `ledger/src/json.rs` and `lens/src/json.rs`, and two copies of
/// the rule that keeps floats out of an exact system is the one thing
/// in that duplication that could actually have gone wrong.
pub fn ratio_str(r: &Ratio) -> String {
    match r.denom() == &num_bigint::BigInt::from(1) {
        true => r.numer().to_string(),
        false => format!("{}/{}", r.numer(), r.denom()),
    }
}

// ─────────────────────────────────────────────────────────────────────
// THE TURN — exact angular geometry
// ─────────────────────────────────────────────────────────────────────
//
// **RELOCATED FROM `cfe::dop` (2026-08-05)** so the solver can address a
// SPHERE in the same frame the universe sites its orbs in (operator
// ruling: *grid needs to be discarded for spherical references*).
// `solver-verify` cannot reach `cfe` — the arrow runs the other way —
// and copying the constant would be the two-truths defect. `ratio` sits
// below everything and is where exact-rational geometry belongs.
// `cfe::dop` re-exports every name, so every existing citation resolves
// unchanged.

/// THE ORBITAL SLOTS — a turn divides into SIXTY.
///
/// **Base 60, for the reason base 60 has always been used.** It is the
/// smallest number that divides evenly by 1, 2, 3, 4, 5, 6, 10, 12, 15,
/// 20, 30 and 60 — twelve exact subdivisions. Halves, thirds, quarters,
/// fifths and sixths of a ring all land ON a slot, so a system can be
/// laid out symmetrically at many arities without any angle falling
/// between the teeth. Sixty is not a rounding: it is the number that
/// makes the most placements exact.
///
/// A slot is `k/60` for `k ∈ 0..60`, and `k = 60` is `k = 0` — the ring
/// closes.
///
/// **AND IT IS WHY ANGLES ARE TURNS, NOT RADIANS.** A full turn in
/// radians is `2π`, which no rational closes, so a wrapped angle would
/// need a float and the seam would never meet. In turns the seam is
/// exact and the circle is representable.
pub const TURN_SLOTS: i64 = 60;

/// Is this angle ON a slot? Refusing off-slot angles keeps placement
/// DISCRETE, and discreteness is what makes collision a decidable
/// question rather than a tolerance.
pub fn on_slot(theta: &Ratio) -> bool {
    let scaled = theta.clone() * ratio(TURN_SLOTS, 1);
    scaled.is_integer() && *theta >= ratio(0, 1) && *theta < ratio(1, 1)
}

/// The slot index of an angle that is on one.
pub fn slot_of(theta: &Ratio) -> Option<i64> {
    on_slot(theta).then(|| {
        let scaled = theta.clone() * ratio(TURN_SLOTS, 1);
        scaled.to_integer().try_into().unwrap_or(0)
    })
}

/// The angle of a slot, wrapping: `slot_angle(61)` is `slot_angle(1)`.
pub fn slot_angle(k: i64) -> Ratio {
    ratio(k.rem_euclid(TURN_SLOTS), TURN_SLOTS)
}

/// The wrapped gap between two turn-fractions — the SHORT WAY ROUND,
/// which is what makes each angular axis a circle rather than a segment.
pub fn turn_gap(a: &Ratio, b: &Ratio) -> Ratio {
    let raw = if a >= b { a.clone() - b.clone() } else { b.clone() - a.clone() };
    raw.clone().min(ratio(1, 1) - raw)
}
