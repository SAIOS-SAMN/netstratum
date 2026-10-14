//! Who keeps the record, and is it whole? Per world: the journal's years
//! (gaps), the annals' drained digests, the chronicle's size.
fn main() {
    use ledger::card::{self, jrn};
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut dirs: Vec<_> = std::fs::read_dir(format!("{root}/worlds")).into_iter().flatten().filter_map(|d| d.ok()).map(|d| d.path()).collect();
    dirs.sort();
    let (mut whole, mut gapped, mut missing_total) = (0, 0, 0u64);
    let mut lasts: Vec<u64> = Vec::new();
    for dir in &dirs {
        let years: Vec<u64> = std::fs::read(dir.join("journal.jrn"))
            .map(|d| card::frames(&d).filter_map(|f| card::records(f).find(|(t, _)| *t == jrn::YEAR).and_then(|(_, v)| card::u64_of(v))).collect())
            .unwrap_or_default();
        let digests = std::fs::metadata(dir.join("annals.dig")).map(|m| m.len() / 32).unwrap_or(0);
        let chronicle = std::fs::metadata(dir.join("chronicle.tlv")).map(|m| m.len()).unwrap_or(0);
        let (first, last) = (years.first().copied().unwrap_or(0), years.last().copied().unwrap_or(0));
        let missing = if years.is_empty() { 0 } else { last - first + 1 - years.len() as u64 };
        if missing == 0 { whole += 1 } else { gapped += 1; missing_total += missing }
        lasts.push(last);
        if missing > 0 {
            println!("{:24} journal years {first}..{last}: {} kept, {missing} missing; annals {digests} digests; chronicle {chronicle} bytes", dir.file_name().unwrap().to_string_lossy(), years.len());
        }
    }
    lasts.sort();
    let beat: u64 = std::fs::read_to_string(format!("{root}/conductor.beat")).ok().and_then(|b| b.trim().parse().ok()).unwrap_or(0);
    let behind = lasts.iter().filter(|&&l| l + 2 < beat / 60).count();
    println!("JOURNALS — last kept year: lowest {:?}, median {:?}, highest {:?}; universe year {} (beat {beat}); {behind} worlds more than a year behind", lasts.first(), lasts.get(lasts.len() / 2), lasts.last(), beat / 60);
    // The universe journal: beat frames continuous? year frames at each close?
    let (mut beats, mut years, mut gaps, mut prev): (Vec<u64>, Vec<(u64, usize)>, u64, Option<u64>) = (Vec::new(), Vec::new(), 0, None);
    if let Ok(data) = std::fs::read(format!("{root}/universe.jrn")) {
        for frame in card::frames(&data) {
            let recs: Vec<(u8, &[u8])> = card::records(frame).collect();
            let get = |t: u8| recs.iter().find(|(x, _)| *x == t).and_then(|(_, v)| card::u64_of(v));
            let (Some(b), Some(k)) = (get(conductor::annal::tag::BEAT), get(conductor::annal::tag::KIND)) else { continue };
            if k == conductor::annal::BEAT_FRAME {
                if let Some(p) = prev { gaps += b.saturating_sub(p + 1); }
                prev = Some(b);
                beats.push(b);
            } else {
                years.push((b, recs.iter().filter(|(t, _)| *t == conductor::annal::tag::LINE).count()));
            }
        }
    }
    println!("UNIVERSE JOURNAL — {} beat frames ({:?}..{:?}), {gaps} beats missing between; {} year closes {:?}", beats.len(), beats.first(), beats.last(), years.len(), years);
    println!("JOURNALS — {} worlds: {whole} whole, {gapped} with gaps ({missing_total} years missing)", dirs.len());
}
