//! THE HUE INVARIANT — enforced at construction, not merely documented.
//!
//! `Grid` declares "hues 0–9" in its own doc comment and `Grid::new`
//! never checked it. The loader checked separately (`loader.rs`), which
//! is the tell: the invariant lived in the callers, so every door that
//! was not the loader was open.
//!
//! Two panics follow an out-of-range hue, both inside SEALED MATH the
//! kernel calls:
//!   · `Op::Recolor(map)` — `map[h as usize]` on a `[u8; 10]`;
//!   · `objects::background` — `counts[cell as usize]` on `[usize; 10]`.
//!
//! Under work-shapes v1 (forge-only) neither is reachable from the
//! judge: every grid `verify_work` feeds to `run` comes from the forge,
//! and the wire-carried output grid is only compared and hashed. That
//! is a property of v1's SHAPE, not of the code being total — and
//! work-shapes v2 (the corpus registry, 16_UNIT §7a) ends it, because a
//! corpus task cannot be re-derived from a digest and must ride in the
//! payload. These pins seal the door BEFORE that lands.

use solver_verify::grid::{Grid, Op};
use solver_verify::work::{decode_work, encode_work, WorkProof};
use solver_verify::{forge_task, objects, Difficulty, SolveCertificate};

/// The constructor is the door: a hue above 9 is not a grid.
#[test]
fn a_grid_refuses_an_out_of_range_hue() {
    assert!(Grid::new(1, 2, vec![0, 9]).is_some(), "the full lawful range admits");
    assert!(Grid::new(1, 1, vec![10]).is_none(), "10 is past the ladder");
    assert!(Grid::new(1, 1, vec![255]).is_none(), "the wire's worst byte");
    assert!(
        Grid::new(2, 2, vec![1, 2, 200, 4]).is_none(),
        "one bad cell condemns the grid — not a clamp, a refusal"
    );
}

/// The two operators that index a ten-slot table stay total across the
/// whole lawful range. This is the pin that says WHY the door matters.
#[test]
fn the_ten_slot_operators_are_total_over_lawful_hues() {
    for hue in 0..=9u8 {
        let g = Grid::new(2, 2, vec![hue, hue, hue, hue]).expect("lawful");
        // Recolor indexes map[hue]; background indexes counts[hue].
        let _ = Op::Recolor([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).apply(&g);
        let _ = objects::background(&g);
        for op in objects::OBJ_OPS {
            let _ = Op::Object(op).apply(&g);
        }
    }
}

/// THE DEGENERACY LAW has one name, and it admits the identity task.
/// `forge_task` panics on a zero dimension and `Difficulty`'s fields are
/// `pub`, so the guard can only live at the doors — it must therefore be
/// callable, not restated inline at each one (audit HOLE B's shape).
#[test]
fn the_degeneracy_law_is_named_and_admits_the_copy_task() {
    for level in 0..=5 {
        assert!(Difficulty::level(level).lawful(), "every ladder rung is lawful");
    }
    let rung = Difficulty::level(3);
    assert!(!Difficulty { rows: 0, ..rung }.lawful(), "a zero row panics the forge");
    assert!(!Difficulty { cols: 0, ..rung }.lawful(), "a zero col panics the forge");
    assert!(
        !Difficulty { train_pairs: 0, ..rung }.lawful(),
        "no training pair proves nothing — the vacuity"
    );
    assert!(
        Difficulty { word_len: 0, ..rung }.lawful(),
        "the empty secret word is the identity task, and a copy task is a task"
    );
}

/// THE WIRE DOOR. `decode_work` reads the output grid from raw bytes;
/// an out-of-range hue must refuse as FORM, the same law that already
/// refuses degenerate difficulty — refuse never guess.
#[test]
fn the_wire_refuses_a_payload_carrying_an_out_of_range_hue() {
    // Build one honest proof, then corrupt a single hue byte in the
    // encoded output grid. Everything else stays well-formed, so the
    // refusal can only be the hue.
    let difficulty = Difficulty::level(3);
    let task = forge_task([13u8; 32], difficulty);
    let program = vec![Op::Rotate90];
    let output = solver_verify::grid::run(&program, &task.test_in);
    let certificate = SolveCertificate::seal(&task, &program, &output);
    let proof = WorkProof {
        forge_digest: [13u8; 32],
        difficulty,
        program,
        output: output.clone(),
        certificate,
    };
    let bytes = encode_work(&proof).expect("encodes");
    assert!(decode_work(&bytes).is_some(), "the honest payload decodes");

    // The output grid's cells are the only run of raw hue bytes whose
    // length equals the output's cell count; find it by value and bend
    // one byte past the ladder.
    let mut bent = bytes.clone();
    let first_cell = bytes
        .windows(output.cells.len())
        .position(|w| w == output.cells.as_slice())
        .expect("the output cells ride in the payload");
    bent[first_cell] = 200;
    assert!(
        decode_work(&bent).is_none(),
        "a hue past the ladder refuses as form — it must never reach an operator"
    );
}
