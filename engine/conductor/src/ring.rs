//! docs/83 "The derivation" and docs/111: the ring-density law (with
//! Xylarium, shared form). A core's bounce emits one front, the geodesic
//! sphere of light-time about it, growing from S^0; a body the front meets
//! receives it and emits nothing, except as a surface (`surface`), whose
//! share of the front is its own bounce. The front crosses empty space to a
//! place at r (in light-rings) in r and is slowed by the holding inside r,
//! `H(<r)`: it arrives after `r + H(<r)` (`arrival`). Mass is holding
//! (`M = H`); no G, no c, no mass unit.
//!
//! - **The clock:** a place's rate is the fronts it receives,
//!   `r / (r + H)`.
//! - **The deficit:** the share of the turn the holding withholds,
//!   `δ = H / (r + H)`, so `δ < 1`, approaching 1 as `r → 0`; the clock is
//!   `1 − δ`.
//! - **The turning:** each front a place receives turns it by its deficit,
//!   `ω = δ`, so a turn takes `P = 1 / δ = 1 + R / H` received fronts:
//!   linear, no power.
//! - **The spin** (reach × holding, with Xylarium): `ℓ = R · H`, a product
//!   of two distinct readings (the rings of reach and the rings held inside
//!   it), never the square of one. Conserved; a system adds `Σ m·ℓ`.
//! - **The balance:** a body of spin ℓ stands at `R = ℓ / H`. Orbits
//!   breathe: the law fixes no direction; an orbit draws in when the holding
//!   inside it grows or it loses spin, and moves out when the holding falls
//!   or it gains spin. The ledger decides which. Escape is read at events
//!   (the stretch crossing e → 1), never a share of H.
//! - **The throat** (docs/111 §3.1): a surface, never matter. A body of
//!   holding `H_b` and extent `R_b` has its throat, the mouth of the tube at
//!   its equator `R = H`, iff `R_b ≤ H_b`; its density share
//!   `ρ = min(1, H_b / R_b)` is how near it stands to opening one.
//!
//! In transition: H is read from the grains' holds in units of the deepest
//! hole (the core's own state hold is not yet on the ledger, docs/83), and a
//! received front is a conductor bell.
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// A place's clock: `r / (r + H)`; 1 where nothing holds.
pub fn rate(r: &Ratio, held: &Ratio) -> Ratio {
    if held.is_zero() {
        return ratio(1, 1);
    }
    r.clone() / (r.clone() + held.clone())
}

/// The deficit: `H / (r + H)`, the speed of a turning at r as a share of
/// light.
pub fn deficit(r: &Ratio, held: &Ratio) -> Ratio {
    if held.is_zero() {
        return ratio(0, 1);
    }
    held.clone() / (r.clone() + held.clone())
}

/// A turn's length at reach r: `1 / δ = 1 + R / H` received rings; zero
/// where nothing holds (nothing turns it).
pub fn period(r: &Ratio, held: &Ratio) -> Ratio {
    if held.is_zero() {
        return ratio(0, 1);
    }
    ratio(1, 1) + r.clone() / held.clone()
}

/// The front's arrival at reach r through the holding inside it,
/// `a(r) = r + H(<r)` (docs/111 §2): its delay through the held medium.
pub fn arrival(r: &Ratio, held: &Ratio) -> Ratio {
    r.clone() + held.clone()
}

/// A body's density share (docs/111 §3.1): `min(1, H_b / R_b)`, 1 where its
/// extent lies within its equator (its throat is open); 1 at no extent with
/// any holding, 0 where nothing holds.
pub fn density(extent: &Ratio, held: &Ratio) -> Ratio {
    if held.is_zero() {
        return ratio(0, 1);
    }
    if extent <= held {
        return ratio(1, 1);
    }
    held.clone() / extent.clone()
}

/// Whether a body's throat is open: its extent within its equator,
/// `R_b ≤ H_b` (the surface deficit `δ_s ≥ ½`).
pub fn open(extent: &Ratio, held: &Ratio) -> bool {
    !held.is_zero() && extent <= held
}

/// A body's share of a front as a surface (docs/111 §3): its density and
/// its activity each count, `σ = 1 − (1 − ρ)(1 − α)`.
pub fn surface(density: &Ratio, activity: &Ratio) -> Ratio {
    let one = ratio(1, 1);
    one.clone() - (one.clone() - density.clone()) * (one - activity.clone())
}

