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
//!   own extent (`R(1 ± e)`, `e` its pinch share) within the throat, in both planes,
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
use num_traits::{Signed, Zero};

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

/// A mote's own share at a deficit δ, read on its stream's axis (one face
/// law, docs/117 §8.2, register B16): the continuum's turning share `δ`, the
/// vacuum's (the continuum inverted) `1 − δ`.
pub fn stream_delta(delta: &Ratio, vacuum: bool) -> Ratio {
    crate::polarity::faces_on(delta, crate::polarity::Axis::of_stream(vacuum)).0
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
/// held on the grain (the shares sum to one exactly, never re-divided
/// without bound). Isotropic where nothing turns.
///
/// No plane takes the rest (docs/78: no fill orders): each share is floored
/// to the grain, and the grains the floors lost (fewer than the planes)
/// return to the planes that lost the most, greatest first; planes that lost
/// alike share alike, splitting a grain between them where too few remain.
/// So the tilt depends on the spins alone, never on which plane is which:
/// permuting the spins permutes the tilt.
pub fn tilt_from(spins: &[Ratio]) -> Vec<Ratio> {
    let sum = spins.iter().fold(ratio(0, 1), |s, l| s + l.clone());
    if sum.is_zero() {
        return Vec::new();
    }
    let exact: Vec<Ratio> = spins.iter().map(|l| l.clone() / sum.clone()).collect();
    let mut t: Vec<Ratio> = exact.iter().map(crate::ring::grain).collect();
    let lost: Vec<Ratio> = exact
        .iter()
        .zip(&t)
        .map(|(e, g)| e.clone() - g.clone())
        .collect();
    let step = crate::ring::grain_step();
    let mut left = ratio(1, 1) - t.iter().fold(ratio(0, 1), |s, x| s + x.clone());
    let mut levels: Vec<&Ratio> = lost.iter().filter(|l| !l.is_zero()).collect();
    levels.sort_by(|a, b| b.cmp(a));
    levels.dedup();
    for level in levels {
        if left.is_zero() {
            break;
        }
        let alike: Vec<usize> = (0..t.len()).filter(|&j| lost[j] == *level).collect();
        let n = Ratio::from_integer(num_bigint::BigInt::from(alike.len()));
        let each = if left >= step.clone() * n.clone() {
            step.clone()
        } else {
            left.clone() / n.clone()
        };
        for j in alike {
            t[j] += each.clone();
        }
        left -= each * n;
    }
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

/// The shortest arc on the circle holding every phase in `orbit` (each in
/// `[0, 1)`): one less the widest gap between neighbouring phases, the gap
/// across 0 included. Zero for one phase or none.
pub fn circular_extent(orbit: &mut [Ratio]) -> Ratio {
    if orbit.len() < 2 {
        return ratio(0, 1);
    }
    orbit.sort();
    let first = orbit[0].clone();
    let last = orbit[orbit.len() - 1].clone();
    let widest = orbit
        .windows(2)
        .map(|w| w[1].clone() - w[0].clone())
        .fold(first + ratio(1, 1) - last, |m, g| m.max(g));
    ratio(1, 1) - widest
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
    /// Its orbit spin per spatial plane (in `SPACE` order), own units:
    /// `ℓ_k = ℓ_o·t_k`. The orbit spin `ℓ_o` is their sum (reach `ℓ_o/H`),
    /// its tilt their shares of it (`ell_o`, `tilt`).
    pub spins: Vec<Ratio>,
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
    /// Its lift (docs/121 §14.6, the S lane): its reach's displacement from
    /// its spin balance as a dilation share `u ∈ (−1, 1)`, its reach
    /// `R = R_ℓ·D(u)` (`D` LAW-DILATION, `R_ℓ = ℓ/H(<ℓ)`); 0 on its orbit.
    /// The magnet's boost moves it; spin is never touched by it.
    pub lift: Ratio,
}

impl Mote {
    /// The plane spins from an orbit spin and a tilt: `ℓ_k = ℓ·t_k`, an
    /// empty tilt isotropic (a third in each plane).
    pub fn spins_of(ell: &Ratio, tilt: &[Ratio]) -> Vec<Ratio> {
        tilt_of(tilt).into_iter().map(|t| ell.clone() * t).collect()
    }

    /// Its orbit spin: the sum of its plane spins (L1, so its reach is
    /// `ℓ_o/H`).
    pub fn ell_o(&self) -> Ratio {
        self.spins.iter().fold(ratio(0, 1), |s, l| s + l.clone())
    }

    /// Its tilt (docs/96 step 1): each plane's share of its orbit spin, the
    /// shares summing to one; empty (isotropic) where it does not turn.
    pub fn tilt(&self) -> Vec<Ratio> {
        let ell = self.ell_o();
        if ell.is_zero() {
            return Vec::new();
        }
        self.spins.iter().map(|l| l.clone() / ell.clone()).collect()
    }

    /// Its reach in an enclosure and the held share there (docs/121 §14.6):
    /// on its orbit (lift 0) the spin balance `ℓ/H(<ℓ)` and its deficit, as
    /// `Enclosed::reach` reads them; lifted, `R_ℓ·D(u)` and the hold standing
    /// within that reach, `δ = H(≤R)/(R + H(≤R))`.
    pub fn reach_in(&self, enc: &Enclosed) -> (Ratio, Ratio) {
        let (r, _, delta) = lifted(enc, &self.ell_o().abs(), &self.lift);
        (r, delta)
    }

    /// The hold it turns about at its reach (docs/121 §14.6): `H(<ℓ)` on its
    /// orbit, the hold standing within its reach when lifted.
    pub fn hold_in(&self, enc: &Enclosed) -> Ratio {
        lifted(enc, &self.ell_o().abs(), &self.lift).1
    }

    /// Sets its orbit spin, keeping its tilt.
    pub fn set_ell_o(&mut self, v: Ratio) {
        self.spins = Mote::spins_of(&v, &self.tilt());
    }

    /// Sets its tilt, keeping its orbit spin.
    pub fn set_tilt(&mut self, t: Vec<Ratio>) {
        self.spins = Mote::spins_of(&self.ell_o(), &t);
    }
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

/// A kind's open share at a heat (the hot reading of `open`, register §D:
/// one reading per quantity): an atom's open frames are what its windings
/// keep at that heat (`atom::open_at`; hydrogen is stripped at heat 1, heavy
/// kinds keep their deep windings), over its weight. A cluster has no
/// windings of its own and reads `open`.
pub fn open_hot(k: &crate::chem::Kind, heat: &Ratio) -> Ratio {
    match crate::chem::atom_w(k) {
        Some(w) => ratio(
            i64::from(cfe::almanac::atom::open_at(crate::chem::protons(w), heat)),
            i64::from(crate::chem::weight(k).max(1)),
        ),
        None => open(k),
    }
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

/// Its torsion's sense (docs/100 §8): the tension on its tightest-wound
/// pair of planes against its enclosure's plane (`torsion::tension`, the
/// bivector read on the pair it is wound tightest on), turning with the
/// route, forward on the continuum leg and backward on the vacuum leg, so a
/// Möbius pass reverses it. Kept as the tilt, relaxed by the attractor law
/// (`settle_disc`), never a scalar of its own.
pub fn torsion_signed(x: &Mote, plane: &[Ratio]) -> Ratio {
    let t = crate::torsion::tension(&x.tilt(), plane);
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
    let t = tilt_of(&x.tilt());
    SPACE.iter().zip(&t).fold(ratio(0, 1), |c, (&k, tk)| {
        c + tk.clone()
            * in_step(
                x.phase.get(k).unwrap_or(&ratio(0, 1)),
                psi.get(k).unwrap_or(&ratio(0, 1)),
            )
    })
}

/// Two motes of a collective are BONDED (docs/111 §4.2) where they stand on
/// one held hemisphere (`step_with ≥ ½`, each against the other), both carry
/// open frames, and the bounce moved them together: their perturbations
/// `Δφ` (the displacements the bounce gave them) in step, tilt-weighted,
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
        let t = tilt_of(&x.tilt());
        SPACE.iter().zip(&t).fold(ratio(0, 1), |c, (&k, tk)| {
            c + tk.clone() * in_step(da.get(k).unwrap_or(&zero), db.get(k).unwrap_or(&zero))
        })
    };
    moved(a) >= half && moved(b) >= half
}

/// A collective's COLLECTIONS (docs/111 §4.2): the connected sets of its
/// bonds, read from one bounce's perturbations (each mote's displacement), never
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
    // The bond test, exact, on whole numbers where it can be (the pairs are
    // many): every phase, move and tilt share of a firmament mote stands on
    // the spin grain, tilt shares split at most three ways, so each is a
    // whole count of `1/D`, `D = 6·60⁶`. On that grid `in_step` is
    // `D − 2·off` and each condition of `bonded` (a tilt-weighted sum of
    // in-step shares at least ½) is `2·Σ t_k·i_k ≥ D²`, exact in i128
    // (`D² < 2⁷⁷`). A mote with any value off the grid is read by the
    // rational test instead. Either way the bonds are exactly `bonded`'s.
    let grid = {
        let d = num_bigint::BigInt::from(6) * num_bigint::BigInt::from(60).pow(6);
        Ratio::from_integer(d)
    };
    let d_whole: i128 = 6 * 60i128.pow(6);
    let whole = |x: &Ratio| -> Option<i128> {
        use num_traits::ToPrimitive;
        let c = x.clone() * grid.clone();
        c.is_integer().then(|| c.to_integer().to_i128()).flatten()
    };
    struct Whole {
        open: bool,
        phase: [i128; 3],
        tilt: [i128; 3],
        moved: [i128; 3],
    }
    let wholes: Vec<Option<Whole>> = (0..n)
        .map(|i| {
            let x = &motes[i];
            let zero = ratio(0, 1);
            let at = |v: &[Ratio], k: usize| whole(v.get(k).unwrap_or(&zero));
            let t = tilt_of(&x.tilt());
            let mut w = Whole {
                open: crate::chem::unpaired(&x.key) != 0,
                phase: [0; 3],
                tilt: [0; 3],
                moved: [0; 3],
            };
            for (j, &k) in SPACE.iter().enumerate() {
                w.phase[j] = at(&x.phase, k)?;
                w.tilt[j] = whole(&t[j])?;
                w.moved[j] = at(&moves[i], k)?;
            }
            Some(w)
        })
        .collect();
    let in_step_w = |a: i128, b: i128| {
        let d = (a - b).rem_euclid(d_whole);
        d_whole - 2 * d.min(d_whole - d)
    };
    let holds = |t: &[i128; 3], a: &[i128; 3], b: &[i128; 3]| {
        2 * (0..3).map(|k| t[k] * in_step_w(a[k], b[k])).sum::<i128>() >= d_whole * d_whole
    };
    let bonded_whole = |a: &Whole, b: &Whole| {
        a.open
            && b.open
            && holds(&a.tilt, &a.phase, &b.phase)
            && holds(&b.tilt, &b.phase, &a.phase)
            && holds(&a.tilt, &a.moved, &b.moved)
            && holds(&b.tilt, &a.moved, &b.moved)
    };
    for i in 0..n {
        if moves[i].is_empty() {
            continue;
        }
        for j in i + 1..n {
            if moves[j].is_empty() {
                continue;
            }
            let bond = match (&wholes[i], &wholes[j]) {
                (Some(a), Some(b)) => bonded_whole(a, b),
                _ => bonded(&motes[i], &motes[j], &moves[i], &moves[j]),
            };
            if bond {
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

/// A reach read off its spin balance by a lift (docs/121 §14.6): at spin `ℓ`
/// with lift `u`, (its reach, the hold it turns about there, its deficit).
/// On the orbit (`u = 0`) the spin balance `ℓ/H(<ℓ)` and `H(<ℓ)`, as
/// `Enclosed::reach` reads them; lifted, `R = R_ℓ·D(u)` and the hold standing
/// within `R`. A lift outside `(−1, 1)` reads the orbit.
pub fn lifted(enc: &Enclosed, ell: &Ratio, lift: &Ratio) -> (Ratio, Ratio, Ratio) {
    let on_orbit = || {
        let h = enc.hold(ell);
        let (r, d) = enc.reach(ell);
        (r, h, d)
    };
    if lift.is_zero() {
        return on_orbit();
    }
    let Some(dil) = crate::ring::dilation(lift) else {
        return on_orbit();
    };
    let r = enc.reach(ell).0 * dil;
    let h = enc.hold_within(&r);
    let d = if (r.clone() + h.clone()).is_zero() {
        ratio(0, 1)
    } else {
        h.clone() / (r.clone() + h.clone())
    };
    (r, h, d)
}

/// The magnet's share on a body at reach `R` (docs/121 §14.3): the dipole
/// contrast of the ring law across its own extent `d`, `n_d = M/(R − d + M) −
/// M/(R + d + M)` (`M` the open frames enclosed), signed by the poles `s =
/// −σ_i·σ_c` and gripped by the fluidities `κ = o_i·o_c`: `m = s·κ·n_d`,
/// inward positive, in `(−1, 1)`. None where the contrast is undefined.
pub fn magnet_share(r: &Ratio, d: &Ratio, m: &Ratio, s: &Ratio, kappa: &Ratio) -> Option<Ratio> {
    let near = r.clone() - d.clone() + m.clone();
    let far = r.clone() + d.clone() + m.clone();
    if near <= ratio(0, 1) || far <= ratio(0, 1) {
        return None;
    }
    let n_d = m.clone() / near - m.clone() / far;
    Some(s.clone() * kappa.clone() * n_d)
}

/// The boost on reach (docs/121 §14.4): the inward share `y = u ⊕ m` (the
/// offset from the spin balance, which is the lift `u` itself, composed with
/// the magnet's share by the flow law), and the reach dilated by it,
/// `R′ = R/D(y)`, which is the lift `u′ = u ⊖ y`. The two compose to `−m`:
/// each bell the lift stands at the magnet's share where the body stands;
/// with no magnet (`m = 0`) it is back on its orbit. Spin is not touched.
/// None outside the flow law's domain.
pub fn boosted(u: &Ratio, m: &Ratio) -> Option<Ratio> {
    let y = crate::ring::compose(u, m)?;
    crate::ring::compose(u, &-y)
}

/// The magnet on reach for every mote of an enclosure (docs/121 §14), read
/// at once from where they stand, then moved. Each mote at reach `R` (its
/// spin balance lifted, `reach_in`), its own extent `d = R·e`, its pole and
/// fluidity on its frames `(3Z − U, U)` (on the vacuum leg exchanged, the
/// passage's pole swap: `σ ↦ −σ`, `o ↦ 1 − o`); what stands within `R`:
/// the base (the core's state, its frames the throat's composition, a
/// closed pole where its throat holds nothing) and every mote whose reach is
/// below `R`, each by its holding share, its frames in proportion. Each lift
/// moves by `boosted`, held on the grain. Returns the motes moved and the sum
/// of their lifts' change.
pub fn magnet(
    motes: &mut [Mote],
    enc: &Enclosed,
    total: &Ratio,
    base_frames: &(Ratio, Ratio),
) -> (usize, Ratio) {
    let n = motes.len();
    if n == 0 || total.is_zero() {
        return (0, ratio(0, 1));
    }
    let frames_of = |x: &Mote| {
        let (a, q) = crate::chem::frames(&x.key);
        let (a, q) = (ratio(i64::from(a), 1), ratio(i64::from(q), 1));
        if x.vacuum {
            (q, a)
        } else {
            (a, q)
        }
    };
    // Each mote's reach, extent, holding and frames, at the bell's start.
    let reach: Vec<Ratio> = motes.iter().map(|x| x.reach_in(enc).0).collect();
    let mut by: Vec<usize> = (0..n).collect();
    by.sort_by(|&a, &b| reach[a].cmp(&reach[b]).then(a.cmp(&b)));
    let held: Vec<(Ratio, Ratio)> = motes
        .iter()
        .map(|x| {
            let h = x.amount.clone() * weight(&x.key) / total.clone();
            let (a, q) = frames_of(x);
            let whole = a.clone() + q.clone();
            if whole.is_zero() {
                (h, ratio(0, 1))
            } else {
                (h.clone() * a / whole.clone(), h * q / whole)
            }
        })
        .collect();
    // Prefix sums by reach: the frames standing below each mote.
    let mut inside: Vec<(Ratio, Ratio)> = vec![(ratio(0, 1), ratio(0, 1)); n];
    let (mut a_run, mut q_run) = base_frames.clone();
    let mut j = 0;
    while j < n {
        // Motes at one reach stand beside each other, not inside.
        let mut k = j;
        while k < n && reach[by[k]] == reach[by[j]] {
            inside[by[k]] = (a_run.clone(), q_run.clone());
            k += 1;
        }
        for &i in &by[j..k] {
            a_run += held[i].0.clone();
            q_run += held[i].1.clone();
        }
        j = k;
    }
    let mut lifts = Vec::with_capacity(n);
    for (i, x) in motes.iter().enumerate() {
        let (a, q) = frames_of(x);
        let whole = a.clone() + q.clone();
        let (ac, qc) = &inside[i];
        let enclosed = ac.clone() + qc.clone();
        let next = if whole.is_zero() || enclosed.is_zero() {
            None
        } else {
            let (s_i, o_i) = ((a.clone() - q.clone()) / whole.clone(), q.clone() / whole);
            let (s_c, o_c) = (
                (ac.clone() - qc.clone()) / enclosed.clone(),
                qc.clone() / enclosed,
            );
            let d = reach[i].clone() * own_enclosure(x, enc, total)[0].clone();
            magnet_share(&reach[i], &d, qc, &-(s_i * s_c), &(o_i * o_c))
                .and_then(|m| boosted(&x.lift, &m))
        };
        lifts.push(next.map(|u| crate::ring::grain(&u)));
    }
    let (mut moved, mut change) = (0usize, ratio(0, 1));
    for (x, u) in motes.iter_mut().zip(lifts) {
        let Some(u) = u else { continue };
        if u != x.lift && u > ratio(-1, 1) && u < ratio(1, 1) {
            let d = u.clone() - x.lift.clone();
            change += if d < ratio(0, 1) { -d } else { d };
            x.lift = u;
            moved += 1;
        }
    }
    (moved, change)
}

/// A body's own enclosure (docs/120 §8c, operator 2026-11-05), read and
/// never stored: on each plane of its orbit (the orbit, the circulation),
/// where its own pull meets its enclosure's, the pinch share `m/(m + H(<R))`
/// of its reach (`ring::pinch_share`), a share of its reach. Its reading in
/// degrees is open (register K3). `m` is its holding in the enclosure's
/// unit `whole`.
pub fn own_enclosure(mote: &Mote, enc: &Enclosed, whole: &Ratio) -> [Ratio; 2] {
    let m = if whole.is_zero() {
        ratio(0, 1)
    } else {
        mote.amount.clone() * weight(&mote.key) / whole.clone()
    };
    [
        crate::ring::pinch_share(&m, &mote.hold_in(enc)),
        crate::ring::pinch_share(&m, &enc.hold(&mote.ell_orth.abs())),
    ]
}

/// A body's own enclosure in degrees on each plane of its orbit (register
/// K3): its extent `d = R·e` (`e` the pinch share, `own_enclosure`) as an arc
/// of its closed plane. Size is L1 (docs/118 F1), and the universe's own
/// circle of radius `R` is the L1 diamond, `8R` a whole turn (docs/61), so
/// the arc `d` is `d/(8R)` of a turn: `45°·e` on each side of its place.
/// It holds for an untwisted plane only; a plane bends as it spins (torsion,
/// docs/117 §11.4), and a twisted plane's arcs are read from its composed
/// motion (register K11).
pub fn own_degrees(mote: &Mote, enc: &Enclosed, whole: &Ratio) -> [Ratio; 2] {
    own_enclosure(mote, enc, whole).map(|e| ratio(45, 1) * e)
}

/// Whether two bodies' own enclosures meet on a plane: their separation
/// there, in degrees on the closed circle (`[0°, 180°]`), at most the sum
/// of their two half-arcs.
pub fn enclosures_meet(separation: &Ratio, a: &Ratio, b: &Ratio) -> bool {
    *separation <= a.clone() + b.clone()
}

/// The cut as A reading, by the one enclosure (one grammar: the hold the
/// exits and the ring law read, `H(<ℓ)` with the throat its base, never a
/// second hold of its own): a mote's held share in each plane, its reach
/// `R = |ℓ|/H(<|ℓ|)` with its own enclosure about it (`own_enclosure`), the
/// part of it within the hold there, `f_o·f_c`. A spin's sense is a
/// direction, never a reach.
pub fn held(mote: &Mote, enc: &Enclosed, whole: &Ratio) -> Ratio {
    let own = own_enclosure(mote, enc, whole);
    let (r_o, h_o, _) = lifted(enc, &mote.ell_o().abs(), &mote.lift);
    let l = mote.ell_orth.abs();
    held_share(&r_o, &h_o, &own[0]) * held_share(&enc.reach(&l).0, &enc.hold(&l), &own[1])
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
    /// Each body's own reach (`ℓ/H(<ℓ)`), in order, with the holding standing
    /// at or within it summed from the base: `hold_within` by bisection.
    places: Vec<Ratio>,
    within: Vec<Ratio>,
}

impl Enclosed {
    /// From the base and each body's (spin ℓ, holding), in one unit.
    pub fn new(base: Ratio, bodies: Vec<(Ratio, Ratio)>) -> Enclosed {
        Enclosed::with_lifts(
            base,
            bodies.into_iter().map(|(l, h)| (l, h, ratio(0, 1))).collect(),
        )
    }

    /// From the base and each body's (spin ℓ, holding, lift), in one unit
    /// (docs/121 §14.6): the hold by spin as `new`, each body placed at its
    /// spin balance lifted, `ℓ/H(<ℓ)·D(u)`, so the hold within a reach counts
    /// a lifted body where it stands. A lift outside `(−1, 1)` places it on
    /// its orbit.
    pub fn with_lifts(base: Ratio, mut bodies: Vec<(Ratio, Ratio, Ratio)>) -> Enclosed {
        bodies.sort_by(|a, b| a.0.cmp(&b.0));
        let mut held = Vec::with_capacity(bodies.len() + 1);
        let mut run = crate::ring::grain(&base);
        held.push(run.clone());
        // On the coordinate's grain (the carrier, never law): an exact sum of
        // thousands of holdings otherwise grows its denominators without
        // bound, and every reach, contact and torque inherits them.
        for (_, h, _) in &bodies {
            run += crate::ring::grain(h);
            held.push(run.clone());
        }
        let mut by_place: Vec<(Ratio, Ratio)> = bodies
            .iter()
            .enumerate()
            .filter(|(i, _)| !held[*i].is_zero())
            .map(|(i, (l, _, u))| {
                let on = l.clone() / held[i].clone();
                (
                    if u.is_zero() {
                        on
                    } else {
                        crate::ring::dilation(u).map_or(on.clone(), |d| on * d)
                    },
                    held[i + 1].clone() - held[i].clone(),
                )
            })
            .collect();
        by_place.sort_by(|a, b| a.0.cmp(&b.0));
        let mut within = Vec::with_capacity(by_place.len() + 1);
        let mut run = held[0].clone();
        within.push(run.clone());
        for (_, h) in &by_place {
            run += h.clone();
            within.push(run.clone());
        }
        Enclosed {
            spins: bodies.into_iter().map(|(l, _, _)| l).collect(),
            held,
            places: by_place.into_iter().map(|(r, _)| r).collect(),
            within,
        }
    }

    /// `H(<ℓ)`: the base and everything turning with less spin.
    pub fn hold(&self, ell: &Ratio) -> Ratio {
        self.held[self.spins.partition_point(|l| l < ell)].clone()
    }

    /// The rim (the rim law, `enclosure::rim_of`; docs/112 §7.1): where the
    /// vacuum's hold equals the continuum's push, deficit ½, so the reach
    /// equals the holding enclosed there, `R_rim = H(<R_rim)`. Read from the
    /// whole holding inward: where every body stands inside it, the rim is
    /// the whole holding; a body past it is outside, and the rim is read
    /// again on what stays within.
    pub fn rim(&self) -> Ratio {
        let mut r = self.whole();
        loop {
            let h = self.hold_within(&r);
            if h >= r || h.is_zero() {
                return r;
            }
            r = h;
        }
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
        self.within[self.places.partition_point(|at| at < r)].clone()
    }

    /// The spin a body standing at reach `r` carries (register B11, one
    /// reach): the exact inverse of `reach`, the least ℓ with `ℓ = r·H(<ℓ)`.
    /// `H(<ℓ)` is constant between neighbouring spins and only grows, so the
    /// steps' readings overlap and never leave a gap: each step is read once,
    /// in order, and the first whose interval holds `r·H` is the answer.
    pub fn spin_at(&self, r: &Ratio) -> Ratio {
        for (k, h) in self.held.iter().enumerate() {
            let ell = r.clone() * h.clone();
            if self.spins.get(k).is_none_or(|top| ell <= *top) {
                return ell;
            }
        }
        r.clone() * self.whole()
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
            let (g, ab) = torque_one(x, enc, &lo, tide, total, cross);
            given += g;
            absorbed += ab;
            continue;
        }
        let (mut xa, mut xb) = (x.clone(), x.clone());
        let (mut ca, mut cb) = (
            crate::enfold::Cross::default(),
            crate::enfold::Cross::default(),
        );
        let (ga, aa) = torque_one(&mut xa, enc, &lo, tide, total, &mut ca);
        let (gb, ab) = torque_one(&mut xb, enc, &hi, tide, total, &mut cb);
        if xa == xb && ca == cb && ga == gb && aa == ab {
            *x = xa;
            cross.join(&ca);
            given += ga;
            absorbed += aa;
        } else {
            let (g, ab) = torque_one(x, enc, p, tide, total, cross);
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
    cross: &mut crate::enfold::Cross,
) -> (Ratio, Ratio) {
    // Each turns by what is enclosed inside it (the core and all inside),
    // at the reach its orthogonal orbit swings it to this bell (docs/90 D2): the
    // motion law at its present reach, so it turns faster inward and
    // slower outward; through the centre where the swing passes zero.
    full(&mut x.phase);
    let (now, through_c) = swung(&x.ell_o(), &x.ell_orth, &x.phase[LOOP]);
    let (r_o, h, d_o) = lifted(enc, &now, &x.lift);
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
    let tilt = x.tilt();
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
    let (g, ab) = spin_torque(x, &from, &d_o, p, &torque_orth, &m, &scar);
    // Descent: its weight past its round stands one place past its mark.
    settle(x, &levers, place.as_ref(), cross);
    (g, ab)
}

/// The torque on one mote's SPINS over one bell's passage, the one law at
/// every tier (a galaxy's core on its motes, a star on its system's): from
/// the orbit phase it stood at (`from`) and the reach and deficit it turns
/// at (`r_o`, `d_o`, inside the hold `h`), the own family's push in the orbit
/// plane, the parent's push along
/// the orthogonal orbit (`torque_orth`) less the hold's return, and what is
/// not delivered in step left as heat. Returns the spin given (Σ m·Δℓ, the
/// antipole's to pay) and the pulse absorbed (Σ m·|Δℓ|).
#[allow(clippy::too_many_arguments)]
pub fn spin_torque(
    x: &mut Mote,
    from: &[Ratio],
    d_o: &Ratio,
    p: &Ratio,
    torque_orth: &Ratio,
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
    let t = tilt_of(&x.tilt());
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
    // No radial law at the state level (operator 2026-11-06: the spin
    // balance): a mote's reach is the reading `ℓ/H(<ℓ)` of its spin, and its
    // orbit stands where its spin and the hold enclosed balance; the band
    // (docs/104 §6) and its rest are retired (register K12), the coupling
    // acting on spin alone (`shear`, docs/121 §10).
    let band = ratio(0, 1);
    // Each plane's spin: its share of the orbit spin, its own torque and
    // its share of the band (radial). A plane torqued past zero carries the
    // mote through the centre in that plane (its phase there on half a
    // turn), never piled at zero.
    let mut planes_l: Vec<Ratio> = Vec::with_capacity(t.len());
    for (j, tk) in t.iter().enumerate() {
        let lk = x.ell_o() * tk.clone()
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
    // Each plane keeps its own spin (docs/120 §8b′, a plane closes on
    // itself): the components are the state, each on the grain, never summed
    // into one spin and split again by a tilt.
    let planes_l: Vec<Ratio> = planes_l.iter().map(crate::ring::grain).collect();
    let new_o = planes_l.iter().fold(ratio(0, 1), |s, l| s + l.clone());
    let dl = new_o.clone() - x.ell_o();
    absorbed += m.clone()
        * if dl < ratio(0, 1) {
            -dl.clone()
        } else {
            dl.clone()
        };
    given += m.clone() * dl;
    x.spins = planes_l;
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

/// The shear (docs/121 §10–§11; operator 2026-11-05: "shear needs to be the
/// degree of modulation at any dimensional level"), at the phase level: each
/// mote is a dipole on each spatial plane, its pole the sign of its span
/// (closed frames against open, `coupling::pole`), its strength its open
/// share, its orientation its phase there. It answers its range's field as
/// one (docs/121 §9.2): the motes whose own enclosures meet its own on reach
/// (`|R_i − R_j| ≤ R_i·e_i + R_j·e_j`, `own_enclosure`), each by the ring
/// law's nearness of their open frames and the hemisphere law's sign. Its
/// rate on plane k, `x = 2δ·ℓ_k/ℓ` (the torque's rate, a share of the
/// luminal bound), is composed by the flow law toward the range's
/// holding-weighted rate where the field attracts and away from it where it
/// repels (`coupling::pulled`). At its reach the spin follows the rate,
/// `ℓ_k′ = ℓ_k·x′/x`, and the range pays the change by holding, so `Σ m·ℓ_k`
/// is unchanged on every plane. All is read from the bell's start, then
/// applied; a component carried past zero passes through the centre on its
/// own plane (docs/120 §8b′). Each mote's step is read on the grain, the
/// stored components stand on it once summed, and the grain's remainder is
/// not booked. Returns the spin moved.
pub fn shear(motes: &mut [Mote], enc: &Enclosed, total: &Ratio) -> Ratio {
    shear_booked(motes, enc, total, false).0
}

/// The motes whose own enclosures meet each one's on reach, itself excluded:
/// `|R_i − R_j| ≤ x_i + x_j` with `x = R·e` its extent, which is that their
/// spans `[R − x, R + x]` overlap. The spans in order of their lower ends: each
/// meets exactly the spans after it whose lower end stands within its upper
/// end, so one walk forward from each finds every meeting pair once, in
/// `n log n` plus the pairs, never a test of every pair (register A28).
fn meeting(reach: &[Ratio], extent: &[Ratio]) -> Vec<Vec<usize>> {
    let n = reach.len();
    let lo: Vec<Ratio> = (0..n)
        .map(|i| reach[i].clone() - extent[i].clone())
        .collect();
    let hi: Vec<Ratio> = (0..n)
        .map(|i| reach[i].clone() + extent[i].clone())
        .collect();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|a, b| lo[*a].cmp(&lo[*b]).then(a.cmp(b)));
    let mut met: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (p, &i) in order.iter().enumerate() {
        for &j in &order[p + 1..] {
            if lo[j] > hi[i] {
                break;
            }
            met[i].push(j);
            met[j].push(i);
        }
    }
    for m in met.iter_mut() {
        m.sort_unstable();
    }
    met
}

/// The shear with its grain remainder per plane, `Σ m·(stored − exact)` in
/// the enclosure's unit, where `book` asks for it (`shear`; summed exactly
/// over every mote its denominators compound, so the read leaves it).
fn shear_booked(
    motes: &mut [Mote],
    enc: &Enclosed,
    total: &Ratio,
    book: bool,
) -> (Ratio, Vec<Ratio>) {
    let planes = SPACE.len();
    let mut left = vec![ratio(0, 1); planes];
    let n = motes.len();
    if n < 2 || total.is_zero() {
        return (ratio(0, 1), left);
    }
    // Each mote's readings at the bell's start: its holding w, its share m in
    // the enclosure's unit, its open frames in that unit, its pole and open
    // share, its reach and extent, and its rate on each plane.
    let w: Vec<Ratio> = motes
        .iter()
        .map(|x| x.amount.clone() * weight(&x.key))
        .collect();
    let m: Vec<Ratio> = w.iter().map(|h| h.clone() / total.clone()).collect();
    let frames: Vec<Ratio> = motes
        .iter()
        .map(|x| {
            x.amount.clone() * ratio(i64::from(crate::chem::unpaired(&x.key)), 1) / total.clone()
        })
        .collect();
    // The dipole on its 3Z frames (docs/121 §10.1; Mycelium F1 with
    // Xylarium): its pole the sign of held against open, its strength the
    // open share of its frames, `U/3Z`.
    let pole: Vec<Ratio> = motes
        .iter()
        .map(|x| {
            let (a, q) = crate::chem::frames(&x.key);
            crate::coupling::pole(&ratio(i64::from(a), 1), &ratio(i64::from(q), 1))
        })
        .collect();
    let o: Vec<Ratio> = motes
        .iter()
        .map(|x| {
            let (a, q) = crate::chem::frames(&x.key);
            if a + q == 0 {
                ratio(0, 1)
            } else {
                ratio(i64::from(q), i64::from(a + q))
            }
        })
        .collect();
    let mut reach = Vec::with_capacity(n);
    let mut extent = Vec::with_capacity(n);
    let mut rate: Vec<Vec<Ratio>> = Vec::with_capacity(n);
    for x in motes.iter() {
        let ell = x.ell_o();
        let (r, delta) = x.reach_in(enc);
        let d = stream_delta(&delta, x.vacuum);
        extent.push(r.clone() * own_enclosure(x, enc, total)[0].clone());
        reach.push(r);
        rate.push(
            (0..planes)
                .map(|k| match x.spins.get(k) {
                    Some(l) if !ell.is_zero() => ratio(2, 1) * d.clone() * l.clone() / ell.clone(),
                    _ => ratio(0, 1),
                })
                .collect(),
        );
    }
    let at = |x: &Mote, k: usize| {
        x.phase
            .get(SPACE[k])
            .cloned()
            .unwrap_or_else(|| ratio(0, 1))
    };
    let bound = |r: &Ratio| r.abs() < ratio(1, 1);
    let range = meeting(&reach, &extent);
    let mut dl = vec![vec![ratio(0, 1); planes]; n];
    let mut moved = ratio(0, 1);
    for i in 0..n {
        let near = &range[i];
        let whole = near.iter().fold(ratio(0, 1), |s, &j| s + w[j].clone());
        if near.is_empty() || whole.is_zero() || m[i].is_zero() {
            continue;
        }
        for k in 0..planes {
            let x = &rate[i][k];
            let Some(lk) = motes[i].spins.get(k) else {
                continue;
            };
            if x.is_zero() || !bound(x) {
                continue;
            }
            // The range's rate on this plane, by holding.
            let xbar = near
                .iter()
                .fold(ratio(0, 1), |s, &j| s + w[j].clone() * rate[j][k].clone())
                / whole.clone();
            if !bound(&xbar) {
                continue;
            }
            let field = crate::coupling::field(
                &near
                    .iter()
                    .map(|&j| crate::coupling::Neighbour {
                        holding: w[j].clone(),
                        pole: pole[j].clone(),
                        open: o[j].clone(),
                        nearness: crate::coupling::nearness(
                            &(reach[i].clone() - reach[j].clone()),
                            &(frames[i].clone() + frames[j].clone()),
                        ),
                        side: crate::coupling::hemisphere(&at(&motes[i], k), &at(&motes[j], k)),
                    })
                    .collect::<Vec<_>>(),
            );
            let Some(x2) = crate::coupling::pulled(x, &xbar, &pole[i], &o[i], &field) else {
                continue;
            };
            if x2 == *x {
                continue;
            }
            // The spin follows the rate at the mote's reach, the step read on
            // the grain (the carrier, never the law: unheld, each step's
            // denominators compound through the range's payments and the
            // spin moved); the range pays it by holding, exactly.
            let step = crate::ring::grain(&(lk.clone() * x2 / x.clone())) - lk.clone();
            if step.is_zero() {
                continue;
            }
            let given = m[i].clone() * step.clone();
            dl[i][k] += step;
            // Each neighbour pays by its holding, `given·w_j/(whole·m_j)`,
            // which is `given·total/whole` for every one of them, held on the
            // grain as the step is (each range has its own `whole`, so a
            // neighbour's exact payments from many ranges compound their
            // denominators without bound); what the grain leaves is booked
            // with the shear's remainder.
            let pay = crate::ring::grain(&(given.clone() * total.clone() / whole.clone()));
            let mut paid = ratio(0, 1);
            for &j in near {
                if !m[j].is_zero() {
                    dl[j][k] -= pay.clone();
                    paid += m[j].clone() * pay.clone();
                }
            }
            if book {
                left[k] += given.clone() - paid;
            }
            moved += given.abs();
        }
    }
    for (i, x) in motes.iter_mut().enumerate() {
        for (k, d) in dl[i].iter().enumerate() {
            if d.is_zero() {
                continue;
            }
            if x.spins.len() <= k {
                x.spins.resize(planes, ratio(0, 1));
            }
            let exact = x.spins[k].clone() + d.clone();
            // Past zero, through the centre on this plane alone: its phase
            // there half a turn on, its magnitude kept.
            let stored = if exact < ratio(0, 1) {
                full(&mut x.phase);
                let ph = &mut x.phase[SPACE[k]];
                *ph = frac(&(ph.clone() + ratio(1, 2)));
                let s = crate::ring::grain(&-exact.clone());
                if book {
                    left[k] += m[i].clone() * (-s.clone() - exact);
                }
                s
            } else {
                let s = crate::ring::grain(&exact);
                if book {
                    left[k] += m[i].clone() * (s.clone() - exact);
                }
                s
            };
            x.spins[k] = stored;
        }
    }
    (moved, left)
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
                spins: Mote::spins_of(&ell_o, &[]),
                ell_orth: ratio(0, 1),
                phase,
                agitation: crate::ring::grain(agitation),
                born,
                vacuum: false,
                lever: Vec::new(),
                lever_place: ratio(0, 1),
                lift: ratio(0, 1),
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
        .partition(|x| x.reach_in(&enc).0 >= rim);
    *motes = stay;
    gone
}

/// The galaxy's held and free manifests, read from its motes: on the
/// disc leg by the cut in its one enclosure; on the axial leg, in transit, never held.
pub fn read_cut(motes: &[Mote], enc: &Enclosed, whole: &Ratio) -> (Manifest, Manifest) {
    let (mut kept, mut free) = (Manifest::new(), Manifest::new());
    for x in motes {
        let share = if on_disc_leg(x) {
            held(x, enc, whole)
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
/// hemisphere in every plane, the S⁰ face test) whose own extents
/// `r(1 ± e)` (`e` its pinch share) overlap link, and the links are
/// transitive: a chain is a
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
/// plane), motes whose own extents `r(1 ± e)` overlap, linked
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
    total: &Ratio,
    disc_leg: bool,
) -> std::collections::BTreeMap<Vec<bool>, Vec<(Vec<usize>, Ratio)>> {
    let mut beads_by_face: std::collections::BTreeMap<Vec<bool>, Vec<(Vec<usize>, Ratio)>> =
        std::collections::BTreeMap::new();
    if total.is_zero() {
        return beads_by_face;
    }
    let half = ratio(1, 2);
    let hh: Vec<Ratio> = motes.iter().map(|x| x.hold_in(enc)).collect();
    let rr: Vec<Ratio> = motes.iter().map(|x| x.reach_in(enc).0).collect();
    // Each mote's own extent about its reach (docs/120 §8c): its pinch share
    // `e = m/(m + H(<ℓ))` of its reach on each side, derived, never one band
    // for the galaxy.
    let ee: Vec<Ratio> = motes
        .iter()
        .zip(&hh)
        .map(|(x, h)| {
            crate::ring::pinch_share(&(x.amount.clone() * weight(&x.key) / total.clone()), h)
        })
        .collect();
    let lov: Vec<Ratio> = rr
        .iter()
        .zip(&ee)
        .map(|(r, e)| r.clone() * (ratio(1, 1) - e.clone()))
        .collect();
    let hiv: Vec<Ratio> = rr
        .iter()
        .zip(&ee)
        .map(|(r, e)| r.clone() * (ratio(1, 1) + e.clone()))
        .collect();
    let delta_of = |j: usize, r: &Ratio| {
        stream_delta(
            &(hh[j].clone() / (r.clone() + hh[j].clone())),
            motes[j].vacuum,
        )
    };
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

pub fn strands(motes: &[Mote], enc: &Enclosed, total: &Ratio) -> Vec<Strand> {
    if total.is_zero() {
        return Vec::new();
    }
    // Each mote's reach and band, read once by the one enclosure (exact;
    // recomputing them at every use was most of the cost): its reach `ℓ/H(<ℓ)`
    // and the hold it turns about.
    let hh: Vec<Ratio> = motes.iter().map(|x| x.hold_in(enc)).collect();
    let rr: Vec<Ratio> = motes.iter().map(|x| x.reach_in(enc).0).collect();
    // Each mote's own extent about its reach (docs/120 §8c): its pinch share
    // `e = m/(m + H(<ℓ))` of its reach on each side, derived, never one band
    // for the galaxy.
    let ee: Vec<Ratio> = motes
        .iter()
        .zip(&hh)
        .map(|(x, h)| {
            crate::ring::pinch_share(&(x.amount.clone() * weight(&x.key) / total.clone()), h)
        })
        .collect();
    let lov: Vec<Ratio> = rr
        .iter()
        .zip(&ee)
        .map(|(r, e)| r.clone() * (ratio(1, 1) - e.clone()))
        .collect();
    let hiv: Vec<Ratio> = rr
        .iter()
        .zip(&ee)
        .map(|(r, e)| r.clone() * (ratio(1, 1) + e.clone()))
        .collect();
    // The deficit at a reach about the hold a member turns about.
    let delta_of = |j: usize, r: &Ratio| {
        stream_delta(
            &(hh[j].clone() / (r.clone() + hh[j].clone())),
            motes[j].vacuum,
        )
    };
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
        // Its extent round the orbit: in each spatial plane, the shortest
        // arc that holds every member, on the circle (one less the widest
        // gap between neighbouring members, the gap across 0 included),
        // their mean over the planes.
        let extent = space_mean(|k| {
            let mut orbit: Vec<Ratio> = members
                .iter()
                .map(|&j| frac(motes[j].phase.get(k).unwrap_or(&ratio(0, 1))))
                .collect();
            circular_extent(&mut orbit)
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
    let beads_by_face = beads(motes, enc, total, true);
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

/// The attractor's keep at deficit δ (docs/100 §8.1, E38): a displacement
/// keeps `turned/(drag + turned)` a bell, `turned = 2δ`, `drag = 1 − δ`,
/// which is `1 − 1/D(δ)` with `D` LAW-DILATION (`ring::dilation`): at the
/// throat `D(½) = 3`, a keep of ⅔. None at `δ = 1`, where nothing drags and
/// the whole displacement is kept.
pub fn attractor_keep(delta: &Ratio) -> Option<Ratio> {
    let d = crate::ring::dilation(delta)?;
    if d.is_zero() {
        return None;
    }
    Some(ratio(1, 1) - ratio(1, 1) / d)
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
            .map(|x| (x.amount.clone() * weight(&x.key), x.ell_o(), x.tilt()))
            .collect::<Vec<_>>(),
    );
    if plane.is_empty() {
        return plane;
    }
    for x in motes.iter_mut() {
        let d = stream_delta(&x.reach_in(&enc).1, x.vacuum);
        let Some(keep) = attractor_keep(&d) else {
            continue;
        };
        let t = tilt_of(&x.tilt());
        let moved: Vec<Ratio> = t
            .iter()
            .zip(&plane)
            .map(|(tk, pk)| pk.clone() + (tk.clone() - pk.clone()) * keep.clone())
            .collect();
        x.set_tilt(tilt_from(&moved));
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
/// at ρ = 0. Each mote's matter is spread over its own extent `r(1 ± e)`. A band's
/// pair holds where its matter's bond meets its heat and the tide across
/// it (`formation::crest_collapse`, docs/96 §8); a band where a
/// star already stands is that star's (its capture reads it). Returns the
/// holding bands as bindings: (the band's middle reach, each member's share),
/// every member on the disc leg (docs/99 §2.1).
pub fn band_pairs(
    motes: &[Mote],
    enc: &Enclosed,
    plane: &[Ratio],
    g_total: &Ratio,
) -> Vec<(Ratio, Vec<(usize, Ratio)>)> {
    let ring = enc.base();
    let whole = enc.whole();
    if ring.is_zero() || whole.is_zero() || motes.is_empty() {
        return Vec::new();
    }
    let total = holding(motes);
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
    // Each mote's own extent about its reach (docs/120 §8c), its pinch share.
    let band = |i: usize| {
        let r = motes[i].reach_in(&enc).0;
        let e = own_enclosure(&motes[i], enc, g_total)[0].clone();
        (r.clone() * (ratio(1, 1) - e.clone()), r * (ratio(1, 1) + e))
    };
    let bands_all: Vec<(Ratio, Ratio)> = (0..motes.len()).map(band).collect();
    // What stands inside the pair at `x`: the ring's own share and every
    // mote's holding by its band's share within the pair. Each share divides
    // by its own band's width, so the exact sum's denominator grows with
    // every mote; `read` gives each share's grain mark below (`up` false) or
    // the next (`up` true), bounding the sum with narrow terms.
    let inside_by = |x: &Ratio, read: &dyn Fn(Ratio) -> Ratio| -> Ratio {
        let (a, b) = pair(x);
        (0..motes.len()).fold(ring.clone() / whole.clone(), |acc, i| {
            acc + m_of(i) * read(share_in(&bands_all[i].0, &bands_all[i].1, &a, &b))
        })
    };
    let step_mark = crate::ring::grain_step();
    let below = |s: Ratio| crate::ring::grain(&s);
    let above = |s: Ratio| {
        let lo = crate::ring::grain(&s);
        if lo == s {
            lo
        } else {
            lo + step_mark.clone()
        }
    };
    let exact = |s: Ratio| s;
    crate::profile::mark("pairs:strip");
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
        // The step's outcome rises with what stands inside (`strip_step`),
        // so it is read at the two bounds and, only where they differ, at
        // the exact sum.
        let (lo, hi) = (
            strip_step(&at, &inside_by(&at, &below)),
            strip_step(&at, &inside_by(&at, &above)),
        );
        let outcome = if lo == hi {
            lo
        } else {
            strip_step(&at, &inside_by(&at, &exact))
        };
        match outcome {
            Strip::Next(next) => rhos.push(next),
            Strip::Held | Strip::Whole => break,
        }
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
        crate::profile::mark("pairs:members");
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
        crate::profile::mark("pairs:sums");
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
        crate::profile::mark("pairs:lead");
        // Its members' net lead (their tilts blended by holding: aligned
        // leads add, opposed cancel) against the disc plane, by holding.
        let lead = blend(
            &members
                .iter()
                .map(|(i, s)| (held_of(*i, s), motes[*i].ell_o(), motes[*i].tilt()))
                .collect::<Vec<_>>(),
        );
        // Its torsion, on the pair its lead is wound tightest on (the bivector
        // read per pair, `torsion::tension`), by holding.
        let tau = m.clone() * crate::torsion::tension(&lead, plane);
        // The nuclear layer ν: the members' holding by their nuclei's grip at
        // each one's held share `g = 1 − δ`. Summed exactly, its denominator
        // grows with every member (each δ its own), so it is read certified
        // first: each g between its grain mark below and the next, the grip
        // linear and increasing in g, so ν between the two sums
        // (`crest_count`); the exact sum only where they decide differently.
        crate::profile::mark("pairs:nu");
        let g_of = |i: usize| {
            crate::polarity::faces(&stream_delta(
                &motes[i].reach_in(&enc).1,
                motes[i].vacuum,
            ))
            .0
        };
        let nu_at = |g: &dyn Fn(usize) -> Ratio| {
            members.iter().fold(ratio(0, 1), |acc, (i, s)| {
                acc + held_of(*i, s) * grip_share(&motes[*i], &g(*i))
            })
        };
        let step = crate::ring::grain_step();
        let g_lo = |i: usize| crate::ring::grain(&g_of(i));
        let g_hi = |i: usize| {
            let g = g_of(i);
            let lo = crate::ring::grain(&g);
            if lo == g {
                lo
            } else {
                lo + step.clone()
            }
        };
        let Some(count) = crest_count(
            &(
                m.clone(),
                mu.clone(),
                tau.clone(),
                a.clone(),
                mid.clone(),
                h_mid.clone(),
            ),
            &nu_at(&g_lo),
            &nu_at(&g_hi),
            &|| nu_at(&g_of),
        ) else {
            continue;
        };
        if count == 0 {
            continue;
        }
        crate::profile::mark("pairs:arcs");
        // The disc: the plane its tilt leans in most, or every plane that
        // ties for it (none chosen over another by its index; docs/78). A
        // binding's arc is its place in each of them.
        let lean = tilt_of(plane);
        let most = lean.iter().max().cloned().unwrap_or_else(|| ratio(0, 1));
        let disc: Vec<usize> = (0..lean.len())
            .filter(|&j| lean[j] == most)
            .map(|j| SPACE[j])
            .collect();
        let n = Ratio::from_integer(num_bigint::BigInt::from(count));
        let mut arcs: std::collections::BTreeMap<Vec<num_bigint::BigInt>, Vec<(usize, Ratio)>> =
            std::collections::BTreeMap::new();
        for (i, sh) in members {
            let at: Vec<num_bigint::BigInt> = disc
                .iter()
                .map(|&k| {
                    (frac(motes[i].phase.get(k).unwrap_or(&ratio(0, 1))) * n.clone())
                        .floor()
                        .to_integer()
                })
                .collect();
            arcs.entry(at).or_default().push((i, sh));
        }
        for (_, binding) in arcs {
            out.push((crate::ring::grain(&mid), binding));
        }
    }
    out
}

/// One step of the strip (`band_pairs`) from the share `s_in` standing
/// inside the pair at `at`: none outside (`Whole`, the pair holds it all),
/// the step no wider (`Held`, `k ≤ 1`), or the next pair's edge on the
/// grain, `⌊at·k⌋`, `k = s_in/(1 − s_in)·(1 + at)/(1 − at)`. As `s_in`
/// rises the outcome passes `Held`, then edges never narrower, then `Whole`.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Strip {
    Held,
    Next(Ratio),
    Whole,
}

fn strip_step(at: &Ratio, s_in: &Ratio) -> Strip {
    let g_out = ratio(1, 1) - s_in.clone();
    if g_out <= ratio(0, 1) {
        return Strip::Whole;
    }
    let k_at = s_in.clone() / g_out * (ratio(1, 1) + at.clone()) / (ratio(1, 1) - at.clone());
    if k_at <= ratio(1, 1) {
        return Strip::Held;
    }
    let next = crate::ring::grain(&(at.clone() * k_at));
    Strip::Next(if next < ratio(1, 1) {
        next
    } else {
        ratio(1, 1)
    })
}

/// The crest's count of bindings (`formation::crest_collapse`) from its
/// nuclear layer certified between `nu_lo` and `nu_hi`: the count falls as ν
/// rises (a larger ν binds wider), so where both bounds give the same
/// outcome it is the exact one; only where they differ is the exact ν read.
fn crest_count(
    c: &(Ratio, Ratio, Ratio, Ratio, Ratio, Ratio),
    nu_lo: &Ratio,
    nu_hi: &Ratio,
    exact: &dyn Fn() -> Ratio,
) -> Option<u64> {
    let (m, mu, tau, a, r, h) = c;
    let at = |nu: &Ratio| crate::formation::crest_collapse(m, mu, tau, nu, a, r, h).map(|x| x.1);
    let (lo, hi) = (at(nu_lo), at(nu_hi));
    if lo == hi {
        return lo;
    }
    at(&exact())
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
            let exact_o = (my.clone() * y.ell_o() + mx.clone() * x.ell_o()) / both.clone();
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
            let joined = blend(&[
                (my.clone(), y.ell_o(), y.tilt()),
                (mx.clone(), x.ell_o(), x.tilt()),
            ]);
            y.spins = Mote::spins_of(&go, &joined);
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
                    spins: Mote::spins_of(&crate::ring::grain(&ell), &[]),
                    ell_orth: ratio(0, 1),
                    phase,
                    agitation: ratio(0, 1),
                    born: 0,
                    vacuum: false,
                    lever: Vec::new(),
                    lever_place: ratio(0, 1),
                    lift: ratio(0, 1),
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
        let d = x.reach_in(&enc).1;
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
/// (holding-weighted), its reach `R` (its pinch), its width (its members'
/// own extents `R(1 ± e)`, edge to edge), its density `ρ = m/width`, its pressure `P = ρ·ā` plus its
/// magnetic pressure, and its magnetization `M = Σ u·in_step(φ, φ̄)` (each
/// member's open frames by its field phase's in-step with the ring's mean,
/// the field phase its circulation phase, frozen into the particle, so the
/// rings' differential turning winds the field). Between neighbouring rings
/// on a face (the inner at the lesser reach) the pressure's exchange moves
/// turning by the exchange law (the outer gains `u·m_in`, the inner gives
/// `u·m_out`, `u = share·(ℓ̄_out − ℓ̄_in)/(m_in + m_out)`, Σ m·ℓ unchanged),
/// its share the hydrostatic balance restoring the pair, signed. The field
/// between rings is the one dipole coupling (`shear`, register C18): no
/// second, outward-only magnetic exchange stands here. Every member of a
/// ring moves alike. Returns (the pressure's spin moved, the spin moved
/// around the shells' circles, and the number of shell planes that parted
/// into cells), by holding.
pub fn fluid(
    motes: &mut [Mote],
    enc: &Enclosed,
    total: &Ratio,
) -> (Ratio, Ratio, usize) {
    if total.is_zero() || motes.len() < 2 {
        return (ratio(0, 1), ratio(0, 1), 0);
    }
    let rings_by_face = beads(motes, enc, total, false);
    let share = |x: &Mote| x.amount.clone() * weight(&x.key) / total.clone();
    let mut dl = vec![ratio(0, 1); motes.len()];
    let mut moved_p = ratio(0, 1);
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
                let ell = members
                    .iter()
                    .fold(ratio(0, 1), |a, &i| a + share(&motes[i]) * motes[i].ell_o())
                    / m.clone();
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
                // Its width, its members' own extents edge to edge (each
                // `R(1 ± e)`, `e` its pinch share), never one band.
                let edge = |i: usize, sign: i64| {
                    let r = motes[i].reach_in(&enc).0;
                    let e = own_enclosure(&motes[i], enc, total)[0].clone();
                    r.clone() * (ratio(1, 1) + ratio(sign, 1) * e)
                };
                let width = members.iter().map(|&i| edge(i, 1)).max().unwrap_or_else(|| ratio(0, 1))
                    - members.iter().map(|&i| edge(i, -1)).min().unwrap_or_else(|| ratio(0, 1));
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
            // S3 (docs/106 §2): the pressure's exchange is the hydrostatic
            // balance restoring the pair, signed (outward where the inner
            // ring is over-pressured, inward where under), zero at balance.
            // The field's coupling between rings is the one dipole coupling
            // (`shear`, register C18), never a second exchange here.
            let one = ratio(1, 1);
            let s_p = restoring(
                &inn.4,
                &out.4,
                &(one.clone() - inn.6.clone()),
                &(one - out.6.clone()),
            );
            {
                let (s, moved) = (s_p, &mut moved_p);
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
        // this face whose own extents `R(1 ± e)` overlap (connected by their bands,
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
                let (r, d) = motes[i].reach_in(&enc);
                (i, r, stream_delta(&d, motes[i].vacuum))
            })
            .collect();
        on_face.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        // Each member's own extent about its reach (`R(1 ± e)`).
        let one = ratio(1, 1);
        let ext = |i: usize| own_enclosure(&motes[i], enc, total)[0].clone();
        let lo = |i: usize, r: &Ratio| r.clone() * (one.clone() - ext(i));
        let hi = |i: usize, r: &Ratio| r.clone() * (one.clone() + ext(i));
        let mut shells: Vec<Vec<(usize, Ratio, Ratio)>> = Vec::new();
        let mut reach_hi = ratio(-1, 1);
        for m in on_face {
            if shells.is_empty() || lo(m.0, &m.1) > reach_hi {
                shells.push(Vec::new());
            }
            if hi(m.0, &m.1) > reach_hi {
                reach_hi = hi(m.0, &m.1);
            }
            shells.last_mut().expect("a shell").push(m);
        }
        for shell in shells.iter().filter(|s| s.len() >= 2) {
            let width = shell.iter().map(|m| hi(m.0, &m.1)).max().expect("members")
                - shell.iter().map(|m| lo(m.0, &m.1)).min().expect("members");
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
                        let ell = mean(&|i| motes[i].ell_o());
                        let tilt = mean(&|i| tilt_of(&motes[i].tilt())[j].clone());
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
            let now = crate::ring::grain(&(x.ell_o() + d));
            let ell = through_centre(now, &mut x.phase);
            x.set_ell_o(ell);
        }
    }
    (moved_p, moved_around, parted)
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
        let reach: Vec<Ratio> = ms.iter().map(|x| x.ell_o() / h.clone()).collect();
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
        let mut by: Vec<(Ratio, usize)> =
            ms.iter().enumerate().map(|(i, x)| (x.ell_o(), i)).collect();
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
            let mut by: Vec<(Ratio, usize)> =
                ms.iter().enumerate().map(|(i, x)| (x.ell_o(), i)).collect();
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
    fn the_strips_step_rises_with_what_stands_inside() {
        // The order the certified strip relies on: Held, then edges never
        // narrower, then Whole, as the inside share rises.
        use super::{strip_step, Strip};
        let rank = |s: &Strip| match s {
            Strip::Held => (0, ratio(0, 1)),
            Strip::Next(r) => (1, r.clone()),
            Strip::Whole => (2, ratio(0, 1)),
        };
        for at in [ratio(1, 10), ratio(1, 3), ratio(7, 10)] {
            let mut last = rank(&strip_step(&at, &ratio(0, 1)));
            for k in 1..=400 {
                let now = rank(&strip_step(&at, &ratio(k, 400)));
                assert!(now >= last, "at {at}, s {k}/400");
                last = now;
            }
        }
    }

    #[test]
    fn the_crests_count_certified_is_the_exact_count() {
        // Wherever the bounds agree they give the exact count; where they do
        // not, the exact ν decides. Swept over ν and its bounds.
        let c = (
            ratio(3, 10),
            ratio(1, 10),
            ratio(0, 1),
            ratio(1, 50),
            ratio(2, 1),
            ratio(1, 1),
        );
        let exact_at = |nu: &Ratio| {
            crate::formation::crest_collapse(&c.0, &c.1, &c.2, nu, &c.3, &c.4, &c.5).map(|x| x.1)
        };
        for k in 0..200 {
            let nu = ratio(k, 97);
            let (lo, hi) = (ratio(k, 97) - ratio(1, 300), ratio(k, 97) + ratio(1, 300));
            let lo = if lo < ratio(0, 1) { ratio(0, 1) } else { lo };
            assert_eq!(
                super::crest_count(&c, &lo, &hi, &|| nu.clone()),
                exact_at(&nu),
                "ν = {k}/97"
            );
        }
    }

    #[test]
    fn a_motes_lever_keeps_its_true_place() {
        use super::{settle, true_phase, Mote};
        let mut x = Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            spins: Mote::spins_of(&ratio(1, 1), &[]),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(1, 60), ratio(0, 1)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
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
    fn motes_the_bounce_moves_together_bond_and_motes_moved_apart_do_not() {
        // docs/111 §4.2: near on the sphere, open, and moved in step.
        let mote = |key: &str, ph: (i64, i64)| Mote {
            key: crate::chem::Kind::parse(key).expect("a kind"),
            amount: ratio(1, 1),
            spins: Mote::spins_of(&ratio(1, 1), &[]),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(ph.0, ph.1), ratio(0, 1), ratio(0, 1)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
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
    fn a_whole_turn_averages_nothing_and_half_turns_sign_the_hemisphere() {
        assert_eq!(mean_response(&ratio(1, 3), &ratio(1, 1)), ratio(0, 1));
        assert_eq!(mean_response(&ratio(0, 1), &ratio(1, 2)), ratio(1, 2));
        assert_eq!(mean_response(&ratio(1, 2), &ratio(1, 2)), ratio(-1, 2));
        // Across the bell: from ¾ to 1¼, the loss then the gain cancel.
        assert_eq!(mean_response(&ratio(3, 4), &ratio(1, 2)), ratio(0, 1));
    }

    #[test]
    fn the_screened_collections_are_the_exact_ones() {
        // Phases on a fine grid near and far from every threshold, tilts
        // uneven, moves of both senses: the screened pass reads the same
        // bonds as testing every pair exactly.
        let kinds = ["W0001", "W0004", "W0007", "W0010"];
        let mut motes = Vec::new();
        let mut moves = Vec::new();
        for i in 0..48i64 {
            let mut x = Mote {
                key: crate::chem::Kind::parse(kinds[(i % 4) as usize]).expect("a kind"),
                amount: ratio(1, 1),
                spins: Mote::spins_of(
                    &ratio(1, 2),
                    &tilt_from(&[ratio(1 + i % 3, 1), ratio(2, 1), ratio(1 + i % 5, 1)]),
                ),
                ell_orth: ratio(0, 1),
                phase: rest(ratio(1, 2)),
                agitation: ratio(0, 1),
                born: 0,
                vacuum: false,
                lever: Vec::new(),
                lever_place: ratio(0, 1),
                lift: ratio(0, 1),
            };
            for (j, &k) in super::SPACE.iter().enumerate() {
                x.phase[k] = ratio((i * (7 + j as i64)) % 24, 24);
            }
            let mut m = vec![ratio(0, 1); super::PLANES];
            for (j, &k) in super::SPACE.iter().enumerate() {
                // Half the moves on the spin grid (the whole-number test),
                // half off it (the rational test).
                let den = if i % 2 == 0 { 24 } else { 26 };
                m[k] = ratio((i * (5 + j as i64)) % 13 - 6, den);
            }
            motes.push(x);
            moves.push(if i % 11 == 0 { Vec::new() } else { m });
        }
        let mut exact: Vec<(usize, usize)> = Vec::new();
        for i in 0..motes.len() {
            for j in i + 1..motes.len() {
                if !moves[i].is_empty()
                    && !moves[j].is_empty()
                    && super::bonded(&motes[i], &motes[j], &moves[i], &moves[j])
                {
                    exact.push((i, j));
                }
            }
        }
        assert!(!exact.is_empty(), "the fixture bonds some pairs");
        // The exact collections: connected sets of the exact bonds.
        let mut root: Vec<usize> = (0..motes.len()).collect();
        fn find(root: &mut [usize], i: usize) -> usize {
            if root[i] == i {
                i
            } else {
                let r = find(root, root[i]);
                root[i] = r;
                r
            }
        }
        for &(i, j) in &exact {
            let (a, b) = (find(&mut root, i), find(&mut root, j));
            if a != b {
                root[a.max(b)] = a.min(b);
            }
        }
        let mut sets: std::collections::BTreeMap<usize, Vec<usize>> = Default::default();
        for i in 0..motes.len() {
            if exact.iter().any(|&(a, b)| a == i || b == i) {
                let r = find(&mut root, i);
                sets.entry(r).or_default().push(i);
            }
        }
        assert_eq!(
            super::collections(&motes, &moves),
            sets.into_values().collect::<Vec<_>>()
        );
    }

    #[test]
    fn an_extent_is_read_on_the_circle_across_zero() {
        let mut across = vec![ratio(19, 20), ratio(1, 20)];
        assert_eq!(super::circular_extent(&mut across), ratio(1, 10));
        let mut inside = vec![ratio(1, 4), ratio(1, 2), ratio(3, 8)];
        assert_eq!(super::circular_extent(&mut inside), ratio(1, 4));
        assert_eq!(super::circular_extent(&mut [ratio(1, 3)]), ratio(0, 1));
        let mut even = vec![ratio(0, 1), ratio(1, 2)];
        assert_eq!(super::circular_extent(&mut even), ratio(1, 2));
    }

    #[test]
    fn a_tilt_knows_its_spins_never_which_plane_is_which() {
        // Every permutation of the spins permutes the tilt the same way, so
        // no plane is favoured by its index (docs/78: no fill orders).
        let perms = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];
        for spins in [
            [ratio(1, 7), ratio(2, 7), ratio(4, 7)],
            [ratio(1, 3), ratio(1, 3), ratio(1, 3)],
            [ratio(1, 1), ratio(1, 1), ratio(5, 1)],
            [ratio(2, 1), ratio(0, 1), ratio(2, 1)],
            [ratio(1, 11), ratio(1, 11), ratio(1, 13)],
        ] {
            let t = tilt_from(&spins);
            assert_eq!(
                t.iter().fold(ratio(0, 1), |s, x| s + x.clone()),
                ratio(1, 1)
            );
            for p in perms {
                let moved: Vec<Ratio> = p.iter().map(|&j| spins[j].clone()).collect();
                let want: Vec<Ratio> = p.iter().map(|&j| t[j].clone()).collect();
                assert_eq!(tilt_from(&moved), want, "permuted with {p:?}");
            }
        }
        // Equal spins part equally; a plane that does not turn holds none.
        assert_eq!(
            tilt_from(&[ratio(1, 7), ratio(1, 7), ratio(1, 7)]),
            vec![ratio(1, 3); 3]
        );
        assert_eq!(
            tilt_from(&[ratio(1, 7), ratio(0, 1), ratio(1, 7)])[1],
            ratio(0, 1)
        );
    }

    #[test]
    fn a_tilt_parts_the_spin_over_three_planes_and_sums_to_one() {
        // Shares from plane spins sum to one exactly, no plane taking the rest.
        let t = tilt_from(&[ratio(1, 7), ratio(2, 7), ratio(4, 7)]);
        assert_eq!(
            t.iter().fold(ratio(0, 1), |s, x| s + x.clone()),
            ratio(1, 1)
        );
        // Each share stands within a grain of its exact share.
        for (tk, e) in t.iter().zip([ratio(1, 7), ratio(2, 7), ratio(4, 7)]) {
            let gap = if *tk > e {
                tk.clone() - e
            } else {
                e - tk.clone()
            };
            assert!(gap < crate::ring::grain_step(), "within a grain");
        }
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
            spins: Mote::spins_of(&ratio(1, 2), &[ratio(1, 2), ratio(1, 3), ratio(1, 6)]),
            ell_orth: ratio(0, 1),
            phase: rest(ratio(1, 2)),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
        };
        x.phase[0] = ratio(1, 10);
        x.phase[2] = ratio(7, 10);
        let from = space_of(&x.phase);
        let _ = spin_torque(
            &mut x,
            &from,
            &ratio(2, 3),
            &ratio(1, 4),
            &ratio(0, 1),
            &ratio(1, 1),
            &ratio(0, 1),
        );
        assert_eq!(
            x.tilt().iter().fold(ratio(0, 1), |s, v| s + v.clone()),
            ratio(1, 1)
        );
        assert!(
            x.tilt()[0] != ratio(1, 2) || x.tilt()[1] != ratio(1, 3),
            "the planes torqued apart by their own responses"
        );
    }

    #[test]
    fn the_mobius_jetstream_twice_is_the_identity() {
        use super::{mobius, Mote};
        let mut x = Mote {
            key: crate::chem::Kind::parse("W0041").expect("a kind"),
            amount: ratio(1, 1),
            spins: Mote::spins_of(&ratio(1, 3), &[]),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(3, 10), ratio(1, 7)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
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
    fn a_hot_star_holds_its_hydrogen_against_the_wind() {
        // One reading per quantity: at the heart's heat hydrogen is stripped
        // (no open frames for the wind to take); cold it is wholly open; a
        // heavy kind keeps its deep windings' open frames at that heat.
        let k = |w: u32| crate::chem::Kind::atom(w).expect("a kind");
        assert_eq!(super::open(&k(1)), ratio(1, 1), "cold hydrogen");
        assert_eq!(super::open_hot(&k(1), &ratio(1, 1)), ratio(0, 1), "stripped");
        assert_eq!(super::open_hot(&k(1), &ratio(0, 1)), super::open(&k(1)), "heat 0 is the atom");
    }

    #[test]
    fn the_magnet_on_reach_pushes_aligned_poles_out_and_rests_where_it_balances() {
        // docs/121 §14: the dipole contrast falls faster than a monopole, and
        // the boost leaves the lift at the magnet's share where the body
        // stands (u' = u ⊖ (u ⊕ m) = −m).
        use super::{boosted, magnet_share};
        let (r, d, m) = (ratio(3, 1), ratio(1, 10), ratio(1, 2));
        let kappa = ratio(1, 9);
        // Aligned (s < 0): an outward share, the lift raised past the orbit.
        let push = magnet_share(&r, &d, &m, &ratio(-1, 3), &kappa).expect("defined");
        assert!(push < ratio(0, 1));
        assert_eq!(boosted(&ratio(0, 1), &push), Some(-push.clone()), "lifted out by the push");
        // From inside its rest (below −m) it is driven out; from outside drawn in.
        let rest = -push.clone();
        let inside = rest.clone() - ratio(1, 100);
        let outside = rest.clone() + ratio(1, 100);
        assert!(boosted(&inside, &push).expect("in") > inside, "driven out");
        assert!(boosted(&outside, &push).expect("in") < outside, "drawn in");
        assert_eq!(boosted(&rest, &push), Some(rest.clone()), "at its rest it stays");
        // Opposed (s > 0): inward, inside the orbit.
        let pull = magnet_share(&r, &d, &m, &ratio(1, 3), &kappa).expect("defined");
        assert!(pull > ratio(0, 1) && boosted(&ratio(0, 1), &pull).expect("in") < ratio(0, 1));
        // No open frames: back on its orbit.
        let none = magnet_share(&r, &d, &m, &ratio(1, 3), &ratio(0, 1)).expect("defined");
        assert_eq!(boosted(&ratio(1, 5), &none), Some(ratio(0, 1)), "o = 0: on its orbit");
        // The dipole falls faster than the monopole share it is measured against.
        let far = ratio(30, 1);
        let mono = |r: &Ratio| m.clone() / (r.clone() + m.clone());
        let dip = |r: &Ratio| magnet_share(r, &d, &m, &ratio(1, 1), &ratio(1, 1)).expect("defined");
        assert!(dip(&far) / dip(&r) < mono(&far) / mono(&r), "the dipole falls faster");
        // The boost stays inside the horizon for the strongest share.
        for u in [ratio(-9, 10), ratio(0, 1), ratio(9, 10)] {
            for s in [ratio(-99, 100), ratio(99, 100)] {
                let v = boosted(&u, &s).expect("in");
                assert!(v > ratio(-1, 1) && v < ratio(1, 1));
            }
        }
    }

    #[test]
    fn a_motes_reach_is_its_spin_balance_lifted_on_the_s_lane() {
        // docs/121 §14.6: on its orbit (lift 0) a mote reads exactly the spin
        // balance; lifted by u it stands at R_ℓ·D(u), its hold the hold within
        // that reach, its spin untouched.
        use super::{Enclosed, Mote};
        let enc = Enclosed::new(ratio(1, 1), vec![(ratio(1, 2), ratio(1, 4)), (ratio(3, 1), ratio(1, 2))]);
        let mut x = Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            spins: Mote::spins_of(&ratio(2, 1), &[]),
            ell_orth: ratio(0, 1),
            phase: super::rest(ratio(0, 1)),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
        };
        assert_eq!(x.reach_in(&enc), enc.reach(&ratio(2, 1)), "on its orbit");
        assert_eq!(x.hold_in(&enc), enc.hold(&ratio(2, 1)));
        let spin = x.ell_o();
        x.lift = ratio(1, 3);
        let (r, delta) = x.reach_in(&enc);
        assert_eq!(r, enc.reach(&ratio(2, 1)).0 * ratio(2, 1), "D(1/3) = 2");
        let h = enc.hold_within(&r);
        assert_eq!(delta, h.clone() / (r + h));
        assert_eq!(x.ell_o(), spin, "spin untouched");
    }

    #[test]
    fn torsion_is_the_lead_kept_it_relaxes_by_the_attractor_and_turns_with_the_leg() {
        use super::{mobius, settle_disc, torsion_signed, Enclosed, Mote};
        use crate::torsion::tension as torsion;
        let flat = vec![ratio(1, 1), ratio(0, 1), ratio(0, 1)];
        // Settled in its enclosure's plane: none; wound wholly into another,
        // the luminal bound on that pair: 1. Half wound: its leads (−½, ½, 0),
        // the tightest pair (1,2) at `−½ ⊖ ½ = −4/5`, read 4/5, never the L1
        // sum's ½.
        assert_eq!(torsion(&flat, &flat), ratio(0, 1));
        assert_eq!(
            torsion(&[ratio(0, 1), ratio(1, 1), ratio(0, 1)], &flat),
            ratio(1, 1)
        );
        assert_eq!(
            torsion(&[ratio(1, 2), ratio(1, 2), ratio(0, 1)], &flat),
            ratio(4, 5)
        );
        // The Möbius pass reverses its sense.
        let mut x = Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            spins: Mote::spins_of(&ratio(3, 1), &[ratio(1, 2), ratio(1, 2), ratio(0, 1)]),
            ell_orth: ratio(0, 1),
            phase: super::rest(ratio(1, 10)),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
        };
        let before = torsion_signed(&x, &flat);
        mobius(&mut x);
        assert_eq!(torsion_signed(&x, &flat), -before);
        mobius(&mut x);
        // Relaxed by the attractor law: a wound mote among settled ones keeps
        // the share `turned/(d + turned)` of its lead, exactly.
        let heavy = Mote {
            amount: ratio(1000, 1),
            spins: Mote::spins_of(&x.ell_o(), &flat),
            ..x.clone()
        };
        let mut motes = vec![heavy, x.clone()];
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        let plane = settle_disc(&mut motes, &enc);
        assert!(
            torsion(&motes[1].tilt(), &plane) < torsion(&x.tilt(), &plane),
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
            spins: Mote::spins_of(&ratio(1, 1), &tilt),
            ell_orth: ratio(0, 1),
            phase,
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
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
            spins: Mote::spins_of(&ratio(1, 1), &[]),
            ell_orth: ratio(0, 1),
            phase: super::rest(ratio(loop_at.0, loop_at.1)),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
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
                spins: Mote::spins_of(&ratio(3, 2), &[ratio(1, 1), ratio(0, 1), ratio(0, 1)]),
                ell_orth: ratio(0, 1),
                phase,
                agitation: ratio(0, 1),
                born: 0,
                vacuum: false,
                lever: Vec::new(),
                lever_place: ratio(0, 1),
                lift: ratio(0, 1),
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
            spins: Mote::spins_of(&ratio(ell.0, ell.1), &[]),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(1, 10), ratio(1, 7)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
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
            spins: Mote::spins_of(&ratio(ell.0, ell.1), &[]),
            ell_orth: ratio(0, 1),
            phase: vec![ratio(1, 10), ratio(1, 7)],
            agitation: ratio(0, 1),
            born: 0,
            vacuum,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
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
            let d = x.reach_in(&enc).1;
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

#[cfg(test)]
mod stream_tests {
    use super::stream_delta;
    use cfe::ratio::ratio;

    #[test]
    fn a_vacuum_mote_reads_the_continuums_exchanged_share() {
        // Register B16: one face law on both streams, never one stream's
        // face for every mote.
        let d = ratio(3, 11);
        assert_eq!(stream_delta(&d, false), d);
        assert_eq!(stream_delta(&d, true), ratio(1, 1) - d.clone());
        assert_eq!(
            stream_delta(&d, true) + stream_delta(&d, false),
            ratio(1, 1)
        );
    }
}

#[cfg(test)]
mod one_reach_tests {
    use super::Enclosed;
    use cfe::ratio::ratio;

    #[test]
    fn a_spin_set_at_a_reach_reads_back_that_reach() {
        // Register B11: the spin a body is given at reach r reads back at r by
        // the same enclosure, wherever a spin reads r exactly.
        let enc = Enclosed::new(
            ratio(1, 1),
            vec![
                (ratio(1, 2), ratio(1, 4)),
                (ratio(3, 1), ratio(1, 2)),
                (ratio(7, 1), ratio(1, 3)),
            ],
        );
        for r in [ratio(1, 3), ratio(2, 1), ratio(9, 2), ratio(5, 1)] {
            let ell = enc.spin_at(&r);
            let (back, _) = enc.reach(&ell);
            assert_eq!(back, r, "reach {r} read back");
        }
        // Where two spins read one reach (the steps overlap), the least.
        let shared = ratio(12, 5);
        assert_eq!(enc.spin_at(&shared), ratio(3, 1));
        assert_eq!(enc.reach(&ratio(3, 1)).0, shared);
    }
}

#[cfg(test)]
mod hold_by_place_tests {
    use super::Enclosed;
    use cfe::ratio::{ratio, Ratio};

    #[test]
    fn the_hold_by_place_bisected_is_the_hold_counted_body_by_body() {
        // Register A34: the same sum as testing every body's own reach.
        let bodies: Vec<(Ratio, Ratio)> = (1..40i64)
            .map(|i| (ratio((i * 37) % 53 + 1, 7), ratio(i % 5 + 1, 97)))
            .collect();
        let enc = Enclosed::new(ratio(1, 3), bodies.clone());
        let mut sorted = bodies.clone();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        for r in [
            ratio(0, 1),
            ratio(1, 2),
            ratio(3, 1),
            ratio(7, 2),
            ratio(20, 1),
        ] {
            let mut by_hand = enc.hold(&ratio(0, 1));
            let mut below = enc.hold(&ratio(0, 1));
            for (l, h) in &sorted {
                if l.clone() / below.clone() < r {
                    by_hand += crate::ring::grain(h);
                }
                below += crate::ring::grain(h);
            }
            assert_eq!(enc.hold_within(&r), by_hand, "at {r}");
        }
    }
}

#[cfg(test)]
mod extent_degree_tests {
    use super::{enclosures_meet, own_degrees, own_enclosure, Enclosed, Mote};
    use cfe::ratio::ratio;

    #[test]
    fn an_extent_reads_45_degrees_of_its_pinch_share_a_side() {
        // Register K3: an arc d of the L1 circle of radius R is d/(8R) of a
        // turn; d = R*e, so 45 degrees times e on each side.
        let x = Mote {
            key: crate::chem::Kind::parse("W0001").expect("a kind"),
            amount: ratio(1, 1),
            spins: Mote::spins_of(&ratio(1, 1), &[]),
            ell_orth: ratio(1, 2),
            phase: super::rest(ratio(1, 2)),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
        };
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        let e = own_enclosure(&x, &enc, &ratio(2, 1));
        let deg = own_degrees(&x, &enc, &ratio(2, 1));
        // Holding 1/2 against the hold 1: e = 1/3, so 15 degrees a side.
        assert_eq!(e[0], ratio(1, 3));
        assert_eq!(deg[0], ratio(15, 1));
        assert!(enclosures_meet(&ratio(30, 1), &deg[0], &deg[0]));
        assert!(!enclosures_meet(&ratio(31, 1), &deg[0], &deg[0]));
    }
}

#[cfg(test)]
mod coupling_tests {
    use super::{meeting, own_enclosure, shear_booked, weight, Enclosed, Mote, SPACE};
    use cfe::ratio::{ratio, Ratio};
    use num_traits::{Signed, Zero};

    fn mote(kind: &str, amount: Ratio, ell: Ratio, tilt: &[Ratio], phase: Vec<Ratio>) -> Mote {
        Mote {
            key: crate::chem::Kind::parse(kind).expect("a kind"),
            amount,
            spins: Mote::spins_of(&ell, tilt),
            ell_orth: ratio(0, 1),
            phase,
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
        }
    }

    /// `Σ m·ℓ_k` per plane, in holding.
    fn books(ms: &[Mote]) -> Vec<Ratio> {
        (0..SPACE.len())
            .map(|k| {
                ms.iter().fold(ratio(0, 1), |s, x| {
                    s + x.amount.clone() * weight(&x.key) * x.spins[k].clone()
                })
            })
            .collect()
    }

    fn rate(x: &Mote, enc: &Enclosed, k: usize) -> Ratio {
        let ell = x.ell_o();
        let d = super::stream_delta(&enc.reach(&ell).1, x.vacuum);
        ratio(2, 1) * d * x.spins[k].clone() / ell
    }

    #[test]
    fn the_shear_keeps_each_planes_spin_exactly() {
        // Three motes in contact, of both poles, phases across hemispheres:
        // per plane, Σ m·ℓ_k after, less the grain's remainder, is Σ m·ℓ_k
        // before, exactly.
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        let mut motes = vec![
            mote(
                "W0007",
                ratio(1, 1),
                ratio(1, 2),
                &[ratio(1, 2), ratio(1, 3), ratio(1, 6)],
                vec![ratio(0, 1), ratio(0, 1), ratio(1, 3), ratio(1, 8)],
            ),
            mote(
                "W0001",
                ratio(3, 1),
                ratio(3, 5),
                &[],
                vec![ratio(1, 2), ratio(0, 1), ratio(1, 3), ratio(5, 8)],
            ),
            mote(
                "W0007",
                ratio(1, 2),
                ratio(2, 3),
                &[ratio(1, 6), ratio(1, 3), ratio(1, 2)],
                vec![ratio(1, 16), ratio(0, 1), ratio(0, 1), ratio(0, 1)],
            ),
        ];
        let total = ratio(14, 1);
        let before = books(&motes);
        let (moved, left) = shear_booked(&mut motes, &enc, &total, true);
        assert!(moved > ratio(0, 1));
        let after = books(&motes);
        for k in 0..SPACE.len() {
            assert!(motes.iter().all(|x| x.spins[k] >= ratio(0, 1)));
            assert_eq!(
                after[k].clone() / total.clone() - left[k].clone(),
                before[k].clone() / total.clone(),
                "plane {k}"
            );
        }
    }

    #[test]
    fn opposed_dipoles_lock_and_aligned_ones_split() {
        // Two lithium motes (closed frames outweigh open: pole +1) in
        // contact on reach. Phases half a turn apart on every plane: opposed,
        // their rates draw together. Phases alike: aligned, apart.
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        let total = ratio(14, 1);
        for (apart, locks) in [(ratio(1, 2), true), (ratio(0, 1), false)] {
            let mut motes = vec![
                mote(
                    "W0007",
                    ratio(1, 1),
                    ratio(1, 2),
                    &[],
                    super::rest(ratio(0, 1)),
                ),
                mote(
                    "W0007",
                    ratio(1, 1),
                    ratio(3, 5),
                    &[],
                    vec![apart.clone(), ratio(0, 1), apart.clone(), apart.clone()],
                ),
            ];
            let gap: Vec<Ratio> = (0..SPACE.len())
                .map(|k| rate(&motes[0], &enc, k) - rate(&motes[1], &enc, k))
                .collect();
            let start: Vec<Vec<Ratio>> = motes.iter().map(|x| x.spins.clone()).collect();
            shear_booked(&mut motes, &enc, &total, true);
            for k in 0..SPACE.len() {
                // At its own reach each mote's plane spin follows its rate;
                // the inner (faster) mote's moves toward the outer's where
                // they lock, away where they split.
                let inner = motes[0].spins[k].clone() - start[0][k].clone();
                assert!(gap[k] > ratio(0, 1));
                assert_eq!(inner < ratio(0, 1), locks, "plane {k}, locks {locks}");
            }
        }
    }

    #[test]
    fn a_mote_whose_enclosure_meets_none_is_untouched() {
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        let total = ratio(14, 1);
        let far = mote(
            "W0007",
            ratio(1, 1),
            ratio(50, 1),
            &[],
            vec![ratio(1, 2), ratio(0, 1), ratio(1, 2), ratio(1, 2)],
        );
        let mut motes = vec![
            mote(
                "W0007",
                ratio(1, 1),
                ratio(1, 2),
                &[],
                super::rest(ratio(0, 1)),
            ),
            mote(
                "W0007",
                ratio(1, 1),
                ratio(3, 5),
                &[],
                vec![ratio(1, 2), ratio(0, 1), ratio(1, 2), ratio(1, 2)],
            ),
            far.clone(),
        ];
        let moved = shear_booked(&mut motes, &enc, &total, true).0;
        assert!(moved > ratio(0, 1), "the pair in contact couples");
        assert_eq!(motes[2], far);
        // Alone, nothing moves at all.
        let mut alone = vec![far.clone()];
        assert!(shear_booked(&mut alone, &enc, &total, true).0.is_zero());
        assert_eq!(alone[0], far);
    }

    #[test]
    fn a_component_past_zero_flips_its_own_planes_phase_only() {
        // Two hydrogen motes (3Z frames: two held, one open, pole +1/3 by
        // degrees), aligned: they repel. The outer turns hardly at all on the
        // first plane, its rate there far under the inner's, so the push
        // carries that component past zero: through the centre on that plane
        // alone.
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        let total = ratio(2, 1);
        let mut motes = vec![
            mote(
                "W0001",
                ratio(1, 1),
                ratio(1, 2),
                &[],
                super::rest(ratio(0, 1)),
            ),
            mote(
                "W0001",
                ratio(1, 1),
                ratio(3, 5),
                &[ratio(1, 100000), ratio(99999, 200000), ratio(99999, 200000)],
                super::rest(ratio(0, 1)),
            ),
        ];
        shear_booked(&mut motes, &enc, &total, true);
        let b = &motes[1];
        assert_eq!(b.phase[SPACE[0]], ratio(1, 2), "the first plane through");
        assert!(b.spins[0] > ratio(0, 1), "its magnitude kept");
        assert_eq!(b.phase[SPACE[1]], ratio(0, 1));
        assert_eq!(b.phase[SPACE[2]], ratio(0, 1));
        assert_eq!(b.phase[super::LOOP], ratio(0, 1));
        assert!(motes[0].phase.iter().all(|p| p.is_zero()));
    }

    #[test]
    fn the_range_by_bisection_is_every_pair_tested() {
        // 48 motes of mixed kinds, amounts and spins in one enclosure with
        // bodies inside: the ranges read by bisection are those found by
        // testing every pair.
        let kinds = ["W0001", "W0004", "W0007", "W0012", "W0016"];
        let mut motes = Vec::new();
        for i in 0..48i64 {
            motes.push(mote(
                kinds[(i % 5) as usize],
                ratio(1 + (i * 7) % 11, 3),
                ratio(1 + (i * 13) % 37, 9),
                &[],
                super::rest(ratio(0, 1)),
            ));
        }
        let total = super::holding(&motes);
        let bodies: Vec<(Ratio, Ratio)> = motes
            .iter()
            .map(|x| (x.ell_o(), x.amount.clone() * weight(&x.key) / total.clone()))
            .collect();
        let enc = Enclosed::new(ratio(1, 3), bodies);
        let reach: Vec<Ratio> = motes.iter().map(|x| x.reach_in(&enc).0).collect();
        let extent: Vec<Ratio> = motes
            .iter()
            .zip(&reach)
            .map(|(x, r)| r.clone() * own_enclosure(x, &enc, &total)[0].clone())
            .collect();
        let by_bisection = meeting(&reach, &extent);
        let mut met = 0;
        for i in 0..motes.len() {
            let every: Vec<usize> = (0..motes.len())
                .filter(|&j| {
                    j != i
                        && (reach[i].clone() - reach[j].clone()).abs()
                            <= extent[i].clone() + extent[j].clone()
                })
                .collect();
            met += every.len();
            assert_eq!(by_bisection[i], every, "mote {i}");
        }
        assert!(met > 0 && met < motes.len() * (motes.len() - 1));
    }
}
