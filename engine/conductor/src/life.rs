//! docs/82, step 1: what each world offers life. Per world (an orb of the
//! pulse universe): its heat, size, gravity, atmosphere and pressure, and
//! for each medium it holds (chemical, geologic, plasmic, informational)
//! how well that medium carries a pattern, in halvings of its copy error.
//! Nothing is assumed about life's form (docs/78, design law 10).
//!
//! - Compression (operator 2026-10-14: no caps, a world's gravity must grow
//!   with it): a body's own weight squeezes it, `R = R_loose / (1 + M/M_a)`,
//!   R_loose its nuclei's atomic sizes summed (the atom law's outer orbit,
//!   `Q_out · P_out`), `M_a` its tier's compression scale (the median body
//!   of the tier, a placeholder). Linear homology made every body a scaled
//!   copy (gravity fixed by composition, every star's innermost world at
//!   one heat); compression breaks it.
//! - Heat: the world's own heat in the universe (`Body.heat`: born from its
//!   disc's light, fed by its star's light through its unlocked face
//!   (`breath::shine`), cooling to the void each bell), read in kelvin on the
//!   hydrogen grip (`T = heat · HYDROGEN_K`): the heat `open_at` and
//!   `layer_chemistry` read, one scale (docs/89, B1).
//! - Its matter is its layers as they stand: the clusters its chemistry made
//!   (water among them) as well as its atoms.
//! - Gravity `g = M / R` (mass by `A = 2Z`), growing with the world's mass.
//! - Core: its network formers (two or more bonds) sink and shield; their
//!   share of its mass is `κ`.
//! - Atmosphere (Jeans escape, linear in every term): of each gas it holds
//!   (volatile atoms: closed, or bonding once, so they close in pairs; and
//!   plasma, atoms whose own grip the heat beats), the world keeps
//!   `1 − e^(−λ)`, `λ = A · g · (1 + κ) / (k · T)`: heavy gases on massive,
//!   cool, shielded worlds hold; light ones on small, hot, bare worlds go.
//!   `k` is calibrated on the median world: a hydrogen-weight gas (A = 2)
//!   there has λ = 1 (a placeholder until escape has a unit).
//! - Pressure `P = m_atm · g`, read as compression against the median
//!   world's (a placeholder until pressure has a unit).
//! - A medium carries a pattern where `x = cohesion · (1 + compression) /
//!   agitation`, and its copy error is `e^(−x)`: held as halvings, `x/ln 2`,
//!   read by the lower bound `1/ln 2 ≥ 1442/1000` (toward fewer minds). A
//!   medium carries when its error is below one half (a halving or more).
use crate::pulse::{mass, Manifest, Universe};
use cfe::almanac::atom::Atom;
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// The hydrogen anchor: an atom of tightness 1 holds to 157,800 K (its
/// grip: what a bond holds against heat, for phase and plasma).
pub const HYDROGEN_K: i64 = 157_800;
/// A contact, what a copy fails on (docs/82 step 2): far weaker than a
/// bond's grip. Anchored on water's copy error, about 1e-4 at 300 K
/// (Xylarium's life.rs): a contact of tightness 1 holds about 2,760 K.
pub const CONTACT_K: i64 = 2_760;
/// The doublings from a bare lineage to a mind's blueprint, `log2(B / b)`:
/// B of order a genome (6e9 bits), b of order a first replicator (100
/// bits), so 26 (anchors).
pub const BLUEPRINT_DOUBLINGS: i64 = 26;

/// What a world offers.
#[derive(Debug, Clone)]
pub struct World {
    pub path: Vec<u64>,
    pub heat: Ratio,
    /// Its star's burn against the home star (a luminosity, home = 1): read
    /// only by the era-0 thermal stand-in (`seating::site_of`, docs/89 B4).
    pub burn: Ratio,
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
    /// The planet's clock against its star's (the coupling), and its
    /// galaxy's echo delay in bells.
    pub coupling: Ratio,
    pub delay: u64,
    /// The system's belts and comet clouds (its medium) as a share of the
    /// system: the impacts' source.
    pub belts: Ratio,
    /// The echoes before the star no longer dominates its system (the
    /// climb's window).
    pub window: u64,
    /// Its days a year: the turns its clock slips against its star's in one
    /// orbit, `(1 − coupling)·P/T_u` (its recoherences a year); None where
    /// the universe has no echo.
    pub days: Option<Ratio>,
}

fn z_of(key: &crate::chem::Kind) -> Option<u32> {
    crate::chem::atom_w(key)
}

