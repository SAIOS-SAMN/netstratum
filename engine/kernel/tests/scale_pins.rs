//! SCALE — the readings at 10x and beyond, and where throughput ends.
//!
//! The four-orb ring proves the mechanism and says nothing about
//! behaviour at size. These pins carry the structure up an order of
//! magnitude and measure what actually changes.

use cfe::dop::ColonyOrb;
use cfe::ratio::ratio;
use cfe::world::{Bond, Committed, World};
use kernel::bridge::{project_complex, read_entangled};
use rfa_core::{dual_pole, holonomies, boundary_form, Vacuity};

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

/// A closed ring over orbs `1..=n`, every bond the same rate.
fn ring(n: u64, rate: i64) -> Committed {
    let mut state = orbs(n);
    state.bonds = (1..=n).map(|i| bond(i, (i % n) + 1, rate)).collect();
    state
}

/// Rank and dV read WITHOUT the cubic residual scan — the two poles
/// only. `read_entangled` also collects residuals, which is `O(n³·f)`;
/// these pins separate the two costs deliberately (see the throughput
/// pin).
fn poles(state: &Committed) -> (usize, usize, Vacuity) {
    let (delta, support) = project_complex(state);
    let rank = holonomies(&delta, &support).len();
    let form = boundary_form(&delta, &support);
    (rank, form.rank(), dual_pole(rank, &form))
}

#[test]
fn a_ring_of_forty_is_still_rank_one_size_does_not_make_structure() {
    // THE FINDING AT SCALE. A ring of any size has `E − V + C = 1`
    // independent cycle. Forty orbs, forty obligations, and the rank is
    // still ONE — and every orb still owes exactly what it is owed, so
    // it is still pure circulation and still Stasis.
    //
    // **Scale of circulation is not structure.** A bigger circle is a
    // circle. Rank comes from independent cycles, never from size, and
    // no amount of throughput around a loop changes the verdict.
    for n in [4u64, 10, 40, 100] {
        let (rank, flows, seen) = poles(&ring(n, 3));
        assert_eq!(rank, 1, "a ring of {n} carries exactly one cycle");
        assert_eq!(flows, 0, "and nets to zero at every one of {n} vertices");
        assert_eq!(seen, Vacuity::Stasis, "circulation at any size is still stasis");
    }
}

#[test]
fn rank_grows_with_independent_relations_never_with_size() {
    // Ten times the rank needs ten times the independent cycles, not
    // ten times the orbs. Each chord across the ring closes exactly one
    // more, so nine chords take rank from 1 to 10 — on the SAME forty
    // orbs that carried rank 1 above.
    let mut state = ring(40, 3);
    for k in 0..9u64 {
        state.bonds.push(bond(1, 4 + 4 * k, 1));
    }
    let (rank, flows, seen) = poles(&state);
    assert_eq!(rank, 10, "nine chords, nine more cycles: 10x the rank");
    assert!(flows > 0, "and the chords unbalance vertices: dV ≢ 0");
    assert_eq!(seen, Vacuity::Entangled, "structure AND gradient — admitted");
}

#[test]
fn ten_disjoint_rings_read_rank_ten_and_are_all_still_stasis() {
    // Parallel circulation scales the rank — `E − V + C` gives one per
    // component — and changes the verdict not at all. Ten separate
    // wheels turning is ten times the structure and zero times the
    // gradient.
    let mut state = orbs(40);
    for r in 0..10u64 {
        let base = r * 4 + 1;
        for i in 0..4u64 {
            state.bonds.push(bond(base + i, base + (i + 1) % 4, 3));
        }
    }
    let (rank, flows, seen) = poles(&state);
    assert_eq!(rank, 10, "one independent cycle per component");
    assert_eq!(flows, 0, "every wheel balanced");
    assert_eq!(seen, Vacuity::Stasis, "ten times the circulation is still circulation");
}

#[test]
fn permanence_once_entangled_further_relations_keep_it_admitted() {
    // D > 0 WITH gradient is the durable row: growing the complex from
    // an admitted state keeps it admitted, and the rank only rises.
    // Stasis is a state you leave once and do not fall back into by
    // adding structure.
    let mut state = ring(40, 3);
    state.bonds.push(bond(1, 7, 1));
    let (rank_at_entry, _, entered) = poles(&state);
    assert_eq!(entered, Vacuity::Entangled, "one asymmetric relation lifts stasis");

    let mut rank_was = rank_at_entry;
    for k in 0..8u64 {
        state.bonds.push(bond(2 + k, 20 + k, 1));
        let (rank, _, seen) = poles(&state);
        assert_eq!(seen, Vacuity::Entangled, "still admitted after {} additions", k + 1);
        assert!(rank >= rank_was, "and the rank never retreats");
        rank_was = rank;
    }
}

#[test]
fn throughput_the_pole_reading_is_quadratic_and_scales() {
    // The two-pole reading is `O(n²·f)` — a forest walk plus a scan of
    // the remaining edges. Four hundred orbs and four hundred
    // obligations complete comfortably, which is 100x the §8 ring.
    let (rank, flows, seen) = poles(&ring(400, 3));
    assert_eq!(rank, 1);
    assert_eq!(flows, 0);
    assert_eq!(seen, Vacuity::Stasis);
}

#[test]
fn throughput_the_residual_scan_is_the_ceiling_and_it_is_cubic() {
    // THE LIMIT, named rather than hidden. `read_entangled` also
    // collects the residual field via `for_each_residual`, which visits
    // every ordered distinct TRIPLE: `O(n³·f)`. At forty orbs that is
    // ~187k visits and fine; at four hundred it is ~191 MILLION and is
    // not.
    //
    // So the two costs must not be conflated. The POLES scale
    // quadratically and are what Articles II and IV need. The RESIDUAL
    // FIELD is cubic and is what emission needs. Anything wanting the
    // second at scale needs the triple scan restricted to filled
    // triangles at enumeration time — the support already knows which
    // those are, and it is not used to prune yet.
    let read = read_entangled(&ring(40, 3));
    assert_eq!(read.rank, 1, "forty orbs through the full read");
    assert!(read.residuals.is_empty(), "a ring fills no triangle");
    assert!(read.rank_is_defined());
}
