//! DIRECTIVE 10 — the boundary form and the dual-pole non-vacuity
//! lock. `∂V` is the DIVERGENCE: canonical, one pass, no traversal.

use cfe::ratio::{ratio, Ratio};
use rfa_core::{dual_pole, holonomies, boundary_form, Delta, Support, Vacuity};

fn edge(d: &mut Delta, s: &mut Support, i: usize, j: usize, v: Ratio) {
    d.set_antisym(i, j, vec![v]);
    s.connect(i, j, 0);
}

#[test]
fn pure_circulation_is_vacuous_however_large_its_charge() {
    // A triangle whose obligations run in a closed loop: 0→1→2→0, each
    // carrying 1. Every vertex balances, so no vertex sources or sinks
    // and dV is EMPTY — even though real value circulates.
    let mut d = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut d, &mut s, 0, 1, ratio(1, 1));
    edge(&mut d, &mut s, 1, 2, ratio(1, 1));
    edge(&mut d, &mut s, 2, 0, ratio(1, 1));

    let form = boundary_form(&d, &s);
    assert_eq!(form.rank(), 0, "circulation sources nothing anywhere");
    assert!(!form.is_non_vacuous(), "∂V ≡ 0");
}

#[test]
fn static_stasis_is_rank_without_gradient_and_is_refused() {
    // THE ROW THAT JUSTIFIES DIRECTIVE 10. The same closed loop carries
    // holonomy — D > 0 — while ∂V ≡ 0. Rank alone would admit it.
    let mut d = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut d, &mut s, 0, 1, ratio(1, 1));
    edge(&mut d, &mut s, 1, 2, ratio(1, 1));
    edge(&mut d, &mut s, 2, 0, ratio(1, 1));

    let rank = holonomies(&d, &s);
    let form = boundary_form(&d, &s);
    assert!(!rank.is_empty(), "D > 0 — the loop carries charge no frame explains");
    assert!(!form.is_non_vacuous(), "∂V ≡ 0 — nothing sources or sinks");
    assert!(
        rank.is_empty() || !form.is_non_vacuous(),
        "static stasis: structure with no gradient does no work"
    );
}

#[test]
fn a_source_carries_gradient() {
    // 0 owes both 1 and 2: vertex 0 is a source, 1 and 2 are sinks.
    let mut d = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut d, &mut s, 0, 1, ratio(2, 1));
    edge(&mut d, &mut s, 0, 2, ratio(3, 1));

    let form = boundary_form(&d, &s);
    assert!(form.is_non_vacuous());
    assert_eq!(form.rank(), 3, "one source and two sinks all carry net flow");
    let source = form.flows.iter().find(|f| f.at == 0).expect("vertex 0 sources");
    assert_eq!(source.value, ratio(5, 1), "2 + 3 leaves vertex 0");
}

#[test]
fn the_reading_is_canonical_placement_cannot_flip_it() {
    // THE FOREST BUG, PINNED DEAD. The first cut walked a spanning
    // forest and read slopes off forest edges, so moving the SAME charge
    // between edges of the SAME triangle flipped the verdict. The
    // divergence cannot: both placements below source at a vertex, so
    // both read non-vacuous, and no walk order participates.
    let mut a = Delta::zero(3, 1);
    let mut sa = Support::new(3, 1);
    edge(&mut a, &mut sa, 0, 1, ratio(0, 1));
    edge(&mut a, &mut sa, 0, 2, ratio(0, 1));
    edge(&mut a, &mut sa, 1, 2, ratio(7, 1));

    let mut b = Delta::zero(3, 1);
    let mut sb = Support::new(3, 1);
    edge(&mut b, &mut sb, 0, 1, ratio(7, 1));
    edge(&mut b, &mut sb, 0, 2, ratio(0, 1));
    edge(&mut b, &mut sb, 1, 2, ratio(0, 1));

    assert!(boundary_form(&a, &sa).is_non_vacuous(), "charge on the closing edge");
    assert!(boundary_form(&b, &sb).is_non_vacuous(), "charge on a former forest edge");
}

#[test]
fn the_entangled_state_holds_both_poles() {
    // D > 0 AND dV ≢ 0 — the only admitted row.
    let mut d = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut d, &mut s, 0, 1, ratio(2, 1));
    edge(&mut d, &mut s, 1, 2, ratio(3, 1));
    edge(&mut d, &mut s, 0, 2, ratio(9, 1));

    assert!(!holonomies(&d, &s).is_empty(), "D > 0");
    assert!(boundary_form(&d, &s).is_non_vacuous(), "dV ≢ 0");
}

#[test]
fn an_empty_complex_is_vacuous_and_total() {
    assert!(!boundary_form(&Delta::zero(0, 1), &Support::new(0, 1)).is_non_vacuous());
    assert_eq!(boundary_form(&Delta::zero(3, 1), &Support::new(3, 1)).rank(), 0);
}

