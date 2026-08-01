//! THE CAPACITY PINS — the D meter held by witnesses, not by prose.
//!
//! Closes `18_TORSION.md` §6.2's first half. What these pin:
//!
//! 1. THE PAIRWISE LAW — a Δ built from node-local scalar differences
//!    is a pure coboundary and reads D = 0 EVERYWHERE. The meter is
//!    only as real as its feed.
//! 2. THE HOUSE FIXTURE IS VACUOUS — `anchor_3`, the canonical
//!    equilibrium anchor, is itself a coboundary. Recorded, not hidden.
//! 3. A NON-TRANSITIVE TRIANGLE CARRIES CAPACITY — the positive case.
//! 4. FRAME-FREE — a gauge shift leaves D identical (D1: frame
//!    artifacts contribute NOTHING).
//! 5. IDEMPOTENT — why D is monotone by algebra, not by decree.
//! 6. CONTAINER ≠ CAPACITY — arity is not D (18 §5).
//! 7. THE POSITIVE CONSTRUCTION — capacity is the CIRCULATION, built by
//!    projection, not the residue left after stripping a frame.
//! 8. DIVERGENCE IS ONE PASS — no walk, no basepoint; div is grad's
//!    adjoint, so it detects the gradient exactly.
//! 9. THE SIGN IS THE LAW — div > 0 egress, div < 0 trapped interior
//!    (§2d forbids), div = 0 holonomic.
//! 10. RELABELLING-INVARIANT — Article I made a test.

use rfa_core::capacity::{
    capacity, capacity_anchored, circulation, divergence, egress, ext_x, ground,
    preserves_hodge_split, Egress, Extent,
};
use rfa_core::{coboundary_reduce, Delta};

use cfe::ratio::{ratio, Ratio};

fn q(n: i64, d: i64) -> Ratio {
    ratio(n, d)
}

/// A 3-entity Delta from three pair values on one fiber layer.
fn triangle(d01: Ratio, d12: Ratio, d02: Ratio) -> Delta {
    let mut delta = Delta::zero(3, 1);
    delta.set_antisym(0, 1, vec![d01]);
    delta.set_antisym(1, 2, vec![d12]);
    delta.set_antisym(0, 2, vec![d02]);
    delta
}

/// The house's canonical equilibrium anchor (`fold_pins::anchor_3`):
/// Δ01 = 1, Δ12 = 1/2, Δ02 = 3/2.
fn anchor_3() -> Delta {
    triangle(q(1, 1), q(1, 2), q(3, 2))
}

/// Build a Delta PURELY from node-local potentials: `Δ_ij = p_j − p_i`.
/// This is the shape the meter must read as vacuous.
fn from_potentials(potentials: &[Ratio]) -> Delta {
    let dim = potentials.len();
    let mut delta = Delta::zero(dim, 1);
    for i in 0..dim {
        for j in (i + 1)..dim {
            delta.set_antisym(i, j, vec![&potentials[j] - &potentials[i]]);
        }
    }
    delta
}

#[test]
fn a_pure_coboundary_sources_but_does_not_circulate() {
    // THE PAIRWISE LAW. Δ_ij = p_j − p_i for ANY potential p reduces to
    // exactly zero: Δ'_ij = (p_j−p_i) − (p_j−p_0) + (p_i−p_0) = 0.
    // A meter fed node-local scalar differences reads D = 0 forever —
    // working meter, vacuous feed.
    for potentials in [
        vec![q(0, 1), q(1, 1), q(3, 2)],
        vec![q(5, 3), q(-7, 2), q(11, 4), q(0, 1)],
        vec![q(2, 1), q(2, 1), q(2, 1), q(2, 1), q(9, 5)],
    ] {
        let delta = from_potentials(&potentials);
        let read = capacity(&delta);
        // NO CIRCULATION — the holonomic pole is empty.
        assert!(!read.circulates(), "a pure coboundary carries no holonomy: {read:?}");
        assert_eq!(read.entities, 0, "no entity stands in the circulation");
        assert_eq!(read.layers, 0, "no layer stands in the circulation");
        // It sits at the GROUND — which is NOT zero (Extent::GROUND).
        assert!(read.is_ground(), "a pure coboundary stands at the floor: {read:?}");
        assert_eq!(read, ground());
    }
}

