//! CORE — accessors and small deeds that name no larger concern.
//!
//! Anything here that grows a concern of its own should leave.

use super::*;

impl World {
    /// TECH-1a: has this world's people made the SOLAR LEAP — do they
    /// know the powers of their sun? Pre-solar is a lawful state
    /// (12_UNION §5a); the reading gates licensing and the shield,
    /// never the world's own life.
    pub fn knows_its_sun(&self) -> bool {
        self.techs.iter().any(|(_, condition, _, _)| *condition == 0)
    }
    /// TERRA-1: is the BASE-7 stack complete — the solar leap plus
    /// all five crystallized conditions? Six earned nodes; the gifted
    /// seventh (terraforming) completes base-7. Derived, never stored.
    pub fn base_seven_complete(&self) -> bool {
        (0u8..=5).all(|condition| {
            self.techs.iter().any(|(_, held, _, _)| *held == condition)
        })
    }
    /// **THE FULL BASE-11 STACK** (operator ruling 2026-08-02) — the
    /// base tier complete AND every chartered work claimed on evidence.
    ///
    /// [`Self::base_seven_complete`] is deliberately left exactly as it
    /// landed: the crown's gate does not move, and every law that binds
    /// to it reads the same thing it always did.
    pub fn base_eleven_complete(&self, orb: u64) -> bool {
        self.base_seven_complete()
            && (crate::world::survey::first_work()..=crate::world::survey::last_condition())
                .all(|condition| self.work_claimed(orb, condition))
    }

    /// WANT-1: the struck cognitive wants — (orb, kind).
    pub fn wants(&self) -> &[(u64, u8)] {
        &self.wants
    }
    /// COUNCIL-1 (12_UNION §7) — the WARRING IMPERIUM reading, derived
    /// from the landed books and never stored: an orb whose extraction
    /// runs past double its reciprocity while its breaches outweigh
    /// its fulfilments has stepped outside the council's protection by
    /// its own receipted conduct. Thresholds are placeholder
    /// parameters (the 06 discipline).
    pub fn is_imperial(&self, orb: u64) -> bool {
        let Some(held) = peek(&self.orbs, orb) else {
            return false;
        };
        held.extracted_across > ratio(0, 1)
            && held.extracted_across > ratio(2, 1) * held.returned_across.clone()
            && held.forwards_breached > held.forwards_fulfilled
    }
    /// AEON-1a (11 §5): the FOLD — seal this world's part in the
    /// closing era and seat it on the new aeon's G-sheet (`base` is
    /// the advanced base gauge). Archival by construction: the receipt
    /// is the seal; rotation of the var root is the conductor's deed.
    pub fn seal_fold(&mut self, era: u64, base: Ratio) {
        let gauge = wrap_gauge(base);
        self.gauge[0] = gauge.clone();
        self.close(ClosureEvent::EraFolded { era, gauge }, SOVEREIGN);
    }
    pub fn now(&self) -> u64 {
        self.clock.now()
    }
    pub fn chronicle(&self) -> &Chronicle {
        &self.chronicle
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
    /// **THIS WORLD'S OWN NAME** — the genesis colony's, which is what
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
        // #41: the refund halves what the build SPENT, which for a
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
    /// Collect ONE orb — settle its workspace against its bank;
    /// Collect scope; relief re-checked for that orb.
    /// THE DISCOVERY MINT (U1, tag 86) — value created against VERIFIED
    /// WORK, banked on the solving orb.
    ///
    /// This is the emitter the PoUW loop was missing. Before it, a
    /// solved task wrote a file and incremented a counter, and the
    /// economy never heard of it: cognition proved out and minted
    /// nothing.
    ///
    /// The world PROPOSES; it does not gate. `work` rides WHOLE as the
    /// opaque payload (THE OPAQUE-PAYLOAD LAW — cfe never parses it),
    /// and the kernel re-runs `verify_work` at the door plus the ruled
    /// bound `f(difficulty)`. A caller computes its amount from
    /// `kernel::mint_bound` so proposer and gate read ONE function.
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
        // K4 THE LIVE GATE. The emitter cannot VERIFY — the
        // opaque-payload law bars `cfe` from importing the verifier, and
        // `recurrence_guard` fails the build on any attempt. So it does
        // the lawful thing instead: it CALLS BACK to whoever can.
        //
        // `cfe` never parses the payload; it hands the bytes to the
        // warrant and asks what they earn. A caller with no verifier
        // cannot mint at all, and a caller with one cannot mint past
        // what the work bounds — checked BEFORE `close`, so a refused
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
        // THE ESCROW. The deed is built, the state it WOULD produce is
        // built, and the gate is asked — all before anything is
        // closed. `earns` above priced the work; this asks whether the
        // TRANSITION stands, which is where D4 and conservation live.
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
        // THE CREDIT LANDS ON THE LIVE ORB (found 2026-08-06 by the
        // seam's digest perturbation): the escrow refactor banked the
        // mint on the CANDIDATE — a throwaway built for the gate — and
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
            stations: self.stations.clone(),
            hulls: self.hulls.clone(),
        }
    }
    /// ELEMENT-GRAMMAR-1 (14 §3) — the extraction closure: richness ×
    /// elapsed banked in ONE receipt (the bond-settle shape). A vein
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
        // ECO-1 (#40): the loop's two inputs, read before the vein is
        // borrowed — the star (proven Some only once a vein is found:
        // prospecting requires it, and nothing uncommissions a sun)
        // and the regenerators standing on this orb (prior state; the
        // kernel re-derives both from the same reads).
        let star = self.star;
        let regenerators = self
            .orb_by_id(orb)
            .map(|held| {
                held.zones
                    .iter()
                    .filter(|zone| zone.kind == crate::dop::ZoneKind::Regenerator)
                    .count() as u64
            })
            .unwrap_or(0);
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
        let elapsed = ratio(span as i64, 1);
        let amount = vein.richness.clone() * elapsed;
        // ECO-1: wear and recovery settle in the SAME receipt — the
        // one-home law prices both (the kernel calls the same fn).
        let class = star.expect("a vein stands, so a star was commissioned");
        let (_, _, natural) = crate::dop::element_properties(&element, class);
        let after =
            crate::dop::vein_wear(&vein.richness, &natural, &amount, regenerators, span);
        vein.richness = after.clone();
        vein.last_assay = now;
        let held = peel(&mut self.orbs, orb).expect("judged above");
        held.bank_element(element, amount.clone());
        self.close(
            ClosureEvent::AssayStruck {
                orb_id: orb,
                element,
                amount: amount.clone(),
                richness: Some(after),
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
        for receipt in self.chronicle.receipts() {
            match &receipt.event {
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
    /// this DERELICT world's chronicle as its own substrate (07 §3).
    /// The refusal is computed pure (a living line refuses, nothing
    /// moves); on the deed the living tree drops to the world's kernel
    /// (condition 0 — the sun is known to the WORLD, not to the line)
    /// and every other node leaves the tree while standing in the
    /// chronicle forever. The successor's conditions then evaluate
    /// over the WHOLE inherited chronicle, and its node ids seed from
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
        // ANEW (entry in the bands). The old seal stands in the
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
}
