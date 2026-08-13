//! **THE PROVISIONING LAYER** (`43_DISTRIBUTION`, task #45+).
//!
//! The granary is the infrastructure that turns the #40–42 subsistence
//! ledger into an economy. It holds no stock of its own — the depot IS
//! the orb's banked ration (ruled 2026-08-13) — so this layer stores
//! nothing new and mints no deed: everything it adds (the stratified
//! demand, the leisure tier, the stability reading, the disposition
//! factors) is DERIVED at the collect touch, the `regrown_ration`
//! precedent. This module holds the readings the consumption cycle (L2+)
//! consults; L1 is the gate itself.

use super::*;
use crate::dop::ZoneKind;

impl World {
    /// **DOES THIS ORB STAND A GRANARY?** The gate that flips the
    /// provisioned economy on. Where it answers false, the population
    /// runs the #40–42 subsistence model verbatim (harvest→stomach, no
    /// leisure, no stability, byte-identical replay); where true, the
    /// stratified consumption cycle and its disposition factors come into
    /// force. An ACTIVE granary only — one that cannot pay its own upkeep
    /// is dark, and a dark provisioning node provisions nothing.
    pub fn has_granary(&self, orb: u64) -> bool {
        self.orb_by_id(orb)
            .map(|held| {
                held.zones
                    .iter()
                    .any(|zone| zone.kind == ZoneKind::Granary && held.zone_active(zone))
            })
            .unwrap_or(false)
    }
}