#[test]
fn the_equilibrium_anchor_is_a_coboundary() {
    // RECORDED, not hidden: the canonical fixture every other pin is
    // built on satisfies Δ02 = Δ01 + Δ12 exactly, so it IS a coboundary
    // (potentials 0, 1, 3/2) and carries zero capacity. Its RAW extent
    // is non-zero — three standing pairs — which is precisely the
    // container/capacity distinction of 18 §5.
    let anchor = anchor_3();
    assert_eq!(
        ext_x(&anchor),
        Extent { entities: 3, layers: 1, coordinates: 3 },
        "the raw extent counts what the CONTAINER carries"
    );
    let read = capacity(&anchor);
    assert!(!read.circulates(), "the equilibrium anchor carries no holonomy");
    assert!(read.is_ground(), "it stands at the GROUND — the floor, never zero");
    // The divergence is still there and still readable; it is simply
    // not capacity, because it is gauge-dependent.
    assert!(egress(&anchor).sources > 0, "the anchor does source — read by egress()");
}

#[test]
fn a_non_transitive_triangle_carries_capacity() {
    // The positive case: Δ02 ≠ Δ01 + Δ12 is a genuine H¹ cocycle
    // defect — an obligation triangle that does not close.
    //
    // THE POSITIVE CONSTRUCTION NAMES THE WHOLE CYCLE. The circulation
    // reading puts all three entities and all three pairs in the
    // extent, because the class IS carried by the 3-cycle. The former
    // anchored reading returned {2, 1, 1} — one pair, two entities —
    // which was an artifact of gauge-fixing at vertex 0, not a fact
    // about the universe. Same class, honest extent.
    let broken = triangle(q(1, 1), q(1, 1), q(0, 1));
    let read = capacity(&broken);
    assert!(read.circulates(), "a non-transitive triangle MUST circulate");
    assert_eq!(
        read,
        Extent { entities: 3, layers: 1, coordinates: 3 },
        "the circulation carries the whole cycle, not one anchored pair"
    );
    // And the anchored reading still AGREES on existence, differing
    // only on extent — the drift is in the representative, never the
    // class.
    assert!(capacity_anchored(&broken).circulates());
}

#[test]
fn divergence_detects_the_gradient_in_one_pass() {
    // No walk, no spanning structure, no basepoint. div is the adjoint
    // of grad, so the gradient component vanishes exactly when the
    // divergence does. A pure coboundary has a gradient everywhere; a
    // closed loop has none.
    let anchor = anchor_3();
    assert!(
        divergence(&anchor).iter().any(|row| row.iter().any(|d| *d != q(0, 1))),
        "a pure gradient must show a non-zero divergence"
    );
    let loop3 = triangle(q(1, 1), q(1, 1), q(-1, 1));
    assert!(
        divergence(&loop3).iter().all(|row| row.iter().all(|d| *d == q(0, 1))),
        "a closed loop is divergence-free at every vertex"
    );
    // Divergence always sums to zero — antisymmetry closes the books.
    for delta in [anchor_3(), triangle(q(1, 1), q(1, 1), q(0, 1))] {
        let total = divergence(&delta)
            .iter()
            .fold(q(0, 1), |acc, row| acc + row[0].clone());
        assert_eq!(total, q(0, 1), "the divergence books close over the vertices");
    }
}

#[test]
fn a_closed_loop_is_pure_holonomy_and_non_vacuous() {
    // STATIC STASIS, honestly built: 0→1→2→0, each leg carrying 1.
    // Every vertex balances (dV is empty), yet the loop carries a
    // non-trivial class — ∮ ≠ 0. This is `S⁺_hol` with no `S⁺_out`
    // anywhere: motion that takes nothing and leaves nothing, and
    // still is not nothing.
    let loop3 = triangle(q(1, 1), q(1, 1), q(-1, 1));
    let census = egress(&loop3);
    assert_eq!(
        census,
        Egress { sources: 0, trapped: 0, balanced: 3 },
        "a closed loop sources nothing and traps nothing"
    );
    assert!(census.is_white_hole(), "no trapped interior (§2d)");
    assert!(
        capacity(&loop3).circulates(),
        "holonomy alone is non-vacuous — this is what rank ALONE would \
         have admitted, and why the dual pole earns its place"
    );
    assert!(
        egress(&loop3).sources == 0,
        "and it sources NOTHING — pure holonomy, the mirror of the anchor"
    );
    // The circulation of a div-free form is itself: nothing to strip.
    assert_eq!(circulation(&loop3), loop3);
}