/// A grip in kelvin: `τ · 157,800`.
fn grip(a: &Atom) -> Ratio {
    a.tightness.clone() * ratio(HYDROGEN_K, 1)
}

/// Read one world: its matter, its own heat in kelvin (`Body.heat ·
/// HYDROGEN_K`), and its clock against its star (the coupling's rate). None
/// where it is cold through (nothing agitates it) or holds nothing.
#[allow(clippy::too_many_arguments)]
pub fn read(path: Vec<u64>, matter: &Manifest, heat: &Ratio, coupling: &Ratio, star_burn: &Ratio, world_scale: &Ratio, delay: u64) -> Option<World> {
    if heat.is_zero() || matter.is_empty() {
        return None;
    }
    let heat = heat.clone();
    let atoms: Vec<(Atom, Ratio)> = matter.iter().filter_map(|(k, g)| z_of(k).and_then(crate::chem::atom_of).map(|a| (a, g.clone()))).collect();
    // Every kind's atoms are its size, a cluster's by its parts
    // (`chem::components`): a world of compounds is not empty.
    let loose = matter.iter().fold(ratio(0, 1), |s, (k, g)| {
        let sized = crate::chem::components(k).iter().filter_map(|&(n, z)| crate::chem::atom_of(z).map(|a| i64::from(n) * i64::from(a.enclosed * a.depth))).sum::<i64>();
        s + g.clone() * ratio(sized, 1)
    });
    if loose.is_zero() {
        return None;
    }
    // Normalised to the home world (Xylarium's ruler, shared 2026-10-14):
    // `R = 2 · R_loose / (1 + r)`, r its mass over the home world's.
    let size = ratio(2, 1) * loose / (ratio(1, 1) + mass(matter) / world_scale.clone());
    let gravity = mass(matter) / size.clone();
    let plasma = |a: &Atom| grip(a) < heat;
    let volatile = |a: &Atom| !plasma(a) && a.unpaired <= 1;
    // The volatiles: closed atoms or those bonding once, plasma, and the
    // closed clusters its chemistry made (they bond no further), by their
    // holding (`chem::hold`).
    let mut gases: Vec<(u32, Ratio)> = atoms.iter().filter(|(a, _)| volatile(a) || plasma(a)).map(|(a, g)| (cfe::almanac::atom::hold(a.z), g.clone())).collect();
    gases.extend(matter.iter().filter(|(k, _)| z_of(k).is_none() && crate::chem::unpaired(k) == 0).map(|(k, g)| (crate::chem::hold(k), g.clone())));
    let core_matter: Manifest = matter.iter().filter(|(k, _)| z_of(k).and_then(crate::chem::atom_of).is_some_and(|a| !plasma(&a) && a.unpaired >= 2)).map(|(k, g)| (k.clone(), g.clone())).collect();
    let core = mass(&core_matter) / mass(matter);
    let hold = gravity.clone() * (ratio(1, 1) + core.clone()) / heat.clone();
    // Chemical: the compound it lives in. First the closed clusters its
    // layers already hold (`formation::layer_chemistry` made them, water
    // among them) whose grip the heat does not beat; then the pairs that
    // `chem::closing` closes at this heat (`open_at`). The most abundant.
    let tight_k = |k: &crate::chem::Kind| crate::chem::tightness(k) * ratio(HYDROGEN_K, 1);
    let mut chemical: Option<(Ratio, Ratio)> = None;
    let mut consider = |share: Ratio, contact_tightness: Ratio| {
        if chemical.as_ref().is_none_or(|(s, _)| share > *s) {
            chemical = Some((share, contact_tightness * ratio(CONTACT_K, 1) / heat.clone()));
        }
    };
    for (k, g) in matter.iter() {
        if crate::chem::atom_w(k).is_none() && tight_k(k) > heat {
            consider(g.clone(), crate::chem::tightness(k));
        }
    }
    let grip_units = heat.clone() / ratio(HYDROGEN_K, 1);
    let bonders: Vec<&(Atom, Ratio)> = atoms.iter().filter(|(a, _)| !plasma(a) && a.unpaired > 0).collect();
    for (i, (a, ga)) in bonders.iter().enumerate() {
        for (b, gb) in bonders.iter().skip(i) {
            let (ua, ub) = (cfe::almanac::atom::open_at(a.z, &grip_units), cfe::almanac::atom::open_at(b.z, &grip_units));
            if crate::chem::closing(a.z, ua, b.z, ub).is_none() {
                continue;
            }
            if grip(a).min(grip(b)) <= heat {
                continue;
            }
            consider(ga.clone().min(gb.clone()), a.tightness.clone().min(b.tightness.clone()));
        }
    }
    // Geologic: the most abundant network former (two or more bonds) the
    // heat does not melt to plasma.
    // A lattice holds a pattern by as many contacts as the former bonds.
    let geologic = atoms.iter().filter(|(a, _)| !plasma(a) && a.unpaired >= 2).max_by(|x, y| x.1.cmp(&y.1)).map(|(a, _)| a.tightness.clone() * ratio(CONTACT_K * i64::from(a.unpaired), 1) / heat.clone());
    // Informational: the coupling's coherence over its slip, `rate / (1 − rate)`.
    let informational = (*coupling < ratio(1, 1)).then(|| coupling.clone() / (ratio(1, 1) - coupling.clone()));
    // Plasmic: the field the coupling sets holding the ionised share.
    let plasma_share = {
        let all = mass(matter);
        let ionised: Manifest = matter.iter().filter(|(k, _)| z_of(k).and_then(crate::chem::atom_of).is_some_and(|a| plasma(&a))).map(|(k, g)| (k.clone(), g.clone())).collect();
        if all.is_zero() { ratio(0, 1) } else { mass(&ionised) / all }
    };
    let plasmic = informational.clone().filter(|_| plasma_share > ratio(0, 1)).map(|x| x * plasma_share);
    Some(World { path, heat, burn: star_burn.clone(), size, gravity, core, hold, atmosphere: ratio(0, 1), pressure: ratio(0, 1), gases, chemical: chemical.map(|(_, x)| x), geologic, plasmic, informational, coupling: coupling.clone(), delay, belts: ratio(0, 1), window: u64::MAX, days: None })
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

/// Step 2, the climb (docs/82). The ladder's rungs are written at the
/// coupling's recoherences (each time the planet's slip against its star
/// completes a turn, the coupling locks and a pattern can be written): a
/// medium of halvings h writes a rung of `h` doublings, so a mind's
/// blueprint needs `⌈D / h⌉` rungs (D the blueprint's doublings). Decay: a
/// medium of cohesion x forgets `1/x` of its progress while it waits, so it
/// needs `⌈rungs / (1 − 1/x)⌉` recoherences. A medium with less than a
/// halving (x ≤ ln 2) carries nothing. Returns the recoherences needed.
pub fn recoherences_needed(x: &Ratio, compression: &Ratio) -> Option<u64> {
    let h = halvings(x, compression);
    if h < ratio(1, 1) {
        return None;
    }
    let rungs = (ratio(BLUEPRINT_DOUBLINGS, 1) / h).ceil();
    let effective = x.clone() * (ratio(1, 1) + compression.clone());
    let keep = ratio(1, 1) - ratio(1, 1) / effective;
    if keep <= ratio(0, 1) {
        return None;
    }
    (rungs / keep).ceil().to_integer().try_into().ok()
}

/// The universal bell a mind wakes: the k-th recoherence of the world's
/// coupling (its slip per echo `1 − coupling`), echoes `t_u` apart after
/// its galaxy's delay. None when the coupling never slips.
pub fn wake_bell(w: &World, k: u64, t_u: u64) -> Option<u64> {
    let slip = ratio(1, 1) - w.coupling.clone();
    if slip <= ratio(0, 1) {
        return None;
    }
    let echoes: u64 = (ratio(k as i64, 1) / slip).ceil().to_integer().try_into().ok()?;
    Some(w.delay + echoes.checked_mul(t_u)?)
}

/// The home reference bodies (masses) and the home world's turn (bells).
pub struct Home {
    pub star: Ratio,
    pub world: Ratio,
    pub year_bells: Ratio,
    /// The home world's own heat in kelvin (`Body.heat · HYDROGEN_K`).
    pub heat: Ratio,
}

/// The home galaxy is the deepest core (the largest hole, the unit of mass)
/// among those that have formed A WORLD (formation, docs/83: a galaxy holds
/// no system until its gas collapses); its innermost star that holds a world
/// and that star's innermost world are the rulers (the breath moves and
/// empties systems, so the first in order need not hold one).
pub fn home(u: &Universe) -> Option<Home> {
    let core = u.cores.iter().filter(|c| c.satellites.iter().any(|s| !s.satellites.is_empty())).max_by(|a, b| mass(&a.kept).cmp(&mass(&b.kept)))?;
    let sys = core.satellites.iter().filter(|s| !s.satellites.is_empty()).min_by(|a, b| a.radius.cmp(&b.radius))?;
    let orb = sys.satellites.iter().min_by(|a, b| a.radius.cmp(&b.radius))?;
    Some(Home { star: mass(&sys.kept), world: mass(&orb.kept), year_bells: orb.period.clone(), heat: orb.heat.clone() * ratio(HYDROGEN_K, 1) })
}

/// The bell (Xylarium's ladder, shared): the home core's ring, the pulse's
/// round trip to the disc's edge and back at light speed, about 97,850
/// standard years. The standard year is the home world's local year (the
/// system ruler).
pub const BELL_YEARS: i64 = 97_850;
/// What one copier carries before a body must (cells: a generation is a
/// local day), and a mind's blueprint (bodies: a generation is a local
/// year): of order a cell's genome and an animal's (anchors).
pub const CELL_BITS: i64 = 10_000_000;
pub const GENOME_BITS: i64 = 6_000_000_000;
/// A viable people (Franklin's ~500), counted in nuclei (operator
/// 2026-10-14): a mind's carrier needs about a nucleus of its medium a bit of
/// its blueprint, so 500 minds need `500 · GENOME_BITS` nuclei of it. A grain
/// is 60^30 nuclei, so matter never limits a people; its energy (its star's
/// flux) does, in the economy (docs/74). The gate is exact and kept.
pub const VIABLE_PEOPLE: i64 = 500;

/// Whether a world's matter can carry a viable people (exact, in nuclei).
pub fn viable(matter: &Manifest) -> bool {
    let grains = matter.values().fold(ratio(0, 1), |s, a| s + a.clone() * ratio(1024, 1));
    let nuclei = grains.floor().to_integer() * crate::pulse::nuclei_per_grain();
    nuclei >= num_bigint::BigInt::from(VIABLE_PEOPLE) * num_bigint::BigInt::from(GENOME_BITS)
}

/// The bells a medium's climb takes on a world (docs/82, with xylarium's
/// ruler): selection writes at most a bit a generation, cells a local day a
/// generation up to a copier's capacity, bodies a local year for the rest;
/// decay stretches it by `1 / (1 − 1/x)`. Its local year in standard years
/// is its turn over the home world's; a bell is BELL_YEARS of them. Its day
/// is its own (`days` a year, its clock's slips against its star in one
/// orbit): a world with no day (locked to its star) gives cells no
/// generations, and climbs nothing.
pub fn climb_bells(period_bells: &Ratio, home_year_bells: &Ratio, days: &Ratio, x: &Ratio, compression: &Ratio) -> Option<Ratio> {
    let _rungs = recoherences_needed(x, compression)?;
    if *days <= ratio(0, 1) {
        return None;
    }
    let year = period_bells.clone() / home_year_bells.clone();
    let cells = ratio(CELL_BITS, 1) * year.clone() / days.clone();
    let bodies = ratio(GENOME_BITS - CELL_BITS, 1) * year;
    let effective = x.clone() * (ratio(1, 1) + compression.clone());
    let keep = ratio(1, 1) - ratio(1, 1) / effective;
    if keep <= ratio(0, 1) {
        return None;
    }
    Some((cells + bodies) / keep / ratio(BELL_YEARS, 1))
}

/// The WINDOW: a star sheds about `ε/2` of itself an echo (its coherence
/// averaging a half; docs/78, the breath), so it stays above its worlds'
/// mass for `log2(M_star / M_worlds) · ln 2 · 2/ε` echoes: counted in whole
/// halvings (`⌊log2⌋`) with `ln 2 ≥ 693/1000`, erring short.
pub fn window_echoes(star: &Ratio, worlds: &Ratio) -> u64 {
    if worlds.is_zero() || star <= worlds {
        return 0;
    }
    let q = (star.clone() / worlds.clone()).floor().to_integer();
    let halvings = q.bits().saturating_sub(1);
    let epsilon = crate::pulse::tier(1).medium;
    (ratio(halvings as i64, 1) * ratio(693, 1000) * ratio(2, 1) / epsilon).floor().to_integer().try_into().unwrap_or(0)
}

/// A world's twist a bell (docs/100 §2): its galaxy's core's winding from its
/// clock, plane by plane, as the world's own planes take it (its tilt), the
/// home's on the source's clock; never drawn, never flattened to one plane.
/// One reading for every impact law (`climb`, `readings`).
pub fn twist_of(u: &Universe, path: &[u64]) -> Ratio {
    let galaxy = path.first().copied();
    let body_tilt = u.cores.iter().find(|c| c.path.first().copied() == galaxy)
        .and_then(|c| c.satellites.iter().find(|s| s.path.get(1) == path.get(1)))
        .and_then(|s| s.satellites.iter().find(|o| o.path.get(2) == path.get(2)))
        .map(|o| o.tilt.clone()).unwrap_or_default();
    u.cores.iter().position(|c| c.path.first().copied() == galaxy).map(|i| {
        let rate = crate::breath::galaxy_rate(u, i, &crate::breath::unit_mass(u)).unwrap_or_else(|| ratio(1, 1));
        crate::quantum::twist(&rate, &u.cores[i].tilt, &body_tilt)
    }).unwrap_or_else(|| ratio(0, 1))
}

/// The climb with erasure (docs/82): recoherence by recoherence, each
/// writing a rung unless an impact strikes there (its chance the system's
/// belt share, shielded by the atmosphere: `belts / (1 + compression)`),
/// MEASURED on the world's hidden phase, its galaxy's twist and its own turn
/// (`quantum::hidden_phase`, the one impact law `advance` reads; docs/102 A4:
/// never drawn) — an impact wastes that recoherence, the rung's need kept,
/// life never undone. The mind wakes at the recoherence its last rung is
/// written, if within the star's window; else never.
#[allow(clippy::too_many_arguments)]
pub fn climb(w: &World, x: &Ratio, compression: &Ratio, twist: &Ratio, t_u: u64, period_bells: &Ratio, home_year_bells: &Ratio) -> Option<u64> {
    let bells = climb_bells(period_bells, home_year_bells, w.days.as_ref()?, x, compression)?;
    let slip = ratio(1, 1) - w.coupling.clone();
    if slip <= ratio(0, 1) {
        return None;
    }
    // The mind is locked in at a recoherence of the coupling: the first
    // after its climb completes, each impact at a recoherence before then
    // costing that interval again (the rung's need kept). Within the star's
    // window, or never.
    let last: u64 = (ratio(w.window.min(u32::MAX as u64) as i64, 1) * slip.clone()).floor().to_integer().try_into().unwrap_or(0);
    let p = w.belts.clone() / (ratio(1, 1) + compression.clone());
    let mut due = ratio(w.delay as i64, 1) + bells;
    for k in 1..=last {
        let at = wake_bell(w, k, t_u)?;
        if ratio(at as i64, 1) < due {
            if crate::quantum::measure(&crate::quantum::hidden_phase(twist, at, period_bells), &p) {
                let next = wake_bell(w, k + 1, t_u)?;
                due += ratio((next - at) as i64, 1);
            }
            continue;
        }
        return Some(at);
    }
    None
}

/// A mind due to wake: its bell, its world and its medium.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Waking {
    pub bell: u64,
    pub path: Vec<u64>,
    pub medium: &'static str,
}

