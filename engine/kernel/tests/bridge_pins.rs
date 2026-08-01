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
use kernel::bridge::{
    fold_certified, fold_guard, observe, project, read_entangled, read_entangled_observed,
    refold, FoldGuard, Observed, CHANNELS, CHANNEL_BOND,
};
use rfa_core::{coboundary_reduce, Degree, Delta};

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
fn a_world_that_relates_nothing_stands_on_the_ground() {
    // Article II's floor case, measured. NOTHING CIRCULATES — rank 0,
    // correctly, because rank counts what stands ABOVE the floor.
    let read = read_entangled(&orbs(3));
    assert_eq!(read.rank, 0, "no obligations, nothing circulating");
    assert!(read.residuals.is_empty(), "nothing to fail to compose");
    assert!(read.rank_is_defined(), "a zero cocycle IS a cocycle");

    // AND IT HAS NOT VANISHED. `D` is the ELEVATION, and it reads the
    // GROUND — the world stands on the surface holding one degree,
    // itself. This is the reading Article II consults.
    assert_eq!(read.degree(), Degree::GROUND, "it stands ON the ground, not in it");
    assert_eq!(read.degree().get(), 1, "D reads ONE at the floor, never zero");
    assert!(read.degree().is_ground());
    assert!(!read.degree().is_above_ground());

    // A lone orb relates to nothing at all, lawfully — and still stands.
    let read = read_entangled(&orbs(1));
    assert_eq!(read.rank, 0);
    assert_eq!(read.degree(), Degree::GROUND, "one orb is still a universe that exists");
    assert!(read.rank_is_defined());

    // Even the EMPTY world stands. There is no world so small that its
    // degree falls through the floor.
    let read = read_entangled(&orbs(0));
    assert_eq!(read.degree(), Degree::GROUND, "the empty world stands on the ground");
}

/// **THE KERNEL-SIDE TRIPWIRE — `D` never falls through the floor on
/// any committed world.** The algebra's floor is pinned in
/// `rfa-core/tests/capacity_pins.rs`; this is the same law asserted
/// where the kernel actually reads it, across the shapes real play
/// produces: empty, isolated, trees, composing cycles, non-composing
/// cycles, and multi-cycle worlds.
#[test]
fn no_committed_world_ever_reads_below_the_ground() {
    let mut worlds: Vec<Committed> = Vec::new();
    for n in 0..5u64 {
        worlds.push(orbs(n)); // no obligations at all
    }
    // A tree of obligations — pure frame, nothing circulates.
    let mut tree = orbs(4);
    tree.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(3, 4, 5)];
    worlds.push(tree);
    // A cycle that composes — still frame.
    let mut composing = orbs(3);
    composing.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 5)];
    worlds.push(composing);
    // A cycle that does not — genuine structure, above the ground.
    let mut charged = orbs(3);
    charged.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];
    worlds.push(charged);
    // Two independent charged cycles.
    let mut two_cycles = orbs(4);
    two_cycles.bonds = vec![
        bond(1, 2, 2),
        bond(2, 3, 3),
        bond(1, 3, 9),
        bond(1, 4, 4),
        bond(2, 4, 11),
    ];
    worlds.push(two_cycles);

    for (n, state) in worlds.iter().enumerate() {
        let read = read_entangled(state);
        let degree = read.degree();
        assert!(degree >= Degree::GROUND, "world {n}: degree {degree} fell below the ground");
        assert!(degree.get() >= 1, "world {n}: D read zero");
        // The elevation partitions exactly, and `above()` is the rank.
        assert_ne!(degree.is_ground(), degree.is_above_ground(), "world {n}");
        assert_eq!(degree.above(), read.rank, "world {n}: above-ground IS the rank");
        assert_eq!(degree.get(), read.rank + 1, "world {n}: D = GROUND + circulation");
    }
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
    assert!(read.residuals.is_empty(), "5 = 2 + 3: the triangle composes");
    assert!(read.rank_is_defined(), "a cocycle has a class to reduce to");
    assert_eq!(read.rank, 0, "composing obligation is pure frame");
}

