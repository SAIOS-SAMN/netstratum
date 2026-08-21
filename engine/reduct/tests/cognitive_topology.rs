//! COGNITION UNDER DIFFERENT TOPOLOGIES (doc 55 §9, dimensional fluidity).
//!
//! The Ignition Law was established for point-houses gluing into circles (H¹
//! cognition). Here we run civilizations whose classifications glue into DIFFERENT
//! mathematical topologies and read collective cognition as a per-grade SPECTRUM —
//! keeping the dimensional capacity of the coherence FLUID (never collapsed to one
//! scalar or one dimension). A civilization can hold cognition in H¹, H², … at
//! once, and houses of different intrinsic dimension each contribute in their own
//! grade with no zero-padding (the sheaf's heterogeneous-stalk law, one level up).

use num_bigint::BigInt;
use reduct::atlas::{delta_h, delta_h_spectrum};
use reduct::sheaf::CellSheaf;

fn one() -> BigInt {
    BigInt::from(1)
}
/// A 0-dimensional house (a contractible knowledge-point).
fn point_house() -> CellSheaf {
    CellSheaf::new(vec![vec![1]], vec![]).expect("point house")
}
/// A 2-dimensional house: a knowledge-body carrying its own H² (one 0-cell + one
/// 2-cell, no 1-cell — a minimal sphere-shaped mind). Heterogeneous dimension.
fn plane_house() -> CellSheaf {
    CellSheaf::new(vec![vec![1], vec![], vec![1]], vec![]).expect("plane house")
}
/// n disjoint contact points.
fn points(n: usize) -> CellSheaf {
    CellSheaf::new(vec![vec![1; n.max(1)]], vec![]).expect("points")
}
/// A circle overlap: one vertex + one edge (a loop), betti [1,1].
fn circle() -> CellSheaf {
    CellSheaf::new(vec![vec![1], vec![1]], vec![]).expect("circle")
}
/// A circle plus `extra` disjoint contact points: betti [1+extra, 1].
fn circle_plus(extra: usize) -> CellSheaf {
    let mut g0 = vec![1];
    g0.extend(std::iter::repeat(1).take(extra));
    CellSheaf::new(vec![g0, vec![1]], vec![]).expect("circle+points")
}
/// `n` disjoint circles: betti [n, n].
fn n_circles(n: usize) -> CellSheaf {
    let n = n.max(1);
    CellSheaf::new(vec![vec![1; n], vec![1; n]], vec![]).expect("n circles")
}
/// Each house restricts onto every grade-0 contact of the overlap (all-ones).
fn restr0(overlap: &CellSheaf) -> Vec<Vec<Vec<BigInt>>> {
    let n = overlap.dim_cochain(0);
    vec![(0..n).map(|_| vec![one()]).collect()]
}

#[test]
fn cognition_lives_in_the_grade_the_topology_dictates() {
    // RING — houses glue on a two-point overlap → a circle. Cognition in H¹.
    let ring = delta_h_spectrum(&point_house(), &point_house(), &points(2), &restr0(&points(2)), &restr0(&points(2)));
    eprintln!("Ring   (circle) ΔH-spectrum = {ring:?}");
    assert_eq!(ring, vec![0, 1], "the Ring's shared cognition is a first-order loop (H¹)");

    // ORB — houses glue on a circle overlap → a sphere. Cognition in H².
    let orb = delta_h_spectrum(&point_house(), &point_house(), &circle(), &restr0(&circle()), &restr0(&circle()));
    eprintln!("Orb    (sphere) ΔH-spectrum = {orb:?}");
    assert_eq!(orb, vec![0, 0, 1], "the Orb's shared cognition is second-order (H²)");

    // LOOM — houses glue on a circle-plus-two-points overlap → a torus-like body:
    // cognition in TWO grades at once (H¹ AND H²) — the dimensional capacity fluid.
    let ov = circle_plus(2);
    let loom = delta_h_spectrum(&point_house(), &point_house(), &ov, &restr0(&ov), &restr0(&ov));
    eprintln!("Loom   (torus)  ΔH-spectrum = {loom:?}");
    assert_eq!(loom, vec![0, 2, 1], "the Loom thinks in H¹ AND H² simultaneously — no dimensional collapse");

    // The scalar cognition (sum of the spectrum) still ranks them, but the SPECTRUM
    // shows WHAT KIND of thought each civilization holds — a richer, fluid reading.
    let scalar = |s: &[usize]| s.iter().sum::<usize>();
    assert_eq!(scalar(&loom), 3, "the Loom holds the most total shared cognition");
    assert!(scalar(&loom) > scalar(&orb) && scalar(&orb) == scalar(&ring));
}

