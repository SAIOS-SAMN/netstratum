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
use crate::pulse::{less, mass, Body, Universe};
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
    /// Star encounters at this echo: mergers (sticks) and bounces.
    pub merged: usize,
    pub encounters: usize,
}

/// Breathe the universe over bells `from` (exclusive) to `until`, echoes
/// every `t_u` bells (the conductor calls it once a beat, `(beat − 1, beat]`).
/// Events in time order. Each echo is a perpendicular write down; each birth
/// answers it with a perpendicular write up (the galaxy's mass within the
/// new orbit changes), closing a loop.
pub fn breathe(u: &mut Universe, _seed: &[u8; 32], t_u: u64, from: u64, until: u64) -> Vec<Event> {
    let unit = unit_mass(u);
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
    for (bell, i, rung) in arrivals {
        // DECOHERE: each star sheds `ε · (1 − c)` of what it holds, c its
        // coherence with the echo at this bell (its nested clock's slip).
        let g_rate = galaxy_rate(u, i, &unit);
        // THE RETURN (`crate::ocean`, with xylarium): the galaxy's swing
        // crosses the ocean's standing crests and gathers there (STAGED: the
        // ocean is not yet motes, so its gathering is still a share of it,
        // labelled owed). What arrives keeps its crossing's turning as state.
        let (gathered, turning, delta_u) = {
            let c = &u.cores[i];
            // Read on the grain where they feed the law (the carrier).
            let whole = u.cores.iter().fold(crate::pulse::held(&u.medium), |s, c| s + crate::pulse::held(&c.total()));
            let h_g = crate::ring::grain(&((crate::pulse::core_state(c) + crate::pulse::held(&c.total())) / whole.clone()));
            let ring_bells = ratio((t_u / 4) as i64, 1);
            let reach = c.radius.clone() / ring_bells;
            let halves = |b: u64| if c.period.is_zero() { 0 } else { (ratio(2 * b as i64, 1) / c.period.clone()).floor().to_integer().try_into().unwrap_or(0u64) };
            let half_swings = halves(bell).saturating_sub(halves(bell.saturating_sub(t_u)));
            let m = crate::ring::grain(&(crate::pulse::held(&u.medium) / whole));
            let h_u = crate::ocean::universe_hold(u);
            let share = crate::ocean::gathered(rung, &reach, &h_g, &m, half_swings, &h_u);
            let delta_u = crate::ring::grain(&(h_u.clone() / (reach.clone() + h_u.clone())));
            (crate::ocean::take(&u.medium, &share), crate::ocean::arrival_turning(rung, &reach, &h_g, &m, &h_u), delta_u)
        };
        less(&mut u.medium, &gathered);
        let core = &mut u.cores[i];
        let mut returned = ratio(0, 1);
        let mut dissolved = 0;
        let (mut merged, mut encounters) = (0usize, 0usize);
        // The galaxy's holding constant through the echo (its grains only
        // move within it), so its core's state is read once.
        let state = crate::pulse::core_state(core) / unit.clone();
        let mut enclosed_in = mass(&core.kept) / unit.clone() + state.clone();
        let mut keep: Vec<Body> = Vec::new();
        // What the stars shed crosses whole into the galaxy's flows.
        let mut shed: Vec<(crate::pulse::Manifest, Ratio, Vec<Ratio>)> = Vec::new();
        // The galaxy's push and throat in its own units, for the wind's
        // meeting with the medium.
        let g_before = mass(&core.total());
        let pre = crate::pulse::galaxy_push(&core.total());
        let scale_pre = g_before.clone() / unit.clone();
        // THE ENCLOSED HOLDING, read once from the echo's start: the
        // singularity's own state and every mote and star inside, by spin.
        let enc = pre.as_ref().filter(|_| !g_before.is_zero()).map(|(_, h_g, _)| {
            let motes = core.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g_before.clone()));
            let stars = core.satellites.iter().map(|b| (b.radius.clone() / scale_pre.clone() * h_g.clone(), mass(&b.total()) / g_before.clone()));
            crate::flows::Enclosed::new(ratio(1, 1), motes.chain(stars).collect())
        });
        // STARS COLLIDE (operator 2026-10-18; with xylarium): the one
        // collide law among the galaxy's stars, before each reads its echo;
        // each carries the coherence it had before the encounter.
        let planes = crate::flows::PLANES;
        let c_pre: Vec<Ratio> = core.satellites.iter().map(|b| if b.phase.len() < planes || b.bell.len() < planes { ratio(1, 1) } else { (0..planes).fold(ratio(1, 1), |c, k| c * crate::flows::in_step(&b.phase[k], &b.bell[k])) }).collect();
        let (met, star_sticks, star_bounces) = match &pre {
            Some((p_g, h_g, _)) if enc.is_some() => crate::formation::collide_stars(std::mem::take(&mut core.satellites), c_pre, &scale_pre, h_g, p_g, &g_before, enc.as_ref().expect("checked")),
            _ => (std::mem::take(&mut core.satellites).into_iter().zip(c_pre).collect(), 0, 0),
        };
        // Worlds of emptied remnants, re-anchored after the stars are read.
        let mut orphans: Vec<(Body, Ratio)> = Vec::new();
        merged += star_sticks;
        encounters += star_bounces;
        for (mut sys, c_was) in met {
            let own = g_rate.clone() * rate(&sys.radius, &enclosed_in);
            // THE STAR'S BELL (with xylarium): its own spin turns it,
            // `ω_s = δ(ℓ_s/H_s)` a ring, `H_s = 1 + held/held_at_birth`; its
            // phases advance by the rings its clock received this echo.
            if sys.phase.len() < planes {
                sys.phase = vec![ratio(0, 1); planes];
            }
            let h_s = if sys.state.is_zero() { ratio(1, 1) } else { ratio(1, 1) + mass(&sys.kept) / sys.state.clone() };
            let omega = h_s.clone() / (sys.turning.clone() / h_s.clone() + h_s.clone());
            let rings_at = |at: u64| -> u64 { (own.clone() * ratio(at.saturating_sub(sys.born) as i64, 1)).floor().to_integer().try_into().unwrap_or(0) };
            // Its own bell rings on its whole turns (its spin, ω_s): it times
            // its outbursts and its system, never a phase against its galaxy.
            let rang = (omega.clone() * ratio(rings_at(bell) as i64, 1)).floor() > (omega.clone() * ratio(rings_at(bell.saturating_sub(t_u)) as i64, 1)).floor();
            // ITS REFERENCE (operator 2026-10-18: "this is how you get stars
            // colliding, or random strange star events"): its galaxy's own
            // phase at its place, born at the star's own phases.
            if sys.bell.len() < planes {
                sys.bell = sys.phase.clone();
            }
            // Its coherence with its galaxy BEFORE this echo's encounters.
            let c_before = c_was;
            // The galaxy's phase at the star's reach turns as its flows do
            // there, and the star RIDES it (with xylarium): it orbits with what
            // it was born of, so its coherence moves only where something
            // displaces it from its flows.
            if let (Some((p_g, h_g, _)), Some(enc)) = (&pre, &enc) {
                if !scale_pre.is_zero() {
                    // At its reach by what is ENCLOSED inside it (the core
                    // and every mote and star inside): the cyclotron.
                    let ell_s = sys.radius.clone() / scale_pre.clone() * h_g.clone();
                    let (r, d_enc) = enc.reach(&ell_s);
                    let h_in = enc.hold(&ell_s);
                    let was = sys.bell.clone();
                    crate::flows::turn(&mut sys.bell, &r, &h_in, p_g, &g_rate, &delta_u);
                    // THE RETURN (with xylarium): its galaxy's torque gives
                    // back `d/(d + turned)` of its displacement in each plane,
                    // `d = 1 − δ_gal` its drag, `turned` its own spin's turns
                    // this echo: a fast spinner is a gyroscope.
                    let d = ratio(1, 1) - d_enc;
                    let turned = omega.clone() * ratio(rings_at(bell).saturating_sub(rings_at(bell.saturating_sub(t_u))) as i64, 1);
                    let keep_share = if (d.clone() + turned.clone()).is_zero() { ratio(1, 1) } else { turned.clone() / (d + turned) };
                    for k in 0..planes {
                        let off = sys.phase[k].clone() - was[k].clone();
                        let f = off.clone() - off.floor();
                        let signed = if f > ratio(1, 2) { f - ratio(1, 1) } else { f };
                        let g = sys.bell[k].clone() + signed * keep_share.clone();
                        sys.phase[k] = crate::ring::grain(&(g.clone() - g.floor()));
                    }
                }
            }
            // THE REMNANT (the closed limit ruled (a), operator 2026-10-18:
            // "keep the stars recycling as accurate as possible ... so that we
            // can have this universe be self sustaining"; with xylarium): a
            // spent star stands at its throat condition, a small throat whose
            // other face is its galactic core. Each echo it passes `δ(r)/2` of
            // its CLOSING mass, kind by kind as it holds them, through that
            // face: into the galaxy's flows on the outbound axial leg (loop
            // phase ½). It burns, bursts and sheds nothing else. Emptied, it is
            // gone, and its worlds re-anchor on the nearest star still burning.
            if sys.remnant {
                let d = match (&pre, &enc) {
                    (Some((_, h_g, _)), Some(enc)) if !scale_pre.is_zero() => enc.reach(&(sys.radius.clone() / scale_pre.clone() * h_g.clone())).1,
                    _ => ratio(1, 2),
                };
                let now = mass(&sys.kept);
                if !now.is_zero() {
                    let share = (d / ratio(2, 1) * sys.state.clone() / now).min(ratio(1, 1));
                    let mut out = crate::pulse::Manifest::new();
                    for (k, a) in &sys.kept {
                        let x = if share == ratio(1, 1) { a.clone() } else { crate::pulse::nucleus(&(a.clone() * share.clone())).min(a.clone()) };
                        if x > ratio(0, 1) {
                            out.insert(k.clone(), x);
                        }
                    }
                    less(&mut sys.kept, &out);
                    returned += mass(&out) / unit.clone();
                    let orbit = sys.phase.first().cloned().unwrap_or_else(|| ratio(0, 1));
                    shed.push((out, sys.radius.clone(), vec![orbit, ratio(1, 2)]));
                }
                if mass(&sys.kept).is_zero() {
                    // Emptied: its medium crosses whole; its worlds re-anchor.
                    let all = sys.total();
                    let worlds = std::mem::take(&mut sys.satellites);
                    let rest = all.into_iter().filter(|(_, a)| *a > ratio(0, 1)).collect::<crate::pulse::Manifest>();
                    let mut left = rest;
                    for w in &worlds {
                        less(&mut left, &w.total());
                    }
                    if !left.is_empty() {
                        shed.push((left, sys.radius.clone(), sys.phase.clone()));
                    }
                    orphans.extend(worlds.into_iter().map(|w| (w, sys.radius.clone())));
                    dissolved += 1;
                    continue;
                }
                // Its system still moves: worlds and debris meet, and what
                // oversteps the rim crosses into the galaxy.
                let (_, _, escaped) = crate::formation::collide_system(&mut sys, &unit, bell, rang);
                if !escaped.is_empty() {
                    shed.push((escaped, sys.radius.clone(), sys.phase.clone()));
                }
                enclosed_in += mass(&sys.total()) / unit.clone();
                keep.push(sys);
                continue;
            }
            let kept_before = mass(&sys.kept);
            // THE OUTBURST (operator: made IN the star; with xylarium): on
            // its bell, the heart re-cut on its burning mix throws out what
            // the new cut no longer holds.
            // The outburst lands in the system's OWN MEDIUM (with xylarium):
            // motes at each kind's carried spin and the star's phases.
            // THE OUTBURST IS THROWN COOL (operator 2026-10-18: "heat
            // dissipates into the void/vacuum"): only the deep heart is hot;
            // the outer layers it is thrown from hold whole atoms.
            let heat = ratio(0, 1);
            let burst = if rang { crate::formation::heart_cut(&sys.kept, &ratio(0, 1), &heat).map(|(_, released, _)| released).unwrap_or_default() } else { crate::pulse::Manifest::new() };
            if !burst.is_empty() {
                let spins = crate::formation::carried(&sys.kept, &ratio(0, 1), &heat);
                less(&mut sys.kept, &burst);
                // The ejecta leave together, as a spray over directions.
                for (k, a) in &burst {
                    *sys.medium.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
                }
                let star_phase = sys.phase.clone();
                let push = if mass(&sys.kept).is_zero() { ratio(0, 1) } else { ratio(1, 2) * crate::pulse::magnetism(&sys.kept) / mass(&sys.kept) };
                sys.flows.extend(crate::flows::spray(&burst, &spins, &star_phase, &ratio(0, 1), bell, &crate::ring::grain(&push)));
            }
            let mut back = crate::pulse::Manifest::new();
            // THE WIND (operator: made at the INTERFACE): the star's
            // outbound continuum meets the medium's counter-moving vacuum
            // stream. The counterflow is the vacuum-stream motes within the
            // star's dispersal band, each by its holding and the PRODUCT OVER
            // PLANES of its in-step share with the star (flows meet where
            // they coincide, pass where they don't); each open kind leaves by
            // its open share times that counterflow. No counterflow, no wind.
            if let Some((p_g, h_g, _)) = &pre {
                if !scale_pre.is_zero() && !g_before.is_zero() {
                    let r = sys.radius.clone() / scale_pre.clone();
                    let (lo, hi) = (r.clone() * (ratio(1, 1) - p_g.clone()), r.clone() * (ratio(1, 1) + p_g.clone()));
                    let chi = core.flows.iter().filter(|x| x.vacuum).fold(ratio(0, 1), |acc, x| {
                        let rx = x.ell_o.clone() / h_g.clone();
                        if rx < lo || rx > hi {
                            return acc;
                        }
                        let c = (0..planes).fold(ratio(1, 1), |c, k| c * crate::flows::in_step(&x.phase[k], &sys.phase[k]));
                        acc + x.amount.clone() * crate::flows::weight(&x.key) / g_before.clone() * c
                    }).min(ratio(1, 1));
                    if chi > ratio(0, 1) {
                        for (k, a) in sys.kept.clone() {
                            let left = a.clone() - back.get(&k).cloned().unwrap_or_else(|| ratio(0, 1));
                            let w = crate::pulse::nucleus(&(left.clone() * crate::flows::open(&k) * chi.clone())).min(left);
                            if w > ratio(0, 1) {
                                *back.entry(k).or_insert_with(|| ratio(0, 1)) += w;
                            }
                        }
                    }
                }
            }
            // SPIN-DOWN: what leaves carries its share of the star's spin.
            if !kept_before.is_zero() {
                sys.turning = crate::ring::grain(&(sys.turning.clone() * (ratio(1, 1) - mass(&back) / kept_before)));
            }
            // DECOHERENCE (operator: "coherence holds, decoherence splits";
            // with xylarium): the star's coherence with its GALAXY'S BELL,
            // the product over planes of its phases' in-step shares against
            // the bell, `C_sg`. As it slips it SHEDS GRADUALLY, the share
            // `1 − C_sg` of what its heart holds above its ignition share;
            // where `C_sg` falls below its heart's hold share (what it holds
            // of its system), the DECOHERENCE FRONT: it comes apart whole and
            // everything returns to the galaxy's flows with its state.
            let c_sg = (0..planes).fold(ratio(1, 1), |c, k| c * crate::flows::in_step(&sys.phase[k], &sys.bell[k]));
            let sys_total = mass(&sys.total());
            let heart_share = if sys_total.is_zero() { ratio(0, 1) } else { mass(&sys.kept) / sys_total };
            // The front needs the slip HELD across a full bell of the star's
            // own (with xylarium): out of step at both ends of a ring it
            // completed, never an instantaneous dip.
            if rang && c_before < heart_share && c_sg < heart_share {
                core.medium_spin += subtree_spin(&sys, &unit);
                let all = sys.total();
                returned += mass(&all) / unit.clone();
                shed.push((all, sys.radius.clone(), sys.phase.clone()));
                dissolved += 1;
                continue;
            }
            // It sheds by what it SLIPPED this echo (the fall in its
            // coherence since the last), once, not by its whole distance
            // from step every echo (with xylarium).
            let above = (ratio(1, 1) - sys.reservoir.clone()).max(ratio(0, 1));
            let slipped = (c_before - c_sg).max(ratio(0, 1)) * above;
            for (k, a) in sys.kept.clone() {
                let left = a.clone() - back.get(&k).cloned().unwrap_or_else(|| ratio(0, 1));
                let d = crate::pulse::nucleus(&(left.clone() * slipped.clone())).min(left);
                if d > ratio(0, 1) {
                    *back.entry(k).or_insert_with(|| ratio(0, 1)) += d;
                }
            }
            less(&mut sys.kept, &back);
            returned += mass(&back) / unit.clone();
            // THE SPIN LEDGER (docs/83 Phase 0): shed matter carries its
            // system's specific spin into the medium.
            core.medium_spin += mass(&back) / unit.clone() * sys.ell.clone();
            shed.push((back, sys.radius.clone(), sys.phase.clone()));
            if mass(&sys.kept).is_zero() {
                // Spent: all it held returns, with all its spin (its orbit's
                // and everything turning inside it).
                core.medium_spin += subtree_spin(&sys, &unit);
                let all = sys.total();
                returned += mass(&all) / unit.clone();
                shed.push((all, sys.radius.clone(), sys.phase.clone()));
                dissolved += 1;
                continue;
            }
            // BURNING (the burn rate, `formation::burn`; mycelium:
            // transmutation is the universe's own physics): for each rung,
            // the halvings of its fuel due since the last echo, on the rings
            // its nested clock received, at its coherence with the echo,
            // fuse Z + Z → 2Z (H + H → He, He + He → Be). Holding +
            // magnetism (3Z) is conserved; the system's spin is kept exact as
            // its holding transduces.
            let in_phase = coherence(&slip(&own, bell, t_u));
            // EVERY KIND A RUNG (the law names no rungs: Z + Z → 2Z wherever
            // the fusion changes the open count, `formation::burn`).
            let rungs: Vec<u32> = sys.kept.keys().filter_map(|k| k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok())).collect();
            for z in rungs {
                let key = format!("Z{z:03}");
                if !sys.kept.contains_key(&key) {
                    continue;
                }
                let since = |at: u64| (own.clone() * ratio(at.saturating_sub(sys.born) as i64, 1)).floor().to_integer().try_into().unwrap_or(0u64);
                let (now_rings, then_rings) = (since(bell), since(bell.saturating_sub(t_u)));
                if let (Some((every, _)), Some(fuel)) = (crate::formation::burn(&sys.kept, &(mass(&sys.kept) / mass(&sys.total())), z, &in_phase), sys.kept.get(&key).cloned()) {
                    let due = (now_rings / every).saturating_sub(then_rings / every).min(64) as u32;
                    if due > 0 {
                        let left = fuel.clone() / Ratio::from_integer(num_bigint::BigInt::from(2).pow(due));
                        // Whole grains of the product: two fuel grains each.
                        let made = crate::pulse::nucleus(&((fuel.clone() - left) / ratio(2, 1)));
                        if made > ratio(0, 1) {
                            let spin = mass(&sys.total()) * sys.ell.clone();
                            less(&mut sys.kept, &[(key.clone(), made.clone() * ratio(2, 1))].into());
                            *sys.kept.entry(crate::formation::product(z)).or_insert_with(|| ratio(0, 1)) += made;
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
            // THE CLOSED LIMIT: no kind it holds can burn on (whatever its
            // rhythm), so it is spent and becomes a remnant, its closing mass
            // recorded.
            let share_now = if mass(&sys.total()).is_zero() { ratio(0, 1) } else { mass(&sys.kept) / mass(&sys.total()) };
            // Spent means it cannot fuse one pair of nuclei within an echo even
            // in full step: matter is counted in nuclei, so a burn slower
            // than that makes nothing, ever, at its rate.
            let rings_echo = rings_at(bell).saturating_sub(rings_at(bell.saturating_sub(t_u))).max(1);
            let pair = Ratio::new(2.into(), crate::pulse::nuclei_per_grain() * num_bigint::BigInt::from(1024));
            // It fuses at least one pair in an echo when the fuel the echo's
            // rings take (a share `rings/every` of it, the halving's lower
            // bound) reaches a pair; a slow halving of a large stock still
            // fuses a great many nuclei.
            let can_burn = sys.kept.iter().any(|(k, a)| {
                *a >= pair && k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok()).is_some_and(|z| crate::formation::burn(&sys.kept, &share_now, z, &ratio(1, 1)).is_some_and(|(every, _)| a.clone() * ratio(rings_echo as i64, 1) >= pair.clone() * ratio(every.min(i64::MAX as u64) as i64, 1)))
            });
            if !can_burn && !sys.kept.is_empty() {
                sys.remnant = true;
                sys.state = mass(&sys.kept);
            }
            // COLLISIONS in the system's own medium: worlds grow; what
            // overreaches the system's rim crosses into the galaxy.
            let (_, _, escaped) = crate::formation::collide_system(&mut sys, &unit, bell, rang);
            if !escaped.is_empty() {
                shed.push((escaped, sys.radius.clone(), sys.phase.clone()));
            }
            enclosed_in += mass(&sys.total()) / unit.clone();
            keep.push(sys);
        }
        // RE-ANCHORING (with xylarium): an emptied remnant's worlds pass to the
        // nearest star still burning, by reach; with none, they cross whole
        // into the galaxy's flows.
        for (w, at) in orphans {
            let host = keep.iter_mut().filter(|s| !s.remnant).min_by_key(|s| if s.radius > at { s.radius.clone() - at.clone() } else { at.clone() - s.radius.clone() });
            match host {
                Some(h) => h.satellites.push(w),
                None => shed.push((w.total(), at, vec![ratio(0, 1); planes])),
            }
        }
        core.satellites = keep;
        // THE FLOWS (`crate::flows`; docs/83, xylarium 11.0; STAGED on two
        // planes, the orbit's and the circulation's): the galaxy's matter is
        // motes carrying their state; the core's bell kicks every mote
        // by its hemisphere against the bell; the exits, the cut and the
        // clumps are READ from where the motes' states have carried them.
        let mut jets = 0;
        let mut stars = 0;
        let mut to_ocean = crate::pulse::Manifest::new();
        let mut with = core.total();
        for (k, a) in &gathered {
            *with.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
        }
        let g_mass = mass(&with);
        if let Some((p, h, mean)) = crate::pulse::galaxy_push(&with) {
            let scale = g_mass.clone() / unit.clone();
            let zero_phase = vec![ratio(0, 1); crate::flows::PLANES];
            // Arrivals: the push carries them as every kind, and they keep
            // the crossing's turning in each plane (carried spins add), in
            // step with the bell where the crest gathered them.
            for (k, a) in gathered {
                let ell_o = p.clone() * mean.clone() / crate::flows::weight(&k) + g_rate.clone() * turning.0.clone();
                let ell_c = g_rate.clone() * turning.1.clone();
                // It arrives with the Wellspring's winding: its loop leads its orbit by
                // the push share (½ at the universe tier), the jet's helix.
                core.flows.push(crate::flows::Mote { key: k, amount: a, ell_o: crate::ring::grain(&ell_o), ell_c: crate::ring::grain(&ell_c), phase: vec![ratio(0, 1), ratio(1, 2)], agitation: ratio(0, 1), born: bell, vacuum: true });
            }
            // Shed matter crosses whole with its star's orbit (`ℓ = R·H`).
            for (m, r, star_phase) in shed {
                let r_own = if scale.is_zero() { ratio(0, 1) } else { crate::ring::grain(&(r / scale.clone())) };
                for (k, a) in m {
                    // It keeps its star's phases (state, inherited), on the
                    // star's continuum stream.
                    let phase = if star_phase.len() == crate::flows::PLANES { star_phase.clone() } else { zero_phase.clone() };
                    core.flows.push(crate::flows::Mote { key: k, amount: a, ell_o: r_own.clone() * h.clone(), ell_c: ratio(0, 1), phase, agitation: ratio(0, 1), born: bell, vacuum: false });
                }
            }
            // THE KICK, its spin from the core's reservoir (the antipole).
            // The reservoir is kept in the galaxy's FOUNDED holding (its
            // core's state, never drained), one fixed unit, so its exact sum
            // never compounds denominators echo on echo.
            let ledger_unit = crate::pulse::core_state(core);
            let throat = crate::flows::Enclosed::new(h.clone(), Vec::new());
            let given = crate::flows::kick(&mut core.flows, enc.as_ref().unwrap_or(&throat), &p, &g_rate, &delta_u, &ledger_unit);
            core.reservoir -= given;
            // THE SHEAR: neighbouring rings pass turning outward; the inner
            // falls in (the loop becomes an orbit). Σ m·ℓ unchanged.
            let _ = crate::flows::shear(&mut core.flows, enc.as_ref().unwrap_or(&throat), &ledger_unit);
            // THE EXITS: the jet (along the axis) and radial escape cross
            // whole into the ocean (their state's onward carriage owed: the
            // ocean is not yet motes).
            for x in crate::flows::exits(&mut core.flows, &h) {
                *to_ocean.entry(x.key).or_insert_with(|| ratio(0, 1)) += x.amount;
                jets += 1;
            }
            let (kept, free) = crate::flows::read_cut(&core.flows, &h, &p);
            core.kept = kept;
            core.medium = free;
            // THE CLUMPS, where the motes are; the echo compresses only
            // its coherent part (recoherence).
            let h_units = mass(&core.kept) / unit.clone() + state.clone();
            let chains = crate::flows::chains(&core.flows, &h, &p, &g_mass);
            // The stream of every mote, read by force against its filament.
            let _ = crate::flows::stream(&mut core.flows, &chains, &(p.clone() * g_rate.clone() * delta_u.clone()));
            let clumps = crate::flows::clumps_of(&core.flows, chains);
            // Each take: (reach, clump, agitation, the star's own spin, phases).
            let mut takes: Vec<(Ratio, crate::pulse::Manifest, Ratio, Ratio, Vec<Ratio>)> = Vec::new();
            for (reach, shares) in clumps {
                let mut clump = crate::pulse::Manifest::new();
                let (mut heat, mut amt) = (ratio(0, 1), ratio(0, 1));
                let mut members: Vec<(Ratio, Ratio, Vec<Ratio>)> = Vec::new();
                for (j, s) in shares {
                    let x = &mut core.flows[j];
                    let amount = crate::pulse::nucleus(&(x.amount.clone() * s)).min(x.amount.clone());
                    if amount <= ratio(0, 1) {
                        continue;
                    }
                    x.amount -= amount.clone();
                    heat += amount.clone() * x.agitation.clone();
                    amt += amount.clone();
                    members.push((amount.clone() * crate::flows::weight(&x.key), x.ell_o.clone(), x.phase.clone()));
                    *clump.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += amount;
                }
                if !clump.is_empty() {
                    let a = if amt.is_zero() { ratio(0, 1) } else { heat / amt };
                    // THE STAR'S OWN SPIN AND PHASES, captured at birth (with
                    // xylarium): its members' holding-weighted spread of
                    // orbital spin about their own mean, and their
                    // holding-weighted phases.
                    let m = members.iter().fold(ratio(0, 1), |s, (w, _, _)| s + w.clone());
                    let (spin, phases) = if m.is_zero() {
                        (ratio(0, 1), vec![ratio(0, 1); crate::flows::PLANES])
                    } else {
                        let mean = members.iter().fold(ratio(0, 1), |s, (w, l, _)| s + w.clone() * l.clone()) / m.clone();
                        let spread = members.iter().fold(ratio(0, 1), |s, (w, l, _)| {
                            let d = l.clone() - mean.clone();
                            s + w.clone() * if d < ratio(0, 1) { -d } else { d }
                        }) / m.clone();
                        let phases = (0..crate::flows::PLANES).map(|k| crate::ring::grain(&crate::flows::circle_mean(&members.iter().map(|(w, _, ph)| (w.clone(), ph[k].clone())).collect::<Vec<_>>()))).collect();
                        (crate::ring::grain(&spread), phases)
                    };
                    takes.push((reach, clump, a, spin, phases));
                }
            }
            for (reach, star, a_own, spin_own, phases_own) in takes {
                let m_u = mass(&star) / unit.clone();
                if m_u.is_zero() {
                    continue;
                }
                // IGNITION: its agitation plus half the tide's unlocked face.
                let tide = crate::ring::deficit(&reach, &h);
                let a_clump = a_own.clone() + (ratio(1, 1) - tide.clone()) / ratio(2, 1);
                // Its orbit: the motes' reach carried onto the ledger.
                let taken = m_u.clone() * crate::ring::spin(&(reach.clone() * scale.clone()), &h_units);
                let held = core.satellites.iter().fold(mass(&core.kept), |acc, sys| acc + mass(&sys.total())) / unit.clone() + state.clone();
                let r_guess = crate::ring::radius(&(taken.clone() / m_u.clone()), &held).unwrap_or_else(|| reach.clone() * scale.clone());
                let siblings: Vec<u64> = core.satellites.iter().map(|l| l.path[1]).collect();
                let path = vec![core.path[0], crate::ring::birth_key(bell, &r_guess, &held, &siblings)];
                // CARRIED SPINS ADD: the star's own spin, captured from its
                // members' spread, is the turning its heart cut adds to every
                // kind (no weight ratio).
                let lit = crate::formation::ignition_share(&a_clump).and_then(|_| crate::formation::system(&star, &a_clump, &spin_own, &path, &unit, &tide));
                let Some((kept, orbs, medium, medium_spin, world_load, disc_motes)) = lit else {
                    // No heart: the matter stays a mote where it was.
                    for (k, a) in star {
                        core.flows.push(crate::flows::Mote { key: k, amount: a, ell_o: reach.clone() * h.clone(), ell_c: ratio(0, 1), phase: vec![ratio(0, 1); crate::flows::PLANES], agitation: a_own.clone(), born: bell, vacuum: false });
                    }
                    continue;
                };
                // Its STATE is its holding at birth (a heart's state, set at
                // its forming, never drained); its own spin and phases.
                let held_at_birth = mass(&kept);
                // A star keeps its IGNITION SHARE (the heart's share at its
                // throat condition) in `reservoir`: decoherence sheds only
                // what it holds above it.
                let ignition = crate::formation::ignition_share(&a_clump).unwrap_or_else(|| ratio(0, 1));
                let mut sys = Body { path, radius: r_guess, period: ratio(0, 1), kept, satellites: orbs, medium, ell: ratio(0, 1), medium_spin, gas: Vec::new(), born: bell, state: held_at_birth, flows: disc_motes, reservoir: ignition, turning: spin_own, bell: phases_own.clone(), phase: phases_own, load: world_load, remnant: false, heat: ratio(0, 1) };
                let internal = subtree_spin(&sys, &unit);
                let own = taken - internal;
                sys.ell = crate::ring::grain(&(own.clone() / m_u.clone()));
                core.medium_spin += own - m_u * sys.ell.clone();
                core.satellites.push(sys);
                stars += 1;
            }
            core.flows.retain(|x| x.amount > ratio(0, 1));
            let remainder = crate::flows::merge(&mut core.flows, &ledger_unit);
            core.reservoir += remainder;
            let (kept, free) = crate::flows::read_cut(&core.flows, &h, &p);
            core.kept = kept;
            core.medium = free;
            core.gas.clear();
        }
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
                    // THE STAR'S VELOCITY (xylarium's operator, 2026-10-17:
                    // "the inverse of the drag on the planets"): its pull
                    // through the galaxy, shared against its worlds' load,
                    // `v = δ_gal·H/(H + d·Σx)` (H its system's own hold, 1
                    // in its units; d its torsion `r/(r + H_gal)`): a loaded
                    // star turns slower, its period stretched by `1 + d·Σx`.
                    let d = r.clone() / (r.clone() + inside.clone());
                    sys.period = crate::ring::period(&r, &inside) * (ratio(1, 1) + d * sys.load.clone());
                    sys.radius = r;
                }
            }
            inside += mass(&sys.total()) / unit.clone();
        }
        let galaxy_id = core.path[0] as u32;
        for (k, a) in to_ocean {
            *u.medium.entry(k).or_insert_with(|| ratio(0, 1)) += a;
        }
        events.push(Event { bell, galaxy: galaxy_id, rung, write: Write::Perpendicular, stars, jets, returned, dissolved, merged, encounters });
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