/// The front's strength past the surfaces it crossed, `S = Π (1 − σ_j)`.
pub fn strength(surfaces: &[Ratio]) -> Ratio {
    surfaces
        .iter()
        .fold(ratio(1, 1), |s, x| s * (ratio(1, 1) - x.clone()))
}

/// The pair share of two mouths' holds (docs/114 T3):
/// `p = (h_b − h_a)/(h_b + h_a)`, in `(−1, 1)`; None where neither holds.
pub fn tube_share(h_a: &Ratio, h_b: &Ratio) -> Option<Ratio> {
    let both = h_a.clone() + h_b.clone();
    if both.is_zero() {
        return None;
    }
    Some((h_b.clone() - h_a.clone()) / both)
}

/// LAW-DILATION, `D(p) = (1 + p)/(1 − p)`: the scale of mouth B against
/// mouth A, `h_b/h_a = D(p)` (docs/114 T3). Refused at `p ≤ −1` or `p ≥ 1`.
pub fn dilation(p: &Ratio) -> Option<Ratio> {
    let one = ratio(1, 1);
    if *p <= -one.clone() || *p >= one {
        return None;
    }
    Some((one.clone() + p.clone()) / (one - p.clone()))
}

/// The flow law, `x ⊕ y = (x + y)/(1 + x·y)`: chained tubes compose their
/// pair shares by it, `p_ac = p_ab ⊕ p_bc` (docs/114 T3). None where
/// `1 + x·y = 0`.
pub fn compose(x: &Ratio, y: &Ratio) -> Option<Ratio> {
    let den = ratio(1, 1) + x.clone() * y.clone();
    if den.is_zero() {
        return None;
    }
    Some((x.clone() + y.clone()) / den)
}

/// The flow law over a stride (docs/132 §1): `x` composed with itself `n`
/// times, the rate whose dilation is `D(x)ⁿ`,
/// `((1 + x)ⁿ − (1 − x)ⁿ)/((1 + x)ⁿ + (1 − x)ⁿ)`, exact: the law composed,
/// never a rate times n. `n = 1` is `x`; `n = 0` is rest. None where the
/// composition has no value (a pole, `x = −1` at even n).
pub fn compose_n(x: &Ratio, n: u32) -> Option<Ratio> {
    use num_traits::Pow;
    let (up, down) = (ratio(1, 1) + x.clone(), ratio(1, 1) - x.clone());
    let (a, b) = (Pow::pow(&up, n), Pow::pow(&down, n));
    let den = a.clone() + b.clone();
    if den.is_zero() {
        return None;
    }
    Some((a - b) / den)
}

/// The specific spin of a body at reach r about the holding H inside it:
/// `ℓ = R · H`.
pub fn spin(r: &Ratio, held: &Ratio) -> Ratio {
    r.clone() * held.clone()
}

/// The pinch's share (operator 2026-10-18; the enfold law's gravity reach):
/// where a body's pull on its own outweighs its parent's, `H_b/(d + H_b) =
/// H_p/((R − d) + H_p)`, so its reach is this share of its own reach about
/// the parent, `H_b/(H_b + H_p)`; zero where neither holds.
pub fn pinch_share(h_body: &Ratio, h_parent: &Ratio) -> Ratio {
    let both = h_body.clone() + h_parent.clone();
    if both.is_zero() {
        return ratio(0, 1);
    }
    h_body.clone() / both
}

/// The pinch: a body's own reach at reach `r` about its parent,
/// `d = r·H_b/(H_b + H_p)`.
pub fn pinch(r: &Ratio, h_body: &Ratio, h_parent: &Ratio) -> Ratio {
    r.clone() * pinch_share(h_body, h_parent)
}

/// The balance: the reach where spin ℓ stands about H, `ℓ / H`; None where
/// nothing holds it or it carries no spin.
pub fn radius(ell: &Ratio, held: &Ratio) -> Option<Ratio> {
    if held.is_zero() || *ell <= ratio(0, 1) {
        return None;
    }
    Some(ell.clone() / held.clone())
}

