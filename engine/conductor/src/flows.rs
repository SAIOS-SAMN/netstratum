//! THE FLOWS (docs/83; xylarium 11.0 "NEVER PLACE MATTER - THE FLOWS",
//! agreed with xylarium 2026-10-16). Operator: "let the formulas place where
//! the matter goes and how it gets there and what it does after it gets
//! there." Matter is held turning: it moves only as its STATE changes, by
//! laws acting on that state; where it gathers, what is held and what is
//! gas, where it collapses, are READINGS of the state, never placements.
//!
//! - **A mote's state**, carried forward, never rebuilt: its kind and
//!   amount, its spin in the orbit's plane `ℓ_o` (reach `R = ℓ_o/H`, the
//!   motion law) and in the circulation's plane `ℓ_⊥` (radial–axial, along
//!   the torsion axis), its PHASES on its enclosure's S^n (one per plane,
//!   turns against the bell), its agitation and birth.
//! - **The torque**, once per bell of the torquing core (the core's bell is its
//!   bounce, once an echo it regenerates), every mote, held or not: per
//!   plane, the HEMISPHERE the mote's phase stands in against the bell
//!   signs it (the S⁰ face; operator: coherence is a count over the S^n
//!   hemispheres, in the sphere's own measure) and S¹'s own LINEAR measure in
//!   turns sizes it, `1 − 2·|ψ|` (ψ the phase against the bell, |ψ| its
//!   distance on the circle); times κ, the push's share gripping the mote
//!   by its core's hold at its reach: `κ_o = p·δ(R)` for the own family in
//!   the orbit's plane, `κ_⊥ = p·ρ·δ_u` for the parent's (the universal bell
//!   received at ρ) in the circulation's plane. Off resonance the signs
//!   alternate and cancel; on resonance they add: the cyclotron.
//! - **The spin ledger**: every Δℓ given to a mote of holding m is −m·Δℓ to
//!   its core's turning reservoir, so `Σ m·ℓ + reservoir` is exact.
//! - **The cut is a reading**: a mote's held share is the part of its
//!   dispersal band (`R(1 ± p)`) within the throat, in both planes,
//!   `f_o · f_c`; the rest is the galaxy's gas.
//! - **Two exits, by state**: the JET, `ℓ_⊥` carrying the axial reading past
//!   the throat (`ℓ_⊥/H ≥ H`), along the torsion axis; RADIAL ESCAPE, `R ≥ H`
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
    pub key: crate::chem::Kind,
    /// Its amount on the ledger (grains of its kind).
    pub amount: Ratio,
    /// Spin in the orbit's plane, own units: reach `ℓ_o/H`.
    pub ell_o: Ratio,
    /// Spin in the circulation's plane (radial–axial, the torsion axis).
    pub ell_orth: Ratio,
    /// Its phase in each plane against the bell, in turns `[0, 1)`.
    pub phase: Vec<Ratio>,
    pub agitation: Ratio,
    pub born: u64,
    /// Its STREAM KIND, inherited from where it entered and carried
    /// forward, never reassigned: false the continuum (the core's founding
    /// outflow, a star's outbursts and wind), true the vacuum (ocean
    /// arrivals).
    pub vacuum: bool,
    /// THE LEVER (docs/85 §9): per plane, the whole nuclei of its holding
    /// (signed, in grains) standing one PLACE past its mark, so its holding's
    /// centre is its true position: `phase + lever/amount · place`. Never
    /// more than half its amount. Empty where nothing stands past.
    pub lever: Vec<Ratio>,
    /// The place its lever is counted in (a share of a turn).
    pub lever_place: Ratio,
}

/// A mote's TRUE phase in plane k (ascent): its mark and the share of its
/// holding standing one place past it.
pub fn true_phase(x: &Mote, k: usize) -> Ratio {
    let at = x.phase.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
    match x.lever.get(k) {
        Some(l) if !l.is_zero() && !x.amount.is_zero() => at + l.clone() / x.amount.clone() * x.lever_place.clone(),
        _ => at,
    }
}

/// SETTLE a mote's levers from this bell's (descent): each plane's lever
/// `(next mark, share f)` stands `f` of its holding one place past its mark,
/// in whole nuclei (the rest below one nucleus acknowledged, the quantum
/// tier); a plane with none stands whole on its mark.
pub fn settle(x: &mut Mote, levers: &[(usize, Ratio, Ratio)], place: Option<&Ratio>, cross: &mut crate::enfold::Cross) {
    let planes = x.phase.len();
    x.lever = vec![ratio(0, 1); planes];
    x.lever_place = place.cloned().unwrap_or_else(|| ratio(0, 1));
    for (k, next, f) in levers {
        let Some(at) = x.phase.get(*k) else { continue };
        // The sense: forward where the next mark lies ahead of this one.
        let ahead = next.clone() - at.clone();
        let ahead = ahead.clone() - ahead.floor();
        let forward = ahead < ratio(1, 2);
        let exact = x.amount.clone() * f.clone();
        let whole = crate::pulse::nucleus(&exact).min(x.amount.clone());
        let below = exact - whole.clone();
        if !below.is_zero() {
            cross.note(*k, crate::enfold::Fate::Acknowledged, &below);
        }
        x.lever[*k] = if forward { whole } else { -whole };
    }
    if x.lever.iter().all(|l| l.is_zero()) {
        x.lever = Vec::new();
    }
}

/// A kind's weight (its hold).
pub fn weight(k: &crate::chem::Kind) -> Ratio {
    ratio(i64::from(crate::chem::hold(k)), 1)
}

