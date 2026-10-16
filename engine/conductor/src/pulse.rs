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

/// One element's amount, by name: an almanac element by its id in hex, or a
/// matter resource as `resource:<name>`.
pub type Manifest = BTreeMap<String, Ratio>;

/// The deployment's grain (the resource ledger's, 1/1024; the element
/// stocks' 1/64 lies on it).
fn grain(x: &Ratio) -> Ratio {
    cfe::world::smelt_resource(x)
}

/// Mass for the turning law: each grain of Z holds its atom's rings
/// (`cfe::almanac::atom::hold`, mass is holding); a key that is no Z (era
/// 0's kinds, before they cross) holds one.
pub fn mass(m: &Manifest) -> Ratio {
    m.iter().fold(ratio(0, 1), |s, (k, a)| {
        let weight = k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok()).map(cfe::almanac::atom::hold).unwrap_or(1);
        s + a.clone() * ratio(i64::from(weight), 1)
    })
}

fn hex(id: &[u8; 32]) -> String {
    id.iter().map(|b| format!("{b:02x}")).collect()
}

/// THE STAND-IN FOUNDING MANIFEST (docs/78, Open): era 0's matter as its
/// worlds' snapshots hold it, every orb's element stocks and matter
/// resources (iron, biomass, alloy, diamond, crystal, scrap). Energy and
/// capacity (joule, sink) and money (STAMP) are not matter. Returns the
/// manifest and the worlds read.
pub fn stand_in(era0: &Path) -> (Manifest, usize) {
    let mut manifest = Manifest::new();
    let mut read = 0;
    let Ok(dirs) = std::fs::read_dir(era0.join("worlds")) else { return (manifest, 0) };
    let mut dirs: Vec<_> = dirs.filter_map(|d| d.ok()).map(|d| d.path()).collect();
    dirs.sort();
    for dir in dirs {
        let Some((_, _, state)) = crate::snapshot::read_state(&dir) else { continue };
        read += 1;
        for orb in &state.orbs {
            for (id, amount) in &orb.elements {
                *manifest.entry(hex(id)).or_insert_with(|| ratio(0, 1)) += amount.clone();
            }
            for (resource, amount) in &orb.banked {
                if matches!(resource, cfe::dop::Resource::Joule | cfe::dop::Resource::Sink) {
                    continue;
                }
                *manifest.entry(format!("resource:{resource:?}")).or_insert_with(|| ratio(0, 1)) += amount.clone();
            }
            *manifest.entry("resource:Scrap".into()).or_insert_with(|| ratio(0, 1)) += orb.scrap.clone();
        }
    }
    manifest.retain(|_, a| *a > ratio(0, 1));
    for a in manifest.values_mut() {
        *a = grain(a);
    }
    manifest.retain(|_, a| *a > ratio(0, 1));
    (manifest, read)
}

/// The nuclei in one grain of any Z (docs/78, "The atom"): 60^30.
pub fn nuclei_per_grain() -> num_bigint::BigInt {
    num_bigint::BigInt::from(60u32).pow(30)
}

/// A manifest as mycelium rules it (MANIFEST, ACT_DOMAIN 17): nuclei per Z.
pub type Nuclei = BTreeMap<u16, num_bigint::BigInt>;

/// The Z whose atom (`cfe::almanac::atom`) reads an era-0 kind's line and
/// binding: the lowest Z at that depth with that unpaired count, else the
/// nearest unpaired count at that depth (lowest Z on a tie). The stand-in's
/// map only; era 1's own elements are one per Z.
fn z_of_kind(id: &[u8; 32]) -> u16 {
    let cell = cfe::almanac::periodic::periodic_of(id);
    let (line, binds) = (u32::from(cell.line), u32::from(cell.binds));
    (1u32..=118)
        .filter_map(cfe::almanac::atom::atom)
        .filter(|a| a.depth == line)
        .min_by_key(|a| (a.unpaired.abs_diff(binds), a.z))
        .map(|a| a.z as u16)
        .unwrap_or(1)
}

/// An era-0 manifest key's id: an almanac element's own, or a bulk
/// resource's derived from its name (so it too reads a cell by the same
/// law, with no formula of ours).
fn kind_id(key: &str) -> [u8; 32] {
    if let Some(name) = key.strip_prefix("resource:") {
        return *blake3::Hasher::new_derive_key("netstratum era 0 resource").update(name.as_bytes()).finalize().as_bytes();
    }
    let mut id = [0u8; 32];
    for (i, b) in id.iter_mut().enumerate() {
        *b = key.get(2 * i..2 * i + 2).and_then(|h| u8::from_str_radix(h, 16).ok()).unwrap_or(0);
    }
    id
}

