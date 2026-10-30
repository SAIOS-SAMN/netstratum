//! P5-G deep rails — R1→R2 promotion; R3/R4 optional feature gates (30 / tasks).
//!
//! R1 Hearth and R2 Aperture are always lawful. R3 Strand / R4 Court promote
//! only when `deep-rail-strand` / `deep-rail-court` features are enabled on
//! cfe — no hard path dependency on external strand/datum adapters.

use super::*;
use crate::dop::Rail;

impl World {
    /// Max rail altitude settled on any element route (pure).
    pub fn max_rail(&self) -> u8 {
        self.element_routes
            .iter()
            .map(|(_, _, _, r)| *r)
            .max()
            .unwrap_or(Rail::Hearth as u8)
    }

    /// Pure deep-rail surface for desk / CLI.
    pub fn deep_rail_status(&self) -> DeepRailStatus {
        DeepRailStatus {
            max_rail: self.max_rail(),
            routes: self.element_routes.len() as u64,
            strand_enabled: cfg!(feature = "deep-rail-strand"),
            court_enabled: cfg!(feature = "deep-rail-court"),
            anchor_seated: self
                .stations
                .iter()
                .any(|(_, s)| *s == crate::dop::Station::Anchor as u8),
            lease_hull_open: false, // Q-F6 deferred — own-mint remains v1
        }
    }

    /// Promote an element-route rail one step (R1→R2 always; R2→R3 / R3→R4 gated).
    pub fn promote_rail(&mut self, deal: u64) -> Result<u8, ActionRefused> {
        self.promote_rail_as(SOVEREIGN, deal)
    }

    pub fn promote_rail_as(&mut self, actor: u64, deal: u64) -> Result<u8, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let at = self
            .element_routes
            .iter()
            .position(|(d, _, _, _)| *d == deal)
            .ok_or(ActionRefused::RouteUnknown { deal })?;
        let from = self.element_routes[at].3;
        let from_rail = Rail::from_wire(from).ok_or(ActionRefused::RailUnknown { rail: from })?;
        let to_rail = match from_rail {
            Rail::Hearth => Rail::Aperture,
            Rail::Aperture => {
                if cfg!(feature = "deep-rail-strand") {
                    Rail::Strand
                } else {
                    return Err(ActionRefused::DeepRailGated {
                        rail: Rail::Strand as u8,
                        feature: "deep-rail-strand",
                    });
                }
            }
            Rail::Strand => {
                if cfg!(feature = "deep-rail-court") {
                    Rail::Court
                } else {
                    return Err(ActionRefused::DeepRailGated {
                        rail: Rail::Court as u8,
                        feature: "deep-rail-court",
                    });
                }
            }
            Rail::Court => {
                return Err(ActionRefused::RailAtCeiling { deal, rail: from });
            }
        };
        // T9 Anchor required for R3/R4 exits (30_OFFICES).
        if matches!(to_rail, Rail::Strand | Rail::Court)
            && !self
                .stations
                .iter()
                .any(|(_, s)| *s == crate::dop::Station::Anchor as u8)
        {
            return Err(ActionRefused::AnchorRequired {
                rail: to_rail as u8,
            });
        }
        let to = to_rail as u8;
        self.element_routes[at].3 = to;
        self.close(
            ClosureEvent::RailPromoted {
                deal,
                from_rail: from,
                to_rail: to,
            },
            actor,
        );
        Ok(to)
    }

    /// Element routes public slice for CLI.
    pub fn element_routes(&self) -> &[(u64, crate::dop::Unit, [u8; 32], u8)] {
        &self.element_routes
    }
}

/// Deep-rail integrity snapshot.
#[derive(Debug, Clone)]
pub struct DeepRailStatus {
    pub max_rail: u8,
    pub routes: u64,
    pub strand_enabled: bool,
    pub court_enabled: bool,
    pub anchor_seated: bool,
    pub lease_hull_open: bool,
}

