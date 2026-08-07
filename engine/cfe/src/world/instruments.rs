//! PHASE-4 — market Notes · Futures · Shares · Perps (32 / TASKS P4).
//!
//! Notes ≠ ideology BondSealed. Futures/Perps mark only against receipted
//! last clears (no float mark-to-model). Shares dividend from yield window.
//! Listing filtered by evidence and Exchange seat (Q-F5).

use super::*;
use crate::dop::{
    hull_scrap_cost, HullRole, MarketFuture, MarketNote, MarketPerp, MarketShare, Resource,
    Station,
};

impl World {
    /// Does T6 Exchange stand on this world's station list?
    pub fn exchange_seated(&self) -> bool {
        self.stations
            .iter()
            .any(|(_, s)| *s == Station::Exchange as u8)
    }

    /// NOTE book listable: STAMP or crystal evidence, or Exchange seat.
    pub fn notes_listable(&self) -> bool {
        if self.exchange_seated() {
            return true;
        }
        self.orb().is_some_and(|o| {
            o.stamp_amount() > ratio(0, 1)
                || o.banked_amount(Resource::Crystal) > ratio(0, 1)
        })
    }

    /// FUTURE book listable: Exchange, expansionist freight affinity,
    /// or local cargo evidence (iron/crystal/biomass banked).
    pub fn futures_listable(&self) -> bool {
        if self.exchange_seated() {
            return true;
        }
        if self
            .genesis
            .as_ref()
            .is_some_and(|g| g.disposition == 1)
        {
            return true; // expansionist freight affinity
        }
        self.orb().is_some_and(|o| {
            o.banked_amount(Resource::Iron) > ratio(0, 1)
                || o.banked_amount(Resource::Crystal) > ratio(0, 1)
                || o.banked_amount(Resource::Biomass) > ratio(0, 1)
        })
    }

    /// Last-clear mark (STAMP per unit cargo). Default identity 1 when
    /// no clear receipted (bootstrap only; Future settle prefers explicit mark).
    pub fn last_clear_of(&self, resource: Resource) -> Ratio {
        self.last_clears
            .iter()
            .rev()
            .find(|(r, _)| *r == resource)
            .map(|(_, p)| p.clone())
            .unwrap_or_else(|| ratio(1, 1))
    }

    /// Receipt a last-clear mark (instrument mark authority).
    pub fn mark_last_clear(
        &mut self,
        orb: u64,
        resource: Resource,
        price: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.mark_last_clear_as(SOVEREIGN, orb, resource, price)
    }

    pub fn mark_last_clear_as(
        &mut self,
        actor: u64,
        orb: u64,
        resource: Resource,
        price: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        if !self.notes_listable() && !self.futures_listable() {
            return Err(ActionRefused::InstrumentBookClosed);
        }
        let p = price.value().clone();
        self.last_clears.retain(|(r, _)| *r != resource);
        self.last_clears.push((resource, p.clone()));
        self.close(
            ClosureEvent::LastClearMarked {
                orb_id: orb,
                resource,
                price: p,
            },
            actor,
        );
        Ok(())
    }

    /// Issue a STAMP Note: lock `collateral` STAMP; face `notional`.
    /// Maturity = now + term_ticks. Issuer pays collateral from orb stamp.
    pub fn issue_note(
        &mut self,
        orb: u64,
        holder: u64,
        notional: crate::dop::Positive,
        collateral: crate::dop::Positive,
        term_ticks: u64,
    ) -> Result<u64, ActionRefused> {
        self.issue_note_as(SOVEREIGN, orb, holder, notional, collateral, term_ticks)
    }

