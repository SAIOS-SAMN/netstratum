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
    ratio(i64::from(crate::chem::hold(k)), 1)
}

/// A kind's open share, unpaired over hold.
pub fn open(k: &str) -> Ratio {
    ratio(i64::from(crate::chem::unpaired(k)), i64::from(crate::chem::hold(k).max(1)))
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

/// THE KICK'S RESPONSE OVER ITS PASSAGE (with xylarium: a kick sampled at
/// one instant while the mote sweeps near a whole turn falls on one
/// hemisphere every time, and the core pays without end): the exact mean of
/// the sawtooth `1 − 2·frac(ψ)` over the arc `[ψ, ψ + len]`. Within each
/// turn it is linear, so a piece's mean is `1 − (f₀ + f₁)` from its ends'
/// fractions; no square; a whole turn averages 0.
pub fn mean_response(psi: &Ratio, len: &Ratio) -> Ratio {
    if len.is_zero() {
        return response(psi);
    }
    let end = psi.clone() + len.clone();
    let mut at = psi.clone();
    let mut sum = ratio(0, 1);
    while at < end {
        let turn = at.floor();
        let next = (turn.clone() + ratio(1, 1)).min(end.clone());
        let (f0, f1) = (at.clone() - turn.clone(), next.clone() - turn);
        sum += (next.clone() - at.clone()) * (ratio(1, 1) - (f0 + f1));
        at = next;
    }
    sum / len.clone()
}

/// The in-step share of two phases in one plane (S¹'s linear measure).
pub fn in_step(a: &Ratio, b: &Ratio) -> Ratio {
    ratio(1, 1) - ratio(2, 1) * off(&(a.clone() - b.clone()))
}

/// THE FIRMAMENT'S ORDER (operator 2026-10-19: "the pulses over time and the
/// reverberation/pulsation of the bouncing antipoles should move the
/// firmament into order (think, dodecahedron - but that needs to be derived
/// from the mathematics, not set as a dodecahedron)"; "build the
/// firmament"). The heart's own law, one tier up: matter pushes matter by
/// the ELECTROMAGNETIC REPULSION gripping its open frames. On each pulse,
/// every mote is pushed away from every other in each plane, by the other's
/// open charge (`a·e`, its amount times its open frames) times their
/// closeness on the sphere (the product over planes of their in-step
/// shares), signed away from it; summed over the whole charge, it moves by
/// the bounce's `push` times its own carried share `(e/ē)·(w̄/w)` (its open
/// frames over the mean, the mean weight over its own: the heaviest barely
/// move), at most half a turn. All read at once, then moved. Equal charges
/// pushing apart on a sphere settle into the polyhedra (the Thomson
/// arrangements): whatever order forms is the mathematics', never set.
pub fn repel(motes: &mut [Mote], push: &Ratio) {
    let n = motes.len();
    if n < 2 {
        return;
    }
    let planes = motes.iter().map(|x| x.phase.len()).min().unwrap_or(0);
    let open = |x: &Mote| ratio(i64::from(crate::chem::unpaired(&x.key)), 1);
    let amount = motes.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone());
    let charge = motes.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * open(x));
    let held = motes.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * weight(&x.key));
    if amount.is_zero() || charge.is_zero() || held.is_zero() || planes == 0 {
        return;
    }
    let (e_mean, w_mean) = (charge.clone() / amount.clone(), held / amount);
    let half = ratio(1, 2);
    let away = |d: &Ratio| {
        let f = frac(d);
        if f.is_zero() || f == half { ratio(0, 1) } else if f < half { ratio(1, 1) } else { ratio(-1, 1) }
    };
    let mut moves: Vec<Vec<Ratio>> = vec![vec![ratio(0, 1); planes]; n];
    for i in 0..n {
        let e_i = open(&motes[i]);
        if e_i.is_zero() {
            continue;
        }
        let carried = push.clone() * (e_i / e_mean.clone()) * (w_mean.clone() / weight(&motes[i].key));
        for j in 0..n {
            if i == j {
                continue;
            }
            let q = motes[j].amount.clone() * open(&motes[j]);
            if q.is_zero() {
                continue;
            }
            let close = (0..planes).fold(ratio(1, 1), |c, k| c * in_step(&motes[i].phase[k], &motes[j].phase[k]));
            if close.is_zero() {
                continue;
            }
            for k in 0..planes {
                let sg = away(&(motes[i].phase[k].clone() - motes[j].phase[k].clone()));
                if !sg.is_zero() {
                    moves[i][k] += q.clone() * close.clone() * sg;
                }
            }
        }
        for k in 0..planes {
            let m = (carried.clone() * moves[i][k].clone() / charge.clone()).max(-half.clone()).min(half.clone());
            moves[i][k] = m;
        }
    }
    for (x, m) in motes.iter_mut().zip(moves) {
        for k in 0..planes {
            x.phase[k] = crate::ring::grain(&frac(&(x.phase[k].clone() + m[k].clone())));
        }
    }
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
        let mut run = crate::ring::grain(&base);
        held.push(run.clone());
        // On the coordinate's grain (the carrier, never law): an exact sum of
        // thousands of holdings otherwise grows its denominators without
        // bound, and every reach, contact and kick inherits them.
        for (_, h) in &bodies {
            run += crate::ring::grain(h);
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
        let r = crate::ring::grain(&(ell.clone() / h.clone()));
        (r.clone(), crate::ring::grain(&(h.clone() / (r + h))))
    }
}

