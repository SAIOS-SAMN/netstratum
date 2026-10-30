//! The RFA carrier (NS-1 §1): exact rationals p/q. `BigRational`
//! construction reduces to canonical coprime form with a positive
//! denominator — there is exactly one representation of every value,
//! which is what makes the wire encoding byte-exact.

use num_bigint::{BigInt, BigUint, Sign};
use num_rational::BigRational;
use num_traits::One;

pub type Ratio = BigRational;

/// The number grammar's ℝ: a span held by its exact bounds (docs/78 law 11).
pub mod span;
pub use span::{poles, Span};

/// Build a ratio from small integers (test and rate literals).
pub fn ratio(numer: i64, denom: i64) -> Ratio {
    BigRational::new(BigInt::from(numer), BigInt::from(denom))
}

/// The sealed definition of canonical form (kernel I3 calls this; it never
/// defines canonicity itself — the equations are shared, never copied,
/// kernel Charter §0b). A value is canonical iff reducing it changes
/// nothing: `BigRational::new` normalizes to coprime terms with a positive
/// denominator, so `is_canonical(r)` holds exactly when `r` already equals
/// its own normalization. Every value constructed through this type is
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

/// **The CANONICAL text form of an exact rational.** Integers plain,
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
// the turn — exact angular geometry
// ─────────────────────────────────────────────────────────────────────
//
// **Relocated from `cfe::dop` (2026-08-05)** so the solver can address a
// sphere in the same frame the universe sites its orbs in (operator
// ruling: *grid needs to be discarded for spherical references*).
// `solver-verify` cannot reach `cfe` — the arrow runs the other way —
// and copying the constant would be the two-truths defect. `ratio` sits
// below everything and is where exact-rational geometry belongs.
// `cfe::dop` re-exports every name, so every existing citation resolves
// unchanged.

/// **The derivation of the ring, computed rather than asserted.**
///
/// `gcd` and `lcm` as `const fn` so the ring's size is not a number
/// anyone typed — it is the answer to the requirement, evaluated by the
/// compiler. Change the requirement and the constant follows; there is
/// no second place to edit and no way for the two to disagree.
pub const fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (if a < 0 { -a } else { a }, if b < 0 { -b } else { b });
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// **The requirement the ring answers, and it is not a number either**
/// (operator ruling 2026-08-05: *all of the constants here need to be
/// derived dynamically*).
///
/// A first cut wrote `RING_SUBDIVISIONS = 6` — "halves through sixths
/// must land on a slot" — which derived sixty from a typed six and moved
/// the hard-coded constant one level down rather than removing it. Six was
/// never the requirement. The requirement is the engine's own CLOSED
/// vocabularies: a population must be layoutable symmetrically by any
/// partition it actually has, so the ring must admit each of their
/// arities.
///
/// ```text
///     3   dispositions   expansionist · empiricist · epicurean
///     3   metabolisms    mineral · biotic · plasmic
///     4   perceptions    planar · volumetric · harmonic · umbral
///     5   practices      the five receipted disciplines
/// ```
///
/// The distinct arities are `{3, 4, 5}` and `lcm(3, 4, 5) = 60`.
///
/// **STATIONS is not in this set (ruling 2026-08-23).** `STATIONS`
/// (`ledger::registry`) is an append-only OFFICE/ROLE taxonomy — T4–T9
/// (Wave-1, 2026-08-07), then T10–T15 (2026-08-08), then `elder`/
/// `ancient` (2026-08-13) grew it 4 → 10 → 16 → 18, and it will keep
/// growing as new offices land. It was folded into this set when it
/// still happened to divide 60 (at 10); once it grew to 18, `ring_
/// derivation_pins` caught the drift and named the fork: either bump
/// `TURN_SLOTS` to `lcm(…, 18) = 180` (cross-cutting — `solver-verify/
/// src/shell.rs`'s orbital shell geometry and `shell_pins.rs` hard-
/// assert `TURN_SLOTS == 60`), or recognize the coupling was never
/// load-bearing for stations in the first place. No production code
/// positions a station on the turn ring (`STATIONS.len()` appeared
/// only in this test's own arrays, never in `shell.rs`, never in a
/// ring-placement call) — unlike dispositions/metabolisms/perceptions,
/// which the incursion generator does draw uniformly over
/// (`conductor::lib::beacon.uniform_index`), a real (if not yet
/// ring-positioned) population partition. Sixty already is the
/// requirement's own answer for the four vocabularies that remain in
/// it — the base-60 orbital ring (below) stands on its own, unmoved
/// by however many offices the game later names.
///
/// **The vocabularies live above this crate** — `ledger::registry` and
/// `cfe` both depend on `ratio`, never the reverse — so the arities are
/// stated here and pinned against the live vocabularies from up there
/// (`ring_derivation_pins`). Copying them down would be the two-truths
/// defect; stating the requirement and proving it matches is not.
pub const RING_ARITIES: [i64; 3] = [3, 4, 5];

