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

/// The unit of mass: the deepest core's hole, as founded (`Universe::unit`).
pub fn unit_mass(u: &Universe) -> Ratio {
    u.unit.clone()
}

/// A galaxy's clock against the universal source: the share of the source's
/// rings its core receives at its distance, about the holding nearer the
/// epicenter (the universal core's state, and each nearer galaxy's core state
/// and grains). The universal bell IS the source's own pulse, and a clock is
/// the share of it a receiver gets, `r/(r + H)`.
/// None for the HOME: its core is the source itself, so no receiver stands
/// between the home's bodies and the source. (`rate(0, H) = 0` would be the
/// clock of a receiver standing AT the singularity, where nothing stands.)
pub fn galaxy_rate(u: &Universe, i: usize, unit: &Ratio) -> Option<Ratio> {
    if crate::pulse::is_home(&u.cores[i]) {
        return None;
    }
    let d = &u.cores[i].radius;
    let nearer = u.cores.iter().filter(|c| c.radius < *d).fold(crate::pulse::universal_state(u) / unit.clone(), |s, c| s + (crate::pulse::core_state(c) + mass(&c.total())) / unit.clone());
    Some(rate(d, &nearer))
}

/// THE NESTED CLOCK: a body's clock against the source is the product of the
/// shares each receiver between them takes, the galaxy's (none for the home)
/// and its own about what is inside its reach.
pub fn nested(galaxy: Option<&Ratio>, own: Ratio) -> Ratio {
    match galaxy {
        Some(g) => g.clone() * own,
        None => own,
    }
}

/// The state a galaxy's bodies orbit about: its core's, and for the home the
/// singularity's own (`pulse::universal_state`), never a second state.
pub fn centre_state(u: &Universe, i: usize) -> Ratio {
    let c = &u.cores[i];
    if crate::pulse::is_home(c) { crate::pulse::universal_state(u) } else { crate::pulse::core_state(c) }
}

