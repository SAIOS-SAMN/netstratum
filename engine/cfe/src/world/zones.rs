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
        // **#41 — THE LADDER'S THIRD RUNG, ENFORCED.** A minted kind
        // builds on the base its mint declared, or on the one its
        // channels infer. Where neither answers, the entry is lawful
        // vocabulary that names a building nobody can raise — so the
        // refusal lands HERE, at the verb, and the mint stays lawful.
        //
        // A fact of the chain, not a shape: what the fold holds decides
        // it, so it can never move to a door.
        let base = match kind {
            ZoneKind::Minted(code) => {
                let fold = crate::vocabulary::fold(self.chronicle(), crate::vocabulary::Axis::Zone);
                let Some(entry) = fold.iter().find(|entry| entry.code == code) else {
                    return Err(ActionRefused::ZoneKindUnknown { code });
                };
                match crate::vocabulary::zone_base(entry.base, &entry.binding).kind() {
                    Some(base) => Some(base),
                    None => return Err(ActionRefused::ZoneKindUnbuildable { code }),
                }
            }
            // A compiled kind is its own base and declares none.
            _ => None,
        };
        // The build is PRICED on the base — the binding modulates what
        // a building does, never what it costs to raise.
        let priced = base.unwrap_or(kind);
        let held = peel(&mut self.orbs, orb)
            .ok_or(ActionRefused::NoSuchOrb { orb })?;
        if held.zones.iter().any(|zone| zone.tile == tile) {
            return Err(ActionRefused::TileAlreadyZoned { tile });
        }
        for (resource, cost) in priced.cost() {
            let have = held.banked_amount(resource);
            if have < cost {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(cost),
                    have: Box::new(have),
                });
            }
        }
        for (resource, cost) in priced.cost() {
            held.bank(resource, -cost);
        }
        held.zones.push(Zone { tile, kind, base });
        held.zones.sort_by_key(|zone| zone.tile);
        let orb_id = held.orb_id;
        self.close(ClosureEvent::ZoneBuilt { orb_id, tile, kind, base }, actor);
        Ok(())
    }
}
