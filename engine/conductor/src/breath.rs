//! docs/78 "The breath": the universal pulse, star formation, layered time.
//!
//! - **The universal pulse.** TUniversal sets the quantum clock at the Big
//!   Bang, at the universal core: the singularity's bounce pulses every `T_u`
//!   bells, and each echo is a beat of time carried through the quantum
//!   state. An echo reaches a galaxy after its distance from the universal
//!   core (one length a bell).
//! - **The breath.** Each echo compresses a galaxy's medium. Where the clump
//!   at the next reach boundary can hold its own reach (the tidal law,
//!   `m = μ · M(<r)`), a star system is born there, its orbs by the same
//!   law one tier down. The medium is consumed, nothing created.
//! - **Layered time.** A layer at radius r about the mass M(<r) it encloses
//!   runs at `r / (r + M(<r))` of the layer above: linear, exact, the
//!   weak-field dilation `1 − GM/rc²` to first order. Nested wells multiply.
//!   Each echo a layer receives is one RUNG; its proper time is its AEON.
//! - **Elastic: decohere and recohere** (operator: consumption alone is pure
//!   entropy, with no syntropic regeneration). On each echo every star
//!   returns to its galaxy's medium a share of what it holds, scaled by its
//!   own clock against the universal pulse: `ε · rate`, ε the star tier's
//!   medium share (a placeholder until stellar lifetimes are physics), rate
//!   its nested dilation. A star spent below a grain dissolves, all it held
//!   returning. Echoes recohere the medium into new systems. Periods re-read
//!   the law after each echo, `P = r / M(<r)`.
//! - **Time writes.** A write within a layer, on its own clock, is PARALLEL;
//!   a write across layers is PERPENDICULAR: an echo coming down, a birth
//!   changing the mass above. A perpendicular write down answered by one up
//!   closes a causal LOOP; the loops are the chain.
use crate::pulse::{less, mass, split, take, tier, Body, Universe};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// A layer's clock against the layer above: `r / (r + M(<r))`.
pub fn rate(r: &Ratio, enclosed: &Ratio) -> Ratio {
    if enclosed.is_zero() {
        return ratio(1, 1);
    }
    r.clone() / (r.clone() + enclosed.clone())
}

/// The unit of mass: the deepest core's hole.
pub fn unit_mass(u: &Universe) -> Ratio {
    u.cores.iter().map(|c| mass(&c.kept)).max().filter(|m| !m.is_zero()).unwrap_or_else(|| ratio(1, 1))
}

/// A galaxy's clock against the universal core's: its distance against the
/// cores' mass nearer the epicenter.
pub fn galaxy_rate(u: &Universe, i: usize, unit: &Ratio) -> Ratio {
    let d = &u.cores[i].radius;
    let nearer = u.cores.iter().filter(|c| c.radius < *d).fold(ratio(0, 1), |s, c| s + mass(&c.total()) / unit.clone());
    rate(d, &nearer)
}

/// What a write crossed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Write {
    /// Within a layer, on its own clock.
    Parallel,
    /// Across layers.
    Perpendicular,
}

/// One event of the breath.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub bell: u64,
    pub galaxy: u32,
    pub rung: u64,
    pub write: Write,
    /// A system born (its index in the galaxy), or none: the echo found no
    /// clump that could hold its reach.
    pub born: Option<usize>,
    /// The mass the galaxy's stars returned to its medium at this echo
    /// (perpendicular writes up), in units of the deepest hole.
    pub returned: Ratio,
    /// Stars dissolved at this echo.
    pub dissolved: usize,
}

