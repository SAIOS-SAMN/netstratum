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
//! - **Decohere and recohere, by the phase** (operator: consumption alone is
//!   pure entropy; aligned to the quantum phase, docs/79 and docs/81). A
//!   layer's phase runs at its proper rate, so a dilated layer SLIPS behind
//!   the universal echo, `Δ = (1 − rate) · t / T_u` turns. Its COHERENCE is
//!   how near Δ stands to a whole turn, `c = 1 − 2·|Δ − round Δ|` (1 in
//!   phase, 0 opposed; linear, exact, docs/79's `(1 + cos)/2` at 0, ½, 1).
//!   Each completed slip is a RECOHERENCE, a rung: `⌊Δ⌋`. Matter follows
//!   the phase: a star sheds `ε · (1 − c)` of what it holds to the medium
//!   (entropy; ε the star tier's medium share, a placeholder), and an echo
//!   compresses only the coherent part of a galaxy's medium, `c_g · medium`
//!   (syntropy). A star spent below a grain dissolves. Periods re-read the
//!   motion law after each echo (`crate::ring`).
//! - **Time writes.** A write within a layer, on its own clock, is PARALLEL;
//!   a write across layers is PERPENDICULAR: an echo coming down, a birth
//!   changing the mass above. A perpendicular write down answered by one up
//!   closes a causal LOOP; the loops are the chain.
use crate::pulse::{less, mass, split, take, tier, Body, Universe};
use crate::ring::rate;
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

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

/// A layer's slip behind the universal echo at bell `b`, in turns:
/// `(1 − rate) · b / T_u`.
pub fn slip(rate_: &Ratio, b: u64, t_u: u64) -> Ratio {
    (ratio(1, 1) - rate_.clone()) * ratio(b as i64, t_u as i64)
}

/// Coherence: `1 − 2·|Δ − round Δ|`, 1 in phase, 0 opposed.
pub fn coherence(delta: &Ratio) -> Ratio {
    let frac = delta.clone() - delta.floor();
    let off = if frac > ratio(1, 2) { ratio(1, 1) - frac } else { frac };
    ratio(1, 1) - ratio(2, 1) * off
}

/// Rungs climbed: completed slips, `⌊Δ⌋`.
pub fn rungs(delta: &Ratio) -> u64 {
    delta.floor().to_integer().try_into().unwrap_or(0)
}

