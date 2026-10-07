//! A universe's matter budget by receipt kind: every world's census,
//! summed — what each kind made (sources) and unmade (sinks), per holding.
//!   cargo run --release -p ns-hypha --example sinks -- var-g2
use ns_hypha::census::{self, grains, Holding};
use std::collections::BTreeMap;
fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut dirs: Vec<_> = std::fs::read_dir(format!("{root}/worlds")).unwrap().flatten().map(|e| e.path()).filter(|p| p.join("chronicle.tlv").exists()).collect();
    dirs.sort();
    let mut total: BTreeMap<(String, Holding), i128> = BTreeMap::new();
    for d in &dirs {
        let c = cfe::persist::load(&d.join("chronicle.tlv")).unwrap();
        let n = c.receipts().len();
        if n == 0 { continue; }
        let Ok(window) = census::take("", &c, 0, n) else { continue };
        for (kind, h, v) in window.declared {
            *total.entry((kind, h)).or_default() += v;
        }
    }
    let mut rows: Vec<_> = total.into_iter().filter(|((_, h), _)| *h != Holding::Words).collect();
    rows.sort_by_key(|(_, v)| *v);
    for ((kind, h), v) in rows {
        let side = if v < 0 { "SINK  " } else { "source" };
        println!("{side} {kind:<26} {:<12} {:>16}", format!("{h:?}").replace("Matter(", "").replace(')', ""), grains(v));
    }
}
