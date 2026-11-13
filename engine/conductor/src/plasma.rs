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
    let pool = pool_spin(parts);
    parts
        .iter()
        .map(|(_, w, f, b)| b.clone() * own_spin(w, f) + (one.clone() - b.clone()) * pool.clone())
        .collect()
}

/// The ionized pool's spin `S = Σ n(1 − b)F / Σ n(1 − b)w` over the parts
/// `(n, w, F, b)`: the one spin the pool's parcel leaves with (docs/134 §6).
pub fn pool_spin(parts: &[(Ratio, Ratio, Ratio, Ratio)]) -> Ratio {
    let one = ratio(1, 1);
    let (impulse, holding) =
        parts
            .iter()
            .fold((ratio(0, 1), ratio(0, 1)), |(i, h), (n, w, f, b)| {
                let free = n.clone() * (one.clone() - b.clone());
                (i + free.clone() * f.clone(), h + free * w.clone())
            });
    if holding.is_zero() {
        ratio(0, 1)
    } else {
        impulse / holding
    }
}

/// A bound part's own spin `F/w`, by its own inertia.
pub fn own_spin(w: &Ratio, f: &Ratio) -> Ratio {
    if w.is_zero() {
        ratio(0, 1)
    } else {
        f.clone() / w.clone()
    }
}

/// The bounce's split at the throat, cold (docs/134 §6): each kind of `pool`
/// given the impulse `w̄` a nucleus; returns the pool's spin over `p`, `S`,
/// and each kind's bound share `b` at the throat's held face, heat 0. The
/// pooled parcel leaves at `p·S`, each bound part at `p·w̄/w`.
pub fn bounce_split(pool: &Manifest, mean: &Ratio) -> (Ratio, Manifest) {
    let g = crate::polarity::throat_held();
    let b: Manifest = pool
        .keys()
        .map(|k| (k.clone(), bound_share(k, &g, &ratio(0, 1))))
        .collect();
    let parts: Vec<(Ratio, Ratio, Ratio, Ratio)> = pool
        .iter()
        .map(|(k, n)| {
            (
                n.clone(),
                ratio(i64::from(crate::chem::weight(k)), 1),
                mean.clone(),
                b[k].clone(),
            )
        })
        .collect();
    (pool_spin(&parts), b)
}

/// The drift (docs/134 §8, A9, T8): each bound part `(holding m_b, carried
/// c_b)` against the pool (`m_π`, `c_π`) on the continuum keeps the velocity
/// share `k_b = δ_b = 1/(c_b + 1)` of its departure from the pool's spin a
/// bell and gives the drag share to the pool, the medium's charged share
/// (on the vacuum the faces exchange: `k = 1 − δ`, `vacuum` true).
/// `c_b' = c_π + k_b(c_b − c_π)`, `c_π' = c_π + Σ m_b(c_b − c_b')/m_π`.
/// On the grain (docs/133 §4): the keep enters the product on the grain and
/// each spin written is floored onto it; the pool takes exactly what the
/// parts gave (`Σ m·c` kept before the pool's floor), and the pool's floor's
/// remainder stays with whoever pays for the spin it leaves with (the
/// core's reservoir pays only what a parcel carries). Returns the parts'
/// carried spins and the pool's.
pub fn drift(
    m_pool: &Ratio,
    c_pool: &Ratio,
    parts: &[(Ratio, Ratio)],
    vacuum: bool,
) -> (Vec<Ratio>, Ratio) {
    let one = ratio(1, 1);
    let g = |x: &Ratio| crate::ring::grain(x);
    if m_pool.is_zero() {
        return (
            parts.iter().map(|(_, c)| c.clone()).collect(),
            c_pool.clone(),
        );
    }
    let mut given = ratio(0, 1);
    let out = parts
        .iter()
        .map(|(m, c)| {
            let delta = g(&(one.clone() / (c.clone() + one.clone())));
            let k = if vacuum { one.clone() - delta } else { delta };
            let now = g(&(c_pool.clone() + k * (c.clone() - c_pool.clone())));
            given += m.clone() * (c.clone() - now.clone());
            now
        })
        .collect();
    (out, g(&(c_pool.clone() + given / m_pool.clone())))
}

