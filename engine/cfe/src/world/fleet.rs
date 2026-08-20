//! FLEET-1 — scrap, shipyards, hulls (33 Phase 3 / TASKS_TRADE_FLEET_DESK).
//!
//! Scrap is one pool on the orb (Q-F3). A ShipYard zone is required to
//! mint (Q-F4). Hull capacity/η/upkeep DERIVE from role+grade — no
//! undervied stat block. Hauls with positive melt require free capacity.

use super::*;
use crate::dop::{hull_capacity, hull_eta, hull_scrap_cost, Hull, HullRole};

impl World {
    /// Free haul capacity available for `deal` (0 = idle pool only).
    /// Idle hulls + hulls assigned to this deal count.
    pub fn free_capacity(&self, deal: u64) -> Ratio {
        self.hulls
            .iter()
            .filter(|h| h.assigned_route == 0 || h.assigned_route == deal)
            .map(|h| hull_capacity(h.role, h.grade))
            .fold(ratio(0, 1), |s, c| s + c)
    }

    /// Best η among hulls available for `deal` (idle or assigned).
    pub fn best_hull_eta(&self, deal: u64) -> Ratio {
        self.hulls
            .iter()
            .filter(|h| h.assigned_route == 0 || h.assigned_route == deal)
            .map(|h| hull_eta(h.role, h.grade))
            .max()
            .unwrap_or_else(|| ratio(0, 1))
    }

    pub fn hulls(&self) -> &[Hull] {
        &self.hulls
    }

    /// Salvage scrap from iron (identity convert of amount). Requires
    /// a ShipYard on the orb. Iron debit, scrap credit.
    pub fn salvage_scrap(
        &mut self,
        orb: u64,
        amount: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.salvage_scrap_as(SOVEREIGN, orb, amount)
    }

