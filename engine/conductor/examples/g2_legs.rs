//! Match every aperture's send leg to its receive leg by deal id, across a
//! universe: unmatched legs and per-deal losses, per carrier.
use cfe::dop::ClosureEvent as E;
use cfe::ratio::Ratio;
use std::collections::BTreeMap;
fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut sent: BTreeMap<(String, u64), (String, Ratio)> = BTreeMap::new();
    let mut got: BTreeMap<(String, u64), (String, Ratio)> = BTreeMap::new();
    let mut dirs: Vec<_> = std::fs::read_dir(format!("{root}/worlds")).unwrap().flatten().map(|e| e.path()).filter(|p| p.join("chronicle.tlv").exists()).collect();
    dirs.sort();
    for d in &dirs {
        let w = d.file_name().unwrap().to_str().unwrap().to_string();
        let c = cfe::persist::load(&d.join("chronicle.tlv")).unwrap();
        for r in c.receipts() {
            match r.event() {
                E::StampApertureSent { deal, amount, .. } => { sent.insert(("STAMP".into(), *deal), (w.clone(), amount.clone())); }
                E::StampApertureReceived { deal, amount, .. } => { got.insert(("STAMP".into(), *deal), (w.clone(), amount.clone())); }
                E::ApertureSent { deal, resource, amount, .. } => { sent.insert((format!("{resource:?}"), *deal), (w.clone(), amount.clone())); }
                E::ApertureReceived { deal, resource, amount, .. } => { got.insert((format!("{resource:?}"), *deal), (w.clone(), amount.clone())); }
                _ => {}
            }
        }
    }
    let f = |r: &Ratio| num_traits::ToPrimitive::to_f64(r).unwrap_or(f64::NAN);
    let mut by: BTreeMap<String, (usize, usize, f64, f64, f64)> = BTreeMap::new();
    for ((carrier, deal), (_, a)) in &sent {
        let e = by.entry(carrier.clone()).or_default();
        match got.get(&(carrier.clone(), *deal)) {
            Some((_, b)) => { e.0 += 1; e.2 += f(a) - f(b); }
            None => { e.1 += 1; e.3 += f(a); }
        }
    }
    for ((carrier, deal), (_, b)) in &got {
        if !sent.contains_key(&(carrier.clone(), *deal)) { by.entry(carrier.clone()).or_default().4 += f(b); }
    }
    println!("{:<8} {:>8} {:>10} {:>14} {:>16} {:>16}", "carrier", "paired", "send-only", "paired loss", "send-only amt", "recv-only amt");
    for (c, (p, so, loss, soa, roa)) in by {
        println!("{c:<8} {p:>8} {so:>10} {loss:>14.3} {soa:>16.3} {roa:>16.3}");
    }
}
