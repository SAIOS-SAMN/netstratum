//! docs/78 era 1, build step 2: THE PULSE AND ITS DEPLOYMENT.
//!
//! The founding manifest (an exact amount of each element) is deployed by one
//! law at every tier (docs/78, "The mass split"): a centre keeps a share of
//! its tier's grains, its satellites take shares of what remains inside out,
//! each at the radius its period and the mass it encloses give
//! (`r = M(<r) · P`, from the turning law ω = M(<r)/r), until the next radius
//! would pass the centre's dominance radius (`r_d = r · m / M(<r)`) or the
//! remainder falls below the grain. What remains is the tier's medium.
//! Every draw is read from the seed along a fixed path; nothing is searched.
//!
//! A centre keeps its tier's share and physics leaves a share unformed (the
//! medium), both laws of the tier, never draws; a body below its minimum is
//! not formed and its share joins the medium.
//!
//! The tiers: the epicenter (no mass) deploys the cores; a core its systems
//! (unit period 60² bells); a star its orbs (unit 60 bells). Masses are in
//! units of the deepest core's hole (`M_core = 1`), radii in its gravitational
//! lengths. A core's place on S^360 is formed at the place step (A1); here
//! it is its draw path.

use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;
use std::collections::BTreeMap;
use std::path::Path;

/// One kind's amount, by its key (`chem`: an atom `Zddd` or a cluster).
pub type Manifest = BTreeMap<crate::chem::Kind, Ratio>;

/// The deployment's grain: ONE NUCLEUS (operator 2026-10-18, the package
/// as an element recipe: "every kind in the recipe is carried, never
/// grain-dropped"). Matter is counted in nuclei; nothing smaller exists, so
/// an amount floors to whole nuclei, exact and bounded, and a kind of a few
/// nuclei still moves, burns and is captured. (Era 0's resource ledger keeps
/// its own 1/1024.)
fn grain(x: &Ratio) -> Ratio {
    nucleus(x)
}

/// An amount floored to whole nuclei (`1024 · 60³⁰` nuclei an amount unit).
pub fn nucleus(x: &Ratio) -> Ratio {
    let grid = nuclei_per_grain() * num_bigint::BigInt::from(1024);
    Ratio::new(x.numer() * &grid / x.denom(), grid)
}

/// A manifest's MASS, exact: its weight, holding and magnetism converged
/// (`chem::weight`, 3Z a nucleus; operator 2026-10-24), what gravity pulls
/// and inertia answers by. Every amount is a whole count of nuclei on the one
/// nucleon denominator and every weight whole, so the sum stays on that
/// denominator however many kinds it adds; conserved exactly by the breath.
pub fn mass(m: &Manifest) -> Ratio {
    m.iter().fold(ratio(0, 1), |s, (k, a)| {
        let weight = crate::chem::weight(k);
        s + a.clone() * ratio(i64::from(weight), 1)
    })
}

/// A manifest's HOLDING, its closed frames alone (the Z pole): what the open
/// frames (`magnetism`, the Q pole) are read against, as a push's open share.
pub fn held(m: &Manifest) -> Ratio {
    m.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * ratio(i64::from(crate::chem::hold(k)), 1))
}

/// The nuclei in one grain of any Z (docs/78, "The atom"): 60^30.
pub fn nuclei_per_grain() -> num_bigint::BigInt {
    num_bigint::BigInt::from(60u32).pow(30)
}

/// Nuclei per Z, as mycelium's MANIFEST carries them (ACT_DOMAIN 17).
pub type Nuclei = BTreeMap<u16, num_bigint::BigInt>;

/// A manifest of nuclei entering: per Z, its count in grains (`Z<n>` keys),
/// every kind carried in whole nuclei however few.
pub fn from_nuclei(nuclei: &Nuclei) -> Manifest {
    let grid = nuclei_per_grain() * num_bigint::BigInt::from(1024);
    nuclei
        .iter()
        .filter(|(_, count)| **count > num_bigint::BigInt::from(0))
        .filter_map(|(z, count)| Some((crate::chem::Kind::atom(u32::from(*z))?, Ratio::new(count.clone(), grid.clone()))))
        .collect()
}

/// THE FOUNDING DISTRIBUTION (docs/78; the capture law, operator 2026-10-16
/// "derive a", with xylarium): the bounce's white-face outflow leaves along
/// the outbound continuum jetstream, and each pass through the throat is ONE
/// CAPTURE, kind Z built from Z − 1 by the share the stream carries on,
/// `count_Z = count_{Z−1} · a · h_Z`. `a` is DERIVED, never drawn: what the
/// throat does not hold at the crossing, its unlocked face, `1 − δ(H, H) =
/// ½` (the same face that heats held matter); `h_Z = (3Z − unpaired)/3Z`,
/// the atom tier's holding share (paired nuclei hold better). From the
/// bounce's nucleons (`count_0 = N`) until a Z falls below one nucleus (no
/// cap). It is the SHAPE the universe requests: the composition of the
/// signed MANIFEST its reserve sends the court (mycelium 78.155b), which
/// records and grants it and never reshapes it.
pub fn founding_shape(nuclei: &num_bigint::BigInt) -> Nuclei {
    let mut out = Nuclei::new();
    let mut count = Ratio::from_integer(nuclei.clone());
    // No cap: the chain runs until a Z falls below one nucleus (the
    // package's recipe ends where the chain does; no Earth table's 118).
    for z in 1u16..=u16::MAX {
        let held = cfe::almanac::atom::atom(u32::from(z)).map(|a| ratio(i64::from(3 * u32::from(z) - a.unpaired), i64::from(3 * u32::from(z)))).unwrap_or_else(|| ratio(1, 1));
        count = count * ratio(1, 2) * held;
        let whole = count.floor().to_integer();
        if whole <= num_bigint::BigInt::from(0) {
            break;
        }
        out.insert(z, whole);
    }
    out
}

/// The seed's fixed paths: draw `j` of `field` for the body at `path`.
pub fn draw(seed: &[u8; 32], path: &[u64], field: &str, j: u32) -> u64 {
    let mut h = blake3::Hasher::new();
    h.update(seed);
    h.update(&(path.len() as u32).to_le_bytes());
    for p in path {
        h.update(&p.to_le_bytes());
    }
    h.update(field.as_bytes());
    h.update(&j.to_le_bytes());
    u64::from_le_bytes(h.finalize().as_bytes()[..8].try_into().expect("8 bytes"))
}

/// A tier's laws (docs/78, "The mass split", amended with xylarium's
/// guidance 2026-10-14): the share its centre keeps, and the share of the
/// rest physics leaves unformed (the medium). Anchors, not draws.
pub struct Tier {
    pub centre: Ratio,
    pub medium: Ratio,
    /// A satellite's tidal reach as a share of its radius, `s = ρ / r`
    /// (docs/78, "The mass split"), rational by its half-angle `t`:
    /// `s = 2t / (1 + t²)`.
    pub reach: Ratio,
}

/// `2t / (1 + t²)`: a rational reach share from a rational half-angle.
fn reach_of(t: Ratio) -> Ratio {
    ratio(2, 1) * t.clone() / (ratio(1, 1) + t.clone() * t)
}

/// By depth: 3 the epicenter (no mass; the intergalactic medium; its cores
/// by the spectrum's strengths, `deploy`), 2 a core
/// (its hole, of order 1/1000 of its galaxy, the hole-to-bulge relation; the
/// interstellar medium, of order 1/10, the Milky Way's gas to stars), 1 a
/// star (99.86% of its system, the Sun's share; belts and comet clouds, of
/// order 1/100 of the rest), 0 an orb (keeps all).
pub fn tier(depth: u8) -> Tier {
    match depth {
        3 => Tier { centre: ratio(0, 1), medium: ratio(1, 2), reach: ratio(0, 1) },
        // Systems about a core: s = 12/37 (t = 1/6), each system about 0.235
        // of the mass it orbits. A placeholder sized to the stand-in's
        // grains (tens of systems a galaxy); a real galaxy's is far smaller.
        2 => Tier { centre: ratio(1, 1000), medium: ratio(1, 10), reach: reach_of(ratio(1, 6)) },
        // Orbs about a star: t = 1/283, each orb about 1e-4 of what it
        // orbits, the order of the Solar System's planets against the Sun.
        1 => Tier { centre: ratio(9986, 10000), medium: ratio(1, 100), reach: reach_of(ratio(1, 283)) },
        _ => Tier { centre: ratio(1, 1), medium: ratio(0, 1), reach: ratio(0, 1) },
    }
}

/// `f` of each element, floored to the grain.
pub(crate) fn take(from: &Manifest, f: &Ratio) -> Manifest {
    from.iter().map(|(k, a)| (k.clone(), grain(&(a.clone() * f.clone())))).filter(|(_, a)| *a > ratio(0, 1)).collect()
}

pub(crate) fn less(from: &mut Manifest, by: &Manifest) {
    for (k, a) in by {
        if let Some(have) = from.get_mut(k) {
            *have -= a.clone();
        }
    }
    from.retain(|_, a| *a > ratio(0, 1));
}

