//! docs/78 "The breath": the universal pulse, star formation, layered time.
//!
//! - **The universal pulse.** TUniversal sets the quantum clock at the Big
//!   Bang, at the universal core: the singularity's bounce pulses every `T_u`
//!   bells, and each echo is a beat of time carried through the quantum
//!   state. An echo reaches a galaxy after its distance from the universal
//!   core (one length a bell).
//! - **The breath.** Each echo compresses a galaxy's medium. Where the binding
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

/// A GALAXY'S ONE FRAME (docs/85 §7): its enclosure in its own units (the
/// throat 1, every mote and star inside by its share of the galaxy's
/// holding at its own spin, the home's firmament too); as the breath reads
/// it. A star keeps its spin in this frame.
pub fn galaxy_frame(u: &Universe, i: usize, unit: &Ratio) -> Option<crate::flows::Enclosed> {
    let core = &u.cores[i];
    let home = crate::pulse::is_home(core);
    let g = if home { mass(&u.total()) } else { mass(&core.total()) };
    if num_traits::Zero::is_zero(&g) || num_traits::Zero::is_zero(unit) {
        return None;
    }
    let held = |x: &crate::flows::Mote| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g.clone());
    let firm: Vec<(Ratio, Ratio)> = if home { u.firmament.iter().map(held).collect() } else { Vec::new() };
    // The base: the core's state, 1, set at the bounce and never drained. It
    // IS the throat's grains, so the throat's unreleased hold is never added
    // again (counted twice it inflated the ring; docs/96 §7).
    Some(crate::flows::Enclosed::new(ratio(1, 1), core.flows.iter().map(held).chain(core.satellites.iter().map(|b| (b.ell.clone(), mass(&b.total()) / g.clone()))).chain(firm).collect()))
}

/// THE OUTFLOW'S BOOK (with xylarium 2026-10-24; the settling book's
/// inverse): matter leaving a system at spin `ℓ_at` (its region's edge,
/// `beyond`) takes `m·ℓ_at` with it, and the system's orbit pays it, `W·ℓ =
/// (W − m)·ℓ′ + m·ℓ_at`, W its whole holding before the outflow; through the
/// centre past zero; the grain's remainder to the galaxy's medium spin.
/// `m` is the holding that left (already taken from the system); a system
/// left with nothing is no recoil (it dissolved: it leaves at its own orbit).
#[allow(clippy::too_many_arguments)]
fn book_outflow(sys: &mut Body, m: &Ratio, at: &Ratio, unit: &Ratio, medium_spin: &mut Ratio, identity: &mut Vec<(&'static str, Ratio)>) {
    use num_traits::Zero as _;
    let after = mass(&sys.total());
    if after.is_zero() || m.is_zero() || unit.is_zero() {
        return;
    }
    let whole = after.clone() + m.clone();
    let before = whole * sys.ell.clone();
    let exact = (before.clone() - m.clone() * at.clone()) / after.clone();
    let stored = crate::flows::through_centre(crate::ring::grain(&exact), &mut sys.phase);
    let signed = if exact < ratio(0, 1) { -stored.clone() } else { stored.clone() };
    let rest = before.clone() - m.clone() * at.clone() - after.clone() * signed.clone();
    *medium_spin += rest.clone() / unit.clone();
    identity.push(("outflow", after * signed + m.clone() * at.clone() + rest - before));
    sys.ell = stored;
}

/// A star system's nested clock: its galaxy's share (none for the home) times
/// its orbit's, the rings it receives at its reach, `1 − δ`, δ by its
/// galaxy's one enclosure at its own spin (`galaxy_frame`).
pub fn system_clock(u: &Universe, i: usize, s: usize, unit: &Ratio) -> Ratio {
    let core = &u.cores[i];
    let own = match galaxy_frame(u, i, unit) {
        Some(enc) => ratio(1, 1) - enc.reach(&core.satellites[s].ell).1,
        None => ratio(0, 1),
    };
    nested(galaxy_rate(u, i, unit).as_ref(), own)
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

/// The spin a body carries with everything inside it: its orbit's and its
/// loop's (its mass times each specific spin, docs/90), its medium's, and
/// every satellite's within.
pub fn subtree_spin(b: &Body, unit: &Ratio) -> Ratio {
    let own = mass(&b.total()) / unit.clone() * (b.ell.clone() + b.ell_orth.clone());
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
    /// Meetings in the systems' own media this echo (`collide_system`):
    /// sticks (bodies grown), bounces, the motes and worlds the systems hold
    /// after, and the matter that overstepped their rims.
    pub grown: (usize, usize, usize, usize),
    pub escaped: Ratio,
    /// The bodies the systems grew this echo: worlds, stars ignited, and
    /// bodies whose core held nothing.
    pub fates: (usize, usize, usize, usize),
    /// Their weight: holding and nuclei of the grown groups that left past the
    /// rim, and of the worlds grown.
    pub weights: [Ratio; 4],
    /// The systems' bounces that struck as matter (extents overlapping), and
    /// the tugs between neighbouring worlds, this echo.
    pub impacts: (usize, usize, usize),
    /// The discs the stars born this echo released (holding), and the share
    /// of it that stood at or past its system's rim when released.
    pub discs: (Ratio, Ratio),
    /// THE INFALL (docs/91): the bindings' holding this echo, and of it what
    /// fell into the hearts (the rest stayed out as their discs).
    pub collapse: (Ratio, Ratio),
    /// THE MOTES' LEDGER this echo (the carrier's growth, read): the motes
    /// that entered the galaxy's flows from the ocean's arrivals, from what
    /// its stars shed and passed through their throats, from its throat's
    /// recycling, and the bindings' members returned; then how many the merge
    /// folded away, and how many the galaxy holds at the echo's end.
    pub motes: [usize; 6],
    /// THE BOUNCE'S RELEASE this echo (docs/96 step 3): the motes its tide
    /// released from the core's throat onto the continuum, their holding (in
    /// units of the deepest hole), and what the throat still holds after.
    pub released: (usize, Ratio, Ratio),
    /// THE CIRCULATION this echo (docs/94, docs/96): the spin the shear moved
    /// outward, the pressure's and the field's, in holding units of the
    /// ledger; the motes the luminal bound turned onto the Möbius leg, and
    /// the motes the stream reads on the vacuum.
    pub circulation: (Ratio, Ratio, Ratio, usize, usize),
    /// The stars' own bells that rang this echo (each a torque on its whole
    /// system, an outburst's cut, a decoherence front's check), of the stars
    /// read.
    pub rang: (usize, usize),
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
    /// THE LIGHT LEDGER this echo (units of the deepest hole): the light made
    /// (the magnetism transduced into holding, the 3Z ledger's ℚ → ℤ flux),
    /// what the bodies on its continuum face received as heat, and what
    /// drained into the vacuum. `made = received + drained`, exactly.
    pub light: Light,
    /// THE TOLLS this echo: every star that passed.
    pub tolls: Vec<Toll>,
    /// THE SPIN IDENTITIES this echo (docs/90): for each ledger the spin
    /// moves through, the residual of each check, `after − before` of its
    /// whole sum (0 where exact). `torque`: a galaxy's motes, both orbits, and
    /// its reservoir across the torques; `ride`: each star's orthogonal spin and
    /// its galaxy's reservoir across its ride.
    pub identity: Vec<(&'static str, Ratio)>,
}

/// How a star passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Passing {
    /// Its slip from its galaxy's bell held below its heart's share across a
    /// full turn of its own bell: it came apart whole.
    Decohered,
    /// It shed and burned all it held.
    Spent,
    /// A remnant whose last matter drained through its throat.
    Emptied,
}

/// THE TOLL (operator 2026-10-21: "derive the bell's toll for a star's
/// dissolution"; "build the missing components"): a star's passing as a bell
/// event. It is struck on a ring of the star's own bell (the dissolution law
/// fires only there), its STROKES the whole turns that bell made over the
/// star's life on its own clock (the passing bell's one stroke a year),
/// carrying back what it held and its spin; each galaxy hears it after its
/// route through the throats (`Toll::heard_after`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Toll {
    pub star: Vec<u64>,
    pub bell: u64,
    pub passing: Passing,
    /// None past what the carrier counts (refused, never defaulted).
    pub strokes: Option<u64>,
    /// The holding it returned (units of the deepest hole).
    pub returned: Ratio,
    /// The spin it returned (units of the deepest hole).
    pub spin: Ratio,
    /// Its reach in its galaxy and its galaxy's reach from the universal
    /// core, in light-rings: the route's arcs.
    pub star_reach: Ratio,
    pub galaxy_reach: Ratio,
}

impl Toll {
    /// The light-rings before galaxy `g` hears it: up the star's reach to its
    /// core, and for another galaxy on through the universal core (the hub
    /// ruling) and out to that galaxy's core (`enclosure::route`'s arcs).
    pub fn heard_after(&self, u: &Universe, g: usize) -> Option<Ratio> {
        let core = u.cores.get(g)?;
        if core.path.first() == self.star.first() {
            return Some(self.star_reach.clone());
        }
        Some(self.star_reach.clone() + self.galaxy_reach.clone() + core.radius.clone())
    }
}

/// The light ledger's three entries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Light {
    pub made: Ratio,
    pub received: Ratio,
    pub drained: Ratio,
}

/// LIGHT ON THE CONTINUUM FACE (operator 2026-10-21: "derive and build the
/// light ledger"): light `l` (holding units) leaves a source through the
/// continuum face, and each body standing in the enclosure it crosses takes
/// the share that face leaves UNLOCKED at its reach, `1 − δ(R)`, `δ = H/(R +
/// H)` (the ignition heat's law, `formation::system`, read at every echo and
/// at each body's own reach): its agitation rises by `l·(1 − δ)/M`, M the
/// enclosure's holding. What no body takes drains into the vacuum ("heat
/// dissipates into the void"). Every body: the motes, and the WORLDS standing
/// in it (docs/89 B2), each at its own reach `radius/scale` in the
/// enclosure's own units, its heat rising the same way. Returns (received,
/// drained).
pub fn shine(motes: &mut [crate::flows::Mote], worlds: &mut [Body], enc: &crate::flows::Enclosed, l: &Ratio, whole: &Ratio) -> (Ratio, Ratio) {
    use num_traits::Zero as _;
    if l.is_zero() || whole.is_zero() {
        return (ratio(0, 1), l.clone());
    }
    let mut received = ratio(0, 1);
    // Each body's unlocked face at its reach, read by the one enclosure it
    // stands in, at its own spin there.
    let unlocked = |ell: &Ratio| ratio(1, 1) - enc.reach(ell).1;
    for w in worlds.iter_mut() {
        let warm = l.clone() * unlocked(&w.ell) / whole.clone();
        let now = crate::ring::grain(&(w.heat.clone() + warm));
        received += (now.clone() - w.heat.clone()) * mass(&w.kept);
        w.heat = now;
    }
    for x in motes.iter_mut() {
        let warm = l.clone() * unlocked(&x.ell_o) / whole.clone();
        // What actually entered its heat (agitation still stands on the
        // coordinate's grain, docs/84 §3): the ledger counts that, so made =
        // received + drained stays exact.
        let now = crate::ring::grain(&(x.agitation.clone() + warm));
        received += (now.clone() - x.agitation.clone()) * x.amount.clone() * crate::flows::weight(&x.key);
        x.agitation = now;
    }
    let drained = l.clone() - received.clone();
    (received, drained)
}

