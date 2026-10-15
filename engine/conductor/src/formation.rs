//! docs/83 FORMATION, the chain to stars (with xylarium, 2026-10-15): the
//! disc's cut leaves a galaxy's light elements as GAS; the gas cools by the
//! light it cannot hold; a stretch of gas COLLAPSES into stars where its own
//! deficit beats its agitation and its shear. Every reading is a share or a
//! product of distinct sums; every moment is one crossing, never stepped.
//!
//! In the galaxy's own units its core's state holds 1 and its grains
//! together 1 (`pulse::core_state`); H, the holding inside the gas, is the
//! core's state plus its disc, `1 + H_disc`.
//!
//! - **Shells.** Each escaped element z is carried the spin
//!   `ℓ_z = p·(w̄/w_z)/s_g` (`pulse::disc_cut`) and stands at
//!   `R_z = ℓ_z / H`, where it re-locks only the deficit
//!   `δ_z = H / (R_z + H) < p`; the rest of the push is its AGITATION,
//!   `a_z = (p − δ_z) / p` (absorbed less re-locked, over absorbed).
//!   Agitation spreads its spin `ℓ_z (1 ± a_z)`, so the shell stands over
//!   `R_z (1 − a_z) … R_z (1 + a_z)` (a box: the first build; the sphere's
//!   measure is the refinement).
//! - **Cooling.** What a grain cannot hold it radiates: its open frames over
//!   its held frames, `e_z = unpaired / hold` a ring; held as HALVINGS, its
//!   agitation halves every `h_z = ⌈1 / e_z⌉` rings (closed-shell atoms,
//!   with no open frame, never cool by themselves).
//! - **Collapse.** Overlapping shells sum to a stretch of linear holding λ.
//!   A clump of width w in it collapses where
//!   `λ (K)(K + w) ≥ (1 + λ)[a (K)(K + w) + H w]`, `K = R + H` at its near
//!   side: its own deficit `λ / (1 + λ)` against its agitation and its own
//!   shear `σ = H w / ((R + H)(R + w + H))`. σ rises with w, so the largest
//!   clump that holds is ONE crossing w*, and the stretch forms as many stars
//!   of holding `λ · w*` as fit.
use crate::pulse::{mass, Manifest};
use cfe::almanac::atom::{atom, hold};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// A shell of one escaped element, in the galaxy's own units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shell {
    pub key: String,
    /// Its holding, a share of the galaxy's grains.
    pub held: Ratio,
    pub inner: Ratio,
    pub outer: Ratio,
    /// Its agitation at the cut.
    pub agitation: Ratio,
    /// Rings a halving of its agitation; None: it never cools by itself.
    pub halving: Option<u64>,
    /// The ring it was laid down (the cut: 0; a jet's landing: later); it
    /// cools from then.
    pub born: u64,
}

fn z_of(key: &str) -> Option<u32> {
    key.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok())
}

/// The gas's shells and the disc's cold rings: the galaxy's grains `all`,
/// its core's disc `disc`, its share of the universe's holding `share`.
pub fn shells(all: &Manifest, disc: &Manifest, share: &Ratio) -> (Vec<Shell>, Ratio) {
    let weight = |k: &str| z_of(k).map(hold).unwrap_or(1);
    let total = mass(all);
    let count = all.values().fold(ratio(0, 1), |s, a| s + a.clone());
    let h_disc = mass(disc) / total.clone();
    let h = ratio(1, 1) + h_disc.clone();
    let push = h_disc.clone() / h.clone();
    if total.is_zero() || share.is_zero() || push.is_zero() {
        return (Vec::new(), h);
    }
    let mean = total.clone() / count;
    let mut out = Vec::new();
    for (k, amount) in all {
        let w = ratio(i64::from(weight(k)), 1);
        let ell = push.clone() * mean.clone() / w.clone() / share.clone();
        let r = ell / h.clone();
        let delta = h.clone() / (r.clone() + h.clone());
        // A held element (the disc's) re-locks all its push: a cold RING at
        // its reach, no width. An escaped one keeps its agitation.
        let agitation = if disc.contains_key(k) || delta >= push { ratio(0, 1) } else { (push.clone() - delta) / push.clone() };
        let halving = z_of(k).and_then(atom).and_then(|a| (a.unpaired > 0).then(|| u64::from((hold(a.z) + a.unpaired - 1) / a.unpaired)));
        out.push(Shell {
            key: k.clone(),
            held: amount.clone() * w / total.clone(),
            inner: r.clone() * (ratio(1, 1) - agitation.clone()),
            outer: r * (ratio(1, 1) + agitation.clone()),
            agitation,
            halving,
            born: 0,
        });
    }
    (out, h)
}

