//! THE OCEAN: the return leg of the recycling circuit (docs/83, "Recycling
//! and recyclotronic propulsion"; with xylarium, proposed, to be tested).
//!
//! - **The crests.** The universal core regenerates its pulse at full
//!   strength (operator ruling (i)); in universe units `H_u` (`universe_hold`:
//!   its state 1 and every galactic core with its galaxy) and its bell rings once in `P_b = P(H_u) = 2` own rings, so
//!   the crests stand at `R_{p/q} = H_u(P_b·p/q − 1) = H_u(2p/q − 1)` core rings
//!   for `½ < p/q ≤ 1`. A crest couples to the kick by `a_q = 1/q` for odd q,
//!   0 for even (xylarium's ruling).
//! - **Crests form with age** (proposed): a crest of order q is in phase only
//!   after q passes of the kick, so at echo n the crests with `q ≤ n` stand.
//!   No constant bounds them; the structure grows with the universe.
//! - **The ocean on the crests** (proposed): the kick collects it by coupling,
//!   crest p/q holding `a_q / Σ a` of it; stateless, re-read every echo.
//! - **The swing** (docs/83, "The swing"): a galactic core swings about its
//!   reach R between `R(1 − e)` and `R(1 + e)`, `e = τ/(τ + h_g)`, the
//!   universal core's torsion `τ = R/(R + H_u)` (the face it twists, not the
//!   face it holds) against the galaxy's own holding `h_g`.
//! - **Gathering by crossings, not time** (xylarium (C)): every half-swing
//!   crosses each standing crest within the swing once; at each crossing
//!   the galaxy takes the in-phase share `a_q` of the medium on it, scaled by
//!   its own pull `h_g/(h_g + m)` (a pair share). Crossings are counted on
//!   whole half-swings elapsed, exact, never stepped.
use crate::pulse::{mass, Manifest};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// THE UNIVERSAL CORE'S HOLDING (operator: it must be strong enough to
/// contain every galaxy's orbit, the load distributed between the galactic
/// cores; with xylarium): one law at every tier, a core holds its state and
/// what it holds. In universe units (the founding's grains 1) its state is
/// 1, set at the bounce and never drained, and it holds every galactic core
/// with its galaxy: `H_u = 1 + Σ_g (state_g + grains_g)` (3 at a founding
/// with no ocean). Matter in the ocean is not held by a galaxy, so the hold
/// breathes with the circuit. THE MAGNETISM COUNTS TOO (operator
/// 2026-10-16; xylarium 11.0 (1), the galaxies' magnetism sets the core's
/// strength): each held nucleus bears on the universal core at its WHOLE
/// 3Z, holding plus magnetism (its unpaired frames), not its holding face
/// alone: `H_u = 1 + Σ_g (state_g + (holding_g + magnetism_g))`, read in the
/// founding's unit (the universe's grains' holding 1).
pub fn universe_hold(u: &crate::pulse::Universe) -> Ratio {
    // The founding's unit, FIXED: the galaxies' states (their grains at the
    // bounce, never drained). The current total holding rises as burning
    // transduces magnetism into holding, so it would drift the strength the
    // span (3Z, conserved) already holds (read 2026-10-17: 3.162 -> 3.060).
    let whole = u.cores.iter().fold(Ratio::from_integer(0.into()), |s, c| s + crate::pulse::core_state(c));
    if whole.is_zero() {
        return ratio(2, 1);
    }
    u.cores.iter().fold(ratio(1, 1), |s, c| {
        let all = c.total();
        s + crate::ring::grain(&((crate::pulse::core_state(c) + crate::pulse::mass(&all) + crate::pulse::magnetism(&all)) / whole.clone()))
    })
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// The coupled crests standing at echo `n`: `(reach in core rings, a_q)`.
pub fn crests(n: u64, h_u: &Ratio) -> Vec<(Ratio, Ratio)> {
    let h = h_u.clone();
    let mut out = Vec::new();
    let mut q: i64 = 1;
    while q as u64 <= n.max(1) {
        // p/q in (½, 1].
        for p in (q / 2 + 1)..=q {
            if gcd(p, q) != 1 {
                continue;
            }
            out.push((h.clone() * (ratio(2 * p, q) - ratio(1, 1)), ratio(1, q)));
        }
        q += 2;
    }
    out
}

/// A galaxy's swing about reach `r` (core rings), its holding `h_g` in
/// universe units: `(R₋, R₊)`.
pub fn swing(r: &Ratio, h_g: &Ratio, h_u: &Ratio) -> (Ratio, Ratio) {
    let h_u = h_u.clone();
    let tau = r.clone() / (r.clone() + h_u);
    if (tau.clone() + h_g.clone()).is_zero() {
        return (r.clone(), r.clone());
    }
    let e = tau.clone() / (tau + h_g.clone());
    (r.clone() * (ratio(1, 1) - e.clone()), r.clone() * (ratio(1, 1) + e))
}

/// The share of the ocean a galaxy gathers over `half_swings` half-swings at
/// echo `n`: its reach `r` (core rings), its holding `h_g` and the ocean's
/// `m` (both in universe units). Each crest within the swing is crossed
/// once a half-swing; the take from a crest never exceeds what stands there.
pub fn gathered(n: u64, r: &Ratio, h_g: &Ratio, m: &Ratio, half_swings: u64, h_u: &Ratio) -> Ratio {
    if half_swings == 0 || m.is_zero() || h_g.is_zero() {
        return ratio(0, 1);
    }
    let all = crests(n, h_u);
    let sum = all.iter().fold(ratio(0, 1), |s, (_, a)| s + a.clone());
    let (lo, hi) = swing(r, h_g, h_u);
    all.iter().filter(|(at, _)| *at >= lo && *at <= hi).fold(ratio(0, 1), |s, (_, a)| {
        let on = a.clone() / sum.clone();
        let m_q = m.clone() * on.clone();
        let each = a.clone() * h_g.clone() / (h_g.clone() + m_q);
        s + on * (each * ratio(half_swings as i64, 1)).min(ratio(1, 1))
    })
}

/// Take the share `share` of the ocean, kind by kind on the grain.
pub fn take(ocean: &Manifest, share: &Ratio) -> Manifest {
    if share.is_zero() || mass(ocean).is_zero() {
        return Manifest::new();
    }
    crate::pulse::take(ocean, &share.clone().min(ratio(1, 1)))
}

/// The relative turning at a crossing (proposed): the galaxy keeps its
/// orbit's turning, `δ` at its guiding reach `r` (its spin `ℓ = r·H` carried
/// through the swing), while the crest's ring at `at` turns at `δ(at)`; the
/// share `|δ(r) − δ(at)| / δ(r)`, about the universe's holding.
pub fn relative_turning(r: &Ratio, at: &Ratio, h_u: &Ratio) -> Ratio {
    let h = h_u.clone();
    let d = |x: &Ratio| h.clone() / (x.clone() + h.clone());
    let (dr, da) = (d(r), d(at));
    let diff = if dr > da { dr.clone() - da } else { da - dr.clone() };
    diff / dr
}

/// The swing's RADIAL share at a crossing of the crest at `at` (proposed; a
/// reading, never a trajectory): the crossing's place along the swing as a
/// share `u = (at − R₋)/(R₊ − R₋)`, read by the coherence form
/// `e·(1 − |2u − 1|)`: zero at the turnings, e at the guiding reach, never
/// more than e; no square, no root.
pub fn radial_share(r: &Ratio, h_g: &Ratio, at: &Ratio, h_u: &Ratio) -> Ratio {
    let (lo, hi) = swing(r, h_g, h_u);
    if hi <= lo {
        return ratio(0, 1);
    }
    let e = (hi.clone() - r.clone()) / r.clone();
    let u = (at.clone() - lo.clone()) / (hi - lo);
    let off = ratio(2, 1) * u - ratio(1, 1);
    let off = if off < ratio(0, 1) { -off } else { off };
    e * (ratio(1, 1) - off).max(ratio(0, 1))
}

/// The crossings' turning in the two planes (the bivector, never reduced to
/// a magnitude), each weighted by what its crest gave: (the orbit's plane,
/// tangential; the swing's plane, radial).
pub fn arrival_turning(n: u64, r: &Ratio, h_g: &Ratio, m: &Ratio, h_u: &Ratio) -> (Ratio, Ratio) {
    let all = crests(n, h_u);
    let sum = all.iter().fold(ratio(0, 1), |s, (_, a)| s + a.clone());
    let (lo, hi) = swing(r, h_g, h_u);
    let (mut w, mut t, mut rad) = (ratio(0, 1), ratio(0, 1), ratio(0, 1));
    for (at, a) in all.iter().filter(|(at, _)| *at >= lo && *at <= hi) {
        let m_q = m.clone() * a.clone() / sum.clone();
        let each = a.clone() * h_g.clone() / (h_g.clone() + m_q) * a.clone() / sum.clone();
        t += each.clone() * relative_turning(r, at, h_u);
        rad += each.clone() * radial_share(r, h_g, at, h_u);
        w += each;
    }
    if w.is_zero() { (ratio(0, 1), ratio(0, 1)) } else { (t / w.clone(), rad / w) }
}

/// WHERE ARRIVING MATTER LANDS (xylarium, proposed: CARRIED SPINS ADD,
/// PLANE BY PLANE). The bounce's push is the isoclinic harmonic pulse
/// (xylarium's operator), so it acts in every plane at once; arriving
/// matter also brings the crossing's turning in two planes, so kind z
/// carries in each plane `ℓ = p·(w̄/w_z) + ρ_g·t_plane` (t the crossing's
/// share in that plane, read in the galaxy's own units through its received
/// share ρ_g). Each plane's cut is the founding's: about `R = ℓ/H`, spread
/// over `R(1 ± p)`, its held share the part within the throat; a kind is
/// HELD only as held in both planes, `f_orbit · f_swing` (a product of
/// distinct shares). The rest is GAS at the orbit plane's reach, agitation
/// `(p − δ)/p`, cooling by both channels from bell `born`. Returns (held,
/// free, the free part's motes); `galaxy` is the galaxy's grains with the
/// arrivals already counted.
pub fn land(galaxy: &Manifest, arriving: &Manifest, rho: &Ratio, turning: &(Ratio, Ratio), born: u64) -> (Manifest, Manifest, Vec<crate::formation::Shell>) {
    let weight = |k: &crate::chem::Kind| crate::chem::hold(k);
    let (mut held, mut free, mut motes) = (Manifest::new(), Manifest::new(), Vec::new());
    let total = mass(galaxy);
    let Some((p, h, mean)) = crate::pulse::galaxy_push(galaxy) else { return (held, arriving.clone(), motes) };
    for (k, amount) in arriving {
        let w = ratio(i64::from(weight(k)), 1);
        let push = p.clone() * mean.clone() / w.clone();
        let held_share = |t: &Ratio| {
            let r = (push.clone() + rho.clone() * t.clone()) / h.clone();
            let lo = r.clone() * (ratio(1, 1) - p.clone());
            let span = ratio(2, 1) * p.clone() * r.clone();
            if span.is_zero() { if lo <= h { ratio(1, 1) } else { ratio(0, 1) } } else { ((h.clone() - lo) / span).max(ratio(0, 1)).min(ratio(1, 1)) }
        };
        let f = held_share(&turning.0) * held_share(&turning.1);
        // The gas stands where the plane carrying it farthest leaves it.
        let r = (push.clone() + rho.clone() * turning.0.clone().max(turning.1.clone())) / h.clone();
        let kept = crate::ring::grain(&(amount.clone() * f));
        let loose = amount.clone() - kept.clone();
        if kept > ratio(0, 1) {
            held.insert(k.clone(), kept);
        }
        if loose > ratio(0, 1) {
            let inner = (r.clone() * (ratio(1, 1) - p.clone())).max(h.clone());
            let outer = r.clone() * (ratio(1, 1) + p.clone());
            let delta = h.clone() / (r.clone() + h.clone());
            let agitation = if delta < p { (p.clone() - delta) / p.clone() } else { ratio(0, 1) };
            if outer > inner && !total.is_zero() {
                motes.push(crate::formation::Shell { key: k.clone(), held: loose.clone() * w / total.clone(), inner, outer: outer.clone(), agitation, radiates: crate::formation::radiates(k), born, vacuum: crate::ring::deficit(&outer, &h) });
            }
            free.insert(k.clone(), loose);
        }
    }
    (held, free, motes)
}

/// THE PUMP (convection, with xylarium; docs/83): the universal tide a
/// swinging galaxy feels differs between its turnings,
/// `π = δ_u(R₋) − δ_u(R₊)`, zero for a circle. The outside tide enters the
/// galaxy as one received quantity, so it varies only in time, with the
/// swing.
pub fn pump(r: &Ratio, h_g: &Ratio, h_u: &Ratio) -> Ratio {
    let h = h_u.clone();
    let (lo, hi) = swing(r, h_g, h_u);
    let d = |x: &Ratio| h.clone() / (x.clone() + h.clone());
    d(&lo) - d(&hi)
}