    pub fn salvage_scrap_as(
        &mut self,
        actor: u64,
        orb: u64,
        amount: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        let spent = amount.value().clone();
        let held = peek(&self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        if !held.zones.iter().any(|z| z.kind == ZoneKind::ShipYard) {
            return Err(ActionRefused::NoShipYard { orb });
        }
        let have = held.banked_amount(Resource::Iron);
        if have < spent {
            return Err(ActionRefused::CannotAfford {
                resource: Resource::Iron,
                need: Box::new(spent),
                have: Box::new(have),
            });
        }
        let held = peel(&mut self.orbs, orb).expect("checked");
        held.bank(Resource::Iron, -spent.clone());
        held.bank_scrap(spent.clone());
        self.close(
            ClosureEvent::ScrapSalvaged {
                orb_id: orb,
                amount: spent.clone(),
                iron_spent: spent.clone(),
            },
            actor,
        );
        // Recycling residual — not order-work gated (34 §2.2).
        let (ent, syn) = self.order_arms_salvage(orb, &spent);
        self.post_order_worked(actor, orb, balance::ORDER_SALVAGE, ent, syn);
        Ok(())
    }

    /// Salvage scrap from a banked element (identity amount). Yard
    /// required. Ties alchemy surplus / raw matter into the fleet loop
    /// (34 §3). Not order-gated — recycling path.
    pub fn salvage_element_scrap(
        &mut self,
        orb: u64,
        element: [u8; 32],
        amount: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.salvage_element_scrap_as(SOVEREIGN, orb, element, amount)
    }

    pub fn salvage_element_scrap_as(
        &mut self,
        actor: u64,
        orb: u64,
        element: [u8; 32],
        amount: crate::dop::Positive,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        // #136 D3 — the element salvage debit declares its grain: the spend SMELTS
        // to the element grain (1/1024) so the stock stays on-grain after the draw (an
        // off-grain spend knocks an on-grain stock off it). Scrap fills with the same
        // smelted spend and the receipt carries it, so replay is byte-identical.
        let spent = smelt_element(&amount.value().clone());
        let held = peek(&self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        if !held.zones.iter().any(|z| z.kind == ZoneKind::ShipYard) {
            return Err(ActionRefused::NoShipYard { orb });
        }
        let have = held.element_amount(&element);
        if have < spent {
            return Err(ActionRefused::CannotAffordElement {
                element,
                need: Box::new(spent),
                have: Box::new(have),
            });
        }
        let held = peel(&mut self.orbs, orb).expect("checked");
        held.bank_element(element, -spent.clone());
        held.bank_scrap(spent.clone());
        self.close(
            ClosureEvent::ElementSalvaged {
                orb_id: orb,
                element,
                amount: spent.clone(),
            },
            actor,
        );
        let (ent, syn) = self.order_arms_salvage(orb, &spent);
        self.post_order_worked(actor, orb, balance::ORDER_SALVAGE, ent, syn);
        Ok(())
    }

    /// Mint a hull at the yard. Spends scrap (role/grade cost). Catalog
    /// filter by genesis metabolism when sealed (P3.6).
    pub fn mint_hull(
        &mut self,
        orb: u64,
        role: HullRole,
        grade: u8,
    ) -> Result<u64, ActionRefused> {
        self.mint_hull_as(SOVEREIGN, orb, role, grade)
    }

    pub fn mint_hull_as(
        &mut self,
        actor: u64,
        orb: u64,
        role: HullRole,
        grade: u8,
    ) -> Result<u64, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let grade = grade.min(7);
        if !self.hull_catalog_allows(role) {
            return Err(ActionRefused::HullRoleForbidden {
                role: role.as_wire(),
            });
        }
        let held = peek(&self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        if !held.zones.iter().any(|z| z.kind == ZoneKind::ShipYard) {
            return Err(ActionRefused::NoShipYard { orb });
        }
        let cost = hull_scrap_cost(role, grade);
        let have = held.scrap_amount();
        if have < cost {
            return Err(ActionRefused::CannotAffordScrap {
                need: Box::new(cost),
                have: Box::new(have),
            });
        }
        // 34_MATTER_BALANCE: hull mint is order-work.
        self.require_order_balance(orb)?;
        let (ent, syn) = self.order_arms_hull(&cost);
        let hull_id = self
            .hulls
            .iter()
            .map(|h| h.hull_id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        let held = peel(&mut self.orbs, orb).expect("checked");
        held.bank_scrap(-cost.clone());
        self.hulls.push(Hull {
            hull_id,
            role: role.as_wire(),
            grade,
            home_orb: orb,
            assigned_route: 0,
        });
        self.close(
            ClosureEvent::HullMinted {
                hull_id,
                orb_id: orb,
                role: role.as_wire(),
                grade,
                scrap_spent: cost,
            },
            actor,
        );
        self.post_order_worked(actor, orb, balance::ORDER_HULL, ent, syn);
        Ok(hull_id)
    }

    /// Assign hull to a route deal (0 = return to idle pool).
    pub fn assign_hull(&mut self, hull_id: u64, deal: u64) -> Result<(), ActionRefused> {
        self.assign_hull_as(SOVEREIGN, hull_id, deal)
    }

    pub fn assign_hull_as(
        &mut self,
        actor: u64,
        hull_id: u64,
        deal: u64,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let Some(hull) = self.hulls.iter_mut().find(|h| h.hull_id == hull_id) else {
            return Err(ActionRefused::NoSuchHull { hull_id });
        };
        if deal != 0
            && !self.routes.iter().any(|(d, _)| *d == deal)
            && !self.element_routes.iter().any(|(d, _, _, _)| *d == deal)
        {
            return Err(ActionRefused::RouteUnknown { deal });
        }
        hull.assigned_route = deal;
        self.close(
            ClosureEvent::HullAssigned { hull_id, deal },
            actor,
        );
        Ok(())
    }

    /// Scrap a hull — reverse half the mint cost as scrap (derive half
    /// as 1+1). Removes the hull.
    pub fn scrap_hull(&mut self, hull_id: u64) -> Result<(), ActionRefused> {
        self.scrap_hull_as(SOVEREIGN, hull_id)
    }

    pub fn scrap_hull_as(
        &mut self,
        actor: u64,
        hull_id: u64,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let at = self
            .hulls
            .iter()
            .position(|h| h.hull_id == hull_id)
            .ok_or(ActionRefused::NoSuchHull { hull_id })?;
        let hull = self.hulls.remove(at);
        let role = HullRole::from_wire(hull.role).unwrap_or(HullRole::Hauler);
        let full = hull_scrap_cost(role, hull.grade);
        let returned = full / (ratio(1, 1) + ratio(1, 1));
        let orb = hull.home_orb;
        if let Some(held) = peel(&mut self.orbs, orb) {
            held.bank_scrap(returned.clone());
        }
        self.close(
            ClosureEvent::HullScrapped {
                hull_id,
                orb_id: orb,
                scrap_returned: returned,
            },
            actor,
        );
        Ok(())
    }

    /// Metabolism filter (P3.6): plasmic may mint Runner freely;
    /// biotic prefers Hauler; mineral full catalog; unsealed = all.
    fn hull_catalog_allows(&self, role: HullRole) -> bool {
        let Some(g) = &self.genesis else {
            return true;
        };
        // P5-A: Escort is always catalog-legal (KIN defense); freight roles stay
        // metabolism-coloured.
        if matches!(role, HullRole::Escort) {
            return true;
        }
        match g.metabolism {
            // mineral — full catalog
            1 => true,
            // biotic — hauler + semi (organic bulk)
            2 => matches!(role, HullRole::Hauler | HullRole::Semi),
            // plasmic — runner + hauler (courier emphasis)
            3 => matches!(role, HullRole::Runner | HullRole::Hauler | HullRole::Semi),
            _ => true,
        }
    }
}

/// Voice name for a hull role under a metabolism (P3.7 — voice only).
pub fn hull_voice(role: HullRole, metabolism: Option<u8>) -> &'static str {
    match (metabolism, role) {
        (Some(2), HullRole::Hauler) => "spore-barge",
        (Some(2), HullRole::Semi) => "chitin-semi",
        (Some(2), HullRole::Runner) => "pollen-runner",
        (Some(2), HullRole::Escort) => "spine-escort",
        (Some(3), HullRole::Hauler) => "plasma-sled",
        (Some(3), HullRole::Semi) => "arc-semi",
        (Some(3), HullRole::Runner) => "spark-runner",
        (Some(3), HullRole::Escort) => "flare-escort",
        (_, HullRole::Hauler) => "hauler",
        (_, HullRole::Semi) => "semi",
        (_, HullRole::Runner) => "runner",
        (_, HullRole::Escort) => "escort",
    }
}

/// Capacity gate for positive-melt hauls (Phase 3 exit).
pub(super) fn require_hull_capacity(
    world: &World,
    deal: u64,
    amount: &Ratio,
) -> Result<(), ActionRefused> {
    let have = world.free_capacity(deal);
    if have < *amount {
        return Err(ActionRefused::HullCapacityShort {
            need: Box::new(amount.clone()),
            have: Box::new(have),
        });
    }
    Ok(())
}
