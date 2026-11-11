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

#[cfg(test)]
mod tests {
    use super::*;

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
