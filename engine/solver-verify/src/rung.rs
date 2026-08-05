//! **THE RANK-GENERIC RUNG AND FORGE** (operator ruling 2026-08-04:
//! *remove Grid entirely and upgrade to larger n-dimensional capacity*).
//!
//! One ladder, one forge, any rank.
//!
//! **What this replaces and why it existed.** `forge::Difficulty` spelled
//! a shape as `rows`/`cols`; `volume::VolDifficulty` spelled it as
//! `layers`/`rows`/`cols`. They shared no field, so they shared no code:
//! two `level()` tables, two `top()`, two `rung()`, two `lawful()`, two
//! forges, two proof shapes and two arms in the judge. That duplication
//! was not an accident of style — a three-dimensional rung **cannot be
//! spelled in planar fields**, so the moment a volumetric class landed a
//! parallel everything had to land with it.
//!
//! `shape: Vec<usize>` spells every rank, and rank falls out of
//! `shape.len()`. A fourth-rank class needs no new type, no new ladder
//! and no new judge arm — which is THE MOTION clause 5 (*ARC is a TYPE
//! of work, not the only type; the machinery must be work-CLASS generic,
//! never grid-specific*) made structural instead of promised.
//!
//! **The two ladders are preserved exactly**, rung for rung, so nothing
//! is repriced by this change: `Rung::level(2, n)` reproduces the planar
//! table and `Rung::level(3, n)` the volumetric one.

use crate::field::{Axis, Field, Word, HUES};
use physics::Beacon;

/// One rung of the hardness ladder, at any rank.
///
/// **FIELDS PRIVATE, ONE DOOR** (29_CONSTRUCTION C1, operator ruling
/// 2026-08-05: *apply from theorems — never judge the stochastic
/// outcome*). A `Rung` that exists IS lawful: the theorem is checked at
/// [`Rung::new`], once, with a proof — so the forge that draws from it
/// is TOTAL and nothing downstream re-judges what construction already
/// proved. The old `lawful()` judge arm is dead and deleted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rung {
    tier: u8,
    word_len: usize,
    shape: Vec<usize>,
    train_pairs: usize,
}

/// **WHY A RUNG STANDS OR DOES NOT** — the door's proof, with the
/// numbers that decided it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RungVerdict {
    /// A zero extent: the field it would forge cannot exist.
    EmptyExtent { axis: usize },
    /// A shapeless rung — rank 0 is the vacuous-arm class.
    NoAxes,
    /// The cell count overflows.
    TooVast,
    /// No training pairs: "maps every training pair" would be true over
    /// none, and an empty word would verify and mint for nothing.
    NothingToProve,
}

impl Rung {
    /// **THE ONE DOOR.** Everything the struck `lawful()` judged is
    /// checked here, once, and a refusal names its reason. Past this
    /// door there is no unlawful rung to catch.
    ///
    /// `word_len: 0` is LAWFUL and deliberately admitted: the empty word
    /// is the identity task, and a copy task is a task.
    pub fn new(
        tier: u8,
        word_len: usize,
        shape: Vec<usize>,
        train_pairs: usize,
    ) -> Result<Rung, RungVerdict> {
        if shape.is_empty() {
            return Err(RungVerdict::NoAxes);
        }
        if let Some(axis) = shape.iter().position(|&e| e == 0) {
            return Err(RungVerdict::EmptyExtent { axis });
        }
        if shape.iter().try_fold(1usize, |n, &e| n.checked_mul(e)).is_none() {
            return Err(RungVerdict::TooVast);
        }
        if train_pairs == 0 {
            return Err(RungVerdict::NothingToProve);
        }
        Ok(Rung { tier, word_len, shape, train_pairs })
    }

