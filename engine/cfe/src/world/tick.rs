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
        use crate::dop::{encasement_throughput, joule_leak_per_tick, radiator_base};
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
            // COVENANT-1: the vitrified pacts resolve at the boundary —
            // fulfilment releases, the passed deadline slashes;
            // fulfilment wins the tie. World-driven, the famine shape.
            let due: Vec<(u64, u64, Resource, bool)> = self
                .covenants
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
                let Some(at) = self.covenants.iter().position(|pact| {
                    pact.obligor == obligor
                        && pact.beneficiary == beneficiary
                        && pact.resource == resource
                }) else {
                    continue;
                };
                let pact = self.covenants.remove(at);
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
                        true => held.covenants_fulfilled += stake,
                        false => held.covenants_breached += stake,
                    }
                }
                let event = match fulfilled {
                    true => ClosureEvent::CovenantReleased {
                        obligor_orb: obligor,
                        beneficiary_orb: beneficiary,
                        resource,
                        collateral: pact.collateral,
                    },
                    false => ClosureEvent::CovenantSlashed {
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
            // members judged together (the group over the individual;
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
                            true => held.covenants_fulfilled += stake,
                            false => held.covenants_breached += stake,
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
        let flows: Vec<(u64, Vec<Zone>)> = self
            .orbs
            .iter()
            .map(|orb| {
                (
                    orb.orb_id,
                    orb.zones
                        .iter()
                        .filter(|zone| orb.zone_active(zone))
                        .cloned()
                        .collect(),
                )
            })
            .collect();
        let one = ratio(1, 1);
        let zero = ratio(0, 1);
        let stamp_now = self.clock.now();
        // D3: the charted seed, read once — uncharted boosts read 1.
        let terrain = self.terrain_seed;
        // PRACTICE-1: the bound disciplines, read once — one benefit,
        // one friction each (14 §4; constants placeholder).
        let fast = self.practices.contains(&1);
        let icon = self.practices.contains(&2);
        let hoard = self.practices.contains(&4);
        let athanor = self.practices.contains(&5);
        // TECH-1b: each orb's crystallized boons, read once — [unused,
        // insulation, discipline, deepening, doctrine, stores].
        let boons: Vec<(u64, [bool; 6])> = flows
            .iter()
            .map(|(orb_id, _)| {
                let mut held = [false; 6];
                for (orb, condition, _) in &self.techs {
                    if orb == orb_id && (*condition as usize) < 6 {
                        held[*condition as usize] = true;
                    }
                }
                (*orb_id, held)
            })
            .collect();
        for (orb_id, active) in &flows {
            let boon = boons
                .iter()
                .find(|(held, _)| held == orb_id)
                .map(|(_, held)| *held)
                .unwrap_or([false; 6]);
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
                let t_eq4 = thermal::t_eq4(&class.luminosity(), &site.albedo, &site.orbit_r);
                // ECON-J freeze-lock (05 §1.2): a sub-mesic orb in joule
                // famine is frozen whole — no flows until fuel arrives.
                let sub_mesic = t_eq4 < thermal::SubstrateBand::Mesic.bounds4().0;
                frozen = sub_mesic && joule_famined;
                let stellar_load = t_eq4.clone() / thermal::thermal_k();
                let factory: Ratio = active
                    .iter()
                    .fold(zero.clone(), |sum, zone| sum + zone.kind.factory_heat());
                // TECH-1b: the doctrine crystallized — the radiator
                // sheds at 5/4 (10 §3).
                let radiator = match boon[4] {
                    true => radiator_base() * ratio(5, 4),
                    false => radiator_base(),
                };
                let heat_net = stellar_load + factory - radiator;
                // ECON-J (05 §1.2): heating upkeep on a sub-mesic orb —
                // every active zone burns joules against the cold.
                if sub_mesic && !frozen && !active.is_empty() {
                    let mut burn =
                        crate::dop::heating_per_zone() * ratio(active.len() as i64, 1);
                    // TECH-1b: insulation crystallized — the cold
                    // bites at 3/4 (10 §3).
                    if boon[1] {
                        burn *= ratio(3, 4);
                    }
                    accrue(&mut self.accrued, *orb_id, Resource::Joule, -burn);
                }
                // SINK-1 (05 §1.4): vault-band headroom is captured as
                // bankable sink by active cryo-lattice arrays — the
                // infrastructure gate (research/throttle gates deferred).
                if !frozen && heat_net < zero && t_eq4 <= thermal::cryo_vault_max4() {
                    let arrays = active
                        .iter()
                        .filter(|zone| zone.kind == ZoneKind::CryoArray)
                        .count() as i64;
                    if arrays > 0 {
                        let headroom = -heat_net.clone();
                        let mut lattice = crate::dop::sink_throughput() * ratio(arrays, 1);
                        // TECH-1b: the lattice deepened — capture at
                        // 5/4 (10 §3).
                        if boon[3] {
                            lattice *= ratio(5, 4);
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
                    let shells = active
                        .iter()
                        .filter(|zone| zone.kind == ZoneKind::Encasement)
                        .count() as i64;
                    let mut throughput = encasement_throughput() * ratio(shells, 1);
                    // TECH-1b: discipline crystallized — the shells
                    // encase at 5/4 (10 §3).
                    if boon[2] {
                        throughput *= ratio(5, 4);
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
            if !frozen {
                // G2 (04 §1.2): every flow reads through the sealed
                // metabolism's ration/harvest map — biotic and
                // unsealed read ruleset v1 verbatim.
                let metabolism = self.genesis.as_ref().map(|held| held.metabolism).unwrap_or(2);
                for zone in active {
                    let (resource, amount) = crate::dop::zone_yield(zone.kind, metabolism);
                    let boost = terrain_boost(&terrain, *orb_id, zone, active);
                    let mut yield_factor = ratio(1, 1);
                    let mut upkeep_factor = ratio(1, 1);
                    if athanor {
                        yield_factor *= ratio(5, 4);
                        upkeep_factor *= ratio(5, 4);
                    }
                    match zone.kind {
                        ZoneKind::Habitat => {
                            if icon {
                                yield_factor *= ratio(5, 4);
                                upkeep_factor *= ratio(2, 1);
                            }
                            if fast {
                                yield_factor *= ratio(3, 4);
                            }
                        }
                        ZoneKind::Extractor if fast => {
                            upkeep_factor *= ratio(3, 4);
                        }
                        ZoneKind::Smelter | ZoneKind::Synthesizer | ZoneKind::Resonator
                            if hoard =>
                        {
                            yield_factor *= ratio(3, 4);
                        }
                        _ => {}
                    }
                    accrue(
                        &mut self.accrued,
                        *orb_id,
                        resource,
                        amount * efficiency.clone() * boost * yield_factor.clone(),
                    );
                    for (resource, amount) in crate::dop::zone_upkeep(zone.kind, metabolism) {
                        accrue(&mut self.accrued, *orb_id, resource, -amount * upkeep_factor.clone());
                    }
                }
            }
            // The cryo law (05 §1.2): accrued joules leak outside the
            // vault (T⁴ > 100⁴), floored at zero; lossless in the cold.
            if let Some(t4) = t4 {
                if t4 > thermal::cryo_vault_max4() {
                    let held_joules = triple_amount(&self.accrued, *orb_id, Resource::Joule);
                    if held_joules > zero {
                        let leak = held_joules.min(joule_leak_per_tick());
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
                        // novelty: an empiricist mind starved of
                        // signal (a discovery must EXIST to starve
                        // for the next).
                        (3, sealed.disposition == 2, sheet.last_node, 480),
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
                        .any(|(held, kind, _)| *held == orb && *kind == condition)
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
                    self.techs.push((orb, condition, node));
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
        let Some(held) = peel(&mut self.orbs, orb) else {
            return;
        };
        let heating_need: Ratio = match (star, &held.thermal) {
            (Some(class), Some(site)) => {
                let t4 =
                    crate::thermal::t_eq4(&class.luminosity(), &site.albedo, &site.orbit_r);
                match t4 < crate::thermal::SubstrateBand::Mesic.bounds4().0 {
                    true => {
                        crate::dop::heating_per_zone() * ratio(held.zones.len() as i64, 1)
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
}
