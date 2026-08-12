//! THE RECURRENCE GUARD — CHARTER §11 rule 5 as a STANDING TEST.
//!
//! "The mirror was never illegal, only leashed, which is why accretion
//! captured it. Vigilance did not save us; a standing test does."
//! (KERNEL_RELAY §2.3.) This witness mechanizes the review lint at the
//! same tier as clippy-zero and replay-MATCHES: the mirror-SHAPE fails
//! the board, not a reviewer's memory.
//!
//! What it pins (each check names its law):
//!
//! 1. THE DEAD NAME — `engine/mirror` stays deleted; no workspace
//!    member is ever named mirror again (d606b01).
//! 2. THE ANTI-MIRROR — `cfe::opposition` stays excised (c7cad9e):
//!    you do not reconcile what cannot diverge.
//! 3. RETIRED CODES — tag 65 (the consilience receipt) is never
//!    reassigned; retired wire codes are never reused.
//! 4. THE ARROW LAW — the kernel imports the sealed math tier + cfe
//!    and NOTHING imports upward into it; cfe never imports the
//!    verifier (the opaque-payload law's cycle guard). The allowed
//!    sets grow only by amendment: editing this test IS the amendment
//!    (U2 adds rfa-core to the kernel's set in its own landing).
//! 5. NO NEW READ-BACK PATH EVER — the ledger/conductor/cli files
//!    that parse JSON back in are pre-chain Tier-A debts, adopted at
//!    #36, PINNED BY NAME. A new source file that reads a display
//!    export back in fails here, named. Exports are write-only
//!    (§11 rule 2); tests are witnesses and stay out of scope.
//! 6. THE UPLINK LAW — the mesh carries no file of the eye: no JSON
//!    token in `engine/mesh/src` (only chain-verified bytes ascend;
//!    the arbor's own store is the chain, not an export).
//! 7. THE TEETH — the detectors are proven on synthetic violations
//!    before the clean pass is trusted (a guard that cannot bite is
//!    the vacuous check this audit exists to kill).
//!
//! File I/O lives HERE, in a witness — the kernel itself stays pure.

use std::fs;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("engine/kernel sits two levels under the root")
        .to_path_buf()
}

/// Every `.rs` file under `dir`, recursively.
fn rust_sources(dir: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_sources(&path, found);
        } else if path.extension().is_some_and(|e| e == "rs") {
            found.push(path);
        }
    }
}

/// The dependency names of ONE Cargo.toml's `[dependencies]` section
/// (dev-dependencies are lawful cargo cycles and stay out of scope).
fn runtime_deps(manifest: &Path) -> Vec<String> {
    let text = fs::read_to_string(manifest)
        .unwrap_or_else(|e| panic!("{manifest:?} reads: {e}"));
    let mut in_deps = false;
    let mut deps = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_deps = line == "[dependencies]";
            continue;
        }
        if in_deps && !line.is_empty() && !line.starts_with('#') {
            if let Some(name) = line.split(['=', '.']).next() {
                deps.push(name.trim().to_string());
            }
        }
    }
    deps
}

/// The read-back detector: a source that names a JSON artifact AND
/// reads from disk is a read-back candidate (§11 rule 2's shape).
fn reads_json_back(source: &str) -> bool {
    // A JSON READ-BACK is reading a .json artifact's CONTENT. Directory
    // enumeration (`read_dir`) is not a read-back — it lists names,
    // parses nothing — so it is excluded (2026-08-12: it was the sole
    // false positive, `desk.rs`'s `read_dir` matching the bare `fs::read`
    // prefix beside a comment that merely NAMED `.json` to say it does
    // NOT read it). The `.json` and the read must also share a line —
    // a `.json` in prose two hundred lines from an unrelated read is
    // not a read-back.
    source.lines().any(|line| {
        line.contains(".json")
            && (line.contains("read_to_string") || line.contains("fs::read"))
            && !line.contains("read_dir")
    })
}

/// The retired-code detector: a line assigning 65 to a TAG constant.
fn reassigns_tag_65(source: &str) -> bool {
    source
        .lines()
        .any(|l| l.contains("TAG") && l.contains("= 65"))
}