/// The climb as STATE (docs/82 step 3b, operator 2026-10-14: wakings come
/// from the system's activity, never a boot-time schedule). Each world
/// keeps its climb: the share of its climb written, its recoherence count,
/// the bell of its last and next recoherence. At each recoherence of its
/// coupling the world reads its current state and writes the bells since
/// its last over its climb's length now (an impact there writes nothing,
/// the rung's need kept); it wakes when the share reaches one, within its
/// star's window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Climb {
    pub written: Ratio,
    pub k: u64,
    pub last: u64,
    pub next: u64,
    pub woken: bool,
}

pub type Climbs = std::collections::BTreeMap<Vec<u64>, Climb>;

/// One world's current reading for the climb: the world, its compression,
/// and its fastest medium's climb length in bells (none: it cannot climb).
pub struct Reading {
    pub world: World,
    pub compression: Ratio,
    pub climb: Option<(Ratio, &'static str)>,
    /// Its turn about its star (bells) and its galaxy's twist a bell (docs/100
    /// §2, derived): the hidden phase of its interactions (docs/79).
    pub period: Ratio,
    pub torsion: Ratio,
}

/// The median world's pressure: compression is read against it (`readings`).
/// One where no world holds an atmosphere (the reading's absence: nothing
/// compresses).
pub fn reference_pressure(worlds: &[World]) -> Ratio {
    let mut pressures: Vec<Ratio> = worlds.iter().map(|w| w.pressure.clone()).collect();
    pressures.sort();
    pressures.get(pressures.len() / 2).cloned().filter(|p| *p > ratio(0, 1)).unwrap_or_else(|| ratio(1, 1))
}

/// A people's medium, from its metabolism in the registry's vocabulary (the
/// waking's map read back): mineral minds are geologic, biotic chemical,
/// plasmic plasmic.
pub fn medium_of(metabolism: &str) -> Option<&'static str> {
    match metabolism {
        "mineral" => Some("geologic"),
        "biotic" => Some("chemical"),
        "plasmic" => Some("plasmic"),
        _ => None,
    }
}

