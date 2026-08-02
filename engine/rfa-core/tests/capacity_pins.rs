//! THE CAPACITY PINS — the D meter held by witnesses, not by prose.
//!
//! Closes `18_TORSION.md` §6.2's first half. What these pin:
//!
//! 1. THE PAIRWISE LAW — a Δ built from node-local scalar differences
//!    is a pure coboundary and CIRCULATES NOWHERE. It stands ON THE
//!    GROUND: its degree reads `Degree::GROUND`, never zero. The meter
//!    is only as real as its feed.
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
//!    (§2d THE SIGN LAW), div = 0 balanced/circulating.
//! 10. RELABELLING-INVARIANT — Article I made a test.

use rfa_core::capacity::{
    borne, borne_axial, capacity, capacity_anchored, circulation, circulation_on, divergence,
    ext_x, flux,
    ground,
    preserves_hodge_split, Degree, Extent, Flux,
};
use rfa_core::{coboundary_reduce, Delta, Support};

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
    // A meter fed node-local scalar differences never circulates —
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
        // It sits ON THE GROUND — and the DEGREE proves it, not a
        // comment: the reading is 1, the floor, never 0.
        assert!(read.is_ground(), "a pure coboundary stands at the floor: {read:?}");
        assert_eq!(read.degree(), Degree::GROUND);
        assert_eq!(read.degree().get(), 1, "on the ground reads ONE, never zero");
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
    assert!(flux(&anchor).sources > 0, "the anchor does source — read by flux()");
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
    // non-trivial class — ∮ ≠ 0. Pure circulation with no source
    // anywhere: motion that takes nothing and leaves nothing, and
    // still is not nothing.
    let loop3 = triangle(q(1, 1), q(1, 1), q(-1, 1));
    let census = flux(&loop3);
    assert_eq!(
        census,
        Flux { sources: 0, sinks: 0, balanced: 3 },
        "a closed loop sources nothing and traps nothing"
    );
    assert!(census.is_pure_circulation(), "nothing sources, nothing sinks (§2d)");
    assert!(
        capacity(&loop3).circulates(),
        "holonomy alone is non-vacuous — this is what rank ALONE would \
         have admitted, and why the dual pole earns its place"
    );
    assert!(
        flux(&loop3).sources == 0,
        "and it sources NOTHING — pure holonomy, the mirror of the anchor"
    );
    // The circulation of a div-free form is itself: nothing to strip.
    assert_eq!(circulation(&loop3), loop3);
}

