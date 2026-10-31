//! P8.1 — shared AP budget across build / prospect / research / quote.
//!
//! Anti click-to-win: one exact-rational capacity window per year-voice
//! (`ORBIT_PHASE_TICKS` = 60). Spending on one gated deed class costs
//! AP that cannot be spent on another in the same window.
//!
//! Pure re-derivation from the chronicle — no residual, no free mint.
//! Call-not-copy: capacity and spend tables live here once.

use super::*;
use crate::dop::ClosureEvent;
use crate::vrp::Receipt;

/// Year-voice window length (same constant as orbit phase / journal year).
pub const AP_YEAR_TICKS: u64 = ORBIT_PHASE_TICKS;

// ── P8.5 ruled AP table (exact; amend only with workspace sweep) ──
/// Base AP per year-voice for an un-habited colony.
pub const AP_BASE: u64 = 16;
/// Extra AP per standing habitat zone.
pub const AP_HABITAT_MULT: u64 = 4;
/// Extra AP when T6 Exchange is seated (books open).
pub const AP_EXCHANGE_BONUS: u64 = 2;

/// Which gated deed family spends AP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApKind {
    /// Zone build / raze-scale infra.
    Build,
    /// Prospect a stratum.
    Prospect,
    /// Research forge instrument / practice adopt.
    Research,
    /// Market quote (post offer).
    Quote,
}

impl ApKind {
    /// Exact AP cost per deed (unit cost — no dials).
    pub fn cost(self) -> u64 {
        1
    }

    pub fn word(self) -> &'static str {
        match self {
            ApKind::Build => "build",
            ApKind::Prospect => "prospect",
            ApKind::Research => "research",
            ApKind::Quote => "quote",
        }
    }
}

/// Playable AP reading for CLI / pins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApBoard {
    pub capacity: u64,
    pub spent: u64,
    pub available: u64,
    pub window_start: u64,
    pub window_end: u64,
    pub now: u64,
    pub habitats: u64,
    pub next_hint: String,
}

impl World {
    /// Habitats on the primary (or asked) orb — capacity root.
    pub fn ap_habitats(&self, orb: u64) -> u64 {
        peek(&self.orbs, orb)
            .map(|o| {
                o.zones
                    .iter()
                    .filter(|z| z.kind == ZoneKind::Habitat)
                    .count() as u64
            })
            .unwrap_or(0)
    }

    /// **AP capacity** for the current year-voice window.
    ///
    /// Ruled table (P8.5 — not a free dial):
    /// `BASE + HAB_MULT×habitats + EXCH_BONUS×(exchange seated)`.
    /// Enough for founding arcs and pure-NPC beat cadence; tight enough
    /// that spam across all four families hits opportunity cost.
    pub fn ap_capacity(&self) -> u64 {
        let hab = self.ap_habitats(FIRST_ORB);
        let exch = if self.exchange_seated() {
            AP_EXCHANGE_BONUS
        } else {
            0
        };
        AP_BASE
            .saturating_add(hab.saturating_mul(AP_HABITAT_MULT))
            .saturating_add(exch)
    }

    /// Start tick of the current year-voice window.
    pub fn ap_window_start(&self) -> u64 {
        let now = self.clock.now();
        (now / AP_YEAR_TICKS) * AP_YEAR_TICKS
    }

    /// Count of AP-costing receipts in the current window.
    pub fn ap_spent(&self) -> u64 {
        let start = self.ap_window_start();
        // Backward from the head, only the current window: the history is in
        // stamp order, so the walk stops at the window's start (nothing
        // older is decoded).
        self.chronicle
            .history()
            .rev()
            .take_while(|r| r.stamp() >= start)
            .map(|r| receipt_ap_cost(&r))
            .sum()
    }

    pub fn ap_available(&self) -> u64 {
        self.ap_capacity().saturating_sub(self.ap_spent())
    }

    /// Refuse when the window cannot cover `kind.cost()`.
    pub fn require_ap(&self, kind: ApKind) -> Result<(), ActionRefused> {
        let need = kind.cost();
        let have = self.ap_available();
        if have < need {
            return Err(ActionRefused::ApExhausted {
                kind: kind.word().into(),
                need,
                have,
                capacity: self.ap_capacity(),
            });
        }
        Ok(())
    }

    /// Full AP board for hub / CLI.
    pub fn ap_board(&self) -> ApBoard {
        let capacity = self.ap_capacity();
        let spent = self.ap_spent();
        let available = capacity.saturating_sub(spent);
        let window_start = self.ap_window_start();
        let window_end = window_start.saturating_add(AP_YEAR_TICKS);
        let now = self.clock.now();
        let habitats = self.ap_habitats(FIRST_ORB);
        let next_hint = if available == 0 {
            format!(
                "AP exhausted ({spent}/{capacity}) — wait year voice t{window_end} or raise habitats"
            )
        } else if spent * 2 >= capacity {
            format!(
                "AP tight ({available} left of {capacity}) — choose build|prospect|research|quote"
            )
        } else {
            format!("AP {available}/{capacity} this year-voice — shared across build·prospect·research·quote")
        };
        ApBoard {
            capacity,
            spent,
            available,
            window_start,
            window_end,
            now,
            habitats,
            next_hint,
        }
    }
}

/// How much AP a receipt spent (0 if not a gated family).
fn receipt_ap_cost(receipt: &Receipt) -> u64 {
    match receipt.event() {
        ClosureEvent::ZoneBuilt { .. } => ApKind::Build.cost(),
        ClosureEvent::ProspectStruck { .. } => ApKind::Prospect.cost(),
        ClosureEvent::InstrumentForged { .. } | ClosureEvent::PracticeAdopted { .. } => {
            ApKind::Research.cost()
        }
        ClosureEvent::OfferPosted { .. } => ApKind::Quote.cost(),
        _ => 0,
    }
}
