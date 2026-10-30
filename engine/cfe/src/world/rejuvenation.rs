//! **Planetary rejuvenation — the lazy READ** (task #28, `40_REJUVENATION`).
//!
//! A world's natural ration — biomass for a biotic line, iron for a
//! mineral line, a minted element for a stone-eater (the G2 map,
//! [`crate::dop::metabolic_ration_of`]) — recovers toward a carrying
//! capacity `K` along the ruled S-curve
//! ([`crate::almanac::grained_logistic`]). Nothing here is hardcoded to
//! biomass: what a world regrows, how fast, and toward what ceiling all
//! follow the sitting line's own metabolism and clade, the same
//! species⊗planet⊗archetype composition every other flow reads
//! (`zone_yield`, `taxon_economy_of`). Deterministic physics get no
//! deed: the curve is recomputed from `(P0, K, r, Δt)` at need, exactly
//! as demurrage's decay is (`World::purse`) — a READ settles nothing
//! and writes nothing, and `Δt` runs from the orb's `last_touch`, the
//! same clock the demurrage organ already carries.
//!
//! This is the read-only half (a planet scan). The mutating half — the
//! settle at a value-moving touch, with its receipt digest — banks the
//! grown stock and advances `last_touch`, and lands with the wiring.

use super::*;
use crate::almanac::{carrying_capacity, grained_logistic, intrinsic_rate, region_of, FLATLANDER_CLADE, ThermalRegion};
use crate::dop::{metabolic_ration_of, Ration, ZoneKind};

impl World {
    /// The living ecological footprint of an orb — its habitats and
    /// regenerators, the zones that hold and restore life. The count
    /// that scales carrying capacity.
    pub fn living_zones_of(&self, orb: u64) -> u64 {
        self.orb_by_id(orb)
            .map(|held| {
                held.zones
                    .iter()
                    .filter(|zone| {
                        matches!(zone.kind, ZoneKind::Habitat | ZoneKind::Regenerator)
                    })
                    .count() as u64
            })
            .unwrap_or(0)
    }

    /// The thermal region this orb sits in, or `None` where it has no
    /// place at all (unsited, or no star) — a world with no place bears
    /// no natural stock, so regrowth reads a barren capacity there.
    pub fn orb_region(&self, orb: u64) -> Option<ThermalRegion> {
        self.orb_heat(orb).map(|t4| region_of(&t4))
    }

    /// The sitting CLADE — unsealed worlds default flatlander for play
    /// readability, the same default `taxon_economy_of` reads.
    pub fn rejuvenation_clade(&self) -> u16 {
        self.taxon
            .map(|(_, clade)| clade)
            .unwrap_or(u16::from(FLATLANDER_CLADE))
    }

    /// The ration this orb's line farms and eats — a closed [`Resource`]
    /// carrier for the three compiled metabolisms, or the line's own
    /// minted element for a G2 element-eater. Reads the same genesis
    /// metabolism and vocabulary fold every other flow does
    /// (`World::metabolic_element`).
    ///
    /// [`Resource`]: crate::dop::Resource
    pub fn ration_of(&self, orb: u64) -> Ration {
        // ING-2b: the almanac staple first — a species eats the food element
        // its own world grows (`staple_diet_of`, the morphodynamic projection
        // over the planet's prospected veins), dissolving the 3-`Resource`
        // food enum at the mechanism. Falls back to the closed carrier only
        // where the ground bears no vein this line can metabolise
        // (unprospected, or matter it cannot eat) — an unsealed/v1 world, or a
        // world before its first strike, reads byte-identically.
        if let Some(staple) = self.staple_diet_of(orb) {
            return Ration::Element(staple);
        }
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        metabolic_ration_of(metabolism, self.metabolic_element())
    }