/// A shell's agitation at ring `n`: `a₀ / 2^⌊(n − born) / h⌋`; nothing
/// before it is laid down.
fn cooled(s: &Shell, n: u64) -> Ratio {
    match s.halving {
        Some(h) => {
            let halvings = (n.saturating_sub(s.born) / h).min(4096) as u32;
            s.agitation.clone() / Ratio::from_integer(num_bigint::BigInt::from(2).pow(halvings))
        }
        None => s.agitation.clone(),
    }
}

/// A stretch of gas between two shell edges, its linear holding and the
/// holding-weighted agitation of what overlaps it.
#[derive(Debug, Clone)]
pub struct Stretch {
    pub inner: Ratio,
    pub outer: Ratio,
    pub lambda: Ratio,
    pub agitation: Ratio,
    /// The members' holding shares of this stretch: (shell index, holding).
    pub members: Vec<(usize, Ratio)>,
}

/// The gas profile at ring `n`: piecewise between every shell edge.
pub fn stretches(shells: &[Shell], n: u64) -> Vec<Stretch> {
    // The disc's cold rings lie at single reaches; they join whichever
    // stretch of gas passes over them (the disc's outer edge mixing with the
    // gas), and add no edge of their own.
    let mut edges: Vec<Ratio> = shells.iter().filter(|s| s.outer > s.inner).flat_map(|s| [s.inner.clone(), s.outer.clone()]).collect();
    edges.sort();
    edges.dedup();
    let mut out = Vec::new();
    for pair in edges.windows(2) {
        let (lo, hi) = (&pair[0], &pair[1]);
        let width = hi.clone() - lo.clone();
        if width.is_zero() {
            continue;
        }
        let mut members = Vec::new();
        let (mut lambda, mut weighted) = (ratio(0, 1), ratio(0, 1));
        for (i, s) in shells.iter().enumerate() {
            let span = s.outer.clone() - s.inner.clone();
            if span.is_zero() {
                if s.inner >= *lo && s.inner < *hi && s.born <= n {
                    members.push((i, s.held.clone()));
                    lambda += s.held.clone() / width.clone();
                }
                continue;
            }
            if s.inner > *lo || s.outer < *hi || s.born > n {
                continue;
            }
            let density = s.held.clone() / span;
            members.push((i, density.clone() * width.clone()));
            weighted += density.clone() * cooled(s, n);
            lambda += density;
        }
        if lambda.is_zero() {
            continue;
        }
        let agitation = weighted / lambda.clone();
        out.push(Stretch { inner: lo.clone(), outer: hi.clone(), lambda, agitation, members });
    }
    out
}

/// Does a clump of width `w` at the stretch's near side collapse?
fn holds(st: &Stretch, h: &Ratio, w: &Ratio) -> bool {
    let k = st.inner.clone() + h.clone();
    let kw = k.clone() * (k + w.clone());
    st.lambda.clone() * kw.clone() >= (ratio(1, 1) + st.lambda.clone()) * (st.agitation.clone() * kw + h.clone() * w.clone())
}

