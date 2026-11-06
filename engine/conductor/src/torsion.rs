// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Torsion as a bivector, and its pressure (operator 2026-11-06: "the
//! multi-dimensional torsion equation in order to provide the pressure for
//! the twist in the expungement").
//!
//! - **Torsion is one signed component per pair of planes,** never the scalar
//!   `Σ|t_k − t̄_k|/2`. On the spatial planes the pairs are `(1,2)`, `(2,3)`,
//!   `(3,1)`; R's pairs follow with the R-plane turnings (issue #118).
//! - **Its rate is a velocity share** of the luminal bound: a pair's twist
//!   rate is the flow-law difference of its two planes' rates, `x_jk = x_j ⊖
//!   x_k`, signed and antisymmetric (`x_kj = −x_jk`), and bounded.
//! - **Its pressure** on a pair is the twisting matter's density times that
//!   rate, `P_jk = ρ·x_jk`, first degree.
//! - **A plane's pressure** is the flow-law composition of the two pairs it
//!   stands in, `P_k = P_kj ⊕ P_kl`, never their sum.
//! - **The expungement takes both** (operator 2026-11-06): the release share
//!   composes with the three planes' pressures, `s′ = s ⊕ P₁ ⊕ P₂ ⊕ P₃`, and a
//!   released parcel's rate on each plane with its plane's, `x_k′ = x_k ⊕ P_k`.

use cfe::ratio::{ratio, Ratio};

/// The flow law, `(x + y)/(1 + x·y)`; None outside its domain.
fn compose(x: &Ratio, y: &Ratio) -> Option<Ratio> {
    crate::ring::compose(x, y)
}

/// A pair's twist rate: `x_j ⊖ x_k`.
pub fn pair_rate(xj: &Ratio, xk: &Ratio) -> Option<Ratio> {
    compose(xj, &-xk.clone())
}

/// The spatial pairs, cyclic: `(1,2)`, `(2,3)`, `(3,1)`, as plane indices.
pub const PAIRS: [(usize, usize); 3] = [(0, 1), (1, 2), (2, 0)];

/// The pressure on each pair, `ρ·(x_j ⊖ x_k)`, from three plane rates.
pub fn pair_pressures(rates: &[Ratio; 3], rho: &Ratio) -> Option<[Ratio; 3]> {
    let mut out: [Ratio; 3] = Default::default();
    for (n, (j, k)) in PAIRS.iter().enumerate() {
        out[n] = rho.clone() * pair_rate(&rates[*j], &rates[*k])?;
    }
    Some(out)
}

/// Each plane's pressure: the flow-law composition of the two pairs it stands
/// in, each read with the plane's own sign (plane k gains where it leads its
/// partner): `P_k = P_kj ⊕ P_kl`.
pub fn plane_pressures(pairs: &[Ratio; 3]) -> Option<[Ratio; 3]> {
    let signed = |k: usize| -> Vec<Ratio> {
        PAIRS
            .iter()
            .enumerate()
            .filter_map(|(n, (j, l))| {
                if *j == k {
                    Some(pairs[n].clone())
                } else if *l == k {
                    Some(-pairs[n].clone())
                } else {
                    None
                }
            })
            .collect()
    };
    let mut out: [Ratio; 3] = Default::default();
    for (k, slot) in out.iter_mut().enumerate() {
        let parts = signed(k);
        *slot = parts
            .iter()
            .try_fold(ratio(0, 1), |acc, p| compose(&acc, p))?;
    }
    Some(out)
}

/// The release share taking the three planes' pressures, `s ⊕ P₁ ⊕ P₂ ⊕ P₃`,
/// held in `[0, 1)` (a share of what the throat holds); None outside the
/// flow law's domain.
pub fn released_share(s: &Ratio, planes: &[Ratio; 3]) -> Option<Ratio> {
    let v = planes
        .iter()
        .try_fold(s.clone(), |acc, p| compose(&acc, p))?;
    Some(v.max(ratio(0, 1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_untwisted_core_presses_nothing() {
        let third = ratio(1, 3);
        let rates = [third.clone(), third.clone(), third.clone()];
        let pairs = pair_pressures(&rates, &ratio(1, 2)).expect("in the bound");
        assert!(pairs.iter().all(|p| *p == ratio(0, 1)));
        let planes = plane_pressures(&pairs).expect("in the bound");
        assert!(planes.iter().all(|p| *p == ratio(0, 1)));
        assert_eq!(released_share(&ratio(1, 4), &planes), Some(ratio(1, 4)));
    }

    #[test]
    fn a_twist_is_antisymmetric_bounded_and_presses_its_leading_plane() {
        let rates = [ratio(1, 2), ratio(1, 3), ratio(1, 6)];
        assert_eq!(
            pair_rate(&rates[0], &rates[1]),
            pair_rate(&rates[1], &rates[0]).map(|x| -x)
        );
        let pairs = pair_pressures(&rates, &ratio(1, 1)).expect("in the bound");
        assert!(pairs.iter().all(|p| *p > ratio(-1, 1) && *p < ratio(1, 1)));
        let planes = plane_pressures(&pairs).expect("in the bound");
        // Plane 1 leads both its partners, plane 3 trails both.
        assert!(planes[0] > ratio(0, 1) && planes[2] < ratio(0, 1));
        // The release share moves with the twist and stays a share.
        let s = released_share(&ratio(1, 4), &planes).expect("in the bound");
        assert!(s >= ratio(0, 1) && s < ratio(1, 1));
    }
}