    pub fn tier(&self) -> u8 {
        self.tier
    }
    pub fn word_len(&self) -> usize {
        self.word_len
    }
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }
    pub fn train_pairs(&self) -> usize {
        self.train_pairs
    }

    /// The rank this rung stands at.
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    /// **THE STATIC LADDER, PER RANK.** Network-rate adjustment lands
    /// with the mesh (N6); until then the levels are explicit.
    ///
    /// Rank 2 reproduces the landed planar table and rank 3 the landed
    /// volumetric one, rung for rung — this change repriced nothing.
    /// An unknown rank falls back to the planar shape widened to that
    /// rank, so a fourth-rank class is forgeable the day someone asks
    /// for it rather than the day someone writes a second ladder.
    pub fn level(rank: usize, level: u32) -> Rung {
        match rank {
            2 => match level {
                0 | 1 => Rung { tier: 1, word_len: 1, shape: vec![3, 3], train_pairs: 2 },
                2 => Rung { tier: 1, word_len: 2, shape: vec![4, 4], train_pairs: 2 },
                3 => Rung { tier: 2, word_len: 2, shape: vec![4, 5], train_pairs: 3 },
                4 => Rung { tier: 3, word_len: 2, shape: vec![3, 3], train_pairs: 3 },
                _ => Rung { tier: 3, word_len: 3, shape: vec![4, 4], train_pairs: 3 },
            },
            3 => match level {
                0 | 1 => Rung { tier: 1, word_len: 1, shape: vec![2, 3, 3], train_pairs: 2 },
                2 => Rung { tier: 2, word_len: 2, shape: vec![3, 3, 3], train_pairs: 2 },
                3 => Rung { tier: 2, word_len: 2, shape: vec![3, 4, 4], train_pairs: 3 },
                4 => Rung { tier: 3, word_len: 3, shape: vec![4, 4, 4], train_pairs: 3 },
                _ => Rung { tier: 3, word_len: 3, shape: vec![5, 4, 5], train_pairs: 3 },
            },
            // Any other rank: the planar progression, widened. Deliberate
            // rather than a refusal — a rank with no hand-written table
            // is still forgeable, which is the whole point of the shape
            // being a vector.
            rank => {
                let planar = Rung::level(2, level);
                let mut shape = vec![3usize; rank.max(1)];
                for (at, extent) in planar.shape.iter().enumerate() {
                    if at < shape.len() {
                        shape[at] = *extent;
                    }
                }
                Rung { shape, ..planar }
            }
        }
    }

    /// The top rung at a rank, derived: the first level nothing rises
    /// above. Compared against SATURATION, never against the next rung —
    /// rungs 0 and 1 are deliberately the same opening rung, and a
    /// plateau is not a ceiling.
    pub fn top(rank: usize) -> u32 {
        (0..).find(|&n| !Rung::climbs(rank, n)).unwrap_or(0)
    }

    /// Does the ladder still rise above `level` at this rank?
    pub fn climbs(rank: usize, level: u32) -> bool {
        Rung::level(rank, level) != Rung::level(rank, u32::MAX)
    }

    /// **WHERE THIS SHAPE SITS** — the pricing key, as a POSITION rather
    /// than a shape.
    ///
    /// Returns `(rank, level)`. A payload cannot name its own price by
    /// inventing a convenient shape: the shape is matched WHOLE against
    /// the ladder at its own rank, and anything off the ladder has no
    /// position and therefore no price.
    pub fn position(&self) -> Option<(usize, u32)> {
        let rank = self.rank();
        (0..=Rung::top(rank))
            .find(|&n| Rung::level(rank, n) == *self)
            .map(|level| (rank, level))
    }

    // `lawful()` LIVED HERE and is DELETED (29_CONSTRUCTION C1): every
    // check it performed moved into `Rung::new`, so a rung that exists
    // has nothing left to be judged for. The degeneracy law did not
    // weaken — it moved from a judgment to a precondition of existence.
}

/// A forged task, at any rank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldTask {
    pub name: String,
    pub train: Vec<(Field, Field)>,
    pub test_in: Field,
    pub test_expected: Option<Field>,
}

/// Sample one transform for position `slot`, drawn from the rung's tier
/// and the field's own rank.
///
/// **The axis indices are drawn from the RANK**, which is the whole
/// reason one sampler serves every class: at rank 2 a `Reverse` is a
/// flip and a `Swap(0,1)` is a transpose; at rank 3 the same two draws
/// are what `Volume` had to call `FlipStack` and a layer exchange.
fn sample_axis(beacon: &Beacon, rung: &Rung, slot: usize) -> Axis {
    let rank = rung.rank();
    let families = match rung.tier {
        1 => 2,
        2 => 3,
        _ => 4,
    };
    match beacon.uniform_index(&format!("nforge.family.{slot}"), families) {
        0 => Axis::Reverse(beacon.uniform_index(&format!("nforge.rev.{slot}"), rank as u64) as usize),
        1 => {
            let a = beacon.uniform_index(&format!("nforge.swap.a.{slot}"), rank as u64) as usize;
            // A swap with itself is the identity, which would make a
            // word-length-1 task a copy task by accident rather than by
            // the ladder's choice. Step to a genuine partner.
            let b = (a + 1 + beacon.uniform_index(&format!("nforge.swap.b.{slot}"), (rank.max(2) - 1) as u64) as usize) % rank;
            Axis::Swap(a, b)
        }
        2 => {
            let mut map = [0u8; HUES as usize];
            for (hue, slot_hue) in map.iter_mut().enumerate() {
                *slot_hue = hue as u8;
            }
            for i in (1..HUES as usize).rev() {
                let j = beacon.uniform_index(&format!("nforge.recolor.{slot}.{i}"), (i + 1) as u64);
                map.swap(i, j as usize);
            }
            Axis::Recolor(map)
        }
        _ => {
            let axis = beacon.uniform_index(&format!("nforge.roll.a.{slot}"), rank as u64) as usize;
            let extent = rung.shape[axis].max(2);
            let by = 1 + beacon.uniform_index(&format!("nforge.roll.b.{slot}"), (extent - 1) as u64) as usize;
            Axis::Roll(axis, by)
        }
    }
}