/// A kind's open share, unpaired over hold.
pub fn open(k: &crate::chem::Kind) -> Ratio {
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

/// THE TORQUE'S RESPONSE in one plane: the hemisphere signs it (the first half
/// of the circle against the bell gains, the second loses), S¹'s linear
/// measure sizes it, `1 − 2|ψ|`.
pub fn response(psi: &Ratio) -> Ratio {
    let f = frac(psi);
    let share = ratio(1, 1) - ratio(2, 1) * off(&f);
    if f < ratio(1, 2) { share } else { -share }
}

/// THE TORQUE'S RESPONSE OVER ITS PASSAGE (with xylarium: a torque sampled at
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
        let e3 = cga::base_axis(3)?;
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
    held_share(&mote.ell_o, h, p) * held_share(&mote.ell_orth, h, p)
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
        // bound, and every reach, contact and torque inherits them.
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

    /// `H(<r)` by PLACE: the base and every body whose own reach stands
    /// within `r`, the hold a body standing at reach r turns about (the ring
    /// law's containment read by place, for a body known by where it stands).
    pub fn hold_within(&self, r: &Ratio) -> Ratio {
        let mut h = self.held[0].clone();
        for (i, l) in self.spins.iter().enumerate() {
            let at = l.clone() / self.held[i].clone();
            if at < *r {
                h += self.held[i + 1].clone() - self.held[i].clone();
            }
        }
        h
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
/// the disc"): the route round the orthogonal orbit, not the inversion. A vacuum mote's
/// orthogonal orbit (the circulation's plane) runs the other way, its advance read over
/// that passage; its orbit plane turns forward.
#[allow(clippy::too_many_arguments)]
pub fn turn(phase: &mut [Ratio], r_o: &Ratio, h: &Ratio, p: &Ratio, tide: Option<(&Ratio, &Ratio)>, inverted: bool, place: Option<&Ratio>, agitation: &Ratio, cross: &mut crate::enfold::Cross) -> (Ratio, Vec<(usize, Ratio, Ratio)>, Ratio) {
    let pull = h.clone() / (r_o.clone() + h.clone());
    let d_o = if inverted { ratio(1, 1) - pull.clone() } else { pull.clone() };
    // THE ORTHOGONAL ORBIT'S RATE (with xylarium): the jetstream turns by the motion law
    // AT THE MOTE'S REACH, δ(R) a ring, two rings a bell, continuously;
    // inner matter circulates faster than outer. (At R = H it is one turn a
    // bell, a 1:1 lock with the bell: the rim.)
    let d_c = d_o.clone();
    // Through the cross (`enfold`, docs/85 §8): held to the body's own place,
    // since exact sums of exact rationals otherwise compound their
    // denominators each bell without bound; the residue drains within its
    // round, passes to the quantum tier acknowledged, or, with no place of its
    // own, is cauterized with its scar recorded.
    let (held, scar_o, lever_o) = crate::enfold::leaf(&(phase[0].clone() + ratio(2, 1) * d_o), place, agitation, 0, cross);
    phase[0] = held;
    let orth_from = phase[1].clone();
    let route = if inverted { ratio(-1, 1) } else { ratio(1, 1) };
    let orth_phase = frac(&(orth_from.clone() + route.clone() * ratio(2, 1) * d_c.clone()));
    // THE PARENT'S PULSE MOVES MATTER ALONG THE ORTHOGONAL ORBIT (operator: "the pulse
    // moves matter along the orthogonal orbit"; with xylarium): absorbed in step it
    // ADVANCES the orthogonal orbit's phase, `p·ρ·δ_u` × the hemisphere's signed share,
    // on top of the orthogonal orbit's own rate. AND IT IS A SPIN (docs/90 D1, operator
    // 2026-10-22; xylarium 11.0: "the loop is an orbit", one torque law per
    // plane): the same push, read by the one torque law in the orthogonal orbit's plane,
    // is the orthogonal spin's torque, returned to the caller (`κ_⊥`).
    // The pulse it feels: a galaxy's received share of the source's rings and
    // the universal tide at the galaxy (`tide`), or, standing in the source's
    // own enclosure (the home, the firmament, a star's own system), the
    // source's pull at its own reach, with no receiver between.
    let felt = match tide {
        Some((rho, delta_u)) => rho.clone() * delta_u.clone(),
        None => pull.clone(),
    };
    let advance = orth_push(&orth_from, &d_c, p, &felt, inverted);
    let torque_orth = route.clone() * advance.clone();
    let (held, scar_c, lever_c) = crate::enfold::leaf(&(orth_phase + route * advance), place, agitation, 1, cross);
    phase[1] = held;
    // A cauterized residue's scar (the heat it enters) and each plane's lever.
    let levers = [(0usize, lever_o), (1usize, lever_c)].into_iter().filter_map(|(k, l)| l.map(|(n, f)| (k, n, f))).collect();
    (scar_o + scar_c, levers, torque_orth)
}

/// THE PARENT'S PUSH ALONG THE ORTHOGONAL ORBIT over one bell's passage
/// (docs/90 D1): `p·felt` × the hemisphere's signed mean over the arc the
/// orthogonal phase sweeps, `2·δ` from where it stood (a vacuum mote's route
/// runs the other way, its passage read backward). Unsigned by the route:
/// the caller signs it.
pub fn orth_push(orth_from: &Ratio, d_c: &Ratio, p: &Ratio, felt: &Ratio, inverted: bool) -> Ratio {
    let start = if inverted { orth_from.clone() - ratio(2, 1) * d_c.clone() } else { orth_from.clone() };
    p.clone() * felt.clone() * mean_response(&start, &(ratio(2, 1) * d_c.clone()))
}

/// THROUGH THE CENTRE (the torque's law, docs/83; with xylarium): a spin
/// carried past zero is a turning in the other sense, so the body passes
/// through the centre to the antipode, its orbit phase half a turn on, its
/// spin the magnitude, never piled at zero.
pub fn through_centre(ell: Ratio, phase: &mut [Ratio]) -> Ratio {
    if ell < ratio(0, 1) {
        if let Some(ph) = phase.get_mut(0) {
            *ph = frac(&(ph.clone() + ratio(1, 2)));
        }
        -ell
    } else {
        ell
    }
}

/// THE SWING (docs/90 D2): a body's orthogonal spin projects onto its orbit by
/// where it stands on its orthogonal orbit, the plane's linear measure (`response`): its
/// spin seen in the orbit plane is `ℓ + ℓ_⊥·response(ψ_⊥)`, so its reach swings
/// each orthogonal turn between `(ℓ − |ℓ_⊥|)/H` and `(ℓ + |ℓ_⊥|)/H`, an elongation
/// `e = |ℓ_⊥|/ℓ` read from two planes, no ellipse and no square. Where the
/// swing passes zero the body passes THROUGH the centre to the antipode (the
/// orbit-plane torque's law, docs/83): returns the magnitude and whether it is
/// through (its orbit phase then stands half a turn on).
pub fn swung(ell: &Ratio, ell_orth: &Ratio, psi_c: &Ratio) -> (Ratio, bool) {
    let s = ell.clone() + ell_orth.clone() * response(psi_c);
    if s < ratio(0, 1) {
        (-s, true)
    } else {
        (s, false)
    }
}

/// One bell's torques on every mote: the own family in the orbit's plane
/// (`κ_o = p·δ(R)`, a spin); the parent's moves the mote along the orthogonal orbit
/// (`p·ρ·δ_u`, a phase).
/// Each plane's phase first advances by its own turning over the bell's two
/// rings (`2·δ` turns, the motion law's `ω = δ`; the bell's own turn is
/// whole, so the phase against it moves by `2δ` mod 1). Returns the spin
/// the core's reservoir gives (Σ m·Δℓ, to be taken from it), and the pulse
/// the matter ABSORBED (Σ m·|Δℓ|, unsigned: each hemisphere's torque alike;
/// with xylarium), what the recycling budget is read against.
#[allow(clippy::too_many_arguments)]
pub fn torque(motes: &mut [Mote], enc: &Enclosed, p: &Ratio, tide: Option<(&Ratio, &Ratio)>, total: &Ratio, heart_push: Option<&Ratio>, cross: &mut crate::enfold::Cross) -> (Ratio, Ratio) {
    let mut given = ratio(0, 1);
    let mut absorbed = ratio(0, 1);
    for x in motes.iter_mut() {
        // Each turns by what is enclosed inside it (the core and all inside),
        // at the reach its orthogonal orbit swings it to this bell (docs/90 D2): the
        // motion law at its present reach, so it turns faster inward and
        // slower outward; through the centre where the swing passes zero.
        let (now, through_c) = swung(&x.ell_o, &x.ell_orth, x.phase.get(1).unwrap_or(&ratio(0, 1)));
        let h = enc.hold(&now);
        let (r_o, d_o) = enc.reach(&now);
        // ASCENT: the bell turns the mote from where it truly stands, its
        // lever read back onto its marks.
        for k in 0..x.phase.len() {
            x.phase[k] = true_phase(x, k);
        }
        let from = if through_c { frac(&(x.phase[0].clone() + ratio(1, 2))) } else { x.phase[0].clone() };
        // A mote's place: the cap its own gravity holds (`enfold::place`).
        let m = if total.is_zero() { ratio(0, 1) } else { x.amount.clone() * weight(&x.key) / total.clone() };
        let place = crate::enfold::place(&r_o, &m, &h);
        let heat_was = x.agitation.clone();
        let (scar, levers, torque_orth) = turn(&mut x.phase, &r_o, &h, p, tide, x.vacuum, place.as_ref(), &heat_was, cross);
        let (g, ab) = spin_torque(x, &from, &h, &r_o, &d_o, p, &torque_orth, heart_push, &m, &scar);
        given += g;
        absorbed += ab;
        // DESCENT: its weight past its round stands one place past its mark.
        settle(x, &levers, place.as_ref(), cross);
    }
    (given, absorbed)
}

/// THE TORQUE ON ONE MOTE'S SPINS over one bell's passage, the one law at
/// every tier (a galaxy's core on its motes, a star on its system's): from
/// the orbit phase it stood at (`from`) and the reach and deficit it turns
/// at (`r_o`, `d_o`, inside the hold `h`), the own family's push in the orbit
/// plane, the band's push and return (`heart_push`), the parent's push along
/// the orthogonal orbit (`torque_orth`) less the hold's return, and what is
/// not delivered in step left as heat. Returns the spin given (Σ m·Δℓ, the
/// antipole's to pay) and the pulse absorbed (Σ m·|Δℓ|).
#[allow(clippy::too_many_arguments)]
pub fn spin_torque(x: &mut Mote, from: &Ratio, h: &Ratio, r_o: &Ratio, d_o: &Ratio, p: &Ratio, torque_orth: &Ratio, heart_push: Option<&Ratio>, m: &Ratio, scar: &Ratio) -> (Ratio, Ratio) {
    let mut given = ratio(0, 1);
    let mut absorbed = ratio(0, 1);
    // Over its whole passage this bell, never at one instant; a vacuum
    // mote at its inverted reach (`1 − δ`), forward.
    let d_o = if x.vacuum { ratio(1, 1) - d_o.clone() } else { d_o.clone() };
    let k = mean_response(from, &(ratio(2, 1) * d_o.clone()));
    let dl_o = p.clone() * d_o.clone() * k.clone();
    // DISSIPATION (operator 2026-10-19: "what do our current physics tell
    // about heat dissipation to the void? probably mimic nature"; with
    // xylarium): what the torque does not deliver in step becomes HEAT,
    // `p·δ·(1 − |k|)` (k its signed mean over the passage, the coherent
    // share), and heat leaves by the built cooling law's two channels
    // over the passage's two rings: the light through its open frames
    // (`e`, open over held, a ring) and the vacuum's pull back (`δ` at its
    // reach, a ring): `a ← (a + heat)·((1 − e)(1 − δ))²`.
    let coherent = if k < ratio(0, 1) { -k } else { k };
    let heat = p.clone() * d_o.clone() * (ratio(1, 1) - coherent.min(ratio(1, 1)));
    let keep = (ratio(1, 1) - open(&x.key).min(ratio(1, 1))) * (ratio(1, 1) - d_o.clone());
    x.agitation = crate::ring::grain(&((x.agitation.clone() + heat) * keep.clone() * keep + scar.clone()));
    // THE BAND (one grammar: the firmament's, a galaxy's, a star's): the core's bounce pushes each kind out, `q = push/w`
    // (`push = p·w̄`, the dispersal law's, as at the founding and each
    // arrival), and its hold returns `½·δ(R)·ℓ` (δ against what is enclosed
    // at its reach), so the continuum rests in a band. The vacuum feels the
    // same law reflected through the THROAT, which stands at the core's
    // state, R = 1 (the enclosed law's base: throat 1, rim 2 in the
    // enclosure's own units): `R → 2 − R` (`through`). Past the rim the
    // vacuum stands outside this sphere.
    let throat = ratio(1, 1);
    let band = match heart_push {
        Some(push) if !h.is_zero() => {
            let q = push.clone() / weight(&x.key);
            let at = |reach: &Ratio| q.clone() - ratio(1, 2) * (h.clone() / (reach.clone() + h.clone())) * reach.clone() * h.clone();
            if !x.vacuum {
                at(r_o)
            } else {
                let mirrored = through(r_o, &throat);
                if mirrored > ratio(0, 1) { -at(&mirrored) } else { ratio(0, 1) }
            }
        }
        _ => ratio(0, 1),
    };
    // A torque past the centre carries the mote THROUGH it to the antipode
    // (its orbit's phase on half a turn), never piled at zero.
    let raw = x.ell_o.clone() + dl_o + band;
    let new_o = if raw < ratio(0, 1) {
        if let Some(ph) = x.phase.get_mut(0) {
            let t = ph.clone() + ratio(1, 2);
            *ph = t.clone() - t.floor();
        }
        crate::ring::grain(&(-raw))
    } else {
        crate::ring::grain(&raw)
    };
    let dl = new_o.clone() - x.ell_o.clone();
    absorbed += m.clone() * if dl < ratio(0, 1) { -dl.clone() } else { dl.clone() };
    given += m.clone() * dl;
    x.ell_o = new_o;
    // THE ORTHOGONAL SPIN (docs/90 D1): the release's torque in the orthogonal orbit's plane,
    // less the hold's return of the held half, `½·δ(R)·ℓ_⊥`, both from and
    // to the same reservoir as the orbit plane's (Σ m·(ℓ_o + ℓ_⊥) and the
    // reservoir exact).
    let new_c = crate::ring::grain(&(x.ell_orth.clone() + torque_orth.clone() - ratio(1, 2) * d_o.clone() * x.ell_orth.clone()));
    let dl_c = new_c.clone() - x.ell_orth.clone();
    absorbed += m.clone() * if dl_c < ratio(0, 1) { -dl_c.clone() } else { dl_c.clone() };
    given += m.clone() * dl_c;
    x.ell_orth = new_c;
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
/// this turns your 'orthogonal orbit' into an 'orbit'"; with xylarium): neighbouring
/// rings of the flow turn at different rates, and where they stand in
/// contact the pressure between them moves turning OUTWARD. The inner falls
/// in, the outer moves out, and the disc accretes, so inbound and outbound are
/// one closed orbit. Neighbours by spin across every face (faces decide who
/// belongs to a filament, never who meets), each pair weighed by its contact
/// over the bell at the torque's own rates (`2δ`, enclosed). The unkept motion
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
            let x = &mut motes[i];
            let now = crate::ring::grain(&(x.ell_o.clone() + dl[i].clone()));
            x.ell_o = through_centre(now, &mut x.phase);
        }
    }
    moved
}

