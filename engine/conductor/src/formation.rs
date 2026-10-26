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
//!   its held frames, `e_z = unpaired / hold` of its agitation a ring; and
//!   the VACUUM takes back the deficit's share `δ = H/(R + H)` a ring at its
//!   reach (closed-shell gas cools this way alone; close-in gas fastest).
//!   Both shares compound ring by ring (`crate::compound`): n rings keep
//!   `((1 − e_z)(1 − δ))^n`.
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
    pub key: crate::chem::Kind,
    /// Its holding, a share of the galaxy's grains.
    pub held: Ratio,
    pub inner: Ratio,
    pub outer: Ratio,
    /// Its agitation at the cut.
    pub agitation: Ratio,
    /// The share of its agitation its open frames radiate a ring, `e =
    /// unpaired / hold` (0: it never cools by itself).
    pub radiates: Ratio,
    /// The ring it was laid down (the cut: 0; a jet's landing: later); it
    /// cools from then.
    pub born: u64,
    /// The share of its agitation the VACUUM takes back a ring (operator
    /// 2026-10-16, with xylarium): the deficit `δ = H/(R + H)` at its reach.
    /// Every mote cools this way, closed-shell ones included. 0: no vacuum.
    pub vacuum: Ratio,
}

/// The share of a kind's agitation its open frames radiate a ring: its open
/// frames over its held frames, `unpaired / hold` (at most ½, since a kind
/// opens no more frames than its charge and holds at least twice it).
pub fn radiates(k: &crate::chem::Kind) -> Ratio {
    let held = crate::chem::hold(k);
    if held == 0 {
        return ratio(0, 1);
    }
    ratio(i64::from(crate::chem::unpaired(k)), i64::from(held))
}

/// The disc's cold bands and the gas's shells: the galaxy's grains `all`,
/// its core's disc `disc`, in its own units (regeneration). Returns (held,
/// free, H).
pub fn shells(all: &Manifest, disc: &Manifest) -> (Vec<Shell>, Vec<Shell>, Ratio) {
    let weight = |k: &crate::chem::Kind| crate::chem::weight(k);
    let total = mass(all);
    let Some((push, h, mean)) = crate::pulse::galaxy_push(all) else { return (Vec::new(), Vec::new(), ratio(1, 1)) };
    let (mut held_out, mut out) = (Vec::new(), Vec::new());
    for (k, amount) in all {
        let w = ratio(i64::from(weight(k)), 1);
        let ell = push.clone() * mean.clone() / w.clone();
        let r = ell / h.clone();
        let delta = h.clone() / (r.clone() + h.clone());
        let radiates = radiates(k);
        // THE DISPERSAL (`pulse::disperse`; operator: "disc and gas both
        // mixed"): the held part of the kind is the part of its spread
        // within the throat, a COLD BAND `[R(1 − p), min(R(1 + p), H)]` (the
        // disc); the rest spreads as gas beyond the throat, its agitation the
        // push it absorbed and could not re-lock there.
        let held_part = disc.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
        if held_part > ratio(0, 1) {
            let lo = r.clone() * (ratio(1, 1) - push.clone());
            let hi = (r.clone() * (ratio(1, 1) + push.clone())).min(h.clone()).max(lo.clone());
            held_out.push(Shell { key: k.clone(), held: held_part.clone() * w.clone() / total.clone(), inner: lo, outer: hi.clone(), agitation: ratio(0, 1), radiates: radiates.clone(), born: 0, vacuum: crate::ring::deficit(&hi, &h) });
        }
        let free = amount.clone() - held_part;
        let inner = (r.clone() * (ratio(1, 1) - push.clone())).max(h.clone());
        let outer = r.clone() * (ratio(1, 1) + push.clone());
        if free > ratio(0, 1) && outer > inner {
            let agitation = if delta < push { (push.clone() - delta) / push.clone() } else { ratio(0, 1) };
            out.push(Shell { key: k.clone(), held: free * w / total.clone(), inner, outer: outer.clone(), agitation, radiates, born: 0, vacuum: crate::ring::deficit(&outer, &h) });
        }
    }
    (held_out, out, h)
}

/// A shell's agitation at ring `n`: both channels, its open frames' light
/// (`e`) and the vacuum's pull back (`δ`), compounding over its age,
/// `a₀·((1 − e)(1 − δ))^age`, read on the grain; nothing before it is laid
/// down. None where a share lies outside the whole.
fn cooled(s: &Shell, n: u64) -> Option<Ratio> {
    let age = n.saturating_sub(s.born);
    let remaining = crate::compound::after(&s.radiates, age)?.after(&s.vacuum, age).ok()?;
    crate::compound::read(&s.agitation, &remaining, crate::ring::grain)
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

/// The gas profile at ring `n`: piecewise between every shell edge; None
/// where a shell's cooling is refused.
pub fn stretches(shells: &[Shell], n: u64) -> Option<Vec<Stretch>> {
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
        let open = crate::flows::open;
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
            weighted += density.clone() * cooled(s, n)?;
            mu += density.clone() * open(&s.key);
            lambda += density;
        }
        if lambda.is_zero() {
            continue;
        }
        let agitation = weighted / lambda.clone();
        out.push(Stretch { inner: lo.clone(), outer: hi.clone(), lambda, mu, agitation, members });
    }
    Some(out)
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
    // w_J ≤ width, so at least one fits; a count past the carrier is refused.
    let count = crate::ring::whole(&(width / hi.clone()))?;
    Some((hi, count))
}

/// THE RING FRAGMENTS, READ IN THE CLUMP'S OWN ENCLOSURE (docs/96 §7;
/// operator 2026-10-26; the ruling of 2026-10-16, "shares read in the body's
/// own enclosure"). A ring at reach R about the galaxy's hold H carries `m` of
/// the galaxy's holding and `μ` of its magnetism, its heat `a`. A clump of arc
/// w on it carries `x = (m + μ)·w/R`; in its OWN enclosure that is its whole
/// hold (1), and its half-width `w/2` is a length dilated once by its share,
/// `r_c = (w/2)/x`, so it binds by its own deficit,
/// `δ_c = 1/(r_c + 1) = λ/(1 + λ)`, `λ = 2(m + μ)/R` (the ring's line
/// density; the arc cancels). Against it stand its heat, the unlocked face of
/// the galaxy at the ring, `½(1 − δ(R))`, and the galaxy's TIDE across the
/// clump, the contrast of the galaxy's deficit at its two edges,
/// `(δ(R − w/2) − δ(R + w/2))/(δ(R − w/2) + δ(R + w/2)) = w/(2(R + H))`
/// (exact, first degree). The binding does not depend on w and the tide grows
/// with it, so the ring parts into the LARGEST clumps that still hold,
/// `w_max = 2(R + H)·(λ/(1 + λ) − a − face)`, `⌊R/w_max⌋` of them (one where
/// the whole ring holds). None where even a point of the ring cannot hold.
/// (It was read against the galaxy's own enclosure, `x/(R + H + x)` with the
/// galaxy's full shear, which capped any ring's binding near 0.25–0.33 against
/// a barrier near 0.48: no ring could ever hold.)
pub fn crest_collapse(m: &Ratio, mu: &Ratio, a: &Ratio, r: &Ratio, h: &Ratio) -> Option<(Ratio, u64)> {
    if r.is_zero() || (m.clone() + mu.clone()).is_zero() {
        return None;
    }
    let lambda = ratio(2, 1) * (m.clone() + mu.clone()) / r.clone();
    let bind = lambda.clone() / (ratio(1, 1) + lambda);
    let face = (ratio(1, 1) - h.clone() / (r.clone() + h.clone())) / ratio(2, 1);
    let budget = bind - a.clone() - face;
    if budget <= ratio(0, 1) {
        return None;
    }
    let w = (ratio(2, 1) * (r.clone() + h.clone()) * budget).min(r.clone());
    let count = crate::ring::whole(&(r.clone() / w.clone()))?.max(1);
    Some((w, count))
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
    let charge = star.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * per_grain.clone() * ratio(i64::from(crate::chem::parts(k).and_then(|(z, _)| atom(z)).map(|x| x.enclosed).unwrap_or(0)), 1));
    if charge.is_zero() || universe_hold.is_zero() {
        return None;
    }
    Some(mass(star) / universe_hold.clone() / charge)
}

/// The kind a burn rung makes from fuel `z`: two nuclei meet, `z + z → 2z`.
pub fn product(z: u32) -> Option<crate::chem::Kind> {
    crate::chem::Kind::atom(z.checked_mul(2)?)
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
/// Each received ring fuses the share `feed = (|Δm|/3Z_p)·τ·s/Q̄·c` of the
/// fuel that remains, compounding (`crate::compound`): n rings leave
/// `(1 − feed)^n` of it. The light per received ring is the transduced flux,
/// `feed × fuel share × |Δm|`. Returns (the share a ring, light a ring);
/// None where the rung does not run, or its share lies past the whole.
pub fn burn(star: &Manifest, share: &Ratio, z: u32, coherence: &Ratio) -> Option<(Ratio, Ratio)> {
    let fuel = star.get(&crate::chem::Kind::atom(z)?)?.clone();
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
    let charge = star.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * ratio(i64::from(crate::chem::parts(k).and_then(|(z, _)| atom(z)).map(|x| x.enclosed).unwrap_or(0)), 1));
    let q_mean = charge / count;
    if q_mean.is_zero() {
        return None;
    }
    let feed = driver * tau * share.clone() / q_mean * coherence.clone();
    if feed.is_zero() || feed > ratio(1, 1) {
        return None;
    }
    let fuel_share = fuel * ratio(i64::from(hold(z)), 1) / total;
    Some((feed.clone(), feed * fuel_share * ratio(dm, 1)))
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
/// stands within its hold, `ℓ_z/H ≤ H` (deficit ½); past it, it is released. One
/// crossing on the cumulative hold (the runaway inside it: every kind held
/// raises H for the next). Returns (held, released, the held share).
/// A mix's open frames after IONIZATION BY HEAT (`atom::open_at`): what the
/// heat leaves unpaired, by amount. Heat 0 is the whole atoms (magnetism).
pub fn open_mass(m: &Manifest, heat: &Ratio) -> Ratio {
    m.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * ratio(i64::from(crate::chem::atom_z(k).map(|z| cfe::almanac::atom::open_at(z, heat)).unwrap_or_else(|| crate::chem::unpaired(k))), 1))
}

