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

/// Every `.rs` file in the lane's own source, with its path.
fn lane_sources(root: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for crate_name in LENS_CRATES {
        let dir = root.join("engine").join(crate_name).join("src");
        let mut stack = vec![dir];
        while let Some(at) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&at) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    let text = std::fs::read_to_string(&path)
                        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
                    let rel = path
                        .strip_prefix(root)
                        .unwrap_or(&path)
                        .display()
                        .to_string();
                    out.push((rel, text));
                }
            }
        }
    }
    assert!(!out.is_empty(), "the lane has sources to check");
    out
}

/// Strip `//`-comments and doc comments so PROSE about a banned
/// construct never trips the guard that bans it. This file's own
/// vocabulary would otherwise fail its own tests.
fn code_only(text: &str) -> String {
    text.lines()
        .map(|l| match l.trim_start().starts_with("//") {
            true => "",
            false => l.split("//").next().unwrap_or(""),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// **RULE 2 — RENDER ON DEMAND MEANS COMPUTE LESS, NEVER STORE MORE.**
///
/// The sharpest edge in the lane (`19_LENS` §1a). The obvious way to
/// "render only when needed" is a cache — compute once, keep it, serve
/// it — and a cache IS the second image, arriving as an optimization
/// exactly the way the excised dashboard did.
///
/// A reading type is taken by `&self`, so the ONLY way to memoise
/// behind one is interior mutability or a global. That makes the fault
/// mechanically detectable rather than a matter of review: ban the
/// mechanism and the optimization cannot be smuggled in as a
/// convenience.
///
/// Today the lane contains none of these, so this is a guard against
/// introduction, which is when it would actually be argued for.
#[test]
fn no_reading_may_memoise_behind_a_shared_reference() {
    let root = workspace_root();
    // The interior-mutability toolbox, plus the global-state forms.
    const VECTORS: [&str; 9] = [
        "RefCell",
        "UnsafeCell",
        "Mutex",
        "RwLock",
        "OnceCell",
        "OnceLock",
        "LazyLock",
        "lazy_static",
        "static mut",
    ];
    let mut found = Vec::new();

    for (path, text) in lane_sources(&root) {
        let code = code_only(&text);
        for (n, line) in code.lines().enumerate() {
            for vector in VECTORS {
                if line.contains(vector) {
                    found.push(format!("  {path}:{}  {}", n + 1, line.trim()));
                }
            }
        }
    }

    assert!(
        found.is_empty(),
        "A CACHE VECTOR ENTERED THE LENS.\n\
         Interior mutability behind a `&self` reading is how memoisation gets\n\
         added, and a cache is the SECOND IMAGE that 19_LENS §1a rule 2 and\n\
         00_CHARTER §3a forbid: from that moment two things exist that can\n\
         disagree. Render-on-demand means computing LESS (not descending a peel\n\
         nobody looks through), never storing more.\n{}",
        found.join("\n")
    );
}

/// **RULE 1 — DERIVE, NEVER RETAIN.**
///
/// Every reading is a pure function of committed state, computed when
/// asked and discarded after. No lens type holds a field that is a COPY
/// of world state.
///
/// The mechanical form: a lens type may BORROW world state and may
/// never OWN it. `Observation` holds `&'a Delta`, `&'a Support` and
/// `Option<&'a ColonyOrb>` — borrowed, so there is nowhere to put a
/// copy and nothing that can go stale. An owned `World` or `Committed`
/// on a reading type is a private snapshot of the chain, which is a
/// second timeline whatever it is called.
///
/// Derived VALUES are not world state and are not caught: a `Ratio`
/// weight, a `Degree`, a materialised `View` are all things the lens
/// computed rather than copied.
#[test]
fn no_lens_type_owns_a_copy_of_world_state() {
    let root = workspace_root();
    // The state carriers. Owning one of these on a reading type is a
    // snapshot; borrowing one is the lawful shape.
    const STATE: [&str; 5] = ["World", "Committed", "Chronicle", "ColonyOrb", "Receipt"];
    let mut owned = Vec::new();

    for (path, text) in lane_sources(&root) {
        let code = code_only(&text);
        let mut in_struct = false;
        for (n, line) in code.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("pub struct") || t.starts_with("struct") {
                in_struct = !t.ends_with(';');
                continue;
            }
            if in_struct && t.starts_with('}') {
                in_struct = false;
                continue;
            }
            if !in_struct {
                continue;
            }
            // A field line: `name: Type,`. A reference is lawful.
            let Some((_, ty)) = t.split_once(':') else {
                continue;
            };
            if ty.contains('&') {
                continue;
            }
            for state in STATE {
                // Word-boundary-ish: the type name followed by a
                // non-identifier character, so `WorldSpan` is not `World`.
                if ty.split(|c: char| !c.is_alphanumeric() && c != '_').any(|w| w == state) {
                    owned.push(format!("  {path}:{}  {t}", n + 1));
                }
            }
        }
    }

    assert!(
        owned.is_empty(),
        "A LENS TYPE OWNS A COPY OF WORLD STATE.\n\
         A reading BORROWS the chain and never keeps it (19_LENS §1a rule 1).\n\
         An owned snapshot on a reading type is a private timeline: there are\n\
         then two records that can disagree, and every reconciler after it is a\n\
         tool built to manage a problem that should not exist.\n{}",
        owned.join("\n")
    );
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
