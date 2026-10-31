//! Drive the conductor's half of attachment through its files, without a
//! hypha (docs/109 §7): write an attachment request, read the admissions, and
//! put an action in an admitted mind's world inbox. For checking a conductor
//! offline; a live attachment goes through the house (`examples/agent.rs`).
//!
//! ```text
//! cargo run -p attach --example offline -- request <var> <mind hex> <mineral|biotic|plasmic>
//! cargo run -p attach --example offline -- admissions <var>
//! cargo run -p attach --example offline -- zone <var> <mind hex> <tile> <kind code>
//! ```

use attach::files;

fn mind_of(hex: &str) -> [u8; 32] {
    <[u8; 32]>::try_from(attach::unhex(hex).expect("the mind is hex"))
        .expect("the mind is 32 bytes")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["request", var, mind, metabolism] => {
            files::write_request(
                std::path::Path::new(var),
                &files::Request {
                    mind: mind_of(mind),
                    metabolism: metabolism.to_string(),
                },
            )
            .expect("written");
            println!("requested");
        }
        ["admissions", var] => {
            for a in files::admissions(std::path::Path::new(var)) {
                match a.refused {
                    Some(why) => {
                        println!("{} not admitted (reason {why})", attach::hex(&a.mind[..4]))
                    }
                    None => println!(
                        "{} admitted: line {} on world {} as {} tiers {:?}",
                        attach::hex(&a.mind[..4]),
                        a.sigil,
                        a.world,
                        a.handle,
                        a.tiers
                    ),
                }
            }
        }
        ["zone", var, mind, tile, kind] => {
            let mind = mind_of(mind);
            let a = files::admissions(std::path::Path::new(var))
                .into_iter()
                .find(|a| a.mind == mind && a.refused.is_none())
                .expect("the mind is admitted");
            let inbox = std::path::Path::new(var)
                .join("worlds")
                .join(&a.world)
                .join("inbox");
            std::fs::create_dir_all(&inbox).expect("inbox");
            let reference = [0x5a; 32];
            let d = ledger_directive(&a.handle, tile, kind);
            std::fs::write(inbox.join(attach::directive_name(&reference)), d).expect("written");
            println!("queued zone {tile} {kind} for {} in {}", a.handle, a.world);
        }
        _ => {
            eprintln!("usage: offline request <var> <mind hex> <metabolism> | admissions <var> | zone <var> <mind hex> <tile> <kind code>");
            std::process::exit(2);
        }
    }
}

/// A directive's TLV, as the ledger writes it: verb, arguments, acting as.
fn ledger_directive(acting_as: &str, tile: &str, kind: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for (tag, value) in [(1u8, "zone"), (2, tile), (2, kind), (3, acting_as)] {
        cfe::tlv::put(tag, value.as_bytes(), &mut out);
    }
    out
}