/// THE GALAXY'S PHASES AT A REACH turned by one bell: what every mote's
/// phases do there, and a star's reference (its galaxy's own phase at its
/// place, operator 2026-10-18), alike.
///
/// THE VACUUM IS THE CONTINUUM INVERTED (operator 2026-10-19: "the vacuum and
/// the continuum must be mathematically inverted across all dimensional
/// facets"; "reversal is not an inversion because the inversion happens on
/// an orthogonal dimension"; with xylarium): inversion through the throat,
/// `(R, θ) → (H²/R, θ)`, acts on the RADIAL dimension, orthogonal to every
/// plane the matter turns in, and keeps every angle. So an `inverted`
/// (vacuum) mote turns FORWARD in every plane, at its inverted reach, where
/// the pull reads `δ(H²/R) = 1 − δ(R)`, the continuum's push. Its stored spin
/// is untouched (`Σ m·ℓ` exact).
///
/// ITS ROUTE (operator 2026-10-16, xylarium 11.0 (11): "a continuum jetstream
/// carries matter out of the singularity along the axis and home through
/// the disc; a vacuum jetstream draws it in along the axis and out through
/// the disc"): the route round the loop, not the inversion. A vacuum mote's
/// loop (the circulation's plane) runs the other way, its advance read over
/// that passage; its orbit plane turns forward.
pub fn turn(phase: &mut [Ratio], r_o: &Ratio, h: &Ratio, p: &Ratio, rho: &Ratio, delta_u: &Ratio, inverted: bool) {
    let pull = h.clone() / (r_o.clone() + h.clone());
    let d_o = if inverted { ratio(1, 1) - pull } else { pull };
    // THE LOOP'S RATE (with xylarium): the jetstream turns by the motion law
    // AT THE MOTE'S REACH, δ(R) a ring, two rings a bell, continuously;
    // inner matter circulates faster than outer. (At R = H it is one turn a
    // bell, a 1:1 lock with the bell: the rim.)
    let d_c = d_o.clone();
    // On the coordinate's grain (`60⁻⁶`, the carrier's resolution, as every
    // spin and place): exact sums of exact rationals otherwise compound
    // their denominators each bell without bound.
    phase[0] = crate::ring::grain(&frac(&(phase[0].clone() + ratio(2, 1) * d_o)));
    let loop_from = phase[1].clone();
    let route = if inverted { ratio(-1, 1) } else { ratio(1, 1) };
    let loop_phase = frac(&(loop_from.clone() + route.clone() * ratio(2, 1) * d_c.clone()));
    // THE PARENT'S PULSE MOVES MATTER ALONG THE LOOP (operator: "the pulse
    // moves matter along the loop"; with xylarium): absorbed in step it
    // ADVANCES the loop's phase, `p·ρ·δ_u` × the hemisphere's signed share,
    // on top of the loop's own rate; nothing accumulates as a spin in the
    // loop's plane.
    let start = if inverted { loop_from - ratio(2, 1) * d_c.clone() } else { loop_from };
    let advance = p.clone() * rho.clone() * delta_u.clone() * mean_response(&start, &(ratio(2, 1) * d_c));
    phase[1] = crate::ring::grain(&frac(&(loop_phase + route * advance)));
}

