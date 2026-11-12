//! Heat by component, read only (issue #143, docs/124 §3): the dispersion
//! of a body's parts' motion about the body's own motion in step, kept by
//! component and never folded into one number. Nothing here writes the
//! universe.
//!
//! A part's motion, as the motes carry it now, is read on these components
//! of `H`'s generator (docs/117 §10.3, docs/120 §8b):
//! - **each plane's phase** (the four planes, R's loop among them): its
//!   distance on the circle from the parts' circle mean (`flows::circle_mean`),
//!   in turns, at most ½;
//! - **each spatial plane's spin**: its distance from the parts' mean spin
//!   on that plane;
//! - **the turning between two spatial planes**: the pair's spin
//!   difference (`ℓ_j − ℓ_k`, the first-degree reading `torsion` builds its
//!   pair rates from), its distance from the parts' mean difference;
//! - **the orthogonal orbit's spin**: its distance from the mean.
//!
//! Each component's dispersion is first degree and holding-weighted per
//! unit of holding, `Σ w·|x − x̄| / Σ w`, signed motion kept by its
//! component and never squared (design law; operator 2026-11-05: no
//! squares). By kind, each kind's phase distance per plane from the body's
//! mean (not its own) shows which kinds stand out of step. The boosts and
//! the volume part `B₁∧B₂` (docs/117 §11.3) are not carried by the motes
//! yet, so they are not read here (#143).
use crate::flows::{circle_mean, weight, Mote, PLANES, SPACE};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;
use std::collections::BTreeMap;

/// One body's parts' dispersion, by component, per unit of holding.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Dispersion {
    /// The holding read.
    pub holding: Ratio,
    /// Per plane (`PLANES`), the phase's distance on the circle from the
    /// mean, in turns.
    pub phase: Vec<Ratio>,
    /// Per spatial plane (`SPACE` order), the spin's distance from the mean.
    pub spin: Vec<Ratio>,
    /// Per pair of spatial planes `(0,1), (1,2), (2,0)`, the spin
    /// difference's distance from the mean difference.
    pub between: Vec<Ratio>,
    /// The orthogonal orbit's spin, its distance from the mean.
    pub orth: Ratio,
    /// By kind: each plane's phase distance from the body's circle mean.
    pub phase_by_kind: BTreeMap<crate::chem::Kind, Vec<Ratio>>,
}

fn frac(x: &Ratio) -> Ratio {
    x.clone() - x.floor()
}

/// The distance of `x` from `mean` on the circle, in `[0, ½]` turns.
fn on_circle(x: &Ratio, mean: &Ratio) -> Ratio {
    let d = frac(&(x.clone() - mean.clone()));
    if d > ratio(1, 2) {
        ratio(1, 1) - d
    } else {
        d
    }
}

fn abs(x: Ratio) -> Ratio {
    if x < ratio(0, 1) {
        -x
    } else {
        x
    }
}

/// `Σ w·|x − x̄| / Σ w` over `(w, x)`, `x̄` the holding-weighted mean; zero
/// where nothing is held.
fn line(parts: &[(Ratio, Ratio)]) -> Ratio {
    let w = parts.iter().fold(ratio(0, 1), |a, (h, _)| a + h.clone());
    if w.is_zero() {
        return ratio(0, 1);
    }
    let mean = parts
        .iter()
        .fold(ratio(0, 1), |a, (h, x)| a + h.clone() * x.clone())
        / w.clone();
    parts.iter().fold(ratio(0, 1), |a, (h, x)| {
        a + h.clone() * abs(x.clone() - mean.clone())
    }) / w
}

