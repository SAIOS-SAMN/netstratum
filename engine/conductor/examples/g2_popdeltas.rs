//! Every PopulationSettled delta in a world, tallied by (orb, delta), with
//! the event kind that came just before each — where a population came from.
use cfe::dop::ClosureEvent as E;
use std::collections::BTreeMap;
fn main() {
    let dir = std::env::args().nth(1).expect("world dir");
    let c = cfe::persist::load(&std::path::Path::new(&dir).join("chronicle.tlv")).unwrap();
    let mut tally: BTreeMap<(u64, i64, String), u32> = BTreeMap::new();
    let mut prev = String::new();
    for r in c.receipts() {
        let kind = format!("{:?}", r.event())
            .split(|c: char| !c.is_ascii_alphanumeric())
            .next()
            .unwrap_or("")
            .to_string();
        if let E::PopulationSettled { orb_id, delta } = r.event() {
            *tally.entry((*orb_id, *delta, prev.clone())).or_default() += 1;
        }
        prev = kind;
    }
    for ((orb, delta, before), n) in tally {
        println!("orb {orb:<3} delta {delta:>7} x{n:<5} after {before}");
    }
}