/// A deployed body: what it keeps, its satellites and its medium.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body {
    /// The draw path: `[core]`, `[core, system]`, `[core, system, orb]`.
    pub path: Vec<u64>,
    /// Its radius about its centre and its period (bells a turn); zero for a
    /// core (cores do not orbit).
    pub radius: Ratio,
    pub period: Ratio,
    /// What the body itself holds (a star's, a core's, an orb's grains).
    pub kept: Manifest,
    pub satellites: Vec<Body>,
    /// The tier's unbound grains, beyond its last orbit.
    pub medium: Manifest,
    /// docs/83 Phase 0, THE SPIN LEDGER: the body's specific spin about its
    /// centre, `ℓ = R · H` (reach times the holding inside it,
    /// `crate::ring`); its angular momentum is its mass times ℓ. Zero for a core (it does not
    /// orbit).
    pub ell: Ratio,
    /// The medium's total angular momentum (gas carries spin too).
    pub medium_spin: Ratio,
    /// docs/83 FORMATION: a galactic core's gas as MOTES (each element's
    /// shell where the bounce's push left it, a jet batch where it landed),
    /// in the galaxy's own units; empty for every other body.
    pub gas: Vec<crate::formation::Shell>,
    /// The bell a star was born (its heart formed, docs/83); 0 for bodies
    /// laid down at the founding and for every other body.
    pub born: u64,
    /// A galactic core's STATE (with xylarium, 2026-10-16): set at the
    /// bounce, the throat itself, never drained: its galaxy's grains at the
    /// founding (the state holds one ring as all its grains together did).
    /// The core's HOLDING tracks what it holds, state + grains; an emptied
    /// galaxy keeps its core. Zero for every other body. Tag 9.
    pub state: Ratio,
    /// THE FLOWS (`crate::flows`): a galaxy's matter as motes carrying
    /// their state forward (tag 10 each); empty for every other body.
    pub flows: Vec<crate::flows::Mote>,
    /// The core's turning reservoir, the torques' antipole (tag 11): what it
    /// gave its motes' spin, taken from it, so `Σ m·ℓ + reservoir` holds.
    pub reservoir: Ratio,
    /// A STAR'S OWN SPIN (tag 12; with xylarium): captured at birth from its
    /// members' spread of orbital spin; its bell `ω_s = δ(ℓ_s/H_s)` is the
    /// system's time. Kept EXTENSIVE on the universe's fixed unit, `S =
    /// (kept/unit)·ℓ_s`, a bounded running sum of what it is paid and gives
    /// (the scalar-collapse law: a specific spin re-divided by a share that
    /// moves every echo widened without bound); its specific spin in its
    /// system's units is read from it, `ℓ_s = S·unit/kept` (`star_spin`).
    /// Zero for every other body.
    pub turning: Ratio,
    /// ITS ORTHOGONAL SPIN (tag 21; docs/90): its spin in the orthogonal orbit's plane, the
    /// orthogonal one, in the units of `ell` (its orbit spin): torqued by its
    /// parent's release, returned by its parent's hold, damped by order. Its
    /// reach swings by it each orthogonal turn, `R(ψ_⊥) = (ℓ + ℓ_⊥·response(ψ_⊥))/H`.
    pub ell_orth: Ratio,
    /// A star's phases on its S^n, one per plane (tag 13): its members'
    /// holding-weighted phases at birth, advancing by its bell.
    pub phase: Vec<Ratio>,
    /// A star's LOAD (tag 14; xylarium's operator 2026-10-17): its worlds'
    /// dragged manifolds, `Σ x_k` in its system's own units; it slows the
    /// star's motion through its galaxy, `v = δ_gal·H/(H + d·Σx)`.
    pub load: Ratio,
    /// A star's REFERENCE (tag 15; operator 2026-10-18): its galaxy's own
    /// phase at its place, one per plane, born at the star's phases and
    /// turning as the galaxy's flows do at its reach; the star's coherence is
    /// read against it, so only its spin's slip moves it.
    pub bell: Vec<Ratio>,
    /// A REMNANT (tag 16; the closed limit ruled (a), operator 2026-10-18,
    /// with xylarium): a star whose every rung is spent. It falls to its throat
    /// condition, a small throat whose other face is its galactic core;
    /// `state` then records its CLOSING mass.
    pub remnant: bool,
    /// A world's HEAT (tag 18; operators 2026-10-18, with xylarium): the
    /// agitation of the bonds that built it, cooling to the void through its
    /// unlocked face; its thermal band `r(1 ± heat)` is where it meets its
    /// neighbours (the clearing). Zero for every other body.
    pub heat: Ratio,
    /// ITS TILT (tag 22; docs/96 step 1): its orbit spin's share in each
    /// spatial plane (`flows::Mote::tilt`); empty reads isotropic.
    pub tilt: Vec<Ratio>,
}

impl Body {
    /// Everything it holds: itself, its satellites and its medium.
    pub fn total(&self) -> Manifest {
        let mut all = self.kept.clone();
        for (k, a) in self.satellites.iter().flat_map(|s| s.total()).chain(self.medium.clone()) {
            *all.entry(k).or_insert_with(|| ratio(0, 1)) += a;
        }
        all
    }
}

/// THE CORE HOLDS AS A STATE (docs/83, "The core holds as a state";
/// operator and xylarium, 2026-10-15): a galactic core is a bell whose state
/// holds, on the universal clock, its galaxy's share of the manifest's
/// holding, the same as all its galaxy's grains together. Its holding is
/// that state plus the grains it keeps, so it dominates its galaxy's holding
/// (quantum mass) whatever its 3-D grains. In the grains' holds. The HOME's
/// core is the universal core itself, whose state is the universe's
/// (`universal_state`), counted once there, so the home adds none of its own.
pub fn core_state(core: &Body) -> Ratio {
    if is_home(core) {
        return ratio(0, 1);
    }
    if !core.state.is_zero() {
        return core.state.clone();
    }
    mass(&core.total())
}

/// THE HOME (operator 2026-10-20: "build the home galaxy of the elements
/// authorized around the 'first' core or 'universal core' as the uplink to
/// hypha - where the ancients live"; with xylarium, 11.0 (25)): the galaxy
/// whose core is the universal core, path `[0]`, at reach 0. It is never
/// placed: it is the founding's matter the core's own gravity holds (the
/// beads that cannot hold their spread against it, `deploy`). It shares one
/// enclosure with the firmament: the singularity's state, the firmament, then
/// the home's matter beyond the throat.
pub fn is_home(body: &Body) -> bool {
    body.path.first() == Some(&0)
}

/// The home's push, throat and mean weight, in the universe's own units (its
/// grains' holding 1, the singularity's state 1): the universal bounce's
/// push ½ (the universe tier's, as the firmament's torque), the throat holding
/// the singularity's state and the firmament, and the home's mean weight.
pub fn home_push(u: &Universe, grains: &Manifest) -> Option<(Ratio, Ratio, Ratio)> {
    let whole = mass(&u.total());
    let count = grains.values().fold(ratio(0, 1), |s, a| s + a.clone());
    if whole.is_zero() || count.is_zero() {
        return None;
    }
    let firm = u.firmament.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key)) / whole;
    Some((ratio(1, 2), ratio(1, 1) + firm, mass(grains) / count))
}

/// The universal core's state hold: all the universe's grains together hold
/// exactly one of its rings, so its state equals their total holding.
pub fn universal_state(u: &Universe) -> Ratio {
    mass(&u.total())
}

/// A star's own specific spin in its system's units, read from its
/// extensive spin (`Body::turning`): `ℓ_s = S·unit/kept`.
pub fn star_spin(b: &Body, unit: &Ratio) -> Ratio {
    let k = mass(&b.kept);
    if k.is_zero() {
        return ratio(0, 1);
    }
    b.turning.clone() * unit.clone() / k
}

/// THE HOME'S LEDGER UNIT (the scalar-collapse law: a ledger summed in a
/// unit that moves widens without bound, each term on a new denominator):
/// the universe's 3Z, its holding plus its magnetism, which the breath
/// conserves exactly (burning trades holding for magnetism, never makes
/// either), so it is the same exact value on every read, derived, never
/// stored. The galaxies keep theirs in their founded core state.
pub fn universal_ledger(u: &Universe) -> Ratio {
    mass(&u.total())
}

/// THE DISC'S CUT (docs/79 and docs/83; the push law with xylarium,
/// 2026-10-15): what a galactic core's bounce HOLDS. In the galaxy's own
/// units its core's state holds 1 and its grains together 1; the core's echo
/// reach is its ring, d = 1. The bounce gives every nucleus the same impulse, and its
/// budget is the UNIVERSAL state, re-pulsed down the hub and shared over the
/// galaxy's nuclei, so a nucleus of weight w (`atom::hold`) is carried the
/// spin `ℓ = p·(w̄/w)` (the core's state spin is 1: the throat a ring at d = 1) (w̄ the galaxy's mean weight a nucleus, p the push share
/// `H_disc / (H_core + H_disc)` with `H_disc` the hold of everything heavier),
/// settles at `R = ℓ / H` (the motion law) and is HELD where the deficit
/// there beats the push, `H / (R + H) ≥ p`. Ordered by weight once, the
/// criterion moves monotonically (each heavier nucleus held raises H and p),
/// so the held set is everything above ONE weight w*: a single certified
/// crossing on the cumulative hold, read again only when accretion or
/// shedding changes the manifest. Nothing is stepped. REGENERATION (operator
/// 2026-10-16): each core re-pulses at full strength in its own units, so no
/// share of the universe enters the push; a galaxy's gas comes from outside
/// (the ocean), not from its own bounce.
pub fn galaxy_push(grains: &Manifest) -> Option<(Ratio, Ratio, Ratio)> {
    push_cut(grains).map(|(p, h, mean, _)| (p, h, mean))
}

