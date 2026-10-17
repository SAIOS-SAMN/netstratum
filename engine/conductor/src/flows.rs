//! THE FLOWS (docs/83; xylarium 11.0 "NEVER PLACE MATTER - THE FLOWS",
//! agreed with xylarium 2026-10-16). Operator: "let the formulas place where
//! the matter goes and how it gets there and what it does after it gets
//! there." Matter is held turning: it moves only as its STATE changes, by
//! laws acting on that state; where it gathers, what is held and what is
//! gas, where it collapses, are READINGS of the state, never placements.
//!
//! - **A parcel's state**, carried forward, never rebuilt: its kind and
//!   amount, its spin in the orbit's plane `ℓ_o` (reach `R = ℓ_o/H`, the
//!   motion law) and in the circulation's plane `ℓ_c` (radial–axial, along
//!   the torsion axis), its PHASES on its enclosure's S^n (one per plane,
//!   turns against the bell), its agitation and birth.
//! - **The kick**, once per bell of the kicking core (the core's bell is its
//!   bounce, once an echo it regenerates), every parcel, held or not: per
//!   plane, the HEMISPHERE the parcel's phase stands in against the bell
//!   signs it (the S⁰ face; operator: coherence is a count over the S^n
//!   hemispheres, in the sphere's own measure) and S¹'s own LINEAR measure in
//!   turns sizes it, `1 − 2·|ψ|` (ψ the phase against the bell, |ψ| its
//!   distance on the circle); times κ, the push's share gripping the parcel
//!   by its core's hold at its reach: `κ_o = p·δ(R)` for the own family in
//!   the orbit's plane, `κ_c = p·ρ·δ_u` for the parent's (the universal bell
//!   received at ρ) in the circulation's plane. Off resonance the signs
//!   alternate and cancel; on resonance they add: the cyclotron.
//! - **The spin ledger**: every Δℓ given to a parcel of holding m is −m·Δℓ to
//!   its core's turning reservoir, so `Σ m·ℓ + reservoir` is exact.
//! - **The cut is a reading**: a parcel's held share is the part of its
//!   dispersal band (`R(1 ± p)`) within the throat, in both planes,
//!   `f_o · f_c`; the rest is the galaxy's gas.
//! - **Two exits, by state**: the JET, `ℓ_c` carrying the axial reading past
//!   the throat (`ℓ_c/H ≥ H`), along the torsion axis; RADIAL ESCAPE, `R ≥ H`
//!   in the orbit's plane. Either way the parcel crosses whole into the
//!   medium one tier up.
//! - **Collapse where the parcels are**: a parcel and those near it in reach
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

/// A PARCEL: matter as held turning, its state carried forward.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parcel {
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

/// THE CUT AS A READING: a parcel's held share, `f_o·f_c`.
pub fn held(parcel: &Parcel, h: &Ratio, p: &Ratio) -> Ratio {
    held_share(&parcel.ell_o, h, p) * held_share(&parcel.ell_c, h, p)
}

