//! Deep bench game boards — pure readings the CLI hub plays on.
//!
//! Research · intel · link · doctrine surfaces as *games state*, not
//! decoration. Call-not-copy: constellation / detection / treaties are
//! the authorities; this module only composes them into playable boards.

use super::*;
use crate::world::survey::{node_kind, ForgeableGrade, NodeKind};

/// One tech/constellation row for the research game.
#[derive(Debug, Clone)]
pub struct ResearchNode {
    pub condition: u8,
    pub kind: NodeKind,
    pub held: bool,
    pub progress: u64,
    pub threshold: u64,
    /// Ready to forge if held crystallized/crown and grade door opens.
    pub forgeable: bool,
    /// Already have an instrument at this grade on the orb.
    pub instrumented: bool,
    pub next_hint: &'static str,
}

/// Research game board for one orb.
#[derive(Debug, Clone)]
pub struct ResearchBoard {
    pub nodes: Vec<ResearchNode>,
    pub held_count: u64,
    pub forgeable_ready: u64,
    pub instruments: u64,
    pub practices: u64,
    pub wants: u64,
    pub novelty_blocked: bool,
    pub next_move: String,
}

/// Intel ladder step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntelStep {
    ForgeDetection,
    Probe,
    Exfil,
    Sabotage,
    Idle,
}

/// Intel game pipeline.
#[derive(Debug, Clone)]
pub struct IntelBoard {
    pub detection: u8,
    pub instruments: usize,
    pub probe_marks: u64,
    pub intel_stash: u64,
    pub veiled: bool,
    pub can_probe: bool,
    pub can_exfil: bool,
    pub can_sabotage: bool,
    pub tech_conditions: Vec<u8>,
    pub next: IntelStep,
    pub next_hint: String,
}

/// Cross-pollination readiness.
#[derive(Debug, Clone)]
pub struct LinkBoard {
    pub crossed: bool,
    pub councils: usize,
    pub council_open: usize,
    pub bonds: usize,
    pub forwards: usize,
    pub treaties: usize,
    pub at_war_count: usize,
    pub techs_licensable: Vec<u8>,
    pub score: Ratio,
    pub next_hint: String,
}

/// Doctrine / domain posture board.
#[derive(Debug, Clone)]
pub struct DoctrineBoard {
    pub primary: String,
    pub secondary: String,
    pub ranks: [u8; 7],
    pub metabolism: u8,
    pub disposition: u8,
    pub perception: u8,
    pub reoriented: bool,
    pub voice: String,
    pub kin_voice: String,
    pub next_hint: String,
}

impl World {
    /// **RESEARCH GAME** — constellation + forge readiness + next move.
    pub fn research_board(&self, orb: u64) -> ResearchBoard {
        let constellation = self.constellation(orb);
        let novelty_blocked = self.wants.iter().any(|(o, k)| *o == orb && *k == 3);
        let mut nodes = Vec::new();
        let mut forgeable_ready = 0u64;
        for (condition, held, progress, threshold) in constellation {
            let kind = node_kind(condition);
            let forgeable = held
                && ForgeableGrade::new(condition).is_some()
                && !self
                    .instruments
                    .iter()
                    .any(|(o, _, g)| *o == orb && *g == condition);
            if forgeable {
                forgeable_ready = forgeable_ready.saturating_add(1);
            }
            let instrumented = self
                .instruments
                .iter()
                .any(|(o, _, g)| *o == orb && *g == condition);
            let next_hint = if !held && progress < threshold {
                "earn progress (tick / hardship / heat)"
            } else if !held {
                "crystallize / claim work"
            } else if forgeable {
                "forge <condition>"
            } else if instrumented {
                "instrument held — research complete for grade"
            } else {
                "held"
            };
            nodes.push(ResearchNode {
                condition,
                kind,
                held,
                progress,
                threshold,
                forgeable,
                instrumented,
                next_hint,
            });
        }
        let held_count = nodes.iter().filter(|n| n.held).count() as u64;
        let instruments = self.instruments.iter().filter(|(o, _, _)| *o == orb).count() as u64;
        let practices = self.practices.len() as u64;
        let wants = self.wants.iter().filter(|(o, _)| *o == orb).count() as u64;
        let next_move = if novelty_blocked {
            "novelty want — relieve via license leap or discovery before crystallize"
                .into()
        } else if forgeable_ready > 0 {
            format!(
                "forge ready on {} grade(s) — `bench research forge <c>` or `forge <c>`",
                forgeable_ready
            )
        } else if held_count == 0 {
            "tick through hardship / commission star / open research path".into()
        } else {
            "advance unheld nodes · adopt practice · elevate".into()
        };
        ResearchBoard {
            nodes,
            held_count,
            forgeable_ready,
            instruments,
            practices,
            wants,
            novelty_blocked,
            next_move,
        }
    }