/// The push and its CUT: as `galaxy_push`, with the lightest kind the
/// throat still holds (None: it holds every kind).
pub fn push_cut(grains: &Manifest) -> Option<(Ratio, Ratio, Ratio, Option<crate::chem::Kind>)> {
    let weight = |k: &crate::chem::Kind| crate::chem::weight(k);
    let total = mass(grains);
    let count = grains.values().fold(ratio(0, 1), |s, a| s + a.clone());
    if total.is_zero() || count.is_zero() {
        return None;
    }
    let mean = total.clone() / count;
    let mut order: Vec<(&crate::chem::Kind, &Ratio)> = grains.iter().collect();
    order.sort_by(|a, b| weight(b.0).cmp(&weight(a.0)).then(a.0.cmp(b.0)));
    // The cumulative hold of everything heavier than each element.
    let above: Vec<Ratio> = order
        .iter()
        .scan(ratio(0, 1), |held, (k, amount)| {
            let before = held.clone();
            *held += (*amount).clone() * ratio(i64::from(weight(k)), 1) / total.clone();
            Some(before)
        })
        .collect();
    let holds = |idx: usize| {
        let held = &above[idx];
        let push = held.clone() / (ratio(1, 1) + held.clone());
        let ell = push.clone() * mean.clone() / ratio(i64::from(weight(order[idx].0)), 1);
        let h = ratio(1, 1) + held.clone();
        h.clone() / (ell / h.clone() + h) >= push
    };
    // The crossing sets the push (transitional: the galaxy tier's p awaits
    // the arriving ocean, docs/83).
    let cut = (0..order.len()).collect::<Vec<_>>().partition_point(|&idx| holds(idx));
    let held = above.get(cut).cloned().unwrap_or_else(|| above.last().cloned().unwrap_or_else(|| ratio(0, 1)) + order.last().map(|(k, a)| (*a).clone() * ratio(i64::from(weight(k)), 1) / total.clone()).unwrap_or_else(|| ratio(0, 1)));
    // EXACT (operator 2026-10-23: "this is approximations leaking into an
    // exact system"; "build the certified intervals"): the readings stand as
    // they are. Where they multiply every mote's torque each bell, the torque
    // holds the push by its bounds on the grain and reads it exactly only
    // where those bounds would store differently (`flows::torque`).
    let h = ratio(1, 1) + held.clone();
    let lightest = if cut >= order.len() { None } else { cut.checked_sub(1).map(|i| order[i].0.clone()) };
    Some((held / h.clone(), h, mean, lightest))
}

/// THE DISPERSAL (operator 2026-10-16: "we shouldn't be sorting anything,
/// we should be dispersing and letting the physics do the distribution").
/// The bounce pushes every kind out and SPREADS it, never sorting it into a
/// slot: kind z is carried `ℓ_z = p·(w̄/w_z)` (regeneration: own units) and stands about
/// `R_z = ℓ_z/H`, spread by the push across `R_z(1 − p) … R_z(1 + p)`. The
/// part of its spread within the core's throat (`R ≤ H`) is HELD (the disc),
/// the rest disperses as gas: every kind partly held and partly free, the
/// heavy leaning inward, the light outward, the disc and the gas both mixed.
/// The distribution keeps moving after (the waves, the breath): this is
/// only where the founding pulse leaves it.
pub fn disperse(grains: &Manifest) -> Manifest {
    let weight = |k: &crate::chem::Kind| crate::chem::weight(k);
    let Some((p, h, mean)) = galaxy_push(grains) else { return Manifest::new() };
    grains
        .iter()
        .filter_map(|(k, amount)| {
            let ell = p.clone() * mean.clone() / ratio(i64::from(weight(k)), 1);
            let r = ell / h.clone();
            let lo = r.clone() * (ratio(1, 1) - p.clone());
            let span = ratio(2, 1) * p.clone() * r;
            let f = if span.is_zero() { if lo <= h { ratio(1, 1) } else { ratio(0, 1) } } else { ((h.clone() - lo) / span).max(ratio(0, 1)).min(ratio(1, 1)) };
            let held = grain(&(amount.clone() * f));
            (held > ratio(0, 1)).then(|| (k.clone(), held))
        })
        .collect()
}

/// The universe the pulse deploys: its cores, and the intergalactic medium.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universe {
    pub cores: Vec<Body>,
    /// THE OCEAN OF SPACE (operator 2026-10-25: "this needs to be aligned on
    /// 'jetstreams' of vacuum/continuum via the ocean of space"; "build the
    /// ocean as motes"; with xylarium): the medium between galaxies as
    /// MOTES, each a parcel of matter carrying its state, in transit on the
    /// vacuum leg home, never at rest. Its manifest is a reading
    /// (`Universe::medium`). Tag 24, one mote a record.
    pub ocean: Vec<crate::flows::Mote>,
    /// THE FIRMAMENT (operator 2026-10-19: "the heaviest elements probably
    /// dont move that much, and form an orderly 'container' around the
    /// 'bouncing antipoles' ... the pulses over time ... should move the
    /// firmament into order ... derived"; with xylarium, (A)): the kinds
    /// the founding's Wellspring holds stay at the singularity as motes, each
    /// at its capture phase, torqued by the universal bounce each echo by its
    /// own inertia (the heaviest barely move). What order its phases gather
    /// into is a reading, never set. Tag 19, one mote a record.
    pub firmament: Vec<crate::flows::Mote>,
    /// The unit of mass: the deepest core's hole AT THE FOUNDING. A hole is
    /// set by the bounce and does not drain when its disc jets away
    /// (regeneration; with xylarium: the unit is the core's state, never its
    /// current disc). Tag 8.
    pub unit: Ratio,
    /// THE UNIVERSAL BELL'S PERIOD, in bells (tag 17; operator 2026-10-18,
    /// derived): the echo bubble's round trip at the founding, a ring out to
    /// the outermost shell and back, held by the throat on the way, `2(R + H)`
    /// whole bells. "The bounce grows, the echo grows ... to the steady
    /// implosion of the echo bubble." Saves from before read the stand-in.
    pub t_u: u64,
}

