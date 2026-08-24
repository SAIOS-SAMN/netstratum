//! TECH AND PRACTICE — conditions, instruments, adopted disciplines.
//!
//! THE CLIFF (task #4/#5): `condition == 5` and `hardships < 8` gate
//! outcomes here, so everything short of the line reads as nothing.
//! P3 tried to remove the first and was reverted -- see `super::rules`.

use super::*;

impl World {
    /// Every adopted discipline's mechanism, in ONE fold walk — the
    /// FOLD, seed ∪ minted, so a minted discipline computes exactly as
    /// a seeded one (#36 step 3, the mint door). An entry never
    /// mutates once minted, so today's fold carries the binding any
    /// adoption stamp saw — the ordering law holds for free.
    pub(super) fn adopted_practice_bindings(&self) -> Vec<Vec<(u8, Ratio)>> {
        let standing =
            crate::vocabulary::fold(self.chronicle(), crate::vocabulary::Axis::Practice);
        self.practices
            .iter()
            .map(|code| {
                standing
                    .iter()
                    .find(|entry| entry.code == *code)
                    .map(|entry| entry.binding.clone())
                    .unwrap_or_default()
            })
            .collect()
    }

    /// The composed factor the ADOPTED disciplines apply on a channel —
    /// the product across practices, 1 where all are silent. This is
    /// the read that replaced the bespoke `contains(&N)` branches
    /// (operator ruling 2026-08-05): a practice ACTS only through its
    /// binding data.
    pub(super) fn practice_factor(&self, channel: u8) -> Ratio {
        self.adopted_practice_bindings()
            .iter()
            .fold(ratio(1, 1), |acc, binding| {
                acc * crate::vocabulary::factor_on(binding, channel)
            })
    }

    /// The strictest gate any adopted discipline declares on a channel
    /// — thresholds take the MAX, never the product: two disciplines
    /// each holding the climb to a half do not soften it to a quarter.
    /// `None` when no adopted practice binds the gate.
    pub(super) fn practice_gate(&self, channel: u8) -> Option<Ratio> {
        self.adopted_practice_bindings()
            .iter()
            .filter_map(|binding| crate::vocabulary::gate_of(binding, channel))
            .max()
    }

