//! Time solver::knowledge_map per world on the words its chronicle minted.
fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut dirs: Vec<_> = std::fs::read_dir(format!("{root}/worlds")).unwrap().flatten().map(|e| e.path()).filter(|p| p.join("chronicle.tlv").exists()).collect();
    dirs.sort();
    for d in dirs {
        let c = cfe::persist::load(&d.join("chronicle.tlv")).unwrap();
        let mut lib = solver::mint::Library::new();
        for (orbit, bytes, cited) in cfe::vocabulary::solver_words(&c) {
            if let Some(expansion) = solver_verify::program_from_bytes(&bytes) {
                lib.adopt(solver::mint::MintedWord { orbit, expansion, cited });
            }
        }
        if lib.words().is_empty() { continue; }
        let t = std::time::Instant::now();
        let _ = solver::knowledge_map(&lib);
        println!("{:<18} words {:>4}  knowledge_map {:?}", d.file_name().unwrap().to_str().unwrap(), lib.words().len(), t.elapsed());
    }
}
