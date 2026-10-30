//! The spine archive (coordinator ruling: archive cold, verified, then prune
//! to the head). For one world's spine: every blob in `store/` must hash to
//! its own name and be the state of a commitment in `arbor.tlv` (opened in
//! full, so the chain itself re-verifies). Writes `<out>/<world>.manifest`
//! (`<state hash> <commitment index>` per archivable blob) and
//! `<out>/<world>.list` (the blob paths to archive: every verified blob but
//! the head's). Exits nonzero, archiving nothing, on any mismatch.
//!   cargo run --release -p conductor --example spine_archive -- <world dir> <out dir>
use std::collections::HashMap;
use std::path::Path;

fn hex(b: &[u8; 32]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (dir, out) = (Path::new(&args[1]), Path::new(&args[2]));
    let world = dir.file_name().unwrap().to_string_lossy().to_string();
    let spine = dir.join("spine");
    let arbor = match mesh::arbor::Arbor::open(&spine) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{world}: the arbor does not verify: {e:?}; nothing archived");
            std::process::exit(2);
        }
    };
    let mut at: HashMap<String, usize> = HashMap::new();
    for (i, record) in arbor.records().iter().enumerate() {
        at.entry(hex(&record.state)).or_insert(i);
    }
    let head = arbor.head("").map(|h| hex(&h.state));
    let mut manifest = String::new();
    let mut list = String::new();
    let (mut archived, mut bytes) = (0u64, 0u64);
    for entry in std::fs::read_dir(spine.join("store")).unwrap().flatten() {
        let path = entry.path();
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let content = std::fs::read(&path).unwrap();
        if hex(blake3::hash(&content).as_bytes()) != name {
            eprintln!("{world}: blob {name} does not hash to its name; nothing archived");
            std::process::exit(3);
        }
        let Some(index) = at.get(&name) else {
            eprintln!("{world}: blob {name} is the state of no commitment; nothing archived");
            std::process::exit(4);
        };
        if Some(&name) == head.as_ref() {
            continue; // the head stays hot
        }
        manifest.push_str(&format!("{name} {index}\n"));
        list.push_str(&format!("{}\n", path.strip_prefix(dir).unwrap().display()));
        archived += 1;
        bytes += content.len() as u64;
    }
    std::fs::create_dir_all(out).unwrap();
    std::fs::write(out.join(format!("{world}.manifest")), manifest).unwrap();
    std::fs::write(out.join(format!("{world}.list")), list).unwrap();
    println!("{world}: {archived} blobs verified for the archive ({bytes} bytes), head kept, {} commitments", arbor.records().len());
}
