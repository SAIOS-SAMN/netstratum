//! The pass per plane (docs/128; operator 2026-11-11: "derive the physics as
//! velocity / pressure resistance / phase state / parcel boundaries"; "add
//! multi-dimensionality"). Readings only: no law reads this module yet
//! (docs/128 §6).
//!
//! - **Velocity:** each plane on its own circle, its spin read by the ring
//!   law at the enclosure's hold at that spin, `R_k = ℓ_k/H(<ℓ_k)`, `δ_k =
//!   H/(R_k + H)`, never a tilt's share of one summed spin.
//! - **The parcel's boundary, the face:** the share of the parcel's own
//!   extent `R_k(1 ± e_k)` standing inside the ring (`R = H`), `λ_k`, first
//!   degree: `0` the continuum face, `1` the vacuum face, one per plane.
//! - **Phase state, the transit:** the plane's share on its face `s_k = δ_k +
//!   λ_k(1 − 2δ_k)` and its sense `σ_k = 1 − 2λ_k` (docs/94 §2); the luminal
//!   bound `s_k ≤ ½` holds by construction, and a whole crossing advances the
//!   phase by `½`.
//! - **Pressure resistance:** the twist's pressure on the plane resists it on
//!   the continuum side of its crossing and aids it past the ring (the helix
//!   reversed): `s_k ⊖ P_k`, or `s_k ⊕ P_k`.
use crate::flows::{Enclosed, Mote};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// One plane's reading of a parcel: its own reach and deficit, its extent's
/// share, its face, its share on that face and its sense.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaneFace {
    /// The plane's own reach, `ℓ_k/H(<ℓ_k)`.
    pub reach: Ratio,
    /// The hold at the plane's spin, `H(<ℓ_k)`: the ring's reach on it.
    pub hold: Ratio,
    /// The plane's own deficit, `H/(R_k + H)`.
    pub delta: Ratio,
    /// The parcel's extent share on the plane, its pinch share.
    pub extent: Ratio,
    /// The face: the share of the parcel inside the ring, `λ_k`.
    pub face: Ratio,
    /// Its share on its face, `δ_k + λ_k(1 − 2δ_k)`.
    pub share: Ratio,
    /// Its sense, `1 − 2λ_k`.
    pub sense: Ratio,
}

/// The face: the share of an extent `r(1 ± e)` standing inside the ring at
/// reach `h`, on the extent's linear measure, in `[0, 1]`. With no extent,
/// `1` inside the ring, `0` outside and `½` on it.
pub fn face_share(r: &Ratio, h: &Ratio, e: &Ratio) -> Ratio {
    let width = ratio(2, 1) * r.clone() * e.clone();
    if width.is_zero() {
        return match r.cmp(h) {
            std::cmp::Ordering::Less => ratio(1, 1),
            std::cmp::Ordering::Greater => ratio(0, 1),
            std::cmp::Ordering::Equal => ratio(1, 2),
        };
    }
    let inner = r.clone() * (ratio(1, 1) - e.clone());
    let inside = (h.clone() - inner) / width;
    inside.max(ratio(0, 1)).min(ratio(1, 1))
}

/// The transit's share on a face `λ` at deficit `δ`: `δ + λ(1 − 2δ)`.
pub fn transit_share(delta: &Ratio, face: &Ratio) -> Ratio {
    delta.clone() + face.clone() * (ratio(1, 1) - ratio(2, 1) * delta.clone())
}

/// The sense on a face `λ`: `1 − 2λ`, flipping at the ring.
pub fn sense(face: &Ratio) -> Ratio {
    ratio(1, 1) - ratio(2, 1) * face.clone()
}

/// A plane's share resisted by the medium's pressure `p` on it: on the
/// continuum side of its crossing (`λ < ½`) the pressure resists, `s ⊖ p`;
/// past the ring the helix is reversed and it aids, `s ⊕ p`; on the ring
/// neither. None outside the flow law's domain.
pub fn resisted(share: &Ratio, p: &Ratio, face: &Ratio) -> Option<Ratio> {
    match face.cmp(&ratio(1, 2)) {
        std::cmp::Ordering::Less => crate::ring::compose(share, &-p.clone()),
        std::cmp::Ordering::Greater => crate::ring::compose(share, p),
        std::cmp::Ordering::Equal => Some(share.clone()),
    }
}

/// One plane read at spin `ell` in `enc` for a parcel of holding share `m`.
pub fn plane_face(enc: &Enclosed, ell: &Ratio, m: &Ratio) -> PlaneFace {
    let hold = enc.hold(ell);
    let reach = if hold.is_zero() {
        ratio(0, 1)
    } else {
        ell.clone() / hold.clone()
    };
    let delta = if (reach.clone() + hold.clone()).is_zero() {
        ratio(0, 1)
    } else {
        hold.clone() / (reach.clone() + hold.clone())
    };
    let extent = crate::ring::pinch_share(m, &hold);
    let face = face_share(&reach, &hold, &extent);
    PlaneFace {
        share: transit_share(&delta, &face),
        sense: sense(&face),
        reach,
        hold,
        delta,
        extent,
        face,
    }
}

