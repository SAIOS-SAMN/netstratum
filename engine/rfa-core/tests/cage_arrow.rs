//! THE ARROW LAW FOR THE CAGE — C cannot leak into D.
//!
//! A standing structural test in the shape of `kernel/tests/
//! recurrence_guard.rs`: the arrow is enforced by a test that reads the
//! source, so EDITING THIS TEST IS THE AMENDMENT. A future author who
//! wants a computing path to consume `C` must come here and say so.

use std::fs;

/// Modules on the COMPUTING PATH. They may CONSTRUCT a caged reading —
/// that is the door in — but must never CONSUME one, name its type, or
/// depend on it in any way.
const COMPUTING_PATH: [&str; 5] = [
    "src/potential.rs",
    "src/reduce.rs",
    "src/fold.rs",
    "src/gates.rs",
    "src/rcf.rs",
];

#[test]
fn no_computing_path_consumes_the_cage() {
    // C IS EXILED. The manifold's own readings — capacity, divergence,
    // the fold verdict, the canonical form, the step gates — are
    // computed with no reference to the caged scalar whatsoever.
    //
    // This is stronger than "does not gate on C". It is "does not know C
    // exists", which is the only version that cannot rot: you cannot
    // accidentally re-introduce a dependency on a name your module never
    // mentions.
    for module in COMPUTING_PATH {
        let whole = fs::read_to_string(module).unwrap_or_else(|_| panic!("{module} readable"));
        let source: String = whole
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !source.contains("Lens"),
            "{module} names the cage — C has leaked into the computing path"
        );
        assert!(
            !source.contains("coherence_functional"),
            "{module} calls C — the computing path must not know it exists"
        );
    }
}

#[test]
fn the_cage_has_no_exit_to_a_computable_type() {
    // Every operation on a caged reading yields a caged reading. The
    // module offers `of` (in) and `show`/`Display` (out to a String),
    // and the closed algebra below — and nothing that produces a value
    // the engine can branch on.
    // CODE ONLY. The module's own prose NAMES the doors it refuses, so
    // a naive scan matches the documentation and fails on a clean file —
    // it did, on the first run. Strip comments and judge the code.
    let whole = fs::read_to_string("src/cage.rs").expect("cage readable");
    let source: String = whole
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    // No accessor returning the interior.
    for escape in ["into_inner", "as_ratio", "impl Deref", "-> Ratio {"] {
        assert!(
            !source.contains(escape),
            "cage.rs exposes `{escape}` — one such door reopens every path this module closes"
        );
    }
    // And no ordering, ever: descent needs it and identity does not.
    assert!(
        !source.contains("PartialOrd") && !source.contains("impl Ord"),
        "cage.rs derives an ordering — thresholds and descent become expressible again"
    );
}
