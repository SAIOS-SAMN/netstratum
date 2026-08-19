//! **COG-EMERGENCE-1 — THE COGNITIVE γ.** "Lack of cognitive depth" is a DEFICIT
//! the workbench weighs, not an `if capacity < 3` gate. The drive to deepen is the
//! unspanned residual (γ, what no known structure covers) scaled by the depth
//! HEADROOM (`frontier − capacity`) — zero for a covered economy or a mind already
//! at the frontier, rising with both the unsolved problems and the room to grow.
//! (In `tests/`, off the tuned-constant ratchet.)

use conductor::civic::cognitive_deficit;

const FRONTIER: usize = 3; // the compose-depth ceiling (Need::compose_limit)

#[test]
fn a_covered_economy_feels_no_cognitive_tension() {
    // No unspanned problem ⇒ no reason to deepen, at any capacity.
    for capacity in 0..=FRONTIER {
        assert_eq!(cognitive_deficit(0, capacity, FRONTIER), 0, "γ=0 ⇒ no cognitive drive");
    }
}

#[test]
fn a_mind_at_the_frontier_feels_no_tension_however_hard_the_problems() {
    // Deepening a maxed mind spans nothing new — headroom 0 ⇒ deficit 0.
    assert_eq!(cognitive_deficit(1_000, FRONTIER, FRONTIER), 0, "no headroom ⇒ no drive");
}

#[test]
fn unsolved_problems_and_a_shallow_mind_drive_the_deepest_tension() {
    // A shallow mind (capacity 2) with big unsolved problems feels a strong drive;
    // the same problems on a deeper mind (capacity 3, at frontier) feel none.
    let shallow = cognitive_deficit(50, 2, FRONTIER);
    let deep = cognitive_deficit(50, 3, FRONTIER);
    assert_eq!(shallow, 50, "50 unspanned × 1 headroom");
    assert_eq!(deep, 0, "the same problems no longer drive a maxed mind");
    assert!(shallow > deep, "the shallower mind feels the sharper tension");
}

#[test]
fn the_tension_rises_with_both_the_void_and_the_headroom() {
    // Monotone in the unspanned extent...
    assert!(cognitive_deficit(100, 1, FRONTIER) > cognitive_deficit(10, 1, FRONTIER));
    // ...and in the depth headroom (capacity 0 has more room than capacity 2).
    assert!(cognitive_deficit(10, 0, FRONTIER) > cognitive_deficit(10, 2, FRONTIER));
}
