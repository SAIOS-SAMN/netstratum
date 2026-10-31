//! **The workbench** (`43_DISTRIBUTION` §7, task #49) — the read-only
//! inspection surface that makes a deployed system legible. Ruled a
//! general, bench-type-agnostic abstraction: one shape over many bench
//! kinds, so the console renders any of them uniformly and a new bench
//! slots in without a new view. A workbench mints nothing and stores
//! nothing — it is a pure reading over committed state, the `stability_of`
//! precedent, replay-inert.
//!
//! The depot bench is the showcase (the whole provisioning/lineage layer
//! made visible: people, castes, the leisure good, stability, the arc
//! stage, the class-modulated ration); the refinery, ship-yard, and lab
//! report their component zones under the same shape.

use super::*;
use crate::dop::ZoneKind;

/// A deployable system the workbench can inspect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchKind {
    /// PROVISION-1: the granary and the society it feeds.
    Depot,
    /// REFINE-1: the element refinery.
    Refinery,
    /// FLEET-1: the ship-yard.
    ShipYard,
    /// The science seats.
    Lab,
    /// Taxation (49 §5): the civic engine — the fiscal loop under
    /// diplomacy (revenue policy, public services, civic infrastructure).
    Civic,
}

impl BenchKind {
    pub fn word(self) -> &'static str {
        match self {
            BenchKind::Depot => "depot",
            BenchKind::Refinery => "refinery",
            BenchKind::ShipYard => "ship-yard",
            BenchKind::Lab => "lab",
            BenchKind::Civic => "civic",
        }
    }
}

/// One component of a bench — a standing part with a health flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchComponent {
    pub label: String,
    pub active: bool,
}

/// A uniform inspection of one bench: its parts, its gauges (label →
/// value), and its unmet requirements. Every bench kind fills the same
/// shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkbenchView {
    pub kind: BenchKind,
    pub components: Vec<BenchComponent>,
    pub gauges: Vec<(String, String)>,
    pub alerts: Vec<String>,
}

impl World {
    /// **Inspect A bench** on an orb — the uniform read. `None` where the
    /// orb does not exist; an empty component list is a bench not yet
    /// deployed (a legible "nothing stands here", not an error).
    pub fn workbench(&self, orb: u64, kind: BenchKind) -> Option<WorkbenchView> {
        self.orb_by_id(orb)?;
        Some(match kind {
            BenchKind::Depot => self.depot_bench(orb),
            BenchKind::Refinery => self.zone_bench(orb, kind, ZoneKind::ElementRefinery),
            BenchKind::ShipYard => self.zone_bench(orb, kind, ZoneKind::ShipYard),
            BenchKind::Lab => self.lab_bench(orb, kind),
            BenchKind::Civic => self.civic_bench(orb),
        })
    }

