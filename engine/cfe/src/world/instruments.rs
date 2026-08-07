//! PHASE-4 — market Notes + Futures (32_STAMP_MARKET / TASKS P4).
//!
//! Notes ≠ ideology BondSealed. Futures mark only against receipted
//! last clears (no float mark-to-model). Listing filtered by evidence
//! and Exchange seat (Q-F5).

use super::*;
use crate::dop::{MarketFuture, MarketNote, Resource, Station};

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
}
