//! IAE-5 — the proposer seam, where the Mind meets the Universe (doc 51).
//!
//! An exact homological engine that enumerates candidates is brute force
//! behind a differential-geometry facade. Cognition lives in *which*
//! candidates are tried. This module is the entity-level **proposer**: given
//! the exact obstruction `γ`, it stochastically samples candidate generators —
//! integer-weighted combinations of the base alphabet — and hands each to the
//! Universe's **exact verifier**. The first candidate the verifier admits is
//! minted as a new generator `v_new` (the Sullivan extension `d(v_new) = γ`).
//!
//! **Determinism is preserved by seeding.** The sampler is driven strictly by
//! `BLAKE3(state_digest)`, so the guessing is replayable pseudo-randomness — it
//! explores non-exhaustively and parallelizes across entities (GPU-offloadable),
//! yet a replay from the same chronicle reproduces the same mint. Floats are
//! lawful in the Mind; only the exact verifier commits (doc 51 §1–§2).

use num_bigint::BigInt;
use num_traits::Zero;

/// A BLAKE3-seeded pseudo-random stream — the Mind's only source of chance.
/// Deterministic in the seed, so the Universe stays replayable.
struct SeededRng {
    seed: [u8; 32],
    counter: u64,
}

impl SeededRng {
    fn new(seed: [u8; 32]) -> Self {
        SeededRng { seed, counter: 0 }
    }

    /// The next 32-byte block: `BLAKE3(seed ‖ counter)`, counter incrementing.
    fn next_block(&mut self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.seed);
        hasher.update(&self.counter.to_le_bytes());
        self.counter += 1;
        *hasher.finalize().as_bytes()
    }

    /// A weight uniformly in `[-range, range]` (range ≥ 0). One block per draw
    /// so the stream position is a clean function of the draw count.
    fn next_weight(&mut self, range: i64) -> i64 {
        let block = self.next_block();
        let v = u64::from_le_bytes(block[0..8].try_into().expect("32 ≥ 8 bytes"));
        let span = (2 * range + 1) as u64;
        (v % span) as i64 - range
    }
}

/// A minted generator: the candidate that passed exact verification, the
/// integer weights that formed it over the base, and how many guesses it took.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mint {
    pub generator: Vec<BigInt>,
    pub weights: Vec<i64>,
    pub tries: usize,
}

/// Form the candidate cochain `Σ wᵢ · baseᵢ` (exact integer combination).
fn combine(base: &[Vec<BigInt>], weights: &[i64]) -> Vec<BigInt> {
    let dim = base.first().map(|g| g.len()).unwrap_or(0);
    let mut out = vec![BigInt::zero(); dim];
    for (w, g) in weights.iter().zip(base) {
        for (o, gi) in out.iter_mut().zip(g) {
            *o += w * gi;
        }
    }
    out
}

/// The propose → verify → mint LOOP (doc 51 §3).
///
/// Seeded by `seed`, sample up to `max_tries` candidate generators — each an
/// integer-weighted combination of `base` with weights in `[-weight_range,
/// weight_range]` — and hand each to the Universe's exact `verify`. Return the
/// first that the verifier admits, minted; `None` if the budget is exhausted
/// without a bridge.
///
/// `verify` is the only contact with the exact Universe: it is the entity's
/// exact test that the candidate resolves the obstruction. The proposer itself
/// knows no homology — it only guesses, which is the whole point.
pub fn propose_and_verify<V>(
    seed: [u8; 32],
    base: &[Vec<BigInt>],
    verify: V,
    max_tries: usize,
    weight_range: i64,
) -> Option<Mint>
where
    V: Fn(&[BigInt]) -> bool,
{
    let mut rng = SeededRng::new(seed);
    for t in 0..max_tries {
        let weights: Vec<i64> = (0..base.len()).map(|_| rng.next_weight(weight_range)).collect();
        let candidate = combine(base, &weights);
        if verify(&candidate) {
            return Some(Mint { generator: candidate, weights, tries: t + 1 });
        }
    }
    None
}
