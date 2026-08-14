//! THE TICK — the world advancing one step.
//!
//! `tick` alone is 582 lines and is the largest single function in the
//! organ. It is isolated here rather than split blind: GROUND §3
//! invariant 4 says line count is a symptom, and the concern this
//! function actually mixes has not been measured yet. Named as owed.

use super::*;

impl World {
    /// One simulation tick for the WHOLE world: every orb's active
    /// zones net their flows into that orb's workspace — through the
    /// heat law where thermal is live (05 §1.2: encase first, only the
    /// overflow vents, degrade by 1/(1+overflow)); famine strikes per
    /// orb; the one clock walks by quorum.
    pub fn tick(&mut self) -> TickOutcome {
        use crate::parameter::Parameter;
        // P2 (tag 103): the thermal rates read the CHARTERED table —
        // one fold walk per tick, seed where nothing is chartered.
        let rates = crate::parameter::table_of(self.chronicle());
        use crate::thermal;
        // ABSORB-1 (09 §2.4): reciprocity windows expire at their
        // exact stamp — the unpaid remainder posts to the debtor's
        // extraction book, receipted (world-driven, the famine shape).
        {
            let now = self.clock.now();
            let window = crate::dop::reciprocity_window();
            let mut lapsed = Vec::new();
            self.debts.retain(|debt| match now >= debt.opened + window {
                true => {
                    lapsed.push(debt.clone());
                    false
                }
                false => true,
            });
            for debt in lapsed {
                if let Some(held) =
                    self.orbs.iter_mut().find(|held| held.orb_id == debt.debtor)
                {
                    held.extracted_across += debt.remainder.clone();
                }
                self.chronicle.append(
                    ClosureEvent::AbsorptionLapsed {
                        debtor_orb: debt.debtor,
                        creditor_orb: debt.creditor,
                        amount: debt.remainder,
                    },
                    now,
                    vec![],
                );
            }
            // BOND-1: term bonds expire ORDERLY at their exact
            // boundary — the final span settles, nothing posts.
            let expired: Vec<(u64, u64, Resource)> = self
                .bonds
                .iter()
                .filter(|bond| bond.term > 0 && now >= bond.sealed + bond.term)
                .map(|bond| (bond.source, bond.target, bond.resource))
                .collect();
            for (source, target, resource) in expired {
                self.settle_bond(source, target, resource, now, vec![], Some(ratio(0, 1)));
            }
            // FORWARD-1: the vitrified pacts resolve at the boundary —
            // fulfilment releases, the passed deadline slashes;
            // fulfilment wins the tie. World-driven, the famine shape.
            let due: Vec<(u64, u64, Resource, bool)> = self
                .forwards
                .iter()
                .filter_map(|pact| {
                    if pact.delivered >= pact.quota {
                        Some((pact.obligor, pact.beneficiary, pact.resource, true))
                    } else if now >= pact.sealed + pact.window {
                        Some((pact.obligor, pact.beneficiary, pact.resource, false))
                    } else {
                        None
                    }
                })
                .collect();
            for (obligor, beneficiary, resource, fulfilled) in due {
                let Some(at) = self.forwards.iter().position(|pact| {
                    pact.obligor == obligor
                        && pact.beneficiary == beneficiary
                        && pact.resource == resource
                }) else {
                    continue;
                };
                let pact = self.forwards.remove(at);
                let stake =
                    pact.collateral.clone() * crate::dop::commensurate_weight(resource);
                let recipient = match fulfilled {
                    true => obligor,
                    false => beneficiary,
                };
                if let Some(held) =
                    peel(&mut self.orbs, recipient)
                {
                    held.bank(resource, pact.collateral.clone());
                }
                if let Some(held) =
                    peel(&mut self.orbs, obligor)
                {
                    match fulfilled {
                        true => held.forwards_fulfilled += stake,
                        false => held.forwards_breached += stake,
                    }
                }
                let event = match fulfilled {
                    true => ClosureEvent::ForwardReleased {
                        obligor_orb: obligor,
                        beneficiary_orb: beneficiary,
                        resource,
                        collateral: pact.collateral,
                    },
                    false => ClosureEvent::ForwardSlashed {
                        obligor_orb: obligor,
                        beneficiary_orb: beneficiary,
                        resource,
                        collateral: pact.collateral,
                    },
                };
                self.chronicle.append(event, now, vec![]);
                self.relieve_recovered(recipient);
            }
            // COUNCIL-1: councils reckon at the BOUNDARY only — all
            // members gated together (the group over the individual;
            // no early release, no early slash). A breacher's
            // collateral divides exactly among the faithful; if every
            // member breached there is no aggrieved party — each
            // collateral goes home, the breaches still post.
            let boundary: Vec<u64> = self
                .councils
                .iter()
                .filter(|pact| now >= pact.sealed + pact.window)
                .map(|pact| pact.council)
                .collect();
            for id in boundary {
                let at = self
                    .councils
                    .iter()
                    .position(|pact| pact.council == id)
                    .expect("due above");
                let pact = self.councils.remove(at);
                let verdicts: Vec<bool> =
                    pact.delivered.iter().map(|given| *given >= pact.quota).collect();
                let faithful = verdicts.iter().filter(|kept| **kept).count() as i64;
                let breachers = pact.members.len() as i64 - faithful;
                let share = match faithful {
                    0 => ratio(0, 1),
                    _ => pact.collateral.clone() * ratio(breachers, 1) / ratio(faithful, 1),
                };
                let weight = crate::dop::commensurate_weight(pact.resource);
                for (seat, member) in pact.members.iter().enumerate() {
                    let fulfilled = verdicts[seat];
                    let banked = match (fulfilled, faithful) {
                        (true, _) => pact.collateral.clone() + share.clone(),
                        (false, 0) => pact.collateral.clone(),
                        (false, _) => ratio(0, 1),
                    };
                    let stake = pact.collateral.clone() * weight.clone();
                    if let Some(held) =
                        peel(&mut self.orbs, *member)
                    {
                        if banked > ratio(0, 1) {
                            held.bank(pact.resource, banked.clone());
                        }
                        match fulfilled {
                            true => held.forwards_fulfilled += stake,
                            false => held.forwards_breached += stake,
                        }
                    }
                    self.chronicle.append(
                        ClosureEvent::CouncilClosed {
                            council: id,
                            member: *member,
                            fulfilled,
                            banked,
                        },
                        now,
                        vec![],
                    );
                    if fulfilled {
                        self.relieve_recovered(*member);
                    }
                }
            }
        }
        let mut struck = Vec::new();
        let mut overheat_struck = Vec::new();
        let mut overheat_cleared = Vec::new();
        // A zone runs at its VIGOUR — the share of its inputs that
        // stand — never all-or-nothing (20_FOUNDING §6a-bis). Zones at
        // zero vigour drop out; the rest carry their reading with them
        // so every downstream sum is weighted rather than counted.
        let flows: Vec<(u64, Vec<(Zone, Ratio)>)> = self
            .orbs
            .iter()
            .map(|orb| {
                (
                    orb.orb_id,
                    orb.zones
                        .iter()
                        .map(|zone| (zone.clone(), orb.zone_vigour(zone)))
                        .filter(|(_, vigour)| *vigour > ratio(0, 1))
                        .collect(),
                )
            })
            .collect();
        let one = ratio(1, 1);
        let zero = ratio(0, 1);
        let stamp_now = self.clock.now();
        // D3: the charted seed, read once — uncharted boosts read 1.
        let terrain = self.terrain_seed;
        // PRACTICE-1: the bound disciplines, read once — PURE DATA over
        // the channel table (operator ruling 2026-08-05): a practice
        // acts only through its binding; the bespoke branches are gone.
        use crate::vocabulary::{
            CHANNEL_CONVERTER_YIELD, CHANNEL_EXTRACTOR_UPKEEP, CHANNEL_HABITAT_UPKEEP,
            CHANNEL_HABITAT_YIELD,
        };
        // ONE fold walk per tick, six channel products from it.
        let practice_bindings = self.adopted_practice_bindings();
        let practice_on = |channel: u8| -> Ratio {
            practice_bindings.iter().fold(ratio(1, 1), |acc, binding| {
                acc * crate::vocabulary::factor_on(binding, channel)
            })
        };
        let practice_yield = practice_on(crate::vocabulary::CHANNEL_YIELD);
        let practice_upkeep = practice_on(crate::vocabulary::CHANNEL_UPKEEP);
        let practice_habitat_yield = practice_on(CHANNEL_HABITAT_YIELD);
        let practice_habitat_upkeep = practice_on(CHANNEL_HABITAT_UPKEEP);
        let practice_extractor_upkeep = practice_on(CHANNEL_EXTRACTOR_UPKEEP);
        let practice_converter_yield = practice_on(CHANNEL_CONVERTER_YIELD);
        // 28_VOCABULARY V0: the sealed class's own mechanism, read once.
        // Before this the class was a term that only resonated — sealed,
        // replayed, and read by no rule (LAW 8's excision test). An
        // UNSEALED world reads the empty binding and is unaffected, so
        // this is strictly additive to every world that has not sealed.
        let archetype = self
            .taxon
            .map(|(class, _)| crate::vocabulary::archetype_binding(class))
            .unwrap_or_default();
        let class_yield = crate::vocabulary::factor_on(&archetype, crate::vocabulary::CHANNEL_YIELD);
        let class_upkeep =
            crate::vocabulary::factor_on(&archetype, crate::vocabulary::CHANNEL_UPKEEP);
        // **#41: THE MINTED BUILDINGS' OWN BINDINGS, SHEARED ONCE.**
        // The zone fold is read HERE, not per zone: folding the
        // chronicle inside the tile loop would rescan the whole chain
        // once per building per tick, which is the find-scan repetition
        // the peel/shear ruling forbids. One pass, then a lookup by
        // code — and a world that has minted no buildings pays for an
        // empty vector.
        let minted_zones: Vec<(u8, Vec<(u8, Ratio)>)> = crate::vocabulary::fold(
            self.chronicle(),
            crate::vocabulary::Axis::Zone,
        )
        .into_iter()
        .filter(|entry| !entry.binding.is_empty())
        .map(|entry| (entry.code, entry.binding))
        .collect();
        let minted_on = |kind: ZoneKind, channel: u8| -> Ratio {
            match kind {
                ZoneKind::Minted(code) => minted_zones
                    .iter()
                    .find(|(held, _)| *held == code)
                    .map(|(_, binding)| crate::vocabulary::factor_on(binding, channel))
                    .unwrap_or_else(|| ratio(1, 1)),
                // A compiled kind claims nothing of its own.
                _ => ratio(1, 1),
            }
        };
        // TECH-1b: each orb's crystallized boons, read once — [unused,
        // insulation, discipline, deepening, doctrine, stores] — each
        // carrying the magnitude ITS RECEIPT asserted (receipt
        // authority, operator ruling 2026-08-05): the factors below
        // read the chain's own numbers, never a table and never an
        // inline dial.
        const NO_BOON: [Option<Ratio>; 6] = [None, None, None, None, None, None];
        let boons: Vec<(u64, [Option<Ratio>; 6])> = flows
            .iter()
            .map(|(orb_id, _)| {
                let mut held = NO_BOON;
                for (orb, condition, _, magnitude) in &self.techs {
                    if orb == orb_id && (*condition as usize) < 6 {
                        held[*condition as usize] = Some(magnitude.clone());
                    }
                }
                (*orb_id, held)
            })
            .collect();
        // G2: the element a minted metabolism eats, read ONCE for the
        // beat (None for every compiled metabolism, so v1 worlds are
        // untouched). Where it stands, the food-slot flows divert from
        // the resource store to this element's bank.
        let element_ration = self.metabolic_element();
        for (orb_id, active) in &flows {
            let boon = boons
                .iter()
                .find(|(held, _)| held == orb_id)
                .map(|(_, held)| held.clone())
                .unwrap_or(NO_BOON);
            // THERMAL-1: the heat law, live only when the sun is
            // commissioned AND this orb is sited (the opt-in guard).
            let holder = peek(&self.orbs, *orb_id);
            let site = holder.and_then(|held| held.thermal.clone());
            let joule_famined = holder
                .map(|held| held.famine.contains(&Resource::Joule))
                .unwrap_or(false);
            let mut efficiency = one.clone();
            let mut t4: Option<Ratio> = None;
            let mut frozen = false;
            if let (Some(class), Some(site)) = (self.star, site) {
                let t_eq4 = thermal::t_eq4(&class.luminosity(), site.albedo.value(), site.orbit_r.value());
                // ECON-J freeze-lock (05 §1.2): a sub-mesic orb in joule
                // famine is frozen whole — no flows until fuel arrives.
                let sub_mesic = t_eq4 < thermal::SubstrateBand::Mesic.bounds4().0;
                frozen = sub_mesic && joule_famined;
                let stellar_load = t_eq4.clone() / thermal::thermal_k();
                // Weighted: a zone running at half its inputs runs its
                // furnace at half.
                let factory: Ratio = active.iter().fold(zero.clone(), |sum, (zone, vigour)| {
                    sum + zone.priced_as().factory_heat() * vigour.clone()
                });
                // TECH-1b: the doctrine crystallized — the radiator
                // sheds at the magnitude its receipt asserted (10 §3).
                let radiator = match &boon[4] {
                    Some(magnitude) => rates.get(Parameter::RadiatorBase).clone() * magnitude.clone(),
                    None => rates.get(Parameter::RadiatorBase).clone(),
                };
                let heat_net = stellar_load + factory - radiator;
                // ECON-J (05 §1.2): heating upkeep on a sub-mesic orb —
                // every active zone burns joules against the cold.
                if sub_mesic && !frozen && !active.is_empty() {
                    let running: Ratio = active
                        .iter()
                        .fold(zero.clone(), |sum, (_, vigour)| sum + vigour.clone());
                    let mut burn = rates.get(Parameter::HeatingPerZone).clone() * running;
                    // TECH-1b: insulation crystallized — the cold
                    // bites at the receipted magnitude (10 §3).
                    if let Some(magnitude) = &boon[1] {
                        burn *= magnitude.clone();
                    }
                    accrue(&mut self.accrued, *orb_id, Resource::Joule, -burn);
                }
                // SINK-1 (05 §1.4): vault-band headroom is captured as
                // bankable sink by active cryo-lattice arrays — the
                // infrastructure gate (research/throttle gates deferred).
                if !frozen && heat_net < zero && t_eq4 <= thermal::cryo_vault_max4() {
                    let arrays: Ratio = active
                        .iter()
                        .filter(|(zone, _)| zone.kind == ZoneKind::CryoArray)
                        .fold(zero.clone(), |sum, (_, vigour)| sum + vigour.clone());
                    if arrays > zero {
                        let headroom = -heat_net.clone();
                        let mut lattice = rates.get(Parameter::SinkThroughput).clone() * arrays;
                        // TECH-1b: the lattice deepened — capture at
                        // the receipted magnitude (10 §3).
                        if let Some(magnitude) = &boon[3] {
                            lattice *= magnitude.clone();
                        }
                        let captured = headroom.min(lattice);
                        if captured > zero {
                            accrue(&mut self.accrued, *orb_id, Resource::Sink, captured);
                        }
                    }
                }
                if !frozen && heat_net > zero {
                    // The syntropic arm FIRST (05 §1.2): shells encase
                    // up to their throughput; joules accrue as yield.
                    let shells: Ratio = active
                        .iter()
                        .filter(|(zone, _)| zone.kind == ZoneKind::Encasement)
                        .fold(zero.clone(), |sum, (_, vigour)| sum + vigour.clone());
                    let mut throughput = rates.get(Parameter::EncasementThroughput).clone() * shells;
                    // TECH-1b: discipline crystallized — the shells
                    // encase at the receipted magnitude (10 §3).
                    if let Some(magnitude) = &boon[2] {
                        throughput *= magnitude.clone();
                    }
                    let encased = heat_net.clone().min(throughput);
                    if encased > zero {
                        accrue(&mut self.accrued, *orb_id, Resource::Joule, encased.clone());
                    }
                    // The entropic arm: only the overflow vents.
                    let overflow = heat_net - encased;
                    if overflow > zero {
                        efficiency = one.clone() / (one.clone() + overflow.clone());
                        // VENT-1: the loss accumulates in workspace,
                        // sealed by VentSettled at collect.
                        match self
                            .vent_pending
                            .iter_mut()
                            .find(|(held, _)| held == orb_id)
                        {
                            Some((_, pending)) => *pending += overflow.clone(),
                            None => self.vent_pending.push((*orb_id, overflow.clone())),
                        }
                        let held = peel(&mut self.orbs, *orb_id)
                            .expect("orb exists");
                        if !held.overheat {
                            held.overheat = true;
                            self.chronicle.append(
                                ClosureEvent::OverheatStruck {
                                    orb_id: *orb_id,
                                    overflow,
                                },
                                stamp_now,
                                vec![],
                            );
                            overheat_struck.push(*orb_id);
                        }
                    }
                }
                t4 = Some(t_eq4);
            }
            if efficiency == one {
                let held = peel(&mut self.orbs, *orb_id)
                    .expect("orb exists");
                if held.overheat {
                    held.overheat = false;
                    self.chronicle.append(
                        ClosureEvent::OverheatCleared { orb_id: *orb_id },
                        stamp_now,
                        vec![],
                    );
                    overheat_cleared.push(*orb_id);
                }
            }
            // G2 STASIS (ruled 2026-08-12): an element-eating line that
            // cannot meet this beat's food bill in its own element is
            // FROZEN — all local production suspended, structures held —
            // exactly the whole-orb freeze a joule-famined sub-mesic orb
            // takes. It is a per-beat CONDITION, recomputed and never
            // stored: the moment trade or minting refills the element
            // bank, the next beat's bill is met and the line runs again.
            if let Some(element) = element_ration {
                let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
                let bill = Self::element_food_bill(active, metabolism);
                let have = peek(&self.orbs, *orb_id)
                    .map(|held| held.element_amount(&element))
                    .unwrap_or_else(|| ratio(0, 1));
                if have < bill {
                    frozen = true;
                }
            }
            if !frozen {
                // G2 (04 §1.2): every flow reads through the sealed
                // metabolism's ration/harvest map — biotic and
                // unsealed read ruleset v1 verbatim.
                let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
                let standing: Vec<Zone> = active.iter().map(|(zone, _)| zone.clone()).collect();
                for (zone, vigour) in active {
                    // #41: a minted building's numbers are its BASE's —
                    // the binding modulates what a building DOES, and
                    // `priced_as` reads the base fixed at build time.
                    let seat = zone.priced_as();
                    let (resource, mut amount) = crate::dop::zone_yield(seat, metabolism);
                    // TAXON ECONOMY (37): extractive · regenerative · balanced
                    // + planet fit scale yields. Flatlander baseline = balanced × native.
                    let eco = self.taxon_economy_of(*orb_id);
                    let mode_scale = match seat {
                        ZoneKind::Extractor => {
                            crate::almanac::extract_yield_scale(eco.mode, eco.fit)
                        }
                        ZoneKind::Regenerator => {
                            crate::almanac::regen_yield_scale(eco.mode, eco.fit)
                        }
                        ZoneKind::Habitat => {
                            crate::almanac::habitat_yield_scale(eco.mode, eco.fit)
                        }
                        _ => ratio(1, 1),
                    };
                    amount = amount * mode_scale;
                    let boost = terrain_boost(&terrain, *orb_id, zone, &standing);
                    // The class's mechanism composes WITH the adopted
                    // practices, never instead of them: identity and
                    // discipline are different claims on the same world.
                    // Every factor here is CHANNEL DATA — the global
                    // channels apply everywhere, the zone-scoped ones
                    // where the zone kind matches, silence reads 1.
                    // The MINTED binding composes with the class's and
                    // the practices' — three claims on one world, none
                    // of them instead of the others.
                    let mut yield_factor = class_yield.clone()
                        * practice_yield.clone()
                        * minted_on(zone.kind, crate::vocabulary::CHANNEL_YIELD);
                    let mut upkeep_factor = class_upkeep.clone()
                        * practice_upkeep.clone()
                        * minted_on(zone.kind, crate::vocabulary::CHANNEL_UPKEEP);
                    // The family channels follow the BASE: a minted
                    // extractor-variant is priced as an extractor.
                    match seat {
                        ZoneKind::Habitat => {
                            yield_factor *= practice_habitat_yield.clone()
                                * minted_on(zone.kind, CHANNEL_HABITAT_YIELD);
                            upkeep_factor *= practice_habitat_upkeep.clone()
                                * minted_on(zone.kind, CHANNEL_HABITAT_UPKEEP);
                        }
                        ZoneKind::Extractor => {
                            upkeep_factor *= practice_extractor_upkeep.clone()
                                * minted_on(zone.kind, CHANNEL_EXTRACTOR_UPKEEP);
                        }
                        ZoneKind::Smelter | ZoneKind::Synthesizer | ZoneKind::Resonator => {
                            yield_factor *= practice_converter_yield.clone()
                                * minted_on(zone.kind, CHANNEL_CONVERTER_YIELD);
                        }
                        _ => {}
                    }
                    // G2: an element-eater's FOOD SLOT (the biomass line)
                    // is not a resource flow — it settles against the
                    // element bank below, so it never accrues here.
                    // Every other yield (iron extraction, joule capture)
                    // rides the resource store verbatim.
                    if !(element_ration.is_some() && resource == Resource::Biomass) {
                        accrue(
                            &mut self.accrued,
                            *orb_id,
                            resource,
                            amount * efficiency.clone() * boost * yield_factor.clone() * vigour.clone(),
                        );
                    }
                    // A zone drawing half its inputs consumes half.
                    for (resource, amount) in crate::dop::zone_upkeep(seat, metabolism) {
                        // G2: the biomass food component of an element-
                        // eater's upkeep is likewise settled in element,
                        // not accrued as a resource.
                        if element_ration.is_some() && resource == Resource::Biomass {
                            continue;
                        }
                        accrue(
                            &mut self.accrued,
                            *orb_id,
                            resource,
                            -amount * upkeep_factor.clone() * vigour.clone(),
                        );
                    }
                }
            }
            // The cryo law (05 §1.2): accrued joules leak outside the
            // vault (T⁴ > 100⁴), floored at zero; lossless in the cold.
            if let Some(t4) = t4 {
                if t4 > thermal::cryo_vault_max4() {
                    let held_joules = triple_amount(&self.accrued, *orb_id, Resource::Joule);
                    if held_joules > zero {
                        let leak = held_joules.min(rates.get(Parameter::JouleLeakPerTick).clone());
                        accrue(&mut self.accrued, *orb_id, Resource::Joule, -leak);
                    }
                }
            }
        }
        // The famine check, per orb (02 §2.4).
        let stamp = self.clock.now();
        for orb in &mut self.orbs {
            for resource in Resource::ALL {
                let accrued = triple_amount(&self.accrued, orb.orb_id, resource);
                let banked = orb.banked_amount(resource);
                let solvent = banked.clone() + accrued >= ratio(0, 1);
                if !solvent && !orb.famine.contains(&resource) {
                    orb.bank(resource, -banked.clone());
                    self.accrued
                        .retain(|(held, res, _)| !(*held == orb.orb_id && *res == resource));
                    orb.famine.push(resource);
                    orb.famine.sort();
                    self.chronicle.append(
                        ClosureEvent::FamineStruck {
                            orb_id: orb.orb_id,
                            resource,
                            drawn: banked,
                        },
                        stamp,
                        vec![],
                    );
                    struck.push((orb.orb_id, resource));
                }
            }
        }
        // E3.7 — material element wants (per tick, famine-shaped).
        self.sweep_element_wants(stamp);
        // TECH-1b (10 §2–§3): CRYSTALLIZATION at the year voice — on
        // each year boundary every orb's chronicle is weighed against
        // the condition table; a condition that holds crystallizes its
        // node ONCE (idempotent per orb per condition). The node id is
        // the seeded synthesis: BLAKE3(latest receipt digest ‖ orb ‖
        // condition) — the chronicle root at the moment of
        // crystallization, so replay re-derives the identical node at
        // the identical tick. No randomness, no external calls.
        if stamp_now > 0 && stamp_now.is_multiple_of(60) {
            // The shear: the INCREMENTAL sweep (#41) — only the new
            // receipts deposit; the crystallizer AND the want check
            // read the same carried sheet.
            let deposits = self.crystal_deposits_at(stamp_now);
            let blank = CrystalDeposit::default();
            let candidates: Vec<u64> = self.orbs.iter().map(|held| held.orb_id).collect();
            // WANT-1 (12_UNION §5b): the cognitive wants, star-
            // INDEPENDENT (a mind needs no sky to starve) and typed by
            // the SEALED identity — no seal, no entity type, no want
            // (pre-G1 worlds are pre-identity, lawfully neutral).
            if let Some(sealed) = self.genesis.clone() {
                for orb in &candidates {
                    let sheet = deposits
                        .iter()
                        .find(|(held, _)| held == orb)
                        .map(|(_, sheet)| sheet)
                        .unwrap_or(&blank);
                    // (kind, applies, fed_within, last, window)
                    let checks: [(u8, bool, Option<u64>, u64); 2] = [
                        // resonance: a harmonic mind starved of links.
                        (2, sealed.perception == 3, sheet.last_link, 240),
                        // novelty: a novelty-hungry mind starved of
                        // signal (a discovery must EXIST to starve for the
                        // next) — the empiricist trait, read generically so a
                        // line minted from it hungers too (`44_LINEAGES`).
                        (
                            3,
                            self.disposition_carries(crate::dop::DispositionTrait::NoveltyHunger),
                            sheet.last_node,
                            480,
                        ),
                    ];
                    for (kind, applies, last, window) in checks {
                        if !applies {
                            continue;
                        }
                        let struck = self.wants.iter().any(|(o, k)| o == orb && *k == kind);
                        match last {
                            Some(stamp)
                                if stamp_now.saturating_sub(stamp) >= window && !struck =>
                            {
                                let reading =
                                    ratio(stamp_now.saturating_sub(stamp) as i64, 1);
                                self.wants.push((*orb, kind));
                                self.chronicle.append(
                                    ClosureEvent::WantStruck {
                                        orb_id: *orb,
                                        kind,
                                        reading,
                                    },
                                    stamp_now,
                                    vec![],
                                );
                            }
                            Some(stamp)
                                if stamp_now.saturating_sub(stamp) >= window => {}
                            Some(_) if struck => {
                                // Fed within the window: relieved.
                                self.wants.retain(|(o, k)| !(o == orb && *k == kind));
                                self.chronicle.append(
                                    ClosureEvent::WantRelieved { orb_id: *orb, kind },
                                    stamp_now,
                                    vec![],
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
            if self.star.is_none() {
                // No sky, no crystallization — but the wants above
                // already spoke.
                let attestations: [Attestation; 3] = [
                    self.keepers[0].attest(&self.clock),
                    self.keepers[1].attest(&self.clock),
                    self.keepers[2].attest(&self.clock),
                ];
                return TickOutcome {
                    read: self.clock.tick(&attestations),
                    struck,
                    overheat_struck,
                    overheat_cleared,
                };
            }
            for orb in candidates {
                let sheet = deposits
                    .iter()
                    .find(|(held, _)| *held == orb)
                    .map(|(_, sheet)| sheet)
                    .unwrap_or(&blank);
                // WANT-1: a novelty-struck mind mints no nodes — the
                // crystallizer rests; relief arrives by licensed leap
                // (the co-op pressure by design).
                if self.wants.iter().any(|(o, k)| *o == orb && *k == 3) {
                    continue;
                }
                for condition in 1u8..=5 {
                    if self
                        .techs
                        .iter()
                        .any(|(held, kind, _, _)| *held == orb && *kind == condition)
                    {
                        continue;
                    }
                    if !self.condition_holds(orb, condition, stamp_now, sheet) {
                        continue;
                    }
                    let (modifier, magnitude) =
                        crate::dop::tech_node_of(condition).expect("1..=5 tabled");
                    let mut seed = self
                        .chronicle
                        .receipts()
                        .last()
                        .expect("a ticking world has receipts")
                        .own
                        .0
                        .to_vec();
                    seed.extend_from_slice(&orb.to_le_bytes());
                    seed.push(condition);
                    let node: [u8; 32] = blake3::hash(&seed).into();
                    // The name is THIS line's own (10 §2 law 4): drawn
                    // from the node hash, never a rung label.
                    let name = crate::dop::tech_node_name(condition, &node);
                    self.techs.push((orb, condition, node, magnitude.clone()));
                    self.chronicle.append(
                        ClosureEvent::TechDiscovered {
                            orb_id: orb,
                            node,
                            condition,
                            modifier,
                            magnitude,
                            name,
                        },
                        stamp_now,
                        vec![],
                    );
                }
            }
        }
        let attestations: [Attestation; 3] = [
            self.keepers[0].attest(&self.clock),
            self.keepers[1].attest(&self.clock),
            self.keepers[2].attest(&self.clock),
        ];
        TickOutcome {
            read: self.clock.tick(&attestations),
            struck,
            overheat_struck,
            overheat_cleared,
        }
    }
    /// Famine relief for ONE orb (02 §2.4). Joule relief on a cold orb
    /// needs the one-tick HEATING bill covered (ECON-J), not zero.
    pub(super) fn relieve_recovered(&mut self, orb: u64) {
        let stamp = self.clock.now();
        let star = self.star;
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
        let heating_rate =
            crate::parameter::of(self.chronicle(), crate::parameter::Parameter::HeatingPerZone);
        let Some(held) = peel(&mut self.orbs, orb) else {
            return;
        };
        let heating_need: Ratio = match (star, &held.thermal) {
            (Some(class), Some(site)) => {
                let t4 =
                    crate::thermal::t_eq4(&class.luminosity(), site.albedo.value(), site.orbit_r.value());
                match t4 < crate::thermal::SubstrateBand::Mesic.bounds4().0 {
                    true => {
                        heating_rate.clone() * ratio(held.zones.len() as i64, 1)
                    }
                    false => ratio(0, 1),
                }
            }
            _ => ratio(0, 1),
        };
        let famined: Vec<Resource> = held.famine.clone();
        for resource in famined {
            // G2: the one-tick bill reads the line's own rations.
            let mut one_tick: Ratio = held
                .zones
                .iter()
                .flat_map(|zone| crate::dop::zone_upkeep(zone.kind, metabolism))
                .filter(|(consumed, _)| *consumed == resource)
                .map(|(_, amount)| amount)
                .fold(ratio(0, 1), |sum, amount| sum + amount);
            if resource == Resource::Joule {
                one_tick += heating_need.clone();
            }
            if held.banked_amount(resource) >= one_tick {
                held.famine.retain(|kept| *kept != resource);
                self.chronicle.append(
                    ClosureEvent::FamineRelieved {
                        orb_id: held.orb_id,
                        resource,
                    },
                    stamp,
                    vec![],
                );
            }
        }
    }

    /// **G2 — THE ELEMENT THIS LINE EATS** (ruled 2026-08-12). The
    /// periodic element a MINTED element-metabolism farms and consumes,
    /// or `None` for the three compiled metabolisms and an unsealed
    /// world. The compiled codes (1..=3) short-circuit BEFORE the fold,
    /// so every v1 world reads `None` in O(1) and its flows are
    /// untouched; only an element-eater (code >= 4, a deliberate mint)
    /// pays the vocabulary fold to read its ration.
    pub fn metabolic_element(&self) -> Option<[u8; 32]> {
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism)?;
        if metabolism < 4 {
            return None;
        }
        crate::vocabulary::fold(self.chronicle(), crate::vocabulary::Axis::Metabolism)
            .into_iter()
            .find(|entry| entry.code == metabolism)
            .and_then(|entry| entry.ration)
    }

    /// **G2 (#29) — THE FOOD-ELEMENT CYCLE at a collect touch.** An
    /// element-eater EATS the stored food its zones ran on this span and
    /// SYNTHESIZES more from ENERGY to refill its larder toward a span's
    /// buffer: it spends Joule to mint the element (a conserved
    /// transmutation, bounded by the element's richness — `minted ≤ joule
    /// × richness`, so a rarer element costs more energy). Returns
    /// `(element, joule_spent, minted, eaten)` to receipt as
    /// `ElementFarmed`, or `None` for a non-eater, no bill, or no star.
    /// Every v1 (biotic/unsealed) world reads `None` in O(1) — byte-identical.
    ///
    /// The larder moves by `minted − eaten`: in steady state the line
    /// farms exactly what it ate and holds the buffer; short of joule it
    /// eats into the buffer and freezes; overstocked it coasts on the
    /// stored food and farms less. `eaten` never exceeds what is stored,
    /// so the larder never goes negative.
    pub(super) fn element_farm_at_touch(
        &self,
        orb: u64,
        stamp: u64,
    ) -> Option<([u8; 32], Ratio, Ratio, Ratio)> {
        let element = self.metabolic_element()?;
        let class = self.star?;
        let metabolism = self.genesis.as_ref().map(|held| held.metabolism)?;
        let held = self.orb_by_id(orb)?;
        let dt = stamp.saturating_sub(held.last_touch);
        if dt == 0 {
            return None;
        }
        // This beat's food bill, from the active zones (vigour-weighted).
        let active: Vec<(crate::dop::Zone, Ratio)> = held
            .zones
            .iter()
            .filter(|zone| held.zone_active(zone))
            .map(|zone| (zone.clone(), held.zone_vigour(zone)))
            .collect();
        let bill = Self::element_food_bill(&active, metabolism);
        if bill <= ratio(0, 1) {
            return None;
        }
        let (_, _, richness) = crate::dop::element_properties(&element, class);
        if richness <= ratio(0, 1) {
            return None;
        }
        let span = bill * ratio(dt as i64, 1); // a span's food: the demand AND the buffer
        let have = held.element_amount(&element);
        // EAT only what is stored — a frozen span (empty larder) eats nothing.
        let eaten = span.clone().min(have.clone());
        // REFILL toward the buffer after eating, joule-bounded.
        let joule = held.banked_amount(Resource::Joule);
        let want = (span - (have - eaten.clone())).max(ratio(0, 1));
        let minted = want.min(joule * richness.clone());
        if minted <= ratio(0, 1) && eaten <= ratio(0, 1) {
            return None;
        }
        Some((element, minted.clone() / richness, minted, eaten))
    }

    /// G2: the base FOOD BILL an element-eater's active zones levy this
    /// beat, in element units — the Biomass upkeep components (the food
    /// slot) summed and vigour-weighted, exactly the shape ECON-J's
    /// heating burn takes. This is both the STASIS threshold (a line
    /// that cannot meet it freezes) and the amount consumed when it can.
    pub(super) fn element_food_bill(
        active: &[(Zone, Ratio)],
        metabolism: u8,
    ) -> Ratio {
        active.iter().fold(ratio(0, 1), |sum, (zone, vigour)| {
            let per_zone: Ratio = crate::dop::zone_upkeep(zone.priced_as(), metabolism)
                .into_iter()
                .filter(|(resource, _)| *resource == Resource::Biomass)
                .fold(ratio(0, 1), |s, (_, amount)| s + amount);
            sum + per_zone * vigour.clone()
        })
    }

    /// E3.7 — strike/relieve material wants for prospected open matter.
    ///
    /// Strike: vein prospected, line within civ_reach, local to star,
    /// bank empty, not already wanted. Reading = ticks since prospect
    /// (last_assay as proxy).
    /// Relieve: bank holds any positive amount of that element.
    fn sweep_element_wants(&mut self, stamp_now: u64) {
        let reach = self.civ_reach();
        let class = self.star;
        let prospects: Vec<_> = self
            .prospects
            .iter()
            .map(|p| (p.orb, p.element, p.last_assay))
            .collect();
        for (orb, element, last) in prospects {
            let line = crate::almanac::periodic_of(&element).line;
            if line == 0 || line > reach {
                continue;
            }
            if let Some(c) = class {
                if !crate::almanac::local_to(&element, c) {
                    continue;
                }
            }
            let banked = self
                .orb_by_id(orb)
                .map(|held| held.element_amount(&element))
                .unwrap_or_else(|| ratio(0, 1));
            let wanted = self
                .element_wants
                .iter()
                .any(|(o, e)| *o == orb && e == &element);
            if banked > ratio(0, 1) {
                if wanted {
                    self.element_wants
                        .retain(|(o, e)| !(*o == orb && e == &element));
                    self.chronicle.append(
                        ClosureEvent::ElementWantRelieved {
                            orb_id: orb,
                            element,
                        },
                        stamp_now,
                        vec![],
                    );
                }
                continue;
            }
            // Empty bank + opened vein → hunger (once).
            if !wanted {
                let reading = ratio(stamp_now.saturating_sub(last) as i64, 1);
                self.element_wants.push((orb, element));
                self.chronicle.append(
                    ClosureEvent::ElementWantStruck {
                        orb_id: orb,
                        element,
                        reading,
                    },
                    stamp_now,
                    vec![],
                );
            }
        }
    }
}
