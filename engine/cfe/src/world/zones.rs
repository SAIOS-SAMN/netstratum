//! ZONES — what a colony builds on its grid.

use super::*;

impl World {
    /// Zone on the genesis orb as the sovereign.
    pub fn zone(&mut self, tile: u16, kind: ZoneKind) -> Result<(), ActionRefused> {
        self.zone_as(SOVEREIGN, FIRST_ORB, tile, kind)
    }
    /// Zone a tile of one orb — the build SPENDS from that orb's bank
    /// (02 §2.2); Build scope; the receipt carries the actor's tag.
    pub fn zone_as(
        &mut self,
        actor: u64,
        orb: u64,
        tile: u16,
        kind: ZoneKind,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        // REGEN-1: regeneration is a TECHNOLOGY — placing a
        // regenerator requires the DEEPENING (condition 3)
        // crystallized on this orb. A fact of the sky, at the verb.
        if kind == ZoneKind::Regenerator
            && !self
                .techs
                .iter()
                .any(|(held, condition, _, _)| *held == orb && *condition == 3)
        {
            return Err(ActionRefused::ConditionUncrystallized { condition: 3 });
        }
        let held = peel(&mut self.orbs, orb)
            .ok_or(ActionRefused::NoSuchOrb { orb })?;
        if held.zones.iter().any(|zone| zone.tile == tile) {
            return Err(ActionRefused::TileAlreadyZoned { tile });
        }
        for (resource, cost) in kind.cost() {
            let have = held.banked_amount(resource);
            if have < cost {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(cost),
                    have: Box::new(have),
                });
            }
        }
        for (resource, cost) in kind.cost() {
            held.bank(resource, -cost);
        }
        held.zones.push(Zone { tile, kind });
        held.zones.sort_by_key(|zone| zone.tile);
        let orb_id = held.orb_id;
        self.close(ClosureEvent::ZoneBuilt { orb_id, tile, kind }, actor);
        Ok(())
    }
}
