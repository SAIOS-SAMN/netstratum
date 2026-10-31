//! The flows (docs/83; Xylarium 11.0 "never place MATTER - the flows",
//! agreed with Xylarium 2026-10-16). Operator: "let the formulas place where
//! the matter goes and how it gets there and what it does after it gets
//! there." Matter is held turning: it moves only as its STATE changes, by
//! laws acting on that state; where it gathers, what is held and what is
//! gas, where it collapses, are readings of the state, never placements.
//!
//! - **A mote's state**, carried forward, never rebuilt: its kind and
//!   amount, its spin in the orbit's plane `ℓ_o` (reach `R = ℓ_o/H`, the
//!   motion law) and in the circulation's plane `ℓ_⊥` (radial–axial, along
//!   the torsion axis), its PHASES on its enclosure's S^n (one per plane,
//!   turns against the bell), its agitation and birth.
//! - **The torque**, once per bell of the torquing core (the core's bell is its
//!   bounce, once an echo it regenerates), every mote, held or not: per
//!   plane, the hemisphere the mote's phase stands in against the bell
//!   signs it (the S⁰ face; operator: coherence is a count over the S^n
//!   hemispheres, in the sphere's own measure) and S¹'s own linear measure in
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
//! - **Two exits, by state**: the jet, `ℓ_⊥` carrying the axial reading past
//!   the throat (`ℓ_⊥/H ≥ H`), along the torsion axis; radial escape, `R ≥ H`
//!   in the orbit's plane. Either way the mote crosses whole into the
//!   medium one tier up.
//! - **Collapse where the motes are**: a mote and those near it in reach
//!   (within its dispersal band) and in phase (the product over planes of
//!   the in-step shares) bind by their combined 3Z read as their own
//!   enclosure, against their agitation (the unlocked face heats, in it) and
//!   the shear.
//!
//! The PLANES (docs/95; ruling of 2026-10-26: the spatial dimension is 3D via
//! 3Z/3Q + R coordinates, positive R denoting 'forward' in linear time):
//! three spatial planes, each axis's closed (Z) and open (Q) frames, and the
//! R loop, the circulation (outbound along the axis, R forward; home through
//! the disc).
use crate::pulse::Manifest;
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// The SPATIAL PLANES (docs/95): a nucleus weighs `3Z`, one frame of each
/// proton on each of three axes, and each axis is a plane of its closed (Z)
/// and open (Q) frames. A body orbits in all three at its one rate `2δ` a
/// bell; their phases are where it stands in each.
pub const SPACE: [usize; 3] = [0, 2, 3];
/// The R LOOP (docs/95 §2): R is reach, the axis paired with none; its loop
/// is the circulation (the orthogonal orbit), its outbound leg R forward.
pub const LOOP: usize = 1;
/// A phase vector's length: the three spatial planes and the R loop.
pub const PLANES: usize = 4;

/// Phases at rest: every spatial plane at the bell, the R loop at `loop_at`.
pub fn rest(loop_at: Ratio) -> Vec<Ratio> {
    let mut v = vec![ratio(0, 1); PLANES];
    v[LOOP] = loop_at;
    v
}

/// A phase vector carried to its full length (the planes it lacks at the
/// bell; the R loop keeps its index).
pub fn full(phase: &mut Vec<Ratio>) {
    if phase.len() < PLANES {
        phase.resize(PLANES, ratio(0, 1));
    }
}

/// The antipode: through the centre, half a turn on in every spatial plane.
pub fn antipode(phase: &mut [Ratio]) {
    for &k in &SPACE {
        if let Some(ph) = phase.get_mut(k) {
            *ph = frac(&(ph.clone() + ratio(1, 2)));
        }
    }
}

/// A tilt read whole: its three shares, isotropic (a third each) where none
/// is carried.
pub fn tilt_of(t: &[Ratio]) -> Vec<Ratio> {
    if t.len() == SPACE.len() {
        t.to_vec()
    } else {
        vec![ratio(1, 3); SPACE.len()]
    }
}

/// A tilt from per-plane spin magnitudes: each plane's share of their sum,
/// held on the grain with the last plane taking the rest (the shares sum to
/// one exactly, never re-divided without bound). Isotropic where nothing
/// turns.
pub fn tilt_from(spins: &[Ratio]) -> Vec<Ratio> {
    let sum = spins.iter().fold(ratio(0, 1), |s, l| s + l.clone());
    if sum.is_zero() {
        return Vec::new();
    }
    let mut t: Vec<Ratio> = spins
        .iter()
        .map(|l| crate::ring::grain(&(l.clone() / sum.clone())))
        .collect();
    let rest = ratio(1, 1)
        - t[..t.len() - 1]
            .iter()
            .fold(ratio(0, 1), |s, x| s + x.clone());
    let last = t.len() - 1;
    t[last] = rest;
    t
}

/// Tilts blended where turning bodies join: each plane's spin added,
/// `Σ w·ℓ·t_k`, re-read as shares (`w` each one's holding, `ℓ` its spin).
pub fn blend(parts: &[(Ratio, Ratio, Vec<Ratio>)]) -> Vec<Ratio> {
    let mut sums = vec![ratio(0, 1); SPACE.len()];
    for (w, l, t) in parts {
        for (k, s) in tilt_of(t).into_iter().enumerate() {
            sums[k] += w.clone() * l.clone() * s;
        }
    }
    tilt_from(&sums)
}

/// A reading's mean over the spatial planes (L1: their sum over three).
pub fn space_mean(f: impl Fn(usize) -> Ratio) -> Ratio {
    SPACE.iter().fold(ratio(0, 1), |s, &k| s + f(k)) / ratio(3, 1)
}

/// The spatial phases of a vector (each plane's, the bell where it lacks one).
pub fn space_of(phase: &[Ratio]) -> Vec<Ratio> {
    SPACE
        .iter()
        .map(|&k| phase.get(k).cloned().unwrap_or_else(|| ratio(0, 1)))
        .collect()
}

/// A mote: matter as held turning, its state carried forward.
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
    /// Its stream KIND, inherited from where it entered and carried
    /// forward, never reassigned: false the continuum (the core's founding
    /// outflow, a star's outbursts and wind), true the vacuum (ocean
    /// arrivals).
    pub vacuum: bool,
    /// The lever (docs/85 §9): per plane, the whole nuclei of its holding
    /// (signed, in grains) standing one place past its mark, so its holding's
    /// centre is its true position: `phase + lever/amount · place`. Never
    /// more than half its amount. Empty where nothing stands past.
    pub lever: Vec<Ratio>,
    /// The place its lever is counted in (a share of a turn).
    pub lever_place: Ratio,
    /// Its tilt (docs/96 step 1): its orbit spin's share in each spatial
    /// plane, `ℓ_k = ℓ_o·t_k`, the shares summing to one (L1, so its reach
    /// stays `ℓ_o/H`); empty reads isotropic, a third in each.
    pub tilt: Vec<Ratio>,
}

/// A mote's true phase in plane k (ascent): its mark and the share of its
/// holding standing one place past it.
pub fn true_phase(x: &Mote, k: usize) -> Ratio {
    let at = x.phase.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
    match x.lever.get(k) {
        Some(l) if !l.is_zero() && !x.amount.is_zero() => {
            at + l.clone() / x.amount.clone() * x.lever_place.clone()
        }
        _ => at,
    }
}

/// Settle a mote's levers from this bell's (descent): each plane's lever
/// `(next mark, share f)` stands `f` of its holding one place past its mark,
/// in whole nuclei (the rest below one nucleus acknowledged, the quantum
/// tier); a plane with none stands whole on its mark.
pub fn settle(
    x: &mut Mote,
    levers: &[(usize, Ratio, Ratio)],
    place: Option<&Ratio>,
    cross: &mut crate::enfold::Cross,
) {
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
    ratio(i64::from(crate::chem::weight(k)), 1)
}

/// A mote's nuclear grip (docs/103 §6, the bond's nuclear layer ν): its
/// nuclei's fibres' grip, `κ_w·g` at the held share `g` of its place (docs/105
/// §3), a cluster's by its parts' holding. In `[0, 1]`.
pub fn grip_share(x: &Mote, g: &Ratio) -> Ratio {
    let p = g.clone();
    let parts = x.key.parts();
    let whole = parts
        .iter()
        .fold(0u64, |s, (n, w)| s + u64::from(*n) * u64::from(*w));
    if whole == 0 {
        return ratio(0, 1);
    }
    parts.iter().fold(ratio(0, 1), |s, (n, w)| {
        s + ratio(i64::from(*n) * i64::from(*w), 1)
            * crate::nuclides::grip(*w, &p).unwrap_or_else(|| ratio(0, 1))
    }) / ratio(i64::try_from(whole).unwrap_or(i64::MAX), 1)
}

/// A kind's open share of its whole weight, `Q/(A + Q)` (Z and Q together,
/// operator 2026-10-26: never one pole over the other, `Q/A` passed one
/// where the open outweighed the held). At most one.
pub fn open(k: &crate::chem::Kind) -> Ratio {
    ratio(
        i64::from(crate::chem::unpaired(k)),
        i64::from(crate::chem::weight(k).max(1)),
    )
}

/// The two faces at a deficit, read through the inversion (docs/104 §0):
/// `(held, unlocked) = (1 − δ, δ)`.
pub fn face(delta: &Ratio) -> (Ratio, Ratio) {
    (ratio(1, 1) - delta.clone(), delta.clone())
}

/// The band at reach R about the hold H for a kind carried `q = ½·w̄_c/w`
/// (docs/104 §6.2): `q·u − ½·g·R·H` on the faces `(g, u)` at
/// `δ = H/(R + H)`; zero at `R·R = 2q` whatever H holds (§6.3).
pub fn band(q: &Ratio, r: &Ratio, h: &Ratio) -> Ratio {
    let (g, u) = face(&(h.clone() / (r.clone() + h.clone())));
    q.clone() * u - ratio(1, 2) * g * r.clone() * h.clone()
}

/// The band as an attractor (docs/112 F3): its step at reach R, never
/// carrying a mote of spin ℓ past its rest point (`R·R = 2q`, read exactly
/// on the squares, never by a root): where a whole step would cross it, the
/// step is halved until it does not, so the motion converges on the rest
/// and never swings through it. What is not given stays with the core.
pub fn toward_rest(q: &Ratio, r: &Ratio, h: &Ratio, ell: &Ratio) -> Ratio {
    let mut step = band(q, r, h);
    if h.is_zero() || step.is_zero() {
        return step;
    }
    let rest = ratio(2, 1) * q.clone();
    let side = |reach: &Ratio| (reach.clone() * reach.clone()).cmp(&rest);
    let now = side(r);
    for _ in 0..64 {
        let after = (ell.clone() + step.clone()) / h.clone();
        let then = side(&after);
        if then == now || then == std::cmp::Ordering::Equal || now == std::cmp::Ordering::Equal {
            return step;
        }
        step /= ratio(2, 1);
    }
    ratio(0, 1)
}

/// A phase difference as a signed displacement in `(−½, ½]`.
pub fn signed(d: &Ratio) -> Ratio {
    let f = frac(d);
    if f > ratio(1, 2) {
        f - ratio(1, 1)
    } else {
        f
    }
}

/// A phase brought onto the circle, `[0, 1)`.
pub fn wrap(x: &Ratio) -> Ratio {
    frac(x)
}

fn frac(x: &Ratio) -> Ratio {
    x.clone() - x.floor()
}

/// The MEAN of PHASES on the circle, never on the line (a line mean of
/// 0.95 and 0.05 is ½, the far face): each phase's signed offset from a
/// member's, `(−½, ½]`, weighted, added back on the ring. Exact; the same
/// for any member as reference while the members share a hemisphere.
pub fn circle_mean(members: &[(Ratio, Ratio)]) -> Ratio {
    let m = members.iter().fold(ratio(0, 1), |s, (w, _)| s + w.clone());
    let Some((_, at)) = members.first() else {
        return ratio(0, 1);
    };
    if m.is_zero() {
        return frac(at);
    }
    let signed = |x: &Ratio| {
        let f = frac(x);
        if f > ratio(1, 2) {
            f - ratio(1, 1)
        } else {
            f
        }
    };
    let d = members.iter().fold(ratio(0, 1), |s, (w, ph)| {
        s + w.clone() * signed(&(ph.clone() - at.clone()))
    }) / m;
    frac(&(at.clone() + d))
}

/// The distance of a phase from the bell on the circle, `|ψ|` in `[0, ½]`.
fn off(psi: &Ratio) -> Ratio {
    let f = frac(psi);
    if f > ratio(1, 2) {
        ratio(1, 1) - f
    } else {
        f
    }
}

/// The torque's response in one plane: the hemisphere signs it (the first half
/// of the circle against the bell gains, the second loses), S¹'s linear
/// measure sizes it, `1 − 2|ψ|`.
pub fn response(psi: &Ratio) -> Ratio {
    let f = frac(psi);
    let share = ratio(1, 1) - ratio(2, 1) * off(&f);
    if f < ratio(1, 2) {
        share
    } else {
        -share
    }
}

/// The torque's response over its passage (with Xylarium: a torque sampled at
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

/// Quantum torsion (docs/100 §8; the bounce is Einstein–Cartan, torsion
/// couples to spin): the winding a quantum state keeps from the twist, its
/// lead between planes against its enclosure's plane, `τ = Σ_k |t_k −
/// t̄_k|/2` (L1, in `[0, 1]`: 0 settled, 1 wound wholly into a plane its
/// enclosure does not turn in). Kept as the tilt, relaxed by the attractor
/// law (`settle_disc`), never a scalar of its own.
pub fn torsion(tilt: &[Ratio], plane: &[Ratio]) -> Ratio {
    let (t, p) = (tilt_of(tilt), tilt_of(plane));
    t.iter().zip(&p).fold(ratio(0, 1), |s, (a, b)| {
        s + if a > b {
            a.clone() - b.clone()
        } else {
            b.clone() - a.clone()
        }
    }) / ratio(2, 1)
}

/// Its sense (docs/100 §8): the winding turns with the route, forward on the
/// continuum leg, backward on the vacuum leg, so a Möbius pass reverses it.
pub fn torsion_signed(x: &Mote, plane: &[Ratio]) -> Ratio {
    let t = torsion(&x.tilt, plane);
    if x.vacuum {
        -t
    } else {
        t
    }
}

/// A mote's step with A body's phase (docs/99 §2.2; ruling of 2026-10-27:
/// phases, not frames): its in-step share
/// across the spatial planes by its own orbit's shares (its tilt, L1; a plane
/// its orbit does not turn in carries no weight), `C = Σ t_k·in_step(ψ_k,
/// ψ̄_k)`. One in step, 0 opposed; it stands on the body's held hemisphere
/// where `C ≥ ½`.
pub fn step_with(x: &Mote, psi: &[Ratio]) -> Ratio {
    let t = tilt_of(&x.tilt);
    SPACE.iter().zip(&t).fold(ratio(0, 1), |c, (&k, tk)| {
        c + tk.clone()
            * in_step(
                x.phase.get(k).unwrap_or(&ratio(0, 1)),
                psi.get(k).unwrap_or(&ratio(0, 1)),
            )
    })
}