/// The stand-in as nuclei per Z: each era-0 kind's grains, at its Z, times
/// the nuclei in a grain. Exact (the amounts lie on the 1/1024 grain).
pub fn to_nuclei(kinds: &Manifest) -> Nuclei {
    let mut out = Nuclei::new();
    for (key, amount) in kinds {
        let grains = amount.clone() * ratio(1024, 1);
        let count = grains.to_integer() * nuclei_per_grain();
        *out.entry(z_of_kind(&kind_id(key))).or_default() += count;
    }
    out.retain(|_, c| *c > num_bigint::BigInt::from(0));
    out
}

/// A manifest of nuclei entering: per Z, whole grains (`Z<n>` keys) and the
/// remainder, under a grain's nuclei, kept in the unformed ledger.
pub fn from_nuclei(nuclei: &Nuclei) -> (Manifest, Nuclei) {
    let per = nuclei_per_grain();
    let mut grains = Manifest::new();
    let mut unformed = Nuclei::new();
    for (z, count) in nuclei {
        let (whole, rest) = (count / &per, count % &per);
        if whole > num_bigint::BigInt::from(0) {
            grains.insert(format!("Z{z:03}"), Ratio::new(whole, num_bigint::BigInt::from(1024)));
        }
        if rest > num_bigint::BigInt::from(0) {
            unformed.insert(*z, rest);
        }
    }
    (grains, unformed)
}

/// Grains back out as nuclei per Z (the unformed added), for the
/// conservation reading.
pub fn out_nuclei(grains: &Manifest, unformed: &Nuclei) -> Nuclei {
    let mut out = unformed.clone();
    for (key, amount) in grains {
        let Some(z) = key.strip_prefix('Z').and_then(|z| z.parse::<u16>().ok()) else { continue };
        *out.entry(z).or_default() += (amount.clone() * ratio(1024, 1)).to_integer() * nuclei_per_grain();
    }
    out
}

/// THE FOUNDING DISTRIBUTION (docs/78; operator 2026-10-15, option B): the
/// shape of the universe's founding manifest, derived from pairing (design
/// law 6: opposite senses pair, and pairing is what closes). A nucleus one
/// step heavier adds one proton–neutron frame, which closes only when its
/// sense opposes, one of two: each step closes half of what tries, so the
/// bounce's nuclei fall off by halving, `count(Z) = N · 2^−Z`, until a Z
/// falls below one nucleus (no cap). Light-dominated by mass, read, never
/// fitted. It is the SHAPE the universe requests: the composition of the
/// signed MANIFEST its reserve sends the court (mycelium 78.155b), which
/// records and grants it and never reshapes it.
pub fn founding_shape(nuclei: &num_bigint::BigInt) -> Nuclei {
    let mut out = Nuclei::new();
    let mut count = nuclei.clone();
    for z in 1u16..=118 {
        count /= 2;
        if count <= num_bigint::BigInt::from(0) {
            break;
        }
        out.insert(z, count.clone());
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
    /// docs/83 FORMATION: a galactic core's gas as PARCELS (each element's
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
/// (quantum mass) whatever its 3-D grains. In the grains' holds.
pub fn core_state(core: &Body) -> Ratio {
    if !core.state.is_zero() {
        return core.state.clone();
    }
    mass(&core.total())
}

/// The universal core's state hold: all the universe's grains together hold
/// exactly one of its rings, so its state equals their total holding.
pub fn universal_state(u: &Universe) -> Ratio {
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
    let weight = |k: &str| k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok()).map(cfe::almanac::atom::hold).unwrap_or(1);
    let total = mass(grains);
    let count = grains.values().fold(ratio(0, 1), |s, a| s + a.clone());
    if total.is_zero() || count.is_zero() {
        return None;
    }
    let mean = total.clone() / count;
    let mut order: Vec<(&String, &Ratio)> = grains.iter().collect();
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
    let h = ratio(1, 1) + held.clone();
    Some((held / h.clone(), h, mean))
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
    let weight = |k: &str| k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok()).map(cfe::almanac::atom::hold).unwrap_or(1);
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
    pub medium: Manifest,
    /// The unit of mass: the deepest core's hole AT THE FOUNDING. A hole is
    /// set by the bounce and does not drain when its disc jets away
    /// (regeneration; with xylarium: the unit is the core's state, never its
    /// current disc). Tag 8.
    pub unit: Ratio,
}

