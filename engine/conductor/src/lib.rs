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
//! **LORE LIVES IN DATA, NOT SOURCE:** egregore specs load from
//! `<var>/egregores.json` when present; `write_default_egregores` seeds that
//! file once so even the founding NPCs are operator-editable. NPC
//! entries carry `founder: "conductor"` — the observer marks them ⚙.
//!
//! **THE AUTARKIES FEEL THE STAR (BOT-THERM):** genesis commissions
//! every egregore world's sun and sites its planet — spec-pinned, or
//! derived from the registry axes through the chronicle beacon (the
//! civilization's own genesis receipts choose its sky). Furnace-sited
//! egregores answer heat with encasement shells and mint joules;
//! Frost-sited egregores raise a cryo-lattice array, bank sink, and hold
//! expansion against the joule runway — honestly allowed to
//! freeze-lock; mesic egregores keep their temperament untouched.
//!
//! Everything is deterministic: same era length → byte-identical
//! receipts. HONEST SEAM: worlds live in process memory; reload from
//! the mirrored chronicle is the G0 persistence seam (coder-02).

use cfe::dop::{Resource, ZoneKind};
use cfe::ipsal::Scope;
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::World;
use mirror::galaxy::Galaxy;
use mirror::registry::{lineage_domain, Civilization, Registry, DISPOSITIONS, METABOLISMS, PERCEPTIONS};
use mirror::{Directive, Disposition, Mirror};
use physics::Beacon;
use solver::{forge_task, solve, Difficulty, SolveRead};

mod egregores;
pub use egregores::{founding_egregores, load_egregores, write_default_egregores};

/// AEON-1b (11 §5): THE FOLD — seal the live era and rotate it into
/// the archive sequence. `EraFolded` (tag 46) appends to the universe
/// ledger with the NEW base gauge (the next aeon seats on its own
/// G-sheet — placeholder advance: era/12, wrapped), then the live
/// `worlds/` and `registry/` move whole into `<base>-era<N>/` beside
/// the var root. Nothing is destroyed — the fold is archival by
/// construction; the next boot CONVERGES the sealed era, and the four
/// C's read which trajectories continue. The roster file STAYS: the
/// civilizations continue into the new aeon.
pub fn fold(var_root: &str) -> std::io::Result<(u64, Ratio)> {
    use cfe::vrp::Chronicle;
    let root = std::path::PathBuf::from(var_root);
    let base = root
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "var".into());
    let parent = match root.parent() {
        Some(p) if p.as_os_str().is_empty() => std::path::PathBuf::from("."),
        Some(p) => p.to_path_buf(),
        None => std::path::PathBuf::from("."),
    };
    // The next era ordinal: one past the highest archived.
    let mut era: u64 = 1;
    if let Ok(entries) = std::fs::read_dir(&parent) {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().into_owned();
            if let Some(n) = name.strip_prefix(&format!("{base}-era")) {
                if let Ok(held) = n.parse::<u64>() {
                    era = era.max(held + 1);
                }
            }
        }
    }
    let worlds = root.join("worlds");
    if !worlds.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no live worlds to fold",
        ));
    }
    // Seal: EraFolded onto the universe ledger, base G advanced.
    let ledger_path = worlds.join("convergence.tlv");
    let mut ledger = match cfe::persist::load(&ledger_path) {
        Ok(held) => held,
        Err(_) => Chronicle::new(),
    };
    let gauge = cfe::world::wrap_gauge(ratio(era as i64, 12));
    let stamp = ledger.receipts().iter().map(|r| r.stamp).max().unwrap_or(0);
    ledger.append(
        cfe::dop::ClosureEvent::EraFolded { era, gauge: gauge.clone() },
        stamp,
        vec![],
    );
    cfe::persist::sync(&ledger, &ledger_path)
        .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
    // Rotate: the archive sequence gains the sealed era, whole.
    let archive = parent.join(format!("{base}-era{era}"));
    std::fs::create_dir_all(&archive)?;
    std::fs::rename(&worlds, archive.join("worlds"))?;
    let registry = root.join("registry");
    if registry.is_dir() {
        std::fs::rename(&registry, archive.join("registry"))?;
    }
    let roster = root.join("egregores.json");
    if roster.is_file() {
        std::fs::copy(&roster, archive.join("bots.json"))?;
    }
    Ok((era, gauge))
}

