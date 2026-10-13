//! The chronicle beacon: deterministic, exact, replayable randomness.
//!
//! **The law (CHARTER §9; substrate contract §6):**
//! "stochastic" in netstratum means chronicle-seeded randomness — every
//! draw derives from a receipt digest already in the chronicle by exact
//! arithmetic. Rational probabilities, no float, no ambient entropy, no
//! wall clock. The world feels stochastic to players; replay re-derives
//! every draw bit-exact, so SEAL B, mesh verification, and anchoring
//! survive untouched.
//!
//! Every draw carries a DOMAIN TAG naming the system that consumed it
//! (e.g. `"hazard.meteor"`, `"market.demand"`); the tag must appear in
//! the receipt that consumed the draw. No game system rolls its own
//! randomness.

use ratio::Ratio;
use num_bigint::BigInt;

/// A beacon anchored to one chronicle digest (the previous tick's
/// receipt digest). All draws from the same digest and domain are
/// byte-identical on every machine, forever.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Beacon {
    digest: [u8; 32],
}

impl Beacon {
    /// Anchor a beacon to a chronicle digest.
    pub fn from_digest(digest: [u8; 32]) -> Self {
        Self { digest }
    }

    /// The raw 128-bit expansion for `(domain, counter)` — keyed BLAKE3
    /// over the domain tag and counter, keyed by the chronicle digest.
    fn expand(&self, domain: &str, counter: u64) -> u128 {
        let mut hasher = blake3::Hasher::new_keyed(&self.digest);
        hasher.update(domain.as_bytes());
        hasher.update(&[0u8]); // domain/counter seam — tags cannot collide by concatenation
        hasher.update(&counter.to_le_bytes());
        let out = hasher.finalize();
        u128::from_le_bytes(out.as_bytes()[..16].try_into().expect("32 ≥ 16"))
    }

    /// One exact draw in the exact rationals ∩ [0, 1): `k / 2¹²⁸` where `k` is the
    /// domain-tagged expansion. Uniform at 2⁻¹²⁸ grain, exact by
    /// construction.
    pub fn draw(&self, domain: &str) -> Ratio {
        let k = self.expand(domain, 0);
        Ratio::new(BigInt::from(k), BigInt::from(1u8) << 128)
    }

    /// An exactly-uniform index in `0..n` — rejection-sampled, so there
    /// is NO modulo bias: expansions falling in the biased tail are
    /// discarded and the counter advances (deterministically — replay
    /// walks the same rejections).
    ///
    /// # Panics
    /// If `n == 0` (an empty range has no lawful draw).
    pub fn uniform_index(&self, domain: &str, n: u64) -> u64 {
        assert!(n > 0, "an empty range has no lawful draw");
        let n = u128::from(n);
        // 2¹²⁸ mod n, computed without overflowing: (MAX mod n + 1) mod n.
        let tail = (u128::MAX % n + 1) % n;
        let mut counter = 0u64;
        loop {
            let k = self.expand(domain, counter);
            // Accept unless k lands in the short tail [2¹²⁸ − tail, 2¹²⁸).
            if tail == 0 || k <= u128::MAX - tail {
                return (k % n) as u64;
            }
            counter += 1;
        }
    }
}
