//! Group a world's rotated copies into FAMILIES: each family is the copies
//! that are prefixes of one history; its head is the longest of them.
fn main() {
    let mut a = std::env::args().skip(1);
    let root = a.next().unwrap_or_else(|| "var-g2".into());
    let world = a.next().unwrap_or_else(|| "forge-reach".into());
    let mut dirs: Vec<(u64, std::path::PathBuf)> = std::fs::read_dir(format!("{root}/worlds"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(&format!("{world}--closed-")))
        })
        .map(|p| {
            (
                std::fs::metadata(p.join("chronicle.tlv"))
                    .map(|m| m.len())
                    .unwrap_or(0),
                p,
            )
        })
        .collect();
    dirs.sort_by(|x, y| y.0.cmp(&x.0)); // largest first
                                        // Each family: (head path, head's own-hashes) — kept one at a time.
    let mut heads: Vec<(std::path::PathBuf, Vec<[u8; 32]>, usize, String)> = Vec::new();
    for (_, d) in &dirs {
        let c = match cfe::persist::load(&d.join("chronicle.tlv")) {
            Ok(c) => c,
            Err(e) => {
                println!("unreadable {} {e:?}", d.display());
                continue;
            }
        };
        let r = c.receipts();
        let last = r.last().map(|x| x.own().0);
        let fam = heads.iter_mut().find(|(_, own, _, _)| {
            r.len() <= own.len() && last == own.get(r.len().wrapping_sub(1)).copied()
        });
        match fam {
            Some(f) => f.2 += 1,
            None => {
                let who = r
                    .iter()
                    .find_map(|x| match x.event() {
                        cfe::dop::ClosureEvent::EntityRegistered { handle, .. }
                            if handle.ends_with("-conductor") =>
                        {
                            Some(handle.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                heads.push((d.clone(), r.iter().map(|x| x.own().0).collect(), 1, who));
            }
        }
    }
    for (head, own, n, who) in &heads {
        println!(
            "family of {:<22} head {} ({} receipts): {n} copies",
            who,
            head.display(),
            own.len()
        );
    }
}
