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

/// IGNITION, THE QUANTUM HEART (operator 2026-10-15; with xylarium): a
/// star is the core's pair one tier down, a containment ring at its centre
/// (its heart) holding its body's grains as one ring of its own, at reach =
/// hold. A collapsing clump runs away (its deficit `m/(w + m)` rising as its
/// width falls) to the throat condition `w = m`, deficit ½, where the heart
/// forms. There `λ = 1`, and the largest part that holds about the heart
/// (`½ ≥ a + x/(x + m)`) is `x* = m (½ − a)/(½ + a)`: the star holds the
/// share `(1 − 2a)/(1 + 2a)` of its clump, a the agitation at ignition.
/// None at `a ≥ ½`: light pushes back before the heart forms.
pub fn ignition_share(agitation: &Ratio) -> Option<Ratio> {
    let half = ratio(1, 2);
    (*agitation < half).then(|| (ratio(1, 1) - ratio(2, 1) * agitation.clone()) / (ratio(1, 1) + ratio(2, 1) * agitation.clone()))
}

/// THE COUPLING, DERIVED at a star's interface (operator 2026-10-15): its
/// heart's hold against its atoms' charge hold. Gravity is the star's share
/// of the universe's holding on the core's clock (`star_hold / universe_hold`,
/// the weights `atom::hold`); charge is each nucleus's outer band's enclosed
/// charge, counted over the star's nuclei (`nuclei_per_grain` a grain). So
/// `κ = (star_hold / universe_hold) / Σ nuclei·Q_out`: gravity is counted as
/// a share of the whole, charge per nucleus, and the universe's nucleus
/// count survives in their ratio (the scale no holding-only law carries).
/// Exact, from the manifest and the atom tier; no constant drawn.
pub fn coupling(star: &Manifest, universe_hold: &Ratio) -> Option<Ratio> {
    let per_grain = Ratio::from_integer(crate::pulse::nuclei_per_grain() * num_bigint::BigInt::from(1024));
    let charge = star.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * per_grain.clone() * ratio(i64::from(z_of(k).and_then(atom).map(|x| x.enclosed).unwrap_or(0)), 1));
    if charge.is_zero() || universe_hold.is_zero() {
        return None;
    }
    Some(mass(star) / universe_hold.clone() / charge)
}

/// THE HEARTBEAT (with xylarium, 2026-10-16): a star burns at its
/// INTERFACE, where its heart's gravity matches its atoms' charge: the share
/// `κ · n_star = s_star / Q̄` of its fuel (s_star its share of the universe's
/// holding, Q̄ its nuclei's mean outer charge). Each received ring feeds the
/// fuel's open share e of that interface: held as HALVINGS, the fuel halves
/// every `⌈Q̄ / (e · s_star)⌉` received rings. Each fusion closes Δu open
/// frames, transducing magnetism into holding, its light the transduction's
/// flux. The first rung here: hydrogen to helium (Δu = 2, e = ½).
/// Returns the rings per halving, the fuel's share left after `rings`
/// received rings, and the light per received ring (`feed × fuel × Δu`).
pub fn heartbeat(star: &Manifest, universe_hold: &Ratio, rings: u64) -> Option<(u64, Ratio, Ratio)> {
    let fuel = star.get("Z001")?.clone();
    let total = mass(star);
    if total.is_zero() || universe_hold.is_zero() {
        return None;
    }
    let count = star.values().fold(ratio(0, 1), |s, a| s + a.clone());
    let charge = star.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * ratio(i64::from(z_of(k).and_then(atom).map(|x| x.enclosed).unwrap_or(0)), 1));
    let q_mean = charge / count;
    let share = total.clone() / universe_hold.clone();
    let e = ratio(1, 2);
    if q_mean.is_zero() {
        return None;
    }
    // The interface feed per received ring, `e · s_star / Q̄`.
    let feed = e * share / q_mean;
    let every: u64 = (ratio(1, 1) / feed.clone()).ceil().to_integer().try_into().unwrap_or(u64::MAX).max(1);
    let halvings = (rings / every).min(4096) as u32;
    let left = ratio(1, 1) / Ratio::from_integer(num_bigint::BigInt::from(2).pow(halvings));
    let light = feed * fuel / total * left.clone() * ratio(2, 1);
    Some((every, left, light))
}

/// A STAR'S SYSTEM AT IGNITION (docs/83; planets from the star's own disc):
/// in the clump's own units (holding 1, its ignition extent w = m = 1), the
/// heart holds the star, the share `x* = (½ − a)/(½ + a)` within reach x*;
/// what lies between the heart's reach and the clump's edge, `[x*, 1]`, is
/// the star's DISC (the clump's own mix, `λ = 1`), anchored on the star
/// (H = x*). It fragments by the same collapse test (`collapse`): as many
/// worlds of holding `w*` as fit, standing at their places in the annulus;
/// what does not fragment stays as the system's medium (its belts). Read in
/// the transition units by the clump's holding; orbits by the motion law.
pub fn system(clump: &Manifest, agitation: &Ratio, path: &[u32], unit: &Ratio) -> Option<(Manifest, Vec<crate::pulse::Body>, Manifest, Ratio)> {
    let x = ignition_share(agitation)?;
    let star = crate::pulse::take(clump, &x);
    let mut disc = clump.clone();
    crate::pulse::less(&mut disc, &star);
    let scale = mass(clump) / unit.clone();
    let star_hold = mass(&star) / unit.clone();
    let annulus = Stretch { inner: x.clone(), outer: ratio(1, 1), lambda: ratio(1, 1), agitation: agitation.clone(), members: Vec::new() };
    let mut worlds = Vec::new();
    let mut inside = star_hold.clone();
    if let Some((w, count)) = collapse(&annulus, &x) {
        let width = ratio(1, 1) - x.clone();
        let share = w.clone() / width;
        for i in 0..count {
            let left_share = ratio(1, 1) - share.clone() * ratio(i as i64, 1);
            if left_share <= ratio(0, 1) {
                break;
            }
            let part = crate::pulse::take(&disc, &(share.clone() / left_share));
            if part.is_empty() {
                continue;
            }
            crate::pulse::less(&mut disc, &part);
            let reach = (x.clone() + w.clone() * (ratio(i as i64, 1) + ratio(1, 2))) * scale.clone();
            let mut at = path.to_vec();
            at.push(i as u32 + 1);
            let period = crate::ring::period(&reach, &inside);
            let ell = crate::ring::spin(&reach, &inside);
            inside += mass(&part) / unit.clone();
            worlds.push(crate::pulse::Body { path: at, radius: reach, period, kept: part, satellites: Vec::new(), medium: Manifest::new(), ell, medium_spin: ratio(0, 1), gas: Vec::new(), born: 0 });
        }
    }
    let mut belts_spin = mass(&disc) / unit.clone() * crate::ring::spin(&scale, &inside);
    belts_spin += dance(&mut worlds, &star_hold, unit);
    Some((star, worlds, disc, belts_spin))
}

