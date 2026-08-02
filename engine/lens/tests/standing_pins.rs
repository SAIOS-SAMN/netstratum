//! THE STANDING PINS — which row a structure occupies (`19_LENS` §12).

use cfe::ratio::{ratio, Ratio};
use lens::standing::Standing;
use rfa_core::{Delta, Support, Vacuity};

fn q(n: i64, d: i64) -> Ratio { ratio(n, d) }

fn triangle(a: Ratio, b: Ratio, c: Ratio) -> (Delta, Support) {
    let mut d = Delta::zero(3, 1);
    d.set_antisym(0, 1, vec![a]);
    d.set_antisym(1, 2, vec![b]);
    d.set_antisym(0, 2, vec![c]);
    let mut s = Support::new(3, 1);
    for (i, j) in [(0, 1), (1, 2), (0, 2)] { s.connect(i, j, 0); }
    (d, s)
}

#[test]
fn an_empty_structure_stands_in_the_void() {
    let seen = Standing::of(&Delta::zero(3, 1), &Support::new(3, 1));
    assert_eq!(seen.row, Vacuity::Void);
    assert_eq!(seen.circulation, 0);
    assert_eq!(seen.flowing, 0);
    assert!(!seen.admitted());
    assert!(seen.wants().contains("first relation"));
}

#[test]
fn a_closed_ring_reads_stasis_and_is_not_called_a_failure() {
    // 0→1→2→0, each leg carrying 1: every vertex balances, real
    // structure, no free energy. LAW 4 — thermodynamic, not moral.
    let (d, s) = triangle(q(1, 1), q(1, 1), q(-1, 1));
    let seen = Standing::of(&d, &s);
    assert_eq!(seen.row, Vacuity::Stasis);
    assert!(seen.circulation > 0, "the ring carries real structure");
    assert_eq!(seen.flowing, 0, "and no vertex sources or sinks");
    assert!(!seen.admitted());
    // Stated as a next step, never as a fault.
    assert!(seen.wants().contains("re-volumizing"));
}

#[test]
fn a_non_composing_triangle_with_flow_is_entangled() {
    let (d, s) = triangle(q(2, 1), q(3, 1), q(9, 1));
    let seen = Standing::of(&d, &s);
    assert_eq!(seen.row, Vacuity::Entangled, "both poles stand");
    assert!(seen.circulation > 0 && seen.flowing > 0);
    assert!(seen.admitted());
    assert!(seen.wants().contains("nothing"));
}

#[test]
fn both_poles_are_carried_so_the_row_is_explicable() {
    // A verdict alone is a bare judgement. The reading shows WHY.
    let (d, s) = triangle(q(2, 1), q(3, 1), q(9, 1));
    let seen = Standing::of(&d, &s);
    let recomputed = Standing::of(&d, &s);
    assert_eq!(seen, recomputed, "the reading derives; it does not drift");
    assert!(seen.legible_at() == lens::peel::Depth::Far, "the row reads at distance");
}
