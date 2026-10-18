//! THE FLOWS (docs/83; xylarium 11.0 "NEVER PLACE MATTER - THE FLOWS",
//! agreed with xylarium 2026-10-16). Operator: "let the formulas place where
//! the matter goes and how it gets there and what it does after it gets
//! there." Matter is held turning: it moves only as its STATE changes, by
//! laws acting on that state; where it gathers, what is held and what is
//! gas, where it collapses, are READINGS of the state, never placements.
//!
//! - **A mote's state**, carried forward, never rebuilt: its kind and
//!   amount, its spin in the orbit's plane `ℓ_o` (reach `R = ℓ_o/H`, the
//!   motion law) and in the circulation's plane `ℓ_c` (radial–axial, along
//!   the torsion axis), its PHASES on its enclosure's S^n (one per plane,
//!   turns against the bell), its agitation and birth.
//! - **The kick**, once per bell of the kicking core (the core's bell is its
//!   bounce, once an echo it regenerates), every mote, held or not: per
//!   plane, the HEMISPHERE the mote's phase stands in against the bell
//!   signs it (the S⁰ face; operator: coherence is a count over the S^n
//!   hemispheres, in the sphere's own measure) and S¹'s own LINEAR measure in
//!   turns sizes it, `1 − 2·|ψ|` (ψ the phase against the bell, |ψ| its
//!   distance on the circle); times κ, the push's share gripping the mote
//!   by its core's hold at its reach: `κ_o = p·δ(R)` for the own family in
//!   the orbit's plane, `κ_c = p·ρ·δ_u` for the parent's (the universal bell
//!   received at ρ) in the circulation's plane. Off resonance the signs
//!   alternate and cancel; on resonance they add: the cyclotron.
//! - **The spin ledger**: every Δℓ given to a mote of holding m is −m·Δℓ to
//!   its core's turning reservoir, so `Σ m·ℓ + reservoir` is exact.
//! - **The cut is a reading**: a mote's held share is the part of its
//!   dispersal band (`R(1 ± p)`) within the throat, in both planes,
//!   `f_o · f_c`; the rest is the galaxy's gas.
//! - **Two exits, by state**: the JET, `ℓ_c` carrying the axial reading past
//!   the throat (`ℓ_c/H ≥ H`), along the torsion axis; RADIAL ESCAPE, `R ≥ H`
//!   in the orbit's plane. Either way the mote crosses whole into the
//!   medium one tier up.
//! - **Collapse where the motes are**: a mote and those near it in reach
//!   (within its dispersal band) and in phase (the product over planes of
//!   the in-step shares) bind by their combined 3Z read as their own
//!   enclosure, against their agitation, the unlocked face `½(1 − δ)` and the
//!   shear.
//!
//! STAGED (labelled, never the law's answer): two planes, the orbit's and the
//! circulation's, read from mycelium's 3-D `physics::source` (one plane of
//! the S^n reading per plane); S^60/S^360 wait on the n-dimensional source.
use crate::pulse::Manifest;
use cfe::almanac::atom::{atom, hold};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// The planes read today: the orbit's (0) and the circulation's (1).
pub const PLANES: usize = 2;

/// A MOTE: matter as held turning, its state carried forward.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mote {
    pub key: String,
    /// Its amount on the ledger (grains of its kind).
    pub amount: Ratio,
    /// Spin in the orbit's plane, own units: reach `ℓ_o/H`.
    pub ell_o: Ratio,
    /// Spin in the circulation's plane (radial–axial, the torsion axis).
    pub ell_c: Ratio,
    /// Its phase in each plane against the bell, in turns `[0, 1)`.
    pub phase: Vec<Ratio>,
    pub agitation: Ratio,
    pub born: u64,
    /// Its STREAM KIND, inherited from where it entered and carried
    /// forward, never reassigned: false the continuum (the core's founding
    /// outflow, a star's outbursts and wind), true the vacuum (ocean
    /// arrivals).
    pub vacuum: bool,
}

