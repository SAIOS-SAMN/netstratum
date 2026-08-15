//! **THE PROVISIONING LAYER** (`43_DISTRIBUTION`, task #45+).
//!
//! The granary is the infrastructure that turns the #40–42 subsistence
//! ledger into an economy. It holds no stock of its own — the depot IS
//! the orb's banked ration (ruled 2026-08-13) — so this layer stores
//! nothing new and mints no deed: everything it adds (the stratified
//! demand, the leisure tier, the stability reading, the disposition
//! factors) is DERIVED at the collect touch, the `regrown_ration`
//! precedent. This module holds the readings the consumption cycle (L2+)
//! consults; L1 is the gate itself.

use super::*;
use crate::dop::{metabolic_leisure, Caste, Ration, Resource, ZoneKind};

impl World {
    /// **THE MICRONUTRIENT FACTOR** (G2 trace elements, ruled 2026-08-14)
    /// — how a line's HARVEST is modulated by the trace elements its own
    /// world offers. The requirement is the PLANET's: the distinct
    /// elements its prospected veins bear ("you eat what your world
    /// grows"). Of those, the line thrives on the ones it actually holds
    /// (a positive bank, mined or traded in) and sickens on the ones it
    /// lacks. SOFT and graded, never a freeze:
    ///
    ///   factor = 1 + (present − absent) / (2 × required)
    ///          = 1 + (2·present − required) / (2·required)     ∈ [½, 3/2]
    ///
    /// Full coverage lifts the harvest by half; a bare larder halves it;
    /// the penalty DEEPENS as more of the world's elements go unmet. A
    /// planet with NO prospected veins bears no requirement, so the factor
    /// is exactly one and every world that never opened a vein is
    /// byte-identical. Derived from vein/bank counts — no dial.
    /// **THE SUBSTRATE'S TRACE SIGNATURE** (G2 trace elements §2,
    /// 2026-08-14) — does this line's chemistry favour `element`? Each
    /// substrate values a different slice of the periodic spine, DERIVED
    /// (never a species table) from the clade's own thermal preference
    /// against the element's family. The spine of four splits at its
    /// median: light families {1,2} vs heavy {3,4}. A HOT-preferring
    /// clade's chemistry is built of and maintained by HEAVY elements
    /// (the plasmic/saurian lines that prefer the torrid bands); a
    /// COLD-preferring one by LIGHT elements (the cryo lines); a temperate
    /// clade takes the whole spine. No star → no family reading → the line
    /// favours all (byte-identical to the pre-signature law).
    pub fn substrate_favours(&self, element: &[u8; 32]) -> bool {
        let Some(class) = self.star else {
            return true;
        };
        let pref = crate::almanac::clade_preferred_region(self.rejuvenation_clade()).flux_level()
            as i64;
        let mesic = crate::almanac::ThermalRegion::Mesic.flux_level() as i64;
        let (family, _, _) = crate::dop::element_properties(element, class);
        let clade_warmth = pref - mesic; // + hot, − cold, 0 temperate
        let element_heat = 2 * family as i64 - 5; // + heavy, − light (spine of 4)
        clade_warmth == 0 || (clade_warmth > 0) == (element_heat > 0)
    }

    /// **THE HARVESTING CATALYST FACTOR** (G2 trace elements §3,
    /// 2026-08-14) — elements are not only eaten, they are TOOLS: a line's
    /// favoured elements flux its EXTRACTION, the reagents that let it mine
    /// more matter per beat. Unlike the micronutrient maintenance need
    /// this is BOOST-ONLY (a reagent lifts extraction where held; its
    /// absence is merely baseline, never a penalty), and it lands on the
    /// EXTRACTOR line alone — food and industry read it not.
    ///
    ///   factor = 1 + present / required                       ∈ [1, 2]
    ///
    /// A full reagent stock doubles extraction; a bare one leaves it at
    /// baseline. The reagents are spent by the same trace vent that eats
    /// them, so a catalysing line is under the same re-mine pressure. A
    /// world with no favoured veins reads 1 — its extraction is untouched
    /// and byte-identical. Derived from vein/bank counts, no dial.
    pub fn catalyst_factor(&self, orb: u64) -> Ratio {
        let full = ratio(1, 1);
        let required = self.required_trace(orb);
        if required.is_empty() {
            return full;
        }
        let Some(held) = self.orb_by_id(orb) else {
            return full;
        };
        let present = required
            .iter()
            .filter(|element| held.element_amount(element) > ratio(0, 1))
            .count() as i64;
        let req = required.len() as i64;
        full + ratio(present, req)
    }

    /// **THE LINE'S TRACE REQUIREMENT** — the distinct elements its world's
    /// veins bear THAT its substrate favours (the intersection of "you eat
    /// what your world grows" with the substrate's own signature). A
    /// world with no matching veins bears no trace need.
    pub fn required_trace(&self, orb: u64) -> Vec<[u8; 32]> {
        let mut required: Vec<[u8; 32]> = self
            .prospects
            .iter()
            .filter(|vein| vein.orb == orb)
            .map(|vein| vein.element)
            .filter(|element| self.substrate_favours(element))
            .collect();
        required.sort();
        required.dedup();
        required
    }

