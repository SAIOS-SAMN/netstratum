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
/// holding nearer the epicenter: the universal core's state, and each nearer
/// galaxy's (its core's state and its grains).
pub fn galaxy_rate(u: &Universe, i: usize, unit: &Ratio) -> Ratio {
    let d = &u.cores[i].radius;
    let nearer = u.cores.iter().filter(|c| c.radius < *d).fold(crate::pulse::universal_state(u) / unit.clone(), |s, c| s + (crate::pulse::core_state(c) + mass(&c.total())) / unit.clone());
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
    /// Stars born at this echo: gas that cooled enough to collapse
    /// (docs/83 FORMATION).
    pub stars: usize,
    /// Jet batches that landed at the rim at this echo.
    pub jets: usize,
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
    let universe_mass = mass(&u.total());
    let s = tier(2).reach.clone();
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
        // The galaxy's holding constant through the echo (its grains only
        // move within it), so its core's state is read once.
        let state = crate::pulse::core_state(core) / unit.clone();
        let mut enclosed_in = mass(&core.kept) / unit.clone() + state.clone();
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
        // FORMATION (docs/83, with xylarium): the core's AXIAL JETS land at
        // the rim, then gas that has cooled enough COLLAPSES into stars.
        let galaxy = core.total();
        let g_mass = mass(&galaxy);
        let share = g_mass.clone() / universe_mass.clone();
        let (now, h) = crate::formation::shells(&galaxy, &core.kept, &share);
        let rings: Vec<crate::formation::Shell> = now.into_iter().filter(|r| r.outer == r.inner).collect();
        let weight = |k: &str| ratio(i64::from(k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok()).map(cfe::almanac::atom::hold).unwrap_or(1)), 1);
        // JETS: the disc halves into the jet every ⌈1/p⌉ rings; each batch
        // due in this echo's span lands at the rim (`formation::jet_batch`),
        // its grains moving from the disc to the gas with no orbital spin.
        let mut jets = 0;
        let every = crate::formation::jet_every(&h);
        let rim = core.gas.iter().map(|g| g.outer.clone()).chain(rings.iter().map(|r| r.outer.clone())).max().unwrap_or_else(|| ratio(1, 1));
        let first_due = bell.saturating_sub(t_u) / every + 1;
        for k in first_due..=bell / every {
            let galaxy_now = core.total();
            let (now, _) = crate::formation::shells(&galaxy_now, &core.kept, &share);
            let rings_now: Vec<crate::formation::Shell> = now.into_iter().filter(|r| r.outer == r.inner).collect();
            for mut parcel in crate::formation::jet_batch(&rings_now, &h, &rim, k * every) {
                let amount = cfe::world::smelt_resource(&(parcel.held.clone() * g_mass.clone() / weight(&parcel.key)));
                let have = core.kept.get(&parcel.key).cloned().unwrap_or_else(|| ratio(0, 1));
                let amount = amount.min(have);
                if amount <= ratio(0, 1) {
                    continue;
                }
                less(&mut core.kept, &[(parcel.key.clone(), amount.clone())].into());
                *core.medium.entry(parcel.key.clone()).or_insert_with(|| ratio(0, 1)) += amount.clone();
                parcel.held = amount * weight(&parcel.key) / g_mass.clone();
                core.gas.push(parcel);
            }
            jets += 1;
        }
        // COLLAPSE: each stretch of the gas (with the disc's rings it passes
        // over) whose clump can hold forms as many stars as fit.
        let mut stars = 0;
        let (now, _) = crate::formation::shells(&core.total(), &core.kept, &share);
        let rings: Vec<crate::formation::Shell> = now.into_iter().filter(|r| r.outer == r.inner).collect();
        let parcels = core.gas.len();
        let all: Vec<crate::formation::Shell> = core.gas.iter().cloned().chain(rings).collect();
        for st in crate::formation::stretches(&all, bell) {
            let Some((w, count)) = crate::formation::collapse(&st, &h) else { continue };
            let width = st.outer.clone() - st.inner.clone();
            // RECOHERE: the echo compresses only the coherent part of the
            // gas, so only that share of what could collapse does now.
            // IGNITION (the quantum heart, docs/83): each clump runs away to
            // the throat condition w = m, where its heart forms, and the star
            // holds its share `(1 − 2a)/(1 + 2a)` of it; at a ≥ ½ light
            // pushes back before the heart forms and the clump stays gas.
            let Some(ignited) = crate::formation::ignition_share(&st.agitation) else { continue };
            let used = (w * ratio(count as i64, 1) / width).min(ratio(1, 1)) * g_coherence.clone() * ignited;
            // The coherent share shrinks what the echo takes, never rounds a
            // holding clump away: at least one star where any gas is taken.
            if g_coherence.is_zero() {
                continue;
            }
            let count = (ratio(count as i64, 1) * g_coherence.clone()).floor().to_integer().try_into().unwrap_or(0u64).max(1);
            // The grains the stars take, from the gas and the disc's rings.
            let mut clump = crate::pulse::Manifest::new();
            let mut from_gas = ratio(0, 1);
            for (i, part) in &st.members {
                let take_held = part.clone() * used.clone();
                let key = all[*i].key.clone();
                let amount = cfe::world::smelt_resource(&(take_held * g_mass.clone() / weight(&key)));
                let source = if *i < parcels { &mut core.medium } else { &mut core.kept };
                let amount = amount.min(source.get(&key).cloned().unwrap_or_else(|| ratio(0, 1)));
                if amount <= ratio(0, 1) {
                    continue;
                }
                less(source, &[(key.clone(), amount.clone())].into());
                if *i < parcels {
                    from_gas += amount.clone() * weight(&key);
                    core.gas[*i].held -= amount.clone() * weight(&key) / g_mass.clone();
                }
                *clump.entry(key).or_insert_with(|| ratio(0, 1)) += amount;
            }
            if clump.is_empty() {
                continue;
            }
            // The stars take the gas's spin by the share of it they took
            // (the disc's rings carry none on the ledger, and the jets none).
            let medium_mass = mass(&core.medium) + from_gas.clone();
            let taken_all = if medium_mass.is_zero() { ratio(0, 1) } else { core.medium_spin.clone() * from_gas / medium_mass };
            core.medium_spin -= taken_all.clone();
            let clump_mass = mass(&clump);
            let mut left = clump;
            for j in 0..count.max(1) {
                let star = if j + 1 == count.max(1) { std::mem::take(&mut left) } else { take(&left, &ratio(1, (count.max(1) - j) as i64)) };
                less(&mut left, &star);
                let m_u = mass(&star) / unit.clone();
                if m_u.is_zero() {
                    continue;
                }
                let taken = taken_all.clone() * mass(&star) / clump_mass.clone();
                let ell_med = taken.clone() / m_u.clone();
                let held = core.satellites.iter().fold(mass(&core.kept), |acc, sys| acc + mass(&sys.total())) / unit.clone() + state.clone();
                let r_guess = crate::ring::radius(&ell_med, &held).unwrap_or_else(|| ell_med.clone());
                let k = core.satellites.iter().map(|l| l.path[1] + 1).max().unwrap_or(1);
                let path = vec![core.path[0], k];
                let (kept, orbs, medium, medium_spin) = split(seed, path.clone(), star, Some(r_guess.clone() * s.clone()), ratio(60, 1), &unit, 1, ratio(0, 1), None);
                let mut sys = Body { path, radius: r_guess, period: ratio(0, 1), kept, satellites: orbs, medium, ell: ratio(0, 1), medium_spin, gas: Vec::new() };
                let internal = subtree_spin(&sys, &unit);
                sys.ell = (taken - internal) / m_u;
                core.satellites.push(sys);
                stars += 1;
            }
            for leftover in [left] {
                for (k, a) in leftover {
                    *core.medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
                }
            }
        }
        core.gas.retain(|g| g.held > ratio(0, 1));
        // THE BALANCE (the motion law, `crate::ring`; cold bodies): each
        // system stands where its spin meets the holding inside it,
        // `R = ℓ / H`, read inside out by spin; its period follows,
        // `P = 1 + R / H`. Orbits breathe: a birth or a loss inside an
        // orbit moves it, the ledger deciding which way.
        core.satellites.sort_by(|a, b| a.ell.cmp(&b.ell));
        let mut inside = mass(&core.kept) / unit.clone() + state.clone();
        for sys in core.satellites.iter_mut() {
            if !inside.is_zero() && sys.ell > ratio(0, 1) {
                if let Some(r) = crate::ring::radius(&sys.ell, &inside) {
                    sys.period = crate::ring::period(&r, &inside);
                    sys.radius = r;
                }
            }
            inside += mass(&sys.total()) / unit.clone();
        }
        events.push(Event { bell, galaxy: core.path[0], rung, write: Write::Perpendicular, stars, jets, returned, dissolved });
    }
    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_slipping_clock_decoheres_and_recoheres_exactly() {
        // A clock at 3/4 slips a quarter turn each echo: opposed at two
        // echoes, recohered (one rung) at four.
        let r = ratio(3, 4);
        assert_eq!(coherence(&slip(&r, 120, 60)), ratio(0, 1));
        assert_eq!((coherence(&slip(&r, 240, 60)), rungs(&slip(&r, 240, 60))), (ratio(1, 1), 1));
    }
}