    /// **ING-2 — the staple diet from the world's own veins.** The food element
    /// this orb's line metabolises best: the ingestion kernel
    /// ([`morphogen::staple`]) over the elements the planet's prospected veins
    /// bear (a species eats what its world grows), keyed on the species' morphodynamic
    /// signature. `None` when no prospected vein aligns — an unprospected world,
    /// or matter this line cannot metabolise (a plasmic reads no material vein; a
    /// mineral reads no organic one) — the Starvation Cull, from the projection.
    ///
    /// This is the almanac-derived diet handle that dissolves the 3-`Resource`
    /// `metabolic_ration` enum: a food element drawn from the open blake3 catalogue
    /// by the species tensor × the planet, never a bucket. It reads only; ING-2b
    /// routes `ration_of`/population/famine through it (a breaking migration off
    /// the closed carriers), so this landing is byte-identical.
    ///
    /// [`morphogen::staple`]: crate::morphogen::staple
    pub fn staple_diet_of(&self, orb: u64) -> Option<[u8; 32]> {
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let species = crate::morphogen::archetype(metabolism);
        let veins: Vec<[u8; 32]> = self
            .prospects
            .iter()
            .filter(|vein| vein.orb == orb)
            .map(|vein| vein.element)
            .collect();
        crate::morphogen::staple(&species, &veins, &ratio(0, 1))
    }

    /// **The carrying capacity of this orb's ration** — the ruled `K`,
    /// from the living footprint and the place (`ZoneKind` ⊗ star ⊗
    /// orbit, the last two folded into the thermal region) as READ by
    /// the sitting CLADE. An unsited orb, or one on a barren rock (for
    /// this clade), carries none.
    pub fn ration_capacity_of(&self, orb: u64) -> Ratio {
        if self.orb_region(orb).is_none() {
            return ratio(0, 1);
        }
        // DIO-2: read the sitting clade's habitability as the continuous flux
        // tent on the exact `t_eq4`, coherent with `primary_production`.
        let t_eq4 = self.insolation_of(orb) * crate::thermal::thermal_k();
        // ONT-1d: production reads the same morphodynamic viability siting will —
        // thermal habitability and the metabolism's phase feasibility across the
        // environmental gradient. Aridity is the orthogonal moisture axis, read
        // from the planet's albedo (an icy/cloudy high-albedo world is
        // volatile-rich and wet; a dark low-albedo rock is arid) — never a
        // re-derivation of temperature.
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let aridity = self.orb_aridity(orb);
        carrying_capacity(
            self.living_zones_of(orb),
            self.rejuvenation_clade(),
            metabolism,
            &t_eq4,
            &aridity,
        )
    }