/// The universal echo, in bells: the universe's own ring, the pulse's round
/// trip to its rim and back (rim 2 core rings, so 4 core rings).
pub const T_U: u64 = 60;

/// THE PULSE: the manifest deployed from `seed`. The unit of mass is the
/// deepest core's hole.
pub fn deploy(seed: &[u8; 32], manifest: &Manifest) -> Universe {
    // The intergalactic medium is what no core holds (the cut one tier up,
    // below); nothing is set aside by a share.
    let mut medium = Manifest::new();
    let mut formed = manifest.clone();
    // THE SPECTRUM (docs/78): the count of modes and each mode's strength
    // `a_i`, seed draws from the base-60 field (1 to 60). Core i takes
    // `a_i / Σa` of the formed matter, so sizes fall in no fixed order and
    // the largest is at most 60 times the smallest. A mode too weak to hold
    // a hole of a grain forms no core; its share stays unformed.
    let modes = 1 + (draw(seed, &[], "modes", 0) % 60) as u32;
    let strengths: Vec<i64> = (1..=modes).map(|i| 1 + (draw(seed, &[u64::from(i)], "strength", 0) % 60) as i64).collect();
    let sum: i64 = strengths.iter().sum();
    // THE CUT ONE TIER UP (operator 2026-10-16, with xylarium): the
    // epicentre's bounce sorts the candidate cores as a galaxy's sorts its
    // nuclei. In universe units (its state 1, all grains 1, H_u = 2, the push
    // share p_u = ½) a core of weight `w_i = a_i/Σa` (N candidates, mean
    // weight 1/N) is carried `ℓ_i = 1/(2N·w_i)` and stands at
    // `d_i = 1/(4N·w_i)` core rings. HELD as a galaxy iff `d_i ≤ rim_u = 2`
    // (the rim law), i.e. `a_i ≥ Σa/(8N)`; lighter cores never form and stay
    // intergalactic. A core reaching inside the throat (`d_i < 1`) is held AT
    // the throat's first ring, d = 1: the heaviest crowd the first ring.
    let n = i64::from(modes);
    let mut shares: Vec<(u32, Manifest, Ratio)> = Vec::new();
    for (i, a) in (1..=modes).zip(&strengths) {
        let m = take(&formed, &ratio(*a, sum));
        if *a * 8 * n < sum || take(&m, &tier(2).centre).is_empty() {
            continue;
        }
        let d = ratio(sum, 4 * n * *a).max(ratio(1, 1));
        shares.push((i, m, d));
    }
    for (_, m, _) in &shares {
        less(&mut formed, m);
    }
    for (k, a) in formed {
        *medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
    }
    // THE BIG BANG POPULATES THE UNIVERSE (docs/78): each core keeps its disc,
    // and its galaxy's first generation of star systems and their planets is
    // laid down by the tidal law; what forms no system is the galaxy's medium,
    // gas the breath later forms into new systems. A core stands where the cut
    // one tier up holds it (above); its echoes arrive that many bells later.
    // Each core keeps what its bounce holds (`disperse`), in its own units.
    let discs: Vec<Manifest> = shares.iter().map(|(_, m, _)| disperse(m)).collect();
    let unit = discs.iter().map(mass).max().filter(|m| !m.is_zero()).unwrap_or_else(|| ratio(1, 1));
    // FORMATION (docs/83): no system is placed. Each core keeps its disc;
    // the rest is its galaxy's GAS, laid down as parcels where the push left
    // each element; stars collapse from it as it cools (`breath`).
    // THE GALAXIES ORBIT THE UNIVERSAL SINGULARITY (operator): inside out,
    // each about the universal state plus every core inside it (each core
    // its galaxy's state and grains, 2·w_j), by the motion law, in core
    // rings; a core ring is `T_U / 4` bells (the universe's own echo is the
    // round trip to its rim 2). The first ring is the deepest well.
    let mut order: Vec<usize> = (0..shares.len()).collect();
    order.sort_by(|&a, &b| shares[a].2.cmp(&shares[b].2));
    let mut orbits: Vec<(Ratio, Ratio, Ratio)> = vec![(ratio(0, 1), ratio(0, 1), ratio(0, 1)); shares.len()];
    let mut inside = ratio(1, 1);
    let bells = ratio((T_U / 4) as i64, 1);
    let mut k = 0;
    while k < order.len() {
        // A ring: every core at the same reach orbits about what is inside it.
        let d = shares[order[k]].2.clone();
        let ring: Vec<usize> = order[k..].iter().copied().take_while(|&j| shares[j].2 == d).collect();
        for &j in &ring {
            orbits[j] = (d.clone() * bells.clone(), crate::ring::period(&d, &inside) * bells.clone(), crate::ring::spin(&d, &inside));
        }
        for &j in &ring {
            inside += ratio(2 * strengths[(shares[j].0 - 1) as usize], sum);
        }
        k += ring.len();
    }
    let cores = shares
        .into_iter()
        .zip(discs)
        .zip(orbits)
        .map(|(((i, m, _), disc), (radius, period, ell))| {
            let (shells, _) = crate::formation::shells(&m, &disc);
            let gas: Vec<crate::formation::Shell> = shells.into_iter().filter(|s| s.outer > s.inner).collect();
            let state = mass(&m) / unit.clone();
            let mut medium = m.clone();
            less(&mut medium, &disc);
            // The gas turns with the push that carried it: each parcel's
            // spin at its reach, in the transition units (`ring::spin`).
            let scale = mass(&m) / unit.clone();
            let h = mass(&disc) / unit.clone() + state.clone();
            let medium_spin = gas.iter().fold(ratio(0, 1), |acc, p| {
                let reach = (p.inner.clone() + p.outer.clone()) / ratio(2, 1) * scale.clone();
                acc + p.held.clone() * scale.clone() * crate::ring::spin(&reach, &h)
            });
            Body { path: vec![u64::from(i)], radius, period, kept: disc, satellites: Vec::new(), medium, ell, medium_spin, gas, born: 0, state: mass(&m) }
        })
        .collect();
    Universe { cores, medium, unit }
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
        let open = k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok()).and_then(cfe::almanac::atom::atom).map(|x| x.unpaired).unwrap_or(0);
        s + a.clone() * ratio(i64::from(open), 1)
    })
}

