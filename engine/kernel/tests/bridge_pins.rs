//! THE BRIDGE — witnesses for the projection from committed state to
//! the relational tensor (Task #25).
//!
//! These pins exist to measure three things the design argued but
//! nothing had ever run: that the coboundary trap is real, that
//! composing obligations carry no structure, and that non-composing
//! ones do.

use cfe::dop::ColonyOrb;
use cfe::ratio::ratio;
use cfe::world::{Bond, Committed, World};
use kernel::bridge::{project, read_entangled, CHANNELS, CHANNEL_BOND};
use rfa_core::{coboundary_reduce, Delta};

fn orbs(n: u64) -> Committed {
    let mut committed = World::new().committed();
    committed.orbs = (1..=n)
        .map(|orb_id| ColonyOrb { orb_id, name: format!("orb{orb_id}"), ..Default::default() })
        .collect();
    committed
}

fn bond(source: u64, target: u64, rate: i64) -> Bond {
    Bond {
        source,
        target,
        resource: cfe::dop::Resource::Iron,
        rate: ratio(rate, 1),
        term: 0,
        sealed: 0,
        anchored: 0,
    }
}

#[test]
fn a_world_that_relates_nothing_carries_no_structure() {
    // Article II's vacuous case, measured: no obligations, no
    // structure. `D = 0` denies standing — and the reading is always
    // available (DIRECTIVE 03: the defect is a reading, never a gate).
    let read = read_entangled(&orbs(3));
    assert_eq!(read.capacity, 0, "no obligations, no surviving structure");
    assert_eq!(read.defect, ratio(0, 1), "nothing to fail to compose");

    // A lone orb relates to nothing at all, lawfully.
    let read = read_entangled(&orbs(1));
    assert_eq!(read.capacity, 0);
}

#[test]
fn the_coboundary_trap_is_real_node_scalars_reduce_to_nothing() {
    // THE MEASUREMENT THAT JUSTIFIES THE WHOLE BRIDGE DESIGN.
    // Had the projection been built from node-local scalars —
    // Δ_ij = scalar(j) − scalar(i), the obvious "difference of banks"
    // bridge — the reduction annihilates it and D reads zero for every
    // universe forever. Built by hand here because the bridge REFUSES
    // to construct it.
    let potentials = [ratio(0, 1), ratio(5, 1), ratio(3, 1)];
    let mut delta = Delta::zero(3, 1);
    for i in 0..3 {
        for j in (i + 1)..3 {
            delta.set_antisym(i, j, vec![potentials[j].clone() - potentials[i].clone()]);
        }
    }
    let reduced = coboundary_reduce(&delta);
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(
                *reduced.entry(i, j, 0),
                ratio(0, 1),
                "a pure coboundary MUST reduce to zero — this is the trap"
            );
        }
    }
}

#[test]
fn obligations_that_compose_carry_no_surviving_structure() {
    // 1→2 owes 2, 2→3 owes 3, 1→3 owes 5. The direct obligation equals
    // the composed path exactly, so the cocycle condition holds — and a
    // cocycle on a COMPLETE relation is a coboundary, so the reduction
    // takes it to nothing. Composing debt is frame, not structure.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 5)];

    let delta = project(&state);
    assert_eq!(*delta.entry(0, 1, CHANNEL_BOND), ratio(2, 1), "1 owes 2");
    assert_eq!(*delta.entry(1, 0, CHANNEL_BOND), ratio(-2, 1), "antisymmetric by construction");

    let read = read_entangled(&state);
    assert_eq!(read.defect, ratio(0, 1), "5 = 2 + 3: the triangle composes");
    assert_eq!(read.capacity, 0, "composing obligation is pure frame");
}

#[test]
fn a_tree_of_obligations_carries_no_capacity() {
    // DIRECTIVE 05, at the bridge. 1→2 and 2→3 are observed; 1→3 is
    // NOT. That is a spanning tree — E − V + C = 2 − 3 + 1 = 0 — so it
    // holds no independent cycle and no capacity.
    //
    // Before support masking the reduction read the absent (1,3) pair
    // as a real zero, anchored it at vertex 0, and manufactured a
    // non-zero survivor: a tree reported capacity 1, and Article II
    // would have granted standing to a world with no structure.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 2), bond(2, 3, 3)];

    let read = read_entangled(&state);
    assert_eq!(read.capacity, 0, "an unobserved pair is not a zero-valued relation");

    // Close the cycle with a non-composing direct obligation and the
    // structure appears — one independent cycle, capacity 1.
    state.bonds.push(bond(1, 3, 9));
    assert_eq!(read_entangled(&state).capacity, 1);
}

#[test]
fn obligations_that_do_not_compose_carry_strain() {
    // Same triangle, but 1→3 owes 9 where the path through 2 totals 5.
    // The obligation does NOT compose: an economic fact, not an error.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];

    let read = read_entangled(&state);
    assert!(read.defect > ratio(0, 1), "a non-composing triangle carries defect");

    // DIRECTIVE 03: the defect is a READING, never a gate. Capacity is
    // still measured, and non-composition ROOTS — it carries structure
    // and grants standing rather than being refused as ill-formed.
    assert_eq!(read.capacity, 1, "the strain IS the capacity, not a disqualification");
}

#[test]
fn opposed_obligations_net_rather_than_double_count() {
    // 1 owes 2 five, 2 owes 1 three: the relation carries 2, not both.
    let mut state = orbs(2);
    state.bonds = vec![bond(1, 2, 5), bond(2, 1, 3)];
    let delta = project(&state);
    assert_eq!(*delta.entry(0, 1, CHANNEL_BOND), ratio(2, 1));
}

#[test]
fn obligations_to_unknown_or_self_are_skipped_never_panic() {
    // Totality: the projection is pure and total like the judge it
    // serves. A bond naming an orb this book does not hold relates
    // nothing here; a self-bond would hit the zero diagonal, which
    // `set_antisym` refuses — so it must never reach it.
    let mut state = orbs(2);
    state.bonds = vec![bond(1, 99, 4), bond(1, 1, 7)];
    let read = read_entangled(&state);
    assert_eq!(read.capacity, 0);
    assert_eq!(read.defect, ratio(0, 1));
}

#[test]
fn the_projection_carries_every_channel() {
    // The fiber arity is the obligation channel — bond, covenant, debt.
    let delta = project(&orbs(2));
    assert_eq!(delta.fiber(), CHANNELS, "one coordinate per obligation channel");
    assert_eq!(delta.dim(), 2, "entities are orbs");
}
