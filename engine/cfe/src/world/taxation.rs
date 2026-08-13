//! **THE CRAFTABLE TAX SYSTEM** (`45_TAXATION`, task #54/#55). A levy is a
//! tax as data — what it draws, when, where it flows (`dop::Levy`). This
//! module holds the readings the levies settle from; the continuity tax
//! (§3) is the first concrete instance, a demurrage on KNOWLEDGE keyed to
//! the lifespan turnover.

use super::*;
use crate::dop::Levy;

impl World {
    /// **THE CONTINUITY TAX a line owes at each turnover** (`45_TAXATION`
    /// §3 / `42_LIFESPAN §3`) — the fraction of its unconfirmed knowledge
    /// (crystallized boons, adopted practices) that LAPSES when a generation
    /// turns over. Derived from lifespan (first-cut rate, flagged for
    /// tuning): a short-lived line (L = one grain) loses ALL of it each
    /// turnover and must re-earn what the last generation knew; a long-lived
    /// line (L = four grains) loses only a quarter — so the Custodian
    /// ACCUMULATES and the Locust CHURNS. `grain/lifespan`, capped at one;
    /// derived (grain and lifespan are structural, not dialed).
    pub fn continuity_tax(&self) -> Ratio {
        let grain = crate::almanac::REGROWTH_GRAIN as i64;
        let lifespan = self.lifespan_of().max(1) as i64;
        ratio(grain, lifespan).min(ratio(1, 1))
    }

    /// The standing continuity levy — its base/trigger/sink (Knowledge /
    /// Turnover / Void). The `continuity_tax` is its measured rate.
    pub fn continuity_levy(&self) -> Levy {
        Levy::CONTINUITY
    }

    /// The standing estate levy (`45_TAXATION` §4) — Estate / Turnover /
    /// Sovereign. Its rate lands with #56; declared here so the seeded tax
    /// system is legible from one place.
    pub fn estate_levy(&self) -> Levy {
        Levy::ESTATE
    }
}