/// A mote's spatial planes, each on its own circle: its spin on each plane
/// read in `enc`, its holding share in the enclosure's unit `whole`.
pub fn plane_faces(x: &Mote, enc: &Enclosed, whole: &Ratio) -> Vec<PlaneFace> {
    let m = if whole.is_zero() {
        ratio(0, 1)
    } else {
        x.amount.clone() * crate::flows::weight(&x.key) / whole.clone()
    };
    x.spins.iter().map(|l| plane_face(enc, l, &m)).collect()
}

/// A plane's FORM (docs/128 §8): the dimension attached, without collapse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaneForm {
    /// Its turning pair in the generator: the plane's component and its R
    /// partner's (the dual pair of S³), exact.
    pub pair: (Ratio, Ratio),
    /// The pair as isoclinic degrees on its readings, in turns: `α = (ψ_k +
    /// ψ_kR)/2`, `β = (ψ_k − ψ_kR)/2`.
    pub isoclinic: (Ratio, Ratio),
    /// Its poles on its own axis: the neutrons' held dimensions and the
    /// protons' open dimensions, a third of the nuclide's each (an atom names
    /// no axis).
    pub poles: (Ratio, Ratio),
    /// Its radial dimension: its own reach, extent and face (§1–§2).
    pub reach: PlaneFace,
}

