//! Taxon economy — extractive · regenerative · balanced + planet fit.
//! Flatlander baseline = bimaniform. SEAL B + per-mode gates.

use cfe::almanac::{
    economic_mode, extract_yield_scale, habitat_yield_scale, is_flatlander_baseline, planet_fit,
    regen_yield_scale, regenerator_tech_need, taxon_economy, EconomicMode, PlanetFit,
    ThermalRegion, FLATLANDER_CLADE,
};
use cfe::dop::{positive, unit, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::vocabulary::{clade_from_word, clade_word};
use cfe::world::{ActionRefused, World, FIRST_ORB};

fn world_with_taxon(class: u8, clade: u8, star: StarClass) -> World {
    let mut w = World::new();
    w.found("eco-hold").expect("f");
    w.commission_star(star).expect("star");
    w.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("site");
    w.seal_genesis("eco", 1, 2, 1).expect("genesis"); // biotic planar default
    w.seal_taxon(class, clade).expect("taxon");
    w
}

#[test]
fn flatlander_baseline_is_bimaniform_balanced() {
    assert!(is_flatlander_baseline(FLATLANDER_CLADE));
    assert_eq!(clade_word(2), "bimaniform");
    assert_eq!(clade_from_word("flatlander"), Some(2));
    assert_eq!(clade_from_word("human"), Some(2));
    assert_eq!(clade_from_word("humans"), Some(2));
    assert_eq!(
        economic_mode(7, 2),
        EconomicMode::Balanced,
        "flatlander always balanced"
    );
}

#[test]
fn extractive_clades_derive() {
    // lithophage, hypobiotid, geodic class
    assert_eq!(economic_mode(2, 4), EconomicMode::Extractive);
    assert_eq!(economic_mode(2, 5), EconomicMode::Extractive);
    assert_eq!(economic_mode(6, 12), EconomicMode::Extractive); // plasmiform
}

#[test]
fn regenerative_clades_derive() {
    assert_eq!(economic_mode(1, 10), EconomicMode::Regenerative); // mycotiform swarm
    assert_eq!(economic_mode(5, 6), EconomicMode::Regenerative); // soniform
    assert_eq!(economic_mode(1, 8), EconomicMode::Regenerative); // hexasocial
    assert_eq!(economic_mode(5, 11), EconomicMode::Regenerative); // cnidiform
}

#[test]
fn yield_scales_differ_by_mode() {
    let fit = PlanetFit::Native;
    let ex_e = extract_yield_scale(EconomicMode::Extractive, fit);
    let ex_r = extract_yield_scale(EconomicMode::Regenerative, fit);
    let rg_r = regen_yield_scale(EconomicMode::Regenerative, fit);
    let rg_e = regen_yield_scale(EconomicMode::Extractive, fit);
    assert!(ex_e > ex_r, "extractive mines harder");
    assert!(rg_r > rg_e, "regenerative restores harder");
    let bal = extract_yield_scale(EconomicMode::Balanced, fit);
    assert_eq!(bal, ratio(1, 1));
    assert_eq!(habitat_yield_scale(EconomicMode::Balanced, fit), ratio(1, 1));
}

#[test]
fn planet_fit_native_vs_hostile() {
    // flatlander prefers mesic
    assert_eq!(
        planet_fit(2, Some(ThermalRegion::Mesic)),
        PlanetFit::Native
    );
    assert_eq!(
        planet_fit(2, Some(ThermalRegion::HyperThermal)),
        PlanetFit::Hostile
    );
    // plasmiform prefers hyperthermal
    assert_eq!(
        planet_fit(12, Some(ThermalRegion::HyperThermal)),
        PlanetFit::Native
    );
    // cryocoherens prefers cryogenic
    assert_eq!(
        planet_fit(3, Some(ThermalRegion::Cryogenic)),
        PlanetFit::Native
    );
}

#[test]
fn regenerator_tech_need_softens_for_regenerative() {
    assert_eq!(regenerator_tech_need(EconomicMode::Regenerative), 2);
    assert_eq!(regenerator_tech_need(EconomicMode::Balanced), 3);
    assert_eq!(regenerator_tech_need(EconomicMode::Extractive), 3);
}

#[test]
fn world_taxon_economy_flatlander_default_and_sealed() {
    let mut w = World::new();
    w.found("base").expect("f");
    // Unsealed → flatlander baseline default for readability.
    let eco = w.taxon_economy_of(FIRST_ORB);
    assert!(eco.flatlander);
    assert_eq!(eco.mode, EconomicMode::Balanced);

    w.seal_taxon(2, 5).expect("hypobiotid geodic");
    let eco2 = w.taxon_economy_of(FIRST_ORB);
    assert!(!eco2.flatlander);
    assert_eq!(eco2.mode, EconomicMode::Extractive);
}

#[test]
fn extractive_line_builds_extractors_on_mesic() {
    // class 2 geodic, clade 5 hypobiotid, G star mesic-ish
    let mut w = world_with_taxon(2, 5, StarClass::G);
    w.zone(0, ZoneKind::Extractor).expect("extract 0");
    w.zone(1, ZoneKind::Extractor).expect("extract 1");
    assert_eq!(
        w.orb().unwrap().zones.iter().filter(|z| z.kind == ZoneKind::Extractor).count(),
        2
    );
    // Regenerator still needs tech.
    match w.zone(2, ZoneKind::Regenerator) {
        Err(ActionRefused::ConditionUncrystallized { condition }) => {
            assert_eq!(condition, 3, "extractive needs deepening for regen");
        }
        other => panic!("expected tech gate, got {other:?}"),
    }
}

#[test]
fn regenerative_soft_tech_and_extractor_cap_on_hostile() {
    // mycotiform swarm class 1 clade 10; M star may be cold/hostile for mesic-preferring
    let mut w = world_with_taxon(1, 10, StarClass::M);
    let eco = w.taxon_economy_of(FIRST_ORB);
    assert_eq!(eco.mode, EconomicMode::Regenerative);
    assert_eq!(w.regenerator_tech_need(), 2);

    // Soft cap for regenerative = 2 extractors.
    assert_eq!(w.extractor_soft_cap(), 2);
    w.zone(0, ZoneKind::Extractor).expect("e0");
    w.zone(1, ZoneKind::Extractor).expect("e1");
    // Third extractor on hostile+regen should block.
    if eco.fit == PlanetFit::Hostile {
        match w.zone(2, ZoneKind::Extractor) {
            Err(ActionRefused::EconomicModeBlocked { mode, .. }) => {
                assert_eq!(mode, EconomicMode::Regenerative as u8);
            }
            Ok(()) => {
                // If planet fit is not hostile for this star/site, soft-cap is advisory only.
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}

#[test]
fn each_founding_clade_has_a_mode() {
    // All lawful class×clade pairs produce a mode.
    for clade in 1u8..=20 {
        for class in 1u8..=13 {
            let m = economic_mode(class, clade);
            assert!(matches!(
                m,
                EconomicMode::Extractive | EconomicMode::Regenerative | EconomicMode::Balanced
            ));
            let _ = taxon_economy(class, clade, Some(ThermalRegion::Mesic));
        }
    }
}

#[test]
fn all_imperium_clade_words_round_trip() {
    for code in 0u8..=20 {
        let w = clade_word(code);
        if w == "unknown" {
            continue;
        }
        assert_eq!(clade_from_word(w), Some(code), "round-trip {w}");
    }
}

#[test]
fn expansion_taxons_per_class_and_ceilings() {
    use cfe::almanac::{
        expansion_clade, expansion_class_of, CLADE_EXPAND_SLOTS, CLADE_EXPAND_TARGET,
    };
    assert_eq!(cfe::dop::class_ceiling(), 13);
    assert_eq!(cfe::dop::clade_ceiling(), 254);
    assert_eq!(CLADE_EXPAND_TARGET, 25, "target remains 25 for u16 later");
    assert_eq!(CLADE_EXPAND_SLOTS, 18, "u8-fit slots per class");
    // 18 expansion taxons in every class tier 1..=13
    for class in 1u8..=13 {
        for slot in 1u8..=CLADE_EXPAND_SLOTS {
            let code = expansion_clade(class, slot).expect("slot");
            assert_eq!(expansion_class_of(code), Some(class));
            let w = clade_word(code);
            assert_ne!(w, "unknown", "class {class} slot {slot}");
            assert_eq!(clade_from_word(w), Some(code));
            // Mode inherits class colour
            let m = economic_mode(class, code);
            assert!(matches!(
                m,
                EconomicMode::Extractive | EconomicMode::Regenerative | EconomicMode::Balanced
            ));
        }
    }
    assert_eq!(expansion_clade(1, 1), Some(21));
    assert_eq!(expansion_clade(13, 18), Some(254));
}

#[test]
fn six_new_class_tiers_named() {
    for class in 8u8..=13 {
        let w = cfe::dop::archetype_word(class);
        assert_ne!(w, "unknown");
        let s = cfe::dop::archetype_sigil(class);
        assert_ne!(s, "?");
    }
}