#[test]
fn the_sign_of_divergence_is_the_egress_law() {
    // §2d: `S⁺_out` is EGRESS — strictly positive divergence, emission
    // only, NO trapped interior. The sign is the law, and it is the
    // negative force that survives the flip: a source is admitted, an
    // ingress is a trapped interior and is forbidden.
    //
    // Antisymmetry makes the books close, so a source at one vertex
    // implies an ingress at another: no relation is all-source. The
    // non-trapping law is therefore a statement about a BOUNDARY, not
    // about an isolated vertex — which is why §2d says "no trapped
    // INTERIOR region".
    let sourced = triangle(q(1, 1), q(0, 1), q(1, 1)); // vertex 0 emits
    let census = egress(&sourced);
    assert!(census.sources >= 1, "vertex 0 emits — an egress stands");
    assert!(
        census.trapped >= 1,
        "and the books close: an egress somewhere is an ingress elsewhere"
    );
    assert_eq!(
        census.sources + census.trapped + census.balanced,
        3,
        "every vertex is classified exactly once"
    );
}

#[test]
fn capacity_is_relabelling_invariant() {
    // ARTICLE I made a test. Any reading that requires a chosen basis
    // is void; the anchored representative depended on which vertex was
    // labelled 0, the circulation does not. Every permutation of the
    // entity labels must return the identical extent.
    let deltas = [
        anchor_3(),
        triangle(q(1, 1), q(1, 1), q(0, 1)),
        triangle(q(1, 1), q(1, 1), q(-1, 1)),
        triangle(q(-3, 7), q(5, 2), q(1, 9)),
    ];
    let perms: [[usize; 3]; 6] =
        [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    for delta in deltas {
        let base = capacity(&delta);
        for perm in perms {
            let mut relabelled = Delta::zero(3, 1);
            for i in 0..3 {
                for j in (i + 1)..3 {
                    relabelled.set_antisym(i, j, vec![delta.entry(perm[i], perm[j], 0).clone()]);
                }
            }
            assert_eq!(
                capacity(&relabelled),
                base,
                "capacity must not move when the entities are relabelled: {perm:?}"
            );
        }
    }
}

#[test]
fn capacity_is_frame_free() {
    // D1: frame artifacts contribute NOTHING. Adding any coboundary
    // (Δ_ij += p_j − p_i) must leave the capacity reading identical —
    // this is what makes D a property of the universe and not of the
    // observer's coordinates.
    let broken = triangle(q(1, 1), q(1, 1), q(0, 1));
    let base = capacity(&broken);

    let potentials = [q(7, 3), q(-2, 5), q(4, 1)];
    let mut shifted = Delta::zero(3, 1);
    for i in 0..3 {
        for j in (i + 1)..3 {
            let gauge = &potentials[j] - &potentials[i];
            shifted.set_antisym(i, j, vec![broken.entry(i, j, 0) + gauge]);
        }
    }

    assert_eq!(
        capacity(&shifted),
        base,
        "a gauge shift must not move the capacity reading"
    );
}

#[test]
fn capacity_is_idempotent() {
    // Π_G is idempotent, so the reading is too. This is WHY D is
    // monotone by algebra rather than by decree: what was frame was
    // never in D, so reducing again cannot lower it (EQUATIONS D4).
    for delta in [
        anchor_3(),
        triangle(q(1, 1), q(1, 1), q(0, 1)),
        triangle(q(-3, 7), q(5, 2), q(1, 9)),
    ] {
        let once = capacity(&delta);
        let twice = capacity(&coboundary_reduce(&delta));
        assert_eq!(once, twice, "capacity(Δ) must equal capacity(Π_G(Δ))");
    }
}

#[test]
fn the_container_is_not_the_capacity() {
    // 18 §5: the arities Delta{dim, fiber} are the CONTAINER the
    // relation rides on, never D. A wide, empty universe holds enormous
    // arity and zero capacity — the Article II vacuous case, and the
    // shape Article III prices at zero when deeds accompany it.
    let empty = Delta::zero(8, 4);
    assert_eq!(empty.dim(), 8);
    assert_eq!(empty.fiber(), 4);
    assert_eq!(
        capacity(&empty),
        ground(),
        "arity is not capacity — a wide empty container sits at the GROUND"
    );
    assert!(capacity(&empty).is_ground());
    // AND THE GROUND IS NOT ZERO. The floor is the point: a universe
    // that exists occupies one degree, itself (Extent::GROUND).
    assert_eq!(Extent::GROUND, 1, "D never reads zero — the ground is the point");
}

/// `A · Δ · Aᵀ` — CONGRUENCE, the transformation law of an alternating
/// 2-form. Δ is a 2-form (`18_TORSION` §2a: "already an alternating
/// form, not a metric tensor"), and a 2-form transforms by congruence,
/// never by the conjugation `A·Δ·A⁻¹` that acts on endomorphisms.
#[allow(clippy::needless_range_loop)] // matrix indices read clearest as indices
fn congruent(delta: &Delta, a: &[Vec<Ratio>]) -> Delta {
    let n = delta.dim();
    let mut inter = vec![vec![q(0, 1); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                inter[i][j] += &a[i][k] * delta.entry(k, j, 0);
            }
        }
    }
    let mut out = Delta::zero(n, 1);
    for i in 0..n {
        for j in (i + 1)..n {
            let mut acc = q(0, 1);
            for k in 0..n {
                acc += &inter[i][k] * &a[j][k];
            }
            out.set_antisym(i, j, vec![acc]);
        }
    }
    out
}