/// The spin a body carries with everything inside it: its orbit's (its mass
/// times its specific spin), its medium's, and every satellite's within.
pub fn subtree_spin(b: &Body, unit: &Ratio) -> Ratio {
    let own = mass(&b.total()) / unit.clone() * b.ell.clone();
    b.satellites.iter().fold(own + b.medium_spin.clone(), |s, x| s + subtree_spin(x, unit))
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

/// Breathe the universe over bells `from` (exclusive) to `until`, echoes
/// every `t_u` bells (the conductor calls it once a beat, `(beat − 1, beat]`).
/// Events in time order. Each echo is a perpendicular write down; each birth
/// answers it with a perpendicular write up (the galaxy's mass within the
/// new orbit changes), closing a loop.
pub fn breathe(u: &mut Universe, seed: &[u8; 32], t_u: u64, from: u64, until: u64) -> Vec<Event> {
    let unit = unit_mass(u);
    let law = tier(2);
    let s = law.reach.clone();
    let mu = ratio(2, 1) * s.clone() * s.clone() / (ratio(1, 1) - s.clone() * s.clone());
    // Every echo's arrival, in time order.
    let mut arrivals: Vec<(u64, usize, u64)> = Vec::new();
    for (i, c) in u.cores.iter().enumerate() {
        let delay: u64 = c.radius.floor().to_integer().try_into().unwrap_or(0);
        let mut k = 1u64;
        while k * t_u + delay <= until {
            if k * t_u + delay > from {
                arrivals.push((k * t_u + delay, i, k));
            }
            k += 1;
        }
    }
    arrivals.sort();
    let mut events = Vec::new();
    let epsilon = tier(1).medium;
    for (bell, i, rung) in arrivals {
        // DECOHERE: each star sheds `ε · (1 − c)` of what it holds, c its
        // coherence with the echo at this bell (its nested clock's slip).
        let g_rate = galaxy_rate(u, i, &unit);
        let g_coherence = coherence(&slip(&g_rate, bell, t_u));
        let core = &mut u.cores[i];
        let mut returned = ratio(0, 1);
        let mut dissolved = 0;
        let mut enclosed_in = mass(&core.kept) / unit.clone();
        let mut keep: Vec<Body> = Vec::new();
        for mut sys in std::mem::take(&mut core.satellites) {
            let own = g_rate.clone() * rate(&sys.radius, &enclosed_in);
            let out_of_phase = ratio(1, 1) - coherence(&slip(&own, bell, t_u));
            let back = take(&sys.kept, &(epsilon.clone() * out_of_phase));
            less(&mut sys.kept, &back);
            returned += mass(&back) / unit.clone();
            // THE SPIN LEDGER (docs/83 Phase 0): shed matter carries its
            // system's specific spin into the medium.
            core.medium_spin += mass(&back) / unit.clone() * sys.ell.clone();
            for (k, a) in back {
                *core.medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
            }
            if mass(&sys.kept).is_zero() {
                // Spent: all it held returns, with all its spin (its orbit's
                // and everything turning inside it).
                core.medium_spin += subtree_spin(&sys, &unit);
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
        // RECOHERE: the echo compresses only the coherent part of the medium.
        let medium_mass = mass(&core.medium) / unit.clone();
        let left = medium_mass.clone() * g_coherence.clone();
        let mut born = None;
        if !enclosed.is_zero() && !left.is_zero() && want <= left {
            let clump = take(&core.medium, &(want / medium_mass.clone()));
            let m_u = mass(&clump) / unit.clone();
            if !m_u.is_zero() {
                less(&mut core.medium, &clump);
                // The clump takes its share of the medium's spin; what spins
                // inside it (its orbs and its belts, about its star) is paid
                // from that spin, and the rest is its orbit's: it settles
                // where the balance puts it (below).
                let taken = core.medium_spin.clone() * m_u.clone() / medium_mass.clone();
                core.medium_spin -= taken.clone();
                let ell_med = taken.clone() / m_u.clone();
                let r_guess = crate::ring::radius(&ell_med, &enclosed).unwrap_or_else(|| ell_med.clone());
                let k = core.satellites.iter().map(|l| l.path[1] + 1).max().unwrap_or(1);
                let path = vec![core.path[0], k];
                let (kept, orbs, medium, medium_spin) = split(seed, path.clone(), clump, Some(r_guess.clone() * s.clone()), ratio(60, 1), &unit, 1);
                let mut sys = Body { path, radius: r_guess, period: ratio(0, 1), kept, satellites: orbs, medium, ell: ratio(0, 1), medium_spin };
                let internal = subtree_spin(&sys, &unit);
                sys.ell = (taken - internal) / m_u;
                core.satellites.push(sys);
                born = Some(core.satellites.len() - 1);
            }
        }
        // THE BALANCE (the motion law, `crate::ring`; cold bodies): each
        // system stands where its spin meets the holding inside it,
        // `R = ℓ / H`, read inside out by spin; its period follows,
        // `P = 1 + R / H`. Orbits breathe: a birth or a loss inside an
        // orbit moves it, the ledger deciding which way.
        core.satellites.sort_by(|a, b| a.ell.cmp(&b.ell));
        let mut inside = mass(&core.kept) / unit.clone();
        for sys in core.satellites.iter_mut() {
            if !inside.is_zero() && sys.ell > ratio(0, 1) {
                if let Some(r) = crate::ring::radius(&sys.ell, &inside) {
                    sys.period = crate::ring::period(&r, &inside);
                    sys.radius = r;
                }
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
        assert!(u.cores.iter().any(|c| !c.satellites.is_empty()), "the Big Bang lays down a first generation");
        let unit = unit_mass(&u);
        let spin0 = crate::pulse::spin_total(&u, &unit);
        let events = breathe(&mut u, &[3; 32], 60, 0, 6000);
        assert!(events.iter().any(|e| e.born.is_some()), "echoes give birth");
        assert_eq!(u.total(), manifest, "the medium is consumed, nothing created");
        assert_eq!(crate::pulse::spin_total(&u, &unit), spin0, "the spin ledger is conserved exactly");
        assert_eq!(rate(&ratio(3, 1), &ratio(1, 1)), ratio(3, 4));
        // A clock at 3/4 slips a quarter turn each echo: opposed at two
        // echoes, recohered (one rung) at four.
        let r = ratio(3, 4);
        assert_eq!(coherence(&slip(&r, 120, 60)), ratio(0, 1));
        assert_eq!((coherence(&slip(&r, 240, 60)), rungs(&slip(&r, 240, 60))), (ratio(1, 1), 1));
    }
}
