//! **SOC-5 — COLLECTIVE BENEFIT: THE GROUP CONVERGES ON A SHARED OBJECTIVE.**
//! SOC-4 partitioned the society into castes over its unmet work; SOC-5 makes
//! that self-organisation STEER the collective act. The abundance fold reads the
//! caste census's concentration (`caste_convergence` ∈ [0, L]) as the surplus's
//! DECISIVENESS: a workforce converged on one shared deficit lowers the `Hoard`
//! floor and COMMITS, a covered or scattered one rests. The tensions still set
//! the direction; the collective sets whether to act. Emergent (a fold of
//! distributed draws, no central plan) and residual-descending (a sharp deficit
//! focuses the surplus that closes it). These witnesses pin the organ and the
//! floor-lowering; the shared reach cancels in the draw. (In `tests/`, off the
//! tuned-constant ratchet.)

use conductor::civic::LATTICE;
use conductor::soc::{
    caste_convergence, investment_weights, resolve_investment, AlgebraicRank, Investment,
};

fn seed(n: u8) -> [u8; 32] {
    *blake3::hash(&[n]).as_bytes()
}

fn reach() -> AlgebraicRank {
    AlgebraicRank { components: 2, cycles: 1, relations: vec![] }
}

const HOARD: usize = 5;

#[test]
fn a_covered_economy_converges_on_nothing() {
    // No caste seated (the economy is covered) ⇒ zero convergence ⇒ the society
    // is at ease, nothing to rally around.
    assert_eq!(caste_convergence(&[0, 0, 0]), 0);
    assert_eq!(caste_convergence(&[]), 0);
}

#[test]
fn a_single_shared_deficit_is_full_convergence() {
    // Every worker drawn to one axis ⇒ the society is of one mind: convergence L.
    assert_eq!(caste_convergence(&[0, 250, 0]), LATTICE as u64);
    assert_eq!(caste_convergence(&[300]), LATTICE as u64);
}

#[test]
fn a_diffuse_spread_converges_only_partially() {
    // An even split over k axes ⇒ the dominant share is 1/k ⇒ convergence L/k:
    // organised, but with no single shared objective to commit to.
    let even3 = caste_convergence(&[100, 100, 100]);
    assert_eq!(even3, LATTICE as u64 / 3, "an even 3-way split converges to L/3");
    // A sharper concentration converges harder than a flatter one on the same
    // headcount — focus, not size.
    let sharp = caste_convergence(&[10, 280, 10]);
    assert!(sharp > even3, "a concentrated society converges harder than a diffuse one");
}

#[test]
fn convergence_lowers_the_hoard_floor_the_collective_commits() {
    // The floor a society at ease rests on (convergence 0) is the untilted
    // `dim·L`; as the collective converges, that floor drops — it banks less and
    // acts more. Full convergence zeroes it: a society of one mind never hoards.
    let at_ease = investment_weights(&reach(), 0, 0, 0, 0, 0, 0)[HOARD];
    let focused = investment_weights(&reach(), 0, 0, 0, 0, 0, LATTICE as u64 / 2)[HOARD];
    let unanimous = investment_weights(&reach(), 0, 0, 0, 0, 0, LATTICE as u64)[HOARD];
    assert!(at_ease > focused, "convergence lowers the rest floor");
    assert!(focused > unanimous, "sharper convergence lowers it further");
    assert_eq!(unanimous, 0, "a unanimous society commits its whole surplus");
}

#[test]
fn convergence_never_touches_the_tensioned_acts_only_the_floor() {
    // SOC-5 sets DECISIVENESS, never DIRECTION: the three tensioned weights are
    // identical whatever the convergence — only Hoard moves.
    let scattered = investment_weights(&reach(), 40, 20, 10, 0, 0, 0);
    let converged = investment_weights(&reach(), 40, 20, 10, 0, 0, LATTICE as u64 / 2);
    assert_eq!(scattered[0], converged[0], "Deepen unmoved by convergence");
    assert_eq!(scattered[1], converged[1], "Expand unmoved by convergence");
    assert_eq!(scattered[2], converged[2], "Synthesize unmoved by convergence");
    assert!(converged[HOARD] < scattered[HOARD], "only the floor moves");
}

#[test]
fn a_converged_society_hoards_less_than_a_scattered_one() {
    // THE COLLECTIVE BENEFIT, in the draw: with the SAME faint tensions, sweeping
    // histories, a fully-converged society reaches the Hoard rest far less often
    // than one at ease — its self-organisation carries it into action.
    let mut hoard_at_ease = 0;
    let mut hoard_converged = 0;
    for n in 0..96u8 {
        if resolve_investment(&reach(), 4, 4, 4, 0, 0, 0, seed(n)) == Investment::Hoard {
            hoard_at_ease += 1;
        }
        if resolve_investment(&reach(), 4, 4, 4, 0, 0, LATTICE as u64, seed(n)) == Investment::Hoard {
            hoard_converged += 1;
        }
    }
    assert!(
        hoard_converged < hoard_at_ease,
        "convergence pulls the collective out of rest: {hoard_converged} < {hoard_at_ease}"
    );
}
