//! THE CONSTITUTIONAL LINT — the law held to the law, mechanically.
//!
//! `00_CHARTER` gates CODE three ways: §1 wants a named consumer and
//! test vectors, §6 wants red-first landings, §11 rule 5 is mechanized
//! by `recurrence_guard.rs`. **Nothing gated the constitution itself.**
//! Prose about the law needed no consumer, no witness, and no
//! obligation to compute — so it became the one unchecked input vector,
//! and ornamental symbols accreted in it until they were mistaken for
//! active constraints.
//!
//! Every symbol struck on 2026-08-01 lived there and computed nothing.
//! The test that removed them — Article III LAW 8, *a term that computes
//! is kept; a term that only resonates is excised* — was written AFTER
//! them, which is itself the proof that no such test existed. Each was
//! caught by a person reading carefully, one at a time. Vigilance is
//! `O(attention)` and degrades under throughput; a standing test is
//! `O(1)` and does not. This file is the standing test.
//!
//! What it pins:
//!
//! 1. EVERY CITATION RESOLVES — a `crate::item` reference in a law
//!    document must name something that exists in `engine/`. This is
//!    `EQUATIONS` §0.3 chapter six mechanized: *a row whose
//!    implementation has been deleted is not stale, it is FALSE.* It
//!    would have caught `cfe::opposition` (a law document prescribing a
//!    module its own test suite forbade from existing) and `ext_x`
//!    (`17_CAPACITY` §7 claiming a function was written when it was not).
//! 2. THE STRUCK STAY STRUCK — a symbol the constitution struck may
//!    appear ONLY on a line that says so. Records are required (*recorded,
//!    never silently dropped*); live use is not. This is the vocabulary
//!    twin of `recurrence_guard`'s dead-name test, and it would have
//!    caught `EQUATIONS` D4 still computing with `Ξ` after `18` §3
//!    struck the meter.
//! 3. THE DETECTORS HAVE TEETH — both are proven on synthetic
//!    violations before a clean pass is trusted.
//!
//! SCOPE, stated honestly. This does NOT verify that every mathematical
//! symbol maps to an AST node — that needs a symbol registry the tree
//! does not have, and a lint that floods false positives is a lint that
//! gets disabled. It verifies the two things that actually went wrong:
//! citations outliving their referents, and struck vocabulary returning
//! as live law. The registry version is a larger landing; this one bites
//! today.
//!
//! File I/O lives HERE, in a witness — the kernel itself stays pure.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("engine/kernel sits two levels under the root")
        .to_path_buf()
}

/// The governing documents: the law, not the world-book.
fn law_documents(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(root.join("docs")) {
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x == "md") {
                out.push(p);
            }
        }
    }
    out.push(root.join("engine/kernel/CHARTER.md"));
    out.sort();
    out
}

/// Every item name declared anywhere in `engine/` — functions (pub or
/// not; the kernel's invariants are private by design), types, modules,
/// consts, traits, and enum variants. Generous by intent: over-capturing
/// only weakens the test, while under-capturing produces false alarms,
/// and a lint that cries wolf is a lint that gets switched off.
fn declared_items(root: &Path) -> HashSet<String> {
    let mut items = HashSet::new();
    let mut stack = vec![root.join("engine")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                if p.file_name().is_some_and(|n| n == "target") {
                    continue;
                }
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                let text = fs::read_to_string(&p).unwrap_or_default();
                for line in text.lines() {
                    let t = line.trim();
                    for kw in ["fn ", "struct ", "enum ", "const ", "type ", "mod ", "trait "] {
                        if let Some(rest) = t.strip_prefix(kw).or_else(|| {
                            t.strip_prefix("pub ").and_then(|r| r.strip_prefix(kw))
                        }) {
                            let name: String = rest
                                .chars()
                                .take_while(|c| c.is_alphanumeric() || *c == '_')
                                .collect();
                            if !name.is_empty() {
                                items.insert(name);
                            }
                        }
                    }
                    // Enum variants: an indented CamelCase head. `=` is
                    // in the follow set because a variant may carry an
                    // explicit discriminant — `Sovereign = 1,` — and
                    // without it every discriminant-form variant was
                    // INVISIBLE to this scan, so a citation to one
                    // false-failed as unresolved. `ZoneKind::Smelter = 3`
                    // sat in the same blind spot; `Office::Sovereign` is
                    // just the first citation that reached for one.
                    if line.starts_with("    ") {
                        let head: String = t
                            .chars()
                            .take_while(|c| c.is_alphanumeric() || *c == '_')
                            .collect();
                        if head.chars().next().is_some_and(char::is_uppercase)
                            && t[head.len()..].trim_start().starts_with(['{', ',', '(', '='])
                        {
                            items.insert(head);
                        }
                    }
                }
            }
        }
    }
    items
}