#[test]
fn obligations_that_do_not_compose_carry_strain() {
    // Same triangle, but 1→3 owes 9 where the path through 2 totals 5.
    // The obligation does NOT compose: an economic fact, not an error.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];

    let read = read_entangled(&state);
    assert!(!read.residuals.is_empty(), "a non-composing triangle carries strain");
    // NON-COMPOSITION DOES NOT BAR THE READ (operator ruling
    // 2026-08-01, the struck residual clause). The old law refused to
    // define capacity here, on the reasoning that Δ ∉ Z¹ leaves no
    // class to land in. But `holonomies` is TOTAL — it reduces against
    // a spanning forest and reads what each remaining edge carries —
    // and a triangle that does not compose is an economic fact, not an
    // error. Non-composition is the SIGNAL; the old clause silenced
    // exactly the states worth reading.
    assert!(
        read.rank_is_defined(),
        "a fully observed complex is readable whether or not it composes"
    );
    assert!(
        !read.residuals.is_empty(),
        "and the strain is carried WHOLE, indices intact — never summed away"
    );
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
    assert_eq!(read.rank, 0);
    assert!(read.residuals.is_empty());
}

#[test]
fn a_book_verifies_absence_over_its_own_orbs() {
    // Within one book the chronicle is COMPLETE over its own orbs, so
    // silence about a pair it holds is a verified Absent — a fact, not
    // an assumption. Present is the receipted obligation.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 4)];
    assert_eq!(observe(&state, 0, 1, CHANNEL_BOND), Observed::Present);
    assert_eq!(observe(&state, 1, 2, CHANNEL_BOND), Observed::Absent);
    // Beyond the book's own orbs nothing is claimed — never Absent.
    assert_eq!(observe(&state, 0, 9, CHANNEL_BOND), Observed::Unknown);
}

#[test]
fn an_unresolved_absence_refuses_capacity_unverified_absence_would_mint() {
    // THE FORGERY GATE. A mistaken absence creates a cycle that is not
    // there, and a phantom cycle carries phantom holonomy — capacity
    // that was never earned. So while any pair stands unobserved, the
    // capacity reading refuses, even though the observed part is a
    // perfectly good cocycle.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 5)];

    let seen = read_entangled(&state);
    assert_eq!(seen.unknown, 0, "one book sees all its own orbs");
    assert!(seen.rank_is_defined(), "fully observed cocycle");

    let unseen = read_entangled_observed(&state, 1);
    assert_eq!(unseen.residuals, seen.residuals, "the strain field is unchanged");
    assert!(
        !unseen.rank_is_defined(),
        "one unobserved pair is enough — absence must be verified, never assumed"
    );
}

#[test]
fn the_fold_unfolds_whole_a_blanket_not_a_crease() {
    // THE RE-VOLUMIZER. A crease in paper is permanent; a blanket keeps
    // its loft and unfolds whole. The certified fold keeps exactly what
    // it stripped, so the original returns byte-for-byte.
    let mut state = orbs(4);
    state.bonds = vec![bond(1, 2, 2), bond(2, 3, 7), bond(3, 4, 5), bond(1, 4, 3)];
    let whole = project(&state);

    let (core, cert) = fold_certified(&whole);
    let recovered = refold(&core, &cert);

    for i in 0..4 {
        for j in 0..4 {
            for l in 0..CHANNELS {
                assert_eq!(
                    recovered.entry(i, j, l),
                    whole.entry(i, j, l),
                    "the blanket unfolds whole at ({i},{j},{l}) — nothing creased away"
                );
            }
        }
    }
}

#[test]
fn what_the_fold_strips_is_exactly_frame_and_nothing_else() {
    // The stripped part must be a PURE COBOUNDARY — the one shape that
    // carries no structural content. If the fold ever took more than
    // this, it would be destroying capacity, which is the loss the
    // certificate exists to make impossible.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 2), bond(2, 3, 7), bond(1, 3, 3)];
    let whole = project(&state);
    let (core, cert) = fold_certified(&whole);

    // stripped = whole − core, rebuilt from the certificate alone.
    let mut stripped = Delta::zero(3, CHANNELS);
    for i in 0..3 {
        for j in (i + 1)..3 {
            let coords: Vec<_> = (0..CHANNELS)
                .map(|l| cert.potentials[j][l].clone() - cert.potentials[i][l].clone())
                .collect();
            stripped.set_antisym(i, j, coords);
        }
    }
    let reduced_strip = coboundary_reduce(&stripped);
    for i in 0..3 {
        for j in 0..3 {
            for l in 0..CHANNELS {
                assert_eq!(
                    *reduced_strip.entry(i, j, l),
                    ratio(0, 1),
                    "the stripped part is pure frame — it reduces to nothing"
                );
            }
        }
    }
    // And core + stripped is the whole again.
    assert_eq!(refold(&core, &cert), whole);
}