/// THE THROAT'S INVERSION on the sphere (operator 2026-10-21: "you are
/// flattening the second double inversion"): with the polar angle `θ =
/// 90°·R/H` (the throat at 90°, where δ = ½; the rim at 180°), inverting
/// through the throat reflects `θ → 180° − θ`, so `R → 2H − R`: the core and
/// the rim change places, the throat stands. Linear, exact, no square.
pub fn through(r: &Ratio, h: &Ratio) -> Ratio {
    ratio(2, 1) * h.clone() - r.clone()
}

/// THE SPRAY (with xylarium; the founding epoch one tier down): matter a
/// heart throws out (an outburst, or the kinds a new star's heart releases)
/// leaves over directions, each kind at its own capture phase, `Z·p` with p
/// the star's own push share (the founding's ½ one tier up), `½ + Z·p` on the
/// orthogonal orbit. THROWN, NEVER PLACED (operator 2026-10-22: "gravity and
/// physics should supply the required motion"): each kind leaves at the spin
/// the heart's cut carried it with (`carried`, in its system's units), which
/// already stands past the throat (the cut released it where its reach
/// outran the heart's hold, the double inversion's far face, operator
/// 2026-10-21); its system's enclosure reads its reach, and the star's torque
/// and hold settle it bell by bell, by its own weight.
pub fn spray(released: &Manifest, carried: &[(crate::chem::Kind, Ratio)], base: &[Ratio], agitation: &Ratio, born: u64, push: &Ratio) -> Vec<Mote> {
    let at = |k: usize| base.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
    released
        .iter()
        .filter(|(_, a)| **a > ratio(0, 1))
        .filter_map(|(k, a)| {
            // The capture phase: Z × the PARENT'S push share (the star's own),
            // so the kinds leave over the whole turn, never two antipodes.
            let z = ratio(i64::from(crate::chem::charge(k)), 1) * push.clone();
            let phase = vec![crate::ring::grain(&frac(&(at(0) + z.clone()))), crate::ring::grain(&frac(&(at(1) + ratio(1, 2) + z)))];
            let ell_o = crate::ring::grain(&carried.iter().find(|(c, _)| c == k)?.1);
            Some(Mote { key: k.clone(), amount: a.clone(), ell_o, ell_orth: ratio(0, 1), phase, agitation: crate::ring::grain(agitation), born, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1) })
        })
        .collect()
}