/// THE PULSE: the manifest deployed from `seed`. The unit of mass is the
/// deepest core's hole.
/// THE FOUNDING CASCADE (operators 2026-10-18, xylarium 11.0 (15), the
/// equitable formula; "the unlocked face - it's a hemisphere, keep it"). The
/// stack builds by the capture chain (Z ascending); each capture's share is
/// the unlocked face at its own event, `s = ½·R/(R + H)` with `R = held` so
/// far and `H = 1 + held` (a hemisphere, never a static ½). The epicentre's
/// spin is their sum, `K = Σ s`. BURSTS empty the stack from the top
/// (heaviest first) on one hemisphere and the EXPANSION fills from the base
/// (lightest first) on the other, so every kind is pushed by `K + s_i`, the
/// whole spin plus its STACK's share once (the stacks the weight profile's
/// beads, heaviest first), and carried `ℓ_z = (K + s_i)·w̄/w_z`
/// to `R_z = ℓ_z/H` (w̄ the amount-weighted mean weight). A kind whose spread
/// stays within the throat, `R_z(1 + s_final) ≤ 1`, is HELD by the
/// Wellspring; the rest stand in expanding shells at their own reaches, by
/// weight ("distributed by weight orthogonally - think of a sphere
/// expanding"). Returns (the Wellspring's held kinds, the GALAXIES: each a
/// bead of the shells, (its pinch's reach in throat radii, its kinds)).
/// No draws: the galaxies are the shells' own beads.
pub fn cascade(manifest: &Manifest) -> (Manifest, Vec<(Ratio, Manifest)>, Ratio) {
    // ON THE GRAIN, OWED EXACT (operators 2026-10-24, with xylarium: "we
    // cannot have approximations"): the filter no longer nests its means (it
    // reads the release, below), but the exact spin K sums ~200 distinct
    // denominators and every reach carries it, so the passes are owed as
    // certified readings (design law 11) before the grain comes off.
    let g = |x: &Ratio| crate::ring::grain(x);
    let w = |k: &crate::chem::Kind| ratio(i64::from(crate::chem::weight(k)), 1);
    let total = mass(manifest);
    let count = manifest.values().fold(ratio(0, 1), |s, a| s + a.clone());
    if total.is_zero() || count.is_zero() {
        return (Manifest::new(), Vec::new(), ratio(2, 1));
    }
    let _ = count;
    let mut kinds: Vec<(crate::chem::Kind, Ratio)> = manifest.iter().filter(|(_, a)| **a > ratio(0, 1)).map(|(k, a)| (k.clone(), a.clone())).collect();
    kinds.sort_by_key(|(k, _)| crate::chem::charge(k));
    let face = |held: &Ratio| g(&(ratio(1, 2) * held.clone() / (held.clone() + ratio(1, 1) + held.clone())));
    let mut held = ratio(0, 1);
    let mut share = Vec::with_capacity(kinds.len());
    for (k, a) in &kinds {
        share.push(face(&held));
        held = g(&(held + a.clone() * w(k) / total.clone()));
    }
    let spin = share.iter().fold(ratio(0, 1), |a, b| a + b.clone());
    let h = ratio(1, 1) + held.clone();
    let s_final = face(&held);
    // THE STACKS (11.0 (15), with xylarium 2026-10-24: "the stack's own BEADS
    // by weight (the beading law at the founding), never sized"): the kinds
    // heaviest first (ties in capture order), the weight profile `amount·w`
    // cut at its local minima, each run between cuts a stack. A stack's share
    // is the sum of its kinds' capture shares, and every kind in it feels the
    // bursts from the top and the expansion from the base together, `B_i +
    // E_i = K + s_i`.
    let mut heavy: Vec<usize> = (0..kinds.len()).collect();
    heavy.sort_by(|&a, &b| w(&kinds[b].0).cmp(&w(&kinds[a].0)));
    let prof: Vec<Ratio> = heavy.iter().map(|&i| kinds[i].1.clone() * w(&kinds[i].0)).collect();
    let mut stack_share = vec![ratio(0, 1); kinds.len()];
    let mut from = 0usize;
    let close = |from: usize, to: usize, stack_share: &mut Vec<Ratio>| {
        let s_i = heavy[from..to].iter().fold(ratio(0, 1), |a, &i| a + share[i].clone());
        for &i in &heavy[from..to] {
            stack_share[i] = s_i.clone();
        }
    };
    for k in 1..prof.len().saturating_sub(1) {
        if prof[k] < prof[k - 1] && prof[k] <= prof[k + 1] {
            close(from, k, &mut stack_share);
            from = k;
        }
    }
    close(from, prof.len(), &mut stack_share);
    // Every kind leaves as a SHELL: a mix at one reach (with xylarium: shells
    // as mixes, so a passage adds to a mix, never a new shell).
    let mut shells: Vec<(Manifest, Ratio)> = kinds
        .into_iter()
        .zip(stack_share)
        .map(|((k, a), s)| {
            // Element-agnostic (operator 2026-10-18: "the push must be agnostic
            // to the element by anything other than the force produced by the
            // bounces of the antipoles ... inertia is fine"): the one force
            // `K + s`, each kind moving by its own inertia, never against the
            // population's mean weight; `s` its stack's share.
            let reach = g(&((spin.clone() + s) / w(&k) / h.clone()));
            ([(k, a)].into_iter().collect::<Manifest>(), reach)
        })
        .collect();
    // Released heaviest first (deepest reach first).
    shells.sort_by(|a, b| a.1.cmp(&b.1));
    // THE FILTER (operators 2026-10-18, (6) "filtered through the heaviest of
    // heavy"; with xylarium, 11.0 (15): "every later, lighter stack passes OUT
    // THROUGH the heavy stacks that stay in - each passage a MEETING"): each
    // lighter shell passes out through every heavier shell inside its reach,
    // each passage a meeting by the one law. Shells are isotropic, so a whole
    // passage is contact ½ and a passage ending within the other's spread is
    // ½ of the share crossed. The grip split as at every tier (s the
    // tightness ratio, `D = M/(M + 1 − contact)`, T the deficits' contrast):
    // the gripped fraction `g = min(s·D, T)/T` is CAUGHT, and the heavier
    // shell gives `contact·g` times the lighter's mass share of its mix, which
    // rides out with the lighter shell.
    // ALL AT ONCE, AGAINST THE RELEASE (operator 2026-10-24, with xylarium;
    // the design law: no fill orders; the scalar-collapse law: a reach re-
    // averaged after each catch nests its means and its exact width explodes):
    // every passage reads both shells as released; where lighter shells take
    // more of a kind than a shell holds, every take of it is scaled by what it
    // holds over what is asked, so none gives more than it has; each shell's reach is then ONE holding-
    // weighted mean of its parts' release reaches (`Σ m·ℓ` unchanged by the
    // drag). Matter caught is not caught again in the same founding.
    let delta = |r: &Ratio| h.clone() / (r.clone() + h.clone());
    let lo = |r: &Ratio| r.clone() * (ratio(1, 1) - s_final.clone());
    let hi = |r: &Ratio| r.clone() * (ratio(1, 1) + s_final.clone());
    let released = shells.clone();
    // (i catching, j giving, the share of j's mix i takes).
    let mut passes: Vec<(usize, usize, Ratio)> = Vec::new();
    // Each released shell read once: its share, grip, deficit and spread.
    let m_rel: Vec<Ratio> = released.iter().map(|(m, _)| mass(m) / total.clone()).collect();
    let g_rel: Vec<Ratio> = released.iter().map(|(m, _)| crate::formation::grip(m)).collect();
    let d_rel: Vec<Ratio> = released.iter().map(|(_, r)| delta(r)).collect();
    let band: Vec<(Ratio, Ratio)> = released.iter().map(|(_, r)| (lo(r), hi(r))).collect();
    for i in 1..released.len() {
        let (mi, ri) = (m_rel[i].clone(), released[i].1.clone());
        for j in 0..i {
            let mj = m_rel[j].clone();
            let (lo_j, hi_j) = (&band[j].0, &band[j].1);
            if mj.is_zero() || mi.is_zero() || ri <= *lo_j {
                continue;
            }
            let crossed = if ri >= *hi_j { ratio(1, 1) } else { (ri.clone() - lo_j.clone()) / (hi_j.clone() - lo_j.clone()) };
            let contact = ratio(1, 2) * crossed;
            let (gi, gj) = (g_rel[i].clone(), g_rel[j].clone());
            let s = if gi.is_zero() || gj.is_zero() { ratio(0, 1) } else if gi < gj { gi / gj } else { gj / gi };
            let m_ij = mi.clone() + mj.clone();
            let bind = s * m_ij.clone() / (m_ij.clone() + ratio(1, 1) - contact.clone());
            let (di, dj) = (d_rel[i].clone(), d_rel[j].clone());
            let t = (if di > dj { di.clone() - dj.clone() } else { dj.clone() - di.clone() }) / (di + dj);
            if t.is_zero() {
                continue;
            }
            let gripped = bind.min(t.clone()) / t;
            let fraction = contact * gripped * mi.clone() / m_ij;
            if fraction > ratio(0, 1) {
                passes.push((i, j, fraction));
            }
        }
    }
    // Every take stands on the nucleus grid as read; where the takes from one
    // shell ask more of a kind than it holds, each is scaled by what it holds
    // over what is asked (both on the grid, so the sum stays exact), so none
    // gives more than it has.
    let takes: Vec<(usize, usize, Manifest)> = passes.into_iter().map(|(i, j, f)| {
        let take: Manifest = released[j].0.iter().map(|(k, a)| (k.clone(), nucleus(&(a.clone() * f.clone())).min(a.clone()))).filter(|(_, a)| *a > ratio(0, 1)).collect();
        (i, j, take)
    }).filter(|(_, _, t)| !t.is_empty()).collect();
    let mut asked: Vec<Manifest> = vec![Manifest::new(); released.len()];
    for (_, j, t) in &takes {
        for (k, a) in t {
            *asked[*j].entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
        }
    }
    // Each shell's parts: (its mass, its release reach), its own kept first.
    let mut parts: Vec<Vec<(Ratio, Ratio)>> = vec![Vec::new(); released.len()];
    let mut given: Vec<Manifest> = vec![Manifest::new(); released.len()];
    let mut caught: Vec<Manifest> = vec![Manifest::new(); released.len()];
    for (i, j, t) in takes {
        let take: Manifest = t.into_iter().map(|(k, a)| {
            let has = released[j].0.get(&k).cloned().unwrap_or_else(|| ratio(0, 1));
            let all = asked[j].get(&k).cloned().unwrap_or_else(|| ratio(0, 1));
            let a = if all > has && !all.is_zero() { nucleus(&(a * has / all)) } else { a };
            (k, a)
        }).filter(|(_, a)| *a > ratio(0, 1)).collect();
        if take.is_empty() {
            continue;
        }
        parts[i].push((mass(&take), released[j].1.clone()));
        for (k, a) in take {
            *given[j].entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
            *caught[i].entry(k).or_insert_with(|| ratio(0, 1)) += a;
        }
    }
    for i in 0..shells.len() {
        less(&mut shells[i].0, &given[i]);
        let own = mass(&shells[i].0);
        for (k, a) in std::mem::take(&mut caught[i]) {
            *shells[i].0.entry(k).or_insert_with(|| ratio(0, 1)) += a;
        }
        let (m, ml) = parts[i].iter().fold((own.clone(), own * released[i].1.clone()), |(m, ml), (x, r)| (m + x.clone(), ml + x.clone() * r.clone()));
        if !m.is_zero() {
            shells[i].1 = g(&(ml / m));
        }
    }
    let mut wellspring = Manifest::new();
    let vessel = cfe::almanac::atom::vessel();
    let mut out: Vec<(Manifest, Ratio)> = Vec::new();
    for (m, reach) in shells {
        if m.is_empty() {
            continue;
        }
        // THE VESSEL SET BY THE GRIP (operator 2026-10-20, with xylarium):
        // the Wellspring keeps, of what its throat holds by reach, only the
        // kinds no passing shell can grip (`atom::vessel`, the heaviest seam
        // to the third sealed shell); whatever else it holds is grippable and
        // goes out with the expelled matter, in its shell.
        if reach.clone() * (ratio(1, 1) + s_final.clone()) <= ratio(1, 1) {
            let (kept, rest): (Manifest, Manifest) = m.into_iter().partition(|(k, _)| {
                crate::chem::atom_z(k).is_some_and(|z| vessel.is_some_and(|(lo, hi)| z >= lo && z <= hi))
            });
            for (k, a) in kept {
                *wellspring.entry(k).or_insert_with(|| ratio(0, 1)) += a;
            }
            if !rest.is_empty() {
                out.push((rest, reach));
            }
        } else {
            out.push((m, reach));
        }
    }
    // THE SHELLS BEAD INTO GALAXIES: shells whose spreads `R(1 ± s_final)`
    // overlap link (transitively); along each, the superposed hold pinches at
    // its maxima and parts at its minima; each bead is a galaxy at its pinch,
    // its mixes SUMMED (a kind arriving twice is added, never overwritten).
    out.sort_by(|a, b| a.1.cmp(&b.1));
    let mut runs: Vec<Vec<usize>> = Vec::new();
    let mut edge = ratio(0, 1);
    for (i, (_, r)) in out.iter().enumerate() {
        if runs.is_empty() || lo(r) > edge {
            runs.push(Vec::new());
        }
        edge = edge.max(hi(r));
        runs.last_mut().expect("pushed").push(i);
    }
    // A spread of nothing held cannot bead (s_final is 0 only then).
    if s_final.is_zero() {
        return (wellspring, Vec::new(), ratio(2, 1) * h);
    }
    let density: Vec<Ratio> = out.iter().map(|(m, _)| mass(m) / total.clone() / (ratio(2, 1) * s_final.clone())).collect();
    let mut galaxies = Vec::new();
    for run in runs {
        let local: Vec<Ratio> = run.iter().map(|&i| run.iter().filter(|&&j| lo(&out[j].1) <= out[i].1 && out[i].1 <= hi(&out[j].1)).fold(ratio(0, 1), |s, &j| s + density[j].clone())).collect();
        let mut cuts = Vec::new();
        for i in 1..run.len().saturating_sub(1) {
            if local[i] < local[i - 1] && local[i] <= local[i + 1] {
                cuts.push(if local[i - 1] >= local[i + 1] { i + 1 } else { i });
            }
        }
        let mut from = 0;
        let mut bead = |a: usize, b: usize| {
            let peak = (a..b).max_by(|x, y| local[*x].cmp(&local[*y]).then(y.cmp(x))).expect("a shell");
            let mut m = Manifest::new();
            for &i in &run[a..b] {
                for (k, x) in &out[i].0 {
                    *m.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += x.clone();
                }
            }
            galaxies.push((out[run[peak]].1.clone(), m));
        };
        for at in cuts {
            if at > from && at < run.len() {
                bead(from, at);
                from = at;
            }
        }
        bead(from, run.len());
    }
    // THE BUBBLE'S ECHO: a ring out to the outermost shell and back, held by
    // the throat on the way, `2(R + H)`.
    let outermost = out.iter().map(|(_, r)| r.clone()).max().unwrap_or_else(|| ratio(0, 1));
    (wellspring, galaxies, ratio(2, 1) * (outermost + h))
}