/// Read the dispersion of `parts` by component.
pub fn read(parts: &[Mote]) -> Dispersion {
    let held: Vec<Ratio> = parts
        .iter()
        .map(|x| x.amount.clone() * weight(&x.key))
        .collect();
    let holding = held.iter().fold(ratio(0, 1), |a, h| a + h.clone());
    if holding.is_zero() {
        return Dispersion::default();
    }
    let component = |f: &dyn Fn(&Mote) -> Ratio| -> Vec<(Ratio, Ratio)> {
        parts
            .iter()
            .zip(&held)
            .map(|(x, w)| (w.clone(), f(x)))
            .collect()
    };
    let at = |v: &[Ratio], k: usize| v.get(k).cloned().unwrap_or_else(|| ratio(0, 1));
    let mut phase = Vec::with_capacity(PLANES);
    let mut means = Vec::with_capacity(PLANES);
    for k in 0..PLANES {
        let ph = component(&|x| x.phase_at(k));
        let mean = circle_mean(&ph);
        let d = ph
            .iter()
            .fold(ratio(0, 1), |a, (w, p)| a + w.clone() * on_circle(p, &mean))
            / holding.clone();
        phase.push(d);
        means.push(mean);
    }
    let spin: Vec<Ratio> = (0..SPACE.len())
        .map(|j| line(&component(&|x| at(&x.spins, j))))
        .collect();
    let between: Vec<Ratio> = [(0usize, 1usize), (1, 2), (2, 0)]
        .iter()
        .map(|&(j, k)| line(&component(&|x| at(&x.spins, j) - at(&x.spins, k))))
        .collect();
    let orth = line(&component(&|x| x.ell_orth()));
    let mut sums: BTreeMap<crate::chem::Kind, (Ratio, Vec<Ratio>)> = BTreeMap::new();
    for (x, w) in parts.iter().zip(&held) {
        let e = sums
            .entry(x.key.clone())
            .or_insert_with(|| (ratio(0, 1), vec![ratio(0, 1); PLANES]));
        e.0 += w.clone();
        for (k, mean) in means.iter().enumerate() {
            e.1[k] += w.clone() * on_circle(&x.phase_at(k), mean);
        }
    }
    let phase_by_kind = sums
        .into_iter()
        .filter(|(_, (w, _))| !w.is_zero())
        .map(|(k, (w, v))| (k, v.into_iter().map(|d| d / w.clone()).collect()))
        .collect();
    Dispersion {
        holding,
        phase,
        spin,
        between,
        orth,
        phase_by_kind,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part(key: &str, amount: i64, ell: Ratio, tilt: &[Ratio], phase: [Ratio; 4]) -> Mote {
        Mote {
            key: crate::chem::Kind::parse(key).expect("a kind"),
            amount: ratio(amount, 1),
            spins: Mote::spins_of(&ell, tilt),
            spins_orth: Mote::orth_of(&(ell.clone() / ratio(3, 1)), tilt),
            orientation: crate::orientation::Orientation::from_phases(phase.as_ref()),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
        }
    }

    fn parts() -> Vec<Mote> {
        let t = [ratio(1, 2), ratio(1, 3), ratio(1, 6)];
        vec![
            part(
                "W0001",
                5,
                ratio(2, 1),
                &t,
                [ratio(1, 10), ratio(0, 1), ratio(9, 10), ratio(1, 4)],
            ),
            part(
                "W0004",
                2,
                ratio(3, 1),
                &[ratio(1, 3), ratio(1, 3), ratio(1, 3)],
                [ratio(3, 10), ratio(1, 2), ratio(1, 5), ratio(1, 4)],
            ),
            part(
                "W0016",
                1,
                ratio(1, 2),
                &t,
                [ratio(7, 10), ratio(1, 3), ratio(0, 1), ratio(1, 2)],
            ),
        ]
    }

    #[test]
    fn parts_in_step_on_every_component_have_no_heat() {
        let t = [ratio(1, 2), ratio(1, 3), ratio(1, 6)];
        let ph = [ratio(1, 7), ratio(2, 7), ratio(3, 7), ratio(4, 7)];
        let d = read(&[
            part("W0001", 5, ratio(2, 1), &t, ph.clone()),
            part("W0004", 3, ratio(2, 1), &t, ph),
        ]);
        assert!(d
            .phase
            .iter()
            .chain(&d.spin)
            .chain(&d.between)
            .all(|x| x.is_zero()));
        assert!(d.orth.is_zero());
    }

    #[test]
    fn a_common_motion_leaves_every_components_dispersion() {
        // The same turn on every part's phase, and the same spin on every
        // part's plane, move the body, not its heat.
        let before = read(&parts());
        let mut moved = parts();
        for x in &mut moved {
            for k in 0..PLANES {
                let v = frac(&(x.phase_at(k) + ratio(5, 11)));
                x.orientation.hold_phase(k, &v);
            }
            for s in &mut x.spins {
                *s += ratio(7, 4);
            }
            {
                let v = x.ell_orth() + ratio(-2, 9);
                x.set_ell_orth(v);
            }
        }
        let after = read(&moved);
        // Held phases stand on the grain (`Orientation::hold_phase`), so the
        // common turn leaves each dispersion within it.
        for (a, b) in after.phase.iter().zip(&before.phase) {
            let d = a.clone() - b.clone();
            let g = crate::ring::grain_step() * ratio(4, 1);
            assert!(d.clone() <= g && -d <= g, "{a} against {b}");
        }
        assert_eq!(after.spin, before.spin);
        assert_eq!(after.between, before.between);
        assert_eq!(after.orth, before.orth);
        for (kind, a) in &after.phase_by_kind {
            let b = &before.phase_by_kind[kind];
            for (x, y) in a.iter().zip(b) {
                let d = x.clone() - y.clone();
                let g = crate::ring::grain_step() * ratio(4, 1);
                assert!(d.clone() <= g && -d <= g, "{kind:?}: {x} against {y}");
            }
        }
    }

    #[test]
    fn one_planes_disorder_moves_no_other_component() {
        // Components are kept apart: a part moved off step on one plane's
        // phase changes that plane's dispersion alone.
        let before = read(&parts());
        let mut moved = parts();
        moved[2].orientation.hold_phase(3, &ratio(9, 10));
        let after = read(&moved);
        assert_ne!(after.phase[3], before.phase[3]);
        for k in 0..3 {
            assert_eq!(after.phase[k], before.phase[k]);
        }
        assert_eq!(after.spin, before.spin);
        assert_eq!(after.between, before.between);
        assert_eq!(after.orth, before.orth);
    }
}
