//! N-I7 — the look costs leisure.
//!
//! Word mint already pays; the LOOK (`think_task`) did not — unpriced speech.
//! This cut prices the look: a Fed line looks only if its orb leisure bank can
//! pay `observe_price`, pays it, then drains. An unaffordable look is FASTING
//! (rest). Pin G fires first (a Short line never asks the bank). The price
//! DERIVES from the seated cost law (`mint_cost([1]) × metabolic_exchange`), so
//! it is positive and needs no new table. `mint_loop_pins` / `seal_b_era_pins`
//! carry the "a rich Fed line still mints, and the debit replays" halves.

use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use conductor::{Polity, PolitySpec, Policy};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("n-i7-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn line(sigil: &str, metabolism: &str) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: format!("{sigil}-keeper"),
            sigil: sigil.into(),
            species: "void-sleeper".into(),
            clade: "tardigrade".into(),
            motto: "the look is not free".into(),
            disposition: "empiricist".into(),
            metabolism: metabolism.into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hearth"),
        planet: format!("{sigil}-prime"),
        colony: format!("first-{sigil}"),
        policy: Policy::Steady,
        position: (ratio(5, 1), ratio(5, 1), ratio(5, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(0, 1)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    }
}

#[test]
fn the_look_has_a_positive_leisure_price() {
    // Speech is not free: the observe price derives from the seated cost law and
    // is strictly positive (a single degree-1 observation still bills).
    assert!(
        Polity::observe_price() > ratio(0, 1),
        "the look costs leisure — mint_cost([1]) × metabolic_exchange > 0",
    );
}

#[test]
fn an_unfunded_line_cannot_afford_the_look() {
    // A genesis line has never run a surplus — its leisure bank is zero (floored
    // there), below the price. The gate FASTS: it cannot look until it earns
    // leisure. This is the metabolic clock on the LOOK, not just the mint.
    let root = scratch("unfunded");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let polity = Polity::genesis(line("hearth", "biotic"), &registry, &book);
    let price = Polity::observe_price();
    assert!(
        polity.world.leisure_bank_of(cfe::world::FIRST_ORB) < price,
        "an unfunded line's leisure ({}) is below the look price ({price}) — it fasts",
        polity.world.leisure_bank_of(cfe::world::FIRST_ORB),
    );
}

#[test]
fn a_line_that_banks_surplus_affords_the_look_and_thinks() {
    // Once a line runs a surplus, its leisure bank clears the price and the look
    // opens — it thinks (solves) rather than fasting forever. Brackets the gate
    // against the unfunded case above.
    let root = scratch("funded");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut polity = Polity::genesis(line("hearth", "biotic"), &registry, &book);
    for _ in 0..16 {
        let _ = polity.cycle(&book, &registry);
    }
    assert!(
        polity.solved > 0,
        "a line that banked surplus afforded its looks and thought (solved={})",
        polity.solved,
    );
}
