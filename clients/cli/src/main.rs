//! The Netstratum CLI — the first slice's face. One command launches
//! it (CHARTER §3.5); every completed action closes as an NS-3 receipt;
//! `verify` and `replay` read the chain the way any peer would.

use std::io::{self, BufRead, Write};

use cfe::dop::ZoneKind;
use cfe::vrp::ChainRead;
use cfe::world::{replay, World};

fn main() {
    let script = std::env::args().skip_while(|arg| arg != "--script").nth(1);
    let mut world = World::new();
    println!("NETSTRATUM — the first slice. Commands: found <name> | zone <tile> <ext|hab> | tick <n> | collect | status | chronicle | verify | replay | quit");
    match script {
        Some(script) => {
            for line in script.split(';') {
                let line = line.trim();
                match line.is_empty() {
                    true => {}
                    false => {
                        println!("> {line}");
                        command(&mut world, line);
                    }
                }
            }
        }
        None => {
            let stdin = io::stdin();
            loop {
                print!("netstratum> ");
                io::stdout().flush().ok();
                let mut line = String::new();
                match stdin.lock().read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        let line = line.trim().to_string();
                        match line.as_str() {
                            "quit" | "exit" => break,
                            "" => {}
                            _ => command(&mut world, &line),
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
}

fn command(world: &mut World, line: &str) {
    let mut words = line.split_whitespace();
    match words.next() {
        Some("found") => {
            let name = words.collect::<Vec<_>>().join(" ");
            match world.found(&name) {
                Ok(()) => println!("colony '{name}' founded — receipt sealed"),
                Err(refusal) => println!("refused: {refusal:?}"),
            }
        }
        Some("zone") => {
            let tile = words.next().and_then(|word| word.parse::<u16>().ok());
            let kind = match words.next() {
                Some("ext") => Some(ZoneKind::Extractor),
                Some("hab") => Some(ZoneKind::Habitat),
                _ => None,
            };
            match (tile, kind) {
                (Some(tile), Some(kind)) => match world.zone(tile, kind) {
                    Ok(()) => println!("tile {tile} zoned {kind:?} — receipt sealed"),
                    Err(refusal) => println!("refused: {refusal:?}"),
                },
                _ => println!("usage: zone <tile> <ext|hab>"),
            }
        }
        Some("tick") => {
            let count = words
                .next()
                .and_then(|word| word.parse::<u64>().ok())
                .unwrap_or(1);
            for _ in 0..count {
                world.tick();
            }
            println!("clock now {}", world.now());
        }
        Some("collect") => match world.collect() {
            Ok(count) => println!("collected — {count} receipt(s) sealed at stamp {}", world.now()),
            Err(refusal) => println!("refused: {refusal:?}"),
        },
        Some("status") => {
            match world.orb() {
                Some(orb) => {
                    println!("colony '{}' (orb {})", orb.name, orb.orb_id);
                    for zone in &orb.zones {
                        println!("  tile {:>3} {:?}", zone.tile, zone.kind);
                    }
                    for (resource, amount) in &orb.banked {
                        println!("  banked {resource:?}: {amount}");
                    }
                }
                None => println!("no colony — 'found <name>' begins the story"),
            }
            for (resource, amount) in world.accrued() {
                println!("  accrued (workspace) {resource:?}: {amount}");
            }
            println!("clock {} | receipts {}", world.now(), world.chronicle().receipts().len());
        }
        Some("chronicle") => {
            for receipt in world.chronicle().receipts() {
                println!(
                    "#{} stamp {} own {} event {:?}",
                    receipt.sequence,
                    receipt.stamp,
                    hex(&receipt.own.0[..8]),
                    receipt.event
                );
            }
        }
        Some("verify") => match world.chronicle().verify_chain() {
            ChainRead::Intact => println!("chain INTACT ({} receipts)", world.chronicle().receipts().len()),
            ChainRead::Broken { at } => println!("chain BROKEN at sequence {at}"),
        },
        Some("replay") => {
            let replayed = replay(world.chronicle());
            match replayed == world.committed() {
                true => println!("replay MATCHES the committed world"),
                false => println!("replay DIVERGES — the chronicle and the world disagree"),
            }
        }
        Some(other) => println!("unknown command '{other}'"),
        None => {}
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