/// One bell's kicks on every mote: the own family in the orbit's plane
/// (`κ_o = p·δ(R)`, a spin); the parent's moves the mote along the loop
/// (`p·ρ·δ_u`, a phase).
/// Each plane's phase first advances by its own turning over the bell's two
/// rings (`2·δ` turns, the motion law's `ω = δ`; the bell's own turn is
/// whole, so the phase against it moves by `2δ` mod 1). Returns the spin
/// the core's reservoir gives (Σ m·Δℓ, to be taken from it), and the pulse
/// the matter ABSORBED (Σ m·|Δℓ|, unsigned: each hemisphere's kick alike;
/// with xylarium), what the recycling budget is read against.
pub fn kick(motes: &mut [Mote], enc: &Enclosed, p: &Ratio, rho: &Ratio, delta_u: &Ratio, total: &Ratio) -> (Ratio, Ratio) {
    let mut given = ratio(0, 1);
    let mut absorbed = ratio(0, 1);
    for x in motes.iter_mut() {
        // Each turns by what is enclosed inside it (the core and all inside).
        let h = enc.hold(&x.ell_o);
        let (r_o, d_o) = enc.reach(&x.ell_o);
        let from = x.phase[0].clone();
        turn(&mut x.phase, &r_o, &h, p, rho, delta_u, x.vacuum);
        // Over its whole passage this bell, never at one instant; a vacuum
        // mote at its inverted reach (`1 − δ`), forward.
        let d_o = if x.vacuum { ratio(1, 1) - d_o } else { d_o };
        let k = mean_response(&from, &(ratio(2, 1) * d_o.clone()));
        let dl_o = p.clone() * d_o.clone() * k.clone();
        // DISSIPATION (operator 2026-10-19: "what do our current physics tell
        // about heat dissipation to the void? probably mimic nature"; with
        // xylarium): what the kick does not deliver in step becomes HEAT,
        // `p·δ·(1 − |k|)` (k its signed mean over the passage, the coherent
        // share), and heat leaves by the built cooling law's two channels
        // over the passage's two rings: the light through its open frames
        // (`e`, open over held, a ring) and the vacuum's pull back (`δ` at its
        // reach, a ring): `a ← (a + heat)·((1 − e)(1 − δ))²`.
        let coherent = if k < ratio(0, 1) { -k } else { k };
        let heat = p.clone() * d_o.clone() * (ratio(1, 1) - coherent.min(ratio(1, 1)));
        let keep = (ratio(1, 1) - open(&x.key).min(ratio(1, 1))) * (ratio(1, 1) - d_o.clone());
        x.agitation = crate::ring::grain(&((x.agitation.clone() + heat) * keep.clone() * keep));
        // Spin cannot carry a mote through zero reach: a turning is a
        // magnitude in its plane; a kick past zero stops at the core.
        let new_o = crate::ring::grain(&(x.ell_o.clone() + dl_o)).max(ratio(0, 1));
        let m = if total.is_zero() { ratio(0, 1) } else { x.amount.clone() * weight(&x.key) / total.clone() };
        let dl = new_o.clone() - x.ell_o.clone();
        absorbed += m.clone() * if dl < ratio(0, 1) { -dl.clone() } else { dl.clone() };
        given += m * dl;
        x.ell_o = new_o;
    }
    (given, absorbed)
}