/// THE SPIN LEDGER's total (docs/83 Phase 0): every orbiting body's mass
/// times its specific spin, and every medium's spin, in units of the unit
/// mass. Conserved exactly across the breath.
pub fn spin_total(u: &Universe, unit: &Ratio) -> Ratio {
    fn of(b: &Body, unit: &Ratio) -> Ratio {
        let own = mass(&b.total()) / unit.clone() * b.ell.clone();
        b.satellites.iter().fold(own + b.medium_spin.clone(), |s, x| s + of(x, unit))
    }
    u.cores.iter().fold(ratio(0, 1), |s, c| s + of(c, unit))
}

impl Universe {
    pub fn total(&self) -> Manifest {
        let mut all = self.medium.clone();
        for (k, a) in self.cores.iter().flat_map(|c| c.total()) {
            *all.entry(k).or_insert_with(|| ratio(0, 1)) += a;
        }
        all
    }
}

fn put_manifest(m: &Manifest, out: &mut Vec<u8>) {
    out.extend_from_slice(&(m.len() as u32).to_le_bytes());
    for (k, a) in m {
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
        g.extend_from_slice(&(p.key.len() as u16).to_le_bytes());
        g.extend_from_slice(p.key.as_bytes());
        for r in [&p.held, &p.inner, &p.outer, &p.agitation] {
            cfe::ratio::encode(r, &mut g);
        }
        g.extend_from_slice(&p.halving.unwrap_or(0).to_le_bytes());
        g.extend_from_slice(&p.born.to_le_bytes());
        g.extend_from_slice(&p.vacuum.to_le_bytes());
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
    let mut v = Vec::new();
    put_manifest(&u.medium, &mut v);
    cfe::tlv::put(2, &v, &mut out);
    let mut w = Vec::new();
    cfe::ratio::encode(&u.unit, &mut w);
    cfe::tlv::put(8, &w, &mut out);
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
        let key = String::from_utf8(v.get(*at + 2..*at + 2 + len)?.to_vec()).ok()?;
        *at += 2 + len;
        let (a, used) = cfe::ratio::decode(v.get(*at..)?)?;
        *at += used;
        m.insert(key, a);
    }
    Some(m)
}

fn get_body(v: &[u8], spun: bool) -> Option<Body> {
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
    let (ell, medium_spin) = if spun {
        let (ell, used) = cfe::ratio::decode(v.get(at..)?)?;
        at += used;
        let (medium_spin, used) = cfe::ratio::decode(v.get(at..)?)?;
        at += used;
        (ell, medium_spin)
    } else {
        (ratio(0, 1), ratio(0, 1))
    };
    let kept = get_manifest(v, &mut at)?;
    let medium = get_manifest(v, &mut at)?;
    Some(Body { path, radius, period, kept, satellites: Vec::new(), medium, ell, medium_spin, gas: Vec::new(), born: 0, state: ratio(0, 1) })
}

