//! The plasma layer (docs/134): each kind's electrons' state at its place and
//! heat, and the impulse a release gives shared through that charge.
//!
//! - **The bound share** (§1): an electron is bound where it is held at the
//!   place (the pair's held share `g`, docs/105 §3) and its winding is
//!   tighter than the heat (`cfe::almanac::atom::open_at`'s stripping), so
//!   `b_z = g·E_z(a)/Q_z`, `E_z(a)` the electrons kept in windings tighter
//!   than `a` (a cluster keeps every electron its bonds hold; a kind with no
//!   protons `b = 1`).
//! - **The release** (§2): the bound share answers by its own inertia, the
//!   ionized share as one plasma, `ℓ_z = b_z·F_z/w_z + (1 − b_z)·S` with
//!   `S = Σ n(1 − b)F / Σ n(1 − b)w`; `Σ n·w·ℓ = Σ n·F` exact for any `b`.
use crate::chem::Kind;
use crate::pulse::Manifest;
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// The electrons kind `k` keeps at heat `a` (hydrogen's unit): an atom its
/// windings held tighter than `a`; a cluster every electron of its parts.
pub fn kept_electrons(k: &Kind, heat: &Ratio) -> u64 {
    match crate::chem::atom_w(k) {
        Some(w) => {
            let q = crate::chem::protons(w);
            if *heat <= ratio(0, 1) {
                return u64::from(q);
            }
            cfe::almanac::atom::windings(q).map_or(0, |(_, ws)| {
                ws.iter()
                    .filter(|(t, _, _)| *t > *heat)
                    .map(|(_, v, _)| *v)
                    .sum()
            })
        }
        None => u64::from(crate::chem::charge(k)),
    }
}

/// The bound share of kind `k` at the place's held face `g` and heat `a`,
/// `g·E(a)/Q` (no protons: 1, nothing to ionize).
pub fn bound_share(k: &Kind, g: &Ratio, heat: &Ratio) -> Ratio {
    let q = crate::chem::charge(k);
    if q == 0 {
        return ratio(1, 1);
    }
    let kept = i64::try_from(kept_electrons(k, heat)).unwrap_or(i64::MAX);
    g.clone() * ratio(kept, 1) / ratio(i64::from(q), 1)
}

/// The release shared through the charge (docs/134 §2): of each part
/// `(amount n, weight w, impulse a nucleus F, bound share b)`, its spin
/// `b·F/w + (1 − b)·S`, `S` the ionized pool's impulse over its holding.
pub fn carried(parts: &[(Ratio, Ratio, Ratio, Ratio)]) -> Vec<Ratio> {
    let one = ratio(1, 1);
    let (impulse, holding) = parts.iter().fold(
        (ratio(0, 1), ratio(0, 1)),
        |(i, h), (n, w, f, b)| {
            let free = n.clone() * (one.clone() - b.clone());
            (i + free.clone() * f.clone(), h + free * w.clone())
        },
    );
    let pool = if holding.is_zero() {
        ratio(0, 1)
    } else {
        impulse / holding
    };
    parts
        .iter()
        .map(|(_, w, f, b)| {
            let own = if w.is_zero() {
                ratio(0, 1)
            } else {
                f.clone() / w.clone()
            };
            b.clone() * own + (one.clone() - b.clone()) * pool.clone()
        })
        .collect()
}

/// The bounce's release at the throat, cold (docs/134 §2): each kind of
/// `pool` given the impulse `w̄` a nucleus (the push's `p·w̄` over `p`), at
/// the throat's held face, heat 0. Returns each kind's spin over `p`.
pub fn bounce_unit(pool: &Manifest, mean: &Ratio) -> Manifest {
    let g = crate::polarity::throat_held();
    let kinds: Vec<&Kind> = pool.keys().collect();
    let parts: Vec<(Ratio, Ratio, Ratio, Ratio)> = pool
        .iter()
        .map(|(k, n)| {
            (
                n.clone(),
                ratio(i64::from(crate::chem::weight(k)), 1),
                mean.clone(),
                bound_share(k, &g, &ratio(0, 1)),
            )
        })
        .collect();
    kinds
        .into_iter()
        .cloned()
        .zip(carried(&parts))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(n: i64, d: i64) -> Ratio {
        ratio(n, d)
    }

    #[test]
    fn the_release_keeps_the_impulse_for_any_bound_shares() {
        let parts = vec![
            (r(7, 1), r(1, 1), r(3, 1), r(1, 5)),
            (r(2, 1), r(12, 1), r(3, 1), r(1, 2)),
            (r(1, 3), r(56, 1), r(5, 2), r(9, 10)),
        ];
        let ell = carried(&parts);
        let held = parts
            .iter()
            .zip(&ell)
            .fold(r(0, 1), |s, ((n, w, _, _), l)| s + n.clone() * w.clone() * l.clone());
        let given = parts
            .iter()
            .fold(r(0, 1), |s, (n, _, f, _)| s + n.clone() * f.clone());
        assert_eq!(held, given);
    }

    #[test]
    fn bound_whole_is_each_alone_and_ionized_whole_is_one_spin() {
        let mut parts = vec![
            (r(7, 1), r(1, 1), r(3, 1), r(1, 1)),
            (r(2, 1), r(12, 1), r(4, 1), r(1, 1)),
        ];
        assert_eq!(carried(&parts), vec![r(3, 1), r(1, 3)]);
        for p in &mut parts {
            p.3 = r(0, 1);
        }
        let s = (r(21, 1) + r(8, 1)) / (r(7, 1) + r(24, 1));
        assert_eq!(carried(&parts), vec![s.clone(), s]);
    }

    #[test]
    fn the_bound_share_is_the_pair_cold_and_none_stripped() {
        let g = crate::polarity::throat_held();
        let h = crate::chem::hydrogen();
        assert_eq!(bound_share(&h, &g, &r(0, 1)), g);
        assert_eq!(bound_share(&h, &g, &r(1, 1)), r(0, 1));
        let iron = crate::chem::Kind::element(26).expect("iron");
        let warm = bound_share(&iron, &g, &r(1, 1));
        assert!(warm > r(0, 1) && warm <= g, "{warm} {}", crate::chem::charge(&iron));
        let top = cfe::almanac::atom::windings(crate::chem::charge(&iron)).expect("w").1.iter().map(|w| w.0.clone()).max().expect("t");
        assert!(bound_share(&iron, &g, &top) < warm);
        assert_eq!(bound_share(&iron, &g, &top), r(0, 1));
    }

    #[test]
    fn the_bounce_at_one_bound_share_pools_at_one() {
        let mut pool = Manifest::new();
        pool.insert(crate::chem::hydrogen(), r(10, 1));
        pool.insert(crate::chem::Kind::element(26).expect("iron"), r(1, 1));
        let total = pool
            .iter()
            .fold(r(0, 1), |s, (k, a)| s + a.clone() * r(i64::from(crate::chem::weight(k)), 1));
        let mean = total / r(11, 1);
        let unit = bounce_unit(&pool, &mean);
        let b = crate::polarity::throat_held();
        for (k, l) in &unit {
            let w = r(i64::from(crate::chem::weight(k)), 1);
            assert_eq!(*l, b.clone() * mean.clone() / w + (r(1, 1) - b.clone()));
        }
    }
}