#[test]
fn the_sign_of_divergence_is_the_egress_law() {
    // §2d THE SIGN LAW: div > 0 is EGRESS (the vertex sources),
    // div < 0 is INGRESS (the vertex sinks), div = 0 is balanced.
    //
    // Antisymmetry makes the books close, so a source at one vertex
    // implies an ingress at another: no relation is all-source. The
    // sign law is therefore a statement about a BOUNDARY, not about
    // an isolated vertex: the books close over the whole relation.
    let sourced = triangle(q(1, 1), q(0, 1), q(1, 1)); // vertex 0 emits
    let census = flux(&sourced);
    assert!(census.sources >= 1, "vertex 0 emits — an egress stands");
    assert!(
        census.sinks >= 1,
        "and the books close: an egress somewhere is an ingress elsewhere"
    );
    assert_eq!(
        census.sources + census.sinks + census.balanced,
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
    // AND THE GROUND IS NOT ZERO — asserted on the READING, not on the
    // constant. The previous form here was `assert_eq!(Extent::GROUND, 1)`,
    // which compares a constant to its own literal and proves nothing
    // about what any state reads; the floor was declared and never
    // computed. This asserts the reading.
    assert_eq!(
        capacity(&empty).degree(),
        Degree::GROUND,
        "a wide empty container stands ON the ground, not in it"
    );
    assert_eq!(capacity(&empty).degree().get(), 1, "the reading is ONE, never zero");
}

/// **THE STANDING TRIPWIRE — `D` NEVER READS ZERO, FOR ANY INPUT.**
///
/// The floor is a property of every reading, so it is pinned over a
/// sweep rather than one fixture: empty containers at many arities,
/// pure coboundaries, genuine cycles, and the raw/anchored/circulation
/// readings alike. If any future change lets a degree reach zero, this
/// fails before it reaches the kernel.
#[test]
fn no_reading_of_any_shape_ever_falls_below_the_ground() {
    let mut shapes: Vec<Delta> = Vec::new();
    // Empty containers across arities — the widest vacuous case.
    for dim in 0..6usize {
        for fiber in 1..4usize {
            shapes.push(Delta::zero(dim, fiber));
        }
    }
    // Pure coboundaries (circulate nowhere) and genuine cycles.
    shapes.push(from_potentials(&[q(0, 1), q(1, 1), q(3, 2)]));
    shapes.push(from_potentials(&[q(5, 3), q(-7, 2), q(11, 4), q(0, 1)]));
    shapes.push(triangle(q(2, 1), q(3, 1), q(9, 1)));
    shapes.push(triangle(q(1, 1), q(1, 2), q(3, 2)));
    shapes.push(triangle(q(-5, 1), q(-5, 1), q(3, 1)));

    for (n, delta) in shapes.iter().enumerate() {
        for (name, read) in [
            ("capacity", capacity(delta)),
            ("capacity_anchored", capacity_anchored(delta)),
            ("ext_x", ext_x(delta)),
            ("ext_x∘circulation", ext_x(&circulation(delta))),
        ] {
            let degree = read.degree();
            assert!(
                degree.get() >= Extent::GROUND,
                "shape {n} via {name}: degree {degree} fell below the ground"
            );
            assert!(degree >= Degree::GROUND, "shape {n} via {name}: below the floor");
            // The two predicates partition the elevation exactly — a
            // reading is on the ground or above it, never elsewhere.
            assert_ne!(
                degree.is_ground(),
                degree.is_above_ground(),
                "shape {n} via {name}: elevation must be exactly one of the two"
            );
            // `above()` is the circulation, and the floor is the rest.
            assert_eq!(degree.above(), read.coordinates, "shape {n} via {name}");
            assert_eq!(degree.get(), Extent::GROUND + read.coordinates);
        }
    }
}

/// The constructor cannot be argued below the floor: every circulation
/// count, including zero, lands on or above the ground. There is no
/// other public way to build a `Degree` — the field is private, there
/// is no `From<usize>`, and `Default` is the GROUND.
#[test]
fn the_only_constructor_can_never_yield_a_below_ground_degree() {
    for circulation_count in 0..64usize {
        let degree = Degree::above_ground(circulation_count);
        assert!(degree >= Degree::GROUND);
        assert_eq!(degree.get(), Extent::GROUND + circulation_count);
        assert_eq!(degree.above(), circulation_count);
        assert_eq!(degree.is_ground(), circulation_count == 0);
        assert_eq!(degree.is_above_ground(), circulation_count > 0);
    }
    assert_eq!(Degree::default(), Degree::GROUND, "a Degree from nowhere still stands");
    assert_eq!(Degree::GROUND.get(), 1);
    assert_eq!(Degree::GROUND.above(), 0, "nothing circulates at the floor");
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
    // (on the ground) is carried by a det-1 shear congruence to a state reading
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
    let anchor = anchor_3(); // a pure coboundary: on the ground
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

// ===========================================================================
// THE BORNE SHARE — SYN-3. `D` says how much room stands; these pin WHO IS
// STANDING IN IT, and that the answer is not an artifact of the labelling.
// ===========================================================================

/// A theta graph on 4 entities: the 4-cycle 0-1-2-3-0 plus the chord 0-2.
/// TWO independent cycles, so a fundamental basis has a real choice to make
/// — which is exactly the choice `borne` must not depend on.
const THETA: [(usize, usize); 5] = [(0, 1), (1, 2), (2, 3), (3, 0), (0, 2)];

fn theta_4(vals: [Ratio; 5]) -> (Delta, Support) {
    let mut delta = Delta::zero(4, 1);
    let mut support = Support::new(4, 1);
    for (k, (i, j)) in THETA.iter().enumerate() {
        delta.set_antisym(*i, *j, vec![vals[k].clone()]);
        support.connect(*i, *j, 0);
    }
    (delta, support)
}

/// A complete support on `n` entities — every relation present.
fn complete(n: usize) -> Support {
    let mut support = Support::new(n, 1);
    for i in 0..n {
        for j in (i + 1)..n {
            support.connect(i, j, 0);
        }
    }
    support
}

/// Every permutation of 4 labels.
fn perms_4() -> Vec<[usize; 4]> {
    let mut out = Vec::new();
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                for d in 0..4 {
                    let p = [a, b, c, d];
                    let mut seen = [false; 4];
                    for x in p {
                        seen[x] = true;
                    }
                    if seen.iter().all(|s| *s) {
                        out.push(p);
                    }
                }
            }
        }
    }
    out
}