/// The home world's heat, the land's ruler: its own `Body.heat` in kelvin.
pub fn home_heat(u: &Universe) -> Option<Ratio> {
    home(u).map(|h| h.heat).filter(|h| !h.is_zero())
}

/// A medium's halvings on a world at its heat and compression; None where
/// the world does not hold the medium.
pub fn medium_halvings(w: &World, compression: &Ratio, medium: &str) -> Option<Ratio> {
    let x = match medium {
        "chemical" => w.chemical.as_ref(),
        "geologic" => w.geologic.as_ref(),
        "plasmic" => w.plasmic.as_ref(),
        _ => None,
    }?;
    Some(halvings(x, compression))
}

/// The land (docs/86, re-pointed by docs/89): the heads a world feeds wild
/// in one medium. What limits a people is energy, its star's flux on its
/// world (docs/82): the world's own heat, kept from the light it receives
/// through its unlocked face (`breath::shine`). Its medium takes up the share
/// it carries as pattern, `1 − 2^(−⌊h⌋)` (whole halvings, none under one).
/// The ruler is the home world, whose heat feeds one viable people
/// (`VIABLE_PEOPLE`, a declared placeholder anchor):
/// `wild = ⌊VIABLE_PEOPLE · T · (1 − 2^(−⌊h⌋)) / T_home⌋`, heads whole.
pub fn wild(w: &World, compression: &Ratio, medium: &str, home_heat: &Ratio) -> Ratio {
    let Some(h) = medium_halvings(w, compression, medium) else { return ratio(0, 1) };
    if home_heat.is_zero() {
        return ratio(0, 1);
    }
    let whole: u32 = h.floor().to_integer().try_into().unwrap_or(u32::MAX).min(64);
    let carried = ratio(1, 1) - Ratio::new(num_bigint::BigInt::from(1), num_bigint::BigInt::from(2u8).pow(whole));
    (ratio(VIABLE_PEOPLE, 1) * w.heat.clone() * carried / home_heat.clone()).floor()
}

