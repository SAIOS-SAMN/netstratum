//! **PLANETARY REJUVENATION — the lazy READ** (task #28, `40_REJUVENATION`).
//!
//! A world's natural RATION — biomass for a biotic line, iron for a
//! mineral line, a minted element for a stone-eater (the G2 map,
//! [`crate::dop::metabolic_ration_of`]) — recovers toward a carrying
//! capacity `K` along the ruled S-curve
//! ([`crate::almanac::grained_logistic`]). Nothing here is hardcoded to
//! biomass: what a world regrows, how fast, and toward what ceiling all
//! FOLLOW the sitting line's own metabolism and clade, the same
//! species⊗planet⊗archetype composition every other flow reads
//! (`zone_yield`, `taxon_economy_of`). Deterministic physics get no
//! deed: the curve is recomputed from `(P0, K, r, Δt)` at need, exactly
//! as demurrage's decay is (`World::purse`) — a READ settles nothing
//! and writes nothing, and `Δt` runs from the orb's `last_touch`, the
//! same clock the demurrage organ already carries.
//!
//! This is the read-only half (a planet SCAN). The mutating half — the
//! settle at a value-moving touch, with its receipt digest — banks the
//! grown stock and advances `last_touch`, and lands with the wiring.

use super::*;
use crate::almanac::{carrying_capacity, grained_logistic, intrinsic_rate, region_of, FLATLANDER_CLADE, ThermalRegion};
use crate::dop::{metabolic_harvest_of, metabolic_ration_of, Ration, ZoneKind};

impl World {
    /// The living ECOLOGICAL FOOTPRINT of an orb — its habitats and
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

    /// The thermal REGION this orb sits in, or `None` where it has no
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

    /// The RATION this orb's line farms and eats — a closed [`Resource`]
    /// carrier for the three compiled metabolisms, or the line's own
    /// minted element for a G2 element-eater. Reads the same genesis
    /// metabolism and vocabulary fold every other flow does
    /// (`World::metabolic_element`).
    ///
    /// [`Resource`]: crate::dop::Resource
    pub fn ration_of(&self, _orb: u64) -> Ration {
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        metabolic_ration_of(metabolism, self.metabolic_element())
    }

    /// **THE CARRYING CAPACITY of this orb's ration** — the ruled `K`,
    /// from the living footprint and the place (`ZoneKind` ⊗ star ⊗
    /// orbit, the last two folded into the thermal region) AS READ BY
    /// THE SITTING CLADE. An unsited orb, or one on a barren rock (for
    /// this clade), carries none.
    pub fn ration_capacity_of(&self, orb: u64) -> Ratio {
        match self.orb_region(orb) {
            Some(region) => {
                carrying_capacity(self.living_zones_of(orb), self.rejuvenation_clade(), region)
            }
            None => ratio(0, 1),
        }
    }

