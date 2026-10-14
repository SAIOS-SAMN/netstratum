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

/// Mass for the turning law: each grain of Z weighs its nucleus's mass
/// number (`cfe::almanac::atom::mass_number`); a key that is no Z (era 0's
/// kinds, before they cross) weighs one.
pub fn mass(m: &Manifest) -> Ratio {
    m.iter().fold(ratio(0, 1), |s, (k, a)| {
        let weight = k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok()).map(cfe::almanac::atom::mass_number).unwrap_or(1);
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

/// The seed's fixed paths: draw `j` of `field` for the body at `path`.
fn draw(seed: &[u8; 32], path: &[u32], field: &str, j: u32) -> u64 {
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
struct Tier {
    centre: Ratio,
    medium: Ratio,
    /// A satellite's tidal reach as a share of its radius, `s = ρ / r`
    /// (docs/78, "The mass split"), rational by its half-angle `t`:
    /// `s = 2t / (1 + t²)`.
    reach: Ratio,
}

/// A core's hole: its tier's share (1/1000) times `h`, h a seed draw from
/// 1/4 to 2 (15/60 to 120/60) as xylarium draws a sibling's hole against
/// the home's. Under ω = M/r a galaxy scaled whole is a copy of another
/// (its turnings kept), so its hole's share is what makes galaxies differ.
/// A placeholder until the pulse's physics sets it.
fn hole_share(seed: &[u8; 32], path: &[u32]) -> Ratio {
    tier(2).centre * ratio(15 + (draw(seed, path, "hole", 0) % 106) as i64, 60)
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
fn tier(depth: u8) -> Tier {
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
fn take(from: &Manifest, f: &Ratio) -> Manifest {
    from.iter().map(|(k, a)| (k.clone(), grain(&(a.clone() * f.clone())))).filter(|(_, a)| *a > ratio(0, 1)).collect()
}

fn less(from: &mut Manifest, by: &Manifest) {
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
    pub path: Vec<u32>,
    /// Its radius about its centre and its period (bells a turn); zero for a
    /// core (cores do not orbit).
    pub radius: Ratio,
    pub period: Ratio,
    /// What the body itself holds (a star's, a core's, an orb's grains).
    pub kept: Manifest,
    pub satellites: Vec<Body>,
    /// The tier's unbound grains, beyond its last orbit.
    pub medium: Manifest,
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

/// One tier: the centre at `path` with `grains` keeps its tier's share, the
/// medium is set aside from the rest, and the satellites break what remains
/// inside `reach` (none: bounded by the grain alone), the first period
/// `unit`, then one tier down. What they leave joins the medium.
fn split(seed: &[u8; 32], path: Vec<u32>, mut grains: Manifest, reach: Option<Ratio>, unit: Ratio, unit_mass: &Ratio, depth: u8) -> (Manifest, Vec<Body>, Manifest) {
    let law = tier(depth);
    let centre = if depth == 2 { hole_share(seed, &path) } else { law.centre.clone() };
    let kept = take(&grains, &centre);
    less(&mut grains, &kept);
    let mut satellites = Vec::new();
    if depth == 0 {
        return (kept, satellites, grains);
    }
    let mut medium = take(&grains, &law.medium);
    less(&mut grains, &medium);
    let mut enclosed = mass(&kept) / unit_mass.clone();
    // THE TIDAL REACH (operator, 2026-10-14, xylarium's law): a body's reach
    // is where its own turning matches the shear of its anchor's across it,
    // `ρ² = r² m / (2M + m)`. Held as its share of radius, `s = ρ / r`, the
    // feeding zone closes on one mass, `m = μ · M(<r)` with
    // `μ = 2s² / (1 − s²)`, exact; reaches just touching place each next
    // satellite at `r · (1 + s) / (1 − s)`. The innermost orbit turns in one
    // unit of the tier (the bell ladder's anchor); each period follows from
    // the law, `P = r / M(<r)`. Satellites form until the formed matter is
    // spent or the next stands beyond the centre's reach; what remains is
    // the medium. Nothing is drawn.
    let s_ = law.reach.clone();
    let mu = ratio(2, 1) * s_.clone() * s_.clone() / (ratio(1, 1) - s_.clone() * s_.clone());
    let step = (ratio(1, 1) + s_.clone()) / (ratio(1, 1) - s_.clone());
    let mut radius: Option<Ratio> = None;
    for k in 1u32.. {
        let left = mass(&grains) / unit_mass.clone();
        let want = if enclosed.is_zero() { left.clone() } else { mu.clone() * enclosed.clone() };
        if left.is_zero() || want > left {
            break;
        }
        let m = take(&grains, &(want / left));
        let m_mass = mass(&m) / unit_mass.clone();
        if m_mass.is_zero() {
            break;
        }
        let r = match &radius {
            None if enclosed.is_zero() => unit.clone(),
            None => enclosed.clone() * unit.clone(),
            Some(last) => last.clone() * step.clone(),
        };
        if reach.as_ref().is_some_and(|bound| r > *bound) {
            break;
        }
        let period = if enclosed.is_zero() { unit.clone() } else { r.clone() / enclosed.clone() };
        less(&mut grains, &m);
        let mut at = path.clone();
        at.push(k);
        let own_reach = Some(r.clone() * s_.clone());
        let (s_kept, s_sats, s_medium) = split(seed, at.clone(), m, own_reach, unit.clone() / ratio(60, 1), unit_mass, depth - 1);
        satellites.push(Body { path: at, radius: r.clone(), period, kept: s_kept, satellites: s_sats, medium: s_medium });
        radius = Some(r);
        enclosed += m_mass;
    }
    for (k, a) in grains {
        *medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
    }
    (kept, satellites, medium)
}

/// The universe the pulse deploys: its cores, and the intergalactic medium.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universe {
    pub cores: Vec<Body>,
    pub medium: Manifest,
}

/// THE PULSE: the manifest deployed from `seed`. The unit of mass is the
/// deepest core's hole.
pub fn deploy(seed: &[u8; 32], manifest: &Manifest) -> Universe {
    // The epicenter keeps nothing; the intergalactic medium is set aside.
    let mut medium = take(manifest, &tier(3).medium);
    let mut formed = manifest.clone();
    less(&mut formed, &medium);
    // THE SPECTRUM (docs/78): the count of modes and each mode's strength
    // `a_i`, seed draws from the base-60 field (1 to 60). Core i takes
    // `a_i / Σa` of the formed matter, so sizes fall in no fixed order and
    // the largest is at most 60 times the smallest. A mode too weak to hold
    // a hole of a grain forms no core; its share stays unformed.
    let modes = 1 + (draw(seed, &[], "modes", 0) % 60) as u32;
    let strengths: Vec<i64> = (1..=modes).map(|i| 1 + (draw(seed, &[i], "strength", 0) % 60) as i64).collect();
    let sum: i64 = strengths.iter().sum();
    let mut shares: Vec<(u32, Manifest)> = Vec::new();
    for (i, a) in (1..=modes).zip(&strengths) {
        let m = take(&formed, &ratio(*a, sum));
        if take(&m, &hole_share(seed, &[i])).is_empty() {
            continue;
        }
        shares.push((i, m));
    }
    for (_, m) in &shares {
        less(&mut formed, m);
    }
    for (k, a) in formed {
        *medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
    }
    // The unit: the deepest core's hole (`M_core = 1`).
    let deepest = shares.iter().map(|(i, m)| mass(&take(m, &hole_share(seed, &[*i])))).max().unwrap_or_else(|| ratio(1, 1));
    // A core's reach waits for the cores' places (A1): until then its
    // systems are bounded by their minimum alone.
    let cores = shares
        .into_iter()
        .map(|(i, m)| {
            let (kept, satellites, medium) = split(seed, vec![i], m, None, ratio(60 * 60, 1), &deepest, 2);
            Body { path: vec![i], radius: ratio(0, 1), period: ratio(0, 1), kept, satellites, medium }
        })
        .collect();
    Universe { cores, medium }
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
    put_manifest(&b.kept, &mut v);
    put_manifest(&b.medium, &mut v);
    cfe::tlv::put(1, &v, out);
    for s in &b.satellites {
        put_body(s, out);
    }
}

/// `<var>/pulse.bin`: tag 1 a body (path, radius, period, kept, medium),
/// parents before their satellites; tag 2 the universe's medium; tag 3 the
/// seed.
pub fn write(var_root: &Path, seed: &[u8; 32], u: &Universe) -> std::io::Result<()> {
    let mut out = Vec::new();
    cfe::tlv::put(3, seed, &mut out);
    for c in &u.cores {
        put_body(c, &mut out);
    }
    let mut v = Vec::new();
    put_manifest(&u.medium, &mut v);
    cfe::tlv::put(2, &v, &mut out);
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

fn get_body(v: &[u8]) -> Option<Body> {
    let mut at = 0;
    let depth = *v.first()? as usize;
    at += 1;
    let mut path = Vec::with_capacity(depth.min(8));
    for _ in 0..depth {
        path.push(u32::from_le_bytes(v.get(at..at + 4)?.try_into().ok()?));
        at += 4;
    }
    let (radius, used) = cfe::ratio::decode(v.get(at..)?)?;
    at += used;
    let (period, used) = cfe::ratio::decode(v.get(at..)?)?;
    at += used;
    let kept = get_manifest(v, &mut at)?;
    let medium = get_manifest(v, &mut at)?;
    Some(Body { path, radius, period, kept, satellites: Vec::new(), medium })
}

/// Read `<var>/pulse.bin` back: the seed and the universe (bodies are
/// written parents first, so each attaches under the last body one level up).
pub fn read(var_root: &Path) -> Option<([u8; 32], Universe)> {
    let bytes = std::fs::read(var_root.join("pulse.bin")).ok()?;
    let mut seed = None;
    let mut medium = Manifest::new();
    let mut flat: Vec<Body> = Vec::new();
    for (t, v) in cfe::tlv::records(&bytes) {
        match t {
            3 => seed = v.try_into().ok(),
            2 => medium = get_manifest(v, &mut 0)?,
            1 => flat.push(get_body(v)?),
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
    Some((seed?, Universe { cores, medium }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pulse_keeps_every_grain() {
        let manifest: Manifest = [("a".to_string(), ratio(1000, 1)), ("b".to_string(), ratio(37, 64))].into();
        let u = deploy(&[7; 32], &manifest);
        assert_eq!(u.total(), manifest);
        assert!(!u.cores.is_empty());
        assert_eq!(deploy(&[7; 32], &manifest), u, "one seed, one universe");
        let dir = std::env::temp_dir().join(format!("pulse-{}", std::process::id()));
        write(&dir, &[7; 32], &u).unwrap();
        assert_eq!(read(&dir), Some(([7; 32], u)), "the file gives the universe back");
        let _ = std::fs::remove_dir_all(dir);
    }
}
