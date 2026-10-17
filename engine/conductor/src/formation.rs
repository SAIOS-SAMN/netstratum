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
//!   agitation halves every `h_z = ⌈1 / e_z⌉` rings; and the VACUUM takes
//!   back the deficit's share each ring, a halving every `⌈1/δ⌉` rings at its
//!   reach (closed-shell gas cools this way alone; close-in gas fastest).
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
    /// Rings a halving of its agitation through the VACUUM (operator
    /// 2026-10-16, with xylarium): each ring the vacuum takes back the
    /// deficit's share δ = H/(R + H) at its reach, so it halves at least
    /// every ⌈1/δ⌉ rings ((1 − δ)^⌈1/δ⌉ ≤ 1/e < ½; exact, no power). Every
    /// parcel cools this way, closed-shell ones included. 0: no vacuum.
    pub vacuum: u64,
}

/// The vacuum's halving at reach `r` about holding `h`: `⌈(r + h)/h⌉`.
pub fn vacuum_halving(r: &Ratio, h: &Ratio) -> u64 {
    if h.is_zero() {
        return 0;
    }
    ((r.clone() + h.clone()) / h.clone()).ceil().to_integer().try_into().unwrap_or(0)
}

fn z_of(key: &str) -> Option<u32> {
    key.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok())
}

/// The disc's cold bands and the gas's shells: the galaxy's grains `all`,
/// its core's disc `disc`, in its own units (regeneration). Returns (held,
/// free, H).
pub fn shells(all: &Manifest, disc: &Manifest) -> (Vec<Shell>, Vec<Shell>, Ratio) {
    let weight = |k: &str| z_of(k).map(hold).unwrap_or(1);
    let total = mass(all);
    let Some((push, h, mean)) = crate::pulse::galaxy_push(all) else { return (Vec::new(), Vec::new(), ratio(1, 1)) };
    let (mut held_out, mut out) = (Vec::new(), Vec::new());
    for (k, amount) in all {
        let w = ratio(i64::from(weight(k)), 1);
        let ell = push.clone() * mean.clone() / w.clone();
        let r = ell / h.clone();
        let delta = h.clone() / (r.clone() + h.clone());
        let halving = z_of(k).and_then(atom).and_then(|a| (a.unpaired > 0).then(|| u64::from((hold(a.z) + a.unpaired - 1) / a.unpaired)));
        // THE DISPERSAL (`pulse::disperse`; operator: "disc and gas both
        // mixed"): the held part of the kind is the part of its spread
        // within the throat, a COLD BAND `[R(1 − p), min(R(1 + p), H)]` (the
        // disc); the rest spreads as gas beyond the throat, its agitation the
        // push it absorbed and could not re-lock there.
        let held_part = disc.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
        if held_part > ratio(0, 1) {
            let lo = r.clone() * (ratio(1, 1) - push.clone());
            let hi = (r.clone() * (ratio(1, 1) + push.clone())).min(h.clone()).max(lo.clone());
            held_out.push(Shell { key: k.clone(), held: held_part.clone() * w.clone() / total.clone(), inner: lo, outer: hi.clone(), agitation: ratio(0, 1), halving, born: 0, vacuum: vacuum_halving(&hi, &h) });
        }
        let free = amount.clone() - held_part;
        let inner = (r.clone() * (ratio(1, 1) - push.clone())).max(h.clone());
        let outer = r.clone() * (ratio(1, 1) + push.clone());
        if free > ratio(0, 1) && outer > inner {
            let agitation = if delta < push { (push.clone() - delta) / push.clone() } else { ratio(0, 1) };
            out.push(Shell { key: k.clone(), held: free * w / total.clone(), inner, outer: outer.clone(), agitation, halving, born: 0, vacuum: vacuum_halving(&outer, &h) });
        }
    }
    (held_out, out, h)
}

/// A shell's agitation at ring `n`: halved through both channels, its open
/// frames' light (`h_e`) and the vacuum's pull back (`h_v`),
/// `a₀ / 2^(⌊age/h_e⌋ + ⌊age/h_v⌋)`; nothing before it is laid down.
fn cooled(s: &Shell, n: u64) -> Ratio {
    let age = n.saturating_sub(s.born);
    let by_light = s.halving.map(|h| age / h).unwrap_or(0);
    let by_vacuum = if s.vacuum > 0 { age / s.vacuum } else { 0 };
    let halvings = (by_light + by_vacuum).min(4096) as u32;
    s.agitation.clone() / Ratio::from_integer(num_bigint::BigInt::from(2).pow(halvings))
}