/// Every world's current reading (refreshed when the universe changes).
pub fn readings(u: &Universe, _seed: &[u8; 32]) -> std::collections::BTreeMap<Vec<u64>, Reading> {
    let worlds = worlds(u);
    let mut out = std::collections::BTreeMap::new();
    let Some(home) = home(u) else { return out };
    let reference = reference_pressure(&worlds);
    for w in worlds {
        let period = (|| {
            let core = u.cores.iter().find(|c| c.path.first() == w.path.first())?;
            let sys = core.satellites.iter().find(|s| s.path.get(1) == w.path.get(1))?;
            sys.satellites.iter().find(|o| o.path.get(2) == w.path.get(2)).map(|o| o.period.clone())
        })();
        let compression = w.pressure.clone() / reference.clone();
        let matter = (|| {
            let core = u.cores.iter().find(|c| c.path.first() == w.path.first())?;
            let sys = core.satellites.iter().find(|s| s.path.get(1) == w.path.get(1))?;
            sys.satellites.iter().find(|o| o.path.get(2) == w.path.get(2)).map(|o| o.kept.clone())
        })();
        // Its galaxy's twist a bell (docs/100 §2): derived, never drawn.
        let torsion = twist_of(u, &w.path);
        let turn = period.clone().unwrap_or_else(|| ratio(0, 1));
        let climb = period.filter(|_| matter.as_ref().is_some_and(viable)).and_then(|p| {
            [("chemical", &w.chemical), ("geologic", &w.geologic), ("plasmic", &w.plasmic)]
                .into_iter()
                .filter_map(|(m, x)| x.as_ref().and_then(|x| climb_bells(&p, &home.year_bells, w.days.as_ref()?, x, &compression)).map(|b| (b, m)))
                .min_by(|a, b| a.0.cmp(&b.0))
        });
        out.insert(w.path.clone(), Reading { world: w, compression, climb, period: turn, torsion });
    }
    out
}

