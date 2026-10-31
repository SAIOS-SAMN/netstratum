//! What a collapsed world keeps receipting after its CivilizationCollapsed.
use cfe::dop::ClosureEvent as E;
use std::collections::BTreeMap;
fn main() {
    for dir in std::env::args().skip(1) {
        let c = cfe::persist::load(&std::path::Path::new(&dir).join("chronicle.tlv")).unwrap();
        let mut after = false;
        let mut kinds: BTreeMap<String, u32> = BTreeMap::new();
        let mut at = 0;
        for (i, r) in c.receipts().iter().enumerate() {
            if after {
                let k = format!("{:?}", r.event())
                    .split(|ch: char| !ch.is_ascii_alphanumeric())
                    .next()
                    .unwrap_or("")
                    .to_string();
                *kinds.entry(k).or_default() += 1;
            }
            if matches!(r.event(), E::CivilizationCollapsed { .. }) && !after {
                after = true;
                at = i;
            }
        }
        println!(
            "{dir}: collapsed at receipt {at} of {}; after it: {kinds:?}",
            c.receipts().len()
        );
    }
}
