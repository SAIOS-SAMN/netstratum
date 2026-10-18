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
    /// mote cools this way, closed-shell ones included. 0: no vacuum.
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
    // A rung runs only where PAIRING RISES (with xylarium): fusion turns
    // magnetism into holding and sends light out, `2·u(z) > u(2z)`. Where
    // the product is more open it would take light in, so it does not run,
    // and the ladder stops by the law (an absolute value let it climb
    // without end on an atom defined at every Z).
    let dm = 2 * open(z) - open(2 * z);
    if dm <= 0 {
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
/// Each kind's CARRIED SPIN in a heart's clump (own units), the push on its
/// open frames plus the place's turning: `ℓ_z = p·(e_z/ē)·(w̄/w_z) + turning`.
pub fn carried(clump: &Manifest, turning: &Ratio) -> Vec<(String, Ratio)> {
    let total = mass(clump);
    let count = clump.values().fold(ratio(0, 1), |s, a| s + a.clone());
    if total.is_zero() || count.is_zero() {
        return Vec::new();
    }
    let open_mean = crate::pulse::magnetism(clump) / total.clone();
    let mean_weight = total / count;
    clump
        .keys()
        .map(|k| {
            let w = ratio(i64::from(z_of(k).map(hold).unwrap_or(1)), 1);
            let open = ratio(i64::from(z_of(k).and_then(atom).map(|x| x.unpaired).unwrap_or(0)), 1) / w.clone();
            let ell = if open_mean.is_zero() { ratio(0, 1) } else { ratio(1, 2) * open / open_mean.clone() * mean_weight.clone() / w } + turning.clone();
            (k.clone(), ell)
        })
        .collect()
}

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

pub fn system(clump: &Manifest, agitation: &Ratio, circulation: &Ratio, path: &[u64], unit: &Ratio, tide: &Ratio) -> Option<(Manifest, Vec<crate::pulse::Body>, Manifest, Ratio, Ratio, Vec<crate::flows::Mote>)> {
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
    // WORLDS ARE GROWN, NEVER PLACED (operators 2026-10-17; with xylarium):
    // the disc the heart releases becomes the system's OWN MEDIUM, motes
    // carrying their state (each kind's carried spin, the star's phases, the
    // disc's agitation), which collide over time (`collide_system`) and grow
    // bodies; the depth-manifold chain is the ORDER that emerges, never a
    // placement at birth.
    let worlds: Vec<crate::pulse::Body> = Vec::new();
    let inside = star_hold.clone();
    let load = ratio(0, 1);
    let spins = carried(clump, circulation);
    let _ = (&h_s, &rim, &disc_agitation, &x);
    let belts_spin = mass(&disc) / unit.clone() * crate::ring::spin(&scale, &inside);
    let motes: Vec<crate::flows::Mote> = disc
        .iter()
        .filter(|(_, a)| **a > ratio(0, 1))
        .map(|(k, a)| {
            let ell = spins.iter().find(|(kk, _)| kk == k).map(|(_, l)| crate::ring::grain(l)).unwrap_or_else(|| ratio(0, 1));
            crate::flows::Mote { key: k.clone(), amount: a.clone(), ell_o: ell, ell_c: ratio(0, 1), phase: vec![ratio(0, 1); crate::flows::PLANES], agitation: crate::ring::grain(&disc_agitation), born: path.len() as u64, vacuum: false }
        })
        .collect();
    let _ = &worlds;
    Some((star, worlds, disc, belts_spin, crate::ring::grain(&load), motes))
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


/// A body's holding-weighted TIGHTNESS (`atom::tightness`, its outer band's
/// grip), the ground of its bonding.
fn grip(m: &Manifest) -> Ratio {
    let total = mass(m);
    if total.is_zero() {
        return ratio(0, 1);
    }
    m.iter().fold(ratio(0, 1), |s, (k, a)| {
        let t = z_of(k).and_then(atom).map(|x| x.tightness).unwrap_or_else(|| ratio(0, 1));
        s + a.clone() * ratio(i64::from(z_of(k).map(hold).unwrap_or(1)), 1) * t
    }) / total
}

/// A group's root (bodies that stuck are one group).
fn root(p: &mut [usize], i: usize) -> usize {
    let mut i = i;
    while p[i] != i {
        p[i] = p[p[i]];
        i = p[i];
    }
    i
}

/// ONE ECHO'S MEETINGS (with xylarium): every neighbour pair is read at once
/// from the echo's start (no order), each with its contact over the echo,
/// each body turning along its orbit at `rate(ℓ)`; `meet` reads a pair and
/// may move spins into `dl`. Returns each body's turning over the echo.
fn meetings(phase0: &[Ratio], ell: &[Ratio], rate: &dyn Fn(&Ratio) -> Ratio, pairs: &[(usize, usize)], meet: &mut dyn FnMut(usize, usize, &Ratio, &mut Vec<Ratio>)) -> (Vec<Ratio>, Vec<Ratio>) {
    let rt: Vec<Ratio> = ell.iter().map(|l| rate(l)).collect();
    let mut dl = vec![ratio(0, 1); ell.len()];
    for &(i, j) in pairs {
        let c = crate::flows::contact(&(phase0[j].clone() - phase0[i].clone()), &(rt[j].clone() - rt[i].clone()));
        meet(i, j, &c, &mut dl);
    }
    (rt, dl)
}

/// THE BOUNCE'S SPIN EXCHANGE (one law at every tier): the outer gains and
/// the inner gives, each by the OTHER's mass share of the unkept motion
/// times their spin gap, `Σ m·ℓ` unchanged; the inner never gives more than
/// it carries (both sides scaled by one k, still exact).
fn exchange(ell: &[Ratio], dl: &mut [Ratio], i: usize, j: usize, mi: &Ratio, mj: &Ratio, unkept: &Ratio) {
    let m_ij = mi.clone() + mj.clone();
    if m_ij.is_zero() || *unkept <= ratio(0, 1) {
        return;
    }
    let (o, n_, mo, mn) = if ell[i] > ell[j] { (i, j, mi, mj) } else { (j, i, mj, mi) };
    let u = unkept.clone() * (ell[o].clone() - ell[n_].clone()) / m_ij;
    let give = u.clone() * mo.clone();
    let has = (ell[n_].clone() + dl[n_].clone()).max(ratio(0, 1));
    let k = if give > has && !give.is_zero() { has / give.clone() } else { ratio(1, 1) };
    dl[o] += u * mn.clone() * k.clone();
    dl[n_] -= give * k;
}

/// COLLISIONS IN A STAR SYSTEM'S OWN MEDIUM (operators 2026-10-17, with
/// xylarium: "run it into collisions over time: activity, spin, bounce,
/// bonding … and as the matter gets bigger you'll get form and order").
/// Every mote and world turns about the star by the motion law in the
/// system's own units (`R = ℓ/H_s`, `δ = H_s/(R + H_s)`, two rings a bell).
/// Two COINCIDE where they stand within each other's dispersal band in reach
/// and on the same hemisphere in every plane. They STICK where the pair's
/// bond share times their combined deficit across the gap reaches their
/// relative turning, `s·D ≥ T` (`s = τ_min/τ_max` of their grips, `D = (mᵢ +
/// mⱼ)/(g + mᵢ + mⱼ)`, `T = |δᵢ − δⱼ|/(δᵢ + δⱼ)`); stuck groups (read all at
/// once, no order) become one body, spins added exactly, the unkept motion
/// heat; otherwise they BOUNCE, both heated. Returns (sticks, bounces).
pub fn collide_system(sys: &mut crate::pulse::Body, unit: &Ratio, bell: u64) -> (usize, usize, Manifest) {
    let total = mass(&sys.total());
    if total.is_zero() || (sys.flows.is_empty() && sys.satellites.is_empty()) {
        return (0, 0, Manifest::new());
    }
    let h_s = mass(&sys.kept) / total.clone();
    if h_s.is_zero() {
        return (0, 0, Manifest::new());
    }
    // The system's motes merge by the shared rule (same kind, stream and
    // sectors; the carrier's resolution), so returns don't multiply.
    let _ = crate::flows::merge(&mut sys.flows, &total);
    let scale = total.clone() / unit.clone();
    let planes = crate::flows::PLANES;
    let frac = |x: &Ratio| x.clone() - x.floor();
    // Items: every mote and every world, (manifest, ℓ, phases, agitation).
    // Items: (matter, ℓ, phases, agitation, whether it is a star).
    let mut items: Vec<(Manifest, Ratio, Vec<Ratio>, Ratio, bool)> = Vec::new();
    let n_motes = sys.flows.len();
    for x in std::mem::take(&mut sys.flows) {
        items.push(([(x.key.clone(), x.amount.clone())].into(), x.ell_o.clone(), x.phase.clone(), x.agitation.clone(), false));
    }
    for w in std::mem::take(&mut sys.satellites) {
        let r_own = if scale.is_zero() { ratio(0, 1) } else { w.radius.clone() / scale.clone() };
        let ph = if w.phase.len() == planes { w.phase.clone() } else { vec![ratio(0, 1); planes] };
        items.push((w.kept.clone(), r_own * h_s.clone(), ph, ratio(0, 1), w.born > 0));
    }
    let delta = |ell: &Ratio| { let r = ell.clone() / h_s.clone(); h_s.clone() / (r + h_s.clone()) };
    let n = items.len();
    let reach = |it: &(Manifest, Ratio, Vec<Ratio>, Ratio, bool)| it.1.clone() / h_s.clone();
    let share = |it: &(Manifest, Ratio, Vec<Ratio>, Ratio, bool)| mass(&it.0) / total.clone();
    let mut parent: Vec<usize> = (0..n).collect();
    let (mut sticks, mut bounces) = (0, 0);
    let mut heat = vec![ratio(0, 1); n];
    // NEIGHBOURS ONLY (with xylarium): by reach, each meets the ones beside
    // it; a body cannot pass through a crowd in one echo. A pair can meet
    // where their thermal bands overlap in reach, `r(1 ± a)`.
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|a, b| reach(&items[*a]).cmp(&reach(&items[*b])).then(a.cmp(b)));
    let band = |r: &Ratio, a: &Ratio| (r.clone() * (ratio(1, 1) - a.clone().min(ratio(1, 1))), r.clone() * (ratio(1, 1) + a.clone()));
    let pairs: Vec<(usize, usize)> = order
        .windows(2)
        .map(|w| (w[0].min(w[1]), w[0].max(w[1])))
        .filter(|&(i, j)| {
            let (ri, rj) = (reach(&items[i]), reach(&items[j]));
            let ((_, hi_lo), (lo_hi, _)) = if ri < rj { (band(&ri, &items[i].3), band(&rj, &items[j].3)) } else { (band(&rj, &items[j].3), band(&ri, &items[i].3)) };
            hi_lo >= lo_hi
        })
        .collect();
    // THE MEETINGS (operators 2026-10-18; with xylarium): each turns by its
    // own deficit, two rings a bell, and every neighbour pair is in contact
    // over the echo by how much it stands in line, read at once.
    let phase0: Vec<Ratio> = items.iter().map(|it| it.2[0].clone()).collect();
    let ell: Vec<Ratio> = items.iter().map(|it| it.1.clone()).collect();
    let rate = |l: &Ratio| ratio(2, 1) * delta(l);
    let (advance, dl) = {
        let items = &items;
        let parent = &mut parent;
        let heat = &mut heat;
        let (sticks, bounces) = (&mut sticks, &mut bounces);
        let ell = &ell;
        meetings(&phase0, ell, &rate, &pairs, &mut |i, j, c, dl| {
            let (gi, gj) = (grip(&items[i].0), grip(&items[j].0));
            let s = if gi.is_zero() || gj.is_zero() { ratio(0, 1) } else if gi < gj { gi / gj } else { gj / gi };
            let (mi, mj) = (share(&items[i]), share(&items[j]));
            // CLOSENESS IS A SHARE ON THE SPHERE (operator 2026-10-18: "the
            // only straight lines should be wormholes"): their contact in the
            // orbit's plane over the echo times their in-step in the other
            // planes, never a length; `T` stays apart as how they move.
            let near = (1..planes).fold(c.clone(), |c, k| c * crate::flows::in_step(&items[i].2[k], &items[j].2[k]));
            let m_ij = mi.clone() + mj.clone();
            let bind = s * m_ij.clone() / (m_ij.clone() + ratio(1, 1) - near);
            let (di, dj) = (delta(&ell[i]), delta(&ell[j]));
            let t = if di > dj { di.clone() - dj.clone() } else { dj.clone() - di.clone() } / (di + dj);
            // THE GRIP SPLIT (one law at every tier, with xylarium): the
            // relative turning T divides into what the bond gripped, heat to
            // each by the OTHER's mass share, and what it did not,
            // `(T − s·D)⁺`, carried by their contact, which parts them.
            // Collision is a tether: `T/(s·D)` its tautness, holding below 1,
            // snapping above (operator).
            let (hi_, hj_) = if m_ij.is_zero() { (ratio(0, 1), ratio(0, 1)) } else { (mj.clone() / m_ij.clone(), mi.clone() / m_ij.clone()) };
            if bind >= t {
                let (a, b) = (root(parent, i), root(parent, j));
                if a != b {
                    parent[a] = b;
                }
                heat[i] += t.clone() * hi_;
                heat[j] += t * hj_;
                *sticks += 1;
            } else {
                heat[i] += bind.clone() * c.clone() * hi_;
                heat[j] += bind.clone() * c.clone() * hj_;
                exchange(ell, dl, i, j, &mi, &mj, &((t - bind) * c.clone()));
                *bounces += 1;
            }
        })
    };
    let ell: Vec<Ratio> = ell.iter().zip(&dl).map(|(l, d)| l.clone() + d.clone()).collect();
    for i in 0..n {
        items[i].1 = crate::ring::grain(&ell[i]).max(ratio(0, 1));
        for k in 0..planes {
            items[i].2[k] = crate::ring::grain(&frac(&(items[i].2[k].clone() + advance[i].clone())));
        }
    }
    // Groups: one body each, spins added exactly (holding-weighted ℓ).
    let mut groups: std::collections::BTreeMap<usize, Vec<usize>> = std::collections::BTreeMap::new();
    for i in 0..n {
        let r = root(&mut parent, i);
        groups.entry(r).or_default().push(i);
    }
    let mut worlds = Vec::new();
    let mut motes = Vec::new();
    let mut inside = mass(&sys.kept) / unit.clone();
    // What overreaches the system's rim (reach past 1, its grains' holding;
    // the rim law) crosses whole into the galaxy.
    let mut escaped = Manifest::new();
    let rim = ratio(1, 1);
    for (_, g) in groups {
        let r_group = {
            let m: Ratio = g.iter().map(|&i| mass(&items[i].0)).fold(ratio(0, 1), |a, b| a + b);
            if m.is_zero() { ratio(0, 1) } else { g.iter().fold(ratio(0, 1), |a, &i| a + mass(&items[i].0) * items[i].1.clone()) / m / h_s.clone() }
        };
        if r_group > rim {
            for &i in &g {
                for (k, a) in &items[i].0 {
                    *escaped.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
                }
            }
            continue;
        }
        // A mote that met nothing stays a mote; a world stays a world.
        if g.len() == 1 && g[0] < n_motes {
            let it = &items[g[0]];
            let (k, a) = it.0.iter().next().map(|(k, a)| (k.clone(), a.clone())).unwrap_or_default();
            motes.push(crate::flows::Mote { key: k, amount: a, ell_o: it.1.clone(), ell_c: ratio(0, 1), phase: it.2.clone(), agitation: crate::ring::grain(&(it.3.clone() + heat[g[0]].clone())), born: 0, vacuum: false });
            continue;
        }
        let mut kept = Manifest::new();
        let (mut m, mut ml) = (ratio(0, 1), ratio(0, 1));
        let mut ph: Vec<Vec<(Ratio, Ratio)>> = vec![Vec::new(); planes];
        for &i in &g {
            let mi = mass(&items[i].0);
            ml += mi.clone() * items[i].1.clone();
            for k in 0..planes {
                ph[k].push((mi.clone(), items[i].2[k].clone()));
            }
            m += mi;
            for (k, a) in &items[i].0 {
                *kept.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
            }
        }
        if m.is_zero() {
            continue;
        }
        let ell = crate::ring::grain(&(ml / m.clone()));
        let phase: Vec<Ratio> = ph.iter().map(|x| crate::ring::grain(&crate::flows::circle_mean(x))).collect();
        // THE PLANETARY CORE (no throat, H = held): the grown body keeps what
        // its core holds; what it releases returns to the system's medium as
        // motes at the body's place. A body whose core holds nothing is no
        // world: its matter stays the medium.
        // THE BODY'S HEAT (its ignition reads it): its members' agitation
        // and the motion their bonds gripped, by holding (a share of the
        // body; each pair's grip already split by the other's mass).
        let heat_g = g.iter().fold(ratio(0, 1), |a, &i| a + mass(&items[i].0) * (heat[i].clone() + items[i].3.clone())) / m.clone();
        // IGNITION OF A GROWN BODY (one accretion path; with xylarium): it
        // becomes a STAR of its system where its own matter binds as a
        // filament, `λ/(1 + λ)` with `λ` its holding and magnetism over its
        // reach, against its agitation and the unlocked face, and passes the
        // ignition test (`2(a + face) < 1`). It keeps its matter whole.
        let r_body = ell.clone() / h_s.clone();
        let was_star = g.iter().any(|&i| i >= n_motes && items[i].4);
        let ignites = was_star || (kept.len() > 1 && !r_body.is_zero() && {
            let own = (mass(&kept) + crate::pulse::magnetism(&kept)) / total.clone();
            let lambda = own / r_body.clone();
            let face = (ratio(1, 1) - h_s.clone() / (r_body.clone() + h_s.clone())) / ratio(2, 1);
            lambda.clone() / (ratio(1, 1) + lambda) >= heat_g.clone() + face.clone() && ratio(2, 1) * (heat_g.clone() + face) < ratio(1, 1)
        });
        // Otherwise THE PLANETARY CORE cuts it, its heat the added turning.
        let (kept, released) = if ignites {
            (kept, Manifest::new())
        } else {
            match heart_cut(&kept, &heat_g) {
                Some((k, r, _)) => (k, r),
                None => (Manifest::new(), kept),
            }
        };
        for (k, a) in released {
            if a > ratio(0, 1) {
                motes.push(crate::flows::Mote { key: k, amount: a, ell_o: ell.clone(), ell_c: ratio(0, 1), phase: phase.clone(), agitation: crate::ring::grain(&heat_g), born: 0, vacuum: false });
            }
        }
        if kept.is_empty() {
            continue;
        }
        let m = mass(&kept);
        let r_own = ell.clone() / h_s.clone();
        let radius = crate::ring::grain(&(r_own * scale.clone()));
        let period = crate::ring::period(&radius, &inside);
        let spin = crate::ring::spin(&radius, &inside);
        inside += m.clone() / unit.clone();
        let siblings: Vec<u64> = worlds.iter().map(|w: &crate::pulse::Body| w.path[sys.path.len()]).collect();
        let mut path = sys.path.clone();
        path.push(crate::ring::birth_key(0, &radius, &inside, &siblings));
        worlds.push(crate::pulse::Body { path, radius, period, kept, satellites: Vec::new(), medium: Manifest::new(), ell: spin, medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: m, flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase, load: ratio(0, 1), bell: Vec::new(), remnant: false });
        if ignites {
            // A star of its system: born marks it (worlds are born 0).
            if let Some(w) = worlds.last_mut() {
                w.born = bell.max(1);
            }
        }
    }
    // The medium is what the motes hold.
    let mut medium = Manifest::new();
    for x in &motes {
        *medium.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
    }
    sys.medium = medium;
    sys.flows = motes;
    sys.satellites = worlds;
    (sticks, bounces, escaped)
}

/// COLLISIONS AMONG A GALAXY'S STARS (operator 2026-10-18: "this is how you
/// get stars colliding, or random strange star events"; with xylarium): the
/// one collide law of `collide_system` one tier up, in the galaxy's own units
/// (`R = radius/scale`, `δ = H_g/(R + H_g)`). Two stars coincide within each
/// other's band in reach and on the same hemisphere in every plane; where
/// `s·D ≥ T` they STICK and merge (mixes, systems, media, spins and states
/// added exactly; place and phases by holding, on the circle); else they
/// BOUNCE, each displaced off its flows in every plane by the other's mass
/// share of the unkept motion `(T − s·D)⁺`, outer ahead and inner behind.
/// `c_pre` is each star's coherence before; a merger's is its members' by
/// holding. Returns the stars, aligned with their coherences, and
/// (sticks, bounces).
pub fn collide_stars(stars: Vec<crate::pulse::Body>, c_pre: Vec<Ratio>, scale: &Ratio, h_g: &Ratio, p_g: &Ratio, g_total: &Ratio, enc: &crate::flows::Enclosed) -> (Vec<(crate::pulse::Body, Ratio)>, usize, usize) {
    let planes = crate::flows::PLANES;
    let n = stars.len();
    if n < 2 || scale.is_zero() || g_total.is_zero() {
        return (stars.into_iter().zip(c_pre).collect(), 0, 0);
    }
    let frac = |x: &Ratio| x.clone() - x.floor();
    let r: Vec<Ratio> = stars.iter().map(|b| b.radius.clone() / scale.clone()).collect();
    let m: Vec<Ratio> = stars.iter().map(|b| mass(&b.total()) / g_total.clone()).collect();
    let g: Vec<Ratio> = stars.iter().map(|b| grip(&b.kept)).collect();
    // Each star's deficit by what is enclosed inside it (the cyclotron).
    let delta = |ell: &Ratio| enc.reach(ell).1;
    let mut parent: Vec<usize> = (0..n).collect();
    let mut disp = vec![ratio(0, 1); n];
    let mut heat = vec![ratio(0, 1); n];
    let (mut sticks, mut bounces) = (0, 0);
    // Neighbours only, by reach; a pair can meet where their dispersal bands
    // overlap, `r(1 ± p)`.
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|a, b| r[*a].cmp(&r[*b]).then(a.cmp(b)));
    let pairs: Vec<(usize, usize)> = order
        .windows(2)
        .map(|w| (w[0].min(w[1]), w[0].max(w[1])))
        .filter(|&(i, j)| {
            if stars[i].phase.len() < planes || r[i].is_zero() || stars[j].phase.len() < planes || r[j].is_zero() {
                return false;
            }
            let (lo, hi) = if r[i] < r[j] { (r[i].clone(), r[j].clone()) } else { (r[j].clone(), r[i].clone()) };
            lo * (ratio(1, 1) + p_g.clone()) >= hi * (ratio(1, 1) - p_g.clone().min(ratio(1, 1)))
        })
        .collect();
    // THE MEETINGS: each star's orbit turns as its galaxy's flows do at its
    // reach (the reference it rides, `2δ` an echo, flows::turn); its orbital
    // spin is `ℓ = R·H_g`, and a bounce's exchange moves its reach.
    let phase0: Vec<Ratio> = stars.iter().map(|b| b.phase.first().cloned().unwrap_or_else(|| ratio(0, 1))).collect();
    let ell: Vec<Ratio> = r.iter().map(|x| x.clone() * h_g.clone()).collect();
    let rate = |l: &Ratio| ratio(2, 1) * delta(l);
    let (_, dl) = {
        let stars = &stars;
        let parent = &mut parent;
        let (heat, disp) = (&mut heat, &mut disp);
        let (sticks, bounces) = (&mut sticks, &mut bounces);
        let ell = &ell;
        meetings(&phase0, ell, &rate, &pairs, &mut |i, j, c, dl| {
            let s = if g[i].is_zero() || g[j].is_zero() { ratio(0, 1) } else if g[i] < g[j] { g[i].clone() / g[j].clone() } else { g[j].clone() / g[i].clone() };
            // Closeness a share on the sphere: their orbit contact over the
            // echo times their in-step in the other planes.
            let near = (1..planes).fold(c.clone(), |c, k| c * crate::flows::in_step(&stars[i].phase[k], &stars[j].phase[k]));
            let m_ij = m[i].clone() + m[j].clone();
            let bind = s * m_ij.clone() / (m_ij.clone() + ratio(1, 1) - near);
            let (di, dj) = (delta(&ell[i]), delta(&ell[j]));
            let t = if di > dj { di.clone() - dj.clone() } else { dj.clone() - di.clone() } / (di + dj);
            if bind >= t {
                let (a, b) = (root(parent, i), root(parent, j));
                if a != b {
                    parent[a] = b;
                }
                // The merger's gripped motion, `T`, heats each by the
                // other's mass share.
                heat[i] += t.clone() * m[j].clone() / m_ij.clone();
                heat[j] += t * m[i].clone() / m_ij;
                *sticks += 1;
            } else {
                heat[i] += bind.clone() * c.clone() * m[j].clone() / m_ij.clone();
                heat[j] += bind.clone() * c.clone() * m[i].clone() / m_ij.clone();
                let unkept = (t - bind) * c.clone();
                // The tilt is pushed, outer ahead and inner behind, each by
                // the other's mass share; the orbit by the spin exchange.
                let (o, n_) = if ell[i] > ell[j] { (i, j) } else { (j, i) };
                disp[o] += unkept.clone() * m[n_].clone() / m_ij.clone();
                disp[n_] -= unkept.clone() * m[o].clone() / m_ij;
                exchange(ell, dl, i, j, &m[i], &m[j], &unkept);
                *bounces += 1;
            }
        })
    };
    let ell: Vec<Ratio> = ell.iter().zip(&dl).map(|(l, d)| l.clone() + d.clone()).collect();
    let mut stars = stars;
    for i in 0..n {
        let moved = ell[i].clone() / h_g.clone();
        if moved != r[i] {
            stars[i].radius = crate::ring::grain(&(moved * scale.clone()));
        }
        if !disp[i].is_zero() && stars[i].phase.len() >= planes {
            for k in 1..planes {
                stars[i].phase[k] = crate::ring::grain(&frac(&(stars[i].phase[k].clone() + disp[i].clone())));
            }
        }
    }
    let mut groups: std::collections::BTreeMap<usize, Vec<usize>> = std::collections::BTreeMap::new();
    for i in 0..n {
        let k = root(&mut parent, i);
        groups.entry(k).or_default().push(i);
    }
    let mut slots: Vec<Option<crate::pulse::Body>> = stars.into_iter().map(Some).collect();
    let mut out = Vec::new();
    for (_, mut idx) in groups {
        // The heaviest member is the body the others join (its name kept).
        idx.sort_by(|a, b| m[*b].cmp(&m[*a]).then(a.cmp(b)));
        let w: Vec<Ratio> = idx.iter().map(|&i| m[i].clone()).collect();
        let wt = w.iter().fold(ratio(0, 1), |a, b| a + b.clone());
        let c = if wt.is_zero() { c_pre[idx[0]].clone() } else { idx.iter().zip(&w).fold(ratio(0, 1), |a, (&i, x)| a + x.clone() * c_pre[i].clone()) / wt.clone() };
        // THE STAR'S HEAT (ignition heat): its agitation, read back from
        // its ignition share `(1 − 2a)/(1 + 2a)`, and the motion its
        // encounters gripped, by holding; the share it holds above is read
        // again from it (none at `a ≥ ½`: light pushes back).
        let agit = |b: &crate::pulse::Body| {
            let x = b.reservoir.clone().max(ratio(0, 1)).min(ratio(1, 1));
            (ratio(1, 1) - x.clone()) / (ratio(2, 1) * (ratio(1, 1) + x))
        };
        let a_g = if wt.is_zero() { ratio(0, 1) } else { idx.iter().zip(&w).fold(ratio(0, 1), |a, (&i, x)| a + x.clone() * (agit(slots[i].as_ref().expect("each star once")) + heat[i].clone())) / wt.clone() };
        let hot = idx.iter().any(|&i| !heat[i].is_zero());
        let mut head = slots[idx[0]].take().expect("each star once");
        if idx.len() > 1 {
            let members: Vec<crate::pulse::Body> = idx[1..].iter().map(|&i| slots[i].take().expect("each star once")).collect();
            let all: Vec<&crate::pulse::Body> = std::iter::once(&head).chain(members.iter()).collect();
            let by = |f: &dyn Fn(&crate::pulse::Body) -> Ratio| if wt.is_zero() { f(all[0]) } else { all.iter().zip(&w).fold(ratio(0, 1), |a, (b, x)| a + x.clone() * f(b)) / wt.clone() };
            let radius = by(&|b| b.radius.clone());
            let ell = by(&|b| b.ell.clone());
            let reservoir = by(&|b| b.reservoir.clone());
            let phase: Vec<Ratio> = (0..planes).map(|k| crate::ring::grain(&crate::flows::circle_mean(&all.iter().zip(&w).map(|(b, x)| (x.clone(), b.phase.get(k).cloned().unwrap_or_else(|| ratio(0, 1)))).collect::<Vec<_>>()))).collect();
            let bell: Vec<Ratio> = (0..planes).map(|k| crate::ring::grain(&crate::flows::circle_mean(&all.iter().zip(&w).map(|(b, x)| (x.clone(), b.bell.get(k).cloned().unwrap_or_else(|| ratio(0, 1)))).collect::<Vec<_>>()))).collect();
            for b in members {
                for (k, a) in b.kept {
                    *head.kept.entry(k).or_insert_with(|| ratio(0, 1)) += a;
                }
                for (k, a) in b.medium {
                    *head.medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
                }
                head.satellites.extend(b.satellites);
                head.flows.extend(b.flows);
                head.gas.extend(b.gas);
                head.state += b.state;
                head.turning += b.turning;
                head.medium_spin += b.medium_spin;
                head.load += b.load;
                head.born = head.born.min(b.born);
                // A merger with a remnant is a remnant.
                head.remnant = head.remnant || b.remnant;
            }
            head.radius = crate::ring::grain(&radius);
            head.ell = crate::ring::grain(&ell);
            head.reservoir = crate::ring::grain(&reservoir);
            head.phase = phase;
            head.bell = bell;
        }
        if hot {
            head.reservoir = crate::ring::grain(&ignition_share(&a_g).unwrap_or_else(|| ratio(0, 1)));
        }
        out.push((head, c));
    }
    (out, sticks, bounces)
}

#[cfg(test)]
mod tests {
    use crate::flows::contact;
    use cfe::ratio::ratio;

    #[test]
    fn contact_is_the_mean_in_step_over_the_echo() {
        // At rest in line: whole contact; at rest on the far face: none.
        assert_eq!(contact(&ratio(0, 1), &ratio(0, 1)), ratio(1, 1));
        assert_eq!(contact(&ratio(1, 2), &ratio(0, 1)), ratio(0, 1));
        // A whole passage, either way round, averages one half.
        assert_eq!(contact(&ratio(0, 1), &ratio(1, 1)), ratio(1, 2));
        assert_eq!(contact(&ratio(1, 3), &ratio(-1, 1)), ratio(1, 2));
        // Drifting from in line to a quarter turn: in-step 1 to ½, mean ¾.
        assert_eq!(contact(&ratio(0, 1), &ratio(1, 4)), ratio(3, 4));
        // Across the bell: from −¼ to ¼ passes the peak, mean ¾.
        assert_eq!(contact(&ratio(-1, 4), &ratio(1, 2)), ratio(3, 4));
    }
}
