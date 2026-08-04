//! ECONOMY DEEDS — collect, transfer, absorb, offers, the purse.
//!
//! Every deed here touches banked value, so every one of them settles
//! demurrage at the touch through `super::rules`. That is the organ's
//! single sealed-rule surface and it is deliberate.

use super::*;

impl World {
    /// MARKET-1: the resting offers.
    pub fn offers(&self) -> &[Offer] {
        &self.offers
    }
    /// MARKET-1 (BUILD_ORDER §3a): POST an offer — a receipted intent
    /// to give one resource for another at an exact-rational price
    /// (want/give — no absolute price exists, guardrail 6). No escrow:
    /// the fill judges affordability when a crossing clears; an offer
    /// that outlived its bank refuses THEN, harming no one now. An
    /// unmatched offer rests.
    pub fn post_offer(
        &mut self,
        orb: u64,
        give: Resource,
        give_amount: Ratio,
        want: Resource,
        want_amount: Ratio,
        offer: u64,
    ) -> Result<(), ActionRefused> {
        if give == want
            || give_amount <= ratio(0, 1)
            || want_amount <= ratio(0, 1)
            || self.offers.iter().any(|held| held.offer == offer)
        {
            return Err(ActionRefused::BadOffer);
        }
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        let posted = self.clock.now();
        self.offers.push(Offer {
            offer,
            orb,
            give,
            give_amount: give_amount.clone(),
            want,
            want_amount: want_amount.clone(),
            posted,
        });
        self.close(
            ClosureEvent::OfferPosted { offer, orb_id: orb, give, give_amount, want, want_amount },
            SOVEREIGN,
        );
        Ok(())
    }
    /// Collect the genesis orb as the sovereign.
    pub fn collect(&mut self) -> Result<usize, ActionRefused> {
        self.collect_as(SOVEREIGN, FIRST_ORB)
    }
    pub fn collect_as(&mut self, actor: u64, orb: u64) -> Result<usize, ActionRefused> {
        // P2: the standing supply the sink's EPOCH 1 rate reads,
        // snapshotted BEFORE any orb is peeled — an owned Vec, so no
        // borrow of `self.orbs` is held across the mutable touch below.
        let supply = standing_supply(&self.orbs);
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        // MASK-1a: the veil's standing toll settles at the touch — an
        // unpayable toll LIFTS the veil by receipt, never silently.
        if self.veiled {
            let toll = crate::dop::veil_toll();
            let held_joules = self
                .orbs
                .iter()
                .find(|held| held.orb_id == orb)
                .map(|held| held.banked_amount(Resource::Joule))
                .unwrap_or_else(|| ratio(0, 1));
            if held_joules >= toll {
                if let Some(held) = peel(&mut self.orbs, orb) {
                    held.bank(Resource::Joule, -toll.clone());
                }
                let stamp = self.clock.now();
                self.chronicle.append(
                    ClosureEvent::UpkeepSettled { orb_id: orb, resource: Resource::Joule, amount: toll },
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
        if settleable.is_empty() {
            return Err(ActionRefused::NothingAccrued);
        }
        let orb_id = held.orb_id;
        let stamp = self.clock.now();
        let count = settleable.len();
        // SINK-1 (Q2): receipted demurrage at touch — un-vaulted banked
        // joules/sink decay for the elapsed span, settled openly.
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
            // ENCASEMENT FIRST, now obeyed here too (`EQUATIONS` T3,
            // 05 §1.2; operator ruling 2026-08-03: friction entropy is
            // thermodynamically identical to thermal entropy — the same
            // physical currency, so the same capture law).
            //
            // This block used to vent 100% straight to the void while
            // writing into the very ledger whose own law puts capture
            // FIRST. Heat is encased and only the overflow disperses;
            // friction declined that arm rather than lacking one.
            //
            // NO INVENTED CONSTANT: the shells, their throughput and
            // the TECH-1b discipline boon are read exactly as the
            // per-tick heat path reads them.
            let shells = held
                .zones
                .iter()
                .filter(|zone| held.zone_active(zone) && zone.kind == ZoneKind::Encasement)
                .count() as i64;
            let mut throughput = crate::dop::encasement_throughput() * ratio(shells, 1);
            if self
                .techs
                .iter()
                .any(|(owner, condition, _)| *owner == orb_id && *condition == 2)
            {
                throughput *= ratio(5, 4);
            }
            let captured = friction.clone().min(throughput);
            let overflow = friction - captured.clone();

            // THE SYNTROPIC ARM, FIRST. Captured friction is encased
            // exactly as captured heat is — it banks joules and rides
            // the existing `YieldCollected` receipt, which is what feeds
            // `encased_entries` and `heat_encased_of`. No new deed.
            if captured > ratio(0, 1) {
                held.bank(Resource::Joule, captured.clone());
                held.syntropy_contained += captured.clone();
                self.chronicle.append(
                    ClosureEvent::YieldCollected {
                        orb_id,
                        resource: Resource::Joule,
                        amount: captured,
                    },
                    stamp,
                    vec![actor],
                );
            }
            // THE ENTROPIC ARM: only the overflow vents.
            //
            // `FrictionSettled` now carries THE VENTED PART, not the
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
                    held.bank(resource, amount.clone());
                    // THE SYNTROPIC BOOK — the inverse of
                    // entropy_vented. A joule yield IS an encased joule
                    // (the crystal sheet already reads it that way), so
                    // containing one books it. Mirrored verbatim in
                    // `replay`.
                    if resource == Resource::Joule {
                        held.syntropy_contained += amount.clone();
                    }
                    self.chronicle.append(
                        ClosureEvent::YieldCollected { orb_id, resource, amount },
                        stamp,
                        vec![actor],
                    );
                }
                false => {
                    let drawn = -amount;
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
        self.relieve_recovered(orb);
        Ok(count)
    }
    /// Transfer banked value between two orbs as the sovereign.
    pub fn transfer(
        &mut self,
        from: u64,
        to: u64,
        resource: Resource,
        amount: Ratio,
    ) -> Result<(), ActionRefused> {
        self.transfer_as(SOVEREIGN, from, to, resource, amount)
    }
    /// TRADE-1 (09 §2.1) — move BANKED value orb-to-orb within the
    /// world: the chartering conservation law generalized (nothing
    /// minted, nothing lost). Both sides settle due demurrage at the
    /// touch (the SINK-1 law) before value moves; affordability is
    /// judged on the post-decay bank; the compassion books post iff
    /// the counterparty stands in receipted, uncleared crisis at this
    /// stamp — a replay fact, never a judgment call. Collect scope;
    /// relief for the receiver re-checked at the landed law (02 §2.4).
    pub fn transfer_as(
        &mut self,
        actor: u64,
        from: u64,
        to: u64,
        resource: Resource,
        amount: Ratio,
    ) -> Result<(), ActionRefused> {
        // P2: the standing supply the sink's EPOCH 1 rate reads,
        // snapshotted BEFORE any orb is peeled — an owned Vec, so no
        // borrow of `self.orbs` is held across the mutable touch below.
        let supply = standing_supply(&self.orbs);
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if from == to || amount <= ratio(0, 1) {
            return Err(ActionRefused::BadTransfer);
        }
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
            // PRACTICE-1 the Hoard: the stores keep — decay halves.
            let hoard_keep = if self.practices.contains(&4) { ratio(1, 2) } else { ratio(1, 1) };
            let decay = hoard_keep * demurrage_due(source, star, stamp, &ease_of(&self.techs, from, stamp), &super::rules::standing_supply(&self.orbs))
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
        // Crisis reads BEFORE the move — the replay-fact order.
        let from_crisis = {
            let held = peek(&self.orbs, from).expect("checked");
            in_crisis(held)
        };
        let to_crisis = {
            let held = peek(&self.orbs, to).expect("checked");
            in_crisis(held)
        };
        // IDEO-1 (09 §3.2): friction prices DIVERGENCE, not vice — the
        // handling overhead of unlike measured boundary policies, read
        // BEFORE the move, accrued as workspace against the mover and
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
        let friction = crate::dop::friction_rate() * divergence * value;
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
        source: u64,
        absorber: u64,
        amount: Ratio,
    ) -> Result<(), ActionRefused> {
        self.absorb_as(SOVEREIGN, source, absorber, amount)
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
    /// touch; affordability judged on the post-decay sink, computed
    /// pure — a refusal orphans nothing. Collect scope.
    pub fn absorb_as(
        &mut self,
        actor: u64,
        source: u64,
        absorber: u64,
        amount: Ratio,
    ) -> Result<(), ActionRefused> {
        // P2: the standing supply the sink's EPOCH 1 rate reads,
        // snapshotted BEFORE any orb is peeled — an owned Vec, so no
        // borrow of `self.orbs` is held across the mutable touch below.
        let supply = standing_supply(&self.orbs);
        self.permitted(actor, Scope::Collect)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if source == absorber || amount <= ratio(0, 1) {
            return Err(ActionRefused::BadTransfer);
        }
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
            // PRACTICE-1 the Hoard: the stores keep — decay halves.
            let hoard_keep = if self.practices.contains(&4) { ratio(1, 2) } else { ratio(1, 1) };
            let decay = hoard_keep * demurrage_due(held, star, stamp, &ease_of(&self.techs, absorber, stamp), &super::rules::standing_supply(&self.orbs))
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
        // Crisis reads BEFORE the move — the replay-fact order.
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
    /// 20_FOUNDING §5 — THE PURSE: a line's economy read whole across
    /// every orb it holds.
    ///
    /// **A PROJECTION, NEVER A SETTLING TOUCH.** What each orb OWES in
    /// demurrage is deducted from the reading and NOTHING IS WRITTEN.
    /// Looking at a purse is not a chain event, so a status render
    /// costs no receipts; the due settles for real at the next genuine
    /// touch (the SINK-1 law, unchanged). Value is still not hidden
    /// from decay — the due is already subtracted in what you see.
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

    /// **THE VALUE-LOSS BOOK — derived, never stored** (SYN-1/SYN-2,
    /// operator ruling 2026-08-02).
    ///
    /// What an orb has LOST to holding costs: demurrage decay plus
    /// upkeep draws, summed from the chronicle in joule-commensurable
    /// units (09 §1). Before this, value left the universe with no
    /// record it had ever left — the thermal arm closes its books
    /// (`encased + vented = total excess`) and the value arm closed
    /// nothing.
    ///
    /// **IT IS NOT `entropy_vented`, and must never be.** That field is
    /// the SHED-HEAT figure and 05 §1.4's absorption invariant binds it
    /// exactly: `sink_consumed = heat_absorbed = vented_after_shed`.
    /// Value decay is a monetary velocity cost, not a thermal loss;
    /// summing them would corrupt a landed physical measurement, which
    /// `world_absorb::the_absorption_invariant_measures_across_two_orbs`
    /// proves by failing the moment they are mixed. The two ledgers
    /// stay decoupled: physics on one side, economics on the other.
    ///
    /// **NO SCHEMA IS SPENT.** `DemurrageSettled { orb_id, .. }` and
    /// `UpkeepSettled { orb_id, .. }` already carry their orb, so the
    /// whole book is arithmetic over deeds already receipted — the
    /// derived-never-stored law (09 §1.2), and the same shape the
    /// syntropy ratio takes.
    pub fn value_lost_of(&self, orb: u64) -> Ratio {
        self.chronicle
            .receipts()
            .iter()
            .fold(ratio(0, 1), |carried, receipt| match &receipt.event {
                ClosureEvent::DemurrageSettled { orb_id, amount, .. }
                | ClosureEvent::UpkeepSettled { orb_id, amount, .. } if *orb_id == orb => {
                    carried + amount.clone()
                }
                _ => carried,
            })
    }

    /// The same book, per feeder, carried WHOLE rather than summed into
    /// one figure — which of the two costs took it, and how much. A
    /// caller that needs the split reads this; a caller that needs the
    /// total reads [`World::value_lost_of`].
    pub fn value_lost_split_of(&self, orb: u64) -> (Ratio, Ratio) {
        self.chronicle.receipts().iter().fold(
            (ratio(0, 1), ratio(0, 1)),
            |(decay, upkeep), receipt| match &receipt.event {
                ClosureEvent::DemurrageSettled { orb_id, amount, .. } if *orb_id == orb => {
                    (decay + amount.clone(), upkeep)
                }
                ClosureEvent::UpkeepSettled { orb_id, amount, .. } if *orb_id == orb => {
                    (decay, upkeep + amount.clone())
                }
                _ => (decay, upkeep),
            },
        )
    }

    /// **THE UNION REVIEW — derived, never stored** (D5, operator
    /// ruling 2026-08-02). When the sovereign union was struck, and
    /// whether its review has fallen due at `stamp`.
    ///
    /// `Some((sealed_at, due))` if the union stands; `None` if it was
    /// never struck. **`due` is a REVIEW falling due, NEVER a lapse.**
    /// Nothing settles, nothing expires, and no capacity is dropped
    /// when it turns true: a `BOND-1` term ENDS its bond at
    /// `sealed + term`, and if the union did that, D5's breach clause
    /// would fire at stamp 1500 — *D stops and V suffocates; V stops
    /// and D freezes* — and the foundation would lapse on a timer. The
    /// union stands past the horizon; what falls due is the operator's
    /// re-approval.
    ///
    /// Read from the chronicle on demand, so there is no counter to
    /// drift and no field to migrate (`09_LEDGER` §1.2). This is the
    /// consumer that keeps the deed from being a term that only
    /// resonates (`18` Article III LAW 8).
    pub fn union_review(&self, stamp: u64) -> Option<(u64, bool)> {
        self.chronicle
            .receipts()
            .iter()
            .find(|receipt| {
                matches!(receipt.event, ClosureEvent::SovereignUnionSealed { .. })
            })
            .map(|receipt| {
                let horizon = match &receipt.event {
                    ClosureEvent::SovereignUnionSealed { term_stamps, .. } => *term_stamps,
                    _ => crate::dop::UNION_REVIEW_HORIZON,
                };
                (receipt.stamp, stamp >= receipt.stamp.saturating_add(horizon))
            })
    }


    /// **THE HEAT ENCASED — the syntropic arm of the thermal book,
    /// derived, never stored** (the missing-leg sweep, 2026-08-02).
    ///
    /// What an orb has CAPTURED over its whole life: every joule yield
    /// it collected, which is the encasement arm by this engine's own
    /// reading — `YieldCollected { resource: Joule }` is exactly what
    /// feeds `encased_entries` on the crystal sheet.
    ///
    /// **WHY IT WAS OWED.** The thermal dual was two-armed in one place
    /// and one-armed in another. On the SHEET both arms stand and are
    /// symmetric — `vented_entries` from `VentSettled`, `encased_entries`
    /// from the joule yield — but both are WINDOWED and pruned at each
    /// boundary, and both are `pub(super)`, so they answer one tech
    /// condition and nothing else. On the ORB, which is committed state,
    /// replayed and public, only `entropy_vented` stands, and it is
    /// CUMULATIVE FOREVER.
    ///
    /// So the permanent public book recorded only LOSS. An orb could
    /// always be asked what it had shed to the void for its whole life,
    /// and could not be asked what it had ever kept — the gain was
    /// forgotten as soon as the window moved past it. That is the
    /// negentropic shape `EQUATIONS` §0.3 chapter five names: *a map
    /// that inherits one leg of a dual has not simplified the law, it
    /// has reversed it.*
    ///
    /// **NO SCHEMA IS SPENT.** `YieldCollected { orb_id, resource,
    /// amount }` already carries its orb, so the whole arm is
    /// arithmetic over deeds already receipted — the derived-never-stored
    /// law (`09_LEDGER` §1.2), and the same shape [`Self::value_lost_of`]
    /// took. `entropy_vented` is NOT touched: it is a landed field bound
    /// by 05 §1.4's absorption invariant, and the cure for a missing arm
    /// is to add the arm, never to disturb the one that stands.
    pub fn heat_encased_of(&self, orb: u64) -> Ratio {
        self.chronicle
            .receipts()
            .iter()
            .filter_map(|receipt| match &receipt.event {
                ClosureEvent::YieldCollected { orb_id, resource: Resource::Joule, amount }
                    if *orb_id == orb =>
                {
                    Some(amount.clone())
                }
                _ => None,
            })
            .fold(ratio(0, 1), |a, b| a + b)
    }

    /// **THE THERMAL BOOK, BOTH ARMS AT ONCE** — `(encased, vented)`.
    ///
    /// Returned as a PAIR and never as a difference or a ratio: a net
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


    /// **RECIPROCITY — the orb's own measured conduct, surfaced.**
    ///
    /// `returned_across / (returned_across + extracted_across)`: of the
    /// value this orb drew across a boundary, how much it closed the
    /// loop on. `09` §2.4's measured boundary policy, and the quantity
    /// `FrictionSettled` is priced from.
    ///
    /// **`None` MEANS NO BASIS, and that is the whole point of this
    /// return type.** The private reading friction uses answers `1/1`
    /// when an orb has no cross-orb history at all — a sensible default
    /// for pricing a cost (charge nothing when there is nothing to
    /// compare), and a TRAP for anything that reads alignment as merit:
    /// **two orbs that have never traded read as perfectly aligned.**
    /// Absence would present as agreement, which is the same shape as
    /// the borne share's farming-by-absence. Here the absence is named
    /// instead of being dressed as a perfect score.
    ///
    /// The private `reciprocity_reading` is NOT changed: friction is
    /// consensus law and its `1/1` default is correct for pricing. This
    /// adds a reading; it moves nothing.
    pub fn reciprocity_of(&self, orb: u64) -> Option<Ratio> {
        let held = self.orbs.iter().find(|held| held.orb_id == orb)?;
        let denominator = held.returned_across.clone() + held.extracted_across.clone();
        match denominator > ratio(0, 1) {
            true => Some(held.returned_across.clone() / denominator),
            false => None,
        }
    }

    /// **CONVERGENCE — the syntropic arm of `FrictionSettled`,
    /// derived, never stored.**
    ///
    /// ```text
    ///     convergence(a, b) = 1 − |reciprocity(a) − reciprocity(b)|
    /// ```
    ///
    /// **THE ENGINE ALREADY COMPUTES THIS AND THROWS IT AWAY.** On
    /// every cross-orb transfer it evaluates
    /// `divergence = |recip(from) − recip(to)|`, prices friction from
    /// it, and then:
    ///
    /// ```text
    ///     if friction > 0 { ...receipt... }
    /// ```
    ///
    /// When conduct MATCHES, divergence is zero, friction is zero, and
    /// nothing is receipted. The syntropic half is calculated and
    /// destroyed in the same expression, every time, and no caller
    /// could even ask for it — `reciprocity_reading` was private. That
    /// is the one-legged dual at its sharpest: not an arm that was
    /// never built, but one built and discarded on each use.
    ///
    /// **Stated positively, as a magnitude.** Convergence is HIGH when
    /// conduct agrees, rather than friction being ABSENT. "The reward
    /// for alignment is the absence of a penalty" is the negentropic
    /// shape this work exists to refuse — good defined as the lack of
    /// bad.
    ///
    /// **`None` when either party has no basis** (see
    /// [`Self::reciprocity_of`]): two orbs that have never traded have
    /// nothing to agree ABOUT, and reading that as perfect alignment
    /// would pay the inert exactly what it pays the reciprocal. This
    /// reading refuses to; it is a precondition for any rule that ever
    /// pays on convergence, not a payment itself.
    ///
    /// **THIS PAYS NOTHING AND MINTS NOTHING.** Whether convergence
    /// should EARN is an economic ruling that creates value, and it is
    /// not taken here (`17_CAPACITY` §3d.3).
    pub fn convergence_between(&self, a: u64, b: u64) -> Option<Ratio> {
        let left = self.reciprocity_of(a)?;
        let right = self.reciprocity_of(b)?;
        let spread = match left >= right {
            true => left - right,
            false => right - left,
        };
        Some(ratio(1, 1) - spread)
    }

}