/// The convergents `p/q` of a positive rational, in order.
fn convergents(x: &Ratio) -> Vec<Ratio> {
    let mut out = Vec::new();
    let (mut h0, mut h1) = (num_bigint::BigInt::from(0), num_bigint::BigInt::from(1));
    let (mut k0, mut k1) = (num_bigint::BigInt::from(1), num_bigint::BigInt::from(0));
    let mut rest = x.clone();
    for _ in 0..64 {
        let a = rest.floor().to_integer();
        let (h2, k2) = (&a * &h1 + &h0, &a * &k1 + &k0);
        out.push(Ratio::new(h2.clone(), k2.clone()));
        (h0, h1, k0, k1) = (h1, h2, k1, k2);
        let frac = rest.clone() - rest.floor();
        if frac.is_zero() {
            break;
        }
        rest = ratio(1, 1) / frac;
    }
    out
}

/// THE DANCE (with xylarium, 2026-10-16): a system's worlds lock into
/// resonance with its DOMINANT world (the largest holding), which sets the
/// ladder. World k reads its period ratio to the dominant, `P_k / P_dom`
/// (exact: `P = 1 + R/H`), and LOCKS to the convergent `p/q` of that ratio
/// with the smallest q whose distance from it is within the dominant world's
/// deficit at k's separation, `δ = H_dom / (|R_k − R_dom| + H_dom)` (a heavy
/// dominant captures wide, a light one only near-exact ratios; no integer is
/// chosen and no constant set). The lock moves `R_k` to the convergent's
/// exact period about the holding inside it; the spin it gives up or takes
/// is exchanged with the system's belts, so `Σ m·ℓ` is conserved. A world no
/// convergent reaches stays free. Returns the spin the belts gain.
pub fn dance(worlds: &mut [crate::pulse::Body], star_hold: &Ratio, unit: &Ratio) -> Ratio {
    let Some(dom) = (0..worlds.len()).max_by(|&a, &b| mass(&worlds[a].kept).cmp(&mass(&worlds[b].kept))) else { return ratio(0, 1) };
    let (p_dom, r_dom, h_dom) = (worlds[dom].period.clone(), worlds[dom].radius.clone(), mass(&worlds[dom].kept) / unit.clone());
    let mut exchanged = ratio(0, 1);
    let mut inside = star_hold.clone();
    for k in 0..worlds.len() {
        let m_k = mass(&worlds[k].kept) / unit.clone();
        if k != dom && !p_dom.is_zero() && !inside.is_zero() {
            let x = worlds[k].period.clone() / p_dom.clone();
            let gap = if worlds[k].radius > r_dom { worlds[k].radius.clone() - r_dom.clone() } else { r_dom.clone() - worlds[k].radius.clone() };
            let width = h_dom.clone() / (gap + h_dom.clone());
            let lock = convergents(&x).into_iter().find(|c| {
                let d = if *c > x { c.clone() - x.clone() } else { x.clone() - c.clone() };
                d <= width
            });
            if let Some(c) = lock {
                let period = p_dom.clone() * c;
                if period > ratio(1, 1) {
                    let reach = inside.clone() * (period.clone() - ratio(1, 1));
                    let ell = crate::ring::spin(&reach, &inside);
                    exchanged += m_k.clone() * (worlds[k].ell.clone() - ell.clone());
                    worlds[k].radius = reach;
                    worlds[k].period = period;
                    worlds[k].ell = ell;
                }
            }
        }
        inside += m_k;
    }
    exchanged
}

/// The interface clock of a centre and the matter it feeds on (one law at
/// every tier: a star's heartbeat, a galactic core's jets): rings a halving,
/// `⌈Q̄ / (e · s)⌉`, s the centre's share of the universe's holding, Q̄ the
/// matter's mean outer charge, e its open share (open frames over held).
pub fn interface_every(matter: &Manifest, share: &Ratio) -> u64 {
    let count = matter.values().fold(ratio(0, 1), |s, a| s + a.clone());
    let charge = matter.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * ratio(i64::from(z_of(k).and_then(atom).map(|x| x.enclosed).unwrap_or(0)), 1));
    let open = crate::pulse::magnetism(matter);
    let held = mass(matter);
    if count.is_zero() || charge.is_zero() || open.is_zero() || held.is_zero() || share.is_zero() {
        return u64::MAX;
    }
    let feed = open / held * share.clone() / (charge / count);
    (ratio(1, 1) / feed).ceil().to_integer().try_into().unwrap_or(u64::MAX).max(1)
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