/// Paragraph indices whose text records a strike. Scope is the
/// PARAGRAPH, never the line: these documents state a strike in a
/// sentence and then discuss it for several lines, so a line-scoped
/// exemption would flag a document's own strike record — the one thing
/// the discipline REQUIRES ("recorded, never silently dropped").
fn struck_paragraph_lines(text: &str) -> HashSet<usize> {
    let mut exempt = HashSet::new();
    let mut start = 0usize;
    let mut marked = false;
    let lines: Vec<&str> = text.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            if marked {
                exempt.extend(start..i);
            }
            start = i + 1;
            marked = false;
        } else if records_a_strike(line) {
            marked = true;
        }
    }
    if marked {
        exempt.extend(start..lines.len());
    }
    exempt
}

/// A line that RECORDS a strike rather than asserting live law. The
/// tree's own discipline requires these ("recorded, never silently
/// dropped"), so they are exempt by construction — what is barred is
/// live use, never the record of removal.
fn records_a_strike(line: &str) -> bool {
    const MARKERS: [&str; 17] = [
        "STRUCK", "struck", "EXCISED", "excised", "DEPRECATED", "deprecated", "RETIRED",
        // `retired` lowercase was MISSING while every other marker
        // carried both cases — found by this file's own vocabulary
        // lint flagging the sentence that describes the retirement.
        "retired", "formerly", "REMOVED", "withdrawn",
        // A CORRECTION is a record too. `Ω ≡ V` could only be retired by
        // paragraphs that QUOTE it while retiring it, exactly as a strike
        // record quotes the symbol it strikes.
        "CORRECTED", "corrected", "AMENDED", "amended", "RENOTATED", "renotated",
    ];
    MARKERS.iter().any(|m| line.contains(m))
}

/// A RELAY LINE — a blockquote. These documents use `>` for one thing:
/// recording an operator ruling verbatim, "as given" (`18` §1). A relay
/// is a record of what was SAID, never an assertion of law, so a struck
/// symbol inside one is a quotation — exactly the case the strike
/// records are already exempt for. `18` §1 has carried `Ω(D)` and
/// `D(V)+V(D)` since the day it was relayed, and editing an operator's
/// words to satisfy a lint would be the worse defect.
///
/// The scope is deliberately narrow: this exempts QUOTATION, not
/// commentary. Prose about a relay sits outside the `>` and is judged
/// like any other law.
fn is_relay(line: &str) -> bool {
    line.trim_start().starts_with('>')
}

/// Roots outside this workspace: the standard library and vendored
/// crates resolve elsewhere and are not this lint's business.
fn is_foreign_root(path: &str) -> bool {
    const FOREIGN: [&str; 9] = [
        "std", "fs", "core", "alloc", "serde", "blake3", "num_traits", "num_bigint",
        "num_integer",
    ];
    path.split("::")
        .next()
        .is_some_and(|r| FOREIGN.contains(&r))
}

/// Extract backticked `path::item` citations from one line.
fn citations(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    for chunk in line.split('`').skip(1).step_by(2) {
        let c = chunk.trim();
        if !c.contains("::") || c.contains(' ') || c.contains('(') {
            continue;
        }
        if c.split("::").all(|seg| {
            !seg.is_empty() && seg.chars().all(|ch| ch.is_alphanumeric() || ch == '_')
        }) {
            out.push(c.to_string());
        }
    }
    out
}

