//! ZONES — what a colony builds on its grid.

use super::*;

impl World {
    /// Zone on the genesis orb as the sovereign.
    pub fn zone(&mut self, tile: u16, kind: ZoneKind) -> Result<(), ActionRefused> {
        self.zone_as(SOVEREIGN, FIRST_ORB, tile, kind)
    }
    /// Zone a tile of one orb — the build spends from that orb's bank
    /// (02 §2.2); Build scope; the receipt carries the actor's tag.
    pub fn zone_as(
        &mut self,
        actor: u64,
        orb: u64,
        tile: u16,
        kind: ZoneKind,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        // P8.1 — shared AP (build family).
        self.require_ap(crate::world::ApKind::Build)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        // REGEN-1 / REFINE-1: regeneration and elemental refining are
        // technologies. Extractive · regenerative · balanced modes
        // (37 / almanac::taxon_economy) colour the tech door:
        // regenerative lines may seat regenerators at condition 2;
        // balanced/extractive stay at deepening (3). Refinery always 3.
        if matches!(kind, ZoneKind::Regenerator | ZoneKind::ElementRefinery) {
            let need = if matches!(kind, ZoneKind::Regenerator) {
                self.regenerator_tech_need()
            } else {
                3
            };
            if !self
                .techs
                .iter()
                .any(|(held, condition, _, _)| *held == orb && *condition == need)
            {
                return Err(ActionRefused::ConditionUncrystallized { condition: need });
            }
        }
        // Extractive soft-cap: regenerative lines feel pressure after few extractors.
        if matches!(kind, ZoneKind::Extractor) {
            let standing = peek(&self.orbs, orb)
                .map(|o| {
                    o.zones
                        .iter()
                        .filter(|z| z.kind == ZoneKind::Extractor)
                        .count() as u64
                })
                .unwrap_or(0);
            let cap = self.extractor_soft_cap();
            if standing >= cap {
                // Not a hard refuse — residual is informational via board.
                // Hard refuse only on hostile planet + regenerative over-cap.
                let eco = self.taxon_economy_of(orb);
                if eco.mode == crate::almanac::EconomicMode::Regenerative
                    && eco.fit == crate::almanac::PlanetFit::Hostile
                    && standing >= cap
                {
                    return Err(ActionRefused::EconomicModeBlocked {
                        mode: eco.mode.as_wire(),
                        kind: ZoneKind::Extractor.wire(),
                        reason: 1, // over extract soft-cap on hostile world
                    });
                }
            }
        }
        // **#41 — the ladder's third RUNG, enforced.** A minted kind
        // builds on the base its mint declared, or on the one its
        // channels infer. Where neither answers, the entry is lawful
        // vocabulary that names a building nobody can raise — so the
        // refusal lands here, at the verb, and the mint stays lawful.
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
        // The build is priced on the base — the binding modulates what
        // a building does, never what it costs to raise.
        let priced = base.unwrap_or(kind);
        // The zone_upkeep/zone_yield substitution, applied to cost: the
        // FOOD-slot (Biomass) line prices in the line's own ration;
        // structural lines never substitute.
        let metabolism = self
            .genesis
            .as_ref()
            .map(|held| held.metabolism)
            .unwrap_or(2);
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        if held.zones.iter().any(|zone| zone.tile == tile) {
            return Err(ActionRefused::TileAlreadyZoned { tile });
        }
        for (resource, cost) in crate::dop::zone_cost(priced, metabolism) {
            let have = held.banked_amount(resource);
            if have < cost {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(cost),
                    have: Box::new(have),
                });
            }
        }
        for (resource, cost) in crate::dop::zone_cost(priced, metabolism) {
            held.bank(resource, -cost);
        }
        held.zones.push(Zone { tile, kind, base });
        held.zones.sort_by_key(|zone| zone.tile);
        let orb_id = held.orb_id;
        self.close(
            ClosureEvent::ZoneBuilt {
                orb_id,
                tile,
                kind,
                base,
            },
            actor,
        );
        Ok(())
    }
}
