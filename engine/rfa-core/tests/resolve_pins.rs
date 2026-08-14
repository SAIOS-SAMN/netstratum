//! **THE RESOLVE, WITNESSED** — `forest_potentials` is the coherent frame
//! a sparse reduction fits to a web of pairwise relations, and it is the
//! other half of the same walk `holonomies` reads.

use cfe::ratio::{ratio, Ratio};
use rfa_core::{forest_potentials, holonomies, Delta, Support};

/// A path `0 — 1 — 2` (fiber 1), edges 0→1 = 2, 1→2 = 3. A path fills no
/// cycle, so the frame explains it WHOLE: the potentials are exactly the
/// accumulated relation (0, 2, 5) and there is no holonomy.
#[test]
fn a_path_resolves_to_its_potential_with_no_tension() {
    let mut delta = Delta::zero(3, 1);
    delta.set_antisym(0, 1, vec![ratio(2, 1)]);
    delta.set_antisym(1, 2, vec![ratio(3, 1)]);
    let mut support = Support::new(3, 1);
    support.connect(0, 1, 0);
    support.connect(1, 2, 0);

    let p = forest_potentials(&delta, &support);
    assert_eq!(p[0], vec![ratio(0, 1), ratio(2, 1), ratio(5, 1)], "the frame is the accumulated relation");
    assert!(holonomies(&delta, &support).is_empty(), "a path carries no irreducible cycle");
}

/// EVERY FOREST EDGE IS `p_j − p_i` — the defining property of the frame.
/// On a triangle whose three edges DO compose (0→1 = 1, 1→2 = 1, 0→2 = 2),
/// the closing edge agrees with the potentials, so the whole triangle is a
/// coboundary and no holonomy survives.
#[test]
fn a_composing_triangle_is_pure_frame() {
    let mut delta = Delta::zero(3, 1);
    delta.set_antisym(0, 1, vec![ratio(1, 1)]);
    delta.set_antisym(1, 2, vec![ratio(1, 1)]);
    delta.set_antisym(0, 2, vec![ratio(2, 1)]); // 1 + 1 — composes
    let mut support = Support::new(3, 1);
    support.connect(0, 1, 0);
    support.connect(1, 2, 0);
    support.connect(0, 2, 0);

    let p = &forest_potentials(&delta, &support)[0];
    // The closing edge's holonomy is exactly Δ_02 − (p_2 − p_0), and here
    // that is zero — the frame explains the triangle whole.
    let closing: Ratio = delta.entry(0, 2, 0).clone() - (p[2].clone() - p[0].clone());
    assert_eq!(closing, ratio(0, 1), "a composing edge agrees with the frame");
    assert!(holonomies(&delta, &support).is_empty(), "no charge no frame can explain");
}

/// A NON-COMPOSING triangle splits into frame + tension: the potentials
/// still explain the two forest edges, and the one closing edge carries
/// the entire irreducible charge. Frame and holonomy are the two halves of
/// the one resolve, and they reconstruct the edge exactly.
#[test]
fn a_broken_triangle_splits_into_frame_and_holonomy() {
    let mut delta = Delta::zero(3, 1);
    delta.set_antisym(0, 1, vec![ratio(1, 1)]);
    delta.set_antisym(1, 2, vec![ratio(1, 1)]);
    delta.set_antisym(0, 2, vec![ratio(5, 1)]); // ≠ 1 + 1 — does not compose
    let mut support = Support::new(3, 1);
    support.connect(0, 1, 0);
    support.connect(1, 2, 0);
    support.connect(0, 2, 0);

    let p = &forest_potentials(&delta, &support)[0];
    let charges = holonomies(&delta, &support);
    assert_eq!(charges.len(), 1, "one independent cycle, one charge");
    let h = &charges[0];
    // frame + holonomy reconstructs the closing edge byte-exact.
    let rebuilt: Ratio = (p[h.j].clone() - p[h.i].clone()) + h.value.clone();
    assert_eq!(rebuilt, *delta.entry(h.i, h.j, 0), "frame ⊕ tension = the relation, whole");
    // The forest rooted at 0 spans edges (0,1) and (0,2), leaving (1,2)
    // as the cycle-closing edge: its charge is Δ_12 − (p_2 − p_1) = 1 − 4.
    assert_eq!((h.i, h.j), (1, 2), "the non-forest edge closes the cycle");
    assert_eq!(h.value, ratio(-3, 1), "the charge the frame cannot explain");
}

/// An isolated vertex stands at its own ground — potential zero, relating
/// to nothing — never a phantom coordinate.
#[test]
fn an_isolated_vertex_stands_at_ground() {
    let mut delta = Delta::zero(3, 1);
    delta.set_antisym(0, 1, vec![ratio(4, 1)]);
    let mut support = Support::new(3, 1);
    support.connect(0, 1, 0);
    // vertex 2 is never connected.

    let p = &forest_potentials(&delta, &support)[0];
    assert_eq!(p[2], ratio(0, 1), "an unrelated vertex reads ground, not a guess");
}