/// The lcm of a set of arities — the smallest ring admitting them all.
pub const fn lcm_of(arities: &[i64]) -> i64 {
    let mut held = 1i64;
    let mut at = 0usize;
    while at < arities.len() {
        let k = arities[at];
        if k > 0 {
            held = held / gcd(held, k) * k;
        }
        at += 1;
    }
    held
}

/// The orbital slots — a turn divides into sixty.
///
/// **Base 60, for the reason base 60 has always been used.** It is the
/// smallest number that divides evenly by 1, 2, 3, 4, 5, 6, 10, 12, 15,
/// 20, 30 and 60 — twelve exact subdivisions. Halves, thirds, quarters,
/// fifths and sixths of a ring all land on a slot, so a system can be
/// laid out symmetrically at many arities without any angle falling
/// between the teeth. Sixty is not a rounding: it is the number that
/// makes the most placements exact.
///
/// A slot is `k/60` for `k ∈ 0..60`, and `k = 60` is `k = 0` — the ring
/// closes.
///
/// **And it is why angles are turns, not radians.** A full turn in
/// radians is `2π`, which no rational closes, so a wrapped angle would
/// need a float and the seam would never meet. In turns the seam is
/// exact and the circle is representable.
/// **Derived: `lcm(3, 4, 5) = 60`.** Not typed — evaluated from
/// [`RING_ARITIES`], the arities of the engine's own closed
/// vocabularies. Sixty is not a rounding and not a tradition here: it is
/// the smallest ring on which every partition the population actually
/// has can be laid out symmetrically. Land a sixth practice and the ring
/// follows on its own.
pub const TURN_SLOTS: i64 = lcm_of(&RING_ARITIES);

/// **Does A division of the ring hold?** The proof a caller gets instead
/// of a bool: the arity, the ring it was checked against, and the
/// remainder that decided it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DivisionProof {
    pub arity: i64,
    pub ring: i64,
    pub remainder: i64,
    pub cell_span: i64,
}

impl DivisionProof {
    pub fn holds(&self) -> bool {
        self.remainder == 0 && self.arity > 0
    }
    /// Why it holds or does not, in the caller's own terms.
    pub fn because(&self) -> String {
        match self.holds() {
            true => format!(
                "{} divides the {}-slot ring exactly; each cell spans {} slots",
                self.arity, self.ring, self.cell_span
            ),
            false if self.arity <= 0 => "an arity must be positive".into(),
            false => format!(
                "{} does not divide the {}-slot ring — {} slots are left over, so a cell \
                 centre would fall between teeth",
                self.arity, self.ring, self.remainder
            ),
        }
    }
}

/// Divide a ring of `ring` slots into `arity` cells, proving the result.
pub const fn divide_ring(ring: i64, arity: i64) -> DivisionProof {
    if arity <= 0 {
        return DivisionProof { arity, ring, remainder: -1, cell_span: 0 };
    }
    DivisionProof {
        arity,
        ring,
        remainder: ring % arity,
        cell_span: ring / arity,
    }
}

/// Is this angle on a slot? Refusing off-slot angles keeps placement
/// discrete, and discreteness is what makes collision a decidable
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