/// The passage at a bounce's release (docs/134 §7, A7, T5): each bound part
/// `(kind, amount, carried c)` passes out from the throat through the pool
/// (`pool` at carried `c_pool`). A meeting, by the founding's one law
/// (`pulse::cascade`'s filter: the lighter matter passing out CATCHES from
/// the matter it passes, which rides out with it), all read at once against
/// the release, in the bounce's own units (`whole`, what it releases).
/// - The pool spreads over `c_π(1 ± s_π)`, `s_π = ½c_π/(c_π + 1)` its
///   release share; a part reaching past `c_π(1 − s_π)` crosses the share
///   `crossed` of it, contact `½·crossed`.
/// - The grip splits as at every tier: `s` the grips' tightness ratio,
///   `D = s·m/(m + 1 − contact)`, `T` the deficits' contrast with
///   `d = 1/(c + 1)`, the gripped fraction `min(D, T)/T`.
/// - Each part takes `contact·gripped·m_b/(m_b + m_π)` of the pool's mix, on
///   the nucleus grid, every take of a kind scaled by what the pool holds
///   over what is asked.
///
/// Returns each part's take and its carried after the take joins it, the
/// holding mean on the grain (the floor's remainder stays with the core's
/// reservoir, which pays only what a parcel carries).
pub fn passage(
    pool: &Manifest,
    c_pool: &Ratio,
    parts: &[(Kind, Ratio, Ratio)],
    whole: &Ratio,
) -> Vec<(Manifest, Ratio)> {
    let one = ratio(1, 1);
    let zero = || ratio(0, 1);
    let w = |k: &Kind| ratio(i64::from(crate::chem::weight(k)), 1);
    let m_pool = crate::pulse::mass(pool);
    if m_pool.is_zero() || whole.is_zero() {
        return parts
            .iter()
            .map(|(_, _, c)| (Manifest::new(), c.clone()))
            .collect();
    }
    let s_pool = ratio(1, 2) * c_pool.clone() / (c_pool.clone() + one.clone());
    let (lo, hi) = (
        c_pool.clone() * (one.clone() - s_pool.clone()),
        c_pool.clone() * (one.clone() + s_pool),
    );
    let d = |c: &Ratio| one.clone() / (c.clone() + one.clone());
    let g_pool = crate::formation::grip(pool);
    let mj = m_pool / whole.clone();
    // Each part's share of the pool's mix, read against the release.
    let shares: Vec<Ratio> = parts
        .iter()
        .map(|(k, a, c)| {
            if *c <= lo || a.is_zero() {
                return zero();
            }
            let crossed = if *c >= hi {
                one.clone()
            } else {
                (c.clone() - lo.clone()) / (hi.clone() - lo.clone())
            };
            let contact = ratio(1, 2) * crossed;
            let part: Manifest = [(k.clone(), a.clone())].into_iter().collect();
            let gi = crate::formation::grip(&part);
            let s = if gi.is_zero() || g_pool.is_zero() {
                zero()
            } else if gi < g_pool {
                gi / g_pool.clone()
            } else {
                g_pool.clone() / gi
            };
            let mi = a.clone() * w(k) / whole.clone();
            let mij = mi.clone() + mj.clone();
            let bind = s * mij.clone() / (mij.clone() + one.clone() - contact.clone());
            let (di, dj) = (d(c), d(c_pool));
            let t = (if di > dj {
                di.clone() - dj.clone()
            } else {
                dj.clone() - di.clone()
            }) / (di + dj);
            if t.is_zero() {
                return zero();
            }
            contact * (bind.min(t.clone()) / t) * mi / mij
        })
        .collect();
    let asks: Vec<Manifest> = shares
        .iter()
        .map(|f| {
            pool.iter()
                .map(|(k, a)| {
                    (
                        k.clone(),
                        crate::pulse::nucleus(&(a.clone() * f.clone())).min(a.clone()),
                    )
                })
                .filter(|(_, a)| *a > ratio(0, 1))
                .collect()
        })
        .collect();
    let mut asked = Manifest::new();
    for t in &asks {
        for (k, a) in t {
            *asked.entry(k.clone()).or_insert_with(zero) += a.clone();
        }
    }
    parts
        .iter()
        .zip(asks)
        .map(|((k, a, c), t)| {
            let take: Manifest = t
                .into_iter()
                .map(|(kk, x)| {
                    let has = pool[&kk].clone();
                    let all = asked[&kk].clone();
                    let x = if all > has {
                        crate::pulse::nucleus(&(x * has / all))
                    } else {
                        x
                    };
                    (kk, x)
                })
                .filter(|(_, x)| *x > ratio(0, 1))
                .collect();
            let own = a.clone() * w(k);
            let got = crate::pulse::mass(&take);
            let joined = if got.is_zero() {
                c.clone()
            } else {
                crate::ring::grain(
                    &((own.clone() * c.clone() + got.clone() * c_pool.clone()) / (own + got)),
                )
            };
            (take, joined)
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
    kinds.into_iter().cloned().zip(carried(&parts)).collect()
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
            .fold(r(0, 1), |s, ((n, w, _, _), l)| {
                s + n.clone() * w.clone() * l.clone()
            });
        let given = parts
            .iter()
            .fold(r(0, 1), |s, (n, _, f, _)| s + n.clone() * f.clone());
        assert_eq!(held, given);
        assert_eq!(ell, vec![r(2069, 1460), r(1485, 2336), r(2909, 20440)]);
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
        // R8's fixtures (mycelium law row): iron's shell 2 (tightness 12)
        // stripped at heat 12 leaves its two 1s electrons, b = ½·2/26.
        assert_eq!(bound_share(&iron, &g, &r(12, 1)), r(1, 26));
        assert_eq!(bound_share(&iron, &g, &r(26, 1)), r(0, 1));
        let warm = bound_share(&iron, &g, &r(1, 1));
        assert!(
            warm > r(0, 1) && warm <= g,
            "{warm} {}",
            crate::chem::charge(&iron)
        );
        let top = cfe::almanac::atom::windings(crate::chem::charge(&iron))
            .expect("w")
            .1
            .iter()
            .map(|w| w.0.clone())
            .max()
            .expect("t");
        assert!(bound_share(&iron, &g, &top) < warm);
        assert_eq!(bound_share(&iron, &g, &top), r(0, 1));
    }

    #[test]
    fn the_bounce_at_one_bound_share_pools_at_one() {
        let mut pool = Manifest::new();
        pool.insert(crate::chem::hydrogen(), r(10, 1));
        pool.insert(crate::chem::Kind::element(26).expect("iron"), r(1, 1));
        let total = pool.iter().fold(r(0, 1), |s, (k, a)| {
            s + a.clone() * r(i64::from(crate::chem::weight(k)), 1)
        });
        let mean = total / r(11, 1);
        let unit = bounce_unit(&pool, &mean);
        let b = crate::polarity::throat_held();
        for (k, l) in &unit {
            let w = r(i64::from(crate::chem::weight(k)), 1);
            assert_eq!(*l, b.clone() * mean.clone() / w + (r(1, 1) - b.clone()));
        }
    }

    /// Witness 2 (docs/134 §6): the release split into one pooled parcel
    /// and each kind's bound part keeps every kind's nuclei and the impulse,
    /// `Σ n·w·ℓ = Σ n·F`, the pool as one parcel.
    #[test]
    fn the_split_release_keeps_the_nuclei_and_the_impulse() {
        let mut pool = Manifest::new();
        pool.insert(crate::chem::hydrogen(), r(10, 1));
        pool.insert(crate::chem::Kind::element(26).expect("iron"), r(3, 1));
        pool.insert(crate::chem::Kind::element(6).expect("carbon"), r(5, 2));
        let w = |k: &crate::chem::Kind| r(i64::from(crate::chem::weight(k)), 1);
        let total = pool.iter().fold(r(0, 1), |s, (k, a)| s + a.clone() * w(k));
        let count = pool.values().fold(r(0, 1), |s, a| s + a.clone());
        let mean = total / count.clone();
        let (spin, b) = bounce_split(&pool, &mean);
        let pooled: Manifest = pool
            .iter()
            .map(|(k, a)| (k.clone(), a.clone() * (r(1, 1) - b[k].clone())))
            .collect();
        let bound: Manifest = pool
            .iter()
            .map(|(k, a)| (k.clone(), a.clone() - pooled[k].clone()))
            .collect();
        for k in pool.keys() {
            assert_eq!(pooled[k].clone() + bound[k].clone(), pool[k]);
        }
        let carried = pooled
            .iter()
            .fold(r(0, 1), |s, (k, a)| s + a.clone() * w(k) * spin.clone())
            + bound.iter().fold(r(0, 1), |s, (k, a)| {
                s + a.clone() * w(k) * own_spin(&w(k), &mean)
            });
        assert_eq!(carried, count * mean);
    }

    /// docs/134 §7 L1: over the founded table (Z 1–1092 on S¹¹) the least
    /// tightness of any occupied winding is ½, lithium's (2,0); so the
    /// throat's heat (`a* ≤ 1/3`, T1) strips nothing.
    #[test]
    fn the_loosest_winding_is_lithiums() {
        let least = (1..=1092u32)
            .filter_map(cfe::almanac::atom::windings)
            .flat_map(|(_, ws)| ws.into_iter())
            .filter(|(_, v, _)| *v > 0)
            .map(|(t, _, _)| t)
            .min()
            .expect("windings");
        assert_eq!(least, r(1, 2));
        let three = cfe::almanac::atom::windings(3).expect("lithium").1;
        assert!(three.iter().any(|(t, v, _)| *t == r(1, 2) && *v == 1));
    }

    /// docs/134 §7 T5: a bound part passing out through the pool takes from
    /// its mix; momentum kept (`Σ m·c` of the part and its take); a part
    /// inside the pool's inner edge takes nothing; no kind is taken past what
    /// the pool holds.
    #[test]
    fn the_passage_keeps_momentum_and_takes_only_where_it_passes() {
        let h = crate::chem::hydrogen();
        let fe = crate::chem::Kind::element(26).expect("iron");
        let mut pool = Manifest::new();
        pool.insert(h.clone(), r(40, 1));
        pool.insert(fe.clone(), r(4, 1));
        let c_pool = r(1, 3);
        let parts = vec![
            (h.clone(), r(40, 1), r(5, 1)),
            (fe.clone(), r(4, 1), r(1, 100)),
        ];
        let whole = crate::pulse::mass(&pool) + r(40, 1) + r(4 * 56, 1);
        let out = passage(&pool, &c_pool, &parts, &whole);
        let (take, joined) = &out[0];
        assert!(!take.is_empty(), "hydrogen passing out takes from the pool");
        for (k, a) in take {
            assert!(*a <= pool[k]);
        }
        let got = crate::pulse::mass(take);
        let exact = (r(40, 1) * r(5, 1) + got.clone() * c_pool.clone()) / (r(40, 1) + got);
        assert_eq!(*joined, crate::ring::grain(&exact));
        assert!(
            out[1].0.is_empty(),
            "iron inside the pool's edge takes nothing"
        );
        assert_eq!(out[1].1, r(1, 100));
    }

    /// docs/134 §8 T8, T10: the drift keeps `Σ m·c` within the pool's one
    /// floor (`< m_π` grains of a grain), every spin written on the grain; a
    /// light part carried far keeps less than one of its excess over the pool.
    #[test]
    fn the_drift_keeps_momentum_and_holds_the_light_part() {
        let parts = vec![(r(1, 1), r(30, 1)), (r(56, 1), r(1, 2))];
        let (m_pool, c_pool) = (r(200, 1), crate::ring::grain(&r(1, 3)));
        let (out, pool) = drift(&m_pool, &c_pool, &parts, false);
        let before = parts
            .iter()
            .fold(m_pool.clone() * c_pool.clone(), |s, (m, c)| {
                s + m.clone() * c.clone()
            });
        let after = parts
            .iter()
            .zip(&out)
            .fold(m_pool.clone() * pool.clone(), |s, ((m, _), c)| {
                s + m.clone() * c.clone()
            });
        let floor = before - after;
        assert!(floor >= r(0, 1) && floor < m_pool.clone() * crate::ring::grain_step());
        for c in out.iter().chain(std::iter::once(&pool)) {
            assert_eq!(crate::ring::grain(c), *c, "on the grain");
        }
        assert!(
            out[0].clone() - c_pool.clone() < r(1, 1),
            "the light part is held"
        );
        let (vac, _) = drift(&m_pool, &c_pool, &parts, true);
        assert!(vac[0] > out[0], "on the vacuum the drag is velocity");
    }
}
