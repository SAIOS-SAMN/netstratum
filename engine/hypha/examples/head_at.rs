//! Where in the local colonies line does a given head stand? Prints the entry
//! index (on the whole line) whose head matches, and how many hot entries and
//! logged folds lie after it.
use ns_hypha::line::{FoldLog, TierLine};
fn main() {
    let want = std::env::args().nth(1).expect("hex prefix of a head");
    let home = std::path::Path::new("var/uplink");
    let line = TierLine::open(home, "colonies").unwrap();
    let log = FoldLog::open(home, "colonies").unwrap();
    let hex = |h: &[u8; 32]| h.iter().map(|b| format!("{b:02x}")).collect::<String>();
    println!(
        "line: base {} offset {} hot entries {} head {}",
        &hex(&line.base)[..16],
        line.offset,
        line.entries.len(),
        &hex(&line.head())[..16]
    );
    if hex(&line.base).starts_with(&want) {
        println!(
            "the court's head is the line's BASE (offset {})",
            line.offset
        );
    }
    match line
        .entries
        .iter()
        .position(|e| hex(&e.head).starts_with(&want))
    {
        Some(i) => {
            let at = line.offset + i as u64 + 1;
            let after = log.folds.iter().filter(|f| f.first >= at).count();
            println!("found at hot entry {i} (line index {}); {} hot entries after it; {after} logged folds start after it", at - 1, line.entries.len() - i - 1);
            for f in log.folds.iter().filter(|f| f.first + f.count == at) {
                println!(
                    "  the fold ending there: first {} count {}",
                    f.first, f.count
                );
            }
        }
        None => println!("not on the hot line"),
    }
}