/// THE COLLAPSE: the largest clump width that holds, as one crossing found
/// by bisection on the comparison (None: no clump holds, the gas stays hot),
/// and the stars that fit.
pub fn collapse(st: &Stretch, h: &Ratio) -> Option<(Ratio, u64)> {
    let width = st.outer.clone() - st.inner.clone();
    let tiny = width.clone() / ratio(1 << 20, 1);
    if !holds(st, h, &tiny) {
        return None;
    }
    if holds(st, h, &width) {
        return Some((width, 1));
    }
    let (mut lo, mut hi) = (tiny, width.clone());
    for _ in 0..24 {
        let mid = (lo.clone() + hi.clone()) / ratio(2, 1);
        if holds(st, h, &mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let count = (width / lo.clone()).floor().to_integer().try_into().unwrap_or(u64::MAX);
    Some((lo, count))
}

/// THE CORE'S AXIAL JETS (with xylarium, 2026-10-15): the disc feeds its
/// INNERMOST matter (heaviest first, what falls in) into jets along the
/// dipole's axis, out of the disc's plane, so the in-plane hold does not
/// bind them. The disc's jetted share halves every `⌈1/p⌉` rings of the
/// core's echo (p the push share; halvings, never expanded), and each batch
/// LANDS at the galaxy's RIM, where the core's pulse refocuses, as hot gas
/// with the agitation of the push it carried, `(p − δ_rim)/p`, cooling from
/// its landing by its own open frames. `rings` are the disc's (held,
/// zero-width shells), up to ring `horizon`.
pub fn jets(rings: &[Shell], h: &Ratio, horizon: u64) -> Vec<Shell> {
    let mut disc: Vec<&Shell> = rings.iter().filter(|s| s.outer == s.inner).collect();
    // Innermost (heaviest) first.
    disc.sort_by(|a, b| a.inner.cmp(&b.inner));
    let total = disc.iter().fold(ratio(0, 1), |s, r| s + r.held.clone());
    let push = (h.clone() - ratio(1, 1)) / h.clone();
    if total.is_zero() || push.is_zero() {
        return Vec::new();
    }
    let every: u64 = (ratio(1, 1) / push.clone()).ceil().to_integer().try_into().unwrap_or(u64::MAX).max(1);
    let rim = rings.iter().map(|s| s.outer.clone()).max().unwrap_or_else(|| ratio(1, 1));
    let delta = h.clone() / (rim.clone() + h.clone());
    let agitation = if delta < push { (push.clone() - delta) / push.clone() } else { ratio(0, 1) };
    let mut out = Vec::new();
    // Batch k (landing at k·every) carries the disc's share 2^−k, taken
    // from the cumulative disc, heaviest first.
    let (mut taken, mut left) = (ratio(0, 1), total.clone());
    let mut k = 1u64;
    while k * every <= horizon && k <= 64 {
        let batch = left.clone() / ratio(2, 1);
        left -= batch.clone();
        let (from, to) = (taken.clone(), taken.clone() + batch);
        taken = to.clone();
        let mut at = ratio(0, 1);
        for ring in &disc {
            let (lo, hi) = (at.clone(), at.clone() + ring.held.clone());
            at = hi.clone();
            let part = hi.min(to.clone()) - lo.max(from.clone());
            if part <= ratio(0, 1) {
                continue;
            }
            out.push(Shell {
                key: ring.key.clone(),
                held: part,
                inner: rim.clone() * (ratio(1, 1) - agitation.clone()),
                outer: rim.clone() * (ratio(1, 1) + agitation.clone()),
                agitation: agitation.clone(),
                halving: ring.halving,
                born: k * every,
            });
        }
        k += 1;
    }
    out
}

/// Rings between jet batches: `⌈1/p⌉`, p the push share `(H − 1)/H`.
pub fn jet_every(h: &Ratio) -> u64 {
    let push = (h.clone() - ratio(1, 1)) / h.clone();
    if push <= ratio(0, 1) {
        return u64::MAX;
    }
    (ratio(1, 1) / push).ceil().to_integer().try_into().unwrap_or(u64::MAX).max(1)
}

/// ONE JET BATCH, landing at ring `born`: half of the disc's holding, taken
/// innermost (heaviest) first from its rings, landing hot at the rim.
pub fn jet_batch(rings: &[Shell], h: &Ratio, rim: &Ratio, born: u64) -> Vec<Shell> {
    let mut disc: Vec<&Shell> = rings.iter().filter(|s| s.outer == s.inner).collect();
    disc.sort_by(|a, b| a.inner.cmp(&b.inner));
    let total = disc.iter().fold(ratio(0, 1), |s, r| s + r.held.clone());
    let push = (h.clone() - ratio(1, 1)) / h.clone();
    if total.is_zero() || push <= ratio(0, 1) {
        return Vec::new();
    }
    let delta = h.clone() / (rim.clone() + h.clone());
    let agitation = if delta < push { (push.clone() - delta) / push.clone() } else { ratio(0, 1) };
    let mut want = total / ratio(2, 1);
    let mut out = Vec::new();
    for ring in disc {
        if want <= ratio(0, 1) {
            break;
        }
        let part = ring.held.clone().min(want.clone());
        want -= part.clone();
        out.push(Shell {
            key: ring.key.clone(),
            held: part,
            inner: rim.clone() * (ratio(1, 1) - agitation.clone()),
            outer: rim.clone() * (ratio(1, 1) + agitation.clone()),
            agitation: agitation.clone(),
            halving: ring.halving,
            born,
        });
    }
    out
}

/// The first ring at which a stretch can collapse (its members cooled
/// enough), searched on the halving schedule up to `horizon` rings.
pub fn first_collapse(shells: &[Shell], h: &Ratio, horizon: u64) -> Vec<(u64, Stretch, Ratio, u64)> {
    let mut steps: Vec<u64> = shells.iter().filter_map(|s| s.halving.map(|hv| (s.born, hv))).flat_map(|(b, hv)| (0..=64u64).map(move |k| b + k * hv)).filter(|&n| n <= horizon).collect();
    steps.extend(shells.iter().map(|s| s.born).filter(|&n| n <= horizon));
    steps.push(0);
    steps.sort();
    steps.dedup();
    let mut found: Vec<(u64, Stretch, Ratio, u64)> = Vec::new();
    let mut seen: Vec<(Ratio, Ratio)> = Vec::new();
    for n in steps {
        for st in stretches(shells, n) {
            if seen.iter().any(|(a, b)| *a == st.inner && *b == st.outer) {
                continue;
            }
            if let Some((w, count)) = collapse(&st, h) {
                seen.push((st.inner.clone(), st.outer.clone()));
                found.push((n, st, w, count));
            }
        }
    }
    found
}