fn relabel_4(delta: &Delta, support: &Support, perm: [usize; 4]) -> (Delta, Support) {
    let mut out = Delta::zero(4, 1);
    let mut sup = Support::new(4, 1);
    for i in 0..4 {
        for j in (i + 1)..4 {
            out.set_antisym(i, j, vec![delta.entry(perm[i], perm[j], 0).clone()]);
            if support.has(perm[i], perm[j], 0) {
                sup.connect(i, j, 0);
            }
        }
    }
    (out, sup)
}

#[test]
fn the_borne_share_is_relabelling_invariant() {
    // ARTICLE I, held at ATTRIBUTION. This is the pin the whole shape was
    // chosen for. The intuitive rule — split a cycle's charge among its
    // nodes, |c|/N_C — cannot be written, because a fundamental cycle BASIS
    // is picked by a spanning forest: on this very fixture the cycles read
    // {0,1,2},{0,2,3} under one labelling and {0,1,2,3},{0,1,2} under
    // another, so N_C is 3,3 or 4,3 for identical structure and different
    // entities are paid different amounts for the same world. Only the
    // cycle-space DIMENSION is invariant, never a chosen basis of it.
    //
    // The circulation is the UNIQUE Hodge representative — no forest, no
    // root — so a share read off it moves with the entities and never
    // without them. Measured over all 24 relabellings.
    for (delta, support) in [
        theta_4([q(1, 1), q(2, 1), q(1, 1), q(1, 1), q(-3, 1)]),
        theta_4([q(-5, 3), q(7, 2), q(0, 1), q(9, 4), q(1, 6)]),
        theta_4([q(1, 1), q(1, 1), q(1, 1), q(1, 1), q(1, 1)]),
    ] {
        let base = borne(&delta, &support);
        for perm in perms_4() {
            let (rd, rs) = relabel_4(&delta, &support, perm);
            let got = borne(&rd, &rs);
            let want: Vec<Ratio> = (0..4).map(|i| base[perm[i]].clone()).collect();
            assert_eq!(got, want, "the borne share must travel with the entity: {perm:?}");
        }
    }
}

#[test]
fn an_entity_holding_no_relation_bears_nothing() {
    // FARMING BY ABSENCE — caught before it reached the economy. Read on
    // the COMPLETE graph, four entities with a single relation 0—1 give
    // shares [1/2, 1/2, 1/4, 1/4]: entities 2 and 3 hold nothing, owe
    // nothing, are owed nothing, and each draw a QUARTER of a relation
    // they are not party to. That is cheaper than farming strain — it
    // costs nothing whatsoever.
    //
    // On the SUPPORT, an isolated entity is its own component: it holds no
    // relation to sum, so it bears exactly the ground.
    let mut delta = Delta::zero(4, 1);
    delta.set_antisym(0, 1, vec![q(1, 1)]);
    let mut support = Support::new(4, 1);
    support.connect(0, 1, 0);

    let shares = borne(&delta, &support);
    assert_eq!(shares[2], q(0, 1), "an entity with no relations drew a share");
    assert_eq!(shares[3], q(0, 1), "an entity with no relations drew a share");
}