/// The firmament's order (ruling of 2026-10-19: the pulses over time and the
/// reverberation/pulsation of the bouncing antipoles move the firmament
/// into order, a dodecahedron-like order derived from the mathematics, never
/// set as a dodecahedron; the firmament is to be built). The heart's own law, one tier up: matter pushes matter by
/// the electromagnetic repulsion gripping its open frames. On each pulse,
/// every mote is pushed away from every other in each plane, by the other's
/// open charge (`a·e`, its amount times its open frames) times their
/// closeness on the sphere (the product over planes of their in-step
/// shares), signed away from it; summed over the whole charge, it moves by
/// the bounce's `push` times its own carried share `(e/ē)·(w̄/w)` (its open
/// frames over the mean, the mean weight over its own: the heaviest barely
/// move), at most half a turn. All read at once, then moved. Equal charges
/// pushing apart on a sphere settle into the polyhedra (the Thomson
/// arrangements): whatever order forms is the mathematics', never set.
pub fn repel(motes: &mut [Mote], push: &Ratio) -> Vec<Vec<Ratio>> {
    let n = motes.len();
    if n < 2 {
        return Vec::new();
    }
    let planes = motes.iter().map(|x| x.phase.len()).min().unwrap_or(0);
    let open = |x: &Mote| ratio(i64::from(crate::chem::unpaired(&x.key)), 1);
    let amount = motes.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone());
    let charge = motes
        .iter()
        .fold(ratio(0, 1), |s, x| s + x.amount.clone() * open(x));
    let held = motes
        .iter()
        .fold(ratio(0, 1), |s, x| s + x.amount.clone() * weight(&x.key));
    if amount.is_zero() || charge.is_zero() || held.is_zero() || planes == 0 {
        return Vec::new();
    }
    let (e_mean, w_mean) = (charge.clone() / amount.clone(), held / amount);
    let half = ratio(1, 2);
    let away = |d: &Ratio| {
        let f = frac(d);
        if f.is_zero() || f == half {
            ratio(0, 1)
        } else if f < half {
            ratio(1, 1)
        } else {
            ratio(-1, 1)
        }
    };
    let mut moves: Vec<Vec<Ratio>> = vec![vec![ratio(0, 1); planes]; n];
    for i in 0..n {
        let e_i = open(&motes[i]);
        if e_i.is_zero() {
            continue;
        }
        let carried =
            push.clone() * (e_i / e_mean.clone()) * (w_mean.clone() / weight(&motes[i].key));
        for j in 0..n {
            if i == j {
                continue;
            }
            let q = motes[j].amount.clone() * open(&motes[j]);
            if q.is_zero() {
                continue;
            }
            let close = (0..planes).fold(ratio(1, 1), |c, k| {
                c * in_step(&motes[i].phase[k], &motes[j].phase[k])
            });
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
            let m = (carried.clone() * moves[i][k].clone() / charge.clone())
                .max(-half.clone())
                .min(half.clone());
            moves[i][k] = m;
        }
    }
    for (x, m) in motes.iter_mut().zip(&moves) {
        for k in 0..planes {
            x.phase[k] = crate::ring::grain(&frac(&(x.phase[k].clone() + m[k].clone())));
        }
    }
    moves
}

/// Two motes of a collective are BONDED (docs/111 §4.2) where they stand on
/// one held hemisphere (`step_with ≥ ½`, each against the other), both carry
/// open frames, and the bounce moved them together: their perturbations
/// `Δφ` (the displacements `repel` gave them) in step, tilt-weighted,
/// `C_Δ = Σ_k t_k · in_step(Δφ_k^i, Δφ_k^j) ≥ ½`, by each one's tilt.
pub fn bonded(a: &Mote, b: &Mote, da: &[Ratio], db: &[Ratio]) -> bool {
    let half = ratio(1, 2);
    if crate::chem::unpaired(&a.key) == 0 || crate::chem::unpaired(&b.key) == 0 {
        return false;
    }
    if step_with(a, &b.phase) < half || step_with(b, &a.phase) < half {
        return false;
    }
    let zero = ratio(0, 1);
    let moved = |x: &Mote| {
        let t = tilt_of(&x.tilt);
        SPACE.iter().zip(&t).fold(ratio(0, 1), |c, (&k, tk)| {
            c + tk.clone() * in_step(da.get(k).unwrap_or(&zero), db.get(k).unwrap_or(&zero))
        })
    };
    moved(a) >= half && moved(b) >= half
}

/// A collective's COLLECTIONS (docs/111 §4.2): the connected sets of its
/// bonds, read from one bounce's perturbations (`repel`'s moves), never
/// assigned. Each collection is its members' indices, ascending; a mote
/// bonded to none is in none.
pub fn collections(motes: &[Mote], moves: &[Vec<Ratio>]) -> Vec<Vec<usize>> {
    let n = motes.len().min(moves.len());
    let mut root: Vec<usize> = (0..n).collect();
    fn find(root: &mut [usize], i: usize) -> usize {
        let mut i = i;
        while root[i] != i {
            root[i] = root[root[i]];
            i = root[i];
        }
        i
    }
    let mut any = vec![false; n];
    for i in 0..n {
        if moves[i].is_empty() {
            continue;
        }
        for j in i + 1..n {
            if moves[j].is_empty() {
                continue;
            }
            if bonded(&motes[i], &motes[j], &moves[i], &moves[j]) {
                let (a, b) = (find(&mut root, i), find(&mut root, j));
                if a != b {
                    root[a.max(b)] = a.min(b);
                }
                any[i] = true;
                any[j] = true;
            }
        }
    }
    let mut sets: std::collections::BTreeMap<usize, Vec<usize>> = std::collections::BTreeMap::new();
    for i in 0..n {
        if any[i] {
            let r = find(&mut root, i);
            sets.entry(r).or_default().push(i);
        }
    }
    sets.into_values().collect()
}

/// The held share of a spread about a reach R within the hold H about it
/// (dispersal's form, p the push share): `[R(1 − p), R(1 + p)]` against H.
fn held_share(r: &Ratio, h: &Ratio, p: &Ratio) -> Ratio {
    let lo = r.clone() * (ratio(1, 1) - p.clone());
    let span = ratio(2, 1) * p.clone() * r.clone();
    if span.is_zero() {
        return if lo <= *h { ratio(1, 1) } else { ratio(0, 1) };
    }
    ((h.clone() - lo) / span).max(ratio(0, 1)).min(ratio(1, 1))
}

/// The cut as A reading, by the one enclosure (one grammar: the hold the
/// exits and the ring law read, `H(<ℓ)` with the throat its base, never a
/// second hold of its own): a mote's held share in each plane, its reach
/// `R = |ℓ|/H(<|ℓ|)` spread by the push about the hold there, `f_o·f_c`. A
/// spin's sense is a direction, never a reach.
pub fn held(mote: &Mote, enc: &Enclosed, p: &Ratio) -> Ratio {
    let size = |x: &Ratio| {
        if *x < ratio(0, 1) {
            -x.clone()
        } else {
            x.clone()
        }
    };
    let plane = |ell: &Ratio| {
        let l = size(ell);
        let (r, _) = enc.reach(&l);
        held_share(&r, &enc.hold(&l), p)
    };
    plane(&mote.ell_o) * plane(&mote.ell_orth)
}

/// The enclosed holding (the ring-density law, Xylarium 11.0: a ring is
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
        Enclosed {
            spins: bodies.into_iter().map(|(l, _)| l).collect(),
            held,
        }
    }

    /// `H(<ℓ)`: the base and everything turning with less spin.
    pub fn hold(&self, ell: &Ratio) -> Ratio {
        self.held[self.spins.partition_point(|l| l < ell)].clone()
    }

    /// The enclosure's base: its throat's own hold (the ring's `H`).
    pub fn base(&self) -> Ratio {
        self.held.first().cloned().unwrap_or_else(|| ratio(0, 1))
    }

    /// The enclosure's whole hold: its base and every body in it (the rim
    /// law, docs/78: an enclosure's rim is its own holding, `rim = H`).
    pub fn whole(&self) -> Ratio {
        self.held.last().cloned().unwrap_or_else(|| ratio(0, 1))
    }

    /// `H(<r)` by place: the base and every body whose own reach stands
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

    /// A body's reach and deficit at spin ℓ: `R = ℓ/H(<ℓ)`, `δ = H/(R + H)`,
    /// exact (operator 2026-10-23: "this is approximations leaking into an
    /// exact system"): readings, never stored, so never floored. ℓ and H
    /// already stand on the grain, so their quotients' denominators are
    /// bounded; δ is never 0 while anything holds.
    pub fn reach(&self, ell: &Ratio) -> (Ratio, Ratio) {
        let h = self.hold(ell);
        let r = ell.clone() / h.clone();
        (r.clone(), h.clone() / (r + h))
    }
}

/// The galaxy's PHASES AT A REACH turned by one bell: what every mote's
/// phases do there, and a star's reference (its galaxy's own phase at its
/// place, operator 2026-10-18), alike.
///
/// The vacuum is the continuum inverted (operator 2026-10-19: "the vacuum and
/// the continuum must be mathematically inverted across all dimensional
/// facets"; "reversal is not an inversion because the inversion happens on
/// an orthogonal dimension"; with Xylarium): inversion through the throat
/// acts on the radial dimension, orthogonal to every plane the matter turns
/// in, and keeps every angle; it is read on the share, `δ ↦ 1 − δ` (first
/// degree, never a radius squared: design law 2). So an `inverted` (vacuum)
/// mote turns forward in every plane at its pull's other face, `1 − δ(R)`,
/// the continuum's push. Its stored spin
/// is untouched (`Σ m·ℓ` exact).
///
/// Its route (operator 2026-10-16, Xylarium 11.0 (11): "a continuum jetstream
/// carries matter out of the singularity along the axis and home through
/// the disc; a vacuum jetstream draws it in along the axis and out through
/// the disc"): the route round the orthogonal orbit, not the inversion. A vacuum mote's
/// orthogonal orbit (the circulation's plane) runs the other way, its advance read over
/// that passage; its orbit plane turns forward.
#[allow(clippy::too_many_arguments)]
pub fn turn(
    phase: &mut [Ratio],
    tilt: &[Ratio],
    r_o: &Ratio,
    h: &Ratio,
    p: &Ratio,
    tide: Option<(&Ratio, &Ratio)>,
    inverted: bool,
    place: Option<&Ratio>,
    agitation: &Ratio,
    cross: &mut crate::enfold::Cross,
) -> (Ratio, Vec<(usize, Ratio, Ratio)>, Ratio) {
    let pull = h.clone() / (r_o.clone() + h.clone());
    let d_o = if inverted {
        ratio(1, 1) - pull.clone()
    } else {
        pull.clone()
    };
    // The orthogonal orbit's rate (with Xylarium): the jetstream turns by the motion law
    // AT the mote's REACH, δ(R) a ring, two rings a bell, continuously;
    // inner matter circulates faster than outer. (At R = H it is one turn a
    // bell, a 1:1 lock with the bell: the rim.)
    let d_c = d_o.clone();
    // Through the cross (`enfold`, docs/85 §8): held to the body's own place,
    // since exact sums of exact rationals otherwise compound their
    // denominators each bell without bound; the residue drains within its
    // round, passes to the quantum tier acknowledged, or, with no place of its
    // own, is cauterized with its scar recorded.
    // Every spatial plane turns by its tilt's share of the body's turning
    // (docs/96 step 1; L1: the shares sum to the whole `2δ`), each from
    // where it stands.
    let t = tilt_of(tilt);
    let mut scar_o = ratio(0, 1);
    let mut levers: Vec<(usize, Ratio, Ratio)> = Vec::new();
    for (j, &k) in SPACE.iter().enumerate() {
        let Some(at) = phase.get(k).cloned() else {
            continue;
        };
        let (held, scar, lever) = crate::enfold::leaf(
            &(at + ratio(2, 1) * d_o.clone() * t[j].clone()),
            place,
            agitation,
            k,
            cross,
        );
        phase[k] = held;
        scar_o += scar;
        if let Some((n, f)) = lever {
            levers.push((k, n, f));
        }
    }
    let orth_from = phase[LOOP].clone();
    let route = if inverted { ratio(-1, 1) } else { ratio(1, 1) };
    let orth_phase = frac(&(orth_from.clone() + route.clone() * ratio(2, 1) * d_c.clone()));
    // The parent's pulse moves MATTER along the orthogonal orbit (operator: "the pulse
    // moves matter along the orthogonal orbit"; with Xylarium): absorbed in step it
    // advances the orthogonal orbit's phase, `p·ρ·δ_u` × the hemisphere's signed share,
    // on top of the orthogonal orbit's own rate. And it is A spin (docs/90 D1, operator
    // 2026-10-22; Xylarium 11.0: "the loop is an orbit", one torque law per
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
    let (held, scar_c, lever_c) = crate::enfold::leaf(
        &(orth_phase + route * advance),
        place,
        agitation,
        LOOP,
        cross,
    );
    phase[LOOP] = held;
    // A cauterized residue's scar (the heat it enters) and each plane's lever.
    if let Some((n, f)) = lever_c {
        levers.push((LOOP, n, f));
    }
    (scar_o + scar_c, levers, torque_orth)
}

/// The parent's push along the orthogonal orbit over one bell's passage
/// (docs/90 D1): `p·felt` × the hemisphere's signed mean over the arc the
/// orthogonal phase sweeps, `2·δ` from where it stood (a vacuum mote's route
/// runs the other way, its passage read backward). Unsigned by the route:
/// the caller signs it.
pub fn orth_push(orth_from: &Ratio, d_c: &Ratio, p: &Ratio, felt: &Ratio, inverted: bool) -> Ratio {
    let start = if inverted {
        orth_from.clone() - ratio(2, 1) * d_c.clone()
    } else {
        orth_from.clone()
    };
    p.clone() * felt.clone() * mean_response(&start, &(ratio(2, 1) * d_c.clone()))
}

/// Through the centre (the torque's law, docs/83; with Xylarium): a spin
/// carried past zero is a turning in the other sense, so the body passes
/// through the centre to the antipode, its orbit phase half a turn on, its
/// spin the magnitude, never piled at zero.
pub fn through_centre(ell: Ratio, phase: &mut [Ratio]) -> Ratio {
    if ell < ratio(0, 1) {
        antipode(phase);
        -ell
    } else {
        ell
    }
}

