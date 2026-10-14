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
//! - Core: its network formers (two or more bonds) sink and shield; their
//!   share of its mass is `κ`.
//! - Atmosphere (Jeans escape, linear in every term): of each gas it holds
//!   (volatile atoms: closed, or bonding once, so they close in pairs; and
//!   plasma, atoms whose own grip the heat beats), the world keeps
//!   `1 − e^(−λ)`, `λ = A · g · (1 + κ) / (k · T)`: heavy gases on massive,
//!   cool, shielded worlds hold; light ones on small, hot, bare worlds go.
//!   `k` is calibrated on the median world: a hydrogen-weight gas (A = 2)
//!   there has λ = 1 (a placeholder until escape has a unit).
//! - Pressure `P = m_atm · g`, read as COMPRESSION against the median
//!   world's (a placeholder until pressure has a unit).
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
    /// The core's share of the world's mass, κ.
    pub core: Ratio,
    /// The hold on a gas of mass 2: `g · (1 + κ) / T`.
    pub hold: Ratio,
    /// The atmosphere kept (filled by `worlds`, after calibration).
    pub atmosphere: Ratio,
    pub pressure: Ratio,
    /// The world's gases: (mass number, grains).
    pub gases: Vec<(u32, Ratio)>,
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
    let gases: Vec<(u32, Ratio)> = atoms.iter().filter(|(a, _)| volatile(a) || plasma(a)).map(|(a, g)| (cfe::almanac::atom::mass_number(a.z), g.clone())).collect();
    let core_matter: Manifest = matter.iter().filter(|(k, _)| z_of(k).and_then(atom).is_some_and(|a| !plasma(&a) && a.unpaired >= 2)).map(|(k, g)| (k.clone(), g.clone())).collect();
    let core = mass(&core_matter) / mass(matter);
    let hold = gravity.clone() * (ratio(1, 1) + core.clone()) / heat.clone();
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
    Some(World { path, heat, size, gravity, core, hold, atmosphere: ratio(0, 1), pressure: ratio(0, 1), gases, chemical: chemical.map(|(_, x)| x), geologic, plasmic, informational })
}

/// `e^(−λ)` bounded above by whole halvings, `2^(−⌊λ · 1442/1000⌋)` (so the
/// kept share errs toward a thinner atmosphere).
fn lost(lambda: &Ratio) -> Ratio {
    let n: u32 = (lambda.clone() * ratio(1442, 1000)).floor().to_integer().try_into().unwrap_or(u32::MAX).min(64);
    Ratio::new(num_bigint::BigInt::from(1), num_bigint::BigInt::from(2u8).pow(n))
}

/// The atmosphere a world keeps, its hold calibrated by `k` (the median
/// world's hold: there a gas of mass 2 has λ = 1).
pub fn atmosphere(w: &World, hold_ref: &Ratio) -> Ratio {
    w.gases.iter().fold(ratio(0, 1), |s, (a, g)| {
        let lambda = ratio(i64::from(*a), 2) * w.hold.clone() / hold_ref.clone();
        s + g.clone() * ratio(i64::from(*a), 1) * (ratio(1, 1) - lost(&lambda))
    })
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
    // Calibrate the escape scale on the median world's hold, then keep each
    // world's atmosphere and its pressure.
    let mut holds: Vec<Ratio> = out.iter().map(|w| w.hold.clone()).collect();
    holds.sort();
    let hold_ref = holds.get(holds.len() / 2).cloned().filter(|h| !h.is_zero()).unwrap_or_else(|| ratio(1, 1));
    for w in out.iter_mut() {
        w.atmosphere = atmosphere(w, &hold_ref);
        w.pressure = w.atmosphere.clone() * w.gravity.clone();
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
        assert_eq!(w.gases, vec![(2, ratio(10, 1))], "hydrogen is the volatile");
        assert!(w.core > ratio(0, 1), "Z = 4 forms the core");
        // At its own hold as the reference, hydrogen has λ = 1: one halving
        // lost is the bound, so half is kept.
        assert_eq!(atmosphere(&w, &w.hold), ratio(10, 1));
    }
}