    /// TECH-1a: the discovered nodes — (orb, condition, node id).
    pub fn techs(&self) -> &[(u64, u8, [u8; 32], Ratio)] {
        &self.techs
    }
    /// #40 (the petition's executor): GRANT a technology node under
    /// steward containment — the inbox form of the licensed arrival
    /// (the two-book licensing pair is recorded debt until NS-5
    /// arbitrates a true grantor). The crown law holds HERE: condition
    /// 6 lands only on a complete base-7 stack; the solar leap is
    /// never granted; one node per condition per orb.
    pub fn grant_tech_as(
        &mut self,
        actor: u64,
        orb: u64,
        condition: u8,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if condition == 0 {
            return Err(ActionRefused::SolarLeapNeverLicensed);
        }
        // DERIVED, not a magic range: the licensable nodes are exactly
        // the crystallized tier and the crown — asked of the vocabulary
        // (`node_kind`), so a sixth condition landing moves this gate
        // by itself.
        use crate::world::survey::{node_kind, NodeKind};
        if !matches!(node_kind(condition), NodeKind::Crystallized | NodeKind::Crown) {
            return Err(ActionRefused::NodeUnlicensable { condition });
        }
        if condition == 6 && !self.base_seven_complete() {
            return Err(ActionRefused::BaseSevenUnfinished);
        }
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        if self.techs.iter().any(|(o, c, _, _)| *o == orb && *c == condition) {
            return Err(ActionRefused::NodeAlreadyHeld { condition });
        }
        let mut seed = b"tech.granted".to_vec();
        seed.extend_from_slice(&orb.to_le_bytes());
        seed.push(condition);
        if let Some(receipt) = self.chronicle.receipts().last() {
            seed.extend_from_slice(&receipt.own().0);
        }
        let node: [u8; 32] = blake3::hash(&seed).into();
        let name = format!("gifted {}", crate::dop::tech_node_name(condition, &node));
        let stamp = self.clock.now();
        let (modifier, magnitude) =
            crate::dop::tech_node_of(condition).unwrap_or((condition, ratio(1, 1)));
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
            stamp,
            vec![actor],
        );
        Ok(())
    }
    /// MANU-1 (10 §3f): FORGE AN INSTRUMENT — manufacturing overlays
    /// technology + understanding + industry. Grade IS the
    /// crystallized condition; the converter chain must stand (a
    /// smelter for grades 1–2, smelter + synthesizer for 3+); the
    /// frame draws alloy = grade, and grade 3+ optics draw one
    /// diamond (v1 placeholders at the condition table's cadence).
    /// The scan-reach law reads the best grade held: reach = base +
    /// grade — never a global constant again.
    pub fn forge_instrument(
        &mut self,
        orb: u64,
        grade: crate::world::survey::ForgeableGrade,
    ) -> Result<(), ActionRefused> {
        // P8.1 — shared AP (research family).
        self.require_ap(crate::world::ApKind::Research)?;
        // The grade shape died at the `ForgeableGrade` door
        // (29_CONSTRUCTION): the vocabulary said crystallized or crown,
        // or this call could not be written. Only the facts of this
        // orb's sky remain.
        let condition = grade.condition();
        let tech_hash = self
            .techs
            .iter()
            .find(|(held, held_condition, _, _)| *held == orb && *held_condition == condition)
            .map(|(_, _, node, _)| *node)
            .ok_or(ActionRefused::ConditionUncrystallized { condition })?;
        // P5.21 instrument-guard: refuse identical re-forge (anti-spoof).
        self.instrument_guard_ok(orb, tech_hash, condition)?;
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let chain = match condition {
            1 | 2 => vec![ZoneKind::Smelter],
            _ => vec![ZoneKind::Smelter, ZoneKind::Synthesizer],
        };
        for needs in chain {
            if !held.zones.iter().any(|zone| zone.kind == needs) {
                return Err(ActionRefused::ChainIncomplete { needs });
            }
        }
        let mut costs = vec![(Resource::Alloy, ratio(condition as i64, 1))];
        if condition >= 3 {
            costs.push((Resource::Diamond, ratio(1, 1)));
        }
        for (resource, need) in &costs {
            let have = held.banked_amount(*resource);
            if have < *need {
                return Err(ActionRefused::CannotAfford {
                    resource: *resource,
                    need: Box::new(need.clone()),
                    have: Box::new(have),
                });
            }
        }
        for (resource, need) in costs {
            held.bank(resource, -need);
        }
        self.instruments.push((orb, tech_hash, condition));
        self.close(
            ClosureEvent::InstrumentForged { orb_id: orb, tech_hash, grade: condition },
            SOVEREIGN,
        );
        Ok(())
    }
    /// MANU-1: the held instruments — (orb, tech node, grade).
    pub fn instruments(&self) -> &[(u64, [u8; 32], u8)] {
        &self.instruments
    }
    /// TECH-1b (10 §3): does a condition HOLD, read from the deposit
    /// sheet + committed state? Thresholds stay the table's
    /// placeholder parameters.
    pub(super) fn condition_holds(&self, orb: u64, condition: u8, now: u64, sheet: &CrystalDeposit) -> bool {
        let Some(held) = peek(&self.orbs, orb) else {
            return false;
        };
        match condition {
            1 => {
                let open = sheet
                    .famine_open
                    .map(|struck| now.saturating_sub(struck))
                    .unwrap_or(0);
                sheet.famine_total + open >= 480
            }
            2 => sheet
                .overheat_open
                .is_some_and(|struck| now.saturating_sub(struck) >= 240),
            3 => {
                held.banked_amount(Resource::Sink) >= ratio(100, 1)
                    && demurrage_due(held, self.star, now, &ease_of(&self.techs, orb, now), &standing_supply(&self.orbs))
                        .iter()
                        .all(|(_, due)| *due == ratio(0, 1))
            }
            4 => {
                let floor = now.saturating_sub(self.deposit_window());
                let vented: Ratio = sheet
                    .vented_entries
                    .iter()
                    .filter(|(stamp, _)| *stamp >= floor)
                    .map(|(_, amount)| amount.clone())
                    .sum();
                let encased: Ratio = sheet
                    .encased_entries
                    .iter()
                    .filter(|(stamp, _)| *stamp >= floor)
                    .map(|(_, amount)| amount.clone())
                    .sum();
                vented > ratio(0, 1) && vented > encased
            }
            5 => {
                let active = held
                    .zones
                    .iter()
                    .filter(|zone| held.zone_active(zone))
                    .count() as i64;
                if active == 0 {
                    return false;
                }
                let bill_year =
                    crate::parameter::of(self.chronicle(), crate::parameter::Parameter::HeatingPerZone)
                        * ratio(active, 1)
                        * ratio(60, 1);
                let cold = match (self.star, &held.thermal) {
                    (Some(class), Some(site)) => {
                        let t4 = crate::thermal::t_eq4(
                            &class.luminosity(),
                            site.albedo.value(),
                            site.orbit_r.value(),
                        );
                        t4 < crate::thermal::SubstrateBand::Mesic.bounds4().0
                    }
                    _ => false,
                };
                cold && held.banked_amount(Resource::Joule) >= ratio(4, 1) * bill_year
            }
            _ => false,
        }
    }
    /// **NOVELTY-GATE (tag 166, 38 §4)** — mint a solver WORD into this
    /// line's own library. Build scope (the mint re-rules what this
    /// line can REACH, the founding class). The gates that make the
    /// candidate lawful (unique orbit · productive · verifier-conserved
    /// · three confirms) run prover-side in `solver::mint`; this door
    /// holds the CHAIN's share: an orbit already receipted refuses (a
    /// re-spelling can never re-mint — the fold's own idempotence), an
    /// empty expansion or short confirmation refuses (nothing enters
    /// the library unearned).
    pub fn mint_solver_word_as(
        &mut self,
        actor: u64,
        orbit: [u8; 32],
        expansion: Vec<u8>,
        cited: Vec<[u8; 32]>,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if expansion.is_empty() {
            return Err(ActionRefused::SolverWordEmpty);
        }
        // COMM-1 — CONGRUENCE THRESHOLD. A shared word is admitted when TWO
        // independent vantages have converged on the same orbit (the consilience
        // principle, N-I8: two distinct minds agreeing IS a measurement of truth).
        // The prior bar of three distinct tasks demanded proof of GENERALITY before
        // ADMISSION, and it was so strict the vocabulary never formed (0 words across
        // the live population) — communication died at the gate. Two vantages is the
        // trigger; generality hardens the word afterward through reuse (`pre_reach`),
        // not as a precondition to speaking it. (This count is the crude first gauge
        // of congruence — the degree of agreement thresholded to a mint; the graded
        // residual measure is the evolution once its live behaviour is observed.)
        if cited.len() < 2 {
            return Err(ActionRefused::SolverWordUnconfirmed { confirms: cited.len() as u8 });
        }
        let known = self.chronicle.receipts().iter().any(|receipt| {
            matches!(
                receipt.event(),
                ClosureEvent::SolverWordMinted { orbit: held, .. } if *held == orbit
            )
        });
        if known {
            return Err(ActionRefused::SolverWordKnown { orbit });
        }
        self.close(
            ClosureEvent::SolverWordMinted { orbit, expansion, cited },
            actor,
        );
        Ok(())
    }

    /// **ENTITY-ALIGN-1 (tag 189, docs/56 §3) — RECORD A WORD'S HARDNESS.**
    /// Closed alongside a successful `mint_solver_word_as`, never instead of
    /// it: the mint gate (`cited.len() >= 2`) is untouched, this only
    /// records HOW diverse the confirming vessels' own knowledge was when
    /// they agreed. Purely additive bookkeeping — nothing yet reads or
    /// spends this field.
    pub fn record_word_hardness_as(
        &mut self,
        actor: u64,
        orbit: [u8; 32],
        hardness: u32,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        self.close(ClosureEvent::SolverWordHardness { orbit, hardness }, actor);
        Ok(())
    }

    /// **RCP-2 (tag 181) — RAISE AN EMERGENT RECIPE.** A society builds a
    /// recipe its discovery engine (`conductor::recipe::discover`) minted: a
    /// hashed transformation `∏ inputs → role` over the OPEN almanac, priced
    /// by no `ZoneKind` and gated by no integer level — the doing-layer's
    /// enum and scalar, both dissolved. This door holds the CHAIN's share: a
    /// recipe with no inputs, or one that delivers nothing of its role, is
    /// not a recipe (the discovery engine returns an honest `None` for such a
    /// hole, and the door refuses in kind); and the SAME recipe (inputs ‖
    /// role) already raised on this orb refuses — the built set is a FOLD
    /// (`vocabulary::recipes_built`), minted once, never re-built. The caller
    /// (the conductor's workbench) owns the species affinity and topological
    /// reach gate; this door records the build so the fold reads it.
    pub fn build_recipe_as(
        &mut self,
        actor: u64,
        orb_id: u64,
        inputs: Vec<[u8; 32]>,
        role: usize,
        yield_of_role: Ratio,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if inputs.is_empty() {
            return Err(ActionRefused::RecipeInputsEmpty);
        }
        if yield_of_role <= ratio(0, 1) {
            return Err(ActionRefused::RecipeYieldEmpty);
        }
        let built = self.chronicle.receipts().iter().any(|receipt| {
            matches!(
                receipt.event(),
                ClosureEvent::RecipeBuilt { orb_id: held_orb, inputs: held, role: held_role, .. }
                    if *held_orb == orb_id && *held == inputs && *held_role == role
            )
        });
        if built {
            return Err(ActionRefused::RecipeAlreadyBuilt { orb_id });
        }
        self.close(
            ClosureEvent::RecipeBuilt { orb_id, inputs, role, yield_of_role },
            actor,
        );
        Ok(())
    }

    /// **CONSILIENCE (tag 177, 38 §5)** — record this line's participation
    /// in a shared knowledge axiom. The prover side (`solver::consilience`)
    /// established that INDEPENDENT lines resolved their knowledge to the
    /// same crystallized `signature`; this door holds the CHAIN's share,
    /// recording the line's own leg. Idempotent per signature (a line
    /// already recording this convergence refuses — the fold's own
    /// idempotence), and a consilience is agreement between at least two
    /// lines (`mass < 2` refuses: one line is no measurement).
    pub fn crystallize_knowledge_as(
        &mut self,
        actor: u64,
        signature: [u8; 32],
        mass: u32,
        hardness: u32,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if mass < 2 {
            return Err(ActionRefused::ConsilienceAlone { mass });
        }
        let known = self.chronicle.receipts().iter().any(|receipt| {
            matches!(
                receipt.event(),
                ClosureEvent::KnowledgeConverged { signature: held, .. } if *held == signature
            )
        });
        if known {
            return Err(ActionRefused::KnowledgeKnown { signature });
        }
        self.close(
            ClosureEvent::KnowledgeConverged { signature, mass, hardness },
            actor,
        );
        Ok(())
    }

    /// **THE KNOWLEDGE MARKET (tag 178, ARC C)** — license a word this
    /// line lacks into its own library. The prover side named a word some
    /// OTHER line minted and this line does not hold; this door holds the
    /// CHAIN's share: an orbit already in the library refuses (a line
    /// never licenses what it already knows — minted OR licensed), an
    /// empty expansion refuses (nothing enters the library unearned). The
    /// word is banked-neutral identity; the buyer's PROSPERITY gate (it
    /// can afford the price) is the market's, applied by the caller — this
    /// door only records the acquisition so the fold reads it.
    pub fn license_knowledge_as(
        &mut self,
        actor: u64,
        orbit: [u8; 32],
        expansion: Vec<u8>,
        cited: Vec<[u8; 32]>,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if expansion.is_empty() {
            return Err(ActionRefused::SolverWordEmpty);
        }
        let held = self.chronicle.receipts().iter().any(|receipt| {
            matches!(
                receipt.event(),
                ClosureEvent::SolverWordMinted { orbit: held, .. }
                    | ClosureEvent::KnowledgeLicensed { orbit: held, .. }
                    if *held == orbit
            )
        });
        if held {
            return Err(ActionRefused::SolverWordKnown { orbit });
        }
        self.close(
            ClosureEvent::KnowledgeLicensed { orbit, expansion, cited },
            actor,
        );
        Ok(())
    }

    /// PRACTICE-1 — adopt a discipline: one benefit, one friction,
    /// bound by receipt (the derived creed never binds; the practice
    /// does). Build scope. The Overdrive (5) takes only the tempered —
    /// eight hardship receipts stand in the chronicle or the door
    /// refuses.
    pub fn adopt_practice(&mut self, practice: u8) -> Result<(), ActionRefused> {
        self.adopt_practice_as(SOVEREIGN, practice)
    }
    pub fn adopt_practice_as(&mut self, actor: u64, practice: u8) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        // THE MINT DOOR IS OPEN (#36 step 3): the lawful disciplines
        // are the FOLD — seed ∪ minted — so this is a FACT of the
        // chain, never a compiled range. The 1..=5 guard died here;
        // the arm survives because what the fold holds is not
        // decidable from the code alone (the UnknownAxisValue family).
        let binding = crate::vocabulary::fold(
            self.chronicle(),
            crate::vocabulary::Axis::Practice,
        )
        .into_iter()
        .find(|entry| entry.code == practice)
        .map(|entry| entry.binding)
        .ok_or(ActionRefused::PracticeUnknown { practice })?;
        if self.practices.contains(&practice) {
            return Err(ActionRefused::PracticeAlreadyAdopted { practice });
        }
        // The tempering gate, read from the discipline's OWN data
        // (CHANNEL_TEMPER) — no practice is named here. Only the
        // Overdrive binds it today, so the refusal keeps its name.
        if let Some(threshold) =
            crate::vocabulary::gate_of(&binding, crate::vocabulary::CHANNEL_TEMPER)
        {
            let hardships = self
                .chronicle
                .receipts()
                .iter()
                .filter(|receipt| {
                    matches!(
                        receipt.event(),
                        ClosureEvent::FamineStruck { .. }
                            | ClosureEvent::OverheatStruck { .. }
                            | ClosureEvent::VentSettled { .. }
                            | ClosureEvent::DemurrageSettled { .. }
                            | ClosureEvent::DemurrageTouched { .. }
                    )
                })
                .count();
            if ratio(hardships as i64, 1) < threshold {
                return Err(ActionRefused::OverdriveUntempered { hardships });
            }
        }
        self.close(ClosureEvent::PracticeAdopted { practice }, actor);
        self.practices.push(practice);
        Ok(())
    }
    /// PRACTICE-1 — release is a REDISCOVERY (operator-ruled): lawful
    /// only after new doctrine landed since the adoption. A lawful
    /// release emits the RE-EMERGENCE node (TechDiscovered, condition
    /// 7) seeded from the chronicle root and the practice — divergent
    /// and novel per line per moment; its window-ease boon applies
    /// while alignment holds (the tick reads it).
    pub fn release_practice(&mut self, practice: u8) -> Result<(), ActionRefused> {
        self.release_practice_as(SOVEREIGN, practice)
    }
    pub fn release_practice_as(&mut self, actor: u64, practice: u8) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !self.practices.contains(&practice) {
            return Err(ActionRefused::PracticeNotHeld { practice });
        }
        let adopted_at = self
            .chronicle
            .receipts()
            .iter()
            .rposition(|receipt| {
                matches!(receipt.event(), ClosureEvent::PracticeAdopted { practice: held } if *held == practice)
            })
            .expect("the adoption receipted");
        let rediscovered = self.chronicle.receipts()[adopted_at..]
            .iter()
            .any(|receipt| matches!(receipt.event(), ClosureEvent::TechDiscovered { .. }));
        if !rediscovered {
            return Err(ActionRefused::ReleaseUnearned { practice });
        }
        self.close(ClosureEvent::PracticeReleased { practice }, actor);
        if let Some(at) = self.practices.iter().position(|held| *held == practice) {
            self.practices.remove(at);
        }
        // The re-emergence: a novel node from the current state.
        let root = self
            .chronicle
            .receipts()
            .last()
            .map(|receipt| receipt.own().0)
            .expect("the release receipted");
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&root);
        seed[0] ^= practice;
        let name = crate::dop::tech_node_name(7, &seed);
        let magnitude = ratio(3, 4);
        self.close(
            ClosureEvent::TechDiscovered {
                orb_id: FIRST_ORB,
                node: seed,
                condition: 7,
                modifier: 7,
                magnitude: magnitude.clone(),
                name,
            },
            actor,
        );
        self.techs.push((FIRST_ORB, 7, seed, magnitude));
        Ok(())
    }
}
