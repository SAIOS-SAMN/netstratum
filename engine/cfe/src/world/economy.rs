//! Economy deeds — collect, transfer, absorb, offers, the purse.
//!
//! Every deed here touches banked value, so every one of them settles
//! demurrage at the touch through `super::rules`. That is the organ's
//! single sealed-rule surface and it is deliberate.

use super::*;
use crate::dop::Ration;

impl World {
    /// MARKET-1: the resting offers.
    pub fn offers(&self) -> &[Offer] {
        &self.offers
    }
    /// MARKET-1 / E-MKT-1: post an offer — a receipted intent to give
    /// one cargo for another at an exact-rational price (want/give —
    /// no absolute price, guardrail 6). Cargo is Resource or Element.
    /// No escrow: the fill gates affordability when a crossing clears.
    pub fn post_offer(
        &mut self,
        orb: u64,
        pair: crate::dop::Exchange,
        give_amount: crate::dop::Positive,
        want_amount: crate::dop::Positive,
        offer: u64,
    ) -> Result<(), ActionRefused> {
        // The reflexive trade and the non-positive amounts died at the
        // `Exchange` and `Positive` doors (29_CONSTRUCTION); the
        // fresh-id fact keeps its own name.
        let give = pair.give();
        let want = pair.want();
        let give_amount = give_amount.value().clone();
        let want_amount = want_amount.value().clone();
        // P8.1 — shared AP (quote family).
        self.require_ap(crate::world::ApKind::Quote)?;
        if self.offers.iter().any(|held| held.offer == offer) {
            return Err(ActionRefused::OfferKnown { offer });
        }
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        let posted = self.clock.now();
        self.offers.push(Offer {
            offer,
            orb,
            give: give.clone(),
            give_amount: give_amount.clone(),
            want: want.clone(),
            want_amount: want_amount.clone(),
            posted,
        });
        self.close(
            ClosureEvent::OfferPosted {
                offer,
                orb_id: orb,
                give,
                give_amount,
                want,
                want_amount,
            },
            SOVEREIGN,
        );
        Ok(())
    }
    /// **Convey** (tag 100, 2026-08-06): the postal deed — the estate
    /// chain's last link. World-contained (cross-world rides the
    /// aperture pair). Refusal-pure, checks in address → cap → afford
    /// order:
    ///
    /// 1. Address: both endpoints must stand in the standing survey's
    ///    covering — a postal address is a surveyed parcel
    ///    (`ClaimWithoutWitness`, the custody refusal's fourth use).
    /// 2. Monetary cap: the ask may not exceed the sender parcel's
    ///    monetary portion of its bank — the free portion stays home
    ///    (`ConveyExceedsMonetary`).
    /// 3. Postage: `postal_meter().1 × amount`, drawn from the sender
    ///    with the amount, burned like upkeep — the amount arrives
    ///    whole (`CannotAfford` when bank < amount + postage).
    pub fn convey(
        &mut self,
        route: crate::dop::Between,
        resource: Resource,
        amount: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.convey_as(SOVEREIGN, route, resource, amount)
    }