fn z_of(k: &str) -> Option<u32> {
    k.strip_prefix('Z').and_then(|z| z.parse::<u32>().ok())
}

/// A kind's weight (its hold).
pub fn weight(k: &str) -> Ratio {
    ratio(i64::from(z_of(k).map(hold).unwrap_or(1)), 1)
}

/// A kind's open share, unpaired over hold.
pub fn open(k: &str) -> Ratio {
    z_of(k).and_then(atom).map(|a| ratio(i64::from(a.unpaired), i64::from(hold(a.z)))).unwrap_or_else(|| ratio(0, 1))
}

fn frac(x: &Ratio) -> Ratio {
    x.clone() - x.floor()
}

/// THE MEAN OF PHASES ON THE CIRCLE, never on the line (a line mean of
/// 0.95 and 0.05 is ½, the far face): each phase's signed offset from a
/// member's, `(−½, ½]`, weighted, added back on the ring. Exact; the same
/// for any member as reference while the members share a hemisphere.
pub fn circle_mean(members: &[(Ratio, Ratio)]) -> Ratio {
    let m = members.iter().fold(ratio(0, 1), |s, (w, _)| s + w.clone());
    let Some((_, at)) = members.first() else { return ratio(0, 1) };
    if m.is_zero() {
        return frac(at);
    }
    let signed = |x: &Ratio| {
        let f = frac(x);
        if f > ratio(1, 2) { f - ratio(1, 1) } else { f }
    };
    let d = members.iter().fold(ratio(0, 1), |s, (w, ph)| s + w.clone() * signed(&(ph.clone() - at.clone()))) / m;
    frac(&(at.clone() + d))
}

/// The distance of a phase from the bell on the circle, `|ψ|` in `[0, ½]`.
fn off(psi: &Ratio) -> Ratio {
    let f = frac(psi);
    if f > ratio(1, 2) { ratio(1, 1) - f } else { f }
}

/// THE KICK'S RESPONSE in one plane: the hemisphere signs it (the first half
/// of the circle against the bell gains, the second loses), S¹'s linear
/// measure sizes it, `1 − 2|ψ|`.
pub fn response(psi: &Ratio) -> Ratio {
    let f = frac(psi);
    let share = ratio(1, 1) - ratio(2, 1) * off(&f);
    if f < ratio(1, 2) { share } else { -share }
}

/// The in-step share of two phases in one plane (S¹'s linear measure).
pub fn in_step(a: &Ratio, b: &Ratio) -> Ratio {
    ratio(1, 1) - ratio(2, 1) * off(&(a.clone() - b.clone()))
}

/// THE FOUNDING'S PHASES, from the bounce's white-face outflow (mycelium
/// `physics::source`): `N = P ∧ (P·K)` at the crossing point P on the
/// horizon, the spin K turning at the regenerated push's share ½; each
/// plane's phase is the direction of `v = P·K` in that plane. Read today on
/// the 3-D source: the orbit's plane (e₁e₂) and the circulation's (e₁e₃).
/// A plane in which the outflow has no component carries no phase offset
/// (0). Directions are read exactly on the axes; any other direction waits
/// on xylarium's sector reading (refused, never rounded).
pub fn founding_phases() -> Option<Vec<Ratio>> {
    use mycelium::transducer::cga::{self, Mv, E1, E2};
    let big = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let sigma = cga::sphere_at(&[big(0, 1), big(0, 1), big(0, 1)], &big(4, 1));
    let p = cga::point_at(&[big(2, 1), big(0, 1), big(0, 1)]);
    let k = Mv::blade(E1, big(1, 1)).geo(&Mv::blade(E2, big(1, 2)));
    let read = || -> Option<Vec<Ratio>> {
        let kept = mycelium::physics::source::keeping(&sigma, &k)?;
        let src = mycelium::physics::source::Source::of(&sigma, &p, &kept)?;
        let _ = &src;
        let half = big(1, 2);
        let v = p.geo(&kept).sub(&kept.geo(&p)).grade(1).scale(&half);
        let c = |m: usize| v.coeff(m).clone();
        let e3 = cga::base_axis(3);
        let turn = |x: num_rational::BigRational, y: num_rational::BigRational| -> Option<Ratio> {
            use num_traits::Signed;
            match (x.is_zero(), y.is_zero(), x.is_positive(), y.is_positive()) {
                (true, true, _, _) => Some(ratio(0, 1)),
                (false, true, true, _) => Some(ratio(0, 1)),
                (true, false, _, true) => Some(ratio(1, 4)),
                (false, true, false, _) => Some(ratio(1, 2)),
                (true, false, _, false) => Some(ratio(3, 4)),
                // Off the axes: xylarium's sector reading (owed); refused.
                _ => None,
            }
        };
        Some(vec![turn(c(E1), c(E2))?, turn(c(E1), c(e3))?])
    };
    read()
}