/// A stretch of gas between two shell edges, its linear holding and the
/// holding-weighted agitation of what overlaps it.
#[derive(Debug, Clone)]
pub struct Stretch {
    pub inner: Ratio,
    pub outer: Ratio,
    pub lambda: Ratio,
    /// Its MAGNETISM per unit reach (`μ = Σ λ_z·e_z`, e_z a kind's open
    /// share, unpaired over hold): the open frames that attract each other in
    /// free gas, opposite spins pairing (operator: "full spin magnetism
    /// attractors in the gases"; the antipole of the heart's push on them).
    pub mu: Ratio,
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
        let (mut lambda, mut weighted, mut mu) = (ratio(0, 1), ratio(0, 1), ratio(0, 1));
        let open = |k: &str| z_of(k).and_then(atom).map(|a| ratio(i64::from(a.unpaired), i64::from(hold(a.z)))).unwrap_or_else(|| ratio(0, 1));
        for (i, s) in shells.iter().enumerate() {
            let span = s.outer.clone() - s.inner.clone();
            if span.is_zero() {
                if s.inner >= *lo && s.inner < *hi && s.born <= n {
                    members.push((i, s.held.clone()));
                    lambda += s.held.clone() / width.clone();
                    mu += s.held.clone() / width.clone() * open(&s.key);
                }
                continue;
            }
            if s.inner > *lo || s.outer < *hi || s.born > n {
                continue;
            }
            let density = s.held.clone() / span;
            members.push((i, density.clone() * width.clone()));
            weighted += density.clone() * cooled(s, n);
            mu += density.clone() * open(&s.key);
            lambda += density;
        }
        if lambda.is_zero() {
            continue;
        }
        let agitation = weighted / lambda.clone();
        out.push(Stretch { inner: lo.clone(), outer: hi.clone(), lambda, mu, agitation, members });
    }
    out
}

/// Does a clump of width `w` at the stretch's near side collapse? (with
/// xylarium, 2026-10-16) The CLUMP IS ITS OWN ENCLOSURE: of width w it holds
/// `H_c = λ·w`, and seen from its reach `k = R + H` its hold is the deficit
/// law's `λw/(k + λw)`, growing with its size (no square); it binds by its
/// WHOLE 3Z, holding and magnetism, `(λ + μ)w/(k + (λ + μ)w)` (the same rule
/// as the universal core's hold: the open frames attract). It collapses where
/// that beats its agitation and the shear across it, `σ = h·w/(k(k + w))`.
/// Its agitation is ONE for the clump, the same ignition reads: its cooled
/// residue plus half the enclosing centre's UNLOCKED face at its place,
/// `½·(1 − δ)`, `δ = h/(R + h)` (operator: the face the core does not lock
/// heats; the galaxy's for a star's clump, the star's for a world's).
fn holds(st: &Stretch, h: &Ratio, w: &Ratio) -> bool {
    let k = st.inner.clone() + h.clone();
    let unlocked = ratio(1, 1) - h.clone() / k.clone();
    let a = st.agitation.clone() + unlocked / ratio(2, 1);
    let own = (st.lambda.clone() + st.mu.clone()) * w.clone();
    let kw = k.clone() * (k.clone() + w.clone());
    // λw/(k + λw) ≥ a + h·w/(k(k + w)), cleared of denominators.
    own.clone() * kw.clone() >= (k + own) * (a * kw + h.clone() * w.clone())
}

