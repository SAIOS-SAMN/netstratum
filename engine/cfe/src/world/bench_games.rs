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

/// One periodic line under the star (C1.7 stellar capacity game).
#[derive(Debug, Clone)]
pub struct StellarLineRow {
    /// Periodic line `1..=LINES`.
    pub line: u8,
    /// True when `line <= lines_of(star)`.
    pub reachable: bool,
    /// Pressure fraction if this line is the opened stratum depth.
    pub pressure_at: Ratio,
    /// How many open prospects mint an element on this line.
    pub open_veins: u64,
    /// How many banked element kinds sit on this line.
    pub banked_kinds: u64,
}

/// One opened prospect as capacity play.
#[derive(Debug, Clone)]
pub struct StellarVeinRow {
    pub stratum: u8,
    pub element: [u8; 32],
    pub line: u8,
    pub local: bool,
    pub refines_now: bool,
    pub richness: Ratio,
    pub name: String,
}

/// **C1.7 STELLAR CAPACITY GAME** — `lines_of` as playable board.
///
/// The star caps how deep the system forges. Prospects open strata;
/// locality + pressure decide which veins refine. Pure reading —
/// deeds stay `prospect` / `assay` / `refine` / `star`.
#[derive(Debug, Clone)]
pub struct StellarCapacityBoard {
    pub star: Option<crate::thermal::StarClass>,
    /// `lines_of(class)` — deepest line this sun forges.
    pub reach: u8,
    /// CIV-REACH-1 — deepest line this *people* may open (≤ star reach).
    pub civ_reach: u8,
    /// Table depth (`LINES`).
    pub table_lines: u8,
    pub lines: Vec<StellarLineRow>,
    pub veins: Vec<StellarVeinRow>,
    pub open_strata: u64,
    pub local_veins: u64,
    pub refinable_now: u64,
    pub deepest_open: u8,
    pub pressure_now: Ratio,
    pub next_move: String,
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

    /// **C1.7 STELLAR CAPACITY** — star lines as project (14 §8.4).
    ///
    /// Call-not-copy: [`crate::almanac::lines_of`], [`local_to`],
    /// [`pressure`], [`refines`] are the authorities.
    pub fn stellar_capacity_board(&self, orb: u64) -> StellarCapacityBoard {
        use crate::almanac::{lines_of, local_to, periodic_of, pressure, refines, LINES};
        use crate::dop::element_name;

        let star = self.star;
        let reach = star.map(lines_of).unwrap_or(0);
        let civ_reach = self.civ_reach();
        // Workable depth is the min of sky forge and civilization growth.
        let work_reach = reach.min(civ_reach);
        let deepest_open = self
            .prospects
            .iter()
            .filter(|p| p.orb == orb)
            .map(|p| p.stratum)
            .max()
            .unwrap_or(0);
        // Opened stratum depth as pressure input: clamp to work reach.
        let stratum_for_pressure = if work_reach == 0 {
            deepest_open
        } else {
            deepest_open.min(work_reach)
        };
        let pressure_now = star
            .map(|c| pressure(stratum_for_pressure, c))
            .unwrap_or_else(|| ratio(0, 1));

        // Vein inventory under this orb.
        let mut veins = Vec::new();
        let mut open_per_line = [0u64; 8]; // index by line 1..=7
        for p in self.prospects.iter().filter(|p| p.orb == orb) {
            let per = periodic_of(&p.element);
            let line = per.line;
            if (1..=7).contains(&line) {
                open_per_line[line as usize] =
                    open_per_line[line as usize].saturating_add(1);
            }
            let local = star.is_some_and(|c| local_to(&p.element, c));
            let refines_now = star.is_some_and(|c| refines(&p.element, stratum_for_pressure, c));
            let (family, _, _) = crate::dop::element_properties(&p.element, star.unwrap_or(crate::thermal::StarClass::G));
            let name = element_name(family, &p.element);
            veins.push(StellarVeinRow {
                stratum: p.stratum,
                element: p.element,
                line,
                local,
                refines_now,
                richness: p.richness.clone(),
                name,
            });
        }
        veins.sort_by_key(|v| (v.stratum, v.line));

        let mut banked_per_line = [0u64; 8];
        if let Some(held) = peek(&self.orbs, orb) {
            for (el, amt) in &held.elements {
                if *amt <= ratio(0, 1) {
                    continue;
                }
                let line = periodic_of(el).line;
                if (1..=7).contains(&line) {
                    banked_per_line[line as usize] =
                        banked_per_line[line as usize].saturating_add(1);
                }
            }
        }

        let mut lines = Vec::new();
        for line in 1..=LINES {
            // Reachable to work: sky forge AND civ growth.
            let reachable = work_reach > 0 && line <= work_reach;
            let pressure_at = star
                .map(|c| pressure(line, c))
                .unwrap_or_else(|| ratio(0, 1));
            lines.push(StellarLineRow {
                line,
                reachable,
                pressure_at,
                open_veins: open_per_line[line as usize],
                banked_kinds: banked_per_line[line as usize],
            });
        }

        let open_strata = veins.len() as u64;
        let local_veins = veins.iter().filter(|v| v.local).count() as u64;
        let refinable_now = veins.iter().filter(|v| v.refines_now).count() as u64;

        let next_move = if star.is_none() {
            "commission a star — `star <class>` then reopen capacity".into()
        } else if civ_reach < reach && open_strata == 0 {
            format!(
                "bronze civ_reach {civ_reach} (star forges {reach}) — prospect stratum 1..{civ_reach}; \
                 crystallize sky tech to mine deeper"
            )
        } else if open_strata == 0 {
            format!(
                "prospect stratum 1..{} — open the ground (civ {civ_reach} · star {reach})",
                work_reach.max(1)
            )
        } else if refinable_now > 0 {
            format!(
                "{refinable_now} vein(s) refine at pressure {} — assay · refine through ElementRefinery",
                format!("{}/{}", pressure_now.numer(), pressure_now.denom())
            )
        } else if deepest_open < work_reach {
            format!(
                "prospect deeper (next stratum {}) — work reach {work_reach} (civ {civ_reach} · star {reach})",
                deepest_open.saturating_add(1)
            )
        } else if civ_reach < reach {
            format!(
                "civ_reach {civ_reach} caps the mine (star forges {reach}) — crystallize base-7 tech to deepen"
            )
        } else if local_veins < open_strata {
            format!(
                "deep veins beyond star reach {reach} stay raw here — trade them or age/commission a heavier class"
            )
        } else {
            format!(
                "capacity saturated at work reach {work_reach} — refine local matter · trade off-depth · age star"
            )
        };

        StellarCapacityBoard {
            star,
            reach,
            civ_reach,
            table_lines: LINES,
            lines,
            veins,
            open_strata,
            local_veins,
            refinable_now,
            deepest_open,
            pressure_now,
            next_move,
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