/// The swing (docs/90 D2): a body's orthogonal spin projects onto its orbit by
/// where it stands on its orthogonal orbit, the plane's linear measure (`response`): its
/// spin seen in the orbit plane is `ℓ + ℓ_⊥·response(ψ_⊥)`, so its reach swings
/// each orthogonal turn between `(ℓ − |ℓ_⊥|)/H` and `(ℓ + |ℓ_⊥|)/H`, an elongation
/// `e = |ℓ_⊥|/ℓ` read from two planes, no ellipse and no square. Where the
/// swing passes zero the body passes through the centre to the antipode (the
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
/// the matter absorbed (Σ m·|Δℓ|, unsigned: each hemisphere's torque alike;
/// with Xylarium), what the recycling budget is read against.
#[allow(clippy::too_many_arguments)]
pub fn torque(
    motes: &mut [Mote],
    enc: &Enclosed,
    p: &Ratio,
    tide: Option<(&Ratio, &Ratio)>,
    total: &Ratio,
    heart_push: Option<&Ratio>,
    cross: &mut crate::enfold::Cross,
) -> (Ratio, Ratio) {
    let mut given = ratio(0, 1);
    let mut absorbed = ratio(0, 1);
    // The push, certified (design law 11; operator 2026-10-23: "build the
    // certified intervals"): the push is read from the ledger's totals, its
    // exact denominator the ledger's, and it multiplies every mote's torque.
    // It is held by its two bounds on the grain, which contain it; each mote's
    // update is read at both, and where both store the same state and the
    // same record that is what the exact push stores (the update is monotone
    // in the push and the grain is monotone), so it stands, certified. Only
    // where the bounds straddle a mark is the update read with the exact push.
    let lo = crate::ring::grain(p);
    let hi = if lo == *p {
        lo.clone()
    } else {
        lo.clone() + crate::ring::grain_step()
    };
    for x in motes.iter_mut() {
        if lo == hi {
            let (g, ab) = torque_one(x, enc, &lo, tide, total, heart_push, cross);
            given += g;
            absorbed += ab;
            continue;
        }
        let (mut xa, mut xb) = (x.clone(), x.clone());
        let (mut ca, mut cb) = (
            crate::enfold::Cross::default(),
            crate::enfold::Cross::default(),
        );
        let (ga, aa) = torque_one(&mut xa, enc, &lo, tide, total, heart_push, &mut ca);
        let (gb, ab) = torque_one(&mut xb, enc, &hi, tide, total, heart_push, &mut cb);
        if xa == xb && ca == cb && ga == gb && aa == ab {
            *x = xa;
            cross.join(&ca);
            given += ga;
            absorbed += aa;
        } else {
            let (g, ab) = torque_one(x, enc, p, tide, total, heart_push, cross);
            given += g;
            absorbed += ab;
        }
    }
    (given, absorbed)
}

/// One mote's bell: its phases turned and its spins torqued (`turn`,
/// `spin_torque`), its lever settled; returns the spin given and the pulse
/// absorbed.
#[allow(clippy::too_many_arguments)]
fn torque_one(
    x: &mut Mote,
    enc: &Enclosed,
    p: &Ratio,
    tide: Option<(&Ratio, &Ratio)>,
    total: &Ratio,
    heart_push: Option<&Ratio>,
    cross: &mut crate::enfold::Cross,
) -> (Ratio, Ratio) {
    // Each turns by what is enclosed inside it (the core and all inside),
    // at the reach its orthogonal orbit swings it to this bell (docs/90 D2): the
    // motion law at its present reach, so it turns faster inward and
    // slower outward; through the centre where the swing passes zero.
    full(&mut x.phase);
    let (now, through_c) = swung(&x.ell_o, &x.ell_orth, &x.phase[LOOP]);
    let h = enc.hold(&now);
    let (r_o, d_o) = enc.reach(&now);
    // Ascent: the bell turns the mote from where it truly stands, its
    // lever read back onto its marks.
    for k in 0..x.phase.len() {
        x.phase[k] = true_phase(x, k);
    }
    let mut from = space_of(&x.phase);
    if through_c {
        antipode(&mut from);
    }
    // A mote's place: the cap its own gravity holds (`enfold::place`).
    let m = if total.is_zero() {
        ratio(0, 1)
    } else {
        x.amount.clone() * weight(&x.key) / total.clone()
    };
    let place = crate::enfold::place(&r_o, &m, &h);
    let heat_was = x.agitation.clone();
    let tilt = x.tilt.clone();
    let (scar, levers, torque_orth) = turn(
        &mut x.phase,
        &tilt,
        &r_o,
        &h,
        p,
        tide,
        x.vacuum,
        place.as_ref(),
        &heat_was,
        cross,
    );
    let (g, ab) = spin_torque(
        x,
        &from,
        &h,
        &r_o,
        &d_o,
        p,
        &torque_orth,
        heart_push,
        &m,
        &scar,
    );
    // Descent: its weight past its round stands one place past its mark.
    settle(x, &levers, place.as_ref(), cross);
    (g, ab)
}

/// The torque on one mote's SPINS over one bell's passage, the one law at
/// every tier (a galaxy's core on its motes, a star on its system's): from
/// the orbit phase it stood at (`from`) and the reach and deficit it turns
/// at (`r_o`, `d_o`, inside the hold `h`), the own family's push in the orbit
/// plane, the band's push and return (`heart_push`), the parent's push along
/// the orthogonal orbit (`torque_orth`) less the hold's return, and what is
/// not delivered in step left as heat. Returns the spin given (Σ m·Δℓ, the
/// antipole's to pay) and the pulse absorbed (Σ m·|Δℓ|).
#[allow(clippy::too_many_arguments)]
pub fn spin_torque(
    x: &mut Mote,
    from: &[Ratio],
    h: &Ratio,
    r_o: &Ratio,
    d_o: &Ratio,
    p: &Ratio,
    torque_orth: &Ratio,
    heart_push: Option<&Ratio>,
    m: &Ratio,
    scar: &Ratio,
) -> (Ratio, Ratio) {
    let mut given = ratio(0, 1);
    let mut absorbed = ratio(0, 1);
    // Over its whole passage this bell, never at one instant; a vacuum
    // mote at its inverted reach (`1 − δ`), forward.
    let d_o = if x.vacuum {
        ratio(1, 1) - d_o.clone()
    } else {
        d_o.clone()
    };
    // Its response in each SPATIAL plane over that plane's passage (its
    // tilt's share of `2δ`; docs/96 step 1): the push acts in every plane at
    // once (isoclinic), each plane's spin `ℓ·t_k` torqued by its own
    // response, weighted by its share.
    let t = tilt_of(&x.tilt);
    let resp: Vec<Ratio> = from
        .iter()
        .zip(&t)
        .map(|(f, tk)| mean_response(f, &(ratio(2, 1) * d_o.clone() * tk.clone())))
        .collect();
    let k = resp
        .iter()
        .zip(&t)
        .fold(ratio(0, 1), |s, (r, tk)| s + r.clone() * tk.clone());
    // Dissipation (ruling of 2026-10-19: heat dissipation to the void is read
    // from the current physics, mimicking nature; with
    // Xylarium): what the torque does not deliver in step becomes heat,
    // `p·δ·(1 − |k|)` (k its signed mean over the passage, the coherent
    // share), and heat leaves by the built cooling law's two channels
    // over the passage's two rings: the light through its open frames
    // (`e`, open over held, a ring) and the vacuum's pull back (`δ` at its
    // reach, a ring). The keep `(1 − e)(1 − δ)` is applied once per ring of
    // the double pass (compounding over the two passes, as `(1 − s)ⁿ`
    // compounds the release; never a square of a quantity):
    // `a ← (a + heat)·keep·keep`.
    let coherent = if k < ratio(0, 1) { -k } else { k };
    let heat = p.clone() * d_o.clone() * (ratio(1, 1) - coherent.min(ratio(1, 1)));
    let keep = (ratio(1, 1) - open(&x.key).min(ratio(1, 1))) * (ratio(1, 1) - d_o.clone());
    x.agitation =
        crate::ring::grain(&((x.agitation.clone() + heat) * keep.clone() * keep + scar.clone()));
    // The band (one grammar: the firmament's, a galaxy's, a star's): the
    // core's bounce pushes each kind out, `q = push/w` (`push = ½·w̄_c`, the
    // mean weight of the collective the core holds), and its hold returns it,
    // so its held matter rests in a band (docs/104 §6). The vacuum feels the
    // same law reflected through the throat, which stands at the core's
    // state, R = 1 (the enclosed law's base: throat 1, rim 2 in the
    // enclosure's own units): `R → 2 − R` (`through`). Past the rim the
    // vacuum stands outside this sphere.
    let throat = ratio(1, 1);
    let band = match heart_push {
        Some(push) if !h.is_zero() => {
            // On the inverse faces (docs/104 §6.2): the push on the unlocked
            // face u, the pull back on the held face g by the spin ℓ = R·H;
            // at rest where `R·R = w̄_c/w` (§6.3).
            let q = push.clone() / weight(&x.key);
            let at = |reach: &Ratio| band(&q, reach, h);
            if !x.vacuum {
                toward_rest(&q, r_o, h, &x.ell_o)
            } else {
                let mirrored = through(r_o, &throat);
                if mirrored > ratio(0, 1) {
                    -at(&mirrored)
                } else {
                    ratio(0, 1)
                }
            }
        }
        _ => ratio(0, 1),
    };
    // Each plane's spin: its share of the orbit spin, its own torque and
    // its share of the band (radial). A plane torqued past zero carries the
    // mote through the centre in that plane (its phase there on half a
    // turn), never piled at zero; the orbit spin is the L1 sum, the tilt
    // re-read from the planes.
    let mut planes_l: Vec<Ratio> = Vec::with_capacity(t.len());
    for (j, tk) in t.iter().enumerate() {
        let lk = x.ell_o.clone() * tk.clone()
            + p.clone() * d_o.clone() * resp[j].clone() * tk.clone()
            + band.clone() * tk.clone();
        if lk < ratio(0, 1) {
            if let Some(ph) = x.phase.get_mut(SPACE[j]) {
                *ph = frac(&(ph.clone() + ratio(1, 2)));
            }
            planes_l.push(-lk);
        } else {
            planes_l.push(lk);
        }
    }
    let raw = planes_l.iter().fold(ratio(0, 1), |s, l| s + l.clone());
    let new_o = crate::ring::grain(&raw);
    x.tilt = tilt_from(&planes_l);
    let dl = new_o.clone() - x.ell_o.clone();
    absorbed += m.clone()
        * if dl < ratio(0, 1) {
            -dl.clone()
        } else {
            dl.clone()
        };
    given += m.clone() * dl;
    x.ell_o = new_o;
    // The orthogonal spin (docs/90 D1): the release's torque in the orthogonal orbit's plane,
    // less the hold's return of the held half, `½·δ(R)·ℓ_⊥`, both from and
    // to the same reservoir as the orbit plane's (Σ m·(ℓ_o + ℓ_⊥) and the
    // reservoir exact).
    let new_c = crate::ring::grain(
        &(x.ell_orth.clone() + torque_orth.clone()
            - ratio(1, 2) * d_o.clone() * x.ell_orth.clone()),
    );
    let dl_c = new_c.clone() - x.ell_orth.clone();
    absorbed += m.clone()
        * if dl_c < ratio(0, 1) {
            -dl_c.clone()
        } else {
            dl_c.clone()
        };
    given += m.clone() * dl_c;
    x.ell_orth = new_c;
    (given, absorbed)
}

/// The contact of A pair over one echo (operators 2026-10-18, with
/// Xylarium: interaction is continuous, "not a walk, a dance"): the mean of
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
    let (lo, hi) = if psi1 > *psi0 {
        (psi0.clone(), psi1)
    } else {
        (psi1, psi0.clone())
    };
    let span = hi.clone() - lo.clone();
    // The half-turns strictly inside, in order.
    let mut cuts = vec![lo.clone()];
    let mut k = (lo.clone() * ratio(2, 1)).floor() + ratio(1, 1);
    while k.clone() / ratio(2, 1) < hi {
        cuts.push(k.clone() / ratio(2, 1));
        k += ratio(1, 1);
    }
    cuts.push(hi);
    cuts.windows(2).fold(ratio(0, 1), |acc, w| {
        acc + (w[1].clone() - w[0].clone()) / span.clone() * (f(&w[0]) + f(&w[1])) / ratio(2, 1)
    })
}

/// The shear (ruling of 2026-10-18: the kinematic pressure is sheared as an
/// attractor on opposing hemispheres to provide the shear on the orbit,
/// which turns the 'orthogonal orbit' into an 'orbit'; with Xylarium): neighbouring
/// rings of the flow turn at different rates, and where they stand in
/// contact the pressure between them moves turning outward. The inner falls
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
    // Each mote's share, exact (a reading; only what is stored stands on the
    // grain).
    let m: Vec<Ratio> = motes
        .iter()
        .map(|x| x.amount.clone() * weight(&x.key) / total.clone())
        .collect();
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
        let (to, tn) = (tilt_of(&motes[o].tilt), tilt_of(&motes[n_].tilt));
        let c = SPACE.iter().enumerate().fold(ratio(0, 1), |s, (j, &k)| {
            s + match (motes[o].phase.get(k), motes[n_].phase.get(k)) {
                (Some(a), Some(b)) => contact(
                    &(a.clone() - b.clone()),
                    &(ratio(2, 1) * (d[o].clone() * to[j].clone() - d[n_].clone() * tn[j].clone())),
                ),
                _ => ratio(0, 1),
            }
        }) / ratio(3, 1);
        let t = (if d[o] > d[n_] {
            d[o].clone() - d[n_].clone()
        } else {
            d[n_].clone() - d[o].clone()
        }) / (d[o].clone() + d[n_].clone());
        // The pair's unkept share held on the grain (the carrier, never the
        // law): the exchange below books both sides by the one factor, so
        // `Σ m·ℓ` stays exact whatever it is, and its readings' denominators
        // never compound pair on pair (profiled 2026-10-26: unheld, the
        // shear and the fluid were 96% of an echo).
        let u = crate::ring::grain(&(crate::ring::grain(&c) * crate::ring::grain(&t) * gap / m_ij));
        if u.is_zero() {
            continue;
        }
        // Both sides exact alike, so the exchange stays conserved:
        // m_o·gain = m_o·u·m_n = m_n·give.
        let give = u.clone() * m[o].clone();
        out[n_] += give.clone();
        moves.push((o, n_, u * m[n_].clone(), give));
    }
    let k: Vec<Ratio> = (0..n)
        .map(|i| {
            if out[i] > motes[i].ell_o && !out[i].is_zero() {
                motes[i].ell_o.clone() / out[i].clone()
            } else {
                ratio(1, 1)
            }
        })
        .collect();
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

/// The throat's inversion on the sphere (ruling of 2026-10-21: the second
/// double inversion must not be flattened): with the polar angle `θ =
/// 90°·R/H` (the throat at 90°, where δ = ½; the rim at 180°), inverting
/// through the throat reflects `θ → 180° − θ`, so `R → 2H − R`: the core and
/// the rim change places, the throat stands. Linear, exact, no square.
pub fn through(r: &Ratio, h: &Ratio) -> Ratio {
    ratio(2, 1) * h.clone() - r.clone()
}

/// The spray (with Xylarium; the founding epoch one tier down): matter a
/// heart throws out (an outburst, or the kinds a new star's heart releases)
/// leaves over directions, each kind at its own capture phase, `Z·p` with p
/// the star's own push share (the founding's ½ one tier up), `½ + Z·p` on the
/// orthogonal orbit. Thrown, never placed (operator 2026-10-22: "gravity and
/// physics should supply the required motion"): each kind leaves at the spin
/// the heart's cut carried it with (`carried`, in its system's units), which
/// already stands past the throat (the cut released it where its reach
/// outran the heart's hold, the double inversion's far face, operator
/// 2026-10-21); its system's enclosure reads its reach, and the star's torque
/// and hold settle it bell by bell, by its own weight.
pub fn spray(
    released: &Manifest,
    carried: &[(crate::chem::Kind, Ratio)],
    base: &[Ratio],
    agitation: &Ratio,
    born: u64,
    push: &Ratio,
) -> Vec<Mote> {
    let at = |k: usize| base.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
    released
        .iter()
        .filter(|(_, a)| **a > ratio(0, 1))
        .filter_map(|(k, a)| {
            // The capture phase: Z × the parent's push share (the star's own),
            // so the kinds leave over the whole turn, never two antipodes.
            let z = ratio(i64::from(crate::chem::charge(k)), 1) * push.clone();
            // From the star's own phase in each plane (docs/96: the planes
            // part by the star's own history, never a set offset).
            let mut phase = rest(crate::ring::grain(&frac(
                &(at(LOOP) + ratio(1, 2) + z.clone()),
            )));
            for &s in &SPACE {
                phase[s] = crate::ring::grain(&frac(&(at(s) + z.clone())));
            }
            let ell_o = crate::ring::grain(&carried.iter().find(|(c, _)| c == k)?.1);
            Some(Mote {
                key: k.clone(),
                amount: a.clone(),
                ell_o,
                ell_orth: ratio(0, 1),
                phase,
                agitation: crate::ring::grain(agitation),
                born,
                vacuum: false,
                lever: Vec::new(),
                lever_place: ratio(0, 1),
                tilt: Vec::new(),
            })
        })
        .collect()
}

