//! **The provisioning layer** (`43_DISTRIBUTION`, task #45+).
//!
//! The granary is the infrastructure that turns the #40–42 subsistence
//! ledger into an economy. It holds no stock of its own — the depot is
//! the orb's banked ration (ruled 2026-08-13) — so this layer stores
//! nothing new and mints no deed: everything it adds (the stratified
//! demand, the leisure tier, the stability reading, the disposition
//! factors) is derived at the collect touch, the `regrown_ration`
//! precedent. This module holds the readings the consumption cycle (L2+)
//! consults; L1 is the gate itself.

use super::*;
use crate::dop::{metabolic_leisure, Caste, Ration, Resource, ZoneKind};

/// The leisure grain: every leisure flow is floored onto `1/64` at the source
/// (`settle_leisure`, ONT-4), so the bank is always a bounded `1/64` multiple and
/// its denominator can never explode. Matches the market / regrowth / orbit grids
/// (all `64`). A bare structural constant, not a balance dial.
const LEISURE_GRAIN: i64 = 64;

impl World {
    /// **The micronutrient factor** (G2 trace elements, ruled 2026-08-14)
    /// — how a line's harvest is modulated by the trace elements its own
    /// world offers. The requirement is the PLANET's: the distinct
    /// elements its prospected veins bear (a line eats what its world
    /// grows). Of those, the line thrives on the ones it actually holds
    /// (a positive bank, mined or traded in) and sickens on the ones it
    /// lacks. Soft and graded, never a freeze:
    ///
    ///   factor = 1 + (present − absent) / (2 × required)
    ///          = 1 + (2·present − required) / (2·required)     ∈ [½, 3/2]
    ///
    /// Full coverage lifts the harvest by half; a bare larder halves it;
    /// the penalty deepens as more of the world's elements go unmet. A
    /// planet with no prospected veins bears no requirement, so the factor
    /// is exactly one and every world that never opened a vein is
    /// byte-identical. Derived from vein/bank counts — no dial.
    /// **The substrate's trace signature** (G2 trace elements §2,
    /// 2026-08-14) — does this line's chemistry favour `element`? Each
    /// substrate values a different slice of the periodic spine, derived
    /// (never a species table) from the clade's own thermal preference
    /// against the element's family. The spine of four splits at its
    /// median: light families {1,2} vs heavy {3,4}. A hot-preferring
    /// clade's chemistry is built of and maintained by heavy elements
    /// (the plasmic/saurian lines that prefer the torrid bands); a
    /// cold-preferring one by light elements (the cryo lines); a temperate
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

    /// **The harvesting catalyst factor** (G2 trace elements §3,
    /// 2026-08-14) — elements are not only eaten, they are tools: a line's
    /// favoured elements flux its extraction, the reagents that let it mine
    /// more matter per beat. Unlike the micronutrient maintenance need
    /// this is boost-only (a reagent lifts extraction where held; its
    /// absence is merely baseline, never a penalty), and it lands on the
    /// extractor line alone — food and industry read it not.
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

    /// **The line's trace requirement** — the distinct elements its world's
    /// veins bear that its substrate favours (the intersection of "a line
    /// eats what its world grows" with the substrate's own signature). A
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

    /// **The universal trace-consumption vent** (G2 trace elements, ruled
    /// 2026-08-14) — the trace elements a population burns maintaining its
    /// substrate over the span since the last touch, whatever its clade.
    /// The requirement is the world's (the distinct elements its veins
    /// bear); of those, the line burns a trace of each it actually holds
    /// — the demand scaled to population × the span at trace proportions
    /// (`(p·Δt)/grain` per element, `≪` the gross caloric ration), capped
    /// by what is banked so the larder never goes negative. Returned as
    /// `(element, amount)` draws to receipt as `TraceConsumed` and debit.
    ///
    /// A world with no veins, no population, or an empty larder burns
    /// nothing — so every world without the trace economy is byte-identical.
    /// The depletion is the point: as the larder empties the
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