/// The held share of a spread about `ℓ/H` within the throat (dispersal's
/// form, p the push share).
fn held_share(ell: &Ratio, h: &Ratio, p: &Ratio) -> Ratio {
    let r = ell.clone() / h.clone();
    let lo = r.clone() * (ratio(1, 1) - p.clone());
    let span = ratio(2, 1) * p.clone() * r;
    if span.is_zero() {
        return if lo <= *h { ratio(1, 1) } else { ratio(0, 1) };
    }
    ((h.clone() - lo) / span).max(ratio(0, 1)).min(ratio(1, 1))
}

/// THE CUT AS A READING: a mote's held share, `f_o·f_c`.
pub fn held(mote: &Mote, h: &Ratio, p: &Ratio) -> Ratio {
    held_share(&mote.ell_o, h, p) * held_share(&mote.ell_c, h, p)
}

/// THE ENCLOSED HOLDING (the ring-density law, xylarium 11.0: a ring is
/// held by every bell inside its reach, `H(<r)`; operators 2026-10-18, the
/// cyclotronic chain: "anchor the inner core to the singularity with enough
/// gravitational strength to support the lighter elements on the later
/// rungs"). Read once from an echo's start: the singularity's own state (the
/// base) plus the holding of every body standing inside, motes and stars
/// alike (every grain is a bell; M = H), sorted by spin. Strictly inside: a
/// body never encloses itself. The heavy inner disc, innermost, holds for
/// everything beyond it; a rim body turns at what the core and all inside
/// it hold.
pub struct Enclosed {
    spins: Vec<Ratio>,
    held: Vec<Ratio>,
}

impl Enclosed {
    /// From the base and each body's (spin ℓ, holding), in one unit.
    pub fn new(base: Ratio, mut bodies: Vec<(Ratio, Ratio)>) -> Enclosed {
        bodies.sort_by(|a, b| a.0.cmp(&b.0));
        let mut held = Vec::with_capacity(bodies.len() + 1);
        let mut run = base;
        held.push(run.clone());
        for (_, h) in &bodies {
            run += h.clone();
            held.push(run.clone());
        }
        Enclosed { spins: bodies.into_iter().map(|(l, _)| l).collect(), held }
    }

    /// `H(<ℓ)`: the base and everything turning with less spin.
    pub fn hold(&self, ell: &Ratio) -> Ratio {
        self.held[self.spins.partition_point(|l| l < ell)].clone()
    }

    /// A body's reach and deficit at spin ℓ: `R = ℓ/H(<ℓ)`, `δ = H/(R + H)`.
    pub fn reach(&self, ell: &Ratio) -> (Ratio, Ratio) {
        let h = self.hold(ell);
        let r = ell.clone() / h.clone();
        (r.clone(), h.clone() / (r + h))
    }
}