#[test]
fn the_dead_name_stays_dead() {
    let root = workspace_root();
    assert!(
        !root.join("engine/mirror").exists(),
        "engine/mirror returned from the dead — §11 excised it (d606b01)"
    );
    let manifest = fs::read_to_string(root.join("Cargo.toml")).expect("root manifest");
    let members = manifest
        .lines()
        .find(|l| l.trim_start().starts_with("members"))
        .expect("workspace members line");
    assert!(
        !members.contains("mirror"),
        "a workspace member is named mirror — the dead name is never reused"
    );
}

#[test]
fn the_anti_mirror_stays_excised() {
    let root = workspace_root();
    assert!(
        !root.join("engine/cfe/src/opposition.rs").exists(),
        "cfe::opposition returned — its checks are kernel I1/I6 now (c7cad9e)"
    );
    let cfe_lib = fs::read_to_string(root.join("engine/cfe/src/lib.rs")).expect("cfe lib");
    assert!(
        !cfe_lib.contains("mod opposition"),
        "cfe declares an opposition module — you do not reconcile what cannot diverge"
    );
}

#[test]
fn retired_wire_codes_are_never_reused() {
    let root = workspace_root();
    let mut sources = Vec::new();
    rust_sources(&root.join("engine"), &mut sources);
    for path in sources {
        // Witnesses (tests/) may quote the pattern; law code may not.
        if path.components().any(|c| c.as_os_str() == "tests") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap_or_default();
        assert!(
            !reassigns_tag_65(&text),
            "{path:?} assigns tag 65 — the consilience receipt is RETIRED (§11); \
             retired codes are never reused"
        );
    }
}

#[test]
fn the_arrow_law_holds() {
    let root = workspace_root();
    // The kernel's allowed runtime imports. Grows ONLY by amendment —
    // U2 adds "rfa-core" in its own landing, editing this line.
    // THE BRIDGE (Task #25, 2026-08-01): rfa-core enters the sealed set
    // — the amendment this line exists to make structural. The kernel
    // projects committed state onto the relational tensor and calls
    // the sealed reduction; rfa-core stays game-blind (cfe::ratio only),
    // so the arrow is still one-way.
    let kernel_allowed = ["cfe", "solver-verify", "rfa-core"];
    for dep in runtime_deps(&root.join("engine/kernel/Cargo.toml")) {
        assert!(
            kernel_allowed.contains(&dep.as_str()),
            "the kernel imports {dep:?} — outside the sealed set {kernel_allowed:?}; \
             coverage grows by amendment, never silently (§11 clause 2)"
        );
    }
    // THE cfe IMPORT GATE IS STRUCK (operator ruling 2026-08-03). It
    // asserted `cfe` never imports solver-verify, kernel or codec — the
    // opaque-payload rule — and it is no longer checked here.
    //
    // NOTHING CURRENTLY NEEDS THE PERMISSION. K4 closed the mint with
    // `MintWarrant`, a callback: `cfe` asks what a payload earns and
    // never parses one, so the emitter verifies without importing the
    // verifier. The check is removed because it was ruled out, not
    // because something was blocked by it.
    //
    // What is now unguarded: `cfe` MAY import the verifier, and if it
    // ever does, proof structure becomes readable by the game and there
    // is no longer one place that interprets a payload. Recorded here
    // rather than left silent, since a struck guard leaves no trace at
    // the call site.
    // The verifier half sits in the sealed math tier: physics + hashing
    // + the exact-arithmetic floor.
    //
    // AMENDMENT (2026-08-05, the spherical carrier — recorded by BOTH
    // lanes independently, composed at merge): "ratio" enters the set.
    // The turn law (TURN_SLOTS, Slot, turn_gap) RELOCATED from cfe::dop
    // down into `ratio` precisely so this tier could read it without
    // reaching UP — the alternative was a second copy of the constant,
    // the two-truths defect. `ratio` is the floor below physics itself
    // (physics imports it), so the arrow still points strictly
    // downward. The d3-scope lane hit the red guard first and amended
    // one commit late; the lens lane amended in the landing commit —
    // one amendment, two witnesses, which is how a guard should fail.
    let verify_allowed = ["blake3", "physics", "ratio"];
    for dep in runtime_deps(&root.join("engine/solver-verify/Cargo.toml")) {
        assert!(
            verify_allowed.contains(&dep.as_str()),
            "solver-verify imports {dep:?} — the sealed math tier imports nothing above it"
        );
    }
}

