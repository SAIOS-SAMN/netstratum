//! The Netstratum CLI — the first slice's face. One command launches
//! it (CHARTER §3.5); every completed action closes as an NS-3 receipt;
//! `verify` and `replay` read the chain the way any peer would.
//!
//! Presentation only lives here: the map grid, colors, and friendly
//! refusal text are the client's voice — the engine's typed refusals
//! and exact ratios are the truth underneath, always shown as exact
//! fractions (the C2R rule: "3/2", never 1.5).

use std::io::{self, BufRead, Write};

use cfe::dop::{ColonyOrb, Resource, ZoneKind};
use cfe::ipsal::Scope;
use cfe::ratio::Ratio;
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World, SOVEREIGN};

/// The slice's world is an 8×8 tile grid; tile = row*8 + col.
const GRID: u16 = 8;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let plain = args.iter().any(|arg| arg == "--plain") || std::env::var_os("NO_COLOR").is_some();
    let script = args
        .iter()
        .position(|arg| arg == "--script")
        .and_then(|at| args.get(at + 1))
        .cloned();
    let style = Style { plain };
    let mut world = World::new();
    let mut current: u64 = cfe::world::FIRST_ORB;
    banner(&style);
    match script {
        Some(script) => {
            for line in script.split(';') {
                let line = line.trim();
                if !line.is_empty() {
                    println!("{} {line}", style.dim("»"));
                    command(&mut world, line, &style, &mut current);
                }
            }
        }
        None => {
            let stdin = io::stdin();
            loop {
                print!("{} ", style.prompt("netstratum»"));
                io::stdout().flush().ok();
                let mut line = String::new();
                match stdin.lock().read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        let line = line.trim().to_string();
                        match line.as_str() {
                            "quit" | "exit" => break,
                            "" => {}
                            _ => command(&mut world, &line, &style, &mut current),
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
}

struct Style {
    plain: bool,
}

impl Style {
    fn wrap(&self, code: &str, text: &str) -> String {
        match self.plain {
            true => text.to_string(),
            false => format!("\x1b[{code}m{text}\x1b[0m"),
        }
    }
    fn ok(&self, text: &str) -> String {
        self.wrap("1;32", text) // bold green — a sealed closure
    }
    fn warn(&self, text: &str) -> String {
        self.wrap("1;33", text) // bold yellow — workspace / caution
    }
    fn bad(&self, text: &str) -> String {
        self.wrap("1;31", text) // bold red — a refusal
    }
    fn head(&self, text: &str) -> String {
        self.wrap("1;36", text) // bold cyan — section headers
    }
    fn strong(&self, text: &str) -> String {
        self.wrap("1", text) // bold — numbers the eye should catch
    }
    fn dim(&self, text: &str) -> String {
        self.wrap("2", text) // dim — structure, hashes, hints
    }
    fn prompt(&self, text: &str) -> String {
        self.wrap("1;35", text)
    }
}

fn banner(style: &Style) {
    println!("{}", style.head("╔══════════════════════════════════════════════╗"));
    println!("{}", style.head("║  NETSTRATUM — the first slice                ║"));
    println!("{}", style.head("╚══════════════════════════════════════════════╝"));
    println!(
        "Start with {} then {} — type {} for everything.",
        style.strong("found <name>"),
        style.strong("zone <tile|r:c> <ext|hab>"),
        style.strong("help")
    );
}

fn command(world: &mut World, line: &str, style: &Style, current: &mut u64) {
    let mut words = line.split_whitespace();
    match words.next() {
        Some("help") => help(style),
        Some("found") => {
            let name = words.collect::<Vec<_>>().join(" ");
            match name.is_empty() {
                true => println!(
                    "{} a colony needs a name — try: {}",
                    style.bad("✗"),
                    style.strong("found meridian")
                ),
                false => match world.found(&name) {
                    Ok(()) => {
                        println!(
                            "{} colony {} founded — receipt {} sealed",
                            style.ok("✓"),
                            style.strong(&name),
                            style.dim("#0")
                        );
                        map(world, style, *current);
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                },
            }
        }
        Some("zone") => {
            let tile = words.next().and_then(parse_tile);
            let kind = match words.next() {
                Some("ext") | Some("extractor") => Some(ZoneKind::Extractor),
                Some("hab") | Some("habitat") => Some(ZoneKind::Habitat),
                Some("smelt") | Some("smelter") => Some(ZoneKind::Smelter),
                Some("synth") | Some("synthesizer") => Some(ZoneKind::Synthesizer),
                _ => None,
            };
            match (tile, kind) {
                (Some(tile), Some(kind)) if tile < GRID * GRID => match world.zone_as(SOVEREIGN, *current, tile, kind) {
                    Ok(()) => {
                        let sequence = world.chronicle().receipts().len() - 1;
                        println!(
                            "{} tile {} ({}) zoned {} — receipt {} sealed",
                            style.ok("✓"),
                            style.strong(&tile.to_string()),
                            coordinate(tile),
                            style.strong(kind_name(kind)),
                            style.dim(&format!("#{sequence}"))
                        );
                        map(world, style, *current);
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                },
                (Some(tile), Some(_)) => println!(
                    "{} tile {tile} is off the map — the grid is {GRID}×{GRID} (tiles 0–{}, or row:col up to 7:7)",
                    style.bad("✗"),
                    GRID * GRID - 1
                ),
                _ => println!(
                    "{} usage: {} — e.g. {} or {}",
                    style.bad("✗"),
                    style.strong("zone <tile|row:col> <ext|hab|smelt|synth>"),
                    style.strong("zone 12 ext"),
                    style.strong("zone 1:4 smelt")
                ),
            }
        }
        Some("tick") => {
            let count = words
                .next()
                .and_then(|word| word.parse::<u64>().ok())
                .unwrap_or(1);
            for _ in 0..count {
                let outcome = world.tick();
                for (orb, resource) in outcome.struck {
                    let planet = world
                        .orb_by_id(orb)
                        .map(|held| held.name.clone())
                        .unwrap_or_else(|| orb.to_string());
                    println!(
                        "{} {} on {} — the {} stores ran dry; its consumers fall DORMANT (raze to cut the drain, collect to restock)",
                        style.bad("☠"),
                        style.bad("FAMINE"),
                        style.strong(&planet),
                        style.strong(resource_name(resource))
                    );
                }
            }
            println!(
                "{} {} tick(s) pass — clock now {}",
                style.ok("✓"),
                style.strong(&count.to_string()),
                style.strong(&world.now().to_string())
            );
            accrued_lines(world, style, *current);
        }
        Some("raze") => {
            match words.next().and_then(parse_tile) {
                Some(tile) => match world.raze_as(SOVEREIGN, *current, tile) {
                    Ok(()) => {
                        println!(
                            "{} tile {} ({}) razed — half the build cost refunded, receipt sealed",
                            style.ok("✓"),
                            style.strong(&tile.to_string()),
                            coordinate(tile)
                        );
                        map(world, style, *current);
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                },
                None => println!(
                    "{} usage: {} — e.g. {}",
                    style.bad("✗"),
                    style.strong("raze <tile|row:col>"),
                    style.strong("raze 1:4")
                ),
            }
        }
        Some("collect") => match world.collect_as(SOVEREIGN, *current) {
            Ok(count) => {
                println!(
                    "{} harvest banked — {} receipt(s) sealed at stamp {}",
                    style.ok("✓"),
                    style.strong(&count.to_string()),
                    style.strong(&world.now().to_string())
                );
                banked_lines(world, style, *current);
            }
            Err(refusal) => refuse(world, &refusal, style),
        },
        Some("save") => match world.orb() {
            Some(orb) => {
                let path = std::path::PathBuf::from("var/worlds")
                    .join(&orb.name)
                    .join("chronicle.tlv");
                match cfe::persist::sync(world.chronicle(), &path) {
                    Ok(appended) => println!(
                        "{} world saved — {} receipt(s) appended to {} (the file only grows)",
                        style.ok("✓"),
                        style.strong(&appended.to_string()),
                        style.dim(&path.display().to_string())
                    ),
                    Err(refusal) => println!("{} save refused: {refusal:?}", style.bad("✗")),
                }
            }
            None => println!(
                "{} nothing to save — {} begins the story",
                style.bad("✗"),
                style.strong("found <name>")
            ),
        },
        Some("resume") => match words.next() {
            Some(name) if world.chronicle().receipts().is_empty() => {
                let path = std::path::PathBuf::from("var/worlds")
                    .join(name)
                    .join("chronicle.tlv");
                match cfe::persist::load(&path) {
                    Ok(chronicle) => {
                        let receipts = chronicle.receipts().len();
                        *world = World::resume(chronicle);
                        *current = cfe::world::FIRST_ORB;
                        println!(
                            "{} world {} resumed — {} receipts verified, clock at {}; the story continues",
                            style.ok("✓"),
                            style.strong(name),
                            style.strong(&receipts.to_string()),
                            style.strong(&world.now().to_string())
                        );
                        map(world, style, *current);
                    }
                    Err(refusal) => println!(
                        "{} resume refused: {refusal:?} — {}",
                        style.bad("✗"),
                        style.dim("a chronicle must verify before it becomes a world")
                    ),
                }
            }
            Some(_) => println!(
                "{} this session already has history — resume only into a fresh launch",
                style.bad("✗")
            ),
            None => println!("{} usage: {}", style.bad("✗"), style.strong("resume <world-name>")),
        },
        Some("chart") => {
            let name = words.next().map(|word| word.to_string());
            let source = match (words.next(), words.next()) {
                (Some("from"), Some(planet)) => world
                    .orbs()
                    .iter()
                    .find(|held| held.name == planet)
                    .map(|held| held.orb_id),
                _ => Some(*current),
            };
            match (name, source) {
                (Some(name), Some(source)) => match world.chart(&name, source) {
                    Ok(orb) => {
                        *current = orb;
                        println!(
                            "{} planet {} charted (orb {orb}) — the founding reserves moved from the source colony; you are now here",
                            style.ok("✓"),
                            style.strong(&name)
                        );
                        map(world, style, *current);
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                },
                (None, _) => println!(
                    "{} usage: {} — the reserves move from the source (default: here)",
                    style.bad("✗"),
                    style.strong("chart <planet> [from <planet>]")
                ),
                (_, None) => println!(
                    "{} unknown source planet — {} lists them",
                    style.bad("✗"),
                    style.strong("planets")
                ),
            }
        }
        Some("planets") => {
            for orb in world.orbs() {
                let here = match orb.orb_id == *current {
                    true => style.ok("◉ here"),
                    false => style.dim(" "),
                };
                println!(
                    "  {} {} {} {}",
                    style.dim(&format!("#{}", orb.orb_id)),
                    style.strong(&orb.name),
                    style.dim(&format!("({} zones)", orb.zones.len())),
                    here
                );
            }
        }
        Some("planet") => match words.next() {
            Some(name) => match world.orbs().iter().find(|held| held.name == name) {
                Some(orb) => {
                    *current = orb.orb_id;
                    println!("{} now at {}", style.ok("✓"), style.strong(name));
                    map(world, style, *current);
                }
                None => println!(
                    "{} no planet named {} — {} lists them",
                    style.bad("✗"),
                    style.strong(name),
                    style.strong("planets")
                ),
            },
            None => println!("{} usage: {}", style.bad("✗"), style.strong("planet <name>")),
        },
        Some("agent") => match words.next() {
            Some(handle) => match world.register_agent(handle) {
                Ok(id) => println!(
                    "{} agent {} registered (entity {id}) — contained, ZERO grants; open scopes with {}",
                    style.ok("✓"),
                    style.strong(handle),
                    style.strong("grant <handle> <build|raze|collect>")
                ),
                Err(refusal) => refuse(world, &refusal, style),
            },
            None => println!("{} usage: {}", style.bad("✗"), style.strong("agent <handle>")),
        },
        Some("grant") | Some("revoke") => {
            let granting = line.starts_with("grant");
            let target = words.next().and_then(|handle| entity_id(world, handle));
            let scope = words.next().and_then(parse_scope);
            match (target, scope) {
                (Some(agent), Some(scope)) => {
                    let outcome = match granting {
                        true => world.grant(agent, scope),
                        false => world.revoke(agent, scope),
                    };
                    match outcome {
                        Ok(()) => println!(
                            "{} {} {:?} for entity {agent} — receipt sealed",
                            style.ok("✓"),
                            if granting { "granted" } else { "revoked" },
                            scope
                        ),
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("grant|revoke <handle> <build|raze|collect>")
                ),
            }
        }
        Some("as") => match words.next().and_then(|handle| entity_id(world, handle)) {
            Some(actor) => {
                let rest: Vec<&str> = words.collect();
                act_as(world, actor, &rest, style, current);
            }
            None => println!(
                "{} unknown entity — {} lists everyone",
                style.bad("✗"),
                style.strong("who")
            ),
        },
        Some("who") => {
            for entity in world.entities() {
                let role = match entity.principal {
                    None => style.ok("sovereign"),
                    Some(principal) => style.warn(&format!("agent of {principal}")),
                };
                let grants = match entity.principal {
                    None => style.dim("(all scopes by construction)"),
                    Some(_) => style.dim(&format!("grants: {:?}", entity.grants)),
                };
                println!(
                    "  {} {} — {role} {grants}",
                    style.dim(&format!("#{}", entity.entity_id)),
                    style.strong(&entity.handle)
                );
            }
        }
        Some("map") => map(world, style, *current),
        Some("status") => status(world, style, *current),
        Some("chronicle") => chronicle(world, style),
        Some("verify") => match world.chronicle().verify_chain() {
            ChainRead::Intact => println!(
                "{} chain {} — {} receipts, every link re-verified",
                style.ok("✓"),
                style.ok("INTACT"),
                style.strong(&world.chronicle().receipts().len().to_string())
            ),
            ChainRead::Broken { at } => println!(
                "{} chain {} at receipt #{at} — history has been altered",
                style.bad("✗"),
                style.bad("BROKEN")
            ),
        },
        Some("replay") => {
            let replayed = replay(world.chronicle());
            match replayed == world.committed() {
                true => println!(
                    "{} replay {} — the receipts alone rebuild this exact world",
                    style.ok("✓"),
                    style.ok("MATCHES")
                ),
                false => println!(
                    "{} replay {} — the chronicle and the world disagree",
                    style.bad("✗"),
                    style.bad("DIVERGES")
                ),
            }
        }
        Some(other) => println!(
            "{} unknown command {} — type {} to see them all",
            style.bad("✗"),
            style.strong(other),
            style.strong("help")
        ),
        None => {}
    }
}

fn help(style: &Style) {
    let lines = [
        ("found <name>", "establish your colony (banks 20 iron + 10 biomass) — your first receipt"),
        ("zone <tile|r:c> <kind>", "build — ext: 10 iron → +3/2 iron/t · hab: 5 bio → +1 bio/t · smelt: 15 iron+10 bio → 2 iron+1/2 bio in, +1 alloy/t · synth: 20 alloy+15 bio → 4 alloy+1 bio in, +1/4 diamond/t"),
        ("tick <n>", "let n ticks pass — active zones net yield−upkeep into the workspace; famine strikes if a store runs dry"),
        ("collect", "settle the workspace against the bank — sealed as receipts; only banked value persists"),
        ("raze <tile|r:c>", "demolish a zone for half its cost back — the famine escape hatch"),
        ("chart <planet> [from <p>]", "found a daughter colony: the founding reserves MOVE from the source colony (nothing is minted)"),
        ("save · resume <world>", "sync the chronicle to var/worlds/<name>/ (append-only) · resume a saved world at launch"),
        ("planet <name> · planets", "switch the planet cursor · list this world's planets"),
        ("agent <handle>", "register a contained agent (zero grants until you open scopes)"),
        ("grant|revoke <handle> <scope>", "open or shut an agent's scope: build, raze, collect — revocation is immediate"),
        ("as <handle> <cmd>", "act as an entity — its authority, its generation tag on the receipts"),
        ("who", "everyone in this world: sovereigns, agents, grants"),
        ("map", "the tile grid"),
        ("status", "colony, ledgers, clock, receipts"),
        ("chronicle", "the world's full receipted history"),
        ("verify", "re-check every hash link in the chain"),
        ("replay", "rebuild the world from receipts alone and compare"),
        ("quit", "leave (no disk persistence yet — each launch is a fresh world)"),
    ];
    for (name, what) in lines {
        println!("  {:<28} {}", style.strong(name), what);
    }
}

fn parse_scope(word: &str) -> Option<Scope> {
    match word {
        "build" => Some(Scope::Build),
        "raze" => Some(Scope::Raze),
        "collect" => Some(Scope::Collect),
        _ => None,
    }
}

fn entity_id(world: &World, handle: &str) -> Option<u64> {
    world
        .entities()
        .iter()
        .find(|entity| entity.handle == handle)
        .map(|entity| entity.entity_id)
}

/// Run one action AS an entity — the containment demo: the same verbs,
/// the entity's own authority.
fn act_as(world: &mut World, actor: u64, rest: &[&str], style: &Style, current: &mut u64) {
    match rest {
        ["zone", tile, kind] => {
            let tile = parse_tile(tile);
            let kind = match *kind {
                "ext" => Some(ZoneKind::Extractor),
                "hab" => Some(ZoneKind::Habitat),
                "smelt" => Some(ZoneKind::Smelter),
                "synth" => Some(ZoneKind::Synthesizer),
                _ => None,
            };
            match (tile, kind) {
                (Some(tile), Some(kind)) => match world.zone_as(actor, *current, tile, kind) {
                    Ok(()) => println!(
                        "{} entity {actor} zoned tile {tile} — receipt tagged to it",
                        style.ok("✓")
                    ),
                    Err(refusal) => refuse(world, &refusal, style),
                },
                _ => println!("{} as <handle> zone <tile> <kind>", style.bad("✗")),
            }
        }
        ["raze", tile] => match parse_tile(tile) {
            Some(tile) => match world.raze_as(actor, *current, tile) {
                Ok(()) => println!(
                    "{} entity {actor} razed tile {tile} — receipt tagged to it",
                    style.ok("✓")
                ),
                Err(refusal) => refuse(world, &refusal, style),
            },
            None => println!("{} as <handle> raze <tile>", style.bad("✗")),
        },
        ["collect"] => match world.collect_as(actor, *current) {
            Ok(count) => println!(
                "{} entity {actor} collected — {count} receipt(s) tagged to it",
                style.ok("✓")
            ),
            Err(refusal) => refuse(world, &refusal, style),
        },
        _ => println!(
            "{} as <handle> supports: {} — identity changes authority, not vocabulary",
            style.bad("✗"),
            style.strong("zone · raze · collect")
        ),
    }
}

fn parse_tile(word: &str) -> Option<u16> {
    match word.split_once(':') {
        Some((row, col)) => {
            let row: u16 = row.parse().ok()?;
            let col: u16 = col.parse().ok()?;
            match row < GRID && col < GRID {
                true => Some(row * GRID + col),
                false => Some(u16::MAX), // off-map: reported by the range check
            }
        }
        None => word.parse().ok(),
    }
}

fn coordinate(tile: u16) -> String {
    format!("{}:{}", tile / GRID, tile % GRID)
}

fn kind_name(kind: ZoneKind) -> &'static str {
    match kind {
        ZoneKind::Extractor => "extractor",
        ZoneKind::Habitat => "habitat",
        ZoneKind::Smelter => "smelter",
        ZoneKind::Synthesizer => "synthesizer",
    }
}

fn resource_name(resource: Resource) -> &'static str {
    match resource {
        Resource::Iron => "iron",
        Resource::Biomass => "biomass",
        Resource::Alloy => "alloy",
        Resource::Diamond => "diamond",
    }
}

fn amount(value: &Ratio) -> String {
    match value.denom() == &num_bigint::BigInt::from(1) {
        true => value.numer().to_string(),
        false => format!("{}/{}", value.numer(), value.denom()),
    }
}

fn refuse(world: &World, refusal: &ActionRefused, style: &Style) {
    let text = match refusal {
        ActionRefused::NoColony => format!(
            "no colony yet — start with {}",
            style.strong("found <name>")
        ),
        ActionRefused::ColonyAlreadyFounded => {
            let name = world.orb().map(|orb| orb.name.clone()).unwrap_or_default();
            format!(
                "your colony {} already stands (one colony per world in this slice) — did you mean {}?",
                style.strong(&name),
                style.strong("zone <tile> <ext|hab>")
            )
        }
        ActionRefused::TileAlreadyZoned { tile } => format!(
            "tile {} ({}) is already zoned — pick an empty tile ({} shows the grid)",
            style.strong(&tile.to_string()),
            coordinate(*tile),
            style.strong("map")
        ),
        ActionRefused::NoSuchZone { tile } => format!(
            "tile {} ({}) holds no zone to raze — {} shows what stands where",
            style.strong(&tile.to_string()),
            coordinate(*tile),
            style.strong("map")
        ),
        ActionRefused::CannotAfford { resource, need, have } => format!(
            "that costs {} {} and you have {} — {} to bank your accrual, or {} to reclaim half a build",
            style.strong(&amount(need)),
            resource_name(*resource),
            style.strong(&amount(have)),
            style.strong("collect"),
            style.strong("raze <tile>")
        ),
        ActionRefused::NothingAccrued => format!(
            "nothing to collect — let time pass first: {}",
            style.strong("tick <n>")
        ),
        ActionRefused::NoSuchEntity { entity } if *entity == SOVEREIGN => format!(
            "no one is here yet — {} begins the story",
            style.strong("found <name>")
        ),
        ActionRefused::NoSuchEntity { entity } => format!(
            "no entity holds id {} — {} lists everyone",
            style.strong(&entity.to_string()),
            style.strong("who")
        ),
        ActionRefused::HandleTaken { handle } => format!(
            "the handle {} is taken — every entity's name is its own",
            style.strong(handle)
        ),
        ActionRefused::NotPermitted { entity, scope } => format!(
            "entity {} holds no {:?} grant — containment refused it ({} opens the scope)",
            style.strong(&entity.to_string()),
            scope,
            style.strong("grant <handle> <scope>")
        ),
        ActionRefused::NotAnAgent { entity } => format!(
            "entity {} is sovereign — its authority is by construction, never by grant",
            style.strong(&entity.to_string())
        ),
        ActionRefused::NoSuchOrb { orb } => format!(
            "no planet holds orb id {} — {} lists them",
            style.strong(&orb.to_string()),
            style.strong("planets")
        ),
        ActionRefused::PlanetNameTaken { name } => format!(
            "a planet named {} already turns in this world — planet names are unique here",
            style.strong(name)
        ),
    };
    println!("{} {text}", style.bad("✗"));
}

fn map(world: &World, style: &Style, current: u64) {
    let Some(orb) = world.orb_by_id(current) else {
        println!("{}", style.dim("(no colony yet — the map begins at 'found <name>')"));
        return;
    };
    println!(
        "{}  {}",
        style.head("MAP"),
        style.dim("E extractor · H habitat · S smelter · Y synthesizer · lowercase = dormant · · empty")
    );
    print!("     ");
    for col in 0..GRID {
        print!("{} ", style.dim(&col.to_string()));
    }
    println!();
    for row in 0..GRID {
        print!("  {} ", style.dim(&format!("{row}·")));
        for col in 0..GRID {
            let tile = row * GRID + col;
            let cell = orb.zones.iter().find(|zone| zone.tile == tile);
            let glyph = match cell {
                Some(zone) if !orb.zone_active(zone) => style.bad(match zone.kind {
                    ZoneKind::Extractor => "e",
                    ZoneKind::Habitat => "h",
                    ZoneKind::Smelter => "s",
                    ZoneKind::Synthesizer => "y",
                }),
                Some(zone) => match zone.kind {
                    ZoneKind::Extractor => style.ok("E"),
                    ZoneKind::Habitat => style.head("H"),
                    ZoneKind::Smelter => style.warn("S"),
                    ZoneKind::Synthesizer => style.prompt("Y"),
                },
                None => style.dim("·"),
            };
            print!("{glyph} ");
        }
        println!();
    }
}

fn accrued_lines(world: &World, style: &Style, current: u64) {
    for (resource, value) in &world.accrued_of(current) {
        println!(
            "    {} {} {}  {}",
            style.warn("◔"),
            style.warn("accrued"),
            format_args!("{}: {}", resource_name(*resource), style.strong(&amount(value))),
            style.dim("(workspace — collect to keep)")
        );
    }
}

fn banked_lines(world: &World, style: &Style, current: u64) {
    if let Some(orb) = world.orb_by_id(current) {
        for (resource, value) in &orb.banked {
            println!(
                "    {} banked {}: {}",
                style.ok("▣"),
                resource_name(*resource),
                style.strong(&amount(value))
            );
        }
    }
}

fn status(world: &World, style: &Style, current: u64) {
    match world.orb_by_id(current) {
        Some(orb) => {
            println!(
                "{} colony {} {}",
                style.head("■"),
                style.strong(&orb.name),
                style.dim(&format!("(orb {})", orb.orb_id))
            );
            for resource in &orb.famine {
                println!(
                    "    {} {} famine — its consumers are dormant ({} restocks, {} cuts the drain)",
                    style.bad("☠"),
                    style.bad(resource_name(*resource)),
                    style.strong("collect"),
                    style.strong("raze")
                );
            }
            zone_lines(orb, style);
            banked_lines(world, style, current);
            accrued_lines(world, style, current);
        }
        None => println!(
            "{} no colony — {} begins the story",
            style.head("■"),
            style.strong("found <name>")
        ),
    }
    println!(
        "  {} clock {} · receipts {} {}",
        style.dim("⏱"),
        style.strong(&world.now().to_string()),
        style.strong(&world.chronicle().receipts().len().to_string()),
        style.dim("(verify · replay · chronicle)")
    );
}

fn zone_lines(orb: &ColonyOrb, style: &Style) {
    for zone in &orb.zones {
        let (yields, rate) = zone.kind.yield_per_tick();
        let drains = zone
            .kind
            .upkeep_per_tick()
            .iter()
            .map(|(eats, drain)| format!("−{} {}", amount(drain), resource_name(*eats)))
            .collect::<Vec<_>>()
            .join(" · ");
        let flows = format!(
            "+{} {}/tick · {drains}/tick",
            amount(&rate),
            resource_name(yields)
        );
        let state = match orb.zone_active(zone) {
            true => style.dim(&flows),
            false => style.bad("DORMANT (famine)"),
        };
        println!(
            "    tile {:>3} {}  {} {}",
            style.strong(&zone.tile.to_string()),
            style.dim(&format!("({})", coordinate(zone.tile))),
            kind_name(zone.kind),
            state
        );
    }
}

fn chronicle(world: &World, style: &Style) {
    for receipt in world.chronicle().receipts() {
        let described = match &receipt.event {
            cfe::dop::ClosureEvent::ColonyFounded { name, .. } => {
                format!("⚑ colony {} founded", style.strong(name))
            }
            cfe::dop::ClosureEvent::ZoneBuilt { tile, kind, .. } => format!(
                "⛏ tile {} ({}) zoned {}",
                style.strong(&tile.to_string()),
                coordinate(*tile),
                kind_name(*kind)
            ),
            cfe::dop::ClosureEvent::YieldCollected {
                resource, amount: value, ..
            } => format!(
                "▣ collected {} {}",
                style.strong(&amount(value)),
                resource_name(*resource)
            ),
            cfe::dop::ClosureEvent::UpkeepSettled {
                resource, amount: value, ..
            } => format!(
                "▽ upkeep settled: {} {} drawn from the bank",
                style.strong(&amount(value)),
                resource_name(*resource)
            ),
            cfe::dop::ClosureEvent::FamineStruck {
                resource, drawn, ..
            } => format!(
                "☠ FAMINE — {} ran dry ({} drawn); consumers dormant",
                style.strong(resource_name(*resource)),
                amount(drawn)
            ),
            cfe::dop::ClosureEvent::FamineRelieved { resource, .. } => format!(
                "☀ famine relieved — {} recovered; zones wake",
                style.strong(resource_name(*resource))
            ),
            cfe::dop::ClosureEvent::ZoneRazed { tile, .. } => format!(
                "⌫ tile {} ({}) razed — half cost refunded",
                style.strong(&tile.to_string()),
                coordinate(*tile)
            ),
            cfe::dop::ClosureEvent::EntityRegistered { entity_id, principal, handle } => {
                match principal {
                    0 => format!("☉ sovereign {} joined (entity {entity_id})", style.strong(handle)),
                    _ => format!(
                        "⚙ agent {} registered (entity {entity_id}, contained by {principal})",
                        style.strong(handle)
                    ),
                }
            }
            cfe::dop::ClosureEvent::ScopeGranted { agent, scope } => {
                format!("⚿ {:?} granted to entity {}", scope, style.strong(&agent.to_string()))
            }
            cfe::dop::ClosureEvent::ScopeRevoked { agent, scope } => {
                format!("⚿ {:?} REVOKED from entity {}", scope, style.strong(&agent.to_string()))
            }
            cfe::dop::ClosureEvent::ColonyCharted { orb_id, source_orb, name } => format!(
                "☄ planet {} charted (orb {orb_id}) — reserves moved from orb {source_orb}",
                style.strong(name)
            ),
        };
        let tag = receipt
            .provenance
            .first()
            .map(|entity| style.dim(&format!("by#{entity}")))
            .unwrap_or_else(|| style.dim("world"));
        println!(
            "  {} stamp {} {} {}  {}",
            style.dim(&format!("#{}", receipt.sequence)),
            receipt.stamp,
            style.dim(&hex(&receipt.own.0[..8])),
            tag,
            described
        );
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
