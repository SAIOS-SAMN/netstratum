//! Where do two chronicles of one world part ways? Index, and the events there.
fn main() {
    let mut a = std::env::args().skip(1);
    let (p, q) = (a.next().unwrap(), a.next().unwrap());
    let x = cfe::persist::load(&std::path::Path::new(&p).join("chronicle.tlv"));
    let y = cfe::persist::load(&std::path::Path::new(&q).join("chronicle.tlv"));
    let (x, y) = match (x, y) { (Ok(x), Ok(y)) => (x, y), (x, y) => { println!("load: {:?} / {:?}", x.err(), y.err()); return; } };
    let (rx, ry) = (x.receipts(), y.receipts());
    println!("lengths {} / {}", rx.len(), ry.len());
    match (0..rx.len().min(ry.len())).find(|&i| rx[i].own() != ry[i].own()) {
        Some(i) => {
            println!("first difference at receipt {i}");
            for (n, r) in [("A", &rx[i]), ("B", &ry[i])] { let e = format!("{:?}", r.event()); println!("  {n}: t{} {}", r.stamp(), &e[..e.len().min(200)]); }
            if i > 0 { let e = format!("{:?}", rx[i-1].event()); println!("  before: t{} {}", rx[i-1].stamp(), &e[..e.len().min(160)]); }
        }
        None => println!("one is a prefix of the other"),
    }
}
