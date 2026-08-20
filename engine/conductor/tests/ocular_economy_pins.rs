//! **OCULAR-1 — THE ECONOMY AS A CONNECTION (no scalar, no lattice).** doc 54.
//!
//! The society's deficit read as the `divergence` of an exact-rational economic
//! connection, not a `quantize`d SNF residual. These witnesses pin the two
//! properties the no-scalar law demands: the reading is the SIGNED, per-axis,
//! exact deficit (direction and magnitude both survive), and it is LOSS-FREE --
//! a deficit finer than the retired 1/64 lattice grain, which `quantize` rounded
//! to "covered", is seen here as an exact nonzero divergence. Additive; the live
//! path is unchanged this slice. (In `tests/`, off the tuned-constant ratchet.)

use cfe::ratio::{ratio, Ratio};
use conductor::ocular::{economy_connection, seen_deficit};
use rfa_core::divergence;

#[test]
fn the_deficit_is_the_signed_exact_divergence_per_axis() {
    // Three axes: axis 0 short (demand 3 > supply 1), axis 1 balanced, axis 2 in
    // surplus (supply 5 > demand 2). The reading keeps all three signs exactly.
    let demand = vec![ratio(3, 1), ratio(4, 1), ratio(2, 1)];
    let supply = vec![ratio(1, 1), ratio(4, 1), ratio(5, 1)];
    let seen = seen_deficit(&demand, &supply);
    assert_eq!(seen[0], ratio(2, 1), "a short axis reads a positive (egress) deficit");
    assert_eq!(seen[1], ratio(0, 1), "a balanced axis reads zero -- no need, no surplus");
    assert_eq!(seen[2], ratio(-3, 1), "a surplus axis reads a negative deficit, sign intact");
}

#[test]
fn a_sub_lattice_deficit_is_seen_not_rounded_away() {
    // demand - supply = 1/100 on axis 0. The retired quantize (round(r * 64))
    // would map this to 0 -- a phantom "covered" economy. The connection sees it
    // exactly: no grain, no loss.
    let demand = vec![ratio(1, 100)];
    let supply = vec![ratio(0, 1)];
    let seen = seen_deficit(&demand, &supply);
    assert_eq!(seen[0], ratio(1, 100), "the fine deficit survives exact");
    assert!(seen[0] > ratio(0, 1), "and is a real unmet need, not rounded to covered");
    // The old grain, for contrast: round(1/100 * 64) = round(0.64) = 1, or a
    // finer 1/1000 would round to 0. Either way the exact reading owes nothing to
    // the grain -- this value is neither.
    assert_ne!(seen[0], ratio(1, 64), "the reading is exact, not snapped to the lattice");
}

#[test]
fn the_books_close_conservation_holds() {
    // Antisymmetry gives sum of all divergences = 0: an egress anywhere implies
    // an ingress somewhere (the environment reservoir absorbs the balance). The
    // economy conserves -- no relation is all-source.
    let demand = vec![ratio(7, 2), ratio(1, 3), ratio(9, 4)];
    let supply = vec![ratio(1, 1), ratio(5, 3), ratio(1, 4)];
    let (delta, _support) = economy_connection(&demand, &supply);
    let div = divergence(&delta);
    let total: Ratio = div.iter().map(|node| node[0].clone()).fold(ratio(0, 1), |a, b| a + b);
    assert_eq!(total, ratio(0, 1), "the books close: sum of divergences is exactly zero");
}

#[test]
fn the_environment_carries_the_closing_balance() {
    // Node 0 (the environment reservoir) holds the negated net of every axis:
    // what the axes collectively demand of the world, the world sources.
    let demand = vec![ratio(3, 1), ratio(2, 1)];
    let supply = vec![ratio(1, 1), ratio(1, 1)];
    let (delta, _s) = economy_connection(&demand, &supply);
    let div = divergence(&delta);
    // axes read +2 and +1 (net demand); the environment reads -3, closing them.
    assert_eq!(div[1][0], ratio(2, 1));
    assert_eq!(div[2][0], ratio(1, 1));
    assert_eq!(div[0][0], ratio(-3, 1), "the reservoir sources exactly what the axes demand");
}

// --- OCULAR-2: the organ on the LIVE economy (still additive) ---

use cfe::thermal::StarClass;
use conductor::{Policy, Polity, PolitySpec};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ocular-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn line(sigil: &str) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: format!("{sigil}-eye"),
            sigil: sigil.into(),
            species: "void-sleeper".into(),
            clade: "tardigrade".into(),
            motto: "we see the exact lack".into(),
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
fn the_live_society_sees_its_deficit_as_an_exact_field() {
    // Drive a founding line and read what it sees through the ocular organ: an
    // exact-rational field over the resource axes, one entry per axis, carried
    // whole -- never summed to a magnitude, never rounded to a grain. Once the
    // line carries mouths, a positive (unmet) entry appears -- a real need the
    // society sees exactly. Additive: the live path is unchanged this slice.
    let root = scratch("live-field");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut polity = Polity::genesis(line("iris"), &registry, &book);

    let axes = conductor::civic::resource_axes().len();
    let mut saw_need = false;
    for _ in 1u32..=64 {
        let _ = polity.cycle(&book, &registry);
        let Some(field) = polity.ocular_deficit() else { continue };
        // The reading is a full field over the axes -- graded, not collapsed.
        assert_eq!(field.len(), axes, "the society sees one exact entry per axis");
        // A DERIVED read: the same committed state reads the same field.
        assert_eq!(polity.ocular_deficit(), Some(field.clone()), "the ocular read is pure");
        if field.iter().any(|g| *g > ratio(0, 1)) {
            saw_need = true;
            break;
        }
    }
    assert!(saw_need, "a founding line with mouths sees an unmet need in its exact field");
}
