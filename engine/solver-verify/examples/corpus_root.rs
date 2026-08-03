//! Derive CORPUS_ROOT over an admissible corpus directory and print it
//! as the Rust constant to paste into `corpus.rs`.
//!
//! The root is a CONSTITUTIONAL constant: it is the judge's whole answer
//! to "is this task real". Re-running this on a different corpus yields
//! a different root, which is an amendment, not a rebuild.
//!
//! I/O lives HERE, in an example, and never in the crate: the sealed
//! verifier reads no files ever.
//!
//! Usage: cargo run -p solver-verify --example corpus_root -- <dir>

use solver_verify::{corpus, load_task};

fn main() {
    let dir = std::env::args()
        .nth(1)
        .expect("usage: corpus_root <corpus_dir>");

    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("corpus dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "txt"))
        .collect();
    paths.sort();

    let mut task_ids = Vec::new();
    let mut skipped = 0u32;
    for path in &paths {
        match load_task(path) {
            Ok(task) => task_ids.push(corpus::task_digest(&task.train, &task.test_in)),
            Err(e) => {
                skipped += 1;
                eprintln!("skip {path:?}: {e:?}");
            }
        }
    }

    let admitted = task_ids.len();
    let mut deduped = task_ids.clone();
    deduped.sort_unstable();
    deduped.dedup();

    let root = corpus::merkle_root(&task_ids).expect("a non-empty corpus");

    println!("corpus:   {dir}");
    println!("files:    {} · loaded {admitted} · skipped {skipped}", paths.len());
    println!("distinct: {} task_ids", deduped.len());
    if deduped.len() != admitted {
        println!(
            "NOTE: {} duplicate task_ids — identical CONTENT under different names; \
             the set is what the root commits to, so duplicates collapse.",
            admitted - deduped.len()
        );
    }
    println!("\npub const CORPUS_ROOT: [u8; 32] = [");
    for chunk in root.chunks(16) {
        let row: Vec<String> = chunk.iter().map(|b| format!("0x{b:02x}")).collect();
        println!("    {},", row.join(", "));
    }
    println!("];");

    // Prove the tree is usable, not merely computable: every admitted
    // task must produce a path that verifies against the root.
    let mut checked = 0u32;
    for id in &deduped {
        let path = corpus::inclusion_path(&task_ids, id).expect("a member has a path");
        assert!(
            corpus::verify_inclusion(id, &path, &root),
            "every member must verify against the root it is in"
        );
        checked += 1;
    }
    println!("\nverified: {checked}/{} inclusion paths resolve to the root", deduped.len());
}