#[test]
fn seating_empty_entities_moves_no_share() {
    // THE SECOND HALF OF THE SAME DEFECT. On the complete graph the very
    // same relation reads `circ = 0` among two entities and `1/2` among
    // four, so an orb could enlarge every share in the world simply by
    // seating empty orbs beside it — dilution mistaken for work.
    //
    // Components decouple, so the theta's shares must not move when the
    // world is widened with entities that hold nothing.
    let (delta, support) = theta_4([q(1, 1), q(2, 1), q(1, 1), q(1, 1), q(-3, 1)]);
    let base = borne(&delta, &support);

    for extra in 1..4 {
        let wide = 4 + extra;
        let mut big = Delta::zero(wide, 1);
        let mut big_support = Support::new(wide, 1);
        for (k, (i, j)) in THETA.iter().enumerate() {
            big.set_antisym(*i, *j, vec![delta.entry(*i, *j, 0).clone()]);
            let _ = k;
            big_support.connect(*i, *j, 0);
        }
        let widened = borne(&big, &big_support);
        assert_eq!(&widened[..4], &base[..], "seating {extra} empty entities moved a share");
        for (i, share) in widened.iter().enumerate().skip(4) {
            assert_eq!(*share, q(0, 1), "seated entity {i} drew a share for holding nothing");
        }
    }
}

#[test]
fn the_borne_share_conserves_the_circulation() {
    // NOTHING IS MINTED AT THE JOIN AND NOTHING FALLS OUT OF IT. Each
    // relation's magnitude is halved and BOTH halves are handed out, so the
    // shares sum to exactly the circulation standing on the support. The
    // join is a distribution, not a reduction — and not a fountain.
    let three = complete(3);
    for (delta, support) in [
        theta_4([q(1, 1), q(2, 1), q(1, 1), q(1, 1), q(-3, 1)]),
        theta_4([q(-5, 3), q(7, 2), q(0, 1), q(9, 4), q(1, 6)]),
        (anchor_3(), three.clone()),
        (triangle(q(1, 1), q(1, 1), q(-1, 1)), three.clone()),
    ] {
        let circ = circulation_on(&delta, &support);
        let mut whole = q(0, 1);
        for i in 0..delta.dim() {
            for j in (i + 1)..delta.dim() {
                if !support.has(i, j, 0) {
                    continue;
                }
                let v = circ.entry(i, j, 0).clone();
                whole += if v < q(0, 1) { q(0, 1) - v } else { v };
            }
        }
        let handed: Ratio = borne(&delta, &support).into_iter().fold(q(0, 1), |a, b| a + b);
        assert_eq!(handed, whole, "every half of every relation must reach an entity");
    }
}

#[test]
fn a_pure_coboundary_bears_nothing() {
    // FRAME IS NOT WORK. A Δ built from node-local potentials circulates
    // nowhere, so there is nothing to stand in and no share to draw. This
    // is the floor of the anti-farming property: mere presence earns zero.
    for potentials in [
        vec![q(0, 1), q(1, 1), q(3, 2), q(-2, 5)],
        vec![q(5, 3), q(-7, 2), q(11, 4), q(0, 1)],
    ] {
        let n = potentials.len();
        let mut delta = Delta::zero(n, 1);
        for i in 0..n {
            for j in (i + 1)..n {
                delta.set_antisym(i, j, vec![&potentials[j] - &potentials[i]]);
            }
        }
        for (i, share) in borne(&delta, &complete(n)).into_iter().enumerate() {
            assert_eq!(share, q(0, 1), "entity {i} bore a share of pure frame");
        }
    }
}

#[test]
fn no_share_can_be_farmed_by_adding_frame() {
    // THE ANTI-FARMING PIN. An entity cannot enlarge its own share by
    // dressing the world in coordinates. `borne` reads the circulation
    // ONLY, and a gauge shift lands entirely in the gradient summand, so
    // every share is untouched. Work earns; framing does not.
    let (delta, support) = theta_4([q(1, 1), q(2, 1), q(1, 1), q(1, 1), q(-3, 1)]);
    let base = borne(&delta, &support);

    let potentials = [q(7, 3), q(-2, 5), q(4, 1), q(19, 6)];
    let mut shifted = Delta::zero(4, 1);
    for i in 0..4 {
        for j in (i + 1)..4 {
            let gauge = &potentials[j] - &potentials[i];
            shifted.set_antisym(i, j, vec![delta.entry(i, j, 0) + gauge]);
        }
    }
    assert_eq!(borne(&shifted, &support), base, "a gauge shift must move no entity's share");
}