/// The spin GRAIN (design law 8, exact and bounded): a body's specific spin
/// and its place are held on 60⁻⁶ (six sexagesimal places, the TUniversal
/// coordinate's), floored; what flooring leaves goes back to the medium the
/// spin came from, so the ledger stays exact while no body's numbers grow
/// with the births and echoes that made it.
pub fn grain(x: &Ratio) -> Ratio {
    let places = Ratio::from_integer(num_bigint::BigInt::from(60).pow(6));
    (x.clone() * places.clone()).floor() / places
}

/// One mark of the spin grain, `60⁻⁶`: the width of a certified interval's
/// bounds on the grain (`flows::torque`).
pub fn grain_step() -> Ratio {
    Ratio::new(
        num_bigint::BigInt::from(1),
        num_bigint::BigInt::from(60).pow(6),
    )
}

/// The whole count of a non-negative exact value, `⌊x⌋`, where a `u64`
/// carries it; refused otherwise (never a default).
pub fn whole(x: &Ratio) -> Option<u64> {
    if *x < ratio(0, 1) {
        return None;
    }
    x.floor().to_integer().try_into().ok()
}

/// A share's sector on the 60⁶-mark protractor (the TUniversal coordinate's
/// six sexagesimal places, docs/78): which sector holds it, by exact floor.
pub fn sector(share: &Ratio) -> u64 {
    let places = Ratio::from_integer(num_bigint::BigInt::from(60).pow(6));
    (share.clone() * places)
        .floor()
        .to_integer()
        .try_into()
        .unwrap_or(0)
}

/// A body's IDENTITY is its birth reading, never a row number (operator
/// 2026-10-16: "that's spreadsheet logic"): the bell it was born and the
/// sector of its holding share (the deficit `H/(R + H)` at its reach, the
/// coordinate's G lane) where it was born, `born · 2³⁶ + sector`. Unique
/// among its siblings; on an exact tie the next sector is taken.
pub fn birth_key(born: u64, reach: &Ratio, held: &Ratio, siblings: &[u64]) -> u64 {
    let mut key = (born << 36) | sector(&deficit(reach, held));
    while siblings.contains(&key) {
        key += 1;
    }
    key
}

#[cfg(test)]
mod stride_tests {
    //! docs/132 §1 and §5: the stride's closed forms.
    use super::*;
    use num_traits::Signed as _;

    /// Witness 1 (the flow law): n bells in closed form are n compositions,
    /// at N and 4N.
    #[test]
    fn the_flow_law_over_n_bells_is_its_n_fold_composition() {
        for x in [ratio(1, 3), ratio(-2, 7), ratio(5, 9), ratio(0, 1)] {
            for n in [3u32, 12] {
                let mut step = ratio(0, 1);
                for _ in 0..n {
                    step = compose(&step, &x).expect("in the domain");
                }
                assert_eq!(compose_n(&x, n), Some(step), "{x} over {n}");
            }
            assert_eq!(compose_n(&x, 1), Some(x.clone()));
            assert_eq!(compose_n(&x, 0), Some(ratio(0, 1)));
        }
    }

