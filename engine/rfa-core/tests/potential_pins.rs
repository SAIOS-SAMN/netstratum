//! DIRECTIVE 10 — the potential 1-form and the dual-pole non-vacuity
//! lock. `dV` is carried WHOLE with its positions; non-vacuity is a
//! structural emptiness question, never a comparison against zero.

use cfe::ratio::{ratio, Ratio};
use rfa_core::{holonomies, potential_form, Delta, Support};

fn edge(d: &mut Delta, s: &mut Support, i: usize, j: usize, v: Ratio) {
    d.set_antisym(i, j, vec![v]);
    s.connect(i, j, 0);
}

#[test]
fn a_flat_landscape_is_vacuous_even_when_edges_are_present() {
    // Every pair relates, and every obligation is exactly zero. The
    // potential never slopes: dV is EMPTY. Presence is not potential.
    let mut d = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut d, &mut s, 0, 1, ratio(0, 1));
    edge(&mut d, &mut s, 1, 2, ratio(0, 1));

    let form = potential_form(&d, &s);
    assert_eq!(form.rank(), 0, "a flat landscape slopes nowhere");
    assert!(!form.is_non_vacuous(), "dV ≡ 0 — refused by DIRECTIVE 10");
}

#[test]
fn a_sloping_landscape_is_non_vacuous_and_carries_its_positions() {
    let mut d = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut d, &mut s, 0, 1, ratio(2, 1));
    edge(&mut d, &mut s, 1, 2, ratio(3, 1));

    let form = potential_form(&d, &s);
    assert!(form.is_non_vacuous(), "the landscape slopes");
    assert_eq!(form.rank(), 2, "two forest edges carry slope");
    // Carried WHOLE: every component keeps its position and exact value.
    for slope in &form.slopes {
        assert!(slope.value != ratio(0, 1));
        assert!(slope.l == 0);
    }
}

#[test]
fn static_stasis_capacity_without_gradient_is_the_state_to_refuse() {
    // THE ROW THAT JUSTIFIES DIRECTIVE 10: D > 0 while dV ≡ 0.
    // A triangle whose forest edges are flat but whose closing edge
    // carries charge — all the structure sits in a closed loop and the
    // landscape slopes nowhere. Capacity alone would have admitted it.
    // Both flat edges meet at the root, so the walk takes them into the
    // forest and the charged edge 1—2 is left to close the cycle.
    let mut d = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut d, &mut s, 0, 1, ratio(0, 1)); // forest
    edge(&mut d, &mut s, 0, 2, ratio(0, 1)); // forest
    edge(&mut d, &mut s, 1, 2, ratio(7, 1)); // closes the cycle

    let capacity = holonomies(&d, &s);
    let form = potential_form(&d, &s);

    assert!(!capacity.is_empty(), "D > 0 — the loop carries charge");
    assert!(!form.is_non_vacuous(), "dV ≡ 0 — the landscape is flat");
    // Neither pole alone decides. Both must stand.
    assert!(
        !(capacity.len() > 0 && form.is_non_vacuous()),
        "static stasis: structure with no gradient does no work"
    );
}

#[test]
fn the_entangled_state_holds_both_poles() {
    // D > 0 AND dV ≢ 0 — the only admitted row.
    let mut d = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut d, &mut s, 0, 1, ratio(2, 1));
    edge(&mut d, &mut s, 1, 2, ratio(3, 1));
    edge(&mut d, &mut s, 0, 2, ratio(9, 1)); // 9 != 2+3: charge survives

    let capacity = holonomies(&d, &s);
    let form = potential_form(&d, &s);
    assert!(!capacity.is_empty(), "D > 0");
    assert!(form.is_non_vacuous(), "dV != 0");
}

#[test]
fn an_empty_complex_is_vacuous_and_total() {
    // Totality: no entities, no edges, no panic.
    let form = potential_form(&Delta::zero(0, 1), &Support::new(0, 1));
    assert!(!form.is_non_vacuous());
    let form = potential_form(&Delta::zero(3, 1), &Support::new(3, 1));
    assert_eq!(form.rank(), 0, "entities that relate to nothing slope nowhere");
}

#[test]
fn the_form_is_forest_dependent_and_this_is_recorded_not_hidden() {
    // KNOWN LIMITATION (see the module header). The spanning forest is
    // traversal-dependent, and `dV` is read off the forest — so the SAME
    // state can read flat under one forest and sloping under another.
    //
    // Here the charged edge meets the root, so the walk takes it INTO
    // the forest and the slope shows. In
    // `static_stasis_capacity_without_gradient_is_the_state_to_refuse`
    // the identical triangle (one edge carrying 7, two carrying 0) reads
    // FLAT, because there the charge sits on the closing edge.
    //
    // The capacity pole does not have this problem: holonomy VALUES are
    // class invariants. Non-vacuity of dV is not yet, so DIRECTIVE 10
    // may not become a kernel invariant until it is given a
    // forest-independent form.
    let mut d = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut d, &mut s, 0, 1, ratio(7, 1)); // charge ON a forest edge
    edge(&mut d, &mut s, 0, 2, ratio(0, 1));
    edge(&mut d, &mut s, 1, 2, ratio(0, 1));

    let form = potential_form(&d, &s);
    assert!(
        form.is_non_vacuous(),
        "same shape, different placement — and the reading flips"
    );
}