    /// **Does this orb stand A granary?** The gate that flips the
    /// provisioned economy on. Where it answers false, the population
    /// runs the #40–42 subsistence model verbatim (harvest→stomach, no
    /// leisure, no stability, byte-identical replay); where true, the
    /// stratified consumption cycle and its disposition factors come into
    /// force. An active granary only — one that cannot pay its own upkeep
    /// is dark, and a dark provisioning node provisions nothing.
    pub fn has_granary(&self, orb: u64) -> bool {
        self.granary_footprint(orb) > 0
    }

    /// The count of active granaries standing on an orb — the provisioning
    /// footprint that lifts the population ceiling (§2), distinct from the
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

    /// **The population ceiling `K_pop`** (§2) — the ecological footprint
    /// plus the provisioning footprint plus the evolutionary stage, read by
    /// the sitting clade at its place. A granary is provisioning capacity;
    /// an advanced stage (`44_LINEAGES` §2c) is earned capacity — an
    /// evolved civilization sustains more people per world. Never the food's
    /// own regrowth ceiling, which stays ecological. With no granary and no
    /// advancement the footprint is exactly `living_zones_of`, so the
    /// ceiling — and every #40–42 world — is byte-identical.
    /// **The continuous insolation AT this orb** `I = L·(1−A)/r²` (COG Slice
    /// 2b) — the absorbed stellar flux, read natively from the star's exact
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

    /// The insolation normalized to a reference G-type star at unit orbit and
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
    /// founding digest (immutable after founding) folded with the orb id, so
    /// each world's planets carry distinct area yet every read and every
    /// replay reproduces it bit-for-bit. Not the terminal digest (which moves
    /// each beat); the founding receipt's own hash, fixed for the world's life.
    fn planet_geometry_seed(&self, orb: u64, salt: &[u8]) -> u64 {
        let founding = self
            .chronicle
            .history()
            .next()
            .map(|receipt| receipt.own().0)
            .unwrap_or([0u8; 32]);
        let mut hasher = blake3::Hasher::new();
        hasher.update(salt);
        hasher.update(&founding);
        hasher.update(&orb.to_le_bytes());
        u64::from_le_bytes(hasher.finalize().as_bytes()[..8].try_into().expect("eight bytes"))
    }

    /// **The planet's surface area** (COG Slice 2b) — the total geometric
    /// capacity of the manifold (the `4πR²` of the sphere, held directly as
    /// an exact rational in area units so no irrational `π` enters). Hash-
    /// derived from the founding digest, so a system's worlds range from
    /// small rocks to broad giants deterministically. In `[8, 40)` units.
    pub fn surface_area_of(&self, orb: u64) -> Ratio {
        ratio(8 + (self.planet_geometry_seed(orb, b"netstratum.planet.area.v1") % 32) as i64, 1)
    }

    /// **The arable ratio** `ρ` (COG Slice 2b) — the fraction of the surface
    /// that is chemically and topologically viable for primary production
    /// (the hydrology/nutrient face of the world). Hash-derived in
    /// `(1/16, 15/16)`, so no world is wholly barren or wholly fertile.
    pub fn arable_ratio_of(&self, orb: u64) -> Ratio {
        ratio(1 + (self.planet_geometry_seed(orb, b"netstratum.planet.arable.v1") % 15) as i64, 16)
    }

    /// **The arable area** `Area × ρ` — the growing surface primary
    /// production actually runs over, the area term of the `P_primary` law.
    pub fn arable_area_of(&self, orb: u64) -> Ratio {
        self.surface_area_of(orb) * self.arable_ratio_of(orb)
    }