/// The jetstream is A continuous orbit, the orthogonal orbit (operators 2026-10-16/17: "a
/// continuous flow, inbound and outbound"): the circulation's plane is the
/// orthogonal orbit, a mote's phase in it its place on the orthogonal orbit. The axial leg (out
/// along the torsion axis and over: phase in `[½, 1)`) carries matter in
/// transit; the disc leg (down at the rim and home through the disc: phase
/// in `[0, ½)`) is where the cut and collapse are read. No jet batch, no jet
/// exit: matter goes round, out and back at once.
pub fn on_disc_leg(x: &Mote) -> bool {
    frac(x.phase.get(LOOP).unwrap_or(&ratio(0, 1))) < ratio(1, 2)
}

/// The one exit, by state: a mote whose orthogonal orbit overreaches the rim (its
/// reach at or past the throat, `R = ℓ/H(<ℓ) ≥ H(<ℓ)`, read by the one
/// enclosure, δ at most ½) crosses whole into the medium one tier up, where the
/// enclosing core's orthogonal orbits carry it round the same way. Returns
/// the motes that left; the rest stay.
pub fn exits(motes: &mut Vec<Mote>, enc: &Enclosed) -> Vec<Mote> {
    // The rim is where the loop closes (docs/96 §7, the double pass: a state
    // returns only after `M∘M`): the image `2H − R` of the return leg reaches
    // the centre at `R = 2·base`, base the core's state. It is the rim law's
    // [throat 1, rim 2]. (It was the whole holding, which falls short of the
    // loop while the throat still holds unreleased grains.)
    let rim = ratio(2, 1) * enc.base();
    let (gone, stay): (Vec<Mote>, Vec<Mote>) = std::mem::take(motes)
        .into_iter()
        .partition(|x| enc.reach(&x.ell_o).0 >= rim);
    *motes = stay;
    gone
}

