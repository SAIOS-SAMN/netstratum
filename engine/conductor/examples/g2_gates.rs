//! Why a line does not solve: per world, the leisure it banked and spent,
//! its famines, and its solves — read from the chronicle alone.
use cfe::dop::ClosureEvent as E;
fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".to_string());
    let mut dirs: Vec<_> = std::fs::read_dir(format!("{root}/worlds")).unwrap().flatten().map(|e| e.path()).filter(|p| p.join("chronicle.tlv").exists()).collect();
    dirs.sort();
    println!("{:<18} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}", "world", "accrued", "spent", "looks", "famines", "relief", "pop");
    for d in dirs {
        let c = cfe::persist::load(&d.join("chronicle.tlv")).unwrap();
        let (mut acc, mut spent, mut looks, mut fam, mut rel, mut pop) = (cfe::ratio::ratio(0, 1), cfe::ratio::ratio(0, 1), 0, 0, 0, 0i64);
        for r in c.receipts() {
            match r.event() {
                E::LeisureSettled { delta, .. } => {
                    if *delta > cfe::ratio::ratio(0, 1) { acc += delta.clone() } else { spent -= delta.clone(); looks += 1 }
                }
                E::FamineStruck { .. } => fam += 1,
                E::FamineRelieved { .. } => rel += 1,
                E::PopulationSettled { delta, .. } => pop += delta,
                _ => {}
            }
        }
        let f = |r: &cfe::ratio::Ratio| format!("{:.2}", num_traits::ToPrimitive::to_f64(r).unwrap_or(f64::NAN));
        println!("{:<18} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}", d.file_name().unwrap().to_str().unwrap(), f(&acc), f(&spent), looks, fam, rel, pop);
    }
}