#[test]
fn congruence_preserves_antisymmetry_but_not_the_additive_class() {
    // THE TWO-GAUGE FINDING (measured 2026-08-01, DIRECTIVE 06 probe).
    // Two incompatible gauge groups act on this one tensor:
    //
    //   Π_G / coboundary_reduce : the ADDITIVE group of potentials Qⁿ,
    //                             stripping Δ_ij ↦ Δ_ij + (p_j − p_i)
    //   Operator / orbit_lock   : a MULTIPLICATIVE matrix group acting
    //                             on the entity indices
    //
    // Congruence A·Δ·Aᵀ is the correct transformation law for an
    // alternating 2-form and preserves antisymmetry for EVERY
    // invertible A — that half of DIRECTIVE 06 is sound and is pinned
    // here. But it MIXES entity indices, and the additive reduction
    // anchors at vertex 0, so the additive H¹ class is NOT congruence-
    // invariant.
    //
    // The counterexample is the house anchor itself: a PURE COBOUNDARY
    // (D = 0) is carried by a det-1 shear congruence to a state reading
    // D = 1. Capacity out of frame alone, with no work done — exactly
    // the forgery D1 exists to forbid.
    //
    // CONSEQUENCE, recorded: congruence cannot be adopted while the
    // reduction stays additive. Unify the gauge FIRST (one group for
    // both), then the transformation law follows from it.
    let mut a = vec![vec![q(0, 1); 3]; 3];
    for (i, row) in a.iter_mut().enumerate() {
        row[i] = q(1, 1);
    }
    a[0][1] = q(1, 5); // unipotent shear, det 1, NOT orthogonal

    // Half one — SOUND: antisymmetry survives a non-orthogonal frame.
    let broken = triangle(q(1, 1), q(1, 1), q(0, 1));
    let framed = congruent(&broken, &a);
    assert!(
        framed.verify_conservation(),
        "congruence must keep the form on the antisymmetric surface"
    );

    // Half two — THE BLOCKER: capacity is created out of pure frame.
    let anchor = anchor_3();
    assert!(
        !capacity(&anchor).circulates(),
        "the anchor is a pure coboundary — no holonomy before any frame change"
    );
    let framed_anchor = congruent(&anchor, &a);
    assert!(
        framed_anchor.verify_conservation(),
        "the congruent image is still a lawful alternating form"
    );
    assert!(
        capacity(&framed_anchor).circulates(),
        "MEASURED: a congruence frame change lifts D from 0 — the additive \
         class is not congruence-invariant, so the two gauges are not one"
    );
}