/// THE COLLAPSE: the SMALLEST clump width that holds (the self-binding
/// grows with size, so thin gas holds only from a larger width: fewer,
/// larger clumps), found as one crossing by bisection between 0 (no clump
/// of no width holds while the agitation is above 0) and the stretch's
/// width; None where even the whole stretch does not hold. The stars that
/// fit: `⌊width/w_J⌋`; one, the whole stretch, where `w_J` is the width.
pub fn collapse(st: &Stretch, h: &Ratio) -> Option<(Ratio, u64)> {
    let width = st.outer.clone() - st.inner.clone();
    if width <= ratio(0, 1) || !holds(st, h, &width) {
        return None;
    }
    let (mut lo, mut hi) = (ratio(0, 1), width.clone());
    for _ in 0..24 {
        let mid = (lo.clone() + hi.clone()) / ratio(2, 1);
        if holds(st, h, &mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let count = (width / hi.clone()).floor().to_integer().try_into().unwrap_or(u64::MAX).max(1);
    Some((hi, count))
}

/// COLLAPSE ON A CREST (with xylarium; docs/83, "the ocean view one tier
/// down"): a galaxy's own bell lays crests as on the universe's, and the
/// kick collects the galaxy's matter on them; the tidal wave's crest
/// COMPRESSES what stands there (the tide's other face). A crest ring at
/// reach R (own units, the core's holding H) carries `m` of the galaxy's
/// holding and `μ` of its magnetism; its length is R (one turn, no π). A
/// clump of length w along it binds by its whole 3Z read as its own
/// enclosure, `x/(k + x)`, `x = (m + μ)·w/R`, `k = R + H`, against its
/// agitation, half the tide's UNLOCKED face `½·(1 − H/k)` (operator
/// 2026-10-16: the core HOLDS δ and leaves 1 − δ unlocked; held matter is
/// heated by the face the core does not lock, never one face for both) and
/// the shear `H·w/(k(k + w))`. None
/// where the whole ring does not hold; else the smallest holding length
/// `w_J` by one crossing and `⌊R/w_J⌋` clumps along the ring.
pub fn crest_collapse(m: &Ratio, mu: &Ratio, a: &Ratio, r: &Ratio, h: &Ratio) -> Option<(Ratio, u64)> {
    if r.is_zero() || (m.clone() + mu.clone()).is_zero() {
        return None;
    }
    let k = r.clone() + h.clone();
    let floor = a.clone() + (ratio(1, 1) - h.clone() / k.clone()) / ratio(2, 1);
    let holds = |w: &Ratio| {
        let x = (m.clone() + mu.clone()) * w.clone() / r.clone();
        x.clone() / (k.clone() + x) >= floor.clone() + h.clone() * w.clone() / (k.clone() * (k.clone() + w.clone()))
    };
    if !holds(r) {
        return None;
    }
    let (mut lo, mut hi) = (ratio(0, 1), r.clone());
    for _ in 0..24 {
        let mid = (lo.clone() + hi.clone()) / ratio(2, 1);
        if holds(&mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let count = (r.clone() / hi.clone()).floor().to_integer().try_into().unwrap_or(u64::MAX).max(1);
    Some((hi, count))
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
pub fn heartbeat(star: &Manifest, share: &Ratio, rings: u64) -> Option<(u64, Ratio, Ratio)> {
    let (every, light) = burn(star, share, 1, &ratio(1, 1))?;
    let halvings = (rings / every).min(4096) as u32;
    let left = ratio(1, 1) / Ratio::from_integer(num_bigint::BigInt::from(2).pow(halvings));
    Some((every, left.clone(), light * left))
}

/// The kind a burn rung makes from fuel `z`: two nuclei meet, `z + z → 2z`.
pub fn product(z: u32) -> String {
    format!("Z{:03}", 2 * z)
}

/// THE BURN RATE (W31(a) steps 1–2, with xylarium; derived 2026-10-16), the
/// rung fusing fuel Z into 2Z:
/// - the BARRIER is the charge the two nuclei bring, `Z + Z` (enclosed
///   charges add; never `Z·Z`, a reading times itself);
/// - TUNNELLING is the heart's hold against it, a pair share,
///   `τ = h/(h + 2Z)` (h = 1 regenerated): H + H 1/3, He + He 1/5;
/// - the DRIVER is the magnetism the reaction transduces (3Z conserved),
///   `|Δm|/3Z_product`: H + H → He closes 2 of 6, 1/3; He + He → Be opens 2
///   of 12, 1/6. A rung that transduces nothing does not burn;
/// - the INTERFACE share `s/Q̄`, s the star's share of ITS OWN SYSTEM's
///   holding (operator: every share is read in the body's own enclosure; a
///   star is a tiny part of the universe, a large part of its galaxy, the
///   greater part of its system), never of the universe's;
/// - its RHYTHM the heart's coherence c with the echo (a reading of phase).
/// The fuel halves every `⌈1/feed⌉` received rings, `feed =
/// (|Δm|/3Z_p)·τ·s/Q̄·c`; the light per received ring is the transduced
/// flux, `feed × fuel share × |Δm|`. Returns (rings a halving, light a ring).
pub fn burn(star: &Manifest, share: &Ratio, z: u32, coherence: &Ratio) -> Option<(u64, Ratio)> {
    let fuel = star.get(&format!("Z{z:03}"))?.clone();
    let total = mass(star);
    if total.is_zero() || coherence.is_zero() {
        return None;
    }
    let open = |z: u32| i64::from(atom(z).map(|a| a.unpaired).unwrap_or(0));
    let dm = (open(2 * z) - 2 * open(z)).abs();
    if dm == 0 {
        return None;
    }
    let driver = ratio(dm, 3 * 2 * i64::from(z));
    let tau = ratio(1, 1 + 2 * i64::from(z));
    let count = star.values().fold(ratio(0, 1), |s, a| s + a.clone());
    let charge = star.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * ratio(i64::from(z_of(k).and_then(atom).map(|x| x.enclosed).unwrap_or(0)), 1));
    let q_mean = charge / count;
    if q_mean.is_zero() {
        return None;
    }
    let feed = driver * tau * share.clone() / q_mean * coherence.clone();
    let every: u64 = (ratio(1, 1) / feed.clone()).ceil().to_integer().try_into().unwrap_or(u64::MAX).max(1);
    let fuel_share = fuel * ratio(i64::from(hold(z)), 1) / total;
    Some((every, feed * fuel_share * ratio(dm, 1)))
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
/// THE HEART'S CUT, one law for a star and a world (with xylarium; W1–W4):
/// a heart pushes by the electromagnetic repulsion gripping open frames, at
/// full amplitude in its own units (½, regenerated; its state holds 1), so
/// kind z is carried `ℓ_z = p·(e_z/ē)·(w̄/w_z) + turning`, the turning the
/// place adds to every kind alike (a star's convection circulation; a
/// world's place's heat), no weight ratio. Read in order of the spin
/// carried, the hold grows, `H = 1 + held`, and a kind is HELD where it
/// stands within the throat, `ℓ_z/H ≤ H`; past it, it is released. One
/// crossing on the cumulative hold (the runaway inside it: every kind held
/// raises H for the next). Returns (held, released, the held share).
pub fn heart_cut(clump: &Manifest, turning: &Ratio) -> Option<(Manifest, Manifest, Ratio)> {
    let total = mass(clump);
    let count = clump.values().fold(ratio(0, 1), |s, a| s + a.clone());
    if total.is_zero() || count.is_zero() {
        return None;
    }
    let open_mean = crate::pulse::magnetism(clump) / total.clone();
    // REGENERATION (operator ruling (i), with xylarium): the heart pushes
    // at full amplitude in its own units, its pair's ½; a tide enters as
    // AGITATION, never as a weaker push, or it would be counted twice.
    let push = ratio(1, 2);
    let mean_weight = total.clone() / count;
    let mut kinds: Vec<(String, Ratio, Ratio)> = clump
        .iter()
        .map(|(k, a)| {
            let w = ratio(i64::from(z_of(k).map(hold).unwrap_or(1)), 1);
            let open = ratio(i64::from(z_of(k).and_then(atom).map(|x| x.unpaired).unwrap_or(0)), 1) / w.clone();
            // CARRIED SPINS ADD (with xylarium): the place's turning (in
            // the plane orthogonal to the orbit, the rotoreflection) turns
            // every kind alike, no weight ratio.
            let ell = if open_mean.is_zero() { ratio(0, 1) } else { push.clone() * open / open_mean.clone() * mean_weight.clone() / w.clone() } + turning.clone();
            (k.clone(), ell, a.clone() * w / total.clone())
        })
        .collect();
    kinds.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    // NO THROAT, NO STATE (operators 2026-10-17, with xylarium): a heart
    // (torsion) and a planetary core (magnetism) have no bounce, so they
    // hold only by what they have gathered, `H = held`; the state holds only
    // at a throat (the universal and galactic cores). Read kind by kind in
    // order of the spin carried: a kind is held where it stands within the
    // hold it would join, `ℓ/H ≤ H` with H the held so far and its own
    // share; the condition is not monotone, so every kind is read.
    let mut held = ratio(0, 1);
    let (mut kept, mut released) = (Manifest::new(), Manifest::new());
    for (k, ell, share) in kinds {
        let h = held.clone() + share.clone();
        if !h.is_zero() && ell.clone() / h.clone() <= h {
            held = h;
            kept.insert(k.clone(), clump[&k].clone());
        } else {
            released.insert(k.clone(), clump[&k].clone());
        }
    }
    if kept.is_empty() {
        return None;
    }
    Some((kept, released, held))
}

/// The lightest kind a body holds, by hold (`atom::hold`): "held down to Z".
pub fn lightest(m: &Manifest) -> Option<String> {
    m.iter().filter(|(_, a)| **a > ratio(0, 1)).min_by_key(|(k, _)| (z_of(k).map(hold).unwrap_or(0), z_of(k))).map(|(k, _)| k.clone())
}

pub fn system(clump: &Manifest, agitation: &Ratio, circulation: &Ratio, path: &[u64], unit: &Ratio, tide: &Ratio) -> Option<(Manifest, Vec<crate::pulse::Body>, Manifest, Ratio, Ratio)> {
    // Ignition: a clump at a ≥ ½ never forms a heart.
    ignition_share(agitation)?;
    // THE HEART'S TORSION, THE CUT ONE TIER DOWN (operator, with xylarium,
    // 2026-10-16): a heart has no bounce but twists; its push is the
    // electromagnetic repulsion, which grips the OPEN frames (the magnetic
    // moments). In the clump's own units (its grains hold 1, the heart's
    // state holds 1) the push share is the pair law's ½ restarted at the
    // star's point (`½·ρ_sys`), and each kind is carried
    // `ℓ_z = p·(e_z/ē)·(w̄/w_z)` (e_z its open share, unpaired over
    // hold; ē the clump's). Read in order of the spin carried (the noble
    // first: carried none), the heart's hold grows, `H = 1 + held`, and a
    // kind is HELD in the star where it stands within the throat,
    // `R_z = ℓ_z/H ≤ H`; past it, it is the DISC, spun to orbit. One
    // crossing on the cumulative hold. A clump with no open frames (ē = 0)
    // is all star, no disc: a reading.
    let total = mass(clump);
    let (star, mut disc, held) = heart_cut(clump, circulation)?;
    // The star's hold, gathered (no throat): `H_s = held`, a share of the
    // clump; the disc is the rest, `1 − H_s`, over the annulus from the
    // star's hold to the system's rim (its grains' holding, 1), so its line
    // holding is `λ = 1`.
    let h_s = held.clone();
    let x = h_s.clone();
    let rim = ratio(1, 1);
    let scale = total.clone() / unit.clone();
    let star_hold = mass(&star) / unit.clone();
    // At ignition the system is the clump: the star's share of it.
    let light = heartbeat(&star, &(mass(&star) / total.clone()), 0).map(|(_, _, l)| l).unwrap_or_else(|| ratio(0, 1));
    let mid = (x.clone() + rim.clone()) / ratio(2, 1);
    let unlocked = mid.clone() / (mid + x.clone());
    let heat = light * unlocked.clone();
    // THE RECEIVED FLUX (with xylarium): the galaxy's UNLOCKED face where
    // the system stands, `1 − δ_gal` (operator: heat comes from the face the
    // core does not lock, never the held face; it had read `δ_gal`), times
    // the share the star leaves unlocked at the annulus; the channels add,
    // the clump's own agitation the floor.
    let flux = (ratio(1, 1) - tide.clone()) * unlocked;
    let disc_agitation = (heat + flux).max(agitation.clone());
    let mut worlds = Vec::new();
    let mut inside = star_hold.clone();
    let mut load = ratio(0, 1);
    // THE DEPTH MANIFOLDS (xylarium's operator, 2026-10-17; all linear): each
    // world owns where its pull beats the star's, `x = r·H_w/(H_w + H_s +
    // d·x)`, the star's ORTHOGONAL DRAG `d = 1 − δ_gal` (its torsion in its
    // galaxy) dragging each manifold in its own plane; its core from its own
    // sphere, `H_w = 2λx`, so `x = (2λr − H_s)/(2λ + d)`. THE CHAIN: the
    // first world's inner edge at the star (`r₁ = H_s(2λ + d − 1)/d`), each
    // manifold meeting the next, `r_{k+1} = ((r_k + x_k)(2λ + d) − H_s)/d`.
    // A world holds `2λx` of the disc (what remains, never more); the rest
    // is belts. A disc too agitated (`2a ≥ 1`) is all belts.
    let lambda = ratio(1, 1);
    let d = ratio(1, 1) - tide.clone();
    let mut remaining = ratio(1, 1) - h_s.clone();
    if !disc.is_empty() && d > ratio(0, 1) && ratio(2, 1) * disc_agitation.clone() < ratio(1, 1) && remaining > ratio(0, 1) {
        let two_l = ratio(2, 1) * lambda.clone();
        let mut r = h_s.clone() * (two_l.clone() + d.clone() - ratio(1, 1)) / d.clone();
        let mut guard = 0;
        while remaining > ratio(0, 1) && guard < 4096 {
            guard += 1;
            let x_k = (two_l.clone() * r.clone() - h_s.clone()) / (two_l.clone() + d.clone());
            // A world's WHOLE SPHERE stands inside its system's rim (the rim
            // law: nothing beyond the rim; with xylarium).
            if x_k <= ratio(0, 1) || r.clone() + x_k.clone() > rim {
                break;
            }
            let take_share = (two_l.clone() * x_k.clone()).min(remaining.clone());
            let disc_mass = mass(&disc);
            let fragment = if disc_mass.is_zero() { Manifest::new() } else { crate::pulse::take(&disc, &(take_share.clone() * total.clone() / disc_mass).min(ratio(1, 1))) };
            remaining -= take_share;
            load += x_k.clone();
            let next = ((r.clone() + x_k.clone()) * (two_l.clone() + d.clone()) - h_s.clone()) / d.clone();
            let r_here = r.clone();
            r = next;
            if fragment.is_empty() {
                continue;
            }
            crate::pulse::less(&mut disc, &fragment);
            // THE PLANETARY CORE (W1–W4; no throat, `H = held`): the place's
            // turning, the star's deficit at it times the disc's agitation.
            let place = h_s.clone() / (r_here.clone() + h_s.clone()) * disc_agitation.clone();
            let Some((part, released, _)) = heart_cut(&fragment, &place) else {
                for (k, a) in fragment {
                    *disc.entry(k).or_insert_with(|| ratio(0, 1)) += a;
                }
                continue;
            };
            for (k, a) in released {
                *disc.entry(k).or_insert_with(|| ratio(0, 1)) += a;
            }
            let reach = crate::ring::grain(&(r_here * scale.clone()));
            let siblings: Vec<u64> = worlds.iter().map(|o: &crate::pulse::Body| o.path[path.len()]).collect();
            let mut at = path.to_vec();
            at.push(crate::ring::birth_key(0, &reach, &inside, &siblings));
            let period = crate::ring::period(&reach, &inside);
            let ell = crate::ring::spin(&reach, &inside);
            inside += mass(&part) / unit.clone();
            let state = mass(&part);
            worlds.push(crate::pulse::Body { path: at, radius: reach, period, kept: part, satellites: Vec::new(), medium: Manifest::new(), ell, medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state, flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1) });
        }
    }
    let mut belts_spin = mass(&disc) / unit.clone() * crate::ring::spin(&scale, &inside);
    belts_spin += dance(&mut worlds, &star_hold, unit);
    Some((star, worlds, disc, belts_spin, crate::ring::grain(&load)))
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
/// `⌈Q̄ / (e · s)⌉`, s the centre's share (a core: 1, its own holding, under
/// regeneration; a star: still its share of the universe's, open), Q̄ the
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

/// ONE JET BATCH, at ring `born`: the share `share` of the disc's holding,
/// taken innermost (heaviest) first from its rings, reaching the rim.
pub fn jet_batch(rings: &[Shell], h: &Ratio, rim: &Ratio, born: u64, share: &Ratio) -> Vec<Shell> {
    let mut disc: Vec<&Shell> = rings.iter().collect();
    disc.sort_by(|a, b| a.inner.cmp(&b.inner));
    let total = disc.iter().fold(ratio(0, 1), |s, r| s + r.held.clone());
    let push = (h.clone() - ratio(1, 1)) / h.clone();
    if total.is_zero() || push <= ratio(0, 1) {
        return Vec::new();
    }
    let delta = h.clone() / (rim.clone() + h.clone());
    let agitation = if delta < push { (push.clone() - delta) / push.clone() } else { ratio(0, 1) };
    let mut want = total * share.clone();
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
            vacuum: vacuum_halving(rim, h),
        });
    }
    out
}