    /// Native to the fourth elimination: `Between` proves the postal route has
    /// two distinct endpoints and `Positive` proves the amount — a
    /// self-conveyance and a zero conveyance are not deeds that can be
    /// asked for. The old `BadTransfer` arm died with the shapes it
    /// guarded.
    pub fn convey_as(
        &mut self,
        actor: u64,
        route: crate::dop::Between,
        resource: Resource,
        amount: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        let (from, to) = (route.from(), route.to());
        let amount: Ratio = amount.value().clone();
        if smelt_resource(&amount) != amount {
            return Err(ActionRefused::OffGrain { amount: Box::new(amount) });
        }
        let Some((survey, seed, covered)) =
            self.chronicle.last_survey().and_then(|(own, event)| match event {
                ClosureEvent::EstateSurveyed { seed, covered, .. } => Some((
                    *own,
                    *seed,
                    covered.iter().map(|(orb, _)| *orb).collect::<Vec<u64>>(),
                )),
                _ => None,
            })
        else {
            return Err(ActionRefused::ClaimWithoutWitness { name: "estate unsurveyed".into() });
        };
        for endpoint in [from, to] {
            if !covered.contains(&endpoint) {
                return Err(ActionRefused::ClaimWithoutWitness {
                    name: format!("orb {endpoint} has no postal address — not in the survey"),
                });
            }
        }
        if peek(&self.orbs, to).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb: to });
        }
        let monetary = crate::dop::parcels_of(&seed, &covered)
            .into_iter()
            .find(|parcel| parcel.orb_id == from)
            .map(|parcel| parcel.monetary)
            .expect("covered above");
        let sender = peek(&self.orbs, from).ok_or(ActionRefused::NoSuchOrb { orb: from })?;
        let cap = monetary.clone() * sender.banked_amount(resource);
        if amount > cap {
            return Err(ActionRefused::ConveyExceedsMonetary {
                asked: Box::new(amount),
                monetary: Box::new(cap),
            });
        }
        let (stamp, _) = crate::dop::postal_meter();
        let postage = crate::dop::postage(&amount);
        let have = sender.banked_amount(stamp);
        let need = match stamp == resource {
            true => amount.clone() + postage.clone(),
            false => postage.clone(),
        };
        if have < need {
            return Err(ActionRefused::CannotAfford {
                resource: stamp,
                need: Box::new(need),
                have: Box::new(have),
            });
        }
        let held = peel(&mut self.orbs, from).expect("checked above");
        held.bank(resource, -amount.clone());
        held.bank(stamp, -postage.clone());
        held.last_touch = self.clock.now();
        let dest = peel(&mut self.orbs, to).expect("checked above");
        dest.bank(resource, amount.clone());
        dest.last_touch = self.clock.now();
        self.close(
            ClosureEvent::ParcelConveyed {
                from_orb: from,
                to_orb: to,
                resource,
                amount,
                postage,
                survey,
                seed,
                covered,
            },
            actor,
        );
        Ok(())
    }

    /// **RCP-2 F2 — the built-recipe yield at a collect touch.** Every
    /// emergent recipe a society has raised on this orb delivers its
    /// `yield_of_role` of the role's resource per beat; over the `dt` beats
    /// since `last_touch` it yields `yield_of_role · dt`, folded into this
    /// touch's settlement so it rides the existing `YieldCollected` receipt
    /// (no new deed, SEAL-B-safe — the rejuvenation/population pattern).
    /// A role with no bankable home yet (`role_resource` → `None`) yields
    /// nothing — the honest hole. Empty for a world that raised no recipe, so
    /// a v1 world is byte-identical. Read before the peel, like every other
    /// touch-delta, so no borrow of `self.orbs` is held across it.
    fn recipe_yield_at_touch(&self, orb: u64, stamp: u64) -> Vec<(Resource, Ratio)> {
        let Some(held) = self.orb_by_id(orb) else {
            return Vec::new();
        };
        let dt = stamp.saturating_sub(held.last_touch);
        if dt == 0 {
            return Vec::new();
        }
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let dtr = ratio(dt as i64, 1);
        let mut out: Vec<(Resource, Ratio)> = Vec::new();
        for recipe in crate::vocabulary::recipes_built(self.chronicle()) {
            if recipe.orb_id != orb {
                continue;
            }
            let Some(resource) = crate::dop::role_resource(recipe.role, metabolism) else {
                continue; // an honest hole — no bankable home for this role yet
            };
            let amount = recipe.yield_of_role * dtr.clone();
            match out.iter_mut().find(|(held_res, _)| *held_res == resource) {
                Some((_, held_amount)) => *held_amount += amount,
                None => out.push((resource, amount)),
            }
        }
        out
    }

    /// Collect the genesis orb as the sovereign.
    pub fn collect(&mut self) -> Result<usize, ActionRefused> {
        self.collect_as(SOVEREIGN, FIRST_ORB)
    }
    pub fn collect_as(&mut self, actor: u64, orb: u64) -> Result<usize, ActionRefused> {
        // P2: the standing supply the sink's epoch 1 rate reads,
        // snapshotted before any orb is peeled — an owned Vec, so no
        // borrow of `self.orbs` is held across the mutable touch below.
        let supply = standing_supply(&self.orbs);
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        // MASK-1a: the veil's standing toll settles at the touch — an
        // unpayable toll lifts the veil by receipt, never silently.
        if self.veiled {
            let toll = crate::parameter::of(self.chronicle(), crate::parameter::Parameter::VeilToll);
            let held_embers = self
                .orbs
                .iter()
                .find(|held| held.orb_id == orb)
                .map(|held| held.banked_amount(Resource::Ember))
                .unwrap_or_else(|| ratio(0, 1));
            if held_embers >= toll {
                if let Some(held) = peel(&mut self.orbs, orb) {
                    held.bank(Resource::Ember, -toll.clone());
                }
                let stamp = self.clock.now();
                self.chronicle.append(
                    ClosureEvent::UpkeepSettled { orb_id: orb, resource: Resource::Ember, amount: toll },
                    stamp,
                    vec![actor],
                );
            } else {
                let stamp = self.clock.now();
                self.chronicle.append(
                    ClosureEvent::VeilLifted { starved: true },
                    stamp,
                    vec![actor],
                );
                self.veiled = false;
            }
        }
        let encasement_rate = crate::parameter::of(
            self.chronicle(),
            crate::parameter::Parameter::EncasementThroughput,
        );
        // Rejuvenation (task #28) / quarantine decay (task #30): the
        // natural ration's delta since the last touch settles here — a
        // recovery credit for a free world, a starving debit for a
        // quarantined one — the exact mirror of the demurrage decay
        // `touch()` seals just below, computed before the touch so it
        // reads the old `last_touch`. It folds into this touch's own
        // yield receipt (no deed of its own — the §2 digest rides the
        // settled amount), so replay banks it straight from the receipt
        // and `last_touch` advances by the same carrier.
        // ING-2b PERF: resolve this line's ration (the almanac-staple scan,
        // `ration_of`) once for the whole touch and thread it through both the
        // rejuvenation settle and the population delta (and its `natural`
        // recursion), instead of each reader re-resolving it — the same value,
        // computed once. The staple depends only on the (unchanging this
        // touch) prospects/genesis, so this is a pure memoization.
        let ration = self.ration_of(orb);
        let rejuvenated = self.ration_touch_delta_with(orb, self.clock.now(), ration.clone());
        // Population (task #40/#41/#42): the elastic population eats its
        // ration, births from the surplus, and turns over — computed here
        // from the pre-touch state (the old `last_touch`, the old bank),
        // exactly as the regrowth is. The food eaten folds into this
        // touch's ration as a debit (`pop_consumed`); the net headcount
        // change (`pop_delta`) receipts as its own `PopulationSettled`
        // after the touch, at the same stamp.
        let population = self.population_touch_delta_with(orb, self.clock.now(), ration);
        // G2 (#29): the food-element an element-eater synthesizes this
        // touch (ember → element, conserved). Read pre-touch; settled as
        // its own ElementFarmed receipt after the touch. `None` for every
        // non-eater, so a v1 world is byte-identical.
        let farm = self.element_farm_at_touch(orb, self.clock.now());
        // G2 trace (2026-08-14): the trace elements the population burns
        // maintaining its substrate this span — read pre-touch (over the
        // span just ended), settled as `TraceConsumed` debits after the
        // touch. Empty for a world with no veins or no population, so it is
        // byte-identical.
        let trace = self.trace_consumption_at_touch(orb, self.clock.now());
        // Taxation (49 §5): the welfare outflow is read here, before the
        // touch advances `last_touch` — the demand it funds is measured
        // over the span just ended (the same pre-touch reading as the
        // population and the regrowth). Applied to the treasury after the
        // touch, at the same stamp.
        let welfare = self.welfare_spend(orb, self.clock.now());
        // Taxation (49 §4): the estate tax at a generation turnover — every
        // L beats a generation turns and its population-proportional estate
        // is taxed to the treasury. The boundaries crossed this span are
        // read from the pre-touch `last_touch` (like the welfare/population),
        // so a long-quiet colony settles all crossings at once.
        let lifespan = self.lifespan_of().max(1);
        let last_touch = self.orb_by_id(orb).map(|held| held.last_touch).unwrap_or(0);
        let generations = self.clock.now() / lifespan - last_touch / lifespan;
        let estate = self.estate_due_at_touch(orb, generations);
        // Taxation (49 §3): the continuity levy — practices the line does
        // not keep across the same generation turnover lapse to the void.
        let lapse = self.continuity_lapse_at_touch(generations);
        // RCP-2 F2: the built recipes' yield over the span just ended — read
        // pre-touch (over the old `last_touch`) like the regrowth and the
        // population, folded into this touch's settlement below. Empty until a
        // recipe is raised, so a v1 world is byte-identical.
        let recipe_yields = self.recipe_yield_at_touch(orb, self.clock.now());
        let held = peel(&mut self.orbs, orb)
            .ok_or(ActionRefused::NoSuchOrb { orb })?;
        let mut settleable: Vec<(Resource, Ratio)> = Vec::new();
        self.accrued.retain(|(owner, resource, amount)| {
            match *owner == orb && amount != &ratio(0, 1) {
                true => {
                    settleable.push((*resource, amount.clone()));
                    false
                }
                false => *owner != orb,
            }
        });
        // ONT-2b: the food settle rides whichever carrier this line's ration
        // is. A Resource ration folds into `settleable` (YieldCollected, as
        // ever). An almanac-ELEMENT staple no longer routes through the element
        // bank — the vein is the larder: the band's `bank_draw` this span (the
        // demand beyond the free wild flow) is captured here and mined from the
        // staple vein after the touch (`StapleMined`, `extracted += eaten`).
        // `(element, bank_draw)`; resolved to the floored, richness-capped
        // `eaten` below. A Resource line leaves it `None`, byte-identical.
        let mut staple_mine: Option<([u8; 32], Ratio)> = None;
        // Fold the regrown ration into this touch's yield — merged with
        // its resource where the tick also accrued some, otherwise its
        // own line. A world can collect on regrowth ALONE, so this joins
        // before the empty gate.
        if let Some((ration, grown)) = rejuvenated {
            match ration {
                Ration::Resource(resource) => {
                    match settleable.iter_mut().find(|(held_res, _)| *held_res == resource) {
                        Some((_, amount)) => *amount += grown,
                        None => settleable.push((resource, grown)),
                    }
                }
                // ONT-2b: element regrowth no longer feeds the food economy —
                // the vein is a finite larder that boom-busts (no regeneration
                // until ONT-3 #109, and folding the collapse-prone regrowth
                // logistic here is the very ONT-3 scalar this arc eradicates).
                // Food is mined from the vein by eating (below); the grown
                // credit is dropped.
                Ration::Element(_) => {}
            }
        }
        // Fold the food the population ate this span into the same touch as
        // a debit — the mirror of the regrowth credit. The population's
        // `consumed` is computed against what it believed was available
        // (bank + harvested + natural, rejuvenation.rs), but a colony can
        // hold people while its harvest fails to materialise this touch —
        // so the belief can outrun the coin actually in hand. A mind may
        // only ever eat what stands: the pre-touch bank plus the credits
        // this same touch banks (yield + regrown). Clamp the debit to that
        // ceiling so the settled bank can never cross zero — a starving
        // colony eats itself to empty and stops, it does not overdraw and
        // strike a negative-amount famine the kernel must refuse. The
        // element carrier holds the same law against the element larder.
        // SOURCE-1: the draw dispatches on the primary FoodSource the population
        // read (not `match ration`) — the source's own nature says how eating it
        // depletes it. A Resource bank debits (clamped to what stands); a mineral
        // vein is mined (captured here, extracted below). A new source kind (a
        // flux field, #29) adds an arm here, not a fork through the economy.
        if let Some((_, source, consumed)) = &population {
            match source {
                FoodSource::Bank { resource, .. } => {
                    let credit = settleable
                        .iter()
                        .find(|(held_res, _)| held_res == resource)
                        .map(|(_, amount)| amount.clone())
                        .unwrap_or_else(|| ratio(0, 1));
                    let standing = (held.banked_amount(*resource) + credit).max(ratio(0, 1));
                    let eaten = consumed.clone().min(standing);
                    match settleable.iter_mut().find(|(held_res, _)| held_res == resource) {
                        Some((_, amount)) => *amount -= eaten,
                        None => settleable.push((*resource, -eaten)),
                    }
                }
                // ONT-2b: eating mines the staple vein. The band's `bank_draw`
                // this span (the demand beyond the free wild flow) is captured
                // here and extracted from the staple vein after the touch — not
                // debited from the element bank (the bank is bypassed for food;
                // the vein is the larder). The richness cap and lattice floor
                // are applied against the live vein below, where `StapleMined`
                // grows `extracted` and receipts the pull.
                FoodSource::Vein { element, .. } => {
                    staple_mine = Some((*element, consumed.clone()));
                }
                // The primary larder is never the wild flow (it is gathered
                // free, netted out of `consumed`); this arm cannot arise.
                FoodSource::Wild(_) => {}
            }
        }
        // RCP-2 F2: fold each built recipe's span yield into this touch, its
        // role's resource merged with any tick-accrued line — the rejuvenation
        // credit pattern. A world can collect on a recipe's yield ALONE, so
        // this joins before the empty gate below.
        for (resource, amount) in recipe_yields {
            match settleable.iter_mut().find(|(held_res, _)| *held_res == resource) {
                Some((_, held_amount)) => *held_amount += amount,
                None => settleable.push((resource, amount)),
            }
        }
        // ONT-2b: a line whose only movement this touch is mining its staple
        // vein (a positive `bank_draw`) has accrued something — the vein pull
        // counts against the empty gate, exactly as the Resource yield does. A
        // zero draw (the band lives wholly on the free wild flow) does not.
        let staple_mines = staple_mine
            .as_ref()
            .map(|(_, draw)| *draw > ratio(0, 1))
            .unwrap_or(false);
        // #138 the resource GRAIN, the gate's own GRAIN — `settleable`
        // being non-empty is not enough: every entry in it (yield or
        // upkeep draw) smelts to the 1/1024 grain below, and a sub-grain
        // entry banks nothing and receipts nothing. `touch()` anchors
        // `last_touch` unconditionally the moment it is called — its own
        // law is "a zero-due touch still anchors silently, the carrier
        // deed's own replay arm carries the anchor" — so calling it over
        // a `settleable` that smelts to nothing entirely breaks that
        // promise: no receipt follows, live marches on, replay never
        // hears it (the last_touch divergence `lineage_pins` pinned).
        // Gate on survives-the-GRAIN, not merely non-empty.
        let settles_on_grain = settleable.iter().any(|(_, amount)| {
            let magnitude = match *amount >= ratio(0, 1) {
                true => amount.clone(),
                false => -amount.clone(),
            };
            smelt_resource(&magnitude) != ratio(0, 1)
        });
        if !settles_on_grain && farm.is_none() && !staple_mines {
            return Err(ActionRefused::NothingAccrued);
        }
        // Taxation (49 §5): the taxable throughput this touch — the net
        // yield banked (positive lines only), the Trade base a
        // Touch-triggered revenue levy draws on. Snapshotted here, before
        // the receipts consume `settleable`; the levy settles into the
        // treasury after the population, at the same stamp.
        let taxable: Ratio = settleable
            .iter()
            .filter(|(_, amount)| *amount > ratio(0, 1))
            .fold(ratio(0, 1), |sum, (_, amount)| sum + amount.clone());
        let orb_id = held.orb_id;
        let stamp = self.clock.now();
        let count = settleable.len();
        // SINK-1 (Q2): receipted demurrage at touch — un-vaulted banked
        // embers/sink decay for the elapsed span, settled openly.
        let ease = ease_of(&self.techs, orb_id, stamp);
        touch(held, self.star, &mut self.chronicle, stamp, &ease, &supply);
        // VENT-1: seal the pending vented entropy into the ledger.
        let mut vented = ratio(0, 1);
        self.vent_pending.retain(|(held_orb, pending)| match *held_orb == orb_id {
            true => {
                vented += pending.clone();
                false
            }
            false => true,
        });
        if vented > ratio(0, 1) {
            held.entropy_vented += vented.clone();
            self.chronicle.append(
                ClosureEvent::VentSettled { orb_id, amount: vented },
                stamp,
                vec![actor],
            );
        }
        // IDEO-1: the accrued ideological friction seals into the same
        // vented ledger — real entropy, its own receipt.
        let mut friction = ratio(0, 1);
        self.friction_pending.retain(|(held_orb, pending)| match *held_orb == orb_id {
            true => {
                friction += pending.clone();
                false
            }
            false => true,
        });
        if friction > ratio(0, 1) {
            // Encasement first, now obeyed here too (`EQUATIONS` T3,
            // 05 §1.2; operator ruling 2026-08-03: friction entropy is
            // thermodynamically identical to thermal entropy — the same
            // physical currency, so the same capture law).
            //
            // This block used to vent 100% straight to the void while
            // writing into the very ledger whose own law puts capture
            // first. Heat is encased and only the overflow disperses;
            // friction declined that arm rather than lacking one.
            //
            // No invented constant: the shells, their throughput and
            // the TECH-1b discipline boon are read exactly as the
            // per-tick heat path reads them.
            let shells = held
                .zones
                .iter()
                .filter(|zone| held.zone_active(zone) && zone.kind == ZoneKind::Encasement)
                .count() as i64;
            let mut throughput = encasement_rate * ratio(shells, 1);
            if self
                .techs
                .iter()
                .any(|(owner, condition, _, _)| *owner == orb_id && *condition == 2)
            {
                throughput *= ratio(5, 4);
            }
            let captured = friction.clone().min(throughput);
            let overflow = friction - captured.clone();

            // The syntropic arm, first. Captured friction is encased
            // exactly as captured heat is — it banks embers and rides
            // the existing `YieldCollected` receipt, which is what feeds
            // `encased_entries` and `heat_encased_of`. No new deed.
            if captured > ratio(0, 1) {
                held.bank(Resource::Ember, captured.clone());
                held.syntropy_contained += captured.clone();
                self.chronicle.append(
                    ClosureEvent::YieldCollected {
                        orb_id,
                        resource: Resource::Ember,
                        amount: captured,
                    },
                    stamp,
                    vec![actor],
                );
            }
            // The entropic arm: only the overflow vents.
            //
            // `FrictionSettled` now carries the vented part, not the
            // total — replay does `entropy_vented += amount` verbatim
            // and must never recompute physics, so a receipt naming the
            // whole cost would vent the captured half a second time on
            // every replay. This is the same convention `VentSettled`
            // already follows: a receipt records what happened to the
            // ledger it names. The closure still holds and both arms are
            // receipted — friction = captured (YieldCollected) +
            // vented (FrictionSettled). Chains cut before this landing
            // replay unchanged: their receipts say what vented then,
            // and all of it did.
            if overflow > ratio(0, 1) {
                held.entropy_vented += overflow.clone();
                self.chronicle.append(
                    ClosureEvent::FrictionSettled { orb_id, amount: overflow },
                    stamp,
                    vec![actor],
                );
            }
        }
        for (resource, amount) in settleable {
            match amount >= ratio(0, 1) {
                true => {
                    // #138 the resource GRAIN — a yield lands on the 1/1024
                    // resource grain at its fill site. Yields fold demand splits
                    // and rate·elapsed products that carry non-power-of-2
                    // denominators (a biomass half-harvest banks /15, off any
                    // power-of-2 grid); flooring here keeps every banked resource
                    // `denom | 1024`, the grain the standing witness asserts. The
                    // recorded YieldCollected carries the same floored amount, so
                    // declaration == effect and the kernel admits (call-not-copy).
                    let amount = smelt_resource(&amount);
                    if amount == ratio(0, 1) {
                        continue;
                    }
                    held.bank(resource, amount.clone());
                    // The syntropic BOOK — the inverse of
                    // entropy_vented. An ember yield is an encased ember
                    // (the crystal sheet already reads it that way), so
                    // containing one books it. Mirrored verbatim in
                    // `replay`.
                    if resource == Resource::Ember {
                        held.syntropy_contained += amount.clone();
                    }
                    self.chronicle.append(
                        ClosureEvent::YieldCollected { orb_id, resource, amount },
                        stamp,
                        vec![actor],
                    );
                }
                false => {
                    // A negative line is accrued upkeep drawing on the bank.
                    // It may draw only what stands: a bank must never cross
                    // zero, because the per-tick famine check reads the
                    // banked balance and a negative one strikes a nonsensical
                    // negative draw the kernel refuses (AmountNotPositive).
                    // Pay what the bank holds; the unpayable remainder returns
                    // to the accrued pool, where the famine check reads it as
                    // `banked(≥0) + accrued(<0) < 0` and strikes lawfully with
                    // a non-negative draw. A solvent world (bank ≥ owed) draws
                    // the whole owed exactly as before — byte-identical; only a
                    // world that would have overdrawn diverges, and there the
                    // old path was the negative-bank refusal itself.
                    // #138 the resource GRAIN — the upkeep bill lands on the
                    // 1/1024 grain at its fill site. The accrued-upkeep rate folds
                    // off-grain products (a /15-class demand, compounded across
                    // beats through the accrued pool), so flooring `owed` here keeps
                    // the draw — and, since `unpaid = owed − drawn` and the accrued
                    // pool only ever takes floored deltas, the accrued pool too — on
                    // the grain. The recorded UpkeepSettled carries the same floored
                    // `drawn`, so declaration == effect and the kernel admits.
                    let owed = smelt_resource(&(-amount));
                    let standing = held.banked_amount(resource).max(ratio(0, 1));
                    let drawn = owed.clone().min(standing);
                    let unpaid = owed - drawn.clone();
                    if unpaid > ratio(0, 1) {
                        accrue(&mut self.accrued, orb_id, resource, -unpaid);
                    }
                    if drawn > ratio(0, 1) {
                        held.bank(resource, -drawn.clone());
                        self.chronicle.append(
                            ClosureEvent::UpkeepSettled {
                                orb_id,
                                resource,
                                amount: drawn,
                            },
                            stamp,
                            vec![actor],
                        );
                    }
                }
            }
        }
        // BOND-1: the standing orders touching this orb settle their
        // accumulated spans — one receipt each (settle-at-touch).
        let touching: Vec<(u64, u64, Resource)> = self
            .bonds
            .iter()
            .filter(|bond| bond.source == orb || bond.target == orb)
            .map(|bond| (bond.source, bond.target, bond.resource))
            .collect();
        for (source, target, resource) in touching {
            self.settle_bond(source, target, resource, stamp, vec![actor], None);
        }
        // The net headcount change receipts at this same touch — after the
        // ration settled, so the food it ate is banked first. Zero deltas
        // no-op (no deed), so a world at demographic rest is byte-identical.
        if let Some((delta, _, _)) = population {
            self.settle_population_exact(orb, delta);
        }
        // Taxation (49 §5): the Touch-triggered revenue levy collects into
        // the sovereign treasury — governance reach × the taxable
        // throughput. Zero reach (no GovernmentOffice standing) collects
        // nothing, so a world without one is byte-identical.
        let revenue = self.levy_collected_at_touch(orb, &taxable);
        self.settle_treasury(orb, revenue);
        // Taxation (49 §4): the estate tax inflow at each generation
        // boundary crossed this span (read pre-touch above). No office /
        // no population / no boundary → zero → no deed.
        self.settle_treasury(orb, estate);
        // Taxation (49 §5): the welfare outflow — spend the treasury (down
        // to what it holds, read pre-touch above) to close the stability
        // stall. No hub / no stall / empty treasury → zero → no deed, so a
        // world without the welfare loop is byte-identical.
        self.settle_treasury(orb, -welfare);
        // Taxation (49 §3): the continuity levy lapses the unkept practices
        // to the void at the generation turnover (read pre-touch above).
        // Zero → no deed, so a line that keeps its knowledge is byte-identical.
        self.lapse_practices(lapse);
        self.relieve_recovered(orb);
        // BIO quarantine relief (#30): a quarantined orb recovers a step at
        // the touch — both pressure axes ebb by one. Not quarantined → no
        // ebb → no deed, so an unmodified world is byte-identical.
        self.ebb_pressure(orb);
        // G2 (#29): the food-element cycle settles — ember → element minted,
        // net of what the zones ate; receipted as ElementFarmed. `None` for
        // every non-eater, so a v1 world emits nothing and is byte-identical.
        // The ember spend is capped at the post-touch bank (the pre-touch
        // reading may exceed it after this touch's demurrage), and the mint
        // re-derived from the capped spend at the element's richness — so
        // the bank never overdraws and the kernel bound still holds tight.
        if let Some((element, _ember, _minted, eaten)) = farm {
            let stamp = self.clock.now();
            // #136 D3 — the element ledger declares its grain (1/1024) at this fill
            // site: the mint (`ember · richness`) is a product of arbitrary rationals,
            // so it smelts to the element grain before it banks or receipts; the eaten
            // debit is smelted to the same grain so the net `minted − eaten` lands
            // on-grain and the stock's denominator stays bounded (replay banks the
            // recorded, already-smelted, fields — byte-identical). 1/1024 not 1/64:
            // a food mint is naturally sub-1/64 and would floor toward zero.
            let eaten = smelt_element(&eaten);
            let mut ember = ratio(0, 1);
            let mut minted = ratio(0, 1);
            if let Some(class) = self.star {
                let (_, _, richness) = crate::dop::element_properties(&element, class);
                if let Some(held) = peel(&mut self.orbs, orb) {
                    let avail = held.banked_amount(Resource::Ember).max(ratio(0, 1));
                    ember = _ember.min(avail);
                    minted = smelt_element(&(ember.clone() * richness));
                    held.bank(Resource::Ember, -ember.clone());
                    held.bank_element(element, minted.clone() - eaten.clone());
                }
            }
            if minted > ratio(0, 1) || eaten > ratio(0, 1) {
                self.chronicle.append(
                    ClosureEvent::ElementFarmed { orb_id: orb, element, ember, minted, eaten },
                    stamp,
                    vec![actor],
                );
            }
        }
        // ONT-3: every worked vein on the orb recovers at this touch — the
        // Keplerian orbital-forcing injection over the local years elapsed,
        // amplified by the orb's recycling (β₁). Settled before the staple mine
        // reads richness, so a vein that recovered this span can feed the mouths.
        self.rejuvenate_orb_veins(orb, self.clock.now(), actor);
        // ONT-2b: eating mines the staple vein — the vein is the larder. The
        // band's `bank_draw` this span (captured pre-touch) is extracted from
        // the orb's staple vein: `extracted += eaten`, capped at the vein's
        // remaining richness (`natural − extracted`, read live), and
        // LATTICE-floored so the extraction integral's denominator stays
        // bounded forever (the ONT-2 law — a demand draw is a product of
        // rationals with an unbounded denominator; flooring to `1/1024` keeps
        // `extracted` a `1/1024` multiple, denominator-bounded, replay-exact).
        // No bank move: the mined food goes vein → mouths, the element bank
        // bypassed (the home-grown mirror of the syntropic wild gather).
        // Receipts as `StapleMined`; replay folds the same `extracted += eaten`
        // and the kernel re-derives the richness bound. `None`/exhausted for a
        // Resource-ration or worked-out line — no receipt, byte-identical.
        if let Some((element, draw)) = staple_mine {
            let stamp = self.clock.now();
            if let Some(class) = self.star {
                let (_, _, natural) = crate::dop::element_properties(&element, class);
                let lattice = crate::world::stock::lattice();
                let mined = self
                    .prospects
                    .iter_mut()
                    .find(|vein| vein.orb == orb && vein.element == element)
                    .map(|vein| {
                        let richness = crate::dop::vein_richness(&natural, &vein.extracted, &vein.rejuvenated);
                        let eaten =
                            (draw.min(richness) * lattice.clone()).floor() / lattice;
                        if eaten > ratio(0, 1) {
                            // The same book motion an assay makes — the span
                            // anchor moves and the integral grows — so the
                            // live state, replay, and the kernel book agree.
                            vein.extracted += eaten.clone();
                            vein.last_assay = stamp;
                        }
                        eaten
                    })
                    .unwrap_or_else(|| ratio(0, 1));
                if mined > ratio(0, 1) {
                    self.chronicle.append(
                        ClosureEvent::StapleMined { orb_id: orb, element, eaten: mined },
                        stamp,
                        vec![actor],
                    );
                }
            }
        }
        // G2 trace (2026-08-14): burn the trace elements the population
        // consumed this span — one debit per veined element it held, drawn
        // from the larder (read pre-touch). No veins / no population → no
        // draws → no deed → byte-identical.
        let stamp = self.clock.now();
        for (element, amount) in trace {
            if let Some(held) = peel(&mut self.orbs, orb) {
                held.bank_element(element, -amount.clone());
            }
            self.chronicle.append(
                ClosureEvent::TraceConsumed { orb_id: orb, element, amount },
                stamp,
                vec![actor],
            );
        }
        Ok(count)
    }
    /// Transfer banked value between two orbs as the sovereign.
    /// **The radial transit separation** (#50) — the logistics distance
    /// between two orbs, the normalized difference of their orbital radii
    /// `|r_from − r_to| / (r_from + r_to)` ∈ [0, 1). Two orbs at the same
    /// orbit are zero apart; the farther apart their orbits, the closer to
    /// one — bounded, so no transit cost runs away. An orb with no sited
    /// orbit reads radius zero (both unsited → zero). Pure reading, no dial.
    pub fn logistics_distance(&self, from: u64, to: u64) -> Ratio {
        let radius = |orb: u64| {
            self.orb_by_id(orb)
                .and_then(|held| held.thermal.as_ref())
                .map(|site| site.orbit_r.value().clone())
                .unwrap_or_else(|| ratio(0, 1))
        };
        let (a, b) = (radius(from), radius(to));
        let sum = a.clone() + b.clone();
        if sum <= ratio(0, 1) {
            return ratio(0, 1);
        }
        let diff = if a >= b { a - b } else { b - a };
        diff / sum
    }

    pub fn transfer(
        &mut self,
        lane: crate::dop::Between,
        resource: Resource,
        amount: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.transfer_as(SOVEREIGN, lane, resource, amount)
    }
    /// TRADE-1 (09 §2.1) — move banked value orb-to-orb within the
    /// world: the chartering conservation law generalized (nothing
    /// minted, nothing lost). Both sides settle due demurrage at the
    /// touch (the SINK-1 law) before value moves; affordability is
    /// gated on the post-decay bank; the compassion books post iff
    /// the counterparty stands in receipted, uncleared crisis at this
    /// stamp — a replay fact, never a judgment call. Collect scope;
    /// relief for the receiver re-checked at the landed law (02 §2.4).
    pub fn transfer_as(
        &mut self,
        actor: u64,
        lane: crate::dop::Between,
        resource: Resource,
        amount: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        // P2: the standing supply the sink's epoch 1 rate reads,
        // snapshotted before any orb is peeled — an owned Vec, so no
        // borrow of `self.orbs` is held across the mutable touch below.
        let supply = standing_supply(&self.orbs);
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        // Both shapes died at the doors (29_CONSTRUCTION C1).
        let (from, to) = (lane.from(), lane.to());
        let amount = amount.value().clone();
        if !self.orbs.iter().any(|held| held.orb_id == from) {
            return Err(ActionRefused::NoSuchOrb { orb: from });
        }
        if !self.orbs.iter().any(|held| held.orb_id == to) {
            return Err(ActionRefused::NoSuchOrb { orb: to });
        }
        let stamp = self.clock.now();
        let star = self.star;
        // Affordability on the post-decay bank — the books honest at
        // the stamp — computed pure, so a refusal orphans nothing.
        {
            let source = peek(&self.orbs, from)
                .expect("checked above");
            // CHANNEL_DEMURRAGE: the adopted disciplines' composed
            // decay factor (the Hoard binds 1/2 — the stores keep).
            let keep = self.practice_factor(crate::vocabulary::CHANNEL_DEMURRAGE);
            let decay = keep * demurrage_due(source, star, stamp, &ease_of(&self.techs, from, stamp), &super::rules::standing_supply(&self.orbs))
                .into_iter()
                .find(|(due, _)| *due == resource)
                .map(|(_, decay)| decay)
                .unwrap_or_else(|| ratio(0, 1));
            let have = source.banked_amount(resource) - decay;
            if have < amount {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(amount),
                    have: Box::new(have),
                });
            }
        }
        // Crisis reads before the move — the replay-fact order.
        let from_crisis = {
            let held = peek(&self.orbs, from).expect("checked");
            in_crisis(held)
        };
        let to_crisis = {
            let held = peek(&self.orbs, to).expect("checked");
            in_crisis(held)
        };
        // IDEO-1 (09 §3.2): friction prices divergence, not vice — the
        // handling overhead of unlike measured boundary policies, read
        // before the move, accrued as workspace against the mover and
        // sealed at its collect. Bonds and absorptions are exempt by
        // design: the standing loop is the friction-free channel.
        let divergence = {
            let reading = |orb: u64| {
                let held =
                    peek(&self.orbs, orb).expect("checked");
                reciprocity_reading(held)
            };
            let from_reading = reading(from);
            let to_reading = reading(to);
            match from_reading >= to_reading {
                true => from_reading - to_reading,
                false => to_reading - from_reading,
            }
        };
        // #50 logistics friction: the physical cost of moving matter across
        // the system — the radial orbital separation of the two orbs, read
        // before the touch. It prices into the same sanctioned entropy burn
        // as the divergence friction (FrictionSettled at collect), so the
        // moved value stays fully conserved — the mover simply pays a transit
        // overhead. A same-orbit move (or an unsited orb) carries none, so
        // every such transfer is byte-identical.
        let logistics = self.logistics_distance(from, to);
        // The touch, both sides (SINK-1), then the move.
        for orb in [from, to] {
            let ease = ease_of(&self.techs, orb, stamp);
            let held = peel(&mut self.orbs, orb)
                .expect("checked above");
            touch(held, star, &mut self.chronicle, stamp, &ease, &supply);
        }
        let value = amount.clone() * crate::dop::commensurate_weight(resource);
        {
            let source = peel(&mut self.orbs, from)
                .expect("checked above");
            source.bank(resource, -amount.clone());
            if to_crisis {
                source.given_in_crisis += value.clone();
            }
        }
        {
            let target = peel(&mut self.orbs, to)
                .expect("checked above");
            target.bank(resource, amount.clone());
            if from_crisis {
                target.drawn_in_crisis += value.clone();
            }
        }
        // ABSORB-1: the payment compensates open reciprocity windows
        // owed from→to, oldest first.
        settle_debts(&mut self.debts, &mut self.orbs, from, to, value.clone());
        // FORWARD-1: the delivery credits the open pact on this lane.
        credit_forwards(&mut self.forwards, from, to, resource, &amount);
        credit_councils(&mut self.councils, from, to, resource, &amount);
        let friction = crate::parameter::of(self.chronicle(), crate::parameter::Parameter::FrictionRate)
            * (divergence + logistics)
            * value;
        if friction > ratio(0, 1) {
            match self
                .friction_pending
                .iter_mut()
                .find(|(held, _)| *held == from)
            {
                Some((_, pending)) => *pending += friction,
                None => self.friction_pending.push((from, friction)),
            }
        }
        self.chronicle.append(
            ClosureEvent::ResourceTransferred { from_orb: from, to_orb: to, resource, amount },
            stamp,
            vec![actor],
        );
        self.relieve_recovered(to);
        Ok(())
    }
    /// Absorb as the sovereign.
    pub fn absorb(
        &mut self,
        pair: crate::dop::Between,
        amount: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.absorb_as(SOVEREIGN, pair, amount)
    }
    /// ABSORB-1 (05 §1.4, 09 §2.4) — route the source's un-sealed
    /// overflow into the absorber's banked sink instead of the void:
    /// `sink_consumed = heat_absorbed = vented_after_shed`, the
    /// absorber's ledger carrying the shed, the source's staying
    /// clean. Emptying the pending clears the strike (relief, the
    /// famine shape — the imbalance may strike anew next tick). The
    /// draw opens a reciprocity window owed source → absorber; TRADE-1
    /// payments retire it oldest-first; the unpaid remainder lapses to
    /// the extraction book. The absorber settles due demurrage at this
    /// touch; affordability gated on the post-decay sink, computed
    /// pure — a refusal orphans nothing. Collect scope.
    pub fn absorb_as(
        &mut self,
        actor: u64,
        pair: crate::dop::Between,
        amount: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        // P2: the standing supply the sink's epoch 1 rate reads,
        // snapshotted before any orb is peeled — an owned Vec, so no
        // borrow of `self.orbs` is held across the mutable touch below.
        let supply = standing_supply(&self.orbs);
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        // BIO quarantine (task #25): inbound melt reciprocity is barred
        // while the world is quarantined — the same seal the Agent Door
        // takes on trade. The crossing is the only aperture left open.
        if self.world_quarantined() {
            let orb = self.orb().map(|held| held.orb_id).unwrap_or(FIRST_ORB);
            return Err(ActionRefused::WorldQuarantined { orb });
        }
        // Both shapes died at the doors (29_CONSTRUCTION C1).
        let (source, absorber) = (pair.from(), pair.to());
        let amount = amount.value().clone();
        if !self.orbs.iter().any(|held| held.orb_id == source) {
            return Err(ActionRefused::NoSuchOrb { orb: source });
        }
        if !self.orbs.iter().any(|held| held.orb_id == absorber) {
            return Err(ActionRefused::NoSuchOrb { orb: absorber });
        }
        let stamp = self.clock.now();
        let star = self.star;
        let pending = self.vent_pending_of(source);
        if amount > pending {
            return Err(ActionRefused::CannotAbsorb { pending: Box::new(pending) });
        }
        {
            let held = peek(&self.orbs, absorber)
                .expect("checked above");
            // CHANNEL_DEMURRAGE: the adopted disciplines' composed
            // decay factor (the Hoard binds 1/2 — the stores keep).
            let keep = self.practice_factor(crate::vocabulary::CHANNEL_DEMURRAGE);
            let decay = keep * demurrage_due(held, star, stamp, &ease_of(&self.techs, absorber, stamp), &super::rules::standing_supply(&self.orbs))
                .into_iter()
                .find(|(due, _)| *due == Resource::Sink)
                .map(|(_, decay)| decay)
                .unwrap_or_else(|| ratio(0, 1));
            let have = held.banked_amount(Resource::Sink) - decay;
            if have < amount {
                return Err(ActionRefused::CannotAfford {
                    resource: Resource::Sink,
                    need: Box::new(amount),
                    have: Box::new(have),
                });
            }
        }
        // Crisis reads before the move — the replay-fact order.
        let source_crisis = {
            let held = peek(&self.orbs, source).expect("checked");
            in_crisis(held)
        };
        let absorber_crisis = {
            let held = peek(&self.orbs, absorber).expect("checked");
            in_crisis(held)
        };
        let value = amount.clone() * crate::dop::commensurate_weight(Resource::Sink);
        {
            let held = peel(&mut self.orbs, absorber)
                .expect("checked above");
            let ease = ease_of(&self.techs, absorber, stamp);
            touch(held, star, &mut self.chronicle, stamp, &ease, &supply);
            held.bank(Resource::Sink, -amount.clone());
            held.entropy_vented += amount.clone();
            if source_crisis {
                held.given_in_crisis += value.clone();
            }
        }
        if let Some((_, held)) = self
            .vent_pending
            .iter_mut()
            .find(|(held, _)| *held == source)
        {
            *held -= amount.clone();
        }
        self.vent_pending.retain(|(_, pending)| pending != &ratio(0, 1));
        if absorber_crisis {
            let held = peel(&mut self.orbs, source)
                .expect("checked above");
            held.drawn_in_crisis += value;
        }
        self.debts.push(AbsorptionDebt {
            debtor: source,
            creditor: absorber,
            remainder: amount.clone(),
            opened: stamp,
        });
        self.chronicle.append(
            ClosureEvent::OverflowAbsorbed { source_orb: source, absorber_orb: absorber, amount },
            stamp,
            vec![actor],
        );
        // Emptying the pending relieves the strike, receipted.
        if self.vent_pending_of(source) == ratio(0, 1) {
            let held = peel(&mut self.orbs, source)
                .expect("checked above");
            if held.overheat {
                held.overheat = false;
                self.chronicle.append(
                    ClosureEvent::OverheatCleared { orb_id: source },
                    stamp,
                    vec![],
                );
            }
        }
        Ok(())
    }
    /// 20_FOUNDING §5 — the purse: a line's economy read whole across
    /// every orb it holds.
    ///
    /// **A projection, never A settling TOUCH.** What each orb owes in
    /// demurrage is deducted from the reading and nothing is written.
    /// Looking at a purse is not a chain event, so a status render
    /// costs no receipts; the due settles for real at the next genuine
    /// touch (the SINK-1 law, unchanged). Value is still not hidden
    /// from decay — the due is already subtracted in what is read.
    /// Vaulted stores keep their exemption because `demurrage_due`
    /// grants it. Derived at call, never stored (the syntropy-ratio
    /// precedent). Ordered by resource so the read is replayable.
    pub fn purse(&self, stamp: u64) -> Vec<(Resource, Ratio)> {
        let mut held: Vec<(Resource, Ratio)> = Vec::new();
        // P2: standing supply, snapshotted once for the whole reading.
        let supply = standing_supply(&self.orbs);
        for orb in &self.orbs {
            let due = demurrage_due(
                orb,
                self.star,
                stamp,
                &ease_of(&self.techs, orb.orb_id, stamp),
                &supply,
            );
            for (resource, amount) in &orb.banked {
                let owed = due
                    .iter()
                    .find(|(r, _)| r == resource)
                    .map(|(_, d)| d.clone())
                    .unwrap_or_else(|| ratio(0, 1));
                let net = amount.clone() - owed;
                match held.iter_mut().find(|(r, _)| r == resource) {
                    Some((_, sum)) => *sum += net,
                    None => held.push((*resource, net)),
                }
            }
        }
        held.sort_by_key(|(resource, _)| *resource as u8);
        held
    }

    /// **The seat treasury — a fold over `TitheReceipted`, never a
    /// stored balance** (`#32`, `20_FOUNDING` §3, the `purse`/`fold`
    /// precedent). What one seat scope (`SEAT_ELDER`/`SEAT_ANCIENT`)
    /// has been paid across every founding tithe on this chronicle,
    /// summed by resource. There is no seat-holder orb in this engine
    /// to bank the tithe into — this reading is the closest thing to
    /// one until a seat-holder identity exists (`#32` phase 2,
    /// PURCHASE/COMMISSION, still deferred). Replay-inert: reading a
    /// treasury settles nothing and mints nothing.
    pub fn seat_treasury(&self, to_seat: u8) -> Vec<(Resource, Ratio)> {
        let mut held: Vec<(Resource, Ratio)> = Vec::new();
        for receipt in self.chronicle().history() {
            if let ClosureEvent::TitheReceipted { to_seat: seat, resource, amount, .. } =
                receipt.event()
            {
                if *seat == to_seat {
                    match held.iter_mut().find(|(r, _)| r == resource) {
                        Some((_, sum)) => *sum += amount.clone(),
                        None => held.push((*resource, amount.clone())),
                    }
                }
            }
        }
        held.sort_by_key(|(resource, _)| *resource as u8);
        held
    }

    /// **The value-loss BOOK — derived, never stored** (SYN-1/SYN-2,
    /// operator ruling 2026-08-02).
    ///
    /// What an orb has lost to holding costs: demurrage decay plus
    /// upkeep draws, summed from the chronicle in ember-commensurable
    /// units (09 §1). Before this, value left the universe with no
    /// record it had ever left — the thermal arm closes its books
    /// (`encased + vented = total excess`) and the value arm closed
    /// nothing.
    ///
    /// **It is not `entropy_vented`, and must never be.** That field is
    /// the shed-heat figure and 05 §1.4's absorption invariant binds it
    /// exactly: `sink_consumed = heat_absorbed = vented_after_shed`.
    /// Value decay is a monetary velocity cost, not a thermal loss;
    /// summing them would corrupt a landed physical measurement, which
    /// `world_absorb::the_absorption_invariant_measures_across_two_orbs`
    /// proves by failing the moment they are mixed. The two ledgers
    /// stay decoupled: physics on one side, economics on the other.
    ///
    /// **No schema is spent.** `DemurrageSettled { orb_id, .. }` and
    /// `UpkeepSettled { orb_id, .. }` already carry their orb, so the
    /// whole book is arithmetic over deeds already receipted — the
    /// derived-never-stored law (09 §1.2), and the same shape the
    /// syntropy ratio takes.
    pub fn value_lost_of(&self, orb: u64) -> Ratio {
        self.chronicle
            .history()
            .fold(ratio(0, 1), |carried, receipt| match receipt.event() {
                ClosureEvent::DemurrageSettled { orb_id, amount, .. }
                | ClosureEvent::UpkeepSettled { orb_id, amount, .. } if *orb_id == orb => {
                    carried + amount.clone()
                }
                ClosureEvent::DemurrageTouched { orb_id, decays } if *orb_id == orb => {
                    decays.iter().fold(carried, |sum, (_, amount)| sum + amount.clone())
                }
                _ => carried,
            })
    }

    /// **The union review — derived, never stored** (D5, operator
    /// ruling 2026-08-02). When the sovereign union was struck, and
    /// whether its review has fallen due at `stamp`.
    ///
    /// `Some((sealed_at, due))` if the union stands; `None` if it was
    /// never struck. **`due` is a review falling due, never a lapse.**
    /// Nothing settles, nothing expires, and no capacity is dropped
    /// when it turns true: a `BOND-1` term ends its bond at
    /// `sealed + term`, and if the union did that, D5's breach clause
    /// would fire at stamp 1500 — *D stops and V suffocates; V stops
    /// and D freezes* — and the foundation would lapse on a timer. The
    /// union stands past the horizon; what falls due is the operator's
    /// re-approval.
    ///
    /// Read from the chronicle on demand, so there is no counter to
    /// drift and no field to migrate (`09_LEDGER` §1.2). This is the
    /// consumer that keeps the deed from being a term that only
    /// resonates (`18` Article III law 8).
    pub fn union_review(&self, stamp: u64) -> Option<(u64, bool)> {
        self.chronicle
            .history()
            .find(|receipt| {
                matches!(receipt.event(), ClosureEvent::SovereignUnionSealed { .. })
            })
            .map(|receipt| {
                let horizon = match receipt.event() {
                    ClosureEvent::SovereignUnionSealed { term_stamps, .. } => *term_stamps,
                    _ => crate::dop::UNION_REVIEW_HORIZON,
                };
                (receipt.stamp(), stamp >= receipt.stamp().saturating_add(horizon))
            })
    }


    /// **The heat encased — the syntropic arm of the thermal book,
    /// derived, never stored** (the missing-leg sweep, 2026-08-02).
    ///
    /// What an orb has captured over its whole life: every ember yield
    /// it collected, which is the encasement arm by this engine's own
    /// reading — `YieldCollected { resource: Ember }` is exactly what
    /// feeds `encased_entries` on the crystal sheet.
    ///
    /// **Why it was owed.** The thermal dual was two-armed in one place
    /// and one-armed in another. On the sheet both arms stand and are
    /// symmetric — `vented_entries` from `VentSettled`, `encased_entries`
    /// from the ember yield — but both are windowed and pruned at each
    /// boundary, and both are `pub(super)`, so they answer one tech
    /// condition and nothing else. On the orb, which is committed state,
    /// replayed and public, only `entropy_vented` stands, and it is
    /// cumulative forever.
    ///
    /// So the permanent public book recorded only loss. An orb could
    /// always be asked what it had shed to the void for its whole life,
    /// and could not be asked what it had ever kept — the gain was
    /// forgotten as soon as the window moved past it. That is the
    /// negentropic shape `EQUATIONS` §0.3 chapter five names: *a map
    /// that inherits one leg of a dual has not simplified the law, it
    /// has reversed it.*
    ///
    /// **No schema is spent.** `YieldCollected { orb_id, resource,
    /// amount }` already carries its orb, so the whole arm is
    /// arithmetic over deeds already receipted — the derived-never-stored
    /// law (`09_LEDGER` §1.2), and the same shape [`Self::value_lost_of`]
    /// took. `entropy_vented` is not touched: it is a landed field bound
    /// by 05 §1.4's absorption invariant, and the cure for a missing arm
    /// is to add the arm, never to disturb the one that stands.
    pub fn heat_encased_of(&self, orb: u64) -> Ratio {
        self.chronicle
            .history()
            .filter_map(|receipt| match receipt.event() {
                ClosureEvent::YieldCollected { orb_id, resource: Resource::Ember, amount }
                    if *orb_id == orb =>
                {
                    Some(amount.clone())
                }
                _ => None,
            })
            .fold(ratio(0, 1), |a, b| a + b)
    }

    /// **The thermal BOOK, both arms AT once** — `(encased, vented)`.
    ///
    /// Returned as a pair and never as a difference or a ratio: a net
    /// figure is the one-armed reading wearing two arms' clothes, and
    /// `18` §2b bars collapsing a directional quantity to a scalar
    /// wherever the system computes. An orb that captured 100 and vented
    /// 100 is not the same orb as one that captured nothing and vented
    /// nothing, and only the pair can tell them apart.
    pub fn thermal_books_of(&self, orb: u64) -> (Ratio, Ratio) {
        let vented = self
            .orbs
            .iter()
            .find(|held| held.orb_id == orb)
            .map(|held| held.entropy_vented.clone())
            .unwrap_or_else(|| ratio(0, 1));
        (self.heat_encased_of(orb), vented)
    }


}