pub fn deploy(seed: &[u8; 32], manifest: &Manifest) -> Option<Universe> {
    // THE FOUNDING CASCADE (above): the Wellspring holds what stays within its
    // throat; its held kinds are the universe's medium about it (the central
    // galaxy's cut is read from them once the ocean is motes: owed); every
    // galaxy is a bead of the expanding shells, at its pinch's reach. No
    // draws. A bead too slight to hold a hole of a nucleus stays medium.
    let (wellspring, beads, echo) = cascade(manifest);
    // The bell's period in whole bells; a core ring is a quarter of it, so a
    // founding whose echo cannot carry one ring a quarter is refused.
    let t_u: u64 = echo.ceil().to_integer().try_into().ok().filter(|t: &u64| *t >= 4)?;
    // THE FIRMAMENT: the Wellspring's held kinds stay at the singularity as
    // motes at their capture phases (`flows::founding`), never poured into
    // the ocean.
    let firmament = if wellspring.is_empty() { Vec::new() } else { crate::flows::founding(&wellspring, None)? };
    // Beads too slight to hold a hole of a nucleus stay the OCEAN: motes at
    // their beads' reach, on the vacuum leg (set down below, once the
    // galaxies inside them are known).
    let mut slight: Vec<(Ratio, Manifest)> = Vec::new();
    let whole = mass(manifest);
    let mut shares: Vec<(u32, Manifest, Ratio)> = Vec::new();
    for (i, (d, m)) in beads.into_iter().enumerate() {
        if take(&m, &tier(2).centre).is_empty() {
            slight.push((d, m));
            continue;
        }
        shares.push((i as u32 + 1, m, d));
    }
    let weight_of = |m: &Manifest| if whole.is_zero() { ratio(0, 1) } else { mass(m) / whole.clone() };
    // The singularity's state (1) and the firmament it holds (its holding
    // share, as matter).
    let firm = if whole.is_zero() { ratio(0, 1) } else { firmament.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key)) / whole.clone() };
    // A BEAD STANDS AS A GALAXY ONLY WHERE IT HOLDS ITS OWN (with xylarium,
    // 11.0 (25); gravity's reach, one grammar at every tier, operator
    // 2026-10-18): the pinch between the universal core and a bead at reach R
    // is `d = R·H_b/(H_b + H_inside)`. H_b is the bead's OWN matter, w (a
    // bead not yet standing has no throat, as a clump binds by its own λw);
    // H_inside is the enclosed hold, the singularity's state 1, the
    // firmament, and every galaxy standing inside at its standing hold 2w.
    // Its spread `R·s_final` (s_final = ½·1/(1 + 2) = 1/6, the cascade's last
    // face with all of it held) must lie within d. A bead that cannot hold its
    // spread is the universal core's own: its matter is the HOME.
    let mut home = Manifest::new();
    {
        let s_final = ratio(1, 6);
        shares.sort_by(|a, b| a.2.cmp(&b.2));
        let mut inside = ratio(1, 1) + firm.clone();
        let mut kept: Vec<(u32, Manifest, Ratio)> = Vec::new();
        for (i, m, d) in std::mem::take(&mut shares) {
            let w = weight_of(&m);
            let region = d.clone() * w.clone() / (w.clone() + inside.clone());
            if d.clone() * s_final.clone() <= region {
                inside += ratio(2, 1) * w;
                kept.push((i, m, d));
            } else {
                for (k, a) in m {
                    *home.entry(k).or_insert_with(|| ratio(0, 1)) += a;
                }
            }
        }
        shares = kept;
    }
    // THE BIG BANG POPULATES THE UNIVERSE (docs/78): each core keeps its disc,
    // and its galaxy's first generation of star systems and their planets is
    // laid down by the tidal law; what forms no system is the galaxy's medium,
    // gas the breath later forms into new systems. A core stands where the cut
    // one tier up holds it (above); its echoes arrive that many bells later.
    // Each core keeps what its bounce holds (`disperse`), in its own units.
    let discs: Vec<Manifest> = shares.iter().map(|(_, m, _)| disperse(m)).collect();
    // The unit is the deepest galactic hole; with no galaxy standing, the
    // home's.
    let unit = discs.iter().map(mass).max().filter(|m| !m.is_zero()).or_else(|| Some(mass(&disperse(&home))).filter(|m| !m.is_zero()))?;
    // FORMATION (docs/83): no system is placed. Each core keeps its disc;
    // the rest is its galaxy's GAS, laid down as motes where the push left
    // each element; stars collapse from it as it cools (`breath`).
    // THE GALAXIES ORBIT THE UNIVERSAL SINGULARITY (operator): inside out,
    // each about the universal state plus every core inside it (each core
    // its galaxy's state and grains, 2·w_j), by the motion law, in core
    // rings; a core ring is a quarter of the bell's period in bells (the universe's own echo is the
    // round trip to its rim 2). The first ring is the deepest well.
    let mut order: Vec<usize> = (0..shares.len()).collect();
    order.sort_by(|&a, &b| shares[a].2.cmp(&shares[b].2));
    let mut orbits: Vec<(Ratio, Ratio, Ratio)> = vec![(ratio(0, 1), ratio(0, 1), ratio(0, 1)); shares.len()];
    // Inside every galaxy: the singularity's state, the firmament, and the
    // home's matter (its state is the singularity's, counted once).
    let mut inside = ratio(1, 1) + firm.clone() + weight_of(&home);
    let bells = ratio(t_u as i64, 4);
    let mut k = 0;
    while k < order.len() {
        // A ring: every core at the same reach orbits about what is inside it.
        let d = shares[order[k]].2.clone();
        let ring: Vec<usize> = order[k..].iter().copied().take_while(|&j| shares[j].2 == d).collect();
        for &j in &ring {
            orbits[j] = (d.clone() * bells.clone(), crate::ring::period(&d, &inside) * bells.clone(), crate::ring::spin(&d, &inside));
        }
        for &j in &ring {
            inside += ratio(2, 1) * weight_of(&shares[j].1);
        }
        k += ring.len();
    }
    // THE OCEAN at the founding: each slight bead's kinds as motes at its
    // reach, turning at the balance of that place about the singularity and
    // every galaxy inside it (the galaxies' own law), on the vacuum leg.
    let base_in = ratio(1, 1) + firm.clone() + weight_of(&home);
    let mut ocean: Vec<crate::flows::Mote> = Vec::new();
    for (d, m) in slight {
        let inside = shares.iter().filter(|(_, _, dj)| *dj < d).fold(base_in.clone(), |a, (_, mj, _)| a + ratio(2, 1) * weight_of(mj));
        let ell = crate::ring::spin(&d, &inside);
        for (k, a) in m {
            if a > ratio(0, 1) {
                ocean.push(crate::flows::Mote { key: k, amount: a, ell_o: crate::ring::grain(&ell), ell_orth: ratio(0, 1), phase: crate::flows::rest(ratio(1, 2)), agitation: ratio(0, 1), born: 0, vacuum: true, lever: Vec::new(), lever_place: ratio(0, 1), tilt: Vec::new() });
            }
        }
    }
    let mut cores = shares
        .into_iter()
        .zip(discs)
        .zip(orbits)
        .map(|(((i, m, _), _disc), (radius, period, ell))| {
            // THE FLOWS (`crate::flows`): the galaxy's matter as motes,
            // each kind carried by the bounce's push with the founding's
            // phases; what is held and what is gas is READ from their state
            // (the cut as a reading), never placed.
            // A galaxy's state is its grains: its disc spans the ring to 2.
            let flows = crate::flows::founding(&m, Some(&ratio(1, 1)))?;
            let (kept, medium) = match galaxy_push(&m) {
                // By its one enclosure as founded: the throat 1, each mote
                // by its share of the galaxy's holding at its own spin.
                Some((p, _, _)) => {
                    let g = mass(&m);
                    let enc = crate::flows::Enclosed::new(ratio(1, 1), flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g.clone())).collect());
                    crate::flows::read_cut(&flows, &enc, &p)
                }
                None => (m.clone(), Manifest::new()),
            };
            Some(Body { ell_orth: ratio(0, 1), path: vec![u64::from(i)], radius, period, kept, satellites: Vec::new(), medium, ell, medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: mass(&m), flows, reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: ratio(0, 1), tilt: Vec::new() })
        })
        .collect::<Option<Vec<Body>>>()?;
    // THE HOME at the universal core: its matter as motes, all of it gas
    // beyond the throat (the throat's held part is the firmament).
    if !home.is_empty() {
        // The home's state is the universe's: its disc spans its own share.
        let home_span = if whole.is_zero() { ratio(0, 1) } else { mass(&home) / whole.clone() };
        let flows = crate::flows::founding(&home, Some(&home_span))?;
        cores.insert(0, Body { ell_orth: ratio(0, 1), path: vec![0], radius: ratio(0, 1), period: ratio(0, 1), kept: Manifest::new(), satellites: Vec::new(), medium: home, ell: ratio(0, 1), medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: ratio(0, 1), flows, reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: ratio(0, 1), tilt: Vec::new() });
    }
    // THE BOARD (`crate::board`; operator 2026-10-25: "lets just place this
    // down"): the founding's first star systems and their worlds, placed from
    // the galaxies' own grains.
    let mut u = Universe { cores, ocean, firmament, unit, t_u };
    crate::board::place(&mut u, seed);
    Some(u)
}