/// The wrapped gap between two turn-fractions — the short way round,
/// which is what makes each angular axis a circle rather than a segment.
pub fn turn_gap(a: &Ratio, b: &Ratio) -> Ratio {
    let raw = if a >= b { a.clone() - b.clone() } else { b.clone() - a.clone() };
    raw.clone().min(ratio(1, 1) - raw)
}

/// **A slot — an angle proven on the ring at construction** (29_CONSTRUCTION
/// C1: apply from theorems, never judge the outcome).
///
/// `Ratio` can spell any turn-fraction, so every consumer of an angle
/// had to re-check `on_slot` and refuse — a shape refusal (`OffSlot`)
/// standing where a type should be. A `Slot` cannot be off the ring:
/// [`Slot::from_index`] is total and wraps, [`Slot::from_angle`] is the
/// one door, and past it there is nothing left to check. The `OffSlot`
/// refusal died when this type landed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slot(Ratio);

impl Slot {
    /// Total: any index lands on the wrapped ring.
    pub fn from_index(k: i64) -> Slot {
        Slot(slot_angle(k))
    }
    /// The one door for arbitrary angles — `None` carries the fault:
    /// the angle sits between the teeth.
    pub fn from_angle(theta: Ratio) -> Option<Slot> {
        on_slot(&theta).then_some(Slot(theta))
    }
    /// The angle, as the exact turn-fraction it always was.
    pub fn angle(&self) -> &Ratio {
        &self.0
    }
    /// The slot index — total, because a Slot is on the ring by
    /// construction. This is `slot_of` with the `Option` gone: the
    /// `Option` existed only to carry the judgment this type retired.
    pub fn index(&self) -> i64 {
        let scaled = self.0.clone() * ratio(TURN_SLOTS, 1);
        scaled.to_integer().try_into().unwrap_or(0)
    }
}

/// **A unit-interval value — `x ∈ [0, 1)`, proven at construction.**
///
/// Albedo, route efficiency and every other half-open-unit quantity
/// carried bare `Ratio`s, so each verb re-checked the range and refused
/// (`BadSiting`, `BadEta` — shape refusals standing where a type should
/// be). A `Unit` cannot leave the interval; the checks die at every
/// consumer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unit(Ratio);

impl Unit {
    /// The value door — `None` carries the fault.
    pub fn new(value: Ratio) -> Option<Unit> {
        (value >= ratio(0, 1) && value < ratio(1, 1)).then_some(Unit(value))
    }
    pub fn value(&self) -> &Ratio {
        &self.0
    }
}

/// **A strictly positive value — `x > 0`, proven at construction.**
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Positive(Ratio);

impl Positive {
    pub fn new(value: Ratio) -> Option<Positive> {
        (value > ratio(0, 1)).then_some(Positive(value))
    }
    pub fn value(&self) -> &Ratio {
        &self.0
    }
}

/// **The literal doors** — for literals written in source, where the
/// author can see the numbers. They panic on an out-of-range literal,
/// which is the same class as [`ratio`] itself panicking on `d = 0`:
/// a malformed literal is a build error wearing a run-time coat, not a
/// runtime condition. Values that arrive at runtime go through
/// [`Unit::new`] / [`Positive::new`], which fault instead.
pub fn unit(numer: i64, denom: i64) -> Unit {
    Unit::new(ratio(numer, denom)).expect("a unit literal must sit in [0, 1)")
}
pub fn positive(numer: i64, denom: i64) -> Positive {
    Positive::new(ratio(numer, denom)).expect("a positive literal must exceed 0")
}

/// **A non-negative value — `x ≥ 0`, proven at construction.** The mint
/// amount's shape: zero is lawful (a rung can pay nothing), negative is
/// not a quantity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonNegative(Ratio);

impl NonNegative {
    pub fn new(value: Ratio) -> Option<NonNegative> {
        (value >= ratio(0, 1)).then_some(NonNegative(value))
    }
    pub fn value(&self) -> &Ratio {
        &self.0
    }
}

/// The literal door, same class as [`unit`]/[`positive`].
pub fn non_negative(numer: i64, denom: i64) -> NonNegative {
    NonNegative::new(ratio(numer, denom)).expect("a non-negative literal must be ≥ 0")
}