#[test]
fn every_share_is_positive_or_ground() {
    // SYN-3's REASON FOR OPENING. Allocation existed only as WITHHOLDING —
    // a refusal, stated negatively. A share is a magnitude: it is never
    // below the ground, so the rule can only give. What an entity holds up
    // is stated in its own terms, not as the absence of a penalty.
    let three = complete(3);
    for (delta, support) in [
        theta_4([q(-5, 3), q(7, 2), q(0, 1), q(9, 4), q(1, 6)]),
        theta_4([q(-1, 1), q(-1, 1), q(-1, 1), q(-1, 1), q(-1, 1)]),
        (triangle(q(-3, 7), q(5, 2), q(1, 9)), three.clone()),
    ] {
        for (i, share) in borne(&delta, &support).into_iter().enumerate() {
            assert!(share >= q(0, 1), "entity {i} drew a negative share: {share}");
        }
    }
}

#[test]
fn the_axial_share_distinguishes_worlds_the_scalar_cannot() {
    // NEVER ONE SCALAR (`18_TORSION` §2b; operator ruling 2026-08-02:
    // multi-axial on vertices towards uplift). The first landing of the
    // borne share summed across every coordinate, and a sum cannot tell
    // a world that stands on ONE axis from one spread across all of
    // them — which is the only part of the reading an entity can act on.
    //
    // Two worlds, one non-transitive triangle each. In the first, all
    // the circulation stands on coordinate 0. In the second, the same
    // structure is carried on coordinate 1 instead.
    let mut narrow = Delta::zero(3, 2);
    let mut wide = Delta::zero(3, 2);
    let mut support = Support::new(3, 2);
    for (i, j) in [(0, 1), (1, 2), (0, 2)] {
        support.connect(i, j, 0);
        support.connect(i, j, 1);
    }
    // Δ01 = Δ12 = 1, Δ02 = −1 — a triangle that does not compose.
    for (i, j, v) in [(0, 1, q(1, 1)), (1, 2, q(1, 1)), (0, 2, q(-1, 1))] {
        narrow.set_antisym(i, j, vec![v.clone(), q(0, 1)]);
        wide.set_antisym(i, j, vec![q(0, 1), v]);
    }

    // THE SCALARS AGREE — the collapse cannot separate them.
    assert_eq!(
        borne(&narrow, &support),
        borne(&wide, &support),
        "the summed reading is blind to WHICH coordinate carries the structure"
    );

    // THE AXIAL READINGS DO NOT. This is the information the sum destroys.
    let a = borne_axial(&narrow, &support);
    let b = borne_axial(&wide, &support);
    assert_ne!(a, b, "the axial reading separates what the scalar merges");
    for entity in 0..3 {
        assert!(a[entity][0] > q(0, 1), "the narrow world stands on coordinate 0");
        assert_eq!(a[entity][1], q(0, 1), "and coordinate 1 is OPEN to it — the uplift");
        assert_eq!(b[entity][0], q(0, 1), "the wide world stands on the other axis");
        assert!(b[entity][1] > q(0, 1));
    }
}

#[test]
fn the_scalar_share_is_exactly_the_l1_of_the_axial() {
    // The lens form is permitted (`18` §2b allows an L1 at a HUMAN
    // LENS), and it must be honestly derived from the reading that
    // computes — never separately calculated, or the two could drift
    // and the displayed total would stop being the sum of its parts.
    let (delta, support) = theta_4([q(-5, 3), q(7, 2), q(0, 1), q(9, 4), q(1, 6)]);
    let axial = borne_axial(&delta, &support);
    let scalar = borne(&delta, &support);
    for (entity, axes) in axial.into_iter().enumerate() {
        let summed = axes.into_iter().fold(q(0, 1), |a, b| a + b);
        assert_eq!(summed, scalar[entity], "the lens must be the L1 of the parts");
    }
}