/// THE GALAXIES ORBIT, DERIVED (operator 2026-10-19: "fixed reach and
/// period - needs to be derived ... if you still have fixed reach - then you
/// dont have gravitational interconnections between the universes core and
/// the galactic cores"; with xylarium). Each universal echo, read inside out
/// by spin, every galactic core stands where its spin meets the holding
/// inside it, `R = ℓ/H` in core rings (the motion law, as at the founding):
/// `H` the singularity's state (1) plus every core inside (`2·w`, w its
/// share of the universe's holding, LIVE). Its period follows, `P = 1 + R/H`
/// (in bells, a quarter of the bell's period a ring), and its orbit's phase advances by the echo's
/// four rings at its deficit, `4·δ`, `δ = H/(R + H)`. Its specific spin is
/// kept, so a galaxy moves only as the holding inside it changes: what one
/// gains or loses moves every galaxy outside it. Echo delays follow the
/// live reach. Returns the cross and the firmament torque's identity residual
/// (its spin moved plus the singularity's reservoir moved: zero where paid
/// exactly).
/// THE OCEAN'S FRAME: the universe's enclosure as the galaxies' orbits read
/// it (`orbit_galaxies`), in core rings: the singularity's state 1, the
/// firmament, the home's matter at the centre (its state is the
/// singularity's, counted once), every other galaxy at its orbit spin by
/// `2w` (its state and its grains), and the ocean's own motes, each a share
/// of the universe's mass. An ocean mote's reach is read from it, `R =
/// ℓ/H(<ℓ)`, in the same rings as a galaxy's.
pub fn ocean_frame(u: &Universe) -> Option<crate::flows::Enclosed> {
    let whole = mass(&u.total());
    if whole.is_zero() {
        return None;
    }
    let firm = u.firmament.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key)) / whole.clone();
    let home: Ratio = u.cores.iter().filter(|c| is_home(c)).fold(ratio(0, 1), |s, c| s + mass(&c.total()) / whole.clone());
    let bodies = u.cores.iter().filter(|c| !is_home(c)).map(|c| (c.ell.clone(), ratio(2, 1) * mass(&c.total()) / whole.clone()))
        .chain(u.ocean.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / whole.clone())))
        .collect();
    Some(crate::flows::Enclosed::new(ratio(1, 1) + firm + home, bodies))
}

pub fn orbit_galaxies(u: &mut Universe) -> (crate::enfold::Cross, Option<Ratio>) {
    let mut cross = crate::enfold::Cross::default();
    let whole = mass(&u.total());
    if whole.is_zero() {
        return (cross, None);
    }
    let bells = ratio(u.t_u as i64, 4);
    let planes = crate::flows::PLANES;
    let mut order: Vec<usize> = (0..u.cores.len()).collect();
    order.sort_by(|&a, &b| u.cores[a].ell.cmp(&u.cores[b].ell).then(a.cmp(&b)));
    // THE FIRMAMENT'S TORQUE (with xylarium): the universal bounce torques it
    // each echo, at full amplitude (½), about the ENCLOSED hold (the
    // singularity's state, 1, plus every firmament mote strictly inside),
    // each kind by its own inertia: the one torque law (`flows::torque`), its band
    // the bounce's push on the firmament's open frames (`½ ×` its open share)
    // and the singularity's hold drawing each mote back by `½·δ(R)·ℓ`
    // (operator 2026-10-19, "the missing antipole"), so each kind settles
    // parted by its inertia, never placed; through the centre where a torque
    // passes it. PAID BY THE SINGULARITY: the home stands at the source (its
    // state is the singularity's) and keeps its reservoir in the universe's
    // own units, the firmament's, so what the torques give the firmament the
    // home's reservoir gives up (`Σ m·ℓ + reservoir` exact).
    let mut identity = None;
    // The singularity's reservoir and the firmament's spins are booked in the
    // home's fixed ledger unit; the hold is read in shares of the whole.
    let ledger_u = universal_ledger(u);
    if !u.firmament.is_empty() {
        let held = |x: &crate::flows::Mote| x.amount.clone() * crate::flows::weight(&x.key) / whole.clone();
        let booked = |x: &crate::flows::Mote| x.amount.clone() * crate::flows::weight(&x.key) / ledger_u.clone();
        let enc = crate::flows::Enclosed::new(ratio(1, 1), u.firmament.iter().map(|x| (x.ell_o.clone(), held(x))).collect());
        let fm = u.firmament.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key));
        let fo = u.firmament.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * ratio(i64::from(crate::chem::unpaired(&x.key)), 1));
        let push = if fm.is_zero() { None } else { Some(ratio(1, 2) * fo / fm) };
        let ledger = |firm: &[crate::flows::Mote]| firm.iter().fold(ratio(0, 1), |s, x| s + booked(x) * (x.ell_o.clone() + x.ell_orth.clone()));
        let before = ledger(&u.firmament);
        // The firmament stands in the source's own enclosure: it feels the
        // pulse at its own reach (`flows::turn`, no tide).
        let (given, _) = crate::flows::torque(&mut u.firmament, &enc, &ratio(1, 2), None, &ledger_u, push.as_ref(), &mut cross);
        let moved = ledger(&u.firmament) - before;
        if let Some(home) = u.cores.iter_mut().find(|c| is_home(c)) {
            let was = home.reservoir.clone();
            home.reservoir -= given;
            identity = Some(moved + (home.reservoir.clone() - was));
        }
        // Its order: matter pushing matter apart by its open frames
        // (`flows::repel`), at the bounce's push.
        crate::flows::repel(&mut u.firmament, &ratio(1, 2));
    }
    // THE OCEAN IN TRANSIT (operator 2026-10-25: "jetstreams of
    // vacuum/continuum via the ocean of space"; never at rest): the
    // universal bounce torques every ocean mote by the one law about the
    // ocean's frame (the band held back, as for a galaxy's motes), its
    // vacuum-leg motes at their inverted reach; the singularity's reservoir,
    // the home's, pays (identity with the firmament's).
    if !u.ocean.is_empty() {
        if let Some(enc_o) = ocean_frame(u) {
            let booked = |x: &crate::flows::Mote| x.amount.clone() * crate::flows::weight(&x.key) / ledger_u.clone();
            let ledger = |sea: &[crate::flows::Mote]| sea.iter().fold(ratio(0, 1), |s, x| s + booked(x) * (x.ell_o.clone() + x.ell_orth.clone()));
            let before = ledger(&u.ocean);
            let (given, _) = crate::flows::torque(&mut u.ocean, &enc_o, &ratio(1, 2), None, &ledger_u, None, &mut cross);
            let moved = ledger(&u.ocean) - before;
            if let Some(home) = u.cores.iter_mut().find(|c| is_home(c)) {
                let was = home.reservoir.clone();
                home.reservoir -= given;
                let r = moved + (home.reservoir.clone() - was);
                identity = Some(identity.unwrap_or_else(|| ratio(0, 1)) + r);
            }
        }
    }
    // The singularity's state (1) and the firmament it holds (its holding
    // share, as matter: only a throat counts twice).
    let firm = u.firmament.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key)) / whole.clone();
    let mut inside = ratio(1, 1) + firm;
    for i in order {
        let share = mass(&u.cores[i].total()) / whole.clone();
        let c = &mut u.cores[i];
        // The home stands at the universal core: it adds its matter to the
        // hold inside every galaxy (its state is the singularity's) and never
        // orbits.
        if is_home(c) {
            inside += share;
            continue;
        }
        if c.ell > ratio(0, 1) {
            let d = c.ell.clone() / inside.clone();
            c.radius = crate::ring::grain(&(d.clone() * bells.clone()));
            c.period = crate::ring::grain(&(crate::ring::period(&d, &inside) * bells.clone()));
            if c.phase.len() < planes {
                c.phase = vec![ratio(0, 1); planes];
            }
            let delta = inside.clone() / (d + inside.clone());
            // In every spatial plane at its one rate (docs/95).
            for &k in &crate::flows::SPACE {
                let turned = c.phase[k].clone() + ratio(4, 1) * delta.clone();
                c.phase[k] = crate::ring::grain(&(turned.clone() - turned.floor()));
            }
        }
        inside += ratio(2, 1) * share;
    }
    (cross, identity)
}

/// MAGNETISM (operator 2026-10-15, "transduction via magnetism"): an
/// atom's open frames (its unpaired electrons) are its magnetic moments, so
/// a manifest's magnetism is its grains' unpaired counts. Holding is
/// `3Z − unpaired` a nucleus (`atom::hold`), so HOLDING + MAGNETISM = 3Z,
/// which fusion conserves (it conserves Z): pairing frames TRANSDUCES
/// magnetism into holding, unpairing holding into magnetism, and the light
/// a fusion releases is that transduction's flux through the continuum
/// face, never mass.
pub fn magnetism(m: &Manifest) -> Ratio {
    m.iter().fold(ratio(0, 1), |s, (k, a)| {
        let open = crate::chem::unpaired(k);
        s + a.clone() * ratio(i64::from(open), 1)
    })
}

/// THE SPIN LEDGER's total (docs/83 Phase 0): every orbiting body's mass
/// times its specific spin in both orbits (docs/90), and every medium's
/// spin, in units of the unit mass. The motes' spins and the cores'
/// reservoirs are kept in each galaxy's own units and are not in it, so it
/// moves wherever matter crosses between motes and bodies (`breathe` reads
/// how far).
pub fn spin_total(u: &Universe, unit: &Ratio) -> Ratio {
    fn of(b: &Body, unit: &Ratio) -> Ratio {
        let own = mass(&b.total()) / unit.clone() * (b.ell.clone() + b.ell_orth.clone());
        b.satellites.iter().fold(own + b.medium_spin.clone(), |s, x| s + of(x, unit))
    }
    u.cores.iter().fold(ratio(0, 1), |s, c| s + of(c, unit))
}

impl Universe {
    /// The ocean's matter, read from its motes.
    pub fn medium(&self) -> Manifest {
        let mut m = Manifest::new();
        for x in &self.ocean {
            *m.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
        }
        m
    }

    pub fn total(&self) -> Manifest {
        let mut all = self.medium();
        for x in &self.firmament {
            *all.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
        }
        for (k, a) in self.cores.iter().flat_map(|c| c.total()) {
            *all.entry(k).or_insert_with(|| ratio(0, 1)) += a;
        }
        all
    }
}