/// The galaxy's held and free manifests, read from its motes: on the
/// disc leg by the cut in its one enclosure; on the axial leg, in transit, never held.
pub fn read_cut(motes: &[Mote], enc: &Enclosed, p: &Ratio) -> (Manifest, Manifest) {
    let (mut kept, mut free) = (Manifest::new(), Manifest::new());
    for x in motes {
        let share = if on_disc_leg(x) {
            held(x, enc, p)
        } else {
            ratio(0, 1)
        };
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

/// A strand (docs/102: was "filament"; the filament is a core's flow) read whole (operators 2026-10-18, the cyclotronic chain (7):
/// "judged whole, with extent read as shares from band edge to band edge").
pub struct Strand {
    /// Its members, by index.
    pub members: Vec<usize>,
    /// Its holding-weighted reach.
    pub reach: Ratio,
    /// Its bond `λ/(1 + λ)`, `λ` its holding and magnetism over its apartness.
    pub bind: Ratio,
    /// Its holding-weighted agitation.
    pub agitation: Ratio,
    /// The unlocked face `½(1 − δ)` at its reach (a reading: its heat reaches
    /// the members as the light's warming, docs/96 §8).
    pub face: Ratio,
    /// Its extent as a share on the sphere, edge to edge.
    pub apart: Ratio,
}

impl Strand {
    /// It holds where its bond meets its agitation (its tide is in its
    /// apartness; the face heats and is in its agitation, docs/96 §8).
    pub fn holds(&self) -> bool {
        self.bind >= self.agitation
    }
}

/// The chains of a galaxy's disc leg (operators 2026-10-18: membership is
/// the chain, never a centre; with Xylarium). Motes on one face (the same
/// hemisphere in every plane, the S⁰ face test) whose dispersal bands
/// `r(1 ± p)` overlap link, and the links are transitive: a chain is a
/// connected run of overlapping bands on one face, read whole, no centre
/// chosen, no order (each face class sorted by its bands' inner edges, read
/// at n·log n). Each chain is judged by the filament law against its own
/// extent as a share on the sphere, edge to edge, never a length (operator:
/// "the only straight lines should be wormholes"): along the ring its orbit
/// phases' extent, `in_step`; across reach the deficit contrast of its
/// innermost member's INNER edge and its outermost member's outer edge
/// (a point has its own dispersal, so a lone mote's apartness is its own
/// band's, never zero). `apart = 1 − in_step(extent)·(1 − T_edges)`,
/// `λ = x/apart` with x its holding and magnetism in the galaxy's unit. A
/// long thin chain spreads faster than it gathers and fails; a dense one
/// binds.
/// The work a read spends, counted in its own steps (a comparison, an
/// addition): cost is read at N and 4N, never timed (MU-RECURSION rule 13).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Work(pub u64);

/// The hold along A FIBRE: at each member's reach, the sum of the densities
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

/// The motes within A REACH: of motes ordered by reach, the run standing
/// strictly within `d` of `r`, `|r_m − r| < d`, read by two bisections of
/// the order, never mote by mote.
pub fn within_reach<'a>(
    by_reach: &'a [(Ratio, usize)],
    r: &Ratio,
    d: &Ratio,
    work: &mut Work,
) -> &'a [(Ratio, usize)] {
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

/// The beads (ruling of 2026-10-18: the individual fibres are beaded, then
/// the fibres braided together): the matter's own gatherings, read every
/// echo. A FIBRE is one kind's run on one face (the same hemisphere in every
/// plane), motes whose dispersal bands `r(1 ± p)` overlap, linked
/// transitively; it pinches where its hold gathers (each local maximum) and
/// parts at the minima between. Each bead is one body of gas. Returned by
/// face: (its members, the reach where it pinches). Read by the one
/// enclosure: each mote's reach `ℓ/H(<ℓ)` about the hold it turns in. The
/// collapse reads the disc leg alone (`disc_leg`); a body of gas is a body on
/// either leg (the face's orthogonal hemisphere is the leg), so the merge
/// reads both.
pub fn beads(
    motes: &[Mote],
    enc: &Enclosed,
    p: &Ratio,
    total: &Ratio,
    disc_leg: bool,
) -> std::collections::BTreeMap<Vec<bool>, Vec<(Vec<usize>, Ratio)>> {
    let mut beads_by_face: std::collections::BTreeMap<Vec<bool>, Vec<(Vec<usize>, Ratio)>> =
        std::collections::BTreeMap::new();
    if total.is_zero() {
        return beads_by_face;
    }
    let half = ratio(1, 2);
    let (down, up) = (
        ratio(1, 1) - p.clone().min(ratio(1, 1)),
        ratio(1, 1) + p.clone(),
    );
    let hh: Vec<Ratio> = motes.iter().map(|x| enc.hold(&x.ell_o)).collect();
    let rr: Vec<Ratio> = motes
        .iter()
        .zip(&hh)
        .map(|(x, h)| x.ell_o.clone() / h.clone())
        .collect();
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
                (
                    lov[j].clone(),
                    hiv[j].clone(),
                    if band.is_zero() { m } else { m / band },
                )
            })
            .collect();
        let reach: Vec<Ratio> = along.iter().map(|&j| rr[j].clone()).collect();
        hold_along(&reach, &bands, &mut Work::default())
    };
    // The fibres (ruling of 2026-10-18: the individual fibres are beaded,
    // then the fibres braided together): a fibre is one kind's run on one
    // face, motes whose dispersal bands overlap, linked transitively.
    let mut fibres: std::collections::BTreeMap<(Vec<bool>, crate::chem::Kind), Vec<usize>> =
        std::collections::BTreeMap::new();
    for (i, x) in motes.iter().enumerate() {
        if (!disc_leg || on_disc_leg(x)) && !rr[i].is_zero() {
            fibres
                .entry((
                    (0..PLANES)
                        .map(|k| frac(x.phase.get(k).unwrap_or(&ratio(0, 1))) < half)
                        .collect(),
                    x.key.clone(),
                ))
                .or_default()
                .push(i);
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
            // The beads (ruling: halving is an imposed measure, whereas
            // nature measures itself): a fibre
            // pinches where its hold gathers (each local maximum) and parts
            // at the minima between, the minimum member joining the side
            // whose neighbour holds more.
            let mut cuts: Vec<usize> = Vec::new();
            for i in 1..along.len().saturating_sub(1) {
                if local[i] < local[i - 1] && local[i] <= local[i + 1] {
                    cuts.push(if local[i - 1] >= local[i + 1] {
                        i + 1
                    } else {
                        i
                    });
                }
            }
            let mut from = 0;
            let mut push = |a: usize, b: usize| {
                let (k, _) = (a..b)
                    .map(|i| (i, local[i].clone()))
                    .max_by(|x, y| x.1.cmp(&y.1).then(y.0.cmp(&x.0)))
                    .expect("a member");
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

pub fn strands(motes: &[Mote], enc: &Enclosed, p: &Ratio, total: &Ratio) -> Vec<Strand> {
    if total.is_zero() {
        return Vec::new();
    }
    // Each mote's reach and band, read once by the one enclosure (exact;
    // recomputing them at every use was most of the cost): its reach `ℓ/H(<ℓ)`
    // and the hold it turns about.
    let (down, up) = (
        ratio(1, 1) - p.clone().min(ratio(1, 1)),
        ratio(1, 1) + p.clone(),
    );
    let hh: Vec<Ratio> = motes.iter().map(|x| enc.hold(&x.ell_o)).collect();
    let rr: Vec<Ratio> = motes
        .iter()
        .zip(&hh)
        .map(|(x, h)| x.ell_o.clone() / h.clone())
        .collect();
    let lov: Vec<Ratio> = rr.iter().map(|r| r.clone() * down.clone()).collect();
    let hiv: Vec<Ratio> = rr.iter().map(|r| r.clone() * up.clone()).collect();
    // The deficit at a reach about the hold a member turns about.
    let delta_of = |j: usize, r: &Ratio| hh[j].clone() / (r.clone() + hh[j].clone());
    // A group judged whole by the filament law, edge to edge.
    let judge = |members: Vec<usize>| -> Option<Strand> {
        let (mut x, mut held, mut a_w, mut r_w) =
            (ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1));
        for &j in &members {
            let m = motes[j].amount.clone() * weight(&motes[j].key) / total.clone();
            // The additive bond (docs/103 §6): holding, open frames and the
            // nuclei's grip, each a share of the same reach.
            let g_j = crate::polarity::faces(&delta_of(j, &rr[j])).0;
            x += m.clone() * (ratio(1, 1) + open(&motes[j].key) + grip_share(&motes[j], &g_j));
            a_w += m.clone() * motes[j].agitation.clone();
            r_w += m.clone() * rr[j].clone();
            held += m;
        }
        if held.is_zero() {
            return None;
        }
        let j_in = *members
            .iter()
            .min_by(|&&a, &&b| lov[a].cmp(&lov[b]).then(a.cmp(&b)))
            .expect("a member");
        let j_out = *members
            .iter()
            .max_by(|&&a, &&b| hiv[a].cmp(&hiv[b]).then(b.cmp(&a)))
            .expect("a member");
        let (d_in, d_out) = (delta_of(j_in, &lov[j_in]), delta_of(j_out, &hiv[j_out]));
        let t_edges = (d_in.clone() - d_out.clone()) / (d_in + d_out);
        // Its extent round the orbit: in each spatial plane, their mean.
        let extent = space_mean(|k| {
            let orbit: Vec<Ratio> = members
                .iter()
                .map(|&j| frac(motes[j].phase.get(k).unwrap_or(&ratio(0, 1))))
                .collect();
            orbit.iter().max().expect("a member").clone()
                - orbit.iter().min().expect("a member").clone()
        });
        let apart = ratio(1, 1) - in_step(&extent, &ratio(0, 1)) * (ratio(1, 1) - t_edges);
        let reach_w = r_w / held.clone();
        // The hold the chain turns about, by holding.
        let h_w = members.iter().fold(ratio(0, 1), |s, &j| {
            s + motes[j].amount.clone() * weight(&motes[j].key) / total.clone() * hh[j].clone()
        }) / held.clone();
        let face = (ratio(1, 1) - h_w.clone() / (reach_w.clone() + h_w)) / ratio(2, 1);
        // A chain with no extent at all (no push to spread it) is not read as
        // infinitely dense: it does not bind.
        let bind = if apart.is_zero() {
            ratio(0, 1)
        } else {
            crate::formation::bond(&(x / apart.clone()))
        };
        Some(Strand {
            members,
            reach: reach_w,
            bind,
            agitation: a_w / held,
            face,
            apart,
        })
    };
    let beads_by_face = beads(motes, enc, p, total, true);
    // The braid: beads of different fibres twine where each one's pinch
    // stands within the other's extent (mutual, so no order), and the
    // twining carries across; each braid is read whole by the filament law.
    let mut out = Vec::new();
    for (_, beads) in beads_by_face {
        let n = beads.len();
        let span: Vec<(Ratio, Ratio)> = beads
            .iter()
            .map(|(m, _)| {
                (
                    m.iter().map(|&j| lov[j].clone()).min().expect("a member"),
                    m.iter().map(|&j| hiv[j].clone()).max().expect("a member"),
                )
            })
            .collect();
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
        let mut braids: std::collections::BTreeMap<usize, Vec<usize>> =
            std::collections::BTreeMap::new();
        for i in 0..n {
            let r = root(&mut parent, i);
            braids
                .entry(r)
                .or_default()
                .extend(beads[i].0.iter().copied());
        }
        out.extend(braids.into_values().filter_map(&judge));
    }
    out
}

/// Streaming by force (ruling of 2026-10-18: jetstreams are continuums or
/// vacuums in the filament by their force): a mote's stream is READ, never labelled by its origin.
/// Where the jet's push at it (`p·ρ·δ_u`, the torque's own reading) beats the
/// hold of the filament it stands in (`λ/(1 + λ)`), it is carried in the
/// continuum, one moving body with the current; where the hold beats the
/// push, the jet cannot fill the channel and it drifts in the vacuum, the
/// thin counter-moving flank. A mote in no filament (in transit on the
/// axial leg) is carried. Returns how many stand in the vacuum.
///
/// A reading, never a write (docs/102 A1: one writer per state): a mote's
/// face is the luminal bound's alone (`luminal`); this counts the motes whose
/// strand's hold beats the push, never relabels them.
pub fn stream(motes: &[Mote], strands: &[Strand], push: &dyn Fn(&Mote) -> Ratio) -> usize {
    let mut hold: Vec<Option<Ratio>> = vec![None; motes.len()];
    for c in strands {
        for &j in &c.members {
            hold[j] = Some(c.bind.clone());
        }
    }
    motes
        .iter()
        .zip(hold)
        .filter(|(x, h)| h.as_ref().is_some_and(|h| *h >= push(x)))
        .count()
}

/// The bindings of chains already read: those that hold collapse.
/// The disc forms (ruling of 2026-10-26: this is why galaxies form discs;
/// align on nature): angular momentum is kept while the motion off
/// the plane of the net spin is damped away, so matter settles into its
/// galaxy's spin plane. The plane is a reading: the galaxy's tilt, its
/// matter's plane spins added (`blend`, by `m·ℓ`). Each mote's tilt returns
/// toward it by the attractor law ruled for stars (2026-10-18: "the spin has
/// an orbit and a tilt as attractors ... return each echo `d/(d +
/// turned)`"): it keeps `turned/(d + turned)` of its displacement, `d = 1 −
/// δ` its drag at its reach and `turned = 2δ` its own turning a bell. A
/// convex share, so every tilt stays a partition of one. Returns the
/// galaxy's tilt.
pub fn settle_disc(motes: &mut [Mote], enc: &Enclosed) -> Vec<Ratio> {
    let plane = blend(
        &motes
            .iter()
            .map(|x| {
                (
                    x.amount.clone() * weight(&x.key),
                    x.ell_o.clone(),
                    x.tilt.clone(),
                )
            })
            .collect::<Vec<_>>(),
    );
    if plane.is_empty() {
        return plane;
    }
    for x in motes.iter_mut() {
        let d = enc.reach(&x.ell_o).1;
        let drag = ratio(1, 1) - d.clone();
        let turned = ratio(2, 1) * d;
        if (drag.clone() + turned.clone()).is_zero() {
            continue;
        }
        let keep = turned.clone() / (drag + turned);
        let t = tilt_of(&x.tilt);
        let moved: Vec<Ratio> = t
            .iter()
            .zip(&plane)
            .map(|(tk, pk)| pk.clone() + (tk.clone() - pk.clone()) * keep.clone())
            .collect();
        x.tilt = tilt_from(&moved);
    }
    plane
}

/// The GALAXY settles its stars (ruling of 2026-10-26: the stars settle the
/// planets, the planets their moons, and the galaxy its stars, since
/// otherwise the reading is saturated with data; on the strip, Xylarium 11.0
/// (51)(g)): the one law of `formation::settle_system`, one tier up, in the
/// galaxy's own enclosure. On the strip `ρ = R/H − 1` about the galaxy's ring
/// (H its throat's base), every band is a pair, its outbound span and its
/// image through the ring; each band steps by the push of what stands inside
/// its pair against the pull of what stands outside, `k(ρ) = (S/G)·(1 +
/// ρ)/(1 − ρ)`, opening at the ring's own dispersal pair `(S − G)/(S + G)`
/// at ρ = 0. Each mote's matter is spread over its band `r(1 ± p)`. A band's
/// pair holds where its matter's bond meets its heat and the tide across
/// it (`formation::crest_collapse`, docs/96 §8); a band where a
/// star already stands is that star's (its capture reads it). Returns the
/// holding bands as bindings: (the band's middle reach, each member's share),
/// every member on the disc leg (docs/99 §2.1).
pub fn band_pairs(
    motes: &[Mote],
    enc: &Enclosed,
    p: &Ratio,
    plane: &[Ratio],
    g_total: &Ratio,
) -> Vec<(Ratio, Vec<(usize, Ratio)>)> {
    let ring = enc.base();
    let whole = enc.whole();
    if ring.is_zero() || whole.is_zero() || motes.is_empty() {
        return Vec::new();
    }
    let total = holding(motes);
    let p = p.clone().max(ratio(0, 1)).min(ratio(1, 2));
    let share_in = |lo: &Ratio, hi: &Ratio, a: &Ratio, b: &Ratio| -> Ratio {
        if hi <= lo {
            return if lo >= a && lo < b {
                ratio(1, 1)
            } else {
                ratio(0, 1)
            };
        }
        let x = if lo > a { lo.clone() } else { a.clone() };
        let y = if hi < b { hi.clone() } else { b.clone() };
        if y <= x {
            ratio(0, 1)
        } else {
            (y - x) / (hi.clone() - lo.clone())
        }
    };
    // Shares of the whole: the motes carry the galaxy's loose holding.
    let loose = whole.clone() - ring.clone();
    let m_of = |i: usize| {
        if total.is_zero() {
            ratio(0, 1)
        } else {
            motes[i].amount.clone() * weight(&motes[i].key) / total.clone() * loose.clone()
                / whole.clone()
        }
    };
    let pair = |x: &Ratio| {
        (
            ring.clone() * (ratio(1, 1) - x.clone()),
            ring.clone() * (ratio(1, 1) + x.clone()),
        )
    };
    let band = |i: usize| {
        let r = enc.reach(&motes[i].ell_o).0;
        (
            r.clone() * (ratio(1, 1) - p.clone()),
            r * (ratio(1, 1) + p.clone()),
        )
    };
    let bands_all: Vec<(Ratio, Ratio)> = (0..motes.len()).map(band).collect();
    let inside = |x: &Ratio| -> Ratio {
        let (a, b) = pair(x);
        (0..motes.len()).fold(ring.clone() / whole.clone(), |acc, i| {
            acc + m_of(i) * share_in(&bands_all[i].0, &bands_all[i].1, &a, &b)
        })
    };
    let s0 = ring.clone() / whole.clone();
    let rho0 = (ratio(2, 1) * s0 - ratio(1, 1)).max(ratio(0, 1));
    let mut rhos: Vec<Ratio> = vec![ratio(0, 1)];
    if rho0 > ratio(0, 1) && rho0 < ratio(1, 1) {
        rhos.push(crate::ring::grain(&rho0));
    }
    while let Some(at) = rhos.last().cloned() {
        if at >= ratio(1, 1) || at <= ratio(0, 1) || rhos.len() >= 256 {
            break;
        }
        let s_in = inside(&at);
        let g_out = ratio(1, 1) - s_in.clone();
        if g_out <= ratio(0, 1) {
            break;
        }
        let k_at = s_in / g_out * (ratio(1, 1) + at.clone()) / (ratio(1, 1) - at.clone());
        if k_at <= ratio(1, 1) {
            break;
        }
        let next = crate::ring::grain(&(at * k_at));
        rhos.push(if next < ratio(1, 1) {
            next
        } else {
            ratio(1, 1)
        });
    }
    if *rhos.last().expect("one") < ratio(1, 1) {
        rhos.push(ratio(1, 1));
    }
    let mut out = Vec::new();
    for w in rhos.windows(2) {
        let (x, y) = (w[0].clone(), w[1].clone());
        if y <= x {
            continue;
        }
        let (o_lo, o_hi) = (
            ring.clone() * (ratio(1, 1) + x.clone()),
            ring.clone() * (ratio(1, 1) + y.clone()),
        );
        let (i_lo, i_hi) = (
            ring.clone() * (ratio(1, 1) - y.clone()),
            ring.clone() * (ratio(1, 1) - x.clone()),
        );
        // A binding gathers on the disc leg (docs/99 §2.1): the axial leg is
        // matter in transit, half a loop from the disc; the rings above
        // step on all the galaxy's matter, the binding's own is the disc's.
        let members: Vec<(usize, Ratio)> = (0..motes.len())
            .filter(|&i| on_disc_leg(&motes[i]))
            .filter_map(|i| {
                let (lo, hi) = &bands_all[i];
                let s = share_in(lo, hi, &o_lo, &o_hi)
                    + share_in(lo, hi, &i_lo.clone().max(ratio(0, 1)), &i_hi);
                (s > ratio(0, 1)).then(|| (i, crate::ring::grain(&s).min(ratio(1, 1))))
            })
            .filter(|(_, s)| *s > ratio(0, 1))
            .collect();
        if members.is_empty() {
            continue;
        }
        // The ring fragments (the disc's local collapse, nature's way; the
        // crest law, `formation::crest_collapse`, with Xylarium): the ring's
        // holding m and magnetism μ, its heat a, at its middle R about the
        // hold H there, hold along a length w_J against their heat and the
        // shear, so ⌊R/w_J⌋ bindings stand around the turn. Each member
        // joins the binding whose arc holds it, by its phase in the galaxy's
        // disc plane (the plane the disc settled into, `settle_disc`).
        let mid = (o_lo.clone() + o_hi.clone()) / ratio(2, 1);
        let h_mid = enc.hold_within(&mid);
        let held_of = |i: usize, s: &Ratio| {
            motes[i].amount.clone() * s.clone() * weight(&motes[i].key) / g_total.clone()
        };
        let m = members
            .iter()
            .fold(ratio(0, 1), |acc, (i, s)| acc + held_of(*i, s));
        let mu = members.iter().fold(ratio(0, 1), |acc, (i, s)| {
            acc + motes[*i].amount.clone()
                * s.clone()
                * ratio(i64::from(crate::chem::unpaired(&motes[*i].key)), 1)
                / g_total.clone()
        });
        if m.is_zero() {
            continue;
        }
        let a = crate::ring::grain(
            &(members.iter().fold(ratio(0, 1), |acc, (i, s)| {
                acc + held_of(*i, s) * motes[*i].agitation.clone()
            }) / m.clone()),
        );
        // Its members' net lead (their tilts blended by holding: aligned
        // leads add, opposed cancel) against the disc plane, by holding.
        let lead = blend(
            &members
                .iter()
                .map(|(i, s)| {
                    (
                        held_of(*i, s),
                        motes[*i].ell_o.clone(),
                        motes[*i].tilt.clone(),
                    )
                })
                .collect::<Vec<_>>(),
        );
        let tau = m.clone() * torsion(&lead, plane);
        // The nuclear layer ν: the members' holding by their nuclei's grip.
        let nu = members.iter().fold(ratio(0, 1), |acc, (i, s)| {
            acc + held_of(*i, s)
                * grip_share(
                    &motes[*i],
                    &crate::polarity::faces(&enc.reach(&motes[*i].ell_o).1).0,
                )
        });
        let Some((_, count)) =
            crate::formation::crest_collapse(&m, &mu, &tau, &nu, &a, &mid, &h_mid)
        else {
            continue;
        };
        if count == 0 {
            continue;
        }
        let disc = SPACE[tilt_of(plane)
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.cmp(y.1).then(y.0.cmp(&x.0)))
            .map(|(j, _)| j)
            .unwrap_or(0)];
        let n = Ratio::from_integer(num_bigint::BigInt::from(count));
        let mut arcs: std::collections::BTreeMap<num_bigint::BigInt, Vec<(usize, Ratio)>> =
            std::collections::BTreeMap::new();
        for (i, sh) in members {
            let at = frac(motes[i].phase.get(disc).unwrap_or(&ratio(0, 1)));
            arcs.entry((at * n.clone()).floor().to_integer())
                .or_default()
                .push((i, sh));
        }
        for (_, binding) in arcs {
            out.push((crate::ring::grain(&mid), binding));
        }
    }
    out
}

/// Merge by the matter's own gatherings (ruling of 2026-10-23: no grids,
/// since they only distort and provide rigidity;
/// design law 3, no cells in a fixed grid): the motes of one bead (`beads`:
/// one kind on one face, a fibre's run between its hold's minima, one body
/// of gas) and of one stream are one mote. Holding sums exactly; spins and
/// heat join by holding onto the grain, the floored spin's remainder
/// returned for the core's reservoir so the spin ledger stays exact; both
/// full phases are read (ascent), meaned by holding on the circle, and the
/// mean passes through the cross at the merged mote's place. A mote in no
/// bead (on the axial leg, in transit) stands as it is. Returns the spin
/// remainder.
pub fn merge(
    motes: &mut Vec<Mote>,
    beads: &std::collections::BTreeMap<Vec<bool>, Vec<(Vec<usize>, Ratio)>>,
    total: &Ratio,
    place_of: &dyn Fn(&Mote) -> Option<Ratio>,
    cross: &mut crate::enfold::Cross,
) -> Ratio {
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
    let mut at: std::collections::BTreeMap<(usize, bool), usize> =
        std::collections::BTreeMap::new();
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
            let exact_o =
                (my.clone() * y.ell_o.clone() + mx.clone() * x.ell_o.clone()) / both.clone();
            let exact_c =
                (my.clone() * y.ell_orth.clone() + mx.clone() * x.ell_orth.clone()) / both.clone();
            let (go, gc) = (g(&exact_o), g(&exact_c));
            if !total.is_zero() {
                remainder += both.clone() * weight(&x.key) / total.clone()
                    * ((exact_o - go.clone()) + (exact_c - gc.clone()));
            }
            let w = weight(&x.key);
            let heat = (my.clone() * y.agitation.clone() + mx.clone() * x.agitation.clone())
                / both.clone();
            y.agitation = g(&heat);
            // The tilts join as their plane spins add (docs/96 step 1).
            y.tilt = blend(&[
                (my.clone(), y.ell_o.clone(), y.tilt.clone()),
                (mx.clone(), x.ell_o.clone(), x.tilt.clone()),
            ]);
            y.ell_o = go;
            y.ell_orth = gc;
            let place = place_of(y);
            // Both true phases (ascent), before the amounts join.
            let (ty, tx): (Vec<Ratio>, Vec<Ratio>) = (
                (0..y.phase.len()).map(|k| true_phase(y, k)).collect(),
                (0..x.phase.len()).map(|k| true_phase(&x, k)).collect(),
            );
            y.amount = both.clone();
            let mut levers: Vec<(usize, Ratio, Ratio)> = Vec::new();
            for k in 0..ty.len().min(tx.len()) {
                cross.note(k, crate::enfold::Fate::Ascended, &ratio(0, 1));
                let mean = circle_mean(&[
                    (my.clone() * w.clone(), ty[k].clone()),
                    (mx.clone() * w.clone(), tx[k].clone()),
                ]);
                let shift = mean.clone() - ty[k].clone();
                cross.note(k, crate::enfold::Fate::Condensed, &shift);
                let (held, scar, lever) =
                    crate::enfold::leaf(&mean, place.as_ref(), &heat, k, cross);
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
pub fn founding(grains: &Manifest, span: Option<&Ratio>) -> Option<Vec<Mote>> {
    let Some((p, _, mean)) = crate::pulse::galaxy_push(grains) else {
        return Some(Vec::new());
    };
    // The phases stand against the source's own bell (docs/95). The captures turn them (with Xylarium): each pass of the
    // outbound stream through the throat is one capture and turns the
    // crossing point by a = ½, so kind Z leaves at the source's phase plus
    // `Z·½`, and on the orthogonal orbit it starts on the axial leg, outbound (`½ +
    // Z·½`). Two groups at the founding, parted by each kind's own orthogonal orbit rate
    // `δ(R_z)` at its own reach thereafter. (The per-universe offset, the
    // hypha-assigned TUniversal position, is owed with Mycelium.)
    // from the ring (operator 2026-10-25: "make worlds appear"; docs/94 §2:
    // nothing can orbit inside the ring, where `2δ > 1` breaks the luminal
    // bound): with `span` (the matter's share of the enclosure's hold, the
    // disc's span from the ring at 1 to the rim at `1 + span`: 1 for a
    // galaxy, whose state is its grains, the home's share of the universe
    // for the home), kind z flows out from the ring to `R_z = 1 +
    // span·p·w̄/w_z`, the heaviest nearest the ring, turning at the balance of
    // that place `ℓ_z = R_z·H(<R_z)` (the hold by place: the ring's state 1
    // and every kind nearer, each its share times the span). Without a span
    // (the firmament, the Wellspring's held kinds at the singularity) it is
    // carried from the centre, `ℓ_z = p·w̄/w_z`.
    let total = crate::pulse::mass(grains);
    if total.is_zero() {
        return Some(Vec::new());
    }
    let kinds: Vec<(&crate::chem::Kind, &Ratio, Ratio, Ratio)> = grains
        .iter()
        .filter(|(_, a)| **a > ratio(0, 1))
        .map(|(k, a)| {
            let carried = p.clone() * mean.clone() / weight(k);
            let place = match span {
                Some(sp) => ratio(1, 1) + sp.clone() * carried,
                None => carried,
            };
            (
                k,
                a,
                place,
                span.cloned().unwrap_or_else(|| ratio(1, 1)) * a.clone() * weight(k)
                    / total.clone(),
            )
        })
        .collect();
    let hold_at = |r: &Ratio| {
        kinds
            .iter()
            .filter(|(_, _, rj, _)| rj < r)
            .fold(ratio(1, 1), |s, (_, _, _, sj)| s + sj.clone())
    };
    Some(
        kinds
            .iter()
            .map(|(k, a, r, _)| {
                // The captures' half turn a pass, in every plane alike.
                let turn = ratio(i64::from(crate::chem::charge(k)), 2);
                let mut phase = rest(frac(&(ratio(1, 2) + turn.clone())));
                for &s in &SPACE {
                    phase[s] = frac(&turn);
                }
                let ell = if span.is_some() {
                    r.clone() * hold_at(r)
                } else {
                    r.clone()
                };
                Mote {
                    key: (*k).clone(),
                    amount: (*a).clone(),
                    ell_o: crate::ring::grain(&ell),
                    ell_orth: ratio(0, 1),
                    phase,
                    agitation: ratio(0, 1),
                    born: 0,
                    vacuum: false,
                    lever: Vec::new(),
                    lever_place: ratio(0, 1),
                    tilt: Vec::new(),
                }
            })
            .collect(),
    )
}

/// The MÖBIUS jetstream's map (docs/94 §2; operator 2026-10-25: "vacuum-
/// continuum inversion needs to be a mobius inversion through the ocean as a
/// jetstream current"), first degree in the deficit: `(δ, ψ, s) ↦ (1 − δ,
/// ψ + ½, −s)`. On a mote it is its leg turned (continuum ↔ vacuum: the
/// vacuum leg reads its pull at `1 − δ` and runs its circulation the other
/// way, its sense flipped) and its orbit phase half a turn on. Twice is the
/// identity.
pub fn mobius(x: &mut Mote) {
    x.vacuum = !x.vacuum;
    antipode(&mut x.phase);
}

/// The luminal bound (docs/94 §2, docs/98 §4; the UMD kinematic law,
/// SUBSTRATE_C = 1): no particle turns more than one whole orbit in a
/// K-step. A mote turns `2·share` a bell, its share `δ` on the continuum face
/// and `1 − δ` on the vacuum face; where it exceeds ½ (a continuum mote
/// inside the ring, a vacuum mote outside it) the mote would fracture, so it
/// takes the Möbius jetstream onto the other face, where its share is at
/// most ½. Either face, one law. Returns how many took it.
pub fn luminal(motes: &mut [Mote], enc: &Enclosed) -> usize {
    let mut n = 0;
    for x in motes.iter_mut() {
        let d = enc.reach(&x.ell_o).1;
        let share = if x.vacuum { ratio(1, 1) - d } else { d };
        if share > ratio(1, 2) {
            mobius(x);
            n += 1;
        }
    }
    n
}

/// The fluid between rings (docs/94 §4, §6, §7; operator 2026-10-25: "derive
/// the missing components - fluid-MHD and UMD overlay"; "build it"). The
/// rings are the fluid's own beads (`beads`, one kind on one face). Each
/// ring reads, never stores: its holding `m`, its spin `ℓ̄` and heat `ā`
/// (holding-weighted), its reach `R` (its pinch), its width `2pR` (its
/// band's spread), its density `ρ = m/(2pR)`, its pressure `P = ρ·ā` plus its
/// magnetic pressure, and its magnetization `M = Σ u·in_step(φ, φ̄)` (each
/// member's open frames by its field phase's in-step with the ring's mean,
/// the field phase its circulation phase, frozen into the particle, so the
/// rings' differential turning winds the field). Between neighbouring rings
/// on a face (the inner at the lesser reach) two exchanges move turning
/// outward by the exchange law (the outer gains `u·m_in`, the inner gives
/// `u·m_out`, `u = share·(ℓ̄_out − ℓ̄_in)/(m_in + m_out)`, Σ m·ℓ unchanged):
/// the pressure's, whichever ring presses harder expanding, its share
/// `|P_in − P_out|/(P_in + P_out)`; and the field's (magnetic shear), its
/// share `|M_in − M_out|/(M_in + M_out)` times the rings' gear contrast
/// `|δ_in − δ_out|/(δ_in + δ_out)`. First degree, L1, saturating shares;
/// every member of a ring moves alike. Returns (the pressure's spin moved,
/// the field's spin moved, the spin moved around the shells' circles, and the
/// number of shell planes that parted into cells), by holding.
pub fn fluid(
    motes: &mut [Mote],
    enc: &Enclosed,
    p: &Ratio,
    total: &Ratio,
) -> (Ratio, Ratio, Ratio, usize) {
    if total.is_zero() || motes.len() < 2 {
        return (ratio(0, 1), ratio(0, 1), ratio(0, 1), 0);
    }
    let rings_by_face = beads(motes, enc, p, total, false);
    let share = |x: &Mote| x.amount.clone() * weight(&x.key) / total.clone();
    let mut dl = vec![ratio(0, 1); motes.len()];
    let (mut moved_p, mut moved_b) = (ratio(0, 1), ratio(0, 1));
    // docs/110: the spin the balance around the shells' circles moved, and
    // how many shell planes parted into two or more cells.
    let (mut moved_around, mut parted) = (ratio(0, 1), 0usize);
    for rings in rings_by_face.values() {
        // Each ring read once: (members, m, ℓ̄, R, P, M, δ).
        let mut read: Vec<(Vec<usize>, Ratio, Ratio, Ratio, Ratio, Ratio, Ratio)> = rings
            .iter()
            .filter_map(|(members, pinch)| {
                let m = members
                    .iter()
                    .fold(ratio(0, 1), |a, &i| a + share(&motes[i]));
                if m.is_zero() {
                    return None;
                }
                let ell = members.iter().fold(ratio(0, 1), |a, &i| {
                    a + share(&motes[i]) * motes[i].ell_o.clone()
                }) / m.clone();
                let heat = members.iter().fold(ratio(0, 1), |a, &i| {
                    a + share(&motes[i]) * motes[i].agitation.clone()
                }) / m.clone();
                let field_mean = circle_mean(
                    &members
                        .iter()
                        .map(|&i| {
                            (
                                share(&motes[i]),
                                motes[i]
                                    .phase
                                    .get(LOOP)
                                    .cloned()
                                    .unwrap_or_else(|| ratio(0, 1)),
                            )
                        })
                        .collect::<Vec<_>>(),
                );
                let magnet = members.iter().fold(ratio(0, 1), |a, &i| {
                    a + motes[i].amount.clone()
                        * ratio(i64::from(crate::chem::unpaired(&motes[i].key)), 1)
                        / total.clone()
                        * in_step(motes[i].phase.get(1).unwrap_or(&ratio(0, 1)), &field_mean)
                });
                let width = ratio(2, 1) * p.clone() * pinch.clone();
                let pressure = if width.is_zero() {
                    ratio(0, 1)
                } else {
                    (m.clone() * heat + magnet.clone()) / width
                };
                let delta = enc.reach(&ell).1;
                // Each ring's readings held on the grain (the carrier, never the
                // law; the exchange books both sides by one factor, so `Σ m·ℓ`
                // stays exact): unheld, their quotients compounded ring on ring.
                let g = crate::ring::grain;
                Some((
                    members.clone(),
                    m,
                    g(&ell),
                    pinch.clone(),
                    g(&pressure),
                    g(&magnet),
                    g(&delta),
                ))
            })
            .collect();
        read.sort_by(|a, b| a.3.cmp(&b.3));
        let mut columns: Vec<Ratio> = vec![ratio(0, 1); read.len()];
        // The medium's STATE (docs/103 §4): each ring bears the hold of every
        // ring beyond it on its face, `P_h = Σ m_j·δ_j` (the column's weight,
        // gravity's held share at each), a reading of state beside its
        // thermal and magnetic pressure; the contrast of the whole moves the
        // spin (the coupling into motion).
        let mut column = ratio(0, 1);
        for (i, r) in read.iter_mut().enumerate().rev() {
            let own = r.1.clone() * r.6.clone();
            columns[i] = crate::ring::grain(&column);
            r.4 = crate::ring::grain(&(r.4.clone() + column.clone()));
            column += own;
        }
        for w in read.windows(2) {
            let (inn, out) = (&w[0], &w[1]);
            let gap = out.2.clone() - inn.2.clone();
            let m_ij = inn.1.clone() + out.1.clone();
            if gap <= ratio(0, 1) || m_ij.is_zero() {
                continue;
            }
            let contrast = |a: &Ratio, b: &Ratio| {
                let s = a.clone() + b.clone();
                if s.is_zero() {
                    ratio(0, 1)
                } else {
                    (if a > b {
                        a.clone() - b.clone()
                    } else {
                        b.clone() - a.clone()
                    }) / s
                }
            };
            // S3 (docs/106 §2): the pressure's exchange is the hydrostatic
            // balance restoring the pair, signed (outward where the inner
            // ring is over-pressured, inward where under), zero at balance;
            // the field's (magnetic shear) stays the contrast, outward.
            let one = ratio(1, 1);
            let s_p = restoring(
                &inn.4,
                &out.4,
                &(one.clone() - inn.6.clone()),
                &(one - out.6.clone()),
            );
            let s_b = contrast(&inn.5, &out.5) * contrast(&inn.6, &out.6);
            for (s, moved) in [(s_p, &mut moved_p), (s_b, &mut moved_b)] {
                if s.is_zero() {
                    continue;
                }
                let u = crate::ring::grain(&(crate::ring::grain(&s) * gap.clone() / m_ij.clone()));
                let (gain, give) = (u.clone() * inn.1.clone(), u * out.1.clone());
                for &i in &out.0 {
                    dl[i] += gain.clone();
                }
                for &i in &inn.0 {
                    dl[i] -= give.clone();
                }
                let spin = out.1.clone() * gain;
                *moved += if spin < ratio(0, 1) { -spin } else { spin };
            }
        }
        // docs/110: the balance on each shell's fibre. A shell is the motes of
        // this face whose bands `R(1 ± p)` overlap (connected by their bands,
        // the band of `band_pairs`); within it, cell against cell around
        // each spatial plane's circle.
        let column_of: std::collections::BTreeMap<usize, Ratio> = read
            .iter()
            .enumerate()
            .flat_map(|(ri, r)| r.0.iter().map(move |&i| (i, ri)))
            .map(|(i, ri)| (i, columns[ri].clone()))
            .collect();
        let mut on_face: Vec<(usize, Ratio, Ratio)> = read
            .iter()
            .flat_map(|r| r.0.iter().copied())
            .map(|i| {
                let (r, d) = enc.reach(&motes[i].ell_o);
                (i, r, d)
            })
            .collect();
        on_face.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        let one = ratio(1, 1);
        let lo = |r: &Ratio| r.clone() * (one.clone() - p.clone());
        let hi = |r: &Ratio| r.clone() * (one.clone() + p.clone());
        let mut shells: Vec<Vec<(usize, Ratio, Ratio)>> = Vec::new();
        let mut reach_hi = ratio(-1, 1);
        for m in on_face {
            if shells.is_empty() || lo(&m.1) > reach_hi {
                shells.push(Vec::new());
            }
            if hi(&m.1) > reach_hi {
                reach_hi = hi(&m.1);
            }
            shells.last_mut().expect("a shell").push(m);
        }
        for shell in shells.iter().filter(|s| s.len() >= 2) {
            let width = hi(&shell.iter().map(|m| m.1.clone()).max().expect("members"))
                - lo(&shell.iter().map(|m| m.1.clone()).min().expect("members"));
            if width <= ratio(0, 1) {
                continue;
            }
            let members: Vec<usize> = shell.iter().map(|m| m.0).collect();
            let delta_of: std::collections::BTreeMap<usize, Ratio> =
                shell.iter().map(|m| (m.0, m.2.clone())).collect();
            let field_mean = circle_mean(
                &members
                    .iter()
                    .map(|&i| {
                        (
                            share(&motes[i]),
                            motes[i]
                                .phase
                                .get(LOOP)
                                .cloned()
                                .unwrap_or_else(|| ratio(0, 1)),
                        )
                    })
                    .collect::<Vec<_>>(),
            );
            for (j, &k) in SPACE.iter().enumerate() {
                let frac = |x: Ratio| x.clone() - x.floor();
                let on_plane: Vec<(usize, Ratio)> = members
                    .iter()
                    .map(|&i| (i, frac(true_phase(&motes[i], k))))
                    .collect();
                let cells = phase_beads(&on_plane);
                if cells.len() < 2 {
                    continue;
                }
                parted += 1;
                let reads: Vec<CellRead> = cells
                    .iter()
                    .map(|(idx, arc)| {
                        let m = idx.iter().fold(ratio(0, 1), |a, &i| a + share(&motes[i]));
                        let mean = |f: &dyn Fn(usize) -> Ratio| {
                            if m.is_zero() {
                                ratio(0, 1)
                            } else {
                                idx.iter()
                                    .fold(ratio(0, 1), |a, &i| a + share(&motes[i]) * f(i))
                                    / m.clone()
                            }
                        };
                        let ell = mean(&|i| motes[i].ell_o.clone());
                        let tilt = mean(&|i| tilt_of(&motes[i].tilt)[j].clone());
                        let phi = one.clone() - mean(&|i| delta_of[&i].clone());
                        let column =
                            mean(&|i| column_of.get(&i).cloned().unwrap_or_else(|| ratio(0, 1)));
                        let heat = idx.iter().fold(ratio(0, 1), |a, &i| {
                            a + share(&motes[i]) * motes[i].agitation.clone()
                        });
                        let magnet = idx.iter().fold(ratio(0, 1), |a, &i| {
                            a + motes[i].amount.clone()
                                * ratio(i64::from(crate::chem::unpaired(&motes[i].key)), 1)
                                / total.clone()
                                * in_step(
                                    motes[i].phase.get(1).unwrap_or(&ratio(0, 1)),
                                    &field_mean,
                                )
                        });
                        let pressure = if arc.is_zero() {
                            column.clone()
                        } else {
                            crate::ring::grain(&((heat + magnet) / (width.clone() * arc.clone())))
                                + column
                        };
                        CellRead {
                            m,
                            ell: crate::ring::grain(&ell),
                            pressure,
                            tilt,
                            phi: crate::ring::grain(&phi),
                        }
                    })
                    .collect();
                let (d, spin) = around(&reads);
                for ((idx, _), inc) in cells.iter().zip(d) {
                    if !inc.is_zero() {
                        for &i in idx {
                            dl[i] += inc.clone();
                        }
                    }
                }
                moved_around += spin;
            }
        }
    }
    for (x, d) in motes.iter_mut().zip(dl) {
        if !d.is_zero() {
            let now = crate::ring::grain(&(x.ell_o.clone() + d));
            x.ell_o = through_centre(now, &mut x.phase);
        }
    }
    (moved_p, moved_b, moved_around, parted)
}

/// The hydrostatic balance across a ring pair (docs/106 §2, S3), as a signed
/// share: the inner ring's share of the pair's pressure less its share of
/// the pair's potential, `P_in/(P_in + P_out) − Φ_in/(Φ_in + Φ_out)`, with
/// `Φ = g = 1 − δ` the hold inverse gravity gives each. Zero at balance; positive
/// where the inner ring presses harder than its potential bears (the pair
/// expands), negative where it presses less (the pair contracts). First
/// degree, in `(−1, 1)`; zero where either sum is zero.
pub fn restoring(p_in: &Ratio, p_out: &Ratio, phi_in: &Ratio, phi_out: &Ratio) -> Ratio {
    let (sp, sf) = (
        p_in.clone() + p_out.clone(),
        phi_in.clone() + phi_out.clone(),
    );
    if sp.is_zero() || sf.is_zero() {
        return ratio(0, 1);
    }
    p_in.clone() / sp - phi_in.clone() / sf
}

/// A shell's cells on one plane (docs/110 §2): its members, by index and
/// phase on that plane in `[0, 1)`, parted wherever the circular gap between
/// consecutive members is wider than the mean gap `1/n`. Each cell is
/// returned with its arc, the gaps from its first member to the next cell's
/// first member, so the arcs sum to 1. An evenly spread shell, or one of a
/// single member, is one cell of arc 1. No sector count is set: the cells are
/// the shell's own beads.
pub fn phase_beads(members: &[(usize, Ratio)]) -> Vec<(Vec<usize>, Ratio)> {
    let n = members.len();
    if n == 0 {
        return Vec::new();
    }
    let mut by_phase: Vec<(usize, Ratio)> = members.to_vec();
    by_phase.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let one = ratio(1, 1);
    let gaps: Vec<Ratio> = (0..n)
        .map(|i| {
            if i + 1 < n {
                by_phase[i + 1].1.clone() - by_phase[i].1.clone()
            } else {
                by_phase[0].1.clone() + one.clone() - by_phase[i].1.clone()
            }
        })
        .collect();
    let mean = ratio(1, i64::try_from(n).unwrap_or(i64::MAX));
    let starts: Vec<usize> = (0..n)
        .filter(|&i| gaps[i] > mean)
        .map(|i| (i + 1) % n)
        .collect();
    if starts.is_empty() {
        return vec![(by_phase.iter().map(|(i, _)| *i).collect(), one)];
    }
    let mut starts = starts;
    starts.sort_unstable();
    starts.dedup();
    let mut cells = Vec::with_capacity(starts.len());
    for (j, &s) in starts.iter().enumerate() {
        let next = starts[(j + 1) % starts.len()];
        let mut idx = Vec::new();
        let mut arc = ratio(0, 1);
        let mut at = s;
        loop {
            idx.push(by_phase[at].0);
            arc += gaps[at].clone();
            at = (at + 1) % n;
            if at == next {
                break;
            }
        }
        cells.push((idx, arc));
    }
    cells
}

/// A cell's reading for the balance around a shell's circle (docs/110 §3):
/// its holding, mean spin, pressure per unit of arc, and tilt share on the
/// plane.
#[derive(Debug, Clone)]
pub struct CellRead {
    pub m: Ratio,
    pub ell: Ratio,
    pub pressure: Ratio,
    pub tilt: Ratio,
    /// Its potential, `Φ = 1 − δ̄` (holding-weighted over its members).
    pub phi: Ratio,
}

/// The balance around one plane's circle of one shell (docs/110 §3): each
/// pair of neighbouring cells (the last and the first meeting), the
/// over-pressured giving turning to the other by S3's share
/// `s = P_a/(P_a + P_b) − Φ_a/(Φ_a + Φ_b)`, weighted by the pair's tilt share
/// on the plane. Returns each cell's increment to each of its members' spin,
/// and the spin moved by holding; `Σ m·Δℓ` is zero.
pub fn around(cells: &[CellRead]) -> (Vec<Ratio>, Ratio) {
    let n = cells.len();
    let mut d = vec![ratio(0, 1); n];
    let mut moved = ratio(0, 1);
    if n < 2 {
        return (d, moved);
    }
    let pairs = if n == 2 { 1 } else { n };
    for a in 0..pairs {
        let b = (a + 1) % n;
        let (ca, cb) = (&cells[a], &cells[b]);
        let m_ab = ca.m.clone() + cb.m.clone();
        if m_ab.is_zero() {
            continue;
        }
        let s = restoring(&ca.pressure, &cb.pressure, &ca.phi, &cb.phi);
        if s.is_zero() {
            continue;
        }
        let gap = if ca.ell > cb.ell {
            ca.ell.clone() - cb.ell.clone()
        } else {
            cb.ell.clone() - ca.ell.clone()
        };
        let tilt = (ca.m.clone() * ca.tilt.clone() + cb.m.clone() * cb.tilt.clone()) / m_ab.clone();
        let mag = if s < ratio(0, 1) {
            -s.clone()
        } else {
            s.clone()
        };
        let u = crate::ring::grain(&(crate::ring::grain(&mag) * gap * tilt / m_ab));
        if u.is_zero() {
            continue;
        }
        let (giver, taker) = if s > ratio(0, 1) { (a, b) } else { (b, a) };
        d[giver] -= u.clone() * cells[taker].m.clone();
        d[taker] += u.clone() * cells[giver].m.clone();
        moved += u * cells[giver].m.clone() * cells[taker].m.clone();
    }
    (d, moved)
}

/// The motes' total holding (by weight).
pub fn holding(motes: &[Mote]) -> Ratio {
    motes
        .iter()
        .fold(ratio(0, 1), |s, x| s + x.amount.clone() * weight(&x.key))
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
        founding(
            &(1..=k)
                .filter_map(|z| Some((crate::chem::Kind::atom(z)?, ratio(1, 1))))
                .collect(),
            None,
        )
        .expect("the founding's phases are read exactly")
    }

    /// A fibre's reach and bands as the chain law reads them: `R = ℓ/H`, the
    /// band `R(1 ± p)`, the density the member's holding share.
    fn fibre(ms: &[Mote]) -> (Vec<Ratio>, Vec<(Ratio, Ratio, Ratio)>) {
        let (h, p) = (ratio(2, 1), ratio(1, 2));
        let total = ms
            .iter()
            .fold(ratio(0, 1), |s, x| s + x.amount.clone() * weight(&x.key));
        let reach: Vec<Ratio> = ms.iter().map(|x| x.ell_o.clone() / h.clone()).collect();
        let bands = ms
            .iter()
            .zip(&reach)
            .map(|(x, r)| {
                (
                    r.clone() * (ratio(1, 1) - p.clone()),
                    r.clone() * (ratio(1, 1) + p.clone()),
                    x.amount.clone() * weight(&x.key) / total.clone(),
                )
            })
            .collect();
        (reach, bands)
    }

    #[test]
    fn the_hold_along_a_fibre_is_the_sum_of_the_bands_covering_each_reach() {
        let (reach, bands) = fibre(&motes(40));
        let swept = hold_along(&reach, &bands, &mut Work::default());
        for (i, r) in reach.iter().enumerate() {
            let covering = bands
                .iter()
                .filter(|(lo, hi, _)| lo <= r && r <= hi)
                .fold(ratio(0, 1), |s, b| s + b.2.clone());
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
            assert!(
                work.0 <= bound && bound < n * n,
                "n {n}: work {}, bound {bound}",
                work.0
            );
        }
    }

    #[test]
    fn the_motes_within_a_reach_are_exactly_those_strictly_within_it() {
        let ms = motes(60);
        let mut by: Vec<(Ratio, usize)> = ms
            .iter()
            .enumerate()
            .map(|(i, x)| (x.ell_o.clone(), i))
            .collect();
        by.sort();
        let (r, d) = (
            by[30].0.clone(),
            (by[40].0.clone() - by[20].0.clone()) / ratio(2, 1),
        );
        let window: Vec<usize> = within_reach(&by, &r, &d, &mut Work::default())
            .iter()
            .map(|x| x.1)
            .collect();
        let defined: Vec<usize> = by
            .iter()
            .filter(|(x, _)| {
                (if *x > r {
                    x.clone() - r.clone()
                } else {
                    r.clone() - x.clone()
                }) < d
            })
            .map(|x| x.1)
            .collect();
        assert_eq!(window, defined);
    }

    #[test]
    fn the_motes_within_a_reach_cost_two_bisections_not_a_walk() {
        let cost = |k: u32| {
            let ms = motes(k);
            let mut by: Vec<(Ratio, usize)> = ms
                .iter()
                .enumerate()
                .map(|(i, x)| (x.ell_o.clone(), i))
                .collect();
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
    use super::{
        around, blend, mean_response, phase_beads, rest, restoring, space_of, spin_torque,
        tilt_from, tilt_of, CellRead, Mote,
    };
    use cfe::ratio::{ratio, Ratio};
    use num_traits::Zero;

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
        let mut x = Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            ell_o: ratio(1, 1),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(1, 60), ratio(0, 1)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            tilt: Vec::new(),
        };
        let p = ratio(1, 60);
        let mut cross = crate::enfold::Cross::default();
        // Its true place lies 1/240 past its mark: a quarter of a place, so a
        // quarter of its holding stands one place on.
        settle(
            &mut x,
            &[(0, ratio(2, 60), ratio(1, 4))],
            Some(&p),
            &mut cross,
        );
        assert_eq!(true_phase(&x, 0), ratio(1, 60) + ratio(1, 240));
        assert_eq!(
            true_phase(&x, 1),
            ratio(0, 1),
            "a plane with nothing past stands on its mark"
        );
        // Backward: the next mark behind.
        settle(
            &mut x,
            &[(0, ratio(0, 1), ratio(1, 4))],
            Some(&p),
            &mut cross,
        );
        assert_eq!(true_phase(&x, 0), ratio(1, 60) - ratio(1, 240));
    }

    #[test]
    fn the_band_rests_where_the_reach_squared_is_the_collectives_mean_over_the_weight() {
        // docs/104 §6.3: w̄_c/w = 4/9, so R* = 2/3 at any hold.
        let q = ratio(1, 2) * ratio(4, 9);
        for h in [ratio(1, 1), ratio(3, 1), ratio(1, 7)] {
            assert_eq!(super::band(&q, &ratio(2, 3), &h), ratio(0, 1));
            assert!(
                super::band(&q, &ratio(1, 3), &h) > ratio(0, 1),
                "inside, the push wins"
            );
            assert!(
                super::band(&q, &ratio(1, 1), &h) < ratio(0, 1),
                "outside, the pull wins"
            );
        }
        // A kind lighter than the mean rests beyond the throat (R* > 1).
        let light = ratio(1, 2) * ratio(9, 4);
        assert!(super::band(&light, &ratio(1, 1), &ratio(1, 1)) > ratio(0, 1));
        assert_eq!(
            super::face(&ratio(1, 2)),
            (ratio(1, 2), ratio(1, 2)),
            "the faces balance at the ring"
        );
    }

    #[test]
    fn the_band_as_an_attractor_converges_on_its_rest_without_crossing_it() {
        // docs/112 F3: R* = 2/3 about H = 1 (q = 2/9); from R = 1/10 the
        // spin climbs toward ℓ* = 2/3 and never passes it.
        let (q, h) = (ratio(1, 2) * ratio(4, 9), ratio(1, 1));
        let mut ell = ratio(1, 10);
        for _ in 0..60 {
            let r = ell.clone() / h.clone();
            // On the grain each bell, as the torque holds its spins.
            ell = crate::ring::grain(&(ell.clone() + super::toward_rest(&q, &r, &h, &ell)));
            assert!(ell <= ratio(2, 3), "never past the rest");
        }
        assert!(ratio(2, 3) - ell.clone() < ratio(1, 1000), "it converges");
    }

    #[test]
    fn motes_the_bounce_moves_together_bond_and_motes_moved_apart_do_not() {
        // docs/111 §4.2: near on the sphere, open, and moved in step.
        let mote = |key: &str, ph: (i64, i64)| Mote {
            key: crate::chem::Kind::parse(key).expect("a kind"),
            amount: ratio(1, 1),
            ell_o: ratio(1, 1),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(ph.0, ph.1), ratio(0, 1), ratio(0, 1)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            tilt: Vec::new(),
        };
        let (a, b, c) = (
            mote("W0001", (0, 1)),
            mote("W0001", (1, 20)),
            mote("W0001", (1, 10)),
        );
        let with = |x: (i64, i64)| vec![ratio(x.0, x.1), ratio(0, 1), ratio(0, 1)];
        assert!(
            super::bonded(&a, &b, &with((1, 100)), &with((1, 100))),
            "moved rigidly together"
        );
        let all = |x: (i64, i64)| vec![ratio(x.0, x.1); 3];
        assert!(
            !super::bonded(&a, &b, &all((1, 4)), &all((-1, 4))),
            "moved apart in every plane"
        );
        let far = Mote {
            phase: vec![ratio(1, 2); 3],
            ..mote("W0001", (1, 2))
        };
        assert!(
            !super::bonded(&a, &far, &with((0, 1)), &with((0, 1))),
            "opposed on the sphere"
        );
        // Three moved together are one collection; one moved apart is in none.
        let d = mote("W0001", (3, 20));
        let motes = vec![a, b, c, d];
        let moves = vec![all((1, 100)), all((1, 100)), all((1, 100)), all((-49, 100))];
        let sets = super::collections(&motes, &moves);
        assert_eq!(sets, vec![vec![0, 1, 2]]);
    }

    #[test]
    fn an_even_shell_is_one_cell_and_a_clumped_one_parts_at_its_wide_gaps() {
        // docs/110 §2: four members a quarter apart are one cell of arc 1.
        let even: Vec<(usize, Ratio)> = (0..4).map(|i| (i, ratio(i as i64, 4))).collect();
        let cells = phase_beads(&even);
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].1, ratio(1, 1));
        // Two clumps, near 0 and near 1/2: two cells whose arcs sum to 1.
        let clumped = vec![
            (0, ratio(0, 1)),
            (1, ratio(1, 50)),
            (2, ratio(1, 2)),
            (3, ratio(26, 50)),
        ];
        let cells = phase_beads(&clumped);
        assert_eq!(cells.len(), 2, "the shell parts at its two wide gaps");
        assert_eq!(
            cells.iter().fold(ratio(0, 1), |s, c| s + c.1.clone()),
            ratio(1, 1),
            "the arcs cover the circle once"
        );
        assert!(cells.iter().any(|c| c.0 == vec![0, 1]) && cells.iter().any(|c| c.0 == vec![2, 3]));
    }

    #[test]
    fn a_shell_at_balance_around_its_circle_moves_nothing_and_an_over_pressured_cell_gives_turning()
    {
        // docs/110 §3: equal pressures stand still; the over-pressured cell
        // gives turning to its neighbour, and Σ m·ℓ is unchanged.
        let cell = |m: i64, ell: i64, p: i64| CellRead {
            m: ratio(m, 10),
            ell: ratio(ell, 1),
            pressure: ratio(p, 1),
            tilt: ratio(1, 3),
            phi: ratio(1, 2),
        };
        let (d, moved) = around(&[cell(1, 2, 5), cell(2, 3, 5)]);
        assert!(
            d.iter().all(|x| x.is_zero()) && moved.is_zero(),
            "at balance nothing moves"
        );
        let cells = [cell(1, 2, 9), cell(2, 3, 3), cell(3, 4, 3)];
        let (d, moved) = around(&cells);
        assert!(d[0] < ratio(0, 1), "the over-pressured cell gives");
        assert!(moved > ratio(0, 1));
        let total = cells
            .iter()
            .zip(&d)
            .fold(ratio(0, 1), |s, (c, x)| s + c.m.clone() * x.clone());
        assert_eq!(total, ratio(0, 1), "the exchange keeps Σ m·ℓ exact");
    }

    #[test]
    fn a_medium_at_its_balance_moves_nothing_and_displaced_it_returns() {
        // S3's witness (docs/106 §6): pressure shares equal to potential
        // shares stand still; the inner ring over-pressured pushes outward,
        // under-pressured is pushed back inward, by the difference.
        let (phi_in, phi_out) = (ratio(1, 3), ratio(2, 3));
        assert_eq!(
            restoring(&ratio(1, 1), &ratio(2, 1), &phi_in, &phi_out),
            ratio(0, 1),
            "at balance nothing moves"
        );
        let over = restoring(&ratio(2, 1), &ratio(2, 1), &phi_in, &phi_out);
        let under = restoring(&ratio(1, 2), &ratio(2, 1), &phi_in, &phi_out);
        assert!(
            over > ratio(0, 1),
            "an over-pressured inner ring expands the pair"
        );
        assert!(
            under < ratio(0, 1),
            "an under-pressured inner ring is pushed back"
        );
        assert_eq!(over, ratio(1, 2) - ratio(1, 3));
        assert_eq!(under, ratio(1, 5) - ratio(1, 3));
        assert_eq!(
            restoring(&ratio(0, 1), &ratio(0, 1), &phi_in, &phi_out),
            ratio(0, 1),
            "no pressure, no push"
        );
    }

    #[test]
    fn the_shear_moves_turning_outward_and_keeps_the_books() {
        use super::{shear, weight, Enclosed, Mote};
        let mote = |ell: (i64, i64), ph: (i64, i64)| Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 8),
            ell_o: ratio(ell.0, ell.1),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(ph.0, ph.1), ratio(0, 1)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            tilt: Vec::new(),
        };
        let mut motes = vec![
            mote((1, 4), (0, 1)),
            mote((1, 2), (1, 60)),
            mote((3, 4), (1, 30)),
        ];
        let total = ratio(1, 1);
        let book = |ms: &[Mote]| {
            ms.iter().fold(ratio(0, 1), |s, x| {
                s + x.amount.clone() * weight(&x.key) * x.ell_o.clone()
            })
        };
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

    #[test]
    fn a_tilt_parts_the_spin_over_three_planes_and_sums_to_one() {
        // Shares from plane spins sum to one exactly, the last plane taking the rest.
        let t = tilt_from(&[ratio(1, 7), ratio(2, 7), ratio(4, 7)]);
        assert_eq!(
            t.iter().fold(ratio(0, 1), |s, x| s + x.clone()),
            ratio(1, 1)
        );
        // Blending adds plane spins: a body all in plane 0 and one all in
        // plane 2, equal holding and spin, join half and half.
        let b = blend(&[
            (
                ratio(1, 1),
                ratio(1, 1),
                vec![ratio(1, 1), ratio(0, 1), ratio(0, 1)],
            ),
            (
                ratio(1, 1),
                ratio(1, 1),
                vec![ratio(0, 1), ratio(0, 1), ratio(1, 1)],
            ),
        ]);
        assert_eq!(b, vec![ratio(1, 2), ratio(0, 1), ratio(1, 2)]);
        // No tilt reads isotropic.
        assert_eq!(tilt_of(&[]), vec![ratio(1, 3); 3]);
        // A torqued mote keeps its tilt a partition of one and its orbit spin
        // the L1 sum of its planes.
        let mut x = Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            ell_o: ratio(1, 2),
            ell_orth: ratio(0, 1),
            phase: rest(ratio(1, 2)),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            tilt: vec![ratio(1, 2), ratio(1, 3), ratio(1, 6)],
        };
        x.phase[0] = ratio(1, 10);
        x.phase[2] = ratio(7, 10);
        let from = space_of(&x.phase);
        let _ = spin_torque(
            &mut x,
            &from,
            &ratio(1, 1),
            &ratio(1, 2),
            &ratio(2, 3),
            &ratio(1, 4),
            &ratio(0, 1),
            None,
            &ratio(1, 1),
            &ratio(0, 1),
        );
        assert_eq!(
            x.tilt.iter().fold(ratio(0, 1), |s, v| s + v.clone()),
            ratio(1, 1)
        );
        assert!(
            x.tilt[0] != ratio(1, 2) || x.tilt[1] != ratio(1, 3),
            "the planes torqued apart by their own responses"
        );
    }

    #[test]
    fn the_mobius_jetstream_twice_is_the_identity() {
        use super::{mobius, Mote};
        let mut x = Mote {
            key: crate::chem::Kind::parse("W0041").expect("a kind"),
            amount: ratio(1, 1),
            ell_o: ratio(1, 3),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(3, 10), ratio(1, 7)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            tilt: Vec::new(),
        };
        let was = x.clone();
        mobius(&mut x);
        assert!(
            x.vacuum && x.phase[0] == ratio(4, 5),
            "one pass: the other leg, half a turn on"
        );
        mobius(&mut x);
        assert_eq!(x, was, "two passes close");
    }

    #[test]
    fn torsion_is_the_lead_kept_it_relaxes_by_the_attractor_and_turns_with_the_leg() {
        use super::{mobius, settle_disc, torsion, torsion_signed, Enclosed, Mote};
        let flat = vec![ratio(1, 1), ratio(0, 1), ratio(0, 1)];
        // Settled in its enclosure's plane: none; wound wholly into another: 1.
        assert_eq!(torsion(&flat, &flat), ratio(0, 1));
        assert_eq!(
            torsion(&[ratio(0, 1), ratio(1, 1), ratio(0, 1)], &flat),
            ratio(1, 1)
        );
        assert_eq!(
            torsion(&[ratio(1, 2), ratio(1, 2), ratio(0, 1)], &flat),
            ratio(1, 2)
        );
        // The Möbius pass reverses its sense.
        let mut x = Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            ell_o: ratio(3, 1),
            ell_orth: ratio(0, 1),
            phase: super::rest(ratio(1, 10)),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            tilt: vec![ratio(1, 2), ratio(1, 2), ratio(0, 1)],
        };
        let before = torsion_signed(&x, &flat);
        mobius(&mut x);
        assert_eq!(torsion_signed(&x, &flat), -before);
        mobius(&mut x);
        // Relaxed by the attractor law: a wound mote among settled ones keeps
        // the share `turned/(d + turned)` of its lead, exactly.
        let heavy = Mote {
            amount: ratio(1000, 1),
            tilt: flat.clone(),
            ..x.clone()
        };
        let mut motes = vec![heavy, x.clone()];
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        let plane = settle_disc(&mut motes, &enc);
        assert!(
            torsion(&motes[1].tilt, &plane) < torsion(&x.tilt, &plane),
            "the lead loosens"
        );
    }

    #[test]
    fn a_member_falls_by_its_step_with_its_binding_never_by_a_length() {
        use super::{step_with, Mote};
        use cfe::ratio::Ratio;
        let mote = |phase: Vec<Ratio>, tilt: Vec<Ratio>| Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            ell_o: ratio(1, 1),
            ell_orth: ratio(0, 1),
            phase,
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            tilt,
        };
        let centre = vec![ratio(0, 1); super::PLANES];
        let flat = vec![ratio(1, 1), ratio(0, 1), ratio(0, 1)];
        // Displaced only in a plane its orbit does not turn in: in step.
        let mut ph = centre.clone();
        ph[super::SPACE[1]] = ratio(1, 2);
        assert_eq!(step_with(&mote(ph, flat.clone()), &centre), ratio(1, 1));
        // A quarter turn off in its orbit's own plane: on the boundary, ½.
        let mut ph = centre.clone();
        ph[super::SPACE[0]] = ratio(1, 4);
        assert_eq!(step_with(&mote(ph, flat.clone()), &centre), ratio(1, 2));
        // Half a turn off: opposed, the disc.
        let mut ph = centre.clone();
        ph[super::SPACE[0]] = ratio(1, 2);
        assert_eq!(step_with(&mote(ph, flat), &centre), ratio(0, 1));
        // Isotropic, off by a tenth in every plane: 4/5, it falls.
        let ph = vec![ratio(1, 10); super::PLANES];
        assert_eq!(step_with(&mote(ph, Vec::new()), &centre), ratio(4, 5));
    }

    #[test]
    fn a_motes_nuclear_grip_is_its_nuclei_at_its_place() {
        use super::{grip_share, Mote};
        let mote = |key: &str, loop_at: (i64, i64)| Mote {
            key: crate::chem::Kind::parse(key).expect("a kind"),
            amount: ratio(1, 1),
            ell_o: ratio(1, 1),
            ell_orth: ratio(0, 1),
            phase: super::rest(ratio(loop_at.0, loop_at.1)),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            tilt: Vec::new(),
        };
        // The seal at 28 grips 9/14 in step with its echo, its own reading.
        let g = ratio(3, 5);
        assert_eq!(
            grip_share(&mote("W0028", (0, 1)), &g),
            crate::nuclides::grip(28, &g).expect("a grip")
        );
        // Hydrogen, one lone proton, grips nothing.
        assert_eq!(grip_share(&mote("W0001", (0, 1)), &g), ratio(0, 1));
    }

    #[test]
    fn a_binding_gathers_on_the_disc_leg_alone() {
        use super::{band_pairs, on_disc_leg, Enclosed, Mote, LOOP};
        // A heavy cold ring at reach 3/2 about a base of 1, half its motes
        // on the disc leg (loop phase 1/10), half in transit (7/10).
        let mote = |j: i64, loop_at: (i64, i64)| {
            let mut phase = super::rest(ratio(loop_at.0, loop_at.1));
            phase[0] = ratio(j, 40);
            Mote {
                key: crate::chem::Kind::parse("W0016").expect("a kind"),
                amount: ratio(1, 1),
                ell_o: ratio(3, 2),
                ell_orth: ratio(0, 1),
                phase,
                agitation: ratio(0, 1),
                born: 0,
                vacuum: false,
                lever: Vec::new(),
                lever_place: ratio(0, 1),
                tilt: vec![ratio(1, 1), ratio(0, 1), ratio(0, 1)],
            }
        };
        let motes: Vec<Mote> = (0..40)
            .map(|j| mote(j, if j % 2 == 0 { (1, 10) } else { (7, 10) }))
            .collect();
        let enc = Enclosed::new(ratio(1, 1), vec![(ratio(3, 2), ratio(1, 1))]);
        let g = super::holding(&motes) * ratio(2, 1);
        let bindings = band_pairs(
            &motes,
            &enc,
            &ratio(1, 10),
            &[ratio(1, 1), ratio(0, 1), ratio(0, 1)],
            &g,
        );
        assert!(!bindings.is_empty(), "the ring binds");
        for (_, members) in &bindings {
            for (i, _) in members {
                assert!(
                    on_disc_leg(&motes[*i]),
                    "member {i} at loop phase {} is in transit",
                    motes[*i].phase[LOOP]
                );
            }
        }
    }

    #[test]
    fn the_stream_is_a_reading_the_face_is_the_luminal_bounds_alone() {
        use super::{luminal, stream, Enclosed, Mote, Strand};
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        let mote = |ell: (i64, i64), vacuum: bool| Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            ell_o: ratio(ell.0, ell.1),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(1, 10), ratio(1, 7)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            tilt: Vec::new(),
        };
        let mut motes = vec![mote((1, 3), false), mote((3, 1), false)];
        luminal(&mut motes, &enc);
        let faces: Vec<bool> = motes.iter().map(|x| x.vacuum).collect();
        // A strand holding both, its bond beating any push: the stream counts
        // them as standing against the current, and changes no face.
        let strand = Strand {
            members: vec![0, 1],
            reach: ratio(1, 1),
            bind: ratio(1, 1),
            agitation: ratio(0, 1),
            face: ratio(0, 1),
            apart: ratio(1, 2),
        };
        assert_eq!(stream(&motes, &[strand], &|_| ratio(1, 2)), 2);
        assert_eq!(motes.iter().map(|x| x.vacuum).collect::<Vec<_>>(), faces);
    }

    #[test]
    fn the_luminal_bound_reads_either_face_and_leaves_every_share_at_most_half() {
        use super::{luminal, Enclosed, Mote};
        // About a base of 1: reach 3 reads δ = ¼, reach ⅓ reads δ = ¾.
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        let mote = |ell: (i64, i64), vacuum: bool| Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            ell_o: ratio(ell.0, ell.1),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(1, 10), ratio(1, 7)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            tilt: Vec::new(),
        };
        // Outside the ring a vacuum mote reads ¾ and takes the pass; a
        // continuum mote reads ¼ and stays. Inside, the other way.
        let mut motes = vec![
            mote((3, 1), true),
            mote((3, 1), false),
            mote((1, 3), false),
            mote((1, 3), true),
        ];
        assert_eq!(luminal(&mut motes, &enc), 2);
        assert_eq!(
            motes.iter().map(|x| x.vacuum).collect::<Vec<_>>(),
            vec![false, false, true, true]
        );
        for x in &motes {
            let d = enc.reach(&x.ell_o).1;
            let share = if x.vacuum { ratio(1, 1) - d } else { d };
            assert!(
                share <= ratio(1, 2),
                "every mote stands on the face it can turn on"
            );
        }
        // Read again, nothing moves.
        assert_eq!(luminal(&mut motes, &enc), 0);
        // At the ring both faces read ½ and neither passes.
        let mut ring = vec![mote((1, 1), true), mote((1, 1), false)];
        assert_eq!(luminal(&mut ring, &enc), 0);
    }

    #[test]
    fn the_torque_telescopes_its_invariant_holds_whatever_the_advance() {
        // The torque's invariant (with Xylarium 2026-10-24): a bell's push
        // over the arc it sweeps, `p·δ·k` with `k` the passage mean over `2δ`,
        // is `(p/2)·[F(ψ + 2δ) − F(ψ)]`, `F = f(1 − f)` the sawtooth's
        // periodic antiderivative; so `I = ℓ − (p/2)·F(ψ)` stands, bell on
        // bell, for any advances (arcs across the bell and past whole turns).
        use cfe::ratio::Ratio;
        let f_of = |x: &Ratio| {
            let f = x.clone() - x.floor();
            f.clone() * (ratio(1, 1) - f)
        };
        let p = ratio(1, 2);
        let (mut ell, mut psi) = (ratio(7, 10), ratio(3, 10));
        let inv = |ell: &Ratio, psi: &Ratio| ell.clone() - p.clone() / ratio(2, 1) * f_of(psi);
        let start = inv(&ell, &psi);
        for d in [
            ratio(1, 3),
            ratio(9, 20),
            ratio(1, 2),
            ratio(1, 7),
            ratio(5, 6),
            ratio(2, 5),
            ratio(3, 11),
            ratio(1, 1),
        ] {
            let len = ratio(2, 1) * d.clone();
            ell += p.clone() * d * mean_response(&psi, &len);
            psi += len;
            assert_eq!(inv(&ell, &psi), start);
        }
    }
}