/// THE CONTACT OF A PAIR over one echo (operators 2026-10-18, with
/// xylarium: interaction is continuous, "not a walk, a dance"): the mean of
/// their orbit planes' in-step share while their phase difference runs from
/// `ψ0` at the rate `d` a whole echo. `in_step` is linear between its kinks
/// (`ψ` on the half-turns), so each piece's exact mean is the mean of its
/// ends: duration × ½(in-step at entry + at exit), no square. A pair at rest
/// in line has contact 1; a whole passage gives ½; a crossing is the
/// weight's peak, never the only moment they meet.
pub fn contact(psi0: &Ratio, d: &Ratio) -> Ratio {
    let f = |x: &Ratio| in_step(x, &ratio(0, 1));
    if d.is_zero() {
        return f(psi0);
    }
    let psi1 = psi0.clone() + d.clone();
    let (lo, hi) = if psi1 > *psi0 { (psi0.clone(), psi1) } else { (psi1, psi0.clone()) };
    let span = hi.clone() - lo.clone();
    // The half-turns strictly inside, in order.
    let mut cuts = vec![lo.clone()];
    let mut k = (lo.clone() * ratio(2, 1)).floor() + ratio(1, 1);
    while k.clone() / ratio(2, 1) < hi {
        cuts.push(k.clone() / ratio(2, 1));
        k += ratio(1, 1);
    }
    cuts.push(hi);
    cuts.windows(2).fold(ratio(0, 1), |acc, w| acc + (w[1].clone() - w[0].clone()) / span.clone() * (f(&w[0]) + f(&w[1])) / ratio(2, 1))
}

/// THE SHEAR (operator 2026-10-18: "shear the kinematic pressure as an
/// attractor on opposing hemispheres to provide the shear on the orbit …
/// this turns your 'loop' into an 'orbit'"; with xylarium): neighbouring
/// rings of the flow turn at different rates, and where they stand in
/// contact the pressure between them moves turning OUTWARD. The inner falls
/// in, the outer moves out, and the disc accretes, so inbound and outbound are
/// one closed orbit. Neighbours by spin across every face (faces decide who
/// belongs to a filament, never who meets), each pair weighed by its contact
/// over the bell at the kick's own rates (`2δ`, enclosed). The unkept motion
/// is `contact × T`, with `T` the deficits' contrast; gas has no grip, so it is
/// the tether law without the bond. The conserved spin exchange: the outer
/// gains, the inner gives, each by the other's mass share. All pairs are read
/// at once, then each mote's whole outflow is scaled by one k so it never
/// gives more than it carries (exact, no order). Returns the spin moved.
pub fn shear(motes: &mut [Mote], enc: &Enclosed, total: &Ratio) -> Ratio {
    let n = motes.len();
    if n < 2 || total.is_zero() {
        return ratio(0, 1);
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|a, b| motes[*a].ell_o.cmp(&motes[*b].ell_o).then(a.cmp(b)));
    // Shares on the coordinate's grain (the carrier): exact shares of a
    // large ledger carry its denominators into every pair's product.
    let m: Vec<Ratio> = motes.iter().map(|x| crate::ring::grain(&(x.amount.clone() * weight(&x.key) / total.clone()))).collect();
    let d: Vec<Ratio> = motes.iter().map(|x| enc.reach(&x.ell_o).1).collect();
    // (outer, inner, the outer's gain, the inner's give), all from the start.
    let mut moves: Vec<(usize, usize, Ratio, Ratio)> = Vec::new();
    let mut out = vec![ratio(0, 1); n];
    for w in order.windows(2) {
        let (n_, o) = (w[0], w[1]);
        let gap = motes[o].ell_o.clone() - motes[n_].ell_o.clone();
        let m_ij = m[o].clone() + m[n_].clone();
        if gap <= ratio(0, 1) || m_ij.is_zero() {
            continue;
        }
        let c = crate::ring::grain(&contact(&(motes[o].phase[0].clone() - motes[n_].phase[0].clone()), &(ratio(2, 1) * (d[o].clone() - d[n_].clone()))));
        let t = crate::ring::grain(&(if d[o] > d[n_] { d[o].clone() - d[n_].clone() } else { d[n_].clone() - d[o].clone() } / (d[o].clone() + d[n_].clone())));
        let u = crate::ring::grain(&(c * t * gap / m_ij));
        if u.is_zero() {
            continue;
        }
        // Both sides on the grain alike, so the exchange stays conserved:
        // m_o·gain = m_o·u·m_n = m_n·give.
        let give = u.clone() * m[o].clone();
        out[n_] += give.clone();
        moves.push((o, n_, u * m[n_].clone(), give));
    }
    let k: Vec<Ratio> = (0..n).map(|i| if out[i] > motes[i].ell_o && !out[i].is_zero() { motes[i].ell_o.clone() / out[i].clone() } else { ratio(1, 1) }).collect();
    let mut dl = vec![ratio(0, 1); n];
    let mut moved = ratio(0, 1);
    for (o, n_, gain, give) in moves {
        dl[o] += gain.clone() * k[n_].clone();
        dl[n_] -= give * k[n_].clone();
        moved += m[o].clone() * gain * k[n_].clone();
    }
    for i in 0..n {
        if !dl[i].is_zero() {
            motes[i].ell_o = crate::ring::grain(&(motes[i].ell_o.clone() + dl[i].clone())).max(ratio(0, 1));
        }
    }
    moved
}

