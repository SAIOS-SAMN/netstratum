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
                Some("enc") | Some("encasement") => Some(ZoneKind::Encasement),
                Some("cryo") | Some("cryo-array") => Some(ZoneKind::CryoArray),
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
        Some("star") => match words.next().map(parse_star) {
            Some(Some(class)) => match world.commission_star(class) {
                Ok(()) => println!(
                    "{} sun commissioned: {} (L = {}) — site your planets: {}",
                    style.ok("✓"),
                    style.strong(star_name(class as u8)),
                    amount(&class.luminosity()),
                    style.strong("site <albedo> <r>")
                ),
                Err(refusal) => refuse(world, &refusal, style),
            },
            _ => println!(
                "{} usage: {} — classes: O B A F G K M · bd wd rg ns wr lwh",
                style.bad("✗"),
                style.strong("star <class>")
            ),
        },
        Some("site") => {
            let albedo = words.next().and_then(parse_ratio_arg);
            let orbit_r = words.next().and_then(parse_ratio_arg);
            match (albedo, orbit_r) {
                (Some(albedo), Some(orbit_r)) => {
                    match world.site_planet(*current, albedo, orbit_r) {
                        Ok(()) => {
                            thermal_line(world, style, *current);
                            println!("{} orbit sited — thermal is live for this planet", style.ok("✓"));
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
                _ => println!(
                    "{} usage: {} (exact fractions) — e.g. {}",
                    style.bad("✗"),
                    style.strong("site <albedo> <r>"),
                    style.strong("site 3/10 1")
                ),
            }
        }
        Some("thermal") => thermal_line(world, style, *current),
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
        Some("send") => {
            let value = words.next().and_then(parse_ratio_arg);
            let resource = words.next().and_then(parse_resource);
            let to_word = words.next();
            let target_name = words.next().map(|word| word.to_string());
            let source = match (words.next(), words.next()) {
                (Some("from"), Some(planet)) => world
                    .orbs()
                    .iter()
                    .find(|held| held.name == planet)
                    .map(|held| held.orb_id),
                _ => Some(*current),
            };
            match (value, resource, to_word, target_name) {
                (Some(value), Some(resource), Some("to"), Some(planet)) => {
                    let target = world
                        .orbs()
                        .iter()
                        .find(|held| held.name == planet)
                        .map(|held| held.orb_id);
                    match (target, source) {
                        (Some(target), Some(source)) => {
                            let famined_before = world
                                .orb_by_id(target)
                                .map(|held| held.famine.clone())
                                .unwrap_or_default();
                            match world.transfer_as(SOVEREIGN, source, target, resource, value.clone()) {
                                Ok(()) => {
                                    println!(
                                        "{} {} {} sent to {} — receipt sealed at stamp {}; the books of both planets balance",
                                        style.ok("✓"),
                                        style.strong(&amount(&value)),
                                        resource_name(resource),
                                        style.strong(&planet),
                                        style.strong(&world.now().to_string())
                                    );
                                    let famined_after = world
                                        .orb_by_id(target)
                                        .map(|held| held.famine.clone())
                                        .unwrap_or_default();
                                    for relieved in famined_before
                                        .iter()
                                        .filter(|held| !famined_after.contains(held))
                                    {
                                        println!(
                                            "{} famine on {} relieved — the gift covered the one-tick bill; its zones wake",
                                            style.ok("☀"),
                                            style.strong(&planet)
                                        );
                                        let _ = relieved;
                                    }
                                }
                                Err(refusal) => refuse(world, &refusal, style),
                            }
                        }
                        _ => println!(
                            "{} unknown planet — {} lists them",
                            style.bad("✗"),
                            style.strong("planets")
                        ),
                    }
                }
                _ => println!(
                    "{} usage: {} (exact fractions) — e.g. {}",
                    style.bad("✗"),
                    style.strong("send <amount> <resource> to <planet> [from <planet>]"),
                    style.strong("send 5/2 joule to outpost")
                ),
            }
        }
        Some("absorb") => {
            let value = words.next().and_then(parse_ratio_arg);
            let from_word = words.next();
            let source_name = words.next().map(|word| word.to_string());
            let absorber = match (words.next(), words.next()) {
                (Some("into"), Some(planet)) => world
                    .orbs()
                    .iter()
                    .find(|held| held.name == planet)
                    .map(|held| held.orb_id),
                _ => Some(*current),
            };
            match (value, from_word, source_name) {
                (Some(value), Some("from"), Some(planet)) => {
                    let source = world
                        .orbs()
                        .iter()
                        .find(|held| held.name == planet)
                        .map(|held| held.orb_id);
                    match (source, absorber) {
                        (Some(source), Some(absorber)) => {
                            match world.absorb_as(SOVEREIGN, source, absorber, value.clone()) {
                                Ok(()) => {
                                    println!(
                                        "{} {} overflow absorbed from {} — sink consumed, shed to the void through the absorber's arrays; the reciprocity window opens ({} ticks)",
                                        style.ok("♻"),
                                        style.strong(&amount(&value)),
                                        style.strong(&planet),
                                        cfe::dop::reciprocity_window()
                                    );
                                    if !world
                                        .orb_by_id(source)
                                        .map(|held| held.overheat)
                                        .unwrap_or(false)
                                        && world.vent_pending_of(source) == cfe::ratio::ratio(0, 1)
                                    {
                                        println!(
                                            "{} the overheat on {} clears — its waste found a home instead of the void",
                                            style.ok("❄"),
                                            style.strong(&planet)
                                        );
                                    }
                                }
                                Err(refusal) => refuse(world, &refusal, style),
                            }
                        }
                        _ => println!(
                            "{} unknown planet — {} lists them",
                            style.bad("✗"),
                            style.strong("planets")
                        ),
                    }
                }
                _ => println!(
                    "{} usage: {} (exact fractions) — e.g. {}",
                    style.bad("✗"),
                    style.strong("absorb <amount> from <planet> [into <planet>]"),
                    style.strong("absorb 1/2 from furnace into coldstore")
                ),
            }
        }
        Some("bond") => {
            let rate = words.next().and_then(parse_ratio_arg);
            let resource = words.next().and_then(parse_resource);
            let to_word = words.next();
            let target_name = words.next().map(|word| word.to_string());
            let mut term = 0u64;
            let mut source = Some(*current);
            loop {
                match (words.next(), words.next()) {
                    (Some("for"), Some(span)) => term = span.parse().unwrap_or(0),
                    (Some("from"), Some(planet)) => {
                        source = world
                            .orbs()
                            .iter()
                            .find(|held| held.name == planet)
                            .map(|held| held.orb_id)
                    }
                    _ => break,
                }
            }
            match (rate, resource, to_word, target_name) {
                (Some(rate), Some(resource), Some("to"), Some(planet)) => {
                    let target = world
                        .orbs()
                        .iter()
                        .find(|held| held.name == planet)
                        .map(|held| held.orb_id);
                    match (target, source) {
                        (Some(target), Some(source)) => {
                            match world.seal_bond_as(SOVEREIGN, source, target, resource, rate.clone(), term) {
                                Ok(()) => println!(
                                    "{} bond sealed: {} {}/tick streams to {}{} — settles at each touch; severing without notice posts the load",
                                    style.ok("⚭"),
                                    style.strong(&amount(&rate)),
                                    resource_name(resource),
                                    style.strong(&planet),
                                    match term {
                                        0 => " (open-ended)".to_string(),
                                        span => format!(" (term {span} ticks — expiry posts nothing)"),
                                    }
                                ),
                                Err(refusal) => refuse(world, &refusal, style),
                            }
                        }
                        _ => println!(
                            "{} unknown planet — {} lists them",
                            style.bad("✗"),
                            style.strong("planets")
                        ),
                    }
                }
                _ => println!(
                    "{} usage: {} — e.g. {}",
                    style.bad("✗"),
                    style.strong("bond <rate> <resource> to <planet> [for <term>] [from <planet>]"),
                    style.strong("bond 1/4 joule to coldstore for 12")
                ),
            }
        }
        Some("sever") => {
            let resource = words.next().and_then(parse_resource);
            let to_word = words.next();
            let target_name = words.next().map(|word| word.to_string());
            let source = match (words.next(), words.next()) {
                (Some("from"), Some(planet)) => world
                    .orbs()
                    .iter()
                    .find(|held| held.name == planet)
                    .map(|held| held.orb_id),
                _ => Some(*current),
            };
            match (resource, to_word, target_name) {
                (Some(resource), Some("to"), Some(planet)) => {
                    let target = world
                        .orbs()
                        .iter()
                        .find(|held| held.name == planet)
                        .map(|held| held.orb_id);
                    match (target, source) {
                        (Some(target), Some(source)) => {
                            match world.sever_bond_as(SOVEREIGN, source, target, resource) {
                                Ok(()) => println!(
                                    "{} bond severed — the final span settled; the notice load posts to the cutter's book ({} shows it)",
                                    style.bad("✂"),
                                    style.strong("chronicle")
                                ),
                                Err(refusal) => refuse(world, &refusal, style),
                            }
                        }
                        _ => println!(
                            "{} unknown planet — {} lists them",
                            style.bad("✗"),
                            style.strong("planets")
                        ),
                    }
                }
                _ => println!(
                    "{} usage: {} — e.g. {}",
                    style.bad("✗"),
                    style.strong("sever <resource> to <planet> [from <planet>]"),
                    style.strong("sever joule to coldstore")
                ),
            }
        }
        Some("bonds") => {
            match world.bonds().is_empty() {
                true => println!("{}", style.dim("(no standing bonds)")),
                false => {
                    for bond in world.bonds() {
                        let source = world
                            .orb_by_id(bond.source)
                            .map(|held| held.name.clone())
                            .unwrap_or_else(|| bond.source.to_string());
                        let target = world
                            .orb_by_id(bond.target)
                            .map(|held| held.name.clone())
                            .unwrap_or_else(|| bond.target.to_string());
                        println!(
                            "  {} {} {}/tick {} → {} {}",
                            style.ok("⚭"),
                            style.strong(&amount(&bond.rate)),
                            resource_name(bond.resource),
                            style.strong(&source),
                            style.strong(&target),
                            style.dim(&match bond.term {
                                0 => format!("(open-ended, anchored {})", bond.anchored),
                                span => format!("(expires at {}, anchored {})", bond.sealed + span, bond.anchored),
                            })
                        );
                    }
                }
            }
        }
        Some("covenant") => {
            let quota = words.next().and_then(parse_ratio_arg);
            let resource = words.next().and_then(parse_resource);
            let to_word = words.next();
            let target_name = words.next().map(|word| word.to_string());
            let mut window = None;
            let mut obligor = Some(*current);
            loop {
                match (words.next(), words.next()) {
                    (Some("within"), Some(span)) => window = span.parse().ok(),
                    (Some("from"), Some(planet)) => {
                        obligor = world
                            .orbs()
                            .iter()
                            .find(|held| held.name == planet)
                            .map(|held| held.orb_id)
                    }
                    _ => break,
                }
            }
            match (quota, resource, to_word, target_name, window) {
                (Some(quota), Some(resource), Some("to"), Some(planet), Some(window)) => {
                    let beneficiary = world
                        .orbs()
                        .iter()
                        .find(|held| held.name == planet)
                        .map(|held| held.orb_id);
                    match (beneficiary, obligor) {
                        (Some(beneficiary), Some(obligor)) => {
                            match world.seal_covenant_as(
                                SOVEREIGN, obligor, beneficiary, resource, quota.clone(), window,
                            ) {
                                Ok(()) => println!(
                                    "{} covenant vitrified: {} {} owed to {} within {} ticks — the collateral sits in the lattice until the word is kept or broken",
                                    style.ok("⛨"),
                                    style.strong(&amount(&quota)),
                                    resource_name(resource),
                                    style.strong(&planet),
                                    style.strong(&window.to_string())
                                ),
                                Err(refusal) => refuse(world, &refusal, style),
                            }
                        }
                        _ => println!(
                            "{} unknown planet — {} lists them",
                            style.bad("✗"),
                            style.strong("planets")
                        ),
                    }
                }
                _ => println!(
                    "{} usage: {} — e.g. {}",
                    style.bad("✗"),
                    style.strong("covenant <quota> <resource> to <planet> within <ticks> [from <planet>]"),
                    style.strong("covenant 3 iron to outpost within 10")
                ),
            }
        }
        Some("covenants") => {
            match world.covenants().is_empty() {
                true => println!("{}", style.dim("(no open covenants)")),
                false => {
                    for pact in world.covenants() {
                        let obligor = world
                            .orb_by_id(pact.obligor)
                            .map(|held| held.name.clone())
                            .unwrap_or_else(|| pact.obligor.to_string());
                        let beneficiary = world
                            .orb_by_id(pact.beneficiary)
                            .map(|held| held.name.clone())
                            .unwrap_or_else(|| pact.beneficiary.to_string());
                        println!(
                            "  {} {} owes {} {} {} — delivered {} of {}, deadline {}, {} locked",
                            style.ok("⛨"),
                            style.strong(&obligor),
                            style.strong(&beneficiary),
                            style.strong(&amount(&pact.quota)),
                            resource_name(pact.resource),
                            style.strong(&amount(&pact.delivered)),
                            amount(&pact.quota),
                            style.strong(&(pact.sealed + pact.window).to_string()),
                            style.strong(&amount(&pact.collateral))
                        );
                    }
                }
            }
        }
        Some("ideology") => {
            match world.orb_by_id(*current) {
                Some(orb) => {
                    println!(
                        "{}  {}",
                        style.head("IDEOLOGY"),
                        style.dim("the dual cohesion ledger — deeds, not dials; every ratio derives at read")
                    );
                    let axes: [(&str, &str, &Ratio, &str, &Ratio); 4] = [
                        ("compassion", "gave", &orb.given_in_crisis, "drew", &orb.drawn_in_crisis),
                        ("love", "streamed", &orb.bond_flow, "dropped", &orb.severance_load),
                        ("integrity", "honoured", &orb.covenants_fulfilled, "broke", &orb.covenants_breached),
                        ("morality", "returned", &orb.returned_across, "extracted", &orb.extracted_across),
                    ];
                    for (axis, syn_name, syn, ent_name, ent) in axes {
                        let reading = match cfe::dop::cohesion_ratio(syn, ent) {
                            Some(value) => format!("ratio {}", style.strong(&amount(&value))),
                            None => style.dim("no deeds yet"),
                        };
                        println!(
                            "  {:<11} {} {} · {} {} — {}",
                            axis,
                            syn_name,
                            style.strong(&amount(syn)),
                            ent_name,
                            style.strong(&amount(ent)),
                            reading
                        );
                    }
                }
                None => println!(
                    "{} no colony yet — {} begins the story",
                    style.bad("✗"),
                    style.strong("found <name>")
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
        Some("journal") => journal_feed(world, style),
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
        ("zone <tile|r:c> <kind>", "build — ext: 10 iron → +3/2 iron/t · hab: 5 bio → +1 bio/t · smelt: 15 iron+10 bio → 2 iron+1/2 bio in, +1 alloy/t · synth: 20 alloy+15 bio → 4 alloy+1 bio in, +1/4 diamond/t · enc: 8 iron → encases excess heat as joules"),
        ("tick <n>", "let n ticks pass — active zones net yield−upkeep into the workspace; famine strikes if a store runs dry"),
        ("collect", "settle the workspace against the bank — sealed as receipts; only banked value persists"),
        ("raze <tile|r:c>", "demolish a zone for half its cost back — the famine escape hatch"),
        ("chart <planet> [from <p>]", "found a daughter colony: the founding reserves MOVE from the source colony (nothing is minted)"),
        ("send <amt> <res> to <p> [from <p>]", "move banked value between planets — conservation exact; demurrage settles both sides at the touch; a gift covering the one-tick bill relieves famine"),
        ("absorb <amt> from <p> [into <p>]", "route a hot planet's un-sealed overflow into the absorber's banked sink — emptying it clears the strike; the draw owes the absorber within the reciprocity window or it lapses to the extraction book"),
        ("bond <rate> <res> to <p> [for <n>]", "seal a standing order: rate/tick streams without per-tick clearing, settled in one receipt at each touch — a term bond expires orderly; severing or defaulting posts the notice load"),
        ("sever <res> to <p> · bonds", "cut a standing bond (the load posts to the cutter) · list the standing orders"),
        ("covenant <q> <res> to <p> within <n>", "vitrify a delivery pact in the Glasskeeper lattice — collateral (priced by your fidelity books, never barred) locks until the quota is delivered (release) or the deadline passes (BREACH_AND_SLASH); `covenants` lists the open pacts"),
        ("ideology", "the dual cohesion ledger for this planet — the four axes, their books, and the derived readings; divergent trade partners pay a friction entropy tax, aligned ones pay zero"),
        ("save · resume <world>", "sync the chronicle to var/worlds/<name>/ (append-only) · resume a saved world at launch"),
        ("planet <name> · planets", "switch the planet cursor · list this world's planets"),
        ("agent <handle>", "register a contained agent (zero grants until you open scopes)"),
        ("grant|revoke <handle> <scope>", "open or shut an agent's scope: build, raze, collect — revocation is immediate"),
        ("as <handle> <cmd>", "act as an entity — its authority, its generation tag on the receipts"),
        ("who", "everyone in this world: sovereigns, agents, grants"),
        ("star <class>", "commission the sun (once): O B A F G K M · bd wd rg ns wr lwh — fixes L for every orbit"),
        ("site <albedo> <r>", "site this planet's orbit (exact fractions) — thermal wakes: encase first, only overflow vents"),
        ("thermal", "the thermal read: T_eq⁴, band, overheat, encased joules"),
        ("map", "the tile grid"),
        ("status", "colony, ledgers, clock, receipts"),
        ("chronicle", "the world's full receipted history"),
        ("journal", "the chronicle feed: one entry per completed 60-tick year, newest first"),
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
        ZoneKind::Encasement => "encasement",
        ZoneKind::CryoArray => "cryo-array",
    }
}

fn parse_resource(word: &str) -> Option<Resource> {
    match word.to_ascii_lowercase().as_str() {
        "iron" => Some(Resource::Iron),
        "biomass" | "bio" => Some(Resource::Biomass),
        "alloy" => Some(Resource::Alloy),
        "diamond" => Some(Resource::Diamond),
        "joule" | "joules" => Some(Resource::Joule),
        "sink" => Some(Resource::Sink),
        _ => None,
    }
}

fn parse_star(word: &str) -> Option<cfe::thermal::StarClass> {
    use cfe::thermal::StarClass as S;
    match word.to_ascii_lowercase().as_str() {
        "o" => Some(S::O),
        "b" => Some(S::B),
        "a" => Some(S::A),
        "f" => Some(S::F),
        "g" => Some(S::G),
        "k" => Some(S::K),
        "m" => Some(S::M),
        "bd" => Some(S::BrownDwarf),
        "wd" => Some(S::WhiteDwarf),
        "rg" => Some(S::RedGiant),
        "ns" => Some(S::NeutronStar),
        "wr" => Some(S::WolfRayet),
        "lwh" => Some(S::LuminalWhiteHole),
        _ => None,
    }
}

fn star_name(class: u8) -> &'static str {
    match cfe::thermal::StarClass::from_wire(class) {
        Some(cfe::thermal::StarClass::O) => "class O (blue)",
        Some(cfe::thermal::StarClass::B) => "class B (blue-white)",
        Some(cfe::thermal::StarClass::A) => "class A (white)",
        Some(cfe::thermal::StarClass::F) => "class F (yellow-white)",
        Some(cfe::thermal::StarClass::G) => "class G (yellow, Sol-type)",
        Some(cfe::thermal::StarClass::K) => "class K (orange)",
        Some(cfe::thermal::StarClass::M) => "class M (red dwarf)",
        Some(cfe::thermal::StarClass::BrownDwarf) => "brown dwarf",
        Some(cfe::thermal::StarClass::WhiteDwarf) => "white dwarf",
        Some(cfe::thermal::StarClass::RedGiant) => "red giant",
        Some(cfe::thermal::StarClass::NeutronStar) => "neutron star",
        Some(cfe::thermal::StarClass::WolfRayet) => "Wolf-Rayet",
        Some(cfe::thermal::StarClass::LuminalWhiteHole) => "Luminal White Hole ✦",
        None => "unknown class",
    }
}

fn parse_ratio_arg(word: &str) -> Option<Ratio> {
    use num_bigint::BigInt;
    use std::str::FromStr;
    let (n, d) = match word.split_once('/') {
        Some((n, d)) => (n, d),
        None => (word, "1"),
    };
    let denom = BigInt::from_str(d).ok()?;
    if denom == BigInt::from(0) {
        return None;
    }
    Some(Ratio::new(BigInt::from_str(n).ok()?, denom))
}

/// The thermal read for one orb (05 §1): T_eq⁴, band, the two arms.
fn thermal_line(world: &World, style: &Style, current: u64) {
    use cfe::thermal;
    let Some(class) = world.star() else {
        println!("{}", style.dim("(no sun — 'star <class>' commissions one)"));
        return;
    };
    let Some(orb) = world.orb_by_id(current) else {
        println!("{}", style.dim("(no colony yet)"));
        return;
    };
    println!(
        "{}  sun {}",
        style.head("THERMAL"),
        style.strong(star_name(class as u8))
    );
    let Some(site) = &orb.thermal else {
        println!("{}", style.dim("  (this planet is unsited — 'site <albedo> <r>')"));
        return;
    };
    let t4 = thermal::t_eq4(&class.luminosity(), &site.albedo, &site.orbit_r);
    let band = match thermal::band_of(&t4) {
        Some(thermal::SubstrateBand::Cryogenic) => "CRYOGENIC (the vault: lossless joule store)",
        Some(thermal::SubstrateBand::Mesic) => "MESIC (near-zero life-support overhead)",
        Some(thermal::SubstrateBand::HyperThermal) => "HYPER-THERMAL (Furnace-grade)",
        None => match t4 <= thermal::cryo_vault_max4() {
            true => "below every band — deep Frost (lossless vault)",
            false => "between/above bands — no substrate holds equilibrium",
        },
    };
    println!(
        "  T_eq⁴ {} · band: {} · overheat: {}",
        style.strong(&amount(&t4)),
        band,
        match orb.overheat {
            true => style.bad("YES — overflow venting"),
            false => style.ok("no"),
        }
    );
    let joules = world
        .accrued_of(current)
        .into_iter()
        .find(|(resource, _)| *resource == Resource::Joule)
        .map(|(_, amount)| amount);
    if let Some(value) = joules {
        println!("  accrued joules {} (encased, uncollected)", style.strong(&amount(&value)));
    }
}

fn resource_name(resource: Resource) -> &'static str {
    match resource {
        Resource::Iron => "iron",
        Resource::Biomass => "biomass",
        Resource::Alloy => "alloy",
        Resource::Diamond => "diamond",
        Resource::Joule => "joule",
        Resource::Sink => "sink",
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
        ActionRefused::StarAlreadyCommissioned => format!(
            "this world's sun is already commissioned — one star per world (NS-6 §6); {} shows it",
            style.strong("status")
        ),
        ActionRefused::NoStarCommissioned => format!(
            "no sun yet — commission one first: {} (O B A F G K M · bd wd rg ns wr lwh)",
            style.strong("star <class>")
        ),
        ActionRefused::AlreadySited { orb } => format!(
            "orb {} is already sited — an orbit does not move",
            style.strong(&orb.to_string())
        ),
        ActionRefused::BadSiting => format!(
            "siting needs {} with 0 ≤ A < 1 and r > 0 — e.g. {}",
            style.strong("site <albedo> <r>"),
            style.strong("site 3/10 1")
        ),
        ActionRefused::BadTransfer => format!(
            "a transfer moves a positive amount between two different planets — e.g. {}",
            style.strong("send 5/2 iron to outpost")
        ),
        ActionRefused::NoSuchCouncil { council } => format!(
            "no open council {} stands — delegation settles inside diplomacy; seal the council first",
            style.strong(&format!("{council}"))
        ),
        ActionRefused::SolarLeapNeverLicensed => {
            "the solar leap is never licensed — the young meet their sun themselves, or not".into()
        }
        ActionRefused::BelowTheThreshold => {
            "a licensed leap lands only on a union member — the receiving world has not crossed".into()
        }
        ActionRefused::BadCouncil => format!(
            "a council is three or more distinct planets, a positive quota and collateral, a real window, a fresh id — e.g. {}",
            style.strong("council 1 with base,outpost,haven: 5 iron in 60, stake 3")
        ),
        ActionRefused::ElevationBarred { settled, coherent, crossed } => format!(
            "the climb is barred — {}{}{}; each gate is escapable: settle the books, close a loop this year, cross the void",
            if *settled { "" } else { "the thermal books stand unsettled; " },
            if *coherent { "" } else { "no loop closed within the year voice; " },
            if *crossed { "" } else { "this world has not crossed; " }
        ),
        ActionRefused::BadClimb => format!(
            "a climb moves strictly upward in G — the fiber advances and never retreats; e.g. {}",
            style.strong("elevate 1/6")
        ),
        ActionRefused::Shielded { source_shielded, dest_shielded } => format!(
            "the shield holds — {} not met its sun; a people makes the solar leap itself ({} wakes thermal), or it is left to its sky",
            style.strong(match (source_shielded, dest_shielded) {
                (true, true) => "neither world has",
                (true, false) => "this world has",
                _ => "the far world has",
            }),
            style.strong("site")
        ),
        ActionRefused::CannotAbsorb { pending } => format!(
            "only un-sealed overflow absorbs, and {} stands pending — {} shows the thermal read",
            style.strong(&amount(pending)),
            style.strong("thermal")
        ),
        ActionRefused::BondAlreadySealed => format!(
            "a bond already stands on that resource between those planets — {} it before sealing a new rate",
            style.strong("sever")
        ),
        ActionRefused::NoSuchBond => format!(
            "no bond stands there — {} lists the standing orders",
            style.strong("bonds")
        ),
        ActionRefused::CovenantAlreadySealed => format!(
            "an open covenant already stands on that lane — a vitrified pact cannot be cancelled, only fulfilled or slashed ({} lists them)",
            style.strong("covenants")
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
        style.dim("E extractor · H habitat · S smelter · Y synthesizer · C encasement · K cryo-array · lowercase = dormant · · empty")
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
                    ZoneKind::Encasement => "c",
                    ZoneKind::CryoArray => "k",
                }),
                Some(zone) => match zone.kind {
                    ZoneKind::Extractor => style.ok("E"),
                    ZoneKind::Habitat => style.head("H"),
                    ZoneKind::Smelter => style.warn("S"),
                    ZoneKind::Synthesizer => style.prompt("Y"),
                    ZoneKind::Encasement => style.warn("C"),
                    ZoneKind::CryoArray => style.head("K"),
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

/// JOURNAL-1 (08 §3): the chronicle feed for THIS world, newest first —
/// a rendering of the chronicle, generated on demand, never state. The
/// sovereign's session speaks in the reference register (the axes a
/// registry entry would carry; the CLI world has no registry identity
/// until the founding dialogue lands).
fn journal_feed(world: &World, style: &Style) {
    let Some(orb) = world.orb() else {
        println!(
            "{} no colony yet — {} begins the story",
            style.bad("✗"),
            style.strong("found <name>")
        );
        return;
    };
    let voice = mirror::journal::Voice {
        civilization: orb.name.clone(),
        disposition: "empiricist".into(),
        metabolism: "biotic".into(),
        perception: "planar".into(),
    };
    let entries = mirror::journal::journal(&orb.name, world, &voice, mirror::journal::YEAR_TICKS);
    if entries.is_empty() {
        println!(
            "{} no completed year yet — a year is {} ticks; the clock stands at {}",
            style.dim("…"),
            style.strong(&mirror::journal::YEAR_TICKS.to_string()),
            style.strong(&world.now().to_string())
        );
        return;
    }
    for entry in entries.iter().rev() {
        let headline = match entry.alert {
            mirror::journal::Alert::Warning => style.bad(&entry.headline),
            mirror::journal::Alert::Milestone => style.head(&entry.headline),
            mirror::journal::Alert::Normal => style.ok(&entry.headline),
        };
        println!(
            "{} {} {}",
            style.head(&format!("YEAR {}", entry.year)),
            style.dim(&format!(
                "cycle {} · {} · {} · {}",
                entry.cycle, entry.world, entry.archetype, entry.alert.word()
            )),
            headline
        );
        for line in entry.body.lines() {
            match line.starts_with("RED") {
                true => println!("    {}", style.bad(line)),
                false => println!("    {line}"),
            }
        }
        let economics = &entry.economics;
        println!(
            "    {} joules {} · heating bill {} · vented {} · sink {} · zones {} · growth {}",
            style.dim("▤"),
            style.strong(&economics.banked_joules),
            style.strong(&economics.heating_need_year),
            style.strong(&economics.entropy_vented),
            style.strong(&economics.banked_sink),
            style.strong(&economics.zones.to_string()),
            style.strong(&economics.growth_per_year)
        );
        if let Some(request) = &entry.trade_request {
            println!("    {} {}", style.warn("⇄ trade:"), request);
        }
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
            cfe::dop::ClosureEvent::EraAnnexed { era, world, offset, .. } => format!(
                "✦ era {} world {} annexed at epoch offset {offset}",
                style.dim(era),
                style.strong(world)
            ),
            cfe::dop::ClosureEvent::OverheatStruck { overflow, .. } => format!(
                "♨ OVERHEAT — overflow {} past encasement; yields degrade 1/(1+overflow)",
                style.strong(&amount(overflow))
            ),
            cfe::dop::ClosureEvent::OverheatCleared { .. } => {
                "❄ overheat cleared — flows recover whole".to_string()
            }
            cfe::dop::ClosureEvent::StarCommissioned { class } => format!(
                "☀ sun commissioned — {}",
                style.strong(star_name(*class))
            ),
            cfe::dop::ClosureEvent::PlanetSited { orb_id, albedo, orbit_r } => format!(
                "◍ orb {} sited — albedo {} at orbit r {}",
                style.strong(&orb_id.to_string()),
                amount(albedo),
                amount(orbit_r)
            ),
            cfe::dop::ClosureEvent::VentSettled { amount: vented, .. } => format!(
                "♨ vented entropy settled: {} into the cumulative ledger",
                style.strong(&amount(vented))
            ),
            cfe::dop::ClosureEvent::DemurrageSettled { resource, amount: decay, .. } => format!(
                "◌ demurrage settled: {} {} decayed un-vaulted",
                style.strong(&amount(decay)),
                resource_name(*resource)
            ),
            cfe::dop::ClosureEvent::ResourceTransferred {
                from_orb, to_orb, resource, amount: moved,
            } => format!(
                "⇄ {} {} moved orb {from_orb} → orb {to_orb} — conservation holds",
                style.strong(&amount(moved)),
                resource_name(*resource)
            ),
            cfe::dop::ClosureEvent::OverflowAbsorbed {
                source_orb, absorber_orb, amount: heat,
            } => format!(
                "♻ {} overflow absorbed orb {source_orb} → orb {absorber_orb}'s sink — shed through its arrays; the window opens",
                style.strong(&amount(heat))
            ),
            cfe::dop::ClosureEvent::AbsorptionLapsed {
                debtor_orb, creditor_orb, amount: unpaid,
            } => format!(
                "⌛ absorption window lapsed: {} unpaid orb {debtor_orb} → orb {creditor_orb} — posted to the extraction book",
                style.strong(&amount(unpaid))
            ),
            cfe::dop::ClosureEvent::BondSealed {
                source_orb, target_orb, resource, rate, term,
            } => format!(
                "⚭ bond sealed: {} {}/tick orb {source_orb} → orb {target_orb}{}",
                style.strong(&amount(rate)),
                resource_name(*resource),
                match term {
                    0 => " (open-ended)".to_string(),
                    span => format!(" (term {span} ticks)"),
                }
            ),
            cfe::dop::ClosureEvent::BondSettled {
                source_orb, target_orb, resource, amount: span, severed, load,
            } => match severed {
                false => format!(
                    "⚭ bond settled: {} {} streamed orb {source_orb} → orb {target_orb}",
                    style.strong(&amount(span)),
                    resource_name(*resource)
                ),
                true => format!(
                    "✂ bond ended: {} {} final orb {source_orb} → orb {target_orb} — load {}",
                    style.strong(&amount(span)),
                    resource_name(*resource),
                    style.strong(&amount(load))
                ),
            },
            cfe::dop::ClosureEvent::CovenantSealed {
                obligor_orb, beneficiary_orb, resource, quota, window, collateral,
            } => format!(
                "⛨ covenant vitrified: {} {} owed orb {obligor_orb} → orb {beneficiary_orb} within {window} ticks — {} collateral locked in the lattice",
                style.strong(&amount(quota)),
                resource_name(*resource),
                style.strong(&amount(collateral))
            ),
            cfe::dop::ClosureEvent::CovenantReleased {
                obligor_orb, resource, collateral, ..
            } => format!(
                "⛨ covenant fulfilled — {} {} collateral returns to orb {obligor_orb}; the word held",
                style.strong(&amount(collateral)),
                resource_name(*resource)
            ),
            cfe::dop::ClosureEvent::CovenantSlashed {
                obligor_orb, beneficiary_orb, resource, collateral, ..
            } => format!(
                "⚔ BREACH_AND_SLASH — orb {obligor_orb} broke its word; {} {} forfeits to orb {beneficiary_orb}",
                style.strong(&amount(collateral)),
                resource_name(*resource)
            ),
            cfe::dop::ClosureEvent::FrictionSettled { orb_id, amount: heat } => format!(
                "⇋ ideological friction settled: {} entropy on orb {orb_id} — divergence has a handling cost",
                style.strong(&amount(heat))
            ),
            cfe::dop::ClosureEvent::TechLicensed { deal, council, name, .. } => format!(
                "⌁ {} licensed out under deal {deal} — sealed through council {council}; the far sky learns",
                style.strong(name)
            ),
            cfe::dop::ClosureEvent::CouncilSealed { council, members, collateral, .. } => format!(
                "⚭ council {council} seals — {} members bound alike, {} collateral each; the group over the individual",
                style.strong(&format!("{}", members.len())),
                style.strong(&amount(collateral))
            ),
            cfe::dop::ClosureEvent::CouncilClosed { council, member, fulfilled, banked } => format!(
                "⚖ council {council} reckons — orb {member} {}; {} banked home",
                if *fulfilled { style.strong("kept faith") } else { style.strong("BREACHED") },
                style.strong(&amount(banked))
            ),
            cfe::dop::ClosureEvent::GaugeShifted { gauge } => format!(
                "♪ the key moves — this world now plays at {}",
                style.strong(&amount(gauge))
            ),
            cfe::dop::ClosureEvent::Diverged { branch, gauge } => format!(
                "⑂ the line cleaves — {} departs as a peer; our key moves to {}",
                style.strong(branch),
                style.strong(&amount(gauge))
            ),
            cfe::dop::ClosureEvent::Confluent { partner, gauge } => format!(
                "⑃ two lines combine — with {}; the key settles at the interference, {}",
                style.strong(partner),
                style.strong(&amount(gauge))
            ),
            cfe::dop::ClosureEvent::StarAged { class } => format!(
                "☀ the star moves in its region — class {} now lights every orbit",
                style.strong(&format!("{class}"))
            ),
            cfe::dop::ClosureEvent::EraFolded { era, gauge } => format!(
                "∞ aeon {era} folds — the universe seats a new sheet at {}",
                style.strong(&amount(gauge))
            ),
            cfe::dop::ClosureEvent::TechDiscovered { orb_id, name, condition, .. } => format!(
                "✶ {} — orb {orb_id} (condition {condition}): a technology enters the chronicle",
                style.strong(name)
            ),
            cfe::dop::ClosureEvent::ApertureSent {
                deal, from_orb, resource, amount: moved,
            } => format!(
                "⎋ {} {} crossed out orb {from_orb} → deal {deal} — the far leg lands in another world",
                style.strong(&amount(moved)),
                resource_name(*resource)
            ),
            cfe::dop::ClosureEvent::ApertureReceived {
                deal, to_orb, resource, amount: moved,
            } => format!(
                "⎋ {} {} crossed in deal {deal} → orb {to_orb} — value arrived from another world",
                style.strong(&amount(moved)),
                resource_name(*resource)
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