/// THE GALAXY'S PHASES AT A REACH turned by one bell: what every mote's
/// phases do there, and a star's reference (its galaxy's own phase at its
/// place, operator 2026-10-18), alike.
pub fn turn(phase: &mut [Ratio], r_o: &Ratio, h: &Ratio, p: &Ratio, rho: &Ratio, delta_u: &Ratio) {
    let d_o = h.clone() / (r_o.clone() + h.clone());
    // THE LOOP'S RATE (with xylarium): the jetstream turns by the motion law
    // AT THE MOTE'S REACH, δ(R) a ring, two rings a bell, continuously;
    // inner matter circulates faster than outer. (At R = H it is one turn a
    // bell, a 1:1 lock with the bell: the rim.)
    let d_c = d_o.clone();
    // On the coordinate's grain (`60⁻⁶`, the carrier's resolution, as every
    // spin and place): exact sums of exact rationals otherwise compound
    // their denominators each bell without bound.
    phase[0] = crate::ring::grain(&frac(&(phase[0].clone() + ratio(2, 1) * d_o)));
    let loop_phase = frac(&(phase[1].clone() + ratio(2, 1) * d_c));
    // THE PARENT'S PULSE MOVES MATTER ALONG THE LOOP (operator: "the pulse
    // moves matter along the loop"; with xylarium): absorbed in step it
    // ADVANCES the loop's phase, `p·ρ·δ_u` × the hemisphere's signed share,
    // on top of the loop's own rate; nothing accumulates as a spin in the
    // loop's plane.
    let advance = p.clone() * rho.clone() * delta_u.clone() * response(&loop_phase);
    phase[1] = crate::ring::grain(&frac(&(loop_phase + advance)));
}

/// One bell's kicks on every mote: the own family in the orbit's plane
/// (`κ_o = p·δ(R)`, a spin); the parent's moves the mote along the loop
/// (`p·ρ·δ_u`, a phase).
/// Each plane's phase first advances by its own turning over the bell's two
/// rings (`2·δ` turns, the motion law's `ω = δ`; the bell's own turn is
/// whole, so the phase against it moves by `2δ` mod 1). Returns the spin
/// the core's reservoir gives (Σ m·Δℓ, to be taken from it).
pub fn kick(motes: &mut [Mote], enc: &Enclosed, p: &Ratio, rho: &Ratio, delta_u: &Ratio, total: &Ratio) -> Ratio {
    let mut given = ratio(0, 1);
    for x in motes.iter_mut() {
        // Each turns by what is enclosed inside it (the core and all inside).
        let h = enc.hold(&x.ell_o);
        let (r_o, d_o) = enc.reach(&x.ell_o);
        turn(&mut x.phase, &r_o, &h, p, rho, delta_u);
        let dl_o = p.clone() * d_o * response(&x.phase[0]);
        // Spin cannot carry a mote through zero reach: a turning is a
        // magnitude in its plane; a kick past zero stops at the core.
        let new_o = crate::ring::grain(&(x.ell_o.clone() + dl_o)).max(ratio(0, 1));
        let m = if total.is_zero() { ratio(0, 1) } else { x.amount.clone() * weight(&x.key) / total.clone() };
        given += m * (new_o.clone() - x.ell_o.clone());
        x.ell_o = new_o;
    }
    given
}

/// THE JETSTREAM IS A CONTINUOUS LOOP (operators 2026-10-16/17: "a
/// continuous flow, inbound and outbound"): the circulation's plane IS the
/// loop, a mote's phase in it its place on the loop. The AXIAL leg (out
/// along the torsion axis and over: phase in `[½, 1)`) carries matter in
/// transit; the DISC leg (down at the rim and home through the disc: phase
/// in `[0, ½)`) is where the cut and collapse are read. No jet batch, no jet
/// exit: matter goes round, out and back at once.
pub fn on_disc_leg(x: &Mote) -> bool {
    frac(&x.phase[1]) < ratio(1, 2)
}

