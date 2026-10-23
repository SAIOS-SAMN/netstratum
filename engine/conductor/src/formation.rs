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
    let weight = |k: &crate::chem::Kind| crate::chem::hold(k);
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
        let open = |k: &crate::chem::Kind| ratio(i64::from(crate::chem::unpaired(k)), i64::from(crate::chem::hold(k).max(1)));
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

/// COLLAPSE ON A CREST (with xylarium; docs/83, "the ocean view one tier
/// down"): a galaxy's own bell lays crests as on the universe's, and the
/// torque collects the galaxy's matter on them; the tidal wave's crest
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
    let count = crate::ring::whole(&(r.clone() / hi.clone()))?;
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
/// stands within the throat, `ℓ_z/H ≤ H`; past it, it is released. One
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

/// THE INFALL (docs/91, the disc from the collapse): of a clump's members,
/// each `(holding, distance from the clump's centre in the clump's own
/// units)`, which fall into the forming heart. A member falls where it stands
/// within the throat of what has fallen, `r ≤ H*` (`δ = H*/(r + H*) ≥ ½`, the
/// heart cut's own test read by place), `H*` the share the fallen hold, read
/// from all fallen down to the fixed point (order-free, as the heart cut's
/// held set). The rest stays out: the disc.
pub fn infall(members: &[(Ratio, Ratio)]) -> Vec<bool> {
    let m = members.iter().fold(ratio(0, 1), |s, (w, _)| s + w.clone());
    if m.is_zero() {
        return vec![false; members.len()];
    }
    let mut fallen = vec![true; members.len()];
    let mut held = ratio(1, 1);
    loop {
        let mut changed = false;
        for i in 0..members.len() {
            if fallen[i] && members[i].1 > held {
                fallen[i] = false;
                held -= members[i].0.clone() / m.clone();
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
            let w = ratio(i64::from(crate::chem::hold(k)), 1);
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
            let w = ratio(i64::from(crate::chem::hold(k)), 1);
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
        let wk = kept[k].clone() * ratio(i64::from(crate::chem::hold(k)), 1) / total.clone();
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
/// whole nuclei; with the stripped kinds' carried spins (in the world's own
/// units), which throw them.
pub fn bind_struck(kept: &Manifest, heat: &Ratio, strike: &Ratio, e: &Ratio) -> (Manifest, Manifest, Vec<(crate::chem::Kind, Ratio)>) {
    let Some((bands, _)) = struck_bands(kept, heat, strike) else { return (Manifest::new(), kept.clone(), Vec::new()) };
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
            thrown.push((k, ell));
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
/// ignition). The heart is cut in the fallen matter's own units and carried
/// into the system's by λ, the fallen share (spin by λ², reach and hold
/// alike); `circulation` is the star's own spin in the system's units.
#[allow(clippy::too_many_arguments)]
pub fn system(clump: &Manifest, outside: Vec<(crate::flows::Mote, Ratio, Ratio)>, whole: &Ratio, agitation: &Ratio, circulation: &Ratio, path: &[u64], unit: &Ratio, tide: &Ratio) -> Option<(Manifest, Vec<crate::pulse::Body>, Manifest, Ratio, Ratio, Vec<crate::flows::Mote>)> {
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
    let fell = mass(clump);
    if fell.is_zero() || whole.is_zero() {
        return None;
    }
    let total = whole.clone();
    let lambda = fell.clone() / total.clone();
    let lambda2 = lambda.clone() * lambda.clone();
    // The star's own spin read in the fallen matter's units (a smaller unit
    // dilates spin by 1/λ²).
    let circulation_f = circulation.clone() / lambda2.clone();
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
    let (star, disc, _) = heart_cut(clump, &circulation_f, &ionizing)?;
    let held = mass(&star) / total.clone();
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
    let worlds: Vec<crate::pulse::Body> = Vec::new();
    let inside = star_hold.clone();
    let load = ratio(0, 1);
    let _ = (&h_s, &rim, &disc_agitation, &x);
    let _ = &inside;
    // The heart's released kinds leave as a spray over directions.
    // The newborn star's push: ½ × its hot heart's open share.
    // The spray leaves from the star's cool outer layers: their push (whole
    // atoms), never the hot heart's stripped share.
    let star_push = if mass(&star).is_zero() { ratio(0, 1) } else { crate::ring::grain(&(ratio(1, 2) * crate::pulse::magnetism(&star) / mass(&star))) };
    // THE WIND: what the heart releases leaves at the spin its cut carried it
    // with, carried from the fallen matter's units into the system's.
    let thrown: Vec<(crate::chem::Kind, Ratio)> = carried(clump, &circulation_f, &ionizing).into_iter().map(|(k, l)| (k, l * lambda2.clone())).collect();
    let mut motes: Vec<crate::flows::Mote> = crate::flows::spray(&disc, &thrown, &vec![ratio(0, 1); crate::flows::PLANES], &disc_agitation, path.len() as u64, &star_push);
    // THE DISC: what never fell, where it stands, turning at the balance of
    // its place in each plane, `ℓ = r·H(<r)` (the ring law: reach × the hold
    // inside it), the hold the system's own enclosure: the star as it stands
    // after its cut and every disc member nearer it (members at one distance
    // hold each other alike); its phases its own; warmed by the newborn star's
    // light and the galaxy's unlocked face. Read, never placed.
    let mut outside = outside;
    outside.sort_by(|a, b| a.1.cmp(&b.1));
    let mut inside = h_s.clone();
    let mut i = 0;
    while i < outside.len() {
        let mut j = i;
        while j < outside.len() && outside[j].1 == outside[i].1 {
            j += 1;
        }
        let at = inside.clone();
        for (x, r, c) in &outside[i..j] {
            motes.push(crate::flows::Mote { ell_o: crate::ring::grain(&(r.clone() * at.clone())), ell_orth: crate::ring::grain(&(c.clone() * at.clone())), agitation: crate::ring::grain(&x.agitation.clone().max(disc_agitation.clone())), ..x.clone() });
            inside += x.amount.clone() * crate::flows::weight(&x.key) / total.clone();
        }
        i = j;
    }
    let mut medium = Manifest::new();
    for x in &motes {
        *medium.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
    }
    // The medium's spin on the ledger: each mote's own, carried into the
    // ledger's units by the system's dilation (`ℓ_ledger = ℓ_sys·scale²`).
    let medium_spin = motes.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key) / unit.clone() * (x.ell_o.clone() + x.ell_orth.clone()) * scale.clone() * scale.clone());
    let _ = &worlds;
    Some((star, worlds, medium, medium_spin, crate::ring::grain(&load), motes))
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
            radiates: ring.radiates.clone(),
            born,
            vacuum: crate::ring::deficit(rim, h),
        });
    }
    out
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
        s + a.clone() * ratio(i64::from(crate::chem::hold(k)), 1) * t
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
/// heat; otherwise they BOUNCE, both heated. Returns (sticks, bounces), the
/// matter that overstepped the rim, and, on the star's bell, the residual of
/// its torque's identity (`h_s·Δℓ_s + Σ m·Δℓ`, the star's spin and its system's
/// across the torques: zero where the star paid exactly what it gave).
pub fn collide_system(sys: &mut crate::pulse::Body, unit: &Ratio, bell: u64, rang: bool, _t_u: u64, cross: &mut crate::enfold::Cross) -> (usize, usize, Manifest, Option<Ratio>, (usize, usize, usize, usize), [Ratio; 4], (usize, usize)) {

    let total = mass(&sys.total());
    if total.is_zero() || (sys.flows.is_empty() && sys.satellites.is_empty()) {
        return (0, 0, Manifest::new(), None, (0, 0, 0, 0), [ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1)], (0, 0));
    }
    let h_s = mass(&sys.kept) / total.clone();
    if h_s.is_zero() {
        return (0, 0, Manifest::new(), None, (0, 0, 0, 0), [ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1)], (0, 0));
    }
    // The star's torque on its system: one force, each kind by its inertia.
    // DISSOCIATION AT THE PLACE'S OWN HEAT (with xylarium): a cluster mote
    // whose heat reaches its looser winding (τ_min) parts back to its atoms.
    let mut parted: Vec<crate::flows::Mote> = Vec::new();
    for x in sys.flows.iter_mut() {
        if crate::chem::atom_z(&x.key).is_none() && !x.agitation.is_zero() && x.agitation >= crate::chem::tightness(&x.key) {
            for (k, a) in dissociate(&[(x.key.clone(), x.amount.clone())].into()) {
                // Each part carries its own share of the parent's lever.
                let share = if x.amount.is_zero() { ratio(0, 1) } else { a.clone() / x.amount.clone() };
                let lever = x.lever.iter().map(|l| crate::pulse::nucleus(&(l.clone() * share.clone()))).collect();
                parted.push(crate::flows::Mote { key: k, amount: a, lever, ..x.clone() });
            }
            x.amount = ratio(0, 1);
        }
    }
    sys.flows.retain(|x| x.amount > ratio(0, 1));
    sys.flows.extend(parted);
    let scale = total.clone() / unit.clone();
    let planes = crate::flows::PLANES;
    let frac = |x: &Ratio| x.clone() - x.floor();
    // THE SYSTEM'S ENCLOSURE (the one grammar: a galaxy's core and every body
    // inside it hold what turns beyond; with xylarium 11.0, the ring-density
    // law): read once at the echo's start, the star's hold the base and every
    // mote and world inside adding its own, in the system's own units (its
    // holding 1, a world's spin `R·h_s` from its radius).
    // ONE FRAME (the ring law read in the system's own units, docs/85 §7: a
    // tier's unit is its own holding): reach and hold are both shares of the
    // system's holding, `R = radius/scale`, `H = H_ledger/scale`, so a world's
    // ledger spin `ℓ = R·H` is its spin here dilated twice, `ℓ_ledger =
    // ℓ_sys·scale²`, while its deficit, and so its period, read the same in
    // both. A world's reach is read from its spin by the one enclosure,
    // never from the star's hold alone.
    let dil = scale.clone() * scale.clone();
    let own_spin = |l: &Ratio| if dil.is_zero() { ratio(0, 1) } else { l.clone() / dil.clone() };
    let enc = crate::flows::Enclosed::new(
        h_s.clone(),
        sys.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / total.clone())).chain(sys.satellites.iter().map(|w| (own_spin(&w.ell), mass(&w.total()) / total.clone()))).collect(),
    );
    // The star's push on its system, `½ ×` its heart's open share.
    let star_push = if mass(&sys.kept).is_zero() { ratio(0, 1) } else { ratio(1, 2) * crate::pulse::magnetism(&sys.kept) / mass(&sys.kept) };
    // THE STAR'S TORQUE ON ITS SYSTEM, on the star's own bell only (its clock),
    // never at every arrival: the one torque law (`flows::spin_torque`), the star
    // its system's core. Each mote, at the reach its orthogonal orbit swings
    // it to, takes the push in its orbit plane and its band (the star's push
    // on its open frames, each kind by its inertia, its hold returning
    // `½·δ·ℓ`), the push along its orthogonal orbit less the hold's return,
    // and keeps what is not delivered in step as heat; through the centre
    // where a torque passes it.
    // PAID BY THE STAR'S OWN SPIN (magnetic braking; the antipole a galaxy's
    // core keeps as its reservoir): the star's spin `ℓ_s` is a specific spin
    // in its system's units (its heart cut adds it to every kind's carried
    // spin at birth), its share of the system's ledger `h_s·ℓ_s`, so what the
    // torques give its system, `Σ m·Δℓ`, it gives up, `Δℓ_s = −Σ m·Δℓ/h_s`;
    // what falls back spins it up. A star that pushes hard turns slower and
    // rings less often.
    let mut paid = ratio(0, 1);
    let ledger = |flows: &[crate::flows::Mote]| flows.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key) / total.clone() * (x.ell_o.clone() + x.ell_orth.clone()));
    let motes_before = ledger(&sys.flows);
    if rang {
        for x in sys.flows.iter_mut() {
            let psi_c = x.phase.get(1).cloned().unwrap_or_else(|| ratio(0, 1));
            let (now, through_c) = crate::flows::swung(&x.ell_o, &x.ell_orth, &psi_c);
            let h = enc.hold(&now);
            let (r_o, d_o) = enc.reach(&now);
            let psi_o = x.phase.first().cloned().unwrap_or_else(|| ratio(0, 1));
            let from = if through_c { frac(&(psi_o + ratio(1, 2))) } else { psi_o };
            let m = x.amount.clone() * crate::flows::weight(&x.key) / total.clone();
            // In its star's own enclosure it feels the star's pull at its own
            // reach (no receiver between); a vacuum mote at its inverted reach,
            // its route the other way.
            let d_c = if x.vacuum { ratio(1, 1) - d_o.clone() } else { d_o.clone() };
            let route = if x.vacuum { ratio(-1, 1) } else { ratio(1, 1) };
            let torque_orth = route * crate::flows::orth_push(&psi_c, &d_c, &star_push, &d_o, x.vacuum);
            let (given, _) = crate::flows::spin_torque(x, &from, &h, &r_o, &d_o, &star_push, &torque_orth, Some(&star_push), &m, &ratio(0, 1));
            paid += given;
        }
    }
    let motes_moved = ledger(&sys.flows) - motes_before;
    let mut worlds_moved = ratio(0, 1);
    // The system's motes merge by the shared rule (same kind, stream and
    // sectors; the carrier's resolution), so returns don't multiply.
    // Each mote's place in the system: the cap its own gravity holds about the
    // star (the system's own units: its holding 1, the star's h_s).
    let enc_s = crate::flows::Enclosed::new(h_s.clone(), sys.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / total.clone())).collect());
    let place_of = |x: &crate::flows::Mote| {
        let (r, _) = enc_s.reach(&x.ell_o);
        crate::enfold::place(&r, &(x.amount.clone() * crate::flows::weight(&x.key) / total.clone()), &enc_s.hold(&x.ell_o))
    };
    let _ = crate::flows::merge(&mut sys.flows, &total, &place_of, cross);
    // Items: every mote and every world, (manifest, ℓ, phases, agitation).
    // Items: (matter, ℓ, phases, agitation, whether it is a star).
    let mut items: Vec<(Manifest, Ratio, Vec<Ratio>, Ratio, bool)> = Vec::new();
    // Each item's ORTHOGONAL SPIN in the system's own units (docs/90): a mote's own;
    // a world's from the units of its `ell`, by the two readings of its orbit
    // spin.
    let mut orth_of: Vec<Ratio> = Vec::new();
    let n_motes = sys.flows.len();
    // Each world's identity, its birth reading, kept while it stands.
    let mut paths: Vec<Vec<u64>> = Vec::new();
    for x in std::mem::take(&mut sys.flows) {
        orth_of.push(x.ell_orth.clone());
        items.push(([(x.key.clone(), x.amount.clone())].into(), x.ell_o.clone(), x.phase.clone(), x.agitation.clone(), false));
    }
    for w in std::mem::take(&mut sys.satellites) {
        let ell_sys = own_spin(&w.ell);
        let ph = if w.phase.len() == planes { w.phase.clone() } else { vec![ratio(0, 1); planes] };
        // A world carries its heat; on its star's bell it cools to the void
        // through its unlocked face, `½(1 − δ)` of it at its reach.
        let heat = if rang {
            let (_, d) = enc.reach(&ell_sys);
            crate::ring::grain(&(w.heat.clone() * (ratio(1, 1) - ratio(1, 2) * (ratio(1, 1) - d))))
        } else {
            w.heat.clone()
        };
        // Its layers meet inside it at their own heat (11.0 (16) inside a
        // world); a star of the system keeps its matter whole.
        let mut kept = w.kept.clone();
        if w.born == 0 {
            layer_chemistry(&mut kept, &heat);
        }
        // ITS ORTHOGONAL SPIN (docs/90 D1): on its star's bell the star's release
        // pushes it along its orthogonal orbit by the one law (`flows::orth_push`),
        // and its hold returns the held half, `½·δ·ℓ_⊥`, δ at the reach its
        // orthogonal orbit swings it to; the star's own spin pays it (exact).
        let mut orth_sys = own_spin(&w.ell_orth);
        if rang && !dil.is_zero() {
            let (now, _) = crate::flows::swung(&ell_sys, &orth_sys, &ph[1]);
            let (_, d) = enc.reach(&now);
            let after = crate::ring::grain(&(orth_sys.clone() + crate::flows::orth_push(&ph[1], &d, &star_push, &d, false) - ratio(1, 2) * d * orth_sys.clone()));
            let moved = mass(&w.total()) / total.clone() * (after.clone() - orth_sys.clone());
            paid += moved.clone();
            worlds_moved += moved;
            orth_sys = after;
        }
        orth_of.push(orth_sys);
        items.push((kept, ell_sys, ph, heat, w.born > 0));
        paths.push(w.path.clone());
    }
    // Each turns by what is enclosed inside it (the star and all inside).
    let delta = |ell: &Ratio| enc.reach(ell).1;
    let n = items.len();
    let reach = |it: &(Manifest, Ratio, Vec<Ratio>, Ratio, bool)| enc.reach(&it.1).0;
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
    // A body's REACH OF MEETING (operator 2026-10-18: "gravity is the stretch
    // of the manifold between the star and the planet where the maximum pull
    // of the pull to the void verse pull towards the star or larger orbital
    // mass is congrued"): a WORLD reaches as far as its pull outweighs its
    // star's, `H_w/(d + H_w) = H_s/((r − d) + H_s)`, so `d = r·H_w/(H_w + H_s)`,
    // its pinch (exact, no root; our pull falls as one over the distance). A
    // mote of gas reaches by its thermal band, its agitation.
    let width = |i: usize| -> Ratio {
        if i >= n_motes {
            let hw = mass(&items[i].0) / total.clone();
            crate::ring::grain(&crate::ring::pinch_share(&hw, &enc.hold(&items[i].1)))
        } else {
            items[i].3.clone()
        }
    };
    // Each band widened by the reach its orthogonal orbit sweeps, `r(1 ± |ℓ_⊥|/ℓ)` (docs/90 D2).
    let sweep = |i: usize| -> Ratio {
        if items[i].1.is_zero() {
            return ratio(0, 1);
        }
        let e = orth_of[i].clone() / items[i].1.clone();
        if e < ratio(0, 1) { -e } else { e }
    };
    let pairs: Vec<(usize, usize)> = order
        .windows(2)
        .map(|w| (w[0].min(w[1]), w[0].max(w[1])))
        .filter(|&(i, j)| {
            let (ri, rj) = (reach(&items[i]), reach(&items[j]));
            let (wi, wj) = (width(i) + sweep(i), width(j) + sweep(j));
            let ((_, hi_lo), (lo_hi, _)) = if ri < rj { (band(&ri, &wi), band(&rj, &wj)) } else { (band(&rj, &wj), band(&ri, &wi)) };
            hi_lo >= lo_hi
        })
        .collect();
    // THE MEETINGS (operators 2026-10-18; with xylarium): each turns by its
    // own deficit, two rings a bell, and every neighbour pair is in contact
    // over the echo by how much it stands in line, read at once.
    let ell: Vec<Ratio> = items.iter().map(|it| it.1.clone()).collect();
    // Where each stands this echo on its swing (docs/90 D2): turning and
    // relative turning at its present reach, its orbit phase half a turn on
    // where the swing carried it through the centre.
    let swing: Vec<(Ratio, bool)> = (0..n).map(|i| crate::flows::swung(&ell[i], &orth_of[i], items[i].2.get(1).unwrap_or(&ratio(0, 1)))).collect();
    let ell_now: Vec<Ratio> = swing.iter().map(|(l, _)| l.clone()).collect();
    let phase0: Vec<Ratio> = items.iter().zip(&swing).map(|(it, (_, through))| if *through { frac(&(it.2[0].clone() + ratio(1, 2))) } else { it.2[0].clone() }).collect();
    let rate = |l: &Ratio| ratio(2, 1) * delta(l);
    // The hydride bonds formed at this echo's meetings: (centre, hydrogen,
    // their keys, the centre's open frames, the hydrides formed).
    let mut bonds: Vec<(usize, usize, crate::chem::Kind, crate::chem::Kind, u32, u32, crate::chem::Kind, Ratio)> = Vec::new();
    // EACH BODY'S EXTENT (its own matter, not its pull): a world's outermost
    // layer band in its own units, dilated into the system's by its holding
    // share (a child's unit is its own holding); a mote has none. Two bodies
    // whose extents overlap STRIKE as matter; within each other's pull
    // without touching, they scatter.
    let ext: Vec<Ratio> = (0..n).map(|i| if i < n_motes { ratio(0, 1) } else { extent(&items[i].0, &items[i].3) * share(&items[i]) }).collect();
    let at_r: Vec<Ratio> = items.iter().map(|it| reach(it)).collect();
    // The heat a strike gives each body, read into its layers by their
    // exposed faces (`struck_bands`), apart from the heat it carries whole.
    let mut strike = vec![ratio(0, 1); n];
    // The bounces that struck as matter, and the tugs between neighbours.
    let (mut struck_n, mut tugs_n) = (0usize, 0usize);
    let (advance, dl) = {
        let items = &items;
        let parent = &mut parent;
        let heat = &mut heat;
        let strike = &mut strike;
        let struck_n = &mut struck_n;
        let (ext, at_r) = (&ext, &at_r);
        let (sticks, bounces) = (&mut sticks, &mut bounces);
        let ell = &ell;
        let ell_now = &ell_now;
        let bonds = &mut bonds;
        meetings(&phase0, ell_now, &rate, &pairs, &mut |i, j, c, dl| {
            let (gi, gj) = (grip(&items[i].0), grip(&items[j].0));
            let s = if gi.is_zero() || gj.is_zero() { ratio(0, 1) } else if gi < gj { gi / gj } else { gj / gi };
            let (mi, mj) = (share(&items[i]), share(&items[j]));
            // CLOSENESS IS A SHARE ON THE SPHERE (operator 2026-10-18: "the
            // only straight lines should be wormholes"): their contact in the
            // orbit's plane over the echo times their in-step in the other
            // planes, never a length; `T` stays apart as how they move.
            let near = (1..planes).fold(c.clone(), |c, k| c * crate::flows::in_step(&items[i].2[k], &items[j].2[k]));
            let m_ij = mi.clone() + mj.clone();
            // THE CONTACT GRIP (the chemistry layer, operator 2026-10-18; the
            // meeting law at the atom tier): bodies in contact also grip by
            // the share of their open frames that can PAIR across it,
            // independent of mass, `2·min(oᵢ, oⱼ)/(oᵢ + oⱼ)` times the
            // contact. Dust sticks by chemistry, large bodies by gravity's
            // share: the bind is the greater.
            // ONE GRIP (with xylarium, 969536c): the pairable-frame share at
            // the place's own heat times the bond's shared share (its two
            // windings' tightness, the lesser over the greater).
            let place_heat = items[i].3.clone().max(items[j].3.clone());
            let open_hot = |m: &Manifest| m.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * ratio(i64::from(crate::chem::atom_z(k).map(|z| cfe::almanac::atom::open_at(z, &place_heat)).unwrap_or_else(|| crate::chem::unpaired(k))), 1));
            let tight = |m: &Manifest| m.keys().map(|k| crate::chem::tightness(k)).max().unwrap_or_else(|| ratio(0, 1));
            let (oi, oj) = (open_hot(&items[i].0), open_hot(&items[j].0));
            let (ti, tj) = (tight(&items[i].0), tight(&items[j].0));
            let shared = if ti.is_zero() || tj.is_zero() { ratio(0, 1) } else if ti < tj { ti.clone() / tj.clone() } else { tj.clone() / ti.clone() };
            // The grip is the pairable share times the bond's shared share
            // (with xylarium, 969536c); the bind is the encounter law as
            // written, grip × D, D the pair's hold across what parts them
            // (in full contact D is 1 whatever their mass, so dust sticks).
            let pairable = if (oi.clone() + oj.clone()).is_zero() { ratio(0, 1) } else { ratio(2, 1) * oi.clone().min(oj.clone()) / (oi.clone() + oj.clone()) };
            let grip_share = (pairable * shared).max(s.clone());
            let _ = &s;
            let bind = grip_share * m_ij.clone() / (m_ij.clone() + ratio(1, 1) - near.clone());
            let (di, dj) = (delta(&ell_now[i]), delta(&ell_now[j]));
            let t = if di > dj { di.clone() - dj.clone() } else { dj.clone() - di.clone() } / (di + dj);
            // THE BOND (the chemistry layer, with xylarium 969536c): any two
            // atoms meeting, each with frames still open at the place's own
            // heat, close in their SMALLEST WHOLE CLUSTER; of the contact,
            // the bond holds the share `min(1, τ_min/heat)` (τ_min the looser
            // winding, the bond's grip), and the rest parts whole.
            if i < n_motes && j < n_motes {
                let key = |x: usize| items[x].0.keys().next().cloned();
                if let (Some(ka), Some(kb)) = (key(i), key(j)) {
                    if let (Some(za), Some(zb)) = (crate::chem::atom_z(&ka), crate::chem::atom_z(&kb)) {
                        let (ua, ub) = (cfe::almanac::atom::open_at(za, &place_heat), cfe::almanac::atom::open_at(zb, &place_heat));
                        if let Some((na, nb, ck)) = crate::chem::closing(za, ua, zb, ub) {
                            let t_min = crate::chem::tightness(&ka).min(crate::chem::tightness(&kb));
                            let held = if place_heat.is_zero() { ratio(1, 1) } else { (t_min / place_heat.clone()).min(ratio(1, 1)) };
                            let (have_a, have_b) = (items[i].0[&ka].clone(), items[j].0[&kb].clone());
                            let formed = crate::pulse::nucleus(&(near.clone() * held * (have_a / ratio(i64::from(na), 1)).min(have_b / ratio(i64::from(nb), 1))));
                            if formed > ratio(0, 1) && za != zb {
                                bonds.push((i, j, ka.clone(), kb.clone(), na, nb, ck, formed));
                            }
                        }
                    }
                }
            }
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
                // A STRIKE where their extents overlap: the gripped motion
                // enters each body's layers by their exposure; a scattering
                // heats each whole.
                let gap = if at_r[i] > at_r[j] { at_r[i].clone() - at_r[j].clone() } else { at_r[j].clone() - at_r[i].clone() };
                let hit = gap <= ext[i].clone() + ext[j].clone();
                if hit {
                    *struck_n += 1;
                }
                let into = if hit { &mut *strike } else { &mut *heat };
                into[i] += bind.clone() * c.clone() * hi_;
                into[j] += bind.clone() * c.clone() * hj_;
                exchange(ell, dl, i, j, &mi, &mj, &((t - bind) * c.clone()));
                *bounces += 1;
            }
        })
    };
    // THE TUGS (operator 2026-10-22: "that needs physics"; the order of
    // worlds is gravity's, never a target): neighbouring worlds that do not
    // meet torque each other by the one torque law, the neighbour the source:
    // its pull at their separation, `δ = H/(gap + H)` (H the pair's holding),
    // signed by where their conjunction runs over the echo (the hemisphere's
    // mean over the arc their relative turning sweeps, `mean_response`): the
    // body ahead is drawn back and the one behind drawn on, in each plane, by
    // their spin gap, `Σ m·ℓ` unchanged. What does not land in step is TIDAL
    // HEAT, `δ·(1 − |k|)` of their gap's share, each by the other's mass. Off
    // a whole-number ratio the conjunction drifts and the tugs cancel echo on
    // echo; on one it recurs and they add: resonance, the first layer of
    // order, is the tugs' sum, never a chosen ratio.
    let mut dl = dl;
    let mut dl_c = vec![ratio(0, 1); n];
    let met: std::collections::BTreeSet<(usize, usize)> = pairs.iter().copied().collect();
    for w in order.windows(2) {
        let (i, j) = (w[0].min(w[1]), w[0].max(w[1]));
        if i < n_motes || met.contains(&(i, j)) {
            continue;
        }
        let (mi, mj) = (share(&items[i]), share(&items[j]));
        let m_ij = mi.clone() + mj.clone();
        if m_ij.is_zero() {
            continue;
        }
        let gap = if at_r[i] > at_r[j] { at_r[i].clone() - at_r[j].clone() } else { at_r[j].clone() - at_r[i].clone() };
        let pull = m_ij.clone() / (gap + m_ij.clone());
        tugs_n += 1;
        let rel = advance[j].clone() - advance[i].clone();
        for (k, (spins, into)) in [(&ell, &mut dl), (&orth_of, &mut dl_c)].into_iter().enumerate() {
            let psi = if k == 0 { phase0[j].clone() - phase0[i].clone() } else { items[j].2.get(1).cloned().unwrap_or_else(|| ratio(0, 1)) - items[i].2.get(1).cloned().unwrap_or_else(|| ratio(0, 1)) };
            let (start, len) = if rel < ratio(0, 1) { (psi.clone() + rel.clone(), -rel.clone()) } else { (psi, rel.clone()) };
            let kk = crate::flows::mean_response(&start, &len);
            let spread = spins[i].clone() + spins[j].clone();
            let gap_l = if spins[i] > spins[j] { spins[i].clone() - spins[j].clone() } else { spins[j].clone() - spins[i].clone() };
            if gap_l.is_zero() {
                continue;
            }
            // j ahead (k > 0) is drawn back, i drawn on.
            let u = pull.clone() * kk.clone() * gap_l.clone() / m_ij.clone();
            into[i] += u.clone() * mj.clone();
            into[j] -= u * mi.clone();
            if !spread.is_zero() {
                let coherent = if kk < ratio(0, 1) { -kk } else { kk };
                let tide = pull.clone() * (ratio(1, 1) - coherent.min(ratio(1, 1))) * gap_l / spread;
                heat[i] += tide.clone() * mj.clone() / m_ij.clone();
                heat[j] += tide * mi.clone() / m_ij.clone();
            }
        }
    }
    for i in 0..n {
        orth_of[i] = crate::ring::grain(&(orth_of[i].clone() + dl_c[i].clone()));
    }
    let ell: Vec<Ratio> = ell.iter().zip(&dl).map(|(l, d)| l.clone() + d.clone()).collect();
    for i in 0..n {
        items[i].1 = crate::ring::grain(&ell[i]).max(ratio(0, 1));
        for k in 0..planes {
            items[i].2[k] = crate::ring::grain(&frac(&(items[i].2[k].clone() + advance[i].clone())));
        }
    }
    // The bonds made: each takes its share from the two motes (never more than
    // they hold) and the hydride stands as a new mote at the centre's place.
    let mut bonded: Vec<crate::flows::Mote> = Vec::new();
    for (ia, ib, ka, kb, na, nb, ck, formed) in bonds {
        let have_a = items[ia].0.get(&ka).cloned().unwrap_or_else(|| ratio(0, 1));
        let have_b = items[ib].0.get(&kb).cloned().unwrap_or_else(|| ratio(0, 1));
        let formed = crate::pulse::nucleus(&formed.min(have_a / ratio(i64::from(na), 1)).min(have_b / ratio(i64::from(nb), 1)));
        if formed <= ratio(0, 1) {
            continue;
        }
        *items[ia].0.get_mut(&ka).expect("held") -= formed.clone() * ratio(i64::from(na), 1);
        *items[ib].0.get_mut(&kb).expect("held") -= formed.clone() * ratio(i64::from(nb), 1);
        // The cluster stands where its heavier part stood.
        let at = if crate::chem::charge(&ka) >= crate::chem::charge(&kb) { ia } else { ib };
        bonded.push(crate::flows::Mote { key: ck, amount: formed, ell_o: items[at].1.clone(), ell_orth: orth_of[at].clone(), phase: items[at].2.clone(), agitation: items[at].3.clone(), born: bell, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1) });
    }
    for it in items.iter_mut() {
        it.0.retain(|_, a| *a > ratio(0, 1));
    }
    // Groups: one body each, spins added exactly (holding-weighted ℓ).
    let mut groups: std::collections::BTreeMap<usize, Vec<usize>> = std::collections::BTreeMap::new();
    for i in 0..n {
        let r = root(&mut parent, i);
        groups.entry(r).or_default().push(i);
    }
    let mut worlds = Vec::new();
    let mut motes = Vec::new();
    // What overreaches the system's rim (reach past 1, its grains' holding;
    // the rim law) crosses whole into the galaxy.
    let mut escaped = Manifest::new();
    // The fates of the bodies grown this echo: worlds, stars ignited, and
    // bodies whose core held nothing (their matter back to the medium).
    let mut fates = (0usize, 0usize, 0usize, 0usize);
    // Their weight: the holding and the nuclei of the grown groups that left
    // past the rim, and of the worlds grown (a body's mean hold a nucleus is
    // how heavy its kinds are).
    let mut weighs = [ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1)];
    let nuclei = |m: &Manifest| m.values().fold(ratio(0, 1), |s, a| s + a.clone());
    let rim = ratio(1, 1);
    for (_, g) in groups {
        let r_group = {
            let m: Ratio = g.iter().map(|&i| mass(&items[i].0)).fold(ratio(0, 1), |a, b| a + b);
            // Its outer swing (docs/90 D2): it escapes when its orthogonal orbit carries
            // it past the rim, not only its mean reach.
            let out = |i: usize| items[i].1.clone() + if orth_of[i] < ratio(0, 1) { -orth_of[i].clone() } else { orth_of[i].clone() };
            if m.is_zero() { ratio(0, 1) } else { enc.reach(&(g.iter().fold(ratio(0, 1), |a, &i| a + mass(&items[i].0) * out(i)) / m)).0 }
        };
        if r_group > rim {
            if g.len() > 1 {
                fates.3 += 1;
                for &i in &g {
                    weighs[0] += mass(&items[i].0);
                    weighs[1] += nuclei(&items[i].0);
                }
            }
            for &i in &g {
                for (k, a) in &items[i].0 {
                    *escaped.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
                }
            }
            continue;
        }
        // A mote that met nothing stays a mote; a world stays a world. THE RIM
        // LAW for every body (a mote too, never only a grown group): a mote
        // past the system's rim crosses whole into the galaxy. Motes that
        // stick grow a body (worlds are grown, never placed).
        if g.len() == 1 && g[0] < n_motes {
            let it = &items[g[0]];
            let swept = if orth_of[g[0]] < ratio(0, 1) { -orth_of[g[0]].clone() } else { orth_of[g[0]].clone() };
            if enc.reach(&(it.1.clone() + swept)).0 > rim {
                for (k, a) in &it.0 {
                    *escaped.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
                }
                continue;
            }
            let Some((k, a)) = it.0.iter().next().map(|(k, a)| (k.clone(), a.clone())) else { continue };
            motes.push(crate::flows::Mote { key: k, amount: a, ell_o: it.1.clone(), ell_orth: orth_of[g[0]].clone(), phase: it.2.clone(), agitation: crate::ring::grain(&(it.3.clone() + heat[g[0]].clone())), born: 0, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1) });
            continue;
        }
        let mut kept = Manifest::new();
        let grown = g.len() > 1;
        let (mut m, mut ml, mut ml_c) = (ratio(0, 1), ratio(0, 1), ratio(0, 1));
        let mut ph: Vec<Vec<(Ratio, Ratio)>> = vec![Vec::new(); planes];
        for &i in &g {
            let mi = mass(&items[i].0);
            ml += mi.clone() * items[i].1.clone();
            ml_c += mi.clone() * orth_of[i].clone();
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
        // Its orthogonal spin by holding (docs/90): Σ m·ℓ_⊥ kept through the meeting.
        let own_c = crate::ring::grain(&(ml_c / m.clone()));
        let phase: Vec<Ratio> = ph.iter().map(|x| crate::ring::grain(&crate::flows::circle_mean(x))).collect();
        // THE PLANETARY CORE (no throat, H = held): the grown body keeps what
        // its core holds; what it releases returns to the system's medium as
        // motes at the body's place. A body whose core holds nothing is no
        // world: its matter stays the medium.
        // THE BODY'S HEAT (its ignition reads it): its members' agitation
        // and the motion their bonds gripped, by holding (a share of the
        // body; each pair's grip already split by the other's mass).
        let heat_g = g.iter().fold(ratio(0, 1), |a, &i| a + mass(&items[i].0) * (heat[i].clone() + items[i].3.clone())) / m.clone();
        // What its strikes gave it, by holding, read into its layers.
        let strike_g = g.iter().fold(ratio(0, 1), |a, &i| a + mass(&items[i].0) * strike[i].clone()) / m.clone();
        // IGNITION OF A GROWN BODY (one accretion path; with xylarium): it
        // becomes a STAR of its system where its own matter binds as a
        // filament, `λ/(1 + λ)` with `λ` its holding and magnetism over its
        // reach, against its agitation and the unlocked face, and passes the
        // ignition test (`2(a + face) < 1`). It keeps its matter whole.
        let (r_body, d_body) = enc.reach(&ell);
        let h_in = enc.hold(&ell);
        let was_star = g.iter().any(|&i| i >= n_motes && items[i].4);
        let ignites = was_star || (kept.len() > 1 && !r_body.is_zero() && {
            let own = (mass(&kept) + crate::pulse::magnetism(&kept)) / total.clone();
            let lambda = own / r_body.clone();
            let face = (ratio(1, 1) - d_body.clone()) / ratio(2, 1);
            lambda.clone() / (ratio(1, 1) + lambda) >= heat_g.clone() + face.clone() && ratio(2, 1) * (heat_g.clone() + face) < ratio(1, 1)
        });
        // Otherwise THE PLANETARY CORE cuts it, its heat the added turning.
        let mut thrown: Vec<(crate::chem::Kind, Ratio)> = Vec::new();
        let hw_body = mass(&kept) / total.clone();
        let (kept, released) = if ignites {
            (kept, Manifest::new())
        } else {
            // A planetary core is cool: its atoms are whole (heat 0). THE
            // PLANET KEEPS ITS LAYERS (with xylarium): of what the core
            // releases, what stands within the edge the star's tide leaves,
            // `1 − e`, stays bound; only the stripped returns to the medium.
            // STRUCK, its layers take the strike by their exposed faces;
            // what no longer stands within the edge is stripped: atmosphere
            // and volatiles first, then crust, by the matter's own holds.
            match heart_cut(&kept, &heat_g, &ratio(0, 1)) {
                Some(_) => {
                    let e = crate::ring::pinch_share(&hw_body, &h_in);
                    let (bound, stripped, carried_out) = bind_struck(&kept, &heat_g, &strike_g, &e);
                    thrown = carried_out;
                    (bound, stripped)
                }
                None => (Manifest::new(), kept),
            }
        };
        // THROWN, NEVER PLACED: what a body releases leaves at its orbit's
        // spin plus the spin its own cut carried it with, from the body's
        // units into the system's (`λ²`, λ its holding share).
        for (k, a) in released {
            if a > ratio(0, 1) {
                let throw = thrown.iter().find(|(c, _)| *c == k).map(|(_, l)| l.clone() * hw_body.clone() * hw_body.clone()).unwrap_or_else(|| ratio(0, 1));
                motes.push(crate::flows::Mote { key: k, amount: a, ell_o: crate::ring::grain(&(ell.clone() + throw)), ell_orth: own_c.clone(), phase: phase.clone(), agitation: crate::ring::grain(&heat_g), born: 0, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1) });
            }
        }
        if grown {
            if ignites {
                fates.1 += 1;
            } else if kept.is_empty() {
                fates.2 += 1;
            } else {
                fates.0 += 1;
                weighs[2] += mass(&kept);
                weighs[3] += nuclei(&kept);
            }
        }
        if kept.is_empty() {
            continue;
        }
        let m = mass(&kept);
        // Its reach and period by the one enclosure; its spin carried exactly
        // (Σ m·ℓ of what met), back into the ledger's units.
        let radius = crate::ring::grain(&(r_body.clone() * scale.clone()));
        let period = crate::ring::period(&r_body, &h_in);
        let spin = crate::ring::grain(&(ell.clone() * dil.clone()));
        // A world that gathered stays that world (its heaviest world's birth
        // reading); a body grown from motes alone is born here.
        let heaviest = g.iter().filter(|&&i| i >= n_motes).max_by(|&&a, &&b| mass(&items[a].0).cmp(&mass(&items[b].0)).then(b.cmp(&a)));
        let path = match heaviest {
            Some(&i) => paths[i - n_motes].clone(),
            None => {
                let siblings: Vec<u64> = worlds.iter().map(|w: &crate::pulse::Body| w.path[sys.path.len()]).chain(paths.iter().filter_map(|p| p.get(sys.path.len()).copied())).collect();
                let mut path = sys.path.clone();
                path.push(crate::ring::birth_key(bell, &r_body, &h_in, &siblings));
                path
            }
        };
        // It keeps the heat of its making (its members' and the bonds'),
        // which sets its thermal band for meeting its neighbours.
        let ell_orth = crate::ring::grain(&(own_c.clone() * dil.clone()));
        worlds.push(crate::pulse::Body { ell_orth, path, radius, period, kept, satellites: Vec::new(), medium: Manifest::new(), ell: spin, medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: m, flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase, load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: crate::ring::grain(&heat_g).min(ratio(1, 1)) });
        if ignites {
            // A star of its system: born marks it (worlds are born 0).
            if let Some(w) = worlds.last_mut() {
                w.born = bell.max(1);
            }
        }
    }
    // The bonds' clusters join the medium's motes before it is read.
    motes.extend(bonded);
    // The medium is what the motes hold.
    let mut medium = Manifest::new();
    for x in &motes {
        *medium.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
    }
    sys.medium = medium;
    sys.flows = motes;
    sys.satellites = worlds;
    // The star gives up what its torques gave its system.
    let identity = if rang {
        let was = sys.turning.clone();
        sys.turning -= paid / h_s.clone();
        Some(h_s * (sys.turning.clone() - was) + motes_moved + worlds_moved)
    } else {
        None
    };
    (sticks, bounces, escaped, identity, fates, weighs, (struck_n, tugs_n))
}