/// A build temperament — the only thing that differs between egregores.
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
pub struct EgregoreSpec {
    pub civilization: Civilization,
    pub world_name: String,
    pub planet: String,
    pub colony: String,
    pub policy: Policy,
    /// The world's spatial coordinate S = (x, y, z) on the 3-torus.
    pub position: (Ratio, Ratio, Ratio),
    /// BOT-THERM: the world's sun, operator-pinned; `None` derives at
    /// genesis from the civilization's registry axes, beacon-drawn
    /// from its own genesis receipts (CHARTER §9 — never wall clock).
    pub star: Option<StarClass>,
    /// BOT-THERM: the orbit albedo, pinned; `None` derives (see `star`).
    pub albedo: Option<Ratio>,
    /// BOT-THERM: the orbit radius, pinned; `None` derives (see `star`).
    pub orbit: Option<Ratio>,
    /// AEON-1b: the world's opening gauge key, wrapped [0, 1); `None`
    /// founds at zero. A divergence branch carries its cleaved key.
    pub gauge: Option<Ratio>,
}

/// One live egregore world: the sim, its contained actor, its research rung.
pub struct Egregore {
    pub spec: EgregoreSpec,
    pub world: World,
    /// The egregore's contained agent (build/raze/collect granted).
    automaton: u64,
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

/// BOT-THERM: the Frost runway law — a sub-mesic colony expands only
/// when its banked joules cover the one-tick heating bill of the
/// GROWN city (ECON-J, 05 §1.2: `heating_per_zone × (zones + 1)`).
/// Exact rational comparison: meeting the bill exactly holds the
/// runway; one grain short refuses.
pub fn joule_runway_allows(banked_joules: &Ratio, zones: usize) -> bool {
    *banked_joules >= cfe::dop::heating_per_zone() * ratio(zones as i64 + 1, 1)
}

/// BOT-THERM: the thermal regime a egregore world lives in. The mesic band
/// is home (temperament unchanged — `None`); above its ceiling the
/// Furnace laws apply, below its floor the Frost laws. Unsited worlds
/// carry no regime (the engine's opt-in guard, mirrored here).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Regime {
    Furnace,
    Frost,
}

fn regime(world: &World) -> Option<(Regime, Ratio)> {
    let class = world.star()?;
    let site = world.orb()?.thermal.clone()?;
    let t4 = cfe::thermal::t_eq4(&class.luminosity(), &site.albedo, &site.orbit_r);
    let (mesic_lo, mesic_hi) = cfe::thermal::SubstrateBand::Mesic.bounds4();
    if t4 < mesic_lo {
        Some((Regime::Frost, t4))
    } else if t4 > mesic_hi {
        Some((Regime::Furnace, t4))
    } else {
        None
    }
}

/// BOT-THERM: the axis→regime preference (06 §1) for specs that pin no
/// siting — a mineral metabolism settles hot (the Furnace mints; 05
/// §3.1), a biotic one settles mesic. The exact seat inside the regime
/// is beacon-drawn from the civilization's OWN genesis receipts:
/// deterministic, replayable, never authored. A Frost seat is never
/// derived — settling the vault is a deliberate act (the operator's,
/// or the founding file's, which seeds one).
fn derived_siting(civilization: &Civilization, beacon: &Beacon) -> (StarClass, Ratio, Ratio) {
    match civilization.metabolism.as_str() {
        // A plasmic metabolism is of the stellar environs (12_UNION
        // §5a) — it shares the Furnace regime's seats; the mesic band
        // would starve a current sheet.
        "mineral" | "plasmic" => {
            // Furnace seats: a G sun at half orbit, stellar loads
            // 2, 11/5, 12/5 — every one past the mesic ceiling; the
            // encasement shells will answer.
            let albedos = [ratio(1, 2), ratio(9, 20), ratio(2, 5)];
            let drawn = beacon.uniform_index("genesis.furnace.albedo", 3) as usize;
            (StarClass::G, albedos[drawn].clone(), ratio(1, 2))
        }
        _ => {
            // Mesic seats: stellar loads 1, 9/10, 25/36 — every one
            // inside the mesic band (05 §2).
            let seats = [
                (ratio(0, 1), ratio(1, 1)),
                (ratio(1, 10), ratio(1, 1)),
                (ratio(0, 1), ratio(6, 5)),
            ];
            let drawn = beacon.uniform_index("genesis.mesic.seat", 3) as usize;
            (StarClass::G, seats[drawn].0.clone(), seats[drawn].1.clone())
        }
    }
}