/// A mix with every molecule broken back to its parts (the chemistry layer:
/// what a hot heart does). Charge and the 3Z ledger are unchanged.
pub fn dissociate(m: &Manifest) -> Manifest {
    let mut out = Manifest::new();
    for (k, a) in m {
        match crate::chem::components(k) {
            [(1, _)] => *out.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone(),
            ps => {
                for &(n, z) in ps {
                    if let Some(atom) = crate::chem::Kind::atom(z) {
                        *out.entry(atom).or_insert_with(|| ratio(0, 1)) += a.clone() * ratio(i64::from(n), 1);
                    }
                }
            }
        }
    }
    out
}

/// A star's agitation read back from its ignition share `x = (1 − 2a)/(1 + 2a)`.
pub fn agitation_of(ignition: &Ratio) -> Ratio {
    let x = ignition.clone().max(ratio(0, 1)).min(ratio(1, 1));
    (ratio(1, 1) - x.clone()) / (ratio(2, 1) * (ratio(1, 1) + x))
}

/// THE INFALL (docs/91, the disc from the collapse; operator 2026-10-25,
/// with xylarium: "the core of stars which should be 'reactors'"): of a
/// clump's members, each `(its matter, its holding, its distance from the
/// clump's centre in the clump's own units)`, which fall into the forming
/// heart. A reactor has NO THROAT, so what falls is never read by the throat
/// condition (`r ≤ H*`, `δ ≥ ½`): it is read by the REACTOR'S OWN EXTENT. A
/// member falls where its place stands within `λ·R_max`, `λ` the share of
/// the clump that has fallen and `R_max` the reach of the outermost kind the
/// DARK cut of what has fallen holds (`heart_cut` at turning 0 and heat 0,
/// its push ½), read from all fallen down to the fixed point, order-free (each
/// release shrinks λ and re-reads the cut). The rest stays out: the disc,
/// between the extent and the system's breathing rim.
pub fn infall(members: &[(Manifest, Ratio, Ratio)]) -> Vec<bool> {
    let m = members.iter().fold(ratio(0, 1), |s, (_, w, _)| s + w.clone());
    if m.is_zero() {
        return vec![false; members.len()];
    }
    let mut fallen = vec![true; members.len()];
    loop {
        let mut fell = Manifest::new();
        let mut held_w = ratio(0, 1);
        for ((k, w, _), f) in members.iter().zip(&fallen) {
            if *f {
                for (kind, a) in k {
                    *fell.entry(kind.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
                }
                held_w += w.clone();
            }
        }
        if fell.is_empty() {
            break;
        }
        let fell = dissociate(&fell);
        let extent = match heart_cut(&fell, &ratio(0, 1), &ratio(0, 1)) {
            Some((held, _, h_k)) if !h_k.is_zero() => {
                let r_max = carried(&fell, &ratio(0, 1), &ratio(0, 1)).into_iter().filter(|(k, _)| held.contains_key(k)).map(|(_, l)| l / h_k.clone()).max().unwrap_or_else(|| ratio(0, 1));
                held_w / m.clone() * r_max
            }
            _ => ratio(0, 1),
        };
        let mut changed = false;
        for (i, (_, _, d)) in members.iter().enumerate() {
            if fallen[i] && *d > extent {
                fallen[i] = false;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    fallen
}

/// Each kind's CARRIED SPIN in a heart's clump (own units), the push on its
/// open frames plus the place's turning: `ℓ_z = p·(e_z/ē)·(w̄/w_z) + turning`.
pub fn carried(clump: &Manifest, turning: &Ratio, heat: &Ratio) -> Vec<(crate::chem::Kind, Ratio)> {
    let total = mass(clump);
    let count = clump.values().fold(ratio(0, 1), |s, a| s + a.clone());
    if total.is_zero() || count.is_zero() {
        return Vec::new();
    }
    let open_mean = open_mass(clump, heat) / total.clone();
    let mean_weight = total / count;
    clump
        .keys()
        .map(|k| {
            let w = ratio(i64::from(crate::chem::weight(k)), 1);
            let open = ratio(i64::from(crate::chem::atom_z(k).map(|z| cfe::almanac::atom::open_at(z, heat)).unwrap_or_else(|| crate::chem::unpaired(k))), 1) / w.clone();
            let ell = if open_mean.is_zero() { ratio(0, 1) } else { ratio(1, 2) * open / open_mean.clone() * mean_weight.clone() / w } + turning.clone();
            (k.clone(), ell)
        })
        .collect()
}

pub fn heart_cut(clump: &Manifest, turning: &Ratio, heat: &Ratio) -> Option<(Manifest, Manifest, Ratio)> {
    let total = mass(clump);
    let count = clump.values().fold(ratio(0, 1), |s, a| s + a.clone());
    if total.is_zero() || count.is_zero() {
        return None;
    }
    let open_mean = open_mass(clump, heat) / total.clone();
    // REGENERATION (operator ruling (i), with xylarium): the heart pushes
    // at full amplitude in its own units, its pair's ½; a tide enters as
    // AGITATION, never as a weaker push, or it would be counted twice.
    let push = ratio(1, 2);
    let mean_weight = total.clone() / count;
    let mut kinds: Vec<(crate::chem::Kind, Ratio, Ratio)> = clump
        .iter()
        .map(|(k, a)| {
            let w = ratio(i64::from(crate::chem::weight(k)), 1);
            let open = ratio(i64::from(crate::chem::atom_z(k).map(|z| cfe::almanac::atom::open_at(z, heat)).unwrap_or_else(|| crate::chem::unpaired(k))), 1) / w.clone();
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
    // at a throat (the universal and galactic cores). THE HELD SET IS
    // ORDER-FREE (with xylarium, 2026-10-20; design law: no fill orders): a
    // kind is held where it stands within the hold of the WHOLE held set,
    // `ℓ/H* ≤ H*`, `H* = Σ` the held kinds' shares, the greatest such set,
    // read from all held down (each kind released lowers H*, so the set only
    // shrinks to its fixed point). A running sum in reading order released
    // the lightest-carried kind (stripped hydrogen) before the heavy kinds'
    // hold was counted.
    let mut held_set: Vec<bool> = vec![true; kinds.len()];
    let mut held = kinds.iter().fold(ratio(0, 1), |s, k| s + k.2.clone());
    loop {
        let mut changed = false;
        for (i, (_, ell, share)) in kinds.iter().enumerate() {
            // Held where the heart's deficit at the kind's reach meets its
            // push: `H/(R + H) ≥ p`, `R = ℓ/H` (the deficit law; never a
            // reach against the hold's own square).
            let lost = held.is_zero() || {
                let r = ell.clone() / held.clone();
                held.clone() / (r + held.clone()) < push
            };
            if held_set[i] && lost {
                held_set[i] = false;
                held -= share.clone();
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let (mut kept, mut released) = (Manifest::new(), Manifest::new());
    for (i, (k, _, _)) in kinds.iter().enumerate() {
        if held_set[i] {
            kept.insert(k.clone(), clump[k].clone());
        } else {
            released.insert(k.clone(), clump[k].clone());
        }
    }
    if kept.is_empty() {
        return None;
    }
    Some((kept, released, held))
}

/// THE PLANET KEEPS ITS LAYERS (with xylarium, c0dbfae; 11.0 (16)): a world's
/// layers are a READING of its kept mix by its own cut (`heart_cut`, the
/// heart's push ½, its heat the turning, its atoms whole). The core's held
/// kinds stand at the centre; each other kind at `R_z = ℓ_z/H` in the
/// planet's own unit (H the held share, no throat), spread `R_z(1 ± ½)`. The
/// share within its edge stays BOUND (mantle, crust, the lightest bound
/// gases its atmosphere, in order of reach); the star's tide strips beyond
/// `1 − e`, `e = m/(m + M_star)`. Returns each kind's band `(lo, hi)` (the
/// core's at the centre, `(0, 0)`) and the held share H.
pub fn layer_bands(kept: &Manifest, heat: &Ratio) -> Option<(Vec<(crate::chem::Kind, Ratio, Ratio)>, Ratio)> {
    let (core, _, h) = heart_cut(kept, heat, &ratio(0, 1))?;
    if h.is_zero() {
        return None;
    }
    let push = ratio(1, 2);
    let bands = carried(kept, heat, &ratio(0, 1))
        .into_iter()
        .map(|(k, ell)| {
            if core.contains_key(&k) {
                (k, ratio(0, 1), ratio(0, 1))
            } else {
                let r = ell / h.clone();
                (k, r.clone() * (ratio(1, 1) - push.clone()), r * (ratio(1, 1) + push.clone()))
            }
        })
        .collect();
    Some((bands, h))
}

/// A WORLD STRUCK (an impact: two bodies whose own extents overlap): its
/// layers by its own cut at its heat (`layer_bands`), each kind's carried
/// spin raised by the strike's heat by its EXPOSED FACE, `½(1 − δ)` at its
/// band's middle in the world's own units (`δ = H/(R + H)`): the outer
/// layers, most exposed, take the most and the core, at the centre, none,
/// shared so the holding-weighted strike is the strike (never a fill from
/// the outside in). Returns each kind's band `(lo, hi)` and its carried spin
/// after the strike, the held share H, or None where the core holds nothing.
pub fn struck_bands(kept: &Manifest, heat: &Ratio, strike: &Ratio) -> Option<(Vec<(crate::chem::Kind, Ratio, Ratio, Ratio)>, Ratio)> {
    let (core, _, h) = heart_cut(kept, heat, &ratio(0, 1))?;
    if h.is_zero() {
        return None;
    }
    let push = ratio(1, 2);
    let base = carried(kept, heat, &ratio(0, 1));
    let total = mass(kept);
    let face = |ell: &Ratio| { let r = ell.clone() / h.clone(); (ratio(1, 1) - h.clone() / (r + h.clone())) / ratio(2, 1) };
    // The holding-weighted mean face of what stands outside the core.
    let (fw, ww) = base.iter().filter(|(k, _)| !core.contains_key(k)).fold((ratio(0, 1), ratio(0, 1)), |(f, w), (k, ell)| {
        let wk = kept[k].clone() * ratio(i64::from(crate::chem::weight(k)), 1) / total.clone();
        (f + wk.clone() * face(ell), w + wk)
    });
    let mean_face = if ww.is_zero() { ratio(0, 1) } else { fw / ww };
    let bands = base
        .into_iter()
        .map(|(k, ell)| {
            if core.contains_key(&k) {
                (k, ratio(0, 1), ratio(0, 1), ratio(0, 1))
            } else {
                let took = if mean_face.is_zero() { ratio(0, 1) } else { strike.clone() * face(&ell) / mean_face.clone() };
                let ell = ell + took;
                let r = ell.clone() / h.clone();
                (k, r.clone() * (ratio(1, 1) - push.clone()), r * (ratio(1, 1) + push.clone()), ell)
            }
        })
        .collect();
    Some((bands, h))
}

/// A world's extent in its own units: its outermost band's outer edge.
pub fn extent(kept: &Manifest, heat: &Ratio) -> Ratio {
    struck_bands(kept, heat, &ratio(0, 1)).map(|(b, _)| b.into_iter().map(|(_, _, hi, _)| hi).max().unwrap_or_else(|| ratio(0, 1))).unwrap_or_else(|| ratio(0, 1))
}

/// A world's bound and stripped matter after a strike (`struck_bands`): each
/// kind's band share within `1 − e` stays bound, the rest is stripped, in
/// whole nuclei; with the stripped kinds' reaches in the world's own units
/// (`ℓ/H`, its carried spin over its core's hold), which throw them.
pub fn bind_struck(kept: &Manifest, heat: &Ratio, strike: &Ratio, e: &Ratio) -> (Manifest, Manifest, Vec<(crate::chem::Kind, Ratio)>) {
    let Some((bands, h)) = struck_bands(kept, heat, strike) else { return (Manifest::new(), kept.clone(), Vec::new()) };
    let edge = ratio(1, 1) - e.clone();
    let (mut bound, mut stripped, mut thrown) = (Manifest::new(), Manifest::new(), Vec::new());
    for (k, lo, hi, ell) in bands {
        let a = kept[&k].clone();
        let f = if hi <= edge { ratio(1, 1) } else if lo >= edge { ratio(0, 1) } else { (edge.clone() - lo.clone()) / (hi - lo) };
        let b = if f == ratio(1, 1) { a.clone() } else { crate::pulse::nucleus(&(a.clone() * f)).min(a.clone()) };
        if b > ratio(0, 1) {
            bound.insert(k.clone(), b.clone());
        }
        if a.clone() - b.clone() > ratio(0, 1) {
            stripped.insert(k.clone(), a - b);
            thrown.push((k, ell / h.clone()));
        }
    }
    (bound, stripped, thrown)
}

/// A forming world's bound and stripped matter: of what its core releases,
/// each kind's band share within `1 − e` stays bound, the rest is stripped
/// by the star's tide (in whole nuclei).
pub fn bind_layers(kept: &Manifest, heat: &Ratio, e: &Ratio) -> (Manifest, Manifest) {
    let Some((bands, _)) = layer_bands(kept, heat) else { return (Manifest::new(), kept.clone()) };
    let edge = ratio(1, 1) - e.clone();
    let (mut bound, mut stripped) = (Manifest::new(), Manifest::new());
    for (k, lo, hi) in bands {
        let a = kept[&k].clone();
        let f = if hi <= edge { ratio(1, 1) } else if lo >= edge { ratio(0, 1) } else { (edge.clone() - lo.clone()) / (hi - lo) };
        let b = if f == ratio(1, 1) { a.clone() } else { crate::pulse::nucleus(&(a.clone() * f)).min(a.clone()) };
        if b > ratio(0, 1) {
            bound.insert(k.clone(), b.clone());
        }
        if a.clone() - b.clone() > ratio(0, 1) {
            stripped.insert(k, a - b);
        }
    }
    (bound, stripped)
}

/// MEETINGS INSIDE A WORLD'S LAYERS (with xylarium, 11.0 (16) inside a
/// world): two kinds whose bands overlap meet, their CONTACT the overlap's
/// share of the narrower band (a share on the planet's own sphere, never a
/// length). The heat at a layer is the world's heat carried out by the
/// motion law, `heat(R) = heat₀·H/(R + H)`: deep hot, crust cool, nothing
/// set. A cluster whose loosest winding the heat at its own layer reaches
/// PARTS; two atoms with frames open there close in their smallest whole
/// cluster, the bond holding `min(1, τ_min/heat(R))` of the contact, in
/// whole nuclei. Read at once against the mix as it stands; 3Z exact.
pub fn layer_chemistry(kept: &mut Manifest, heat: &Ratio) {
    let Some((bands, h)) = layer_bands(kept, heat) else { return };
    let at = |r: &Ratio| heat.clone() * h.clone() / (r.clone() + h.clone());
    // Deep clusters part.
    let mut parted = Manifest::new();
    for (k, lo, hi) in &bands {
        if crate::chem::atom_z(k).is_none() {
            let mid = (lo.clone() + hi.clone()) / ratio(2, 1);
            let t = at(&mid);
            if !t.is_zero() && t >= crate::chem::tightness(k) {
                if let Some(a) = kept.remove(k) {
                    for (pk, pa) in dissociate(&[(k.clone(), a)].into()) {
                        *parted.entry(pk).or_insert_with(|| ratio(0, 1)) += pa;
                    }
                }
            }
        }
    }
    for (k, a) in parted {
        *kept.entry(k).or_insert_with(|| ratio(0, 1)) += a;
    }
    // Atoms meet where their bands overlap.
    let atoms: Vec<(crate::chem::Kind, u32, Ratio, Ratio)> = bands.iter().filter_map(|(k, lo, hi)| crate::chem::atom_z(k).filter(|_| kept.contains_key(k)).map(|z| (k.clone(), z, lo.clone(), hi.clone()))).collect();
    let mut formed: Vec<(crate::chem::Kind, crate::chem::Kind, u32, u32, crate::chem::Kind, Ratio)> = Vec::new();
    for (i, (ka, za, la, ha)) in atoms.iter().enumerate() {
        for (kb, zb, lb, hb) in atoms.iter().skip(i + 1) {
            if za == zb {
                continue;
            }
            let (lo, hi) = (la.clone().max(lb.clone()), ha.clone().min(hb.clone()));
            let narrow = (ha.clone() - la.clone()).min(hb.clone() - lb.clone());
            // Two at the centre (zero width) meet whole; else by overlap.
            let contact = if narrow.is_zero() { if lo <= hi { ratio(1, 1) } else { continue } } else if hi <= lo { continue } else { ((hi.clone() - lo.clone()) / narrow).min(ratio(1, 1)) };
            let t = at(&((lo + hi) / ratio(2, 1)));
            let (ua, ub) = (cfe::almanac::atom::open_at(*za, &t), cfe::almanac::atom::open_at(*zb, &t));
            let Some((na, nb, ck)) = crate::chem::closing(*za, ua, *zb, ub) else { continue };
            let t_min = crate::chem::tightness(ka).min(crate::chem::tightness(kb));
            let held = if t.is_zero() { ratio(1, 1) } else { (t_min / t).min(ratio(1, 1)) };
            let n = crate::pulse::nucleus(&(contact * held * (kept[ka].clone() / ratio(i64::from(na), 1)).min(kept[kb].clone() / ratio(i64::from(nb), 1))));
            if n > ratio(0, 1) {
                formed.push((ka.clone(), kb.clone(), na, nb, ck, n));
            }
        }
    }
    for (ka, kb, na, nb, ck, n) in formed {
        let (have_a, have_b) = (kept.get(&ka).cloned().unwrap_or_else(|| ratio(0, 1)), kept.get(&kb).cloned().unwrap_or_else(|| ratio(0, 1)));
        let n = crate::pulse::nucleus(&n.min(have_a / ratio(i64::from(na), 1)).min(have_b / ratio(i64::from(nb), 1)));
        if n <= ratio(0, 1) {
            continue;
        }
        for (k, c) in [(&ka, na), (&kb, nb)] {
            let left = kept[k].clone() - n.clone() * ratio(i64::from(c), 1);
            if left > ratio(0, 1) { kept.insert(k.clone(), left); } else { kept.remove(k); }
        }
        *kept.entry(ck).or_insert_with(|| ratio(0, 1)) += n;
    }
}

/// The lightest kind a body holds, by hold (`atom::hold`): "held down to Z".
pub fn lightest(m: &Manifest) -> Option<crate::chem::Kind> {
    m.iter().filter(|(_, a)| **a > ratio(0, 1)).min_by_key(|(k, _)| (crate::chem::hold(k), crate::chem::charge(k))).map(|(k, _)| k.clone())
}

/// A STAR'S SYSTEM AT IGNITION (docs/91: the disc from the collapse, the wind
/// from the heart): `clump` is what FELL into the heart (`collapse`), `outside`
/// the members that did not, each with its distance in its orbit's plane and
/// its offset on its orthogonal orbit in the system's units, `whole` the
/// whole clump's holding (the system at
/// ignition). The heart is cut in the fallen matter's own units; every
/// crossing between those units and the system's is read as a product of
/// two readings, each moved once (no square, operator 2026-10-23): a reach
/// as a length by λ, the fallen share, and a hold read in the frame it lands
/// in. `fell_spins` are what fell, each `(holding, its place's reach r′,
/// the clump's hold within it H_c(<r′))` in the system's units.
#[allow(clippy::too_many_arguments)]
pub fn system(clump: &Manifest, outside: Vec<(crate::flows::Mote, Ratio, Ratio)>, whole: &Ratio, agitation: &Ratio, fell_spins: &[(Ratio, Ratio, Ratio)], path: &[u64], tide: &Ratio) -> Option<(Manifest, Manifest, Ratio, Vec<crate::flows::Mote>)> {
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
    // kind is HELD in the star where it stands within its hold,
    // `R_z = ℓ_z/H ≤ H`; past it, it is the DISC, spun to orbit. One
    // crossing on the cumulative hold. A clump with no open frames (ē = 0)
    // is all star, no disc: a reading.
    let fell = mass(clump);
    if fell.is_zero() || whole.is_zero() {
        return None;
    }
    let total = whole.clone();
    let lambda = fell.clone() / total.clone();
    // The star's own spin in the fallen matter's units: each member's two
    // readings moved once, its reach `r′/λ` and its hold `H_c/λ`, their
    // product, by holding.
    let fw = fell_spins.iter().fold(ratio(0, 1), |a, (w, _, _)| a + w.clone());
    let circulation_f = if fw.is_zero() || lambda.is_zero() { ratio(0, 1) } else { fell_spins.iter().fold(ratio(0, 1), |a, (w, r, h)| a + w.clone() * (r.clone() / lambda.clone()) * (h.clone() / lambda.clone())) / fw };
    // The new heart is hot: its heat strips the loosest windings (hydrogen
    // first), so it keeps its fuel.
    // THE NEWBORN HEART IS HOT (operator 2026-10-18, "proceed with your
    // recommendation"): its turning in whole turns of its bell, at most one;
    // a heart that rings strips what is held as loosely as hydrogen's lone
    // electron, so it keeps its hydrogen to burn.
    let ionizing = ratio(1, 1);
    let _ = agitation;
    // DISSOCIATION (the chemistry layer): the hot heart breaks every molecule
    // back to its parts (hydrogen's winding, tightness 1, is stripped), so a
    // star never holds molecules and its hydrogen is fuel.
    let clump = &dissociate(clump);
    let (star, disc, held_f) = heart_cut(clump, &circulation_f, &ionizing)?;
    let held = mass(&star) / total.clone();
    // The star's hold, gathered (no throat): `H_s = held`, a share of the
    // clump; the disc is the rest, `1 − H_s`, over the annulus from the
    // star's hold to the system's rim (its grains' holding, 1), so its line
    // holding is `λ = 1`.
    let h_s = held.clone();
    let x = h_s.clone();
    let rim = ratio(1, 1);
    // At ignition the system is the clump: the star's share of it.
    let light = burn(&star, &(mass(&star) / total.clone()), 1, &ratio(1, 1)).map(|(_, l)| l).unwrap_or_else(|| ratio(0, 1));
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
    // WORLDS ARE GROWN, NEVER PLACED (operators 2026-10-16/17, "never place
    // matter"; with xylarium): the disc the heart releases becomes the
    // system's OWN MEDIUM, motes carrying their state (each kind's carried
    // spin, the star's phases, the disc's agitation), which meet over time
    // (`collide_system`) and grow bodies. How many worlds a star holds, and
    // where, is its disc's own (operator 2026-10-21: "a random number of
    // planets attached to each star, for diversity"; 2026-10-22: by physics,
    // never a draw).
    // The heart's released kinds leave as a spray over directions.
    // The newborn star's push: ½ × its hot heart's open share.
    // The spray leaves from the star's cool outer layers: their push (whole
    // atoms), never the hot heart's stripped share.
    let star_push = crate::pulse::push_of(&star);
    // THE DISC (read first: the wind's hold counts it): what never fell, where it stands, turning at the balance of
    // its place in each plane, `ℓ = r·H(<r)` (the ring law: reach × the hold
    // inside it), the hold the system's own enclosure: the star as it stands
    // after its cut and every disc member nearer it (members at one distance
    // hold each other alike); its phases its own; warmed by the newborn star's
    // light and the galaxy's unlocked face. Read, never placed.
    let mut outside = outside;
    outside.sort_by(|a, b| a.1.cmp(&b.1));
    let mut motes: Vec<crate::flows::Mote> = Vec::new();
    let mut inside = h_s.clone();
    let mut i = 0;
    while i < outside.len() {
        let mut j = i;
        while j < outside.len() && outside[j].1 == outside[i].1 {
            j += 1;
        }
        let at = inside.clone();
        for (x, r, c) in &outside[i..j] {
            // In-plane, the balance spin of its place (the ring law); about
            // the orthogonal axis, the circulation it carried, its share `c`
            // of its orbit spin (never its offset read as a second orbit).
            let ell_o = crate::ring::grain(&(r.clone() * at.clone()));
            motes.push(crate::flows::Mote { ell_orth: crate::ring::grain(&(c.clone() * ell_o.clone())), ell_o, agitation: crate::ring::grain(&x.agitation.clone().max(disc_agitation.clone())), ..x.clone() });
            inside += x.amount.clone() * crate::flows::weight(&x.key) / total.clone();
        }
        i = j;
    }
    // THE WIND: what the heart releases leaves at the spin its cut carried it
    // with, read in the system as two readings: its reach in the heart's units
    // (`ℓ_z/H*`) moved once as a length (by λ), and the system's hold at that
    // reach (the star and the disc nearer).
    let hold_at = |r: &Ratio| outside.iter().filter(|(_, d, _)| d < r).fold(h_s.clone(), |a, (x, _, _)| a + x.amount.clone() * crate::flows::weight(&x.key) / total.clone());
    let thrown: Vec<(crate::chem::Kind, Ratio)> = if held_f.is_zero() { Vec::new() } else { carried(clump, &circulation_f, &ionizing).into_iter().map(|(k, l)| { let r = l / held_f.clone() * lambda.clone(); let h = hold_at(&r); (k, r * h) }).collect() };
    motes.extend(crate::flows::spray(&disc, &thrown, &vec![ratio(0, 1); crate::flows::PLANES], &disc_agitation, path.len() as u64, &star_push));
    let mut medium = Manifest::new();
    for x in &motes {
        *medium.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
    }
    // The medium's spin, in the system's own frame: each mote's share times
    // its spins.
    let medium_spin = motes.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key) / total.clone() * (x.ell_o.clone() + x.ell_orth.clone()));
    Some((star, medium, medium_spin, motes))
}




/// A body's holding-weighted TIGHTNESS (`atom::tightness`, its outer band's
/// grip), the ground of its bonding.
pub fn grip(m: &Manifest) -> Ratio {
    let total = mass(m);
    if total.is_zero() {
        return ratio(0, 1);
    }
    m.iter().fold(ratio(0, 1), |s, (k, a)| {
        let t = crate::chem::parts(k).and_then(|(z, _)| atom(z)).map(|x| x.tightness).unwrap_or_else(|| ratio(0, 1));
        s + a.clone() * ratio(i64::from(crate::chem::weight(k)), 1) * t
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
fn meetings(phase0: &[Vec<Ratio>], tilts: &[Vec<Ratio>], ell: &[Ratio], rate: &dyn Fn(&Ratio) -> Ratio, pairs: &[(usize, usize)], meet: &mut dyn FnMut(usize, usize, &Ratio, &mut Vec<Ratio>)) -> (Vec<Ratio>, Vec<Ratio>) {
    let rt: Vec<Ratio> = ell.iter().map(|l| rate(l)).collect();
    let mut dl = vec![ratio(0, 1); ell.len()];
    for &(i, j) in pairs {
        // Their contact in each spatial plane over the echo, its mean (docs/95).
        // Each plane turns by its share of the body's turning (its tilt).
        let (ti, tj) = (crate::flows::tilt_of(&tilts[i]), crate::flows::tilt_of(&tilts[j]));
        let c = phase0[i].iter().zip(&phase0[j]).enumerate().fold(ratio(0, 1), |s, (k, (a, b))| s + crate::flows::contact(&(b.clone() - a.clone()), &(rt[j].clone() * tj[k].clone() - rt[i].clone() * ti[k].clone()))) / ratio(3, 1);
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

/// A STAR SYSTEM'S ONE FRAME (docs/85 §7; the ring law in the system's own
/// units, its holding 1; operator 2026-10-23, no squares: each body keeps its
/// spin in its own enclosure's frame): its enclosure, the star's hold the
/// base and every mote and world inside by its holding share at its own spin.
/// None where the system holds nothing.
pub fn system_frame(sys: &crate::pulse::Body, unit: &Ratio) -> Option<crate::flows::Enclosed> {
    let total = mass(&sys.total());
    if total.is_zero() || unit.is_zero() {
        return None;
    }
    Some(crate::flows::Enclosed::new(
        mass(&sys.kept) / total.clone(),
        sys.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / total.clone())).chain(sys.satellites.iter().map(|w| (w.ell.clone(), mass(&w.total()) / total.clone()))).collect(),
    ))
}



/// COLLISIONS AMONG A GALAXY'S STARS (operator 2026-10-18: "this is how you
/// get stars colliding, or random strange star events"; with xylarium): the
/// one collide law of `collide_system` one tier up, in the galaxy's own units
/// and its one enclosure (a star's spin kept in that frame, its reach
/// `R = ℓ/H(<ℓ)`). Two stars coincide within each
/// other's band in reach and on the same hemisphere in every plane; where
/// `s·D ≥ T` they STICK and merge (mixes, systems, media, spins and states
/// added exactly; place and phases by holding, on the circle); else they
/// BOUNCE, the unkept motion `(T − s·D)⁺` exchanged between them in each
/// plane by the one exchange law (pair-conserving): their orbit spins and
/// their orthogonal spins, kept on their own ledgers.
/// `c_pre` is each star's coherence before; a merger's is its members' by
/// holding. Returns the stars, aligned with their coherences, and
/// (sticks, bounces).
#[allow(clippy::too_many_arguments)]
pub fn collide_stars(stars: Vec<crate::pulse::Body>, c_pre: Vec<Ratio>, scale: &Ratio, g_total: &Ratio, enc: &crate::flows::Enclosed, cross: &mut crate::enfold::Cross) -> (Vec<(crate::pulse::Body, Ratio)>, usize, usize) {
    let planes = crate::flows::PLANES;
    let n = stars.len();
    if n < 2 || scale.is_zero() || g_total.is_zero() {
        return (stars.into_iter().zip(c_pre).collect(), 0, 0);
    }
    // The galaxy's frame: a star's spins are kept there, its reach the one
    // enclosure's.
    let own: Vec<Ratio> = stars.iter().map(|b| b.ell.clone()).collect();
    let own_c: Vec<Ratio> = stars.iter().map(|b| b.ell_orth.clone()).collect();
    let r: Vec<Ratio> = own.iter().map(|l| enc.reach(l).0).collect();
    let m: Vec<Ratio> = stars.iter().map(|b| mass(&b.total()) / g_total.clone()).collect();
    let g: Vec<Ratio> = stars.iter().map(|b| grip(&b.kept)).collect();
    // Each star's deficit by what is enclosed inside it (the cyclotron).
    let delta = |ell: &Ratio| enc.reach(ell).1;
    let mut parent: Vec<usize> = (0..n).collect();
    let mut heat = vec![ratio(0, 1); n];
    let (mut sticks, mut bounces) = (0, 0);
    // Each star's elongation, its orthogonal spin over its orbit spin (docs/90 D2):
    // its reach swings over `r(1 ± |e|)` each orthogonal turn.
    let e_c: Vec<Ratio> = stars.iter().map(|b| if b.ell.is_zero() { ratio(0, 1) } else { b.ell_orth.clone() / b.ell.clone() }).collect();
    let abs = |x: &Ratio| if *x < ratio(0, 1) { -x.clone() } else { x.clone() };
    // Neighbours only, by reach; a pair can meet where their dispersal bands
    // overlap, `r(1 ± p)`, each band widened by the reach its orthogonal orbit sweeps.
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|a, b| r[*a].cmp(&r[*b]).then(a.cmp(b)));
    let pairs: Vec<(usize, usize)> = order
        .windows(2)
        .map(|w| (w[0].min(w[1]), w[0].max(w[1])))
        .filter(|&(i, j)| {
            if stars[i].phase.len() < planes || r[i].is_zero() || stars[j].phase.len() < planes || r[j].is_zero() {
                return false;
            }
            // Two stars meet where their GRAVITY SPHERES overlap (operator
            // 2026-10-25, the board: the galaxy's dispersal band, ±p_g of a
            // star's whole reach, made every star a third of its galaxy wide,
            // so neighbours of one mix stuck as soon as they stood in step):
            // each its tether sphere, radius `k·r`, `k = h/H(<ℓ)` (the sphere
            // settling reads), widened by the reach its orthogonal orbit
            // sweeps.
            let (inner, outer) = if r[i] < r[j] { (i, j) } else { (j, i) };
            let reach_out = r[inner].clone() * (ratio(1, 1) + abs(&e_c[inner]));
            let reach_in = r[outer].clone() * (ratio(1, 1) - abs(&e_c[outer]).min(ratio(1, 1)));
            let sphere = |x: usize| { let h = enc.hold(&own[x]); if h.is_zero() { ratio(0, 1) } else { m[x].clone() / h * r[x].clone() } };
            // They meet where their spheres OVERLAP (docs/78: placed reaches
            // touch and never overlap).
            reach_out + sphere(inner) > reach_in - sphere(outer)
        })
        .collect();
    // THE MEETINGS: each star's orbit turns as its galaxy's flows do at its
    // reach (the reference it rides, `2δ` an echo, flows::turn), by its own
    // spin; a bounce's exchange moves its spins, and so its reach.
    let ell: Vec<Ratio> = own.clone();
    let mut dl_c = vec![ratio(0, 1); n];
    // Where each stands this echo on its swing (docs/90 D2): its turning and
    // its relative turning read at its present reach, its orbit phase half a
    // turn on where the swing has carried it through the centre.
    let swing: Vec<(Ratio, bool)> = (0..n).map(|i| crate::flows::swung(&ell[i], &(ell[i].clone() * e_c[i].clone()), stars[i].phase.get(crate::flows::LOOP).unwrap_or(&ratio(0, 1)))).collect();
    let ell_now: Vec<Ratio> = swing.iter().map(|(l, _)| l.clone()).collect();
    let phase0: Vec<Vec<Ratio>> = stars.iter().zip(&swing).map(|(b, (_, through))| {
        let mut p = crate::flows::space_of(&b.phase);
        if *through {
            crate::flows::antipode(&mut p);
        }
        p
    }).collect();
    let rate = |l: &Ratio| ratio(2, 1) * delta(l);
    let (_, dl) = {
        let stars = &stars;
        let parent = &mut parent;
        let heat = &mut heat;
        let dl_c = &mut dl_c;
        let own_c = &own_c;
        let (sticks, bounces) = (&mut sticks, &mut bounces);
        let ell = &ell;
        let ell_now = &ell_now;
        let tilts: Vec<Vec<Ratio>> = stars.iter().map(|b| b.tilt.clone()).collect();
        meetings(&phase0, &tilts, ell_now, &rate, &pairs, &mut |i, j, c, dl| {
            let s = if g[i].is_zero() || g[j].is_zero() { ratio(0, 1) } else if g[i] < g[j] { g[i].clone() / g[j].clone() } else { g[j].clone() / g[i].clone() };
            // Closeness a share on the sphere: their contact over the echo in
            // the spatial planes times their in-step on the R loop.
            let lp = crate::flows::LOOP;
            let near = c.clone() * crate::flows::in_step(stars[i].phase.get(lp).unwrap_or(&ratio(0, 1)), stars[j].phase.get(lp).unwrap_or(&ratio(0, 1)));
            let m_ij = m[i].clone() + m[j].clone();
            let bind = s * m_ij.clone() / (m_ij.clone() + ratio(1, 1) - near);
            let (di, dj) = (delta(&ell_now[i]), delta(&ell_now[j]));
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
                // The unkept motion parts them in each plane by the one
                // exchange law: their orbit spins and their orthogonal spins.
                exchange(ell, dl, i, j, &m[i], &m[j], &unkept);
                exchange(own_c, dl_c, i, j, &m[i], &m[j], &unkept);
                *bounces += 1;
            }
        })
    };
    let mut stars = stars;
    for i in 0..n {
        // The exchange stands on the star's own spins; its reach read from them.
        if !dl[i].is_zero() {
            let now = own[i].clone() + dl[i].clone();
            stars[i].ell = crate::ring::grain(&now);
            stars[i].radius = crate::ring::grain(&(enc.reach(&now).0 * scale.clone()));
        }
        if !dl_c[i].is_zero() {
            stars[i].ell_orth = crate::ring::grain(&(own_c[i].clone() + dl_c[i].clone()));
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
        let agit = |b: &crate::pulse::Body| agitation_of(&b.reservoir);
        let a_g = if wt.is_zero() { ratio(0, 1) } else { idx.iter().zip(&w).fold(ratio(0, 1), |a, (&i, x)| a + x.clone() * (agit(slots[i].as_ref().expect("each star once")) + heat[i].clone())) / wt.clone() };
        let hot = idx.iter().any(|&i| !heat[i].is_zero());
        let mut head = slots[idx[0]].take().expect("each star once");
        if idx.len() > 1 {
            let members: Vec<crate::pulse::Body> = idx[1..].iter().map(|&i| slots[i].take().expect("each star once")).collect();
            let all: Vec<&crate::pulse::Body> = std::iter::once(&head).chain(members.iter()).collect();
            let by = |f: &dyn Fn(&crate::pulse::Body) -> Ratio| if wt.is_zero() { f(all[0]) } else { all.iter().zip(&w).fold(ratio(0, 1), |a, (b, x)| a + x.clone() * f(b)) / wt.clone() };
            let ell = by(&|b| b.ell.clone());
            // Its tilt: the members' plane spins added (docs/96 step 1).
            let tilt = crate::flows::blend(&all.iter().zip(&w).map(|(b, x)| (x.clone(), b.ell.clone(), b.tilt.clone())).collect::<Vec<_>>());
            // Its reach read from its spin by the one enclosure.
            let at_ell = ell.clone();
            let radius = enc.reach(&at_ell).0 * scale.clone();
            let reservoir = by(&|b| b.reservoir.clone());
            // Their orthogonal spin by holding (docs/90): Σ m·ℓ_⊥ is kept.
            let ell_orth = by(&|b| b.ell_orth.clone());
            // CONDENSE (the cross): every merging star's full phase is read
            // (ascent), meaned by holding on the circle, and held to the merged
            // star's own place; its residue is the detail of the merged
            // system, which the members' re-reading against it keeps.
            let merged_hold = all.iter().fold(ratio(0, 1), |s, b| s + mass(&b.total())) / g_total.clone();
            let place = crate::enfold::place(&enc.reach(&at_ell).0, &merged_hold, &enc.hold(&at_ell));
            let phase: Vec<Ratio> = (0..planes)
                .map(|k| {
                    let mean = crate::flows::circle_mean(&all.iter().zip(&w).map(|(b, x)| (x.clone(), b.phase.get(k).cloned().unwrap_or_else(|| ratio(0, 1)))).collect::<Vec<_>>());
                    for _ in &all {
                        cross.note(k, crate::enfold::Fate::Ascended, &ratio(0, 1));
                    }
                    let (held, residue) = crate::enfold::hold(&mean, place.as_ref());
                    cross.note(k, crate::enfold::Fate::Condensed, &residue);
                    held
                })
                .collect();
            let bell: Vec<Ratio> = (0..planes).map(|k| crate::enfold::hold(&crate::flows::circle_mean(&all.iter().zip(&w).map(|(b, x)| (x.clone(), b.bell.get(k).cloned().unwrap_or_else(|| ratio(0, 1)))).collect::<Vec<_>>()), place.as_ref()).0).collect();
            // A system stands against its star: each merging star's worlds and
            // motes are re-read against the merged star by the difference of
            // the two stars' phases, so every body stays where it is.
            let frac_of = |x: Ratio| x.clone() - x.floor();
            let rebase = |sats: &mut Vec<crate::pulse::Body>, flows: &mut Vec<crate::flows::Mote>, was: &[Ratio]| {
                for k in 0..planes.min(was.len()).min(phase.len()) {
                    let shift = was[k].clone() - phase[k].clone();
                    for w in sats.iter_mut() {
                        if let Some(ph) = w.phase.get_mut(k) {
                            *ph = frac_of(ph.clone() + shift.clone());
                        }
                    }
                    for x in flows.iter_mut() {
                        if let Some(ph) = x.phase.get_mut(k) {
                            *ph = frac_of(ph.clone() + shift.clone());
                        }
                    }
                }
            };
            // THE STARS' OWN SPINS ADD (conservation; no frame to cross): each
            // is kept extensive on the universe's fixed unit (`Body::turning`),
            // so the merged star's is their sum, exact.
            let spin_sum = std::iter::once(&head).chain(members.iter()).fold(ratio(0, 1), |a, b| a + b.turning.clone());
            let head_was = head.phase.clone();
            rebase(&mut head.satellites, &mut head.flows, &head_was);
            for mut b in members {
                let was = b.phase.clone();
                rebase(&mut b.satellites, &mut b.flows, &was);
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
                head.medium_spin += b.medium_spin;
                head.load += b.load;
                head.born = head.born.min(b.born);
                // A merger with a remnant is a remnant.
                head.remnant = head.remnant || b.remnant;
            }
            head.turning = spin_sum;
            head.tilt = tilt;
            head.radius = crate::ring::grain(&radius);
            head.ell = crate::ring::grain(&ell);
            head.ell_orth = crate::ring::grain(&ell_orth);
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

/// THE KICKS OVER THE AGE (xylarium 11.0 (52), its time; netstratum's
/// square-free reading, operator 2026-10-26: "wire it"): two neighbours at
/// rates `ω_i, ω_j` a ring meet once a synodic turn, so over the age `a` (the
/// rings their star's clock received since its birth, `breath::system_clock`
/// nested in the galaxy's and the universe's) they meet `N = a·|ω_i − ω_j|`
/// times. Each meeting turns one by `ε = (m/(r(r + m)))/|Δω|` (the other's
/// pull a ring over the rings between meetings), an excursion `ε·R` at its
/// reach. The meetings are never random in an exact universe: the encounter
/// phase steps by `s = frac(ω_i/Δω)`, rational, so the kicks repeat with
/// period q, the denominator of the simplest `p/q` that tracks s over the
/// age (`|s − p/q| ≤ 1/(N·q)`, from s's continued fraction). Over a period
/// the hemisphere law's responses cancel but for one, its residual `c =
/// in_step(q·ψ_0)` at their phase offset ψ_0. So the greatest excursion is
/// `ε·R·(⌊N/q⌋·c + min(N mod q, q))`, linear: no square, no root. They CROSS
/// (and join) where it reaches their gap. Co-orbital pairs (Δω = 0) never
/// meet and never join; low-order resonances (small q) add many periods and
/// cross; high-order pairs nearly cancel and stand.
fn crossed(w_i: &Ratio, w_j: &Ratio, r: &Ratio, m: &Ratio, reach: &Ratio, psi0: &Ratio, age: u64) -> bool {
    if r.is_zero() {
        return true;
    }
    let dw = if w_i > w_j { w_i.clone() - w_j.clone() } else { w_j.clone() - w_i.clone() };
    if dw.is_zero() || age == 0 {
        return false;
    }
    let n_meet = (ratio(i64::try_from(age).unwrap_or(i64::MAX), 1) * dw.clone()).floor();
    if n_meet.is_zero() {
        return false;
    }
    let eps = m.clone() / (r.clone() * (r.clone() + m.clone())) / dw.clone();
    // The encounter phase's step and the period it repeats on over the age.
    let s = {
        let x = w_i.clone() / dw.clone();
        x.clone() - x.floor()
    };
    let q = {
        // Continued-fraction convergents of s: the first that tracks it over N.
        let (mut h0, mut h1) = (num_bigint::BigInt::from(0), num_bigint::BigInt::from(1));
        let (mut k0, mut k1) = (num_bigint::BigInt::from(1), num_bigint::BigInt::from(0));
        let mut x = s.clone();
        let mut q_found: Option<num_bigint::BigInt> = None;
        for _ in 0..64 {
            let a = x.floor().to_integer();
            let (h2, k2) = (a.clone() * h1.clone() + h0.clone(), a * k1.clone() + k0.clone());
            let approx = Ratio::new(h2.clone(), k2.clone());
            let gap = if approx > s { approx - s.clone() } else { s.clone() - approx };
            if gap * n_meet.clone() * Ratio::from_integer(k2.clone()) <= ratio(1, 1) {
                q_found = Some(k2.clone());
                break;
            }
            h0 = h1;
            h1 = h2;
            k0 = k1;
            k1 = k2;
            let f = x.clone() - x.floor();
            if f.is_zero() {
                break;
            }
            x = ratio(1, 1) / f;
        }
        Ratio::from_integer(q_found.unwrap_or_else(|| s.denom().clone()))
    };
    let periods = (n_meet.clone() / q.clone()).floor();
    let rest = n_meet.clone() - periods.clone() * q.clone();
    let c = crate::flows::in_step(&(q.clone() * psi0.clone()), &ratio(0, 1));
    let kicks = periods * c + rest.min(q);
    eps * reach.clone() * kicks >= *r
}

/// THE SYSTEM'S WORLDS BY CONVERGENCE (xylarium 11.0 (52), the N-body master
/// equation, read on our strip; operator 2026-10-26: "build it"). Every other
/// body turns a body toward itself and never speeds it up, so neighbours
/// stand apart only while their differential turning outruns the turning
/// each gives the other: `|ω_i − ω_j| ≥ m/(r·(r + m))` for each of the pair
/// (`ω = 2δ` a bell, `δ = H(<R)/(R + H(<R))`, r their gap, m the other's
/// share, c = 1). Where the test fails their paths converge and they JOIN.
/// The system's loose matter starts as small bodies, each kind at the reach
/// its heart's cut carries it to, with every world already standing; pairs
/// that fail the test join (all at once, then read again) until every
/// neighbour passes. What stands is the system: the number and spacing of
/// its worlds are outcomes, no ruler and no dice. THE DOUBLE PASS stays:
/// nothing rests on one leg (`M∘M = id`), so matter on the return leg inside
/// the ring stands at its image `2H − R` through the ring (`flows::through`),
/// the loop closing at `2H`; what lies past it leaves for the galaxy, and
/// what the heart holds (inside its extent) returns to the star. A body whose
/// core holds nothing is no world: its matter stays the belts. Each world
/// stands at its reach, turning at the balance there about the star and
/// every world inside it, in closed form between events. The spin the
/// re-reading moves is booked to the system's medium. Returns (what left
/// past the loop, worlds formed, the spin moved).
#[allow(clippy::too_many_arguments)]
pub fn settle_system(sys: &mut crate::pulse::Body, unit: &Ratio, age: u64, r_echo: u64, rang: bool, bell: u64) -> (Manifest, Settled) {
    let mut escaped = Manifest::new();
    let mut read = Settled::default();
    let total = mass(&sys.total());
    if total.is_zero() || sys.kept.is_empty() || unit.is_zero() {
        return (escaped, read);
    }
    let planes = crate::flows::PLANES;
    let frac = |x: Ratio| x.clone() - x.floor();
    let spin_of = |b: &crate::pulse::Body| b.satellites.iter().fold(ratio(0, 1), |s, w| s + mass(&w.total()) / unit.clone() * w.ell.clone()) + b.flows.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key) / unit.clone() * x.ell_o.clone());
    let spin_before = spin_of(sys);
    let h_s = mass(&sys.kept) / total.clone();
    let pool: Manifest = std::mem::take(&mut sys.medium);
    sys.flows.clear();
    let mut mix = sys.kept.clone();
    for (kind, a) in &pool {
        *mix.entry(kind.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
    }
    let (extent, reach_of) = match heart_cut(&mix, &ratio(0, 1), &ratio(0, 1)) {
        Some((held, _, h_k)) if !h_k.is_zero() => {
            let reaches: Vec<(crate::chem::Kind, Ratio)> = carried(&mix, &ratio(0, 1), &ratio(0, 1)).into_iter().map(|(kind, l)| (kind, h_s.clone() * l / h_k.clone())).collect();
            let ext = reaches.iter().filter(|(kind, _)| held.contains_key(kind)).map(|(_, r)| r.clone()).max().unwrap_or_else(|| ratio(0, 1));
            (ext, reaches)
        }
        _ => (ratio(0, 1), Vec::new()),
    };
    let ring = h_s.clone();
    let rim = ratio(2, 1) * ring.clone();
    let b0 = if extent > ring { extent } else { ring.clone() };
    // A small body: (reach, share, matter, phases, tilt, path, heat).
    // A small body: (reach, share, matter, phases, tilt, path, heat, the
    // strike its joining gave it: the turning contrast of what met).
    type Small = (Ratio, Ratio, Manifest, Vec<Ratio>, Vec<Ratio>, Option<Vec<u64>>, Ratio, Ratio);
    let mut bodies: Vec<Small> = Vec::new();
    let mut to_star = Manifest::new();
    for (kind, a) in &pool {
        let r = reach_of.iter().find(|(c, _)| c == kind).map(|(_, r)| r.clone()).unwrap_or_else(|| b0.clone());
        // The return leg stands at its image through the ring.
        let r = if r < ring { crate::flows::through(&r, &ring) } else { r };
        if r > rim {
            *escaped.entry(kind.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
            continue;
        }
        if r < b0 {
            *to_star.entry(kind.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
            continue;
        }
        let z_turn = ratio(i64::from(crate::chem::charge(kind)), 2);
        let mut phase = crate::flows::rest(ratio(1, 2));
        for &s in &crate::flows::SPACE {
            phase[s] = crate::ring::grain(&frac(sys.phase.get(s).cloned().unwrap_or_else(|| ratio(0, 1)) + z_turn.clone()));
        }
        bodies.push((r, mass(&[(kind.clone(), a.clone())].into()) / total.clone(), [(kind.clone(), a.clone())].into(), phase, sys.tilt.clone(), None, ratio(0, 1), ratio(0, 1)));
    }
    for (kind, a) in to_star {
        *sys.kept.entry(kind).or_insert_with(|| ratio(0, 1)) += a;
    }
    for w in std::mem::take(&mut sys.satellites) {
        let r = if total.is_zero() { b0.clone() } else { w.radius.clone() * unit.clone() / total.clone() };
        let r = if r < b0 { b0.clone() } else { r };
        let mut ph = w.phase.clone();
        crate::flows::full(&mut ph);
        bodies.push((r, mass(&w.total()) / total.clone(), w.total(), ph, w.tilt.clone(), Some(w.path.clone()), w.heat.clone(), ratio(0, 1)));
    }
    // CONVERGENCE: read every neighbouring pair at once; those failing the
    // test join; read again until all pass.
    loop {
        bodies.sort_by(|a, b| a.0.cmp(&b.0));
        let n = bodies.len();
        if n < 2 {
            break;
        }
        // Each body's turning a received ring, `ω = δ = H(<R)/(R + H(<R))`
        // (the turning law), on its star's clock.
        let mut inner = h_s.clone();
        let mut omega: Vec<Ratio> = Vec::with_capacity(n);
        for b in &bodies {
            omega.push(inner.clone() / (b.0.clone() + inner.clone()));
            inner += b.1.clone();
        }
        let mut join = vec![false; n - 1];
        for i in 0..n - 1 {
            let r = bodies[i + 1].0.clone() - bodies[i].0.clone();
            let psi0 = crate::flows::SPACE.iter().fold(ratio(0, 1), |s, &kp| s + bodies[i].3.get(kp).cloned().unwrap_or_else(|| ratio(0, 1)) - bodies[i + 1].3.get(kp).cloned().unwrap_or_else(|| ratio(0, 1))) / ratio(3, 1);
            // The excursion each gives the other over the system's age (the
            // larger reads the pair): its pull a ring at their gap, `m/(r(r + m))`.
            let reach = bodies[i].0.clone();
            join[i] = [&bodies[i + 1].1, &bodies[i].1].iter().any(|m| crossed(&omega[i], &omega[i + 1], &r, m, &reach, &psi0, age));
        }
        if !join.iter().any(|j| *j) {
            break;
        }
        let mut merged: Vec<Small> = Vec::new();
        let mut group: Vec<Small> = Vec::new();
        let mut rates: Vec<Ratio> = Vec::new();
        for (i, b) in std::mem::take(&mut bodies).into_iter().enumerate() {
            group.push(b);
            rates.push(omega[i].clone());
            if i < n - 1 && join[i] {
                continue;
            }
            // Join the group: holding adds; reach and heat by holding; phases
            // meaned on the circle; tilts by their plane spins; the heaviest
            // standing world keeps its name.
            let m: Ratio = group.iter().fold(ratio(0, 1), |s, g| s + g.1.clone());
            if group.len() == 1 || m.is_zero() {
                merged.extend(group.drain(..));
                rates.clear();
                continue;
            }
            read.joins += group.len() - 1;
            // The strike: how differently what met was turning, `(ω_max −
            // ω_min)/(ω_max + ω_min)`, and any strike its members carried.
            let (w_hi, w_lo) = (rates.iter().max().cloned().unwrap_or_else(|| ratio(0, 1)), rates.iter().min().cloned().unwrap_or_else(|| ratio(0, 1)));
            let contrast = if (w_hi.clone() + w_lo.clone()).is_zero() { ratio(0, 1) } else { (w_hi.clone() - w_lo.clone()) / (w_hi + w_lo) };
            let strike = crate::ring::grain(&group.iter().fold(contrast, |s, g| s.max(g.7.clone())));
            rates.clear();
            let r = crate::ring::grain(&(group.iter().fold(ratio(0, 1), |s, g| s + g.1.clone() * g.0.clone()) / m.clone()));
            let heat = crate::ring::grain(&(group.iter().fold(ratio(0, 1), |s, g| s + g.1.clone() * g.6.clone()) / m.clone()));
            let mut matter = Manifest::new();
            for g in &group {
                for (kind, a) in &g.2 {
                    *matter.entry(kind.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
                }
            }
            let phase: Vec<Ratio> = (0..planes).map(|kp| crate::ring::grain(&crate::flows::circle_mean(&group.iter().map(|g| (g.1.clone(), g.3.get(kp).cloned().unwrap_or_else(|| ratio(0, 1)))).collect::<Vec<_>>()))).collect();
            let tilt = crate::flows::blend(&group.iter().map(|g| (g.1.clone(), g.0.clone(), g.4.clone())).collect::<Vec<_>>());
            let path = group.iter().filter(|g| g.5.is_some()).max_by(|a, b| a.1.cmp(&b.1)).and_then(|g| g.5.clone());
            group.clear();
            merged.push((r, m, matter, phase, tilt, path, heat, strike));
        }
        bodies = merged;
    }
    // What stands: a world where its core holds; otherwise the belts.
    bodies.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out: Vec<crate::pulse::Body> = Vec::new();
    let mut belts = Manifest::new();
    let mut formed = 0usize;
    let mut inner = mass(&sys.kept) / total.clone();
    let mut siblings: Vec<u64> = bodies.iter().filter_map(|b| b.5.as_ref().and_then(|p| p.last().copied())).collect();
    for (r, m, matter, phase, tilt, path, heat, strike) in bodies {
        let delta_at = inner.clone() / (r.clone() + inner.clone());
        let face = (ratio(1, 1) - delta_at.clone()) / ratio(2, 1);
        // IGNITION OF A GROWN BODY (one accretion path; with xylarium): it is
        // a STAR of its system where its own matter binds as a filament,
        // `λ/(1 + λ)`, `λ` its share over its reach, against its heat and the
        // unlocked face, and passes the ignition test `2(a + face) < 1`.
        let ignites = matter.len() > 1 && !r.is_zero() && {
            let lambda = m.clone() / r.clone();
            lambda.clone() / (ratio(1, 1) + lambda) >= heat.clone() + face.clone() && ratio(2, 1) * (heat.clone() + face.clone()) < ratio(1, 1)
        };
        // Otherwise THE PLANETARY CORE: a body whose core holds nothing is no
        // world; one that holds keeps what stands within the edge its star's
        // tide leaves, `1 − e` (`e` its pinch share), struck by its joining
        // where it met (`bind_struck`), else as it formed (`bind_layers`);
        // the stripped is the belts.
        let (matter, stripped) = if ignites {
            (matter, Manifest::new())
        } else if heart_cut(&matter, &heat, &ratio(0, 1)).map(|(held, _, _)| held.is_empty()).unwrap_or(true) {
            (Manifest::new(), matter)
        } else {
            let e = crate::ring::pinch_share(&m, &inner);
            if strike.is_zero() {
                bind_layers(&matter, &heat, &e)
            } else {
                let (bound, gone, _) = bind_struck(&matter, &heat, &strike, &e);
                (bound, gone)
            }
        };
        for (kind, a) in stripped {
            *belts.entry(kind).or_insert_with(|| ratio(0, 1)) += a;
        }
        if matter.is_empty() {
            continue;
        }
        let m = mass(&matter) / total.clone();
        let path = match path {
            Some(p) => p,
            None => {
                let key = crate::ring::birth_key(bell, &r, &inner, &siblings);
                siblings.push(key);
                formed += 1;
                let mut p = sys.path.clone();
                p.push(key);
                p
            }
        };
        let delta = delta_at;
        let mut w = crate::pulse::Body { ell_orth: ratio(0, 1), path, radius: crate::ring::grain(&(r.clone() * total.clone() / unit.clone())), period: crate::ring::period(&r, &inner), kept: matter, satellites: Vec::new(), medium: Manifest::new(), ell: crate::ring::grain(&(r.clone() * inner.clone())), medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: ratio(0, 1), flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase, load: ratio(0, 1), bell: Vec::new(), remnant: false, heat, tilt, throat: Default::default() };
        w.state = mass(&w.kept);
        if ignites {
            // A star of its system: born marks it (worlds are born 0).
            w.born = bell.max(1);
            read.ignited += 1;
        } else {
            read.worlds_mass += mass(&w.kept);
            read.worlds_nuclei += w.kept.values().fold(ratio(0, 1), |s, a| s + a.clone());
        }
        // CLOSED-FORM TURNING between events: `2δ` a ring over this echo's
        // rings, each spatial plane by its tilt, the loop whole.
        let advance = ratio(2, 1) * delta.clone() * Ratio::from_integer(num_bigint::BigInt::from(r_echo));
        let t = crate::flows::tilt_of(&w.tilt);
        for kp in 0..planes.min(w.phase.len()) {
            let share = crate::flows::SPACE.iter().position(|&s| s == kp).map(|j| t[j].clone()).unwrap_or_else(|| ratio(1, 1));
            w.phase[kp] = crate::ring::grain(&frac(w.phase[kp].clone() + advance.clone() * share));
        }
        if rang {
            w.heat = crate::ring::grain(&(w.heat.clone() * (ratio(1, 1) - ratio(1, 2) * (ratio(1, 1) - delta))));
        }
        if w.born == 0 {
            layer_chemistry(&mut w.kept, &w.heat);
        }
        inner += m;
        out.push(w);
    }
    sys.satellites = out;
    sys.medium = belts;
    let medium_was = sys.medium_spin.clone();
    let moved = spin_of(sys) - spin_before.clone();
    sys.medium_spin -= moved.clone();
    read.formed = formed;
    read.moved = moved;
    // Its identity: the spin it re-read stands booked (bodies and medium
    // together unchanged).
    read.residual = spin_of(sys) + (sys.medium_spin.clone() - medium_was) - spin_before;
    (escaped, read)
}

/// What a system's settling read (`settle_system`).
#[derive(Debug, Clone, Default)]
pub struct Settled {
    /// Worlds born this settling.
    pub formed: usize,
    /// Small bodies joined into others.
    pub joins: usize,
    /// Grown bodies that ignited as companion stars.
    pub ignited: usize,
    /// The holding and the nuclei of the worlds standing.
    pub worlds_mass: Ratio,
    pub worlds_nuclei: Ratio,
    /// The spin the re-reading moved, booked to the system's medium.
    pub moved: Ratio,
    /// Its identity residual (zero where booked exactly).
    pub residual: Ratio,
}

#[cfg(test)]
mod crest_tests {
    use super::crest_collapse;
    use cfe::ratio::ratio;

    #[test]
    fn a_ring_parts_into_the_largest_clumps_that_hold_in_their_own_enclosure() {
        // The bell-462 outermost ring: m + μ = 0.57 at R 1.64 about H 1.94,
        // heat 0.069: λ = 0.695, binding 0.41 beats heat + face 0.30.
        let got = crest_collapse(&ratio(52, 100), &ratio(5, 100), &ratio(69, 1000), &ratio(164, 100), &ratio(194, 100));
        let (w, n) = got.expect("a dense ring holds");
        assert!(w > ratio(0, 1) && n >= 1);
        // A thin inner ring (m + μ = 0.045) does not: its binding 0.08 falls
        // short of its face alone.
        assert!(crest_collapse(&ratio(40, 1000), &ratio(5, 1000), &ratio(75, 1000), &ratio(1015, 1000), &ratio(1089, 1000)).is_none());
        // Denser rings part into more, smaller clumps? No: binding grows, so
        // the largest holding arc grows and the clumps are fewer and larger.
        let (w2, n2) = crest_collapse(&ratio(80, 100), &ratio(5, 100), &ratio(69, 1000), &ratio(164, 100), &ratio(194, 100)).expect("holds");
        assert!(w2 >= w && n2 <= n);
    }
}

#[cfg(test)]
mod crossing_tests {
    use super::crossed;
    use cfe::ratio::ratio;

    #[test]
    fn neighbours_cross_by_their_kicks_over_the_age_never_by_a_ruler() {
        let (r, m, reach, psi) = (ratio(1, 10), ratio(1, 100), ratio(1, 1), ratio(0, 1));
        // Co-orbital: they never meet, however old.
        assert!(!crossed(&ratio(1, 3), &ratio(1, 3), &r, &m, &reach, &psi, 1_000_000));
        // No age, no meetings.
        assert!(!crossed(&ratio(1, 3), &ratio(1, 4), &r, &m, &reach, &psi, 0));
        // A young pair stands where an old one at a low-order resonance
        // crosses: a light neighbour's kick is far short of the gap, so it
        // takes many meetings in step (each kick 4·10^-4 of a gap of 0.1).
        let light = ratio(1, 1_000_000);
        let young = crossed(&ratio(1, 2), &ratio(1, 4), &r, &light, &reach, &psi, 10);
        let old = crossed(&ratio(1, 2), &ratio(1, 4), &r, &light, &reach, &psi, 10_000_000);
        assert!(!young && old, "the kicks of a 2:1 accumulate over the age");
        // Far apart for their masses, they stand even when old.
        assert!(!crossed(&ratio(1, 2), &ratio(1, 4), &ratio(1, 1), &ratio(1, 1_000_000), &reach, &psi, 1000));
    }
}

#[cfg(test)]
mod tests {
    use super::{layer_bands, layer_chemistry};
    use num_traits::Zero;
    use crate::flows::contact;
    use cfe::ratio::ratio;

    #[test]
    fn a_cool_world_keeps_layers_and_makes_water_in_them() {
        use crate::pulse::Manifest;
        let three_z = |m: &Manifest| m.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * ratio(3 * i64::from(crate::chem::charge(k)), 1));
        let mut world: Manifest = [(crate::chem::Kind::parse("Z026").expect("a kind"), ratio(40, 1)), (crate::chem::Kind::parse("Z017").expect("a kind"), ratio(7, 1)), (crate::chem::Kind::parse("Z011").expect("a kind"), ratio(11, 1))].into();
        let before = three_z(&world);
        // Its layers are read by its own cut: the core at the centre.
        let (bands, h) = layer_bands(&world, &ratio(0, 1)).expect("a core");
        assert!(h > ratio(0, 1));
        assert!(bands.iter().any(|(_, lo, hi)| lo.is_zero() && hi.is_zero()));
        // Cool, oxygen and hydrogen meet where their bands overlap.
        layer_chemistry(&mut world, &ratio(0, 1));
        assert_eq!(three_z(&world), before);
        assert!(world.keys().any(|k| crate::chem::atom_z(k).is_none()), "some cluster forms where bands overlap, cool");
        // Hot past every winding, every cluster parts again.
        let mut hot = world.clone();
        layer_chemistry(&mut hot, &ratio(1000, 1));
        assert_eq!(three_z(&hot), before);
    }

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