#[test]
fn every_citation_in_the_law_resolves() {
    let root = workspace_root();
    let items = declared_items(&root);
    assert!(items.len() > 100, "the item scan found the engine");

    // THE ADOPTED SET — pre-existing unresolved citations, PINNED BY
    // NAME. This list only ever SHRINKS. A new unresolved citation
    // fails here, named, with its file and line.
    //
    // `mirror::Mirror` and friends: `03_SUBSTRATE_CONTRACT.md` §7/§8
    // still document the deleted `engine/mirror` crate as a live
    // primitive. Known, reported, and owned by the substrate node —
    // it needs an operator relay to fix, not a silent edit from here.
    // `crate::fn` is EQUATIONS §0.1's own placeholder for "the sealed
    // implementation", not a citation.
    // `cfe::opposition` in KERNEL_RELAY: the reasoning narrative that
    // ARGUED the excision, kept as the record of why. Another lane.
    let adopted: HashSet<&str> = ["mirror::Mirror", "crate::fn", "cfe::opposition"]
        .into_iter()
        .collect();

    let mut unresolved = Vec::new();
    for doc in law_documents(&root) {
        let text = fs::read_to_string(&doc).unwrap_or_default();
        let exempt = struck_paragraph_lines(&text);
        for (n, line) in text.lines().enumerate() {
            if exempt.contains(&n) {
                continue;
            }
            for c in citations(line) {
                if is_foreign_root(&c) || adopted.contains(c.as_str()) {
                    continue;
                }
                let tail = c.rsplit("::").next().unwrap_or(&c);
                if !items.contains(tail) {
                    let rel = doc.strip_prefix(&root).unwrap_or(&doc).display();
                    unresolved.push(format!("{rel}:{}  `{c}`", n + 1));
                }
            }
        }
    }

    assert!(
        unresolved.is_empty(),
        "a law document cites something that does not exist in engine/.\n\
         A citation is a CLAIM, and a row whose implementation has been \
         deleted is not stale — it is FALSE (EQUATIONS §0.3 ch.6).\n\
         Either the code is owed, or the line must record the strike:\n  {}",
        unresolved.join("\n  ")
    );
}

#[test]
fn the_struck_symbols_stay_struck() {
    let root = workspace_root();

    // Vocabulary struck by operator ruling. Each may appear ONLY on a
    // line that records its removal. Live use is the defect this
    // catches — EQUATIONS D4 went on computing with `Ξ` after `18` §3
    // struck the meter, and nothing noticed for a day.
    // `T_k` is deliberately ABSENT. The struck one is Article III's
    // topicality score; PhiK's `I^(5)` stamp is ALSO written `T_k` and
    // is live. A token lint cannot separate them, and guarding an
    // ambiguous token would flag correct law — the false-positive that
    // gets a lint disabled. Ambiguous vocabulary is out of scope here
    // and belongs to a symbol registry, if one is ever built.
    const STRUCK: [(&str, &str); 9] = [
        ("Ξ", "the cosmetic meter, struck by 18 §3"),
        ("Ω ≡ V", "the room identified with the substance, struck 2026-08-02 (17 §3)"),
        ("Ω  ≡  V", "the room identified with the substance, struck 2026-08-02 (17 §3)"),
        ("Ω(D)", "the totality made a function of one office, retired 2026-08-02 (17 §3)"),
        ("S⁺_out", "twin-singularity esoterica, struck by 18 §2d"),
        ("S⁺_hol", "twin-singularity esoterica, struck by 18 §2d"),
        ("ζ∇V", "the damped-gradient term, struck 2026-08-01"),
        ("∇V", "V enclosed in a nabla, struck 2026-08-01 (18 §2b)"),
        ("∂V", "V enclosed in a boundary, struck 2026-08-01 (18 §2b)"),
    ];

    // ADOPTED, pinned by file:line. Historical records and recorded
    // REJECTED proposals in `17_CAPACITY`'s audit sections, plus
    // `KERNEL_RELAY`'s reasoning narrative. Both are another lane's
    // territory and both are records rather than assertions — but they
    // lack a marker word, so they are pinned here rather than edited
    // across a seam. THIS LIST ONLY SHRINKS.
    let adopted_lines: HashSet<(&str, usize)> = [
        ("docs/17_CAPACITY.md", 399),
        ("docs/17_CAPACITY.md", 473),
        ("docs/17_CAPACITY.md", 772),
        ("docs/17_CAPACITY.md", 847),
        ("docs/17_CAPACITY.md", 869),
    ]
    .into_iter()
    .collect();

    let mut live = Vec::new();
    for doc in law_documents(&root) {
        let text = fs::read_to_string(&doc).unwrap_or_default();
        let exempt = struck_paragraph_lines(&text);
        for (n, line) in text.lines().enumerate() {
            if exempt.contains(&n) || is_relay(line) {
                continue;
            }
            for (sym, why) in STRUCK {
                if line.contains(sym) {
                    let rel = doc.strip_prefix(&root).unwrap_or(&doc).display();
                    if adopted_lines.contains(&(rel.to_string().as_str(), n + 1)) {
                        continue;
                    }
                    live.push(format!("{rel}:{}  `{sym}` — {why}", n + 1));
                }
            }
        }
    }

    assert!(
        live.is_empty(),
        "a struck symbol is used as LIVE LAW.\n\
         Records of a strike are required and exempt; use is not. A \
         struck symbol left standing in the corpus is still a symbol in \
         the corpus:\n  {}",
        live.join("\n  ")
    );
}

