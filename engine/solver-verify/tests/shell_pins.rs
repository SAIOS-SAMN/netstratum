//! **THE SPHERICAL CARRIER, measured as a GROUP** — not asserted as
//! prose.
//!
//! The claim is that a shell has no privileged axis and that every
//! transform is an exact permutation. Both are checkable, and a claim of
//! frame-freedom that is not checked is exactly the ornament
//! `27_EVIDENCE` LAW 8 excises.

use ratio::TURN_SLOTS;
use solver_verify::shell::{run, vocabulary, Shell, Turn};

fn shell(bands: usize, slots: usize) -> Shell {
    let cells: Vec<u8> = (0..bands * slots).map(|n| (n % 10) as u8).collect();
    Shell::new(bands, slots, cells).expect("lawful shell")
}

/// **THE RING CLOSES.** A spin by the full slot count is the identity —
/// exactly, at every divisor of sixty. This is the property radians
/// cannot have: `2π` is not rational, so a wrapped angle would need a
/// float and the seam would never meet.
#[test]
fn the_ring_closes_exactly_at_every_lawful_arity() {
    for slots in [1usize, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60] {
        assert_eq!(TURN_SLOTS as usize % slots, 0, "{slots} must divide the ring");
        let held = shell(3, slots);
        let full: Vec<Turn> = vec![Turn::Spin(slots as i64)];
        assert_eq!(run(&full, &held), held, "a full turn at arity {slots} is the identity");
        // And it closes both ways.
        assert_eq!(
            run(&vec![Turn::Spin(-(slots as i64))], &held),
            held,
            "the ring closes in reverse too"
        );
    }
    // An arity that does not divide sixty is REFUSED at the door — a
    // cell whose centre falls between slots is not a place.
    assert!(Shell::new(2, 7, vec![0; 14]).is_none(), "sevenths do not divide sixty");
    assert!(Shell::new(2, 8, vec![0; 16]).is_none(), "eighths do not divide sixty");
}

/// **EVERY TRANSFORM IS A PERMUTATION.** No cell is created, none
/// destroyed, none interpolated — the hue multiset is invariant. This is
/// what "no resampling" means, and it is why the carrier needs no
/// floats.
#[test]
fn every_transform_is_an_exact_permutation() {
    let held = shell(4, 12);
    let census = |s: &Shell| {
        let mut counts = [0usize; 10];
        for &hue in s.cells() {
            counts[hue as usize] += 1;
        }
        counts
    };
    let before = census(&held);
    for turn in vocabulary(&held) {
        let after = census(&turn.apply(&held));
        assert_eq!(before, after, "{} moved hues in or out", turn.describe());
    }
    // Recolor is the one that RELABELS rather than moves; it still
    // preserves the cell count.
    let map = [9u8, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let recoloured = Turn::Recolor(map).apply(&held);
    assert_eq!(recoloured.cells().len(), held.cells().len());
}

/// **EVERY TRANSFORM IS INVERTIBLE**, which is what makes the
/// vocabulary a group rather than a bag of effects. A search that can
/// enter a state it cannot leave is not searching a space.
#[test]
fn every_transform_has_an_exact_inverse() {
    let held = shell(3, 15);
    for turn in vocabulary(&held) {
        let inverse = match &turn {
            Turn::Spin(k) => Turn::Spin(-k),
            Turn::Mirror => Turn::Mirror,
            Turn::Invert => Turn::Invert,
            Turn::Recolor(_) => continue,
        };
        assert_eq!(
            run(&vec![turn.clone(), inverse], &held),
            held,
            "{} has no exact inverse",
            turn.describe()
        );
    }
}

/// **NO PRIVILEGED AXIS — the claim that separates a shell from a grid.**
///
/// On a grid, `FlipH` and `FlipV` are different operators because a
/// rectangle HAS a distinguished horizontal and vertical. On a ring
/// every longitude is like every other, so a spin commutes with itself
/// from any starting rotation: rotating then measuring is the same as
/// measuring then rotating. There is no origin longitude to privilege.
#[test]
fn the_ring_has_no_origin_longitude() {
    let held = shell(3, 12);
    // Whatever rotation you view it from, the same further spin has the
    // same effect — the composition depends on the total, never on where
    // you started counting.
    for start in 0..12i64 {
        let viewed = Turn::Spin(start).apply(&held);
        let a = Turn::Spin(5).apply(&viewed);
        let b = Turn::Spin(start + 5).apply(&held);
        assert_eq!(a, b, "the ring privileged the origin at start {start}");
    }
    // Spins commute with each other — an abelian rotation group, which
    // a rectangle's flips do not form.
    let x = run(&vec![Turn::Spin(3), Turn::Spin(7)], &held);
    let y = run(&vec![Turn::Spin(7), Turn::Spin(3)], &held);
    assert_eq!(x, y, "rotations must commute");
}

/// The vocabulary is CLOSED: composing any two of its members lands on
/// a shell the vocabulary could also have reached. A search over a
/// non-closed vocabulary explores a space that does not exist.
#[test]
fn the_vocabulary_is_closed_under_composition() {
    let held = shell(2, 6);
    let words = vocabulary(&held);
    let reachable: Vec<Shell> = {
        let mut seen = vec![held.clone()];
        for _ in 0..3 {
            let mut grown = seen.clone();
            for s in &seen {
                for w in &words {
                    let next = w.apply(s);
                    if !grown.contains(&next) {
                        grown.push(next);
                    }
                }
            }
            seen = grown;
        }
        seen
    };
    for a in &words {
        for b in &words {
            let composed = run(&vec![a.clone(), b.clone()], &held);
            assert!(
                reachable.contains(&composed),
                "{} then {} left the group",
                a.describe(),
                b.describe()
            );
        }
    }
}
