//! THE STERILE BOUNDARY, held by a standing test (`19_LENS` §1a,
//! operator ruling 2026-08-02).
//!
//! Rule 5 of the lens lane is *the lens is deletable*: removing this
//! lane must cost the system nothing but visibility. As prose that is
//! worth nothing — prose failed twice in one day in this tree, which is
//! what `LAW 8` and `constitution_lint` exist to say. This is the
//! mechanical form.
//!
//! **THE INVARIANT: the lens is never UPSTREAM of the judge.**
//!
//! ```text
//!     W \ L  ≡  W          remove the lens, the world is unchanged
//! ```
//!
//! The boundary is directional, not a blanket ban on depending. A
//! CLIENT may render — that is what a client is for, and `16_UNIT` §2
//! names the conductor a client of units. What may never happen is a
//! crate the JUDGE depends on reaching for a lens, because from that
//! moment a reading has entered the operational dependency graph and
//! deleting it would break a rule rather than dim a light.
//!
//! The excised dashboard crate (`00_CHARTER` §3a) did exactly that, one
//! useful change at a time, and no rule existed to stop it. This test
//! is that rule.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The lane's crates — what must stay deletable.
const LENS_CRATES: [&str; 2] = ["lens", "scribe"];

/// THE SEALED CORE — the judge, the math it judges with, and everything
/// they stand on. No crate here may depend on a lens crate in any
/// capacity, dev-dependencies included: a law test that renders is a
/// law test that cannot run once the lens is gone.
const CORE: [&str; 8] = [
    "engine/kernel",
    "engine/rfa-core",
    "engine/cfe",
    "engine/codec",
    "engine/mesh",
    "engine/physics",
    "engine/solver",
    "engine/solver-verify",
];

fn workspace_root() -> PathBuf {
    // engine/lens/tests/ -> engine/lens -> engine -> root
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("engine/lens sits two levels under the workspace root")
        .to_path_buf()
}

/// Every crate named in a `[dependencies]`, `[dev-dependencies]` or
/// `[build-dependencies]` table, by the name on the left of the `=`.
/// Comment lines are ignored — the ledger carries a PROSE reference to
/// the lens in a comment, and a comment is not an edge.
fn declared_dependencies(manifest: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut in_deps = false;
    for raw in manifest.lines() {
        let line = raw.trim();
        if line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_deps = line.contains("dependencies");
            continue;
        }
        if !in_deps {
            continue;
        }
        if let Some((name, _)) = line.split_once('=') {
            let name = name.trim().trim_matches('"');
            if !name.is_empty() {
                found.insert(name.to_string());
            }
        }
    }
    found
}

