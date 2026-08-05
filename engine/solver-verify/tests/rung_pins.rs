//! **THE ONE LADDER** — that it reprices nothing, and that it does the
//! thing the two ladders structurally could not.

use solver_verify::forge::Difficulty;
use solver_verify::rung::{forge_field, Rung};
use solver_verify::volume::VolDifficulty;

/// **NOTHING IS REPRICED.** The rank-2 ladder reproduces the landed
/// planar table and the rank-3 ladder the landed volumetric one, rung
/// for rung. If this drifts, live work changes price.
#[test]
fn the_one_ladder_reproduces_both_old_ones() {
    for level in 0..=8u32 {
        let planar = Difficulty::level(level);
        let rung = Rung::level(2, level);
        assert_eq!(rung.shape().to_vec(), vec![planar.rows, planar.cols], "planar shape at {level}");
        assert_eq!(rung.word_len(), planar.word_len, "planar word_len at {level}");
        assert_eq!(rung.train_pairs(), planar.train_pairs, "planar pairs at {level}");
        assert_eq!(rung.tier(), planar.tier, "planar tier at {level}");

        let vol = VolDifficulty::level(level);
        let rung = Rung::level(3, level);
        assert_eq!(
            rung.shape().to_vec(),
            vec![vol.layers, vol.rows, vol.cols],
            "volumetric shape at {level}"
        );
        assert_eq!(rung.word_len(), vol.word_len, "volumetric word_len at {level}");
        assert_eq!(rung.train_pairs(), vol.train_pairs, "volumetric pairs at {level}");
    }
    // And the saturation points match, so `climbs` still stops where it did.
    assert_eq!(Rung::top(2), Difficulty::top());
    assert_eq!(Rung::top(3), VolDifficulty::top());
}

/// **THE THING TWO LADDERS COULD NOT DO.** A rank-4 task forges from the
/// same ladder, the same forge and the same vocabulary — no new type, no
/// new table, no new judge arm. This is the ruling's actual point.
#[test]
fn a_fourth_rank_task_forges_with_no_new_code() {
    let digest = [7u8; 32];
    for rank in 2..=5usize {
        let rung = Rung::level(rank, 3);
        assert_eq!(rung.rank(), rank);
        // No `lawful()` to ask — a Rung that exists IS lawful, which is
        // the C1 point: the ladder's own rungs come out of the same
        // door everything else must pass.
        let task = forge_field(digest, &rung);
        assert_eq!(task.train.len(), rung.train_pairs());
        for (input, output) in &task.train {
            assert_eq!(input.rank(), rank, "the task stands at rank {rank}");
            assert_eq!(input.cells().len(), output.cells().len(), "a transform is total");
        }
        assert!(task.test_expected.is_some());
    }
}

/// The forge is DETERMINISTIC: same digest, same rung, byte-identical
/// task. This is what makes replay possible at all.
#[test]
fn the_forge_is_deterministic_at_every_rank() {
    for rank in 2..=4usize {
        let rung = Rung::level(rank, 4);
        let a = forge_field([3u8; 32], &rung);
        let b = forge_field([3u8; 32], &rung);
        assert_eq!(a, b, "rank {rank}: same digest, same task");
        let c = forge_field([4u8; 32], &rung);
        assert_ne!(a, c, "rank {rank}: a different chronicle forges differently");
    }
}

/// **THE POSITION IS THE PRICE.** A shape off the ladder has no position
/// and therefore no price — a payload cannot name its own rung by
/// inventing a convenient shape.
#[test]
fn an_invented_shape_has_no_position() {
    let real = Rung::level(3, 2);
    assert_eq!(real.position(), Some((3, 2)));

    let invented = Rung::new(3, 1, vec![9, 9, 9], 9).expect("well-formed, just unpriced");
    assert_eq!(invented.position(), None, "off the ladder is off the price list");

    // THE DEGENERACY LAW IS NOW THE DOOR (29_CONSTRUCTION C1): a
    // degenerate rung is not judged unlawful — it CANNOT EXIST, and the
    // refusal names why. This replaced `lawful()`, whose every check
    // moved here; there is no rung left for it to judge.
    use solver_verify::rung::RungVerdict;
    assert_eq!(
        Rung::new(1, 1, vec![3, 0], 2),
        Err(RungVerdict::EmptyExtent { axis: 1 }),
        "a zero extent refuses at the door, naming the axis"
    );
    assert_eq!(
        Rung::new(1, 1, vec![3, 3], 0),
        Err(RungVerdict::NothingToProve),
        "no training pairs is the vacuity, refused before it exists"
    );
    assert_eq!(Rung::new(1, 1, vec![], 2), Err(RungVerdict::NoAxes));
    // The empty word stands: a copy task is a task.
    assert!(Rung::new(1, 0, vec![3, 3], 2).is_ok());
}
