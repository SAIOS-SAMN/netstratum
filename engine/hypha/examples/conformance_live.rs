//! Mycelium's client conformance read against a running hypha, KERNEL
//! profile: `cargo run -p ns-hypha --example conformance_live -- <host:port>`.
//! Spoken under a key minted for the read alone (the read founds its own
//! domain on it), so no Netstratum identity is bound to the hypha read.
use ns_hypha::mycelium_sdk::conformance::{run, Profile};
use std::io::Read;

fn main() {
    let Some(endpoint) = std::env::args().nth(1) else {
        println!("usage: conformance_live <host:port>");
        return;
    };
    let mut seed = [0u8; 32];
    if let Err(e) = std::fs::File::open("/dev/urandom").and_then(|mut f| f.read_exact(&mut seed)) {
        println!("no key for the read: {e}");
        return;
    }
    let key = ed25519_dalek::SigningKey::from_bytes(&seed);
    let report = run(&endpoint, &key, &[Profile::Kernel]);
    println!("{report}");
    println!("CONFORMS {}", report.conforms());
}
