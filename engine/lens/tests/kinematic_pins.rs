//! THE KINEMATIC PINS — the pressure of a twist, held by witnesses
//! (`19_LENS` §11).
//!
//! 1. A FLAT FIBER HOLDS NO PRESSURE — the abelian case, and the reason
//!    the layer needed a matrix connection to exist at all.
//! 2. NON-COMMUTING DIRECTIONS STRAIN — the circulation IS the field.
//! 3. A CLOSED WALK COSTS NOTHING TO HOLD; an open one has a holding
//!    pressure, carried whole.
//! 4. THE ORDER MATTERS — the whole difference between a sum and a
//!    product, and the thing an abelian reading cannot see.
//! 5. DRIFT IS PER-COMPONENT — direction survives, which is what the
//!    ancestor's signed magnitude threw away.
//! 6. THE STRAIN IS CAGED — shown, never returned as a number.

use cfe::ratio::{ratio, Ratio};
use lens::kinematic::{Drift, PhaseLock, Pressure};
use rfa_core::MatrixFiber2x2;

fn q(n: i64, d: i64) -> Ratio {
    ratio(n, d)
}

/// A scalar connection wearing matrix clothes: `c·I`. Multiples of the
/// identity commute with everything, so this is the flat case.
fn scalar(c: i64) -> MatrixFiber2x2 {
    MatrixFiber2x2::new(q(c, 1), q(0, 1), q(0, 1), q(c, 1))
}

/// A shear — does not commute with a general matrix.
fn shear(s: i64) -> MatrixFiber2x2 {
    MatrixFiber2x2::new(q(1, 1), q(s, 1), q(0, 1), q(1, 1))
}

/// A lower shear — does not commute with the upper one.
fn lower(s: i64) -> MatrixFiber2x2 {
    MatrixFiber2x2::new(q(1, 1), q(0, 1), q(s, 1), q(1, 1))
}

#[test]
fn a_flat_fiber_holds_no_pressure() {
    // THE ABELIAN CASE, and the reason this layer could not exist on a
    // scalar connection: [cI, dI] = 0 identically. A flat fiber reports
    // slack at every size — which is a FACT about the fiber, never a
    // fact about the structure it was meant to measure.
    for (a, b) in [(2, 3), (7, 11), (1, 1)] {
        let pressure = Pressure::between(&scalar(a), &scalar(b));
        assert!(pressure.is_slack(), "scalars commute: {a} against {b}");
        assert!(pressure.curvature.is_zero());
    }
}

#[test]
fn non_commuting_directions_strain() {
    // "The circulation IS the field." An upper and a lower shear do not
    // commute, so the cell they span carries flux.
    let pressure = Pressure::between(&shear(1), &lower(1));
    assert!(!pressure.is_slack(), "an upper and a lower shear must strain");
    assert!(!pressure.curvature.is_zero());

    // And the strain is CARRIED WHOLE — a caller can ask which way the
    // cell pushes, which a length could never answer.
    let f = &pressure.curvature;
    let any_component_stands =
        (0..2).any(|r| (0..2).any(|c| f.m[r][c] != q(0, 1)));
    assert!(any_component_stands, "the curvature names its components");
}

#[test]
fn a_closed_walk_costs_nothing_to_hold() {
    // A loop that returns the identity has closed: it came back where it
    // began, and the lock is free to maintain.
    let closed = PhaseLock::around(&[shear(2), shear(-2)]);
    assert!(closed.closed(), "a shear and its inverse close the walk");
    assert!(
        closed.holding_pressure().is_zero(),
        "a closed walk holds no flux, so holding it costs nothing"
    );

    // An empty walk closes too — nowhere to go, nothing acquired.
    let nowhere = PhaseLock::around(&[]);
    assert!(nowhere.closed());
    assert!(nowhere.holding_pressure().is_zero());
}

#[test]
fn an_open_walk_has_a_holding_pressure_carried_whole() {
    // A walk that does not return the identity is holding flux, and the
    // pressure required to hold it is how far the transport sits from
    // having closed — per component, never a length.
    let open = PhaseLock::around(&[shear(1), lower(1)]);
    assert!(!open.closed(), "these do not compose back to the identity");

    let holding = open.holding_pressure();
    assert!(!holding.is_zero(), "an open walk costs something to hold");
    // Whole, not summarised: the reading names WHERE the lock strains.
    let located = (0..2).any(|r| (0..2).any(|c| holding.m[r][c] != q(0, 1)));
    assert!(located, "the holding pressure names its components");
}

#[test]
fn the_order_matters_which_is_the_whole_point() {
    // "The holonomy is an ordered PRODUCT, not a sum." An abelian
    // reading cannot tell these two walks apart; a non-abelian one must.
    let forward = PhaseLock::around(&[shear(1), lower(1)]);
    let backward = PhaseLock::around(&[lower(1), shear(1)]);
    assert_ne!(
        forward.holonomy, backward.holonomy,
        "reversing a non-commuting walk MUST change what it accumulated"
    );

    // And repeating one factor does not return the identity — Aⁿ ≠ I,
    // which is what it means for the space to carry a field.
    let repeated = PhaseLock::around(&[shear(1), shear(1), shear(1)]);
    assert!(!repeated.closed(), "A³ is not the identity");
}

#[test]
fn drift_is_carried_per_component_so_direction_survives() {
    // The lawful form of the ancestor's coherence velocity. It threw
    // away direction by collapsing to a signed magnitude; this keeps it.
    let earlier = MatrixFiber2x2::new(q(1, 1), q(0, 1), q(0, 1), q(1, 1));
    let later = MatrixFiber2x2::new(q(1, 1), q(5, 1), q(-2, 1), q(1, 1));

    let drift = Drift::between(&earlier, &later);
    assert!(!drift.is_still());
    // Two components moved, and they moved in OPPOSITE directions — a
    // magnitude would have reported one number and lost the fact.
    assert_eq!(drift.component(0, 1), q(5, 1));
    assert_eq!(drift.component(1, 0), q(-2, 1));
    assert_eq!(drift.component(0, 0), q(0, 1), "the diagonal did not move");

    // Out of range reads as no movement — the reading is total.
    assert_eq!(drift.component(9, 9), q(0, 1));
}

#[test]
fn a_still_reading_does_not_claim_convergence() {
    // The ancestor named this arm `Converged`. A structure that did not
    // move may be at rest or may be stuck, and this reading refuses to
    // presume which — it reports STILL and says nothing more.
    let same = MatrixFiber2x2::new(q(3, 1), q(1, 1), q(0, 1), q(3, 1));
    let drift = Drift::between(&same, &same);
    assert!(drift.is_still());
    assert!(drift.delta.is_zero());
}

#[test]
fn the_strain_is_shown_and_never_returned_as_a_number() {
    // THE OPERATOR DISPLAY BOUND (§1.0). The magnitude is lawful at this
    // surface and is caged: exact rational in, text out, no accessor and
    // no ordering, so no rule can branch on how hard a cell is pushing.
    let pressure = Pressure::between(&shear(3), &lower(5));
    let shown = pressure.strain_shown();
    assert!(!shown.is_empty(), "a strain renders");
    assert!(
        shown.chars().all(|c| c.is_ascii_digit() || c == '/' || c == '-'),
        "it leaves as a terminal string, never as a comparable value: {shown}"
    );
}
