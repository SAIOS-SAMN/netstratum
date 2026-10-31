//! Receipt kinds across a universe, by count and share — what the bulk is.
use std::collections::BTreeMap;
fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut n: BTreeMap<String, u64> = BTreeMap::new();
    let mut total = 0u64;
    for d in std::fs::read_dir(format!("{root}/worlds"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.join("chronicle.tlv").exists())
    {
        let Ok(c) = cfe::persist::load(&d.join("chronicle.tlv")) else {
            continue;
        };
        for r in c.receipts() {
            let k = format!("{:?}", r.event())
                .split(|ch: char| !ch.is_ascii_alphanumeric())
                .next()
                .unwrap_or("")
                .to_string();
            *n.entry(k).or_default() += 1;
            total += 1;
        }
    }
    let mut v: Vec<_> = n.into_iter().collect();
    v.sort_by_key(|(_, c)| std::cmp::Reverse(*c));
    let mut cum = 0u64;
    for (k, c) in v {
        cum += c;
        println!(
            "{k:<28} {c:>9}  {:>5.1}%  cum {:>5.1}%",
            100.0 * c as f64 / total as f64,
            100.0 * cum as f64 / total as f64
        );
    }
    println!("total {total}");
}