/// A gas parcel (tag 6), attached to the body read before it.
fn get_parcel(v: &[u8]) -> Option<crate::formation::Shell> {
    let len = u16::from_le_bytes(v.get(0..2)?.try_into().ok()?) as usize;
    let key = String::from_utf8(v.get(2..2 + len)?.to_vec()).ok()?;
    let mut at = 2 + len;
    let mut r = Vec::new();
    for _ in 0..4 {
        let (x, used) = cfe::ratio::decode(v.get(at..)?)?;
        at += used;
        r.push(x);
    }
    let halving = u64::from_le_bytes(v.get(at..at + 8)?.try_into().ok()?);
    let born = u64::from_le_bytes(v.get(at + 8..at + 16)?.try_into().ok()?);
    // The vacuum's halving (absent in parcels written before it: none).
    let vacuum = v.get(at + 16..at + 24).and_then(|b| b.try_into().ok()).map(u64::from_le_bytes).unwrap_or(0);
    let mut r = r.into_iter();
    Some(crate::formation::Shell { key, held: r.next()?, inner: r.next()?, outer: r.next()?, agitation: r.next()?, halving: (halving > 0).then_some(halving), born, vacuum })
}

/// Read `<var>/pulse.bin` back: the seed and the universe (bodies are
/// written parents first, so each attaches under the last body one level up).
pub fn read(var_root: &Path) -> Option<([u8; 32], Universe)> {
    let bytes = std::fs::read(var_root.join("pulse.bin")).ok()?;
    let mut seed = None;
    let mut medium = Manifest::new();
    let mut unit = ratio(0, 1);
    let mut flat: Vec<Body> = Vec::new();
    // Tag 5 marks the spin ledger's format (docs/83 Phase 0); a file without
    // it reads with no spin (a redeploy gives it its spin).
    let spun = cfe::tlv::records(&bytes).any(|(t, _)| t == 5);
    for (t, v) in cfe::tlv::records(&bytes) {
        match t {
            3 => seed = v.try_into().ok(),
            2 => medium = get_manifest(v, &mut 0)?,
            1 => flat.push(get_body(v, spun)?),
            6 => flat.last_mut()?.gas.push(get_parcel(v)?),
            7 => flat.last_mut()?.born = u64::from_le_bytes(v.try_into().ok()?),
            8 => unit = cfe::ratio::decode(v)?.0,
            9 => flat.last_mut()?.state = cfe::ratio::decode(v)?.0,
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
    Some((seed?, Universe { cores, medium, unit }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pulse_file_reads_back_exactly() {
        let parcel = crate::formation::Shell { key: "Z001".into(), held: ratio(1, 5), inner: ratio(1, 3), outer: ratio(2, 3), agitation: ratio(1, 2), halving: Some(2), born: 7, vacuum: 3 };
        let orb = Body { path: vec![1, 1, 1], radius: ratio(3, 2), period: ratio(5, 2), kept: [("Z008".to_string(), ratio(1, 64))].into(), satellites: Vec::new(), medium: Manifest::new(), ell: ratio(7, 3), medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: ratio(0, 1) };
        let sys = Body { path: vec![1, 1], radius: ratio(9, 2), period: ratio(11, 2), kept: [("Z001".to_string(), ratio(3, 1024))].into(), satellites: vec![orb], medium: Manifest::new(), ell: ratio(1, 9), medium_spin: ratio(1, 7), gas: Vec::new(), born: 0, state: ratio(0, 1) };
        let core = Body { path: vec![1], radius: ratio(60, 1), period: ratio(0, 1), kept: [("Z026".to_string(), ratio(1, 2))].into(), satellites: vec![sys], medium: [("Z001".to_string(), ratio(5, 4))].into(), ell: ratio(0, 1), medium_spin: ratio(13, 11), gas: vec![parcel], born: 0, state: ratio(0, 1) };
        let u = Universe { cores: vec![core], medium: [("Z002".to_string(), ratio(1, 1024))].into(), unit: ratio(1, 2) };
        let dir = std::env::temp_dir().join(format!("pulse-{}", std::process::id()));
        write(&dir, &[7; 32], &u).unwrap();
        assert_eq!(read(&dir), Some(([7; 32], u)), "the file gives the universe back");
        let _ = std::fs::remove_dir_all(dir);
    }
}
