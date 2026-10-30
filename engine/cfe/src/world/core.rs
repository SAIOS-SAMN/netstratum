//! Core — accessors and small deeds that name no larger concern.
//!
//! Anything here that grows a concern of its own should leave.

use super::*;

impl World {
    /// TECH-1a: has this world's people made the stellar leap — do they
    /// know the powers of their star? Pre-stellar is a lawful state
    /// (12_UNION §5a); the reading gates licensing and the shield,
    /// never the world's own life.
    pub fn knows_its_star(&self) -> bool {
        self.techs.iter().any(|(_, condition, _, _)| *condition == 0)
    }
    /// WORLDSHAPE-1: is the BASE-7 stack complete — the stellar leap plus
    /// all five crystallized conditions? Six earned nodes; the gifted
    /// seventh (worldshaping) completes base-7. Derived, never stored.
    pub fn base_seven_complete(&self) -> bool {
        (0u8..=5).all(|condition| {
            self.techs.iter().any(|(_, held, _, _)| *held == condition)
        })
    }
    /// WANT-1: the struck cognitive wants — (orb, kind).
    pub fn wants(&self) -> &[(u64, u8)] {
        &self.wants
    }

    /// E3.7: material wants — (orb, ElementId).
    pub fn element_wants(&self) -> &[(u64, [u8; 32])] {
        &self.element_wants
    }
    pub fn now(&self) -> u64 {
        self.clock.now()
    }
    pub fn chronicle(&self) -> &Chronicle {
        &self.chronicle
    }
    /// The memory bound: drain the held receipts below absolute position
    /// `upto` into the chronicle's annals (the head always stays held).
    /// Only a carrier whose snapshot already holds them calls this.
    pub fn drain_chronicle_to(&mut self, upto: u64) {
        self.chronicle.drain_to(upto);
    }
    /// The memory bound: file the chronicle's drained own-hashes not yet in
    /// the digest file at `path` (append-only), then let memory release them.
    /// A file holding more than the annals have filed (appended, then the
    /// snapshot naming it never landed) is cut back first; nothing already
    /// filed moves.
    pub fn file_annals(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        use std::io::Write;
        let annals = self.chronicle.annals_mut();
        let (_, filed) = annals.filed();
        let file = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
        let len = file.metadata()?.len();
        if len != filed * 32 {
            if len < filed * 32 {
                return Err(std::io::Error::other("the digest file is shorter than what it has filed"));
            }
            file.set_len(filed * 32)?;
        }
        let mut file = file;
        let mut out = Vec::with_capacity(annals.unfiled().len() * 32);
        for d in annals.unfiled() {
            out.extend_from_slice(d);
        }
        file.write_all(&out)?;
        file.flush()?;
        annals.mark_filed(path.to_path_buf());
        Ok(())
    }
    /// MESH-1b: hold or release the writing pen — every internal
    /// append then carries the hand (the chronicle funnel stamps it).
    pub fn hold_pen(&mut self, hand: Option<u64>) {
        self.chronicle.set_hand(hand);
    }
    /// MESH-1b: the mesh doors (yield, consile) take the chronicle
    /// mutably. Append-only holds — Chronicle exposes no other write.
    pub fn chronicle_mut(&mut self) -> &mut Chronicle {
        &mut self.chronicle
    }
    /// **This world's own NAME** — the genesis colony's, which is what
    /// `found` set and what every other world calls it (#42). A treaty
    /// names its partner by this, so the pair is identified by the same
    /// string on both books.
    pub fn name(&self) -> &str {
        self.orbs.first().map(|held| held.name.as_str()).unwrap_or("")
    }
    /// The genesis orb (convenience for the single-planet reads).
    pub fn orb(&self) -> Option<&ColonyOrb> {
        self.orbs.first()
    }
    pub fn orbs(&self) -> &[ColonyOrb] {
        &self.orbs
    }
    pub fn orb_by_id(&self, orb: u64) -> Option<&ColonyOrb> {
        peek(&self.orbs, orb)
    }
    /// This orb's accrued workspace.
    pub fn accrued_of(&self, orb: u64) -> Vec<(Resource, Ratio)> {
        self.accrued
            .iter()
            .filter(|(held, _, _)| *held == orb)
            .map(|(_, resource, amount)| (*resource, amount.clone()))
            .collect()
    }
    /// The genesis orb's workspace (single-planet convenience).
    pub fn accrued(&self) -> Vec<(Resource, Ratio)> {
        self.accrued_of(FIRST_ORB)
    }
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
    /// Register a scripted agent contained by the sovereign (N3).
    pub fn register_agent(&mut self, handle: &str) -> Result<u64, ActionRefused> {
        if self.entities.is_empty() {
            return Err(ActionRefused::NoSuchEntity { entity: SOVEREIGN });
        }
        if self.entities.iter().any(|held| held.handle == handle) {
            return Err(ActionRefused::HandleTaken {
                handle: handle.to_string(),
            });
        }
        let entity_id = self.entities.iter().map(|held| held.entity_id).max().unwrap_or(0) + 1;
        self.entities.push(Entity {
            entity_id,
            handle: handle.to_string(),
            principal: Some(SOVEREIGN),
            grants: Vec::new(),
        });
        self.close(
            ClosureEvent::EntityRegistered {
                entity_id,
                principal: SOVEREIGN,
                handle: handle.to_string(),
            },
            SOVEREIGN,
        );
        Ok(entity_id)
    }
    /// Grant an agent one action scope — a closure by the sovereign.
    pub fn grant(&mut self, agent: u64, scope: Scope) -> Result<(), ActionRefused> {
        let held = self
            .entities
            .iter_mut()
            .find(|held| held.entity_id == agent)
            .ok_or(ActionRefused::NoSuchEntity { entity: agent })?;
        if held.principal.is_none() {
            return Err(ActionRefused::NotAnAgent { entity: agent });
        }
        if !held.grants.contains(&scope) {
            held.grants.push(scope);
            held.grants.sort();
            self.close(ClosureEvent::ScopeGranted { agent, scope }, SOVEREIGN);
        }
        Ok(())
    }
    /// Revoke a grant — immediate; the agent's next such action refuses.
    pub fn revoke(&mut self, agent: u64, scope: Scope) -> Result<(), ActionRefused> {
        let held = self
            .entities
            .iter_mut()
            .find(|held| held.entity_id == agent)
            .ok_or(ActionRefused::NoSuchEntity { entity: agent })?;
        if held.principal.is_none() {
            return Err(ActionRefused::NotAnAgent { entity: agent });
        }
        if held.grants.contains(&scope) {
            held.grants.retain(|granted| *granted != scope);
            self.close(ClosureEvent::ScopeRevoked { agent, scope }, SOVEREIGN);
        }
        Ok(())
    }
    pub(super) fn permitted(&self, actor: u64, scope: Scope) -> Result<(), ActionRefused> {
        // 50_WIN_CONDITIONS: a collapsed line accepts no further scoped
        // action — checked first, ahead of entity/grant lookups, so a
        // collapsed world refuses uniformly regardless of who's asking.
        if self.collapsed.is_some() {
            return Err(ActionRefused::CivilizationCollapsed);
        }
        let entity = self
            .entity(actor)
            .ok_or(ActionRefused::NoSuchEntity { entity: actor })?;
        match entity.permits(scope) {
            true => Ok(()),
            false => Err(ActionRefused::NotPermitted { entity: actor, scope }),
        }
    }
    /// Raze on the genesis orb as the sovereign.
    pub fn raze(&mut self, tile: u16) -> Result<(), ActionRefused> {
        self.raze_as(SOVEREIGN, FIRST_ORB, tile)
    }
    /// Raze a zone of one orb for half its cost back (02 §2.4); Raze
    /// scope; famine relief re-checked for that orb.
    pub fn raze_as(&mut self, actor: u64, orb: u64, tile: u16) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Raze)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        let held = peel(&mut self.orbs, orb)
            .ok_or(ActionRefused::NoSuchOrb { orb })?;
        let at = held
            .zones
            .iter()
            .position(|zone| zone.tile == tile)
            .ok_or(ActionRefused::NoSuchZone { tile })?;
        let zone = held.zones.remove(at);
        // #41: the refund halves what the build spent, which for a
        // minted kind is its base's cost — `priced_as` is the one home
        // both ends read, so a building cannot be raised at one price
        // and salvaged at another.
        for (resource, refund) in zone.priced_as().raze_refund() {
            held.bank(resource, refund);
        }
        let orb_id = held.orb_id;
        self.close(ClosureEvent::ZoneRazed { orb_id, tile }, actor);
        self.relieve_recovered(orb);
        Ok(())
    }
    /// Collect one orb — settle its workspace against its bank;
    /// Collect scope; relief re-checked for that orb.
    /// The discovery mint (U1, tag 86) — value created against verified
    /// work, banked on the solving orb.
    ///
    /// This is the emitter the PoUW loop was missing. Before it, a
    /// solved task wrote a file and incremented a counter, and the
    /// economy never heard of it: cognition proved out and minted
    /// nothing.
    ///
    /// The world proposes; it does not gate. `work` rides whole as the
    /// opaque payload (the OPAQUE-PAYLOAD law — cfe never parses it),
    /// and the kernel re-runs `verify_work` at the door plus the ruled
    /// bound `f(difficulty)`. A caller computes its amount from
    /// `kernel::mint_bound` so proposer and gate read one function.
    ///
    /// Collect-scoped, matching `scope_of(DiscoveryMinted)`.
    pub fn mint_discovery_as(
        &mut self,
        actor: u64,
        orb: u64,
        resource: Resource,
        amount: crate::dop::NonNegative,
        work: Vec<u8>,
        warrant: &dyn MintWarrant,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        // A negative mint is unrepresentable (NonNegative); zero stays
        // lawful — a saturated rung honestly pays nothing.
        let amount = amount.value().clone();
        if !self.orbs.iter().any(|held| held.orb_id == orb) {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        // K4 the live gate. The emitter cannot verify — the
        // opaque-payload law bars `cfe` from importing the verifier, and
        // `recurrence_guard` fails the build on any attempt. So it does
        // the lawful thing instead: it calls back to whoever can.
        //
        // `cfe` never parses the payload; it hands the bytes to the
        // warrant and asks what they earn. A caller with no verifier
        // cannot mint at all, and a caller with one cannot mint past
        // what the work bounds — checked before `close`, so a refused
        // mint leaves no receipt. Previously the gate caught this
        // afterwards, by which point the chronicle held it and `replay`
        // granted the value.
        let bound = warrant.earns(&work).ok_or(ActionRefused::WorkRefused)?;
        if amount > bound {
            return Err(ActionRefused::MintExceedsWork {
                bound: Box::new(bound),
                claimed: Box::new(amount),
            });
        }
        // The escrow. The deed is built, the state it would produce is
        // built, and the gate is asked — all before anything is
        // closed. `earns` above priced the work; this asks whether the
        // transition stands, which is where D4 and conservation live.
        //
        // A mint that fails here leaves no receipt and moves no value:
        // it was never in the record to take back. Previously the gate
        // met these deeds afterwards, and a 60-beat era banked mints it
        // then refused — the drift the escrow makes unreachable rather
        // than merely visible.
        let event =
            ClosureEvent::DiscoveryMinted { orb_id: orb, resource, amount: amount.clone(), work };
        let prior = self.committed();
        let mut candidate = prior.clone();
        if let Some(held) = candidate.orbs.iter_mut().find(|held| held.orb_id == orb) {
            held.bank(resource, amount.clone());
        }
        let stamp = self.now();
        if !warrant.admits(&prior, &candidate, &event, actor, stamp) {
            return Err(ActionRefused::WorkRefused);
        }
        // The credit lands on the live orb (found 2026-08-06 by the
        // seam's digest perturbation): the escrow refactor banked the
        // mint on the candidate — a throwaway built for the gate — and
        // closed the receipt without ever crediting the world, so live
        // starved while replay granted (a SEAL B split, dormant until a
        // mint landed inside a replay-pinned arc). The mint banks like
        // a yield and anchors like a touch, exactly as replay applies
        // its receipt.
        if let Some(held) = peel(&mut self.orbs, orb) {
            held.bank(resource, amount);
            held.last_touch = stamp;
        }
        self.close(event, actor);
        Ok(())
    }
    /// The committed projection of the live world.
    pub fn committed(&self) -> Committed {
        Committed {
            orbs: self.orbs.clone(),
            entities: self.entities.clone(),
            star: self.star,
            debts: self.debts.clone(),
            bonds: self.bonds.clone(),
            forwards: self.forwards.clone(),
            councils: self.councils.clone(),
            offers: self.offers.clone(),
            techs: self.techs.clone(),
            wants: self.wants.clone(),
            element_wants: self.element_wants.clone(),
            crossed: self.crossed,
            gauge: self.gauge.clone(),
            genesis: self.genesis.clone(),
            terrain_seed: self.terrain_seed,
            practices: self.practices.clone(),
            instruments: self.instruments.clone(),
            routes: self.routes.clone(),
            element_routes: self.element_routes.clone(),
            treaties: self.treaties.clone(),
            prospects: self.prospects.clone(),
            warded: self.warded,
            veiled: self.veiled,
            taxon: self.taxon,
            region_seed: self.region_seed,
            evolution_stage: self.evolution_stage,
            collapsed: self.collapsed,
            stations: self.stations.clone(),
            hulls: self.hulls.clone(),
            notes: self.notes.clone(),
            futures: self.futures.clone(),
            last_clears: self.last_clears.clone(),
            shares: self.shares.clone(),
            perps: self.perps.clone(),
            clear_depths: self.clear_depths.clone(),
            market_sealed_until: self.market_sealed_until,
            cascade_pressures: self.cascade_pressures.clone(),
            tamper_pressures: self.tamper_pressures.clone(),
            posture_disposition: self.posture_disposition,
            posture_perception: self.posture_perception,
            posture_primary: self.posture_primary,
            intel_hashes: self.intel_hashes.clone(),
            spy_marks: self.spy_marks.clone(),
            vault_stamp: self.vault_stamp.clone(),
            chem_catalog: self.chem_catalog.clone(),
            science_seats: self.science_seats.clone(),
            clone_counts: self.clone_counts.clone(),
            pop_accrued: self.pop_accrued.clone(),
            treasury_balances: self.treasury_balances.clone(),
            leisure_banks: self.leisure_banks.clone(),
            synth_mods: self.synth_mods.clone(),
            skin_mods: self.skin_mods.clone(),
            rogue_pressure: self.rogue_pressure.clone(),
            element_allowlists: self.element_allowlists.clone(),
        }
    }
    /// ELEMENT-GRAMMAR-1 (14 §3) — the extraction closure: richness ×
    /// elapsed banked in one receipt (the bond-settle shape). A vein
    /// with no new span refuses NothingAccrued; an unopened identity
    /// refuses typed. Sovereign convenience.
    pub fn assay(&mut self, orb: u64, element: [u8; 32]) -> Result<Ratio, ActionRefused> {
        self.assay_as(SOVEREIGN, orb, element)
    }
    /// The contained form. Collect scope: the assay moves banked value.
    pub fn assay_as(
        &mut self,
        actor: u64,
        orb: u64,
        element: [u8; 32],
    ) -> Result<Ratio, ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        if self.orb_by_id(orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        // P5-C false-assay gate: assay needs a real prospect (below) and
        // at least surface detection when the line is veiled (instrument
        // or clear eyes). Unveiled free assay of opened veins stands.
        if self.is_veiled() {
            self.require_detection(Self::detection_need_veiled_scan())?;
        }
        let Some(class) = self.star else {
            return Err(ActionRefused::NoStarCommissioned);
        };
        // Vein must stand before workability gates (identity already mint).
        let Some(opened) = self
            .prospects
            .iter()
            .find(|held| held.orb == orb && held.element == element)
        else {
            return Err(ActionRefused::NoSuchElement { element });
        };
        // CIV-REACH-1 + locality + pressure: banking deep matter needs
        // the people's growth, the sky's forge, and opened stratum.
        if !crate::almanac::local_to(&element, class) {
            return Err(ActionRefused::ElementNotLocal { element });
        }
        let line = crate::almanac::periodic_of(&element).line;
        let reach = self.civ_reach();
        if line > reach {
            return Err(ActionRefused::CivReachShort {
                need: line,
                have: reach,
            });
        }
        if !crate::almanac::refines(&element, opened.stratum, class) {
            return Err(ActionRefused::PressureUnreached { element });
        }
        // ONT-2: the star is read before the vein is borrowed (proven Some
        // once a vein is found: prospecting requires it, and nothing
        // uncommissions a star). Regenerators no longer enter here — recovery
        // is no longer an in-assay recurrence term; it derives separately
        // (ONT-3: orbital forcing / topological cycle injection).
        let star = self.star;
        let now = self.clock.now();
        let Some(vein) = self
            .prospects
            .iter_mut()
            .find(|held| held.orb == orb && held.element == element)
        else {
            return Err(ActionRefused::NoSuchElement { element });
        };
        if now <= vein.last_assay {
            return Err(ActionRefused::NothingAccrued);
        }
        let span = now - vein.last_assay;
        // ONT-2: richness is derived (natural − extracted), never a stored
        // scalar; the flow is a geometric decay capped at the remaining
        // richness; the extraction integral grows by exactly what banks. No
        // wear recurrence, so nothing can collapse. The kernel re-derives the
        // identical amount from the same geometry and the prior integral.
        let class = star.expect("a vein stands, so a star was commissioned");
        let (_, _, natural) = crate::dop::element_properties(&element, class);
        let richness = crate::dop::vein_richness(&natural, &vein.extracted, &vein.rejuvenated);
        // #136 D3 — the element ledger declares its grain (1/1024) at the assay fill:
        // the vein flow is a geometric decay (a product of arbitrary rationals), so it
        // smelts to the element grain before it folds into the extraction integral,
        // receipts, or banks. The receipt carries this smelted Δ; replay reads it
        // (never re-derives), so `extracted` and the stock stay on-grain, byte-exact.
        let amount = smelt_element(&crate::dop::vein_flow(&richness, &natural, span));
        vein.extracted += amount.clone();
        vein.last_assay = now;
        let held = peel(&mut self.orbs, orb).expect("judged above");
        held.bank_element(element, amount.clone());
        // E3.7: banking feeds the material want.
        if self
            .element_wants
            .iter()
            .any(|(o, e)| *o == orb && e == &element)
        {
            self.element_wants
                .retain(|(o, e)| !(*o == orb && e == &element));
            self.close(
                ClosureEvent::ElementWantRelieved {
                    orb_id: orb,
                    element,
                },
                actor,
            );
        }
        self.close(
            ClosureEvent::AssayStruck {
                orb_id: orb,
                element,
                amount: amount.clone(),
                // ONT-2: no after-richness to carry — it is derived, never
                // stored. The receipt is the extraction integral's Δ (`amount`);
                // replay folds it into `extracted` and re-derives richness.
                richness: None,
            },
            actor,
        );
        Ok(amount)
    }
    /// G1 (04 §1.1, §4) — the disposition-weighted progress read:
    /// derived at read time from the same receipts every line shares
    /// (territory closures, discoveries, consumed yield), weighted by
    /// the sealed disposition. None on an unsealed world — no receipt,
    /// no read (04 §5 binding 2: identity is never law).
    pub fn scored_progress(&self) -> Option<Ratio> {
        let genesis = self.genesis.as_ref()?;
        let weights = crate::dop::disposition_weights(genesis.disposition)?;
        let mut territory = ratio(0, 1);
        let mut discovery = ratio(0, 1);
        let mut consumed = ratio(0, 1);
        for receipt in self.chronicle.history() {
            match receipt.event() {
                ClosureEvent::ZoneBuilt { .. } => territory += ratio(1, 1),
                ClosureEvent::TechDiscovered { .. } => discovery += ratio(1, 1),
                ClosureEvent::YieldCollected { amount, .. } => consumed += amount.clone(),
                _ => {}
            }
        }
        let [on_territory, on_discovery, on_consumed] = weights;
        Some(on_territory * territory + on_discovery * discovery + on_consumed * consumed)
    }
    /// NOVEL-1 (10 §3c) — the succession deed: a successor line adopts
    /// this derelict world's chronicle as its own substrate (07 §3).
    /// The refusal is computed pure (a living line refuses, nothing
    /// moves); on the deed the living tree drops to the world's kernel
    /// (condition 0 — the star is known to the WORLD, not to the line)
    /// and every other node leaves the tree while standing in the
    /// chronicle forever. The successor's conditions then evaluate
    /// over the whole inherited chronicle, and its node ids seed from
    /// roots that contain that history — novel by construction, no new
    /// crystallizer code: the deposit sheet and the seeded synthesis
    /// already read this way. Sovereign convenience.
    pub fn succeed(&mut self, successor: &str) -> Result<(), ActionRefused> {
        self.succeed_as(SOVEREIGN, successor)
    }
    /// The contained form. Build scope: succession re-founds the line
    /// (the founding-class act).
    pub fn succeed_as(&mut self, actor: u64, successor: &str) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        let famished = self.orbs.iter().filter(|held| !held.famine.is_empty()).count();
        if famished < self.orbs.len() {
            return Err(ActionRefused::LineStillLiving {
                orbs: self.orbs.len(),
                famished,
            });
        }
        self.techs.retain(|(_, condition, _, _)| *condition == 0);
        // The audit law (G1 × NOVEL-1): a line's axes never outlive
        // the line — the deed releases the seal; the successor seals
        // anew (entry in the bands). The old seal stands in the
        // chronicle forever.
        self.genesis = None;
        self.close(
            ClosureEvent::LineSucceeded { successor: successor.to_string() },
            actor,
        );
        Ok(())
    }
    /// PRACTICE-1 (14 §4): the syntropy ratio, 09 §2.5's reading made
    /// an engine read — syntropic books over the whole, summed across
    /// orbs; None when no deed ever posted. Derived at call, never
    /// stored.
    pub fn syntropy_ratio(&self) -> Option<Ratio> {
        let zero = ratio(0, 1);
        let mut syntropic = zero.clone();
        let mut entropic = zero.clone();
        for orb in &self.orbs {
            syntropic += orb.given_in_crisis.clone()
                + orb.returned_across.clone()
                + orb.forwards_fulfilled.clone()
                + orb.bond_flow.clone();
            entropic += orb.extracted_across.clone()
                + orb.forwards_breached.clone()
                + orb.severance_load.clone();
        }
        let whole = syntropic.clone() + entropic;
        if whole == zero {
            return None;
        }
        Some(syntropic / whole)
    }
    pub(super) fn close(&mut self, event: ClosureEvent, actor: u64) {
        let stamp = self.clock.now();
        self.chronicle.append(event, stamp, vec![actor]);
    }

    /// **The bounce arrives** (the cores design, slice 2): `core`'s yearly
    /// pulse of a holding (`None` = STAMP) credited to this world's first
    /// orb, receipted `CoreReturned`. The universe is the actor (the
    /// sovereign seat), as it is for every world-driven consequence.
    pub fn receive_from_core(
        &mut self,
        core: &str,
        resource: Option<crate::dop::Resource>,
        amount: crate::dop::Positive,
    ) -> Result<(), super::ActionRefused> {
        let orb_id = super::FIRST_ORB;
        let held = self
            .orbs
            .iter_mut()
            .find(|orb| orb.orb_id == orb_id)
            .ok_or(super::ActionRefused::NoSuchOrb { orb: orb_id })?;
        let amount = amount.value().clone();
        match resource {
            Some(resource) => held.bank(resource, amount.clone()),
            None => held.bank_stamp(amount.clone()),
        }
        self.close(
            ClosureEvent::CoreReturned { orb_id, core: core.to_string(), resource, amount },
            super::SOVEREIGN,
        );
        Ok(())
    }
}