    /// **THE REGROWN STOCK — a SCAN, never a deed.** What this orb's
    /// banked ration has recovered to as of `stamp`, folding the ruled
    /// S-curve over the beats elapsed since its `last_touch`. Pure and
    /// replay-inert: looking at a planet is not a chain event, so it
    /// settles nothing and advances no clock — the grown value banks for
    /// real only at the next genuine touch (the demurrage precedent,
    /// `World::purse`).
    pub fn regrown_ration(&self, orb: u64, stamp: u64) -> Ratio {
        let Some(held) = self.orb_by_id(orb) else {
            return ratio(0, 1);
        };
        let ration = self.ration_of(orb);
        let p0 = held.ration_amount(&ration);
        let k = self.ration_capacity_of(orb);
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let dt = stamp.saturating_sub(held.last_touch);
        let grown = grained_logistic(&p0, &k, &intrinsic_rate(metabolism, self.metabolic_element()), dt);
        // FLOOR ONTO THE LATTICE (the `stock.rs` §lattice ruling). The
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

    /// **THE SETTLE — what a real TOUCH banks.** The stock the natural
    /// ration has GROWN by since the last touch, ready to fold into the
    /// touch's own `YieldCollected` receipt (the `40_REJUVENATION` §2
    /// digest, carried implicitly in the collected amount rather than a
    /// separate deed — "deterministic physics get no deed of their own").
    /// `None` when nothing grew: a stock already at or above `K` (a
    /// well-farmed world), a barren place, or — for now — an
    /// element-eater, whose ration settles with the element economy
    /// (task #29). Only a Resource ration below its ceiling regrows here.
    ///
    /// Because the growth folds into `YieldCollected`, which the kernel
    /// already admits as a `+amount` yield mint and replay banks straight
    /// from the receipt, the whole settle is SEAL-B-safe with no new deed,
    /// no wire change, and no kernel arm.
    pub fn rejuvenation_settle(&self, orb: u64, stamp: u64) -> Option<(Resource, Ratio)> {
        let Ration::Resource(resource) = self.ration_of(orb) else {
            return None;
        };
        let held = self.orb_by_id(orb)?;
        let grown = self.regrown_ration(orb, stamp) - held.banked_amount(resource);
        (grown > ratio(0, 1)).then_some((resource, grown))
    }

    /// **THE QUARANTINE DECAY — what a sealed world LOSES at a touch**
    /// (task #30, ruled 2026-08-12). While a world is quarantined
    /// (bio-pressure past the threshold), its ration does not recover —
    /// it STARVES, decaying at the same intrinsic rate it would else
    /// grow: a derived flat `r·Δt`, the per-beat decay the ruling names,
    /// settled lazily at the touch exactly as the regrowth it replaces.
    /// Capped at the stock (never below zero). `None` when the world
    /// stands free, holds no ration, or is an element-eater (whose stock
    /// rides the element economy, #29).
    ///
    /// Folds into the touch as a resource DEBIT (an `UpkeepSettled`) — the
    /// mirror of the regrowth credit — so it too is SEAL-B-safe with no
    /// new deed, wire, or kernel arm.
    pub fn quarantine_decay(&self, orb: u64, stamp: u64) -> Option<(Resource, Ratio)> {
        if !self.world_quarantined() {
            return None;
        }
        let Ration::Resource(resource) = self.ration_of(orb) else {
            return None;
        };
        let held = self.orb_by_id(orb)?;
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let dt = ratio(stamp.saturating_sub(held.last_touch) as i64, 1);
        let decay = (intrinsic_rate(metabolism, self.metabolic_element()) * dt)
            .min(held.banked_amount(resource));
        (decay > ratio(0, 1)).then_some((resource, decay))
    }

    /// The ration delta a collect TOUCH settles: a starving DEBIT while
    /// quarantined, else the recovery CREDIT. One or the other, never
    /// both — a sealed world does not recover.
    pub fn ration_touch_delta(&self, orb: u64, stamp: u64) -> Option<(Resource, Ratio)> {
        if self.world_quarantined() {
            self.quarantine_decay(orb, stamp)
                .map(|(resource, decay)| (resource, -decay))
        } else {
            self.rejuvenation_settle(orb, stamp)
        }
    }

    /// **THE POPULATION NET-DELTA at a collect touch** (#40/#41/#42,
    /// `41_PROVISION` §2–3 / `42_LIFESPAN` §1–2). The elastic population
    /// eats its ration, births from the surplus toward `K_pop`, and turns
    /// over across its lifespan — all computed lazily at the touch from
    /// the beats elapsed since `last_touch`, the same clock the ration
    /// settle reads. Returns `(Δpop, resource, consumed)`:
    /// `settle_population(Δpop)` receipts the headcount, and `consumed`
    /// folds into the touch's ration as a DEBIT (the food eaten this
    /// span), the exact mirror of the regrowth credit — so the whole
    /// thing rides existing receipts and stays SEAL-B-safe.
    ///
    /// `None` when nothing is driven here: a world with no people, a
    /// zero-span touch, an element-eater (its ration rides #29), or a
    /// rate-0 CAPTURE line (the plasmoid, whose energy population rides
    /// the heat economy, not a farmed stock). Only a farming Resource
    /// line with standing people is driven.
    ///
    /// The food AVAILABLE this span is the standing bank plus the harvest
    /// the same beats produced (farmed yield + natural regrowth), so a
    /// world that farms plenty feeds its people however seldom it collects.
    /// - **FED** (`available ≥ demand`): the population grows an S-curve
    ///   toward its ceiling ([`grained_logistic`] on people, the #28
    ///   curve), less the ordinary mortality; it eats its full demand.
    /// - **STARVED** (`available < demand`): only the people the span's
    ///   food can feed survive — the unfed die atop the mortality — and
    ///   all of it is eaten.
    pub fn population_touch_delta(&self, orb: u64, stamp: u64) -> Option<(i64, Resource, Ratio)> {
        let p = self.population_of(orb);
        if p == 0 {
            return None;
        }
        let Ration::Resource(resource) = self.ration_of(orb) else {
            return None;
        };
        let held = self.orb_by_id(orb)?;
        let dt = stamp.saturating_sub(held.last_touch);
        if dt == 0 {
            return None;
        }
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let element = self.metabolic_element();
        // R6 — per-capita demand is HALF the line's own farming rate: a
        // PAIR shares a habitat's yield. The 2 is structural (the pair),
        // DERIVED as 1+1 rather than dialed — a bare two-to-one literal is
        // a tuned dial the census bars, but an identity ratio is algebra
        // (value 1 is uncounted), the same derive-don't-dial discipline as
        // `grain*4`. A rate-0 capture line (plasmoid) has no farmed ration
        // to draw on; its population rides the energy economy — defer (#29).
        let pair = ratio(1, 1) + ratio(1, 1);
        let per_capita = metabolic_harvest_of(metabolism, element).1 / pair;
        if per_capita <= ratio(0, 1) {
            return None;
        }
        let pr = ratio(p as i64, 1);
        let dtr = ratio(dt as i64, 1);
        let demand = pr.clone() * per_capita.clone() * dtr.clone();
        // R7b — mortality: the population turns over once per lifespan `L`,
        // so `p·Δt/L` die this span. u128 math floored, never zero-div
        // (`lifespan_of` ≥ one grain).
        let mortality = ((p as u128 * dt as u128) / self.lifespan_of() as u128) as i64;
        // THE FOOD AVAILABLE THIS SPAN — the standing bank PLUS what the
        // world produced over the same beats (the farmed yield accrued by
        // the ticks, and the natural regrowth/decay settling at this same
        // touch). The people eat from the period's whole harvest, not last
        // touch's leftover — a world that farms plenty feeds its people
        // however seldom its sovereign collects, the same path-independence
        // the lazy ration settle already gives. All three fold into this
        // touch, so `consumed ≤ available` keeps the settled bank ≥ 0.
        let harvested: Ratio = self
            .accrued_of(orb)
            .into_iter()
            .filter(|(res, _)| *res == resource)
            .map(|(_, amount)| amount)
            .fold(ratio(0, 1), |sum, amount| sum + amount);
        let natural = match self.ration_touch_delta(orb, stamp) {
            Some((res, signed)) if res == resource => signed,
            _ => ratio(0, 1),
        };
        let available = held.banked_amount(resource) + harvested + natural;
        if available >= demand {
            // FED — births ride the #28 logistic on PEOPLE toward K_pop,
            // less the ordinary mortality; the people eat their full demand.
            let k = self.population_capacity(orb);
            let r = intrinsic_rate(metabolism, element);
            let grown = grained_logistic(&pr, &k, &r, dt);
            // STABILITY GATE (`43_DISTRIBUTION` §3–4): where a granary
            // stands, births scale by the leisure coverage — a fed but
            // un-entertained people STALLS (births→0, only mortality runs)
            // rather than starving. Without a granary stability reads 1, so
            // this multiplies by one and the #40–42 path is byte-identical.
            let births_raw = (grown - pr).floor();
            let births: i64 = (births_raw * self.stability_of(orb, stamp))
                .floor()
                .to_integer()
                .try_into()
                .unwrap_or(0);
            Some((births - mortality, resource, demand))
        } else {
            // STARVED — only the people the span's food can feed survive;
            // the unfed die atop the ordinary mortality, and all of it is
            // eaten.
            let fed: i64 = (available.clone() / per_capita / dtr)
                .floor()
                .to_integer()
                .try_into()
                .unwrap_or(0);
            let unfed = (p as i64 - fed).max(0);
            Some((-(mortality + unfed), resource, available))
        }
    }
}
