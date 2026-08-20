//! **SOC-4a — THE CASTE PARTITION.** Internal groupings self-organise to FILL the
//! residual: the unspanned deficit γ (`EconomicPeel.residual`) is the society's
//! unmet work, and it partitions its members into castes proportional to that γ,
//! by the SAME digest-seeded partition draw the zone workbench and the abundance
//! fold use. These witnesses pin the properties: a covered economy forms NO
//! caste; the deepest deficit draws the largest caste; and two societies with the
//! same residual but different histories organise DIFFERENTLY (emergence, not a
//! caste table). SOC-4b now takes the organ LIVE: `Polity::caste_partition`
//! reads the REAL peeled residual off the running beat, so the self-organisation
//! happens in the sim, not just over a fixture — pinned by the final witness.
//! (In `tests/`, off the tuned-constant ratchet.)

use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::FIRST_ORB;
use conductor::soc::{caste_census, caste_draw, caste_weights};
use conductor::{Policy, Polity, PolitySpec};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use num_bigint::BigInt;
use std::fs;
use std::path::PathBuf;

fn seed(n: u8) -> [u8; 32] {
    *blake3::hash(&[n]).as_bytes()
}

fn residual(axes: &[i64]) -> Vec<BigInt> {
    axes.iter().map(|a| BigInt::from(*a)).collect()
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("soc4-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn line(sigil: &str) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: format!("{sigil}-clan"),
            sigil: sigil.into(),
            species: "void-sleeper".into(),
            clade: "tardigrade".into(),
            motto: "we divide the work by the work".into(),
            disposition: "empiricist".into(),
            metabolism: "mineral".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hearth"),
        planet: format!("{sigil}-prime"),
        colony: format!("first-{sigil}"),
        policy: Policy::Steady,
        position: (ratio(3, 1), ratio(3, 1), ratio(3, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(0, 1)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    }
}

#[test]
fn a_covered_economy_forms_no_caste() {
    // γ = 0 on every axis ⇒ no unmet work ⇒ a homogeneous society, no caste.
    let covered = residual(&[0, 0, 0]);
    assert_eq!(caste_draw(&covered, seed(1)), None, "no deficit ⇒ no caste drawn");
    let census = caste_census(&covered, 100, seed(1));
    assert_eq!(census, vec![0, 0, 0], "a covered economy organises no caste");
}

#[test]
fn caste_weights_track_the_residual_magnitude() {
    // The weight of an axis is the MAGNITUDE of its unspanned deficit (sign-free —
    // an over- or under-shoot is unmet work either way).
    assert_eq!(caste_weights(&residual(&[5, 0, -3])), vec![5, 0, 3]);
}

#[test]
fn the_deepest_deficit_draws_the_largest_caste() {
    // A lopsided residual (axis 1 dominates) ⇒ the census concentrates there.
    let lopsided = residual(&[1, 40, 1]);
    let census = caste_census(&lopsided, 400, seed(7));
    assert!(census[1] > census[0], "the deepest deficit draws more than a shallow one");
    assert!(census[1] > census[2], "the deepest deficit draws the largest caste");
    // A covered axis (weight 0) can never seat a caste.
    let with_void = residual(&[0, 10, 0]);
    let c = caste_census(&with_void, 200, seed(7));
    assert_eq!(c[0], 0, "a covered axis seats no caste");
    assert_eq!(c[2], 0, "a covered axis seats no caste");
    assert_eq!(c[1], 200, "every member joins the one axis with unmet work");
}

#[test]
fn every_member_finds_a_caste_when_a_deficit_exists() {
    // With any unmet work, the partition is total — no member is casteless.
    let r = residual(&[3, 7, 2]);
    let census = caste_census(&r, 250, seed(3));
    assert_eq!(census.iter().sum::<u64>(), 250, "every member joins some caste");
}

#[test]
fn castes_self_organise_differently_on_history() {
    // EMERGENCE, NOT A TABLE: identical residual, different digests ⇒ different
    // caste censuses. If the census were a fixed quota it would not move.
    let r = residual(&[10, 11, 9]); // near-balanced so the draw is non-degenerate
    let a = caste_census(&r, 300, seed(11));
    let b = caste_census(&r, 300, seed(12));
    assert_ne!(a, b, "two histories organise the same deficit differently");
    // ...but each is still a total partition of its members.
    assert_eq!(a.iter().sum::<u64>(), 300);
    assert_eq!(b.iter().sum::<u64>(), 300);
}

/// **SOC-4b — THE LIVE PARTITION.** Drive a founding line and read the caste
/// census off its REAL peeled residual (`Polity::caste_partition`), not a
/// fixture. The organ is finally on the beat: a line with unmet work partitions
/// EVERY worker into a caste proportional to its own deficit, a covered one
/// forms none, and the read is pure over committed state (replay-exact).
#[test]
fn the_live_society_self_organises_over_its_real_deficit() {
    let root = scratch("live-caste");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut polity = Polity::genesis(line("clan"), &registry, &book);

    let mut organised = false;
    for _ in 1u32..=64 {
        let _ = polity.cycle(&book, &registry);
        let members = polity.world.labor_force(FIRST_ORB);
        if members == 0 {
            continue; // no mouths yet — nothing to organise
        }
        let census = polity.caste_partition();
        let seated: u64 = census.iter().sum();
        // BOUNDED: a caste is drawn from the workforce, never beyond it.
        assert!(seated <= members, "castes over-partition the workforce: {seated} > {members}");
        // DERIVED + REPLAY-EXACT: the same committed state reads the same census.
        assert_eq!(census, polity.caste_partition(), "the live census is a pure derived read");
        if seated > 0 {
            // An uncovered economy draws EVERY worker into a caste — the
            // partition fills the deficit, off the line's own history.
            assert_eq!(seated, members, "unmet work seats every worker in a caste");
            organised = true;
            break;
        }
    }
    assert!(organised, "a founding line with unmet work self-organises over its live residual");
}