    /// **The aridity axis** (ONT: the orthogonal moisture dimension) — read from
    /// the planet's albedo: an icy/cloudy world reflects (high albedo) and is
    /// volatile-rich and wet; a dark rock absorbs (low albedo) and is arid. A
    /// separate physical parameter from the orbit's temperature, so the
    /// environmental gradient is genuinely multi-axis. Zero (arid) for an
    /// unsited orb.
    fn orb_aridity(&self, orb: u64) -> Ratio {
        self.orb_by_id(orb)
            .and_then(|held| held.thermal.as_ref())
            .map(|site| site.albedo.value().clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// **The regrown stock — a scan, never a deed.** What this orb's
    /// banked ration has recovered to as of `stamp`, folding the ruled
    /// S-curve over the beats elapsed since its `last_touch`. Pure and
    /// replay-inert: looking at a planet is not a chain event, so it
    /// settles nothing and advances no clock — the grown value banks for
    /// real only at the next genuine touch (the demurrage precedent,
    /// `World::purse`).
    pub fn regrown_ration(&self, orb: u64, stamp: u64) -> Ratio {
        self.regrown_ration_with(orb, stamp, &self.ration_of(orb))
    }

    /// `regrown_ration` given a PRE-resolved ration. ING-2b PERF: `ration_of`
    /// now resolves the almanac staple by an O(prospects) scan + projection
    /// (`staple_diet_of`), and the collect touch has many readers of it
    /// (regrowth · settle · population · their `natural` recursion). The
    /// collect path resolves the ration once and threads it through these
    /// `_with` variants, so the scan runs once per touch instead of ~6× — a
    /// pure memoization (the ration is a deterministic function of the
    /// unchanging prospects/genesis, so the value is identical). The public
    /// wrappers keep resolving it themselves, for direct (test) callers.
    pub(super) fn regrown_ration_with(&self, orb: u64, stamp: u64, ration: &Ration) -> Ratio {
        let Some(held) = self.orb_by_id(orb) else {
            return ratio(0, 1);
        };
        let p0 = held.ration_amount(ration);
        let k = self.ration_capacity_of(orb);
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let dt = stamp.saturating_sub(held.last_touch);
        let grown = grained_logistic(&p0, &k, &intrinsic_rate(metabolism, self.metabolic_element()), dt);
        // Floor onto the LATTICE (the `stock.rs` §lattice ruling). The
        // logistic is an exact rational whose denominator compounds every
        // time its output is banked and fed back as the next `p0` — the
        // very 2^414305-after-16-ticks explosion that lattice-flooring
        // exists to stop, measured here at ~1 denominator-digit per collect
        // on the settle path. Quantizing to `LATTICE` bounds the
        // denominator forever, is a deterministic replayable floor (never a
        // round), and preserves the curve: a settle mints the floored
        // amount, its receipt carries it, and replay banks the same value.
        // No residue is owed — `YieldCollected` is an admitted free mint
        // with no conservation counterparty (the #28 settle law), so
        // minting the floored quantity leaks nothing.
        let lattice = crate::world::stock::lattice();
        (grown * lattice.clone()).floor() / lattice
    }

    /// **The settle — what a real TOUCH banks.** The stock the natural
    /// ration has grown by since the last touch, ready to fold into the
    /// touch's own `YieldCollected` receipt (the `40_REJUVENATION` §2
    /// digest, carried implicitly in the collected amount rather than a
    /// separate deed — "deterministic physics get no deed of their own").
    /// `None` when nothing grew: a stock already at or above `K` (a
    /// well-farmed world), a barren place, or a still-empty larder (the
    /// logistic is dead at zero — a stock must be reseeded, so an
    /// almanac-staple line regrows only once assay has struck its first
    /// vein). ING-2b: the ration is carrier-agnostic — a Resource stock or
    /// an almanac element both regrow here, read through `ration_amount`;
    /// the collect banks a Resource grown as `YieldCollected`, an element
    /// grown as `RationElementSettled`.
    ///
    /// Because the growth folds into `YieldCollected`, which the kernel
    /// already admits as a `+amount` yield mint and replay banks straight
    /// from the receipt, the whole settle is SEAL-B-safe with no new deed,
    /// no wire change, and no kernel arm.
    pub fn rejuvenation_settle(&self, orb: u64, stamp: u64) -> Option<(Ration, Ratio)> {
        self.rejuvenation_settle_with(orb, stamp, self.ration_of(orb))
    }

    /// `rejuvenation_settle` on a PRE-resolved ration (ING-2b PERF memo; see
    /// [`Self::regrown_ration_with`]).
    pub(super) fn rejuvenation_settle_with(
        &self,
        orb: u64,
        stamp: u64,
        ration: Ration,
    ) -> Option<(Ration, Ratio)> {
        let held = self.orb_by_id(orb)?;
        let grown = self.regrown_ration_with(orb, stamp, &ration) - held.ration_amount(&ration);
        (grown > ratio(0, 1)).then_some((ration, grown))
    }

    /// **docs/67 N4 — the sustainable offer, a scan.** How much of
    /// `resource` this orb may put on a market without fracturing its
    /// biosphere. `None` where `resource` is not the orb's regrowing ration
    /// (mined ore, banked crystal: not the biosphere's to keep). Where it is,
    /// the line keeps its stock at the curve's half point `K/2`, where the
    /// logistic regrows fastest: it may offer what it holds above `K/2`,
    /// and below `K/2` only the one beat's growth its stock makes now
    /// (harvest within regrowth keeps the stock, the regrowth law).
    /// Lattice-floored like every regrowth read.
    pub fn sustainable_offer(&self, orb: u64, resource: Resource) -> Option<Ratio> {
        let ration = self.ration_of(orb);
        if ration != Ration::Resource(resource) {
            return None;
        }
        let held = self.orb_by_id(orb)?;
        let stock = held.ration_amount(&ration);
        let k = self.ration_capacity_of(orb);
        let half_k = k.clone() / ratio(2, 1);
        let lattice = crate::world::stock::lattice();
        let floor = |x: Ratio| (x * lattice.clone()).floor() / lattice.clone();
        if stock > half_k {
            return Some(floor(stock - half_k));
        }
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let next = grained_logistic(&stock, &k, &intrinsic_rate(metabolism, self.metabolic_element()), 1);
        Some(floor(next - stock).max(ratio(0, 1)))
    }

    /// **ONT-3 — the vein rejuvenation settle.** At a collect touch every worked
    /// vein on the orb recovers. The injection is Keplerian: one
    /// `natural/REGROWTH_GRAIN` quantum per LOCAL YEAR elapsed since the vein's
    /// last book motion (`⌊now/P⌋ − ⌊last/P⌋`, exact off the two-clock model),
    /// amplified by the orb's topological recycling (`β₁` = the count of
    /// Regenerator return-pathways closing a metabolic loop), capped so a vein
    /// never recovers past its endowment. Lattice-floored (the ONT-2 denominator
    /// discipline) and receipted as `VeinRejuvenated` — a delta-0 element move
    /// that advances the vein book alone. Runs before the staple mine, so a vein
    /// that recovered this touch can feed the mouths the same span. A world with
    /// no star or no orbit rejuvenates nothing (no place, no forcing clock).
    pub(super) fn rejuvenate_orb_veins(&mut self, orb: u64, stamp: u64, actor: u64) {
        let Some(class) = self.star else { return };
        let mass = class.mass();
        let Some(orbit_r) = self
            .orb_by_id(orb)
            .and_then(|held| held.thermal.as_ref())
            .map(|site| site.orbit_r.value().clone())
        else {
            return;
        };
        // β₁ — the recycling return-pathways: the orb's Regenerator zones, the
        // loop-closing edge that raises the metabolic complex's first Betti
        // number ([[soc-society-buildout]] `SpeciesManifold`, read locally so
        // cfe carries no upward dependency on the conductor).
        let cycles = self
            .orb_by_id(orb)
            .map(|held| {
                held.zones
                    .iter()
                    .filter(|zone| matches!(zone.kind, ZoneKind::Regenerator))
                    .count() as u64
            })
            .unwrap_or(0);
        let now_years = crate::thermal::local_years(&orbit_r, &mass, stamp);
        let lattice = crate::world::stock::lattice();
        // Phase 1 (READ): the floored injection each vein has earned since its
        // last book motion. Immutable scan, so no borrow spans the mutation.
        let injections: Vec<([u8; 32], Ratio)> = self
            .prospects
            .iter()
            .filter(|vein| vein.orb == orb)
            .filter_map(|vein| {
                let (_, _, natural) = crate::dop::element_properties(&vein.element, class);
                let then_years = crate::thermal::local_years(&orbit_r, &mass, vein.last_assay);
                let orbits = now_years.saturating_sub(then_years);
                let raw = crate::almanac::vein_forcing_injection(
                    &natural,
                    &vein.extracted,
                    &vein.rejuvenated,
                    orbits,
                    cycles,
                );
                let inject = (raw * lattice.clone()).floor() / lattice.clone();
                (inject > ratio(0, 1)).then_some((vein.element, inject))
            })
            .collect();
        // Phase 2 (write): advance the vein book and receipt each recovery — the
        // mirror of the staple mine's `extracted += eaten`.
        for (element, inject) in injections {
            if let Some(vein) = self
                .prospects
                .iter_mut()
                .find(|vein| vein.orb == orb && vein.element == element)
            {
                vein.rejuvenated += inject.clone();
                vein.last_assay = stamp;
            }
            self.chronicle.append(
                crate::dop::ClosureEvent::VeinRejuvenated { orb_id: orb, element, injected: inject },
                stamp,
                vec![actor],
            );
        }
    }

    /// **The quarantine decay — what a sealed world loses at a touch**
    /// (task #30, ruled 2026-08-12). While a world is quarantined
    /// (bio-pressure past the threshold), its ration does not recover —
    /// it starves, decaying at the same intrinsic rate it would else
    /// grow: a derived flat `r·Δt`, the per-beat decay the ruling names,
    /// settled lazily at the touch exactly as the regrowth it replaces.
    /// Capped at the stock (never below zero). `None` when the world
    /// stands free, holds no ration, or is an element-eater (whose stock
    /// rides the element economy, #29).
    ///
    /// Folds into the touch as a resource debit (an `UpkeepSettled`) — the
    /// mirror of the regrowth credit — so it too is SEAL-B-safe with no
    /// new deed, wire, or kernel arm.
    pub fn quarantine_decay(&self, orb: u64, stamp: u64) -> Option<(Ration, Ratio)> {
        self.quarantine_decay_with(orb, stamp, self.ration_of(orb))
    }

    /// `quarantine_decay` on a PRE-resolved ration (ING-2b PERF memo).
    pub(super) fn quarantine_decay_with(
        &self,
        orb: u64,
        stamp: u64,
        ration: Ration,
    ) -> Option<(Ration, Ratio)> {
        if !self.world_quarantined() {
            return None;
        }
        let held = self.orb_by_id(orb)?;
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let dt = ratio(stamp.saturating_sub(held.last_touch) as i64, 1);
        let decay = (intrinsic_rate(metabolism, self.metabolic_element()) * dt)
            .min(held.ration_amount(&ration));
        (decay > ratio(0, 1)).then_some((ration, decay))
    }

    /// The ration delta a collect TOUCH settles: a starving debit while
    /// quarantined, else the recovery credit. One or the other, never
    /// both — a sealed world does not recover.
    pub fn ration_touch_delta(&self, orb: u64, stamp: u64) -> Option<(Ration, Ratio)> {
        self.ration_touch_delta_with(orb, stamp, self.ration_of(orb))
    }

    /// `ration_touch_delta` on a PRE-resolved ration (ING-2b PERF memo).
    pub(super) fn ration_touch_delta_with(
        &self,
        orb: u64,
        stamp: u64,
        ration: Ration,
    ) -> Option<(Ration, Ratio)> {
        if self.world_quarantined() {
            self.quarantine_decay_with(orb, stamp, ration)
                .map(|(ration, decay)| (ration, -decay))
        } else {
            self.rejuvenation_settle_with(orb, stamp, ration)
        }
    }

    /// **The population net-delta at a collect touch** (#40/#41/#42,
    /// `41_PROVISION` §2–3 / `42_LIFESPAN` §1–2). The elastic population
    /// eats its ration, births from the surplus toward `K_pop`, and turns
    /// over across its lifespan — all computed lazily at the touch from
    /// the beats elapsed since `last_touch`, the same clock the ration
    /// settle reads. Returns `(Δpop, source, consumed)`: `settle_population(Δpop)`
    /// receipts the headcount, `consumed` is the amount drawn from the primary
    /// (non-wild) [`FoodSource`] this span, and `source` is that source — the
    /// collect draws `consumed` from it, depleting it via its own receipt (a
    /// bank debit or a vein mine), the exact mirror of the regrowth credit, so
    /// the whole thing rides existing receipts and stays SEAL-B-safe.
    ///
    /// `None` when nothing is driven here: a world with no people, a
    /// zero-span touch, or a rate-0 capture line (the plasmoid, whose
    /// energy population rides the heat economy, not a farmed stock).
    ///
    /// SOURCE-1: the food available this span is summed over the line's derived
    /// [`FoodSource`]s ([`Self::sustenance_sources_with`]) — the wild flow it
    /// gathers plus the stock its metabolism grows (a Resource bank, or its
    /// mineral vein) — so this reads no `match ration`; the species → source
    /// mapping lives in the derivation alone.
    /// - **Fed** (`available ≥ demand`): the population grows an S-curve
    ///   toward its ceiling ([`grained_logistic`] on people, the #28
    ///   curve), less the ordinary mortality; it eats its full demand.
    /// - **Starved** (`available < demand`): only the people the span's
    ///   food can feed survive — the unfed die atop the mortality — and
    ///   all of it is eaten.
    pub fn population_touch_delta(
        &self,
        orb: u64,
        stamp: u64,
    ) -> Option<(Ratio, FoodSource, Ratio)> {
        self.population_touch_delta_with(orb, stamp, self.ration_of(orb))
    }

    /// `population_touch_delta` on a PRE-resolved ration (ING-2b PERF memo).
    /// The collect touch resolves the almanac staple once and threads it here
    /// and into the `natural` recursion, so `staple_diet_of`'s scan does not
    /// re-run per reader — a pure memoization (the ration is deterministic).
    pub(super) fn population_touch_delta_with(
        &self,
        orb: u64,
        stamp: u64,
        ration: Ration,
    ) -> Option<(Ratio, FoodSource, Ratio)> {
        // The population is exact-rational (COG Slice 2): the integer
        // headcount plus the fractional carry the logistic has accrued but
        // not yet promoted to a whole colonist. The logistic is a multiplier
        // on this mass, so a sub-one band (a founding seed part-way to its
        // next birth) still grows — reading only the integer count would
        // freeze it, and flooring its births would discard the growth.
        let pr = ratio(self.population_of(orb) as i64, 1) + self.pop_accrued_of(orb);
        if pr <= ratio(0, 1) {
            return None;
        }
        let held = self.orb_by_id(orb)?;
        let dt = stamp.saturating_sub(held.last_touch);
        if dt == 0 {
            return None;
        }
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let element = self.metabolic_element();
        // R6 — per-capita demand is the line's own half-harvest, modulated
        // by the sitting class (`per_capita_ration`): a costly archetype
        // eats more per head, a frugal one less, so all 14 classes (and any
        // minted class) provision distinctly, from the fold, no new dial. A
        // rate-0 capture line (plasmoid) has no farmed ration to draw on;
        // its population rides the energy economy — defer (#29).
        let per_capita = self.per_capita_ration();
        if per_capita <= ratio(0, 1) {
            return None;
        }
        let dtr = ratio(dt as i64, 1);
        let demand = pr.clone() * per_capita.clone() * dtr.clone();
        // R7b — mortality: the population turns over once per lifespan `L`,
        // so `⌊head·Δt/L⌋` whole colonists die this span. Kept integer (as
        // before Slice 2) and read on the whole headcount — turnover claims
        // whole people, and a long-lived line whose `head·Δt` has not yet
        // reached one lifespan neither grows nor dies (the leisure stall
        // holds exactly, `43_DISTRIBUTION` §3–4). Only BIRTHS carry a
        // fraction; deaths-by-turnover stay whole.
        let head = self.population_of(orb) as u128;
        let mortality = ratio(
            ((head * dt as u128) / self.lifespan_of().max(1) as u128) as i64,
            1,
        );
        // The FOOD available this SPAN — the standing bank plus what the
        // world produced over the same beats (the farmed yield accrued by
        // the ticks, and the natural regrowth/decay settling at this same
        // touch). The people eat from the period's whole harvest, not last
        // touch's leftover — a world that farms plenty feeds its people
        // however seldom its sovereign collects, the same path-independence
        // the lazy ration settle already gives. All three fold into this
        // touch, so `consumed ≤ available` keeps the settled bank ≥ 0.
        // SOURCE-1: derive this line's sustenance sources and read the food it
        // has this span off the vector — the wild flow it gathers (eaten first,
        // never banked) plus the stock its metabolism grows (a Resource bank +
        // this span's harvest, or its mineral vein's unextracted richness). The
        // species → source mapping lives only in `sustenance_sources_with`; here
        // the population just sums availability and nets the wild share — no
        // `match ration`, so a flux source (energy-eaters, #29) or a multi-source
        // diet slots in with no change to this loop.
        let sources = self.sustenance_sources_with(orb, stamp, &ration);
        let syntropic: Ratio = sources
            .iter()
            .filter(|source| source.is_wild())
            .map(|source| source.available())
            .fold(ratio(0, 1), |sum, amount| sum + amount);
        let available: Ratio = sources
            .iter()
            .map(|source| source.available())
            .fold(ratio(0, 1), |sum, amount| sum + amount);
        // The primary larder — the first non-wild source, the one the draw
        // depletes. `None` only where the derivation found no stock (an unsited
        // line), which the guards above have already turned back.
        let Some(larder) = sources.into_iter().find(|source| !source.is_wild()) else {
            return None;
        };
        if available >= demand {
            // Fed — births ride the #28 logistic on people toward K_pop,
            // less the ordinary mortality; the people eat their full demand.
            let k = self.population_capacity(orb);
            // Moving footprint (`44_LINEAGES` §2b): a mobile line in a medium
            // spreads faster — its intrinsic rate takes the growth boost. A
            // non-mobile line's factor is 1, so the #40–42 rate is unchanged.
            let r = intrinsic_rate(metabolism, element) * self.footprint_growth_factor(orb);
            let grown = grained_logistic(&pr, &k, &r, dt);
            // Stability gate (`43_DISTRIBUTION` §3–4): where a granary
            // stands, births scale by the leisure coverage — a fed but
            // un-entertained people stalls (births→0, only mortality runs)
            // rather than starving. Without a granary stability reads 1, so
            // this multiplies by one and the #40–42 path is byte-identical.
            // Exact births — the logistic's fractional output is preserved,
            // not floored (COG Slice 2). Every modulator is a rational factor
            // the settle later promotes into whole colonists as the carry
            // crosses one; nothing is truncated here.
            // TFR term III: births are gated by the unified economic contentment
            // `max(0, 1 − σ)` — leisure exhaustion and insolvency — not leisure
            // alone. It subsumes the old `stability_of` gate (a solvent, rested
            // line reads exactly `stability_of`, so unchanged) and adds the
            // wealth axis, precipitating the Sated Collapse. `43_DISTRIBUTION`.
            let births = (grown - pr.clone())
                * self.economic_contentment(orb, stamp)
                * self.flux_output_factor(orb);
            // DISPOSITION (§5): the cognition caste spends its share of
            // surplus growth on knowledge, not mouths — an exact fraction
            // (`cognition / people`) of births, so it too carries no floor.
            // Only a positive surplus is siphoned; a cull is never redirected.
            let people = self.population_of(orb);
            let births = if births > ratio(0, 1) && people > 0 {
                let cognition = self.caste_of(orb, crate::dop::Caste::Cognition);
                let kept = ratio(1, 1) - ratio(cognition as i64, 1) / ratio(people as i64, 1);
                births * kept
            } else {
                births
            };
            // The wild flow is eaten first (COG Slice 2c, made real at the
            // ledger). The syntropic food the band gathers is free and drawn
            // before any cultivated/banked stock, so only the demand beyond the
            // wild flow debits the bank. A founding band whose whole demand sits
            // under the syntropic flow draws nothing from the granary — it lives
            // by gathering, and the industrial surplus it never touches is
            // preserved. The bank is pressured only once the population outgrows
            // the wild flow — which is the cultivation trigger (the standing
            // deficit that drives the Mind). This is the third element: the
            // amount economy.rs debits from the stored bank, wild food already
            // netted out.
            let bank_draw = (demand - syntropic).max(ratio(0, 1));
            Some((births - mortality, larder, bank_draw))
        } else {
            // Starved — only the fraction of the band the span's food can
            // feed survives; the unfed die atop the ordinary mortality, and
            // all of it is eaten. Exact-rational: a transient shortfall shaves
            // a fraction of a colonist rather than removing a whole one, so a
            // fed line is not collapsed by one lean touch.
            let fed = available.clone() / (per_capita * dtr);
            let unfed = (pr - fed).max(ratio(0, 1));
            // The bank draw nets out the wild flow here too: a starving band
            // eats everything it can reach, but the syntropic share was
            // gathered free, so only the non-wild remainder (bank + harvested +
            // natural) debits the granary — which is exactly what stands there.
            let bank_draw = (available - syntropic).max(ratio(0, 1));
            Some((-(mortality + unfed), larder, bank_draw))
        }
    }
}