fn sample_field(beacon: &Beacon, rung: &Rung, pair: usize) -> Field {
    let cells: Vec<u8> = (0..rung.shape.iter().product::<usize>())
        .map(|at| beacon.uniform_index(&format!("nforge.cell.{pair}.{at}"), u64::from(HUES)) as u8)
        .collect();
    Field::new(rung.shape.clone(), cells).expect("a lawful rung forges a lawful field")
}

/// **THE N-FORGE.** Beacon-seeded task generation at any rank.
///
/// Same digest and rung, byte-identical task on every machine; a
/// different chronicle forges a different task. The secret word is
/// sampled, applied, then DISCARDED — the task carries only
/// observations, and finding A word that closes them is the solver's
/// work (it need not be the forged one; the verifier is the authority
/// either way).
///
/// THE BOUNDARY LAW holds unchanged: the forge allocates rewards and
/// paces difficulty. It is never a security primitive.
pub fn forge_field(digest: [u8; 32], rung: &Rung) -> FieldTask {
    // TOTAL, deliberately. A `Rung` is lawful by construction (the one
    // door), so the stochastic draw ranges over the lawful space and
    // there is no outcome left to judge — the theorem was applied
    // BEFORE the beacon ever drew.
    let beacon = Beacon::from_digest(digest);
    let word: Word = (0..rung.word_len)
        .map(|slot| sample_axis(&beacon, rung, slot))
        .collect();

    let render = |field: &Field| crate::field::run(&word, field);
    let train: Vec<(Field, Field)> = (0..rung.train_pairs)
        .map(|pair| {
            let input = sample_field(&beacon, rung, pair);
            let output = render(&input);
            (input, output)
        })
        .collect();
    let test_in = sample_field(&beacon, rung, rung.train_pairs);
    let test_expected = Some(render(&test_in));

    let mut name = String::from("nforge_");
    for byte in &digest[..6] {
        name.push_str(&format!("{byte:02x}"));
    }
    FieldTask {
        name: format!("{name}_r{}w{}", rung.rank(), rung.word_len),
        train,
        test_in,
        test_expected,
    }
}

/// **THE CLOSED VOCABULARY AT A RANK** — every transform a word may be
/// built from, derived from the shape rather than listed per class.
///
/// This is what `volume::vocabulary()` was, generalized: that function
/// hand-listed the volumetric operators because a planar list could not
/// express them. Here the axis indices come from the rank and the roll
/// distances from the extents, so rank 2 yields the planar set, rank 3
/// the volumetric set, and rank 4 a set nobody had to write.
///
/// Bounded by construction: `rank` mirrors, `rank·(rank−1)/2` swaps, and
/// the rolls of each axis. Recolor is EXCLUDED — the hue permutations
/// are `10!`, far too many to enumerate, and a search that must guess a
/// permutation is not a fair-depth search. The forge only draws recolor
/// at tier 2 and above, which is exactly where the corpus path (not the
/// forge path) is meant to carry the work.
pub fn vocabulary(rank: usize, shape: &[usize]) -> Vec<Axis> {
    let mut out = Vec::new();
    for axis in 0..rank {
        out.push(Axis::Reverse(axis));
    }
    for a in 0..rank {
        for b in (a + 1)..rank {
            out.push(Axis::Swap(a, b));
        }
    }
    for axis in 0..rank {
        let extent = shape.get(axis).copied().unwrap_or(1);
        for by in 1..extent {
            out.push(Axis::Roll(axis, by));
        }
    }
    out
}