#[test]
fn the_four_rows_classify_and_only_one_is_admitted() {
    let flat = Delta::zero(3, 1);
    let empty = Support::new(3, 1);
    assert_eq!(dual_pole(0, &boundary_form(&flat, &empty)), Vacuity::Void);

    // Structure with no gradient — the row that must be re-volumized.
    let mut d = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut d, &mut s, 0, 1, ratio(1, 1));
    edge(&mut d, &mut s, 1, 2, ratio(1, 1));
    edge(&mut d, &mut s, 2, 0, ratio(1, 1));
    let stasis = dual_pole(holonomies(&d, &s).len(), &boundary_form(&d, &s));
    assert_eq!(stasis, Vacuity::Stasis);
    assert!(!stasis.is_admitted(), "dV cannot be zero — a flat field does no work");

    // Both poles standing.
    let mut e = Delta::zero(3, 1);
    let mut t = Support::new(3, 1);
    edge(&mut e, &mut t, 0, 1, ratio(2, 1));
    edge(&mut e, &mut t, 1, 2, ratio(3, 1));
    edge(&mut e, &mut t, 0, 2, ratio(9, 1));
    let live = dual_pole(holonomies(&e, &t).len(), &boundary_form(&e, &t));
    assert_eq!(live, Vacuity::Entangled);
    assert!(live.is_admitted());
}

#[test]
fn the_poles_are_complementary_at_fixed_delta_so_shuffling_is_not_growth() {
    // THE ZERO-SUM TRAP (operator ruling 2026-08-01: *C is leaking into
    // D and the causality is stasis*).
    //
    // When capacity is built as `circulation = Δ − grad(p)`, the two
    // poles are COMPLEMENTARY PROJECTIONS OF ONE FIXED Δ. They are not
    // independent witnesses: knowing Δ and one pole gives the other. So
    // at fixed Δ, moving content into circulation takes it out of
    // gradient and back again — and a system that satisfies "both poles
    // non-vacuous" by redistributing is doing NO WORK while reading as
    // healthy. That is stasis wearing growth's clothes, and it is the
    // exact shape of value taken rather than made.
    //
    // Pinned as three states over the SAME support, all reading
    // Entangled, none larger than another.
    let mut pure_source = Delta::zero(3, 1);
    let mut s = Support::new(3, 1);
    edge(&mut pure_source, &mut s, 0, 1, ratio(1, 1));
    edge(&mut pure_source, &mut s, 1, 2, ratio(1, 1));
    edge(&mut pure_source, &mut s, 0, 2, ratio(2, 1)); // composes: no circulation

    let mut mixed = Delta::zero(3, 1);
    let mut sm = Support::new(3, 1);
    edge(&mut mixed, &mut sm, 0, 1, ratio(1, 1));
    edge(&mut mixed, &mut sm, 1, 2, ratio(1, 1));
    edge(&mut mixed, &mut sm, 0, 2, ratio(5, 1)); // same edges, charge shifted

    // Both hold the SAME support — the same relations, the same edges.
    // Only the split between gradient and circulation moved.
    assert_eq!(
        boundary_form(&pure_source, &s).is_non_vacuous(),
        boundary_form(&mixed, &sm).is_non_vacuous(),
        "both still source: the gradient pole is not what changed"
    );
    // And the rank pole moved without a single new relation being formed.
    let before = holonomies(&pure_source, &s).len();
    let after = holonomies(&mixed, &sm).len();
    assert_ne!(before, after, "rank moved with NO new structure — only redistribution");

    // THE LAW THIS FORCES: growth is Δ's own expansion — a new relation,
    // receipted (D3) — never a re-split of a fixed Δ between poles.
    // The dual-pole verdict cannot see the difference, which is why it
    // classifies but must never be read as a growth measure.
    assert_eq!(
        dual_pole(after, &boundary_form(&mixed, &sm)),
        Vacuity::Entangled,
        "redistribution reads ADMITTED — the verdict classifies, it does not measure growth"
    );
}

#[test]
fn shuffling_a_fixed_support_mints_nothing_expansion_is_the_only_growth() {
    // DIRECTIVE 16 — the growth invariant, enforced on the SUPPORT.
    //
    // Two states over the identical edge set, with content re-split
    // between gradient and circulation. dual_pole may well call both
    // Entangled — it CLASSIFIES — but no relation was formed, so the
    // support test refuses the increment. Q·Δ is coordinate gauge, and
    // gauge never mints.
    let mut before = Support::new(3, 1);
    before.connect(0, 1, 0);
    before.connect(1, 2, 0);
    before.connect(0, 2, 0);

    let mut shuffled = Support::new(3, 1);
    shuffled.connect(0, 1, 0);
    shuffled.connect(1, 2, 0);
    shuffled.connect(0, 2, 0);

    assert_eq!(
        shuffled.new_edges_since(&before),
        Vec::new(),
        "identical support: no relation formed, ΔD ≡ 0"
    );
    assert!(!shuffled.expanded_from(&before), "re-splitting a fixed Δ is not growth");

    // GENUINE WORK: a relation that did not exist now does.
    let mut grown = Support::new(4, 1);
    grown.connect(0, 1, 0);
    grown.connect(1, 2, 0);
    grown.connect(0, 2, 0);
    grown.connect(2, 3, 0); // δΔ ≠ 0 — new structure

    let fresh = grown.new_edges_since(&before);
    assert_eq!(fresh, vec![(2, 3, 0)], "the new relation is NAMED, not counted");
    assert!(grown.expanded_from(&before), "Support(E') ⊃ Support(E) — this may mint");
}