/// Advance every world's climb to `bell`, reading each at its own
/// recoherences; returns the minds that wake.
pub fn advance(readings: &std::collections::BTreeMap<Vec<u64>, Reading>, climbs: &mut Climbs, t_u: u64, bell: u64) -> (Vec<Waking>, u64) {
    let mut woke = Vec::new();
    let mut measured = 0u64;
    for (path, r) in readings {
        let w = &r.world;
        let climb = climbs.entry(path.clone()).or_insert_with(|| Climb { written: ratio(0, 1), k: 1, last: w.delay, next: wake_bell(w, 1, t_u).unwrap_or(u64::MAX), woken: false });
        if climb.woken {
            continue;
        }
        let close = w.delay.saturating_add(w.window.saturating_mul(t_u));
        while climb.next <= bell && climb.next <= close {
            let at = climb.next;
            if let Some((length, medium)) = &r.climb {
                // The impact is A measurement (docs/79): it strikes when the
                // hidden phase at this recoherence falls below its chance.
                let p = w.belts.clone() / (ratio(1, 1) + r.compression.clone());
                let u = crate::quantum::hidden_phase(&r.torsion, at, &r.period);
                let struck = crate::quantum::measure(&u, &p);
                measured += 1;
                if !struck && *length > ratio(0, 1) {
                    climb.written += ratio(at.saturating_sub(climb.last) as i64, 1) / length.clone();
                }
                if climb.written >= ratio(1, 1) {
                    climb.woken = true;
                    woke.push(Waking { bell: at, path: path.clone(), medium });
                }
            }
            climb.last = at;
            climb.k += 1;
            climb.next = wake_bell(w, climb.k, t_u).unwrap_or(u64::MAX);
            if climb.woken {
                break;
            }
        }
    }
    woke.sort_by(|a, b| a.bell.cmp(&b.bell).then(a.path.cmp(&b.path)));
    (woke, measured)
}

