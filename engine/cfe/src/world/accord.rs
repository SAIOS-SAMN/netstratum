//! ACCORDS — bonds, forwards, councils, severance.
//!
//! The syntropic books (given_in_crisis, returned_across,
//! forwards_fulfilled) and the entropic ones (extracted_across,
//! forwards_breached, severance_load) are written here.

use super::*;

impl World {
    /// COUNCIL-1 (12_UNION §7) — seal a COUNCIL: a multilateral
    /// forward among three or more distinct orbs, every member bound
    /// alike. Each locks the SAME collateral (a bank withdrawal, the
    /// vitrified-vault law) against delivering ≥ quota to fellow
    /// members within the window; the reckoning judges all together at
    /// the boundary — the whole of the group over the needs of the
    /// individual. Any member unable to lock refuses the WHOLE seal:
    /// a council that starts unequal never starts.
    pub fn seal_council(
        &mut self,
        council: u64,
        members: Vec<u64>,
        resource: Resource,
        quota: Ratio,
        window: u64,
        collateral: Ratio,
    ) -> Result<(), ActionRefused> {
        let mut distinct = members.clone();
        distinct.sort_unstable();
        distinct.dedup();
        if distinct.len() < 3
            || distinct.len() != members.len()
            || quota <= ratio(0, 1)
            || collateral <= ratio(0, 1)
            || window == 0
            || self.councils.iter().any(|held| held.council == council)
        {
            return Err(ActionRefused::BadCouncil);
        }
        for member in &members {
            if !self.orbs.iter().any(|held| held.orb_id == *member) {
                return Err(ActionRefused::NoSuchOrb { orb: *member });
            }
        }
        // Every member must afford its lock BEFORE any lock happens —
        // a refusal moves nothing (the aperture discipline).
        for member in &members {
            let held = peek(&self.orbs, *member)
                .expect("checked");
            let have = held.banked_amount(resource);
            if have < collateral {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(collateral.clone()),
                    have: Box::new(have),
                });
            }
        }
        for member in &members {
            let held = peel(&mut self.orbs, *member)
                .expect("checked");
            held.bank(resource, -collateral.clone());
        }
        let stamp = self.clock.now();
        self.councils.push(Council {
            council,
            members: members.clone(),
            resource,
            quota: quota.clone(),
            delivered: vec![ratio(0, 1); members.len()],
            collateral: collateral.clone(),
            sealed: stamp,
            window,
        });
        self.close(
            ClosureEvent::CouncilSealed { council, resource, quota, window, collateral, members },
            SOVEREIGN,
        );
        Ok(())
    }
    /// COUNCIL-1: the open councils.
    pub fn councils(&self) -> &[Council] {
        &self.councils
    }
    /// COUNCIL-1 (12_UNION §7) — the DIPLOMACY-FAILED reading: how
    /// many council reckonings this orb has breached, straight from
    /// the chronicle. The slash is the receipt that unbinds; until it
    /// exists, the council holds.
    pub fn council_breaches(&self, orb: u64) -> usize {
        self.chronicle
            .receipts()
            .iter()
            .filter(|receipt| {
                matches!(
                    &receipt.event,
                    ClosureEvent::CouncilClosed { member, fulfilled: false, .. }
                        if *member == orb
                )
            })
            .count()
    }
    /// BOND-1: the standing orders.
    pub fn bonds(&self) -> &[Bond] {
        &self.bonds
    }
    /// Seal a bond as the sovereign.
    pub fn seal_bond(
        &mut self,
        source: u64,
        target: u64,
        resource: Resource,
        rate: Ratio,
        term: u64,
    ) -> Result<(), ActionRefused> {
        self.seal_bond_as(SOVEREIGN, source, target, resource, rate, term)
    }
    /// BOND-1 (09 §2.2) — seal a standing order: source streams `rate`
    /// of `resource` per tick to target, non-transactional, settled by
    /// ONE receipt at each touch. One bond per (source, target,
    /// resource); `term` 0 is open-ended, otherwise the bond expires
    /// ORDERLY at sealed+term posting nothing. Collect scope.
    pub fn seal_bond_as(
        &mut self,
        actor: u64,
        source: u64,
        target: u64,
        resource: Resource,
        rate: Ratio,
        term: u64,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if source == target || rate <= ratio(0, 1) {
            return Err(ActionRefused::BadTransfer);
        }
        if !self.orbs.iter().any(|held| held.orb_id == source) {
            return Err(ActionRefused::NoSuchOrb { orb: source });
        }
        if !self.orbs.iter().any(|held| held.orb_id == target) {
            return Err(ActionRefused::NoSuchOrb { orb: target });
        }
        if self.bonds.iter().any(|bond| {
            bond.source == source && bond.target == target && bond.resource == resource
        }) {
            return Err(ActionRefused::BondAlreadySealed);
        }
        let stamp = self.clock.now();
        self.bonds.push(Bond {
            source,
            target,
            resource,
            rate: rate.clone(),
            term,
            sealed: stamp,
            anchored: stamp,
        });
        self.chronicle.append(
            ClosureEvent::BondSealed {
                source_orb: source,
                target_orb: target,
                resource,
                rate,
                term,
            },
            stamp,
            vec![actor],
        );
        Ok(())
    }
    /// Sever a bond as the sovereign.
    pub fn sever_bond(
        &mut self,
        source: u64,
        target: u64,
        resource: Resource,
    ) -> Result<(), ActionRefused> {
        self.sever_bond_as(SOVEREIGN, source, target, resource)
    }
    /// BOND-1 — cut a standing bond (v1 ruling: the SOURCE side cuts;
    /// the load posts to it — the giver stopped giving without the
    /// notice). The final span settles in the same receipt; the
    /// severance load is `rate × bond_notice_window()`. Collect scope.
    pub fn sever_bond_as(
        &mut self,
        actor: u64,
        source: u64,
        target: u64,
        resource: Resource,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        let Some(bond) = self.bonds.iter().find(|bond| {
            bond.source == source && bond.target == target && bond.resource == resource
        }) else {
            return Err(ActionRefused::NoSuchBond);
        };
        let load = bond.rate.clone() * ratio(crate::dop::bond_notice_window() as i64, 1);
        let stamp = self.clock.now();
        self.settle_bond(source, target, resource, stamp, vec![actor], Some(load));
        Ok(())
    }
    /// BOND-1 — settle one bond's accumulated span up to `stamp`
    /// (capped at the term boundary) by ONE BondSettled receipt.
    /// `ending: Some(load)` ends the bond deliberately (sever: rate ×
    /// notice; term expiry: zero). Insolvency — the span exceeding the
    /// source's post-decay bank — pays what stands and ends the bond
    /// as a receipted DEFAULT with the notice load. Bond flow posts
    /// the love book only (the crisis books belong to deliberate
    /// deeds); the payment still compensates open reciprocity windows.
    pub(super) fn settle_bond(
        &mut self,
        source: u64,
        target: u64,
        resource: Resource,
        stamp: u64,
        provenance: Vec<u64>,
        ending: Option<Ratio>,
    ) {
        let Some(at) = self.bonds.iter().position(|bond| {
            bond.source == source && bond.target == target && bond.resource == resource
        }) else {
            return;
        };
        let (rate, term, sealed, anchored) = {
            let bond = &self.bonds[at];
            (bond.rate.clone(), bond.term, bond.sealed, bond.anchored)
        };
        let boundary = match term {
            0 => stamp,
            span => stamp.min(sealed + span),
        };
        let owed = rate.clone() * ratio(boundary.saturating_sub(anchored) as i64, 1);
        let star = self.star;
        let affordable = {
            let held = peek(&self.orbs, source)
                .expect("bond endpoints exist");
            // PRACTICE-1 the Hoard: the stores keep — decay halves.
            let hoard_keep = if self.practices.contains(&4) { ratio(1, 2) } else { ratio(1, 1) };
            let decay = hoard_keep * demurrage_due(held, star, stamp, &ease_of(&self.techs, source, stamp), &standing_supply(&self.orbs))
                .into_iter()
                .find(|(due, _)| *due == resource)
                .map(|(_, decay)| decay)
                .unwrap_or_else(|| ratio(0, 1));
            held.banked_amount(resource) - decay
        };
        let pay = owed.clone().min(affordable);
        let insolvent = pay < owed;
        let ended = ending.is_some() || insolvent;
        // The caller's ruling wins (expiry: zero — the term was
        // served); an undirected insolvency is the mid-winter default.
        let load = match (&ending, insolvent) {
            (Some(load), _) => load.clone(),
            (None, true) => rate * ratio(crate::dop::bond_notice_window() as i64, 1),
            (None, false) => ratio(0, 1),
        };
        if pay == ratio(0, 1) && !ended {
            return; // nothing streamed, nothing ends — no receipt
        }
        let value = pay.clone() * crate::dop::commensurate_weight(resource);
        // P2: snapshotted before either endpoint is peeled.
        let supply = standing_supply(&self.orbs);
        if pay > ratio(0, 1) {
            for orb in [source, target] {
                let held = peel(&mut self.orbs, orb)
                    .expect("bond endpoints exist");
                let ease = ease_of(&self.techs, orb, stamp);
                touch(held, star, &mut self.chronicle, stamp, &ease, &supply);
            }
            {
                let held = peel(&mut self.orbs, source)
                    .expect("bond endpoints exist");
                held.bank(resource, -pay.clone());
                held.bond_flow += value.clone();
            }
            {
                let held = peel(&mut self.orbs, target)
                    .expect("bond endpoints exist");
                held.bank(resource, pay.clone());
            }
            settle_debts(&mut self.debts, &mut self.orbs, source, target, value);
            // FORWARD-1: the stream credits the open pact on this lane.
            credit_forwards(&mut self.forwards, source, target, resource, &pay);
            credit_councils(&mut self.councils, source, target, resource, &pay);
        }
        if ended {
            self.bonds.remove(at);
            if load > ratio(0, 1) {
                let held = peel(&mut self.orbs, source)
                    .expect("bond endpoints exist");
                held.severance_load += load.clone();
            }
        } else {
            self.bonds[at].anchored = boundary;
        }
        self.chronicle.append(
            ClosureEvent::BondSettled {
                source_orb: source,
                target_orb: target,
                resource,
                amount: pay.clone(),
                severed: ended,
                load,
            },
            stamp,
            provenance,
        );
        if pay > ratio(0, 1) {
            self.relieve_recovered(target);
        }
    }
    /// Seal a forward as the sovereign.
    pub fn seal_forward(
        &mut self,
        obligor: u64,
        beneficiary: u64,
        resource: Resource,
        quota: Ratio,
        window: u64,
    ) -> Result<(), ActionRefused> {
        self.seal_forward_as(SOVEREIGN, obligor, beneficiary, resource, quota, window)
    }
    /// FORWARD-1 (01 §7, 09 §2.3) — seal a vitrified delivery pact:
    /// the obligor locks collateral (the fidelity curve prices it —
    /// `dop::collateral_required`, the guardrail-6 ceiling holding)
    /// against delivering ≥ quota within the window. The lock is a
    /// bank withdrawal: every spend gate sees its absence, and the
    /// vitrified matrix vaults it losslessly by construction. Once
    /// sealed, the pact runs to release or slash — no cancel. One
    /// open pact per (obligor, beneficiary, resource). Collect scope.
    pub fn seal_forward_as(
        &mut self,
        actor: u64,
        obligor: u64,
        beneficiary: u64,
        resource: Resource,
        quota: Ratio,
        window: u64,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        // **THE COVENANT IS A T3 INSTRUMENT** (operator ruling
        // 2026-08-05: *drop covenant to T3 and below*).
        //
        // `tier_of(None) = T3` is the only road to the common band
        // (`12_UNION` §2a), so the gate reads the world's OWN line and
        // refuses any that holds a seat. A seated office has chartered
        // authority and does not need to post collateral against a
        // deadline to be believed; the common has nothing else.
        //
        // An unsealed world holds no line and therefore no seat, so it
        // is common by construction — the same reading `tier_of` gives.
        if let Some(line) = self.genesis.as_ref().map(|held| held.civilization.clone()) {
            if let Some(station) = self.station_of(&line) {
                return Err(ActionRefused::CovenantAboveTheCommon { line, station });
            }
        }
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if obligor == beneficiary || quota <= ratio(0, 1) || window == 0 {
            return Err(ActionRefused::BadTransfer);
        }
        if !self.orbs.iter().any(|held| held.orb_id == obligor) {
            return Err(ActionRefused::NoSuchOrb { orb: obligor });
        }
        if !self.orbs.iter().any(|held| held.orb_id == beneficiary) {
            return Err(ActionRefused::NoSuchOrb { orb: beneficiary });
        }
        if self.forwards.iter().any(|held| {
            held.obligor == obligor
                && held.beneficiary == beneficiary
                && held.resource == resource
        }) {
            return Err(ActionRefused::ForwardAlreadySealed);
        }
        let stamp = self.clock.now();
        let collateral = {
            let held = peek(&self.orbs, obligor)
                .expect("checked above");
            let required = crate::dop::collateral_required(
                &held.forwards_fulfilled,
                &held.forwards_breached,
                &quota,
            );
            let have = held.banked_amount(resource);
            if have < required {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(required),
                    have: Box::new(have),
                });
            }
            required
        };
        {
            let held = peel(&mut self.orbs, obligor)
                .expect("checked above");
            held.bank(resource, -collateral.clone());
        }
        self.forwards.push(Forward {
            obligor,
            beneficiary,
            resource,
            quota: quota.clone(),
            delivered: ratio(0, 1),
            collateral: collateral.clone(),
            sealed: stamp,
            window,
        });
        self.chronicle.append(
            ClosureEvent::ForwardSealed {
                obligor_orb: obligor,
                beneficiary_orb: beneficiary,
                resource,
                quota,
                window,
                collateral,
            },
            stamp,
            vec![actor],
        );
        Ok(())
    }
    /// FORWARD-1: the open vitrified pacts.
    pub fn forwards(&self) -> &[Forward] {
        &self.forwards
    }
}
