//! Is every `<world>--closed-*` copy a PREFIX of the live `<world>`
//! chronicle (a pure duplicate, no history of its own)? One at a time.
fn main() {
    let mut a = std::env::args().skip(1);
    let root = a.next().unwrap_or_else(|| "var-g2".into());
    let world = a.next().unwrap_or_else(|| "forge-reach".into());
    let (mut prefix, mut not, mut unread) = (0, 0, 0);
    let mut dirs: Vec<_> = std::fs::read_dir(format!("{root}/worlds")).unwrap().flatten().map(|e| e.path())
        .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with(&format!("{world}--closed-")))).collect();
    dirs.sort();
    // The reference is the live world if it stands, else the LARGEST copy.
    let live_dir = std::path::PathBuf::from(format!("{root}/worlds/{world}"));
    let reference = if live_dir.join("chronicle.tlv").exists() { live_dir } else {
        dirs.iter().max_by_key(|d| std::fs::metadata(d.join("chronicle.tlv")).map(|m| m.len()).unwrap_or(0)).cloned().expect("a copy")
    };
    println!("reference: {}", reference.display());
    let live = cfe::persist::load(&reference.join("chronicle.tlv")).expect("reference chronicle");
    let lr = live.receipts();
    for d in &dirs {
        match cfe::persist::load(&d.join("chronicle.tlv")) {
            Ok(c) => {
                let r = c.receipts();
                if r.len() <= lr.len() && r.last().map(|x| x.own()) == lr.get(r.len().wrapping_sub(1)).map(|x| x.own()) { prefix += 1 }
                else { not += 1; println!("NOT A PREFIX: {} ({} receipts)", d.display(), r.len()); }
            }
            Err(e) => { unread += 1; println!("unreadable: {} {e:?}", d.display()); }
        }
    }
    println!("{world}: {} copies — {prefix} pure prefixes of the live chronicle ({} receipts), {not} not, {unread} unreadable", dirs.len(), lr.len());
}
