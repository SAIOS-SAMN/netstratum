//! NOVELTY-GATE pins (38_COGNITION_CONVERGENCE §4, task #13).

use solver::mint::{consider, pre_reach, standing_orbits, word_orbit, Library, MintedWord};
use solver_verify::grid::{Grid, Op};
use solver_verify::loader::{Task, TaskPair};

fn ramp(rows: usize, cols: usize) -> Grid {
    let cells = (0..rows * cols).map(|k| (k % 10) as u8).collect();
    Grid::new(rows, cols, cells).expect("ramp")
}

/// A re-spelling can never re-mint: rot90∘rot90 IS rot180 by action,
/// and the fingerprint says so exactly.
#[test]
fn a_respelling_shares_the_orbit() {
    assert_eq!(
        word_orbit(&[Op::Rotate90, Op::Rotate90]),
        word_orbit(&[Op::Rotate180]),
        "two spellings, one action, one orbit"
    );
    assert_eq!(
        word_orbit(&[Op::FlipH, Op::FlipH]),
        word_orbit(&[]),
        "an involution squared is the identity"
    );
    assert_ne!(
        word_orbit(&[Op::Rotate90]),
        word_orbit(&[Op::Rotate270]),
        "distinct actions, distinct orbits"
    );
}

/// Gate 1 refuses re-spellings of standing letters and library words;
/// a genuinely new composition passes to confirmation.
#[test]
fn the_unique_gate_bites() {
    let standing_letters = [
        Op::Rotate90,
        Op::Rotate180,
        Op::Rotate270,
        Op::FlipH,
        Op::FlipV,
        Op::Transpose,
    ];
    let standing = standing_orbits(&standing_letters);
    let library = Library::new();
    // rot90∘rot90 re-spells rot180 — refused at gate 1.
    assert_eq!(
        consider(&[Op::Rotate90, Op::Rotate90], &library, &standing),
        None,
        "a re-spelling of a standing letter never mints"
    );
    // A single letter is already in the alphabet.
    assert_eq!(consider(&[Op::Rotate90], &library, &standing), None);
    // rot90∘flipH IS Transpose — the gate knows its group theory and
    // refuses the re-spelling (D4 at depth 2 mostly folds back in).
    assert_eq!(
        consider(&[Op::Rotate90, Op::FlipH], &library, &standing),
        None,
        "rot90∘flipH re-spells Transpose"
    );
    // The ANTI-TRANSPOSE (rot90∘flipV) is the D4 element the standing
    // six actually lack — the gate discovers the alphabet's own gap.
    let candidate = consider(&[Op::Rotate90, Op::FlipV], &library, &standing);
    assert!(candidate.is_some(), "the missing D4 element passes gate 1");
    // Once adopted, every spelling of the SAME action refuses forever.
    let mut library = library;
    library.adopt(MintedWord {
        orbit: candidate.expect("checked"),
        expansion: vec![Op::Rotate90, Op::FlipV],
        cited: Vec::new(),
    });
    assert_eq!(
        consider(&[Op::FlipH, Op::Rotate90], &library, &standing),
        None,
        "flipH∘rot90 is the anti-transpose's other spelling — the \
         adopted orbit refuses it"
    );
}

/// THE PRE-REACH: a library word closes its task family at once, and
/// what it returns is the EXPANSION — the certificate's own form; the
/// judge never learns a macro.
#[test]
fn the_library_extends_the_reach_as_expansions() {
    let word = vec![Op::Rotate90, Op::FlipH];
    let input = ramp(2, 3);
    let mut output = input.clone();
    for op in &word {
        output = op.apply(&output);
    }
    let task = Task {
        name: "mint-probe".into(),
        train: vec![TaskPair { input: input.clone(), output: output.clone() }],
        test_in: input.clone(),
        test_expected: Some(output.clone()),
    };
    let empty = Library::new();
    assert_eq!(pre_reach(&task, &empty), None, "no library, no pre-reach");
    let mut library = Library::new();
    library.adopt(MintedWord {
        orbit: word_orbit(&word),
        expansion: word.clone(),
        cited: Vec::new(),
    });
    assert_eq!(
        pre_reach(&task, &library),
        Some(word),
        "the word closes at once and speaks as letters"
    );
    // The vacuous-solve refusal holds here too.
    let vacuous = Task {
        name: "mint-vacuous".into(),
        train: Vec::new(),
        test_in: input,
        test_expected: Some(output),
    };
    assert_eq!(pre_reach(&vacuous, &library), None, "nothing to close, nothing proved");
}