/// **THE SEALED CORE IS LENS-FREE.** The judge and the sealed math may
/// not reach for a reading, in any dependency table.
#[test]
fn no_sealed_crate_depends_on_a_lens() {
    let root = workspace_root();
    let mut leaks = Vec::new();

    for crate_dir in CORE {
        let manifest_path = root.join(crate_dir).join("Cargo.toml");
        let manifest = std::fs::read_to_string(&manifest_path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", manifest_path.display()));
        let deps = declared_dependencies(&manifest);
        for lens in LENS_CRATES {
            if deps.contains(lens) {
                leaks.push(format!("  {crate_dir} depends on `{lens}`"));
            }
        }
    }

    assert!(
        leaks.is_empty(),
        "A LENS HAS LEAKED UPSTREAM OF THE JUDGE.\n\
         The sealed core must build and be judged with the lens crates absent;\n\
         `W \\ L ≡ W` (19_LENS §1a). A reading that the judge depends on is no\n\
         longer a reading — deleting it would break a rule rather than dim a\n\
         light, which is precisely how the excised dashboard crate became\n\
         load-bearing (00_CHARTER §3a).\n{}",
        leaks.join("\n")
    );
}

/// **THE LEDGER STAYS CLEAN AT RUNTIME.** `#37` moves the ledger's
/// display half into the lens; until then the ledger may build fixtures
/// through the display writers in DEV ONLY. A runtime edge would put a
/// renderer under the law in fact.
#[test]
fn the_ledger_never_depends_on_a_lens_at_runtime() {
    let root = workspace_root();
    let manifest = std::fs::read_to_string(root.join("engine/ledger/Cargo.toml"))
        .expect("the ledger manifest reads");

    // Runtime table only — everything before the first `[dev-` or
    // `[build-` section header.
    let runtime: String = manifest
        .lines()
        .take_while(|l| {
            let t = l.trim();
            !(t.starts_with("[dev-") || t.starts_with("[build-"))
        })
        .collect::<Vec<_>>()
        .join("\n");

    let deps = declared_dependencies(&runtime);
    for lens in LENS_CRATES {
        assert!(
            !deps.contains(lens),
            "the ledger took a RUNTIME dependency on `{lens}` — the display \
             half must stay above the law, not under it (#37, 00_CHARTER §11 step 4)"
        );
    }
}

/// **THE DIRECTION IS THE LAW, and this records where the one lawful
/// edge lives.** The conductor is a CLIENT of units (`16_UNIT` §2), and
/// a client renders — that is what it is for. Its `scribe` edge is
/// therefore lawful, and this test PINS ITS SHAPE rather than its
/// absence: every call is made for effect and its result discarded, so
/// no conductor decision is taken on what a lens returned.
///
/// If a conductor branch ever consults a lens RETURN VALUE, this
/// assertion is where it should be caught — a client that renders is
/// lawful; a client that DECIDES on a rendering is a mirror.
///
/// **A CONSTANT IS NOT A READING, and the distinction is the whole
/// precision of this test.** The conductor branches on
/// `scribe::journal::YEAR_TICKS` to decide WHEN to write a journal.
/// That is a cadence — a fixed number that says how often to render —
/// and branching on it decides only whether a light comes on. A
/// READING is what a lens COMPUTES FROM WORLD STATE, and a branch on
/// one of those puts the world's behaviour downstream of an
/// observation. Only the latter is the mirror fault.
///
/// The rule this encodes: **a lens CALL result may never be bound or
/// branched on.** A lens constant may be, and the coupling it creates
/// is recorded in `19_LENS` rather than refused.
#[test]
fn the_conductor_renders_but_never_decides_on_a_reading() {
    let root = workspace_root();
    let mut deciding = Vec::new();

    for file in ["engine/conductor/src/lib.rs", "engine/conductor/src/main.rs"] {
        let src = std::fs::read_to_string(root.join(file))
            .unwrap_or_else(|e| panic!("cannot read {file}: {e}"));
        for (n, line) in src.lines().enumerate() {
            let t = line.trim();
            let Some(at) = t.find("scribe::").or_else(|| t.find("lens::")) else {
                continue;
            };
            // A CALL invokes the lens; a constant path does not. The
            // fault is consuming what a lens COMPUTED, so only a call
            // can commit it.
            let is_call = t[at..].split_once('(').is_some_and(|(head, _)| !head.contains(' '));
            if !is_call {
                continue; // a constant path — cadence, not a reading
            }
            let discarded = t.starts_with("let _ =");
            let bound = t.starts_with("let ") && !discarded;
            let branched = t.starts_with("if ") || t.starts_with("match ");
            if bound || branched {
                deciding.push(format!("  {file}:{}  {t}", n + 1));
            }
        }
    }

    assert!(
        deciding.is_empty(),
        "A CLIENT DECIDED ON A READING.\n\
         Rendering is lawful for a client; branching on what a lens RETURNED is\n\
         not — that is the reading becoming load-bearing (19_LENS §1a). Every\n\
         lens call from the conductor must be made for effect and discarded.\n{}",
        deciding.join("\n")
    );
}