/// THE SPRAY (with xylarium; the founding epoch one tier down): matter a
/// heart throws out (an outburst, or the kinds a new star's heart releases)
/// leaves TOGETHER, every kind at the mean carried reach (holding-weighted),
/// and over directions: each kind at its own capture phase, `Z·p` with p the
/// star's own push share (the founding's ½ one tier up), `½ + Z·p` on the loop. It
/// sorts only as it spreads, by each kind's own inertia under the star's
/// kick, so early meetings catch mixed matter.
pub fn spray(released: &Manifest, spins: &[(String, Ratio)], base: &[Ratio], agitation: &Ratio, born: u64, push: &Ratio) -> Vec<Mote> {
    let held = released.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * weight(k));
    let reach = if held.is_zero() {
        ratio(0, 1)
    } else {
        crate::ring::grain(&(released.iter().fold(ratio(0, 1), |s, (k, a)| s + a.clone() * weight(k) * spins.iter().find(|(kk, _)| kk == k).map(|(_, l)| l.clone()).unwrap_or_else(|| ratio(0, 1))) / held))
    };
    let at = |k: usize| base.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
    released
        .iter()
        .filter(|(_, a)| **a > ratio(0, 1))
        .map(|(k, a)| {
            // The capture phase: Z × the PARENT'S push share (the star's own),
            // so the kinds leave over the whole turn, never two antipodes.
            let z = ratio(i64::from(crate::chem::charge(k)), 1) * push.clone();
            let phase = vec![crate::ring::grain(&frac(&(at(0) + z.clone()))), crate::ring::grain(&frac(&(at(1) + ratio(1, 2) + z)))];
            Mote { key: k.clone(), amount: a.clone(), ell_o: reach.clone(), ell_c: ratio(0, 1), phase, agitation: crate::ring::grain(agitation), born, vacuum: false }
        })
        .collect()
}