/// A star system's nested clock: its galaxy's share (none for the home) times
/// its orbit's about the centre's state, the core's held grains, and every
/// system inside it.
pub fn system_clock(u: &Universe, i: usize, s: usize, unit: &Ratio) -> Ratio {
    let core = &u.cores[i];
    let inside = core.satellites[..s].iter().fold(mass(&core.kept) + centre_state(u, i), |a, sys| a + mass(&sys.total())) / unit.clone();
    nested(galaxy_rate(u, i, unit).as_ref(), rate(&core.satellites[s].radius, &inside))
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

/// Rungs climbed: completed slips, `⌊Δ⌋` (refused past the carrier).
pub fn rungs(delta: &Ratio) -> Option<u64> {
    crate::ring::whole(delta)
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
    /// RECYCLING at the galaxy's throat this echo (`crate::recycle`): nuclei
    /// pressed and peeled, the light given and the holding opened, and the
    /// holding trapped, in holding units of the deepest hole.
    pub recycled: crate::recycle::Recycled,
    /// THE BURN AS A RATE (TUniversal 11.0 (23): path-dependent laws are read
    /// as rates from a checkpoint): the fuel the galaxy's stars fused this
    /// echo, by kind, in grains.
    pub burned: crate::pulse::Manifest,
    /// THE BURN'S RUNS this echo, as the composition feeds the form
    /// (`crate::compound`): each star's path, the rung's fuel Z, the share a
    /// ring and the rings its own clock received.
    pub burns: Vec<(Vec<u64>, u32, Ratio, u64)>,
    /// THE CROSS this echo (`enfold::Cross`): every residue's fate by plane.
    pub cross: crate::enfold::Cross,
}

/// Breathe the universe over bells `from` (exclusive) to `until`, echoes
/// every `t_u` bells (the conductor calls it once a beat, `(beat − 1, beat]`).
/// Events in time order. Each echo is a perpendicular write down; each birth
/// answers it with a perpendicular write up (the galaxy's mass within the
/// new orbit changes), closing a loop.
pub fn breathe(u: &mut Universe, _seed: &[u8; 32], t_u: u64, from: u64, until: u64) -> Vec<Event> {
    let unit = unit_mass(u);
    use num_traits::Zero as _;
    // THE GALAXIES ORBIT (`pulse::orbit_galaxies`): on each universal echo,
    // before its arrivals are read, every core re-reads its reach.
    // The firmament's cross is carried into the first arrival's record.
    let mut pending = crate::enfold::Cross::default();
    if t_u > 0 && until / t_u > from / t_u {
        pending = crate::pulse::orbit_galaxies(u);
    }
    // Every echo's arrival, in time order.
    let mut arrivals: Vec<(u64, usize, u64)> = Vec::new();
    for (i, c) in u.cores.iter().enumerate() {
        let Some(delay) = crate::ring::whole(&c.radius) else { continue };
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
            let whole = u.cores.iter().fold(crate::pulse::mass(&u.medium), |s, c| s + crate::pulse::mass(&c.total()));
            let h_g = crate::ring::grain(&((crate::pulse::core_state(c) + crate::pulse::mass(&c.total())) / whole.clone()));
            let ring_bells = ratio((t_u / 4) as i64, 1);
            let reach = c.radius.clone() / ring_bells;
            // Half swings completed by a bell (none before the founding).
            let halves = |b: u64| if c.period.is_zero() { Some(0) } else { crate::ring::whole(&(ratio(i64::try_from(b).ok()?, 1) * ratio(2, 1) / c.period.clone())) };
            let before = if bell > t_u { halves(bell - t_u) } else { Some(0) };
            let Some(half_swings) = halves(bell).zip(before).and_then(|(now, then)| now.checked_sub(then)) else { continue };
            let m = crate::ring::grain(&(crate::pulse::mass(&u.medium) / whole));
            let h_u = crate::ocean::universe_hold(u);
            let share = crate::ocean::gathered(rung, &reach, &h_g, &m, half_swings, &h_u);
            let delta_u = crate::ring::grain(&(h_u.clone() / (reach.clone() + h_u.clone())));
            (crate::ocean::take(&u.medium, &share), crate::ocean::arrival_turning(rung, &reach, &h_g, &m, &h_u), delta_u)
        };
        less(&mut u.medium, &gathered);
        // THE PARENT'S PULSE on this galaxy's bodies: its received share of the
        // source's rings and the universal tide at its distance; None in the
        // home, whose bodies stand in the source's own enclosure and feel the
        // pulse at their own reach.
        let tide_of = g_rate.clone().map(|g| (g, delta_u.clone()));
        let tide = tide_of.as_ref().map(|(g, d)| (g, d));
        // THE HOME (`pulse::is_home`) is read in the universe's own units: its
        // core is the universal core, its state the singularity's, its push
        // the universal bounce's, and the firmament stands in its enclosure.
        let home = crate::pulse::is_home(&u.cores[i]);
        let whole_u = mass(&u.total());
        let home_push = if home { crate::pulse::home_push(u, &u.cores[i].total()) } else { None };
        let firm_held: Vec<(Ratio, Ratio)> = if home && !whole_u.is_zero() { u.firmament.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / whole_u.clone())).collect() } else { Vec::new() };
        let core = &mut u.cores[i];
        let mut returned = ratio(0, 1);
        let mut dissolved = 0;
        let (mut merged, mut encounters) = (0usize, 0usize);
        // The galaxy's holding constant through the echo (its grains only
        // move within it), so its core's state is read once.
        let state = if home { whole_u.clone() } else { crate::pulse::core_state(core) } / unit.clone();
        let mut enclosed_in = mass(&core.kept) / unit.clone() + state.clone();
        let mut keep: Vec<Body> = Vec::new();
        // What the stars shed crosses whole into the galaxy's flows.
        let mut shed: Vec<(crate::pulse::Manifest, Ratio, Vec<Ratio>)> = Vec::new();
        // What a remnant passes through its other face, the galactic core's
        // throat (recycled there, then on with the shed).
        let mut infall: Vec<(crate::pulse::Manifest, Ratio, Vec<Ratio>)> = Vec::new();
        let mut recycled = crate::recycle::Recycled::default();
        let mut burned = crate::pulse::Manifest::new();
        let mut burns: Vec<(Vec<u64>, u32, Ratio, u64)> = Vec::new();
        let mut cross = std::mem::take(&mut pending);
        // The galaxy's push and throat in its own units, for the wind's
        // meeting with the medium.
        let g_before = if home { whole_u.clone() } else { mass(&core.total()) };
        let pre = if home { home_push.clone() } else { crate::pulse::galaxy_push(&core.total()) };
        let scale_pre = g_before.clone() / unit.clone();
        // THE ENCLOSED HOLDING, read once from the echo's start: the
        // singularity's own state and every mote and star inside, by spin.
        let enc = pre.as_ref().filter(|_| !g_before.is_zero()).map(|(_, h_g, _)| {
            let motes = core.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g_before.clone()));
            let stars = core.satellites.iter().map(|b| (b.radius.clone() / scale_pre.clone() * h_g.clone(), mass(&b.total()) / g_before.clone()));
            crate::flows::Enclosed::new(ratio(1, 1), motes.chain(stars).chain(firm_held.iter().cloned()).collect())
        });
        // STARS COLLIDE (operator 2026-10-18; with xylarium): the one
        // collide law among the galaxy's stars, before each reads its echo;
        // each carries the coherence it had before the encounter.
        let planes = crate::flows::PLANES;
        let c_pre: Vec<Ratio> = core.satellites.iter().map(|b| if b.phase.len() < planes || b.bell.len() < planes { ratio(1, 1) } else { (0..planes).fold(ratio(1, 1), |c, k| c * crate::flows::in_step(&b.phase[k], &b.bell[k])) }).collect();
        let (met, star_sticks, star_bounces) = match &pre {
            Some((p_g, h_g, _)) if enc.is_some() => crate::formation::collide_stars(std::mem::take(&mut core.satellites), c_pre, &scale_pre, h_g, p_g, &g_before, enc.as_ref().expect("checked"), &mut cross),
            _ => (std::mem::take(&mut core.satellites).into_iter().zip(c_pre).collect(), 0, 0),
        };
        // WHERE LEAVING MATTER IS SET DOWN (with xylarium; the wind law: the
        // outflow ends at the star's rim): at the edge of the star's own
        // region, R_s(1 + e) with e = H_s/(H_s + H_g), never at its orbit,
        // where the star would recapture it at once.
        let beyond = |b: &Body| -> Ratio {
            match &pre {
                Some((_, h_g, _)) if !g_before.is_zero() => {
                    let h_s = mass(&b.total()) / g_before.clone();
                    crate::ring::grain(&(b.radius.clone() * (ratio(1, 1) + h_s.clone() / (h_s + h_g.clone()))))
                }
                _ => b.radius.clone(),
            }
        };
        // Worlds of emptied remnants, re-anchored after the stars are read.
        let mut orphans: Vec<(Body, Ratio, Vec<Ratio>)> = Vec::new();
        merged += star_sticks;
        encounters += star_bounces;
        for (mut sys, c_was) in met {
            let own = nested(g_rate.as_ref(), rate(&sys.radius, &enclosed_in));
            // THE STAR'S BELL (with xylarium): its own spin turns it,
            // `ω_s = δ(ℓ_s/H_s)` a ring, `H_s = 1 + held/held_at_birth`; its
            // phases advance by the rings its clock received this echo.
            if sys.phase.len() < planes {
                sys.phase = vec![ratio(0, 1); planes];
            }
            let h_s = if sys.state.is_zero() { ratio(1, 1) } else { ratio(1, 1) + mass(&sys.kept) / sys.state.clone() };
            let omega = h_s.clone() / (sys.turning.clone() / h_s.clone() + h_s.clone());
            // Rings its clock received from its birth to a bell (none before).
            let rings_at = |at: u64| -> Option<u64> { if at > sys.born { crate::ring::whole(&(own.clone() * ratio(i64::try_from(at - sys.born).ok()?, 1))) } else { Some(0) } };
            let echo_start = if bell > t_u { bell - t_u } else { 0 };
            let (Some(r_now), Some(r_then)) = (rings_at(bell), rings_at(echo_start)) else {
                keep.push(sys);
                continue;
            };
            let Some(r_echo) = r_now.checked_sub(r_then) else {
                keep.push(sys);
                continue;
            };
            // Its own bell rings on its whole turns (its spin, ω_s): it times
            // its outbursts and its system, never a phase against its galaxy.
            let as_ratio = |n: u64| Ratio::from_integer(num_bigint::BigInt::from(n));
            let rang = (omega.clone() * as_ratio(r_now)).floor() > (omega.clone() * as_ratio(r_then)).floor();
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
                    // The star's place: the cap its own gravity holds about it.
                    let place = crate::enfold::place(&r, &(mass(&sys.total()) / g_before.clone()), &h_in);
                    let heat_was = sys.heat.clone();
                    crate::flows::turn(&mut sys.bell, &r, &h_in, p_g, tide, false, place.as_ref(), &heat_was, &mut cross);
                    // THE RETURN (with xylarium): its galaxy's torque gives
                    // back `d/(d + turned)` of its displacement in each plane,
                    // `d = 1 − δ_gal` its drag, `turned` its own spin's turns
                    // this echo: a fast spinner is a gyroscope.
                    let d = ratio(1, 1) - d_enc;
                    let turned = omega.clone() * as_ratio(r_echo);
                    let keep_share = if (d.clone() + turned.clone()).is_zero() { ratio(1, 1) } else { turned.clone() / (d + turned) };
                    for k in 0..planes {
                        let off = sys.phase[k].clone() - was[k].clone();
                        let f = off.clone() - off.floor();
                        let signed = if f > ratio(1, 2) { f - ratio(1, 1) } else { f };
                        let g = sys.bell[k].clone() + signed * keep_share.clone();
                        // ENFOLD (docs/85 §8, E2″): the star is held to its own
                        // place; what lies finer is the detail of its own
                        // enclosure, so each member's place against the star
                        // takes the residue and stays exactly where it is.
                        // Nothing moves and nothing is dropped.
                        let (held, residue) = crate::enfold::hold(&g, place.as_ref());
                        sys.phase[k] = held;
                        if !residue.is_zero() && sys.flows.is_empty() && sys.satellites.is_empty() {
                            // A star with no members holds its residue as a
                            // leaf: in its round it drains, past it the
                            // quantum tier acknowledges it.
                            let fate = match crate::enfold::residue(&residue, &sys.heat, None) {
                                crate::enfold::Residue::Drained => crate::enfold::Fate::Drained,
                                _ => crate::enfold::Fate::Acknowledged,
                            };
                            cross.note(k, fate, &residue);
                        } else if !residue.is_zero() {
                            cross.note(k, crate::enfold::Fate::Descended, &residue);
                            let shift = |ph: &mut Ratio| {
                                let x = ph.clone() + residue.clone();
                                *ph = x.clone() - x.floor();
                            };
                            for x in sys.flows.iter_mut() {
                                if let Some(ph) = x.phase.get_mut(k) {
                                    shift(ph);
                                }
                            }
                            for w in sys.satellites.iter_mut() {
                                if let Some(ph) = w.phase.get_mut(k) {
                                    shift(ph);
                                }
                            }
                        }
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
                // It drains where its galaxy is read (the deficit at its reach)
                // and its orbit's phase is carried; otherwise nothing moves.
                let d = match (&pre, &enc) {
                    (Some((_, h_g, _)), Some(enc)) if !scale_pre.is_zero() => Some(enc.reach(&(sys.radius.clone() / scale_pre.clone() * h_g.clone())).1),
                    _ => None,
                };
                let now = mass(&sys.kept);
                if let (Some(d), Some(orbit), false) = (d, sys.phase.first().cloned(), now.is_zero()) {
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
                    // It passes through the galactic core's throat (its
                    // other face): matter standing there to be recycled.
                    infall.push((out, beyond(&sys), vec![orbit, ratio(1, 2)]));
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
                        shed.push((left, beyond(&sys), sys.phase.clone()));
                    }
                    orphans.extend(worlds.into_iter().map(|w| (w, sys.radius.clone(), sys.phase.clone())));
                    dissolved += 1;
                    continue;
                }
                // Its system still moves: worlds and debris meet, and what
                // oversteps the rim crosses into the galaxy.
                let (_, _, escaped) = crate::formation::collide_system(&mut sys, &unit, bell, rang, t_u, &mut cross);
                if !escaped.is_empty() {
                    shed.push((escaped, beyond(&sys), sys.phase.clone()));
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
                let push = if mass(&sys.kept).is_zero() { ratio(0, 1) } else { ratio(1, 2) * crate::pulse::magnetism(&sys.kept) / mass(&sys.kept) };
                // The system's motes stand against their star (its frame, as
                // at birth and capture): the spray leaves from the star itself.
                sys.flows.extend(crate::flows::spray(&burst, &spins, &vec![ratio(0, 1); planes], &ratio(0, 1), bell, &crate::ring::grain(&push)));
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
                shed.push((all, beyond(&sys), sys.phase.clone()));
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
            shed.push((back, beyond(&sys), sys.phase.clone()));
            if mass(&sys.kept).is_zero() {
                // Spent: all it held returns, with all its spin (its orbit's
                // and everything turning inside it).
                core.medium_spin += subtree_spin(&sys, &unit);
                let all = sys.total();
                returned += mass(&all) / unit.clone();
                shed.push((all, beyond(&sys), sys.phase.clone()));
                dissolved += 1;
                continue;
            }
            // BURNING (the burn rate, `formation::burn`; mycelium:
            // transmutation is the universe's own physics): for each rung,
            // each ring its nested clock received this echo fuses the rung's
            // share of the fuel that remains, at its coherence with the echo,
            // compounding (`crate::compound`), Z + Z → 2Z (H + H → He,
            // He + He → Be). Holding + magnetism (3Z) is conserved; the
            // system's spin is kept exact as its holding transduces.
            let in_phase = coherence(&slip(&own, bell, t_u));
            // EVERY KIND A RUNG (the law names no rungs: Z + Z → 2Z wherever
            // the fusion changes the open count, `formation::burn`).
            let rungs: Vec<u32> = sys.kept.keys().filter_map(crate::chem::atom_z).collect();
            for z in rungs {
                let (Some(key), Some(made_kind)) = (crate::chem::Kind::atom(z), crate::formation::product(z)) else { continue };
                if !sys.kept.contains_key(&key) {
                    continue;
                }
                if let (Some((feed, _)), Some(fuel)) = (crate::formation::burn(&sys.kept, &(mass(&sys.kept) / mass(&sys.total())), z, &in_phase), sys.kept.get(&key).cloned()) {
                    // The product in whole nuclei, two fuel nuclei each: half
                    // of what the echo's rings fused, `fuel − fuel·(1 − feed)^r`.
                    burns.push((sys.path.clone(), z, feed.clone(), r_echo));
                    let Some(made) = crate::compound::after(&feed, r_echo).and_then(|remaining| crate::compound::read(&fuel, &remaining, |left| crate::pulse::nucleus(&((fuel.clone() - left.clone()) / ratio(2, 1))))) else { continue };
                    if made > ratio(0, 1) {
                        *burned.entry(key.clone()).or_insert_with(|| ratio(0, 1)) += made.clone() * ratio(2, 1);
                        let spin = mass(&sys.total()) * sys.ell.clone();
                        less(&mut sys.kept, &[(key.clone(), made.clone() * ratio(2, 1))].into());
                        *sys.kept.entry(made_kind.clone()).or_insert_with(|| ratio(0, 1)) += made;
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
            // THE CLOSED LIMIT: no kind it holds can burn on (whatever its
            // rhythm), so it is spent and becomes a remnant, its closing mass
            // recorded.
            let share_now = if mass(&sys.total()).is_zero() { ratio(0, 1) } else { mass(&sys.kept) / mass(&sys.total()) };
            // Spent means it cannot fuse one pair of nuclei within an echo even
            // in full step: matter is counted in nuclei, so a burn slower
            // than that makes nothing, ever, at its rate.
            // An echo that brought no ring is read as one, its passage itself.
            let rings_echo = r_echo.max(1);
            let pair = Ratio::new(2.into(), crate::pulse::nuclei_per_grain() * num_bigint::BigInt::from(1024));
            // It fuses at least one pair in an echo when the fuel the echo's
            // rings take, `a − a·(1 − feed)^r`, reaches a pair; a slow burn
            // of a large stock still fuses a great many nuclei.
            let can_burn = sys.kept.iter().any(|(k, a)| {
                *a >= pair && crate::chem::atom_z(k).is_some_and(|z| crate::formation::burn(&sys.kept, &share_now, z, &ratio(1, 1)).and_then(|(feed, _)| crate::compound::read(a, &crate::compound::after(&feed, rings_echo)?, |left| a.clone() - left.clone() >= pair)).unwrap_or(false))
            });
            if !can_burn && !sys.kept.is_empty() {
                sys.remnant = true;
                sys.state = mass(&sys.kept);
            }
            // COLLISIONS in the system's own medium: worlds grow; what
            // overreaches the system's rim crosses into the galaxy.
            let (_, _, escaped) = crate::formation::collide_system(&mut sys, &unit, bell, rang, t_u, &mut cross);
            if !escaped.is_empty() {
                shed.push((escaped, beyond(&sys), sys.phase.clone()));
            }
            enclosed_in += mass(&sys.total()) / unit.clone();
            keep.push(sys);
        }
        // RE-ANCHORING (with xylarium): an emptied remnant's worlds pass to the
        // nearest star still burning, by reach; with none, they cross whole
        // into the galaxy's flows. A world stands against its star, so it is
        // re-read against its new one by the two stars' phase difference (and
        // in the galaxy's frame by its old star's phase): it stays where it is.
        let frac_of = |x: Ratio| x.clone() - x.floor();
        for (mut w, at, was) in orphans {
            let host = keep.iter_mut().filter(|s| !s.remnant).min_by_key(|s| if s.radius > at { s.radius.clone() - at.clone() } else { at.clone() - s.radius.clone() });
            match host {
                Some(h) => {
                    for k in 0..w.phase.len().min(was.len()).min(h.phase.len()) {
                        w.phase[k] = frac_of(w.phase[k].clone() + was[k].clone() - h.phase[k].clone());
                    }
                    h.satellites.push(w)
                }
                None => {
                    let place: Vec<Ratio> = (0..planes).map(|k| frac_of(w.phase.get(k).cloned().unwrap_or_else(|| ratio(0, 1)) + was.get(k).cloned().unwrap_or_else(|| ratio(0, 1)))).collect();
                    shed.push((w.total(), at, place))
                }
            }
        }
        // CAPTURE FROM THE GALAXY MEDIUM (with xylarium; the inbound half of a
        // star's circulation, the wind its outbound): a star holds the
        // galaxy's motes standing within its gravitational reach, its pinch
        // `d = R_s·H_s/(H_s + H_g)` (operator: gravity's reach), and in
        // contact with it by phase, by that contact's share, in whole nuclei;
        // a mote several stars claim divides among them. What is held joins
        // the system's own medium at its reach in the system's unit (the
        // region's edge, d, is the system's rim, 1), its phase relative to
        // the star's, carrying its own heat. Remnants do not capture.
        if let (Some((_, h_g, _)), false) = (&pre, scale_pre.is_zero() || g_before.is_zero()) {
            let reach_m = |x: &crate::flows::Mote| x.ell_o.clone() / h_g.clone();
            let mut claims: Vec<Vec<(usize, Ratio)>> = vec![Vec::new(); core.flows.len()];
            // The motes ordered by reach once; each star reads those within its
            // gravity's reach (`flows::within_reach`).
            let mut by_reach: Vec<(Ratio, usize)> = core.flows.iter().enumerate().map(|(mi, x)| (reach_m(x), mi)).collect();
            by_reach.sort();
            for (si, s) in keep.iter().enumerate() {
                if s.remnant || s.phase.is_empty() {
                    continue;
                }
                let r_s = s.radius.clone() / scale_pre.clone();
                let h_s = mass(&s.total()) / g_before.clone();
                let d = r_s.clone() * h_s.clone() / (h_s + h_g.clone());
                if d.is_zero() {
                    continue;
                }
                let d_s = h_g.clone() / (r_s.clone() + h_g.clone());
                for &(ref r_m, mi) in crate::flows::within_reach(&by_reach, &r_s, &d, &mut crate::flows::Work::default()) {
                    let x = &core.flows[mi];
                    let r_m = r_m.clone();
                    let gap = if r_m > r_s { r_m.clone() - r_s.clone() } else { r_s.clone() - r_m.clone() };
                    if gap >= d {
                        continue;
                    }
                    // ONCE PER PASSAGE (with xylarium): the mote is taken each
                    // time their orbit phases' difference crosses a whole turn
                    // this echo (the dance's conjunctions, in closed form), a
                    // whole passage's contact (½) each time; a mote moving
                    // with the star is never swept again.
                    let d_m = h_g.clone() / (r_m + h_g.clone());
                    let psi = x.phase[0].clone() - s.phase[0].clone();
                    let turn = ratio(2, 1) * (d_m - d_s.clone());
                    let crossed = (psi.clone() + turn).floor() - psi.floor();
                    let crossings = if crossed < ratio(0, 1) { -crossed } else { crossed };
                    // Each passage holds half of what still stands: k passages
                    // hold 1 − 2^−k. Both deficits lie in (0, 1], so the turn
                    // is under two and k is 0, 1 or 2; a count past the carrier
                    // is refused, never capped.
                    let Ok(k) = u32::try_from(crossings.to_integer()) else { continue };
                    let c = ratio(1, 1) - Ratio::new(1.into(), num_bigint::BigInt::from(2).pow(k));
                    if c > ratio(0, 1) {
                        claims[mi].push((si, c));
                    }
                }
            }
            for (mi, cl) in claims.into_iter().enumerate() {
                if cl.is_empty() {
                    continue;
                }
                let n = ratio(cl.len() as i64, 1);
                let r_m = reach_m(&core.flows[mi]);
                for (si, c) in cl {
                    let take = crate::pulse::nucleus(&(core.flows[mi].amount.clone() * c / n.clone())).min(core.flows[mi].amount.clone());
                    if take <= ratio(0, 1) {
                        continue;
                    }
                    core.flows[mi].amount -= take.clone();
                    let s = &mut keep[si];
                    let r_s = s.radius.clone() / scale_pre.clone();
                    let h_s_g = mass(&s.total()) / g_before.clone();
                    let d = r_s.clone() * h_s_g.clone() / (h_s_g + h_g.clone());
                    let gap = if r_m > r_s { r_m.clone() - r_s } else { r_s - r_m.clone() };
                    let tot = mass(&s.total());
                    let h_sys = if tot.is_zero() { ratio(1, 1) } else { mass(&s.kept) / tot };
                    let x = &core.flows[mi];
                    let phase: Vec<Ratio> = x.phase.iter().zip(&s.phase).map(|(a, b)| { let f = a.clone() - b.clone(); crate::ring::grain(&(f.clone() - f.floor())) }).collect();
                    let key = x.key.clone();
                    let mote = crate::flows::Mote { key: key.clone(), amount: take.clone(), ell_o: crate::ring::grain(&(gap / d * h_sys)), ell_c: ratio(0, 1), phase, agitation: x.agitation.clone(), born: bell, vacuum: false };
                    *s.medium.entry(key).or_insert_with(|| ratio(0, 1)) += take;
                    s.flows.push(mote);
                }
            }
            core.flows.retain(|x| x.amount > ratio(0, 1));
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
        let g_mass = if home { whole_u.clone() } else { mass(&with) };
        let push_with = if home { home_push.clone() } else { crate::pulse::galaxy_push(&with) };
        if let Some((p, h, mean)) = push_with {
            let scale = g_mass.clone() / unit.clone();
            let zero_phase = vec![ratio(0, 1); crate::flows::PLANES];
            // Arrivals: the push carries them as every kind, and they keep
            // the crossing's turning in each plane (carried spins add), in
            // step with the bell where the crest gathered them.
            // THE MATTER STANDING IN THE THROAT (with xylarium): the vacuum
            // stream's inbound leg (the arrivals) and the remnants' infall.
            // (The arrivals ride the vacuum's loop; each passes the throat
            // when its loop phase crosses ½ in the kicks, below.)
            let mut in_throat: Vec<usize> = Vec::new();
            for (k, a) in gathered {
                let ell_o = p.clone() * mean.clone() / crate::flows::weight(&k) + nested(g_rate.as_ref(), turning.0.clone());
                let ell_c = nested(g_rate.as_ref(), turning.1.clone());
                // It arrives with the Wellspring's winding: its loop leads its orbit by
                // the push share (½ at the universe tier), the jet's helix.
                core.flows.push(crate::flows::Mote { key: k, amount: a, ell_o: crate::ring::grain(&ell_o), ell_c: crate::ring::grain(&ell_c), phase: vec![ratio(0, 1), ratio(1, 2)], agitation: ratio(0, 1), born: bell, vacuum: true });
            }
            // Shed matter crosses whole with its star's orbit (`ℓ = R·H`).
            let falling = infall.len();
            for (n, (m, r, star_phase)) in infall.into_iter().chain(shed).enumerate() {
                let r_own = if scale.is_zero() { ratio(0, 1) } else { crate::ring::grain(&(r / scale.clone())) };
                for (k, a) in m {
                    if n < falling {
                        in_throat.push(core.flows.len());
                    }
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
            let ledger_unit = if home { whole_u.clone() } else { crate::pulse::core_state(core) };
            let throat = crate::flows::Enclosed::new(h.clone(), Vec::new());
            // A VACUUM MOTE PASSES THE THROAT (with xylarium) when its loop
            // phase crosses ½ during the kicks (both streams pass the core
            // at loop phase ½): the inbound leg (the axis, for the vacuum)
            // delivers it to the throat.
            let loop_was: Vec<(usize, Ratio)> = core.flows.iter().enumerate().filter(|(_, x)| x.vacuum && x.phase.len() > 1).map(|(j, x)| (j, x.phase[1].clone())).collect();
            let (given, absorbed) = crate::flows::kick(&mut core.flows, enc.as_ref().unwrap_or(&throat), &p, tide, &ledger_unit, &mut cross);
            core.reservoir -= given;
            // RECYCLING (`crate::recycle`; xylarium 11.0 (18)): AFTER the
            // kicks (kicks are motion, recycling the residue), the bounce the
            // container holds presses and peels the matter in the throat.
            let frac = |x: Ratio| x.clone() - x.floor();
            // Its route runs the loop backward (drawn in along the axis, out
            // through the disc): it reaches the throat crossing ½ from above.
            for (j, was) in loop_was {
                let to_half = frac(was.clone() - ratio(1, 2));
                if to_half > ratio(0, 1) && to_half <= frac(was - core.flows[j].phase[1].clone()) {
                    in_throat.push(j);
                }
            }
            // Clusters part in the throat's heat (1, as the newborn heart's
            // cut) before they split (with xylarium).
            in_throat = crate::recycle::part(&mut core.flows, &in_throat);
            // PUNCTURE: each body entering the throat's other face.
            for _ in &in_throat {
                cross.note(0, crate::enfold::Fate::Punctured, &ratio(0, 1));
            }
            // THE CONTAINER (11.0 (18)): the core's state and the kinds held
            // at or above its cut, the heaviest that moved least; the bounce
            // it contains, B = p − Σ m·|Δℓ|.
            let container = ledger_unit.clone() + (h.clone() - ratio(1, 1)) * g_mass.clone();
            let bounce = crate::ring::grain(&(p.clone() - absorbed));
            let r = crate::recycle::recycle(&mut core.flows, &in_throat, &bounce, &container, &(g_mass.clone() - (h.clone() - ratio(1, 1)) * g_mass.clone()), bell);
            // Whatever passed the throat, split or not, leaves on the
            // continuum (operator 2026-10-19: "redistributed back via the
            // continuum"); the vacuum refills only from the ocean.
            for &j in &in_throat {
                core.flows[j].vacuum = false;
                // SEAL: it leaves on the continuum.
                cross.note(0, crate::enfold::Fate::Sealed, &ratio(0, 1));
            }
            recycled = crate::recycle::Recycled { pressed: r.pressed, light: r.light / unit.clone(), peeled: r.peeled, spent: r.spent / unit.clone(), trapped: r.trapped / unit.clone(), hydrogen: r.hydrogen };
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
            // The pulse at a mote: a galaxy's received tide, or, in the home,
            // the source's own at the mote's reach.
            let enc_now = crate::flows::Enclosed::new(ratio(1, 1), core.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g_mass.clone())).collect());
            let push_at = |x: &crate::flows::Mote| match tide {
                Some((rho, du)) => p.clone() * rho.clone() * du.clone(),
                None => p.clone() * enc_now.reach(&x.ell_o).1,
            };
            let _ = crate::flows::stream(&mut core.flows, &chains, &push_at);
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
                let Some(ignition) = crate::formation::ignition_share(&a_clump) else { continue };
                let mut sys = Body { path, radius: r_guess, period: ratio(0, 1), kept, satellites: orbs, medium, ell: ratio(0, 1), medium_spin, gas: Vec::new(), born: bell, state: held_at_birth, flows: disc_motes, reservoir: ignition, turning: spin_own, bell: phases_own.clone(), phase: phases_own, load: world_load, remnant: false, heat: ratio(0, 1) };
                let internal = subtree_spin(&sys, &unit);
                let own = taken - internal;
                sys.ell = crate::ring::grain(&(own.clone() / m_u.clone()));
                core.medium_spin += own - m_u * sys.ell.clone();
                core.satellites.push(sys);
                stars += 1;
            }
            core.flows.retain(|x| x.amount > ratio(0, 1));
            // Each mote's place for the condensation: the cap its own gravity
            // holds in the galaxy's enclosure.
            let enc_m = crate::flows::Enclosed::new(ratio(1, 1), core.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g_mass.clone())).collect());
            let place_of = |x: &crate::flows::Mote| {
                let (r, _) = enc_m.reach(&x.ell_o);
                crate::enfold::place(&r, &(x.amount.clone() * crate::flows::weight(&x.key) / ledger_unit.clone()), &enc_m.hold(&x.ell_o))
            };
            let remainder = crate::flows::merge(&mut core.flows, &ledger_unit, &place_of, &mut cross);
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
        events.push(Event { bell, galaxy: galaxy_id, rung, write: Write::Perpendicular, stars, jets, returned, dissolved, merged, encounters, recycled, burned, burns, cross });
    }
    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_home_has_no_clock_of_its_own_and_its_stars_read_the_source_directly() {
        use crate::pulse::{Body, Manifest, Universe};
        let body = |path: Vec<u64>, radius: Ratio, kept: Manifest, satellites: Vec<Body>| Body { path, radius, period: ratio(0, 1), kept, satellites, medium: Manifest::new(), ell: ratio(0, 1), medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: ratio(0, 1), flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: ratio(0, 1) };
        let h = || crate::chem::Kind::parse("Z001").expect("a kind");
        let star = body(vec![0, 1], ratio(3, 1), [(h(), ratio(1, 4))].into(), Vec::new());
        let home = body(vec![0], ratio(0, 1), Manifest::new(), vec![star]);
        let galaxy = body(vec![1], ratio(5, 1), [(h(), ratio(1, 2))].into(), Vec::new());
        let u = Universe { cores: vec![home, galaxy], medium: Manifest::new(), firmament: Vec::new(), unit: ratio(1, 1), t_u: 15 };
        // The home's core is the source: no receiver, no clock.
        assert_eq!(galaxy_rate(&u, 0, &ratio(1, 1)), None);
        assert!(galaxy_rate(&u, 1, &ratio(1, 1)).is_some());
        // Its star reads its own share about the singularity's state alone.
        let singularity = crate::pulse::universal_state(&u);
        assert_eq!(centre_state(&u, 0), singularity.clone());
        assert_eq!(system_clock(&u, 0, 0, &ratio(1, 1)), rate(&ratio(3, 1), &singularity));
        // A body's nested clock multiplies only the receivers it has.
        assert_eq!(nested(None, ratio(2, 3)), ratio(2, 3));
        assert_eq!(nested(Some(&ratio(1, 2)), ratio(2, 3)), ratio(1, 3));
    }

    #[test]
    fn a_slipping_clock_decoheres_and_recoheres_exactly() {
        // A clock at 3/4 slips a quarter turn each echo: opposed at two
        // echoes, recohered (one rung) at four.
        let r = ratio(3, 4);
        assert_eq!(coherence(&slip(&r, 120, 60)), ratio(0, 1));
        assert_eq!((coherence(&slip(&r, 240, 60)), rungs(&slip(&r, 240, 60))), (ratio(1, 1), Some(1)));
    }
}
