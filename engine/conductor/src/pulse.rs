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
//! The tiers: the epicenter (no mass) deploys the cores; a core its systems
//! (unit period 60² bells); a star its orbs (unit 60 bells). Masses are in
//! units of core 1's mass (`M_core = 1`), radii in its gravitational
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

fn mass(m: &Manifest) -> Ratio {
    m.values().fold(ratio(0, 1), |s, a| s + a.clone())
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

/// A share `q/60`, q from 1 to 59 (one field of the base-60 odometer).
fn share(seed: &[u8; 32], path: &[u32], j: u32) -> Ratio {
    ratio(1 + (draw(seed, path, "share", j) % 59) as i64, 60)
}

/// The resonance step `(q+1)/q`, q from 1 to 60.
fn resonance(seed: &[u8; 32], path: &[u32], j: u32) -> Ratio {
    let q = 1 + (draw(seed, path, "period", j) % 60) as i64;
    ratio(q + 1, q)
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

/// One tier: the centre at `path` with `grains`, keeping `f_0` first (none
/// for the epicenter), its satellites inside `reach` (none: bounded by the
/// grain alone), the first period `unit`, then one tier down to `depth`.
fn split(seed: &[u8; 32], path: Vec<u32>, mut grains: Manifest, keeps: bool, reach: Option<Ratio>, unit: Ratio, unit_mass: &Ratio, depth: u8) -> (Manifest, Vec<Body>, Manifest) {
    let kept = if keeps { take(&grains, &share(seed, &path, 0)) } else { Manifest::new() };
    less(&mut grains, &kept);
    let mut satellites = Vec::new();
    if depth == 0 {
        return (kept, satellites, grains);
    }
    let mut enclosed = mass(&kept) / unit_mass.clone();
    let mut period = unit.clone();
    for k in 1u32.. {
        let m = take(&grains, &share(seed, &path, k));
        let m_mass = mass(&m) / unit_mass.clone();
        if m_mass.is_zero() {
            break;
        }
        if k > 1 {
            period = period * resonance(seed, &path, k);
        }
        // The epicenter encloses no mass: its cores stand at their periods'
        // lengths (they do not orbit; their places are formed at A1).
        let radius = if enclosed.is_zero() { period.clone() } else { enclosed.clone() * period.clone() };
        if reach.as_ref().is_some_and(|r| radius > *r) {
            break;
        }
        less(&mut grains, &m);
        let mut at = path.clone();
        at.push(k);
        // The satellite's own reach: r_d = r · m / M(<r).
        let own_reach = (!enclosed.is_zero()).then(|| radius.clone() * m_mass.clone() / enclosed.clone());
        let (s_kept, s_sats, s_medium) = split(seed, at.clone(), m, true, own_reach, unit.clone() / ratio(60, 1), unit_mass, depth - 1);
        let cores = path.is_empty();
        satellites.push(Body { path: at, radius: if cores { ratio(0, 1) } else { radius }, period: if cores { ratio(0, 1) } else { period.clone() }, kept: s_kept, satellites: s_sats, medium: s_medium });
        enclosed += m_mass;
    }
    (kept, satellites, grains)
}

/// The universe the pulse deploys: its cores, and the intergalactic medium.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universe {
    pub cores: Vec<Body>,
    pub medium: Manifest,
}

/// THE PULSE: the manifest deployed from `seed`. The unit of mass is core 1's
/// own (its kept grains), read first from the same draws.
pub fn deploy(seed: &[u8; 32], manifest: &Manifest) -> Universe {
    // Core 1's grains: the epicenter's first share, of which core 1 keeps its own.
    let first = take(manifest, &share(seed, &[], 1));
    let core_one = mass(&take(&first, &share(seed, &[1], 0)));
    let unit_mass = if core_one.is_zero() { ratio(1, 1) } else { core_one };
    let (_, cores, medium) = split(seed, Vec::new(), manifest.clone(), false, None, ratio(60 * 60 * 60, 1), &unit_mass, 3);
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
    }
}