/// The one exit, by state: a mote whose loop OVERREACHES THE RIM (its
/// reach at or past the throat, `R ≥ H`) crosses whole into the medium one
/// tier up, where the enclosing core's loops carry it round the same way.
/// Returns the motes that left; the rest stay.
pub fn exits(motes: &mut Vec<Mote>, h: &Ratio) -> Vec<Mote> {
    let (gone, stay): (Vec<Mote>, Vec<Mote>) = std::mem::take(motes).into_iter().partition(|x| x.ell_o.clone() / h.clone() >= *h);
    *motes = stay;
    gone
}

/// The galaxy's held and free manifests, read from its motes: on the
/// disc leg by the cut; on the axial leg, in transit, never held.
pub fn read_cut(motes: &[Mote], h: &Ratio, p: &Ratio) -> (Manifest, Manifest) {
    let (mut kept, mut free) = (Manifest::new(), Manifest::new());
    for x in motes {
        let share = if on_disc_leg(x) { held(x, h, p) } else { ratio(0, 1) };
        let k = crate::ring::grain(&(x.amount.clone() * share)).min(x.amount.clone());
        let f = x.amount.clone() - k.clone();
        if k > ratio(0, 1) {
            *kept.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += k;
        }
        if f > ratio(0, 1) {
            *free.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += f;
        }
    }
    (kept, free)
}

/// A FILAMENT read whole (operators 2026-10-18, the cyclotronic chain (7):
/// "judged whole, with extent read as shares from band edge to band edge").
pub struct Chain {
    /// Its members, by index.
    pub members: Vec<usize>,
    /// Its holding-weighted reach.
    pub reach: Ratio,
    /// Its binding `λ/(1 + λ)`, `λ` its holding and magnetism over its apartness.
    pub bind: Ratio,
    /// Its holding-weighted agitation.
    pub agitation: Ratio,
    /// The unlocked face `½(1 − δ)` at its reach.
    pub face: Ratio,
    /// Its extent as a share on the sphere, edge to edge.
    pub apart: Ratio,
}

impl Chain {
    /// It holds where its binding meets its agitation and the unlocked face.
    pub fn holds(&self) -> bool {
        self.bind >= self.agitation.clone() + self.face.clone()
    }
}