/// THE CORE'S LIGHT ON ITS SYSTEMS (docs/91; operator 2026-10-22: "derive and
/// build them - start with the core's release heating worlds"): the same
/// continuum-face law as `shine`, one tier on, the products across tiers
/// (docs/82). Each star system standing in the galaxy takes, per unit of its
/// mass, the share the core's face leaves unlocked at its reach,
/// `l·(1 − δ(R))/M_g` (R its reach in the galaxy's own units, `radius/scale`);
/// what the system took crosses into it and is shone over its own bodies, its
/// worlds and its motes, each at its own reach in the system, by the same law
/// (`shine`, the system's own hold `h_s`); what no body inside takes drains.
/// Returns (received by the bodies inside, drained inside); the system's share
/// not taken there is in `drained`, so `l = received + drained` stays the
/// caller's exact ledger together with the motes' shine.
pub fn shine_systems(systems: &mut [Body], l: &Ratio, enc: &crate::flows::Enclosed, whole: &Ratio, unit: &Ratio) -> (Ratio, Ratio) {
    use num_traits::Zero as _;
    let (mut received, mut drained) = (ratio(0, 1), ratio(0, 1));
    if l.is_zero() || whole.is_zero() || unit.is_zero() {
        return (received, drained);
    }
    for sys in systems.iter_mut() {
        let held = mass(&sys.total());
        if held.is_zero() {
            continue;
        }
        // The system's unlocked face at its reach in the galaxy's one frame.
        let unlocked = ratio(1, 1) - enc.reach(&sys.ell).1;
        let taken = l.clone() * unlocked * held.clone() / whole.clone();
        let Some(enc_s) = crate::formation::system_frame(sys, unit) else {
            drained += taken;
            continue;
        };
        let (got, gone) = shine(&mut sys.flows, &mut sys.satellites, &enc_s, &taken, &held);
        received += got;
        drained += gone;
    }
    (received, drained)
}