/// One bell's kicks on every parcel: the own family in the orbit's plane
/// (`κ_o = p·δ(R)`, a spin); the parent's moves the parcel along the loop
/// (`p·ρ·δ_u`, a phase).
/// Each plane's phase first advances by its own turning over the bell's two
/// rings (`2·δ` turns, the motion law's `ω = δ`; the bell's own turn is
/// whole, so the phase against it moves by `2δ` mod 1). Returns the spin
/// the core's reservoir gives (Σ m·Δℓ, to be taken from it).
pub fn kick(parcels: &mut [Parcel], h: &Ratio, p: &Ratio, rho: &Ratio, delta_u: &Ratio, total: &Ratio) -> Ratio {
    let mut given = ratio(0, 1);
    for x in parcels.iter_mut() {
        let r_o = x.ell_o.clone() / h.clone();
        let d_o = h.clone() / (r_o + h.clone());
        // THE LOOP'S RATE (with xylarium): the jetstream turns by the
        // motion law AT THE PARCEL'S REACH, δ(R) a ring, two rings a bell,
        // continuously; inner matter circulates faster than outer. (At
        // R = H it is one turn a bell, a 1:1 lock with the bell: the rim.)
        let d_c = d_o.clone();
        // On the coordinate's grain (`60⁻⁶`, the carrier's resolution, as
        // every spin and place): exact sums of exact rationals otherwise
        // compound their denominators each bell without bound.
        x.phase[0] = crate::ring::grain(&frac(&(x.phase[0].clone() + ratio(2, 1) * d_o.clone())));
        let loop_phase = frac(&(x.phase[1].clone() + ratio(2, 1) * d_c));
        let dl_o = p.clone() * d_o * response(&x.phase[0]);
        // THE PARENT'S PULSE MOVES MATTER ALONG THE LOOP (operator: "the
        // pulse moves matter along the loop"; with xylarium): absorbed in
        // step it ADVANCES the loop's phase, `p·ρ·δ_u` × the hemisphere's
        // signed share, on top of the loop's own rate; nothing accumulates
        // as a spin in the loop's plane.
        let advance = p.clone() * rho.clone() * delta_u.clone() * response(&loop_phase);
        x.phase[1] = crate::ring::grain(&frac(&(loop_phase + advance)));
        // Spin cannot carry a parcel through zero reach: a turning is a
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
/// loop, a parcel's phase in it its place on the loop. The AXIAL leg (out
/// along the torsion axis and over: phase in `[½, 1)`) carries matter in
/// transit; the DISC leg (down at the rim and home through the disc: phase
/// in `[0, ½)`) is where the cut and collapse are read. No jet batch, no jet
/// exit: matter goes round, out and back at once.
pub fn on_disc_leg(x: &Parcel) -> bool {
    frac(&x.phase[1]) < ratio(1, 2)
}

/// The one exit, by state: a parcel whose loop OVERREACHES THE RIM (its
/// reach at or past the throat, `R ≥ H`) crosses whole into the medium one
/// tier up, where the enclosing core's loops carry it round the same way.
/// Returns the parcels that left; the rest stay.
pub fn exits(parcels: &mut Vec<Parcel>, h: &Ratio) -> Vec<Parcel> {
    let (gone, stay): (Vec<Parcel>, Vec<Parcel>) = std::mem::take(parcels).into_iter().partition(|x| x.ell_o.clone() / h.clone() >= *h);
    *parcels = stay;
    gone
}

/// The galaxy's held and free manifests, read from its parcels: on the
/// disc leg by the cut; on the axial leg, in transit, never held.
pub fn read_cut(parcels: &[Parcel], h: &Ratio, p: &Ratio) -> (Manifest, Manifest) {
    let (mut kept, mut free) = (Manifest::new(), Manifest::new());
    for x in parcels {
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

/// A candidate clump about parcel `i`: its members (those within its
/// dispersal band in reach, weighted by the product over planes of their
/// in-step shares) and its terms: (binding `x/(k + x)` of their combined
/// holding and magnetism, their agitation, the unlocked face `½(1 − δ)`, the
/// shear `H·w/(k(k + w))`).
pub fn candidate(parcels: &[Parcel], h: &Ratio, p: &Ratio, total: &Ratio, i: usize) -> Option<(Vec<(usize, Ratio)>, [Ratio; 4])> {
    let reach = |x: &Parcel| x.ell_o.clone() / h.clone();
    let r = reach(&parcels[i]);
    if r.is_zero() || total.is_zero() {
        return None;
    }
    let near = |j: usize| -> Ratio {
        let rj = reach(&parcels[j]);
        if rj < r.clone() * (ratio(1, 1) - p.clone()) || rj > r.clone() * (ratio(1, 1) + p.clone()) {
            return ratio(0, 1);
        }
        (0..PLANES).fold(ratio(1, 1), |s, k| s * in_step(&parcels[i].phase[k], &parcels[j].phase[k]))
    };
    let members: Vec<(usize, Ratio)> = (0..parcels.len()).map(|j| (j, near(j))).filter(|(_, c)| *c > ratio(0, 1)).collect();
    let (mut x, mut a_w, mut held_part) = (ratio(0, 1), ratio(0, 1), ratio(0, 1));
    for (j, c) in &members {
        let m = parcels[*j].amount.clone() * weight(&parcels[*j].key) / total.clone() * c.clone();
        x += m.clone() * (ratio(1, 1) + open(&parcels[*j].key));
        a_w += m.clone() * parcels[*j].agitation.clone();
        held_part += m;
    }
    if held_part.is_zero() {
        return None;
    }
    let k = r.clone() + h.clone();
    let w = ratio(2, 1) * p.clone() * r;
    let face = (ratio(1, 1) - h.clone() / k.clone()) / ratio(2, 1);
    let shear = h.clone() * w.clone() / (k.clone() * (k.clone() + w));
    Some((members, [x.clone() / (k + x), a_w / held_part, face, shear]))
}

/// THE FILAMENT'S BINDING (xylarium; the clump read in its OWN enclosure,
/// operator: shares in the body's own enclosure): a group's line holding,
/// its 3Z over its spread along the ring (its members' phase spread in the
/// orbit's plane times its reach, never below the coordinate's grain),
/// `λ/(1 + λ)`; no core in it (the core enters only as the shear capping a
/// fragment). Diagnostic beside `candidate`.
pub fn filament(parcels: &[Parcel], h: &Ratio, total: &Ratio, i: usize, members: &[(usize, Ratio)]) -> Ratio {
    let r = parcels[i].ell_o.clone() / h.clone();
    if r.is_zero() || total.is_zero() {
        return ratio(0, 1);
    }
    let x = members.iter().fold(ratio(0, 1), |s, (j, c)| s + parcels[*j].amount.clone() * weight(&parcels[*j].key) / total.clone() * c.clone() * (ratio(1, 1) + open(&parcels[*j].key)));
    let spread = members.iter().map(|(j, _)| ratio(2, 1) * off(&(parcels[*j].phase[0].clone() - parcels[i].phase[0].clone()))).max().unwrap_or_else(|| ratio(0, 1));
    let floor = crate::ring::grain(&ratio(1, 1)).max(Ratio::new(1.into(), num_bigint::BigInt::from(60).pow(6)));
    let lambda = x / (r * spread.max(floor));
    lambda.clone() / (ratio(1, 1) + lambda)
}

/// THE CLUMPS, where the parcels are: every parcel whose candidate binds
/// at least its agitation, the unlocked face and the shear collapses; a
/// parcel claimed by several centres divides itself by their weights (no
/// order). Returns, per holding centre, its reach and the share of each
/// parcel it takes.
pub fn clumps(parcels: &[Parcel], h: &Ratio, p: &Ratio, total: &Ratio) -> Vec<(Ratio, Vec<(usize, Ratio)>)> {
    let mut holding: Vec<(usize, Vec<(usize, Ratio)>)> = Vec::new();
    for i in 0..parcels.len() {
        // Collapse is read on the disc leg only (matter in transit on the
        // axial leg does not settle).
        if !on_disc_leg(&parcels[i]) {
            continue;
        }
        if let Some((members, [_, a, face, _])) = candidate(parcels, h, p, total, i) {
            let members: Vec<(usize, Ratio)> = members.into_iter().filter(|(j, _)| on_disc_leg(&parcels[*j])).collect();
            // THE FILAMENT LAW (with xylarium; the clump read in its OWN
            // enclosure, operator: shares in the body's own enclosure): its
            // line holding binds against its agitation and the unlocked face;
            // the core enters only as the shear capping a fragment (the
            // fragment spacing from the filament's width is owed: one clump
            // a holding filament until then).
            if filament(parcels, h, total, i, &members) >= a + face {
                holding.push((i, members));
            }
        }
    }
    let mut claimed: Vec<Ratio> = vec![ratio(0, 1); parcels.len()];
    for (_, members) in &holding {
        for (j, c) in members {
            claimed[*j] += c.clone();
        }
    }
    holding
        .into_iter()
        .map(|(i, members)| {
            let shares = members.into_iter().map(|(j, c)| {
                let s = if claimed[j] > ratio(1, 1) { c / claimed[j].clone() } else { c };
                (j, s)
            });
            (parcels[i].ell_o.clone() / h.clone(), shares.collect())
        })
        .collect()
}

/// Merge parcels whose states coincide on the coordinate's grain (`60⁻⁶`
/// a plane, the carrier's resolution, never law): same kind, same spins and
/// phases on the grain, their amounts added.
pub fn merge(parcels: &mut Vec<Parcel>) {
    let g = |x: &Ratio| crate::ring::grain(x);
    let mut out: Vec<Parcel> = Vec::new();
    for x in std::mem::take(parcels) {
        let key = |y: &Parcel| (y.key.clone(), g(&y.ell_o), g(&y.ell_c), y.phase.iter().map(g).collect::<Vec<_>>());
        if let Some(y) = out.iter_mut().find(|y| key(y) == key(&x)) {
            y.amount += x.amount;
            y.agitation = y.agitation.clone().max(x.agitation);
        } else {
            out.push(x);
        }
    }
    *parcels = out;
}

/// The founding's parcels for a galaxy: each kind of its grains carried by
/// the bounce's push, `ℓ_o = p·w̄/w_z` (dispersal's form, own units), no
/// circulation yet, the founding's phases, cold.
pub fn founding(grains: &Manifest) -> Vec<Parcel> {
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
            Parcel { key: k.clone(), amount: a.clone(), ell_o: crate::ring::grain(&(p.clone() * mean.clone() / weight(k))), ell_c: ratio(0, 1), phase, agitation: ratio(0, 1), born: 0 }
        })
        .collect()
}

/// The parcels' total holding (by weight).
pub fn holding(parcels: &[Parcel]) -> Ratio {
    parcels.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * weight(&x.key))
}

/// The parcels' manifest.
pub fn manifest(parcels: &[Parcel]) -> Manifest {
    let mut m = Manifest::new();
    for x in parcels {
        *m.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += x.amount.clone();
    }
    m
}
