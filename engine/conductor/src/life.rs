//! docs/82, step 1: what each world offers life. Per world (an orb of the
//! pulse universe): its heat, size, gravity, atmosphere and pressure, and
//! for each MEDIUM it holds (chemical, geologic, plasmic, informational)
//! how well that medium carries a pattern, in HALVINGS of its copy error.
//! Nothing is assumed about life's form (docs/78, design law 10).
//!
//! - Heat: `T = K / P`, P the world's turn in bells, K = 60,000 K·bells
//!   (the innermost world of a star, one tier unit, at 1,000 K; an anchor).
//! - Size: each nucleus's atomic size (the atom law's outer orbit,
//!   `Q_out · P_out`) summed over the world's grains; gravity `g = M / R`
//!   (mass by `A = 2Z`).
//! - Atmosphere: its volatile atoms (closed, or bonding once: they close in
//!   pairs) and its plasma (atoms whose own grip the heat beats); pressure
//!   `P = m_atm · g`, read as COMPRESSION against the median world's (a
//!   placeholder until pressure has a unit).
//! - A medium carries a pattern where `x = cohesion · (1 + compression) /
//!   agitation`, and its copy error is `e^(−x)`: held as halvings, `x/ln 2`,
//!   read by the lower bound `1/ln 2 ≥ 1442/1000` (toward fewer minds). A
//!   medium CARRIES when its error is below one half (a halving or more).
use crate::pulse::{mass, Manifest, Universe};
use cfe::almanac::atom::{atom, closes, Atom};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// The heat anchor: K·bells.
pub const K: i64 = 60_000;
/// The hydrogen anchor: an atom of tightness 1 holds to 157,800 K.
pub const HYDROGEN_K: i64 = 157_800;

/// What a world offers.
#[derive(Debug, Clone)]
pub struct World {
    pub path: Vec<u32>,
    pub heat: Ratio,
    pub size: Ratio,
    pub gravity: Ratio,
    pub pressure: Ratio,
    /// x before compression, per medium, if the world holds it.
    pub chemical: Option<Ratio>,
    pub geologic: Option<Ratio>,
    pub plasmic: Option<Ratio>,
    pub informational: Option<Ratio>,
}

fn z_of(key: &str) -> Option<u32> {
    key.strip_prefix('Z').and_then(|z| z.parse().ok())
}

/// A grip in kelvin: `τ · 157,800`.
fn grip(a: &Atom) -> Ratio {
    a.tightness.clone() * ratio(HYDROGEN_K, 1)
}

/// Read one world: its matter, its turn (bells), and its clock against its
/// star (the coupling's rate).
pub fn read(path: Vec<u32>, matter: &Manifest, period: &Ratio, coupling: &Ratio) -> Option<World> {
    if period.is_zero() || matter.is_empty() {
        return None;
    }
    let heat = ratio(K, 1) / period.clone();
    let atoms: Vec<(Atom, Ratio)> = matter.iter().filter_map(|(k, g)| z_of(k).and_then(atom).map(|a| (a, g.clone()))).collect();
    let size = atoms.iter().fold(ratio(0, 1), |s, (a, g)| s + g.clone() * ratio(i64::from(a.enclosed * a.depth), 1));
    if size.is_zero() {
        return None;
    }
    let gravity = mass(matter) / size.clone();
    let plasma = |a: &Atom| grip(a) < heat;
    let volatile = |a: &Atom| !plasma(a) && a.unpaired <= 1;
    let atmosphere: Manifest = matter.iter().filter(|(k, _)| z_of(k).and_then(atom).is_some_and(|a| volatile(&a) || plasma(&a))).map(|(k, g)| (k.clone(), g.clone())).collect();
    let pressure = mass(&atmosphere) * gravity.clone();
    // CHEMICAL: the most abundant closing pair whose weaker grip holds.
    let bonders: Vec<&(Atom, Ratio)> = atoms.iter().filter(|(a, _)| !plasma(a) && a.unpaired > 0).collect();
    let mut chemical: Option<(Ratio, Ratio)> = None;
    for (i, (a, ga)) in bonders.iter().enumerate() {
        for (b, gb) in bonders.iter().skip(i) {
            if !closes(&[a.z, b.z]) {
                continue;
            }
            let hold = grip(a).min(grip(b));
            if hold <= heat {
                continue;
            }
            let share = ga.clone().min(gb.clone());
            if chemical.as_ref().is_none_or(|(s, _)| share > *s) {
                chemical = Some((share, hold / heat.clone()));
            }
        }
    }
    // GEOLOGIC: the most abundant network former (two or more bonds) the
    // heat does not melt to plasma.
    let geologic = atoms.iter().filter(|(a, _)| !plasma(a) && a.unpaired >= 2).max_by(|x, y| x.1.cmp(&y.1)).map(|(a, _)| grip(a) / heat.clone());
    // INFORMATIONAL: the coupling's coherence over its slip, `rate / (1 − rate)`.
    let informational = (*coupling < ratio(1, 1)).then(|| coupling.clone() / (ratio(1, 1) - coupling.clone()));
    // PLASMIC: the field the coupling sets holding the ionised share.
    let plasma_share = {
        let all = mass(matter);
        let ionised: Manifest = matter.iter().filter(|(k, _)| z_of(k).and_then(atom).is_some_and(|a| plasma(&a))).map(|(k, g)| (k.clone(), g.clone())).collect();
        if all.is_zero() { ratio(0, 1) } else { mass(&ionised) / all }
    };
    let plasmic = informational.clone().filter(|_| plasma_share > ratio(0, 1)).map(|x| x * plasma_share);
    Some(World { path, heat, size, gravity, pressure, chemical: chemical.map(|(_, x)| x), geologic, plasmic, informational })
}

/// Halvings of a medium's copy error at compression c: `x · (1 + c) · 1442/1000`.
pub fn halvings(x: &Ratio, compression: &Ratio) -> Ratio {
    x.clone() * (ratio(1, 1) + compression.clone()) * ratio(1442, 1000)
}

/// Every world of the universe, read.
pub fn worlds(u: &Universe) -> Vec<World> {
    let mut out = Vec::new();
    for core in &u.cores {
        for sys in &core.satellites {
            let star = mass(&sys.kept);
            let mut inside = star.clone();
            for orb in &sys.satellites {
                let coupling = crate::breath::rate(&orb.radius, &(inside.clone() / crate::breath::unit_mass(u)));
                if let Some(w) = read(orb.path.clone(), &orb.kept, &orb.period, &coupling) {
                    out.push(w);
                }
                inside += mass(&orb.total());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_world_reads_its_heat_and_media() {
        // Hydrogen (single bond) and oxygen-like Z = 4 (two bonds) at 60 bells.
        let matter: Manifest = [("Z001".to_string(), ratio(10, 1)), ("Z004".to_string(), ratio(5, 1))].into();
        let w = read(vec![1, 1, 1], &matter, &ratio(60, 1), &ratio(59, 60)).unwrap();
        assert_eq!(w.heat, ratio(1000, 1));
        assert!(w.chemical.is_some(), "1 + 4 close and hold at 1,000 K");
        assert!(w.geologic.is_some());
        assert_eq!(w.informational, Some(ratio(59, 1)));
        assert!(w.pressure > ratio(0, 1), "hydrogen is volatile");
    }
}