/// `A · Δ · Aᵀ` on the entity indices — the BASE congruence.
#[allow(clippy::needless_range_loop)]
fn base_congruence(delta: &Delta, a: &[Vec<Ratio>]) -> Delta {
    let n = delta.dim();
    let mut inter = vec![vec![q(0, 1); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                inter[i][j] += &a[i][k] * delta.entry(k, j, 0);
            }
        }
    }
    let mut out = Delta::zero(n, 1);
    for i in 0..n {
        for j in (i + 1)..n {
            let mut acc = q(0, 1);
            for k in 0..n {
                acc += &inter[i][k] * &a[j][k];
            }
            out.set_antisym(i, j, vec![acc]);
        }
    }
    out
}

#[test]
#[allow(clippy::needless_range_loop)] // matrix indices read clearest as indices
fn the_base_group_is_the_constants_stabiliser() {
    // THE RESIDUE OF THE f05bec8 MINT-LEAK, measured and sealed.
    //
    // The Hodge flip removed the ANCHOR but not this: a base congruence
    // preserves capacity iff A·1 = c·1 (every row summing alike),
    // because grad = 1pᵀ − p1ᵀ maps to (A1)(Ap)ᵀ − (Ap)(A1)ᵀ, which is
    // a gradient again only when A1 stays on the constants line. 1
    // spans ker L, so an A that leaves it does not carry potentials to
    // potentials and "pure frame" stops being well defined.
    let anchor = anchor_3(); // a pure coboundary: D = 0
    assert!(!capacity(&anchor).circulates());

    // (1) A SHEAR — row sums 6/5, 1, 1. NOT constant, NOT lawful.
    let mut shear = vec![vec![q(0, 1); 3]; 3];
    for i in 0..3 {
        shear[i][i] = q(1, 1);
    }
    shear[0][1] = q(1, 5);
    assert!(
        !preserves_hodge_split(&shear),
        "a shear breaks the constants line"
    );
    assert!(
        capacity(&base_congruence(&anchor, &shear)).circulates(),
        "MEASURED: the unlawful congruence FORGES capacity from pure \
         frame — 0 -> 3. This is why the base group must be restricted"
    );

    // (2) SCALING by 2 — row sums 2,2,2; det = 8. VOLUME SURVIVES.
    let mut scale = vec![vec![q(0, 1); 3]; 3];
    for i in 0..3 {
        scale[i][i] = q(2, 1);
    }
    assert!(preserves_hodge_split(&scale), "constant row sums are lawful");
    assert!(
        !capacity(&base_congruence(&anchor, &scale)).circulates(),
        "a volumetric scaling must NOT create capacity from frame"
    );

    // (3) NON-TRIVIAL MIXING, rows all summing to 1 — lawful, and it
    // genuinely moves the entities (not a permutation, not a scaling).
    let mix = vec![
        vec![q(1, 1), q(1, 1), q(-1, 1)],
        vec![q(-1, 1), q(1, 1), q(1, 1)],
        vec![q(1, 1), q(-1, 1), q(1, 1)],
    ];
    assert!(preserves_hodge_split(&mix));
    assert!(
        !capacity(&base_congruence(&anchor, &mix)).circulates(),
        "lawful mixing preserves the vacuous class"
    );

    // And it preserves a NON-vacuous class too — 1 -> 1, not 1 -> 0.
    let loop3 = triangle(q(1, 1), q(1, 1), q(-1, 1));
    assert!(capacity(&loop3).circulates());
    assert!(
        capacity(&base_congruence(&loop3, &mix)).circulates(),
        "lawful congruence must not destroy capacity either"
    );

    // The degenerate refusal: A·1 = 0 annihilates the constants.
    let zero = vec![vec![q(0, 1); 3]; 3];
    assert!(!preserves_hodge_split(&zero));
}