/// THE LENS LAW as a standing test (`00_CHARTER` §3a).
///
/// `recurrence_guard` keeps the mirror CRATE dead and the reconciler
/// excised. Neither watches the specifications — which is exactly how
/// `03_SUBSTRATE_CONTRACT` went on documenting `mirror::Mirror` as a
/// live primitive for two days after the crate was deleted. The code
/// was guarded; the prose was not. This closes that half.
///
/// A LENS OFFERS SOLUTIONS: you turn it ON something to resolve it, and
/// it returns a reading that did not exist before. A MIRROR RETURNS THE
/// SELF: it points inward and gives back what was already there. The
/// copy a mirror must hold is the consequence of that, not its
/// definition. The name may appear ONLY where a paragraph records its
/// removal.
#[test]
fn no_mirror_in_the_law() {
    let root = workspace_root();
    let mut live = Vec::new();
    for doc in law_documents(&root) {
        // BUILD_ORDER and the audit are the LANDING RECORD — they say
        // what happened under the names it happened under, and
        // rewriting history is not excision. The law documents are the
        // scope: what the system IS, not what it did.
        let name = doc.file_name().unwrap_or_default().to_string_lossy().to_string();
        // 00_CHARTER carries §3a itself — the article defining the ban
        // must be able to name what it bans, and §11 keeps the history
        // of why the rule exists. BUILD_ORDER and the audit are the
        // LANDING RECORD: they say what happened under the names it
        // happened under, and rewriting history is not excision.
        if name == "00_CHARTER.md"
            || name == "BUILD_ORDER.md"
            || name.starts_with("AUDIT_")
            || name == "KERNEL_RELAY.md"
            || name == "HANDOFF.md"
            || name == "PICKUP.md"
            || name.ends_with("_DIRECTIVES.md")
        {
            continue;
        }
        let text = fs::read_to_string(&doc).unwrap_or_default();
        let exempt = struck_paragraph_lines(&text);
        for (n, line) in text.lines().enumerate() {
            if exempt.contains(&n) {
                continue;
            }
            // STRUCTURE, never the verb. "X mirrors Y" and "mirroring"
            // are ordinary English and hold nothing; what §3a bans is a
            // mirror that EXISTS — a crate, a module, a type, or "the
            // mirror" as a thing in the system. `TileMirror` / FlipH
            // are lawful geometry: a mirrored GRID is a transform, not
            // a second truth.
            let structure = line.contains("engine/mirror")
                || line.contains("mirror::")
                || line.contains("the mirror")
                || (line.contains("Mirror") && !line.contains("TileMirror"));
            if structure {
                let rel = doc.strip_prefix(&root).unwrap_or(&doc).display();
                live.push(format!("{rel}:{}  {}", n + 1, line.trim()));
            }
        }
    }
    assert!(
        live.is_empty(),
        "a law document names a MIRROR as live (00_CHARTER §3a, THE LENS LAW).\n\
         A lens is turned ON something and resolves it; a mirror is \
         turned on the SELF and returns what was already there. Use the \
         lens, or record the strike:\n  {}",
        live.join("\n  ")
    );
}

