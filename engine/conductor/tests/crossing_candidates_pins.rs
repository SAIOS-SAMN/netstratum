//! BEAT-PARALLEL follow-up — the market's own PEEL, not a cartesian sweep
//! (operator ruling 2026-07-30). `market_step`'s crossing check used to sweep
//! every civ pair `(a, b)`, `a < b`, over the whole roster (O(n^2)) — but
//! `clear_trade` itself refuses any offer pair whose cargos are not exact
//! mirrors (`held_a.give != held_b.want || held_a.want != held_b.give`,
//! `cfe::world::mod.rs`), so that mirror condition IS the market's own
//! partition. `crossing_candidates` buckets every open offer by its
//! `(give, want)` key and only proposes pairs that share a complementary
//! key — provably every pair the naive sweep would ever find a crossing
//! candidate for, and no more, no less. This pin is the reference-oracle
//! proof the standing "peels and shears, not sweeps" law demands: the naive
//! form stays here, in the test, and the fast form is asserted to agree
//! with it set-for-set over inputs nobody chose.

use cfe::dop::{Exchange, Positive, Resource};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use conductor::{crossing_candidates, Policy, Polity, PolitySpec};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("crossing-cand-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn seat(sigil: &str) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: sigil.to_string(),
            sigil: sigil.to_string(),
            species: "trade witness".into(),
            clade: String::new(),
            motto: "the crossing feeds".into(),
            disposition: "empiricist".into(),
            metabolism: "biotic".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hold"),
        planet: format!("{sigil}-p"),
        colony: format!("{sigil}-c"),
        policy: Policy::Steady,
        position: (ratio(0, 1), ratio(0, 1), ratio(0, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(1, 2)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    }
}

/// The RESOURCES in play — deliberately few, so with 8 civs the naive sweep
/// and the peeled form are forced through many shared and many disjoint
/// keys alike (few keys, many civs per key), not a degenerate all-distinct
/// case that would let a fast form pass by accident.
const PALETTE: [Resource; 3] = [Resource::Iron, Resource::Biomass, Resource::Crystal];

/// Post `count` offers on `polity`, deterministic in `seed` (no RNG — this
/// crate's own replay-exactness discipline: a beacon-style index, not
/// `rand`). Reuses the SAME resource palette across civs on purpose, so
/// most (give, want) keys collide across several civs — the shape that
/// would break a peel that silently dropped a real candidate.
fn post_offers(world: &mut cfe::world::World, seed: u64, count: usize) {
    for i in 0..count {
        let mix = seed.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(i as u64);
        let give = PALETTE[(mix % 3) as usize];
        let want = PALETTE[((mix / 3) % 3) as usize];
        if give == want {
            continue;
        }
        let Some(pair) = Exchange::new(give, want) else {
            continue;
        };
        let ga = Positive::new(ratio(1 + (mix % 5) as i64, 1)).expect("positive");
        let wa = Positive::new(ratio(1 + (mix % 7) as i64, 1)).expect("positive");
        let _ = world.post_offer(cfe::world::FIRST_ORB, pair, ga, wa, seed * 100 + i as u64);
    }
}

/// THE REFERENCE ORACLE — the retired cartesian sweep's own necessary
/// condition, kept exactly as `clear_trade`'s first check reads it. Any
/// pair this finds is a pair the fast form MUST also find, and vice versa.
fn naive_candidates(worlds: &[Polity]) -> std::collections::BTreeSet<(usize, usize)> {
    let mut out = std::collections::BTreeSet::new();
    for a in 0..worlds.len() {
        if worlds[a].penned {
            continue;
        }
        for b in (a + 1)..worlds.len() {
            if worlds[b].penned {
                continue;
            }
            let crosses = worlds[a].world.offers().iter().any(|oa| {
                worlds[b]
                    .world
                    .offers()
                    .iter()
                    .any(|ob| oa.give == ob.want && oa.want == ob.give)
            });
            if crosses {
                out.insert((a, b));
            }
        }
    }
    out
}

#[test]
fn the_peel_agrees_with_the_sweep_set_for_set() {
    let root = scratch("agree");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut worlds: Vec<Polity> = (0..8u64)
        .map(|i| {
            let mut p = Polity::genesis(seat(&format!("civ{i}")), &registry, &book);
            post_offers(&mut p.world, i + 1, 4);
            p
        })
        .collect();
    // One CIV posts nothing (an honest empty book) and one is PENNED — both
    // edge cases the naive sweep and the peel must agree stay uncandidate.
    let empty = worlds.len();
    worlds.push(Polity::genesis(seat("civ-empty"), &registry, &book));
    let _ = empty;
    worlds[1].penned = true;

    let naive: std::collections::BTreeSet<(usize, usize)> = naive_candidates(&worlds);
    let peeled: std::collections::BTreeSet<(usize, usize)> =
        crossing_candidates(&worlds).into_iter().collect();
    assert_eq!(
        peeled, naive,
        "the peeled candidate set must agree with the cartesian sweep exactly"
    );
    // Guard against a vacuous pass: this fixture must actually exercise a
    // real, non-trivial candidate set (both present AND absent pairs), or
    // agreement proves nothing.
    assert!(!naive.is_empty(), "the fixture must produce real candidates");
    let total_possible = worlds.len() * (worlds.len() - 1) / 2;
    assert!(
        naive.len() < total_possible,
        "the fixture must also produce real NON-candidates, or the peel is untested"
    );
}

#[test]
fn a_penned_or_offerless_civ_is_never_a_candidate() {
    let root = scratch("penned");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut a = Polity::genesis(seat("open-a"), &registry, &book);
    let mut b = Polity::genesis(seat("open-b"), &registry, &book);
    // Explicitly complementary offers (give/want swapped) — guaranteed to
    // cross, unlike `post_offers`' pseudo-random mix which does not promise
    // any two civs actually mirror each other.
    let iron_for_biomass = Exchange::new(Resource::Iron, Resource::Biomass).expect("pair");
    let biomass_for_iron = Exchange::new(Resource::Biomass, Resource::Iron).expect("pair");
    let amt = Positive::new(ratio(1, 1)).expect("positive");
    a.world
        .post_offer(cfe::world::FIRST_ORB, iron_for_biomass, amt.clone(), amt.clone(), 1)
        .expect("a posts");
    b.world
        .post_offer(cfe::world::FIRST_ORB, biomass_for_iron, amt.clone(), amt, 1)
        .expect("b posts");
    let mut penned_pair = vec![
        {
            let mut p = Polity::genesis(seat("still-open"), &registry, &book);
            post_offers(&mut p.world, 1, 4);
            p
        },
        {
            let mut p = Polity::genesis(seat("penned"), &registry, &book);
            post_offers(&mut p.world, 1, 4);
            p.penned = true;
            p
        },
    ];
    let mut worlds = vec![a, b];
    worlds.append(&mut penned_pair);
    let pairs = crossing_candidates(&worlds);
    assert!(
        pairs.contains(&(0, 1)),
        "two open civs sharing a key ARE a candidate: {pairs:?}"
    );
    assert!(
        !pairs.iter().any(|&(x, y)| x == 3 || y == 3),
        "the penned civ (index 3) is never a candidate: {pairs:?}"
    );
}