/// Breathe the universe from bell 0 to `until`, echoes every `t_u` bells.
/// Events in time order. Each echo is a perpendicular write down; each birth
/// answers it with a perpendicular write up (the galaxy's mass within the
/// new orbit changes), closing a loop.
pub fn breathe(u: &mut Universe, seed: &[u8; 32], t_u: u64, until: u64) -> Vec<Event> {
    let unit = unit_mass(u);
    let law = tier(2);
    let s = law.reach.clone();
    let mu = ratio(2, 1) * s.clone() * s.clone() / (ratio(1, 1) - s.clone() * s.clone());
    let step = (ratio(1, 1) + s.clone()) / (ratio(1, 1) - s.clone());
    // Every echo's arrival, in time order.
    let mut arrivals: Vec<(u64, usize, u64)> = Vec::new();
    for (i, c) in u.cores.iter().enumerate() {
        let delay: u64 = c.radius.floor().to_integer().try_into().unwrap_or(0);
        let mut k = 1u64;
        while k * t_u + delay <= until {
            arrivals.push((k * t_u + delay, i, k));
            k += 1;
        }
    }
    arrivals.sort();
    let mut events = Vec::new();
    let epsilon = tier(1).medium;
    for (bell, i, rung) in arrivals {
        // DECOHERE: each star returns `ε · rate` of what it holds, rate its
        // nested clock (the galaxy's about the universal core, times its own
        // about the galaxy).
        let g_rate = galaxy_rate(u, i, &unit);
        let core = &mut u.cores[i];
        let mut returned = ratio(0, 1);
        let mut dissolved = 0;
        let mut enclosed_in = mass(&core.kept) / unit.clone();
        let mut keep: Vec<Body> = Vec::new();
        for mut sys in std::mem::take(&mut core.satellites) {
            let own = rate(&sys.radius, &enclosed_in);
            let back = take(&sys.kept, &(epsilon.clone() * g_rate.clone() * own));
            less(&mut sys.kept, &back);
            returned += mass(&back) / unit.clone();
            for (k, a) in back {
                *core.medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
            }
            if mass(&sys.kept).is_zero() {
                // Spent: all it held returns.
                let all = sys.total();
                returned += mass(&all) / unit.clone();
                for (k, a) in all {
                    *core.medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
                }
                dissolved += 1;
                continue;
            }
            enclosed_in += mass(&sys.total()) / unit.clone();
            keep.push(sys);
        }
        core.satellites = keep;
        let enclosed = core.satellites.iter().fold(mass(&core.kept), |acc, sys| acc + mass(&sys.total())) / unit.clone();
        let want = mu.clone() * enclosed.clone();
        let left = mass(&core.medium) / unit.clone();
        let r = match core.satellites.last() {
            None => enclosed.clone() * ratio(60 * 60, 1),
            Some(last) => last.radius.clone() * step.clone(),
        };
        let mut born = None;
        if !enclosed.is_zero() && !left.is_zero() && want <= left {
            let clump = take(&core.medium, &(want / left));
            if !mass(&clump).is_zero() {
                less(&mut core.medium, &clump);
                let k = core.satellites.last().map(|l| l.path[1] + 1).unwrap_or(1);
                let path = vec![core.path[0], k];
                let (kept, orbs, medium) = split(seed, path.clone(), clump, Some(r.clone() * s.clone()), ratio(60, 1), &unit, 1);
                let period = r.clone() / enclosed.clone();
                core.satellites.push(Body { path, radius: r, period, kept, satellites: orbs, medium });
                born = Some(core.satellites.len() - 1);
            }
        }
        // The law re-read: every period from the mass now enclosed.
        let mut inside = mass(&core.kept) / unit.clone();
        for sys in core.satellites.iter_mut() {
            if !inside.is_zero() {
                sys.period = sys.radius.clone() / inside.clone();
            }
            inside += mass(&sys.total()) / unit.clone();
        }
        events.push(Event { bell, galaxy: core.path[0], rung, write: Write::Perpendicular, born, returned, dissolved });
    }
    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_breath_forms_systems_and_keeps_every_grain() {
        let manifest: crate::pulse::Manifest = [("Z001".to_string(), ratio(4_000_000, 1)), ("Z008".to_string(), ratio(1_000_000, 1))].into();
        let mut u = crate::pulse::deploy(&[3; 32], &manifest);
        assert!(u.cores.iter().all(|c| c.satellites.is_empty()), "the pulse lays down no systems");
        let events = breathe(&mut u, &[3; 32], 60, 6000);
        assert!(events.iter().any(|e| e.born.is_some()), "echoes give birth");
        assert_eq!(u.total(), manifest, "the medium is consumed, nothing created");
        assert_eq!(rate(&ratio(3, 1), &ratio(1, 1)), ratio(3, 4));
    }
}