/// THE STAR'S KICK ON ITS SYSTEM (with xylarium; the galaxy's law one tier
/// down): each echo the star's bell pushes its system's motes with ONE force,
/// its push on its open frames (`½ ×` the heart's open share), each kind
/// moving by its own inertia, and the star's hold returns the held
/// hemisphere, `½·δ(R)·ℓ` (`R = ℓ/H_s`).
pub fn star_kick(motes: &mut [Mote], heart: &Manifest, h_s: &Ratio) {
    let held = crate::pulse::mass(heart);
    if held.is_zero() || h_s.is_zero() {
        return;
    }
    let push = ratio(1, 2) * crate::pulse::magnetism(heart) / held;
    for x in motes.iter_mut() {
        let r = x.ell_o.clone() / h_s.clone();
        let d = h_s.clone() / (r + h_s.clone());
        x.ell_o = crate::ring::grain(&(x.ell_o.clone() + push.clone() / weight(&x.key) - ratio(1, 2) * d * x.ell_o.clone())).max(ratio(0, 1));
    }
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
    // A group judged whole by the filament law, edge to edge.
    let judge = |members: Vec<usize>| -> Option<Chain> {
        let (mut x, mut held, mut a_w, mut r_w) = (ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1));
        for &j in &members {
            let m = motes[j].amount.clone() * weight(&motes[j].key) / total.clone();
            x += m.clone() * (ratio(1, 1) + open(&motes[j].key));
            a_w += m.clone() * motes[j].agitation.clone();
            r_w += m.clone() * reach(&motes[j]);
            held += m;
        }
        if held.is_zero() {
            return None;
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
        // A chain with no extent at all (no push to spread it) is not read as
        // infinitely dense: it does not bind.
        let bind = if apart.is_zero() { ratio(0, 1) } else { let lambda = x / apart.clone(); lambda.clone() / (ratio(1, 1) + lambda) };
        Some(Chain { members, reach: reach_w, bind, agitation: a_w / held, face, apart })
    };
    // The hold along a group by reach: each member spread over its own band,
    // the hold at a place the sum of every member whose band covers it.
    let profile = |along: &[usize]| -> Vec<Ratio> {
        let density: Vec<Ratio> = along
            .iter()
            .map(|&j| {
                let (d0, d1) = (delta(&lo(&motes[j])), delta(&hi(&motes[j])));
                let band = (d0.clone() - d1.clone()) / (d0 + d1);
                let m = motes[j].amount.clone() * weight(&motes[j].key) / total.clone();
                if band.is_zero() { m } else { m / band }
            })
            .collect();
        along
            .iter()
            .map(|&i| {
                let at = reach(&motes[i]);
                along.iter().zip(&density).filter(|(&j, _)| lo(&motes[j]) <= at && at <= hi(&motes[j])).fold(ratio(0, 1), |s, (_, d)| s + d.clone())
            })
            .collect()
    };
    // THE FIBRES (operator 2026-10-18: "you bead the individual fibers and
    // then braid the fibers together"): a fibre is one kind's run on one
    // face, motes whose dispersal bands overlap, linked transitively.
    let mut fibres: std::collections::BTreeMap<(Vec<bool>, String), Vec<usize>> = std::collections::BTreeMap::new();
    for (i, x) in motes.iter().enumerate() {
        if on_disc_leg(x) && !reach(x).is_zero() {
            fibres.entry(((0..PLANES).map(|k| frac(&x.phase[k]) < half).collect(), x.key.clone())).or_default().push(i);
        }
    }
    // Each fibre's beads, per face: (members, the pinch's reach).
    let mut beads_by_face: std::collections::BTreeMap<Vec<bool>, Vec<(Vec<usize>, Ratio)>> = std::collections::BTreeMap::new();
    for ((face, _), mut idx) in fibres {
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
        let out = beads_by_face.entry(face).or_default();
        for run in runs {
            let mut along = run;
            along.sort_by(|a, b| reach(&motes[*a]).cmp(&reach(&motes[*b])).then(a.cmp(b)));
            let local = profile(&along);
            // THE BEADS (operator: "halving is a human derived measure,
            // whereas anything in nature is measured itself"): a fibre
            // pinches where its hold gathers (each local maximum) and parts
            // at the minima between, the minimum member joining the side
            // whose neighbour holds more.
            let mut cuts: Vec<usize> = Vec::new();
            for i in 1..along.len().saturating_sub(1) {
                if local[i] < local[i - 1] && local[i] <= local[i + 1] {
                    cuts.push(if local[i - 1] >= local[i + 1] { i + 1 } else { i });
                }
            }
            let mut from = 0;
            let mut push = |a: usize, b: usize| {
                let (k, _) = (a..b).map(|i| (i, local[i].clone())).max_by(|x, y| x.1.cmp(&y.1).then(y.0.cmp(&x.0))).expect("a member");
                out.push((along[a..b].to_vec(), reach(&motes[along[k]])));
            };
            for at in cuts {
                if at > from && at < along.len() {
                    push(from, at);
                    from = at;
                }
            }
            push(from, along.len());
        }
    }
    // THE BRAID: beads of different fibres twine where each one's pinch
    // stands within the other's extent (mutual, so no order), and the
    // twining carries across; each braid is read whole by the filament law.
    let mut out = Vec::new();
    for (_, beads) in beads_by_face {
        let n = beads.len();
        let span: Vec<(Ratio, Ratio)> = beads.iter().map(|(m, _)| (m.iter().map(|&j| lo(&motes[j])).min().expect("a member"), m.iter().map(|&j| hi(&motes[j])).max().expect("a member"))).collect();
        let mut parent: Vec<usize> = (0..n).collect();
        fn root(p: &mut [usize], i: usize) -> usize {
            let mut i = i;
            while p[i] != i {
                p[i] = p[p[i]];
                i = p[i];
            }
            i
        }
        // Beads sorted by pinch; a pinch can lie within another's extent only
        // where that extent reaches it, so each bead reads forward until the
        // pinches pass its outer edge.
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|a, b| beads[*a].1.cmp(&beads[*b].1).then(a.cmp(b)));
        for (x, &a) in order.iter().enumerate() {
            for &b in &order[x + 1..] {
                if beads[b].1 > span[a].1 {
                    break;
                }
                if span[b].0 <= beads[a].1 && beads[a].1 <= span[b].1 && span[a].0 <= beads[b].1 {
                    let (ra, rb) = (root(&mut parent, a), root(&mut parent, b));
                    if ra != rb {
                        parent[ra] = rb;
                    }
                }
            }
        }
        let mut braids: std::collections::BTreeMap<usize, Vec<usize>> = std::collections::BTreeMap::new();
        for i in 0..n {
            let r = root(&mut parent, i);
            braids.entry(r).or_default().extend(beads[i].0.iter().copied());
        }
        out.extend(braids.into_values().filter_map(|m| judge(m)));
    }
    out
}

