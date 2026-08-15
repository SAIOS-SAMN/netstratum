//! The Netstratum CLI — the first slice's face. One command launches
//! it (CHARTER §3.5); every completed action closes as an NS-3 receipt;
//! `verify` and `replay` read the chain the way any peer would.
//!
//! Presentation only lives here: the map grid, colors, and friendly
//! refusal text are the client's voice — the engine's typed refusals
//! and exact ratios are the truth underneath, always shown as exact
//! fractions (the C2R rule: "3/2", never 1.5).

mod agent;
mod corpus;
mod desk;
mod help_menu;
mod hubs;
mod style;

use std::io::{self, BufRead, Write};

use cfe::dop::{ColonyOrb, Resource, ZoneKind};
use cfe::ipsal::Scope;
use cfe::ratio::{ratio, Ratio};
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World, SOVEREIGN};
use style::Style;

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
    let style = Style::new(plain);
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
                // Bind the read's Result BEFORE matching on it — matching
                // directly on `stdin.lock().read_line(...)` extends the
                // StdinLock temporary's lifetime across the whole match
                // arm (a standard Rust footgun), which deadlocks any
                // command below (like the guided genesis picker) that
                // locks stdin again while still "inside" this match.
                let read = stdin.lock().read_line(&mut line);
                match read {
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

fn banner(style: &Style) {
    println!("{}", style.border("╔══════════════════════════════════════════════════╗"));
    println!("{}", style.title("║  NETSTRATUM — bench · intel · trade · diplo      ║"));
    println!("{}", style.border("╚══════════════════════════════════════════════════╝"));
    println!(
        "  {} · {} · {} · {} · {}",
        style.verb("bench"),
        style.verb("intel"),
        style.verb("trade"),
        style.verb("diplo"),
        style.key("help")
    );
}


/// BLOCKER A3: the active profile's station — the authority the
/// Operator seat claim — **session config**, not colony law.
///
/// Plain text `var/operators/<name>` only (`station <word>` lines).
/// Engine deeds never trust this string as physics — mint/station
/// gates re-check the chain.
fn active_station() -> String {
    let Some(name) = active_line() else {
        return "player".into();
    };
    if let Ok(text) = std::fs::read_to_string(format!("var/operators/{name}")) {
        if let Some(s) = profile_field(&text, "station") {
            return s;
        }
    }
    "player".into()
}

/// The active operator's own name — the LINE the mint's standing gate
/// reads. Session config only; the engine re-reads seats from the chain.
fn active_line() -> Option<String> {
    std::fs::read_to_string("var/operators/active")
        .ok()
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
}

fn tier0_gate(style: &Style) -> bool {
    if active_station() == "progenitor" {
        return true;
    }
    println!(
        "{} a tier-0 deed — the board is made by its makers: {}",
        style.bad("✗"),
        style.strong("login <name> as progenitor")
    );
    false
}

/// Read a key from plain operator profile text (`key value` / `key=value`).
/// **Not for colony state** — session config only. Colony truth is chronicle.tlv.
fn profile_field(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once(char::is_whitespace) {
            if k.eq_ignore_ascii_case(key) {
                return Some(v.trim().to_string());
            }
        }
        if let Some((k, v)) = line.split_once('=') {
            if k.trim().eq_ignore_ascii_case(key) {
                return Some(v.trim().trim_matches('"').to_string());
            }
        }
    }
    None
}

/// THE FAR WORLD (union wiring, 2026-07-30): cross-world deeds load
/// the partner from its RESUME carrier (var/worlds/<name>/chronicle.tlv
/// — CLI-saved worlds carry one; polity worlds are export-only and
/// refuse with the cure named). After a two-legged deed BOTH sides
/// sync immediately — conservation is never left half-saved.
fn far_world(name: &str) -> Result<World, String> {
    let path = std::path::PathBuf::from("var/worlds").join(name).join("chronicle.tlv");
    if !path.exists() {
        return Err(format!(
            "{name} carries no resume chronicle — cross-world deeds reach CLI-saved worlds (save there first)"
        ));
    }
    let chronicle = cfe::persist::load(&path)
        .map_err(|refusal| format!("{name}'s chronicle refused to verify: {refusal:?}"))?;
    // MESH-1b (#2): THE WALK RETIRED THE GUARD. A sealed arbor names
    // its one lawful hand; ours writes, any other refuses by name. A
    // pre-mesh chronicle is lawful for its single writer — so a
    // conductor world still waits for its first seal (`pen <world>`
    // asks; the conductor seals at its next beat boundary).
    match mesh::authority(&chronicle) {
        Err(violation) => Err(format!("{name}'s arbor refuses — {violation:?}")),
        Ok(Some(hand)) if hand == seat_hand() => {
            let mut world = World::resume(chronicle);
            world.hold_pen(Some(hand));
            Ok(world)
        }
        Ok(Some(hand)) if hand == cfe::vrp::CONDUCTOR_HAND => Err(format!(
            "{name}'s pen is with the conductor — ask with: pen {name}"
        )),
        Ok(Some(hand)) => Err(format!(
            "{name}'s pen is with hand {hand} — one writer per clock (NS-5 law 3)"
        )),
        Ok(None) if conductor_founded(name) => Err(format!(
            "{name} beats under the conductor's clock — ask for the pen with: pen {name}"
        )),
        Ok(None) => Ok(World::resume(chronicle)),
    }
}

/// Pre-mesh ownership: does this world beat under the conductor?
///
/// **No state.json.** Authority is chronicle.tlv (MESH pen) or the
/// conductor-written `spine/` arbor directory. Display files never
/// decide who holds the clock.
fn conductor_founded(name: &str) -> bool {
    let world_dir = std::path::PathBuf::from("var/worlds").join(name);
    let tlv = world_dir.join("chronicle.tlv");
    if let Ok(chronicle) = cfe::persist::load(&tlv) {
        match mesh::authority(&chronicle) {
            Ok(Some(hand)) if hand == cfe::vrp::CONDUCTOR_HAND => return true,
            Ok(Some(_)) => return false, // another hand holds the pen
            _ => {}
        }
    }
    // Conductor-written arbor spine — not a display file.
    world_dir.join("spine").is_dir()
}

/// The active profile's name — the seat whose hand signs (MESH-1b).
fn active_name() -> String {
    std::fs::read_to_string("var/operators/active")
        .ok()
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "player".into())
}

fn seat_hand() -> u64 {
    cfe::vrp::hand_of(&active_name())
}

/// MARKET-2: a world's charted position.
///
/// Topology export (`starchart.card`) is a **chart cache** of
/// facts — not colony banks. Prefer resume of the live world and
/// fall back to the chart file only as coordinates (never balance).
/// Uncharted reads as the origin.
fn charted_position(name: &str) -> (Ratio, Ratio, Ratio) {
    let zero = || cfe::ratio::ratio(0, 1);
    // Prefer desk/engine chart reader (same path as desk; not banks).
    let (x, y, z) = desk::charted_position(name);
    if x != zero() || y != zero() || z != zero() {
        return (x, y, z);
    }
    (zero(), zero(), zero())
}

/// Parse `p/q` or a plain integer (the lens's exact rendering).
// parse_ratio_text retired with data_field starchart scrape — desk::charted_position owns chart reads.

fn sync_world(world: &World) -> Result<usize, String> {
    let name = world.orb().map(|orb| orb.name.clone()).ok_or("no colony")?;
    sync_world_at(world, &name)
}

/// Sync to a NAMED world dir — a far world's dir is its WORLD name,
/// not its colony's (the conductor seats them apart; MESH-1b writes
/// must land where the conductor re-resumes from).
fn sync_world_at(world: &World, name: &str) -> Result<usize, String> {
    let path = std::path::PathBuf::from("var/worlds").join(name).join("chronicle.tlv");
    cfe::persist::sync(world.chronicle(), &path).map_err(|r| format!("{r:?}"))
}