    pub fn issue_note_as(
        &mut self,
        actor: u64,
        orb: u64,
        holder: u64,
        notional: crate::dop::Positive,
        collateral: crate::dop::Positive,
        term_ticks: u64,
    ) -> Result<u64, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !self.notes_listable() {
            return Err(ActionRefused::InstrumentBookClosed);
        }
        let face = notional.value().clone();
        let coll = collateral.value().clone();
        if face == ratio(0, 1) || coll == ratio(0, 1) {
            return Err(ActionRefused::InstrumentEmpty);
        }
        // Collateral must cover at least half face (1/(1+1)).
        let half = face.clone() / (ratio(1, 1) + ratio(1, 1));
        if coll < half {
            return Err(ActionRefused::InstrumentUndercollateralized);
        }
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.stamp_amount();
        if have < coll {
            return Err(ActionRefused::CannotAffordStamp {
                need: Box::new(coll),
                have: Box::new(have),
            });
        }
        held.bank_stamp(-coll.clone());
        let note_id = self
            .notes
            .iter()
            .map(|n| n.note_id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        let maturity = self.clock.now().saturating_add(term_ticks.max(1));
        self.notes.push(MarketNote {
            note_id,
            orb_id: orb,
            issuer: actor,
            holder,
            notional: face.clone(),
            collateral: coll.clone(),
            maturity,
            open: true,
        });
        self.close(
            ClosureEvent::NoteIssued {
                note_id,
                orb_id: orb,
                issuer: actor,
                holder,
                notional: face,
                collateral: coll,
                maturity,
            },
            actor,
        );
        Ok(note_id)
    }

    /// Redeem a mature Note: pay notional STAMP to holder (from free
    /// stamp or seize collateral). Before maturity refuses.
    pub fn redeem_note(&mut self, note_id: u64) -> Result<(), ActionRefused> {
        self.redeem_note_as(SOVEREIGN, note_id)
    }

    pub fn redeem_note_as(
        &mut self,
        actor: u64,
        note_id: u64,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let at = self
            .notes
            .iter()
            .position(|n| n.note_id == note_id && n.open)
            .ok_or(ActionRefused::NoSuchNote { note_id })?;
        let note = self.notes[at].clone();
        if self.clock.now() < note.maturity {
            return Err(ActionRefused::NoteImmature {
                note_id,
                maturity: note.maturity,
            });
        }
        let orb = note.orb_id;
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let free = held.stamp_amount();
        let (paid, seized) = if free >= note.notional {
            held.bank_stamp(-note.notional.clone());
            // release collateral back
            held.bank_stamp(note.collateral.clone());
            (note.notional.clone(), ratio(0, 1))
        } else {
            // seize collateral toward face; remainder unpaid (integrity later)
            let seize = note.collateral.clone();
            // collateral already locked (out of bank); credit holder via free stamp path:
            // release seize into bank then treat as payment
            held.bank_stamp(seize.clone());
            let pay = if seize < note.notional {
                seize.clone()
            } else {
                note.notional.clone()
            };
            held.bank_stamp(-pay.clone());
            (pay, seize)
        };
        self.notes[at].open = false;
        self.close(
            ClosureEvent::NoteRedeemed {
                note_id,
                orb_id: orb,
                paid,
                seized,
            },
            actor,
        );
        Ok(())
    }