    /// **The primary production potential** (COG Slice 2b/2c) — the absolute
    /// thermodynamic food flow the star could grow over this world's arable
    /// surface for this clade:
    ///
    /// ```text
    ///   P_primary = I_norm × (Area × ρ) × η × REGROWTH_GRAIN
    /// ```
    ///
    /// the star's raw insolation `I` delivered over the arable surface, gated
    /// by the sitting clade's biological efficiency `η = habitability` — the
    /// crucial shift: habitability is not a cap on capacity, it is the
    /// fraction of the raw starlight this species can convert. A clade matched
    /// to its band reads η=1 and drinks the planet's full light; a mismatched
    /// one (a flatlander on a scorched rock, a plasmoid on a temperate world)
    /// reads η→0 and starves under the same sun. This is the ceiling the
    /// syntropic and cultivated flows draw from — never itself the population
    /// ceiling.
    pub fn primary_production(&self, orb: u64) -> Ratio {
        if self.orb_region(orb).is_none() {
            return ratio(0, 1);
        }
        // DIO-2: the species-relative η is now a continuous flux tent read on
        // the exact equilibrium flux `t_eq4 = insolation · K`, so a
        // continuously-sited Diophantine orbit grades smoothly (no 8-way
        // bucket cliffs). A clade off its optimum converts less of the same
        // starlight; far enough off, it starves.
        let t_eq4 = self.insolation_of(orb) * crate::thermal::thermal_k();
        let eta = crate::almanac::habitability_flux(self.rejuvenation_clade(), &t_eq4);
        if eta <= ratio(0, 1) {
            return ratio(0, 1);
        }
        self.insolation_norm(orb)
            * self.arable_area_of(orb)
            * eta
            * ratio(crate::almanac::REGROWTH_GRAIN, 1)
    }

    /// **The syntropic baseline flow** (COG Slice 2c, operator ruling
    /// 2026-08-15) — the wild share of primary production the planet grows
    /// with no infrastructure and no labor, driven purely by physics (the
    /// star pumps it in). Because it is syntropic it can never sit at zero on
    /// a lit, habitable world: a stock stripped bare regenerates to this
    /// flow. A genesis band subsists on exactly this — gatherers on the wild
    /// baseline — and it is the floor the ambient ration stock recovers to.
    pub fn syntropic_flow(&self, orb: u64) -> Ratio {
        // Era 1'S land (docs/86): on a world standing on an era-1 world, the
        // wild flow is the heads its own medium feeds untended, each drawing
        // its ration; the founding orb is that world.
        if let (Some(wild), true) = (self.land.as_ref(), orb == super::FIRST_ORB) {
            return wild.clone() * self.per_capita_ration();
        }
        // A quarter of the potential is the untended wild yield; the other
        // three-quarters is unlocked only by cultivation (operator ruling
        // 2026-08-15). The marginal habitable band already reads a quarter
        // efficiency (`habitability` = 1/2² at the two-band edge), so a
        // forager on the wild fringe gathering a quarter is the same law.
        self.primary_production(orb) * ratio(1, 4) // APPROVED-SEED
    }