/// THE CLUMPS, where the motes are: every chain that holds collapses, each
/// member taking its filament's coherence share (a chain is a partition, so
/// nothing is claimed twice). Returns, per holding chain, its reach and the
/// share of each mote it takes.
pub fn clumps(motes: &[Mote], h: &Ratio, p: &Ratio, total: &Ratio) -> Vec<(Ratio, Vec<(usize, Ratio)>)> {
    clumps_of(motes, chains(motes, h, p, total))
}

/// STREAMING BY FORCE (operator 2026-10-18: "jetstreams depending on their
/// force are either continuums or vacuums in the filament … analogous to the
/// gulf stream"): a mote's stream is READ, never labelled by its origin.
/// Where the jet's push at it (`p·ρ·δ_u`, the kick's own reading) beats the
/// hold of the filament it stands in (`λ/(1 + λ)`), it is carried in the
/// CONTINUUM, one moving body with the current; where the hold beats the
/// push, the jet cannot fill the channel and it drifts in the VACUUM, the
/// thin counter-moving flank. A mote in no filament (in transit on the
/// axial leg) is carried. Returns how many stand in the vacuum.
pub fn stream(motes: &mut [Mote], chains: &[Chain], push: &Ratio) -> usize {
    let mut hold: Vec<Option<Ratio>> = vec![None; motes.len()];
    for c in chains {
        for &j in &c.members {
            hold[j] = Some(c.bind.clone());
        }
    }
    let mut vacuum = 0;
    for (x, h) in motes.iter_mut().zip(hold) {
        x.vacuum = h.is_some_and(|h| h >= *push);
        vacuum += usize::from(x.vacuum);
    }
    vacuum
}