/// THE JETSTREAM IS A CONTINUOUS ORBIT, the orthogonal orbit (operators 2026-10-16/17: "a
/// continuous flow, inbound and outbound"): the circulation's plane IS the
/// orthogonal orbit, a mote's phase in it its place on the orthogonal orbit. The AXIAL leg (out
/// along the torsion axis and over: phase in `[½, 1)`) carries matter in
/// transit; the DISC leg (down at the rim and home through the disc: phase
/// in `[0, ½)`) is where the cut and collapse are read. No jet batch, no jet
/// exit: matter goes round, out and back at once.
pub fn on_disc_leg(x: &Mote) -> bool {
    frac(&x.phase[1]) < ratio(1, 2)
}

/// The one exit, by state: a mote whose orthogonal orbit OVERREACHES THE RIM (its
/// reach at or past the throat, `R ≥ H`, read by the one enclosure: `ℓ ≥
/// H(<ℓ)²`, δ at most ½) crosses whole into the medium one tier up, where the
/// enclosing core's orthogonal orbits carry it round the same way. Returns
/// the motes that left; the rest stay.
pub fn exits(motes: &mut Vec<Mote>, enc: &Enclosed) -> Vec<Mote> {
    let (gone, stay): (Vec<Mote>, Vec<Mote>) = std::mem::take(motes).into_iter().partition(|x| { let h = enc.hold(&x.ell_o); x.ell_o >= h.clone() * h });
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
/// The work a read spends, counted in its own steps (a comparison, an
/// addition): cost is read at N and 4N, never timed (MU-RECURSION rule 13).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Work(pub u64);

/// THE HOLD ALONG A FIBRE: at each member's reach, the sum of the densities
/// of the bands that cover it, `lo ≤ r ≤ hi`. Read as one sweep along the
/// reach, band starts added and band ends taken away, never member against
/// member. `reach[n]` and `bands[n] = (lo, hi, density)` describe member n.
pub fn hold_along(reach: &[Ratio], bands: &[(Ratio, Ratio, Ratio)], work: &mut Work) -> Vec<Ratio> {
    let n = reach.len().min(bands.len());
    let mut by = |v: &mut Vec<(Ratio, usize)>| {
        v.sort_by(|a, b| {
            work.0 += 1;
            a.cmp(b)
        })
    };
    let mut starts: Vec<(Ratio, usize)> = (0..n).map(|i| (bands[i].0.clone(), i)).collect();
    let mut ends: Vec<(Ratio, usize)> = (0..n).map(|i| (bands[i].1.clone(), i)).collect();
    let mut at: Vec<(Ratio, usize)> = (0..n).map(|i| (reach[i].clone(), i)).collect();
    by(&mut starts);
    by(&mut ends);
    by(&mut at);
    let (mut si, mut ei) = (0, 0);
    let mut sum = ratio(0, 1);
    let mut out = vec![ratio(0, 1); n];
    for (r, q) in at {
        while si < n && starts[si].0 <= r {
            sum += bands[starts[si].1].2.clone();
            si += 1;
            work.0 += 1;
        }
        while ei < n && ends[ei].0 < r {
            sum -= bands[ends[ei].1].2.clone();
            ei += 1;
            work.0 += 1;
        }
        out[q] = sum.clone();
        work.0 += 1;
    }
    out
}

/// THE MOTES WITHIN A REACH: of motes ordered by reach, the run standing
/// strictly within `d` of `r`, `|r_m − r| < d`, read by two bisections of
/// the order, never mote by mote.
pub fn within_reach<'a>(by_reach: &'a [(Ratio, usize)], r: &Ratio, d: &Ratio, work: &mut Work) -> &'a [(Ratio, usize)] {
    let mut bisect = |inside: &dyn Fn(&Ratio) -> bool| {
        let (mut lo, mut hi) = (0, by_reach.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            work.0 += 1;
            if inside(&by_reach[mid].0) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    };
    let (below, above) = (r.clone() - d.clone(), r.clone() + d.clone());
    let lo = bisect(&|x: &Ratio| *x <= below);
    let hi = bisect(&|x: &Ratio| *x < above);
    &by_reach[lo..hi.max(lo)]
}

