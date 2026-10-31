//! What the annals keep in memory, by kind: every G2 world booted as the
//! conductor does, one at a time and dropped, its kept receipts tallied by
//! event kind (count and wire bytes) across the universe.
use conductor::snapshot;
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut dirs: Vec<_> = std::fs::read_dir(Path::new(&root).join("worlds"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.join("state.snap").exists() && !p.to_string_lossy().contains("--closed-"))
        .collect();
    dirs.sort();
    let mut kinds: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    let (mut worlds, mut kept, mut held) = (0u64, 0u64, 0u64);
    for d in dirs {
        let Some((w, _, _)) = snapshot::boot(&d) else {
            continue;
        };
        worlds += 1;
        let c = w.chronicle();
        held += c.receipts().len() as u64;
        for r in c.receipts() {
            let name = format!("{:?}", r.event());
            let kind = format!("tail:{}", name.split([' ', '{', '(']).next().unwrap_or("?"));
            let e = kinds.entry(kind).or_default();
            e.0 += 1;
            e.1 += r.wire_bytes().len() as u64;
        }
        for r in c.annals().kept_iter() {
            kept += 1;
            let name = format!("{:?}", r.event());
            let kind = name
                .split([' ', '{', '('])
                .next()
                .unwrap_or("?")
                .to_string();
            let e = kinds.entry(kind).or_default();
            e.0 += 1;
            e.1 += r.wire_bytes().len() as u64;
        }
    }
    let mut rows: Vec<_> = kinds.into_iter().collect();
    rows.sort_by_key(|(_, (_, bytes))| std::cmp::Reverse(*bytes));
    println!("{worlds} worlds: {kept} kept, {held} held in the tails");
    for (kind, (n, bytes)) in rows.iter().take(40) {
        println!("{n:9} {:8} KB  {kind}", bytes / 1024);
    }
}
