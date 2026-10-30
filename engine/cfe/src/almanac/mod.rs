//! The almanac — the periodic law of elements, the locality a star and
//! a planet give it, the pressure that refines them, and the gems they
//! compound into.
//!
//! `14_ALMANAC.md` rules the grammar; `dop::element_properties` landed
//! its first half. This is the reading a refinement deed must be
//! written against, and it is **broken apart by concern so each axis
//! can widen without disturbing the others** (operator ruling
//! 2026-08-04):
//!
//!   [`periodic`]  the five axes — line, lane, shell, phase, spin
//!   [`locality`]  what a star makes reachable, and what pressure
//!                 refines
//!   [`compound`]  what elements make together, and the gem's identity
//!   [`science`]   civ utility · dark-7 gravity · isomorphic union
//!   [`thermal`]   the temperature line closed, and the orbit grain
//!
//! **Everything here is A derivation.** Pure, total, exact-rational, no
//! I/O and no deed. `element_properties`, `thermal::band_of` and
//! `site_planet` are all untouched, so nothing that has replayed
//! replays differently.

pub mod atom;
pub mod compound;
pub mod locality;
pub mod periodic;
pub mod rejuvenation;
pub mod science;
pub mod taxon_economy;
pub mod taxon_table;
pub mod thermal;

pub use compound::{compounds, gem_of, knowledge_of, recipe_of, Compoundable};
pub use locality::{
    civ_may_work, civ_reach, lines_of, local_to, pressure, refines, strata_lines, workable_in,
};
pub use periodic::{matter_law_digest, periodic_of, Periodic, CELLS, LANES, LINES, PHASES, SHELLS, SPINS};
pub use rejuvenation::{
    carrying_capacity, grained_logistic, habitability, habitability_flux, intrinsic_rate,
    morphodynamic_thermal, rejuvenated_richness, rejuvenation_quantum, substrate_viability,
    vein_forcing_injection, REGROWTH_GRAIN,
};
pub use science::{
    bill_crystal_portion, dark_utility, explodes, explosion_risk, explosion_threshold, isomorphic,
    iso_union_knowledge, iso_union_product, iso_union_recipe, lab_create_bill, lab_pole_card,
    lab_requires_off_planet, net_utility, reveals_hidden_negative, reveals_hidden_positive,
    science_seat_affinity, science_seat_bill, science_seat_cost_classes, science_seat_fee,
    science_seat_fee_for, seat_requires_off_planet, survival_utility, CivAxes, CraftCostClass,
    DarkPull, ExplosionKind, LabPath, LabPoleCard, ScienceDiscipline, SeatAffinity,
};
pub use taxon_economy::{
    clade_preferred_region, economic_mode, extract_yield_scale, extractor_soft_cap,
    flatlander_species, flatlander_word, habitat_yield_scale, is_flatlander_baseline, planet_fit,
    regen_yield_scale, regenerator_tech_need, research_affinity_mult, taxon_economy, EconomicMode,
    PlanetFit, TaxonEconomy, FLATLANDER_CLADE,
};
pub use taxon_table::{
    archetype_sigil as table_archetype_sigil, archetype_word as table_archetype_word,
    class_ceiling as table_class_ceiling, class_taxon_capacity, clade_ceiling as table_clade_ceiling,
    clade_from_word as table_clade_from_word, clade_word as table_clade_word, expansion_clade,
    expansion_class_of, expansion_slot_of, is_lawful_clade, office_seat_bonus, CLADE_EXPAND_SLOTS,
    CLADE_EXPAND_TARGET, CLADE_LEGACY_MAX,
};
pub use thermal::{
    ceiling4, grained_t_eq4, orbit_on_grain, region_of, workable_at, ThermalRegion, ORBIT_GRAIN,
};