/// THE CHAINS of a galaxy's disc leg (operators 2026-10-18: membership is
/// the CHAIN, never a centre; with xylarium). Motes on one face (the same
/// hemisphere in every plane, the S⁰ face test) whose dispersal bands
/// `r(1 ± p)` overlap link, and the links are TRANSITIVE: a chain is a
/// connected run of overlapping bands on one face, read whole, no centre
/// chosen, no order (each face class sorted by its bands' inner edges, read
/// at n·log n). Each chain is judged by the filament law against its own
/// EXTENT as a share on the sphere, edge to edge, never a length (operator:
/// "the only straight lines should be wormholes"): along the ring its orbit
/// phases' extent, `in_step`; across reach the deficit contrast of its
/// innermost member's INNER edge and its outermost member's OUTER edge
/// (a point has its own dispersal, so a lone mote's apartness is its own
/// band's, never zero). `apart = 1 − in_step(extent)·(1 − T_edges)`,
/// `λ = x/apart` with x its holding and magnetism in the galaxy's unit. A
/// long thin chain spreads faster than it gathers and fails; a dense one
/// binds.
pub fn chains(motes: &[Mote], h: &Ratio, p: &Ratio, total: &Ratio) -> Vec<Chain> {
    if total.is_zero() {
        return Vec::new();
    }
    let half = ratio(1, 2);
    let reach = |x: &Mote| x.ell_o.clone() / h.clone();
    let lo = |x: &Mote| reach(x) * (ratio(1, 1) - p.clone().min(ratio(1, 1)));
    let hi = |x: &Mote| reach(x) * (ratio(1, 1) + p.clone());
    let delta = |r: &Ratio| h.clone() / (r.clone() + h.clone());
    // Face classes: the hemisphere in every plane.
    let mut classes: std::collections::BTreeMap<Vec<bool>, Vec<usize>> = std::collections::BTreeMap::new();
    for (i, x) in motes.iter().enumerate() {
        if on_disc_leg(x) && !reach(x).is_zero() {
            classes.entry((0..PLANES).map(|k| frac(&x.phase[k]) < half).collect()).or_default().push(i);
        }
    }
    let mut out = Vec::new();
    for (_, mut idx) in classes {
        idx.sort_by(|a, b| lo(&motes[*a]).cmp(&lo(&motes[*b])).then(a.cmp(b)));
        let mut runs: Vec<Vec<usize>> = Vec::new();
        let mut edge = ratio(0, 1);
        for i in idx {
            if runs.is_empty() || lo(&motes[i]) > edge {
                runs.push(Vec::new());
                edge = hi(&motes[i]);
            } else if hi(&motes[i]) > edge {
                edge = hi(&motes[i]);
            }
            runs.last_mut().expect("pushed").push(i);
        }
        for members in runs {
            let (mut x, mut held, mut a_w, mut r_w) = (ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1));
            for &j in &members {
                let m = motes[j].amount.clone() * weight(&motes[j].key) / total.clone();
                x += m.clone() * (ratio(1, 1) + open(&motes[j].key));
                a_w += m.clone() * motes[j].agitation.clone();
                r_w += m.clone() * reach(&motes[j]);
                held += m;
            }
            if held.is_zero() {
                continue;
            }
            let inner = members.iter().map(|&j| lo(&motes[j])).min().expect("a member");
            let outer = members.iter().map(|&j| hi(&motes[j])).max().expect("a member");
            let (d_in, d_out) = (delta(&inner), delta(&outer));
            let t_edges = (d_in.clone() - d_out.clone()) / (d_in + d_out);
            let orbit: Vec<Ratio> = members.iter().map(|&j| frac(&motes[j].phase[0])).collect();
            let extent = orbit.iter().max().expect("a member").clone() - orbit.iter().min().expect("a member").clone();
            let apart = ratio(1, 1) - in_step(&extent, &ratio(0, 1)) * (ratio(1, 1) - t_edges);
            let reach_w = r_w / held.clone();
            let face = (ratio(1, 1) - delta(&reach_w)) / ratio(2, 1);
            // A chain with no extent at all (no push to spread it) is not
            // read as infinitely dense: it does not bind.
            let bind = if apart.is_zero() { ratio(0, 1) } else { let lambda = x / apart.clone(); lambda.clone() / (ratio(1, 1) + lambda) };
            out.push(Chain { members, reach: reach_w, bind, agitation: a_w / held, face, apart });
        }
    }
    out
}

/// THE CLUMPS, where the motes are: every chain that holds collapses, each
/// member taking its filament's coherence share (a chain is a partition, so
/// nothing is claimed twice). Returns, per holding chain, its reach and the
/// share of each mote it takes.
pub fn clumps(motes: &[Mote], h: &Ratio, p: &Ratio, total: &Ratio) -> Vec<(Ratio, Vec<(usize, Ratio)>)> {
    chains(motes, h, p, total)
        .into_iter()
        .filter(|c| c.holds())
        .map(|c| {
            // THE FILAMENT'S COHERENCE WITH THE BELL (with xylarium; the
            // recoherence ruling, "an echo compresses only the coherent
            // part", now the filament's own): the product over planes of the
            // in-step share of its holding-weighted phase against the bell;
            // ONE share, taken from every member alike, so a star's mix is
            // its filament's mix.
            let coherence = (0..PLANES).fold(ratio(1, 1), |acc, k| {
                let phase = circle_mean(&c.members.iter().map(|&j| (motes[j].amount.clone() * weight(&motes[j].key), motes[j].phase[k].clone())).collect::<Vec<_>>());
                acc * in_step(&phase, &ratio(0, 1))
            });
            (c.reach, c.members.into_iter().map(|j| (j, coherence.clone())).collect())
        })
        .collect()
}