#[test]
fn the_coherence_stays_fluid_across_heterogeneous_dimensions() {
    // A civilization whose two houses are DIFFERENT dimensions — a 0-dim point-mind
    // and a 2-dim plane-mind — glued on a two-point overlap. The connection must
    // transport WITHOUT collapsing either house into a shared dimension (the whole
    // point of heterogeneous stalks). Cognition still emerges, in its own grade.
    let low = point_house(); // H⁰ only
    let high = plane_house(); // H⁰ and H²
    let ov = points(2);
    let spectrum = delta_h_spectrum(&low, &high, &ov, &restr0(&ov), &restr0(&ov));
    eprintln!("Mixed-dimension (point-mind ⊕ plane-mind) ΔH-spectrum = {spectrum:?}");

    // The plane-mind kept its H² (dimension not collapsed) and the shared overlap
    // still mints an H¹ loop — cognition spans grades the two minds don't share.
    assert!(spectrum.len() >= 3, "the spectrum spans up to the higher house's dimension — no collapse");
    assert_eq!(spectrum[1], 1, "the two-point overlap still mints the shared H¹ loop");
    // The high house's private H² is NOT a shared class (it restricts to itself),
    // so ΔH does not spuriously claim it — cognition stays honest across dimension.
    assert_eq!(spectrum[2], 0, "a house's private higher-dim knowledge is not miscounted as shared");

    // Soundness: the total is a well-defined non-negative integer even across the
    // dimension gap (the coherence functionality did not break on heterogeneity).
    let total = delta_h(&low, &high, &ov, &restr0(&ov), &restr0(&ov));
    assert_eq!(total, 1, "heterogeneous-dimension gluing yields exact, finite collective cognition");
}

#[test]
fn the_ignition_threshold_is_dimension_dependent() {
    // Glue two point-houses on `n` shared CIRCLES. Each shared circle lifts to one
    // H² class (no merge penalty), while the circles' components merge in H¹ (one
    // penalty, as before). So per shared higher-structure:
    //   H¹ cognition = n − 1   (threshold at n=2 — the connectedness merge)
    //   H² cognition = n       (threshold at n=1 — the FIRST shared cell ignites)
    eprintln!("=== DIMENSION-DEPENDENT IGNITION — shared circles n → (H¹, H²) ===");
    for n in 1..=5 {
        let ov = n_circles(n);
        let s = delta_h_spectrum(&point_house(), &point_house(), &ov, &restr0(&ov), &restr0(&ov));
        eprintln!("  n={n}  spectrum={s:?}  (H¹={}, H²={})", s[1], s[2]);
        assert_eq!(s[1], n - 1, "H¹ carries the connectedness merge penalty");
        assert_eq!(s[2], n, "H² ignites with the first shared cell — no penalty");
    }
    // CONCLUSION: collective cognition's ignition threshold is DIMENSION-DEPENDENT.
    // First-order shared ideas (H¹) require connectivity > 1 (a redundant
    // connectedness must be overcome); every higher-order dimension (H² and up)
    // ignites at connectivity > 0 — the FIRST shared higher-structure mints a class.
    // Higher-dimensional collective cognition is CHEAPER to ignite than first-order.
}