impl Egregore {
    /// Found the era: registry entry (join if present), the colony,
    /// the contained egregore actor, the steward entity, and the world's
    /// place on the star-chart.
    pub fn genesis(spec: EgregoreSpec, registry: &Registry, mirror: &Mirror) -> Egregore {
        let _ = registry.create(&spec.civilization);
        let opening = spec
            .gauge
            .clone()
            .map(cfe::world::wrap_gauge)
            .unwrap_or_else(|| ratio(0, 1));
        let _ = mirror.chart_world(
            &spec.world_name,
            (&spec.position.0, &spec.position.1, &spec.position.2),
            // AEON-1b: a divergence branch charts at its cleaved key;
            // everyone else founds neutral and G moves by receipt.
            &opening,
        );
        let mut world = World::new();
        world.found(&spec.colony).expect("a fresh world founds");
        if opening != ratio(0, 1) {
            world.shift_gauge(opening.clone());
        }
        let automaton = world
            .register_agent(&format!("{}-conductor", spec.civilization.slug))
            .expect("registers");
        let steward = world.register_agent("steward").expect("registers");
        for scope in [Scope::Build, Scope::Raze, Scope::Collect] {
            world.grant(automaton, scope).expect("grants the automaton");
            world.grant(steward, scope).expect("grants the steward");
        }
        // BOT-THERM: thermal genesis — every egregore world founds under a
        // REAL sun. Spec-pinned fields rule; absent ones derive from
        // the registry axes, beacon-seeded by the world's own genesis
        // receipt digest (the chronicle-seeded discipline, CHARTER §9).
        let digest = world
            .chronicle()
            .receipts()
            .last()
            .expect("genesis receipts exist")
            .own
            .0;
        let beacon = Beacon::from_digest(digest);
        let (derived_star, derived_albedo, derived_orbit) =
            derived_siting(&spec.civilization, &beacon);
        let class = spec.star.unwrap_or(derived_star);
        let albedo = spec.albedo.clone().unwrap_or(derived_albedo);
        let orbit = spec.orbit.clone().unwrap_or(derived_orbit);
        world.commission_star(class).expect("a fresh world has no sun yet");
        // An unlawful operator-pinned siting (albedo ≥ 1, orbit ≤ 0)
        // leaves the orb thermally DORMANT (the engine's opt-in guard)
        // instead of killing the board; derived seats are always lawful.
        let _ = world.site_planet(cfe::world::FIRST_ORB, albedo, orbit);
        Egregore {
            spec,
            world,
            automaton,
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

        let settlement: Option<ZoneKind> = match self.spec.policy {
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
        };

        // BOT-THERM: thermal temperament. A Furnace-sited egregore answers
        // rising heat with encasement shells BEFORE anything else —
        // minted joules are what the regime is FOR (05 §1.2); a
        // Frost-sited egregore raises its sink charter (one cryo-lattice
        // array, 05 §1.4), then holds every further build against the
        // joule runway — famine avoidance, honestly allowed to fail
        // (a freeze-lock is legitimate drama, not a bug). Mesic-sited
        // and unsited egregores keep their temperament untouched.
        let affords = |world: &World, kind: ZoneKind| {
            kind.cost()
                .into_iter()
                .all(|(resource, need)| banked(world, resource) >= need)
        };
        let wanted: Option<ZoneKind> = match regime(world) {
            Some((Regime::Furnace, t4)) => {
                let shells = zone_count(world, ZoneKind::Encasement) as i64;
                // Famine-free here (guarded above), so every zone is
                // active and this factory sum matches the tick law's.
                let factory = world
                    .orb()
                    .map(|orb| {
                        orb.zones
                            .iter()
                            .fold(ratio(0, 1), |sum, zone| sum + zone.kind.factory_heat())
                    })
                    .unwrap_or_else(|| ratio(0, 1));
                let heat_net =
                    t4 / cfe::thermal::thermal_k() + factory - cfe::dop::radiator_base();
                let throughput = cfe::dop::encasement_throughput() * ratio(shells, 1);
                if heat_net > throughput {
                    // The shell answer: joule collection outranks
                    // sprawl — raise the next shell, or HOARD the ore
                    // toward it (expanding deeper into uncaptured heat
                    // only degrades every yield, 05 §1.2). Shells come
                    // from SURPLUS: the colony waits until the shell
                    // AND the next extractor are both funded, so one
                    // purchase cannot flip its compounding arm.
                    let iron_for = |kind: ZoneKind| {
                        kind.cost()
                            .into_iter()
                            .find(|(resource, _)| *resource == Resource::Iron)
                            .map(|(_, need)| need)
                            .unwrap_or_else(|| ratio(0, 1))
                    };
                    let shell_line =
                        iron_for(ZoneKind::Encasement) + iron_for(ZoneKind::Extractor);
                    (iron >= shell_line).then_some(ZoneKind::Encasement)
                } else if can_smelt
                    && smelters < 3
                    && iron >= ratio(30, 1)
                    && bio < ratio(10, 1)
                {
                    // Industry outranks new settlement on a Furnace
                    // seat: when ONLY the biomass line still blocks an
                    // unlocked smelter, hold the rations for it — a
                    // habitat spent now would push the forge past the
                    // horizon again.
                    industrial.or(match settlement {
                        Some(ZoneKind::Habitat) => None,
                        held => held,
                    })
                } else {
                    industrial.or(settlement)
                }
            }
            Some((Regime::Frost, _)) => {
                if zone_count(world, ZoneKind::CryoArray) == 0 {
                    // The sink charter: a Frost colony exists to bank
                    // the cold; its first array is founded on the
                    // founding iron, before any joule flows.
                    affords(world, ZoneKind::CryoArray).then_some(ZoneKind::CryoArray)
                } else if joule_runway_allows(&banked(world, Resource::Joule), total) {
                    industrial.or(settlement)
                } else {
                    None // the runway is short: STOP expanding
                }
            }
            None => industrial.or(settlement),
        };
        if let (Some(kind), Some(tile)) = (wanted, next_free_tile(world)) {
            let _ = world.zone_as(self.automaton, cfe::world::FIRST_ORB, tile, kind);
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

    /// DIVERGENCE (AEON-1b, 11 §3): at rung 5 with 20 confirmed
    /// solves — sustained pressure — the line CLEAVES into two PEER
    /// branches: no parent, no child. The origin's own key moves +delta
    /// and receipts `Diverged` naming the departing branch; the branch
    /// founds its own world at the opposite phase (−delta, wrapped).
    /// The mutation stays chronicle-seeded: the line's own history
    /// decides where its branches land on the manifold. The lineage
    /// pointer records LINE membership (LINEAGE-1), never parenthood.
    fn divergence_step(&mut self, registry: &Registry) -> Option<EgregoreSpec> {
        if self.spawned || self.level < 5 || self.solved < 20 {
            return None;
        }
        let receipt = self.world.chronicle().receipts().last()?;
        let beacon = Beacon::from_digest(receipt.own.0);
        let parent = &self.spec.civilization;
        // The cleave IS a key change (11 §3): delta in 1/12..=5/12,
        // beacon-drawn; the branches take opposite gauge phases.
        let delta = ratio(1 + beacon.uniform_index("diverge.delta", 5) as i64, 12);
        let origin_key = self.world.gauge().clone();

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
        // A PEER, not a child: the species is the LINE's species,
        // whole — no "(descendant)" suffix, no "child of" (11 §3
        // retired the daughter).
        let branch = Civilization {
            name,
            slug: slug.clone(),
            species: root_species.to_string(),
            motto: format!("one line, two songs — cleaved from {}", parent.name),
            disposition,
            metabolism,
            perception,
            founder: "conductor".into(),
            lineage: Some(parent.slug.clone()),
            station: None,
        };
        registry.create(&branch).ok()?;
        self.spawned = true;
        // The origin's key moves and the cleave is receipted, naming
        // the departing branch (tag 43).
        self.world.diverge(&slug, delta.clone());

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
        Some(EgregoreSpec {
            civilization: branch,
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
            // BOT-THERM: a branch's seat is never inherited — it is
            // drawn from its OWN genesis chronicle (the mutated axes
            // may prefer another regime entirely).
            star: None,
            albedo: None,
            orbit: None,
            // The opposite phase of the cleave (11 §3), wrapped.
            gauge: Some(cfe::world::wrap_gauge(origin_key - delta)),
        })
    }

    /// One full heartbeat. A speciation returns the newborn's spec —
    /// the era loop gives it a world of its own.
    pub fn cycle(&mut self, mirror: &Mirror, registry: &Registry) -> Option<EgregoreSpec> {
        self.drain_inbox(mirror);
        self.build_step();
        self.world.tick();
        let _ = self.world.collect_as(self.automaton, cfe::world::FIRST_ORB);
        self.research_step(mirror);
        let newborn = self.divergence_step(registry);
        let _ = mirror.record(
            &self.spec.world_name,
            &self.spec.planet,
            &self.spec.civilization.slug,
            &self.world,
        );
        newborn
    }
}

/// GALAXY-1b (13_GALAXY §3, §5): the year-voice observation sweep.
/// Every crossed, charted world OBSERVES; an anchor past the frontier
/// creates the galaxy it observes, executed as the progenitor's deed
/// (12 §2a — the board-maker function). Idempotent by geometry: a
/// created galaxy encloses its observer, so the next sweep reads
/// inside-reach and creates nothing.
pub fn observe_crossings(
    worlds: &[Egregore],
    registry: &Registry,
    mirror: &Mirror,
    stamp: u64,
) -> Vec<Galaxy> {
    let mut created = Vec::new();
    for egregore in worlds {
        if !egregore.world.has_crossed() {
            continue;
        }
        let Some(((x, y, z), _)) = mirror.charted_position(&egregore.spec.world_name) else {
            continue;
        };
        let creator = galaxy_creator(registry, &egregore.spec.civilization);
        if let Ok(Some(galaxy)) =
            mirror.observe_galaxy(&egregore.spec.civilization.slug, &creator, (&x, &y, &z), stamp)
        {
            created.push(galaxy);
        }
    }
    created
}

/// The creator seat (13_GALAXY §3): the observer's lineage head when
/// it holds a progenitor seat; else the first seated progenitor of the
/// observer's domain (the founding head stands creator for a headless
/// line — lawful, 12 §5); else the conductor — tier 0's own operations
/// ARE the progenitors' deeds, named (12 §2a).
fn galaxy_creator(registry: &Registry, civilization: &Civilization) -> String {
    if let Some(head) = registry.resolve_head(&civilization.slug) {
        if registry.load(&head).and_then(|c| c.station).as_deref() == Some("progenitor") {
            return head;
        }
    }
    let domain = lineage_domain(&civilization.metabolism, None);
    registry
        .list()
        .into_iter()
        .find(|seated| {
            seated.station.as_deref() == Some("progenitor")
                && lineage_domain(&seated.metabolism, None) == domain
        })
        .map(|seated| seated.slug)
        .unwrap_or_else(|| "conductor".into())
}

/// Run one era: CONVERGE the archived eras (07 §1.6), then genesis for
/// every egregore, then `cycles` heartbeats; daughters born mid-era join the
/// beat. Returns the live worlds.
pub fn run_era(var_root: &str, cycles: u32) -> Vec<Egregore> {
    let registry = Registry::at(format!("{var_root}/registry"));
    let mirror = Mirror::at(format!("{var_root}/worlds"));
    let eras = mirror::converge::default_era_roots(var_root);
    let epoch_base = mirror::converge::converge(&mirror, &eras)
        .map(|read| read.epoch_base)
        .unwrap_or(0);
    // 11 §5.2: the live aeon's G-sheet (see main.rs — same law).
    let live_base = cfe::world::wrap_gauge(ratio(eras.len() as i64, 12));
    let mut worlds: Vec<Egregore> = egregores::founding_egregores()
        .into_iter()
        .map(|mut spec| {
            let opening = spec.gauge.take().unwrap_or_else(|| ratio(0, 1));
            spec.gauge = Some(cfe::world::wrap_gauge(opening + live_base.clone()));
            Egregore::genesis(spec, &registry, &mirror)
        })
        .collect();
    for beat in 0..cycles {
        // TUniversal T = epoch base + this era's beat (07 §1.3).
        let _ = mirror.set_universal_clock(epoch_base + u64::from(beat) + 1);
        let mut newborns = Vec::new();
        for egregore in &mut worlds {
            if let Some(spec) = egregore.cycle(&mirror, &registry) {
                newborns.push(spec);
            }
        }
        for spec in newborns {
            worlds.push(Egregore::genesis(spec, &registry, &mirror));
        }
        // JOURNAL-1 (08 §3): on year boundaries every civilization
        // speaks — the mirror renders journals + wall, a REFLECTION
        // of the chronicle, never a state change.
        if (u64::from(beat) + 1).is_multiple_of(mirror::journal::YEAR_TICKS) {
            for egregore in &worlds {
                let _ = mirror.record_journal(
                    &egregore.spec.world_name,
                    &egregore.world,
                    &mirror::journal::Voice::of(&egregore.spec.civilization),
                );
            }
            // GALAXY-1b: the observation sweep rides the year voice
            // (11 §1 — board-scale shifts read yearly).
            let _ = observe_crossings(&worlds, &registry, &mirror, epoch_base + u64::from(beat) + 1);
        }
    }
    worlds
}