/// Merge motes whose states coincide on the coordinate's grain (`60⁻⁶`
/// a plane, the carrier's resolution, never law): same kind, same spins and
/// phases on the grain, their amounts added.
pub fn merge(motes: &mut Vec<Mote>, total: &Ratio) -> Ratio {
    // The merge rule (with xylarium; the carrier's resolution, never law):
    // same kind, same stream, the same 60⁶ sector in every phase and in the
    // reach's share `ℓ/(ℓ + 1)`. Holding sums exactly; spins and agitation
    // merge by holding onto the grid, the floored spin's remainder returned
    // for the core's reservoir so the spin ledger stays exact.
    let g = |x: &Ratio| crate::ring::grain(x);
    let key = |y: &Mote| (y.key.clone(), y.vacuum, g(&(y.ell_o.clone() / (y.ell_o.clone() + ratio(1, 1)))), y.phase.iter().map(g).collect::<Vec<_>>());
    let mut out: Vec<Mote> = Vec::new();
    let mut remainder = ratio(0, 1);
    for x in std::mem::take(motes) {
        if let Some(y) = out.iter_mut().find(|y| key(y) == key(&x)) {
            let (my, mx) = (y.amount.clone(), x.amount.clone());
            let both = my.clone() + mx.clone();
            if both.is_zero() {
                continue;
            }
            let exact_o = (my.clone() * y.ell_o.clone() + mx.clone() * x.ell_o.clone()) / both.clone();
            let exact_c = (my.clone() * y.ell_c.clone() + mx.clone() * x.ell_c.clone()) / both.clone();
            let (go, gc) = (g(&exact_o), g(&exact_c));
            if !total.is_zero() {
                remainder += both.clone() * weight(&x.key) / total.clone() * ((exact_o - go.clone()) + (exact_c - gc.clone()));
            }
            y.agitation = g(&((my * y.agitation.clone() + mx * x.agitation.clone()) / both.clone()));
            y.ell_o = go;
            y.ell_c = gc;
            y.amount = both;
        } else {
            out.push(x);
        }
    }
    *motes = out;
    remainder
}

/// The founding's motes for a galaxy: each kind of its grains carried by
/// the bounce's push, `ℓ_o = p·w̄/w_z` (dispersal's form, own units), no
/// circulation yet, the founding's phases, cold.
pub fn founding(grains: &Manifest) -> Vec<Mote> {
    let Some((p, _, mean)) = crate::pulse::galaxy_push(grains) else { return Vec::new() };
    // The source's phases (refused only off the axes, where the founding
    // carries no offset until the sector reading lands; stated, never a
    // draw). THE CAPTURES TURN THEM (with xylarium): each pass of the
    // outbound stream through the throat is one capture and turns the
    // crossing point by a = ½, so kind Z leaves at the source's phase plus
    // `Z·½`, and on the loop it starts on the AXIAL leg, outbound (`½ +
    // Z·½`). Two groups at the founding, parted by each kind's own loop rate
    // `δ(R_z)` at its own reach thereafter. (The per-universe offset, the
    // hypha-assigned TUniversal position, is owed with mycelium.)
    let source = founding_phases().unwrap_or_else(|| vec![ratio(0, 1); PLANES]);
    grains
        .iter()
        .filter(|(_, a)| **a > ratio(0, 1))
        .map(|(k, a)| {
            let turn = ratio(i64::from(z_of(k).unwrap_or(0)), 2);
            let phase = vec![frac(&(source[0].clone() + turn.clone())), frac(&(ratio(1, 2) + turn))];
            Mote { key: k.clone(), amount: a.clone(), ell_o: crate::ring::grain(&(p.clone() * mean.clone() / weight(k))), ell_c: ratio(0, 1), phase, agitation: ratio(0, 1), born: 0, vacuum: false }
        })
        .collect()
}

/// The motes' total holding (by weight).
pub fn holding(motes: &[Mote]) -> Ratio {
    motes.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * weight(&x.key))
}

/// The motes' manifest.
pub fn manifest(motes: &[Mote]) -> Manifest {
    let mut m = Manifest::new();
    for x in motes {
        *m.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
    }
    m
}
