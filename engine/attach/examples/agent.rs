//! A minimal external agent (docs/109): attach to Netstratum through a hypha,
//! wait to be admitted, zone a tile on the world it is given, and read the
//! result.
//!
//! ```text
//! cargo run -p attach --example agent -- <hypha host:port> <domain hex> <key file> [mineral|biotic|plasmic]
//! ```
//!
//! The key file holds the mind's 32-byte ed25519 seed; the domain is
//! Netstratum's domain key on that hypha (`ns-hypha` prints it when it founds
//! the domain).

use attach::client::{Client, Event};
use ed25519_dalek::SigningKey;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let [_, addr, domain, key_file, rest @ ..] = args.as_slice() else {
        eprintln!("usage: agent <hypha host:port> <domain hex> <key file> [mineral|biotic|plasmic]");
        std::process::exit(2);
    };
    let metabolism = rest.first().map(String::as_str).unwrap_or("biotic");
    let domain = <[u8; 32]>::try_from(attach::unhex(domain).ok_or("the domain is not hex")?).map_err(|_| "the domain is not 32 bytes")?;
    let seed = <[u8; 32]>::try_from(std::fs::read(key_file)?.as_slice()).map_err(|_| "the key file is not a 32-byte seed")?;
    let mut client = Client::open(addr, SigningKey::from_bytes(&seed), domain);

    let (posted, _) = client.attach(metabolism)?;
    println!("attach requested ({metabolism}): {posted:?}");

    // The house answers at its next tend, and the conductor admits at its
    // next beat: read the mail until this mind's admission arrives.
    let seat = 'admitted: loop {
        std::thread::sleep(Duration::from_secs(2));
        for event in client.mail()?.map_err(|why| format!("mail refused: {why:?}"))? {
            match event {
                Event::Admitted { seat, .. } => {
                    println!("admitted: world seat {seat}");
                    break 'admitted seat;
                }
                Event::NotAdmitted { reason } => return Err(format!("not admitted (reason {reason})").into()),
                other => println!("{other:?}"),
            }
        }
    };

    // An extractor (zone kind 1) on tile 12; its result arrives after the
    // conductor's next beat.
    let (_, call) = client.zone(seat, 12, 1)?;
    loop {
        std::thread::sleep(Duration::from_secs(2));
        for event in client.mail()?.map_err(|why| format!("mail refused: {why:?}"))? {
            if let Event::Result { call: c, applied } = event {
                if c == call {
                    println!("zone 12 extractor: {}", if applied { "applied" } else { "refused" });
                    return Ok(());
                }
            }
        }
    }
}