#[test]
fn the_detectors_have_teeth() {
    // A guard that cannot bite is the vacuous check this file exists to
    // kill. Prove every detector on synthetic input first.
    assert_eq!(
        citations("the sealed `rfa_core::coboundary_reduce` performs it"),
        vec!["rfa_core::coboundary_reduce".to_string()],
        "the citation extractor must find a real citation"
    );
    assert!(
        citations("see `12_UNION` §2c and the four C's").is_empty(),
        "a section reference is not a crate path"
    );
    assert!(
        records_a_strike("**`Ξ` is STRUCK** — the cosmetic meter"),
        "a strike record must be recognised and exempted"
    );
    assert!(
        !records_a_strike("the loop trap is Ξ(w) = deeds(w) / (1 + ΔD(w))"),
        "live use must NOT be mistaken for a record"
    );
    // The paragraph scope: a marker exempts its whole paragraph, and
    // NOT the next one.
    let doc = "`Ξ` is STRUCK here\nand discussed on this line too\n\nbut Ξ used here is live\n";
    let ex = struck_paragraph_lines(doc);
    assert!(ex.contains(&0) && ex.contains(&1), "the strike paragraph is exempt whole");
    assert!(!ex.contains(&3), "the NEXT paragraph is not exempt");

    // THE LENS LAW detector: structures bite, verbs do not.
    let bites = |l: &str| {
        l.contains("engine/mirror")
            || l.contains("mirror::")
            || l.contains("the mirror")
            || (l.contains("Mirror") && !l.contains("TileMirror"))
    };
    assert!(bites("- **Primitive:** `mirror::Mirror` (`record`, ...)"), "a mirror STRUCTURE must bite");
    assert!(bites("the mirror is a REFLECTION, never an authority"), "'the mirror' as a thing must bite");
    assert!(!bites("external tokens mirror immutable facts"), "the VERB holds nothing and must pass");
    assert!(!bites("`TileMirror` / FlipH — a mirrored GRID is a transform"), "grid geometry is lawful");

    assert!(is_foreign_root("std::collections::HashSet"), "std is foreign");
    assert!(!is_foreign_root("rfa_core::Delta"), "our crates are not foreign");

    // And the honest negatives: the live operators must never be
    // mistaken for the struck ones they replaced.
    let live_line = "`∂Δ` is the boundary of the relation, `∮` its charge";
    assert!(
        !live_line.contains("∂V") && !live_line.contains("∇V"),
        "∂Δ is LIVE and must not collide with the struck ∂V / ∇V"
    );
}

/// THE RETIRED VOCABULARY STAYS RETIRED — over CODE, not only law.
///
/// `the_struck_symbols_stay_struck` reads `law_documents()`, which is
/// `docs/*.md` plus the kernel charter. It has never opened a `.rs`
/// file. So a ruling could retire a word in the law and the code would
/// never hear it, which is exactly what happened:
///
/// `11_AEON` §3 retired "Daughter" on 2026-07-30, giving the reason in
/// full — *it imported terrestrial unary budding (one parent, mutate
/// one axis) — half a model with no second half*. Three days later the
/// word still stood in 24 files across `cfe`, `kernel`, `conductor`,
/// `scribe`, the CLI and six doc files, and a `conductor_pins`
/// assertion still REQUIRED the retired model (`mutations == 1`).
/// Nothing was watching, because the only watcher read prose.
///
/// **The word was never carrying gender as decoration.** It carried a
/// REPRODUCTIVE MODEL — one parent, ownership, inheritance without
/// earning — and this engine refuses all three: the cleave is
/// peer-to-peer with no parent and no child (`11` §3), lineage records
/// LINE membership rather than parenthood, and a successor inherits the
/// chronicle and not the tree (`17` §3c). The vocabulary was smuggling
/// in the model the law had already refused.
#[test]
fn the_retired_vocabulary_stays_retired() {
    let root = workspace_root();

    // Retired by ruling, with the ruling that retired each.
    //
    // NOUNS ONLY. Gendered PRONOUNS are deliberately absent: a token
    // lint cannot separate "she" naming a system office from "she"
    // naming a real person in a relay, and guarding an ambiguous token
    // would flag correct text — the false positive that gets a lint
    // switched off (the same reasoning `T_k` is absent from STRUCK).
    // The offices were degendered by hand: V was "she", C was "he".
    const RETIRED: [(&str, &str); 14] = [
        ("daughter", "kin word for a charted colony / branch"),
        ("Daughter", "kin word for a charted colony / branch"),
        ("mother", "kin word for a source colony"),
        ("Mother", "kin word for a source colony"),
        ("speciation", "unary budding by another name"),
        ("Speciation", "unary budding by another name"),
        ("sib\u{6c}ing", "kin word for a counterpart"),
        ("Sib\u{6c}ing", "kin word for a counterpart"),
        ("ances\u{74}ry", "kin word for provenance"),
        ("Ances\u{74}ry", "kin word for provenance"),
        ("kins\u{68}ip", "the frame itself: this engine has no kin, only lines"),
        ("Kins\u{68}ip", "the frame itself: this engine has no kin, only lines"),
        ("fami\u{6c}y tree", "a line is a graph, not a tree of kin"),
        ("fami\u{6c}y-tree", "a line is a graph, not a tree of kin"),
    ];

    let mut live = Vec::new();
    let mut scanned = 0usize;
    let mut stack = vec![root.join("engine"), root.join("clients"), root.join("docs")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n == "target") {
                    continue;
                }
                stack.push(path);
                continue;
            }
            let is_source = path.extension().is_some_and(|x| x == "rs" || x == "md");
            if !is_source {
                continue;
            }
            let text = fs::read_to_string(&path).unwrap_or_default();
            scanned += 1;
            let exempt = struck_paragraph_lines(&text);
            for (n, line) in text.lines().enumerate() {
                // A record of the retirement, or an operator's own words,
                // may name the word — use may not.
                if exempt.contains(&n) || is_relay(line) {
                    continue;
                }
                for (word, why) in RETIRED {
                    if line.contains(word) {
                        let rel = path.strip_prefix(&root).unwrap_or(&path).display();
                        live.push(format!("{rel}:{}  `{word}` — {why}", n + 1));
                    }
                }
            }
        }
    }

    assert!(scanned > 100, "the vocabulary scan reached the tree ({scanned} files)");
    assert!(
        live.is_empty(),
        "retired vocabulary is LIVE in the tree.\n\
         A ruling that retires a word retires it in the CODE too — prose \
         cannot enforce, which is how \"Daughter\" outlived its own \
         retirement by three days across 24 files:\n  {}",
        live.join("\n  ")
    );
}