/// The clumps of chains already read: those that hold collapse.
pub fn clumps_of(motes: &[Mote], chains: Vec<Chain>) -> Vec<(Ratio, Vec<(usize, Ratio)>)> {
    chains
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
    // same kind, same stream, the same mark in every phase (60 to the turn)
    // and in the reach's share `ℓ/(ℓ + 1)` (360 marks). At the 60⁻⁶ grain
    // motes almost never coincide and nothing bounds their number; the
    // readings must not move with the marks (A/B, with xylarium). Holding
    // sums exactly; spins and agitation merge by holding onto the grid, the
    // floored spin's remainder returned for the core's reservoir so the spin
    // ledger stays exact.
    let g = |x: &Ratio| crate::ring::grain(x);
    let (phase_marks, reach_marks) = std::env::var("NS_MERGE_MARKS").ok().and_then(|v| v.split_once('/').and_then(|(a, b)| Some((a.parse::<i64>().ok()?, b.parse::<i64>().ok()?)))).unwrap_or((60, 360));
    let mark = |x: &Ratio, n: i64| (x.clone() * ratio(n, 1)).floor();
    let key = |y: &Mote| (y.key.clone(), y.vacuum, mark(&(y.ell_o.clone() / (y.ell_o.clone() + ratio(1, 1))), reach_marks), y.phase.iter().map(|ph| mark(&frac(ph), phase_marks)).collect::<Vec<_>>());
    let mut out: Vec<Mote> = Vec::new();
    // Each mote's sector key read once, and found by key, never by a scan
    // of everything kept (each group keyed where its first mote stood).
    let mut at: std::collections::BTreeMap<(String, bool, Ratio, Vec<Ratio>), usize> = std::collections::BTreeMap::new();
    let mut remainder = ratio(0, 1);
    for x in std::mem::take(motes) {
        let kx = key(&x);
        if let Some(y) = at.get(&kx).map(|&i| &mut out[i]) {
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
            at.insert(kx, out.len());
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
            let turn = ratio(i64::from(crate::chem::charge(k)), 2);
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

#[cfg(test)]
mod passage_tests {
    use super::mean_response;
    use cfe::ratio::ratio;

    #[test]
    fn the_shear_moves_turning_outward_and_keeps_the_books() {
        use super::{shear, weight, Enclosed, Mote};
        let mote = |ell: (i64, i64), ph: (i64, i64)| Mote { key: "Z001".into(), amount: ratio(1, 8), ell_o: ratio(ell.0, ell.1), ell_c: ratio(0, 1), phase: vec![ratio(ph.0, ph.1), ratio(0, 1)], agitation: ratio(0, 1), born: 0, vacuum: false };
        let mut motes = vec![mote((1, 4), (0, 1)), mote((1, 2), (1, 60)), mote((3, 4), (1, 30))];
        let total = ratio(1, 1);
        let book = |ms: &[Mote]| ms.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * weight(&x.key) * x.ell_o.clone());
        let before = book(&motes);
        let (inner, outer) = (motes[0].ell_o.clone(), motes[2].ell_o.clone());
        let moved = shear(&mut motes, &Enclosed::new(ratio(1, 1), Vec::new()), &total);
        assert!(moved > ratio(0, 1));
        // The inner falls in, the outer moves out.
        assert!(motes[0].ell_o < inner && motes[2].ell_o > outer);
        // Σ m·ℓ within the grain (each spin lands on the coordinate's grain).
        let drift = book(&motes) - before;
        let drift = if drift < ratio(0, 1) { -drift } else { drift };
        assert!(drift <= ratio(3, 1) / ratio(60, 1).pow(6));
    }

    #[test]
    fn a_whole_turn_averages_nothing_and_half_turns_sign_the_hemisphere() {
        assert_eq!(mean_response(&ratio(1, 3), &ratio(1, 1)), ratio(0, 1));
        assert_eq!(mean_response(&ratio(0, 1), &ratio(1, 2)), ratio(1, 2));
        assert_eq!(mean_response(&ratio(1, 2), &ratio(1, 2)), ratio(-1, 2));
        // Across the bell: from ¾ to 1¼, the loss then the gain cancel.
        assert_eq!(mean_response(&ratio(3, 4), &ratio(1, 2)), ratio(0, 1));
    }
}
