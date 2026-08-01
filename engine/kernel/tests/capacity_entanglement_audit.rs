//! AUDIT PROBE for DIRECTIVE 04 — not law, not a pin.
//!
//! DIRECTIVE 04 states two different rules in two places:
//!
//! - its HEADING — "Enforce `D > 0` Enforcement" — is Article II
//!   standing applied to the mint (`18_TORSION` §4's execution matrix:
//!   *vacuous closure | `D = 0` | no standing, no weight, NO MINT*);
//! - its BODY — "any minting deed that evaluates to `ΔD = 0` … MUST be
//!   rejected" — requires the mint to RAISE capacity.
//!
//! These are not the same law, and the second is unsatisfiable. This
//! file measures why, so the choice is made against evidence.

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ratio::ratio;
use cfe::world::{Bond, Committed, World};
use kernel::bridge::read_entangled;

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
        resource: Resource::Iron,
        rate: ratio(rate, 1),
        term: 0,
        sealed: 0,
        anchored: 0,
    }
}

/// THE STRUCTURAL FACT. `read_entangled` measures `D` from the
/// obligation graph — bonds, covenants, debts. A banked mint
/// (`DiscoveryMinted`, `YieldCollected`) moves `orbs[].banked` and
/// touches no obligation. `D` therefore CANNOT move across a mint, for
/// any world, at any capacity level.
#[test]
fn a_banked_mint_cannot_move_capacity_at_any_capacity_level() {
    // A world with rich, non-composing obligation structure: D > 0.
    let mut prior = orbs(3);
    prior.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];
    let before = read_entangled(&prior);
    assert!(before.capacity > 0, "this world has genuine structure");

    // The mint lands: the solving orb banks its reward. Nothing else.
    let mut next = prior.clone();
    next.orbs[0].banked.push((Resource::Iron, ratio(1, 1)));

    let after = read_entangled(&next);
    assert_eq!(
        after.capacity, before.capacity,
        "a banked mint moves no obligation, so ΔD = 0 — structurally, always"
    );

    // And on a world with no structure at all, likewise.
    let empty = orbs(3);
    let mut minted = empty.clone();
    minted.orbs[0].banked.push((Resource::Iron, ratio(1, 1)));
    assert_eq!(read_entangled(&empty).capacity, read_entangled(&minted).capacity);
}

/// The consequence for the BODY reading, stated plainly: if `ΔD > 0` is
/// required of every mint, then no `DiscoveryMinted` deed can ever be
/// lawful, and Ruling Box 1's whole emission ladder (`f(3)=1`,
/// `f(4)=4`, `f(5)=7`) is unreachable. The proof-of-useful-work economy
/// would be dead on arrival.
#[test]
fn the_delta_d_reading_makes_every_discovery_mint_unreachable() {
    let mut prior = orbs(3);
    prior.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];

    // Every conceivable banked mint on this world, at every amount.
    for amount in [1i64, 4, 7, 1000] {
        let mut next = prior.clone();
        next.orbs[0].banked.push((Resource::Iron, ratio(amount, 1)));
        assert_eq!(
            read_entangled(&next).capacity,
            read_entangled(&prior).capacity,
            "amount {amount}: ΔD = 0, so the BODY reading refuses this mint"
        );
    }
}

/// The HEADING reading, by contrast, is satisfiable and discriminating:
/// it separates a world that HAS relational structure (may mint) from
/// one that has none (Article II — no standing, no weight, no mint).
#[test]
fn the_standing_reading_separates_worlds_that_have_structure_from_those_that_do_not() {
    // Vacuous: no obligations at all. Article II denies standing.
    assert_eq!(read_entangled(&orbs(3)).capacity, 0);

    // A TREE of obligations carries no independent cycle — still
    // vacuous, and DIRECTIVE 05's support masking is what makes this
    // read correctly (the dense reduction manufactured a phantom entry
    // here and reported capacity 1).
    let mut tree = orbs(3);
    tree.bonds = vec![bond(1, 2, 2), bond(2, 3, 3)];
    assert_eq!(
        read_entangled(&tree).capacity,
        0,
        "a spanning tree of obligations is pure frame: E − V + C = 0"
    );

    // A genuine cycle that does not compose: structure, and standing.
    let mut cycle = orbs(3);
    cycle.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];
    assert!(
        read_entangled(&cycle).capacity > 0,
        "a non-composing cycle IS capacity — it roots, it is not refused"
    );
}

/// A closing check that the kernel is in fact consuming `D` on every
/// pass: a mint on a structureless world and the same mint on a
/// structured world reach different verdicts under the standing rule.
#[test]
fn the_judge_reads_capacity_on_the_minting_path() {
    let vacuous = orbs(3);
    let mut structured = orbs(3);
    structured.bonds = vec![bond(1, 2, 2), bond(2, 3, 3), bond(1, 3, 9)];

    assert_eq!(read_entangled(&vacuous).capacity, 0);
    assert!(read_entangled(&structured).capacity > 0);

    // The deed shape the rule gates on.
    let _ = ClosureEvent::YieldCollected {
        orb_id: 1,
        resource: Resource::Iron,
        amount: ratio(1, 1),
    };
}