    /// **The civic bench** (`49_TAXATION` §5) — the fiscal loop made
    /// legible under diplomacy, in the three panels the sovereign works:
    /// Revenue POLICY (the treasury, governance reach, the standing
    /// levies), public services (the welfare spend it funds, the
    /// stability it buys), and civic infrastructure (the offices and hubs
    /// that stand). A pure reading — deploying a zone or crafting a levy
    /// is the act; this only shows what stands. Empty everywhere for a
    /// line that has raised no civic zone.
    fn civic_bench(&self, orb: u64) -> WorkbenchView {
        // The civic zones that stand — the Infrastructure panel.
        let components = self
            .orb_by_id(orb)
            .map(|h| {
                h.zones
                    .iter()
                    .filter(|z| matches!(z.kind, ZoneKind::GovernmentOffice | ZoneKind::WelfareHub))
                    .map(|z| BenchComponent {
                        label: format!("{} @{}", crate::dop::zone_word(z.kind), z.tile),
                        active: h.zone_active(z),
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Project the flow readings over one nominal grain so the welfare
        // spend and stability are meaningful at rest (dt>0).
        let horizon = self.now() + crate::almanac::REGROWTH_GRAIN as u64;
        let continuity = self.continuity_levy();
        let estate = self.estate_levy();
        let gauges = vec![
            // — Revenue POLICY —
            (
                "treasury".into(),
                crate::ratio::ratio_str(&self.treasury_of(orb)),
            ),
            (
                "governance reach".into(),
                crate::ratio::ratio_str(&self.governance_reach(orb)),
            ),
            (
                "continuity tax".into(),
                crate::ratio::ratio_str(&self.continuity_tax()),
            ),
            ("continuity levy".into(), format!("{continuity:?}")),
            ("estate levy".into(), format!("{estate:?}")),
            // — Public services —
            (
                "welfare spend".into(),
                crate::ratio::ratio_str(&self.welfare_spend(orb, horizon)),
            ),
            (
                "stability".into(),
                crate::ratio::ratio_str(&self.stability_of(orb, horizon)),
            ),
        ];

        let mut alerts = Vec::new();
        if !self.has_government_office(orb) {
            alerts.push("no government office — levies slip uncollected (zero reach)".into());
        }
        if !self.has_welfare_hub(orb) {
            alerts.push("no welfare hub — the treasury funds no stability".into());
        } else if self.treasury_of(orb) <= ratio(0, 1) {
            alerts.push("treasury empty — welfare cannot pay the stall".into());
        }

        WorkbenchView {
            kind: BenchKind::Civic,
            components,
            gauges,
            alerts,
        }
    }

    /// The depot bench — the provisioning society, fully read.
    fn depot_bench(&self, orb: u64) -> WorkbenchView {
        let held = self.orb_by_id(orb);
        let components = held
            .map(|h| {
                h.zones
                    .iter()
                    .filter(|z| z.kind == ZoneKind::Granary)
                    .map(|z| BenchComponent {
                        label: format!("granary @{}", z.tile),
                        active: h.zone_active(z),
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Project the flow readings over one nominal grain so coverage and
        // stability are meaningful even at rest (dt>0).
        let horizon = self.now() + crate::almanac::REGROWTH_GRAIN as u64;
        let people = self.population_of(orb);
        // Castes are open — one bucket per disposition the vocabulary knows,
        // named by its own word, so a minted role shows here without a code
        // change.
        let disposition_word: std::collections::BTreeMap<u8, String> =
            crate::vocabulary::fold(self.chronicle(), crate::vocabulary::Axis::Disposition)
                .into_iter()
                .map(|entry| (entry.code, entry.word))
                .collect();
        let castes = self
            .caste_split(orb)
            .into_iter()
            .map(|(code, n)| {
                let name = disposition_word
                    .get(&code)
                    .cloned()
                    .unwrap_or_else(|| code.to_string());
                format!("{name} {n}")
            })
            .collect::<Vec<_>>()
            .join(", ");
        let stability = self.stability_of(orb, horizon);
        let mut gauges = vec![
            ("people".into(), people.to_string()),
            (
                "castes".into(),
                if castes.is_empty() {
                    "—".into()
                } else {
                    castes
                },
            ),
            (
                "eats/head".into(),
                crate::ratio::ratio_str(&self.per_capita_ration()),
            ),
            (
                "leisure good".into(),
                self.leisure_of(orb)
                    .map(|r| format!("{r:?}"))
                    .unwrap_or_else(|| "—".into()),
            ),
            ("stability".into(), crate::ratio::ratio_str(&stability)),
            ("arc stage".into(), self.evolution_stage().to_string()),
            (
                "ceiling K".into(),
                crate::ratio::ratio_str(&self.population_capacity(orb)),
            ),
            ("granaries".into(), self.granary_footprint(orb).to_string()),
        ];
        if self.flux_affine() {
            gauges.push((
                "flux output".into(),
                format!(
                    "{} (band {})",
                    crate::ratio::ratio_str(&self.flux_output_factor(orb)),
                    self.stellar_flux_of(orb)
                ),
            ));
        }
        if self.cooling_affine() {
            gauges.push((
                "cooling K".into(),
                crate::ratio::ratio_str(&self.cooling_k_factor(orb)),
            ));
        }
        if self.mobility_affine(orb) {
            gauges.push((
                "mobile footprint".into(),
                format!(
                    "grow {} · permanence {}",
                    crate::ratio::ratio_str(&self.footprint_growth_factor(orb)),
                    crate::ratio::ratio_str(&self.footprint_permanence_factor(orb))
                ),
            ));
        }
        if people > 0 {
            let cog = self.research_siphon(orb, people as i64);
            if cog > 0 {
                gauges.push(("research/cohort".into(), cog.to_string()));
            }
        }

        let mut alerts = Vec::new();
        if !self.has_granary(orb) {
            alerts.push("no granary — subsistence only (no leisure economy)".into());
        } else if stability < ratio(1, 1) {
            alerts.push("leisure short — growth STALLING before starvation".into());
        }
        if self.world_quarantined() {
            alerts.push("QUARANTINED — the ration is decaying, not recovering".into());
        }

        WorkbenchView {
            kind: BenchKind::Depot,
            components,
            gauges,
            alerts,
        }
    }

    /// A bench whose components are simply the standing zones of one kind
    /// (refinery, ship-yard) — the uniform minimal read.
    fn zone_bench(&self, orb: u64, kind: BenchKind, zone: ZoneKind) -> WorkbenchView {
        let (components, active) = self
            .orb_by_id(orb)
            .map(|h| {
                let comps: Vec<BenchComponent> = h
                    .zones
                    .iter()
                    .filter(|z| z.kind == zone)
                    .map(|z| BenchComponent {
                        label: format!("{} @{}", crate::dop::zone_word(zone), z.tile),
                        active: h.zone_active(z),
                    })
                    .collect();
                let active = comps.iter().filter(|c| c.active).count();
                (comps, active)
            })
            .unwrap_or_default();
        let gauges = vec![("active".into(), active.to_string())];
        let alerts = if components.is_empty() {
            vec![format!("no {} stands on this orb", kind.word())]
        } else {
            Vec::new()
        };
        WorkbenchView {
            kind,
            components,
            gauges,
            alerts,
        }
    }

    /// The lab bench — the seated science disciplines.
    fn lab_bench(&self, orb: u64, kind: BenchKind) -> WorkbenchView {
        let components: Vec<BenchComponent> = self
            .science_seats
            .iter()
            .filter(|(_, seat_orb, _)| *seat_orb == orb)
            .map(|(discipline, _, grade)| BenchComponent {
                label: format!("discipline {discipline} · grade {grade}"),
                active: true,
            })
            .collect();
        let gauges = vec![("seats".into(), components.len().to_string())];
        let alerts = if components.is_empty() {
            vec!["no science seated on this orb".into()]
        } else {
            Vec::new()
        };
        WorkbenchView {
            kind,
            components,
            gauges,
            alerts,
        }
    }
}