/// A mote's three plane forms, each read from its orientation, its kind and
/// its plane's own circle in `enc` (`whole` the enclosure's holding unit).
pub fn plane_forms(x: &Mote, enc: &Enclosed, whole: &Ratio) -> Vec<PlaneForm> {
    use crate::orientation::{reading, PARTNER, SPATIAL};
    let dims = crate::chem::dimensions(&x.key);
    let third = |v: u32| ratio(i64::from(v), 3);
    let faces = plane_faces(x, enc, whole);
    let g = x.orientation.g();
    (0..SPATIAL.len())
        .map(|j| {
            let (a, b) = (g[SPATIAL[j]].clone(), g[PARTNER[j]].clone());
            let (ra, rb) = (reading(&a), reading(&b));
            PlaneForm {
                isoclinic: (
                    (ra.clone() + rb.clone()) / ratio(2, 1),
                    (ra - rb) / ratio(2, 1),
                ),
                pair: (a, b),
                poles: (third(dims.held), third(dims.open)),
                reach: faces
                    .get(j)
                    .cloned()
                    .unwrap_or_else(|| plane_face(enc, &ratio(0, 1), &ratio(0, 1))),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_planes_form_is_its_pair_its_poles_and_its_reach() {
        // docs/128 §8: the pair from G, the isoclinic degrees from its
        // readings, a third of the nuclide's dimensions on each axis, the
        // reach from its own circle; the three axes' poles make the whole.
        let mut x = crate::flows::Mote {
            key: crate::chem::Kind::atom(4).expect("a kind"),
            amount: ratio(2, 1),
            spins: crate::flows::Mote::spins_of(
                &ratio(3, 2),
                &[ratio(1, 2), ratio(1, 3), ratio(1, 6)],
            ),
            ell_orth: ratio(0, 1),
            orientation: crate::orientation::Orientation::from_phases(&[
                ratio(1, 4),
                ratio(1, 8),
                ratio(1, 2),
                ratio(3, 4),
            ]),
            agitation: ratio(0, 1),
            born: 0,
            vacuum: false,
            lever: Vec::new(),
            lever_place: ratio(0, 1),
            lift: ratio(0, 1),
        };
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        let forms = plane_forms(&x, &enc, &ratio(1, 1));
        assert_eq!(forms.len(), 3);
        let dims = crate::chem::dimensions(&x.key);
        let held = forms.iter().fold(ratio(0, 1), |s, f| s + f.poles.0.clone());
        let open = forms.iter().fold(ratio(0, 1), |s, f| s + f.poles.1.clone());
        assert_eq!(
            held + open,
            ratio(i64::from(dims.whole()), 1),
            "the axes' poles make 3w"
        );
        for (j, f) in forms.iter().enumerate() {
            let (a, b) = (
                crate::orientation::reading(&f.pair.0),
                crate::orientation::reading(&f.pair.1),
            );
            assert_eq!(
                f.isoclinic.0.clone() + f.isoclinic.1.clone(),
                a,
                "α + β is the plane"
            );
            assert_eq!(
                f.isoclinic.0.clone() - f.isoclinic.1.clone(),
                b,
                "α − β its partner"
            );
            assert_eq!(
                f.reach,
                plane_face(
                    &enc,
                    &x.spins[j],
                    &(x.amount.clone() * crate::flows::weight(&x.key))
                )
            );
        }
        x.orientation.hold_phase(0, &ratio(1, 3));
        assert_ne!(
            plane_forms(&x, &enc, &ratio(1, 1))[0].pair,
            forms[0].pair,
            "the pair is the state"
        );
    }

    #[test]
    fn planes_with_their_own_spins_turn_at_their_own_rates() {
        // docs/128 §1: each plane on its own circle. Unequal spins read
        // unequal shares, where the tilt's `2δ·t_k` read one deficit for all.
        let enc = Enclosed::new(ratio(1, 1), vec![(ratio(1, 2), ratio(1, 4))]);
        let m = ratio(1, 100);
        let (a, b) = (
            plane_face(&enc, &ratio(1, 3), &m),
            plane_face(&enc, &ratio(3, 1), &m),
        );
        assert_ne!(a.share, b.share, "the planes part by their own velocities");
        // And their R partners, turned with their own planes, part too.
        let mut o = crate::orientation::Orientation::default();
        let (sa, sb) = (ratio(2, 1) * a.share.clone(), ratio(2, 1) * b.share.clone());
        o.bell(
            &[sa.clone(), sb.clone(), ratio(0, 1)],
            &[sa, sb, ratio(0, 1)],
        )
        .expect("a bell");
        let g = o.g();
        assert_ne!(
            g[crate::orientation::PARTNER[0]],
            g[crate::orientation::PARTNER[1]]
        );
    }

    #[test]
    fn wholly_outside_or_inside_reads_the_two_faces() {
        // docs/128 §7: outside the ring, the continuum, λ 0, s = δ, σ 1;
        // inside, the vacuum, λ 1, s = 1 − δ, σ −1.
        let (h, e) = (ratio(1, 1), ratio(1, 10));
        let out = face_share(&ratio(3, 1), &h, &e);
        assert_eq!(out, ratio(0, 1));
        let inn = face_share(&ratio(1, 3), &h, &e);
        assert_eq!(inn, ratio(1, 1));
        let d = ratio(1, 4);
        assert_eq!(transit_share(&d, &out), d.clone());
        assert_eq!(transit_share(&d, &inn), ratio(3, 4));
        assert_eq!(sense(&out), ratio(1, 1));
        assert_eq!(sense(&inn), ratio(-1, 1));
    }

    #[test]
    fn the_luminal_bound_holds_by_construction_on_every_plane() {
        // A plane's share on its face never passes ½, whatever its reach and
        // its extent: no flip event is needed.
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        for ell in [
            ratio(1, 9),
            ratio(1, 2),
            ratio(1, 1),
            ratio(2, 1),
            ratio(7, 1),
        ] {
            for m in [ratio(0, 1), ratio(1, 50), ratio(1, 3), ratio(2, 1)] {
                let f = plane_face(&enc, &ell, &m);
                assert!(f.share <= ratio(1, 2), "ℓ {ell}, m {m}: s {} > ½", f.share);
                assert!(f.face >= ratio(0, 1) && f.face <= ratio(1, 1));
            }
        }
    }

    #[test]
    fn on_the_ring_the_plane_stands_at_the_fixed_point() {
        // R = H: λ = ½, s = ½, σ = 0, whatever its extent.
        let enc = Enclosed::new(ratio(1, 1), Vec::new());
        for m in [ratio(0, 1), ratio(1, 7)] {
            let f = plane_face(&enc, &ratio(1, 1), &m);
            assert_eq!(f.reach, f.hold);
            assert_eq!(f.face, ratio(1, 2));
            assert_eq!(f.share, ratio(1, 2));
            assert_eq!(f.sense, ratio(0, 1));
        }
    }

    #[test]
    fn a_parcel_straddling_the_ring_crosses_by_its_share() {
        // R 1, H 1, e ½: the extent [½, 3/2] is half inside; at R 3/4 more of
        // it is: λ = (1 − 3/8)/(3/4) = 5/6.
        assert_eq!(
            face_share(&ratio(1, 1), &ratio(1, 1), &ratio(1, 2)),
            ratio(1, 2)
        );
        assert_eq!(
            face_share(&ratio(3, 4), &ratio(1, 1), &ratio(1, 2)),
            ratio(5, 6)
        );
    }

    #[test]
    fn the_pressure_resists_before_the_ring_and_aids_past_it() {
        // (s ⊖ P) ⊕ P = s: no pressure leaves the share bare.
        let (s, p) = (ratio(1, 3), ratio(1, 5));
        let before = resisted(&s, &p, &ratio(0, 1)).expect("in domain");
        assert!(before < s, "resisted on the continuum side");
        let past = resisted(&s, &p, &ratio(1, 1)).expect("in domain");
        assert!(past > s, "aided past the ring");
        assert_eq!(crate::ring::compose(&before, &p), Some(s.clone()));
        assert_eq!(resisted(&s, &ratio(0, 1), &ratio(0, 1)), Some(s.clone()));
        assert_eq!(resisted(&s, &p, &ratio(1, 2)), Some(s));
    }
}