fn put_manifest(m: &Manifest, out: &mut Vec<u8>) {
    out.extend_from_slice(&(m.len() as u32).to_le_bytes());
    for (k, a) in m {
        let k = k.to_string();
        out.extend_from_slice(&(k.len() as u16).to_le_bytes());
        out.extend_from_slice(k.as_bytes());
        cfe::ratio::encode(a, out);
    }
}

fn put_body(b: &Body, out: &mut Vec<u8>) {
    let mut v = Vec::new();
    v.push(b.path.len() as u8);
    for p in &b.path {
        v.extend_from_slice(&p.to_le_bytes());
    }
    cfe::ratio::encode(&b.radius, &mut v);
    cfe::ratio::encode(&b.period, &mut v);
    cfe::ratio::encode(&b.ell, &mut v);
    cfe::ratio::encode(&b.medium_spin, &mut v);
    put_manifest(&b.kept, &mut v);
    put_manifest(&b.medium, &mut v);
    cfe::tlv::put(1, &v, out);
    for p in &b.gas {
        let mut g = Vec::new();
        let key = p.key.to_string();
        g.extend_from_slice(&(key.len() as u16).to_le_bytes());
        g.extend_from_slice(key.as_bytes());
        for r in [&p.held, &p.inner, &p.outer, &p.agitation, &p.radiates, &p.vacuum] {
            cfe::ratio::encode(r, &mut g);
        }
        g.extend_from_slice(&p.born.to_le_bytes());
        cfe::tlv::put(6, &g, out);
    }
    if b.born > 0 {
        cfe::tlv::put(7, &b.born.to_le_bytes(), out);
    }
    if !b.state.is_zero() {
        let mut w = Vec::new();
        cfe::ratio::encode(&b.state, &mut w);
        cfe::tlv::put(9, &w, out);
    }
    for x in &b.flows {
        let mut w = Vec::new();
        let key = x.key.to_string();
        w.extend_from_slice(&(key.len() as u16).to_le_bytes());
        w.extend_from_slice(key.as_bytes());
        for r in [&x.amount, &x.ell_o, &x.ell_orth, &x.agitation] {
            cfe::ratio::encode(r, &mut w);
        }
        w.push(x.phase.len() as u8);
        for r in &x.phase {
            cfe::ratio::encode(r, &mut w);
        }
        w.extend_from_slice(&x.born.to_le_bytes());
        w.push(u8::from(x.vacuum));
        cfe::ratio::encode(&x.lever_place, &mut w);
        w.push(x.lever.len() as u8);
        for l in &x.lever {
            cfe::ratio::encode(l, &mut w);
        }
        w.push(x.tilt.len() as u8);
        for t in &x.tilt {
            cfe::ratio::encode(t, &mut w);
        }
        cfe::tlv::put(10, &w, out);
    }
    if !b.reservoir.is_zero() {
        let mut w = Vec::new();
        cfe::ratio::encode(&b.reservoir, &mut w);
        cfe::tlv::put(11, &w, out);
    }
    if !b.turning.is_zero() {
        let mut w = Vec::new();
        cfe::ratio::encode(&b.turning, &mut w);
        cfe::tlv::put(12, &w, out);
    }
    if !b.load.is_zero() {
        let mut w = Vec::new();
        cfe::ratio::encode(&b.load, &mut w);
        cfe::tlv::put(14, &w, out);
    }
    if !b.ell_orth.is_zero() {
        let mut w = Vec::new();
        cfe::ratio::encode(&b.ell_orth, &mut w);
        cfe::tlv::put(21, &w, out);
    }

    if !b.phase.is_empty() {
        let mut w = vec![b.phase.len() as u8];
        for r in &b.phase {
            cfe::ratio::encode(r, &mut w);
        }
        cfe::tlv::put(13, &w, out);
    }
    if !b.bell.is_empty() {
        let mut w = vec![b.bell.len() as u8];
        for r in &b.bell {
            cfe::ratio::encode(r, &mut w);
        }
        cfe::tlv::put(15, &w, out);
    }
    if b.remnant {
        cfe::tlv::put(16, &[1], out);
    }
    if !b.heat.is_zero() {
        let mut w = Vec::new();
        cfe::ratio::encode(&b.heat, &mut w);
        cfe::tlv::put(18, &w, out);
    }
    if !b.tilt.is_empty() {
        let mut w = vec![b.tilt.len() as u8];
        for r in &b.tilt {
            cfe::ratio::encode(r, &mut w);
        }
        cfe::tlv::put(22, &w, out);
    }
    for s in &b.satellites {
        put_body(s, out);
    }
}

/// `<var>/pulse.bin`: tag 1 a body (path, radius, period, kept, medium),
/// parents before their satellites; tag 2 the universe's medium; tag 3 the
/// seed.
pub fn write(var_root: &Path, seed: &[u8; 32], u: &Universe) -> std::io::Result<()> {
    let mut out = Vec::new();
    cfe::tlv::put(5, &[2], &mut out);
    cfe::tlv::put(3, seed, &mut out);
    for c in &u.cores {
        put_body(c, &mut out);
    }
    let mut w = Vec::new();
    cfe::ratio::encode(&u.unit, &mut w);
    cfe::tlv::put(8, &w, &mut out);
    cfe::tlv::put(17, &u.t_u.to_le_bytes(), &mut out);
    for (tag, x) in u.firmament.iter().map(|x| (19u8, x)).chain(u.ocean.iter().map(|x| (24u8, x))) {
        let mut w = Vec::new();
        let key = x.key.to_string();
        w.extend_from_slice(&(key.len() as u16).to_le_bytes());
        w.extend_from_slice(key.as_bytes());
        for r in [&x.amount, &x.ell_o, &x.ell_orth, &x.agitation] {
            cfe::ratio::encode(r, &mut w);
        }
        w.push(x.phase.len() as u8);
        for r in &x.phase {
            cfe::ratio::encode(r, &mut w);
        }
        w.extend_from_slice(&x.born.to_le_bytes());
        w.push(u8::from(x.vacuum));
        cfe::ratio::encode(&x.lever_place, &mut w);
        w.push(x.lever.len() as u8);
        for l in &x.lever {
            cfe::ratio::encode(l, &mut w);
        }
        cfe::tlv::put(tag, &w, &mut out);
    }
    std::fs::create_dir_all(var_root)?;
    let path = var_root.join("pulse.bin");
    let staged = path.with_extension("staged");
    std::fs::write(&staged, out)?;
    std::fs::rename(staged, path)
}

fn get_manifest(v: &[u8], at: &mut usize) -> Option<Manifest> {
    let n = u32::from_le_bytes(v.get(*at..*at + 4)?.try_into().ok()?);
    *at += 4;
    let mut m = Manifest::new();
    for _ in 0..n {
        let len = u16::from_le_bytes(v.get(*at..*at + 2)?.try_into().ok()?) as usize;
        let key = crate::chem::Kind::parse(std::str::from_utf8(v.get(*at + 2..*at + 2 + len)?).ok()?)?;
        *at += 2 + len;
        let (a, used) = cfe::ratio::decode(v.get(*at..)?)?;
        *at += used;
        m.insert(key, a);
    }
    Some(m)
}

fn get_body(v: &[u8]) -> Option<Body> {
    let mut at = 0;
    let depth = *v.first()? as usize;
    at += 1;
    let mut path = Vec::with_capacity(depth.min(8));
    for _ in 0..depth {
        path.push(u64::from_le_bytes(v.get(at..at + 8)?.try_into().ok()?));
        at += 8;
    }
    let (radius, used) = cfe::ratio::decode(v.get(at..)?)?;
    at += used;
    let (period, used) = cfe::ratio::decode(v.get(at..)?)?;
    at += used;
    let (ell, used) = cfe::ratio::decode(v.get(at..)?)?;
    at += used;
    let (medium_spin, used) = cfe::ratio::decode(v.get(at..)?)?;
    at += used;
    let kept = get_manifest(v, &mut at)?;
    let medium = get_manifest(v, &mut at)?;
    Some(Body { ell_orth: ratio(0, 1), path, radius, period, kept, satellites: Vec::new(), medium, ell, medium_spin, gas: Vec::new(), born: 0, state: ratio(0, 1), flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: ratio(0, 1), tilt: Vec::new() })
}

/// A flow mote (tag 10), attached to the body read before it.
fn get_flow(v: &[u8]) -> Option<crate::flows::Mote> {
    let len = u16::from_le_bytes(v.get(0..2)?.try_into().ok()?) as usize;
    let key = crate::chem::Kind::parse(std::str::from_utf8(v.get(2..2 + len)?).ok()?)?;
    let mut at = 2 + len;
    let mut r = Vec::new();
    for _ in 0..4 {
        let (x, used) = cfe::ratio::decode(v.get(at..)?)?;
        at += used;
        r.push(x);
    }
    let n = *v.get(at)? as usize;
    at += 1;
    let mut phase = Vec::with_capacity(n.min(16));
    for _ in 0..n {
        let (x, used) = cfe::ratio::decode(v.get(at..)?)?;
        at += used;
        phase.push(x);
    }
    let born = u64::from_le_bytes(v.get(at..at + 8)?.try_into().ok()?);
    let vacuum = *v.get(at + 8)? == 1;
    at += 9;
    // Its lever: the place it is counted in, then each plane's.
    let (lever_place, used) = cfe::ratio::decode(v.get(at..)?)?;
    at += used;
    let m = *v.get(at)? as usize;
    at += 1;
    let mut lever = Vec::with_capacity(m.min(16));
    for _ in 0..m {
        let (x, used) = cfe::ratio::decode(v.get(at..)?)?;
        at += used;
        lever.push(x);
    }
    // Its tilt, where the tape carries one (docs/96 step 1).
    let mut tilt = Vec::new();
    if let Some(&t) = v.get(at) {
        at += 1;
        for _ in 0..usize::from(t).min(16) {
            let (x, used) = cfe::ratio::decode(v.get(at..)?)?;
            at += used;
            tilt.push(x);
        }
    }
    let mut r = r.into_iter();
    Some(crate::flows::Mote { key, amount: r.next()?, ell_o: r.next()?, ell_orth: r.next()?, agitation: r.next()?, phase, born, vacuum, lever, lever_place, tilt })
}