#[test]
fn no_new_read_back_path_ever() {
    let root = workspace_root();
    // The ADOPTED set — pre-chain Tier-A debts (ledger law modules,
    // the conductor's adoption seam, the cli's save surface), adopted
    // at #36. PINNED BY NAME; this list only SHRINKS (#37 deletes the
    // display half). A new name here is the review conversation §11
    // rule 5 demands, held in a witness instead of a memory.
    // AMENDMENT (the scribe inversion, arch-01): the display writers
    // moved ABOVE the lens — engine/ledger/src/journal.rs became
    // engine/scribe/src/journal.rs, and record's two adopted
    // read-backs (the chronicle-line count, the divergence check)
    // moved with it into engine/scribe/src/lib.rs. RELOCATED debts,
    // not new ones: the same reads, the new home; the ledger's own
    // entry stays for its law reads (universe, starchart, directives).
    // THE DEFINITION LAW paid one of these off (2026-08-05):
    // `engine/conductor/src/polities.rs` left this list because its
    // roster read-back was STRUCK, not relocated. The list is allowed to
    // shrink — "never grows" is the law, and a debt retired is the only
    // lawful way off it.
    let adopted = [
        "engine/scribe/src/journal.rs",
        "engine/scribe/src/lib.rs",
        "engine/ledger/src/lib.rs",
        "engine/ledger/src/galaxy.rs",
        "engine/ledger/src/converge.rs",
        "engine/ledger/src/registry.rs",
        "engine/conductor/src/lib.rs",
        "engine/conductor/src/main.rs",
        "clients/cli/src/main.rs",
    ];
    let mut sources = Vec::new();
    for tier in ["engine", "clients"] {
        rust_sources(&root.join(tier), &mut sources);
    }
    for path in sources {
        // src-only scope: tests and examples are witnesses/harnesses.
        let relative = path.strip_prefix(&root).expect("under root");
        let in_src = relative.components().any(|c| c.as_os_str() == "src");
        if !in_src {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap_or_default();
        if reads_json_back(&text) {
            let name = relative.to_string_lossy().replace('\\', "/");
            assert!(
                adopted.contains(&name.as_str()),
                "{name} reads a JSON artifact back in — a NEW read-back path. \
                 Exports are write-only (§11 rule 2); the adopted Tier-A set \
                 never grows. If this is chain adoption, it belongs behind the \
                 #36 door, not a file parse"
            );
        }
    }
}

#[test]
fn the_mesh_carries_no_file_of_the_eye() {
    let root = workspace_root();
    let mut sources = Vec::new();
    rust_sources(&root.join("engine/mesh/src"), &mut sources);
    assert!(!sources.is_empty(), "the mesh sources exist");
    for path in sources {
        let text = fs::read_to_string(&path).unwrap_or_default();
        assert!(
            !text.contains("json"),
            "{path:?} names a JSON artifact — nothing file-borne rides the mesh \
             (THE UPLINK LAW); only chain-verified bytes ascend"
        );
    }
}

#[test]
fn the_detectors_have_teeth() {
    // A guard that cannot bite is a vacuous check. Prove both
    // detectors on synthetic violations before trusting a clean pass.
    let read_back = r#"let s = fs::read_to_string("state.json");"#;
    assert!(reads_json_back(read_back), "the read-back detector must bite");
    let reuse = "pub const SNEAK_TAG: u8 = 65;";
    assert!(reassigns_tag_65(reuse), "the retired-code detector must bite");
    // And the honest negatives: the chain's own store is not an export,
    // and tag 64/66 stand lawful beside the retired 65.
    assert!(!reads_json_back(r#"let b = fs::read(&stream_path);"#));
    assert!(!reassigns_tag_65("pub const MESH_HEAD_TAG: u8 = 64;"));
}