/// C IS OUT OF THE JUDGE AND OUT OF THE LOOP — the arrow law, extended
/// past `rfa-core` to the two places that actually decide things.
///
/// `rfa-core/tests/cage_arrow.rs` holds the arrow inside its own crate:
/// six computing-path modules may CONSTRUCT a caged reading and must
/// never CONSUME one. But the crate is not where the world turns. The
/// JUDGE (`kernel/src`) admits or refuses every deed, and the WORLD
/// LOOP (`cfe/src/world`) advances the state — and neither was covered
/// by any arrow test at all.
///
/// **The standard is "does not know C exists", not "does not gate on
/// C".** A module that never names a symbol cannot accidentally grow a
/// dependency on it; a module that merely declines to branch on one can.
/// That is the only version of this law that cannot rot.
///
/// Editing this test IS the amendment (the `recurrence_guard` shape). A
/// future author who wants the judge or the loop to consume `C` must
/// come here and say so, in the open.
#[test]
fn c_is_out_of_the_judge_and_out_of_the_loop() {
    let root = workspace_root();
    let mut offences = Vec::new();
    let mut scanned = 0usize;
    let mut stack = vec![root.join("engine/kernel/src"), root.join("engine/cfe/src/world")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if !path.extension().is_some_and(|x| x == "rs") {
                continue;
            }
            let whole = fs::read_to_string(&path).unwrap_or_default();
            scanned += 1;
            // CODE ONLY. The prose deliberately NAMES what it refuses —
            // `bridge.rs` says "never `coherence_functional` here" — so a
            // naive scan would fail on the very comment that records the
            // exile. Strip comments and judge the code.
            for (n, line) in whole.lines().enumerate() {
                let t = line.trim_start();
                if t.starts_with("//") {
                    continue;
                }
                for name in ["coherence_functional", "Lens"] {
                    if line.contains(name) {
                        let rel = path.strip_prefix(&root).unwrap_or(&path).display();
                        offences.push(format!("{rel}:{}  names `{name}`", n + 1));
                    }
                }
            }
        }
    }
    assert!(scanned >= 15, "the arrow scan reached both trees ({scanned} files)");
    assert!(
        offences.is_empty(),
        "C has leaked into the judge or the world loop.\n\
         C has MAXIMAL ACCESS and ZERO AUTHORITY (`17` §10ter): everything \
         it produces is a proposal, and a proposal that reaches a gate is \
         no longer a proposal. The judge must be starved of the world to \
         be trusted with the verdict:\n  {}",
        offences.join("\n  ")
    );
}