    /// **INTEL GAME** — detection ladder + spy pipeline next step.
    pub fn intel_board(&self, orb: u64) -> IntelBoard {
        let e = self.espionage_integrity();
        let detection = self.detection_grade();
        let tech_conditions: Vec<u8> = self
            .techs
            .iter()
            .filter(|(o, _, _, _)| *o == orb)
            .map(|(_, c, _, _)| *c)
            .collect();
        let (next, next_hint) = if detection < 1 {
            (
                IntelStep::ForgeDetection,
                "forge grade ≥1 first — no probe without detection".into(),
            )
        } else if !self.spy_marks.iter().any(|(o, _)| *o == orb) {
            (
                IntelStep::Probe,
                "spy probe — pay crystal, leave scout mark".into(),
            )
        } else if e.can_exfil && tech_conditions.iter().any(|c| {
            self.techs
                .iter()
                .find(|(o, cond, _, _)| *o == orb && cond == c)
                .is_some_and(|(_, _, h, _)| !self.intel_hashes.contains(h))
        }) {
            (
                IntelStep::Exfil,
                "spy exfil <condition> — hash to stash (never free tech mint)".into(),
            )
        } else if e.can_sabotage {
            (
                IntelStep::Sabotage,
                "spy sabotage — remove lowest-grade instrument (typed residual)".into(),
            )
        } else {
            (
                IntelStep::Idle,
                "pipeline clear or blocked — raise forge / hold techs".into(),
            )
        };
        IntelBoard {
            detection,
            instruments: self.instrument_count(),
            probe_marks: e.spy_marks,
            intel_stash: e.intel_count,
            veiled: e.veiled,
            can_probe: e.can_probe,
            can_exfil: e.can_exfil,
            can_sabotage: e.can_sabotage,
            tech_conditions,
            next,
            next_hint,
        }
    }

    /// **LINK GAME** — cross-pollination readiness score (exact rational).
    pub fn link_board(&self) -> LinkBoard {
        let now = self.clock.now();
        let councils = self.councils.len();
        let council_open = self
            .councils
            .iter()
            .filter(|c| c.window_open(now))
            .count();
        let bonds = self.bonds.len();
        let forwards = self
            .forwards
            .iter()
            .filter(|f| {
                now < f.sealed.saturating_add(f.window) && f.delivered < f.quota
            })
            .count();
        let treaties = self.treaties.len();
        let at_war_count = self
            .treaties
            .iter()
            .filter(|t| t.kind == crate::treaty::TreatyKind::War)
            .count();
        let techs_licensable: Vec<u8> = self
            .techs
            .iter()
            .filter(|(_, c, _, _)| *c >= 1 && *c <= 6)
            .map(|(_, c, _, _)| *c)
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        // Score: each open council + bond + licensable tech; war subtracts.
        let mut score = ratio(0, 1);
        score += ratio(council_open as i64, 1);
        score += ratio(bonds as i64, 1);
        score += ratio(techs_licensable.len() as i64, 1);
        if self.crossed {
            score += ratio(1, 1);
        }
        score = if score > ratio(at_war_count as i64, 1) {
            score - ratio(at_war_count as i64, 1)
        } else {
            ratio(0, 1)
        };
        let next_hint = if at_war_count > 0 {
            "war walls trade — peace or renounce before license haul".into()
        } else if !self.crossed && techs_licensable.is_empty() {
            "earn solar leap / techs, then open council for licensed leap".into()
        } else if council_open == 0 && !techs_licensable.is_empty() {
            "seal council then `license <c> to <world> via <council> deal <n>`".into()
        } else if !techs_licensable.is_empty() && council_open > 0 {
            "license tech across the difference — Hail Mary confirmation".into()
        } else {
            "bond / forward / treaty trade to thicken the mesh".into()
        };
        LinkBoard {
            crossed: self.crossed,
            councils,
            council_open,
            bonds,
            forwards,
            treaties,
            at_war_count,
            techs_licensable,
            score,
            next_hint,
        }
    }

    /// **DOCTRINE GAME** — posture ranks as playable board.
    pub fn doctrine_board(&self) -> DoctrineBoard {
        let p = self.defense_posture();
        let weak = p
            .ranks
            .iter()
            .enumerate()
            .min_by_key(|(_, r)| *r)
            .map(|(i, _)| i)
            .unwrap_or(0);
        let domain = DefenseDomain::from_wire((weak + 1) as u8)
            .map(|d| d.word())
            .unwrap_or("?");
        let next_hint = if p.reoriented {
            format!("doctrine live — weakest domain {domain}; escort/market/forge to raise")
        } else {
            format!("posture set … to reorient; weakest rank domain {domain}")
        };
        DoctrineBoard {
            primary: p.primary.word().into(),
            secondary: p.secondary.word().into(),
            ranks: p.ranks,
            metabolism: p.metabolism,
            disposition: p.disposition,
            perception: p.perception,
            reoriented: p.reoriented,
            voice: p.voice.clone(),
            kin_voice: self.kin_residual_voice().into(),
            next_hint,
        }
    }
}

impl crate::world::state::Council {
    /// Open if window not expired (for link board).
    pub fn window_open(&self, now: u64) -> bool {
        // Council has sealed + window fields from state
        now < self.sealed.saturating_add(self.window)
    }
}
