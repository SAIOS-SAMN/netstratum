//! N-CONDUCTOR + N-LINEAGE — the heartbeat and the family tree.
//!
//! Scripted civilizations play their colonies autonomously, each
//! through a CONTAINED AGENT (NS-6: granted scopes, every receipt
//! attributed `by#agent` — automation on a leash). Each heartbeat a
//! world: drains the steward inbox, plays its policy, ticks,
//! collects, grinds the forge (its OWN latest receipt digest seeds
//! the puzzle — CHARTER §9), and mirrors everything.
//!
//! **TECH IS EARNED (the pyramid loop closed):** smelters unlock at
//! forge rung 3, synthesizers at rung 5 — for every temperament.
//! Research depth is the tech tree; surplus funds search; search
//! unlocks industry; industry funds deeper search.
//!
//! **SPECIATION IS A MINT:** a civilization at rung 5 with 20
//! confirmed solves spawns ONE daughter per era — irreversible
//! dimensional growth, exactly `physics::FlowEvent::Mint`'s law worn
//! as gameplay. The mutation is chronicle-seeded: the parent's latest
//! receipt digest draws WHICH axis mutates and what the child is
//! called — the parent's own history decides what its children
//! become. Lineage is registry-recorded now, receipted at G1. A
//! daughter founds a NEW world (binding #1: one chronicle per world —
//! a shared-chronicle sibling planet waits on the multi-colony
//! engine).
//!
//! **LORE LIVES IN DATA, NOT SOURCE:** bot specs load from
//! `<var>/bots.json` when present; `write_default_bots` seeds that
//! file once so even the founding NPCs are operator-editable. NPC
//! entries carry `founder: "conductor"` — the observer marks them ⚙.
//!
//! Everything is deterministic: same era length → byte-identical
//! receipts. HONEST SEAM: worlds live in process memory; reload from
//! the mirrored chronicle is the G0 persistence seam (coder-02).

use cfe::dop::{Resource, ZoneKind};
use cfe::ipsal::Scope;
use cfe::ratio::{ratio, Ratio};
use cfe::world::World;
use mirror::registry::{Civilization, Registry, DISPOSITIONS, METABOLISMS, PERCEPTIONS};
use mirror::{Directive, Disposition, Mirror};
use physics::Beacon;
use solver::{forge_task, solve, Difficulty, SolveRead};

mod bots;
pub use bots::{founding_bots, load_bots, write_default_bots};

/// A build temperament — the only thing that differs between bots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    /// Two extractors per habitat, expand at the food knife-edge.
    Aggressive,
    /// One-to-one, build only with a full cost buffer banked.
    Steady,
    /// Extractor-heavy early, first in line for unlocked industry.
    Industry,
}

impl Policy {
    pub fn parse(word: &str) -> Option<Policy> {
        match word {
            "aggressive" => Some(Policy::Aggressive),
            "steady" => Some(Policy::Steady),
            "industry" => Some(Policy::Industry),
            _ => None,
        }
    }

    pub fn describe(&self) -> &'static str {
        match self {
            Policy::Aggressive => "aggressive",
            Policy::Steady => "steady",
            Policy::Industry => "industry",
        }
    }
}

/// One scripted civilization and where it settles.
#[derive(Debug, Clone)]
pub struct BotSpec {
    pub civilization: Civilization,
    pub world_name: String,
    pub planet: String,
    pub colony: String,
    pub policy: Policy,
    /// The world's spatial coordinate S = (x, y, z) on the 3-torus.
    pub position: (Ratio, Ratio, Ratio),
}

/// One live bot world: the sim, its contained actor, its research rung.
pub struct BotWorld {
    pub spec: BotSpec,
    pub world: World,
    /// The bot's contained agent (build/raze/collect granted).
    actor: u64,
    /// The steward's contained entity (drains the operator's inbox).
    steward: u64,
    /// The forge rung this civilization currently attempts.
    pub level: u32,
    /// Confirms at the current rung (3 promote).
    streak: u32,
    /// Confirmed solves this era.
    pub solved: u32,
    /// One daughter per era — speciation is rare, not a flood.
    spawned: bool,
}