#[test]
fn a_corrected_observation_re_folds_rather_than_re_earning() {
    // The capacity-loss case the certificate closes. A pair mis-taken
    // as unrelated, then corrected: the fold re-runs over the corrected
    // relation and the recovered whole carries the new obligation. No
    // capacity had to be re-earned, because none was destroyed.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 2), bond(2, 3, 7)];
    let (core, cert) = fold_certified(&project(&state));
    assert_eq!(refold(&core, &cert), project(&state));

    // The telescope reaches further: 1→3 was there all along.
    state.bonds.push(bond(1, 3, 4));
    let corrected = project(&state);
    let (core2, cert2) = fold_certified(&corrected);
    assert_eq!(refold(&core2, &cert2), corrected, "the corrected fold is whole too");
    assert_ne!(core, core2, "the correction changed the core, as it must");
}

#[test]
fn a_universe_with_an_unclosed_obligation_ring_reads_capacity() {
    // LET THERE BE LIGHT. A real committed state, read through the
    // kernel, carrying non-zero D for the first time.
    //
    // Four orbs in a ring of standing bonds — 1→2→3→4→1 — and NO
    // chords: orb 1 and orb 3 have never dealt with each other, orb 2
    // and orb 4 likewise. Those absences are VERIFIED (one book sees
    // all its own orbs), so the ring leaves a genuine hole.
    //
    // No triple is a filled triangle, so there is no cocycle condition
    // to fail: the ring IS a cocycle. And walking the loop does not
    // return where it started, so it is NOT a coboundary. That is
    // non-trivial H¹ — capacity no change of frame can explain away.
    let mut state = orbs(4);
    state.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3)];

    let read = read_entangled(&state);
    assert!(read.residuals.is_empty(), "no filled triangle: nothing fails to compose");
    assert_eq!(read.unknown, 0, "one book verifies absence over its own orbs");
    assert!(read.rank_is_defined(), "a fully observed cocycle HAS a class");
    assert_eq!(read.rank, 1, "one independent cycle, carrying charge: D = 1");
}

#[test]
fn a_ring_that_closes_consistently_carries_no_capacity() {
    // The same ring, but the obligations balance around the loop, so
    // the walk returns to its origin. Structure is the FAILURE to
    // close, never the loop itself.
    let mut state = orbs(4);
    state.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(1, 4, 9)];

    let read = read_entangled(&state);
    assert!(read.rank_is_defined());
    assert_eq!(read.rank, 0, "3+3+3 = 9: the loop closes, and closure is frame");
}

#[test]
fn a_chord_fills_the_triangles_and_the_capacity_becomes_strain() {
    // Add the missing 1—3 relation to the ring. Now triangles ARE
    // filled, the cocycle condition applies, and what was clean
    // topology becomes measurable strain instead — the two readings are
    // the same fact seen through the support.
    let mut state = orbs(4);
    state.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3), bond(1, 3, 1)];

    let read = read_entangled(&state);
    assert!(!read.residuals.is_empty(), "the chord fills a triangle that does not compose");
    // The read STANDS (the struck residual clause, 2026-08-01). Filling
    // a triangle converts topology into strain; it does not make the
    // universe unreadable. Both readings are the same fact seen through
    // the support, and both are available.
    assert!(
        read.rank_is_defined(),
        "a filled triangle that fails to compose is still a readable universe"
    );
}

