//! FORGE-2a pins (14_CREEDS §4): the difficulty vector derives from
//! the discrete moduli — deterministic, exact, bounded; the genesis
//! key reads the ladder's floor; growth in planes and doctrine widens
//! the word, the grid, and the tier, never past the caps.

use solver::Difficulty;

#[test]
fn the_genesis_key_reads_the_floor() {
    let d = Difficulty::from_key(1, 0, false);
    assert_eq!((d.tier, d.word_len, d.rows, d.cols, d.train_pairs), (1, 1, 3, 3, 2));
}

#[test]
fn planes_and_doctrine_widen_the_vector_to_the_caps() {
    let d = Difficulty::from_key(2, 3, true);
    assert_eq!((d.tier, d.word_len, d.rows, d.cols, d.train_pairs), (2, 3, 4, 4, 3));
    // Far past every cap: the vector saturates, never overflows.
    let d = Difficulty::from_key(9, 40, true);
    assert_eq!((d.tier, d.word_len, d.rows, d.cols, d.train_pairs), (3, 4, 6, 6, 3));
}

#[test]
fn the_derivation_is_a_pure_function() {
    for planes in 0..6 {
        for doctrine in 0..8 {
            for crossed in [false, true] {
                assert_eq!(
                    Difficulty::from_key(planes, doctrine, crossed),
                    Difficulty::from_key(planes, doctrine, crossed),
                );
            }
        }
    }
}
