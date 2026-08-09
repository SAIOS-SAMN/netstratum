//! P8.3 — labor / population derived from habitats (and clones).
//!
//! Pure reading. Habitats seat living capacity; seated clone labs add
//! controlled population. No free labor mint — only standing zones +
//! residual clone counts (science lab path).

use super::*;
use crate::dop::ZoneKind;

/// Labor reading for one orb.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaborBoard {
    /// Habitat zone count (base seats).
    pub habitats: u64,
    /// Clone residual count (bio path).
    pub clones: u64,
    /// Working force = habitats + clones (exact).
    pub force: u64,
    /// Soft capacity for simultaneous AP-adjacent work (force/2, min 1 if force>0).
    pub work_slots: u64,
    pub next_hint: String,
}

impl World {
    /// Habitat count on an orb.
    pub fn habitat_count(&self, orb: u64) -> u64 {
        peek(&self.orbs, orb)
            .map(|o| {
                o.zones
                    .iter()
                    .filter(|z| z.kind == ZoneKind::Habitat)
                    .count() as u64
            })
            .unwrap_or(0)
    }

    /// **P8.3 LABOR** — force derived from habitats + clones.
    pub fn labor_force(&self, orb: u64) -> u64 {
        self.habitat_count(orb)
            .saturating_add(self.clone_count_of(orb))
    }

    /// Full labor board for CLI / pins.
    pub fn labor_board(&self, orb: u64) -> LaborBoard {
        let habitats = self.habitat_count(orb);
        let clones = self.clone_count_of(orb);
        let force = habitats.saturating_add(clones);
        let work_slots = if force == 0 {
            0
        } else {
            (force / 2).max(1)
        };
        let next_hint = if force == 0 {
            "no labor — build habitat zones (`zone <tile> hab`) or seat clone path".into()
        } else if habitats == 0 && clones > 0 {
            format!("{clones} clone labor without habitats — seat hearths for stability")
        } else {
            format!("{force} force ({habitats} hab + {clones} clone) · {work_slots} work slot(s)")
        };
        LaborBoard {
            habitats,
            clones,
            force,
            work_slots,
            next_hint,
        }
    }

    /// **P8.2 ARC READING** — trajectory band derived from world state.
    ///
    /// Conductor `Trajectory` is polity-local (not chronicle); the
    /// observer reads this pure stand-in so boards stay SEAL B pure:
    /// decline if famine, stagnation if cognitive wants, else growth.
    pub fn arc_band(&self) -> &'static str {
        let famished = self.orb().is_some_and(|o| !o.famine.is_empty());
        if famished {
            return "decline";
        }
        if !self.wants.is_empty() {
            return "stagnation";
        }
        "growth"
    }

    /// Hunger kinds struck (WANT-1 wire names).
    pub fn hunger_words(&self) -> Vec<&'static str> {
        self.wants
            .iter()
            .filter_map(|(_, k)| match k {
                1 => Some("epistemological"),
                2 => Some("resonance"),
                3 => Some("novelty"),
                _ => Some("want"),
            })
            .collect()
    }

    /// Rail word for max open element rail (R1–R4).
    pub fn max_rail_word(&self) -> &'static str {
        match self.max_rail() {
            1 => "R1",
            2 => "R2",
            3 => "R3",
            4 => "R4",
            _ => "R?",
        }
    }
}