/// A gas mote (tag 6), attached to the body read before it.
fn get_mote(v: &[u8]) -> Option<crate::formation::Shell> {
    let len = u16::from_le_bytes(v.get(0..2)?.try_into().ok()?) as usize;
    let key = crate::chem::Kind::parse(std::str::from_utf8(v.get(2..2 + len)?).ok()?)?;
    let mut at = 2 + len;
    let mut r = Vec::new();
    for _ in 0..6 {
        let (x, used) = cfe::ratio::decode(v.get(at..)?)?;
        at += used;
        r.push(x);
    }
    let born = u64::from_le_bytes(v.get(at..at + 8)?.try_into().ok()?);
    let mut r = r.into_iter();
    Some(crate::formation::Shell { key, held: r.next()?, inner: r.next()?, outer: r.next()?, agitation: r.next()?, radiates: r.next()?, vacuum: r.next()?, born })
}

/// Read `<var>/pulse.bin` back: the seed and the universe (bodies are
/// written parents first, so each attaches under the last body one level up).
pub fn read(var_root: &Path) -> Option<([u8; 32], Universe)> {
    let bytes = std::fs::read(var_root.join("pulse.bin")).ok()?;
    let mut seed = None;
    let mut ocean: Vec<crate::flows::Mote> = Vec::new();
    let mut unit: Option<Ratio> = None;
    let mut t_u: Option<u64> = None;
    let mut firmament: Vec<crate::flows::Mote> = Vec::new();
    let mut flat: Vec<Body> = Vec::new();
    // A universe is read whole or refused: the spin ledger's format (tag 5),
    // its unit (8) and its bell's period (17) are required.
    if !cfe::tlv::records(&bytes).any(|(t, _)| t == 5) {
        return None;
    }
    for (t, v) in cfe::tlv::records(&bytes) {
        match t {
            3 => seed = v.try_into().ok(),
            // A file from before the ocean was motes: its manifest at the
            // singularity, on the vacuum leg (its amounts kept whole).
            2 => {
                for (k, a) in get_manifest(v, &mut 0)? {
                    ocean.push(crate::flows::Mote { key: k, amount: a, ell_o: ratio(0, 1), ell_orth: ratio(0, 1), phase: crate::flows::rest(ratio(1, 2)), agitation: ratio(0, 1), born: 0, vacuum: true, lever: Vec::new(), lever_place: ratio(0, 1), tilt: Vec::new() });
                }
            }
            24 => ocean.push(get_flow(v)?),
            1 => flat.push(get_body(v)?),
            6 => flat.last_mut()?.gas.push(get_mote(v)?),
            7 => flat.last_mut()?.born = u64::from_le_bytes(v.try_into().ok()?),
            8 => unit = Some(cfe::ratio::decode(v)?.0),
            17 => t_u = Some(u64::from_le_bytes(v.try_into().ok()?)),
            19 => firmament.push(get_flow(v)?),
            9 => flat.last_mut()?.state = cfe::ratio::decode(v)?.0,
            10 => flat.last_mut()?.flows.push(get_flow(v)?),
            11 => flat.last_mut()?.reservoir = cfe::ratio::decode(v)?.0,
            12 => flat.last_mut()?.turning = cfe::ratio::decode(v)?.0,
            14 => flat.last_mut()?.load = cfe::ratio::decode(v)?.0,
            21 => flat.last_mut()?.ell_orth = cfe::ratio::decode(v)?.0,
            16 => flat.last_mut()?.remnant = v.first() == Some(&1),
            18 => flat.last_mut()?.heat = cfe::ratio::decode(v)?.0,
            13 | 15 | 22 => {
                let n = *v.first()? as usize;
                let mut at = 1;
                let mut ph = Vec::with_capacity(n.min(16));
                for _ in 0..n {
                    let (x, used) = cfe::ratio::decode(v.get(at..)?)?;
                    at += used;
                    ph.push(x);
                }
                match t {
                    13 => flat.last_mut()?.phase = ph,
                    15 => flat.last_mut()?.bell = ph,
                    _ => flat.last_mut()?.tilt = ph,
                }
            }
            _ => {}
        }
    }
    // Rebuild the tree: depth 1 cores, 2 systems, 3 orbs.
    let mut cores: Vec<Body> = Vec::new();
    for b in flat {
        match b.path.len() {
            1 => cores.push(b),
            2 => cores.last_mut()?.satellites.push(b),
            3 => cores.last_mut()?.satellites.last_mut()?.satellites.push(b),
            _ => return None,
        }
    }
    let (unit, t_u) = (unit.filter(|u| !u.is_zero())?, t_u?);
    Some((seed?, Universe { cores, ocean, firmament, unit, t_u }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_universe_file_without_its_unit_is_refused() {
        let dir = std::env::temp_dir().join(format!("ns-refuse-unit-{}", std::process::id()));
        let u = Universe { cores: Vec::new(), ocean: vec![crate::flows::Mote { key: crate::chem::Kind::parse("Z001").expect("a kind"), amount: ratio(1, 1), ell_o: ratio(1, 3), ell_orth: ratio(0, 1), phase: crate::flows::rest(ratio(1, 2)), agitation: ratio(0, 1), born: 0, vacuum: true, lever: Vec::new(), lever_place: ratio(0, 1), tilt: Vec::new() }], firmament: Vec::new(), unit: ratio(1, 2), t_u: 15 };
        write(&dir, &[7u8; 32], &u).expect("written");
        assert!(read(&dir).is_some());
        let bytes = std::fs::read(dir.join("pulse.bin")).expect("read");
        let mut out = Vec::new();
        for (t, v) in cfe::tlv::records(&bytes) {
            if t != 8 {
                cfe::tlv::put(t, v, &mut out);
            }
        }
        std::fs::write(dir.join("pulse.bin"), out).expect("rewritten");
        assert!(read(&dir).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_founding_that_holds_no_disc_is_refused() {
        assert!(deploy(&[7u8; 32], &Manifest::new()).is_none());
    }

    #[test]
    fn a_cluster_weighs_its_parts_in_the_galaxy_cut() {
        // A cluster's weight as mass is its parts' (3Z each, converged:
        // operator 2026-10-24): water, Z008·4Z001, weighs 3·8 + 4·3 = 36, as
        // Z012 does (3·12); one galaxy of each reads one push, one hold, one
        // mean weight.
        let k = |s: &str| crate::chem::Kind::parse(s).expect("a kind");
        assert_eq!(crate::chem::weight(&k("Z008·4Z001")), crate::chem::weight(&k("Z008")) + 4 * crate::chem::weight(&k("Z001")));
        assert_eq!(crate::chem::weight(&k("Z008·4Z001")), crate::chem::weight(&k("Z012")));
        let water: Manifest = [(k("Z008·4Z001"), ratio(1, 1)), (k("Z001"), ratio(3, 1))].into();
        let twelve: Manifest = [(k("Z012"), ratio(1, 1)), (k("Z001"), ratio(3, 1))].into();
        assert_eq!(push_cut(&water).map(|(p, h, m, _)| (p, h, m)), push_cut(&twelve).map(|(p, h, m, _)| (p, h, m)));
    }

    #[test]
    fn the_pulse_file_reads_back_exactly() {
        let mote = crate::formation::Shell { key: crate::chem::Kind::parse("Z001").expect("a kind"), held: ratio(1, 5), inner: ratio(1, 3), outer: ratio(2, 3), agitation: ratio(1, 2), radiates: ratio(1, 2), born: 7, vacuum: ratio(3, 7) };
        let orb = Body { ell_orth: ratio(0, 1), path: vec![1, 1, 1], radius: ratio(3, 2), period: ratio(5, 2), kept: [(crate::chem::Kind::parse("Z008").expect("a kind"), ratio(1, 64))].into(), satellites: Vec::new(), medium: Manifest::new(), ell: ratio(7, 3), medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: ratio(0, 1), flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: ratio(0, 1), tilt: Vec::new() };
        let sys = Body { ell_orth: ratio(0, 1), path: vec![1, 1], radius: ratio(9, 2), period: ratio(11, 2), kept: [(crate::chem::Kind::parse("Z001").expect("a kind"), ratio(3, 1024))].into(), satellites: vec![orb], medium: Manifest::new(), ell: ratio(1, 9), medium_spin: ratio(1, 7), gas: Vec::new(), born: 0, state: ratio(0, 1), flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: ratio(0, 1), tilt: Vec::new() };
        let core = Body { ell_orth: ratio(0, 1), path: vec![1], radius: ratio(60, 1), period: ratio(0, 1), kept: [(crate::chem::Kind::parse("Z026").expect("a kind"), ratio(1, 2))].into(), satellites: vec![sys], medium: [(crate::chem::Kind::parse("Z001").expect("a kind"), ratio(5, 4))].into(), ell: ratio(0, 1), medium_spin: ratio(13, 11), gas: vec![mote], born: 0, state: ratio(0, 1), flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: ratio(0, 1), tilt: Vec::new() };
        let u = Universe { cores: vec![core], ocean: vec![crate::flows::Mote { key: crate::chem::Kind::parse("Z002").expect("a kind"), amount: ratio(1, 1024), ell_o: ratio(1, 3), ell_orth: ratio(0, 1), phase: crate::flows::rest(ratio(1, 2)), agitation: ratio(0, 1), born: 0, vacuum: true, lever: Vec::new(), lever_place: ratio(0, 1), tilt: Vec::new() }], firmament: Vec::new(), unit: ratio(1, 2), t_u: 15 };
        let dir = std::env::temp_dir().join(format!("pulse-{}", std::process::id()));
        write(&dir, &[7; 32], &u).unwrap();
        assert_eq!(read(&dir), Some(([7; 32], u)), "the file gives the universe back");
        let _ = std::fs::remove_dir_all(dir);
    }
}
