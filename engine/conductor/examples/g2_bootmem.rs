//! Boot every G2 world as the conductor does (`snapshot::boot`) and report
//! the resident memory after each, how it came back, and its held/kept
//! receipt counts — to find what fills the conductor's heap.
use conductor::snapshot;
use std::path::Path;

fn rss_mb() -> u64 {
    std::fs::read_to_string("/proc/self/statm").ok().and_then(|s| s.split_whitespace().nth(1)?.parse::<u64>().ok()).unwrap_or(0) * 4096 / 1_048_576
}

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut dirs: Vec<_> = std::fs::read_dir(Path::new(&root).join("worlds")).unwrap().flatten().map(|e| e.path()).filter(|p| p.join("chronicle.tlv").exists() && !p.to_string_lossy().contains("--closed-")).collect();
    dirs.sort();
    let mut worlds = Vec::new();
    let start = rss_mb();
    for d in dirs {
        let before = rss_mb();
        let Some((w, how, _)) = snapshot::boot(&d) else { println!("{:20} did not boot", d.display()); continue };
        let c = w.chronicle();
        println!("{:24} {:?} held {} kept {} unfiled {} +{}MB = {}MB", d.file_name().unwrap().to_string_lossy(), how, c.receipts().len(), c.annals().kept().len(), c.annals().unfiled().len(), rss_mb() - before, rss_mb());
        worlds.push(w);
    }
    println!("{} worlds, {}MB from {}MB", worlds.len(), rss_mb(), start);
}