fn banked(world: &World, resource: Resource) -> Ratio {
    world
        .orb()
        .and_then(|orb| {
            orb.banked
                .iter()
                .find(|(r, _)| *r == resource)
                .map(|(_, amount)| amount.clone())
        })
        .unwrap_or_else(|| ratio(0, 1))
}

fn zone_count(world: &World, kind: ZoneKind) -> usize {
    world
        .orb()
        .map(|orb| orb.zones.iter().filter(|z| z.kind == kind).count())
        .unwrap_or(0)
}

fn next_free_tile(world: &World) -> Option<u16> {
    let orb = world.orb()?;
    (0u16..64).find(|tile| !orb.zones.iter().any(|z| z.tile == *tile))
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

impl BotWorld {
    /// Found the era: registry entry (join if present), the colony,
    /// the contained bot actor, the steward entity, and the world's
    /// place on the star-chart.
    pub fn genesis(spec: BotSpec, registry: &Registry, mirror: &Mirror) -> BotWorld {
        let _ = registry.create(&spec.civilization);
        let _ = mirror.chart_world(
            &spec.world_name,
            (&spec.position.0, &spec.position.1, &spec.position.2),
            &ratio(0, 1), // gauge phase: neutral at genesis; G moves with the fiber landings
        );
        let mut world = World::new();
        world.found(&spec.colony).expect("a fresh world founds");
        let actor = world
            .register_agent(&format!("{}-conductor", spec.civilization.slug))
            .expect("registers");
        let steward = world.register_agent("steward").expect("registers");
        for scope in [Scope::Build, Scope::Raze, Scope::Collect] {
            world.grant(actor, scope).expect("grants the bot");
            world.grant(steward, scope).expect("grants the steward");
        }
        BotWorld {
            spec,
            world,
            actor,
            steward,
            level: 1,
            streak: 0,
            solved: 0,
            spawned: false,
        }
    }

    /// The policy's build decision — at most one zone per beat, never
    /// during famine, always through the contained actor. Industry is
    /// GATED ON RESEARCH for every temperament: smelters need rung 3,
    /// synthesizers rung 5.
    fn build_step(&mut self) {
        let world = &mut self.world;
        if world.orb().is_none_or(|orb| !orb.famine.is_empty()) {
            return; // starving colonies build nothing
        }
        let (ext, hab) = (
            zone_count(world, ZoneKind::Extractor),
            zone_count(world, ZoneKind::Habitat),
        );
        let iron = banked(world, Resource::Iron);
        let bio = banked(world, Resource::Biomass);
        let alloy = banked(world, Resource::Alloy);
        let total = world.orb().map(|o| o.zones.len()).unwrap_or(0);
        let smelters = zone_count(world, ZoneKind::Smelter);

        // The earned tech tree.
        let can_smelt = self.level >= 3;
        let can_synth = self.level >= 5;

        // Unlocked industry takes priority for every temperament —
        // that is what research is FOR.
        let industrial: Option<ZoneKind> = if can_synth
            && alloy >= ratio(20, 1)
            && bio >= ratio(15, 1)
        {
            Some(ZoneKind::Synthesizer)
        } else if can_smelt && smelters < 3 && iron >= ratio(30, 1) && bio >= ratio(10, 1) {
            Some(ZoneKind::Smelter)
        } else {
            None
        };

        let wanted: Option<ZoneKind> = industrial.or(match self.spec.policy {
            Policy::Aggressive => {
                // One extractor SHORT of the knife-edge: a standing
                // +1/tick biomass surplus, so industry stays fundable.
                if total >= 16 {
                    None
                } else if ext + 1 < 2 * hab.max(1) && iron >= ratio(10, 1) {
                    Some(ZoneKind::Extractor)
                } else if bio >= ratio(5, 1) {
                    Some(ZoneKind::Habitat)
                } else {
                    None
                }
            }
            Policy::Steady => {
                if total >= 12 {
                    None
                } else if ext <= hab && iron >= ratio(20, 1) {
                    Some(ZoneKind::Extractor)
                } else if hab < ext && bio >= ratio(10, 1) {
                    Some(ZoneKind::Habitat)
                } else {
                    None
                }
            }
            Policy::Industry => {
                if total >= 20 {
                    None
                } else if ext < 2 * hab.max(1) && iron >= ratio(25, 1) {
                    Some(ZoneKind::Extractor)
                } else if bio >= ratio(10, 1) {
                    Some(ZoneKind::Habitat)
                } else {
                    None
                }
            }
        });
        if let (Some(kind), Some(tile)) = (wanted, next_free_tile(world)) {
            let _ = world.zone_as(self.actor, cfe::world::FIRST_ORB, tile, kind);
        }
    }

    /// Drain the operator's inbox: each directive applies as the
    /// steward entity (scope-checked in-engine) or refuses; archived
    /// either way, never deleted.
    fn drain_inbox(&mut self, mirror: &Mirror) {
        let Ok(pending) = mirror.pending_directives(&self.spec.world_name) else {
            return;
        };
        for item in pending {
            let applied = self.apply_directive(&item.directive);
            let _ = mirror.archive(
                &item,
                if applied {
                    Disposition::Applied
                } else {
                    Disposition::Refused
                },
            );
        }
    }

    fn apply_directive(&mut self, directive: &Directive) -> bool {
        let parse_tile = |word: &str| -> Option<u16> {
            if let Some((r, c)) = word.split_once(':') {
                let (r, c): (u16, u16) = (r.parse().ok()?, c.parse().ok()?);
                (r < 8 && c < 8).then_some(r * 8 + c)
            } else {
                word.parse().ok().filter(|t| *t < 64)
            }
        };
        let parse_kind = |word: &str| match word {
            "ext" => Some(ZoneKind::Extractor),
            "hab" => Some(ZoneKind::Habitat),
            "smelt" => Some(ZoneKind::Smelter),
            "synth" => Some(ZoneKind::Synthesizer),
            _ => None,
        };
        match (directive.verb.as_str(), directive.args.as_slice()) {
            ("zone", [tile, kind]) => match (parse_tile(tile), parse_kind(kind)) {
                (Some(tile), Some(kind)) => self.world.zone_as(self.steward, cfe::world::FIRST_ORB, tile, kind).is_ok(),
                _ => false,
            },
            ("raze", [tile]) => match parse_tile(tile) {
                Some(tile) => self.world.raze_as(self.steward, cfe::world::FIRST_ORB, tile).is_ok(),
                None => false,
            },
            ("collect", []) => self.world.collect_as(self.steward, cfe::world::FIRST_ORB).is_ok(),
            _ => false,
        }
    }

    /// The research arm: the world's own latest receipt digest seeds
    /// one forge attempt at the current rung. Three confirms promote.
    ///
    /// PERCEPTION ROUTES THE RESEARCH (the axis has teeth): a
    /// volumetric civilization natively attempts VOLUMETRIC tasks —
    /// stacked grids, stack operators — while planar and harmonic
    /// civilizations work the planar forge (the harmonic carrier is a
    /// named gap; it lands with its own vocabulary, not as a skin).
    fn research_step(&mut self, mirror: &Mirror) {
        let Some(receipt) = self.world.chronicle().receipts().last() else {
            return;
        };
        let digest = receipt.own.0;
        let solved = if self.spec.civilization.perception == "volumetric" {
            let difficulty = solver::VolDifficulty::level(self.level);
            let task = solver::forge_volume(digest, difficulty);
            match solver::solve_volume(&task, difficulty.word_len) {
                solver::VolumeSolveRead::Solved {
                    program,
                    matches_expected: Some(true),
                    certificate,
                    ..
                } => {
                    let word: Vec<String> = program.iter().map(|op| op.describe()).collect();
                    Some((task.name, word.join(" → "), hex(&certificate)))
                }
                _ => None,
            }
        } else {
            let difficulty = Difficulty::level(self.level);
            let task = forge_task(digest, difficulty);
            match solve(&task, difficulty.word_len) {
                SolveRead::Solved {
                    program,
                    matches_expected: Some(true),
                    certificate,
                    ..
                } => {
                    let word: Vec<String> = program.iter().map(|op| op.describe()).collect();
                    Some((task.name, word.join(" → "), hex(&certificate.output)))
                }
                _ => None,
            }
        };
        match solved {
            Some((name, word, cert_hex)) => {
                let _ = mirror.record_solve(
                    &self.spec.world_name,
                    &name,
                    &word,
                    &cert_hex,
                    self.level,
                );
                self.solved += 1;
                self.streak += 1;
                if self.streak >= 3 && self.level < 5 {
                    self.level += 1;
                    self.streak = 0;
                }
            }
            None => self.streak = 0,
        }
    }

    /// SPECIATION: at rung 5 with 20 confirmed solves, the parent's
    /// latest receipt digest mints ONE daughter — a mutated axis, a
    /// derived name, a new world, the parent's temperament. The
    /// history chooses; nobody authors the child.
    fn speciation_step(&mut self, registry: &Registry) -> Option<BotSpec> {
        if self.spawned || self.level < 5 || self.solved < 20 {
            return None;
        }
        let receipt = self.world.chronicle().receipts().last()?;
        let beacon = Beacon::from_digest(receipt.own.0);
        let parent = &self.spec.civilization;

        // Which axis mutates, and to what (never to itself).
        let mutate = |current: &str, options: &[&str], domain: &str| -> String {
            let others: Vec<&&str> = options.iter().filter(|o| **o != current).collect();
            others[beacon.uniform_index(domain, others.len() as u64) as usize].to_string()
        };
        let axis = beacon.uniform_index("speciation.axis", 3);
        let (mut disposition, mut metabolism, mut perception) = (
            parent.disposition.clone(),
            parent.metabolism.clone(),
            parent.perception.clone(),
        );
        match axis {
            0 => disposition = mutate(&disposition, &DISPOSITIONS, "speciation.disposition"),
            1 => metabolism = mutate(&metabolism, &METABOLISMS, "speciation.metabolism"),
            _ => perception = mutate(&perception, &PERCEPTIONS, "speciation.perception"),
        }

        // THEMED NAMING (operator ruling): the child's name derives
        // from the parent's dominant CAPABILITIES — metabolism names
        // the head, perception names the tail. Fresh two-part names,
        // never concatenations of ancestry (lineage lives in the
        // registry field, not in ever-growing strings).
        let heads: &[&str] = match parent.metabolism.as_str() {
            "mineral" => &["ember", "slag", "basalt", "vein", "cinder", "forge"],
            _ => &["bloom", "grove", "reef", "moss", "fern", "spore"],
        };
        let tails: &[&str] = match perception.as_str() {
            "harmonic" => &["choir", "chord", "hum", "call"],
            "volumetric" => &["vault", "deep", "well", "hollow"],
            _ => &["reach", "field", "span", "march"],
        };
        // Deterministic collision walk: start at the drawn combo,
        // scan forward; if every themed name is taken, the line waits.
        let start = beacon.uniform_index("speciation.name", (heads.len() * tails.len()) as u64)
            as usize;
        let mut chosen: Option<(String, &str, &str)> = None;
        for offset in 0..heads.len() * tails.len() {
            let k = (start + offset) % (heads.len() * tails.len());
            let (head, tail) = (heads[k / tails.len()], tails[k % tails.len()]);
            let slug = format!("{head}{tail}");
            if registry.load(&slug).is_none() {
                chosen = Some((slug, head, tail));
                break;
            }
        }
        let (slug, head, tail) = chosen?;
        let mut name = slug.clone();
        name[..1].make_ascii_uppercase();

        let root_species = parent
            .species
            .strip_suffix(" (descendant)")
            .unwrap_or(&parent.species);
        let daughter = Civilization {
            name,
            slug: slug.clone(),
            species: format!("{root_species} (descendant)"),
            motto: format!("child of {}", parent.name),
            disposition,
            metabolism,
            perception,
            founder: "conductor".into(),
            lineage: Some(parent.slug.clone()),
        };
        registry.create(&daughter).ok()?;
        self.spawned = true;

        // The daughter settles ADJACENT: parent position plus a small
        // beacon-drawn displacement in the exact rationals per spatial axis (half-unit
        // grain, never the zero vector).
        let step = |domain: &str| -> Ratio {
            ratio(beacon.uniform_index(domain, 9) as i64 - 4, 2)
        };
        let (mut dx, dy, dz) = (step("chart.dx"), step("chart.dy"), step("chart.dz"));
        if dx == ratio(0, 1) && dy == ratio(0, 1) && dz == ratio(0, 1) {
            dx = ratio(1, 1);
        }
        Some(BotSpec {
            civilization: daughter,
            // Fresh themed world and planet — ancestry is a registry
            // fact and a star-chart adjacency, never a name chain.
            world_name: format!("{head}-{tail}"),
            planet: format!("{tail}fall"),
            colony: format!("{slug}-landing"),
            policy: self.spec.policy,
            position: (
                &self.spec.position.0 + dx,
                &self.spec.position.1 + dy,
                &self.spec.position.2 + dz,
            ),
        })
    }

    /// One full heartbeat. A speciation returns the newborn's spec —
    /// the era loop gives it a world of its own.
    pub fn cycle(&mut self, mirror: &Mirror, registry: &Registry) -> Option<BotSpec> {
        self.drain_inbox(mirror);
        self.build_step();
        self.world.tick();
        let _ = self.world.collect_as(self.actor, cfe::world::FIRST_ORB);
        self.research_step(mirror);
        let newborn = self.speciation_step(registry);
        let _ = mirror.record(
            &self.spec.world_name,
            &self.spec.planet,
            &self.spec.civilization.slug,
            &self.world,
        );
        newborn
    }
}

/// Run one era: CONVERGE the archived eras (07 §1.6), then genesis for
/// every bot, then `cycles` heartbeats; daughters born mid-era join the
/// beat. Returns the live worlds.
pub fn run_era(var_root: &str, cycles: u32) -> Vec<BotWorld> {
    let registry = Registry::at(format!("{var_root}/registry"));
    let mirror = Mirror::at(format!("{var_root}/worlds"));
    let eras = mirror::converge::default_era_roots(var_root);
    let epoch_base = mirror::converge::converge(&mirror, &eras)
        .map(|read| read.epoch_base)
        .unwrap_or(0);
    let mut worlds: Vec<BotWorld> = bots::founding_bots()
        .into_iter()
        .map(|spec| BotWorld::genesis(spec, &registry, &mirror))
        .collect();
    for beat in 0..cycles {
        // TUniversal T = epoch base + this era's beat (07 §1.3).
        let _ = mirror.set_universal_clock(epoch_base + u64::from(beat) + 1);
        let mut newborns = Vec::new();
        for bot in &mut worlds {
            if let Some(spec) = bot.cycle(&mirror, &registry) {
                newborns.push(spec);
            }
        }
        for spec in newborns {
            worlds.push(BotWorld::genesis(spec, &registry, &mirror));
        }
        // JOURNAL-1 (08 §3): on year boundaries every civilization
        // speaks — the mirror renders journals + wall, a REFLECTION
        // of the chronicle, never a state change.
        if (u64::from(beat) + 1).is_multiple_of(mirror::journal::YEAR_TICKS) {
            for bot in &worlds {
                let _ = mirror.record_journal(
                    &bot.spec.world_name,
                    &bot.world,
                    &mirror::journal::Voice::of(&bot.spec.civilization),
                );
            }
        }
    }
    worlds
}
