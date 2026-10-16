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
use crate::pulse::{less, mass, take, tier, Body, Universe};
use crate::ring::rate;
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// The unit of mass: the deepest core's hole, as founded (`Universe::unit`);
/// a universe saved before it was kept reads its deepest disc.
pub fn unit_mass(u: &Universe) -> Ratio {
    if !u.unit.is_zero() {
        return u.unit.clone();
    }
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
pub fn breathe(u: &mut Universe, _seed: &[u8; 32], t_u: u64, from: u64, until: u64) -> Vec<Event> {
    let unit = unit_mass(u);
    let universe_mass = mass(&u.total());
    use num_traits::Zero as _;
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
        // THE RETURN (`crate::ocean`, proposed with xylarium): the galaxy's
        // swing crosses the ocean's standing crests; what it gathers at the
        // crossings since its last echo comes in from the ocean. Where it
        // lands in the galaxy, and how hot, is owed (docs/83): until then it
        // is the galaxy's gas on the ledger, unshaped.
        let gathered = {
            let c = &u.cores[i];
            let whole = mass(&u.total());
            let h_g = (crate::pulse::core_state(c) + mass(&c.total())) / whole.clone();
            let ring_bells = ratio((t_u / 4) as i64, 1);
            let reach = c.radius.clone() / ring_bells;
            let halves = |b: u64| if c.period.is_zero() { 0 } else { (ratio(2 * b as i64, 1) / c.period.clone()).floor().to_integer().try_into().unwrap_or(0u64) };
            let half_swings = halves(bell).saturating_sub(halves(bell.saturating_sub(t_u)));
            let m = mass(&u.medium) / whole;
            let share = crate::ocean::gathered(rung, &reach, &h_g, &m, half_swings);
            (crate::ocean::take(&u.medium, &share), crate::ocean::arrival_turning(rung, &reach, &h_g, &m), crate::ocean::pump(&reach, &h_g))
        };
        let (gathered, turning, pump) = gathered;
        // CONVECTION's drive, received at the galaxy's share (the tide crosses
        // the boundary like a pulse).
        let drive = g_rate.clone() * pump;
        less(&mut u.medium, &gathered);
        let core = &mut u.cores[i];
        // Where it lands (carried spins add): held joins the disc, the rest
        // is gas, parcels cooling from this echo.
        let mut with = core.total();
        for (k, a) in &gathered {
            *with.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
        }
        let (held_in, free_in, parcels_in) = crate::ocean::land(&with, &gathered, &g_rate, &turning, bell);
        for (k, a) in held_in {
            *core.kept.entry(k).or_insert_with(|| ratio(0, 1)) += a;
        }
        for (k, a) in free_in {
            *core.medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
        }
        core.gas.extend(parcels_in);
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
            // BURNING (the heartbeat, docs/83; mycelium: transmutation is the
            // universe's own physics): the halvings of its hydrogen due since
            // the last echo, on the rings its nested clock received, fuse
            // H + H → He. Holding + magnetism (3Z) is conserved; the system's
            // spin is kept exact as its holding transduces.
            if sys.born > 0 || sys.kept.contains_key("Z001") {
                let since = |at: u64| (own.clone() * ratio(at.saturating_sub(sys.born) as i64, 1)).floor().to_integer().try_into().unwrap_or(0u64);
                let (now_rings, then_rings) = (since(bell), since(bell.saturating_sub(t_u)));
                if let (Some((every, _, _)), Some(fuel)) = (crate::formation::heartbeat(&sys.kept, &universe_mass, now_rings), sys.kept.get("Z001").cloned()) {
                    let due = (now_rings / every).saturating_sub(then_rings / every).min(64) as u32;
                    if due > 0 {
                        let left = fuel.clone() / Ratio::from_integer(num_bigint::BigInt::from(2).pow(due));
                        // Whole grains of helium: two hydrogen grains each.
                        let helium = cfe::world::smelt_resource(&((fuel.clone() - left) / ratio(2, 1)));
                        if helium > ratio(0, 1) {
                            let spin = mass(&sys.total()) * sys.ell.clone();
                            less(&mut sys.kept, &[("Z001".to_string(), helium.clone() * ratio(2, 1))].into());
                            *sys.kept.entry("Z002".to_string()).or_insert_with(|| ratio(0, 1)) += helium;
                            let after = mass(&sys.total());
                            if !after.is_zero() {
                                // The spin grain: the floored remainder goes
                                // to the galaxy's gas (`ring::grain`).
                                sys.ell = crate::ring::grain(&(spin.clone() / after.clone()));
                                core.medium_spin += (spin - after * sys.ell.clone()) / unit.clone();
                            }
                        }
                    }
                }
            }
            enclosed_in += mass(&sys.total()) / unit.clone();
            keep.push(sys);
        }
        core.satellites = keep;
        // FORMATION (docs/83, with xylarium): the core's AXIAL JETS land at
        // the rim, then gas that has cooled enough COLLAPSES into stars.
        let galaxy = core.total();
        let g_mass = mass(&galaxy);
        let (rings, _, h) = crate::formation::shells(&galaxy, &core.kept);
        let weight = |k: &str| ratio(i64::from(k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok()).map(cfe::almanac::atom::hold).unwrap_or(1)), 1);
        // JETS: the core fires on its own BELL, the bounce (the jetstream
        // ruling: the bounce's pressure ejects; with xylarium), once an
        // echo it regenerates; the batch reaches the rim (`formation::jet_batch`)
        // cold, where the push and the deficit balance (δ = p = ½). THE
        // RECYCLING CIRCUIT (docs/83, with xylarium): the rim is the 1:1
        // crest (`P_b = P(H) = 2`, `R_{1/1} = H(2 − 1) = 2`), where the
        // regenerated kick couples at full strength (`a_1 = 1`), so the batch
        // crosses the boundary WHOLE into the medium one tier up, the ocean,
        // along the jet's axis with no orbital spin. The galaxy's gas comes
        // back in from the ocean (owed: the universe's crests).
        let mut jets = 0;
        let mut to_ocean = crate::pulse::Manifest::new();
        // The INTERFACE CLOCK (one law at every tier: the core is a heart
        // one tier up) says HOW MUCH leaves a bounce, not how often: the
        // share `P_b/⌈Q̄/(e·s)⌉` of the disc, what the interface built up
        // over the bounce's own bell (`P_b = 2` own rings; with xylarium:
        // rings in = passed on + absorbed, released at the bounce);
        // closed-shell discs barely jet.
        // REGENERATION (operator ruling (i)): the core runs on its own
        // holding, s = 1; no share of the universe enters it.
        let every = crate::formation::interface_every(&core.kept, &ratio(1, 1));
        let rim = core.gas.iter().map(|g| g.outer.clone()).chain(rings.iter().map(|r| r.outer.clone())).max().unwrap_or_else(|| ratio(1, 1));
        let jet_share = ratio(2, every.min(i64::MAX as u64) as i64).min(ratio(1, 1));
        for k in [bell] {
            let galaxy_now = core.total();
            let (rings_now, _, _) = crate::formation::shells(&galaxy_now, &core.kept);
            for parcel in crate::formation::jet_batch(&rings_now, &h, &rim, k, &jet_share) {
                let amount = cfe::world::smelt_resource(&(parcel.held.clone() * g_mass.clone() / weight(&parcel.key)));
                let have = core.kept.get(&parcel.key).cloned().unwrap_or_else(|| ratio(0, 1));
                let amount = amount.min(have);
                if amount <= ratio(0, 1) {
                    continue;
                }
                less(&mut core.kept, &[(parcel.key.clone(), amount.clone())].into());
                *to_ocean.entry(parcel.key.clone()).or_insert_with(|| ratio(0, 1)) += amount;
            }
            jets += 1;
        }
        // COLLAPSE: each stretch of the gas (with the disc's rings it passes
        // over) whose clump can hold forms as many stars as fit.
        let mut stars = 0;
        let (now, _, h_now) = crate::formation::shells(&core.total(), &core.kept);
        // CONVECTION (`crate::ocean::convect`): the held rings churn in cells
        // driven by the swing's pump, renewed each echo.
        let rings: Vec<crate::formation::Shell> = now.into_iter().map(|r| crate::ocean::convect(&r, &drive, &h_now, bell)).collect();
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
            // THE CLUMP'S AGITATION AT IGNITION (with xylarium): its cooled
            // residue PLUS the galactic tide it stands in, unlocked at its own
            // throat condition (rate ½ at w = m): `a = a_cooled + ½·δ_gal`.
            // Two channels, summed, as the cooling channels are.
            let tide = crate::ring::deficit(&st.inner, &h);
            let a_clump = st.agitation.clone() + tide.clone() / ratio(2, 1);
            let Some(ignited) = crate::formation::ignition_share(&a_clump) else { continue };
            let _ = &ignited;
            // The whole clump is taken: its heart's share becomes the star,
            // the rest its disc and worlds (`formation::system`).
            let used = (w * ratio(count as i64, 1) / width).min(ratio(1, 1)) * g_coherence.clone();
            // The coherent share shrinks what the echo takes, never rounds a
            // holding clump away: at least one star where any gas is taken.
            if g_coherence.is_zero() {
                continue;
            }
            let count = (ratio(count as i64, 1) * g_coherence.clone()).floor().to_integer().try_into().unwrap_or(0u64).max(1);
            // The grains the stars take, from the gas and the disc's rings.
            let mut clump = crate::pulse::Manifest::new();
            let mut from_gas = ratio(0, 1);
            // THE HELD DISC'S SPIN (the motion law): a held ring at reach R
            // orbits, `ℓ = R·H`; it is implicit while held (read from its
            // place, in the founding's transition units) and is carried onto
            // the ledger by what a star takes from it.
            let mut from_rings = ratio(0, 1);
            let scale = g_mass.clone() / unit.clone();
            let h_units = mass(&core.kept) / unit.clone() + state.clone();
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
                } else {
                    let reach = (all[*i].inner.clone() + all[*i].outer.clone()) / ratio(2, 1) * scale.clone();
                    from_rings += amount.clone() * weight(&key) / unit.clone() * crate::ring::spin(&reach, &h_units);
                }
                *clump.entry(key).or_insert_with(|| ratio(0, 1)) += amount;
            }
            if clump.is_empty() {
                continue;
            }
            // The stars take the gas's spin by the share of it they took, and
            // the held rings' by their place (the jets carry none).
            let medium_mass = mass(&core.medium) + from_gas.clone();
            let taken_gas = if medium_mass.is_zero() { ratio(0, 1) } else { core.medium_spin.clone() * from_gas / medium_mass };
            core.medium_spin -= taken_gas.clone();
            let taken_all = taken_gas + from_rings;
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
                // Its identity: its birth reading, never a row number.
                let siblings: Vec<u64> = core.satellites.iter().map(|l| l.path[1]).collect();
                let path = vec![core.path[0], crate::ring::birth_key(bell, &r_guess, &held, &siblings)];
                let Some((kept, orbs, medium, medium_spin)) = crate::formation::system(&star, &a_clump, &st.agitation, &path, &unit, &universe_mass, &tide) else { continue };
                let mut sys = Body { path, radius: r_guess, period: ratio(0, 1), kept, satellites: orbs, medium, ell: ratio(0, 1), medium_spin, gas: Vec::new(), born: bell, state: ratio(0, 1) };
                let internal = subtree_spin(&sys, &unit);
                let own = taken - internal;
                sys.ell = crate::ring::grain(&(own.clone() / m_u.clone()));
                core.medium_spin += own - m_u * sys.ell.clone();
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
        let galaxy_id = core.path[0] as u32;
        for (k, a) in to_ocean {
            *u.medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
        }
        events.push(Event { bell, galaxy: galaxy_id, rung, write: Write::Perpendicular, stars, jets, returned, dissolved });
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