fn put_path(path: &[u64], out: &mut Vec<u8>) {
    out.push(path.len() as u8);
    for p in path {
        out.extend_from_slice(&p.to_le_bytes());
    }
}

/// `<var>/climbs.bin`: tag 1 per world (path, written, k, last, next, woken).
pub fn write_climbs(var_root: &std::path::Path, climbs: &Climbs) {
    let mut out = Vec::new();
    for (path, c) in climbs {
        let mut v = Vec::new();
        put_path(path, &mut v);
        cfe::ratio::encode(&c.written, &mut v);
        for n in [c.k, c.last, c.next] {
            v.extend_from_slice(&n.to_le_bytes());
        }
        v.push(c.woken as u8);
        cfe::tlv::put(1, &v, &mut out);
    }
    let path = var_root.join("climbs.bin");
    let staged = path.with_extension("staged");
    if std::fs::write(&staged, out).is_ok() {
        let _ = std::fs::rename(staged, path);
    }
}

pub fn read_climbs(var_root: &std::path::Path) -> Climbs {
    let bytes = std::fs::read(var_root.join("climbs.bin")).unwrap_or_default();
    cfe::tlv::records(&bytes)
        .filter(|(t, _)| *t == 1)
        .filter_map(|(_, v)| {
            let n = *v.first()? as usize;
            let mut at = 1;
            let mut path = Vec::with_capacity(n.min(8));
            for _ in 0..n {
                path.push(u64::from_le_bytes(v.get(at..at + 8)?.try_into().ok()?));
                at += 8;
            }
            let (written, used) = cfe::ratio::decode(v.get(at..)?)?;
            at += used;
            let mut u = [0u64; 3];
            for x in u.iter_mut() {
                *x = u64::from_le_bytes(v.get(at..at + 8)?.try_into().ok()?);
                at += 8;
            }
            let woken = *v.get(at)? == 1;
            Some((path, Climb { written, k: u[0], last: u[1], next: u[2], woken }))
        })
        .collect()
}

