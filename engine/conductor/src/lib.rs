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

impl BotWorld {
    /// Found the era: registry entry (join if present), the colony,
    /// the contained bot actor, and the steward entity.
    pub fn genesis(spec: BotSpec, registry: &Registry) -> BotWorld {
        let _ = registry.create(&spec.civilization);
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
    fn research_step(&mut self, mirror: &Mirror) {
        let Some(receipt) = self.world.chronicle().receipts().last() else {
            return;
        };
        let difficulty = Difficulty::level(self.level);
        let task = forge_task(receipt.own.0, difficulty);
        if let SolveRead::Solved {
            program,
            matches_expected: Some(true),
            certificate,
            ..
        } = solve(&task, difficulty.word_len)
        {
            let word: Vec<String> = program.iter().map(|op| op.describe()).collect();
            let mut cert_hex = String::new();
            for byte in certificate.output {
                cert_hex.push_str(&format!("{byte:02x}"));
            }
            let _ = mirror.record_solve(
                &self.spec.world_name,
                &task.name,
                &word.join(" → "),
                &cert_hex,
                self.level,
            );
            self.solved += 1;
            self.streak += 1;
            if self.streak >= 3 && self.level < 5 {
                self.level += 1;
                self.streak = 0;
            }
        } else {
            self.streak = 0;
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

        // The child's name, drawn from the digest.
        const HEADS: [&str; 6] = ["cryo", "rust", "ember", "shard", "echo", "drift"];
        const TAILS: [&str; 6] = ["born", "choir", "reach", "kin", "veil", "light"];
        let head = HEADS[beacon.uniform_index("speciation.head", 6) as usize];
        let tail = TAILS[beacon.uniform_index("speciation.tail", 6) as usize];
        let slug = format!("{head}{tail}");
        if registry.load(&slug).is_some() {
            return None; // the name is taken this era; the line waits
        }
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
        Some(BotSpec {
            civilization: daughter,
            world_name: format!("{}-{head}", self.spec.world_name),
            planet: format!("{tail}fall"),
            colony: format!("{slug}-landing"),
            policy: self.spec.policy,
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

/// Run one era: genesis for every bot, then `cycles` heartbeats;
/// daughters born mid-era join the beat. Returns the live worlds.
pub fn run_era(var_root: &str, cycles: u32) -> Vec<BotWorld> {
    let registry = Registry::at(format!("{var_root}/registry"));
    let mirror = Mirror::at(format!("{var_root}/worlds"));
    let mut worlds: Vec<BotWorld> = bots::founding_bots()
        .into_iter()
        .map(|spec| BotWorld::genesis(spec, &registry))
        .collect();
    for _ in 0..cycles {
        let mut newborns = Vec::new();
        for bot in &mut worlds {
            if let Some(spec) = bot.cycle(&mirror, &registry) {
                newborns.push(spec);
            }
        }
        for spec in newborns {
            worlds.push(BotWorld::genesis(spec, &registry));
        }
    }
    worlds
}