    pub fn micronutrient_factor(&self, orb: u64) -> Ratio {
        let full = ratio(1, 1);
        // The world's trace elements this substrate favours (§2).
        let required = self.required_trace(orb);
        if required.is_empty() {
            return full;
        }
        let Some(held) = self.orb_by_id(orb) else {
            return full;
        };
        let present = required
            .iter()
            .filter(|element| held.element_amount(element) > ratio(0, 1))
            .count() as i64;
        let req = required.len() as i64;
        // (present − absent)/(2·req) = (2·present − req)/(2·req). Non-literal
        // ratio (counts), so no tuned constant enters the tree.
        full + ratio(present + present - req, req + req)
    }

    /// **THE UNIVERSAL TRACE-CONSUMPTION VENT** (G2 trace elements, ruled
    /// 2026-08-14) — the trace elements a POPULATION burns maintaining its
    /// substrate over the span since the last touch, whatever its clade.
    /// The requirement is the world's (the distinct elements its veins
    /// bear); of those, the line burns a TRACE of each it actually holds
    /// — the demand scaled to population × the span at trace proportions
    /// (`(p·Δt)/grain` per element, `≪` the gross caloric ration), capped
    /// by what is banked so the larder never goes negative. Returned as
    /// `(element, amount)` draws to receipt as `TraceConsumed` and debit.
    ///
    /// A world with no veins, no population, or an empty larder burns
    /// nothing — so every world without the trace economy is byte-identical.
    /// The DEPLETION is the point: as the larder empties the
    /// `micronutrient_factor` sinks toward its ½ floor, pressuring the line
    /// to re-mine or trade — a perpetual, organic elemental demand.
    pub fn trace_consumption_at_touch(&self, orb: u64, stamp: u64) -> Vec<([u8; 32], Ratio)> {
        let pop = self.population_of(orb);
        if pop == 0 {
            return Vec::new();
        }
        let Some(held) = self.orb_by_id(orb) else {
            return Vec::new();
        };
        let dt = stamp.saturating_sub(held.last_touch);
        if dt == 0 {
            return Vec::new();
        }
        // Trace proportion: 1/grain of an element per head per beat — far
        // under the gross caloric ration. Derived (grain is structural).
        let grain = crate::almanac::REGROWTH_GRAIN;
        let demand = ratio(pop as i64 * dt as i64, grain);
        if demand <= ratio(0, 1) {
            return Vec::new();
        }
        // Only the elements this substrate's signature favours (§2).
        self.required_trace(orb)
            .into_iter()
            .filter_map(|element| {
                let draw = held.element_amount(&element).min(demand.clone());
                (draw > ratio(0, 1)).then_some((element, draw))
            })
            .collect()
    }

    /// **DOES THIS ORB STAND A GRANARY?** The gate that flips the
    /// provisioned economy on. Where it answers false, the population
    /// runs the #40–42 subsistence model verbatim (harvest→stomach, no
    /// leisure, no stability, byte-identical replay); where true, the
    /// stratified consumption cycle and its disposition factors come into
    /// force. An ACTIVE granary only — one that cannot pay its own upkeep
    /// is dark, and a dark provisioning node provisions nothing.
    pub fn has_granary(&self, orb: u64) -> bool {
        self.granary_footprint(orb) > 0
    }

    /// The count of ACTIVE granaries standing on an orb — the provisioning
    /// footprint that lifts the POPULATION ceiling (§2), distinct from the
    /// ecological footprint (`living_zones_of`) that lifts the ration's.
    pub fn granary_footprint(&self, orb: u64) -> u64 {
        self.orb_by_id(orb)
            .map(|held| {
                held.zones
                    .iter()
                    .filter(|zone| zone.kind == ZoneKind::Granary && held.zone_active(zone))
                    .count() as u64
            })
            .unwrap_or(0)
    }