    /// Witnesses 5 and 6 (§5): met at q landing phases a turn's q-th apart
    /// (a mote turning `a/q` between echoes), the push's mean response is
    /// the response's own over q times the arc from q times the phase,
    /// `mean_response(q·ψ₀, q·len)/q`, so it is bounded by `1/q`: a
    /// resonance with small q has a mean, a draw filling the circle tends to
    /// none. Read at q = N and 4N.
    #[test]
    fn the_echo_draw_at_a_resonance_has_the_mean_one_qth_of_the_folded_response() {
        for q in [7i64, 28] {
            for (psi, len) in [
                (ratio(1, 11), ratio(1, 5)),
                (ratio(2, 3), ratio(3, 2)),
                (ratio(1, 20), ratio(1, 4)),
            ] {
                let mean = (0..q).fold(ratio(0, 1), |s, j| {
                    s + crate::flows::mean_response(&(psi.clone() + ratio(j, q)), &len)
                }) / ratio(q, 1);
                let folded = crate::flows::mean_response(
                    &(psi.clone() * ratio(q, 1)),
                    &(len.clone() * ratio(q, 1)),
                ) / ratio(q, 1);
                assert_eq!(mean, folded, "q {q}");
                assert!(mean.abs() <= ratio(1, q), "bounded by 1/q");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_whole_count_is_read_or_refused_never_defaulted() {
        assert_eq!(whole(&ratio(7, 2)), Some(3));
        assert_eq!(whole(&ratio(0, 1)), Some(0));
        assert_eq!(whole(&ratio(-1, 2)), None);
        let past = Ratio::from_integer(num_bigint::BigInt::from(2).pow(70));
        assert_eq!(whole(&past), None);
    }

    #[test]
    fn chained_mouths_compose_by_the_flow_law() {
        let (ha, hb, hc) = (ratio(2, 1), ratio(5, 1), ratio(11, 3));
        // docs/114 T3: the scale between mouths is D of their pair share, and a chain
        // A -> B -> C composes by the flow law.
        let (pab, pbc, pac) = (
            tube_share(&ha, &hb).expect("p"),
            tube_share(&hb, &hc).expect("p"),
            tube_share(&ha, &hc).expect("p"),
        );
        assert_eq!(dilation(&pab), Some(hb.clone() / ha.clone()));
        assert_eq!(
            dilation(&pab).zip(dilation(&pbc)).map(|(x, y)| x * y),
            dilation(&pac)
        );
        assert_eq!(compose(&pab, &pbc), Some(pac));
        assert_eq!(dilation(&ratio(1, 1)), None);
    }

    #[test]
    fn the_luminal_bound_saturates_exactly_at_the_throat() {
        // docs/114 T5: a mote turns 2δ a bell; 2δ > 1 exactly inside the
        // throat (x < 1), 2δ = 1 on it.
        let h = ratio(1, 1);
        assert_eq!(ratio(2, 1) * deficit(&h, &h), ratio(1, 1));
        assert!(ratio(2, 1) * deficit(&ratio(9, 10), &h) > ratio(1, 1));
        assert!(ratio(2, 1) * deficit(&ratio(11, 10), &h) < ratio(1, 1));
    }

    #[test]
    fn a_throat_opens_where_the_extent_lies_within_the_equator() {
        let h = ratio(2, 1);
        assert!(open(&ratio(2, 1), &h), "R = H is the equator");
        assert!(!open(&ratio(3, 1), &h));
        assert_eq!(density(&ratio(4, 1), &h), ratio(1, 2));
        assert_eq!(density(&ratio(1, 1), &h), ratio(1, 1));
        assert_eq!(
            deficit(&ratio(2, 1), &h),
            ratio(1, 2),
            "δ_s = ½ at the throat"
        );
        // Density and activity each count; an open body is wholly a surface.
        assert_eq!(surface(&ratio(1, 2), &ratio(1, 2)), ratio(3, 4));
        assert_eq!(surface(&ratio(1, 1), &ratio(0, 1)), ratio(1, 1));
        assert_eq!(surface(&ratio(0, 1), &ratio(0, 1)), ratio(0, 1));
        assert_eq!(strength(&[ratio(1, 2), ratio(1, 3)]), ratio(1, 3));
        assert_eq!(arrival(&ratio(3, 1), &ratio(1, 1)), ratio(4, 1));
    }

    #[test]
    fn one_law_for_the_clock_the_motion_and_the_balance() {
        let (r, h) = (ratio(3, 1), ratio(1, 1));
        assert_eq!(rate(&r, &h), ratio(3, 4));
        assert_eq!(deficit(&r, &h), ratio(1, 4));
        assert_eq!(
            rate(&r, &h) + deficit(&r, &h),
            ratio(1, 1),
            "the clock is 1 − δ"
        );
        assert_eq!(
            period(&r, &h),
            ratio(4, 1),
            "one turn in 1/δ received rings"
        );
        // The balance gives back the reach its spin was read at.
        assert_eq!(radius(&spin(&r, &h), &h), Some(r.clone()));
        // Orbits breathe: half the holding, twice the reach; double, half.
        assert_eq!(radius(&spin(&r, &h), &ratio(1, 2)), Some(ratio(6, 1)));
        assert_eq!(radius(&spin(&r, &h), &ratio(2, 1)), Some(ratio(3, 2)));
        assert!(
            deficit(&ratio(1, 1_000_000), &ratio(1_000_000, 1)) < ratio(1, 1),
            "bounded below light"
        );
    }
}