#[test]
fn a_new_orb_grows_the_base_and_never_the_capacity() {
    // GROWTH IS ALLOWED, NEVER FORCED (operator ruling 2026-08-01).
    // Chartering an orb grows the BASE — one more vertex, one more
    // place a relation could form — and moves capacity not at all. The
    // base expands; capacity is merely PERMITTED to follow.
    //
    // This is the landed chartering law read on the capacity axis:
    // ColonyCharted MOVES founding reserves between orbs and mints
    // nothing. A new orb arrives owing no one and owed by no one.
    let mut state = orbs(4);
    state.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3)];
    let before = read_entangled(&state);
    assert_eq!(before.rank, 1);

    state.orbs.push(ColonyOrb { orb_id: 5, name: "orb5".into(), ..Default::default() });
    let after = read_entangled(&state);

    assert_eq!(after.rank, before.rank, "a new vertex is not new capacity");
    assert!(after.rank_is_defined(), "and the reading stays defined");
    assert_eq!(
        observe(&state, 0, 4, CHANNEL_BOND),
        Observed::Absent,
        "the newcomer relates to no one, and that absence is VERIFIED"
    );
}

#[test]
fn sparsity_makes_relation_rare_and_a_rare_relation_reshapes() {
    // SPARSITY → RARITY → MORPHODYNAMISM.
    //
    // In a dense complex every pair already relates, so forming a
    // relation changes nothing and relations are worth nothing. Here
    // most pairs do NOT relate, so a relation is SCARCE — and a scarce
    // relation, when it forms, changes the SHAPE. That is where the
    // morphodynamics live: not in growth being pushed, but in each rare
    // relation being able to move the topology.
    let mut state = orbs(5);
    state.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3)];
    let before = read_entangled(&state);
    assert_eq!(before.rank, 1, "the ring; orb 5 stands apart");

    // Orb 5 forms two relations — deliberate deeds, each one rare.
    state.bonds.push(bond(5, 1, 3));
    state.bonds.push(bond(5, 2, 3));
    let after = read_entangled(&state);

    assert_eq!(after.rank, 2, "the rare relations closed a second cycle: the shape moved");
    assert!(after.rank > before.rank, "allowed to grow — because a deed formed it");
}

#[test]
fn reading_capacity_mints_nothing() {
    // The allowance is not an act. Reading D is a LENS (18 §3, Article
    // III's readings-never-objectives clause): it changes no bank, no
    // key, no obligation, and reading twice reads the same.
    let mut state = orbs(4);
    state.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3)];
    let before = state.clone();

    let once = read_entangled(&state);
    let twice = read_entangled(&state);

    assert_eq!(once.rank, twice.rank, "a reading is idempotent");
    assert_eq!(state, before, "and it moved nothing in the world");
}

#[test]
fn the_projection_carries_every_channel() {
    // The fiber arity is the obligation channel — bond, covenant, debt.
    let delta = project(&orbs(2));
    assert_eq!(delta.fiber(), CHANNELS, "one coordinate per obligation channel");
    assert_eq!(delta.dim(), 2, "entities are orbs");
}

#[test]
fn the_degree_rises_above_the_ground() {
    // THE MISSING HALF (operator ruling 2026-08-02: *this is basically a
    // constant — D = GROUND*).
    //
    // Every prior reading of `D` in this tree returned exactly
    // `Degree::GROUND`. The floor pins assert `>= GROUND`, the empty and
    // isolated worlds assert `== GROUND`, and no pin anywhere asserted
    // the reading MOVE. A meter that has only ever returned its floor is
    // indistinguishable from a constant, which is the GROUND fault one
    // level up: that one was declared and never computed, this one was
    // computed and never observed to change.
    //
    // The fixture that shows it was already written — the non-composing
    // triangle of `obligations_that_do_not_compose_carry_strain`. It
    // proved the strain and stopped short of the elevation.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];
    let read = read_entangled(&state);

    assert!(read.rank_is_defined(), "the complex is fully observed");
    assert!(
        read.rank > 0,
        "a non-composing triangle carries a cycle no frame explains — rank {} ",
        read.rank
    );

    let degree = read.degree();
    assert!(
        degree.is_above_ground(),
        "D MUST rise off the floor here: {degree} — otherwise the meter is a constant"
    );
    assert!(degree > Degree::GROUND, "strictly above, never merely at");
    assert_eq!(degree.above(), read.rank, "the elevation IS the circulation");
    assert_eq!(degree.get(), read.rank + 1, "D = GROUND + circulation, and GROUND is 1");

    // AND THE CONTRAST, in one place so the pair is legible: the same
    // three orbs with a COMPOSING triangle stay on the floor. The
    // difference is not the number of obligations — both worlds carry
    // three — it is whether they compose.
    let mut flat = orbs(3);
    flat.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 5)];
    let composed = read_entangled(&flat);
    assert_eq!(composed.rank, 0, "2 + 3 = 5 composes — pure frame");
    assert_eq!(
        composed.degree(),
        Degree::GROUND,
        "and a world of pure frame stands ON the ground, however many bonds it holds"
    );

    // The polytonic reading (EQUATIONS D1): each surviving grade is a
    // TONE. The flat world is single-toned at the point; the strained
    // world is polytonic — the chord it holds is what D counts.
    assert!(composed.degree().get() == 1, "the flat world is one tone");
    assert!(degree.get() > 1, "the strained world is polytonic");
}

