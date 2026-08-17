//! Wave-6 offices as play — Exchange MM · Assay court · Vault custody (30).

use super::*;
use crate::dop::Station;

impl World {
    pub fn assay_seated(&self) -> bool {
        self.stations
            .iter()
            .any(|(_, s)| *s == Station::Assay as u8)
    }

    pub fn vault_seated(&self) -> bool {
        self.stations
            .iter()
            .any(|(_, s)| *s == Station::Vault as u8)
    }

    /// O6.3 — only seated Exchange (market-maker) may list instruments.
    pub fn require_exchange_mm(&self) -> Result<(), ActionRefused> {
        if !self.exchange_seated() {
            return Err(ActionRefused::ExchangeRequired);
        }
        Ok(())
    }

    /// STAMP held in Vault custody for an orb (not free bank).
    pub fn vault_balance_of(&self, orb: u64) -> Ratio {
        self.vault_stamp
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, a)| a.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    pub(super) fn set_vault_balance(&mut self, orb: u64, amount: Ratio) {
        self.vault_stamp.retain(|(o, _)| *o != orb);
        if amount > ratio(0, 1) {
            self.vault_stamp.push((orb, amount));
        }
    }

    /// O6.5 deposit free STAMP into Vault custody.
    pub fn vault_deposit(
        &mut self,
        orb: u64,
        amount: crate::dop::Positive,
    ) -> Result<Ratio, ActionRefused> {
        self.vault_deposit_as(SOVEREIGN, orb, amount)
    }

    pub fn vault_deposit_as(
        &mut self,
        actor: u64,
        orb: u64,
        amount: crate::dop::Positive,
    ) -> Result<Ratio, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !self.vault_seated() {
            return Err(ActionRefused::VaultRequired);
        }
        let amt = amount.value().clone();
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.stamp_amount();
        if have < amt {
            return Err(ActionRefused::CannotAffordStamp {
                need: Box::new(amt),
                have: Box::new(have),
            });
        }
        held.bank_stamp(-amt.clone());
        let next = self.vault_balance_of(orb) + amt.clone();
        self.set_vault_balance(orb, next.clone());
        self.close(
            ClosureEvent::VaultDeposited {
                orb_id: orb,
                amount: amt,
            },
            actor,
        );
        Ok(next)
    }

    /// O6.5 withdraw from Vault (free STAMP back to bank).
    pub fn vault_withdraw(
        &mut self,
        orb: u64,
        amount: crate::dop::Positive,
    ) -> Result<Ratio, ActionRefused> {
        self.vault_withdraw_as(SOVEREIGN, orb, amount)
    }

    pub fn vault_withdraw_as(
        &mut self,
        actor: u64,
        orb: u64,
        amount: crate::dop::Positive,
    ) -> Result<Ratio, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !self.vault_seated() {
            return Err(ActionRefused::VaultRequired);
        }
        let amt = amount.value().clone();
        let have = self.vault_balance_of(orb);
        if have < amt {
            return Err(ActionRefused::VaultShort {
                need: Box::new(amt),
                have: Box::new(have),
            });
        }
        // Margin holds: free vault must cover open instrument margins + note coll.
        let reserved = self.vault_reserved(orb);
        let free = if have > reserved {
            have.clone() - reserved
        } else {
            ratio(0, 1)
        };
        if free < amt {
            return Err(ActionRefused::VaultMarginHeld {
                free: Box::new(free),
                need: Box::new(amt),
            });
        }
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        held.bank_stamp(amt.clone());
        let next = have - amt.clone();
        self.set_vault_balance(orb, next.clone());
        self.close(
            ClosureEvent::VaultWithdrawn {
                orb_id: orb,
                amount: amt,
            },
            actor,
        );
        Ok(next)
    }

    /// Margin + note collateral that must stay in vault when Vault sits.
    pub fn vault_reserved(&self, orb: u64) -> Ratio {
        if !self.vault_seated() {
            return ratio(0, 1);
        }
        let mut r = ratio(0, 1);
        for n in &self.notes {
            if n.open && n.orb_id == orb {
                r += n.collateral.clone();
            }
        }
        for f in &self.futures {
            if f.open && f.orb_id == orb {
                // both sides' margin locked once in v1 (see instruments)
                r += f.margin.clone() + f.margin.clone();
            }
        }
        for p in &self.perps {
            if p.open && p.orb_id == orb {
                r += p.margin.clone() + p.margin.clone();
            }
        }
        r
    }

    // ONT-2: the O6.4 assay-consilience court is DELETED. It existed only to
    // catch `vein_wear`'s stored richness drifting from the geometric truth
    // and slash crystal to snap it back — a patch confessing the scalar was a
    // lossy shadow of the manifold. Richness is now derived as
    // `element_properties − extracted` on every read: it cannot drift, so
    // there is nothing to challenge and no crystal to spend enforcing geometry.
}