    /// Open a Future: long vs short, cargo notional at STAMP price;
    /// each side posts `margin` STAMP (locked from orb stamp once for issuer path v1).
    #[allow(clippy::too_many_arguments)]
    pub fn open_future(
        &mut self,
        orb: u64,
        long: u64,
        short: u64,
        resource: Resource,
        notional: crate::dop::Positive,
        price: crate::dop::Positive,
        margin: crate::dop::Positive,
        term_ticks: u64,
    ) -> Result<u64, ActionRefused> {
        self.open_future_as(
            SOVEREIGN, orb, long, short, resource, notional, price, margin, term_ticks,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn open_future_as(
        &mut self,
        actor: u64,
        orb: u64,
        long: u64,
        short: u64,
        resource: Resource,
        notional: crate::dop::Positive,
        price: crate::dop::Positive,
        margin: crate::dop::Positive,
        term_ticks: u64,
    ) -> Result<u64, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !self.futures_listable() {
            return Err(ActionRefused::InstrumentBookClosed);
        }
        if long == short {
            return Err(ActionRefused::InstrumentEmpty);
        }
        let n = notional.value().clone();
        let p = price.value().clone();
        let m = margin.value().clone();
        if n == ratio(0, 1) || p == ratio(0, 1) || m == ratio(0, 1) {
            return Err(ActionRefused::InstrumentEmpty);
        }
        // Total margin lock: 1+1 sides.
        let lock = m.clone() + m.clone();
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.stamp_amount();
        if have < lock {
            return Err(ActionRefused::CannotAffordStamp {
                need: Box::new(lock),
                have: Box::new(have),
            });
        }
        held.bank_stamp(-lock);
        let fut_id = self
            .futures
            .iter()
            .map(|f| f.fut_id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        let expiry = self.clock.now().saturating_add(term_ticks.max(1));
        self.futures.push(MarketFuture {
            fut_id,
            orb_id: orb,
            long,
            short,
            resource,
            notional: n.clone(),
            price: p.clone(),
            margin: m.clone(),
            expiry,
            open: true,
        });
        self.close(
            ClosureEvent::FutureOpened {
                fut_id,
                orb_id: orb,
                long,
                short,
                resource,
                notional: n,
                price: p,
                margin: m,
                expiry,
            },
            actor,
        );
        Ok(fut_id)
    }

    /// Settle an expired Future against last-clear mark (cash STAMP).
    pub fn settle_future(&mut self, fut_id: u64) -> Result<(), ActionRefused> {
        self.settle_future_as(SOVEREIGN, fut_id)
    }

    pub fn settle_future_as(
        &mut self,
        actor: u64,
        fut_id: u64,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let at = self
            .futures
            .iter()
            .position(|f| f.fut_id == fut_id && f.open)
            .ok_or(ActionRefused::NoSuchFuture { fut_id })?;
        let fut = self.futures[at].clone();
        if self.clock.now() < fut.expiry {
            return Err(ActionRefused::FutureImmature {
                fut_id,
                expiry: fut.expiry,
            });
        }
        let mark = self.last_clear_of(fut.resource);
        // PnL long = notional * (mark - price); short opposite.
        let diff = mark.clone() - fut.price.clone();
        let long_pnl = fut.notional.clone() * diff.clone();
        let short_pnl = ratio(0, 1) - long_pnl.clone();
        let orb = fut.orb_id;
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        // Release both margins (1+1), then apply cash PnL from pool.
        let unlock = fut.margin.clone() + fut.margin.clone();
        held.bank_stamp(unlock);
        // Apply long PnL (may go negative — clamp by available stamp)
        if long_pnl > ratio(0, 1) {
            held.bank_stamp(long_pnl.clone());
        } else if long_pnl < ratio(0, 1) {
            let loss = ratio(0, 1) - long_pnl.clone();
            let have = held.stamp_amount();
            let take = if have < loss { have } else { loss };
            held.bank_stamp(-take);
        }
        self.futures[at].open = false;
        self.close(
            ClosureEvent::FutureSettled {
                fut_id,
                orb_id: orb,
                mark,
                long_pnl,
                short_pnl,
            },
            actor,
        );
        Ok(())
    }

    pub fn notes(&self) -> &[MarketNote] {
        &self.notes
    }

    pub fn futures(&self) -> &[MarketFuture] {
        &self.futures
    }

    pub fn last_clears(&self) -> &[(Resource, Ratio)] {
        &self.last_clears
    }

    pub fn shares(&self) -> &[MarketShare] {
        &self.shares
    }

    pub fn perps(&self) -> &[MarketPerp] {
        &self.perps
    }

    /// SHARE book: Exchange, or production evidence (zones + banked stream).
    pub fn shares_listable(&self) -> bool {
        if self.exchange_seated() {
            return true;
        }
        self.orb().is_some_and(|o| {
            !o.zones.is_empty()
                && (o.banked_amount(Resource::Iron) > ratio(0, 1)
                    || o.banked_amount(Resource::Biomass) > ratio(0, 1)
                    || o.banked_amount(Resource::Crystal) > ratio(0, 1))
        })
    }

    /// PERP book: same gate as futures (index needs last-clear depth).
    pub fn perps_listable(&self) -> bool {
        self.futures_listable()
    }

    /// Issue shares on a production stream (resource). Par STAMP/unit.
    pub fn issue_share(
        &mut self,
        orb: u64,
        holder: u64,
        stream: Resource,
        units: crate::dop::Positive,
        par: crate::dop::Positive,
    ) -> Result<u64, ActionRefused> {
        self.issue_share_as(SOVEREIGN, orb, holder, stream, units, par)
    }

    pub fn issue_share_as(
        &mut self,
        actor: u64,
        orb: u64,
        holder: u64,
        stream: Resource,
        units: crate::dop::Positive,
        par: crate::dop::Positive,
    ) -> Result<u64, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !self.shares_listable() {
            return Err(ActionRefused::SharesBookClosed);
        }
        let u = units.value().clone();
        let p = par.value().clone();
        if u == ratio(0, 1) || p == ratio(0, 1) {
            return Err(ActionRefused::InstrumentEmpty);
        }
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        // Production evidence: zone stands or stream banked.
        let ok = peek(&self.orbs, orb).is_some_and(|o| {
            !o.zones.is_empty() || o.banked_amount(stream) > ratio(0, 1)
        });
        if !ok {
            return Err(ActionRefused::SharesBookClosed);
        }
        let share_id = self
            .shares
            .iter()
            .map(|s| s.share_id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        self.shares.push(MarketShare {
            share_id,
            orb_id: orb,
            holder,
            stream,
            units: u.clone(),
            par: p.clone(),
            open: true,
        });
        self.close(
            ClosureEvent::ShareIssued {
                share_id,
                orb_id: orb,
                holder,
                stream,
                units: u,
                par: p,
            },
            actor,
        );
        Ok(share_id)
    }

    /// Transfer share units to a new holder (secondary).
    pub fn transfer_share(
        &mut self,
        share_id: u64,
        to: u64,
        units: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.transfer_share_as(SOVEREIGN, share_id, to, units)
    }

    pub fn transfer_share_as(
        &mut self,
        actor: u64,
        share_id: u64,
        to: u64,
        units: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let want = units.value().clone();
        let at = self
            .shares
            .iter()
            .position(|s| s.share_id == share_id && s.open)
            .ok_or(ActionRefused::NoSuchShare { share_id })?;
        let from = self.shares[at].holder;
        if self.shares[at].units < want {
            return Err(ActionRefused::ShareOverTransfer);
        }
        self.shares[at].units = self.shares[at].units.clone() - want.clone();
        if self.shares[at].units == ratio(0, 1) {
            self.shares[at].open = false;
        }
        // Credit recipient: new share line or merge if same stream/holder.
        let stream = self.shares[at].stream;
        let orb = self.shares[at].orb_id;
        let par = self.shares[at].par.clone();
        if let Some(dest) = self
            .shares
            .iter_mut()
            .find(|s| s.open && s.holder == to && s.stream == stream && s.orb_id == orb)
        {
            dest.units += want.clone();
        } else {
            let nid = self
                .shares
                .iter()
                .map(|s| s.share_id)
                .max()
                .unwrap_or(0)
                .saturating_add(1);
            self.shares.push(MarketShare {
                share_id: nid,
                orb_id: orb,
                holder: to,
                stream,
                units: want.clone(),
                par,
                open: true,
            });
        }
        self.close(
            ClosureEvent::ShareTransferred {
                share_id,
                from,
                to,
                units: want,
            },
            actor,
        );
        Ok(())
    }

    /// Dividend: exact 1/8 of banked stream × holder units / total open units
    /// (if total zero, refuse). Pays STAMP at last_clear(stream) × resource cut.
    pub fn pay_share_dividend(&mut self, share_id: u64) -> Result<Ratio, ActionRefused> {
        self.pay_share_dividend_as(SOVEREIGN, share_id)
    }

    pub fn pay_share_dividend_as(
        &mut self,
        actor: u64,
        share_id: u64,
    ) -> Result<Ratio, ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        let at = self
            .shares
            .iter()
            .position(|s| s.share_id == share_id && s.open)
            .ok_or(ActionRefused::NoSuchShare { share_id })?;
        let share = self.shares[at].clone();
        let total_units = self
            .shares
            .iter()
            .filter(|s| s.open && s.orb_id == share.orb_id && s.stream == share.stream)
            .fold(ratio(0, 1), |a, s| a + s.units.clone());
        if total_units == ratio(0, 1) {
            return Err(ActionRefused::InstrumentEmpty);
        }
        let pool = peek(&self.orbs, share.orb_id)
            .map(|o| o.banked_amount(share.stream))
            .unwrap_or_else(|| ratio(0, 1));
        // Window cut: eighth of pool (1/(1+1+1+1+1+1+1+1)).
        let eight = {
            let two = ratio(1, 1) + ratio(1, 1);
            let four = two.clone() + two;
            four.clone() + four
        };
        let cut = pool / eight;
        let slice = cut * share.units.clone() / total_units;
        if slice == ratio(0, 1) {
            return Err(ActionRefused::InstrumentEmpty);
        }
        // Convert to STAMP at last clear (or identity).
        let mark = self.last_clear_of(share.stream);
        let stamp_pay = slice.clone() * mark;
        let held = peel(&mut self.orbs, share.orb_id)
            .ok_or(ActionRefused::NoSuchOrb { orb: share.orb_id })?;
        held.bank(share.stream, -slice.clone());
        held.bank_stamp(stamp_pay.clone());
        self.close(
            ClosureEvent::ShareDividend {
                share_id,
                orb_id: share.orb_id,
                stream: share.stream,
                resource_cut: slice,
                amount: stamp_pay.clone(),
            },
            actor,
        );
        Ok(stamp_pay)
    }

    /// Open a Perp: same margin law as Future; entry = last clear.
    #[allow(clippy::too_many_arguments)]
    pub fn open_perp(
        &mut self,
        orb: u64,
        long: u64,
        short: u64,
        resource: Resource,
        notional: crate::dop::Positive,
        margin: crate::dop::Positive,
    ) -> Result<u64, ActionRefused> {
        self.open_perp_as(SOVEREIGN, orb, long, short, resource, notional, margin)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn open_perp_as(
        &mut self,
        actor: u64,
        orb: u64,
        long: u64,
        short: u64,
        resource: Resource,
        notional: crate::dop::Positive,
        margin: crate::dop::Positive,
    ) -> Result<u64, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !self.perps_listable() {
            return Err(ActionRefused::PerpsBookClosed);
        }
        if long == short {
            return Err(ActionRefused::InstrumentEmpty);
        }
        let n = notional.value().clone();
        let m = margin.value().clone();
        if n == ratio(0, 1) || m == ratio(0, 1) {
            return Err(ActionRefused::InstrumentEmpty);
        }
        let entry = self.last_clear_of(resource);
        let lock = m.clone() + m.clone();
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.stamp_amount();
        if have < lock {
            return Err(ActionRefused::CannotAffordStamp {
                need: Box::new(lock),
                have: Box::new(have),
            });
        }
        held.bank_stamp(-lock);
        let perp_id = self
            .perps
            .iter()
            .map(|p| p.perp_id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        self.perps.push(MarketPerp {
            perp_id,
            orb_id: orb,
            long,
            short,
            resource,
            notional: n.clone(),
            entry: entry.clone(),
            margin: m.clone(),
            open: true,
        });
        self.close(
            ClosureEvent::PerpOpened {
                perp_id,
                orb_id: orb,
                long,
                short,
                resource,
                notional: n,
                entry,
                margin: m,
            },
            actor,
        );
        Ok(perp_id)
    }

    /// Funding: long pays short when mark > entry (premium), else reverse.
    /// Amount = notional × |mark − entry| / 8 (eighth ladder).
    pub fn fund_perp(&mut self, perp_id: u64) -> Result<Ratio, ActionRefused> {
        self.fund_perp_as(SOVEREIGN, perp_id)
    }

    pub fn fund_perp_as(
        &mut self,
        actor: u64,
        perp_id: u64,
    ) -> Result<Ratio, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let at = self
            .perps
            .iter()
            .position(|p| p.perp_id == perp_id && p.open)
            .ok_or(ActionRefused::NoSuchPerp { perp_id })?;
        let perp = self.perps[at].clone();
        let mark = self.last_clear_of(perp.resource);
        let diff = if mark >= perp.entry {
            mark.clone() - perp.entry.clone()
        } else {
            perp.entry.clone() - mark.clone()
        };
        let eight = {
            let two = ratio(1, 1) + ratio(1, 1);
            let four = two.clone() + two;
            four.clone() + four
        };
        let funding = perp.notional.clone() * diff / eight;
        // Maintenance: margin must cover quarter of notional×entry value.
        let four = ratio(1, 1) + ratio(1, 1) + ratio(1, 1) + ratio(1, 1);
        let maint = perp.notional.clone() * perp.entry.clone() / four;
        if perp.margin < maint && funding > ratio(0, 1) {
            return Err(ActionRefused::PerpMarginBreach { perp_id });
        }
        self.close(
            ClosureEvent::PerpFunded {
                perp_id,
                orb_id: perp.orb_id,
                funding: funding.clone(),
                mark,
            },
            actor,
        );
        Ok(funding)
    }

    /// Close perp: unlock margins, apply PnL vs mark; liquidated if margin breach.
    pub fn close_perp(&mut self, perp_id: u64) -> Result<(), ActionRefused> {
        self.close_perp_as(SOVEREIGN, perp_id, false)
    }

    pub fn liquidate_perp(&mut self, perp_id: u64) -> Result<(), ActionRefused> {
        self.close_perp_as(SOVEREIGN, perp_id, true)
    }

    pub fn close_perp_as(
        &mut self,
        actor: u64,
        perp_id: u64,
        liquidated: bool,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let at = self
            .perps
            .iter()
            .position(|p| p.perp_id == perp_id && p.open)
            .ok_or(ActionRefused::NoSuchPerp { perp_id })?;
        let perp = self.perps[at].clone();
        let mark = self.last_clear_of(perp.resource);
        let long_pnl = perp.notional.clone() * (mark.clone() - perp.entry.clone());
        let orb = perp.orb_id;
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let unlock = perp.margin.clone() + perp.margin.clone();
        held.bank_stamp(unlock);
        if long_pnl > ratio(0, 1) {
            held.bank_stamp(long_pnl.clone());
        } else if long_pnl < ratio(0, 1) {
            let loss = ratio(0, 1) - long_pnl.clone();
            let have = held.stamp_amount();
            let take = if have < loss { have } else { loss };
            held.bank_stamp(-take);
        }
        self.perps[at].open = false;
        self.close(
            ClosureEvent::PerpClosed {
                perp_id,
                orb_id: orb,
                mark,
                long_pnl,
                liquidated,
            },
            actor,
        );
        Ok(())
    }

    /// P4.10 — mint hull and issue companion yard Note (face = scrap cost / 4
    /// in STAMP identity; coll = half face) when notes listable and STAMP allows.
    pub fn mint_hull_financed(
        &mut self,
        orb: u64,
        role: HullRole,
        grade: u8,
    ) -> Result<(u64, Option<u64>), ActionRefused> {
        self.mint_hull_financed_as(SOVEREIGN, orb, role, grade)
    }

    pub fn mint_hull_financed_as(
        &mut self,
        actor: u64,
        orb: u64,
        role: HullRole,
        grade: u8,
    ) -> Result<(u64, Option<u64>), ActionRefused> {
        let cost = hull_scrap_cost(role, grade);
        let hull_id = self.mint_hull_as(actor, orb, role, grade)?;
        let mut note_id = None;
        if self.notes_listable() {
            let four = ratio(1, 1) + ratio(1, 1) + ratio(1, 1) + ratio(1, 1);
            let face = cost.clone() / four;
            let coll = face.clone() / (ratio(1, 1) + ratio(1, 1));
            let stamp = self
                .orb()
                .map(|o| o.stamp_amount())
                .unwrap_or_else(|| ratio(0, 1));
            if face > ratio(0, 1) && stamp >= coll {
                if let (Some(n), Some(c)) = (
                    crate::dop::Positive::new(face.clone()),
                    crate::dop::Positive::new(coll),
                ) {
                    if let Ok(id) = self.issue_note_as(actor, orb, actor, n, c, 60) {
                        note_id = Some(id);
                        self.close(
                            ClosureEvent::HullFinanced {
                                hull_id,
                                note_id: id,
                                orb_id: orb,
                                scrap_spent: cost,
                                note_face: face,
                            },
                            actor,
                        );
                    }
                }
            }
        }
        Ok((hull_id, note_id))
    }
}