    /// **THE POPULATION CEILING `K_pop`** (§2) — the ecological footprint
    /// PLUS the provisioning footprint PLUS the evolutionary stage, read by
    /// the sitting clade at its place. A granary is provisioning capacity;
    /// an advanced STAGE (`44_LINEAGES` §2c) is earned capacity — an
    /// evolved civilization sustains more people per world. Never the food's
    /// own regrowth ceiling, which stays ecological. With no granary and no
    /// advancement the footprint is exactly `living_zones_of`, so the
    /// ceiling — and every #40–42 world — is byte-identical.
    /// **THE CONTINUOUS INSOLATION AT THIS ORB** `I = L·(1−A)/r²` (COG Slice
    /// 2b) — the absorbed stellar flux, read NATIVELY from the star's exact
    /// luminosity and the planet's albedo/orbit, before any thermal bucket.
    /// Zero for an unsited orb or a starless system.
    pub fn insolation_of(&self, orb: u64) -> Ratio {
        let Some(class) = self.star else {
            return ratio(0, 1);
        };
        self.orb_by_id(orb)
            .and_then(|held| held.thermal.as_ref())
            .map(|site| {
                crate::thermal::insolation(
                    &class.luminosity(),
                    site.albedo.value(),
                    site.orbit_r.value(),
                )
            })
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// The insolation NORMALIZED to a reference G-type star at unit orbit and
    /// zero albedo (`I / L_G`), so a Sol-like seat reads ≈1 and the star's
    /// class is a pure multiplier: an M-dwarf seat reads well below one, an
    /// O-giant seat far above. Dimensionless, exact.
    pub fn insolation_norm(&self, orb: u64) -> Ratio {
        let reference = crate::thermal::StarClass::G.luminosity();
        if reference <= ratio(0, 1) {
            return ratio(0, 1);
        }
        self.insolation_of(orb) / reference
    }

    /// A stable per-orb seed for the planet's hash-derived geometry — the
    /// FOUNDING digest (immutable after founding) folded with the orb id, so
    /// each world's planets carry distinct area yet every read and every
    /// replay reproduces it bit-for-bit. Not the terminal digest (which moves
    /// each beat); the founding receipt's own hash, fixed for the world's life.
    fn planet_geometry_seed(&self, orb: u64, salt: &[u8]) -> u64 {
        let founding = self
            .chronicle
            .receipts()
            .first()
            .map(|receipt| receipt.own.0)
            .unwrap_or([0u8; 32]);
        let mut hasher = blake3::Hasher::new();
        hasher.update(salt);
        hasher.update(&founding);
        hasher.update(&orb.to_le_bytes());
        u64::from_le_bytes(hasher.finalize().as_bytes()[..8].try_into().expect("eight bytes"))
    }

    /// **THE PLANET'S SURFACE AREA** (COG Slice 2b) — the total geometric
    /// capacity of the manifold (the `4πR²` of the sphere, held directly as
    /// an exact rational in area units so no irrational `π` enters). Hash-
    /// derived from the founding digest, so a system's worlds range from
    /// small rocks to broad giants deterministically. In `[8, 40)` units.
    pub fn surface_area_of(&self, orb: u64) -> Ratio {
        ratio(8 + (self.planet_geometry_seed(orb, b"netstratum.planet.area.v1") % 32) as i64, 1)
    }

    /// **THE ARABLE RATIO** `ρ` (COG Slice 2b) — the fraction of the surface
    /// that is chemically and topologically viable for primary production
    /// (the hydrology/nutrient face of the world). Hash-derived in
    /// `(1/16, 15/16)`, so no world is wholly barren or wholly fertile.
    pub fn arable_ratio_of(&self, orb: u64) -> Ratio {
        ratio(1 + (self.planet_geometry_seed(orb, b"netstratum.planet.arable.v1") % 15) as i64, 16)
    }

    /// **THE ARABLE AREA** `Area × ρ` — the growing surface primary
    /// production actually runs over, the area term of the `P_primary` law.
    pub fn arable_area_of(&self, orb: u64) -> Ratio {
        self.surface_area_of(orb) * self.arable_ratio_of(orb)
    }

    /// **THE PRIMARY PRODUCTION POTENTIAL** (COG Slice 2b/2c) — the absolute
    /// thermodynamic food FLOW the star could grow over this world's arable
    /// surface for THIS clade:
    ///
    /// ```text
    ///   P_primary = I_norm × (Area × ρ) × η × REGROWTH_GRAIN
    /// ```
    ///
    /// the star's raw insolation `I` delivered over the arable surface, gated
    /// by the sitting clade's biological efficiency `η = habitability` — the
    /// crucial shift: habitability is not a cap on capacity, it is the
    /// fraction of the raw starlight THIS species can convert. A clade matched
    /// to its band reads η=1 and drinks the planet's full light; a mismatched
    /// one (a flatlander on a scorched rock, a plasmoid on a temperate world)
    /// reads η→0 and starves under the same sun. This is the CEILING the
    /// syntropic and cultivated flows draw from — never itself the population
    /// ceiling.
    pub fn primary_production(&self, orb: u64) -> Ratio {
        let Some(region) = self.orb_region(orb) else {
            return ratio(0, 1);
        };
        let eta = crate::almanac::habitability(self.rejuvenation_clade(), region);
        if eta <= ratio(0, 1) {
            return ratio(0, 1);
        }
        self.insolation_norm(orb)
            * self.arable_area_of(orb)
            * eta
            * ratio(crate::almanac::REGROWTH_GRAIN, 1)
    }

    /// **THE SYNTROPIC BASELINE FLOW** (COG Slice 2c, operator ruling
    /// 2026-08-15) — the WILD share of primary production the planet grows
    /// with NO infrastructure and NO labor, driven purely by physics (the
    /// star pumps it in). Because it is syntropic it can never sit at zero on
    /// a lit, habitable world: a stock stripped bare regenerates to this
    /// flow. A genesis band subsists on exactly this — gatherers on the wild
    /// baseline — and it is the floor the ambient ration stock recovers to.
    pub fn syntropic_flow(&self, orb: u64) -> Ratio {
        // A quarter of the potential is the untended wild yield; the other
        // three-quarters is unlocked only by cultivation (operator ruling
        // 2026-08-15). The marginal habitable band already reads a quarter
        // efficiency (`habitability` = 1/2² at the two-band edge), so a
        // forager on the wild fringe gathering a quarter is the same law.
        self.primary_production(orb) * ratio(1, 4) // APPROVED-SEED
    }

    /// **THE CULTIVATED YIELD** (COG Slice 2c) — the food FLOW extracted
    /// ABOVE the wild baseline by infrastructure the civilization has RAISED:
    /// its ration-producing zones (each scaled by its vigour), its granaries,
    /// its earned evolutionary stage. Zero for a band that has built nothing
    /// — the gatherers. This is the flow the Mind unlocks when starvation
    /// forces it to invent cultivation, and it grows `K` past the wild limit.
    /// Polymorphic: the ration is the sitting species' own metabolic carrier
    /// (biomass, iron, …), so every clade cultivates its own energy.
    pub fn cultivated_yield(&self, orb: u64) -> Ratio {
        let Ration::Resource(ration) = self.ration_of(orb) else {
            return ratio(0, 1); // element-eaters ride the element economy (#29)
        };
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let Some(held) = self.orb_by_id(orb) else {
            return ratio(0, 1);
        };
        let zone_flow: Ratio = held
            .zones
            .iter()
            .filter(|zone| crate::dop::zone_yield(zone.kind, metabolism).0 == ration)
            .map(|zone| crate::dop::zone_yield(zone.kind, metabolism).1 * held.zone_vigour(zone))
            .fold(ratio(0, 1), |sum, yielded| sum + yielded);
        let built = ratio(
            (self.granary_footprint(orb) + self.evolution_stage() as u64) as i64,
            1,
        );
        zone_flow + built
    }

    /// **THE DYNAMIC POPULATION CEILING `K_pop`** (§2, COG Slice 2c) — no
    /// longer a fixed planetary limit but the SUM of the wild and cultivated
    /// food flows over the metabolic draw:
    ///
    /// ```text
    ///   K = (Syntropic Flow + Cultivated Yield) / per_capita_ration
    /// ```
    ///
    /// A genesis band with no infrastructure is bounded by the syntropic flow
    /// alone — it stabilizes at the wild limit, generating a standing
    /// deficit. As the Mind invents cultivation and raises farms, the
    /// cultivated yield lifts `K`, and the population climbs toward the
    /// planet's full primary-production potential. The "habitable zone" is
    /// thus RELATIONAL and the ceiling DYNAMIC — earned, not merely sited.
    pub fn population_ceiling(&self, orb: u64) -> Ratio {
        // A rate-0 capture line (plasmoid) has no ration-based population —
        // it rides the energy economy (#29 deferred); no metabolic draw.
        let draw = self.per_capita_ration();
        if draw <= ratio(0, 1) {
            return ratio(0, 1);
        }
        let flow = self.syntropic_flow(orb) + self.cultivated_yield(orb);
        (flow / draw)
            * self.cooling_k_factor(orb)
            * self.footprint_permanence_factor(orb)
    }

    /// **THE EVOLUTIONARY STAGE** (`44_LINEAGES` §2c) — how far this line
    /// has advanced along its arc. 0 at founding; raised only by the gated,
    /// earned `EvolutionAdvanced` deed.
    pub fn evolution_stage(&self) -> u8 {
        self.evolution_stage
    }

    /// **DOES THIS LINE'S DISPOSITION CARRY A TRAIT?** (`44_LINEAGES`) — the
    /// generic reader that replaces the scattered `disposition == N` checks.
    /// A seeded disposition carries the traits its table declares; a MINTED
    /// disposition inherits the traits of its BASE (the seeded disposition
    /// its mint modulates), read from the fold — so freight-affinity,
    /// novelty-hunger, and vanity are no longer stuck to three codes but flow
    /// to any line that IS, or descends from, one that bears them.
    pub fn disposition_carries(&self, wanted: crate::dop::DispositionTrait) -> bool {
        let Some(code) = self.civ_axes().map(|axes| axes.disposition) else {
            return false;
        };
        if crate::dop::disposition_traits(code).contains(&wanted) {
            return true;
        }
        // A minted disposition inherits its base's traits. FORWARD-COMPAT:
        // the mint law currently binds `base` to the Zone axis only
        // (`BaseOffAxis`), so a disposition entry carries `None` today and
        // this path is inert; the moment base-inheritance is chartered for
        // dispositions, minted lines inherit their base's traits with no
        // further change here.
        if code > 3 {
            if let Some(base) = crate::vocabulary::fold(self.chronicle(), crate::vocabulary::Axis::Disposition)
                .into_iter()
                .find(|entry| entry.code == code)
                .and_then(|entry| entry.base)
            {
                return crate::dop::disposition_traits(base).contains(&wanted);
            }
        }
        false
    }

    /// **THE STELLAR FLUX AN ORB BATHES IN** (`44_LINEAGES` §2b) — the flux
    /// level of its thermal band (0 coldest … 7 hottest). A pure reading.
    pub fn stellar_flux_of(&self, orb: u64) -> u8 {
        self.orb_region(orb).map(|region| region.flux_level()).unwrap_or(0)
    }

    /// **IS THE SITTING CLADE FLUX-DRIVEN?** (`44_LINEAGES` §2b) — a line
    /// whose HOME band is hot (Torrid or hotter) is adapted to run on stellar
    /// flux: its output rises in heat and falls in cold. DERIVED from the
    /// clade's own preferred region (no new map) — the saurian Scale-Matrix
    /// and the plasmoid Vapor-Stalker are flux-driven because their clades
    /// prefer the hot bands; a mesic clade is not.
    pub fn flux_affine(&self) -> bool {
        use crate::almanac::ThermalRegion;
        crate::almanac::clade_preferred_region(self.rejuvenation_clade()) >= ThermalRegion::Torrid
    }

    /// **THE FLUX OUTPUT FACTOR** (`44_LINEAGES` §2b) — a flux-driven line's
    /// output scales with the flux it stands in, relative to the temperate
    /// baseline (Mesic): hot bands lift it, cold bands starve it. A line that
    /// is NOT flux-driven reads 1 (unchanged), so every mesic world — and
    /// every existing pin — is byte-identical. Derived from band ordinals, no
    /// new dial (the ratio's args are computed, not literal).
    pub fn flux_output_factor(&self, orb: u64) -> Ratio {
        use crate::almanac::ThermalRegion;
        if !self.flux_affine() {
            return ratio(1, 1);
        }
        let flux = self.stellar_flux_of(orb).max(1); // never zero the line out
        let baseline = ThermalRegion::Mesic.flux_level();
        ratio(flux as i64, baseline as i64)
    }

    /// **THE MEDIUM DENSITY an orb offers** (`44_LINEAGES` §2b) — read as
    /// its albedo: a world wrapped in atmosphere or ocean reflects; a bare
    /// airless crust or vacuum does not. A pure reading, 0 for an unsited or
    /// airless orb.
    pub fn medium_density_of(&self, orb: u64) -> Ratio {
        self.orb_by_id(orb)
            .and_then(|held| held.thermal.as_ref())
            .map(|site| site.albedo.value().clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// **IS THIS A MOVING-FOOTPRINT (MOBILE) LINE?** (`44_LINEAGES` §2b,
    /// ruled 2026-08-13) — derived from PHYSICAL STATE, never a clade name.
    /// Two objective checks: the line lives in a MEDIUM (its orb carries an
    /// atmosphere/fluid — albedo > 0) AND its footprint is MOBILE (more
    /// drifting bodies than anchored zones — population exceeds the built
    /// footprint, the swarm/spore signature). A solid airless world (albedo
    /// 0) or a well-anchored one is false, so every standard terrestrial
    /// world stays byte-identical.
    pub fn mobility_affine(&self, orb: u64) -> bool {
        let in_medium = self.medium_density_of(orb) > ratio(0, 1);
        let mobile_footprint = self.population_of(orb) > self.living_zones_of(orb);
        in_medium && mobile_footprint
    }

    /// The mobile line's GROWTH multiplier (`44_LINEAGES` §2b): mobility in a
    /// medium accelerates spread (ruled 4/3). A non-mobile line reads 1.
    pub fn footprint_growth_factor(&self, orb: u64) -> Ratio {
        if self.mobility_affine(orb) {
            ratio(4, 3)
        } else {
            ratio(1, 1)
        }
    }

    /// The mobile line's PERMANENCE multiplier — the price of mobility: low
    /// structural permanence on its infrastructure ceiling (ruled 3/4, the
    /// exact reciprocal of the growth boost, so the one dial spawns both).
    pub fn footprint_permanence_factor(&self, orb: u64) -> Ratio {
        if self.mobility_affine(orb) {
            ratio(1, 1) / self.footprint_growth_factor(orb)
        } else {
            ratio(1, 1)
        }
    }

    /// **IS THE SITTING CLADE COOLING-DRIVEN?** (`44_LINEAGES` §2b, ruled
    /// 2026-08-13) — the MIRROR of `flux_affine`: a line whose HOME band is
    /// cold (Sublime or colder) is a structural, slow-growth kind
    /// (lithovore, crystalline) whose living volume and permanence EXPAND as
    /// a world cools. Derived from the clade's preferred region, no new map.
    pub fn cooling_affine(&self) -> bool {
        use crate::almanac::ThermalRegion;
        crate::almanac::clade_preferred_region(self.rejuvenation_clade()) < ThermalRegion::Mesic
    }

    /// **THE COOLING-CYCLE K FACTOR** (`44_LINEAGES` §2b) — for a cooling-
    /// driven line, the population ceiling ratchets INVERSELY with thermal
    /// flux: a hot/volcanic band cramps it (ruled 0.75), a cooled band
    /// unlocks the ceiling (ruled 1.25). The ruled band is expressed through
    /// the Mesic ordinal (`m/(m+1)` = 3/4, `(m+2)/(m+1)` = 5/4) so the factor
    /// stays a reading of the temperature line, not a dialed literal. A line
    /// that is not cooling-driven reads 1 — every existing world byte-exact.
    pub fn cooling_k_factor(&self, orb: u64) -> Ratio {
        use crate::almanac::ThermalRegion;
        if !self.cooling_affine() {
            return ratio(1, 1);
        }
        let flux = self.stellar_flux_of(orb) as i64;
        let mesic = ThermalRegion::Mesic.flux_level() as i64;
        if flux > mesic {
            ratio(mesic, mesic + 1) // 3/4 — cramped by heat
        } else if flux < mesic {
            ratio(mesic + 2, mesic + 1) // 5/4 — the ceiling unlocks in the cold
        } else {
            ratio(1, 1)
        }
    }

    /// **THE DISPOSITION AXIS'S VALUES** — every disposition code the
    /// vocabulary carries (the three seeded, plus any minted), read from the
    /// fold. The caste set is exactly this set, so castes are as open as the
    /// axis is — never a fixed three.
    pub fn disposition_codes(&self) -> Vec<u8> {
        let mut codes: Vec<u8> = crate::vocabulary::fold(self.chronicle(), crate::vocabulary::Axis::Disposition)
            .into_iter()
            .map(|entry| entry.code)
            .filter(|&code| code != 0) // 0 is the unclassed ground state
            .collect();
        codes.sort_unstable();
        codes.dedup();
        codes
    }

    /// **THE CASTE SPLIT** (`44_LINEAGES` §2d) — how the standing population
    /// DIFFERENTIATES into its specialized roles, `(disposition code,
    /// headcount)`, ONE bucket per disposition the vocabulary knows. A
    /// DERIVED partition (no new state, no deed — the `stability_of`
    /// precedent), weighted by `self_emphasis`: the sitting disposition
    /// takes the majority of its own caste, the rest split the remainder.
    /// The headcount is CONSERVED exactly — the flooring remainder falls to
    /// the sitting caste — so the components sum to the population. NOT three
    /// buckets: a minted fourth disposition is a fourth caste here, with no
    /// code change. A line with no sealed disposition is undifferentiated
    /// (an empty split — every hand is the same).
    pub fn caste_split(&self, orb: u64) -> Vec<(u8, u64)> {
        let people = self.population_of(orb);
        let Some(sitting) = self.civ_axes().map(|axes| axes.disposition) else {
            return Vec::new();
        };
        let codes = self.disposition_codes();
        if codes.is_empty() {
            return Vec::new();
        }
        let mut split: Vec<(u8, u64)> = Vec::with_capacity(codes.len());
        let mut assigned = 0u64;
        for (code, weight) in crate::dop::self_emphasis(sitting, &codes) {
            let share: u64 = (ratio(people as i64, 1) * weight)
                .floor()
                .to_integer()
                .try_into()
                .unwrap_or(0);
            split.push((code, share));
            assigned += share;
        }
        // Conserve the headcount: the flooring remainder falls to the
        // sitting disposition's own caste, so the split sums to the
        // population exactly and stays replay-inert.
        if let Some(entry) = split.iter_mut().find(|(code, _)| *code == sitting) {
            entry.1 += people.saturating_sub(assigned);
        }
        split
    }

    /// The headcount standing in one NAMED (seeded) caste — its disposition
    /// bucket in the `caste_split`. Minted castes are read from the split
    /// directly by code.
    pub fn caste_of(&self, orb: u64, caste: Caste) -> u64 {
        self.caste_split(orb)
            .into_iter()
            .find(|(code, _)| *code == caste.disposition())
            .map(|(_, headcount)| headcount)
            .unwrap_or(0)
    }

    /// **ADVANCE THE LINE ALONG ITS ARC** (`44_LINEAGES` §2c) — the
    /// transformation deed. A line does not sit at its founding coordinate:
    /// once it has CRYSTALLIZED the deepening (condition 3, the earned
    /// technology door the regenerator and refinery already pass), it may
    /// advance a stage, lifting its provisioning capacity. Gated (an
    /// un-deepened line is refused), receipted (replay-exact), and bounded
    /// by `u8`. Build scope. The arc is EARNED, never free.
    pub fn evolve(&mut self) -> Result<u8, ActionRefused> {
        self.evolve_as(SOVEREIGN)
    }

    pub fn evolve_as(&mut self, actor: u64) -> Result<u8, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        // The technology door: the deepening must stand crystallized on the
        // genesis orb — evolution is understood, not stumbled into.
        let deepened = self
            .techs
            .iter()
            .any(|(_, condition, _, _)| *condition == 3);
        if !deepened {
            return Err(ActionRefused::ConditionUncrystallized { condition: 3 });
        }
        let Some(stage) = self.evolution_stage.checked_add(1) else {
            return Err(ActionRefused::ConditionUncrystallized { condition: 3 });
        };
        self.close(ClosureEvent::EvolutionAdvanced { stage }, actor);
        self.evolution_stage = stage;
        Ok(stage)
    }

    /// **THE LEISURE BANK** (COG-1, doc 52 §7) — the cognition budget standing
    /// on this orb: everything the cascade's surplus above every survival drive
    /// has banked, net of everything the Mind has spent minting. A line that
    /// has never run a surplus reads zero — a mind that cannot afford to think.
    /// Folded from the `LeisureSettled` receipts (floored at zero).
    pub fn leisure_bank_of(&self, orb: u64) -> Ratio {
        self.leisure_banks
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, bal)| bal.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// **SETTLE THE LEISURE BANK** (COG-1, doc 52 §7, tag 180) — apply a NET
    /// change to the cognition budget (the cascade's surplus banked in, the
    /// Mind's `mint_cost` spent out), floored at zero, and receipt it so replay
    /// applies the same delta. A zero delta emits nothing, so a world that
    /// neither banks surplus nor thinks is byte-identical.
    pub fn settle_leisure(&mut self, orb: u64, delta: Ratio) {
        let zero = ratio(0, 1);
        if delta == zero {
            return;
        }
        let next = (self.leisure_bank_of(orb) + delta.clone()).max(zero.clone());
        self.leisure_banks.retain(|(o, _)| *o != orb);
        if next > zero {
            self.leisure_banks.push((orb, next));
        }
        let stamp = self.clock.now();
        self.chronicle.append(
            crate::dop::ClosureEvent::LeisureSettled { orb_id: orb, delta },
            stamp,
            vec![],
        );
    }

    /// The per-clade LEISURE good this orb's line savours (§3), or `None`
    /// for a line with no closed ration to pair it with (an element-eater,
    /// whose higher tiers ride #29). Reads the same genesis metabolism the
    /// ration does.
    pub fn leisure_of(&self, orb: u64) -> Option<Resource> {
        match self.ration_of(orb) {
            Ration::Resource(_) => {
                let metabolism =
                    self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
                Some(metabolic_leisure(metabolism))
            }
            Ration::Element(_) => None,
        }
    }

    /// **STABILITY** in `[0,1]` (§4) — a DERIVED contentment reading (no
    /// stored field, no deed, the `regrown_ration` precedent). In the fed
    /// regime it is the LEISURE COVERAGE: the banked leisure good measured
    /// against what the standing population demands over the span, capped
    /// at one. Full leisure → full stability → full growth; none → zero →
    /// the population STALLS (births halt, only mortality runs). A world
    /// with no granary, or no leisure pairing, reads full stability — the
    /// leisure tier simply does not apply to it, so it never stalls on it.
    /// **THE BARE LEISURE COVERAGE** — banked leisure over the leisure
    /// demand, capped at one, BEFORE any welfare. `None` when there is no
    /// provisioned stall to speak of (no granary, no leisure good, or no
    /// demand → stability is simply full and welfare is moot). The one
    /// reading both `stability_of` and the welfare loop share.
    pub fn leisure_coverage_of(&self, orb: u64, stamp: u64) -> Option<Ratio> {
        let full = ratio(1, 1);
        if !self.has_granary(orb) {
            return None;
        }
        let leisure = self.leisure_of(orb)?;
        let demand = self.leisure_demand(orb, stamp);
        if demand <= ratio(0, 1) {
            return None;
        }
        let banked = self
            .orb_by_id(orb)
            .map(|held| held.banked_amount(leisure))
            .unwrap_or_else(|| ratio(0, 1));
        Some((banked / demand).min(full))
    }

    pub fn stability_of(&self, orb: u64, stamp: u64) -> Ratio {
        let full = ratio(1, 1);
        let Some(leisure_coverage) = self.leisure_coverage_of(orb, stamp) else {
            return full;
        };
        // THE WELFARE LOOP (`49_TAXATION` §3/§5): a WelfareHub fills the
        // stability gap by spending the sovereign TREASURY —
        //   Stability = Leisure Coverage + Welfare Subsidy
        // capped at one. No hub or an empty treasury → subsidy 0 →
        // byte-identical.
        (leisure_coverage.clone() + self.welfare_subsidy(orb, stamp, &leisure_coverage)).min(full)
    }

    /// **THE PER-CAPITA RATION** — what one head eats per beat. The base is
    /// R6's half-harvest (a pair to a habitat's yield); it is MODULATED by
    /// the sitting CLASS, so an archetype that is costly to keep eats more
    /// per head and a frugal one less. The modifier is the class's OWN
    /// upkeep binding (`class_ration_factor`) — the same `archetype_binding`
    /// the core tick reads — so every one of the 14 archetypes (and any
    /// minted class) provisions distinctly, from the fold, with no new dial.
    /// Both the caloric demand (population loop) and the leisure demand read
    /// this ONE rate.
    pub fn per_capita_ration(&self) -> Ratio {
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let pair = ratio(1, 1) + ratio(1, 1);
        let base = crate::dop::metabolic_harvest_of(metabolism, self.metabolic_element()).1 / pair;
        base * self.class_ration_factor()
    }

    /// **THE CLASS'S RATION MODIFIER** — how much this archetype eats per
    /// head relative to the base, read as its UPKEEP factor from the class
    /// binding. Reads the FOLD so a MINTED class carries its own (a compiled
    /// class takes the fast `archetype_binding` path; a minted one is looked
    /// up in `vocabulary::fold`). An unclassed line eats the base (factor 1).
    pub fn class_ration_factor(&self) -> Ratio {
        use crate::vocabulary::{archetype_binding, factor_on, fold, Axis, CHANNEL_UPKEEP};
        let one = ratio(1, 1);
        let Some((class, _)) = self.taxon else {
            return one;
        };
        // Compiled archetypes take the compiled table directly (no chain
        // scan); a minted class is resolved from the class fold, so its
        // own upkeep binding governs how it eats.
        let binding = if u16::from(class) <= u16::from(crate::dop::class_ceiling()) {
            archetype_binding(class)
        } else {
            fold(self.chronicle(), Axis::Class)
                .into_iter()
                .find(|entry| entry.code == class)
                .map(|entry| entry.binding)
                .unwrap_or_default()
        };
        factor_on(&binding, CHANNEL_UPKEEP)
    }

    /// The leisure the standing population demands over the span since the
    /// last touch — the per-capita rate (class-modulated, `per_capita_ration`)
    /// scaled by the disposition appetite (§5): an expansionist wants none,
    /// an epicurean twice as much, every other line the base. No new dial.
    pub fn leisure_demand(&self, orb: u64, stamp: u64) -> Ratio {
        let zero = ratio(0, 1);
        let p = self.population_of(orb);
        let Some(held) = self.orb_by_id(orb) else {
            return zero;
        };
        let dt = stamp.saturating_sub(held.last_touch);
        if p == 0 || dt == 0 {
            return zero;
        }
        let factor = self.provision_leisure_factor();
        if factor <= zero {
            return zero;
        }
        ratio(p as i64, 1) * self.per_capita_ration() * ratio(dt as i64, 1) * factor
    }

    /// **THE DISPOSITION'S LEISURE APPETITE** (§5) — how much recreation a
    /// line demands, as a factor on the base per-capita rate. Keyed on the
    /// sealed/postured disposition (`civ_axes`, the same axis the
    /// vocabulary mints): expansionist 0 (lean — no luxury tier, grows on
    /// food alone), epicurean 2 (deep — the granary becomes an economic
    /// engine), every other line 1 (the base). Built from `0`, `1`, and
    /// `1+1` — a factor set, never a dial.
    pub fn provision_leisure_factor(&self) -> Ratio {
        match self.civ_axes().map(|axes| axes.disposition) {
            Some(1) => ratio(0, 1),               // expansionist — military rations
            Some(3) => ratio(1, 1) + ratio(1, 1), // epicurean — luxury appetite
            _ => ratio(1, 1),                     // empiricist / default — the base
        }
    }

    /// **THE EMPIRICIST RESEARCH SIPHON** (§5) — the share of would-be
    /// births an empiricist line spends on KNOWLEDGE instead of mouths.
    /// Half the surplus growth is redirected: the population grows slower,
    /// and the redirected count is the research this society mints from its
    /// provisioning margin (the metric here; the `DiscoveryMinted` deed is
    /// the follow-up). Zero for every other disposition — they spend the
    /// surplus on people. Derived (half = 1/(1+1)), no new dial.
    pub fn research_siphon(&self, orb: u64, births: i64) -> i64 {
        if births <= 0 {
            return 0;
        }
        let people = self.population_of(orb);
        if people == 0 {
            return 0;
        }
        // Research is the work of the COGNITION caste (the empiricist-role
        // fraction, `caste_of`). The siphon is the surplus growth that caste
        // spends on knowledge, so it EMERGES from composition — an
        // empiricist line, mostly cognition, spends the most; any line with
        // a cognition caste spends some; a minted line spends per its own
        // mix. No `disposition == N` — this is the vent opened one level
        // deeper: the caste drives the output.
        let cognition = self.caste_of(orb, crate::dop::Caste::Cognition);
        ((ratio(births, 1) * ratio(cognition as i64, 1)) / ratio(people as i64, 1))
            .floor()
            .to_integer()
            .try_into()
            .unwrap_or(0)
    }
}