fn command(world: &mut World, line: &str, style: &Style, current: &mut u64) {
    let mut words = line.split_whitespace();
    match words.next() {
        // Phase-1 trade desk (33 / TASKS_TRADE_FLEET_DESK P1).
        Some("desk") => {
            let pane = words
                .next()
                .and_then(desk::Pane::from_key)
                .unwrap_or(desk::Pane::Trade);
            desk_command(world, pane, style, *current, words);
        }
        Some("stamp") => stamp_command(world, words, style, *current),
        Some("element") => element_command(world, words, style, *current),
        Some("fleet") => fleet_command(world, words, style, *current),
        Some("help") => {
            let topic = words.next();
            help_menu::help(*style, topic);
        }
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
                Some("res") | Some("resonator") => Some(ZoneKind::Resonator),
                Some("regen") | Some("regenerator") => Some(ZoneKind::Regenerator),
                Some("yard") | Some("shipyard") | Some("ship-yard") => Some(ZoneKind::ShipYard),
                // #41: a MINTED building, named by the word its mint
                // gave it. The fold is the authority on what a world
                // can build — a compiled name is matched first so a
                // mint can never shadow a standing kind.
                Some(word) => cfe::vocabulary::fold(
                    world.chronicle(),
                    cfe::vocabulary::Axis::Zone,
                )
                .iter()
                .find(|entry| entry.word == word && entry.code > cfe::dop::ZONE_KINDS)
                .map(|entry| ZoneKind::Minted(entry.code)),
                None => None,
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
                            style.strong(&zone_title(world, kind)),
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
            let albedo = words
                .next()
                .and_then(parse_ratio_arg)
                .and_then(cfe::dop::Unit::new);
            let orbit_r = words
                .next()
                .and_then(parse_ratio_arg)
                .and_then(cfe::dop::Positive::new);
            match (albedo, orbit_r) {
                (Some(albedo), Some(orbit_r)) => {
                    match world.site_planet(*current, albedo, orbit_r, None, None) {
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
        Some("scan") => {
            let exemplar = match words.next() {
                Some(name) => world
                    .orbs()
                    .iter()
                    .find(|held| held.name == name)
                    .map(|held| held.orb_id),
                None => Some(*current),
            };
            match exemplar {
                Some(exemplar) => scan(world, style, *current, exemplar),
                None => println!(
                    "{} no planet by that name — {} lists them; bare {} scans for this planet's twins",
                    style.bad("✗"),
                    style.strong("planets"),
                    style.strong("scan")
                ),
            }
        }
        Some("succeed") => match words.next() {
            Some(successor) => match world.succeed(successor) {
                Ok(()) => println!(
                    "{} the line passes — {} adopts this chronicle as its own substrate; the tree returns to the kernel, the record stands forever",
                    style.ok("⏚"),
                    style.strong(successor)
                ),
                Err(refusal) => refuse(world, &refusal, style),
            },
            None => println!(
                "{} usage: {} — a successor line adopts a DERELICT world's chronicle (10 §3c)",
                style.bad("✗"),
                style.strong("succeed <line>")
            ),
        },
        Some("prospect") => match words.next().and_then(|word| word.parse::<u8>().ok()) {
            Some(stratum) => match world.prospect(*current, stratum) {
                Ok(struck) => println!(
                    "{} {} read from stratum {} — family {}, tier {}, richness {}/tick ({}…)",
                    style.ok("⛏"),
                    style.strong(&struck.name),
                    stratum,
                    struck.family,
                    struck.tier,
                    style.strong(&amount(&struck.richness)),
                    &hex_prefix(&struck.element)
                ),
                Err(refusal) => refuse(world, &refusal, style),
            },
            None => println!(
                "{} usage: {} — read one stratum of what the seed always held",
                style.bad("✗"),
                style.strong("prospect <stratum>")
            ),
        },
        Some("assay") => match words.next() {
            Some(prefix) => {
                let found = world
                    .committed()
                    .prospects
                    .iter()
                    .find(|vein| vein.orb == *current && hex_prefix(&vein.element).starts_with(prefix))
                    .map(|vein| vein.element);
                match found {
                    Some(element) => match world.assay(*current, element) {
                        Ok(drawn) => {
                            println!(
                                "{} {} drawn from the opened vein — one receipt, the whole span",
                                style.ok("⚖"),
                                style.strong(&amount(&drawn))
                            );
                            // ECO-1 (#40): the ground answers back. The
                            // wear settled in that same receipt, so the
                            // player sees what the draw cost the spring
                            // at the moment they take it.
                            let after = world
                                .committed()
                                .prospects
                                .iter()
                                .find(|vein| vein.orb == *current && vein.element == element)
                                .map(|vein| vein.richness.clone());
                            let natural = world.committed().star.map(|star| {
                                let (_, _, natural) =
                                    cfe::dop::element_properties(&element, star);
                                natural
                            });
                            if let (Some(after), Some(natural)) = (after, natural) {
                                println!(
                                    "  {} the spring now runs at {} of {}/tick natural — {}",
                                    style.dim("↳"),
                                    style.strong(&amount(&after)),
                                    amount(&natural),
                                    match after < natural {
                                        true => style.dim(
                                            "a regenerator on this orb restores it between draws",
                                        ),
                                        false => style.dim("still running at its nature"),
                                    }
                                );
                            }
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    None => println!(
                        "{} no opened vein here starts with {} — {} lists nothing yet; prospect first",
                        style.bad("✗"),
                        style.strong(prefix),
                        style.strong("status")
                    ),
                }
            }
            None => println!(
                "{} usage: {} — draw richness × elapsed from an opened vein",
                style.bad("✗"),
                style.strong("assay <element-prefix>")
            ),
        },
        // Root hubs: BENCH · INTEL · TRADE · DIPLOMATIC · POLITY (optional 5th).
        Some("bench") => {
            let rest: Vec<String> = words.map(|w| w.to_string()).collect();
            hubs::bench(world, rest.into_iter(), style, *current);
        }
        Some("intel") | Some("intelligence") | Some("kb") => {
            let rest: Vec<String> = words.map(|w| w.to_string()).collect();
            hubs::intel(world, rest.into_iter(), style, *current);
        }
        Some("diplomatic")
        | Some("diplomacy")
        | Some("diplomatics")
        | Some("diplo") => {
            let rest: Vec<String> = words.map(|w| w.to_string()).collect();
            hubs::diplo(world, rest.into_iter(), style, *current);
        }
        Some("polity") | Some("line") | Some("identity") => {
            let rest: Vec<String> = words.map(|w| w.to_string()).collect();
            hubs::polity(world, rest.into_iter(), style, *current);
        }
        Some("chemistry") | Some("chem") => {
            let mut rest: Vec<String> = vec!["chem".into()];
            rest.extend(words.map(|w| w.to_string()));
            hubs::bench(world, rest.into_iter(), style, *current);
        }
        Some("biology") | Some("bio") => {
            let mut rest: Vec<String> = vec!["biology".into()];
            rest.extend(words.map(|w| w.to_string()));
            hubs::bench(world, rest.into_iter(), style, *current);
        }
        Some("physics") | Some("phys") => {
            let mut rest: Vec<String> = vec!["physics".into()];
            rest.extend(words.map(|w| w.to_string()));
            hubs::bench(world, rest.into_iter(), style, *current);
        }
        Some("math") | Some("maths") | Some("mathematics") => {
            let mut rest: Vec<String> = vec!["math".into()];
            rest.extend(words.map(|w| w.to_string()));
            hubs::bench(world, rest.into_iter(), style, *current);
        }
        Some("science") | Some("sci") => {
            let mut rest: Vec<String> = vec!["science".into()];
            rest.extend(words.map(|w| w.to_string()));
            hubs::bench(world, rest.into_iter(), style, *current);
        }
        Some("iso") | Some("union") | Some("iso-union") => {
            let a = words.next();
            let b = words.next();
            match (a, b) {
                (Some(pa), Some(pb)) => {
                    let committed = world.committed();
                    let held = committed.orbs.iter().find(|orb| orb.orb_id == *current);
                    let resolve = |prefix: &str| {
                        held.and_then(|orb| {
                            orb.elements
                                .iter()
                                .map(|(e, _)| *e)
                                .find(|e| hex_prefix(e).starts_with(prefix))
                        })
                    };
                    match (resolve(pa), resolve(pb)) {
                        (Some(ga), Some(gb)) => match world.iso_union(*current, ga, gb) {
                            Ok(product) => {
                                let _ = sync_world(world);
                                println!(
                                    "{} isomorphic union → {} (same shell/phase family)",
                                    style.ok("◈"),
                                    style.strong(&hex_prefix(&product)[..8])
                                );
                            }
                            Err(refusal) => refuse(world, &refusal, style),
                        },
                        _ => println!(
                            "{} need two banked gem prefixes — `e` lists elements",
                            style.bad("✗")
                        ),
                    }
                }
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("iso <gem-prefix> <gem-prefix>")
                ),
            }
        }
        Some("catalog") | Some("chem-catalog") => {
            let prefix = words.next();
            match prefix {
                Some(p) => {
                    let committed = world.committed();
                    let held = committed.orbs.iter().find(|orb| orb.orb_id == *current);
                    let gem = held.and_then(|orb| {
                        orb.elements
                            .iter()
                            .map(|(e, _)| *e)
                            .find(|e| hex_prefix(e).starts_with(p))
                    });
                    match gem {
                        Some(g) => match world.chem_catalog_gem(*current, g) {
                            Ok(k) => {
                                let _ = sync_world(world);
                                println!(
                                    "{} cataloged {} · knowledge {k} (crystal fee · science known)",
                                    style.ok("◈"),
                                    style.strong(&hex_prefix(&g)[..8])
                                );
                            }
                            Err(refusal) => refuse(world, &refusal, style),
                        },
                        None => println!(
                            "{} no banked gem matches {} — compound first",
                            style.bad("✗"),
                            p
                        ),
                    }
                }
                None => {
                    let cat = world.chem_catalog();
                    println!(
                        "{} CHEMISTRY CATALOG — {} known",
                        style.head("CHEM"),
                        cat.len()
                    );
                    for g in cat {
                        println!("  · {}", &hex_prefix(g)[..8]);
                    }
                    println!(
                        "  {}",
                        style.hint("catalog <gem-prefix> seals a banked gem as known chemistry")
                    );
                }
            }
        }
        Some("compound") => {
            let prefixes: Vec<String> = words.map(|word| word.to_string()).collect();
            if prefixes.len() < 2 {
                println!(
                    "{} usage: {} — bond banked elements into a gem ({} lists what would)",
                    style.bad("✗"),
                    style.strong("compound <prefix> <prefix> …"),
                    style.strong("bench")
                );
            } else {
                let committed = world.committed();
                let held = committed.orbs.iter().find(|orb| orb.orb_id == *current);
                let resolved: Option<Vec<[u8; 32]>> = prefixes
                    .iter()
                    .map(|prefix| {
                        held.and_then(|orb| {
                            orb.elements
                                .iter()
                                .map(|(element, _)| *element)
                                .find(|element| hex_prefix(element).starts_with(prefix.as_str()))
                        })
                    })
                    .collect();
                match resolved {
                    Some(members) => match world.compound_as(cfe::world::SOVEREIGN, *current, &members) {
                        Ok(gem) => {
                            println!(
                                "{} {} bond into {} — the lanes saturate and the matter holds",
                                style.ok("◈"),
                                style.strong(&format!("{} elements", members.len())),
                                style.strong(&hex_prefix(&gem)[..8])
                            );
                            if let Some(set) = cfe::almanac::Compoundable::new(&members) {
                                println!(
                                    "  {} knowledge {} won — this recipe works at every line of \
                                     the same lanes",
                                    style.dim("↳"),
                                    style.strong(&set.knowledge().to_string())
                                );
                            }
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    None => println!(
                        "{} this orb banks nothing matching one of those prefixes — {} lists \
                         what it holds",
                        style.bad("✗"),
                        style.strong("elements")
                    ),
                }
            }
        }
        // THE CORPUS BENCH (#38a): cognition as gameplay. The economy's
        // second head, finally reachable by a player.
        Some("corpus") => corpus_command(world, words, style, *current),
        // #42 THE POLITICAL SPECTRUM — trade · defense · peace · war.
        Some("treaty") | Some("treaties") | Some("pact") => {
            treaty_command(world, words.map(str::to_string), style)
        }
        // 20_FOUNDING §1 step 3: "offered a LIST OF TYPES; picks the
        // one most interesting to them" — bare `genesis` (no args at
        // all) launches the guided picker instead of demanding all
        // four typed arguments up front. `genesis <civ> <disp> <met>
        // <perc>` still works unchanged for anyone who already knows
        // what they want.
        Some("genesis") => {
            let civilization = words.next();
            let axes = (
                words.next().and_then(parse_disposition),
                words.next().and_then(parse_metabolism),
                words.next().and_then(parse_perception),
            );
            match (civilization, axes) {
                (Some(civilization), (Some(disposition), Some(metabolism), Some(perception))) => {
                    match world.seal_genesis(civilization, disposition, metabolism, perception) {
                        Ok(()) => println!(
                            "{} {} seals its three axes — the binding copy lives in THIS chronicle; the registry is a cache from here on",
                            style.ok("✶"),
                            style.strong(civilization)
                        ),
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
                (None, (None, None, None)) => genesis_picker(world, style),
                _ => println!(
                    "{} usage: {} — e.g. {}, or type {} alone for a guided pick",
                    style.bad("✗"),
                    style.strong("genesis <civ> <disposition> <metabolism> <perception>"),
                    style.strong("genesis deep-chorus epicurean biotic harmonic"),
                    style.strong("genesis")
                ),
            }
        }
        // #32, 20_FOUNDING §3: what every founding tithe has paid, by
        // seat — a fold over the chronicle's TitheReceipted receipts,
        // never a stored balance (world.seat_treasury).
        Some("treasury") => {
            for (label, seat) in [("elder", cfe::dop::SEAT_ELDER), ("ancient", cfe::dop::SEAT_ANCIENT)] {
                let held = world.seat_treasury(seat);
                if held.is_empty() {
                    println!("{} the {} treasury holds nothing yet", style.dim("○"), style.strong(label));
                } else {
                    let listed: Vec<String> = held
                        .iter()
                        .map(|(resource, value)| format!("{} {}", amount(value), resource_name(*resource)))
                        .collect();
                    println!(
                        "{} the {} treasury holds {}",
                        style.ok("✶"),
                        style.strong(label),
                        style.strong(&listed.join(", "))
                    );
                }
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
        // NPC CONTROLS (2026-07-30 ruling): the player's hand on the
        // autonomous board — writes var/conductor.ctl, which the
        // conductor polls every beat. Pause holds every polity world
        // (nothing ticks, nothing ages); speed sets the seconds
        // between beats (0 = flat out).
        // THE UNION VERBS (wiring audit, 2026-07-30): every landed
        // deed reachable from the hand.
        // MARKET-1: post a resting intent; clear a crossing with a
        // far world (both offers named — the resting price rules).
        Some("offer") => {
            // offer <give_amt> <give> for <want_amt> <want> id <n>
            let give_amount = words.next().and_then(parse_ratio_arg);
            let give = words.next().and_then(parse_resource);
            let want_amount = match (words.next(), words.next().and_then(parse_ratio_arg)) {
                (Some("for"), Some(amt)) => Some(amt),
                _ => None,
            };
            let want = words.next().and_then(parse_resource);
            let offer = match (words.next(), words.next()) {
                (Some("id"), Some(n)) => n.parse::<u64>().ok(),
                _ => None,
            };
            match (give_amount, give, want_amount, want, offer) {
                (Some(ga), Some(g), Some(wa), Some(w), Some(id)) => {
                    let (Some(pair), Some(ga), Some(wa)) = (
                        cfe::dop::Exchange::new(g, w),
                        cfe::dop::Positive::new(ga),
                        cfe::dop::Positive::new(wa),
                    ) else {
                        println!(
                            "{} an offer trades two DISTINCT resources in positive amounts — e.g. {}",
                            style.bad("✗"),
                            style.strong("offer 4 iron for 2 biomass id 1")
                        );
                        return;
                    };
                    match world.post_offer(*current, pair, ga, wa, id) {
                        Ok(()) => println!(
                            "{} offer {} RESTS on the board — the price is the ratio; a crossing clears it",
                            style.ok("⇋"),
                            style.strong(&id.to_string())
                        ),
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("offer <give_amt> <give> for <want_amt> <want> id <n>")
                ),
            }
        }
        Some("trade") | Some("trades") | Some("tmenu") => {
            // Barter form: trade <offer_id> with <world> …
            // Otherwise: TRADE hub (books · fleet · rails · vault · aperture).
            let rest: Vec<String> = words.map(|w| w.to_string()).collect();
            let barter = rest
                .first()
                .and_then(|w| w.parse::<u64>().ok())
                .is_some();
            if barter {
                let mut it = rest.iter().map(|s| s.as_str());
                let mine = it.next().and_then(|w| w.parse::<u64>().ok());
                let target = match (it.next(), it.next()) {
                    (Some("with"), Some(name)) => Some(name.to_string()),
                    _ => None,
                };
                let theirs = it.next().and_then(|w| w.parse::<u64>().ok());
                let deal = match (it.next(), it.next()) {
                    (Some("deal"), Some(n)) => n.parse::<u64>().ok(),
                    _ => None,
                };
                match (mine, target, theirs, deal) {
                    (Some(mine), Some(target), Some(theirs), Some(deal)) => {
                        match far_world(&target) {
                            Ok(mut far) => {
                                let my_name =
                                    world.orb().map(|o| o.name.clone()).unwrap_or_default();
                                let distance = conductor::spatial_l1(
                                    &charted_position(&my_name),
                                    &charted_position(&target),
                                );
                                match cfe::world::clear_trade(
                                    world, mine, &mut far, theirs, deal, distance,
                                ) {
                                    Ok(()) => match sync_world(world)
                                        .and(sync_world_at(&far, &target))
                                    {
                                        Ok(_) => println!(
                                            "{} the crossing CLEARS — two aperture pairs carried the fill; both chronicles sealed",
                                            style.ok("⇌")
                                        ),
                                        Err(err) => println!(
                                            "{} the fill landed but a sync refused: {err}",
                                            style.bad("✗")
                                        ),
                                    },
                                    Err(refusal) => refuse(world, &refusal, style),
                                }
                            }
                            Err(err) => println!("{} {err}", style.bad("✗")),
                        }
                    }
                    _ => println!(
                        "{} usage: {} · or open hub: {}",
                        style.bad("✗"),
                        style.strong(
                            "trade <my_offer> with <world> <their_offer> deal <n>"
                        ),
                        style.key("trade")
                    ),
                }
            } else {
                hubs::trade(world, rest.into_iter(), style, *current);
            }
        }
        // SCIENCE-1: the JOINT RESEARCH VENTURE — the formal deed for
        // incompatible sciences: a council-bound licensed leap named
        // as what it is. venture <cond> to <world> via <council> deal
        // <n> — sugar over license with the venture's meaning.
        Some("venture") => {
            let rest: Vec<&str> = words.collect();
            let relayed = format!("license {}", rest.join(" "));
            println!(
                "{}",
                style.dim("the venture: two sciences cross what neither tree holds alone")
            );
            command(world, &relayed, style, current);
        }
        // GAUGE-N: the graded shift — a progenitor's quantum-state
        // deed: move (or OPEN) the key at any grade; the whole key
        // renders after.
        Some("shift") => {
            if !tier0_gate(style) {
                return;
            }
            let grade = words.next().and_then(|w| w.parse::<u16>().ok());
            let value = words.next().and_then(parse_ratio_arg);
            match (grade, value) {
                (Some(grade), Some(value)) => match world.shift_gauge_at(grade, value) {
                    Ok(()) => {
                        let key: Vec<String> =
                            world.key().iter().map(amount).collect();
                        println!(
                            "{} the key plays [{}] — {} plane(s) open",
                            style.ok("♫"),
                            style.strong(&key.join(", ")),
                            world.key().len()
                        );
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                },
                _ => println!(
                    "{} usage: {} — grade 0 is the landed dial; a higher grade OPENS a plane",
                    style.bad("✗"),
                    style.strong("shift <grade> <value>")
                ),
            }
        }
        Some("elevate") => match words
            .next()
            .and_then(parse_ratio_arg)
            .and_then(cfe::dop::Positive::new)
        {
            Some(delta) => match world.elevate(delta) {
                Ok(()) => println!(
                    "{} the climb — this world's key now plays at {}",
                    style.ok("✶"),
                    style.strong(&amount(world.gauge()))
                ),
                Err(refusal) => refuse(world, &refusal, style),
            },
            None => println!(
                "{} a climb moves strictly upward in G — usage: {}",
                style.bad("✗"),
                style.strong("elevate <delta>")
            ),
        },
        Some("age") => {
            if !tier0_gate(style) {
                return;
            }
            match words.next().map(parse_star) {
            Some(Some(class)) => match world.age_star(class) {
                Ok(()) => println!(
                    "{} the star MOVES in its region — every sited orbit's band shifts at the next tick",
                    style.ok("☀")
                ),
                Err(refusal) => refuse(world, &refusal, style),
            },
            _ => println!("{} usage: {} — o b a f g k m bd wd rg", style.bad("✗"), style.strong("age <class>")),
        }
        }
        Some("council") => {
            // council <id> <res> <quota> <window> <stake> with <p1>,<p2>,<p3>
            let id = words.next().and_then(|w| w.parse::<u64>().ok());
            let resource = words.next().and_then(parse_resource);
            let quota = words.next().and_then(parse_ratio_arg);
            let window = words.next().and_then(|w| w.parse::<u64>().ok());
            let stake = words.next().and_then(parse_ratio_arg);
            let members: Vec<u64> = match (words.next(), words.next()) {
                (Some("with"), Some(list)) => list
                    .split(',')
                    .filter_map(|name| {
                        world.orbs().iter().find(|held| held.name == name).map(|held| held.orb_id)
                    })
                    .collect(),
                _ => Vec::new(),
            };
            match (id, resource, quota, window, stake) {
                (Some(id), Some(resource), Some(quota), Some(window), Some(stake))
                    if !members.is_empty() =>
                {
                    let (Some(members), Some(quota), Some(window), Some(stake)) = (
                        cfe::dop::Among::new(members),
                        cfe::dop::Positive::new(quota),
                        cfe::dop::Window::new(window),
                        cfe::dop::Positive::new(stake),
                    ) else {
                        println!(
                            "{} a council is three or more distinct planets, a positive quota and collateral, a real window — e.g. {}",
                            style.bad("✗"),
                            style.strong("council 1 with base,outpost,haven: 5 iron in 60, stake 3")
                        );
                        return;
                    };
                    match world.seal_council(id, members, resource, quota, window, stake) {
                        Ok(()) => println!(
                            "{} council {} SEALS — every member bound alike; the reckoning judges all together at the boundary",
                            style.ok("⚭"),
                            style.strong(&id.to_string())
                        ),
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("council <id> <res> <quota> <window> <stake> with <p1>,<p2>,<p3>")
                ),
            }
        }
        Some("aperture") => {
            // aperture <amt> <res> to <world> deal <n>
            let amt = words.next().and_then(parse_ratio_arg);
            let resource = words.next().and_then(parse_resource);
            let target = match (words.next(), words.next()) {
                (Some("to"), Some(name)) => Some(name.to_string()),
                _ => None,
            };
            let deal = match (words.next(), words.next()) {
                (Some("deal"), Some(n)) => n.parse::<u64>().ok(),
                _ => None,
            };
            match (amt, resource, target, deal) {
                (Some(amt), Some(resource), Some(target), Some(deal)) => {
                    match far_world(&target) {
                        Ok(mut far) => {
                            let Some(amt) = cfe::dop::Positive::new(amt) else {
                                println!("{} cargo must be positive — zero moves nothing and a negative is not a quantity", style.bad("✗"));
                                return;
                            };
                            match cfe::world::aperture_transfer(
                                world, *current, &mut far, cfe::world::FIRST_ORB,
                                resource, amt, deal,
                            ) {
                                Ok(()) => {
                                    let saved = sync_world(world).and(sync_world_at(&far, &target));
                                    match saved {
                                        Ok(_) => println!(
                                            "{} the aperture OPENS — deal {} crossed the void; both chronicles sealed",
                                            style.ok("⎋"),
                                            style.strong(&deal.to_string())
                                        ),
                                        Err(err) => println!(
                                            "{} the deed landed but a sync refused: {err}",
                                            style.bad("✗")
                                        ),
                                    }
                                }
                                Err(refusal) => refuse(world, &refusal, style),
                            }
                        }
                        Err(err) => println!("{} {err}", style.bad("✗")),
                    }
                }
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("aperture <amt> <res> to <world> deal <n>")
                ),
            }
        }
        Some("license") => {
            if !tier0_gate(style) {
                return;
            }
            // license <condition 1..6> to <world> via <council> deal <n>
            let condition = words.next().and_then(|w| w.parse::<u8>().ok());
            let target = match (words.next(), words.next()) {
                (Some("to"), Some(name)) => Some(name.to_string()),
                _ => None,
            };
            let council = match (words.next(), words.next()) {
                (Some("via"), Some(n)) => n.parse::<u64>().ok(),
                _ => None,
            };
            let deal = match (words.next(), words.next()) {
                (Some("deal"), Some(n)) => n.parse::<u64>().ok(),
                _ => None,
            };
            match (condition, target, council, deal) {
                (Some(condition), Some(target), Some(council), Some(deal)) => {
                    let concept = [
                        "the solar leap", "insulation", "discipline", "deepening",
                        "doctrine", "stores", "terraforming",
                    ]
                    .get(condition as usize)
                    .copied()
                    .unwrap_or("working");
                    match far_world(&target) {
                        Ok(mut far) => {
                            let leap = cfe::world::Leap {
                                condition,
                                name: format!("granted {concept}"),
                                deal,
                                council,
                            };
                            match cfe::world::license_tech(
                                world, *current, &mut far, cfe::world::FIRST_ORB, leap,
                            ) {
                                Ok(()) => {
                                    let saved = sync_world(world).and(sync_world_at(&far, &target));
                                    match saved {
                                        Ok(_) => println!(
                                            "{} {} LICENSED into {} — the far sky learns; both chronicles sealed{}",
                                            style.ok("⌁"),
                                            style.strong(concept),
                                            style.strong(&target),
                                            if condition == 6 {
                                                " — THE CROWN IS GIFTED"
                                            } else {
                                                ""
                                            }
                                        ),
                                        Err(err) => println!(
                                            "{} the deed landed but a sync refused: {err}",
                                            style.bad("✗")
                                        ),
                                    }
                                }
                                Err(refusal) => refuse(world, &refusal, style),
                            }
                        }
                        Err(err) => println!("{} {err}", style.bad("✗")),
                    }
                }
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("license <condition 1..6> to <world> via <council> deal <n>")
                ),
            }
        }
        // THE ASCENSION (the second-look audit, 2026-07-30): the deed
        // upward advancement was missing — a line that leapt, crossed,
        // and climbed can now CLAIM its seat. The world's own receipts
        // are the evidence (crossed + at least one receipted climb);
        // the registry checks the room (space, never rank).
        Some("ascend") => {
            let target = words.next().map(str::to_string);
            let station = match (words.next(), words.next()) {
                (Some("to"), Some(word)) => Some(word.to_string()),
                _ => None,
            };
            match (target, station) {
                (Some(sigil), Some(station)) => {
                    if !world.has_crossed() {
                        println!(
                            "{} the seat is reached, never appointed — this world has not crossed the void ({} first)",
                            style.bad("✗"),
                            style.strong("aperture …")
                        );
                        return;
                    }
                    if world.gauge() == &cfe::ratio::ratio(0, 1) {
                        println!(
                            "{} the seat is reached through the climb — no receipted ascent stands ({} first)",
                            style.bad("✗"),
                            style.strong("elevate <delta>")
                        );
                        return;
                    }
                    // T1.2 — typed preconditions name the cure (facts of
                    // the chain; seat width stays the registry door).
                    let evidence = {
                        use cfe::dop::ClosureEvent;
                        use ledger::registry::OfficeEvidence;
                        let mut evidence = OfficeEvidence::default();
                        if let Some(orb) = world.orb() {
                            evidence.element_kinds = orb.elements.len() as u64;
                        }
                        for receipt in world.chronicle().receipts() {
                            match &receipt.event {
                                ClosureEvent::AssayStruck { .. } => evidence.assays_struck += 1,
                                ClosureEvent::RouteChartered { .. } => {
                                    evidence.routes_chartered += 1
                                }
                                ClosureEvent::OfferCleared { .. } => evidence.market_clears += 1,
                                _ => {}
                            }
                        }
                        evidence.has_crossed = world.has_crossed();
                        evidence
                    };
                    if let Err(cure) =
                        ledger::registry::office_preconditions(&station, &evidence)
                    {
                        println!(
                            "{} the {} office refuses — {}",
                            style.bad("✗"),
                            style.strong(&station),
                            style.dim(cure)
                        );
                        return;
                    }
                    let registry = ledger::registry::Registry::at("var/registry");
                    match registry.seat(&sigil, &station) {
                        Ok(()) => println!(
                            "{} {} ASCENDS — the {} seat is taken; a seat reached, never appointed",
                            style.ok("✶"),
                            style.strong(&sigil),
                            style.strong(&station)
                        ),
                        Err(ledger::registry::RegistryRefused::NoSeatOpen {
                            station, domain, held, width,
                        }) => println!(
                            "{} the {station} room for the {domain} domain stands FULL ({held}/{width}) — the band grows as the universe does",
                            style.bad("✗")
                        ),
                        Err(refusal) => println!("{} the ascension refused: {refusal:?}", style.bad("✗")),
                    }
                }
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("ascend <sigil> to <station>")
                ),
            }
        }
        // THE CONSTELLATION — this line's own sky, BASE-11 since the
        // uplift (operator ruling 2026-08-02). Held nodes glow with
        // their voice-names, timers show exact progress, the crown
        // shows its gate, and the uplift tier shows the deeds claimed.
        // Per-line, never a shared ladder.
        Some("tech") => {
            // The voice-names live in the receipts (typed events —
            // read, never parsed).
            let mut voices: Vec<(u8, String, u64)> = Vec::new();
            for receipt in world.chronicle().receipts() {
                if let cfe::dop::ClosureEvent::TechDiscovered {
                    orb_id, condition, name, ..
                } = &receipt.event
                {
                    if *orb_id == *current {
                        voices.push((*condition, name.clone(), receipt.stamp));
                    }
                }
            }
            // ELEVEN sky nodes: the base tier as it always was, then
            // one per chartered work. INDEXED BY CONDITION, so slot 7
            // must be held even though the sky never emits it — and the
            // array must stay as long as the highest condition, not as
            // long as the sky.
            let concepts = [
                "the solar leap", "insulation", "discipline", "deepening",
                "doctrine", "stores", "the crown (terraforming)",
                "the re-emergence lesson (rides separately)",
                "VALOR", "HONOR", "COURAGE", "COMMITMENT",
            ];
            println!("{}", style.head("THE CONSTELLATION — this line's own sky"));
            for (condition, held, progress, threshold) in world.constellation(*current) {
                let concept = concepts[condition as usize];
                // The uplift tier reads from DEEDS, not from timers, so
                // it renders on its own terms.
                if condition >= 8 {
                    if condition == 8 {
                        println!(
                            "  {}",
                            style.dim("— the uplift tier: deeds, claimed on chain receipts —")
                        );
                    }
                    let reached = world
                        .orb_by_id(*current)
                        .is_some_and(|orb| {
                            orb.aspirations.iter().any(|held| {
                                held.work == cfe::literature::work_of(concept)
                            })
                        });
                    println!(
                        "  {} {:<28} {}",
                        match held {
                            true => style.strong("◆"),
                            false => style.dim("○"),
                        },
                        concept,
                        match (held, reached) {
                            (true, _) => style.ok("CLAIMED — a civilizational upgrade"),
                            (false, true) => style.dim("reached for; the deed still has to be witnessed").to_string(),
                            (false, false) => style.dim("unreached").to_string(),
                        }
                    );
                    continue;
                }
                let voice = voices.iter().find(|(c, _, _)| *c == condition);
                let line = match (held, voice) {
                    (true, Some((_, name, stamp))) => format!(
                        "  {} {:<28} held t{stamp}",
                        if condition == 6 { style.strong("◆") } else { style.ok("✶") },
                        style.strong(name)
                    ),
                    (true, None) => format!("  {} {concept:<28} held", style.ok("✶")),
                    (false, _) if threshold > 1 && condition != 6 => {
                        let filled = ((progress * 12) / threshold) as usize;
                        format!(
                            "  {} {:<28} [{}{}] {}/{} ticks",
                            style.dim("○"),
                            concept,
                            "=".repeat(filled),
                            style.dim(&"—".repeat(12 - filled)),
                            progress,
                            threshold
                        )
                    }
                    (false, _) if condition == 6 => format!(
                        "  {} {:<28} base-7 {}/6 — {}",
                        style.dim("◇"),
                        concept,
                        progress,
                        if progress >= 6 {
                            style.strong("the stack stands; the gift awaits tier 0")
                        } else {
                            style.dim("earn the stack; the crown is only ever gifted")
                        }
                    ),
                    (false, _) if progress >= threshold => format!(
                        "  {} {:<28} {}",
                        style.ok("◉"),
                        concept,
                        style.strong("the reading HOLDS — crystallizes at the year voice")
                    ),
                    (false, _) => format!(
                        "  {} {concept:<28} {}",
                        style.dim("○"),
                        style.dim("the reading is not yet met")
                    ),
                };
                println!("{line}");
            }
            println!(
                "{}",
                style.dim("  a missing condition another line holds is a joint-venture prompt — council + aperture + licensed leap")
            );
        }
        // TERRA-1: the crown in the hand — re-seat a sited planet's
        // albedo and orbit; only lawful once terraforming is held.
        // MANU-1 (10 §3f): forge an instrument — manufacturing is
        // technology + understanding + industry; grade = condition,
        // and the scan horizon reads the best grade held.
        // The grade is proven at THIS door (29_CONSTRUCTION): the
        // vocabulary decides forgeability, so an unknown grade never
        // reaches the verb.
        Some("forge") => match words
            .next()
            .and_then(|w| w.parse::<u8>().ok())
            .map(|condition| (condition, cfe::world::survey::ForgeableGrade::new(condition)))
        {
            Some((condition, Some(grade))) => {
                match world.forge_instrument(cfe::world::FIRST_ORB, grade) {
                    Ok(()) => println!(
                        "{} the grade-{} instrument FORGES — the scan horizon grows",
                        style.ok("⚙"),
                        style.strong(&condition.to_string())
                    ),
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some((condition, None)) => println!(
                "{} {} is not a forgeable grade — the vocabulary says crystallized or crown, and only those construct",
                style.bad("✗"),
                style.strong(&condition.to_string())
            ),
            None => println!(
                "{} usage: {} — grade is a crystallized or crown condition",
                style.bad("✗"),
                style.strong("forge <condition>")
            ),
        },
        // ROUTE-1: charter a lane to a far world — infrastructure
        // that bends the iceberg; η derives from the pair's best
        // instrument grade (built, never granted).
        Some("charter") => {
            let target = words.next().map(str::to_string);
            let deal = match (words.next(), words.next()) {
                (Some("deal"), Some(n)) => n.parse::<u64>().ok(),
                _ => None,
            };
            match (target, deal) {
                (Some(target), Some(deal)) => match far_world(&target) {
                    Ok(mut far) => {
                        let best_grade = world
                            .instruments()
                            .iter()
                            .chain(far.instruments().iter())
                            .map(|(_, _, grade)| *grade)
                            .max()
                            .unwrap_or(0);
                        let Some(eta) =
                            cfe::dop::Unit::new(ratio(1 + i64::from(best_grade), 8))
                        else {
                            println!(
                                "{} grade {best_grade} prices an efficiency of 1 or more — no lane erases the void",
                                style.bad("✗")
                            );
                            return;
                        };
                        match cfe::world::charter_route(
                            world,
                            cfe::world::FIRST_ORB,
                            &mut far,
                            cfe::world::FIRST_ORB,
                            eta,
                            deal,
                        ) {
                            Ok(()) => match sync_world(world).and(sync_world_at(&far, &target)) {
                                Ok(_) => println!(
                                    "{} the lane CHARTERS — both books sealed; the melt bends",
                                    style.ok("⇉")
                                ),
                                Err(err) => println!(
                                    "{} the charter landed but a sync refused: {err}",
                                    style.bad("✗")
                                ),
                            },
                            Err(refusal) => refuse(world, &refusal, style),
                        }
                    }
                    Err(err) => println!("{} {err}", style.bad("✗")),
                },
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("charter <world> deal <n>")
                ),
            }
        }
        Some("terraform") => {
            let albedo = words
                .next()
                .and_then(parse_ratio_arg)
                .and_then(cfe::dop::Unit::new);
            let orbit = words
                .next()
                .and_then(parse_ratio_arg)
                .and_then(cfe::dop::Positive::new);
            match (albedo, orbit) {
                (Some(albedo), Some(orbit)) => {
                    match world.terraform(*current, albedo, orbit) {
                        Ok(()) => {
                            println!(
                                "{} the orbit MOVES — the world re-seats under the crown",
                                style.ok("✓")
                            );
                            map(world, style, *current);
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
                _ => println!(
                    "{} usage: {} — exact rationals, 0 ≤ albedo < 1, orbit > 0",
                    style.bad("✗"),
                    style.strong("terraform <albedo> <orbit>")
                ),
            }
        }
        // MESH-1b (#2): THE PEN — the receipted handoff's CLI side.
        // `pen <world>` puts the ask on the conductor's control file;
        // the seal lands at the next beat boundary and the walk then
        // names THIS seat the one lawful hand. `pen return <world>`
        // yields it back; the conductor re-resumes from the receipts.
        Some("pen") => match words.next() {
            Some("return") => match words.next() {
                Some(target) => match far_world(target) {
                    Ok(mut far) => {
                        let stamp = far
                            .chronicle()
                            .receipts()
                            .last()
                            .map(|receipt| receipt.stamp)
                            .unwrap_or(0);
                        match mesh::yield_authority(
                            far.chronicle_mut(),
                            seat_hand(),
                            cfe::vrp::CONDUCTOR_HAND,
                            stamp,
                        ) {
                            Ok(sequence) => match sync_world_at(&far, target) {
                                Ok(_) => println!(
                                    "{} the pen returns to the conductor (seal {sequence}) — beats resume within a poll",
                                    style.ok("✔")
                                ),
                                Err(err) => println!(
                                    "{} the seal landed but the sync refused: {err}",
                                    style.bad("✗")
                                ),
                            },
                            Err(refused) => {
                                println!("{} the yield refused — {refused:?}", style.bad("✗"))
                            }
                        }
                    }
                    Err(err) => println!("{} {err}", style.bad("✗")),
                },
                None => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("pen return <world>")
                ),
            },
            Some(target) => {
                let control = std::path::PathBuf::from("var/conductor.ctl");
                let seat = active_name();
                let mut text = std::fs::read_to_string(&control).unwrap_or_default();
                if !text.is_empty() && !text.ends_with('\n') {
                    text.push('\n');
                }
                text.push_str(&format!("yield {target} {seat}\n"));
                match std::fs::write(&control, text) {
                    Ok(()) => println!(
                        "{} the ask is on the board — the conductor seals {} over to {} at its next beat; then your deeds write there directly ({})",
                        style.ok("✔"),
                        style.strong(target),
                        style.strong(&seat),
                        style.dim("pen return <world> when done")
                    ),
                    Err(err) => println!("{} the control file refused: {err}", style.bad("✗")),
                }
            }
            None => println!(
                "{} usage: {} · {}",
                style.bad("✗"),
                style.strong("pen <world>"),
                style.strong("pen return <world>")
            ),
        },
        Some("npcs") => {
            let control = std::path::PathBuf::from("var/conductor.ctl");
            let (mut paused, mut cadence) = {
                let mut paused = false;
                let mut cadence: Option<u64> = None;
                if let Ok(text) = std::fs::read_to_string(&control) {
                    for line in text.lines() {
                        let line = line.trim();
                        if line == "paused" {
                            paused = true;
                        } else if let Some(word) = line.strip_prefix("cadence ") {
                            cadence = word.trim().parse().ok();
                        }
                    }
                }
                (paused, cadence)
            };
            let mut advance: Option<u64> = None;
            let verb = words.next();
            match verb {
                Some("pause") => paused = true,
                Some("resume") | Some("start") => paused = false,
                // PUNCTUATED PLAY (task #11): run N beats flat out,
                // then the board parks itself paused — advance a
                // year, read the board, act, advance again.
                Some("advance") => match words.next().and_then(|w| w.parse::<u64>().ok()) {
                    Some(n) if n > 0 => {
                        advance = Some(n);
                        paused = false;
                    }
                    _ => {
                        println!(
                            "{} advance wants a beat count — e.g. {} (a year is 60)",
                            style.bad("✗"),
                            style.strong("npcs advance 60")
                        );
                        return;
                    }
                },
                Some("speed") => match words.next().and_then(|w| w.parse::<u64>().ok()) {
                    Some(secs) => cadence = Some(secs),
                    None => {
                        println!(
                            "{} speed wants seconds between beats — e.g. {} (0 = flat out)",
                            style.bad("✗"),
                            style.strong("npcs speed 5")
                        );
                        return;
                    }
                },
                Some(other) => {
                    println!(
                        "{} unknown npcs verb {} — pause · resume · speed <seconds> · advance <beats>",
                        style.bad("✗"),
                        style.strong(other)
                    );
                    return;
                }
                None => {
                    println!(
                        "npc board: {} · cadence {}",
                        if paused { style.bad("PAUSED") } else { style.ok("beating") },
                        match cadence {
                            Some(0) => style.strong("flat out"),
                            Some(n) => style.strong(&format!("{n}s")),
                            None => style.dim("conductor default"),
                        }
                    );
                    return;
                }
            }
            let mut text = String::new();
            // Pending `yield` asks (MESH-1b) and any other lines the
            // conductor understands survive the rewrite — this verb
            // owns paused/cadence/advance only (a stale advance must
            // not outlive the pause that countermands it).
            if let Ok(existing) = std::fs::read_to_string(&control) {
                for line in existing.lines() {
                    let line = line.trim();
                    if line == "paused"
                        || line.starts_with("cadence ")
                        || line.starts_with("advance ")
                        || line.is_empty()
                    {
                        continue;
                    }
                    text.push_str(line);
                    text.push('\n');
                }
            }
            if paused {
                text.push_str("paused\n");
            }
            if let Some(secs) = cadence {
                text.push_str(&format!("cadence {secs}\n"));
            }
            if let Some(beats) = advance {
                text.push_str(&format!("advance {beats}\n"));
            }
            match std::fs::write(&control, text) {
                Ok(()) => println!(
                    "{} npc board {} — the conductor reads it on its next beat",
                    style.ok("✓"),
                    if let Some(beats) = advance {
                        format!(
                            "will ADVANCE {} beat(s) flat out, then pause — read the board when it parks",
                            style.strong(&beats.to_string())
                        )
                    } else if paused {
                        style.bad("PAUSED").to_string()
                    } else {
                        match cadence {
                            Some(0) => "beating flat out".into(),
                            Some(n) => format!("beating every {n}s"),
                            None => "beating at the conductor's default".into(),
                        }
                    }
                ),
                Err(refusal) => println!("{} control write refused: {refusal}", style.bad("✗")),
            }
        }
        // THE CIV CONSOLE (2026-07-30 ruling): build a custom
        // civilization — axes, station, seat checks — from inside the
        // client. `civ` / `civ seats` shows the station console; `civ
        // new` registers the line; the conductor derives its world from
        // the registry card at its next boot (THE DEFINITION LAW).
        // THE OPERATOR MENU (prototype — no passwords): profiles under
        // var/operators/<sigil>, many accounts on one machine; login
        // switches, continue resumes the profile's last saved world.
        // THE PROGENITOR CONSOLE (tier 0 — the board-makers' hand):
        // inbox and requests flow UP from the lower dimensions; move
        // re-charts a world on the torus (the separation deed for
        // stressed sectors — warring factions part in S); contract is
        // the fold; clean sweeps the --closed-* ruins.
        Some("t0") => {
            if !tier0_gate(style) {
                return;
            }
            let verb = words.next();
            match verb {
                // ── THE ARCHETYPE EDITOR (28_VOCABULARY §5, V4) ────────
                //
                // `t0 vocab [axis]`  read the live folded vocabulary
                // `t0 mint <axis> <sigil> <word> <ch>:<p>/<q> ...`
                //
                // The CLI FORMATS AND RELAYS; it never validates. Every
                // gate lives at the world door where the chain is — a
                // client-side check that disagreed with the kernel would
                // be a second truth, which is the defect the definition
                // law struck one layer up.
                Some("vocab") => {
                    let axes: Vec<(u8, &str)> = vec![
                        (1, "disposition"), (2, "metabolism"), (3, "perception"),
                        (4, "class"), (5, "clade"), (6, "practice"),
                    ];
                    let wanted = words.next().and_then(|w| {
                        w.parse::<u8>().ok().or_else(|| {
                            axes.iter().find(|(_, name)| *name == w).map(|(code, _)| *code)
                        })
                    });
                    for (code, name) in &axes {
                        if wanted.is_some_and(|w| w != *code) {
                            continue;
                        }
                        let Some(axis) = cfe::vocabulary::Axis::from_code(*code) else {
                            continue;
                        };
                        let seeded = cfe::vocabulary::seed(axis).len();
                        let entries = cfe::vocabulary::fold(world.chronicle(), axis);
                        println!(
                            "\n{} {} — {} entries ({} seeded, {} minted)",
                            style.dim(&format!("axis {code}")),
                            style.strong(name),
                            entries.len(),
                            seeded,
                            entries.len().saturating_sub(seeded)
                        );
                        for (n, entry) in entries.iter().enumerate() {
                            let origin = if n < seeded { "seed" } else { "MINTED" };
                            let mechanism = match entry.binding.is_empty() {
                                true => style.dim("no mechanism").to_string(),
                                false => entry
                                    .binding
                                    .iter()
                                    .map(|(ch, f)| format!("{ch}:{f}"))
                                    .collect::<Vec<_>>()
                                    .join(" "),
                            };
                            println!(
                                "  {:>3}  {:<2} {:<22} {:<7} {}",
                                entry.code, entry.sigil, entry.word,
                                style.dim(origin), mechanism
                            );
                        }
                    }
                }
                // P2 (tag 103): charter a balance parameter — a TIER 0
                // power under the mint's own standing gate. Both doors
                // prove at THIS parse: the key names a parameter, the
                // value stands in the fourfold band of its seed.
                Some("parameter") => {
                    let key = words.next().and_then(|w| {
                        w.parse::<u8>().ok().and_then(cfe::parameter::Parameter::from_code)
                    });
                    let value = words.next().and_then(|w| {
                        let (n, d) = w.split_once('/').unwrap_or((w, "1"));
                        match (n.trim().parse::<i64>(), d.trim().parse::<i64>()) {
                            (Ok(n), Ok(d)) if d != 0 => Some(cfe::ratio::ratio(n, d)),
                            _ => None,
                        }
                    });
                    let Some((key, value)) = key.zip(value) else {
                        println!(
                            "{} usage: {} — keys: 1 encasement · 2 radiator · 3 leak · 4 heating · 5 sink · 6 friction · 7 veil",
                            style.bad("✗"),
                            style.strong("t0 parameter <key> <p>/<q>")
                        );
                        return;
                    };
                    let Some(charter) = cfe::parameter::Chartered::new(key, value) else {
                        println!(
                            "{} the value stands outside the fourfold band of the seed ({} = {})",
                            style.bad("✗"),
                            style.strong(&format!("{key:?}")),
                            style.dim(&key.seed().to_string())
                        );
                        return;
                    };
                    let line = active_line().unwrap_or_else(|| String::from("operator"));
                    match world.charter_parameter(&line, charter) {
                        Ok(()) => {
                            println!(
                                "{} {} chartered — the rule follows the chain",
                                style.ok("⚖"),
                                style.strong(&format!("{key:?}"))
                            );
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
                Some("mint") => {
                    let usage = || {
                        println!(
                            "{} {}\n     {}",
                            style.bad("✗"),
                            style.strong("t0 mint <axis> <sigil> <word> <channel>:<p>/<q> ..."),
                            style.dim("e.g. t0 mint class ⌘ covenantal 1:7/4 2:7/4 · t0 mint zone ⌂ deepworks on extractor 1:2 7:2   (axes: 1 disposition · 2 metabolism · 3 perception · 4 class · 5 clade · 6 practice · 7 zone; channels: 1 yield · 2 upkeep · 3 survey · 4 demurrage · 5 hab-yield · 6 hab-upkeep · 7 ext-upkeep · 8 conv-yield · 9 align gate · 10 temper gate; no axis mints an OFFICE)")
                        )
                    };
                    let axis_word = words.next();
                    let sigil = words.next().unwrap_or("");
                    let word = words.next().unwrap_or("");
                    // The axis is proven at THIS door: an unknown code or
                    // word never reaches the verb (29_CONSTRUCTION).
                    let Some(axis) = axis_word.and_then(|w| {
                        w.parse::<u8>()
                            .ok()
                            .and_then(cfe::vocabulary::Axis::from_code)
                            .or(match w {
                                "disposition" => Some(cfe::vocabulary::Axis::Disposition),
                                "metabolism" => Some(cfe::vocabulary::Axis::Metabolism),
                                "perception" => Some(cfe::vocabulary::Axis::Perception),
                                "class" => Some(cfe::vocabulary::Axis::Class),
                                "clade" => Some(cfe::vocabulary::Axis::Clade),
                                "practice" => Some(cfe::vocabulary::Axis::Practice),
                                // #41: what a world may BUILD.
                                "zone" | "building" => Some(cfe::vocabulary::Axis::Zone),
                                _ => None,
                            })
                    }) else {
                        usage();
                        return;
                    };
                    if word.is_empty() {
                        usage();
                        return;
                    }
                    let mut binding = Vec::new();
                    let mut malformed = None;
                    // #41: `on <base>` names the standing kind a minted
                    // BUILDING modulates — the ladder's first rung.
                    // Omit it and the base is inferred from the channels
                    // the binding touches; where neither answers, the
                    // word stands and nothing can be raised from it.
                    let mut base: Option<u8> = None;
                    while let Some(term) = words.next() {
                        if term == "on" {
                            let named = words.next().unwrap_or("");
                            base = (1..=cfe::dop::ZONE_KINDS)
                                .find(|code| {
                                    cfe::dop::ZoneKind::from_wire_const(*code)
                                        .is_some_and(|kind| cfe::dop::zone_word(kind) == named)
                                })
                                .or_else(|| named.parse::<u8>().ok());
                            if base.is_none() {
                                malformed = Some(named.to_string());
                                break;
                            }
                            continue;
                        }
                        let Some((channel, factor)) = term.split_once(':') else {
                            malformed = Some(term.to_string());
                            break;
                        };
                        let (numer, denom) = factor.split_once('/').unwrap_or((factor, "1"));
                        match (
                            channel.parse::<u8>(),
                            numer.trim().parse::<i64>(),
                            denom.trim().parse::<i64>(),
                        ) {
                            (Ok(channel), Ok(n), Ok(d)) if d != 0 => {
                                binding.push((channel, cfe::ratio::ratio(n, d)))
                            }
                            _ => {
                                malformed = Some(term.to_string());
                                break;
                            }
                        }
                    }
                    if let Some(term) = malformed {
                        println!(
                            "{} {} is not a channel factor — exact rationals only, no decimals ({})",
                            style.bad("✗"),
                            style.strong(&term),
                            style.dim("1:7/4  2:7/4")
                        );
                        return;
                    }
                    if binding.is_empty() {
                        usage();
                        return;
                    }
                    // The trade law is proven at THIS door, fault named
                    // (29_CONSTRUCTION): an unlawful binding never
                    // reaches the verb.
                    let binding = match cfe::vocabulary::TradingBinding::new(binding) {
                        Ok(binding) => binding,
                        Err(fault) => {
                            use cfe::vocabulary::BindingFault;
                            let why = match fault {
                                BindingFault::ChannelUnknown { channel } => format!(
                                    "channel {} is not one the engine computes — the table grows by LANDING a channel ({})",
                                    style.strong(&channel.to_string()),
                                    style.dim("1 yield · 2 upkeep · 3 survey · 4 demurrage · 5 hab-yield · 6 hab-upkeep · 7 ext-upkeep · 8 conv-yield · 9 align gate · 10 temper gate")
                                ),
                                BindingFault::FactorOutOfBounds { channel } => format!(
                                    "the factor on channel {} falls outside {}",
                                    style.strong(&channel.to_string()),
                                    style.dim("[1/4, 4]")
                                ),
                                BindingFault::NoBenefit => String::from(
                                    "every entry is FRICTION — that is a penalty, not a class",
                                ),
                                BindingFault::NoFriction => String::from(
                                    "every entry is BENEFIT — a class strictly better than its neighbours is not a class",
                                ),
                            };
                            println!("{} the binding does not TRADE: {}", style.bad("✗"), why);
                            return;
                        }
                    };
                    // The seat is read from the CHAIN, and the line is the
                    // operator's own. The engine refuses if it is not T0.
                    let line = active_line().unwrap_or_else(|| String::from("operator"));
                    let Some(word_name) = cfe::dop::Name::new(word) else {
                        println!("{} a minted word must not be empty", style.bad("✗"));
                        return;
                    };
                    match world.mint_vocabulary_based_as(
                        SOVEREIGN, &line, axis, sigil, word_name, binding, base,
                    ) {
                        Ok(code) => {
                            println!(
                                "{} {} {} minted on axis {} as code {} — the board grew a class no coder wrote",
                                style.ok("✓"),
                                sigil,
                                style.strong(word),
                                style.strong(&axis.code().to_string()),
                                style.strong(&code.to_string())
                            );
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
                None | Some("inbox") | Some("requests") => {
                    // THE PRIORITY INBOX (operator ruling: flagged,
                    // never spammed): the upward flow aggregates into
                    // three ranked buckets — a progenitor reads three
                    // lines, then drills only where the board burns.
                    let book = ledger::Ledger::at("var/worlds");
                    let mut critical: Vec<String> = Vec::new();
                    let mut petitions: Vec<String> = Vec::new();
                    if let Ok(entries) = std::fs::read_dir("var/worlds") {
                        for entry in entries.filter_map(|e| e.ok()) {
                            if !entry.path().is_dir() {
                                continue;
                            }
                            let name = entry.file_name().to_string_lossy().into_owned();
                            // CRITICAL: stress from chronicle.tlv (SEAL B),
                            // never state.json display.
                            let tlv = entry.path().join("chronicle.tlv");
                            if let Ok(chronicle) = cfe::persist::load(&tlv) {
                                let w = World::resume(chronicle);
                                let mut flags = Vec::new();
                                if let Some(orb) = w.orb() {
                                    if !orb.famine.is_empty() {
                                        flags.push("the want strikes".into());
                                    }
                                    if orb.entropy_vented > ratio(0, 1) {
                                        flags.push(format!(
                                            "venting {}/{}",
                                            orb.entropy_vented.numer(),
                                            orb.entropy_vented.denom()
                                        ));
                                    }
                                    if !flags.is_empty() {
                                        critical.push(format!(
                                            "{name}/{}: {}",
                                            orb.name,
                                            flags.join(" · ")
                                        ));
                                    }
                                }
                            }
                            // PETITIONS: the steward inbox, per world.
                            if let Ok(pending) = book.pending_directives(&name) {
                                for petition in pending {
                                    petitions.push(format!(
                                        "{name}: {} {} (as {})",
                                        petition.directive.verb,
                                        petition.directive.args.join(" "),
                                        petition.directive.acting_as
                                    ));
                                }
                            }
                        }
                    }
                    // DISPUTES: receipted council breaches, this
                    // world's chronicle (cross-world reads await the
                    // mesh).
                    let disputes: usize = world
                        .orbs()
                        .iter()
                        .map(|held| world.council_breaches(held.orb_id))
                        .sum();
                    println!("{}", style.head("T0 INBOX — flagged, never spammed"));
                    let flag = |count: usize, label: &str| -> String {
                        match count {
                            0 => format!("  {} {label}: {}", style.ok("○"), style.dim("clear")),
                            n => format!("  {} {label}: {}", style.bad("●"), style.strong(&n.to_string())),
                        }
                    };
                    println!("{}", flag(critical.len(), "CRITICAL — sector stress"));
                    println!("{}", flag(petitions.len(), "PETITIONS — the upward flow"));
                    println!("{}", flag(disputes, "DISPUTES — receipted breaches (this world)"));
                    for line in critical.iter().take(8) {
                        println!("    {} {}", style.bad("⚠"), line);
                    }
                    if critical.len() > 8 {
                        println!("    {} …and {} more (observer: stress)", style.dim("⚠"), critical.len() - 8);
                    }
                    for line in petitions.iter().take(8) {
                        println!("    {} {}", style.strong("✉"), line);
                    }
                    println!(
                        "{}",
                        style.dim("  deeds: t0 move (separate) · license (relieve) · t0 contract (seal)")
                    );
                }
                Some("move") => {
                    let target = words.next().map(str::to_string);
                    let coords: Vec<_> =
                        words.by_ref().take(3).filter_map(parse_ratio_arg).collect();
                    match (target, coords.len()) {
                        (Some(name), 3) => {
                            let book = ledger::Ledger::at("var/worlds");
                            match book.chart_world(
                                &name,
                                (&coords[0], &coords[1], &coords[2]),
                                &cfe::ratio::ratio(0, 1),
                            ) {
                                Ok(()) => println!(
                                    "{} {} MOVED on the torus — stressed sectors part in S; the board breathes",
                                    style.ok("✓"),
                                    style.strong(&name)
                                ),
                                Err(refusal) => {
                                    println!("{} move refused: {refusal}", style.bad("✗"))
                                }
                            }
                        }
                        _ => println!(
                            "{} usage: {}",
                            style.bad("✗"),
                            style.strong("t0 move <world> <x> <y> <z>")
                        ),
                    }
                }
                Some("galaxies") => {
                    let book = ledger::Ledger::at("var/worlds");
                    let galaxies = book.galaxies();
                    if galaxies.is_empty() {
                        println!(
                            "  no galaxy chartered yet — the founding sky is implicit until the first observation deed (13_GALAXY §2)"
                        );
                    } else {
                        println!("{}", style.head("THE CHARTED SKIES — regions of the one torus"));
                        let worlds = book.charted_worlds();
                        for galaxy in &galaxies {
                            let members = worlds
                                .iter()
                                .filter(|world| book.galaxy_of(world).as_deref() == Some(galaxy.name.as_str()))
                                .count();
                            println!(
                                "  {} — center ({}, {}, {}) · reach {} · {} charted worlds",
                                style.strong(&galaxy.name),
                                galaxy.center.0,
                                galaxy.center.1,
                                galaxy.center.2,
                                galaxy.reach,
                                members
                            );
                        }
                        let adrift = worlds.iter().filter(|world| book.galaxy_of(world).is_none()).count();
                        if adrift > 0 {
                            println!(
                                "  {} intergalactic worlds stand outside every reach — lawful (13_GALAXY §4)",
                                style.dim(&adrift.to_string())
                            );
                        }
                    }
                }
                Some("contract") => match conductor::fold("var") {
                    Ok((era, gauge)) => println!(
                        "{} the board CONTRACTS — era {era} sealed into var-era{era}; the new sheet seats at G = {}",
                        style.ok("✓"),
                        style.strong(&amount(&gauge))
                    ),
                    Err(refusal) => println!("{} the fold refused — {refusal}", style.bad("✗")),
                },
                Some("clean") => {
                    let mut swept = 0;
                    if let Ok(entries) = std::fs::read_dir("var/worlds") {
                        for entry in entries.filter_map(|e| e.ok()) {
                            let name = entry.file_name().to_string_lossy().into_owned();
                            if name.contains("--closed-")
                                && entry.path().is_dir()
                                && std::fs::remove_dir_all(entry.path()).is_ok()
                            {
                                swept += 1;
                                println!("  swept ruin {}", style.dim(&name));
                            }
                        }
                    }
                    println!(
                        "{} {} ruin(s) cleaned — the board holds only the living",
                        style.ok("✓"),
                        style.strong(&swept.to_string())
                    );
                }
                Some("gift") => {
                    // t0 gift <world> <condition> — a STRUCTURED
                    // petition: the conductor executes it as a
                    // receipted grant at the next beat (crown law
                    // checked there; the two-book licensing form is
                    // recorded debt until NS-5 arbitrates).
                    let target = words.next().map(str::to_string);
                    let condition = words.next().map(str::to_string);
                    match (target, condition) {
                        (Some(target), Some(condition)) => {
                            let inbox =
                                std::path::PathBuf::from("var/worlds").join(&target).join("inbox");
                            let _ = std::fs::create_dir_all(&inbox);
                            let stamp = std::fs::read_dir(&inbox)
                                .map(|d| d.count())
                                .unwrap_or(0);
                            let dir = ledger::Directive {
                                verb: "gift".into(),
                                args: vec![condition.clone()],
                                acting_as: "steward".into(),
                            };
                            match ledger::write_directive_tlv(
                                &inbox.join(format!("{stamp:06}-gift.dir")),
                                &dir,
                            ) {
                                Ok(()) => println!(
                                    "{} the gift PETITIONS {} — the conductor executes at its next beat (the crown law holds there)",
                                    style.ok("✉"),
                                    style.strong(&target)
                                ),
                                Err(refusal) => {
                                    println!("{} petition refused: {refusal}", style.bad("✗"))
                                }
                            }
                        }
                        _ => println!(
                            "{} usage: {} — condition 6 gifts the crown",
                            style.bad("✗"),
                            style.strong("t0 gift <world> <condition 1..6>")
                        ),
                    }
                }
                Some("create") => println!(
                    "creation is the civ console: {}",
                    style.strong("civ new <sigil> | <Name> | <species> | <motto> | <axes> [station]")
                ),
                Some(other) => println!(
                    "{} unknown t0 verb {} — inbox · requests · move · galaxies · contract · clean · create",
                    style.bad("✗"),
                    style.strong(other)
                ),
            }
        }
        Some("login") => match words.next() {
            Some(name) => {
                let sigil = name.to_lowercase().replace(' ', "-");
                // BLOCKER A3: the profile carries a STATION — the
                // authority the tier-0 verbs check. Passwordless per
                // ruling; the role is the gate, not the secret.
                let station = match (words.next(), words.next()) {
                    (Some("as"), Some(word)) => {
                        let word = word.to_lowercase();
                        if word != "player"
                            && !ledger::registry::STATIONS.contains(&word.as_str())
                        {
                            println!(
                                "{} unknown station {} — player or {}",
                                style.bad("✗"),
                                style.strong(&word),
                                style.dim(&ledger::registry::STATIONS.join("/"))
                            );
                            return;
                        }
                        word
                    }
                    _ => "player".to_string(),
                };
                let dir = std::path::PathBuf::from("var/operators");
                let _ = std::fs::create_dir_all(&dir);
                // Plain profile — session config, not colony law.
                let profile = dir.join(&sigil);
                let world = std::fs::read_to_string(&profile)
                    .ok()
                    .and_then(|text| profile_field(&text, "world"))
                    .unwrap_or_default();
                let _ = std::fs::write(
                    &profile,
                    format!("# operator seat claim — not a receipt\nstation {station}\nworld {world}\n"),
                );
                let _ = std::fs::write(dir.join("active"), &sigil);
                println!(
                    "{} logged in as {} [{}] — {} resumes your last saved world",
                    style.ok("✓"),
                    style.strong(&sigil),
                    style.strong(&station),
                    style.strong("continue")
                );
            }
            None => {
                let dir = std::path::PathBuf::from("var/operators");
                let active = std::fs::read_to_string(dir.join("active")).unwrap_or_default();
                let mut listed = 0;
                if let Ok(entries) = std::fs::read_dir(&dir) {
                    println!("{}", style.head("OPERATORS ON THIS MACHINE"));
                    for entry in entries.filter_map(|e| e.ok()) {
                        let name = entry.file_name().to_string_lossy().into_owned();
                        if name == "active" || name.contains('.') {
                            // Only extensionless plain profiles are operators.
                            continue;
                        }
                        if entry.path().is_file() {
                            listed += 1;
                            let mark = if name == active.trim() { " ← active" } else { "" };
                            println!("  {}{}", style.strong(&name), style.dim(mark));
                        }
                    }
                }
                if listed == 0 {
                    println!(
                        "no operators yet — {} mints one",
                        style.strong("login <name>")
                    );
                }
            }
        },
        Some("whoami") => {
            let active = std::fs::read_to_string("var/operators/active").unwrap_or_default();
            match active.trim() {
                "" => println!("no one — {} first", style.strong("login <name>")),
                sigil => println!("{}", style.strong(sigil)),
            }
        }
        Some("continue") => {
            let active = std::fs::read_to_string("var/operators/active").unwrap_or_default();
            let sigil = active.trim().to_string();
            if sigil.is_empty() {
                println!("{} {} first", style.bad("✗"), style.strong("login <name>"));
                return;
            }
            let profile =
                std::fs::read_to_string(format!("var/operators/{sigil}")).unwrap_or_default();
            let last = profile_field(&profile, "world").unwrap_or_default();
            match last.as_str() {
                "" => println!(
                    "{} no saved world on this profile yet — {} then {}",
                    style.bad("✗"),
                    style.strong("found <name>"),
                    style.strong("save")
                ),
                name => command(world, &format!("resume {name}"), style, current),
            }
        }
        Some("civ") => {
            use ledger::registry::{
                lineage_domain, seat_width, Civilization, Registry, RegistryRefused,
                DISPOSITIONS, METABOLISMS, PERCEPTIONS, STATIONS,
            };
            let registry = Registry::at("var/registry");
            match words.next() {
                None | Some("seats") => {
                    println!("{}", style.head("THE STATION CONSOLE — seats are space, not rank"));
                    let seated = registry.list();
                    let domains = ["mesic", "thermal", "cryo", "lithic", "plasmic"];
                    let mut line = String::from("  progenitor  ");
                    for domain in domains {
                        let held = seated
                            .iter()
                            .filter(|c| {
                                c.station.as_deref() == Some("progenitor")
                                    && lineage_domain(&c.metabolism, None) == domain
                            })
                            .count();
                        line.push_str(&format!("{domain} {held}/8 · "));
                    }
                    println!("{}", line.trim_end_matches(" · "));
                    for station in ["watcher", "observer", "sovereign"] {
                        let held = seated
                            .iter()
                            .filter(|c| c.station.as_deref() == Some(station))
                            .count();
                        match seat_width(station) {
                            Some(width) => println!("  {station}     {held}/{width}"),
                            None => println!("  {station}   {held} seated — unbounded (every charter-holder)"),
                        }
                    }
                    println!(
                        "  axes: {} · {} · {}",
                        style.dim(&DISPOSITIONS.join("/")),
                        style.dim(&METABOLISMS.join("/")),
                        style.dim(&PERCEPTIONS.join("/"))
                    );
                    println!(
                        "  stations: {} (a people with no station is the common, lawful state)",
                        style.dim(&STATIONS.join("/"))
                    );
                    println!(
                        "  create one: {}",
                        style.strong("civ new <sigil> | <Name> | <species> | <motto> | <disp> <metab> <perc> [station]")
                    );
                }
                Some("seat") => {
                    // #23 — THE TAG-89 DEED'S FIRST CALLER. The seat
                    // was receipted law (`vertical_axis_pins`: "the seat
                    // replays from the receipt, not from a file") with
                    // ZERO callers, so every live seating was a
                    // hand-edit of the cache this deed exists to
                    // replace. The verb: deed FIRST, cache after.
                    let (Some(sigil), Some(station_word)) = (words.next(), words.next())
                    else {
                        println!(
                            "{} the form is {}",
                            style.bad("✗"),
                            style.strong("civ seat <sigil> <progenitor|watcher|observer|sovereign>")
                        );
                        return;
                    };
                    // NATIVE TO THE FOURTH KILL: the word parses
                    // straight to the `Station` enum — the u8 code and
                    // its range check died together; an off-band office
                    // is unconstructable past this door.
                    let Some(station) = STATIONS
                        .iter()
                        .position(|s| *s == station_word)
                        .and_then(|i| cfe::dop::Station::from_wire(i as u8 + 1))
                    else {
                        println!(
                            "{} unknown station {} — {}",
                            style.bad("✗"),
                            style.strong(station_word),
                            style.dim(&STATIONS.join("/"))
                        );
                        return;
                    };
                    // ONE OFFICE PER LINE — a seat REPLACES. Said
                    // before it is done, never after.
                    if let Some(held) = world.station_of(sigil) {
                        if held != station.as_wire() {
                            let was = STATIONS
                                .get(held as usize - 1)
                                .copied()
                                .unwrap_or("unknown");
                            println!(
                                "{} {} vacates its {} seat — one office per line",
                                style.dim("note:"),
                                style.strong(sigil),
                                style.strong(was)
                            );
                        }
                    }
                    let Some(line) = cfe::dop::Name::new(sigil) else {
                        println!("{} a seat needs a named line", style.bad("✗"));
                        return;
                    };
                    match world.seat_station_as(cfe::world::SOVEREIGN, line, station) {
                        Ok(()) => {}
                        Err(refusal) => {
                            refuse(world, &refusal, style);
                            return;
                        }
                    }
                    // The registry render follows the receipt — the
                    // CACHE half, honest until #24 makes the registry a
                    // projection of the chain outright.
                    match registry.seat(sigil, station_word) {
                        Ok(()) => println!(
                            "{} {} seated as {} — receipted on this world's chain; the registry \
                             card follows the receipt",
                            style.ok("✓"),
                            style.strong(sigil),
                            style.strong(station_word)
                        ),
                        Err(RegistryRefused::NoSeatOpen { station, domain, held, width }) => {
                            println!(
                                "{} RECEIPTED, but the {station} room for the {domain} domain \
                                 stands FULL ({held}/{width}) — the card did not follow; the \
                                 chain is the truth and the room law is the registry's",
                                style.bad("✗")
                            );
                        }
                        Err(refusal) => println!(
                            "{} receipted; the card did not follow: {refusal:?}",
                            style.bad("✗")
                        ),
                    }
                }
                Some("new") => {
                    let rest = line
                        .splitn(3, char::is_whitespace)
                        .nth(2)
                        .unwrap_or("")
                        .trim();
                    let parts: Vec<&str> = rest.split('|').map(str::trim).collect();
                    if parts.len() != 5 || parts.iter().any(|p| p.is_empty()) {
                        println!(
                            "{} the form is {}",
                            style.bad("✗"),
                            style.strong("civ new <sigil> | <Name> | <species> | <motto> | <disp> <metab> <perc> [station]")
                        );
                        return;
                    }
                    let sigil = parts[0].to_lowercase().replace(' ', "-");
                    let axis_words: Vec<&str> = parts[4].split_whitespace().collect();
                    if !(3..=4).contains(&axis_words.len()) {
                        println!(
                            "{} the last field is three axes and an optional station — e.g. {}",
                            style.bad("✗"),
                            style.strong("empiricist plasmic umbral progenitor")
                        );
                        return;
                    }
                    let newcomer = Civilization {
                        name: parts[1].into(),
                        sigil: sigil.clone(),
                        species: parts[2].into(),
                        clade: String::new(),
                        motto: parts[3].into(),
                        disposition: axis_words[0].into(),
                        metabolism: axis_words[1].into(),
                        perception: axis_words[2].into(),
                        founder: "operator".into(),
                        lineage: Vec::new(),
                        station: axis_words.get(3).map(|s| s.to_string()),
                    };
                    match registry.create(&newcomer) {
                        Ok(()) => {}
                        Err(RegistryRefused::UnknownAxis { axis, given }) => {
                            println!(
                                "{} unknown {axis} {} — the vocabulary is closed (it grows by landing, never by typo)",
                                style.bad("✗"),
                                style.strong(&given)
                            );
                            return;
                        }
                        Err(RegistryRefused::UnknownStation { given }) => {
                            println!(
                                "{} unknown station {} — {}",
                                style.bad("✗"),
                                style.strong(&given),
                                style.dim(&STATIONS.join("/"))
                            );
                            return;
                        }
                        Err(RegistryRefused::NoSeatOpen { station, domain, held, width }) => {
                            println!(
                                "{} the {station} room for the {domain} domain stands FULL ({held}/{width}) — a room filled, never a rank refused; the band grows as the universe does",
                                style.bad("✗")
                            );
                            return;
                        }
                        Err(RegistryRefused::AlreadyRegistered { sigil }) => {
                            println!("{} {} is already a people — choose a new sigil", style.bad("✗"), style.strong(&sigil));
                            return;
                        }
                        Err(refusal) => {
                            println!("{} the registry refused: {refusal:?}", style.bad("✗"));
                            return;
                        }
                    }
                    // THE DEFINITION LAW (conductor::polities): registering
                    // IS the whole act. This used to splice a JSON entry
                    // onto var/polities.json by stripping the trailing
                    // `]` — a second, unreceipted definition of the line,
                    // written by string surgery. The conductor now derives
                    // the world, planet, colony and placement from the
                    // sigil at boot, so the registry card is sufficient.
                    println!(
                        "{} {} registered{} — the conductor brings the line online at its next boot ({}-hearth awaits)",
                        style.ok("✓"),
                        style.strong(&newcomer.name),
                        newcomer
                            .station
                            .as_ref()
                            .map(|s| format!(" ({s} seat taken)"))
                            .unwrap_or_default(),
                        sigil
                    );
                }
                Some(other) => println!(
                    "{} unknown civ verb {} — {} or {}",
                    style.bad("✗"),
                    style.strong(other),
                    style.strong("civ seats"),
                    style.strong("civ new …")
                ),
            }
        }
        Some(deed @ ("practice" | "release")) => {
            let releasing = deed == "release";
            // The seed five by name; ANY code passes through numeric —
            // the world judges it against the FOLD, so a minted
            // discipline adopts by the code its receipt carries.
            let code = match words.next() {
                Some("austerity") => Some(1u8),
                Some("showpiece") => Some(2),
                Some("restraint") => Some(3),
                Some("hoard") => Some(4),
                Some("overdrive") => Some(5),
                Some(word) if word.parse::<u8>().is_ok() => word.parse::<u8>().ok(),
                other => {
                    println!(
                        "{} name the discipline: {} — or a minted code",
                        style.bad("✗"),
                        style.strong("austerity · showpiece · restraint · hoard · overdrive"),
                    );
                    let _ = other;
                    None
                }
            };
            if let Some(practice) = code {
                let outcome = if releasing {
                    world.release_practice(practice)
                } else {
                    world.adopt_practice(practice)
                };
                match outcome {
                    Ok(()) if releasing => println!(
                        "{} discipline RELEASED — the rediscovery earned, the lesson re-emerges (condition 7)",
                        style.ok("✓")
                    ),
                    Ok(()) => println!(
                        "{} discipline ADOPTED — one benefit, one friction, bound by receipt (14 §4)",
                        style.ok("✓")
                    ),
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
        }
        Some("survey") => match world.chart_terrain() {
            Ok(seed) => println!(
                "{} terrain CHARTED — seed {:02x}{:02x}{:02x}{:02x} (the founding receipt's digest, forever); {} reads the map",
                style.ok("✓"),
                seed[0], seed[1], seed[2], seed[3],
                style.strong("terrain")
            ),
            Err(refusal) => refuse(world, &refusal, style),
        },
        // ESTATE CHAIN (35 / E5b): survey → parcels → appraise any holding.
        Some("estate") => match words.next() {
            None | Some("help") => {
                println!("{}", style.head("ESTATE — survey · parcels · real-estate appraisal"));
                println!("  {}  meter estate (iron × orbs), witness founding receipts", style.strong("estate survey"));
                println!("  {}  derived free/monetary break per orb", style.strong("estate parcels"));
                println!(
                    "  {}  {}",
                    style.strong("appraise estate|planet [orb]|building <tile>|orbital [orb]|stratum <n>"),
                    style.dim("value at every scale — place & exploration claims")
                );
            }
            Some("survey") => {
                let payer = world.orb().map(|o| o.orb_id).unwrap_or(cfe::world::FIRST_ORB);
                match world.survey_estate(payer) {
                    Ok(seed) => {
                        let n = world.orbs().len();
                        println!(
                            "{} estate SURVEYED — {} orb(s), seed {:02x}{:02x}… · parcels derive forever",
                            style.ok("✓"),
                            n,
                            seed[0],
                            seed[1]
                        );
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some("parcels") => match world.parcels() {
                None => println!(
                    "  {}",
                    style.dim("unsurveyed — `estate survey` opens the metered map")
                ),
                Some(parcels) => {
                    println!("{}", style.head("PARCELS — free + monetary = 1 (exact break)"));
                    for p in parcels {
                        println!(
                            "  orb {} · free {} · monetary {}",
                            p.orb_id,
                            amount(&p.free),
                            amount(&p.monetary)
                        );
                    }
                }
            },
            Some(other) => println!(
                "{} unknown estate verb {:?} — try estate survey|parcels|help",
                style.bad("✗"),
                other
            ),
        },
        // P5-A DEFEND / escort · P5-B posture · P5-M MKT integrity
        Some("defend") | Some("defence") => match words.next() {
            None | Some("status") | Some("help") => {
                let p = world.defense_posture();
                let mi = world.market_integrity();
                let atm = world.medium_integrity(*current);
                let reor = if p.reoriented {
                    style.warn("reoriented")
                } else {
                    style.dim("genesis doctrine")
                };
                println!(
                    "{} DEFEND — primary {} · secondary {} · {}",
                    style.head("🛡"),
                    style.strong(p.primary.word()),
                    p.secondary.word(),
                    style.dim(&p.voice)
                );
                println!(
                    "  anatomy {} · doctrine {} · perception {} · {}",
                    metabolism_name(p.metabolism),
                    disposition_name(p.disposition),
                    perception_name(p.perception),
                    reor
                );
                if let (Some(c), Some(l)) = (p.class, p.clade) {
                    println!("  taxon class {c} · clade {l}");
                }
                println!(
                    "  ranks CEL:{} ATM:{} COG:{} CYB:{} INF:{} KIN:{} MKT:{}",
                    p.ranks[0],
                    p.ranks[1],
                    p.ranks[2],
                    p.ranks[3],
                    p.ranks[4],
                    p.ranks[5],
                    p.ranks[6]
                );
                if mi.sealed {
                    println!(
                        "  {} circuit open until t{}",
                        style.warn("MKT SEALED"),
                        mi.resume_at.unwrap_or(0)
                    );
                } else {
                    println!(
                        "  MKT clear depth crystal {}/{} iron {}/{}",
                        mi.crystal_depth, mi.seal_need, mi.iron_depth, mi.seal_need
                    );
                }
                println!(
                    "  ATM pressure {} · dual-use TERRA={}",
                    amount(&atm.pressure),
                    atm.dual_use
                );
                let inf = world.inf_integrity();
                let veil_w = if inf.veiled {
                    style.warn("VEIL")
                } else {
                    style.dim("open")
                };
                let ward_w = if inf.ward_holds {
                    style.warn(inf.ward_state.word())
                } else {
                    style.dim(inf.ward_state.word())
                };
                println!(
                    "  INF rank {} · {} · ward {} · detection grade {} (instruments {})",
                    inf.inf_rank,
                    veil_w,
                    ward_w,
                    inf.detection_grade,
                    inf.instrument_count
                );
                println!(
                    "  {}",
                    style.hint(
                        "veil|ward · posture · escort · market · medium|tamper · forge instrument"
                    )
                );
            }
            Some("escort") => {
                let hull = words.next().and_then(|w| w.parse().ok());
                let deal = words.next().and_then(|w| w.parse().ok());
                match (hull, deal) {
                    (Some(h), Some(d)) => match world.escort_assign(h, d) {
                        Ok(()) => {
                            let _ = sync_world(world);
                            let risk = world.convoy_risk(d);
                            println!(
                                "{} escort hull {h} → deal {d} · risk now {}",
                                style.ok("🛡"),
                                amount(&risk)
                            );
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    _ => println!(
                        "{} usage: {}",
                        style.bad("✗"),
                        style.strong("defend escort <hull_id> <deal>")
                    ),
                }
            }
            Some("risk") => {
                let deal = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                match world.mark_convoy_risk(deal) {
                    Ok(r) => {
                        let _ = sync_world(world);
                        println!(
                            "{} deal {deal} risk {} · cover {} · {}",
                            style.ok("🛡"),
                            amount(&r),
                            amount(&world.escort_cover(deal)),
                            world.convoy_risk_band(deal).word()
                        );
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some(other) => println!(
                "{} defend {:?} — status|escort|risk",
                style.bad("✗"),
                other
            ),
        },
        // P5-C INF — veil / ward surface (MASK-1 / WARDEN-1a compose)
        Some("veil") => match words.next() {
            None | Some("status") | Some("help") => {
                let inf = world.inf_integrity();
                println!(
                    "{} VEIL — {} · toll {} · detection grade {}",
                    style.head("INF"),
                    if inf.veiled {
                        style.warn("DRAWN")
                    } else {
                        style.ok("lifted/open")
                    },
                    amount(&inf.veil_toll),
                    inf.detection_grade
                );
                println!(
                    "  {}",
                    style.hint("veil draw|lift — under veil, scan/prospect/assay need instruments")
                );
            }
            Some("draw") => match world.draw_veil() {
                Ok(()) => {
                    let _ = sync_world(world);
                    println!("{} veil DRAWN — renders cloud; chronicle never lies", style.warn("INF"));
                }
                Err(refusal) => refuse(world, &refusal, style),
            },
            Some("lift") => match world.lift_veil() {
                Ok(()) => {
                    let _ = sync_world(world);
                    println!("{} veil lifted", style.ok("INF"));
                }
                Err(refusal) => refuse(world, &refusal, style),
            },
            Some(other) => println!(
                "{} veil {:?} — status|draw|lift",
                style.bad("✗"),
                other
            ),
        },
        Some("ward") => match words.next() {
            None | Some("status") | Some("help") => {
                let inf = world.inf_integrity();
                println!(
                    "{} WARD — state {} · holds={} (elder reverse shield)",
                    style.head("INF"),
                    inf.ward_state.word(),
                    inf.ward_holds
                );
                println!(
                    "  {}",
                    style.hint("ward elder · ward lift <claimer> <depth> — aperture locked while held")
                );
            }
            Some("elder") | Some("seat") => match world.ward_elder() {
                Ok(()) => {
                    let _ = sync_world(world);
                    println!("{} elder WARD seated — no aperture until lift", style.warn("INF"));
                }
                Err(refusal) => refuse(world, &refusal, style),
            },
            Some("lift") => {
                let claimer = words.next().unwrap_or("sovereign");
                let depth = words
                    .next()
                    .and_then(|w| w.parse().ok())
                    .unwrap_or_else(cfe::world::ward_gate_depth);
                match world.lift_ward(claimer, depth) {
                    Ok(()) => {
                        let _ = sync_world(world);
                        println!("{} ward LIFTED by {claimer} (depth {depth})", style.ok("INF"));
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some(other) => println!(
                "{} ward {:?} — status|elder|lift",
                style.bad("✗"),
                other
            ),
        },
        // P5-B reconfigurable doctrine posture (anatomy sealed)
        Some("posture") => match words.next() {
            None | Some("status") | Some("help") => {
                let p = world.defense_posture();
                println!(
                    "{} POSTURE — primary {} · secondary {}",
                    style.head("🛡"),
                    p.primary.word(),
                    p.secondary.word()
                );
                println!(
                    "  metabolism(anatomy)={} · disposition={} · perception={} · reoriented={}",
                    metabolism_name(p.metabolism),
                    disposition_name(p.disposition),
                    perception_name(p.perception),
                    p.reoriented
                );
                println!("  {}", style.dim(&p.voice));
                println!(
                    "  {}",
                    style.hint(
                        "posture set <expansionist|empiricist|epicurean> <planar|…> [CEL|ATM|…|clear]"
                    )
                );
            }
            Some("set") | Some("reorient") => {
                let dis = words.next().and_then(parse_disposition);
                let per = words.next().and_then(parse_perception);
                let primary_word = words.next();
                let primary = match primary_word {
                    None | Some("clear") | Some("0") | Some("-") => Some(0u8),
                    Some(w) => parse_domain(w),
                };
                match (dis, per, primary) {
                    (Some(d), Some(p), Some(pri)) => {
                        match world.reorient_posture(d, p, pri) {
                            Ok(()) => {
                                let _ = sync_world(world);
                                let now = world.defense_posture();
                                println!(
                                    "{} doctrine reoriented → {} · {} · primary {}",
                                    style.ok("🛡"),
                                    disposition_name(now.disposition),
                                    perception_name(now.perception),
                                    now.primary.word()
                                );
                            }
                            Err(refusal) => refuse(world, &refusal, style),
                        }
                    }
                    _ => println!(
                        "{} usage: {}",
                        style.bad("✗"),
                        style.strong(
                            "posture set <disposition> <perception> [CEL|ATM|COG|CYB|INF|KIN|MKT|clear]"
                        )
                    ),
                }
            }
            Some(other) => println!(
                "{} posture {:?} — status|set",
                style.bad("✗"),
                other
            ),
        },
        // P5-M market defense deeds (clear seal · breaker · raid)
        Some("market") => match words.next() {
            None | Some("status") | Some("help") => {
                let mi = world.market_integrity();
                println!(
                    "{} MARKET — sealed={} resume={:?} · crystal_depth={}/{} iron_depth={}/{} · cascade={}",
                    style.head("MKT"),
                    mi.sealed,
                    mi.resume_at,
                    mi.crystal_depth,
                    mi.seal_need,
                    mi.iron_depth,
                    mi.seal_need,
                    mi.cascade_pressure
                );
                println!(
                    "  books note={} fut={} share={} perp={}",
                    mi.notes_open, mi.futures_open, mi.shares_open, mi.perps_open
                );
                println!(
                    "  {}",
                    style.hint(
                        "market seal|resume · market clear <crystal|iron> <p>/<q> · market raid [orb]"
                    )
                );
            }
            Some("seal") => match world.seal_market() {
                Ok(at) => {
                    let _ = sync_world(world);
                    println!("{} circuit breaker until t{at}", style.warn("MKT SEALED"));
                }
                Err(refusal) => refuse(world, &refusal, style),
            },
            Some("resume") => match world.resume_market() {
                Ok(()) => {
                    let _ = sync_world(world);
                    println!("{} circuit breaker lifted", style.ok("MKT OPEN"));
                }
                Err(refusal) => refuse(world, &refusal, style),
            },
            Some("clear") => {
                let res = words.next().and_then(parse_resource);
                let price = words
                    .next()
                    .and_then(parse_ratio_arg)
                    .and_then(cfe::dop::Positive::new);
                match (res, price) {
                    (Some(resource), Some(p)) => {
                        match world.record_spot_clear(*current, resource, p) {
                            Ok(depth) => {
                                let _ = sync_world(world);
                                println!(
                                    "{} {:?} clear depth {depth}/{}",
                                    style.ok("✓"),
                                    resource,
                                    cfe::world::CLEAR_SEAL_DEPTH
                                );
                            }
                            Err(refusal) => refuse(world, &refusal, style),
                        }
                    }
                    _ => println!(
                        "{} usage: {}",
                        style.bad("✗"),
                        style.strong("market clear <crystal|iron|…> <p>/<q>")
                    ),
                }
            }
            Some("raid") => {
                let orb = words
                    .next()
                    .and_then(|w| w.parse().ok())
                    .unwrap_or(*current);
                match world.liquidity_raid(orb) {
                    Ok(()) => {
                        let _ = sync_world(world);
                        println!("{} liquidity raid struck orb {orb}", style.warn("RAID"));
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some("cascade") => {
                let orb = words
                    .next()
                    .and_then(|w| w.parse().ok())
                    .unwrap_or(*current);
                match world.note_cascade(orb) {
                    Ok(p) => {
                        let _ = sync_world(world);
                        println!(
                            "{} note cascade pressure {p} on orb {orb}",
                            style.warn("CASCADE")
                        );
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some(other) => println!(
                "{} market {:?} — status|seal|resume|clear|raid|cascade",
                style.bad("✗"),
                other
            ),
        },
        // P5-D ATM medium / covert tamper
        Some("medium") | Some("envelope") => match words.next() {
            None | Some("status") | Some("help") => {
                let mi = world.medium_integrity(*current);
                println!(
                    "{} MEDIUM — orb {} pressure {} / {} · compromised={}",
                    style.head("ATM"),
                    mi.orb,
                    amount(&mi.pressure),
                    amount(&mi.threshold),
                    mi.compromised
                );
                println!(
                    "  {}",
                    style.hint("medium reinforce · tamper [orb]  (crystal out / iron in)")
                );
            }
            Some("reinforce") => match world.medium_reinforce(*current) {
                Ok(p) => {
                    let _ = sync_world(world);
                    let mi = world.medium_integrity(*current);
                    println!(
                        "{} envelope reinforced ({:?}) · pressure now {} · {}",
                        style.ok("ATM"),
                        mi.reinforce_resource,
                        amount(&p),
                        style.dim(mi.residual_voice)
                    );
                }
                Err(refusal) => refuse(world, &refusal, style),
            },
            Some(other) => println!(
                "{} medium {:?} — status|reinforce",
                style.bad("✗"),
                other
            ),
        },
        Some("tamper") => {
            let orb = words
                .next()
                .and_then(|w| w.parse().ok())
                .unwrap_or(*current);
            match world.covert_tamper(orb) {
                Ok(p) => {
                    let _ = sync_world(world);
                    let flag = if p >= cfe::world::tamper_threshold() {
                        style.warn("DETECTED")
                    } else {
                        style.dim("covert")
                    };
                    println!(
                        "{} tamper orb {orb} · pressure {} · {}",
                        style.warn("ATM"),
                        amount(&p),
                        flag
                    );
                }
                Err(refusal) => refuse(world, &refusal, style),
            }
        },
        // O6.5 Vault custody · O6.4 Assay court
        Some("vault") => match words.next() {
            None | Some("status") | Some("help") => {
                let orb = world.orb().map(|o| o.orb_id).unwrap_or(*current);
                let bal = world.vault_balance_of(orb);
                let res = world.vault_reserved(orb);
                let free = if bal > res {
                    bal.clone() - res.clone()
                } else {
                    ratio(0, 1)
                };
                println!(
                    "{} VAULT — seated={} · orb {orb} custody {} · reserved {} · free {}",
                    style.head("T8"),
                    world.vault_seated(),
                    amount(&bal),
                    amount(&res),
                    amount(&free)
                );
                println!(
                    "  {}",
                    style.hint("vault deposit <n> · vault withdraw <n>  (STAMP custody; margin holds free)")
                );
            }
            Some("deposit") => {
                let n = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let orb = world.orb().map(|o| o.orb_id).unwrap_or(*current);
                match cfe::dop::Positive::new(ratio(n, 1)) {
                    Some(p) => match world.vault_deposit(orb, p) {
                        Ok(bal) => {
                            let _ = sync_world(world);
                            println!(
                                "{} deposited {n} STAMP · vault {}",
                                style.ok("VAULT"),
                                amount(&bal)
                            );
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    None => println!("{} need positive amount", style.bad("✗")),
                }
            }
            Some("withdraw") => {
                let n = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let orb = world.orb().map(|o| o.orb_id).unwrap_or(*current);
                match cfe::dop::Positive::new(ratio(n, 1)) {
                    Some(p) => match world.vault_withdraw(orb, p) {
                        Ok(bal) => {
                            let _ = sync_world(world);
                            println!(
                                "{} withdrew {n} STAMP · vault {}",
                                style.ok("VAULT"),
                                amount(&bal)
                            );
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    None => println!("{} need positive amount", style.bad("✗")),
                }
            }
            Some(other) => println!(
                "{} vault {:?} — status|deposit|withdraw",
                style.bad("✗"),
                other
            ),
        },
        Some("challenge") | Some("challenge_assay") => {
            let prefix = words.next();
            match prefix {
                Some(p) => {
                    let orb = world.orb().map(|o| o.orb_id).unwrap_or(*current);
                    // Resolve element by prefix against open prospects.
                    let hit = world
                        .committed()
                        .prospects
                        .iter()
                        .find(|v| {
                            let hex: String = v
                                .element
                                .iter()
                                .take(4)
                                .map(|b| format!("{b:02x}"))
                                .collect();
                            hex.starts_with(p) || hex.starts_with(&p.to_lowercase())
                        })
                        .map(|v| v.element);
                    match hit {
                        Some(el) => match world.challenge_assay(orb, el) {
                            Ok(true) => {
                                let _ = sync_world(world);
                                println!(
                                    "{} assay consilience MATCH on {}",
                                    style.ok("ASSAY"),
                                    &hex(&el)[..8]
                                );
                            }
                            Ok(false) => {
                                let _ = sync_world(world);
                                println!(
                                    "{} assay MISMATCH — richness corrected + crystal slash on {}",
                                    style.warn("SLASH"),
                                    &hex(&el)[..8]
                                );
                            }
                            Err(refusal) => refuse(world, &refusal, style),
                        },
                        None => println!(
                            "{} no open vein matches prefix {} — prospect first",
                            style.bad("✗"),
                            p
                        ),
                    }
                }
                None => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("challenge <element-prefix>")
                ),
            }
        },
        // X7 Anchor export/import (feature-gated deeds)
        Some("anchor") => match words.next() {
            None | Some("status") | Some("help") => {
                let d = world.deep_rail_status();
                println!(
                    "{} ANCHOR — seated={} · strand_feat={} · court_feat={}",
                    style.head("T9"),
                    d.anchor_seated,
                    d.strand_enabled,
                    d.court_enabled
                );
                println!(
                    "  {}",
                    style.hint(
                        "anchor export-stamp <p>/<q> · anchor import-strand <hex> · import-court <hex>"
                    )
                );
            }
            Some("export-stamp") | Some("export") => {
                let amt = words
                    .next()
                    .and_then(parse_ratio_arg)
                    .and_then(cfe::dop::Positive::new);
                match amt {
                    Some(a) => {
                        let shown = amount(a.value());
                        match world.export_stamp_strand(*current, a) {
                            Ok(pkt) => {
                                let _ = sync_world(world);
                                println!(
                                    "{} exported {shown} STAMP · packet {} bytes",
                                    style.ok("STRAND"),
                                    pkt.len()
                                );
                            }
                            Err(refusal) => refuse(world, &refusal, style),
                        }
                    }
                    None => println!(
                        "{} usage: {}",
                        style.bad("✗"),
                        style.strong("anchor export-stamp <p>/<q>")
                    ),
                }
            }
            Some("import-strand") => {
                let hx = words.next().unwrap_or("");
                match decode_hex_bytes(hx) {
                    Some(bytes) => match world.import_anchor_strand(*current, &bytes) {
                        Ok(a) => {
                            let _ = sync_world(world);
                            println!(
                                "{} imported {} STAMP via strand proof",
                                style.ok("ANCHOR"),
                                amount(&a)
                            );
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    None => println!("{} need hex packet bytes", style.bad("✗")),
                }
            }
            Some("import-court") => {
                let hx = words.next().unwrap_or("");
                match decode_hex_bytes(hx) {
                    Some(bytes) => match world.import_anchor_court(*current, &bytes) {
                        Ok(a) => {
                            let _ = sync_world(world);
                            println!(
                                "{} imported {} STAMP via court proof",
                                style.ok("ANCHOR"),
                                amount(&a)
                            );
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    None => println!("{} need hex isthmus frame", style.bad("✗")),
                }
            }
            Some(other) => println!(
                "{} anchor {:?} — status|export-stamp|import-strand|import-court",
                style.bad("✗"),
                other
            ),
        },
        // P5-G deep rails
        Some("rail") | Some("rails") => match words.next() {
            None | Some("status") | Some("help") => {
                let d = world.deep_rail_status();
                println!(
                    "{} RAILS — max R{} · element routes {} · strand={} court={} anchor={}",
                    style.head("R"),
                    d.max_rail,
                    d.routes,
                    d.strand_enabled,
                    d.court_enabled,
                    d.anchor_seated
                );
                for (deal, eta, el, rail) in world.element_routes().iter().take(8) {
                    let rw = cfe::dop::Rail::from_wire(*rail)
                        .map(|r| r.word())
                        .unwrap_or("?");
                    println!(
                        "  deal {deal} rail {rw} η {} el {:02x}…",
                        amount(eta.value()),
                        el[0]
                    );
                }
                println!(
                    "  {}",
                    style.hint("rail promote <deal> — R1→R2 always; R3/R4 need features + Anchor")
                );
            }
            Some("promote") => {
                let deal = words.next().and_then(|w| w.parse().ok());
                match deal {
                    Some(d) => match world.promote_rail(d) {
                        Ok(to) => {
                            let _ = sync_world(world);
                            let w = cfe::dop::Rail::from_wire(to)
                                .map(|r| r.word())
                                .unwrap_or("?");
                            println!("{} deal {d} promoted → {w}", style.ok("RAIL"));
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    None => println!(
                        "{} usage: {}",
                        style.bad("✗"),
                        style.strong("rail promote <deal>")
                    ),
                }
            }
            Some(other) => println!(
                "{} rail {:?} — status|promote",
                style.bad("✗"),
                other
            ),
        },
        // P5-F espionage
        Some("spy") => match words.next() {
            None | Some("status") | Some("help") => {
                let e = world.espionage_integrity();
                println!(
                    "{} SPY — detect g{} · marks {} · intel {} · veiled={}",
                    style.head("INF"),
                    e.detection_grade,
                    e.spy_marks,
                    e.intel_count,
                    e.veiled
                );
                println!(
                    "  can_probe={} can_exfil={} can_sabotage={}",
                    e.can_probe, e.can_exfil, e.can_sabotage
                );
                println!(
                    "  {}",
                    style.hint("spy probe [orb] · spy exfil <condition> · spy sabotage [orb]")
                );
            }
            Some("probe") => {
                let orb = words
                    .next()
                    .and_then(|w| w.parse().ok())
                    .unwrap_or(*current);
                match world.spy_probe(orb) {
                    Ok(d) => {
                        let _ = sync_world(world);
                        println!("{} probe orb {orb} depth {d}", style.ok("SPY"));
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some("exfil") => {
                let cond = words.next().and_then(|w| w.parse().ok());
                match cond {
                    Some(c) => match world.tech_exfil(*current, c) {
                        Ok(h) => {
                            let _ = sync_world(world);
                            println!(
                                "{} exfil condition {c} · hash {:02x}{:02x}… (intel stash, not tech mint)",
                                style.warn("EXFIL"),
                                h[0],
                                h[1]
                            );
                        }
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    None => println!(
                        "{} usage: {}",
                        style.bad("✗"),
                        style.strong("spy exfil <condition>")
                    ),
                }
            }
            Some("sabotage") => {
                let orb = words
                    .next()
                    .and_then(|w| w.parse().ok())
                    .unwrap_or(*current);
                match world.sabotage_instrument(orb) {
                    Ok(g) => {
                        let _ = sync_world(world);
                        println!("{} sabotaged instrument grade {g} on orb {orb}", style.warn("SABOTAGE"));
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some(other) => println!(
                "{} spy {:?} — status|probe|exfil|sabotage",
                style.bad("✗"),
                other
            ),
        },
        // Suite shells — UI over deeds (36 §10), not a second physics.
        Some("suite") => match words.next() {
            Some("market") | Some("mkt") | Some("war") => {
                let mi = world.market_integrity();
                let p = world.defense_posture();
                println!(
                    "{} MARKET WAR suite — MKT rank {} · sealed={}",
                    style.head("SUITE"),
                    p.ranks[6],
                    mi.sealed
                );
                println!(
                    "  clear seal crystal {}/{} iron {}/{} · resume={:?}",
                    mi.crystal_depth, mi.seal_need, mi.iron_depth, mi.seal_need, mi.resume_at
                );
                println!(
                    "  instruments note={} fut={} share={} perp={}",
                    mi.notes_open, mi.futures_open, mi.shares_open, mi.perps_open
                );
                let offers = world.offers().len();
                println!("  resting offers {offers}");
                println!(
                    "  {}",
                    style.hint("same deeds: market seal|clear|raid · note|future|share|perp")
                );
            }
            Some("convoy") | Some("kin") => {
                let p = world.defense_posture();
                let deal = world
                    .routes()
                    .first()
                    .map(|(d, _)| *d)
                    .or_else(|| {
                        world.hulls().iter().find(|h| h.assigned_route != 0).map(|h| h.assigned_route)
                    })
                    .unwrap_or(0);
                let ki = world.kin_integrity(deal);
                println!(
                    "{} CONVOY suite — KIN rank {} · primary {} · doctrine dis={}",
                    style.head("SUITE"),
                    ki.kin_rank,
                    p.primary.word(),
                    p.disposition
                );
                println!(
                    "  residual {:?} · {}",
                    ki.residual,
                    style.dim(ki.residual_voice)
                );
                if let Some(per) = ki.place_period {
                    println!("  place period {per} ticks (orbit dynamo)");
                }
                if let Some(fog) = ki.fog {
                    println!("  {} {}", style.warn("FOG"), fog.word());
                }
                let escorts: Vec<_> = world
                    .hulls()
                    .iter()
                    .filter(|h| {
                        h.assigned_route != 0
                            && cfe::dop::HullRole::from_wire(h.role)
                                == Some(cfe::dop::HullRole::Escort)
                    })
                    .collect();
                if escorts.is_empty() {
                    println!("  (no escorts on duty)");
                } else {
                    for h in escorts.iter().take(8) {
                        let k = world.kin_integrity(h.assigned_route);
                        println!(
                            "  hull {} → deal {} · risk {} · {} · cover {}",
                            h.hull_id,
                            h.assigned_route,
                            amount(&k.risk),
                            k.band.word(),
                            amount(&k.cover)
                        );
                    }
                }
                println!(
                    "  {}",
                    style.hint("defend escort <hull> <deal> · defend risk <deal> · posture set")
                );
            }
            Some("spy") | Some("espionage") | Some("intel") => {
                let e = world.espionage_integrity();
                println!(
                    "{} ESPIONAGE suite — detect g{} · marks {} · intel hashes {}",
                    style.head("SUITE"),
                    e.detection_grade,
                    e.spy_marks,
                    e.intel_count
                );
                println!(
                    "  probe={} exfil={} sabotage={} veiled={}",
                    e.can_probe, e.can_exfil, e.can_sabotage, e.veiled
                );
                for (i, h) in world.intel_hashes().iter().take(4).enumerate() {
                    println!("  intel[{i}] {:02x}{:02x}…", h[0], h[1]);
                }
                println!(
                    "  {}",
                    style.hint("spy probe|exfil|sabotage · forge · veil — same deeds")
                );
            }
            Some("cyber") | Some("cyb") => {
                let cyb = world.cyb_integrity();
                let cog = world.cog_integrity();
                println!(
                    "{} CYBER suite — CYB rank {} · {}",
                    style.head("SUITE"),
                    cyb.cyb_rank,
                    style.dim(cyb.voice)
                );
                println!(
                    "  detect g{} · instruments {} · routes {} · mesh_ready={} · stamp_banked={}",
                    cyb.detection_grade,
                    cyb.instrument_count,
                    cyb.routes_open,
                    cyb.mesh_ready,
                    cyb.stamp_banked
                );
                println!(
                    "  COG rank {} · wants {} · practices {} · stressed={} · {}",
                    cog.cog_rank,
                    cog.want_load,
                    cog.practice_load,
                    cog.stressed,
                    style.dim(cog.voice)
                );
                println!(
                    "  stamp mesh hygiene={}",
                    world.stamp_mesh_hygiene()
                );
                println!(
                    "  {}",
                    style.hint("forge <grade> · aperture … · veil · practice — same deeds")
                );
            }
            _ => println!(
                "{} suite market|convoy|cyber — dashboards over the same deeds (36 §10)",
                style.bad("✗")
            ),
        },
        // PHASE-4 instruments (NOTE-1 / FUT-1).
        Some("note") => match words.next() {
            None | Some("list") | Some("help") => {
                println!(
                    "{} NOTES — listable={} · Exchange={}",
                    style.head("NOTE"),
                    world.notes_listable(),
                    world.exchange_seated()
                );
                for n in world.notes() {
                    if !n.open {
                        continue;
                    }
                    println!(
                        "  note {} · face {} STAMP · coll {} · matures t{} · holder {}",
                        n.note_id,
                        amount(&n.notional),
                        amount(&n.collateral),
                        n.maturity,
                        n.holder
                    );
                }
            }
            Some("issue") => {
                let face = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(2);
                let coll = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let term = words
                    .next()
                    .and_then(|w| w.parse::<u64>().ok())
                    .unwrap_or(60);
                let orb = world.orb().map(|o| o.orb_id).unwrap_or(cfe::world::FIRST_ORB);
                let holder = cfe::world::SOVEREIGN;
                match (
                    cfe::dop::Positive::new(ratio(face, 1)),
                    cfe::dop::Positive::new(ratio(coll, 1)),
                ) {
                    (Some(n), Some(c)) => match world.issue_note(orb, holder, n, c, term) {
                        Ok(id) => println!(
                            "{} note {id} ISSUED — face {face} coll {coll} term {term}",
                            style.ok("✓")
                        ),
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    _ => println!("{} need positive face and collateral", style.bad("✗")),
                }
            }
            Some("redeem") => {
                let id = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                match world.redeem_note(id) {
                    Ok(()) => println!("{} note {id} REDEEMED", style.ok("✓")),
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some(other) => println!(
                "{} note {:?} — try note list|issue|redeem",
                style.bad("✗"),
                other
            ),
        },
        Some("share") => match words.next() {
            None | Some("list") | Some("help") => {
                println!(
                    "{} SHARES — listable={}",
                    style.head("SHARE"),
                    world.shares_listable()
                );
                for s in world.shares().iter().filter(|s| s.open) {
                    println!(
                        "  share {} · {:?} units {} par {} holder {}",
                        s.share_id,
                        s.stream,
                        amount(&s.units),
                        amount(&s.par),
                        s.holder
                    );
                }
            }
            Some("issue") => {
                let stream = words
                    .next()
                    .and_then(|w| match w {
                        "iron" | "i" => Some(cfe::dop::Resource::Iron),
                        "bio" | "biomass" => Some(cfe::dop::Resource::Biomass),
                        "crystal" | "c" => Some(cfe::dop::Resource::Crystal),
                        _ => None,
                    })
                    .unwrap_or(cfe::dop::Resource::Iron);
                let units = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let par = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let orb = world.orb().map(|o| o.orb_id).unwrap_or(cfe::world::FIRST_ORB);
                match (
                    cfe::dop::Positive::new(ratio(units, 1)),
                    cfe::dop::Positive::new(ratio(par, 1)),
                ) {
                    (Some(u), Some(p)) => {
                        match world.issue_share(orb, cfe::world::SOVEREIGN, stream, u, p) {
                            Ok(id) => println!(
                                "{} share {id} ISSUED · {:?}×{units} par {par}",
                                style.ok("✓"),
                                stream
                            ),
                            Err(refusal) => refuse(world, &refusal, style),
                        }
                    }
                    _ => println!("{} need positive units/par", style.bad("✗")),
                }
            }
            Some("dividend") | Some("div") => {
                let id = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                match world.pay_share_dividend(id) {
                    Ok(a) => println!("{} share {id} dividend {} STAMP", style.ok("✓"), amount(&a)),
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some("transfer") => {
                let id = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                let to = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                let u = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                if let Some(units) = cfe::dop::Positive::new(ratio(u, 1)) {
                    match world.transfer_share(id, to, units) {
                        Ok(()) => println!("{} share {id} → {to}", style.ok("✓")),
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
            }
            Some(other) => println!(
                "{} share {:?} — list|issue|dividend|transfer",
                style.bad("✗"),
                other
            ),
        },
        Some("perp") => match words.next() {
            None | Some("list") | Some("help") => {
                println!(
                    "{} PERPS — listable={}",
                    style.head("PERP"),
                    world.perps_listable()
                );
                for p in world.perps().iter().filter(|p| p.open) {
                    println!(
                        "  perp {} · {:?}×{} entry {} margin {}",
                        p.perp_id,
                        p.resource,
                        amount(&p.notional),
                        amount(&p.entry),
                        amount(&p.margin)
                    );
                }
            }
            Some("open") => {
                let res = words
                    .next()
                    .and_then(|w| match w {
                        "crystal" | "c" => Some(cfe::dop::Resource::Crystal),
                        "iron" | "i" => Some(cfe::dop::Resource::Iron),
                        _ => None,
                    })
                    .unwrap_or(cfe::dop::Resource::Crystal);
                let notional = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let margin = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let orb = world.orb().map(|o| o.orb_id).unwrap_or(cfe::world::FIRST_ORB);
                let long = cfe::world::SOVEREIGN;
                let short = world
                    .entities()
                    .iter()
                    .map(|e| e.entity_id)
                    .find(|id| *id != long)
                    .unwrap_or(long.saturating_add(1));
                match (
                    cfe::dop::Positive::new(ratio(notional, 1)),
                    cfe::dop::Positive::new(ratio(margin, 1)),
                ) {
                    (Some(n), Some(m)) => match world.open_perp(orb, long, short, res, n, m) {
                        Ok(id) => println!(
                            "{} perp {id} OPEN · {:?}×{notional}",
                            style.ok("✓"),
                            res
                        ),
                        Err(refusal) => refuse(world, &refusal, style),
                    },
                    _ => println!("{} need positive sizes", style.bad("✗")),
                }
            }
            Some("fund") => {
                let id = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                match world.fund_perp(id) {
                    Ok(f) => println!("{} perp {id} funding {}", style.ok("✓"), amount(&f)),
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some("close") => {
                let id = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                match world.close_perp(id) {
                    Ok(()) => println!("{} perp {id} CLOSED", style.ok("✓")),
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some("liquidate") | Some("liq") => {
                let id = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                match world.liquidate_perp(id) {
                    Ok(()) => println!("{} perp {id} LIQUIDATED", style.ok("✓")),
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some(other) => println!(
                "{} perp {:?} — list|open|fund|close|liquidate",
                style.bad("✗"),
                other
            ),
        },
        Some("future") | Some("fut") => match words.next() {
            None | Some("list") | Some("help") => {
                println!(
                    "{} FUTURES — listable={} · clears={}",
                    style.head("FUTURE"),
                    world.futures_listable(),
                    world.last_clears().len()
                );
                for f in world.futures() {
                    if !f.open {
                        continue;
                    }
                    println!(
                        "  fut {} · {}×{:?} @ {} · margin {} · exp t{}",
                        f.fut_id,
                        amount(&f.notional),
                        f.resource,
                        amount(&f.price),
                        amount(&f.margin),
                        f.expiry
                    );
                }
            }
            Some("mark") => {
                let res = words
                    .next()
                    .and_then(|w| match w {
                        "crystal" | "c" => Some(cfe::dop::Resource::Crystal),
                        "iron" | "i" => Some(cfe::dop::Resource::Iron),
                        "bio" | "biomass" => Some(cfe::dop::Resource::Biomass),
                        _ => None,
                    })
                    .unwrap_or(cfe::dop::Resource::Crystal);
                let price_n = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let orb = world.orb().map(|o| o.orb_id).unwrap_or(cfe::world::FIRST_ORB);
                if let Some(p) = cfe::dop::Positive::new(ratio(price_n, 1)) {
                    match world.mark_last_clear(orb, res, p) {
                        Ok(()) => println!(
                            "{} last clear {:?} = {} STAMP/unit",
                            style.ok("✓"),
                            res,
                            price_n
                        ),
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
            }
            Some("open") => {
                let res = words
                    .next()
                    .and_then(|w| match w {
                        "crystal" | "c" => Some(cfe::dop::Resource::Crystal),
                        "iron" | "i" => Some(cfe::dop::Resource::Iron),
                        _ => None,
                    })
                    .unwrap_or(cfe::dop::Resource::Crystal);
                let notional = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let price = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let margin = words
                    .next()
                    .and_then(|w| w.parse::<i64>().ok())
                    .unwrap_or(1);
                let term = words
                    .next()
                    .and_then(|w| w.parse::<u64>().ok())
                    .unwrap_or(60);
                let orb = world.orb().map(|o| o.orb_id).unwrap_or(cfe::world::FIRST_ORB);
                let long = cfe::world::SOVEREIGN;
                let short = world
                    .entities()
                    .iter()
                    .map(|e| e.entity_id)
                    .find(|id| *id != long)
                    .unwrap_or(long.saturating_add(1));
                match (
                    cfe::dop::Positive::new(ratio(notional, 1)),
                    cfe::dop::Positive::new(ratio(price, 1)),
                    cfe::dop::Positive::new(ratio(margin, 1)),
                ) {
                    (Some(n), Some(p), Some(m)) => {
                        match world.open_future(orb, long, short, res, n, p, m, term) {
                            Ok(id) => println!(
                                "{} future {id} OPEN — {:?}×{notional} @ {price} margin {margin}",
                                style.ok("✓"),
                                res
                            ),
                            Err(refusal) => refuse(world, &refusal, style),
                        }
                    }
                    _ => println!("{} need positive sizes", style.bad("✗")),
                }
            }
            Some("settle") => {
                let id = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                match world.settle_future(id) {
                    Ok(()) => println!("{} future {id} SETTLED", style.ok("✓")),
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
            Some(other) => println!(
                "{} future {:?} — try future list|mark|open|settle",
                style.bad("✗"),
                other
            ),
        },
        Some("appraise") => {
            use cfe::dop::Holding;
            let orb = world.orb().map(|o| o.orb_id).unwrap_or(cfe::world::FIRST_ORB);
            let holding = match words.next() {
                None | Some("estate") => Some(Holding::Estate),
                Some("planet") | Some("planetary") => {
                    let o = words
                        .next()
                        .and_then(|w| w.parse().ok())
                        .unwrap_or(orb);
                    Some(Holding::Planetary { orb: o })
                }
                Some("building") | Some("zone") => {
                    let tile = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                    Some(Holding::Building { orb, tile })
                }
                Some("orbital") | Some("orbit") | Some("place") => {
                    let o = words
                        .next()
                        .and_then(|w| w.parse().ok())
                        .unwrap_or(orb);
                    Some(Holding::Orbital { orb: o })
                }
                Some("stratum") | Some("vein") | Some("prospect") => {
                    let s = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
                    Some(Holding::Stratum { orb, stratum: s })
                }
                Some(other) => {
                    println!(
                        "{} unknown scale {:?} — estate|planet|building|orbital|stratum",
                        style.bad("✗"),
                        other
                    );
                    None
                }
            };
            if let Some(h) = holding {
                match world.appraise(h) {
                    Ok(value) => println!(
                        "{} APPRAISED — value {} (iron-commensurate weight) · stale after any later deed",
                        style.ok("✓"),
                        amount(&value)
                    ),
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
        },
        Some("terrain") => match world.terrain_seed() {
            None => println!(
                "  uncharted — every tile reads Standard (·) until {} charts the seed (02 §2.5, the opt-in deed)",
                style.strong("survey")
            ),
            Some(seed) => {
                let zones: Vec<(u16, cfe::dop::ZoneKind)> = world
                    .orb()
                    .map(|orb| orb.zones.iter().map(|z| (z.tile, z.kind)).collect())
                    .unwrap_or_default();
                println!("{}", style.head("THE CHART — · standard  ▲ rich  ░ barren  (letters: zones)"));
                for row in 0..8u16 {
                    let mut line = String::from("  ");
                    for col in 0..8u16 {
                        let tile = row * 8 + col;
                        let glyph = match zones.iter().find(|(t, _)| *t == tile) {
                            Some((_, kind)) => match kind {
                                cfe::dop::ZoneKind::Extractor => 'E',
                                cfe::dop::ZoneKind::Habitat => 'H',
                                cfe::dop::ZoneKind::Smelter => 'S',
                                cfe::dop::ZoneKind::Synthesizer => 'Y',
                                cfe::dop::ZoneKind::Encasement => 'N',
                                cfe::dop::ZoneKind::CryoArray => 'C',
                                cfe::dop::ZoneKind::Resonator => 'R',
                                cfe::dop::ZoneKind::Regenerator => 'G',
                                cfe::dop::ZoneKind::ElementRefinery => 'F',
                                cfe::dop::ZoneKind::ShipYard => 'P',
                                cfe::dop::ZoneKind::Granary => 'A',
                                cfe::dop::ZoneKind::GovernmentOffice => 'O',
                                cfe::dop::ZoneKind::WelfareHub => 'W',
                                // #41: a minted building — the chain
                                // named it, so the map marks it as one.
                                cfe::dop::ZoneKind::Minted(_) => 'M',
                            },
                            None => cfe::dop::tile_class(&seed, cfe::world::FIRST_ORB, tile).glyph(),
                        };
                        line.push(glyph);
                        line.push(' ');
                    }
                    println!("{line}");
                }
                println!(
                    "  {} an extractor beside ▲ runs ×5/4; a habitat beside E runs ×3/4 (02 §2.6)",
                    style.dim("adjacency:")
                );
            }
        },
        Some("save") => match world.orb() {
            Some(orb) => {
                let path = std::path::PathBuf::from("var/worlds")
                    .join(&orb.name)
                    .join("chronicle.tlv");
                // #37: the saved world becomes VISIBLE — the ledger
                // exports it beside the NPCs (civilization = the
                // sealed line, else the active operator).
                let civ = world
                    .genesis()
                    .map(|g| g.civilization.clone())
                    .or_else(|| {
                        std::fs::read_to_string("var/operators/active")
                            .ok()
                            .map(|n| n.trim().to_string())
                            .filter(|n| !n.is_empty())
                    })
                    .unwrap_or_else(|| "operator".into());
                let _ = scribe::record(
                    &ledger::Ledger::at("var/worlds"),
                    &orb.name,
                    &orb.name,
                    &civ,
                    world,
                );
                match cfe::persist::sync(world.chronicle(), &path) {
                    Ok(appended) => {
                        // The active operator's thread picks up here.
                        let active =
                            std::fs::read_to_string("var/operators/active").unwrap_or_default();
                        if !active.trim().is_empty() {
                            let profile = format!("var/operators/{}", active.trim());
                            let station = std::fs::read_to_string(&profile)
                                .ok()
                                .and_then(|text| profile_field(&text, "station"))
                                .unwrap_or_else(|| "player".into());
                            let _ = std::fs::write(
                                &profile,
                                format!(
                                    "# operator seat claim — not a receipt\nstation {station}\nworld {}\n",
                                    orb.name
                                ),
                            );
                        }
                        println!(
                            "{} world saved — {} receipt(s) appended to {} (the file only grows)",
                            style.ok("✓"),
                            style.strong(&appended.to_string()),
                            style.dim(&path.display().to_string())
                        )
                    }
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
                            let (Some(lane), Some(cargo)) = (
                                cfe::dop::Between::new(source, target),
                                cfe::dop::Positive::new(value.clone()),
                            ) else {
                                println!("{} a transfer joins two DIFFERENT planets with a positive amount", style.bad("✗"));
                                return;
                            };
                            match world.transfer_as(SOVEREIGN, lane, resource, cargo) {
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
                            let (Some(pair), Some(cargo)) = (
                                cfe::dop::Between::new(source, absorber),
                                cfe::dop::Positive::new(value.clone()),
                            ) else {
                                println!("{} absorption joins two DIFFERENT planets with a positive amount", style.bad("✗"));
                                return;
                            };
                            match world.absorb_as(SOVEREIGN, pair, cargo) {
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
                            let (Some(pair), Some(flow)) = (
                                cfe::dop::Between::new(source, target),
                                cfe::dop::Positive::new(rate.clone()),
                            ) else {
                                println!("{} a bond streams a positive rate between two DIFFERENT planets", style.bad("✗"));
                                return;
                            };
                            match world.seal_bond_as(SOVEREIGN, pair, resource, flow, term) {
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
        Some("forward") => {
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
                            let (Some(pair), Some(owed), Some(ticks)) = (
                                cfe::dop::Between::new(obligor, beneficiary),
                                cfe::dop::Positive::new(quota.clone()),
                                cfe::dop::Window::new(window),
                            ) else {
                                println!("{} a forward owes a positive quota between two DIFFERENT planets inside at least one tick", style.bad("✗"));
                                return;
                            };
                            match world.seal_forward_as(
                                SOVEREIGN, pair, resource, owed, ticks,
                            ) {
                                Ok(()) => println!(
                                    "{} forward vitrified: {} {} owed to {} within {} ticks — the collateral sits in the lattice until the word is kept or broken",
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
                    style.strong("forward <quota> <resource> to <planet> within <ticks> [from <planet>]"),
                    style.strong("forward 3 iron to outpost within 10")
                ),
            }
        }
        Some("forwards") => {
            match world.forwards().is_empty() {
                true => println!("{}", style.dim("(no open forwards)")),
                false => {
                    for pact in world.forwards() {
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
                        ("integrity", "honoured", &orb.forwards_fulfilled, "broke", &orb.forwards_breached),
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
                    // BOOK-1: the four above are the CHARTER's. A line
                    // may mint its own, and a display that showed only
                    // four would imply the set is closed.
                    let minted: Vec<_> =
                        orb.books().into_iter().filter(|held| !held.chartered).collect();
                    match minted.is_empty() {
                        true => println!(
                            "  {}",
                            style.dim("no minted books — `book mint <name>` opens an account of your own")
                        ),
                        false => {
                            for view in minted {
                                let reading = match view.standing() {
                                    Some(value) => format!("ratio {}", style.strong(&amount(&value))),
                                    None => style.dim("no deeds yet").to_string(),
                                };
                                let mark = match view.archived {
                                    true => style.warn(" (closed)"),
                                    false => String::new(),
                                };
                                println!(
                                    "  {:<11} kept {} · lost {} — {}{}",
                                    view.name,
                                    style.strong(&amount(&view.kept)),
                                    style.strong(&amount(&view.lost)),
                                    reading,
                                    mark
                                );
                            }
                        }
                    }
                }
                None => println!(
                    "{} no colony yet — {} begins the story",
                    style.bad("✗"),
                    style.strong("found <name>")
                ),
            }
        }
        // GROWTH-1 — the six axes, carried whole. No aggregate: six
        // readings and no score, because collapsing them is the struck
        // C-gate under a sixth name.
        // ── HISTORY AS A PLACE (#43) — the chronicle, walkable ──────
        Some("eras") | Some("history") => {
            let receipts = world.chronicle().receipts();
            println!(
                "{} THE RECORD — {} receipts, stamp 0 → {}",
                style.strong("⌛"),
                receipts.len(),
                world.now()
            );
            let mut era = 0u64;
            let mut opened = 0u64;
            let mut count = 0usize;
            for receipt in receipts {
                count += 1;
                match &receipt.event {
                    cfe::dop::ClosureEvent::EraFolded { era: folded, gauge } => {
                        println!(
                            "  era {} — stamps {}..{} · {} receipts · folded at gauge {}",
                            style.strong(&era.to_string()),
                            opened,
                            receipt.stamp,
                            count,
                            amount(gauge)
                        );
                        era = *folded;
                        opened = receipt.stamp;
                        count = 0;
                    }
                    cfe::dop::ClosureEvent::Diverged { branch, gauge } => println!(
                        "  {} the line CLEAVES — branch {} departs at gauge {}",
                        style.warn("⑂"),
                        style.strong(branch),
                        amount(gauge)
                    ),
                    cfe::dop::ClosureEvent::Confluent { partner, gauge } => println!(
                        "  {} confluence with {} at gauge {}",
                        style.ok("⑃"),
                        style.strong(partner),
                        amount(gauge)
                    ),
                    cfe::dop::ClosureEvent::LineSucceeded { successor } => println!(
                        "  {} succession — {} takes the line",
                        style.warn("♔"),
                        style.strong(successor)
                    ),
                    _ => {}
                }
            }
            println!(
                "  era {} — stamps {}..{} · {} receipts · {}",
                style.strong(&era.to_string()),
                opened,
                world.now(),
                count,
                style.ok("open")
            );
            println!("  {}", style.dim("`visit <stamp>` stands you in the past"));
        }
        Some("visit") => match words.next().and_then(|w| w.parse::<u64>().ok()) {
            None => println!(
                "{} usage: {} — reconstruct the world as it stood",
                style.bad("✗"),
                style.strong("visit <stamp>")
            ),
            Some(stamp) => {
                // The prefix replica: re-append every receipt at or
                // before the stamp; digests re-derive identically, and
                // replay stands you in that world (SEAL B is why this
                // is possible at all).
                let mut replica = cfe::vrp::Chronicle::new();
                let mut kept = 0usize;
                for receipt in world.chronicle().receipts() {
                    if receipt.stamp > stamp {
                        break;
                    }
                    replica.append(
                        receipt.event.clone(),
                        receipt.stamp,
                        receipt.provenance.clone(),
                    );
                    kept += 1;
                }
                if kept == 0 {
                    println!("{}", style.dim("nothing had happened yet — the world is unfounded"));
                    return;
                }
                let then = cfe::world::replay(&replica);
                println!(
                    "{} YOU STAND AT STAMP {} — {} receipts deep (of {})",
                    style.strong("⌖"),
                    style.strong(&stamp.to_string()),
                    kept,
                    world.chronicle().receipts().len()
                );
                for orb in &then.orbs {
                    println!(
                        "  {} {} — {} zones · {}",
                        style.strong(&format!("orb {}", orb.orb_id)),
                        orb.name,
                        orb.zones.len(),
                        orb.banked
                            .iter()
                            .map(|(resource, held)| format!(
                                "{} {}",
                                amount(held),
                                resource_name(*resource)
                            ))
                            .collect::<Vec<_>>()
                            .join(" · ")
                    );
                }
                let star = then
                    .star
                    .map(|class| format!("{class:?}"))
                    .unwrap_or_else(|| "dark".into());
                println!(
                    "  sky {} · techs {} · practices {:?}",
                    star,
                    then.techs.len(),
                    then.practices
                );
                println!(
                    "  {}",
                    style.dim("derived by replaying the prefix — the past provably happened")
                );
            }
        },
        // ── THE PLAYER'S ATLAS — every wired surface, listable ──────
        Some("settlements") | Some("orbs") => {
            let committed = world.committed();
            println!("{} SETTLEMENTS — {} standing", style.strong("⌂"), committed.orbs.len());
            for orb in &committed.orbs {
                let mut kinds = std::collections::BTreeMap::new();
                for zone in &orb.zones {
                    *kinds.entry(format!("{:?}", zone.kind)).or_insert(0u32) += 1;
                }
                let zones: Vec<String> =
                    kinds.iter().map(|(kind, held)| format!("{held}×{kind}")).collect();
                let state = if !orb.famine.is_empty() {
                    style.bad(&format!("FAMINE {:?}", orb.famine)).to_string()
                } else if orb.overheat {
                    style.bad("OVERHEAT").to_string()
                } else {
                    style.ok("settled").to_string()
                };
                println!(
                    "  {} {} [{}] — {} — {}",
                    style.strong(&format!("orb {}", orb.orb_id)),
                    orb.name,
                    zones.join(" · "),
                    state,
                    orb.banked
                        .iter()
                        .map(|(resource, held)| format!("{} {}", amount(held), resource_name(*resource)))
                        .collect::<Vec<_>>()
                        .join(" · ")
                );
            }
        }
        Some("rings") | Some("sky") => {
            let committed = world.committed();
            match committed.star {
                None => println!("{} no sun commissioned — the sky is dark", style.dim("✦")),
                Some(class) => {
                    println!(
                        "{} STAR {:?} — luminosity {} · field anchors {} rings",
                        style.strong("✦"),
                        class,
                        amount(&class.luminosity()),
                        style.strong(&class.ring_reach().to_string())
                    );
                    for orb in &committed.orbs {
                        match &orb.thermal {
                            None => println!(
                                "  orb {} {} — {}",
                                orb.orb_id, orb.name, style.dim("unsited (thermally dormant)")
                            ),
                            Some(site) => {
                                let t4 = cfe::thermal::t_eq4(
                                    &class.luminosity(),
                                    site.albedo.value(),
                                    site.orbit_r.value(),
                                );
                                let band = match cfe::thermal::band_of(&t4) {
                                    Some(band) => format!("{band:?}"),
                                    None => format!(
                                        "gap (coverage {})",
                                        amount(&cfe::thermal::coverage(&t4))
                                    ),
                                };
                                let period = cfe::world::orbit_phase_period(world, site);
                                let core = cfe::world::core_magnetism(site);
                                println!(
                                    "  orb {} {} — ring r={} · A={} · core {} · period {}t/slot · {} {}",
                                    orb.orb_id,
                                    orb.name,
                                    amount(site.orbit_r.value()),
                                    amount(site.albedo.value()),
                                    core,
                                    period,
                                    style.strong(&band),
                                    if orb.overheat { style.bad("♨").to_string() } else { String::new() }
                                );
                            }
                        }
                    }
                }
            }
        }
        Some("elements") | Some("periodic") => {
            let committed = world.committed();
            println!("{} THE ELEMENT TABLE — matter this world has named", style.strong("◇"));
            if committed.prospects.is_empty() {
                println!("  {}", style.dim("no veins prospected — `prospect <stratum>` opens the ground"));
            }
            let star = committed.star.unwrap_or(cfe::thermal::StarClass::G);
            for vein in &committed.prospects {
                let (family, tier, natural) =
                    cfe::dop::element_properties(&vein.element, star);
                // The periodic EXTRAPOLATION: group and period derive
                // from the identity hash by the same modular law as
                // family and tier — display until the chemistry lands
                // (#37), exactly as names are display.
                let group = 1 + vein.element[3] % 18;
                let period = 1 + vein.element[4] % 7;
                println!(
                    "  {} — orb {} stratum {} · family {} tier {} · group {} period {} · richness {}/tick · assayed t{}",
                    style.strong(&cfe::dop::element_name(family, &vein.element)),
                    vein.orb,
                    vein.stratum,
                    family,
                    tier,
                    group,
                    period,
                    amount(&vein.richness),
                    vein.last_assay
                );
                // ECO-1 (#40): THE GROUND'S CONDITION. A vein is a
                // spring — the draw tires it and regenerators restore
                // it — so the standing flow is shown against the
                // nature it can never pass, with the tending that
                // decides which way it is heading.
                let tending = committed
                    .orbs
                    .iter()
                    .find(|orb| orb.orb_id == vein.orb)
                    .map(|orb| {
                        orb.zones
                            .iter()
                            .filter(|zone| zone.kind == cfe::dop::ZoneKind::Regenerator)
                            .count()
                    })
                    .unwrap_or(0);
                let worn = vein.richness < natural;
                println!(
                    "      {} {} of {}/tick natural{}",
                    match (worn, tending) {
                        (false, _) => style.dim("○ untouched —"),
                        (true, 0) => style.warn("◔ WORN —"),
                        (true, _) => style.dim("◑ recovering —"),
                    },
                    amount(&vein.richness),
                    amount(&natural),
                    match tending {
                        0 => String::new(),
                        held => format!(" · {held} regenerator(s) tending"),
                    }
                );
            }
            for orb in &committed.orbs {
                for (element, held) in &orb.elements {
                    // ELEMENT-CHEMISTRY (#37): banked matter is either
                    // DRAWN (a vein named it) or MADE (a gem this
                    // world's own bond named). Both are element ids in
                    // one ledger; the chronicle says which is which.
                    let drawn = committed.prospects.iter().any(|vein| &vein.element == element);
                    let made = is_gem(world, element);
                    let name = match (drawn, made) {
                        (true, _) => {
                            let (family, _, _) = cfe::dop::element_properties(element, star);
                            cfe::dop::element_name(family, element)
                        }
                        (false, true) => format!(
                            "{} {}",
                            style.strong("gem"),
                            &hex_prefix(element)[..8]
                        ),
                        (false, false) => format!("{:02x}{:02x}…", element[0], element[1]),
                    };
                    println!(
                        "  banked: {} {} on orb {}",
                        style.strong(&amount(held)),
                        name,
                        orb.orb_id
                    );
                }
            }
        }
        Some("civs") | Some("neighbors") => {
            let registry = ledger::registry::Registry::at("var/registry");
            let roster = registry.list();
            println!("{} KNOWN CIVILIZATIONS — {} registered", style.strong("✧"), roster.len());
            for card in &roster {
                println!(
                    "  {} — {} · {} {} {}",
                    style.strong(&card.sigil),
                    card.name,
                    card.disposition,
                    card.metabolism,
                    card.perception,
                );
            }
            if roster.is_empty() {
                println!("  {}", style.dim("nobody registered — run an era or seat lines"));
            }
        }
        Some("contacts") => {
            let mut deals: std::collections::BTreeSet<u64> = Default::default();
            let mut councils: std::collections::BTreeSet<u64> = Default::default();
            let mut pens = 0u32;
            for receipt in world.chronicle().receipts() {
                match &receipt.event {
                    cfe::dop::ClosureEvent::ApertureSent { deal, .. }
                    | cfe::dop::ClosureEvent::ApertureReceived { deal, .. } => {
                        deals.insert(*deal);
                    }
                    cfe::dop::ClosureEvent::TechLicensed { council, .. } => {
                        councils.insert(*council);
                    }
                    cfe::dop::ClosureEvent::HandoffSealed { .. } => pens += 1,
                    _ => {}
                }
            }
            println!(
                "{} CONTACTS — {} aperture deals · {} licensing councils · {} pen handoffs",
                style.strong("☍"),
                deals.len(),
                councils.len(),
                pens
            );
            for deal in &deals {
                println!("  aperture deal {}", style.strong(&deal.to_string()));
            }
            for council in &councils {
                println!("  council {}", style.strong(&council.to_string()));
            }
            if deals.is_empty() && councils.is_empty() && pens == 0 {
                println!("  {}", style.dim("no cross-world contact yet — the leap, then apertures"));
            }
        }
        Some("routes") => {
            let committed = world.committed();
            println!("{} CHARTERED LANES — {}", style.strong("⇶"), committed.routes.len());
            for (deal, eta) in &committed.routes {
                println!(
                    "  deal {} — η {} (melt shed {})",
                    style.strong(&deal.to_string()),
                    amount(eta.value()),
                    style.dim("melt × (1−η)")
                );
            }
            if committed.routes.is_empty() {
                println!("  {}", style.dim("no lanes — `charter <world> deal <n>` after an instrument forges"));
            }
            println!(
                "{} ELEMENT LANES — {}",
                style.strong("⇢"),
                committed.element_routes.len()
            );
            for (deal, eta, element, rail) in &committed.element_routes {
                let rail_word = cfe::dop::Rail::from_wire(*rail)
                    .map(|r| match r {
                        cfe::dop::Rail::Hearth => "R1",
                        cfe::dop::Rail::Aperture => "R2",
                        cfe::dop::Rail::Strand => "R3",
                        cfe::dop::Rail::Court => "R4",
                    })
                    .unwrap_or("R?");
                println!(
                    "  deal {} — η {} · {rail_word} · element {}",
                    style.strong(&deal.to_string()),
                    amount(eta.value()),
                    style.dim(&hex_prefix(element))
                );
            }
            if committed.element_routes.is_empty() {
                println!(
                    "  {}",
                    style.dim("no element lanes — `element charter <world> <prefix> deal <n>`")
                );
            }
        }
        Some("dashboard") | Some("dash") => {
            let committed = world.committed();
            println!("{}", style.strong("╔═══ NETSTRATUM — THE BOARD ═══╗"));
            let star = committed
                .star
                .map(|class| format!("{class:?}"))
                .unwrap_or_else(|| "dark".into());
            println!(
                "  sky {} · orbs {} · routes {} · veins {}",
                style.strong(&star),
                committed.orbs.len(),
                committed.routes.len(),
                committed.prospects.len()
            );
            if let Some(orb) = committed.orbs.iter().find(|held| held.orb_id == *current) {
                println!(
                    "  {} [{}]: {}",
                    style.strong(&orb.name),
                    orb.zones.len(),
                    orb.banked
                        .iter()
                        .map(|(resource, held)| format!("{} {}", amount(held), resource_name(*resource)))
                        .collect::<Vec<_>>()
                        .join(" · ")
                );
                if !orb.famine.is_empty() {
                    println!("  {}", style.bad(&format!("FAMINE: {:?}", orb.famine)));
                }
            }
            let practices: Vec<String> = committed
                .practices
                .iter()
                .map(|code| cfe::vocabulary::practice_word(*code).to_string())
                .collect();
            if !practices.is_empty() {
                println!("  disciplines: {}", style.strong(&practices.join(" · ")));
            }
            let held_techs = committed.techs.len();
            println!(
                "  sky nodes {} · instruments {} · books open",
                held_techs, committed.instruments.len()
            );
            let payloads: Vec<&[u8]> = world
                .chronicle()
                .receipts()
                .iter()
                .filter_map(|receipt| match &receipt.event {
                    cfe::dop::ClosureEvent::DiscoveryMinted { work, .. } => Some(work.as_slice()),
                    _ => None,
                })
                .collect();
            let spectrum = kernel::spectrum_of(payloads.into_iter());
            println!(
                "  cognition mass {} · {}",
                style.strong(&amount(&spectrum.weighted)),
                style.dim("`spectrum` for the whole reading — `settlements` `rings` `elements` `civs` `contacts` `routes`")
            );
            println!("{}", style.strong("╚══════════════════════════════╝"));
        }
        // P3 — THE SPECTRUM: the cognition reading, derived whole from
        // the work this chronicle actually minted; there is no score.
        Some("spectrum") => {
            let payloads: Vec<&[u8]> = world
                .chronicle()
                .receipts()
                .iter()
                .filter_map(|receipt| match &receipt.event {
                    cfe::dop::ClosureEvent::DiscoveryMinted { work, .. } => {
                        Some(work.as_slice())
                    }
                    _ => None,
                })
                .collect();
            let read = kernel::spectrum_of(payloads.into_iter());
            println!(
                "{} the SPECTRUM — what this chain has actually solved (a reading, never a score)",
                style.strong("◮")
            );
            for (rung, count) in read.forge_rungs.iter().enumerate() {
                if *count > 0 {
                    println!("  forge rung {rung}: {count}");
                }
            }
            println!(
                "  corpus: {} measured ({} blind) · {} unpriced · {} broken",
                read.corpus_measured, read.corpus_blind, read.unpriced, read.broken
            );
            println!(
                "  weighted cognition mass: {}",
                style.strong(&amount(&read.weighted))
            );
            println!(
                "  {}",
                style.dim("the benchmark floor: 34 of 1243 confirmed, 1 of 283 blind — the space above is open")
            );
        }
        Some("growth") => match world.orb_by_id(*current) {
            None => println!(
                "{} no colony yet — {} begins the story",
                style.bad("✗"),
                style.strong("found <name>")
            ),
            Some(_) => {
                let growth = world.growth_of(*current);
                println!(
                    "{}  {}",
                    style.head("GROWTH"),
                    style.dim("what the chain PRODUCED — none of it reachable at stamp 0")
                );
                for (axis, sense, reading) in [
                    ("practice", "TIME      did the writing span real chain time", amount(&growth.practice)),
                    ("knowledge", "BREADTH   accounts written in", growth.knowledge.to_string()),
                    ("expertise", "DEPTH     the deepest account's share", amount(&growth.expertise)),
                    ("wisdom", "TESTING   accounts carrying BOTH sides", amount(&growth.wisdom)),
                    ("locality", "SCOPE     the line's record that sits here", amount(&growth.locality)),
                    ("triviality", "EMPTINESS postings that moved nothing", amount(&growth.triviality)),
                ] {
                    // Pad the PLAIN word before painting it — colour
                    // codes count toward a format width and skew the
                    // column (the same trap as the books display).
                    println!(
                        "  {} {} — {}",
                        style.strong(&format!("{axis:<11}")),
                        style.dim(sense),
                        style.strong(&reading)
                    );
                }
                println!(
                    "  {}",
                    match (growth.practised(), growth.substantive()) {
                        (true, true) => style.ok("this line keeps books: it reads and writes"),
                        (false, _) => style.dim("no practice yet — writing has to SPAN time, not fill one stamp").to_string(),
                        (true, false) => style.warn("the record spans time and moved nothing"),
                    }
                );
            }
        },
        // LIT-1 — the literature's doors. Literacy is DERIVED (a line
        // that keeps its own books has learned to write), so there is
        // no verb that grants it.
        Some("literature") | Some("works") => match world.orb_by_id(*current) {
            None => println!(
                "{} no colony yet — {} begins the story",
                style.bad("✗"),
                style.strong("found <name>")
            ),
            Some(_) => {
                let literate = world.literate(*current);
                println!(
                    "{}  {}",
                    style.head("LITERATURE"),
                    match literate {
                        true => style.dim("this line reads and writes — it may reach for these"),
                        false => style.dim("not yet literate: keep a book of your own and the works open"),
                    }
                );
                for view in world.literature(*current) {
                    let standing = match (view.claimed, view.witnesses) {
                        (true, n) => style.ok(&format!("claimed on {n} receipt(s)")),
                        (false, _) => style.dim("—"),
                    };
                    let reach = match world
                        .orb_by_id(*current)
                        .is_some_and(|orb| orb.aspirations.iter().any(|h| h.work == view.work))
                    {
                        true => style.strong("reached for"),
                        false => style.dim("unreached").to_string(),
                    };
                    println!("  {:<12} {:<14} {}", view.name, reach, standing);
                }
            }
        },
        Some("aspire") => {
            let name = words.collect::<Vec<_>>().join(" ");
            match world.aspire(*current, &name) {
                Ok(_) => println!(
                    "{} this line reaches for {} — the deed still has to be done",
                    style.ok("✓"),
                    style.strong(&name)
                ),
                Err(refusal) => refuse(world, &refusal, style),
            }
        }
        Some("claim") => {
            let rest = words.collect::<Vec<_>>();
            // `claim <work> on <digest> [digest ...]` — the receipts are
            // the point, so the word `on` separates them from the name.
            let split = rest.iter().position(|word| *word == "on");
            match split {
                Some(at) if at > 0 && at + 1 < rest.len() => {
                    let name = rest[..at].join(" ");
                    let mut witnesses = Vec::new();
                    let mut bad = None;
                    for word in &rest[at + 1..] {
                        match resolve_witness(world, word) {
                            Ok(digest) => witnesses.push(digest),
                            Err(why) => bad = Some(why),
                        }
                    }
                    match bad {
                        Some(why) => println!(
                            "{} {} — {} lists the receipts",
                            style.bad("✗"),
                            why,
                            style.strong("chronicle")
                        ),
                        None => match world.claim_work(*current, &name, &witnesses) {
                            Ok(()) => println!(
                                "{} {} CLAIMED on {} chain receipt(s) — the citation is verified, the virtue is not scored",
                                style.ok("✓"),
                                style.strong(&name),
                                style.strong(&witnesses.len().to_string())
                            ),
                            Err(refusal) => refuse(world, &refusal, style),
                        },
                    }
                }
                _ => println!(
                    "{} {} — a claim without receipts is a boast",
                    style.bad("✗"),
                    style.strong("claim <work> on <digest> [digest ...]")
                ),
            }
        }
        // BOOK-1 (19_LENS §16) — the open registry's doors. Amounts
        // come BEFORE the name on `post` so a name may hold spaces
        // without the parse turning ambiguous.
        Some("book") | Some("books") => {
            let sub = words.next();
            match world.orb_by_id(*current) {
                None => println!(
                    "{} no colony yet — {} begins the story",
                    style.bad("✗"),
                    style.strong("found <name>")
                ),
                Some(_) => match sub {
                    None | Some("list") => {
                        println!(
                            "{}  {}",
                            style.head("BOOKS"),
                            style.dim("the charter's four and everything this line minted — one view, no privilege")
                        );
                        let orb = world.orb_by_id(*current).expect("checked above");
                        for view in orb.books() {
                            let standing = match view.standing() {
                                Some(value) => format!("standing {}", style.strong(&amount(&value))),
                                None => style.dim("no entries yet").to_string(),
                            };
                            // Pad the PLAIN word before painting it:
                            // the colour codes count toward a format
                            // width and would skew the column.
                            let mark = match (view.chartered, view.archived) {
                                (true, _) => style.dim(&format!("{:<7}", "charter")),
                                (false, true) => style.warn(&format!("{:<7}", "closed")),
                                (false, false) => style.ok(&format!("{:<7}", "minted")),
                            };
                            println!(
                                "  {} {:<22} kept {} · lost {} — {}",
                                mark,
                                view.name,
                                style.strong(&amount(&view.kept)),
                                style.strong(&amount(&view.lost)),
                                standing
                            );
                        }
                    }
                    Some("mint") => {
                        let name = words.collect::<Vec<_>>().join(" ");
                        let proven = match cfe::books::BookName::new(&name) {
                            Ok(proven) => proven,
                            Err(fault) => {
                                println!("{} {}", style.bad("✗"), book_name_fault(&fault, style));
                                return;
                            }
                        };
                        match world.mint_book(*current, &proven) {
                            Ok(_) => println!(
                                "{} book {} minted — an account of this line's own making",
                                style.ok("✓"),
                                style.strong(&name)
                            ),
                            Err(refusal) => refuse(world, &refusal, style),
                        }
                    }
                    Some("post") => {
                        let kept = words.next().and_then(parse_ratio_arg).and_then(cfe::dop::NonNegative::new);
                        let lost = words.next().and_then(parse_ratio_arg).and_then(cfe::dop::NonNegative::new);
                        let name = words.collect::<Vec<_>>().join(" ");
                        match (kept, lost, name.is_empty()) {
                            (Some(kept), Some(lost), false) => {
                                let kind = cfe::books::kind_of(&name);
                                match world.post_book(*current, kind, kept, lost) {
                                    Ok(()) => println!(
                                        "{} posted to {} — both sides at once, as a book is kept",
                                        style.ok("✓"),
                                        style.strong(&name)
                                    ),
                                    Err(refusal) => refuse(world, &refusal, style),
                                }
                            }
                            _ => println!(
                                "{} {} — the amounts come first so a name may hold spaces; a book accrues, so both sides are non-negative",
                                style.bad("✗"),
                                style.strong("book post <kept> <lost> <name>")
                            ),
                        }
                    }
                    Some("archive") | Some("close") => {
                        let name = words.collect::<Vec<_>>().join(" ");
                        match world.archive_book(*current, cfe::books::kind_of(&name)) {
                            Ok(()) => println!(
                                "{} {} closed to writing — every entry it took stays readable",
                                style.ok("✓"),
                                style.strong(&name)
                            ),
                            Err(refusal) => refuse(world, &refusal, style),
                        }
                    }
                    Some(other) => println!(
                        "{} no such book verb {} — try {}",
                        style.bad("✗"),
                        style.strong(other),
                        style.strong("book · book mint · book post · book archive")
                    ),
                },
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
            // #39 THE AGENT DOOR. `agent <handle>` REGISTERS (the
            // sovereign's deed, unchanged); `agent <handle> <deed> …`
            // SUBMITS through the door. Folded into the one verb
            // rather than given its own, because a second `agent`
            // command would have shadowed this one — which is exactly
            // what a first cut did, and playing it found in one line.
            Some(handle) if words.clone().next().is_some() => {
                let request: Vec<&str> = words.collect();
                let answer = agent::handle(world, handle, &request.join(" "));
                println!("{}", answer.wire(handle));
            }
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
        Some("bench") => workbench_panel(world, style, *current),
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

/// Desk entry: one-shot pane render, or interactive loop (full ANSI).
fn desk_command(
    world: &mut World,
    mut pane: desk::Pane,
    style: &Style,
    current: u64,
    mut words: std::str::SplitWhitespace<'_>,
) {
    let enter = matches!(words.next(), Some("enter") | Some("loop") | Some("i"));
    let show = |world: &World, pane: desk::Pane| {
        for line in desk::render_frame(*style, world, pane, current) {
            println!("{line}");
        }
    };
    show(world, pane);
    if !enter {
        println!(
            "  {} {} · pane keys {} · {}",
            style.hint("hint:"),
            style.verb("desk enter"),
            style.key("t y r a f x g d e"),
            style.key("help desk")
        );
        return;
    }
    let stdin = io::stdin();
    loop {
        print!("{} ", style.prompt("desk»"));
        io::stdout().flush().ok();
        let mut line = String::new();
        // See the netstratum» loop's comment: bind before matching so
        // the StdinLock temporary doesn't outlive the read and deadlock
        // a nested prompt (e.g. the guided genesis picker) reached from
        // a desk command below.
        let read = stdin.lock().read_line(&mut line);
        match read {
            Ok(0) => break,
            Ok(_) => {
                let line = line.trim();
                if line.is_empty() {
                    show(world, pane);
                    continue;
                }
                match line {
                    "q" | "quit" | "exit" | "back" => break,
                    "?" | "help" => help_menu::help(*style, Some("desk")),
                    // H1.3 — deep-link into the pane's primary hub domain.
                    "h" | "hub" => {
                        let mut cur = current;
                        command(world, pane.hub_primary(), style, &mut cur);
                        show(world, pane);
                    }
                    // H1.3 — craft twin (second hub link) when present.
                    "hc" | "hub-craft" | "craft" => {
                        let mut cur = current;
                        let target = pane.hub_craft().unwrap_or_else(|| pane.hub_primary());
                        command(world, target, style, &mut cur);
                        show(world, pane);
                    }
                    key if desk::Pane::from_key(key).is_some() => {
                        pane = desk::Pane::from_key(key).expect("just checked");
                        show(world, pane);
                    }
                    other => {
                        let mut cur = current;
                        command(world, other, style, &mut cur);
                        show(world, pane);
                    }
                }
            }
            Err(_) => break,
        }
    }
}

/// STAMP-1 player verbs (P1.6 / S4.5).
fn stamp_command(
    world: &mut World,
    mut words: std::str::SplitWhitespace<'_>,
    style: &Style,
    current: u64,
) {
    match words.next() {
        None | Some("balance") | Some("bal") => {
            let stamp = world
                .orb_by_id(current)
                .map(|o| amount(&o.stamp_amount()))
                .unwrap_or_else(|| "0/1".into());
            let crystal = world
                .orb_by_id(current)
                .map(|o| amount(&o.banked_amount(Resource::Crystal)))
                .unwrap_or_else(|| "0/1".into());
            println!(
                "{} STAMP {} · crystal {} (mint is 1:1 identity convert)",
                style.strong("∫"),
                style.strong(&stamp),
                style.dim(&crystal)
            );
        }
        Some("mint") => {
            let Some(amt) = words.next().and_then(parse_ratio_arg) else {
                println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("stamp mint <amt>")
                );
                return;
            };
            let Some(amt) = cfe::dop::Positive::new(amt) else {
                println!("{} amount must be positive", style.bad("✗"));
                return;
            };
            match cfe::world::mint_stamp_from_crystal(world, current, amt) {
                Ok(()) => match sync_world(world) {
                    Ok(_) => println!(
                        "{} STAMP minted 1:1 from crystal — balance {}",
                        style.ok("∫"),
                        style.strong(&amount(
                            &world
                                .orb_by_id(current)
                                .map(|o| o.stamp_amount())
                                .unwrap_or_else(|| ratio(0, 1))
                        ))
                    ),
                    Err(err) => println!(
                        "{} minted but sync refused: {err}",
                        style.bad("✗")
                    ),
                },
                Err(refusal) => refuse(world, &refusal, style),
            }
        }
        Some("aperture") => {
            // stamp aperture <amt> to <world> deal <n>
            let amt = words.next().and_then(parse_ratio_arg);
            let target = match (words.next(), words.next()) {
                (Some("to"), Some(name)) => Some(name.to_string()),
                _ => None,
            };
            let deal = match (words.next(), words.next()) {
                (Some("deal"), Some(n)) => n.parse::<u64>().ok(),
                _ => None,
            };
            match (amt, target, deal) {
                (Some(amt), Some(target), Some(deal)) => match far_world(&target) {
                    Ok(mut far) => {
                        let Some(amt) = cfe::dop::Positive::new(amt) else {
                            println!("{} amount must be positive", style.bad("✗"));
                            return;
                        };
                        let my_name = world.name().to_string();
                        let distance = conductor::spatial_l1(
                            &charted_position(&my_name),
                            &charted_position(&target),
                        );
                        let melt = cfe::world::route_melt(world, &far, &distance);
                        match cfe::world::stamp_aperture_transfer(
                            world,
                            current,
                            &mut far,
                            cfe::world::FIRST_ORB,
                            amt,
                            deal,
                            &melt,
                        ) {
                            Ok(()) => match sync_world(world).and(sync_world_at(&far, &target)) {
                                Ok(_) => println!(
                                    "{} STAMP aperture deal {} — melt {} both books sealed",
                                    style.ok("∫⎋"),
                                    style.strong(&deal.to_string()),
                                    style.dim(&amount(&melt))
                                ),
                                Err(err) => println!(
                                    "{} deed landed but sync refused: {err}",
                                    style.bad("✗")
                                ),
                            },
                            Err(refusal) => refuse(world, &refusal, style),
                        }
                    }
                    Err(err) => println!("{} {err}", style.bad("✗")),
                },
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("stamp aperture <amt> to <world> deal <n>")
                ),
            }
        }
        Some(other) => println!(
            "{} unknown stamp verb {other} — balance · mint · aperture",
            style.bad("✗")
        ),
    }
}

/// FLEET-1 player verbs (Phase 3).
fn fleet_command(
    world: &mut World,
    mut words: std::str::SplitWhitespace<'_>,
    style: &Style,
    current: u64,
) {
    match words.next() {
        None | Some("list") | Some("status") => {
            let scrap = world
                .orb_by_id(current)
                .map(|o| amount(&o.scrap_amount()))
                .unwrap_or_else(|| "0/1".into());
            let cap = amount(&world.free_capacity(0));
            println!(
                "{} fleet — scrap {} · free capacity {} · hulls {}",
                style.route("🚢"),
                style.cargo(&scrap),
                style.value(&cap),
                style.strong(&world.hulls().len().to_string())
            );
            let metab = world
                .committed()
                .genesis
                .as_ref()
                .map(|g| g.metabolism);
            for h in world.hulls() {
                let role = cfe::dop::HullRole::from_wire(h.role)
                    .map(|r| cfe::world::hull_voice(r, metab))
                    .unwrap_or("?");
                let cap = amount(&cfe::dop::hull_capacity(h.role, h.grade));
                let assign = if h.assigned_route == 0 {
                    style.dim("idle")
                } else {
                    style.route(&format!("deal {}", h.assigned_route))
                };
                println!(
                    "  hull {} · {} g{} · cap {} · {}",
                    style.strong(&h.hull_id.to_string()),
                    style.accent(role),
                    h.grade,
                    style.cargo(&cap),
                    assign
                );
            }
            if world.hulls().is_empty() {
                println!(
                    "  {}",
                    style.hint("build zone yard · fleet salvage <amt> · fleet mint hauler")
                );
            }
        }
        Some("salvage") => {
            // fleet salvage <amt>  |  fleet salvage element <amt> <prefix>
            if matches!(words.clone().next(), Some("element") | Some("elem")) {
                words.next();
                let amt = words.next().and_then(parse_ratio_arg);
                let prefix = words.next().map(str::to_string);
                match (amt, prefix) {
                    (Some(amt), Some(prefix)) => {
                        let element = world.orb_by_id(current).and_then(|orb| {
                            orb.elements
                                .iter()
                                .map(|(id, _)| *id)
                                .find(|id| hex_prefix(id).starts_with(&prefix))
                        });
                        let Some(element) = element else {
                            println!(
                                "{} no banked element matching {} — `elements` lists stock",
                                style.bad("✗"),
                                style.strong(&prefix)
                            );
                            return;
                        };
                        let Some(amt) = cfe::dop::Positive::new(amt) else {
                            println!("{} amount must be positive", style.bad("✗"));
                            return;
                        };
                        match world.salvage_element_scrap(current, element, amt) {
                            Ok(()) => {
                                let _ = sync_world(world);
                                println!(
                                    "{} element salvaged to scrap — bank {}",
                                    style.ok("⚙"),
                                    style.cargo(&amount(
                                        &world
                                            .orb_by_id(current)
                                            .map(|o| o.scrap_amount())
                                            .unwrap_or_else(|| ratio(0, 1))
                                    ))
                                );
                            }
                            Err(refusal) => refuse(world, &refusal, style),
                        }
                    }
                    _ => println!(
                        "{} usage: {}",
                        style.bad("✗"),
                        style.verb("fleet salvage element <amt> <prefix>")
                    ),
                }
                return;
            }
            let Some(amt) = words.next().and_then(parse_ratio_arg) else {
                println!(
                    "{} usage: {} · {}",
                    style.bad("✗"),
                    style.verb("fleet salvage <amt>"),
                    style.verb("fleet salvage element <amt> <prefix>")
                );
                return;
            };
            let Some(amt) = cfe::dop::Positive::new(amt) else {
                println!("{} amount must be positive", style.bad("✗"));
                return;
            };
            match world.salvage_scrap(current, amt) {
                Ok(()) => {
                    let _ = sync_world(world);
                    println!(
                        "{} scrap banked — {}",
                        style.ok("⚙"),
                        style.cargo(&amount(
                            &world
                                .orb_by_id(current)
                                .map(|o| o.scrap_amount())
                                .unwrap_or_else(|| ratio(0, 1))
                        ))
                    );
                }
                Err(refusal) => refuse(world, &refusal, style),
            }
        }
        Some("mint") => {
            let role_word = words.next().unwrap_or("hauler");
            let role = match role_word {
                "hauler" => cfe::dop::HullRole::Hauler,
                "semi" => cfe::dop::HullRole::Semi,
                "runner" => cfe::dop::HullRole::Runner,
                "escort" => cfe::dop::HullRole::Escort,
                _ => {
                    println!(
                        "{} role must be hauler|semi|runner|escort",
                        style.bad("✗")
                    );
                    return;
                }
            };
            let grade = words
                .next()
                .and_then(|w| w.parse::<u8>().ok())
                .unwrap_or(0);
            let finance = matches!(words.next(), Some("finance") | Some("note") | Some("fin"));
            if finance {
                match world.mint_hull_financed(current, role, grade) {
                    Ok((id, note)) => {
                        let _ = sync_world(world);
                        println!(
                            "{} hull {} minted ({}){}",
                            style.ok("🚢"),
                            style.strong(&id.to_string()),
                            style.accent(cfe::world::hull_voice(
                                role,
                                world.committed().genesis.as_ref().map(|g| g.metabolism)
                            )),
                            note.map(|n| format!(" · yard note {n}"))
                                .unwrap_or_default()
                        );
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                }
            } else {
                match world.mint_hull(current, role, grade) {
                    Ok(id) => {
                        let _ = sync_world(world);
                        println!(
                            "{} hull {} minted ({})",
                            style.ok("🚢"),
                            style.strong(&id.to_string()),
                            style.accent(cfe::world::hull_voice(
                                role,
                                world.committed().genesis.as_ref().map(|g| g.metabolism)
                            ))
                        );
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                }
            }
        }
        Some("assign") => {
            let hull_id = words.next().and_then(|w| w.parse::<u64>().ok());
            let deal = words.next().and_then(|w| w.parse::<u64>().ok());
            match (hull_id, deal) {
                (Some(hull_id), Some(deal)) => match world.assign_hull(hull_id, deal) {
                    Ok(()) => {
                        let _ = sync_world(world);
                        println!(
                            "{} hull {} → deal {}",
                            style.ok("⇉"),
                            style.strong(&hull_id.to_string()),
                            style.route(&deal.to_string())
                        );
                    }
                    Err(refusal) => refuse(world, &refusal, style),
                },
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.verb("fleet assign <hull_id> <deal|0>")
                ),
            }
        }
        Some("scrap") => {
            let Some(hull_id) = words.next().and_then(|w| w.parse::<u64>().ok()) else {
                println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.verb("fleet scrap <hull_id>")
                );
                return;
            };
            match world.scrap_hull(hull_id) {
                Ok(()) => {
                    let _ = sync_world(world);
                    println!("{} hull {} scrapped", style.ok("⚙"), style.strong(&hull_id.to_string()));
                }
                Err(refusal) => refuse(world, &refusal, style),
            }
        }
        Some(other) => println!(
            "{} unknown fleet verb {other} — list · salvage · mint · assign · scrap",
            style.bad("✗")
        ),
    }
}

/// Element aperture + charter player verbs (P1.7).
fn element_command(
    world: &mut World,
    mut words: std::str::SplitWhitespace<'_>,
    style: &Style,
    current: u64,
) {
    match words.next() {
        Some("aperture") => {
            // element aperture <amt> <prefix> to <world> deal <n>
            let amt = words.next().and_then(parse_ratio_arg);
            let prefix = words.next().map(str::to_string);
            let target = match (words.next(), words.next()) {
                (Some("to"), Some(name)) => Some(name.to_string()),
                _ => None,
            };
            let deal = match (words.next(), words.next()) {
                (Some("deal"), Some(n)) => n.parse::<u64>().ok(),
                _ => None,
            };
            match (amt, prefix, target, deal) {
                (Some(amt), Some(prefix), Some(target), Some(deal)) => {
                    let element = world.orb_by_id(current).and_then(|orb| {
                        orb.elements
                            .iter()
                            .map(|(id, _)| *id)
                            .find(|id| hex_prefix(id).starts_with(&prefix))
                    });
                    let Some(element) = element else {
                        println!(
                            "{} no banked element matching prefix {} — `elements` lists stock",
                            style.bad("✗"),
                            style.strong(&prefix)
                        );
                        return;
                    };
                    let Some(amt) = cfe::dop::Positive::new(amt) else {
                        println!("{} amount must be positive", style.bad("✗"));
                        return;
                    };
                    match far_world(&target) {
                        Ok(mut far) => {
                            let my_name = world.name().to_string();
                            let distance = conductor::spatial_l1(
                                &charted_position(&my_name),
                                &charted_position(&target),
                            );
                            let melt = cfe::world::route_melt(world, &far, &distance);
                            match cfe::world::element_aperture_transfer(
                                world,
                                current,
                                &mut far,
                                cfe::world::FIRST_ORB,
                                element,
                                amt,
                                deal,
                                &melt,
                            ) {
                                Ok(()) => match sync_world(world).and(sync_world_at(&far, &target))
                                {
                                    Ok(_) => println!(
                                        "{} element aperture deal {} · {} · melt {} — both books sealed",
                                        style.ok("⇢"),
                                        style.strong(&deal.to_string()),
                                        style.dim(&hex_prefix(&element)),
                                        style.dim(&amount(&melt))
                                    ),
                                    Err(err) => println!(
                                        "{} deed landed but sync refused: {err}",
                                        style.bad("✗")
                                    ),
                                },
                                Err(refusal) => refuse(world, &refusal, style),
                            }
                        }
                        Err(err) => println!("{} {err}", style.bad("✗")),
                    }
                }
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("element aperture <amt> <prefix> to <world> deal <n>")
                ),
            }
        }
        Some("charter") => {
            // element charter <world> <prefix> deal <n> [rail <1-4>]
            let target = words.next().map(str::to_string);
            let prefix = words.next().map(str::to_string);
            let deal = match (words.next(), words.next()) {
                (Some("deal"), Some(n)) => n.parse::<u64>().ok(),
                _ => None,
            };
            let mut rail: u8 = 2; // default R2 aperture for element cargo
            if let (Some("rail"), Some(n)) = (words.next(), words.next()) {
                if let Ok(r) = n.parse::<u8>() {
                    rail = r;
                }
            }
            match (target, prefix, deal) {
                (Some(target), Some(prefix), Some(deal)) => {
                    let element = world.orb_by_id(current).and_then(|orb| {
                        orb.elements
                            .iter()
                            .map(|(id, _)| *id)
                            .find(|id| hex_prefix(id).starts_with(&prefix))
                    });
                    // Element identity may also be known only from prospects.
                    let element = element.or_else(|| {
                        world.committed().prospects.iter().find_map(|p| {
                            if hex_prefix(&p.element).starts_with(&prefix) {
                                Some(p.element)
                            } else {
                                None
                            }
                        })
                    });
                    let Some(element) = element else {
                        println!(
                            "{} no element matching {} — prospect/assay or bank first",
                            style.bad("✗"),
                            style.strong(&prefix)
                        );
                        return;
                    };
                    match far_world(&target) {
                        Ok(mut far) => {
                            let best_grade = world
                                .instruments()
                                .iter()
                                .chain(far.instruments().iter())
                                .map(|(_, _, grade)| *grade)
                                .max()
                                .unwrap_or(0);
                            let Some(eta) =
                                cfe::dop::Unit::new(ratio(1 + i64::from(best_grade), 8))
                            else {
                                println!(
                                    "{} grade {best_grade} prices η ≥ 1 — no lane erases the void",
                                    style.bad("✗")
                                );
                                return;
                            };
                            match cfe::world::charter_element_route(
                                world,
                                current,
                                &mut far,
                                cfe::world::FIRST_ORB,
                                element,
                                eta,
                                rail,
                                deal,
                            ) {
                                Ok(()) => match sync_world(world).and(sync_world_at(&far, &target))
                                {
                                    Ok(_) => println!(
                                        "{} element route CHARTERS deal {} · rail {} · element {}",
                                        style.ok("⇉⇢"),
                                        style.strong(&deal.to_string()),
                                        style.strong(&rail.to_string()),
                                        style.dim(&hex_prefix(&element))
                                    ),
                                    Err(err) => println!(
                                        "{} charter landed but sync refused: {err}",
                                        style.bad("✗")
                                    ),
                                },
                                Err(refusal) => refuse(world, &refusal, style),
                            }
                        }
                        Err(err) => println!("{} {err}", style.bad("✗")),
                    }
                }
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("element charter <world> <prefix> deal <n> [rail <1-4>]")
                ),
            }
        }
        None => println!(
            "{} usage: element aperture … · element charter …",
            style.bad("✗")
        ),
        Some(other) => println!(
            "{} unknown element verb {other} — aperture · charter",
            style.bad("✗")
        ),
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
                "regen" => Some(ZoneKind::Regenerator),
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
        ["book", "mint", name @ ..] if !name.is_empty() => {
            let proven = match cfe::books::BookName::new(&name.join(" ")) {
                Ok(proven) => proven,
                Err(fault) => {
                    println!("{} {}", style.bad("✗"), book_name_fault(&fault, style));
                    return;
                }
            };
            match world.mint_book_as(actor, *current, &proven) {
                Ok(_) => println!(
                    "{} entity {actor} minted a book — receipt tagged to it",
                    style.ok("✓")
                ),
                Err(refusal) => refuse(world, &refusal, style),
            }
        }
        ["book", "post", kept, lost, name @ ..] if !name.is_empty() => {
            match (
                parse_ratio_arg(kept).and_then(cfe::dop::NonNegative::new),
                parse_ratio_arg(lost).and_then(cfe::dop::NonNegative::new),
            ) {
                (Some(kept), Some(lost)) => {
                    let kind = cfe::books::kind_of(&name.join(" "));
                    match world.post_book_as(actor, *current, kind, kept, lost) {
                        Ok(()) => println!(
                            "{} entity {actor} posted a dual entry — receipt tagged to it",
                            style.ok("✓")
                        ),
                        Err(refusal) => refuse(world, &refusal, style),
                    }
                }
                _ => println!("{} as <handle> book post <kept> <lost> <name>", style.bad("✗")),
            }
        }
        ["book", "archive", name @ ..] | ["book", "close", name @ ..] if !name.is_empty() => {
            let kind = cfe::books::kind_of(&name.join(" "));
            match world.archive_book_as(actor, *current, kind) {
                Ok(()) => println!(
                    "{} entity {actor} closed a book to writing — receipt tagged to it",
                    style.ok("✓")
                ),
                Err(refusal) => refuse(world, &refusal, style),
            }
        }
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
            style.strong("zone · raze · collect · book")
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

/// N4 (NS-2 RQP) — the scanner: the lens beside the book. The
/// CURRENT planet pays the draw; the exemplar's structure is the
/// query; hits and near-misses render with their deltas whole.
fn scan(world: &mut World, style: &Style, prober: u64, exemplar: u64) {
    match world.scan(prober, exemplar) {
        Ok(read) => {
            let named = |orb: u64| {
                world
                    .orb_by_id(orb)
                    .map(|held| held.name.clone())
                    .unwrap_or_else(|| format!("orb {orb}"))
            };
            println!(
                "{} the lens opens on {} — {} crystal drawn, stamp {} (the looking's only receipt)",
                style.head("◎"),
                style.strong(&named(exemplar)),
                style.strong(&amount(&cfe::dop::scan_draw())),
                read.stamp
            );
            println!(
                "  address: {} zone(s) under the grid symmetry · resources {} · band {}",
                read.probed.s_canonical.len(),
                match read.probed.g_resources.is_empty() {
                    true => "none".to_string(),
                    false => read
                        .probed
                        .g_resources
                        .iter()
                        .filter_map(|code| Resource::from_wire(*code))
                        .map(resource_name)
                        .collect::<Vec<_>>()
                        .join("+"),
                },
                band_word(read.probed.thermal)
            );
            for hit in &read.hits {
                let survival = read
                    .translations
                    .iter()
                    .find(|(orb, _)| orb == hit)
                    .map(|(_, held)| held.clone())
                    .unwrap_or_else(|| ratio(1, 1));
                let translated = match survival == ratio(1, 1) {
                    true => "full structure survives (one frame)".to_string(),
                    false => format!(
                        "{} of structure survives your read — build a resonance array to close the gap",
                        amount(&survival)
                    ),
                };
                println!(
                    "  {} resonance: {} — the same shape through the lens · {}",
                    style.ok("✓"),
                    style.strong(&named(*hit)),
                    style.dim(&translated)
                );
            }
            for miss in &read.refusals {
                let mut parts: Vec<String> = Vec::new();
                for (resource, residual) in &miss.banked_residuals {
                    parts.push(format!("{} {}", resource_name(*resource), amount(residual)));
                }
                for (tile, ours, theirs) in &miss.zone_deltas {
                    parts.push(format!(
                        "tile {} ({}) {}↔{}",
                        tile,
                        coordinate(*tile),
                        kind_word(*ours),
                        kind_word(*theirs)
                    ));
                }
                println!(
                    "  {} near-miss: {} — delta whole: {} · C(Δ) = {}",
                    style.bad("✗"),
                    style.strong(&named(miss.orb)),
                    parts.join(" · "),
                    style.strong(&amount(&miss.coherence))
                );
            }
            if read.hits.is_empty() && read.refusals.is_empty() {
                println!("{}", style.dim("  (nothing resonates — the bucket is empty)"));
            }
        }
        Err(ActionRefused::CannotAfford { resource: Resource::Crystal, need, have }) => println!(
            "{} scanning draws {} crystal and you have {} — crystals come from a resonator ({})",
            style.bad("✗"),
            style.strong(&amount(&need)),
            style.strong(&amount(&have)),
            style.strong("zone <tile> res")
        ),
        Err(refusal) => refuse(world, &refusal, style),
    }
}

/// ELEMENT-GRAMMAR-1: the element id's short spoken form.
fn hex_prefix(element: &[u8; 32]) -> String {
    element[..4].iter().map(|byte| format!("{byte:02x}")).collect()
}

/// **#42 THE POLITICAL SPECTRUM** — `treaty <kind> with <world> [deal
/// <n>] [for <n>] [melt <p>/<q>]`, and `treaties` to read what stands.
/// Deep-deed door: the DIPLOMATIC hub fires the same grammar through
/// `diplomatic local …` (call-not-copy), so the iterator is generic
/// over owned words rather than tied to the prompt's SplitWhitespace.
pub(crate) fn treaty_command(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
) {
    use cfe::treaty::{TreatyKind, TreatyTerms, PAIR_MELT};
    let kind_word = words.next();
    // Bare `treaties` (or an unknown first word) reads the board.
    let Some(kind) = kind_word.as_deref().and_then(|word| match word {
        "trade" => Some(TreatyKind::Trade),
        "defense" | "defence" => Some(TreatyKind::Defense),
        "peace" => Some(TreatyKind::Peace),
        "war" => Some(TreatyKind::War),
        _ => None,
    }) else {
        // E3.9: fence a standing trade pact to enumerated matter.
        // treaty elements <deal> <world> <id-prefix>...
        if let Some("elements") = kind_word.as_deref() {
            let deal = words.next().and_then(|w| w.parse::<u64>().ok());
            let partner = words.next();
            let prefixes: Vec<String> = words.collect();
            let (Some(deal), Some(partner)) = (deal, partner) else {
                return println!(
                    "{} usage: {} — prefixes resolve against banked stock and open veins",
                    style.bad("✗"),
                    style.strong("treaty elements <deal> <world> <id-prefix>...")
                );
            };
            if prefixes.is_empty() {
                return refuse(world, &ActionRefused::AllowListEmpty, style);
            }
            // Resolve each prefix against every id this book knows —
            // banked stores and opened veins alike (a fence may name
            // matter you hold or matter you mean to receive).
            let known: Vec<[u8; 32]> = {
                let committed = world.committed();
                committed
                    .orbs
                    .iter()
                    .flat_map(|orb| orb.elements.iter().map(|(id, _)| *id))
                    .chain(committed.prospects.iter().map(|vein| vein.element))
                    .collect()
            };
            let mut elements = Vec::new();
            for prefix in &prefixes {
                match known.iter().find(|id| hex_prefix(id).starts_with(prefix.as_str())) {
                    Some(id) => {
                        if !elements.contains(id) {
                            elements.push(*id);
                        }
                    }
                    None => {
                        return println!(
                            "{} no known element matches prefix {} — `elements` lists stock, \
                             `prospects` the open veins",
                            style.bad("✗"),
                            style.strong(prefix)
                        )
                    }
                }
            }
            return match world.treaty_elements(deal, &partner, elements.clone()) {
                Ok(()) => println!(
                    "{} the pact with {} is FENCED — {} element id(s) may cross; all else refuses",
                    style.ok("⚑"),
                    style.strong(&partner),
                    style.strong(&elements.len().to_string())
                ),
                Err(refusal) => refuse(world, &refusal, style),
            };
        }
        if let Some("renounce") = kind_word.as_deref() {
            let deal = words.next().and_then(|w| w.parse::<u64>().ok());
            let partner = words.next();
            return match (deal, partner) {
                (Some(deal), Some(partner)) => match world.renounce(deal, &partner) {
                    Ok(()) => println!(
                        "{} the pact with {} is renounced — your own leg, and only yours",
                        style.ok("⚑"),
                        style.strong(&partner)
                    ),
                    Err(refusal) => refuse(world, &refusal, style),
                },
                _ => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("treaty renounce <deal> <world>")
                ),
            };
        }
        let standing = world.treaties();
        println!(
            "{} THE PACTS — what {} has agreed",
            style.strong("⚑"),
            style.strong(world.name())
        );
        if standing.is_empty() {
            println!(
                "  {}",
                style.dim(
                    "none stand — a pact re-prices the crossing between two worlds; both must \
                     seal it, and either may renounce its own leg"
                )
            );
        }
        for pact in &standing {
            println!(
                "  {} with {} · deal {}{} [{}]",
                style.strong(pact.kind.word()),
                style.strong(&pact.partner),
                pact.deal,
                match pact.term {
                    0 => String::from(" · standing"),
                    held => format!(" · until t{}", pact.sealed + held),
                },
                pact.terms
                    .iter()
                    .map(|(channel, factor)| format!(
                        "{}×{}",
                        match *channel {
                            PAIR_MELT => "melt",
                            _ => "?",
                        },
                        amount(factor)
                    ))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        // E3.9: the fences — live allow-lists riding the pacts above.
        let fences = &world.committed().element_allowlists;
        for (deal, partner, elements) in fences {
            if standing
                .iter()
                .any(|pact| pact.deal == *deal && pact.partner == *partner)
            {
                println!(
                    "  {} deal {} with {} — {}",
                    style.strong("fence"),
                    deal,
                    style.strong(partner),
                    elements
                        .iter()
                        .map(hex_prefix)
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
        }
        if !standing.is_empty() {
            println!(
                "  {} {}",
                style.dim("↳"),
                style.dim("treaty <trade|defense|peace|war> with <world> [deal <n>] [for <n>] [melt <p>/<q>]")
            );
            println!(
                "  {} {}",
                style.dim("↳"),
                style.dim("treaty elements <deal> <world> <id-prefix>... — fence a trade pact to enumerated matter")
            );
        }
        return;
    };
    // `with <world>` names the partner; the rest is optional.
    let mut partner: Option<String> = None;
    let mut deal: u64 = 1;
    let mut term: u64 = 0;
    let mut melt: Option<cfe::ratio::Ratio> = None;
    let mut malformed: Option<String> = None;
    while let Some(word) = words.next() {
        match word.as_str() {
            "with" => partner = words.next(),
            "deal" => deal = words.next().and_then(|w| w.parse().ok()).unwrap_or(1),
            "for" => term = words.next().and_then(|w| w.parse().ok()).unwrap_or(0),
            "melt" => {
                let held = words.next().unwrap_or_default();
                let (numer, denom) = held.split_once('/').unwrap_or((held.as_str(), "1"));
                match (numer.parse::<i64>(), denom.parse::<i64>()) {
                    (Ok(n), Ok(d)) if d != 0 => melt = Some(cfe::ratio::ratio(n, d)),
                    _ => malformed = Some(held.clone()),
                }
            }
            other => malformed = Some(other.to_string()),
        }
    }
    if let Some(word) = malformed {
        println!(
            "{} {} is not part of a pact — {}",
            style.bad("✗"),
            style.strong(&word),
            style.dim("treaty <kind> with <world> [deal <n>] [for <n>] [melt <p>/<q>]")
        );
        return;
    }
    let Some(partner) = partner else {
        println!(
            "{} a pact needs a partner — {}",
            style.bad("✗"),
            style.strong("treaty <kind> with <world> …")
        );
        return;
    };
    // **THE ENGINE CHOOSES NO TERMS.** A first cut defaulted the melt
    // to a halving for a concord and a doubling for a war — two dials,
    // and the ratchet counted them. It was right to: what a pact costs
    // is the PARTIES' business, and an engine that picks it is writing
    // policy nobody agreed to. So a concord or a discord must SAY its
    // factor; only a peace, which binds nothing, needs no terms.
    let terms = match kind {
        TreatyKind::Peace => Vec::new(),
        _ => match melt {
            Some(factor) => vec![(PAIR_MELT, factor)],
            None => {
                println!(
                    "{} a {} pact must SAY what it does to the crossing — {} (the engine \
                     chooses no terms; a concord must ease and a war must burden, inside \
                     the fourfold band)",
                    style.bad("✗"),
                    style.strong(kind.word()),
                    style.strong("melt <p>/<q>")
                );
                return;
            }
        },
    };
    // The trade law is proven at THIS door, fault named: a pact that
    // contradicts its own kind never reaches the verb.
    let terms = match TreatyTerms::new(kind, terms) {
        Ok(terms) => terms,
        Err(fault) => {
            use cfe::treaty::TreatyFault;
            println!(
                "{} {}",
                style.bad("✗"),
                match fault {
                    TreatyFault::PeaceBindsNothing =>
                        "a PEACE binds nothing — that absence is the pact; drop the terms".into(),
                    TreatyFault::TermsEmpty =>
                        "a pact that changes nothing between two worlds is not a pact".into(),
                    TreatyFault::TermsContradictKind { .. } => format!(
                        "those terms contradict a {} pact — a concord must EASE the crossing \
                         and a war must BURDEN it",
                        kind.word()
                    ),
                    TreatyFault::FactorOutOfBounds { .. } =>
                        "the factor falls outside the fourfold band [1/4, 4]".to_string(),
                    TreatyFault::ChannelUnknown { channel } => format!(
                        "channel {channel} is not one the engine computes BETWEEN two worlds"
                    ),
                }
            );
            return;
        }
    };
    match world.treaty(deal, &partner, terms, term) {
        Ok(()) => {
            println!(
                "{} {} sealed with {} — this world's leg; {} must seal theirs for it to bind",
                style.ok("⚑"),
                style.strong(kind.word()),
                style.strong(&partner),
                style.strong(&partner)
            );
            if kind == TreatyKind::War {
                println!(
                    "  {} {}",
                    style.warn("⚠"),
                    style.dim("the crossing is now WALLED — a pair at war exchanges nothing")
                );
            }
        }
        Err(refusal) => refuse(world, &refusal, style),
    }
}

/// **THE CORPUS BENCH (#38a)** — `corpus` serves a priced task,
/// `corpus try` tests a hypothesis against the training pairs, and
/// `corpus solve` seals the proof and mints what the carrier law
/// prices it at.
fn corpus_command(
    world: &mut World,
    mut words: std::str::SplitWhitespace,
    style: &Style,
    current: u64,
) {
    let dir = corpus::corpus_dir();
    let served = corpus::priced(&dir);
    if served.is_empty() {
        println!(
            "{} no priced corpus tasks under {} — the carrier table was measured over that \
             archive; point {} at it",
            style.bad("✗"),
            style.strong(&dir.display().to_string()),
            style.strong("NETSTRATUM_CORPUS")
        );
        return;
    }
    let color = !style.plain;
    match words.next() {
        // `corpus` / `corpus <n>` — serve a task.
        None | Some("show") => show_task(&served, 0, style, color, world),
        Some(word) if word.parse::<usize>().is_ok() => {
            let at = word.parse::<usize>().expect("just parsed");
            match at < served.len() {
                true => show_task(&served, at, style, color, world),
                false => println!(
                    "{} the priced set holds {} tasks (0–{})",
                    style.bad("✗"),
                    style.strong(&served.len().to_string()),
                    served.len() - 1
                ),
            }
        }
        Some("list") => {
            println!(
                "{} THE PRICED SET — {} tasks the carrier law pays for",
                style.strong("✻"),
                style.strong(&served.len().to_string())
            );
            for (at, held) in served.iter().enumerate() {
                let earned = kernel::corpus_weight(&held.task_id);
                println!(
                    "  {:>2}  {} · {} train pairs{} · pays {}{}",
                    at,
                    style.strong(&hex_prefix(&held.task_id)),
                    held.task.train.len(),
                    match held.blind {
                        true => format!(" · {}", style.strong("BLIND")),
                        false => String::new(),
                    },
                    style.strong(
                        &earned.map(|w| amount(&w)).unwrap_or_else(|| "—".into())
                    ),
                    match corpus::already_earned(world.chronicle(), &held.task_id) {
                        true => format!(" · {}", style.dim("already earned here")),
                        false => String::new(),
                    }
                );
            }
        }
        Some(verb @ ("try" | "solve")) => {
            let rest: Vec<&str> = words.collect();
            let (at, program_words) = match rest.split_first() {
                Some((first, tail)) if first.parse::<usize>().is_ok() => {
                    (first.parse::<usize>().expect("just parsed"), tail)
                }
                _ => (0usize, rest.as_slice()),
            };
            if at >= served.len() {
                println!("{} no task {at} in the priced set", style.bad("✗"));
                return;
            }
            if program_words.is_empty() {
                println!(
                    "{} usage: {} — e.g. {}",
                    style.bad("✗"),
                    style.strong(&format!("corpus {verb} [<n>] <move> <move> …")),
                    style.strong(&format!("corpus {verb} 0 tile:3x3"))
                );
                return;
            }
            match corpus::parse_program(program_words) {
                Err(complaint) => println!("{} {complaint}", style.bad("✗")),
                Ok(program) => match verb {
                    "try" => try_program(&served[at], &program, style, color),
                    _ => solve_task(world, &served[at], program, style, current),
                },
            }
        }
        Some(other) => println!(
            "{} {other} is not a corpus verb — {} · {} · {} · {}",
            style.bad("✗"),
            style.strong("corpus [<n>]"),
            style.strong("corpus list"),
            style.strong("corpus try <move>…"),
            style.strong("corpus solve <move>…")
        ),
    }
}

fn show_task(
    served: &[corpus::Served],
    at: usize,
    style: &Style,
    color: bool,
    world: &World,
) {
    let held = &served[at];
    let earned = kernel::corpus_weight(&held.task_id);
    println!(
        "{} CORPUS TASK {} of {} — {}{}",
        style.strong("✻"),
        style.strong(&at.to_string()),
        served.len() - 1,
        style.strong(&hex_prefix(&held.task_id)),
        match held.blind {
            true => format!(" · {}", style.strong("BLIND SPOT")),
            false => String::new(),
        }
    );
    println!(
        "  {}",
        style.dim(
            "find the RULE, not the answer: a proof is a program that reproduces every \
             training pair and then the test input"
        )
    );
    for (n, pair) in held.task.train.iter().enumerate() {
        println!("  {} {}", style.dim("train"), style.dim(&n.to_string()));
        let (input, output) = corpus::pair_grids(pair);
        for row in corpus::beside(input, output, "→", color) {
            println!("    {row}");
        }
    }
    println!("  {}", style.dim("test input"));
    for row in corpus::paint(&held.task.test_in, color) {
        println!("    {row}");
    }
    println!(
        "  {} pays {} · {} to test a rule · {} to submit",
        style.dim("↳"),
        style.strong(&earned.map(|w| amount(&w)).unwrap_or_else(|| "—".into())),
        style.strong(&format!("corpus try {at} <move>…")),
        style.strong(&format!("corpus solve {at} <move>…"))
    );
    if corpus::already_earned(world.chronicle(), &held.task_id) {
        println!(
            "  {} {}",
            style.warn("⚠"),
            style.dim("this world has already earned from this task — a second mint pays nothing")
        );
    }
}

fn try_program(served: &corpus::Served, program: &solver_verify::Program, style: &Style, color: bool) {
    let verdicts = corpus::train_verdicts(&served.task, program);
    let held = verdicts.iter().filter(|held| **held).count();
    for (n, (pair, ok)) in served.task.train.iter().zip(&verdicts).enumerate() {
        let (input, _) = corpus::pair_grids(pair);
        let got = solver_verify::run(program, input);
        println!(
            "  {} {} {}",
            style.dim("train"),
            style.dim(&n.to_string()),
            match ok {
                true => style.ok("✓ explained"),
                false => style.bad("✗ not this rule"),
            }
        );
        if !ok {
            // SHOW THE MISS. A rule that fails is information, and a
            // bench that only says "no" teaches nothing.
            for row in corpus::beside(&got, &pair.output, "vs", color) {
                println!("      {row}");
            }
        }
    }
    match held == verdicts.len() {
        true => println!(
            "  {} every training pair explained — {} to seal it and mint",
            style.ok("✓"),
            style.strong("corpus solve")
        ),
        false => println!(
            "  {} {held}/{} explained — the rule has to hold for ALL of them",
            style.warn("⚠"),
            verdicts.len()
        ),
    }
}

fn solve_task(
    world: &mut World,
    served: &corpus::Served,
    program: solver_verify::Program,
    style: &Style,
    current: u64,
) {
    // The bench refuses to seal a rule that does not explain the
    // training pairs — the gate would refuse it anyway (TrainingMiss),
    // and failing here says WHY rather than "the work did not verify".
    let verdicts = corpus::train_verdicts(&served.task, &program);
    if verdicts.iter().any(|held| !held) {
        let held = verdicts.iter().filter(|h| **h).count();
        println!(
            "{} that rule explains {held}/{} training pairs — {} shows where it misses",
            style.bad("✗"),
            verdicts.len(),
            style.strong("corpus try")
        );
        return;
    }
    // #38d — the once-per-task law, at the emitter (the staleness
    // precedent): the chronicle is what knows, and cfe may not read a
    // payload.
    if corpus::already_earned(world.chronicle(), &served.task_id) {
        println!(
            "{} this world already earned from {} — cognition pays for what it FIRST explained",
            style.bad("✗"),
            style.strong(&hex_prefix(&served.task_id))
        );
        return;
    }
    let proof = corpus::seal(served, program);
    let Some(payload) = corpus::payload(&proof) else {
        println!("{} that proof does not encode — nothing was minted", style.bad("✗"));
        return;
    };
    // THE AMOUNT IS THE KERNEL'S OWN READING, never a number chosen
    // here: the same `carrier_bound` the gate enforces at I4.
    let Some(earned) = kernel::corpus_weight(&served.task_id) else {
        println!(
            "{} that task carries no measured price — the carrier law refuses an unpriced mint",
            style.bad("✗")
        );
        return;
    };
    let Some(amount_earned) = cfe::dop::NonNegative::new(earned.clone()) else {
        println!("{} the measured price is not a lawful mint", style.bad("✗"));
        return;
    };
    match world.mint_discovery_as(
        SOVEREIGN,
        current,
        Resource::Crystal,
        amount_earned,
        payload,
        &kernel::SealedWarrant,
    ) {
        Ok(()) => {
            println!(
                "{} the rule holds — {} crystal minted against VERIFIED work",
                style.ok("✦"),
                style.strong(&amount(&earned))
            );
            println!(
                "  {} {}",
                style.dim("↳"),
                style.dim(
                    "the proof rides the receipt whole: the gate re-hashed the task, re-applied \
                     your program to every pair, and re-chained the certificate"
                )
            );
        }
        Err(refusal) => refuse(world, &refusal, style),
    }
}

/// ELEMENT-CHEMISTRY (#37): did this world MAKE this identity? Read
/// from the chronicle, which is the only place that knows — a gem and
/// a drawn element are both element ids in one ledger, and what
/// separates them is the deed that put them there.
fn is_gem(world: &cfe::world::World, element: &[u8; 32]) -> bool {
    world.chronicle().receipts().iter().any(|receipt| {
        matches!(
            &receipt.event,
            cfe::dop::ClosureEvent::GemCompounded { gem, .. } if gem == element
        )
    })
}

/// The near-miss zone-delta word: a kind wire code, 0 = unzoned.
fn kind_word(code: u8) -> &'static str {
    ZoneKind::from_wire(code).map(kind_name).unwrap_or("empty")
}

/// G1 (04 §1): the axis vocabulary — words to wire codes and back.
/// Closed, append-only; the engine validates codes, the CLI speaks
/// words.
fn parse_disposition(word: &str) -> Option<u8> {
    match word {
        "expansionist" => Some(1),
        "empiricist" => Some(2),
        "epicurean" => Some(3),
        _ => None,
    }
}

fn parse_metabolism(word: &str) -> Option<u8> {
    match word {
        "mineral" => Some(1),
        "biotic" => Some(2),
        "plasmic" => Some(3),
        _ => None,
    }
}

fn parse_perception(word: &str) -> Option<u8> {
    match word {
        "planar" => Some(1),
        "volumetric" => Some(2),
        "harmonic" => Some(3),
        "umbral" => Some(4),
        _ => None,
    }
}

/// **THE GUIDED FOUNDING** (`20_FOUNDING` §1 step 3): walks the three
/// genesis axes one at a time — DISPOSITION, METABOLISM, PERCEPTION —
/// then the civilization name, and seals. Flavor text is drawn from
/// `04_CIVILIZATION_GENESIS.md`/`44_LINEAGES.md`; the codes it returns
/// are the exact ones `parse_disposition`/`parse_metabolism`/
/// `parse_perception` already validate, so a picked axis and a typed
/// one seal identically. `q` at any prompt cancels with nothing
/// chronicled.
fn genesis_picker(world: &mut World, style: &Style) {
    println!(
        "{} {} — three axes, then a name. {} cancels at any point.",
        style.ok("✶"),
        style.strong("GUIDED FOUNDING"),
        style.key("q")
    );
    let stdin = io::stdin();
    let Some(disposition) = pick_axis(
        &stdin,
        style,
        "DISPOSITION",
        &[
            (1, "expansionist", "growth and reach — provisioning appetite, research siphon"),
            (2, "empiricist", "knowledge and precision — the research-first line"),
            (3, "epicurean", "comfort and craft — leisure-weighted provisioning"),
        ],
    ) else {
        println!("{} founding cancelled", style.dim("○"));
        return;
    };
    let Some(metabolism) = pick_axis(
        &stdin,
        style,
        "METABOLISM",
        &[
            (1, "mineral", "eats iron, farms iron — a smelter economy"),
            (2, "biotic", "eats biomass, farms biomass — the common ration"),
            (3, "plasmic", "eats joules — plasmic agriculture IS the thermal loop, encasement-fed"),
        ],
    ) else {
        println!("{} founding cancelled", style.dim("○"));
        return;
    };
    let Some(perception) = pick_axis(
        &stdin,
        style,
        "PERCEPTION",
        &[
            (1, "planar", "the 2D grid — the plain read"),
            (2, "volumetric", "reads the whole depth, not the surface"),
            (3, "harmonic", "frequency-basis — the civ that speaks in music"),
            (4, "umbral", "occlusion-basis — reads by what is shadowed"),
        ],
    ) else {
        println!("{} founding cancelled", style.dim("○"));
        return;
    };
    print!("{} ", style.prompt("civilization name>"));
    io::stdout().flush().ok();
    let mut name = String::new();
    if stdin.lock().read_line(&mut name).unwrap_or(0) == 0 {
        println!("{} founding cancelled", style.dim("○"));
        return;
    }
    let name = name.trim();
    if name.is_empty() || name == "q" {
        println!("{} founding cancelled — no name given", style.dim("○"));
        return;
    }
    match world.seal_genesis(name, disposition, metabolism, perception) {
        Ok(()) => println!(
            "{} {} seals its three axes — the binding copy lives in THIS chronicle; the registry is a cache from here on",
            style.ok("✶"),
            style.strong(name)
        ),
        Err(refusal) => refuse(world, &refusal, style),
    }
}

/// One numbered menu: prints `options`, reads a line, accepts either
/// the numeric code or the axis word typed out, loops on anything
/// else. `q`/`quit`/EOF returns `None` (cancel).
fn pick_axis(stdin: &io::Stdin, style: &Style, label: &str, options: &[(u8, &str, &str)]) -> Option<u8> {
    loop {
        println!("{}", style.strong(label));
        for (code, name, blurb) in options {
            println!("  {} {} — {}", style.key(&code.to_string()), style.strong(name), blurb);
        }
        print!("{} ", style.prompt("choice>"));
        io::stdout().flush().ok();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => return None,
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed == "q" || trimmed == "quit" {
                    return None;
                }
                if let Ok(code) = trimmed.parse::<u8>() {
                    if options.iter().any(|(c, _, _)| *c == code) {
                        return Some(code);
                    }
                }
                if let Some((code, _, _)) = options.iter().find(|(_, name, _)| *name == trimmed) {
                    return Some(*code);
                }
                println!("{} not a valid choice — type a number or the name, or {} to cancel", style.bad("✗"), style.key("q"));
            }
            Err(_) => return None,
        }
    }
}

fn parse_domain(word: &str) -> Option<u8> {
    match word.to_ascii_uppercase().as_str() {
        "CEL" | "1" => Some(1),
        "ATM" | "2" => Some(2),
        "COG" | "3" => Some(3),
        "CYB" | "4" => Some(4),
        "INF" | "5" => Some(5),
        "KIN" | "6" => Some(6),
        "MKT" | "7" => Some(7),
        _ => None,
    }
}

fn decode_hex_bytes(s: &str) -> Option<Vec<u8>> {
    let s = s.trim();
    if !s.len().is_multiple_of(2) || s.is_empty() {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

fn disposition_name(code: u8) -> &'static str {
    match code {
        1 => "expansionist",
        2 => "empiricist",
        3 => "epicurean",
        _ => "unsealed",
    }
}

fn metabolism_name(code: u8) -> &'static str {
    match code {
        1 => "mineral",
        2 => "biotic",
        3 => "plasmic",
        _ => "unsealed",
    }
}

fn perception_name(code: u8) -> &'static str {
    match code {
        1 => "planar",
        2 => "volumetric",
        3 => "harmonic",
        4 => "umbral",
        _ => "unsealed",
    }
}

/// The address band word (NS-2 §2.2 coding).
fn band_word(code: u8) -> &'static str {
    match code {
        1 => "cryogenic",
        2 => "mesic",
        3 => "hyperthermal",
        _ => "unbanded",
    }
}

/// #41: the zone's word, read from `dop::zone_word` — the vocabulary's
/// own home. This used to keep its own copy of the table, which is
/// exactly the drift the zone axis's seed made intolerable: a kind's
/// word is now vocabulary, and vocabulary is not spelled twice.
fn kind_name(kind: ZoneKind) -> &'static str {
    cfe::dop::zone_word(kind)
}

/// #41: a building's word, MINTED ONES RESOLVED. The compiled table
/// answers for its own kinds; a minted kind's word lives in the fold,
/// so the chronicle is asked for it — the same seam the minted species
/// word resolves at (#45). Falls back to the generic word where the
/// fold has no entry, which is what an unbuildable stray code is.
fn zone_title(world: &World, kind: ZoneKind) -> String {
    match kind {
        ZoneKind::Minted(code) => {
            cfe::vocabulary::fold(world.chronicle(), cfe::vocabulary::Axis::Zone)
                .iter()
                .find(|entry| entry.code == code)
                .map(|entry| entry.word.clone())
                .unwrap_or_else(|| format!("minted {code}"))
        }
        held => kind_name(held).to_string(),
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
        "crystal" | "crystals" => Some(Resource::Crystal),
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
    let t4 = thermal::t_eq4(&class.luminosity(), site.albedo.value(), site.orbit_r.value());
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
    let s = orb.syntropy_contained.clone();
    let e = orb.entropy_vented.clone();
    println!(
        "  books S {} · E {} · {}",
        style.ok(&amount(&s)),
        style.bad(&amount(&e)),
        match world.thermal_balance_of(current) {
            None => style.dim("balance unbooked (bootstrap)"),
            Some(b) if b >= cfe::world::World::balance_orderly_floor() => {
                style.ok(&format!("balance {} ORDERLY", amount(&b)))
            }
            Some(b) => style.bad(&format!(
                "balance {} STRAINED — regenerator unlocks order-work",
                amount(&b)
            )),
        }
    );
    if world.regenerator_stands(current) {
        println!("  {}", style.ok("regenerator stands — order-work uncapped by strain"));
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
        Resource::Crystal => "crystal",
    }
}

/// E-MKT-1 display: closed carrier word or short element id hex.
fn cargo_name(cargo: &cfe::dop::Cargo) -> String {
    match cargo {
        cfe::dop::Cargo::Resource(r) => resource_name(*r).to_string(),
        cfe::dop::Cargo::Element(id) => {
            format!(
                "element-{:02x}{:02x}{:02x}{:02x}",
                id[0], id[1], id[2], id[3]
            )
        }
    }
}

fn amount(value: &Ratio) -> String {
    match value.denom() == &num_bigint::BigInt::from(1) {
        true => value.numer().to_string(),
        false => format!("{}/{}", value.numer(), value.denom()),
    }
}

/// The `BookName` door's faults, told at the client's own boundary —
/// the arms died in the engine (29_CONSTRUCTION), the words stay here.
fn book_name_fault(fault: &cfe::books::BookNameFault, style: &Style) -> String {
    match fault {
        cfe::books::BookNameFault::NormalizesToNothing => {
            "a book needs a name that is not only whitespace".to_string()
        }
        cfe::books::BookNameFault::Charter { name } => {
            format!("{} is a charter book — law, not inventory", style.strong(name))
        }
    }
}

fn refuse(world: &World, refusal: &ActionRefused, style: &Style) {
    println!("{} {}", style.bad("✗"), refuse_text(world, refusal, style));
}

/// The refusal's own words, without the mark. Split out so the BENCH
/// can say what stands in a set's way in exactly the voice the deed
/// would use if you tried it — one text, two readers.
fn refuse_text(world: &World, refusal: &ActionRefused, style: &Style) -> String {
    let text = match refusal {
        // K4: the emitter now refuses BEFORE writing, so these are
        // reachable from every caller rather than only from the judge.
        ActionRefused::WorkRefused => {
            "that payload is not a proof — the work did not verify".to_string()
        }
        ActionRefused::MintExceedsWork { bound, claimed } => format!(
            "the work earns {} — {claimed} was claimed",
            style.strong(&bound.to_string())
        ),
        ActionRefused::ConveyExceedsMonetary { asked, monetary } => format!(
            "{asked} outruns the parcel's monetary portion ({monetary}) — the free \
             portion stays home; convey less, or re-survey after the estate grows"
        ),
        ActionRefused::StaleAppraisalCited { .. } => "that appraisal is STALE — the \
             world moved past its stamp; re-appraise, then counter on the fresh \
             reading (staleness triggers renegotiation; it never blocks acceptance)"
            .to_string(),
        ActionRefused::RingReachExceeded { held, reach } => format!(
            "this star's field anchors {} ring(s) and {held} already stand — site on \
             an anchored ring (slots within a ring stay open), or this system has \
             reached its magnetism",
            style.strong(&reach.to_string())
        ),
        // `OffSlot` is DEAD (29_CONSTRUCTION C1): an off-slot angle
        // cannot be constructed, so no verb can be handed one to refuse.
        // The client parses user input through `Slot::from_angle` and
        // reports the fault at ITS door instead.
        ActionRefused::OrbitOccupied { orb, slot } => format!(
            "slot {slot} on that ring is held by {} — two orbs cannot share a place",
            style.strong(&orb.to_string())
        ),        // BOOK-1 (19_LENS §16) — each refusal names what stands rather
        // than a code, because the reason IS the rule here.
        ActionRefused::BookAlreadyMinted { name } => format!(
            "{} is already kept — a book is its normalized name, so re-declaring it is not a second book",
            style.strong(name)
        ),
        // The empty and charter names die at the `BookName` door
        // (29_CONSTRUCTION) — the client reports them at ITS door via
        // `book_name_fault`.
        ActionRefused::CharterBookNotArchivable { name } => format!(
            "{} is a charter book and is closeable by nobody — otherwise it could be closed by whoever it convicts",
            style.strong(name)
        ),
        ActionRefused::NoSuchBook => "no such book on this orb".to_string(),
        ActionRefused::BookClosedToWriting { name } => format!(
            "{} is archived — closed to writing, and still fully readable",
            style.strong(name)
        ),
        ActionRefused::BookAlreadyArchived { name } => {
            format!("{} is already closed", style.strong(name))
        }
        ActionRefused::NoSuchWork { name } => format!(
            "{} is not a chartered work — the literature is {}; a line invents ACCOUNTS ({}), never the works it reaches for",
            style.strong(name),
            style.strong(&cfe::literature::WORKS.join(" · ")),
            style.strong("book mint")
        ),
        ActionRefused::NotLiterate => format!(
            "this line has not learned to write — keep a book of your own first ({} then {})",
            style.strong("book mint <name>"),
            style.strong("book post <kept> <lost> <name>")
        ),
        ActionRefused::WorkAlreadyAspired { name } => {
            format!("{} is already reached for", style.strong(name))
        }
        ActionRefused::ZoneKindUnknown { code } => format!(
            "no building in this world's vocabulary carries code {} — {} lists what stands, and \
             {} is how a new one enters",
            style.strong(&code.to_string()),
            style.strong("buildings"),
            style.strong("t0 mint zone …")
        ),
        ActionRefused::ZoneKindUnbuildable { code } => format!(
            "building {} is a WORD without a workshop — its binding names no base to modulate, so \
             nothing can be raised from it. Mint it again naming a base ({}), or bind a channel \
             that belongs to one family",
            style.strong(&code.to_string()),
            style.strong("t0 mint zone <sigil> <word> on <base> …")
        ),
        ActionRefused::BaseOffAxis { axis } => format!(
            "axis {axis} has no bases — only a minted BUILDING modulates a standing kind, so a \
             base belongs to a {} mint alone",
            style.strong("zone")
        ),
        ActionRefused::BaseUnknown { code } => format!(
            "{} names no standing zone kind — a minted kind modulates one the table already \
             compiles (1–{}), never another mint",
            style.strong(&code.to_string()),
            cfe::dop::ZONE_KINDS
        ),
        ActionRefused::TreatyWithSelf => String::from(
            "a world does not treat with itself — a pact reads two parties",
        ),
        ActionRefused::TreatyAlreadyStands { partner, kind } => format!(
            "a {} pact already stands with {} — a second is not stronger, it is ambiguous \
             (the pair factors multiply). {} it first to re-term it",
            style.strong(
                cfe::treaty::TreatyKind::from_wire(*kind).map(|k| k.word()).unwrap_or("pact")
            ),
            style.strong(partner),
            style.strong("renounce")
        ),
        ActionRefused::NoSuchTreaty { deal } => format!(
            "no pact on this book carries deal {} with that world — {} lists what stands, and a \
             world may only ever end its OWN leg",
            style.strong(&deal.to_string()),
            style.strong("treaties")
        ),
        ActionRefused::SwornToDefend { partner } => format!(
            "you are SWORN TO DEFEND {} — a defense pact spends the option to attack. {} the \
             defense first if that is what is meant; the chronicle will show you did",
            style.strong(partner),
            style.strong("renounce")
        ),
        ActionRefused::AtWar { partner } => format!(
            "the crossing is WALLED — you are at war with {}. A pair at war exchanges nothing; \
             seal a {} to open the way again",
            style.strong(partner),
            style.strong("peace")
        ),
        ActionRefused::ElementNotAllowed { element, partner } => format!(
            "element {} is NOT ON THE LIST — an allow-list stands with {} and only enumerated \
             matter crosses a fenced pair. Annex the id with {} or trade what the pact names",
            style.strong(&hex(element)[..8]),
            style.strong(partner),
            style.strong("treaty elements")
        ),
        ActionRefused::AllowListEmpty => format!(
            "an allow-list that names NOTHING would wall all matter — barring everything is \
             war's job, not a trade annex's. Enumerate at least one {}",
            style.strong("element id")
        ),
        ActionRefused::AllowListNeedsTrade { deal } => format!(
            "no live {} carries deal {} — an allow-list annexes a standing trade pact; seal \
             the pact first, then fence it",
            style.strong("trade treaty"),
            style.strong(&deal.to_string())
        ),
        ActionRefused::SolverWordKnown { orbit } => format!(
            "the word-orbit {} already stands in this line's library — a re-spelling never \
             re-mints",
            style.strong(&hex(orbit)[..8])
        ),
        ActionRefused::SolverWordEmpty => {
            "a word with no expansion spells nothing — nothing enters the library unearned"
                .to_string()
        }
        ActionRefused::SolverWordUnconfirmed { confirms } => format!(
            "{confirms} confirming close(s) — the promote law asks {} distinct tasks before a \
             word mints",
            style.strong("three")
        ),
        ActionRefused::KnowledgeKnown { signature } => format!(
            "the shared axiom {} already stands in this line's chronicle — convergence is \
             recorded once, never re-recorded",
            style.strong(&hex(signature)[..8])
        ),
        ActionRefused::ConsilienceAlone { mass } => format!(
            "a consilience is agreement BETWEEN lines — a mass of {} is one line agreeing with \
             itself, which is no measurement",
            style.strong(&mass.to_string())
        ),
        ActionRefused::MatterDoesNotBond => format!(
            "this matter does not BOND — a set compounds when every member shares the geometry \
             and the phase, the count equals the geometry's arity, and the binds saturate. \
             {} shows every set of what you hold that would",
            style.strong("bench")
        ),
        ActionRefused::InstrumentBookClosed => {
            "instrument book closed — seat Exchange or bank STAMP/cargo evidence".into()
        }
        ActionRefused::InstrumentEmpty => "empty notional, margin, or identical sides".into(),
        ActionRefused::InstrumentUndercollateralized => {
            "note collateral must cover at least half face".into()
        }
        ActionRefused::CannotAffordStamp { need, have } => format!(
            "need {} STAMP, have {}",
            amount(need),
            amount(have)
        ),
        ActionRefused::NoSuchNote { note_id } => format!("no open note {note_id}"),
        ActionRefused::NoteImmature { note_id, maturity } => {
            format!("note {note_id} matures at t{maturity}")
        }
        ActionRefused::NoSuchFuture { fut_id } => format!("no open future {fut_id}"),
        ActionRefused::FutureImmature { fut_id, expiry } => {
            format!("future {fut_id} expires at t{expiry}")
        }
        ActionRefused::SharesBookClosed => {
            "share book closed — need zones+production or Exchange".into()
        }
        ActionRefused::NoSuchShare { share_id } => format!("no open share {share_id}"),
        ActionRefused::ShareOverTransfer => "transfer exceeds share units".into(),
        ActionRefused::PerpsBookClosed => {
            "perp book closed — need cargo evidence or Exchange".into()
        }
        ActionRefused::NoSuchPerp { perp_id } => format!("no open perp {perp_id}"),
        ActionRefused::PerpMarginBreach { perp_id } => {
            format!("perp {perp_id} margin breach — liquidate or fund")
        }
        ActionRefused::EscortRoleRequired { hull_id } => {
            format!("hull {hull_id} is not an escort — `fleet mint escort`")
        }
        ActionRefused::MarketCircuitOpen { resume_at } => {
            format!(
                "market circuit open until t{resume_at} — `market resume` lifts the breaker"
            )
        }
        ActionRefused::ClearSealUnmet {
            resource,
            depth,
            need,
        } => format!(
            "{resource:?} last-clear seal unmet ({depth}/{need} honest clears) — `market clear {:?} <p>/<q>`",
            resource
        ),
        ActionRefused::ResumeAuditUnmet { depth, need } => format!(
            "early market resume refused — crystal clear depth {depth}/{need}; keep clearing or wait for breaker"
        ),
        ActionRefused::GenesisUnsealed => {
            "seal genesis first — posture reorientation needs anatomy (metabolism) sealed".into()
        }
        ActionRefused::DetectionThin { need, have } => format!(
            "detection grade {have} < {need} — forge an instrument (`forge <grade>`) or lift the veil before thin survey"
        ),
        ActionRefused::CorridorFogDepth { deal } => format!(
            "corridor deal {deal} is fogged (volumetric) — prospect a stratum before marking risk"
        ),
        ActionRefused::InstrumentSpoof { orb, grade } => format!(
            "instrument grade {grade} already forged on orb {orb} — anti-spoof refuses re-forge (forge another condition)"
        ),
        ActionRefused::SpyProbeRequired { orb } => format!(
            "spy probe orb {orb} first — exfil/sabotage need a scout mark"
        ),
        ActionRefused::IntelAlreadyHeld => {
            "that tech hash is already in the intel stash — no free mint either way".into()
        }
        ActionRefused::NoInstrumentToSabotage { orb } => {
            format!("no instrument on orb {orb} to sabotage")
        }
        ActionRefused::RailUnknown { rail } => format!("unknown rail code {rail}"),
        ActionRefused::RailAtCeiling { deal, rail } => {
            format!("deal {deal} already at rail ceiling (R{rail})")
        }
        ActionRefused::DeepRailGated { rail, feature } => format!(
            "R{rail} promotion is optional — enable cargo feature `{feature}` (no R1–R2 path dep)"
        ),
        ActionRefused::AnchorRequired { rail } => format!(
            "R{rail} exit needs T9 Anchor seated — `seat anchor` first"
        ),
        ActionRefused::AnchorProofInvalid { reason } => format!(
            "Anchor rejected proof (reason {reason}) — bad strand/isthmus/datum frame"
        ),
        ActionRefused::ExchangeRequired => {
            "T6 Exchange must sit before instruments list — `seat exchange` (O6.3 MM gate)".into()
        }
        ActionRefused::AssayRequired => {
            "T7 Assay must sit before consilience challenge — `seat assay`".into()
        }
        ActionRefused::VaultRequired => {
            "T8 Vault must sit before custody — `seat vault` then `vault deposit`".into()
        }
        ActionRefused::VaultShort { need, have } => format!(
            "vault short — need {} custody, hold {}",
            amount(need),
            amount(have)
        ),
        ActionRefused::VaultMarginHeld { free, need } => format!(
            "vault free {} (after margin reserve) < {} — close instruments or deposit more",
            amount(free),
            amount(need)
        ),
        ActionRefused::NotIsomorphic => {
            "gems are not isomorphic — need same shell and phase (bench science shows families)"
                .into()
        }
        ActionRefused::ChemAlreadyCataloged { gem } => format!(
            "gem {} already in the chemistry catalog",
            &hex(gem)[..8]
        ),
        ActionRefused::NoSuchGem { gem } => format!(
            "no banked gem {} — compound or iso-union first",
            &hex(gem)[..8]
        ),
        ActionRefused::ScienceSeatRequired {
            discipline,
            need_rank,
            have_rank,
        } => format!(
            "science seat required: discipline {discipline} needs rank {need_rank} (have {have_rank}) — `bench science seat <discipline>`"
        ),
        ActionRefused::ScienceAlreadySeated { discipline, rank } => format!(
            "science discipline {discipline} already at max rank {rank}"
        ),
        ActionRefused::EconomicModeBlocked { mode, kind, reason } => format!(
            "taxon economy blocks zone kind {kind} under mode {mode} (reason {reason}) — regenerative lines on hostile worlds cannot over-extract; see docs/37_IMPERIUM_TAXON.md"
        ),
        ActionRefused::OffPlanetResourceRequired { kind } => format!(
            "advanced tech needs off-planet resourcing (kind {kind}) — cross the void, charter an element route, or hold a ROUTE-1 deal before synth/clone/skin R2; not payable in STAMP alone"
        ),
        ActionRefused::ApExhausted {
            kind,
            need,
            have,
            capacity,
        } => format!(
            "AP exhausted for {kind} (need {need}, have {have} of {capacity} this year-voice) — wait the year boundary or raise habitats; shared across build·prospect·research·quote"
        ),
        ActionRefused::BalanceEntropyDominates {
            balance,
            syntropy,
            entropy,
        } => format!(
            "entropy dominates order (balance {} = S {} / (S+E), E {}) — build a regenerator or encase heat before refine/compound/hull mint",
            amount(balance),
            amount(syntropy),
            amount(entropy)
        ),
        ActionRefused::NoShipYard { orb } => format!(
            "no shipyard stands on orb {orb} — build one (`zone <tile> yard`) before salvaging or minting hulls"
        ),
        ActionRefused::CannotAffordScrap { need, have } => format!(
            "need {} scrap, hold {} — salvage iron at the yard first",
            amount(need),
            amount(have)
        ),
        ActionRefused::NoSuchHull { hull_id } => {
            format!("no hull {hull_id} — `fleet` lists what you hold")
        }
        ActionRefused::HullCapacityShort { need, have } => format!(
            "haul needs {} hull capacity, free {} — mint/assign haulers (`fleet mint hauler`)",
            amount(need),
            amount(have)
        ),
        ActionRefused::HullRoleForbidden { role } => {
            format!("this line's metabolism does not mint hull role {role}")
        }
        ActionRefused::RouteUnknown { deal } => {
            format!("no route deal {deal} on this book — charter first")
        }
        ActionRefused::NoRefineryStanding { orb } => format!(
            "orb {} has no element-refinery — raise one (DEEPENING first) before refining",
            style.strong(&orb.to_string())
        ),
        ActionRefused::ElementNotLocal { element } => format!(
            "{} lies on a LINE this system does not carry — deep matter does not occur under a \
             shallow star. Another world holds it; {} is how it gets here",
            style.strong(&hex(element)[..8]),
            style.strong("trade")
        ),
        ActionRefused::CivReachShort { need, have } => format!(
            "this people only works LINE depth {} (bronze surface is 2) — asked {}; \
             crystallize sky tech (base-7 conditions) to deepen the mine, never before",
            style.strong(&have.to_string()),
            style.strong(&need.to_string())
        ),
        ActionRefused::PressureUnreached { element } => format!(
            "{} needs more PRESSURE than this orb has opened — {} deeper strata to bring it to bear",
            style.strong(&hex(element)[..8]),
            style.strong("prospect")
        ),
        ActionRefused::NotWorkableHere { element } => format!(
            "{} is not workable at this orb's temperature — matter has a phase and a place has a \
             heat. The same set bonds on a world in another band",
            style.strong(&hex(element)[..8])
        ),
        ActionRefused::CannotAffordElement { element, need, have } => format!(
            "not enough {} — the bond spends {} and you hold {}; {} the vein for more",
            style.strong(&hex(element)[..8]),
            style.strong(&amount(need)),
            style.strong(&amount(have)),
            style.strong("assay")
        ),
        ActionRefused::WorkNotAspired { name } => format!(
            "{} was never reached for — {} before you claim it",
            style.strong(name),
            style.strong("aspire")
        ),
        ActionRefused::WorkAlreadyClaimed { name } => format!(
            "{} is already claimed — evidence is cited once and never swapped after",
            style.strong(name)
        ),
        ActionRefused::ClaimWithoutWitness { name } => format!(
            "a claim to {} needs CHAIN RECEIPTS — without them it is a boast",
            style.strong(name)
        ),
        ActionRefused::WitnessNotInChronicle { digest } => format!(
            "receipt {} is not in this world's chronicle — the law verifies the citation",
            style.strong(&hex(digest)[..8])
        ),

        ActionRefused::WorldQuarantined { orb } => format!(
            "orb {orb} is under BIO QUARANTINE — its Agent Door is sealed to inbound trade until the bio-pressure drops below the threshold; the crossing stays open",
        ),
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
        ActionRefused::LineStillLiving { orbs, famished } => format!(
            "this line still lives ({famished} of {orbs} planets famished) — succession is the derelict door, not a reset: only a full cascade admits a successor"
        ),
        ActionRefused::GenesisAlreadySealed => format!(
            "this world's anatomy is sealed (one genesis) — reorient doctrine with {}; {} shows posture",
            style.strong("posture set"),
            style.strong("defend status")
        ),
        ActionRefused::UnknownAxisValue { axis, code } => format!(
            "axis {axis} has no value {code} — the vocabulary is closed: dispositions {} · metabolisms {} · perceptions {}",
            style.strong("expansionist|empiricist|epicurean"),
            style.strong("mineral|biotic|plasmic"),
            style.strong("planar|volumetric|harmonic|umbral")
        ),
        ActionRefused::StratumProspected { orb, stratum } => format!(
            "stratum {stratum} on orb {orb} is already read — one stratum, one reading; try the next: {}",
            style.strong("prospect <stratum>")
        ),
        ActionRefused::NoSuchElement { .. } => format!(
            "no opened vein carries that identity here — {} first, then {} what it names",
            style.strong("prospect <stratum>"),
            style.strong("assay <element-prefix>")
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
        ActionRefused::CivilizationCollapsed => {
            "the line has collapsed — the chronicle is closed, there is no further action to take".to_string()
        }
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
        ActionRefused::TheWardHolds => format!(
            "the elder's reverse shield stands — no aperture reaches a warded star; solve the gate ({})",
            style.strong("10 §3e")
        ),
        ActionRefused::WardOutOfOrder =>
            "the elder deeds run in order: ward once, lift once, lift only what is warded".to_string(),
        ActionRefused::DepthBelowCeiling { depth, ceiling } => format!(
            "the depth gate: {} recorded solves stand, {} required — the ceiling is earned",
            style.strong(&depth.to_string()),
            style.strong(&ceiling.to_string())
        ),
        ActionRefused::VeilOutOfOrder =>
            "one veil per star — lift before drawing anew (nothing stacks)".to_string(),
        ActionRefused::PracticeUnknown { practice } => format!(
            "no discipline {} stands in the FOLD — the seed five (1 Austerity · 2 Showpiece · 3 Restraint · 4 Hoard · 5 Overdrive) plus whatever a progenitor has minted ({})",
            style.strong(&practice.to_string()),
            style.dim("t0 mint practice <sigil> <word> <channel>:<p>/<q> ...")
        ),
        ActionRefused::RouteKnown { deal } => format!(
            "deal {} already charters a lane on this book — one charter per deal",
            style.strong(&deal.to_string())
        ),
        // BadEta is DEAD (29_CONSTRUCTION C1): η is a `Unit` and cannot
        // leave [0,1), so no verb can be handed one to refuse.
        ActionRefused::EconomicallyWalled { melt } => format!(
            "the void takes {} of every unit at this distance — this pair is economically WALLED; charter a route or trade nearer",
            style.strong(&format!("{}/{}", melt.numer(), melt.denom()))
        ),
        ActionRefused::ConditionUncrystallized { condition } => format!(
            "condition {} has not crystallized here — the understanding comes before the instrument",
            style.strong(&condition.to_string())
        ),
        ActionRefused::ChainIncomplete { needs } => format!(
            "the converter chain does not stand — this forge needs a {}",
            style.strong(&format!("{needs:?}"))
        ),
        ActionRefused::PracticeAlreadyAdopted { practice } => format!(
            "discipline {} already binds this line — one adoption per star",
            style.strong(&practice.to_string())
        ),
        ActionRefused::PracticeNotHeld { practice } => format!(
            "this line does not hold discipline {} — nothing to release",
            style.strong(&practice.to_string())
        ),
        ActionRefused::ReleaseUnearned { practice } => format!(
            "release is a REDISCOVERY — no new doctrine has landed since discipline {} was adopted; earn the lesson first",
            style.strong(&practice.to_string())
        ),
        ActionRefused::OverdriveUntempered { hardships } => format!(
            "the Overdrive takes only the tempered — {} hardship receipts stand, eight are required",
            style.strong(&hardships.to_string())
        ),
        ActionRefused::TaxonUnknown { class, clade } => format!(
            "no such taxon — class {} · clade {} stands outside the ratified vocabularies (15 §2–§3); the vocabulary grows by amendment, never by deed",
            style.strong(&class.to_string()),
            style.strong(&clade.to_string())
        ),
        ActionRefused::TaxonAlreadySealed =>
            "the taxon is already sealed — one seal per star, identity is forever (15 §4)".to_string(),
        ActionRefused::TerrainAlreadyCharted => format!(
            "the terrain is already charted — one seed per star, forever (02 §2.5); {} reads the map",
            style.strong("terrain")
        ),
        // THE COVENANT IS A T3 INSTRUMENT (operator ruling 2026-08-05).
        // The refusal names the seat that bars it, so the way down is
        // legible: the instrument opens to the common band.
        ActionRefused::CovenantAboveTheCommon { line, station } => format!(
            "{} holds the {} seat — a covenant is a COMMON-BAND instrument. An office with chartered authority decrees, delegates or petitions the council; it does not post collateral against its own word",
            style.strong(line),
            style.strong(match station {
                1 => "progenitor",
                2 => "watcher",
                3 => "observer",
                _ => "sovereign",
            })
        ),
        // 28_VOCABULARY §5-§7 — the three mint gates. Each names the way
        // up as well as the wall: the article gives before it withholds.
        // The unknown-axis arm died at the `Axis` door (29_CONSTRUCTION);
        // this verb's own parse carries that guidance now.
        ActionRefused::MintWithoutStanding { line } => format!(
            "{} holds no progenitor seat — minting is a TIER 0 power ({}), the office that makes the board others play on",
            style.strong(line),
            style.dim("12_UNION ruling 4")
        ),
        ActionRefused::MintNotNovel { like } => format!(
            "that mechanism already stands as code {} — a different word and a different mark are not novelty; give it a trade nothing else makes",
            style.strong(&like.to_string())
        ),
        ActionRefused::MintNameTaken { word } => format!(
            "the mark or the word {} is already taken on that axis — a mark that names two things defines neither",
            style.strong(word)
        ),
        ActionRefused::NoStarCommissioned => format!(
            "no sun yet — commission one first: {} (O B A F G K M · bd wd rg ns wr lwh)",
            style.strong("star <class>")
        ),
        ActionRefused::AlreadySited { orb } => format!(
            "orb {} is already sited — an orbit does not move",
            style.strong(&orb.to_string())
        ),
        // BadSiting is DEAD (29_CONSTRUCTION C1): albedo is a `Unit`
        // and the radius a `Positive` — the client's parse doors report
        // an out-of-range argument before any verb sees it.
        // BadTransfer is DEAD (29_CONSTRUCTION C1): Between, Positive
        // and Window prove the shapes at the doors; the client reports
        // parse faults before any verb runs.
        ActionRefused::NodeAlreadyHeld { condition } => format!(
            "node {} is already held on that side — a technology crystallizes once",
            style.strong(&condition.to_string())
        ),
        ActionRefused::NodeUnlicensable { condition } => format!(
            "condition {} is not licensable — only the crystallized tier and the crown license down (derived from the vocabulary, never a range)",
            style.strong(&condition.to_string())
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
        ActionRefused::BaseSevenUnfinished => {
            "the crown gifts only onto a FINISHED base-7 stack — the solar leap and all five crystallized conditions, earned first".into()
        }
        ActionRefused::TerraformUnearned => format!(
            "the orbit moves only under the crown technology — terraforming is gifted from tier 0, never taken; see {}",
            style.strong("civ seats")
        ),
        ActionRefused::PlaneUnearned { grade, mints } => format!(
            "opening grade {} is EARNED — the fiber has grown {} mint(s); crystallize, diverge, or take a licensed leap first",
            style.strong(&grade.to_string()),
            style.strong(&mints.to_string())
        ),
        ActionRefused::OfferKnown { offer } => format!(
            "offer {} already rests on the board — a fresh intent needs a fresh id",
            style.strong(&offer.to_string())
        ),
        ActionRefused::NoSuchOffer { offer } => format!(
            "no offer {} rests on this book — {} shows the board",
            style.strong(&offer.to_string()),
            style.strong("offers")
        ),
        ActionRefused::NoCrossing => {
            "no crossing — the two offers do not meet (wrong resources, or the prices never cross); an unmatched offer rests".into()
        }
        ActionRefused::CouncilKnown { council } => format!(
            "council {} already seals on this book — a new council needs a fresh id",
            style.strong(&council.to_string())
        ),
        ActionRefused::ElevationBarred { settled, coherent, crossed, aligned } => {
            let gates: Vec<&str> = [
                (!settled, "the thermal books stand unsettled"),
                (!coherent, "no loop closed within the year voice"),
                (!crossed, "this world has not crossed"),
                (!aligned, "the practiced books stand unaligned — resolve the discipline's friction"),
            ]
            .iter()
            .filter(|(barred, _)| *barred)
            .map(|(_, cure)| *cure)
            .collect();
            format!(
                "the climb is barred — {}; each gate is escapable: settle the books, close a loop this year, cross the void",
                gates.join("; ")
            )
        }
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
        ActionRefused::ForwardAlreadySealed => format!(
            "an open forward already stands on that lane — a vitrified pact cannot be cancelled, only fulfilled or slashed ({} lists them)",
            style.strong("forwards")
        ),
    };
    text
}

fn map(world: &World, style: &Style, current: u64) {
    let Some(orb) = world.orb_by_id(current) else {
        println!("{}", style.dim("(no colony yet — the map begins at 'found <name>')"));
        return;
    };
    println!(
        "{}  {}",
        style.head("MAP"),
        style.dim("E extractor · H habitat · S smelter · Y synthesizer · C encasement · K cryo-array · R resonator · lowercase = dormant · · empty")
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
                    ZoneKind::Resonator => "r",
                    ZoneKind::Regenerator => "g",
                    ZoneKind::ElementRefinery => "f",
                    ZoneKind::ShipYard => "p",
                    ZoneKind::Granary => "a",
                    ZoneKind::GovernmentOffice => "o",
                    ZoneKind::WelfareHub => "w",
                    ZoneKind::Minted(_) => "m",
                }),
                Some(zone) => match zone.kind {
                    ZoneKind::Extractor => style.ok("E"),
                    ZoneKind::Habitat => style.head("H"),
                    ZoneKind::Smelter => style.warn("S"),
                    ZoneKind::Synthesizer => style.prompt("Y"),
                    ZoneKind::Encasement => style.warn("C"),
                    ZoneKind::CryoArray => style.head("K"),
                    ZoneKind::Resonator => style.prompt("R"),
                    ZoneKind::Regenerator => style.ok("G"),
                    ZoneKind::ElementRefinery => style.warn("F"),
                    ZoneKind::ShipYard => style.route("P"),
                    ZoneKind::Granary => style.ok("A"),
                    ZoneKind::GovernmentOffice => style.head("O"),
                    ZoneKind::WelfareHub => style.ok("W"),
                    ZoneKind::Minted(_) => style.strong("M"),
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
            if let (Some(genesis), Some(scored)) = (world.genesis(), world.scored_progress()) {
                println!(
                    "    ✶ line {} ({}·{}·{}) — scored progress {}",
                    style.strong(&genesis.civilization),
                    disposition_name(genesis.disposition),
                    metabolism_name(genesis.metabolism),
                    perception_name(genesis.perception),
                    style.strong(&amount(&scored))
                );
            }
            for resource in &orb.famine {
                println!(
                    "    {} {} famine — its consumers are dormant ({} restocks, {} cuts the drain)",
                    style.bad("☠"),
                    style.bad(resource_name(*resource)),
                    style.strong("collect"),
                    style.strong("raze")
                );
            }
            zone_lines(
                orb,
                style,
                world.genesis().map(|held| held.metabolism).unwrap_or(2),
            );
            banked_lines(world, style, current);
            accrued_lines(world, style, current);
            let s = orb.syntropy_contained.clone();
            let e = orb.entropy_vented.clone();
            let scrap = amount(&orb.scrap_amount());
            println!(
                "  {} S {} · E {} · scrap {} · hulls {} · {}",
                style.dim("balance"),
                style.ok(&amount(&s)),
                style.bad(&amount(&e)),
                style.cargo(&scrap),
                style.strong(&world.hulls().len().to_string()),
                match world.thermal_balance_of(current) {
                    None => style.dim("unbooked"),
                    Some(b) if b >= cfe::world::World::balance_orderly_floor() => {
                        style.ok(&format!("{} orderly", amount(&b)))
                    }
                    Some(b) => style.bad(&format!("{} strained", amount(&b))),
                }
            );
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

fn zone_lines(orb: &ColonyOrb, style: &Style, metabolism: u8) {
    for zone in &orb.zones {
        // G2: the panel renders the line's OWN map (04 §4) — the
        // sealed metabolism's rations, never the v1 table verbatim.
        let (yields, rate) = cfe::dop::zone_yield(zone.kind, metabolism);
        let drains = cfe::dop::zone_upkeep(zone.kind, metabolism)
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
    let voice = scribe::journal::Voice {
        civilization: orb.name.clone(),
        disposition: "empiricist".into(),
        metabolism: "biotic".into(),
        perception: "planar".into(),
    };
    let entries = scribe::journal::journal(&orb.name, world, &voice, scribe::journal::YEAR_TICKS);
    if entries.is_empty() {
        println!(
            "{} no completed year yet — a year is {} ticks; the clock stands at {}",
            style.dim("…"),
            style.strong(&scribe::journal::YEAR_TICKS.to_string()),
            style.strong(&world.now().to_string())
        );
        return;
    }
    for entry in entries.iter().rev() {
        let headline = match entry.alert {
            scribe::journal::Alert::Warning => style.bad(&entry.headline),
            scribe::journal::Alert::Milestone => style.head(&entry.headline),
            scribe::journal::Alert::Normal => style.ok(&entry.headline),
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

/// THE WORKBENCH (43_DISTRIBUTION §7) — inspect every bench on the current
/// orb under one uniform render: its components, its gauges, its unmet
/// requirements. The depot bench surfaces the whole provisioning society.
fn workbench_panel(world: &World, style: &Style, current: u64) {
    use cfe::world::workbench::BenchKind;
    for kind in [BenchKind::Depot, BenchKind::Refinery, BenchKind::ShipYard, BenchKind::Lab, BenchKind::Civic] {
        let Some(view) = world.workbench(current, kind) else {
            continue;
        };
        println!("{}", style.head(&format!("⚙ {} bench", kind.word())));
        for component in &view.components {
            let mark = if component.active { style.ok("●") } else { style.bad("○") };
            println!("  {mark} {}", component.label);
        }
        for (label, value) in &view.gauges {
            println!("  {:>18} {}", style.dim(label), style.strong(value));
        }
        for alert in &view.alerts {
            println!("  {} {}", style.warn("!"), alert);
        }
        println!();
    }
}

fn chronicle(world: &World, style: &Style) {
    for receipt in world.chronicle().receipts() {
        let described = match &receipt.event {
            cfe::dop::ClosureEvent::ColonyFounded { name, .. } => {
                format!("⚑ colony {} founded", style.strong(name))
            }
            cfe::dop::ClosureEvent::ParameterChartered { key, value } => format!(
                "⚖ parameter {} CHARTERED to {} — the rule follows the chain",
                style.strong(
                    &cfe::parameter::Parameter::from_code(*key)
                        .map(|held| format!("{held:?}"))
                        .unwrap_or_else(|| format!("key {key}"))
                ),
                style.strong(&amount(value))
            ),
            cfe::dop::ClosureEvent::LineMinted { seed, ceilings } => {
                // The receipt declares the ceilings it drew under; the
                // sealed old form (None) re-derives compiled.
                let draw = match ceilings {
                    Some((class, clade)) => cfe::dop::line_synthesis_at(seed, *class, *clade),
                    None => cfe::dop::line_synthesis(seed),
                };
                // #45: a MINTED class resolves its word from the FOLD
                // at the render seam — the pure draw stays fold-blind.
                let species = match draw.class > cfe::dop::class_ceiling() {
                    true => cfe::vocabulary::fold(
                        world.chronicle(),
                        cfe::vocabulary::Axis::Class,
                    )
                    .into_iter()
                    .find(|entry| entry.code == draw.class)
                    .map(|entry| {
                        let voice = draw.species.split(' ').next().unwrap_or("").to_string();
                        format!("{voice} {}", entry.word)
                    })
                    .unwrap_or_else(|| draw.species.clone()),
                    false => draw.species.clone(),
                };
                format!(
                    "✶ line MINTED — {} ({}), \"{}\"",
                    style.strong(&draw.name),
                    species,
                    draw.motto
                )
            }
            cfe::dop::ClosureEvent::ParcelConveyed { from_orb, to_orb, amount: posted, .. } => {
                format!("✉ parcel CONVEYED — {} posted orb {from_orb} → orb {to_orb}", amount(posted))
            }
            cfe::dop::ClosureEvent::OfferCountered { offer, want_amount, .. } => {
                format!("⇄ offer {offer} COUNTERED — ask now {}", amount(want_amount))
            }
            cfe::dop::ClosureEvent::AppraisalStruck { value, .. } => {
                format!("⚖ APPRAISED — {} in the commensurate unit", amount(value))
            }
            cfe::dop::ClosureEvent::EstateSurveyed { covered, cost, .. } => {
                format!(
                    "⌖ estate SURVEYED — {} orb(s), one witness each, {} iron metered",
                    style.strong(&covered.len().to_string()),
                    amount(cost)
                )
            }
            cfe::dop::ClosureEvent::VocabularyMinted { axis, code, sigil, word, binding, base, ration } => {
                format!(
                    "✎ VOCABULARY MINTED — axis {axis}, code {}, {} {} [{}]{}{}",
                    style.strong(&code.to_string()),
                    sigil,
                    style.strong(word),
                    binding
                        .iter()
                        .map(|(channel, factor)| format!("{channel}:{factor}"))
                        .collect::<Vec<_>>()
                        .join(" "),
                    match base.and_then(cfe::dop::ZoneKind::from_wire) {
                        Some(kind) => format!(" on {}", style.strong(cfe::dop::zone_word(kind))),
                        None => String::new(),
                    },
                    match ration {
                        // G2: a stone-eating line — the element it farms.
                        Some(id) => format!(
                            " eats {}",
                            style.strong(&id[..4].iter().map(|b| format!("{b:02x}")).collect::<String>())
                        ),
                        None => String::new(),
                    }
                )
            }
            cfe::dop::ClosureEvent::SovereignUnionSealed { progenitor, sovereign, term_stamps } => {
                format!(
                    "⚭ SOVEREIGN UNION — {progenitor:?} + {sovereign:?}, reviewed at {} stamps (a horizon, never an expiry)",
                    style.strong(&term_stamps.to_string())
                )
                }
                // LIT-1 — the literature deeds.
            cfe::dop::ClosureEvent::WorkAspired { name, .. } => {
                format!("✍ aspires to {}", style.strong(name))
            }
            cfe::dop::ClosureEvent::WorkClaimed { witnesses, .. } => format!(
                "✍ work CLAIMED on {} chain receipt(s)",
                style.strong(&witnesses.len().to_string())
            ),
            // BOOK-1 — the open registry's three deeds.
            cfe::dop::ClosureEvent::BookMinted { name, .. } => {
                format!("▤ book {} minted", style.strong(name))
            }
            cfe::dop::ClosureEvent::BookPosted { kept, lost, .. } => format!(
                "▤ posted {} kept / {} lost",
                style.strong(&amount(kept)),
                amount(lost)
            ),
            cfe::dop::ClosureEvent::BookArchived { .. } => {
                "▤ book closed to writing (still readable)".to_string()

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
            cfe::dop::ClosureEvent::DiscoveryMinted {
                resource, amount: value, work, ..
            } => format!(
                "◈ minted {} {} against verified work ({} proof bytes)",
                style.strong(&amount(value)),
                resource_name(*resource),
                work.len()
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
            cfe::dop::ClosureEvent::GalaxyObserved { observer, name, .. } => format!(
                "✧ galaxy {} observed into the chart by {}",
                style.strong(name),
                style.dim(observer)
            ),
            cfe::dop::ClosureEvent::IncursionStruck { era, world, line } => format!(
                "⟁ INCURSION — {} of {} wakes as {}",
                style.strong(world),
                style.dim(era),
                style.strong(line)
            ),
            cfe::dop::ClosureEvent::StationSeated { line, station } => format!(
                "◈ station SEATED — {} takes office {} (the seat is now on the chain)",
                style.strong(line),
                style.strong(&station.to_string())
            ),
            cfe::dop::ClosureEvent::TaxonSealed { class, clade } => format!(
                "◈ taxon SEALED — class {} · clade {} (identity, forever)",
                style.strong(&class.to_string()),
                style.strong(&clade.to_string())
            ),
            cfe::dop::ClosureEvent::EvolutionAdvanced { stage } => format!(
                "⇑ the line EVOLVES — stage {} (the arc advances)",
                style.strong(&stage.to_string())
            ),
            cfe::dop::ClosureEvent::TitheReceipted { to_seat, resource, amount, .. } => format!(
                "⚑ the founding TITHE fires — {} {:?} paid to the {}",
                style.strong(&cfe::ratio::ratio_str(amount)),
                resource,
                style.strong(if *to_seat == cfe::dop::SEAT_ELDER { "elder" } else { "ancient" })
            ),
            cfe::dop::ClosureEvent::ElderWarded {} => 
                "⛨ the ELDER stands warded — the reverse shield seats".to_string(),
            cfe::dop::ClosureEvent::WardLifted { claimer } => format!(
                "⛨ the ward LIFTS — the depth gate opened by {}",
                style.strong(claimer)
            ),
            cfe::dop::ClosureEvent::VeilDrawn {} =>
                "◐ the VEIL draws — renders cloud, the chronicle never lies".to_string(),
            cfe::dop::ClosureEvent::VeilLifted { starved } => match starved {
                true => "◐ the veil LIFTS — STARVED, the toll went unpaid".to_string(),
                false => "◐ the veil LIFTS — by deed".to_string(),
            },
            cfe::dop::ClosureEvent::PracticeAdopted { practice } => format!(
                "⚖ discipline {} ADOPTED — one benefit, one friction, bound by receipt",
                style.strong(&practice.to_string())
            ),
            cfe::dop::ClosureEvent::PracticeReleased { practice } => format!(
                "⚖ discipline {} RELEASED — the rediscovery earned; the lesson re-emerges",
                style.strong(&practice.to_string())
            ),
            cfe::dop::ClosureEvent::VoidShare { deal, amount, .. } => format!(
                "∅ deal {}: the void took {} in transit — receipted, never silent",
                deal,
                style.strong(&format!("{}/{}", amount.numer(), amount.denom()))
            ),
            cfe::dop::ClosureEvent::RouteChartered { deal, eta } => format!(
                "⇉ lane {} CHARTERED at η {} — the melt bends around the path",
                deal,
                style.strong(&format!("{}/{}", eta.value().numer(), eta.value().denom()))
            ),
            cfe::dop::ClosureEvent::RouteLapsed { deal } => format!(
                "⇢ lane {} LAPSES — raw space takes the path back",
                style.strong(&deal.to_string())
            ),
            cfe::dop::ClosureEvent::InstrumentForged { orb_id, grade, .. } => format!(
                "⚙ orb {} forges its grade-{} INSTRUMENT — the scan horizon grows",
                orb_id,
                style.strong(&grade.to_string())
            ),
            cfe::dop::ClosureEvent::WorldCharted { seed } => format!(
                "▲ terrain charted — seed {:02x}{:02x}{:02x}{:02x}; the map is worth reading",
                seed[0], seed[1], seed[2], seed[3]
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
            cfe::dop::ClosureEvent::PlanetSited { orb_id, albedo, orbit_r, theta: _, psi: _ } => format!(
                "◍ orb {} sited — albedo {} at orbit r {}",
                style.strong(&orb_id.to_string()),
                amount(albedo.value()),
                amount(orbit_r.value())
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
            cfe::dop::ClosureEvent::DemurrageTouched { decays, .. } => format!(
                "◌ the touch settled: {} — one touch, one receipt",
                decays
                    .iter()
                    .map(|(resource, decay)| format!(
                        "{} {}",
                        style.strong(&amount(decay)),
                        resource_name(*resource)
                    ))
                    .collect::<Vec<_>>()
                    .join(" · ")
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
            cfe::dop::ClosureEvent::ForwardSealed {
                obligor_orb, beneficiary_orb, resource, quota, window, collateral,
            } => format!(
                "⛨ forward vitrified: {} {} owed orb {obligor_orb} → orb {beneficiary_orb} within {window} ticks — {} collateral locked in the lattice",
                style.strong(&amount(quota)),
                resource_name(*resource),
                style.strong(&amount(collateral))
            ),
            cfe::dop::ClosureEvent::ForwardReleased {
                obligor_orb, resource, collateral, ..
            } => format!(
                "⛨ forward fulfilled — {} {} collateral returns to orb {obligor_orb}; the word held",
                style.strong(&amount(collateral)),
                resource_name(*resource)
            ),
            cfe::dop::ClosureEvent::ForwardSlashed {
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
            cfe::dop::ClosureEvent::WantStruck { orb_id, kind, reading } => format!(
                "◌ a famine of the MIND — the {} want strikes orb {orb_id} ({} ticks starved); feed it a link or a leap",
                style.bad(match kind { 2 => "resonance", 3 => "novelty", _ => "unnamed" }),
                style.strong(&amount(reading))
            ),
            cfe::dop::ClosureEvent::WantRelieved { orb_id, kind } => format!(
                "◍ the {} want relieved — orb {orb_id}'s mind is fed",
                style.ok(match kind { 2 => "resonance", 3 => "novelty", _ => "unnamed" })
            ),
            cfe::dop::ClosureEvent::ElementWantStruck { orb_id, element, reading } => format!(
                "◌ a famine of MATTER — orb {orb_id} hungers for element-{:02x}{:02x}{:02x}{:02x} \
                 ({} ticks empty); assay the vein or bid on the market",
                element[0], element[1], element[2], element[3],
                style.strong(&amount(reading))
            ),
            cfe::dop::ClosureEvent::ElementWantRelieved { orb_id, element } => format!(
                "◍ matter want relieved — orb {orb_id} banks element-{:02x}{:02x}{:02x}{:02x}",
                element[0], element[1], element[2], element[3]
            ),
            cfe::dop::ClosureEvent::PopulationSettled { orb_id, delta } => {
                let verb = if *delta >= 0 { "grew" } else { "starved" };
                format!("👥 population {verb} — orb {orb_id} by {delta} (the fed/starved logistic)")
            }
            cfe::dop::ClosureEvent::TreasurySettled { orb_id, delta } => {
                let verb = if *delta >= cfe::ratio::ratio(0, 1) { "collected" } else { "spent" };
                format!(
                    "🏛 treasury {verb} — orb {orb_id} by {} (levies in, welfare out)",
                    style.strong(&amount(delta))
                )
            }
            cfe::dop::ClosureEvent::PracticesLapsed { count } => format!(
                "📉 {} practice(s) LAPSED at the generation turnover — the continuity tax to the void",
                style.strong(&count.to_string())
            ),
            cfe::dop::ClosureEvent::PressureEbbed { orb_id } => format!(
                "🌡 bio-pressure EBBED — orb {orb_id} recovers a step toward lifting quarantine"
            ),
            cfe::dop::ClosureEvent::ElementFarmed { orb_id, joule, minted, .. } => format!(
                "⬡ element SYNTHESIZED — orb {orb_id} spent {} joule to mint {} of its food-element",
                style.strong(&amount(joule)),
                style.strong(&amount(minted))
            ),
            cfe::dop::ClosureEvent::TraceConsumed { orb_id, amount: burned, .. } => format!(
                "· trace BURNED — orb {orb_id} spent {} of a trace element maintaining its substrate",
                style.strong(&amount(burned))
            ),
            cfe::dop::ClosureEvent::EnrichmentStruck { orb_id, richness, .. } => format!(
                "✧ SUPERNOVA enrichment — orb {orb_id} seeded a heavy vein at richness {}",
                style.strong(&amount(richness))
            ),
            cfe::dop::ClosureEvent::CivilizationCollapsed { .. } => format!(
                "{} the line has COLLAPSED — every orb stands at zero; the chronicle closes here, there is no win to reach for instead",
                style.bad("☠")
            ),
            cfe::dop::ClosureEvent::OfferPosted { offer, give, give_amount, want, want_amount, .. } => format!(
                "⇋ offer {offer} RESTS — {} {} for {} {}; the market holds intents, never escrows",
                style.strong(&amount(give_amount)),
                cargo_name(give),
                style.strong(&amount(want_amount)),
                cargo_name(want)
            ),
            cfe::dop::ClosureEvent::OfferCleared { offer, deal } => format!(
                "⇌ offer {offer} CLEARED at the crossing — deal {} carries the fill",
                style.strong(&deal.to_string())
            ),
            cfe::dop::ClosureEvent::GaugeShiftedAt { grade, gauge } => format!(
                "♫ the key moves at grade {grade} — a tonal plane plays {}",
                style.strong(&amount(gauge))
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
            cfe::dop::ClosureEvent::ElementApertureSent {
                deal, from_orb, amount: moved, element, ..
            } => format!(
                "⎋ {} of {} crossed out orb {from_orb} → deal {deal} — open matter leaves this book",
                style.strong(&amount(moved)),
                style.strong(&hex(element)[..8]),
            ),
            cfe::dop::ClosureEvent::ElementApertureReceived {
                deal, to_orb, amount: moved, element, ..
            } => format!(
                "⎋ {} of {} crossed in deal {deal} → orb {to_orb} — open matter arrives",
                style.strong(&amount(moved)),
                style.strong(&hex(element)[..8]),
            ),
            cfe::dop::ClosureEvent::CrystalDrawn { orb_id, amount: drawn } => format!(
                "◎ {} crystal held the lens open on orb {orb_id} — what was seen left no receipt",
                style.strong(&amount(drawn))
            ),
            cfe::dop::ClosureEvent::LineSucceeded { successor } => format!(
                "⏚ the line passes — {} adopts this chronicle as its own substrate; the tree returns to the kernel, the record stands",
                style.strong(successor)
            ),
            cfe::dop::ClosureEvent::GenesisSealed { civilization, .. } => format!(
                "✶ {} seals its three axes — the binding copy lives HERE; the registry is a cache from this receipt on",
                style.strong(civilization)
            ),
            cfe::dop::ClosureEvent::ProspectStruck { stratum, name, family, tier, .. } => format!(
                "⛏ {} read from stratum {} (family {family}, tier {tier}) — what the seed always held",
                style.strong(name),
                style.strong(&stratum.to_string())
            ),
            cfe::dop::ClosureEvent::TreatySealed { kind, partner, term, .. } => format!(
                "⚑ {} sealed with {}{} — the crossing is re-priced by agreement",
                style.strong(
                    cfe::treaty::TreatyKind::from_wire(*kind).map(|k| k.word()).unwrap_or("pact")
                ),
                style.strong(partner),
                match term {
                    0 => String::from(" (standing)"),
                    held => format!(" (for {held})"),
                }
            ),
            cfe::dop::ClosureEvent::TreatyStruck { partner, broken, .. } => format!(
                "⚑ the pact with {} {}",
                style.strong(partner),
                match broken {
                    true => style.bad("is BROKEN — the load posts to this world's own book"),
                    false => style.dim("is renounced"),
                }
            ),
            cfe::dop::ClosureEvent::TreatyElementsAllowed { partner, elements, .. } => format!(
                "⚑ the pact with {} is FENCED to {} — only enumerated matter crosses",
                style.strong(partner),
                style.strong(&format!("{} element id(s)", elements.len()))
            ),
            cfe::dop::ClosureEvent::SolverWordMinted { orbit, cited, .. } => format!(
                "◈ a WORD enters the library — orbit {} confirmed by {} tasks; the language grew",
                style.strong(&hex(orbit)[..8]),
                style.strong(&cited.len().to_string())
            ),
            cfe::dop::ClosureEvent::KnowledgeConverged { signature, mass, hardness } => format!(
                "◈ a shared TRUTH — {} lines resolved to invariant {} (hardness {}); no one line owns it",
                style.strong(&mass.to_string()),
                style.strong(&hex(signature)[..8]),
                style.strong(&hardness.to_string())
            ),
            cfe::dop::ClosureEvent::KnowledgeLicensed { orbit, cited, .. } => format!(
                "◈ an idea LICENSED — word {} ({} confirms) enters the library from the market; knowledge circulates",
                style.strong(&hex(orbit)[..8]),
                style.strong(&cited.len().to_string())
            ),
            cfe::dop::ClosureEvent::GemCompounded { members, gem, .. } => format!(
                "◈ {} bond into {} — the lanes saturate and the matter holds",
                style.strong(&format!("{} elements", members.len())),
                style.strong(&hex(gem)[..8])
            ),
            cfe::dop::ClosureEvent::ElementRefined {
                spent, yielded, refined, ..
            } => format!(
                "◈ {} refined into {} of {} — the refinery converts raw matter",
                style.strong(&amount(spent)),
                style.strong(&amount(yielded)),
                style.strong(&hex(refined)[..8])
            ),
            cfe::dop::ClosureEvent::ElementRouteChartered { deal, eta, rail, element } => format!(
                "⇉⇢ element lane deal {} · η {} · rail {} · {}",
                style.strong(&deal.to_string()),
                style.strong(&amount(eta.value())),
                style.strong(&rail.to_string()),
                style.dim(&hex_prefix(element))
            ),
            cfe::dop::ClosureEvent::StampMinted { amount: a, crystal_spent, .. } => format!(
                "∫ {} STAMP minted from {} crystal (identity)",
                style.strong(&amount(a)),
                style.strong(&amount(crystal_spent))
            ),
            cfe::dop::ClosureEvent::StampApertureSent { deal, amount: a, .. } => format!(
                "∫⎋ deal {} sent {} STAMP across the void",
                style.strong(&deal.to_string()),
                style.strong(&amount(a))
            ),
            cfe::dop::ClosureEvent::StampApertureReceived { deal, amount: a, .. } => format!(
                "∫⎋ deal {} received {} STAMP",
                style.strong(&deal.to_string()),
                style.strong(&amount(a))
            ),
            cfe::dop::ClosureEvent::ScrapSalvaged { amount: a, iron_spent, .. } => format!(
                "⚙ {} scrap salvaged from {} iron",
                style.strong(&amount(a)),
                style.strong(&amount(iron_spent))
            ),
            cfe::dop::ClosureEvent::HullMinted {
                hull_id,
                role,
                grade,
                scrap_spent,
                ..
            } => format!(
                "🚢 hull {} role {} grade {} for {} scrap",
                style.strong(&hull_id.to_string()),
                style.strong(&role.to_string()),
                style.strong(&grade.to_string()),
                style.strong(&amount(scrap_spent))
            ),
            cfe::dop::ClosureEvent::HullAssigned { hull_id, deal } => format!(
                "🚢 hull {} → deal {} (0=idle)",
                style.strong(&hull_id.to_string()),
                style.strong(&deal.to_string())
            ),
            cfe::dop::ClosureEvent::HullScrapped {
                hull_id,
                scrap_returned,
                ..
            } => format!(
                "🚢 hull {} scrapped · {} scrap recovered",
                style.strong(&hull_id.to_string()),
                style.strong(&amount(scrap_returned))
            ),
            cfe::dop::ClosureEvent::OrderWorked {
                kind,
                entropy,
                syntropy,
                ..
            } => format!(
                "⚖ order kind {} — entropy {} · syntropy {}",
                style.strong(&kind.to_string()),
                style.bad(&amount(entropy)),
                style.ok(&amount(syntropy))
            ),
            cfe::dop::ClosureEvent::ElementSalvaged { amount: a, .. } => format!(
                "⚙ {} element mass salvaged to scrap",
                style.strong(&amount(a))
            ),
            cfe::dop::ClosureEvent::NoteIssued {
                note_id,
                notional,
                maturity,
                ..
            } => format!(
                "📜 note {} issued · face {} STAMP · matures t{}",
                style.strong(&note_id.to_string()),
                style.strong(&amount(notional)),
                maturity
            ),
            cfe::dop::ClosureEvent::NoteRedeemed {
                note_id,
                paid,
                seized,
                ..
            } => format!(
                "📜 note {} redeemed · paid {} · seized {}",
                style.strong(&note_id.to_string()),
                amount(paid),
                amount(seized)
            ),
            cfe::dop::ClosureEvent::FutureOpened {
                fut_id,
                resource,
                notional,
                price,
                expiry,
                ..
            } => format!(
                "📈 future {} · {}×{:?} @ {} · exp t{}",
                style.strong(&fut_id.to_string()),
                amount(notional),
                resource,
                amount(price),
                expiry
            ),
            cfe::dop::ClosureEvent::FutureSettled {
                fut_id,
                mark,
                long_pnl,
                ..
            } => format!(
                "📈 future {} settled · mark {} · long PnL {}",
                style.strong(&fut_id.to_string()),
                amount(mark),
                amount(long_pnl)
            ),
            cfe::dop::ClosureEvent::LastClearMarked {
                resource,
                price,
                ..
            } => format!(
                "📌 last clear {:?} = {} STAMP/unit",
                resource,
                amount(price)
            ),
            cfe::dop::ClosureEvent::ShareIssued {
                share_id,
                stream,
                units,
                ..
            } => format!(
                "🏛 share {} · {} units on {:?}",
                style.strong(&share_id.to_string()),
                amount(units),
                stream
            ),
            cfe::dop::ClosureEvent::ShareTransferred {
                share_id,
                to,
                units,
                ..
            } => format!(
                "🏛 share {} transferred {} → {}",
                share_id,
                amount(units),
                to
            ),
            cfe::dop::ClosureEvent::ShareDividend {
                share_id,
                amount: a,
                ..
            } => format!(
                "🏛 share {} dividend {} STAMP",
                share_id,
                amount(a)
            ),
            cfe::dop::ClosureEvent::PerpOpened {
                perp_id,
                resource,
                notional,
                entry,
                ..
            } => format!(
                "∞ perp {} · {}×{:?} @ {}",
                style.strong(&perp_id.to_string()),
                amount(notional),
                resource,
                amount(entry)
            ),
            cfe::dop::ClosureEvent::PerpFunded {
                perp_id,
                funding,
                mark,
                ..
            } => format!(
                "∞ perp {} funding {} @ mark {}",
                perp_id,
                amount(funding),
                amount(mark)
            ),
            cfe::dop::ClosureEvent::PerpClosed {
                perp_id,
                long_pnl,
                liquidated,
                ..
            } => format!(
                "∞ perp {} closed PnL {}{}",
                perp_id,
                amount(long_pnl),
                if *liquidated { " LIQ" } else { "" }
            ),
            cfe::dop::ClosureEvent::HullFinanced {
                hull_id,
                note_id,
                note_face,
                ..
            } => format!(
                "🚢 hull {} financed · note {} face {}",
                hull_id,
                note_id,
                amount(note_face)
            ),
            cfe::dop::ClosureEvent::EscortOnDuty {
                hull_id,
                deal,
                grade,
            } => format!(
                "🛡 escort hull {} g{} on duty · deal {}",
                style.strong(&hull_id.to_string()),
                grade,
                deal
            ),
            cfe::dop::ClosureEvent::InterdictionStruck {
                deal,
                amount: lost,
                risk,
                ..
            } => format!(
                "⚔ interdiction deal {} lost {} (risk {})",
                deal,
                style.bad(&amount(lost)),
                amount(risk)
            ),
            cfe::dop::ClosureEvent::ConvoyRiskMarked {
                deal,
                risk,
                escort_cover,
            } => format!(
                "🛡 convoy deal {} risk {} cover {}",
                deal,
                amount(risk),
                amount(escort_cover)
            ),
            cfe::dop::ClosureEvent::MarketSealed { resume_at } => format!(
                "📉 market sealed — circuit open until t{}",
                style.warn(&resume_at.to_string())
            ),
            cfe::dop::ClosureEvent::MarketResumed {} => {
                "📈 market resumed — circuit breaker lifted".into()
            }
            cfe::dop::ClosureEvent::SpotClearRecorded {
                resource,
                price,
                depth,
                ..
            } => format!(
                "✓ {:?} spot clear @ {} (depth {})",
                resource,
                amount(price),
                depth
            ),
            cfe::dop::ClosureEvent::LiquidityRaidStruck {
                orb_id,
                fee,
                offers_voided,
            } => format!(
                "⚔ liquidity raid orb {} fee {} voided {}",
                orb_id,
                style.bad(&amount(fee)),
                offers_voided
            ),
            cfe::dop::ClosureEvent::ClearPoisonRefused {
                resource,
                depth,
                need,
                ..
            } => format!(
                "⊘ {:?} last-clear poison refused ({}/{})",
                resource, depth, need
            ),
            cfe::dop::ClosureEvent::NoteCascadeStruck {
                orb_id,
                fee,
                pressure,
            } => format!(
                "📉 note cascade orb {} fee {} pressure {}",
                orb_id,
                amount(fee),
                pressure
            ),
            cfe::dop::ClosureEvent::CovertTamperStruck {
                orb_id,
                cost,
                pressure,
            } => format!(
                "🌫 covert tamper orb {} cost {} pressure {}",
                orb_id,
                amount(cost),
                amount(pressure)
            ),
            cfe::dop::ClosureEvent::MediumReinforced {
                orb_id,
                cost,
                pressure,
                resource,
            } => format!(
                "🛡 medium reinforce orb {} {:?} {} pressure {}",
                orb_id,
                resource,
                amount(cost),
                amount(pressure)
            ),
            cfe::dop::ClosureEvent::TamperDetected { orb_id, pressure } => format!(
                "⚠ tamper DETECTED orb {} pressure {}",
                orb_id,
                amount(pressure)
            ),
            cfe::dop::ClosureEvent::PostureReoriented {
                disposition,
                perception,
                primary,
            } => format!(
                "🛡 posture reoriented dis={} per={} primary={}",
                disposition_name(*disposition),
                perception_name(*perception),
                if *primary == 0 {
                    "clear".into()
                } else {
                    cfe::world::DefenseDomain::from_wire(*primary)
                        .map(|d| d.word().to_string())
                        .unwrap_or_else(|| primary.to_string())
                }
            ),
            cfe::dop::ClosureEvent::SpyProbed {
                orb_id,
                depth,
                fee,
            } => format!(
                "👁 spy probe orb {} depth {} fee {}",
                orb_id,
                depth,
                amount(fee)
            ),
            cfe::dop::ClosureEvent::TechExfiltrated {
                condition,
                fee,
                ..
            } => format!(
                "📤 tech exfil condition {} fee {} (intel stash)",
                condition,
                amount(fee)
            ),
            cfe::dop::ClosureEvent::InstrumentSabotaged {
                orb_id,
                grade,
                ..
            } => format!("🕱 instrument sabotage orb {} grade {}", orb_id, grade),
            cfe::dop::ClosureEvent::RailPromoted {
                deal,
                from_rail,
                to_rail,
            } => format!(
                "⬆ rail deal {} R{}→R{}",
                deal, from_rail, to_rail
            ),
            cfe::dop::ClosureEvent::StrandExported {
                amount: a,
                kind,
                ..
            } => format!("↗ strand export kind {kind} amount {}", amount(a)),
            cfe::dop::ClosureEvent::AnchorImported {
                amount: a,
                kind,
                ..
            } => format!("↙ Anchor import kind {kind} amount {}", amount(a)),
            cfe::dop::ClosureEvent::AnchorImportRefused { reason, .. } => {
                format!("⊘ Anchor import refused reason {reason}")
            }
            cfe::dop::ClosureEvent::VaultDeposited { orb_id, amount: a } => {
                format!("🏦 vault deposit orb {} amount {}", orb_id, amount(a))
            }
            cfe::dop::ClosureEvent::VaultWithdrawn { orb_id, amount: a } => {
                format!("🏦 vault withdraw orb {} amount {}", orb_id, amount(a))
            }
            cfe::dop::ClosureEvent::AssayConsilience {
                orb_id,
                matched,
                ..
            } => format!(
                "⚖ assay consilience orb {} — {}",
                orb_id,
                if *matched { "MATCH" } else { "MISMATCH" }
            ),
            cfe::dop::ClosureEvent::AssaySlashed {
                orb_id,
                slash,
                richness_after,
                ..
            } => format!(
                "⚖ assay SLASH orb {} crystal {} · richness {}",
                orb_id,
                amount(slash),
                amount(richness_after)
            ),
            cfe::dop::ClosureEvent::IsoUnionStruck {
                orb_id,
                knowledge,
                ..
            } => format!("◈ iso-union orb {orb_id} knowledge {knowledge}"),
            cfe::dop::ClosureEvent::ChemCataloged {
                orb_id,
                knowledge,
                survival,
                ..
            } => format!(
                "◈ chem catalog orb {orb_id} k{knowledge} survival {}",
                amount(survival)
            ),
            cfe::dop::ClosureEvent::DarkPullStruck {
                orb_id,
                pull,
                weight,
                dark_score,
            } => format!(
                "🌑 dark-{pull} orb {orb_id} weight {} score {}",
                amount(weight),
                amount(dark_score)
            ),
            cfe::dop::ClosureEvent::ScienceSeated {
                orb_id,
                discipline,
                rank,
                fee,
            } => format!(
                "⚗ science seat orb {orb_id} d{discipline} rank {rank} fee {}",
                amount(fee)
            ),
            cfe::dop::ClosureEvent::LabCreated {
                orb_id,
                path,
                risk,
                exploded,
                ..
            } => format!(
                "⚗ lab path {path} orb {orb_id} risk {} {}",
                amount(risk),
                if *exploded { "EXPLODED" } else { "ok" }
            ),
            cfe::dop::ClosureEvent::ScienceExploded {
                orb_id,
                kind,
                slash,
                ..
            } => format!(
                "💥 science explosion kind {kind} orb {orb_id} slash {}",
                amount(slash)
            ),
            cfe::dop::ClosureEvent::RogueFactionStruck { orb_id, pressure } => {
                format!("🕷 rogue faction orb {orb_id} pressure {pressure}")
            }
            cfe::dop::ClosureEvent::AssayStruck { amount: drawn, richness, .. } => format!(
                "⚖ {} drawn from the opened vein — one receipt, the whole span{}",
                style.strong(&amount(drawn)),
                match richness {
                    Some(after) => format!(" · the spring now runs at {}", amount(after)),
                    None => String::new(),
                }
            ),
            cfe::dop::ClosureEvent::HandoffSealed { from, to } => format!(
                "✍ hand {} yields the pen to hand {} (NS-5 §7)",
                style.strong(&from.to_string()),
                style.strong(&to.to_string())
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

/// Parse the hex an operator typed into raw bytes — a full digest or
/// any PREFIX of one, since a prefix is what a display shows.
fn unhex_prefix(word: &str) -> Option<Vec<u8>> {
    let word = word.trim_start_matches("0x");
    if word.is_empty() || word.len() > 64 || !word.len().is_multiple_of(2) {
        return None;
    }
    word.as_bytes()
        .chunks(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).ok()?, 16).ok())
        .collect()
}

/// Resolve a typed prefix against the chronicle to the ONE receipt it
/// names. Zero matches or several are both refused here rather than
/// guessed at — citing evidence is not a place to be approximately
/// right, and the world would refuse an unknown digest anyway.
fn resolve_witness(world: &World, word: &str) -> Result<[u8; 32], String> {
    let prefix = unhex_prefix(word).ok_or_else(|| format!("{word} is not hex"))?;
    let matches: Vec<[u8; 32]> = world
        .chronicle()
        .receipts()
        .iter()
        .map(|receipt| receipt.own.0)
        .filter(|digest| digest.starts_with(&prefix))
        .collect();
    match matches.len() {
        1 => Ok(matches[0]),
        0 => Err(format!("no receipt in this chronicle begins {word}")),
        n => Err(format!("{word} names {n} receipts — type more of the digest")),
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