/// Breathe the universe over bells `from` (exclusive) to `until`, echoes
/// every `t_u` bells (the conductor calls it once a beat, `(beat − 1, beat]`).
/// Events in time order. Each echo is a perpendicular write down; each birth
/// answers it with a perpendicular write up (the galaxy's mass within the
/// new orbit changes), closing a loop.
pub fn breathe(u: &mut Universe, _seed: &[u8; 32], t_u: u64, from: u64, until: u64) -> Vec<Event> {
    // The universe's densest element: every climb stops there (docs/97 §14).
    let densest = u.densest;
    let unit = unit_mass(u);
    use num_traits::Zero as _;
    // THE GALAXIES ORBIT (`pulse::orbit_galaxies`): on each universal echo,
    // before its arrivals are read, every core re-reads its reach.
    // The firmament's cross is carried into the first arrival's record.
    let mut pending = crate::enfold::Cross::default();
    let mut pending_firm: Option<Ratio> = None;
    if t_u > 0 && until / t_u > from / t_u {
        let (c, firm) = crate::pulse::orbit_galaxies(u);
        pending = c;
        pending_firm = firm;
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
        // THE RETURN AS A LANDING (operator 2026-10-25, relayed by xylarium:
        // "jetstreams of vacuum/continuum via the ocean of space"; "build the
        // ocean as motes"): only an INBOUND ocean mote (the vacuum leg) lands,
        // and it lands WHOLE in the one galaxy whose pull is greatest at its
        // reach, `δ_g = h_g/(|r − R_g| + h_g)` (the anchor form), among those
        // whose swing holds it, never divided by holdings; the rest flows on.
        // Each keeps its state; the crossing's turning is read at its reach.
        let (gathered, delta_u) = {
            let whole = u.cores.iter().fold(crate::pulse::mass(&u.medium()), |s, c| s + crate::pulse::mass(&c.total()));
            let ring_bells = ratio(t_u as i64, 4);
            let h_u = crate::ocean::universe_hold(u);
            // Every galaxy's reach, hold and swing, in core rings.
            let galaxies: Vec<(usize, Ratio, Ratio, (Ratio, Ratio))> = u.cores.iter().enumerate().map(|(j, cj)| {
                let hj = if whole.is_zero() { ratio(0, 1) } else { (crate::pulse::core_state(cj) + crate::pulse::mass(&cj.total())) / whole.clone() };
                let rj = cj.radius.clone() / ring_bells.clone();
                let sw = crate::ocean::swing(&rj, &hj, &h_u);
                (j, rj, hj, sw)
            }).collect();
            let reach = galaxies[i].1.clone();
            let h_g = galaxies[i].2.clone();
            // The galaxy's share of the universe: a mote's own spin crosses
            // between the galaxy's frame and the universe's by it, once (×f
            // out, /f home), so its turning survives the round trip.
            let f_g = if whole.is_zero() { ratio(0, 1) } else { crate::pulse::mass(&u.cores[i].total()) / whole.clone() };
            let ell_g = u.cores[i].ell.clone();
            let frame = crate::pulse::ocean_frame(u);
            // Each landing mote with ITS OWN crossing turning (with xylarium):
            // in the orbit plane its spin against the galaxy's orbit there,
            // `|ℓ_m − ℓ_g|/f`; across it, the swing's radial share at its reach.
            let mut landed: Vec<(crate::flows::Mote, Ratio, Ratio)> = Vec::new();
            let mut flowing: Vec<crate::flows::Mote> = Vec::new();
            for x in std::mem::take(&mut u.ocean) {
                let r_m = match (&frame, x.vacuum) {
                    (Some(f), true) => f.reach(&x.ell_o).0,
                    _ => {
                        flowing.push(x);
                        continue;
                    }
                };
                let best = galaxies.iter().filter(|(_, _, hj, (lo, hi))| !hj.is_zero() && *lo <= r_m && r_m <= *hi).max_by(|a, b| {
                    let d = |g: &(usize, Ratio, Ratio, (Ratio, Ratio))| { let gap = if r_m > g.1 { r_m.clone() - g.1.clone() } else { g.1.clone() - r_m.clone() }; g.2.clone() / (gap + g.2.clone()) };
                    d(a).cmp(&d(b)).then(b.0.cmp(&a.0))
                }).map(|g| g.0);
                if best == Some(i) {
                    let rel = if x.ell_o > ell_g { x.ell_o.clone() - ell_g.clone() } else { ell_g.clone() - x.ell_o.clone() };
                    let t_o = if f_g.is_zero() { ratio(0, 1) } else { rel / f_g.clone() };
                    let t_c = crate::ocean::radial_share(&reach, &h_g, &r_m, &h_u);
                    landed.push((x, t_o, t_c));
                } else {
                    flowing.push(x);
                }
            }
            u.ocean = flowing;
            let delta_u = h_u.clone() / (reach.clone() + h_u.clone());
            (landed, delta_u)
        };
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
        // The home's ledger unit: the universe's conserved 3Z, never its
        // moving holding (`pulse::universal_ledger`).
        let ledger_u = crate::pulse::universal_ledger(u);
        let home_push = if home { crate::pulse::home_push(u, &u.cores[i].total()) } else { None };
        let firm_held: Vec<(Ratio, Ratio)> = if home && !whole_u.is_zero() { u.firmament.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / whole_u.clone())).collect() } else { Vec::new() };
        let centre = centre_state(u, i);
        let core = &mut u.cores[i];
        let mut returned = ratio(0, 1);
        let mut dissolved = 0;
        let mut grown = (0usize, 0usize, 0usize, 0usize);
        let mut escaped_all = ratio(0, 1);
        let mut fates = (0usize, 0usize, 0usize, 0usize);
        let mut weights = [ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1)];
        let mut impacts = (0usize, 0usize, 0usize);
        let mut discs = (ratio(0, 1), ratio(0, 1));
        let mut collapse = (ratio(0, 1), ratio(0, 1));
        let mut motes_in = [0usize; 6];
        let mut released = (0usize, ratio(0, 1), ratio(0, 1));
        let mut circulation = (ratio(0, 1), ratio(0, 1), ratio(0, 1), 0usize, 0usize);
        let mut rang_n = (0usize, 0usize);
        let (mut merged, mut encounters) = (0usize, 0usize);
        // The galaxy's holding constant through the echo (its grains only
        // move within it), so its core's state is read once.
        let state = centre / unit.clone();
        let mut enclosed_in = mass(&core.kept) / unit.clone() + state.clone();
        let mut keep: Vec<Body> = Vec::new();
        // What the stars shed crosses whole into the galaxy's flows.
        let mut shed: Vec<(crate::pulse::Manifest, Ratio, Ratio, Vec<Ratio>)> = Vec::new();
        // What a remnant passes through its other face, the galactic core's
        // throat (recycled there, then on with the shed).
        let mut infall: Vec<(crate::pulse::Manifest, Ratio, Ratio, Vec<Ratio>)> = Vec::new();
        let mut recycled = crate::recycle::Recycled::default();
        let mut burned = crate::pulse::Manifest::new();
        let mut burns: Vec<(Vec<u64>, u32, Ratio, u64)> = Vec::new();
        let mut cross = std::mem::take(&mut pending);
        let mut light = Light::default();
        let mut tolls: Vec<Toll> = Vec::new();
        let mut identity: Vec<(&'static str, Ratio)> = Vec::new();
        if let Some(r) = pending_firm.take() {
            identity.push(("firmament", r));
        }
        // The galaxy's push and throat in its own units, for the wind's
        // meeting with the medium.
        let g_before = if home { whole_u.clone() } else { mass(&core.total()) };
        let pre = if home { home_push.clone() } else { crate::pulse::galaxy_push(&core.total()) };
        let scale_pre = g_before.clone() / unit.clone();
        // THE GALAXY'S ONE FRAME (docs/85 §7; the system tier's, one up): its
        // reach and hold are shares of its holding, the throat 1; a star keeps
        // its spin in this frame, its reach and deficit the one enclosure's,
        // its radius that reach dilated once as a length.
        let own_of = |b: &Body| b.ell.clone();
        // THE ENCLOSED HOLDING, read once from the echo's start: the
        // singularity's own state and every mote and star inside, by spin.
        let enc = pre.as_ref().filter(|_| !g_before.is_zero()).map(|_| {
            let motes = core.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g_before.clone()));
            let stars = core.satellites.iter().map(|b| (own_of(b), mass(&b.total()) / g_before.clone()));
            crate::flows::Enclosed::new(ratio(1, 1), motes.chain(stars).chain(firm_held.iter().cloned()).collect())
        });
        // STARS COLLIDE (operator 2026-10-18; with xylarium): the one
        // collide law among the galaxy's stars, before each reads its echo;
        // each carries the coherence it had before the encounter.
        let planes = crate::flows::PLANES;
        let c_pre: Vec<Ratio> = core.satellites.iter().map(|b| if b.phase.len() < planes || b.bell.len() < planes { ratio(1, 1) } else { (0..planes).fold(ratio(1, 1), |c, k| c * crate::flows::in_step(&b.phase[k], &b.bell[k])) }).collect();
        crate::profile::mark("breath:stars-collide");
        let (met, star_sticks, star_bounces) = match &pre {
            Some(_) if enc.is_some() => crate::formation::collide_stars(std::mem::take(&mut core.satellites), c_pre, &scale_pre, &g_before, enc.as_ref().expect("checked"), &mut cross),
            _ => (std::mem::take(&mut core.satellites).into_iter().zip(c_pre).collect(), 0, 0),
        };
        // WHERE LEAVING MATTER IS SET DOWN (with xylarium; the wind law: the
        // outflow ends at the star's rim): at the edge of the star's own
        // region, R_s(1 + e) with e = H_s/(H_s + H_g), never at its orbit,
        // where the star would recapture it at once.
        // Where leaving matter is set down, as a spin in the galaxy's frame:
        // the star's own, carried out to its region's edge, `1 + e` its reach
        // (the hold about it is the star's own there), `e = H_s/(H_s + H)`, H
        // the hold the enclosure gives inside the star.
        let beyond = |b: &Body| -> Ratio {
            let own = own_of(b);
            match &enc {
                Some(enc) if !g_before.is_zero() => {
                    let h_s = mass(&b.total()) / g_before.clone();
                    let h_in = enc.hold(&own);
                    crate::ring::grain(&(own * (ratio(1, 1) + h_s.clone() / (h_s + h_in))))
                }
                _ => crate::ring::grain(&own),
            }
        };
        // THE SYSTEM'S RIM is its loop's (the double pass, operator
        // 2026-10-26): `formation::settle_system` closes it at `2H`, H the
        // system's ring, so it breathes with the star's own held share (it
        // was the breathing rim `R_s/(h_s + H)` against the galaxy, ruled
        // 2026-10-25, which held a system to about its ring).
        // Worlds of emptied remnants, re-anchored after the stars are read.
        let mut orphans: Vec<(Body, Ratio, Ratio, Vec<Ratio>)> = Vec::new();
        merged += star_sticks;
        encounters += star_bounces;
        crate::profile::mark("breath:systems");
        for (mut sys, c_was) in met {
            // Its clock about the galaxy's centre: the rings it receives at its
            // reach, `1 − δ`, δ by the galaxy's one enclosure at its own spin.
            let own = nested(g_rate.as_ref(), match &enc {
                Some(enc) => ratio(1, 1) - enc.reach(&own_of(&sys)).1,
                None => rate(&sys.radius, &enclosed_in),
            });
            // THE STAR'S BELL (with xylarium): its own spin turns it,
            // `ω_s = δ(ℓ_s/H_s)` a ring; its phases advance by the rings its
            // clock received this echo. ONE FRAME: its spin is kept in its
            // system's units (its heart cut carries it there), so its hold is
            // read there too, its share of its system's holding.
            if sys.phase.len() < planes {
                sys.phase = vec![ratio(0, 1); planes];
            }
            let h_s = { let t = mass(&sys.total()); if t.is_zero() { ratio(1, 1) } else { mass(&sys.kept) / t } };
            // Its spin's sense is a direction, never a rate: a star torqued
            // through to the other sense rings by its magnitude.
            let l_s = crate::pulse::star_spin(&sys, &unit);
            let spin_s = if l_s < ratio(0, 1) { -l_s } else { l_s };
            let omega = h_s.clone() / (spin_s / h_s.clone() + h_s.clone());
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
            rang_n = (rang_n.0 + usize::from(rang), rang_n.1 + 1);
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
            if let (Some((p_g, _, _)), Some(enc)) = (&pre, &enc) {
                if !scale_pre.is_zero() {
                    // At its reach by what is ENCLOSED inside it (the core
                    // and every mote and star inside): the cyclotron.
                    let ell_s = own_of(&sys);
                    // ITS ORTHOGONAL SPIN in its galaxy's own units (docs/90): `ℓ_⊥`
                    // is kept in the units of `ell`; the two readings of its
                    // orbit spin give the conversion.
                    let to_flows = ratio(1, 1);
                    let orth_flows = sys.ell_orth.clone() * to_flows.clone();
                    // At the reach its orthogonal orbit swings it to (docs/90 D2).
                    let (now, through_c) = crate::flows::swung(&ell_s, &orth_flows, sys.phase.get(crate::flows::LOOP).unwrap_or(&ratio(0, 1)));
                    let (r, d_enc) = enc.reach(&now);
                    // Its orbit phase as the echo starts (the pull reads it).
                    let psi_o = sys.phase.first().cloned().unwrap_or_else(|| ratio(0, 1));
                    let h_in = enc.hold(&now);
                    let was = sys.bell.clone();
                    // The star's place: the cap its own gravity holds about it.
                    let place = crate::enfold::place(&r, &(mass(&sys.total()) / g_before.clone()), &h_in);
                    let heat_was = sys.heat.clone();
                    // The bell is the galaxy's phase read at the star: a reading,
                    // so it holds to the star's place and carries no weight.
                    let (scar, _, torque_orth) = crate::flows::turn(&mut sys.bell, &sys.tilt.clone(), &r, &h_in, p_g, tide, false, place.as_ref(), &heat_was, &mut cross);
                    sys.heat += scar;
                    // THE RETURN (with xylarium): its galaxy's torque gives
                    // back `d/(d + turned)` of its displacement in each plane,
                    // `d = 1 − δ_gal` its drag, `turned` its own spin's turns
                    // this echo: a fast spinner is a gyroscope.
                    let d = ratio(1, 1) - d_enc.clone();
                    let turned = omega.clone() * as_ratio(r_echo);
                    let keep_share = if (d.clone() + turned.clone()).is_zero() { ratio(1, 1) } else { turned.clone() / (d + turned) };
                    // ITS ORTHOGONAL SPIN (docs/90 D1, D3): its galaxy's release torques
                    // it, its hold returns the held half, and the same return
                    // that gives back its displacement keeps only
                    // `turned/(d + turned)` of it (the first layer of order: a
                    // fast spinner, an ordered star, holds round). What it
                    // gains or gives back is the core reservoir's.
                    if !to_flows.is_zero() {
                        let ledger_unit = if home { ledger_u.clone() } else { crate::pulse::core_state(core) };
                        let share = if ledger_unit.is_zero() { ratio(0, 1) } else { mass(&sys.total()) / ledger_unit };
                        let torqued = orth_flows.clone() + torque_orth - ratio(1, 2) * d_enc.clone() * orth_flows.clone();
                        let kept_c = torqued * keep_share.clone();
                        // Stored on the grain in the units of `ell`; the
                        // reservoir pays exactly the change stored.
                        let before = core.reservoir.clone() + share.clone() * sys.ell_orth.clone() * to_flows.clone();
                        let stored = crate::ring::grain(&(kept_c / to_flows.clone()));
                        core.reservoir -= share.clone() * (stored.clone() - sys.ell_orth.clone()) * to_flows.clone();
                        sys.ell_orth = stored;
                        identity.push(("ride", core.reservoir.clone() + share * sys.ell_orth.clone() * to_flows.clone() - before));
                    }
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
                    // THE CORE PULLS THE STAR (operator 2026-10-24: "they will
                    // be pulled by the torque from their higher order galactic
                    // core"): its orbit spin takes its galaxy's torque by the one
                    // law, as the galaxy's motes do (the band held back, operator
                    // 2026-10-21): the core's push over its passage by
                    // hemisphere, `p·δ·k`, from the orbit phase it stood at.
                    // Through the centre where it passes it; the core's
                    // reservoir pays exactly what is stored.
                    let from = if through_c { psi_o.clone() + ratio(1, 2) - (psi_o.clone() + ratio(1, 2)).floor() } else { psi_o };
                    let k = crate::flows::mean_response(&from, &(ratio(2, 1) * d_enc.clone()));
                    let raw = sys.ell.clone() + p_g.clone() * d_enc.clone() * k;
                    let ledger_unit = if home { ledger_u.clone() } else { crate::pulse::core_state(core) };
                    if !ledger_unit.is_zero() {
                        let share = mass(&sys.total()) / ledger_unit;
                        let before = core.reservoir.clone() + share.clone() * sys.ell.clone();
                        let pulled = crate::flows::through_centre(crate::ring::grain(&raw), &mut sys.phase);
                        core.reservoir -= share.clone() * (pulled.clone() - sys.ell.clone());
                        sys.ell = pulled;
                        identity.push(("pull", core.reservoir.clone() + share * sys.ell.clone() - before));
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
                    (Some(_), Some(enc)) if !scale_pre.is_zero() => Some(enc.reach(&own_of(&sys)).1),
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
                    let at = beyond(&sys);
                    book_outflow(&mut sys, &mass(&out), &at, &unit, &mut core.medium_spin, &mut identity);
                    infall.push((out, at, sys.ell_orth.clone(), vec![orbit, ratio(1, 2)]));
                }
                if mass(&sys.kept).is_zero() {
                    // Emptied: its medium crosses whole; its worlds re-anchor.
                    tolls.push(Toll { star: sys.path.clone(), bell, passing: Passing::Emptied, strokes: crate::ring::whole(&(omega.clone() * as_ratio(r_now))), returned: mass(&sys.total()) / unit.clone(), spin: subtree_spin(&sys, &unit), star_reach: sys.radius.clone(), galaxy_reach: core.radius.clone() });
                    let all = sys.total();
                    let worlds = std::mem::take(&mut sys.satellites);
                    let rest = all.into_iter().filter(|(_, a)| *a > ratio(0, 1)).collect::<crate::pulse::Manifest>();
                    let mut left = rest;
                    for w in &worlds {
                        less(&mut left, &w.total());
                    }
                    if !left.is_empty() {
                        // Dissolved whole: it leaves at its own orbit.
                        shed.push((left, own_of(&sys), sys.ell_orth.clone(), sys.phase.clone()));
                    }
                    orphans.extend(worlds.into_iter().map(|w| (w, own_of(&sys), sys.ell_orth.clone(), sys.phase.clone())));
                    dissolved += 1;
                    continue;
                }
                // Its system still moves: worlds and debris meet, and what
                // oversteps the rim crosses into the galaxy.
                let (escaped, settled) = crate::formation::settle_system(&mut sys, &unit, r_echo, rang, bell);
                let (st, bo, star_identity, made, weighs, met_by): (usize, usize, Vec<(&'static str, Ratio)>, (usize, usize, usize, usize), [Ratio; 4], (usize, usize, usize)) = (settled.joins, 0, vec![("system", settled.residual.clone())], (settled.formed, settled.ignited, 0, 0), [ratio(0, 1), ratio(0, 1), settled.worlds_mass.clone(), settled.worlds_nuclei.clone()], (0, 0, 0));
                impacts = (impacts.0 + met_by.0, impacts.1 + met_by.1, impacts.2 + met_by.2);
                for (a, b) in weights.iter_mut().zip(weighs) {
                    *a += b;
                }
                fates = (fates.0 + made.0, fates.1 + made.1, fates.2 + made.2, fates.3 + made.3);
                identity.extend(star_identity);
                grown = (grown.0 + st, grown.1 + bo, grown.2 + sys.flows.len(), grown.3 + sys.satellites.len());
                escaped_all += mass(&escaped);
                if !escaped.is_empty() {
                    let at = beyond(&sys);
                    book_outflow(&mut sys, &mass(&escaped), &at, &unit, &mut core.medium_spin, &mut identity);
                    shed.push((escaped, at, sys.ell_orth.clone(), sys.phase.clone()));
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
                // Each kind thrown at the spin the cut carried it with, read
                // in the star's own units (its kept matter holding 1) and
                // carried into its system's (a holding unit dilates reach and
                // hold alike, so spin by the square of the dilation's two
                // readings, `λ·λ`, λ the star's share of its system).
                // Two readings, each moved once (no square): its reach in the
                // heart's units, `ℓ_z/H*`, as a length by λ (the star's share of
                // its system), and the system's hold at that reach.
                let lambda = if mass(&sys.total()).is_zero() { ratio(0, 1) } else { mass(&sys.kept) / mass(&sys.total()) };
                let held_k = crate::formation::heart_cut(&sys.kept, &ratio(0, 1), &heat).map(|(_, _, h)| h).unwrap_or_else(|| ratio(0, 1));
                let frame = crate::formation::system_frame(&sys, &unit);
                let thrown: Vec<(crate::chem::Kind, Ratio)> = if held_k.is_zero() { Vec::new() } else { crate::formation::carried(&sys.kept, &ratio(0, 1), &heat).into_iter().map(|(k, l)| { let r = l / held_k.clone() * lambda.clone(); let h = frame.as_ref().map(|e| e.hold_within(&r)).unwrap_or_else(|| lambda.clone()); (k, r * h) }).collect() };
                less(&mut sys.kept, &burst);
                // The ejecta leave together, as a spray over directions.
                for (k, a) in &burst {
                    *sys.medium.entry(k.clone()).or_insert_with(|| ratio(0, 1)) += a.clone();
                }
                let push = crate::pulse::push_of(&sys.kept);
                // The spray leaves from the star itself.
                sys.flows.extend(crate::flows::spray(&burst, &thrown, &vec![ratio(0, 1); planes], &ratio(0, 1), bell, &push));
            }
            let mut back = crate::pulse::Manifest::new();
            // THE WIND (operator: made at the INTERFACE): the star's
            // outbound continuum meets the medium's counter-moving vacuum
            // stream. The counterflow is the vacuum-stream motes within the
            // star's dispersal band, each by its holding and the PRODUCT OVER
            // PLANES of its in-step share with the star (flows meet where
            // they coincide, pass where they don't); each open kind leaves by
            // its open share times that counterflow. No counterflow, no wind.
            if let (Some((p_g, _, _)), Some(enc)) = (&pre, &enc) {
                if !scale_pre.is_zero() && !g_before.is_zero() {
                    let r = enc.reach(&own_of(&sys)).0;
                    let (lo, hi) = (r.clone() * (ratio(1, 1) - p_g.clone()), r.clone() * (ratio(1, 1) + p_g.clone()));
                    let chi = core.flows.iter().filter(|x| x.vacuum).fold(ratio(0, 1), |acc, x| {
                        let rx = enc.reach(&x.ell_o).0;
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
            // SPIN-DOWN: what leaves carries its share of the star's spin (its
            // specific spin unchanged), the share floored at the flow so the
            // extensive spin stays a bounded sum.
            if !kept_before.is_zero() {
                sys.turning = sys.turning.clone() - crate::ring::grain(&(sys.turning.clone() * mass(&back) / kept_before));
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
            // ITS PULSE PHASE (docs/97 §13): on the R loop, against the echo
            // reaching its place (its galaxy's bell there), `p = φ − bell`;
            // and ITS CLOCK read from the rings it received this echo,
            // `ν = 2·r_echo/T_u` (each ring read on both legs).
            let at_loop = |v: &Vec<Ratio>| v.get(crate::flows::LOOP).cloned().unwrap_or_else(|| ratio(0, 1));
            let pulse_p = at_loop(&sys.phase) - at_loop(&sys.bell);
            let nu = ratio(2, 1) * as_ratio(r_echo) / as_ratio(t_u.max(1));
            let sys_total = mass(&sys.total());
            let heart_share = if sys_total.is_zero() { ratio(0, 1) } else { mass(&sys.kept) / sys_total };
            // The front needs the slip HELD across a full bell of the star's
            // own (with xylarium): out of step at both ends of a ring it
            // completed, never an instantaneous dip.
            if rang && c_before < heart_share && c_sg < heart_share {
                tolls.push(Toll { star: sys.path.clone(), bell, passing: Passing::Decohered, strokes: crate::ring::whole(&(omega.clone() * as_ratio(r_now))), returned: mass(&sys.total()) / unit.clone(), spin: subtree_spin(&sys, &unit), star_reach: sys.radius.clone(), galaxy_reach: core.radius.clone() });
                core.medium_spin += subtree_spin(&sys, &unit);
                let mut all = sys.total();
                // A DYING HEART'S RELEASE (docs/97 §11, §13): its heat on its
                // own clock, `ν·h_s − 1` (ν the rings it received this echo a
                // bell, on both legs), breaks what it out-grips at its pulse
                // phase into nucleons; THE RAPID CAPTURE (§10) then runs on
                // them at that phase.
                let heart_heat = crate::nuclides::heat(&h_s, &nu);
                crate::nuclides::release(&mut all, &heart_heat, &pulse_p);
                crate::nuclides::capture_rapid(&mut all, &pulse_p, densest);
                returned += mass(&all) / unit.clone();
                // Dissolved whole: it leaves at its own orbit.
                shed.push((all, own_of(&sys), sys.ell_orth.clone(), sys.phase.clone()));
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
            let at = beyond(&sys);
            book_outflow(&mut sys, &mass(&back), &at, &unit, &mut core.medium_spin, &mut identity);
            shed.push((back, at, sys.ell_orth.clone(), sys.phase.clone()));
            if mass(&sys.kept).is_zero() {
                // Spent: all it held returns, with all its spin (its orbit's
                // and everything turning inside it).
                tolls.push(Toll { star: sys.path.clone(), bell, passing: Passing::Spent, strokes: crate::ring::whole(&(omega.clone() * as_ratio(r_now))), returned: mass(&sys.total()) / unit.clone(), spin: subtree_spin(&sys, &unit), star_reach: sys.radius.clone(), galaxy_reach: core.radius.clone() });
                core.medium_spin += subtree_spin(&sys, &unit);
                let mut all = sys.total();
                // A DYING HEART'S RELEASE (docs/97 §11, §13): its heat on its
                // own clock, `ν·h_s − 1` (ν the rings it received this echo a
                // bell, on both legs), breaks what it out-grips at its pulse
                // phase into nucleons; THE RAPID CAPTURE (§10) then runs on
                // them at that phase.
                let heart_heat = crate::nuclides::heat(&h_s, &nu);
                crate::nuclides::release(&mut all, &heart_heat, &pulse_p);
                crate::nuclides::capture_rapid(&mut all, &pulse_p, densest);
                returned += mass(&all) / unit.clone();
                // Dissolved whole: it leaves at its own orbit.
                shed.push((all, own_of(&sys), sys.ell_orth.clone(), sys.phase.clone()));
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
            let rungs: Vec<u32> = sys.kept.keys().filter_map(crate::chem::atom_w).collect();
            // The light this star's rungs make this echo: each product pair
            // transduces `2u(z) − u(2z)` of magnetism into holding.
            let mut shone = ratio(0, 1);
            for z in rungs {
                // A rung whose product passes the universe's densest element
                // does not run (docs/97 §14).
                if z.checked_mul(2).map_or(true, |p| p > densest) {
                    continue;
                }
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
                        if let Some(l) = crate::recycle::rung_light(z) {
                            shone += made.clone() * ratio(l, 1);
                        }
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
            // THE SLOW CAPTURE (docs/97 §10): each ring its clock received this
            // echo, its heart's nuclides capture its hydrogen, one nucleon a
            // ring, kept by the hemisphere law at its pulse phase.
            crate::nuclides::capture_slow(&mut sys.kept, r_echo, &in_phase, &pulse_p, densest);
            // ITS LIGHT crosses its own system on the continuum face.
            if !shone.is_zero() {
                let whole = mass(&sys.total());
                let (got, gone) = match crate::formation::system_frame(&sys, &unit) {
                    Some(enc_s) => shine(&mut sys.flows, &mut sys.satellites, &enc_s, &shone, &whole),
                    None => (ratio(0, 1), shone.clone()),
                };
                light.made += shone / unit.clone();
                light.received += got / unit.clone();
                light.drained += gone / unit.clone();
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
                *a >= pair && crate::chem::atom_w(k).is_some_and(|z| crate::formation::burn(&sys.kept, &share_now, z, &ratio(1, 1)).and_then(|(feed, _)| crate::compound::read(a, &crate::compound::after(&feed, rings_echo)?, |left| a.clone() - left.clone() >= pair)).unwrap_or(false))
            });
            if !can_burn && !sys.kept.is_empty() {
                sys.remnant = true;
                sys.state = mass(&sys.kept);
            }
            // COLLISIONS in the system's own medium: worlds grow; what
            // overreaches the system's rim crosses into the galaxy.
            let (escaped, settled) = crate::formation::settle_system(&mut sys, &unit, r_echo, rang, bell);
            let (st, bo, star_identity, made, weighs, met_by): (usize, usize, Vec<(&'static str, Ratio)>, (usize, usize, usize, usize), [Ratio; 4], (usize, usize, usize)) = (settled.joins, 0, vec![("system", settled.residual.clone())], (settled.formed, settled.ignited, 0, 0), [ratio(0, 1), ratio(0, 1), settled.worlds_mass.clone(), settled.worlds_nuclei.clone()], (0, 0, 0));
                impacts = (impacts.0 + met_by.0, impacts.1 + met_by.1, impacts.2 + met_by.2);
                for (a, b) in weights.iter_mut().zip(weighs) {
                    *a += b;
                }
                fates = (fates.0 + made.0, fates.1 + made.1, fates.2 + made.2, fates.3 + made.3);
                identity.extend(star_identity);
            grown = (grown.0 + st, grown.1 + bo, grown.2 + sys.flows.len(), grown.3 + sys.satellites.len());
            escaped_all += mass(&escaped);
            if !escaped.is_empty() {
                let at = beyond(&sys);
                book_outflow(&mut sys, &mass(&escaped), &at, &unit, &mut core.medium_spin, &mut identity);
                shed.push((escaped, at, sys.ell_orth.clone(), sys.phase.clone()));
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
        for (mut w, at, orth, was) in orphans {
            // The nearest standing star in the galaxy's frame, by spin.
            let host = keep.iter_mut().filter(|s| !s.remnant).min_by_key(|s| { let o = own_of(s); if o > at { o - at.clone() } else { at.clone() - o } });
            match host {
                Some(h) => {
                    for k in 0..w.phase.len().min(was.len()).min(h.phase.len()) {
                        w.phase[k] = frac_of(w.phase[k].clone() + was[k].clone() - h.phase[k].clone());
                    }
                    h.satellites.push(w)
                }
                None => {
                    let place: Vec<Ratio> = (0..planes).map(|k| frac_of(w.phase.get(k).cloned().unwrap_or_else(|| ratio(0, 1)) + was.get(k).cloned().unwrap_or_else(|| ratio(0, 1)))).collect();
                    shed.push((w.total(), at, orth, place))
                }
            }
        }
        // CAPTURE IS SETTLING (operator 2026-10-24, relayed by xylarium: "the
        // star is caught in its own gravitational motion with the galactic
        // core which settles amongst all larger body gravity wells - and earth
        // settles in the stars in the same formula derived"; the inbound half
        // of a star's circulation, the wind its outbound). A galaxy mote whose
        // place stands inside a star's sphere, where the star's pull meets the
        // core's, `h_s/(d_s + h_s) = H/(d + H)`, so `d_s/d = k = h_s/H` (the
        // Apollonian sphere of the two holds; its inner crossing is the
        // pinch, `R_s·k/(1 + k)`), SETTLES into the star's system: no passage
        // count, a mote moving with the star settles like any other; one
        // several stars hold divides among them, whole nuclei to each alike.
        // THE GALAXY TIER'S BOOK: the system's orbit takes the settled turning
        // by holding in each plane, `ℓ′ = (W·ℓ + m·ℓ_m)/(W + m)` (W the
        // system's whole holding, as its mergers and the folds join), the
        // grain's remainder to the galaxy's medium spin, so the books close.
        // THE SYSTEM TIER: it is set down at its place, radially `gap/(k·r_m)`
        // (the sphere's edge the system's rim, 1), turning at that place's
        // balance, `ℓ = r·H(<r)`; its circulation is what it carried against
        // the star's, as a share of its orbit spin (an offset is a phase,
        // never a second orbit). Remnants do not capture.
        if let (Some(_), Some(enc), false) = (&pre, &enc, scale_pre.is_zero() || g_before.is_zero()) {
            let reach_m = |x: &crate::flows::Mote| enc.reach(&x.ell_o).0;
            let mut claims: Vec<Vec<usize>> = vec![Vec::new(); core.flows.len()];
            let mut by_reach: Vec<(Ratio, usize)> = core.flows.iter().enumerate().map(|(mi, x)| (reach_m(x), mi)).collect();
            by_reach.sort();
            let farthest = by_reach.last().map(|(r, _)| r.clone()).unwrap_or_else(|| ratio(0, 1));
            // Each standing star's sphere: (its reach, its k).
            let spheres: Vec<Option<(Ratio, Ratio)>> = keep.iter().map(|s| {
                if s.remnant || s.phase.is_empty() {
                    return None;
                }
                let r_s = enc.reach(&own_of(s)).0;
                let h_in = enc.hold(&own_of(s));
                if h_in.is_zero() || r_s.is_zero() {
                    return None;
                }
                Some((r_s, mass(&s.total()) / g_before.clone() / h_in))
            }).collect();
            for (si, sp) in spheres.iter().enumerate() {
                let Some((r_s, k)) = sp else { continue };
                // The sphere spans `R_s/(1 + k) .. R_s/(1 − k)` about the core;
                // a star that outholds its place (k ≥ 1) reaches every mote
                // beyond its inner crossing.
                let d = if *k < ratio(1, 1) { r_s.clone() * k.clone() / (ratio(1, 1) - k.clone()) } else { farthest.clone() + r_s.clone() };
                for &(ref r_m, mi) in crate::flows::within_reach(&by_reach, r_s, &d, &mut crate::flows::Work::default()) {
                    let gap = if *r_m > *r_s { r_m.clone() - r_s.clone() } else { r_s.clone() - r_m.clone() };
                    if gap < k.clone() * r_m.clone() {
                        claims[mi].push(si);
                    }
                }
            }
            for (mi, cl) in claims.into_iter().enumerate() {
                if cl.is_empty() {
                    continue;
                }
                let n = ratio(cl.len() as i64, 1);
                let r_m = reach_m(&core.flows[mi]);
                let whole_amount = core.flows[mi].amount.clone();
                for si in cl {
                    let take = if n == ratio(1, 1) { whole_amount.clone() } else { crate::pulse::nucleus(&(whole_amount.clone() / n.clone())) }.min(core.flows[mi].amount.clone());
                    if take <= ratio(0, 1) {
                        continue;
                    }
                    let Some((r_s, _)) = spheres[si].clone() else { continue };
                    core.flows[mi].amount -= take.clone();
                    let x = core.flows[mi].clone();
                    let s = &mut keep[si];
                    let key = x.key.clone();
                    let m = take.clone() * crate::flows::weight(&key);
                    // Its circulation against the star's, read before the book
                    // moves the star's.
                    let carried_c = if x.ell_o.is_zero() { ratio(0, 1) } else { (x.ell_orth.clone() - s.ell_orth.clone()) / x.ell_o.clone() };
                    // The galaxy tier's book, both planes.
                    let w_sys = mass(&s.total());
                    if !(w_sys.clone() + m.clone()).is_zero() {
                        let exact_o = (w_sys.clone() * s.ell.clone() + m.clone() * x.ell_o.clone()) / (w_sys.clone() + m.clone());
                        let exact_c = (w_sys.clone() * s.ell_orth.clone() + m.clone() * x.ell_orth.clone()) / (w_sys.clone() + m.clone());
                        let before = w_sys.clone() * (s.ell.clone() + s.ell_orth.clone()) + m.clone() * (x.ell_o.clone() + x.ell_orth.clone());
                        let (go, gc) = (crate::ring::grain(&exact_o), crate::ring::grain(&exact_c));
                        let rest = before.clone() - (w_sys.clone() + m.clone()) * (go.clone() + gc.clone());
                        core.medium_spin += rest.clone() / unit.clone();
                        identity.push(("settle", (w_sys.clone() + m.clone()) * (go.clone() + gc.clone()) + rest - before));
                        s.tilt = crate::flows::blend(&[(w_sys.clone(), s.ell.clone(), s.tilt.clone()), (m.clone(), x.ell_o.clone(), x.tilt.clone())]);
                        s.ell = go;
                        s.ell_orth = gc;
                    }
                    // The system tier: set down at its place.
                    let gap = if r_m > r_s { r_m.clone() - r_s.clone() } else { r_s.clone() - r_m.clone() };
                    // A galaxy length enters the system divided once by its
                    // holding share, against its breathing rim.
                    let lam = mass(&s.total()) / g_before.clone();
                    let r_sys = if lam.is_zero() { ratio(0, 1) } else { gap / lam };
                    let tot = mass(&s.total());
                    let h_sys = if tot.is_zero() { ratio(1, 1) } else { mass(&s.kept) / tot };
                    let h_at = crate::formation::system_frame(s, &unit).map(|enc_s| enc_s.hold_within(&r_sys)).unwrap_or(h_sys);
                    let ell_o = crate::ring::grain(&(r_sys * h_at));
                    let phase: Vec<Ratio> = x.phase.iter().zip(&s.phase).map(|(a, b)| { let f = a.clone() - b.clone(); crate::ring::grain(&(f.clone() - f.floor())) }).collect();
                    let mote = crate::flows::Mote { key: key.clone(), amount: take.clone(), ell_orth: crate::ring::grain(&(carried_c * ell_o.clone())), ell_o, phase, agitation: x.agitation.clone(), born: bell, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1), tilt: x.tilt.clone() };
                    *s.medium.entry(key).or_insert_with(|| ratio(0, 1)) += take;
                    s.flows.push(mote);
                }
            }
            core.flows.retain(|x| x.amount > ratio(0, 1));
        }
        core.satellites = keep;
        crate::profile::mark("breath:flows-arrivals");
        // THE FLOWS (`crate::flows`; docs/83, xylarium 11.0; STAGED on two
        // planes, the orbit's and the circulation's): the galaxy's matter is
        // motes carrying their state; the core's bell torques every mote
        // by its hemisphere against the bell; the exits, the cut and the
        // bindings are READ from where the motes' states have carried them.
        let mut jets = 0;
        let mut stars = 0;
        let mut to_ocean: Vec<crate::flows::Mote> = Vec::new();
        let mut with = core.total();
        for (x, _, _) in &gathered {
            *with.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
        }
        let g_mass = if home { whole_u.clone() } else { mass(&with) };
        let push_with = if home { home_push.clone() } else { crate::pulse::galaxy_push(&with) };
        if let Some((p, h, mean)) = push_with {
            let scale = g_mass.clone() / unit.clone();
            let zero_phase = vec![ratio(0, 1); crate::flows::PLANES];
            // The galaxy's one enclosure as it stands: its motes and its stars.
            // Its base the core's state, 1 (docs/96 §7: the throat's grains,
            // never counted twice).
            let galaxy_enc = |core: &Body| crate::flows::Enclosed::new(ratio(1, 1), core.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g_mass.clone())).chain(core.satellites.iter().map(|b| (b.ell.clone(), mass(&b.total()) / g_mass.clone()))).collect());
            crate::profile::mark("breath:release");
            // THE BOUNCE RELEASES (docs/96 step 3; operator 2026-10-26: "each
            // bounce from the core doesnt 'throw' - it pushes, that push is the
            // incremental 'flow' of tidal forces between the vacuum and the
            // continuum which release the elements into the void"). Of each
            // kind the throat still holds, carried by the push to `c = p·w̄/w`
            // against the core's state (docs/96 §7), the tide on the unlocked
            // face releases `s = ½(1 − δ) = ½·c/(c + base)` this bounce, in whole
            // nuclei. It leaves outbound on the continuum (R forward) from the
            // ring to its dispersal reach `1 + span·c`, turning at the balance
            // of that place, at the throat's own phase for it this bell: its
            // kind's turning there over the isotropic planes, `b·2δ/3`, plus
            // its captures' half turns `Z/2`, and on each spatial plane the
            // core's own WINDING there at this bell (docs/100 §2: the twist,
            // ½ a turn a ring by the plane's tilt share, over the rings the
            // core has received; derived, never drawn). It leaves with the
            // core's twist as its LOCKED LEAD between its planes (its tilt,
            // docs/100 §4: the jet a helix), loosened later by its own
            // turning (`flows::settle_disc`). The core's reservoir pays the
            // spin it leaves with (the ledger exact).
            {
                let span_r = if home && !whole_u.is_zero() { mass(&core.total()) / whole_u.clone() } else { ratio(1, 1) };
                let rings_now = g_rate.clone().unwrap_or_else(|| ratio(1, 1)) * ratio(bell as i64, 1);
                let twist_t = crate::flows::tilt_of(&core.tilt);
                let lead = core.tilt.clone();
                let enc_r = galaxy_enc(core);
                let ledger_r = if home { ledger_u.clone() } else { crate::pulse::core_state(core) };
                let frac = |x: Ratio| x.clone() - x.floor();
                let held_now: Vec<(crate::chem::Kind, Ratio)> = core.throat.iter().map(|(k, a)| (k.clone(), a.clone())).collect();
                for (k, a) in held_now {
                    let w = crate::flows::weight(&k);
                    let c = p.clone() * mean.clone() / w.clone();
                    // Read against the core's state (docs/96 §7).
                    let base = ratio(1, 1);
                    if (c.clone() + base.clone()).is_zero() {
                        continue;
                    }
                    let s = ratio(1, 2) * c.clone() / (c.clone() + base.clone());
                    let take = crate::pulse::nucleus(&(a.clone() * s)).min(a.clone());
                    if take <= ratio(0, 1) {
                        continue;
                    }
                    let r = ratio(1, 1) + span_r.clone() * c.clone();
                    let ell = crate::ring::grain(&(r.clone() * enc_r.hold_within(&r)));
                    let d_k = base.clone() / (c.clone() + base.clone());
                    let z = ratio(i64::from(crate::chem::charge(&k)), 2);
                    let base = frac(ratio(bell as i64, 1) * ratio(2, 3) * d_k + z.clone());
                    let mut phase = crate::flows::rest(crate::ring::grain(&frac(ratio(1, 2) + z)));
                    for (j, &sp) in crate::flows::SPACE.iter().enumerate() {
                        phase[sp] = crate::ring::grain(&frac(base.clone() + ratio(1, 2) * twist_t[j].clone() * rings_now.clone()));
                    }
                    if let Some(x) = core.throat.get_mut(&k) {
                        *x -= take.clone();
                    }
                    if !ledger_r.is_zero() {
                        core.reservoir -= take.clone() * w.clone() / ledger_r.clone() * ell.clone();
                    }
                    released.0 += 1;
                    released.1 += take.clone() * w / unit.clone();
                    core.flows.push(crate::flows::Mote { key: k, amount: take, ell_o: ell, ell_orth: ratio(0, 1), phase, agitation: ratio(0, 1), born: bell, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1), tilt: lead.clone() });
                }
                core.throat.retain(|_, a| *a > ratio(0, 1));
                released.2 = mass(&core.throat) / unit.clone();
            }
            // Arrivals: the push carries them as every kind, and they keep
            // the crossing's turning in each plane (carried spins add), in
            // step with the bell where the crest gathered them.
            // THE MATTER STANDING IN THE THROAT (with xylarium): the vacuum
            // stream's inbound leg (the arrivals) and the remnants' infall.
            // (The arrivals ride the vacuum's loop; each passes the throat
            // when its orthogonal phase crosses ½ in the torques, below.)
            let mut in_throat: Vec<usize> = Vec::new();
            for (x, t_o, t_c) in gathered {
                let (k, a, agitation) = (x.key, x.amount, x.agitation);
                // It lands FROM THE RING (docs/94 §2), within its disc's span
                // (its matter's share of its hold: 1 for a galaxy, the home's
                // share of the universe for the home), at the balance of its
                // place in the galaxy's one enclosure; its own crossing
                // turning carried.
                let span = if home && !whole_u.is_zero() { mass(&core.total()) / whole_u.clone() } else { ratio(1, 1) };
                let r_land = ratio(1, 1) + span * p.clone() * mean.clone() / crate::flows::weight(&k);
                let h_land = enc.as_ref().map(|e| e.hold_within(&r_land)).unwrap_or_else(|| ratio(1, 1));
                let ell_o = r_land * h_land + nested(g_rate.as_ref(), t_o);
                let ell_orth = nested(g_rate.as_ref(), t_c);
                // It arrives with the Wellspring's winding: its orthogonal orbit leads its orbit by
                // the push share (½ at the universe tier), the jet's helix.
                motes_in[0] += 1;
                core.flows.push(crate::flows::Mote { key: k, amount: a, ell_o: crate::ring::grain(&ell_o), ell_orth: crate::ring::grain(&ell_orth), phase: crate::flows::rest(ratio(1, 2)), agitation, born: bell, vacuum: true, lever: Vec::new(), lever_place: ratio(0, 1), tilt: Vec::new() });
            }
            // Shed matter crosses whole at the spins it left with, in the
            // galaxy's frame: its star's orbit, carried to its region's edge,
            // and its star's orthogonal orbit (carried spins add; never
            // dropped to zero, or `Σ m·ℓ_orth` would leak).
            let falling = infall.len();
            for (n, (m, at, orth, star_phase)) in infall.into_iter().chain(shed).enumerate() {
                for (k, a) in m {
                    if n < falling {
                        in_throat.push(core.flows.len());
                    }
                    // It keeps its star's phases (state, inherited), on the
                    // star's continuum stream.
                    let phase = if star_phase.len() == crate::flows::PLANES { star_phase.clone() } else { zero_phase.clone() };
                    motes_in[1] += 1;
                    core.flows.push(crate::flows::Mote { key: k, amount: a, ell_o: crate::ring::grain(&at), ell_orth: orth.clone(), phase, agitation: ratio(0, 1), born: bell, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1), tilt: Vec::new() });
                }
            }
            crate::profile::mark("breath:torque");
            // THE TORQUE, its spin from the core's reservoir (the antipole).
            // The reservoir is kept in the galaxy's FOUNDED holding (its
            // core's state, never drained), one fixed unit, so its exact sum
            // never compounds denominators echo on echo.
            let ledger_unit = if home { ledger_u.clone() } else { crate::pulse::core_state(core) };
            let throat = crate::flows::Enclosed::new(h.clone(), Vec::new());
            // A VACUUM MOTE PASSES THE THROAT (with xylarium) when its orthogonal orbit
            // phase crosses ½ during the torques (both streams pass the core
            // at orthogonal phase ½): the inbound leg (the axis, for the vacuum)
            // delivers it to the throat.
            let orth_was: Vec<(usize, Ratio)> = core.flows.iter().enumerate().filter(|(_, x)| x.vacuum && x.phase.len() > crate::flows::LOOP).map(|(j, x)| (j, x.phase[crate::flows::LOOP].clone())).collect();
            // The galaxy's band (push and return, `flows::torque`) stands held
            // back (operator 2026-10-21: "pull it back and make it static").
            let ledger_sum = |core: &Body| core.flows.iter().fold(core.reservoir.clone(), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key) / ledger_unit.clone() * (x.ell_o.clone() + x.ell_orth.clone()));
            let before = ledger_sum(core);
            let (given, absorbed) = crate::flows::torque(&mut core.flows, enc.as_ref().unwrap_or(&throat), &p, tide, &ledger_unit, None, &mut cross);
            // THE DISC FORMS: each mote's tilt returns toward its galaxy's spin
            // plane (`flows::settle_disc`).
            core.tilt = crate::flows::settle_disc(&mut core.flows, enc.as_ref().unwrap_or(&throat));
            core.reservoir -= given;
            identity.push(("torque", ledger_sum(core) - before));
            crate::profile::mark("breath:recycle");
            // RECYCLING (`crate::recycle`; xylarium 11.0 (18)): AFTER the
            // torques (torques are motion, recycling the residue), the bounce the
            // container holds presses and peels the matter in the throat.
            let frac = |x: Ratio| x.clone() - x.floor();
            // Its route runs the loop backward (drawn in along the axis, out
            // through the disc): it reaches the throat crossing ½ from above.
            for (j, was) in orth_was {
                let to_half = frac(was.clone() - ratio(1, 2));
                if to_half > ratio(0, 1) && to_half <= frac(was - core.flows[j].phase[crate::flows::LOOP].clone()) {
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
            let bounce = p.clone() - absorbed;
            // What stood in the throat as it entered, to tell how each leaves.
            let entered: Vec<(usize, crate::chem::Kind, Ratio)> = in_throat.iter().filter(|&&j| j < core.flows.len()).map(|&j| (j, core.flows[j].key.clone(), core.flows[j].amount.clone())).collect();
            let before = core.flows.len();
            let r = crate::recycle::recycle(&mut core.flows, &in_throat, &bounce, &container, &(g_mass.clone() - (h.clone() - ratio(1, 1)) * g_mass.clone()), bell);
            motes_in[2] += core.flows.len().saturating_sub(before);
            // Whatever passed the throat, split or not, leaves on the
            // continuum (operator 2026-10-19: "redistributed back via the
            // continuum"); the vacuum refills only from the ocean. Each exit
            // closes its puncture: SEALED where it leaves as it entered (its
            // state read back), CAUTERIZED where the bounce pressed or peeled
            // it (its state cannot be read back), the change its scar. What
            // the throat made (the pressed and peeled products) was born there
            // and leaves on the continuum, cauterized.
            let holding = |k: &crate::chem::Kind, a: &Ratio| a.clone() * crate::flows::weight(k);
            for (j, key, amount) in entered {
                core.flows[j].vacuum = false;
                if core.flows[j].key == key && core.flows[j].amount == amount {
                    cross.note(0, crate::enfold::Fate::Sealed, &ratio(0, 1));
                } else {
                    let scar = holding(&key, &amount) - holding(&core.flows[j].key, &core.flows[j].amount);
                    cross.note(0, crate::enfold::Fate::Cauterized, &scar);
                }
            }
            for x in core.flows.iter_mut().skip(before) {
                x.vacuum = false;
                let scar = -holding(&x.key, &x.amount);
                cross.note(0, crate::enfold::Fate::Cauterized, &scar);
            }
            // The throat's light crosses the galaxy on the continuum face.
            if !r.light.is_zero() {
                // By the galaxy's one enclosure as it stands.
                let enc_l = galaxy_enc(core);
                let (got, _) = shine(&mut core.flows, &mut [], &enc_l, &r.light, &g_mass);
                // And on every star system standing in the galaxy, crossing
                // into each to its worlds and motes (`shine_systems`).
                let (inside, _) = shine_systems(&mut core.satellites, &r.light, &enc_l, &g_mass, &unit);
                light.made += r.light.clone() / unit.clone();
                light.received += (got.clone() + inside.clone()) / unit.clone();
                light.drained += (r.light.clone() - got - inside) / unit.clone();
            }
            // The trapped join the container at the throat, their carried
            // spin with them: the core banks it in its reservoir (the
            // antipole), so `Σ m·ℓ` and the reservoir stay one ledger.
            core.reservoir += r.spun.clone() / ledger_unit.clone();
            recycled = crate::recycle::Recycled { pressed: r.pressed, light: r.light / unit.clone(), peeled: r.peeled, spent: r.spent / unit.clone(), trapped: r.trapped / unit.clone(), spun: r.spun / ledger_unit.clone(), hydrogen: r.hydrogen };
            // THE SHEAR: neighbouring rings pass turning outward; the inner
            // falls in (the loop becomes an orbit). Σ m·ℓ unchanged.
            crate::profile::mark("breath:shear");
            circulation.0 = crate::flows::shear(&mut core.flows, enc.as_ref().unwrap_or(&throat), &ledger_unit);
            // THE FLUID (docs/94): pressure and the field move turning
            // outward between the galaxy's rings; and THE LUMINAL BOUND: a
            // continuum mote inside the ring takes the Möbius jetstream onto
            // the vacuum leg, never orbiting faster than a turn a bell.
            let enc_f = galaxy_enc(core);
            crate::profile::mark("breath:fluid");
            let (by_pressure, by_field) = crate::flows::fluid(&mut core.flows, &enc_f, &p, &ledger_unit);
            circulation.1 = by_pressure;
            circulation.2 = by_field;
            crate::profile::mark("breath:luminal");
            circulation.3 = crate::flows::luminal(&mut core.flows, &enc_f);
            // THE EXITS: the jet (along the axis) and radial escape cross
            // whole into the ocean (their state's onward carriage owed: the
            // ocean is not yet motes).
            crate::profile::mark("breath:exits");
            let enc_x = galaxy_enc(core);
            for x in crate::flows::exits(&mut core.flows, &enc_x) {
                // It leaves WHOLE, keeping its state, co-moving with its
                // galaxy about the universal core (its galaxy's orbit spins
                // and phases), on the vacuum leg home.
                let phase = if core.phase.len() == crate::flows::PLANES { core.phase.clone() } else { x.phase.clone() };
                // Its own spin carried across by its galaxy's share, once
                // (with xylarium): `ℓ_u = ℓ_g + ℓ_m·f` in each plane.
                let f_g = if whole_u.is_zero() { ratio(0, 1) } else { mass(&core.total()) / whole_u.clone() };
                let (ell_u, orth_u) = (crate::ring::grain(&(core.ell.clone() + x.ell_o.clone() * f_g.clone())), crate::ring::grain(&(core.ell_orth.clone() + x.ell_orth.clone() * f_g)));
                to_ocean.push(crate::flows::Mote { ell_o: ell_u, ell_orth: orth_u, phase, vacuum: true, born: bell, lever: Vec::new(), lever_place: ratio(0, 1), ..x });
                jets += 1;
            }
            let (kept, free) = crate::flows::read_cut(&core.flows, &galaxy_enc(core), &p);
            core.kept = kept;
            core.medium = free;
            // THE BINDINGS, where the motes are; the echo compresses only
            // its coherent part (recoherence).
            // The galaxy's one enclosure as its motes now stand.
            let enc_now = galaxy_enc(core);
            crate::profile::mark("breath:chains");
            let chains = crate::flows::chains(&core.flows, &enc_now, &p, &g_mass);
            // The stream of every mote, read by force against its filament.
            // The pulse at a mote: a galaxy's received tide, or, in the home,
            // the source's own at the mote's reach.
            let push_at = |x: &crate::flows::Mote| match tide {
                Some((rho, du)) => p.clone() * rho.clone() * du.clone(),
                None => p.clone() * enc_now.reach(&x.ell_o).1,
            };
            circulation.4 = crate::flows::stream(&mut core.flows, &chains, &push_at);
            crate::profile::mark("breath:bindings");
            // THE GALAXY SETTLES ITS STARS (docs/96; operator 2026-10-26): its
            // band pairs on the strip, never the stepped filaments' bindings.
            let _ = chains;
            let bindings = crate::flows::band_pairs(&core.flows, &enc_now, &p, &core.tilt, &g_mass);
            // Each take: (reach, what fell, the disc outside it, the whole
            // binding's holding, agitation, the star's own spin, phases, its
            // orthogonal spin, the members as they were).
            #[allow(clippy::type_complexity)]
            let mut takes: Vec<(Ratio, crate::pulse::Manifest, Vec<(crate::flows::Mote, Ratio, Ratio)>, Ratio, Ratio, Ratio, Vec<Ratio>, Ratio, Vec<crate::flows::Mote>, Vec<(Ratio, Ratio, Ratio)>, Ratio)> = Vec::new();
            for (reach, shares) in bindings {
                // Each member as it stood, its share of its mote.
                let mut members: Vec<crate::flows::Mote> = Vec::new();
                for (j, s) in shares {
                    let x = &mut core.flows[j];
                    let amount = crate::pulse::nucleus(&(x.amount.clone() * s)).min(x.amount.clone());
                    if amount <= ratio(0, 1) {
                        continue;
                    }
                    x.amount -= amount.clone();
                    members.push(crate::flows::Mote { amount, lever: Vec::new(), lever_place: ratio(0, 1), ..x.clone() });
                }
                let held_of = |x: &crate::flows::Mote| x.amount.clone() * crate::flows::weight(&x.key);
                let m = members.iter().fold(ratio(0, 1), |s, x| s + held_of(x));
                if m.is_zero() {
                    continue;
                }
                // THE INFALL IN PHASES (docs/99 §2.2; operator 2026-10-27: "we
                // shouldnt have frames we should have phases"): the binding's
                // phase is its members' holding-weighted circle mean in each
                // plane; a member's STEP with it is its in-step share across
                // the spatial planes by its own orbit's shares (its tilt, L1:
                // a plane its orbit does not turn in carries no weight),
                // `C = Σ t_k·in_step(ψ_k, ψ̄_k)`. It FALLS on the held
                // hemisphere, `C ≥ ½`; on the open hemisphere it is the disc.
                // Read once, each against the binding's phase: no length, no
                // frame, no extent that shrinks with what fell. Its PLACE in
                // the binding is its phase off step, `1 − C`: the heart's
                // half in `[0, ½]`, the disc's in `(½, 1]` out to the
                // system's rim, the whole turn.
                let psi_c: Vec<Ratio> = (0..crate::flows::PLANES).map(|k| crate::flows::circle_mean(&members.iter().map(|x| (held_of(x), x.phase.get(k).cloned().unwrap_or_else(|| ratio(0, 1)))).collect::<Vec<_>>())).collect();
                let places: Vec<(Ratio, Ratio)> = members.iter().map(|x| (held_of(x), ratio(1, 1) - crate::flows::step_with(x, &psi_c))).collect();
                let fallen: Vec<bool> = places.iter().map(|(_, d)| *d <= ratio(1, 2)).collect();
                let (mut fell, mut fell_held, mut heat, mut amt) = (crate::pulse::Manifest::new(), ratio(0, 1), ratio(0, 1), ratio(0, 1));
                let mut orth_held = ratio(0, 1);
                // The binding's own circulation: its members' orthogonal spins by
                // holding (the frame a member's carried circulation is read
                // against).
                let orth_binding = members.iter().fold(ratio(0, 1), |s, x| s + held_of(x) * x.ell_orth.clone()) / m.clone();
                let mut ph: Vec<Vec<(Ratio, Ratio)>> = vec![Vec::new(); crate::flows::PLANES];
                // THE STAR'S OWN SPIN (with xylarium 2026-10-23): the spin its
                // matter carried before the collapse, conserved through the
                // infall: each member's the balance of its place in the
                // binding, `ℓ′ = d·H_c(<d)` (its phase place d and the hold of
                // every member more in step, fallen or not); the star's is
                // their holding-weighted mean over what fell.
                let hold_c = |r: &Ratio| places.iter().filter(|(_, d)| d < r).fold(ratio(0, 1), |a, (w, _)| a + w.clone() / m.clone());
                let mut fell_spins: Vec<(Ratio, Ratio, Ratio)> = Vec::new();
                let mut outside: Vec<(crate::flows::Mote, Ratio, Ratio)> = Vec::new();
                // THE SYSTEM'S RIM is the whole turn (`1 − C ≤ 1` always):
                // every member off the heart's hemisphere is its disc.
                let rho_birth = ratio(1, 1);
                for (idx, (x, f)) in members.iter().zip(&fallen).enumerate() {
                    let w = held_of(x);
                    let place = places[idx].1.clone();
                    if *f {
                        *fell.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
                        fell_held += w.clone();
                        heat += x.amount.clone() * x.agitation.clone();
                        amt += x.amount.clone();
                        fell_spins.push((w.clone(), place.clone(), hold_c(&place)));
                        orth_held += w.clone() * x.ell_orth.clone();
                        for k in 0..crate::flows::PLANES {
                            ph[k].push((w.clone(), x.phase.get(k).cloned().unwrap_or_else(|| ratio(0, 1))));
                        }
                    } else {
                        // THE DISC: it stays at its phase place off step, its
                        // spins read at the system's birth once the heart is
                        // cut (`formation::system`); its circulation is what it
                        // CARRIED, its orthogonal spin against the binding's,
                        // as a share of its orbit spin.
                        let carried_c = if x.ell_o.is_zero() { ratio(0, 1) } else { (x.ell_orth.clone() - orth_binding.clone()) / x.ell_o.clone() };
                        outside.push((crate::flows::Mote { born: bell, ..x.clone() }, place, carried_c));
                    }
                }
                collapse.0 += m.clone();
                collapse.1 += fell_held.clone();
                if fell.is_empty() || fell_held.is_zero() {
                    // Nothing fell: the members go on as they were.
                    motes_in[3] += members.len();
                    core.flows.extend(members);
                    continue;
                }
                let a = if amt.is_zero() { ratio(0, 1) } else { heat / amt };
                let spin = crate::ring::grain(&(fell_spins.iter().fold(ratio(0, 1), |a, (w, r, h)| a + w.clone() * r.clone() * h.clone()) / fell_held.clone()));
                let phases: Vec<Ratio> = ph.iter().map(|x| crate::ring::grain(&crate::flows::circle_mean(x))).collect();
                let orth_mean = orth_held / fell_held;
                // The newborn's whole: what fell and its disc.
                let whole_c = m.clone();
                takes.push((reach, fell, outside, whole_c, a, spin, phases, orth_mean, members, fell_spins, rho_birth));
            }
            for (reach, star, outside, whole_held, a_own, spin_own, phases_own, orth_mean, members, fell_spins, rho_birth) in takes {
                let whole_mass = whole_held.clone();
                let m_u = whole_mass.clone() / unit.clone();
                if m_u.is_zero() {
                    continue;
                }
                // IGNITION: its own agitation (the galaxy's unlocked face is in
                // it as the light's warming, never added again, docs/96 §8).
                let tide = crate::ring::deficit(&reach, &h);
                let a_binding = a_own.clone();
                // ITS ORBIT, in the galaxy's one frame, where it is kept: the
                // binding's common spin (its members' holding-weighted mean,
                // `Σ w·(ℓ − ℓ̄) = 0`); its reach the enclosure's.
                let mw = members.iter().fold(ratio(0, 1), |acc, x| acc + x.amount.clone() * crate::flows::weight(&x.key));
                let mean_own = if mw.is_zero() { ratio(0, 1) } else { members.iter().fold(ratio(0, 1), |acc, x| acc + x.amount.clone() * crate::flows::weight(&x.key) * x.ell_o.clone()) / mw.clone() };
                let taken = m_u.clone() * mean_own.clone();
                let held = core.satellites.iter().fold(mass(&core.kept), |acc, sys| acc + mass(&sys.total())) / unit.clone() + state.clone();
                let r_guess = crate::ring::grain(&(enc_now.reach(&mean_own).0 * scale.clone()));
                let siblings: Vec<u64> = core.satellites.iter().map(|l| l.path[1]).collect();
                let path = vec![core.path[0], crate::ring::birth_key(bell, &r_guess, &held, &siblings)];
                // CARRIED SPINS ADD: the star's own spin, captured from the
                // spread of what fell, is the turning its heart cut adds to
                // every kind (no weight ratio).
                let lit = crate::formation::ignition_share(&a_binding).and_then(|_| crate::formation::system(&star, outside, &whole_mass, &a_binding, &fell_spins, &path, &tide));
                let Some((kept, medium, medium_spin, disc_motes)) = lit else {
                    // No heart: the members go on as they were.
                    motes_in[3] += members.len();
                    core.flows.extend(members);
                    continue;
                };
                // Its STATE is its holding at birth (a heart's state, set at
                // its forming, never drained); its own spin and phases.
                let held_at_birth = mass(&kept);
                // A star keeps its IGNITION SHARE (the heart's share at its
                // throat condition) in `reservoir`: decoherence sheds only
                // what it holds above it.
                let Some(ignition) = crate::formation::ignition_share(&a_binding) else { continue };
                let mut sys = Body { ell_orth: ratio(0, 1), path, radius: r_guess, period: ratio(0, 1), kept, satellites: Vec::new(), medium, ell: ratio(0, 1), medium_spin, gas: Vec::new(), born: bell, state: held_at_birth.clone(), flows: disc_motes, reservoir: ignition, turning: crate::ring::grain(&(held_at_birth.clone() / unit.clone() * spin_own)), bell: phases_own.clone(), phase: phases_own, load: ratio(0, 1), remnant: false, heat: ratio(0, 1), tilt: crate::flows::blend(&members.iter().map(|x| (x.amount.clone() * crate::flows::weight(&x.key), x.ell_o.clone(), x.tilt.clone())).collect::<Vec<_>>()), throat: Default::default() };
                {
                    // Read by the system's one enclosure, as its meetings read it.
                    if let Some(enc) = crate::formation::system_frame(&sys, &unit) {
                        for x in &sys.flows {
                            let held = x.amount.clone() * crate::flows::weight(&x.key);
                            discs.0 += held.clone();
                            if enc.reach(&x.ell_o).0 >= rho_birth {
                                discs.1 += held;
                            }
                        }
                    }
                }
                // Its orbit: the binding's common spin about the galaxy's core
                // (the members' own spins about the binding sum to nothing there,
                // `Σ w·(ℓ − ℓ̄) = 0`); the disc's spin about the star is its own,
                // on the medium's ledger.
                let own = taken;
                sys.ell = crate::ring::grain(&(own.clone() / m_u.clone()));
                // ITS ORTHOGONAL SPIN, its members' by holding (docs/90), in the
                // galaxy's frame as its orbit spin is.
                sys.ell_orth = crate::ring::grain(&orth_mean);
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
            crate::profile::mark("breath:merge");
            let held_before = core.flows.len();
            // Merged by the galaxy's own gatherings, its beads (no grid).
            let gathered_beads = crate::flows::beads(&core.flows, &galaxy_enc(core), &p, &g_mass, false);
            let remainder = crate::flows::merge(&mut core.flows, &gathered_beads, &ledger_unit, &place_of, &mut cross);
            motes_in[4] += held_before.saturating_sub(core.flows.len());
            motes_in[5] = core.flows.len();
            core.reservoir += remainder;
            let (kept, free) = crate::flows::read_cut(&core.flows, &galaxy_enc(core), &p);
            core.kept = kept;
            core.medium = free;
            core.gas.clear();
        }
        crate::profile::mark("breath:balance");
        // THE BALANCE (the motion law, `crate::ring`; cold bodies): each
        // system stands where its spin meets the holding inside it,
        // `R = ℓ / H(<ℓ)`, read by the galaxy's one enclosure at the echo's
        // end (its throat 1 and every mote and star inside, in its own frame,
        // as its torques, rides and meetings read it); its period follows,
        // `P = 1 + R / H`. Orbits breathe: a birth or a loss inside an orbit
        // moves it, the ledger deciding which way.
        let g_end = if home { whole_u.clone() } else { mass(&core.total()) };
        if !g_end.is_zero() && !unit.is_zero() {
            let scale_end = g_end.clone() / unit.clone();

            let firm_end: Vec<(Ratio, Ratio)> = if home { u.firmament.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g_end.clone())).collect() } else { Vec::new() };
            let enc_end = crate::flows::Enclosed::new(
                ratio(1, 1),
                core.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g_end.clone())).chain(core.satellites.iter().map(|b| (b.ell.clone(), mass(&b.total()) / g_end.clone()))).chain(firm_end).collect(),
            );
            for sys in core.satellites.iter_mut() {
                if sys.ell > ratio(0, 1) {
                    let own = sys.ell.clone();
                    let (r, _) = enc_end.reach(&own);
                    let h_in = enc_end.hold(&own);
                    // THE STAR'S VELOCITY (xylarium's operator, 2026-10-17:
                    // "the inverse of the drag on the planets"): its pull
                    // through the galaxy, shared against its worlds' load,
                    // `v = δ_gal·H/(H + d·Σx)` (H its system's own hold, 1
                    // in its units; d its torsion `r/(r + H_gal)`): a loaded
                    // star turns slower, its period stretched by `1 + d·Σx`.
                    let d = r.clone() / (r.clone() + h_in.clone());
                    sys.period = crate::ring::period(&r, &h_in) * (ratio(1, 1) + d * sys.load.clone());
                    sys.radius = crate::ring::grain(&(r * scale_end.clone()));
                }
            }
        }
        let galaxy_id = core.path[0] as u32;
        u.ocean.extend(to_ocean);
        crate::profile::mark("breath:echo-end");
        events.push(Event { bell, galaxy: galaxy_id, rung, write: Write::Perpendicular, stars, jets, returned, dissolved, merged, encounters, grown, escaped: escaped_all, fates, weights, impacts, discs, collapse, motes: motes_in, released, circulation, rang: rang_n, recycled, burned, burns, cross, light, tolls, identity });
    }
    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_home_has_no_clock_of_its_own_and_its_stars_read_the_source_directly() {
        use crate::pulse::{Body, Manifest, Universe};
        let body = |path: Vec<u64>, radius: Ratio, kept: Manifest, satellites: Vec<Body>| Body { ell_orth: ratio(0, 1), path, radius, period: ratio(0, 1), kept, satellites, medium: Manifest::new(), ell: ratio(0, 1), medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: ratio(0, 1), flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: ratio(0, 1), tilt: Vec::new(), throat: Default::default() };
        let h = || crate::chem::Kind::parse("W0001").expect("a kind");
        let star = body(vec![0, 1], ratio(3, 1), [(h(), ratio(1, 4))].into(), Vec::new());
        let home = body(vec![0], ratio(0, 1), Manifest::new(), vec![star]);
        let galaxy = body(vec![1], ratio(5, 1), [(h(), ratio(1, 2))].into(), Vec::new());
        let mut u = Universe { cores: vec![home, galaxy], ocean: Vec::new(), firmament: Vec::new(), unit: ratio(1, 1), t_u: 15, densest: 1200 };
        // The star's spin, kept in the home's frame: 2.
        u.cores[0].satellites[0].ell = ratio(2, 1);
        // The home's core is the source: no receiver, no clock.
        assert_eq!(galaxy_rate(&u, 0, &ratio(1, 1)), None);
        assert!(galaxy_rate(&u, 1, &ratio(1, 1)).is_some());
        // Its star reads its own share about the singularity's state alone.
        let singularity = crate::pulse::universal_state(&u);
        assert_eq!(centre_state(&u, 0), singularity.clone());
        // Nothing turns inside it: its reach is its spin over the throat's 1,
        // R = 2, δ = 1/3, its clock 2/3, no galaxy share nested in.
        assert_eq!(system_clock(&u, 0, 0, &ratio(1, 1)), ratio(2, 3));
        // A body's nested clock multiplies only the receivers it has.
        assert_eq!(nested(None, ratio(2, 3)), ratio(2, 3));
        assert_eq!(nested(Some(&ratio(1, 2)), ratio(2, 3)), ratio(1, 3));
    }

    #[test]
    fn light_made_is_received_or_drained_exactly() {
        let mote = |ell: (i64, i64), amount: (i64, i64)| crate::flows::Mote { key: crate::chem::Kind::parse("W0001").expect("a kind"), amount: ratio(amount.0, amount.1), ell_o: ratio(ell.0, ell.1), ell_orth: ratio(0, 1), phase: vec![ratio(0, 1), ratio(0, 1)], agitation: ratio(0, 1), born: 0, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1), tilt: Vec::new() };
        let mut motes = vec![mote((1, 2), (1, 4)), mote((3, 1), (1, 8))];
        let l = ratio(1, 1000);
        // An enclosure holding ½ and nothing else: the motes stand at reach
        // ℓ/½, 1 and 6.
        let enc = crate::flows::Enclosed::new(ratio(1, 2), Vec::new());
        let (got, gone) = shine(&mut motes, &mut [], &enc, &l, &ratio(1, 1));
        assert_eq!(got.clone() + gone, l, "made = received + drained");
        assert!(got > ratio(0, 1));
        // The farther body stands more on the unlocked face and warms more.
        assert!(motes[1].agitation > motes[0].agitation);
    }

    #[test]
    fn the_cores_light_crosses_into_its_systems_and_warms_their_worlds() {
        use crate::pulse::{Body, Manifest};
        let kind = |z: &str| crate::chem::Kind::parse(z).expect("a kind");
        let body = |radius: Ratio, kept: Manifest, satellites: Vec<Body>| Body { ell_orth: ratio(0, 1), path: vec![1, 1], radius, period: ratio(0, 1), kept, satellites, medium: Manifest::new(), ell: ratio(0, 1), medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: ratio(0, 1), flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: ratio(0, 1), tilt: Vec::new(), throat: Default::default() };
        // A system of a star and one world, at reach 2 in a galaxy of hold 1/2
        // (its spin `R·H` = 1, kept in the galaxy's frame); the world turning
        // in its system.
        let mut world = body(ratio(1, 1), [(kind("W0010"), ratio(1, 16))].into(), Vec::new());
        world.ell = ratio(1, 1);
        let mut systems = vec![body(ratio(2, 1), [(kind("W0003"), ratio(1, 4))].into(), vec![world])];
        systems[0].ell = ratio(1, 1);
        let enc = crate::flows::Enclosed::new(ratio(1, 2), Vec::new());
        let l = ratio(1, 1000);
        // The galaxy holds the system and as much again besides.
        let whole = mass(&systems[0].total()) * ratio(2, 1);
        let (inside, drained) = shine_systems(&mut systems, &l, &enc, &whole, &ratio(1, 1));
        // The world warmed, and what the system took is received or drained
        // inside it, exactly: never more than the core sent.
        assert!(systems[0].satellites[0].heat > ratio(0, 1));
        assert!(inside > ratio(0, 1));
        assert!(inside.clone() + drained <= l);
    }

    #[test]
    fn a_toll_is_heard_after_its_route_through_the_throats() {
        use crate::pulse::{Body, Manifest, Universe};
        let core = |path: Vec<u64>, radius: Ratio| Body { ell_orth: ratio(0, 1), path, radius, period: ratio(0, 1), kept: Manifest::new(), satellites: Vec::new(), medium: Manifest::new(), ell: ratio(0, 1), medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state: ratio(0, 1), flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), phase: Vec::new(), load: ratio(0, 1), bell: Vec::new(), remnant: false, heat: ratio(0, 1), tilt: Vec::new(), throat: Default::default() };
        let u = Universe { cores: vec![core(vec![1], ratio(5, 1)), core(vec![2], ratio(9, 1))], ocean: Vec::new(), firmament: Vec::new(), unit: ratio(1, 1), t_u: 15, densest: 1200 };
        let t = Toll { star: vec![1, 7], bell: 40, passing: Passing::Spent, strokes: Some(3), returned: ratio(1, 1), spin: ratio(0, 1), star_reach: ratio(2, 1), galaxy_reach: ratio(5, 1) };
        // Its own galaxy hears it after its reach to the core; another after
        // the route through the universal core.
        assert_eq!(t.heard_after(&u, 0), Some(ratio(2, 1)));
        assert_eq!(t.heard_after(&u, 1), Some(ratio(2 + 5 + 9, 1)));
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