/// COLLISIONS AMONG A GALAXY'S STARS (operator 2026-10-18: "this is how you
/// get stars colliding, or random strange star events"; with xylarium): the
/// one collide law of `collide_system` one tier up, in the galaxy's own units
/// and its one enclosure (a star's spin there its ledger spin over `scale²`,
/// its reach `R = ℓ/H(<ℓ)`). Two stars coincide within each
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
pub fn collide_stars(stars: Vec<crate::pulse::Body>, c_pre: Vec<Ratio>, scale: &Ratio, p_g: &Ratio, g_total: &Ratio, enc: &crate::flows::Enclosed, cross: &mut crate::enfold::Cross) -> (Vec<(crate::pulse::Body, Ratio)>, usize, usize) {
    let planes = crate::flows::PLANES;
    let n = stars.len();
    if n < 2 || scale.is_zero() || g_total.is_zero() {
        return (stars.into_iter().zip(c_pre).collect(), 0, 0);
    }
    let frac = |x: &Ratio| x.clone() - x.floor();
    // The galaxy's frame: a star's spin there is its ledger spin over the
    // frame's dilation, `scale²` (reach and hold both scale with the holding
    // unit), its reach the one enclosure's.
    let dil = scale.clone() * scale.clone();
    let own: Vec<Ratio> = stars.iter().map(|b| b.ell.clone() / dil.clone()).collect();
    let own_c: Vec<Ratio> = stars.iter().map(|b| b.ell_orth.clone() / dil.clone()).collect();
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
            let (inner, outer) = if r[i] < r[j] { (i, j) } else { (j, i) };
            let reach_out = r[inner].clone() * (ratio(1, 1) + abs(&e_c[inner]));
            let reach_in = r[outer].clone() * (ratio(1, 1) - abs(&e_c[outer]).min(ratio(1, 1)));
            reach_out * (ratio(1, 1) + p_g.clone()) >= reach_in * (ratio(1, 1) - p_g.clone().min(ratio(1, 1)))
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
    let swing: Vec<(Ratio, bool)> = (0..n).map(|i| crate::flows::swung(&ell[i], &(ell[i].clone() * e_c[i].clone()), stars[i].phase.get(1).unwrap_or(&ratio(0, 1)))).collect();
    let ell_now: Vec<Ratio> = swing.iter().map(|(l, _)| l.clone()).collect();
    let phase0: Vec<Ratio> = stars.iter().zip(&swing).map(|(b, (_, through))| {
        let p0 = b.phase.first().cloned().unwrap_or_else(|| ratio(0, 1));
        if *through { frac(&(p0 + ratio(1, 2))) } else { p0 }
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
        meetings(&phase0, ell_now, &rate, &pairs, &mut |i, j, c, dl| {
            let s = if g[i].is_zero() || g[j].is_zero() { ratio(0, 1) } else if g[i] < g[j] { g[i].clone() / g[j].clone() } else { g[j].clone() / g[i].clone() };
            // Closeness a share on the sphere: their orbit contact over the
            // echo times their in-step in the other planes.
            let near = (1..planes).fold(c.clone(), |c, k| c * crate::flows::in_step(&stars[i].phase[k], &stars[j].phase[k]));
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
        // The exchange stands on the star's own ledger: its spins carried
        // back by the dilation, its reach read from them.
        if !dl[i].is_zero() {
            let now = own[i].clone() + dl[i].clone();
            stars[i].ell = crate::ring::grain(&(now.clone() * dil.clone()));
            stars[i].radius = crate::ring::grain(&(enc.reach(&now).0 * scale.clone()));
        }
        if !dl_c[i].is_zero() {
            stars[i].ell_orth = crate::ring::grain(&((own_c[i].clone() + dl_c[i].clone()) * dil.clone()));
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
            let ell = by(&|b| b.ell.clone());
            // Its reach read from its spin by the one enclosure.
            let at_ell = ell.clone() / dil.clone();
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
                head.turning += b.turning;
                head.medium_spin += b.medium_spin;
                head.load += b.load;
                head.born = head.born.min(b.born);
                // A merger with a remnant is a remnant.
                head.remnant = head.remnant || b.remnant;
            }
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