// ─── THE FOLD GUARD — the occupancy floor on both offices ───────────

#[test]
fn a_total_fold_is_lawful_and_carries_nothing() {
    // The fold as built: the era archived WHOLE, the successor seated
    // fresh at GROUND. Nothing crosses, so nothing can be under-roomed
    // and nothing can be dropped.
    let mut prior = orbs(3);
    prior.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];
    assert!(read_entangled(&prior).degree().is_above_ground(), "the era had structure");

    let successor = orbs(0);
    let archived = vec![1, 2, 3];
    let verdict = fold_guard(&prior, &successor, &archived);
    assert_eq!(verdict, FoldGuard::Total, "a whole-era fold carries nothing");
    assert!(verdict.is_lawful());
    assert_eq!(
        read_entangled(&successor).degree(),
        Degree::GROUND,
        "and the fresh universe seats on the ground, correctly"
    );
}

#[test]
fn a_fold_that_drops_standing_without_account_refuses() {
    // V's half. An orb present before, absent after, and ABSENT FROM
    // THE ARCHIVE has simply ceased. Standing comes from admission and
    // admission is never revoked — a departure must be into the record,
    // never into nothing. I6 on the fold: no difference dies without
    // account.
    let prior = orbs(3);
    let successor = orbs(2); // orb 3 gone
    let verdict = fold_guard(&prior, &successor, &[]); // and unarchived
    assert_eq!(
        verdict,
        FoldGuard::StandingDroppedWithoutAccount { orbs: 1 },
        "an unarchived disappearance is unaccounted"
    );
    assert!(!verdict.is_lawful());

    // Archive it and the same fold is lawful — the orb left into the
    // record rather than out of existence.
    assert!(fold_guard(&prior, &successor, &[3]).is_lawful(), "sealed is accounted");
}

#[test]
fn a_carry_into_fewer_tones_refuses() {
    // D's half, and the reason this guard exists. A fold does not
    // shrink D — it seats a NEW store. So a SELECTIVE carry could land
    // structure in a universe with fewer tones than it occupies, with
    // no plane ever closed and the floor still breached.
    let mut prior = orbs(3);
    prior.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];
    let carried_rank = read_entangled(&prior).rank;
    assert!(carried_rank > 0, "the era carried real circulation");

    // The successor keeps all three orbs and DROPS their obligations —
    // the structure crossed, the room did not.
    let successor = orbs(3);
    assert_eq!(read_entangled(&successor).degree(), Degree::GROUND, "no tones in the successor");

    let verdict = fold_guard(&prior, &successor, &[]);
    assert_eq!(
        verdict,
        FoldGuard::RoomBelowOccupancy { carried_rank, successor_degree: 1 },
        "carried structure landing in fewer tones than it occupies must refuse"
    );
    assert!(!verdict.is_lawful());
}

#[test]
fn a_carry_that_keeps_its_tones_admits() {
    // The lawful carry: the structure crosses and its room crosses with
    // it. Both floors hold, so the fold admits.
    let mut prior = orbs(3);
    prior.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];

    let mut successor = orbs(3);
    successor.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];

    let verdict = fold_guard(&prior, &successor, &[]);
    assert_eq!(verdict, FoldGuard::CarriedWithinFloor);
    assert!(verdict.is_lawful(), "room carried with the structure it holds");
}