/// Every world of the universe, read.
pub fn worlds(u: &Universe) -> Vec<World> {
    let median = |mut v: Vec<Ratio>| -> Ratio {
        v.sort();
        v.get(v.len() / 2).cloned().filter(|m| !m.is_zero()).unwrap_or_else(|| ratio(1, 1))
    };
    // One ruler per tier (shared with Xylarium, 2026-10-14): the home
    // reference body: the home galaxy (the deepest core), its innermost
    // star, and that star's innermost world.
    let _ = median;
    let home = home(u);
    let star_scale = home.as_ref().map(|h| h.star.clone()).unwrap_or_else(|| ratio(1, 1));
    let t_u = ratio(i64::try_from(u.t_u).unwrap_or(i64::MAX), 1);
    let world_scale = home.as_ref().map(|h| h.world.clone()).unwrap_or_else(|| ratio(1, 1));
    let mut out = Vec::new();
    for core in &u.cores {
        for sys in &core.satellites {
            let star = mass(&sys.kept);
            let star_burn = (ratio(1, 1) + star.clone() / star_scale.clone()) / ratio(2, 1);
            let mut inside = star.clone();
            let delay: u64 = core.radius.floor().to_integer().try_into().unwrap_or(0);
            let system = mass(&sys.total());
            let belts = if system.is_zero() { ratio(0, 1) } else { mass(&sys.medium) / system };
            let worlds_mass = sys.satellites.iter().fold(ratio(0, 1), |s, o| s + mass(&o.total()));
            let window = window_echoes(&star, &worlds_mass);
            for orb in &sys.satellites {
                let coupling = crate::ring::rate(&orb.radius, &(inside.clone() / crate::breath::unit_mass(u)));
                if let Some(mut w) = read(orb.path.clone(), &orb.kept, &(orb.heat.clone() * ratio(HYDROGEN_K, 1)), &coupling, &star_burn, &world_scale, delay) {
                    w.belts = belts.clone();
                    w.window = window;
                    w.days = (!t_u.is_zero()).then(|| (ratio(1, 1) - coupling.clone()) * orb.period.clone() / t_u.clone());
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
    fn the_land_takes_up_the_flux_its_medium_carries() {
        let mut w = World { path: vec![], heat: ratio(2, 1), burn: ratio(1, 1), size: ratio(1, 1), gravity: ratio(1, 1), core: ratio(0, 1), hold: ratio(1, 1), atmosphere: ratio(0, 1), pressure: ratio(0, 1), gases: vec![], chemical: Some(ratio(2, 1)), geologic: Some(ratio(1, 2)), plasmic: None, informational: None, coupling: ratio(0, 1), delay: 0, belts: ratio(0, 1), window: 0, days: None };
        // x = 2 at no compression: h = 2·1442/1000 = 2.884, two whole
        // halvings, 3/4 carried; twice the home heat: 500·2·3/4 = 750.
        assert_eq!(wild(&w, &ratio(0, 1), "chemical", &ratio(1, 1)), ratio(750, 1));
        // Under a halving the medium carries nothing; an absent medium too.
        assert_eq!(wild(&w, &ratio(0, 1), "geologic", &ratio(1, 1)), ratio(0, 1));
        assert_eq!(wild(&w, &ratio(0, 1), "plasmic", &ratio(1, 1)), ratio(0, 1));
        // Hotter flux, a weaker medium: the uptake is the product, no band.
        w.heat = ratio(4, 1);
        w.chemical = Some(ratio(1, 1));
        assert_eq!(wild(&w, &ratio(0, 1), "chemical", &ratio(1, 1)), ratio(1000, 1));
    }

    #[test]
    fn a_world_reads_its_heat_and_media() {
        // Hydrogen (single bond) and a two-bond kind at its own heat of
        // 1,000 K: Z = 6 on the harmonic atom (Z = 4 pairs its round shape
        // and bonds nothing).
        let matter: Manifest = [(crate::chem::Kind::parse("W0001").expect("a kind"), ratio(10, 1)), (crate::chem::Kind::parse("W0008").expect("a kind"), ratio(5, 1))].into();
        let w = read(vec![1, 1, 1], &matter, &ratio(1000, 1), &ratio(59, 60), &ratio(1, 1), &ratio(1_000_000, 1), 240).unwrap();
        assert_eq!(w.heat, ratio(1000, 1), "a world's heat is its own");
        let heavy = read(vec![1, 1, 1], &matter, &ratio(1000, 1), &ratio(59, 60), &ratio(1, 1), &ratio(1, 1), 240).unwrap();
        assert!(heavy.gravity > w.gravity, "a compressed world pulls harder");
        // A world cold through offers nothing.
        assert!(read(vec![1, 1, 1], &matter, &ratio(0, 1), &ratio(59, 60), &ratio(1, 1), &ratio(1, 1), 240).is_none());
        // A coupling at 59/60 slips 1/60 a echo: its first recoherence at
        // echo 60, after the galaxy's 240-bell delay.
        assert_eq!(wake_bell(&w, 1, 60), Some(240 + 60 * 60));
        assert_eq!(recoherences_needed(&ratio(1, 2), &ratio(0, 1)), None, "under a halving, nothing carries");
        assert!(w.chemical.is_some(), "1 + 6 close and hold at 1,000 K");
        assert!(w.geologic.is_some());
        assert_eq!(w.informational, Some(ratio(59, 1)));
        assert_eq!(w.gases, vec![(2, ratio(10, 1))], "hydrogen is the volatile");
        assert!(w.core > ratio(0, 1), "Z = 6 forms the core");
        // At its own hold as the reference, hydrogen has λ = 1: one halving
        // lost is the bound, so half is kept.
        assert_eq!(atmosphere(&w, &w.hold), ratio(10, 1));
    }
}