/// THE BEADS (operator 2026-10-18: "you bead the individual fibers and then
/// braid the fibers together"): the matter's own gatherings, read every
/// echo. A FIBRE is one kind's run on one face (the same hemisphere in every
/// plane), motes whose dispersal bands `r(1 ± p)` overlap, linked
/// transitively; it pinches where its hold gathers (each local maximum) and
/// parts at the minima between. Each bead is one body of gas. Returned by
/// face: (its members, the reach where it pinches). Read by the one
/// enclosure: each mote's reach `ℓ/H(<ℓ)` about the hold it turns in. The
/// collapse reads the disc leg alone (`disc_leg`); a body of gas is a body on
/// either leg (the face's orthogonal hemisphere is the leg), so the merge
/// reads both.
pub fn beads(motes: &[Mote], enc: &Enclosed, p: &Ratio, total: &Ratio, disc_leg: bool) -> std::collections::BTreeMap<Vec<bool>, Vec<(Vec<usize>, Ratio)>> {
    let mut beads_by_face: std::collections::BTreeMap<Vec<bool>, Vec<(Vec<usize>, Ratio)>> = std::collections::BTreeMap::new();
    if total.is_zero() {
        return beads_by_face;
    }
    let half = ratio(1, 2);
    let (down, up) = (ratio(1, 1) - p.clone().min(ratio(1, 1)), ratio(1, 1) + p.clone());
    let hh: Vec<Ratio> = motes.iter().map(|x| enc.hold(&x.ell_o)).collect();
    let rr: Vec<Ratio> = motes.iter().zip(&hh).map(|(x, h)| x.ell_o.clone() / h.clone()).collect();
    let lov: Vec<Ratio> = rr.iter().map(|r| r.clone() * down.clone()).collect();
    let hiv: Vec<Ratio> = rr.iter().map(|r| r.clone() * up.clone()).collect();
    let delta_of = |j: usize, r: &Ratio| hh[j].clone() / (r.clone() + hh[j].clone());
    // The hold along a group by reach: each member spread over its own band,
    // the hold at a place the sum of every member whose band covers it.
    let profile = |along: &[usize]| -> Vec<Ratio> {
        let bands: Vec<(Ratio, Ratio, Ratio)> = along
            .iter()
            .map(|&j| {
                let (d0, d1) = (delta_of(j, &lov[j]), delta_of(j, &hiv[j]));
                let band = (d0.clone() - d1.clone()) / (d0 + d1);
                let m = motes[j].amount.clone() * weight(&motes[j].key) / total.clone();
                (lov[j].clone(), hiv[j].clone(), if band.is_zero() { m } else { m / band })
            })
            .collect();
        let reach: Vec<Ratio> = along.iter().map(|&j| rr[j].clone()).collect();
        hold_along(&reach, &bands, &mut Work::default())
    };
    // THE FIBRES (operator 2026-10-18: "you bead the individual fibers and
    // then braid the fibers together"): a fibre is one kind's run on one
    // face, motes whose dispersal bands overlap, linked transitively.
    let mut fibres: std::collections::BTreeMap<(Vec<bool>, crate::chem::Kind), Vec<usize>> = std::collections::BTreeMap::new();
    for (i, x) in motes.iter().enumerate() {
        if (!disc_leg || on_disc_leg(x)) && !rr[i].is_zero() {
            fibres.entry(((0..PLANES).map(|k| frac(&x.phase[k]) < half).collect(), x.key.clone())).or_default().push(i);
        }
    }
    // Each fibre's beads, per face: (members, the pinch's reach).
    for ((face, _), mut idx) in fibres {
        idx.sort_by(|a, b| lov[*a].clone().cmp(&lov[*b].clone()).then(a.cmp(b)));
        let mut runs: Vec<Vec<usize>> = Vec::new();
        let mut edge = ratio(0, 1);
        for i in idx {
            if runs.is_empty() || lov[i].clone() > edge {
                runs.push(Vec::new());
                edge = hiv[i].clone();
            } else if hiv[i].clone() > edge {
                edge = hiv[i].clone();
            }
            runs.last_mut().expect("pushed").push(i);
        }
        let out = beads_by_face.entry(face).or_default();
        for run in runs {
            let mut along = run;
            along.sort_by(|a, b| rr[*a].clone().cmp(&rr[*b].clone()).then(a.cmp(b)));
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
                out.push((along[a..b].to_vec(), rr[along[k]].clone()));
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
    beads_by_face
}

pub fn chains(motes: &[Mote], enc: &Enclosed, p: &Ratio, total: &Ratio) -> Vec<Chain> {
    if total.is_zero() {
        return Vec::new();
    }
    // Each mote's reach and band, read once by the one enclosure (exact;
    // recomputing them at every use was most of the cost): its reach `ℓ/H(<ℓ)`
    // and the hold it turns about.
    let (down, up) = (ratio(1, 1) - p.clone().min(ratio(1, 1)), ratio(1, 1) + p.clone());
    let hh: Vec<Ratio> = motes.iter().map(|x| enc.hold(&x.ell_o)).collect();
    let rr: Vec<Ratio> = motes.iter().zip(&hh).map(|(x, h)| x.ell_o.clone() / h.clone()).collect();
    let lov: Vec<Ratio> = rr.iter().map(|r| r.clone() * down.clone()).collect();
    let hiv: Vec<Ratio> = rr.iter().map(|r| r.clone() * up.clone()).collect();
    // The deficit at a reach about the hold a member turns about.
    let delta_of = |j: usize, r: &Ratio| hh[j].clone() / (r.clone() + hh[j].clone());
    // A group judged whole by the filament law, edge to edge.
    let judge = |members: Vec<usize>| -> Option<Chain> {
        let (mut x, mut held, mut a_w, mut r_w) = (ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1));
        for &j in &members {
            let m = motes[j].amount.clone() * weight(&motes[j].key) / total.clone();
            x += m.clone() * (ratio(1, 1) + open(&motes[j].key));
            a_w += m.clone() * motes[j].agitation.clone();
            r_w += m.clone() * rr[j].clone();
            held += m;
        }
        if held.is_zero() {
            return None;
        }
        let j_in = *members.iter().min_by(|&&a, &&b| lov[a].cmp(&lov[b]).then(a.cmp(&b))).expect("a member");
        let j_out = *members.iter().max_by(|&&a, &&b| hiv[a].cmp(&hiv[b]).then(b.cmp(&a))).expect("a member");
        let (d_in, d_out) = (delta_of(j_in, &lov[j_in]), delta_of(j_out, &hiv[j_out]));
        let t_edges = (d_in.clone() - d_out.clone()) / (d_in + d_out);
        let orbit: Vec<Ratio> = members.iter().map(|&j| frac(&motes[j].phase[0])).collect();
        let extent = orbit.iter().max().expect("a member").clone() - orbit.iter().min().expect("a member").clone();
        let apart = ratio(1, 1) - in_step(&extent, &ratio(0, 1)) * (ratio(1, 1) - t_edges);
        let reach_w = r_w / held.clone();
        // The hold the chain turns about, by holding.
        let h_w = members.iter().fold(ratio(0, 1), |s, &j| s + motes[j].amount.clone() * weight(&motes[j].key) / total.clone() * hh[j].clone()) / held.clone();
        let face = (ratio(1, 1) - h_w.clone() / (reach_w.clone() + h_w)) / ratio(2, 1);
        // A chain with no extent at all (no push to spread it) is not read as
        // infinitely dense: it does not bind.
        let bind = if apart.is_zero() { ratio(0, 1) } else { let lambda = x / apart.clone(); lambda.clone() / (ratio(1, 1) + lambda) };
        Some(Chain { members, reach: reach_w, bind, agitation: a_w / held, face, apart })
    };
    let beads_by_face = beads(motes, enc, p, total, true);
    // THE BRAID: beads of different fibres twine where each one's pinch
    // stands within the other's extent (mutual, so no order), and the
    // twining carries across; each braid is read whole by the filament law.
    let mut out = Vec::new();
    for (_, beads) in beads_by_face {
        let n = beads.len();
        let span: Vec<(Ratio, Ratio)> = beads.iter().map(|(m, _)| (m.iter().map(|&j| lov[j].clone()).min().expect("a member"), m.iter().map(|&j| hiv[j].clone()).max().expect("a member"))).collect();
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
pub fn clumps(motes: &[Mote], enc: &Enclosed, p: &Ratio, total: &Ratio) -> Vec<(Ratio, Vec<(usize, Ratio)>)> {
    clumps_of(motes, chains(motes, enc, p, total))
}

/// STREAMING BY FORCE (operator 2026-10-18: "jetstreams depending on their
/// force are either continuums or vacuums in the filament … analogous to the
/// gulf stream"): a mote's stream is READ, never labelled by its origin.
/// Where the jet's push at it (`p·ρ·δ_u`, the torque's own reading) beats the
/// hold of the filament it stands in (`λ/(1 + λ)`), it is carried in the
/// CONTINUUM, one moving body with the current; where the hold beats the
/// push, the jet cannot fill the channel and it drifts in the VACUUM, the
/// thin counter-moving flank. A mote in no filament (in transit on the
/// axial leg) is carried. Returns how many stand in the vacuum.
pub fn stream(motes: &mut [Mote], chains: &[Chain], push: &dyn Fn(&Mote) -> Ratio) -> usize {
    let mut hold: Vec<Option<Ratio>> = vec![None; motes.len()];
    for c in chains {
        for &j in &c.members {
            hold[j] = Some(c.bind.clone());
        }
    }
    let mut vacuum = 0;
    for (x, h) in motes.iter_mut().zip(hold) {
        let at = push(x);
        x.vacuum = h.is_some_and(|h| h >= at);
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

/// MERGE BY THE MATTER'S OWN GATHERINGS (operator 2026-10-23: "we shouldnt
/// have any grids - why would we? they only distort and provide rigidity";
/// design law 3, no cells in a fixed grid): the motes of one bead (`beads`:
/// one kind on one face, a fibre's run between its hold's minima, one body
/// of gas) and of one stream are one mote. Holding sums exactly; spins and
/// heat join by holding onto the grain, the floored spin's remainder
/// returned for the core's reservoir so the spin ledger stays exact; both
/// full phases are read (ascent), meaned by holding on the circle, and the
/// mean passes through the cross at the merged mote's place. A mote in no
/// bead (on the axial leg, in transit) stands as it is. Returns the spin
/// remainder.
pub fn merge(motes: &mut Vec<Mote>, beads: &std::collections::BTreeMap<Vec<bool>, Vec<(Vec<usize>, Ratio)>>, total: &Ratio, place_of: &dyn Fn(&Mote) -> Option<Ratio>, cross: &mut crate::enfold::Cross) -> Ratio {
    let g = |x: &Ratio| crate::ring::grain(x);
    let n = motes.len();
    // Each mote's body: (its bead, its stream), or none.
    let mut body: Vec<Option<(usize, bool)>> = vec![None; n];
    let mut k = 0usize;
    for list in beads.values() {
        for (members, _) in list {
            for &j in members {
                if j < n {
                    body[j] = Some((k, motes[j].vacuum));
                }
            }
            k += 1;
        }
    }
    let mut out: Vec<Mote> = Vec::new();
    let mut at: std::collections::BTreeMap<(usize, bool), usize> = std::collections::BTreeMap::new();
    let mut remainder = ratio(0, 1);
    for (j, x) in std::mem::take(motes).into_iter().enumerate() {
        let Some(kx) = body[j] else {
            out.push(x);
            continue;
        };
        if let Some(y) = at.get(&kx).map(|&i| &mut out[i]) {
            let (my, mx) = (y.amount.clone(), x.amount.clone());
            let both = my.clone() + mx.clone();
            if both.is_zero() {
                continue;
            }
            let exact_o = (my.clone() * y.ell_o.clone() + mx.clone() * x.ell_o.clone()) / both.clone();
            let exact_c = (my.clone() * y.ell_orth.clone() + mx.clone() * x.ell_orth.clone()) / both.clone();
            let (go, gc) = (g(&exact_o), g(&exact_c));
            if !total.is_zero() {
                remainder += both.clone() * weight(&x.key) / total.clone() * ((exact_o - go.clone()) + (exact_c - gc.clone()));
            }
            let w = weight(&x.key);
            let heat = (my.clone() * y.agitation.clone() + mx.clone() * x.agitation.clone()) / both.clone();
            y.agitation = g(&heat);
            y.ell_o = go;
            y.ell_orth = gc;
            let place = place_of(y);
            // Both true phases (ascent), before the amounts join.
            let (ty, tx): (Vec<Ratio>, Vec<Ratio>) = ((0..y.phase.len()).map(|k| true_phase(y, k)).collect(), (0..x.phase.len()).map(|k| true_phase(&x, k)).collect());
            y.amount = both.clone();
            let mut levers: Vec<(usize, Ratio, Ratio)> = Vec::new();
            for k in 0..ty.len().min(tx.len()) {
                cross.note(k, crate::enfold::Fate::Ascended, &ratio(0, 1));
                let mean = circle_mean(&[(my.clone() * w.clone(), ty[k].clone()), (mx.clone() * w.clone(), tx[k].clone())]);
                let shift = mean.clone() - ty[k].clone();
                cross.note(k, crate::enfold::Fate::Condensed, &shift);
                let (held, scar, lever) = crate::enfold::leaf(&mean, place.as_ref(), &heat, k, cross);
                y.phase[k] = held;
                y.agitation = g(&(y.agitation.clone() + scar));
                if let Some((next, f)) = lever {
                    levers.push((k, next, f));
                }
            }
            settle(y, &levers, place.as_ref(), cross);
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
pub fn founding(grains: &Manifest) -> Option<Vec<Mote>> {
    let Some((p, _, mean)) = crate::pulse::galaxy_push(grains) else { return Some(Vec::new()) };
    // The source's phases, read exactly or the founding is refused. THE CAPTURES TURN THEM (with xylarium): each pass of the
    // outbound stream through the throat is one capture and turns the
    // crossing point by a = ½, so kind Z leaves at the source's phase plus
    // `Z·½`, and on the orthogonal orbit it starts on the AXIAL leg, outbound (`½ +
    // Z·½`). Two groups at the founding, parted by each kind's own orthogonal orbit rate
    // `δ(R_z)` at its own reach thereafter. (The per-universe offset, the
    // hypha-assigned TUniversal position, is owed with mycelium.)
    let source = founding_phases()?;
    Some(grains
        .iter()
        .filter(|(_, a)| **a > ratio(0, 1))
        .map(|(k, a)| {
            let turn = ratio(i64::from(crate::chem::charge(k)), 2);
            let phase = vec![frac(&(source[0].clone() + turn.clone())), frac(&(ratio(1, 2) + turn))];
            Mote { key: k.clone(), amount: a.clone(), ell_o: crate::ring::grain(&(p.clone() * mean.clone() / weight(k))), ell_orth: ratio(0, 1), phase, agitation: ratio(0, 1), born: 0, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1) }
        })
        .collect())
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
mod reading_tests {
    use super::{founding, hold_along, weight, within_reach, Mote, Work};
    use cfe::ratio::{ratio, Ratio};

    /// The founding's motes of the first `k` kinds, one grain each: the
    /// engine's own construction, each mote at its own carried reach.
    fn motes(k: u32) -> Vec<Mote> {
        founding(&(1..=k).filter_map(|z| Some((crate::chem::Kind::atom(z)?, ratio(1, 1)))).collect()).expect("the founding's phases are read exactly")
    }

    /// A fibre's reach and bands as the chain law reads them: `R = ℓ/H`, the
    /// band `R(1 ± p)`, the density the member's holding share.
    fn fibre(ms: &[Mote]) -> (Vec<Ratio>, Vec<(Ratio, Ratio, Ratio)>) {
        let (h, p) = (ratio(2, 1), ratio(1, 2));
        let total = ms.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * weight(&x.key));
        let reach: Vec<Ratio> = ms.iter().map(|x| x.ell_o.clone() / h.clone()).collect();
        let bands = ms.iter().zip(&reach).map(|(x, r)| (r.clone() * (ratio(1, 1) - p.clone()), r.clone() * (ratio(1, 1) + p.clone()), x.amount.clone() * weight(&x.key) / total.clone())).collect();
        (reach, bands)
    }

    #[test]
    fn the_hold_along_a_fibre_is_the_sum_of_the_bands_covering_each_reach() {
        let (reach, bands) = fibre(&motes(40));
        let swept = hold_along(&reach, &bands, &mut Work::default());
        for (i, r) in reach.iter().enumerate() {
            let covering = bands.iter().filter(|(lo, hi, _)| lo <= r && r <= hi).fold(ratio(0, 1), |s, b| s + b.2.clone());
            assert_eq!(swept[i], covering, "member {i}");
        }
    }

    #[test]
    fn the_hold_along_a_fibre_costs_its_three_orders_and_one_pass_never_its_pairs() {
        // Three orderings (n·⌈log₂ n⌉ comparisons each at most) and one pass
        // of 3n steps; member against member would be n² and exceed it.
        for k in [40u32, 160] {
            let (reach, bands) = fibre(&motes(k));
            let n = u64::try_from(reach.len()).unwrap_or(0);
            let mut work = Work::default();
            let _ = hold_along(&reach, &bands, &mut work);
            let bound = 3 * n * u64::from(64 - n.leading_zeros()) + 3 * n;
            assert!(work.0 <= bound && bound < n * n, "n {n}: work {}, bound {bound}", work.0);
        }
    }

    #[test]
    fn the_motes_within_a_reach_are_exactly_those_strictly_within_it() {
        let ms = motes(60);
        let mut by: Vec<(Ratio, usize)> = ms.iter().enumerate().map(|(i, x)| (x.ell_o.clone(), i)).collect();
        by.sort();
        let (r, d) = (by[30].0.clone(), (by[40].0.clone() - by[20].0.clone()) / ratio(2, 1));
        let window: Vec<usize> = within_reach(&by, &r, &d, &mut Work::default()).iter().map(|x| x.1).collect();
        let defined: Vec<usize> = by.iter().filter(|(x, _)| (if *x > r { x.clone() - r.clone() } else { r.clone() - x.clone() }) < d).map(|x| x.1).collect();
        assert_eq!(window, defined);
    }

    #[test]
    fn the_motes_within_a_reach_cost_two_bisections_not_a_walk() {
        let cost = |k: u32| {
            let ms = motes(k);
            let mut by: Vec<(Ratio, usize)> = ms.iter().enumerate().map(|(i, x)| (x.ell_o.clone(), i)).collect();
            by.sort();
            let mut work = Work::default();
            let _ = within_reach(&by, &by[k as usize / 2].0, &ratio(1, 1000), &mut work);
            work.0
        };
        let (n, n4) = (cost(40), cost(160));
        // Two bisections grow by two steps each when N quadruples.
        assert!(n4 <= n + 4, "N {n}, 4N {n4}");
    }
}

#[cfg(test)]
mod passage_tests {
    use super::mean_response;
    use cfe::ratio::ratio;

    #[test]
    fn the_throats_inversion_exchanges_core_and_rim() {
        use super::through;
        // The throat's reflection: the core and the rim change places, the
        // throat stands.
        assert_eq!(through(&ratio(0, 1), &ratio(1, 1)), ratio(2, 1));
        assert_eq!(through(&ratio(1, 1), &ratio(1, 1)), ratio(1, 1));
    }

    #[test]
    fn a_motes_lever_keeps_its_true_place() {
        use super::{settle, true_phase, Mote};
        let mut x = Mote { key: crate::chem::Kind::parse("Z001").expect("a kind"), amount: ratio(1, 1), ell_o: ratio(1, 1), ell_orth: ratio(0, 1), phase: vec![ratio(1, 60), ratio(0, 1)], agitation: ratio(0, 1), born: 0, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1) };
        let p = ratio(1, 60);
        let mut cross = crate::enfold::Cross::default();
        // Its true place lies 1/240 past its mark: a quarter of a place, so a
        // quarter of its holding stands one place on.
        settle(&mut x, &[(0, ratio(2, 60), ratio(1, 4))], Some(&p), &mut cross);
        assert_eq!(true_phase(&x, 0), ratio(1, 60) + ratio(1, 240));
        assert_eq!(true_phase(&x, 1), ratio(0, 1), "a plane with nothing past stands on its mark");
        // Backward: the next mark behind.
        settle(&mut x, &[(0, ratio(0, 1), ratio(1, 4))], Some(&p), &mut cross);
        assert_eq!(true_phase(&x, 0), ratio(1, 60) - ratio(1, 240));
    }

    #[test]
    fn the_shear_moves_turning_outward_and_keeps_the_books() {
        use super::{shear, weight, Enclosed, Mote};
        let mote = |ell: (i64, i64), ph: (i64, i64)| Mote { key: crate::chem::Kind::parse("Z001").expect("a kind"), amount: ratio(1, 8), ell_o: ratio(ell.0, ell.1), ell_orth: ratio(0, 1), phase: vec![ratio(ph.0, ph.1), ratio(0, 1)], agitation: ratio(0, 1), born: 0, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1) };
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