    /// **The cultivated yield** (COG Slice 2c) — the food flow extracted
    /// above the wild baseline by infrastructure the civilization has raised:
    /// its ration-producing zones (each scaled by its vigour), its granaries,
    /// its earned evolutionary stage. Zero for a band that has built nothing
    /// — the gatherers. This is the flow the Mind unlocks when starvation
    /// forces it to invent cultivation, and it grows `K` past the wild limit.
    /// Polymorphic: the ration is the sitting species' own metabolic carrier
    /// (biomass, iron, …), so every clade cultivates its own energy.
    pub fn cultivated_yield(&self, orb: u64) -> Ratio {
        let ration = match self.ration_of(orb) {
            Ration::Resource(ration) => ration,
            // ONT-2b (operator ruling I — mining is a lithotroph's agriculture):
            // for an element-eater the latent capacity is the unextracted
            // richness of its staple veins (Σ vein_richness = natural −
            // extracted). This is the vein-as-larder read that lifts `K_pop`
            // above the wild syntropic limit so the band grows into the vein;
            // as eating mines the vein (`StapleMined`), `extracted` climbs, the
            // richness falls, this yield drops, and `K_pop` follows it down —
            // boom-bust (ruling III). Zero once the staple vein is worked out
            // (no element regrowth until ONT-3) or before a star/vein stands.
            Ration::Element(staple) => {
                let Some(class) = self.star else {
                    return ratio(0, 1);
                };
                let (_, _, natural) = crate::dop::element_properties(&staple, class);
                return self
                    .prospects
                    .iter()
                    .filter(|vein| vein.orb == orb && vein.element == staple)
                    .map(|vein| crate::dop::vein_richness(&natural, &vein.extracted, &vein.rejuvenated))
                    .fold(ratio(0, 1), |sum, richness| sum + richness);
            }
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

    /// **The dynamic population ceiling `K_pop`** (§2, COG Slice 2c) — no
    /// longer a fixed planetary limit but the sum of the wild and cultivated
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
    /// thus relational and the ceiling dynamic — earned, not merely sited.
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

    /// **The evolutionary stage** (`44_LINEAGES` §2c) — how far this line
    /// has advanced along its arc. 0 at founding; raised only by the gated,
    /// earned `EvolutionAdvanced` deed.
    pub fn evolution_stage(&self) -> u8 {
        self.evolution_stage
    }

    /// **The failure STATE** (`50_WIN_CONDITIONS`, tag 172) — `Some(stamp)`
    /// once every orb has settled to true zero population (no headcount, no
    /// fractional carry anywhere); `None` while the line stands. A cheap
    /// `Copy` read, distinct from `committed()` — a per-beat roster check
    /// (`run_era_inner`) must not pay a full-state clone just to ask whether
    /// a line can still act.
    pub fn collapsed(&self) -> Option<u64> {
        self.collapsed
    }

    /// **Does this line's DISPOSITION carry A trait?** (`44_LINEAGES`) — the
    /// generic reader that replaces the scattered `disposition == N` checks.
    /// A seeded disposition carries the traits its table declares; a minted
    /// disposition inherits the traits of its base (the seeded disposition
    /// its mint modulates), read from the fold — so freight-affinity,
    /// novelty-hunger, and vanity are no longer stuck to three codes but flow
    /// to any line that is, or descends from, one that bears them.
    pub fn disposition_carries(&self, wanted: crate::dop::DispositionTrait) -> bool {
        let Some(code) = self.civ_axes().map(|axes| axes.disposition) else {
            return false;
        };
        if crate::dop::disposition_traits(code).contains(&wanted) {
            return true;
        }
        // A minted disposition inherits its base's traits. Forward-compat:
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

    /// **The stellar flux an orb bathes in** (`44_LINEAGES` §2b) — the flux
    /// level of its thermal band (0 coldest … 7 hottest). A pure reading.
    pub fn stellar_flux_of(&self, orb: u64) -> u8 {
        self.orb_region(orb).map(|region| region.flux_level()).unwrap_or(0)
    }

    /// **Is the sitting CLADE flux-driven?** (`44_LINEAGES` §2b) — a line
    /// whose home band is hot (Torrid or hotter) is adapted to run on stellar
    /// flux: its output rises in heat and falls in cold. Derived from the
    /// clade's own preferred region (no new map) — the saurian Scale-Matrix
    /// and the plasmoid Vapor-Stalker are flux-driven because their clades
    /// prefer the hot bands; a mesic clade is not.
    pub fn flux_affine(&self) -> bool {
        use crate::almanac::ThermalRegion;
        crate::almanac::clade_preferred_region(self.rejuvenation_clade()) >= ThermalRegion::Torrid
    }

    /// **The flux output factor** (`44_LINEAGES` §2b) — a flux-driven line's
    /// output scales with the flux it stands in, relative to the temperate
    /// baseline (Mesic): hot bands lift it, cold bands starve it. A line that
    /// is not flux-driven reads 1 (unchanged), so every mesic world — and
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

    /// **The medium density an orb offers** (`44_LINEAGES` §2b) — read as
    /// its albedo: a world wrapped in atmosphere or ocean reflects; a bare
    /// airless crust or vacuum does not. A pure reading, 0 for an unsited or
    /// airless orb.
    pub fn medium_density_of(&self, orb: u64) -> Ratio {
        self.orb_by_id(orb)
            .and_then(|held| held.thermal.as_ref())
            .map(|site| site.albedo.value().clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// **Is this A moving-footprint (mobile) LINE?** (`44_LINEAGES` §2b,
    /// ruled 2026-08-13) — derived from physical STATE, never a clade name.
    /// Two objective checks: the line lives in a medium (its orb carries an
    /// atmosphere/fluid — albedo > 0) and its footprint is mobile (more
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

    /// The mobile line's permanence multiplier — the price of mobility: low
    /// structural permanence on its infrastructure ceiling (ruled 3/4, the
    /// exact reciprocal of the growth boost, so the one dial spawns both).
    pub fn footprint_permanence_factor(&self, orb: u64) -> Ratio {
        if self.mobility_affine(orb) {
            ratio(1, 1) / self.footprint_growth_factor(orb)
        } else {
            ratio(1, 1)
        }
    }

    /// **Is the sitting CLADE cooling-driven?** (`44_LINEAGES` §2b, ruled
    /// 2026-08-13) — the mirror of `flux_affine`: a line whose home band is
    /// cold (Sublime or colder) is a structural, slow-growth kind
    /// (lithovore, crystalline) whose living volume and permanence expand as
    /// a world cools. Derived from the clade's preferred region, no new map.
    pub fn cooling_affine(&self) -> bool {
        use crate::almanac::ThermalRegion;
        crate::almanac::clade_preferred_region(self.rejuvenation_clade()) < ThermalRegion::Mesic
    }

    /// **The cooling-CYCLE K factor** (`44_LINEAGES` §2b) — for a cooling-
    /// driven line, the population ceiling ratchets inversely with thermal
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

    /// **The DISPOSITION axis's values** — every disposition code the
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

    /// **The caste split** (`44_LINEAGES` §2d) — how the standing population
    /// differentiates into its specialized roles, `(disposition code,
    /// headcount)`, one bucket per disposition the vocabulary knows. A
    /// derived partition (no new state, no deed — the `stability_of`
    /// precedent), weighted by `self_emphasis`: the sitting disposition
    /// takes the majority of its own caste, the rest split the remainder.
    /// The headcount is conserved exactly — the flooring remainder falls to
    /// the sitting caste — so the components sum to the population. Not three
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

    /// The headcount standing in one named (seeded) caste — its disposition
    /// bucket in the `caste_split`. Minted castes are read from the split
    /// directly by code.
    pub fn caste_of(&self, orb: u64, caste: Caste) -> u64 {
        self.caste_split(orb)
            .into_iter()
            .find(|(code, _)| *code == caste.disposition())
            .map(|(_, headcount)| headcount)
            .unwrap_or(0)
    }

    /// **Advance the LINE along its ARC** (`44_LINEAGES` §2c) — the
    /// transformation deed. A line does not sit at its founding coordinate:
    /// once it has crystallized the deepening (condition 3, the earned
    /// technology door the regenerator and refinery already pass), it may
    /// advance a stage, lifting its provisioning capacity. Gated (an
    /// un-deepened line is refused), receipted (replay-exact), and bounded
    /// by `u8`. Build scope. The arc is earned, never free.
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

    /// **The leisure bank** (COG-1, doc 52 §7) — the cognition budget standing
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

    /// **Settle the leisure bank** (COG-1, doc 52 §7, tag 180) — apply a net
    /// change to the cognition budget (the cascade's surplus banked in, the
    /// Mind's `mint_cost` spent out), floored at zero, and receipt it so replay
    /// applies the same delta. A zero delta emits nothing, so a world that
    /// neither banks surplus nor thinks is byte-identical.
    /// **The thermodynamic demurrage rate** (ONT-4, operator ruling 2026-08-18) —
    /// the fraction of the leisure bank a world dissipates each beat, derived from
    /// its orbit and planetary vitals, never a dial. Leisure is stored surplus; it
    /// relaxes at a rate the world's own physics sets:
    ///
    /// ```text
    ///   rate = thermal(t_eq4) · dry(albedo) · orbital_tempo(P)
    /// ```
    ///
    /// - `thermal` = the morphodynamic thermal fraction (a scorched world → ~1, a
    ///   frozen one → ~0, mesic → ½): heat spends surplus, cold preserves it.
    /// - `dry` = `1 − albedo`: an arid dark rock burns its surplus fast, a wet
    ///   icy/cloudy world holds it (the moisture axis, ONT-1d).
    /// - `orbital_tempo` = `P_unit² / (P_unit² + P²)` — the world's Keplerian
    ///   period against its own star's unit-orbit year, in squared periods so it
    ///   stays exact (`P` itself is a root): a close, fast-orbit world turns
    ///   leisure over quickly, a distant slow one hoards it.
    ///
    /// Proportional (the caller forms `decay = bank × rate`), so zero at an empty
    /// bank — a poor line still banks its fasting trickle (COG-3) — and `rate ∈
    /// [0, 1)`, so a beat never sheds more than the bank stands. An unsited world
    /// (no star or no orbit, hence no thermal clock) sheds nothing. Exact-rational
    /// and replay-exact throughout.
    pub fn leisure_decay_rate(&self, orb: u64) -> Ratio {
        let zero = ratio(0, 1);
        let one = ratio(1, 1);
        let Some(class) = self.star else {
            return zero;
        };
        let Some((orbit_r, albedo)) = self
            .orb_by_id(orb)
            .and_then(|held| held.thermal.as_ref())
            .map(|site| (site.orbit_r.value().clone(), site.albedo.value().clone()))
        else {
            return zero;
        };
        let mass = class.mass();
        // t_eq4 — the incident flux the orbit sets, the same reading production
        // uses; thermal(t_eq4) is the world's dissipation tempo.
        let t_eq4 = self.insolation_of(orb) * crate::thermal::thermal_k();
        let thermal = crate::almanac::morphodynamic_thermal(&t_eq4);
        // dry ∈ [0,1]: a wet (high-albedo) world holds its surplus; an arid one
        // spends it. albedo is a reflectance ≤ 1, so `1 − albedo` never goes < 0.
        let dry = (one - albedo).max(zero.clone());
        // orbital_tempo from squared periods (both exact rational — no √ formed).
        let p2 = crate::thermal::orbital_period_squared(&orbit_r, &mass);
        let p_unit2 = crate::thermal::orbital_period_squared(&ratio(1, 1), &mass);
        let denom = p_unit2.clone() + p2;
        let orbital_tempo = if denom <= zero { zero.clone() } else { p_unit2 / denom };
        thermal * dry * orbital_tempo
    }

    pub fn settle_leisure(&mut self, orb: u64, delta: Ratio) {
        // ONT-4 — floor the flow, never the accumulator. The bank is a running sum
        // of per-beat deltas (surplus in, decay and mint-cost out); summing
        // rationals of differing denominators compounds the LCM without bound, and
        // the geometric demurrage (`bank × d`) accreted the whole causal history,
        // so the exact-rational denominator detonated (~beat 117 — and the old lazy
        // accumulator-graining that masked it grained the live bank but not replay,
        // which applies the raw receipt delta, so the two split the moment the
        // graining fired). The cure the scalar-collapse law names: floor the flow
        // at the source onto `1/LEISURE_GRAIN`, so the bank is always a bounded
        // `1/GRAIN` multiple and its denominator can never explode — and the
        // receipt carries the floored delta, so replay banks the identical value.
        // SEAL-B is a theorem again, not a truncation coincidence. Signed floor
        // (toward −∞): a spend rounds slightly larger, an inflow slightly smaller,
        // deterministic either way.
        // docs/64 P1 — a spend is a touch: the live bank moves and the receipt
        // carries everything since the bank was last receipted.
        if self.move_leisure(orb, delta) {
            self.receipt_leisure(orb);
        }
    }

    /// **Accrue leisure** (docs/64 P1) — the per-beat flow (surplus in, decay
    /// out) moves the live bank exactly as a settle does, but writes no
    /// receipt: it is integration, not a deed. The next touch (a spend, or
    /// the carry) receipts the difference, so replay and the judge stand on
    /// the same bank at every deed.
    pub fn accrue_leisure(&mut self, orb: u64, delta: Ratio) {
        self.move_leisure(orb, delta);
    }

    /// The live move, floored on the flow (the ONT-4 law); whether it moved.
    fn move_leisure(&mut self, orb: u64, delta: Ratio) -> bool {
        let zero = ratio(0, 1);
        let grid = ratio(LEISURE_GRAIN, 1);
        let flowed = (delta * grid.clone()).floor() / grid;
        if flowed == zero {
            return false;
        }
        let next = (self.leisure_bank_of(orb) + flowed).max(zero.clone());
        self.leisure_banks.retain(|(o, _)| *o != orb);
        if next > zero {
            self.leisure_banks.push((orb, next));
        }
        true
    }

    /// Receipt the live bank against the bank as last receipted: the delta is
    /// `live - receipted`, so replay's `max(receipted + delta, 0)` is the live
    /// bank exactly (it is never negative). Nothing when they agree.
    fn receipt_leisure(&mut self, orb: u64) {
        let zero = ratio(0, 1);
        let live = self.leisure_bank_of(orb);
        let held = self
            .leisure_receipted
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, b)| b.clone())
            .unwrap_or_else(|| zero.clone());
        if live == held {
            return;
        }
        self.leisure_receipted.retain(|(o, _)| *o != orb);
        self.leisure_receipted.push((orb, live.clone()));
        let stamp = self.clock.now();
        self.chronicle.append(
            crate::dop::ClosureEvent::LeisureSettled { orb_id: orb, delta: live - held },
            stamp,
            vec![],
        );
    }

    /// **Flush the leisure BOOKS** (docs/64 P1) — every bank whose live value
    /// moved since its last receipt is receipted now: the carry calls this
    /// before the snapshot, so a snapshot's bank and the receipts after it
    /// never count the same flow twice.
    pub fn flush_leisure(&mut self) {
        let mut orbs: Vec<u64> = self.leisure_banks.iter().map(|(o, _)| *o).collect();
        orbs.extend(self.leisure_receipted.iter().map(|(o, _)| *o));
        orbs.sort_unstable();
        orbs.dedup();
        for orb in orbs {
            self.receipt_leisure(orb);
        }
    }

    /// **The economic contentment** (TFR term III — the Demographic Stress
    /// Tensor, operator ruling 2026-08-15). Births are gated not by leisure
    /// alone but by the unified macroeconomic stress σ, `max(0, 1 − σ)`, which
    /// subsumes the old `stability_of` leisure gate and adds the wealth axis:
    ///
    ///   σ = min(1,  S_leisure  +  S_debt)
    ///
    ///   * `S_leisure = 1 − stability_of` — societal exhaustion, the leisure
    ///     coverage already netted of welfare (so this contains the whole of
    ///     the old gate: a solvent line reads `1 − S_leisure = stability_of`).
    ///   * `S_debt = min(1, |accrued_deficit| / cultivated_yield)` — insolvency:
    ///     resources the line owes but does not hold (`accrued < 0`, the
    ///     True-Wealth Gate), measured against its gross continuous income. Debt
    ///     with no income at all reads full stress; a solvent line reads zero.
    ///
    /// The unit weights (`w₁ = w₂ = 1`) are the operator-approved seed; at them
    /// a rested, solvent line reads `stability_of` exactly, so a world that
    /// never falls into arrears is byte-identical. This is the second face of
    /// death: the starvation cull empties a line that has no food; the sated
    /// collapse contracts one that is fed but has no future — taxed into debt or
    /// stripped of leisure, it simply stops having children.
    pub fn economic_contentment(&self, orb: u64, stamp: u64) -> Ratio {
        let one = ratio(1, 1);
        let zero = ratio(0, 1);
        let s_leisure = (one.clone() - self.stability_of(orb, stamp)).max(zero.clone());
        // Insolvency is the uncovered shortfall — `banked + accrued < 0` per
        // resource (the engine's own insolvency condition, economy.rs). Reading
        // bare `accrued < 0` would count the normal pending upkeep of any tick
        // as debt; only what the line owes beyond what it holds is arrears, so a
        // solvent line reads zero and stays byte-identical.
        let debt: Ratio = self
            .accrued_of(orb)
            .iter()
            .map(|(res, accrued)| {
                let held = self
                    .orb_by_id(orb)
                    .map(|o| o.banked_amount(*res))
                    .unwrap_or_else(|| zero.clone());
                (-(held + accrued.clone())).max(zero.clone())
            })
            .fold(zero.clone(), |sum, shortfall| sum + shortfall);
        let s_debt = if debt <= zero {
            zero.clone()
        } else {
            let income = self.cultivated_yield(orb);
            if income <= zero {
                one.clone() // owes with no income to service it — full stress
            } else {
                (debt / income).min(one.clone())
            }
        };
        let sigma = (s_leisure + s_debt).min(one.clone());
        (one - sigma).max(zero)
    }

    /// The per-clade leisure good this orb's line savours (§3), or `None`
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

    /// **Stability** in `[0,1]` (§4) — a derived contentment reading (no
    /// stored field, no deed, the `regrown_ration` precedent). In the fed
    /// regime it is the leisure coverage: the banked leisure good measured
    /// against what the standing population demands over the span, capped
    /// at one. Full leisure → full stability → full growth; none → zero →
    /// the population stalls (births halt, only mortality runs). A world
    /// with no granary, or no leisure pairing, reads full stability — the
    /// leisure tier simply does not apply to it, so it never stalls on it.
    /// **The bare leisure coverage** — banked leisure over the leisure
    /// demand, capped at one, before any welfare. `None` when there is no
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
        // The welfare LOOP (`49_TAXATION` §3/§5): a WelfareHub fills the
        // stability gap by spending the sovereign treasury —
        //   Stability = Leisure Coverage + Welfare Subsidy
        // capped at one. No hub or an empty treasury → subsidy 0 →
        // byte-identical.
        (leisure_coverage.clone() + self.welfare_subsidy(orb, stamp, &leisure_coverage)).min(full)
    }

    /// **The per-capita ration** — what one head eats per beat. The base is
    /// R6's half-harvest (a pair to a habitat's yield); it is modulated by
    /// the sitting class, so an archetype that is costly to keep eats more
    /// per head and a frugal one less. The modifier is the class's own
    /// upkeep binding (`class_ration_factor`) — the same `archetype_binding`
    /// the core tick reads — so every one of the 14 archetypes (and any
    /// minted class) provisions distinctly, from the fold, with no new dial.
    /// Both the caloric demand (population loop) and the leisure demand read
    /// this one rate.
    pub fn per_capita_ration(&self) -> Ratio {
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let pair = ratio(1, 1) + ratio(1, 1);
        let base = crate::dop::metabolic_harvest_of(metabolism, self.metabolic_element()).1 / pair;
        base * self.class_ration_factor()
    }

    /// **The class's ration modifier** — how much this archetype eats per
    /// head relative to the base, read as its upkeep factor from the class
    /// binding. Reads the fold so a minted class carries its own (a compiled
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

    /// **The disposition's leisure appetite** (§5) — how much recreation a
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

    /// **The empiricist research siphon** (§5) — the share of would-be
    /// births an empiricist line spends on knowledge instead of mouths.
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
        // spends on knowledge, so it emerges from composition — an
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
