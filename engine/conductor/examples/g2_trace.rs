//! A world's first N receipts, in order: tick and event.
fn main() {
    let mut a = std::env::args().skip(1);
    let dir = a.next().expect("world dir");
    let n: usize = a.next().and_then(|s| s.parse().ok()).unwrap_or(40);
    let c = cfe::persist::load(&std::path::Path::new(&dir).join("chronicle.tlv")).unwrap();
    for r in c.receipts().iter().take(n) {
        let e = format!("{:?}", r.event());
        println!("t{:<5} {}", r.stamp(), &e[..e.len().min(220)]);
    }
    println!("... {} receipts", c.receipts().len());
}
