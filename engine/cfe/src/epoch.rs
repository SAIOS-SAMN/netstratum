//! Ruleset EPOCHS (`21_KERNEL_PARAMETERS` §11, operator ruling
//! 2026-08-02) — how a sealed rule changes without any past receipt
//! changing meaning.
//!
//! **The problem this solves, measured rather than argued.** On
//! 2026-08-02 `ease_of`'s values moved from a cliff to a gradient. The
//! arithmetic was correct and every test that exercised it passed —
//! and `conductor/tests/survey_pins.rs` failed, because `replay` and
//! the live proposer reconstructed different worlds from the same
//! receipts (live `Crystal 127/16`, replay `15/16`). The transition is
//! defined twice, so the two agree only while the sealed rules stand
//! still. Every parameter amendment moves a sealed rule, so every one
//! of them hits that wall.
//!
//! **The fix costs nothing on the wire.** A receipt already carries
//! `stamp` (`vrp.rs:100`), already monotone and already inside the
//! frame hash. So the ruleset a receipt is gated under is derived
//! from what it already holds rather than stored beside it:
//!
//! ```text
//! the epoch is a function of the stamp
//! every sealed rule takes the stamp it is being evaluated AT
//! a receipt is gated under the ruleset in force when it was written
//! ```
//!
//! No new field, no schema bump, no rehashing, and no historical world
//! changes meaning — which is precisely why replay cannot drift.
//!
//! **Sealed rules are append-only.** A rule is never edited; a new
//! epoch is added beside it and every prior version stays in the tree
//! forever, because some receipt still needs it. That is the standing
//! cost of this design and it is the correct trade — but it means a
//! parameter amendment is *add an epoch*, never *change a number*.

/// Which ruleset governs a deed, derived from the stamp it carries.
///
/// **Ordered and exhaustive.** Adding a variant forces every sealed
/// rule's `match` to decide what it means, which is the amendment
/// discipline made structural — the same reason `scope_of` is
/// exhaustive in the kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Epoch {
    /// Epoch 0 — everything written before the first amendment. The
    /// rules as they stood when the chain began: `ease_of` is the
    /// `== 5` cliff, `demurrage_per_tick` the flat `1/32`, `mint_bound`
    /// the fixed `0·0·0·1·4·7` ladder.
    Genesis,
    /// Epoch 1 — the continuous rules (`21_KERNEL_PARAMETERS` §11e).
    /// Cliffs become exact rational curves and constants become
    /// functions of standing state. Nothing here reaches backward.
    Continuous,
}

/// The activation POINT of [`Epoch::Continuous`].
///
/// Chosen above every stamp any landed world has reached, so the
/// amendment is inert on existing history by construction rather than
/// by inspection. A live chain would fix this by ruling rather than by
/// choosing; in a prototype it is declared here and pinned by witness.
pub const CONTINUOUS_FROM: u64 = 100_000;

/// The ruleset in force at `stamp` — the one lookup every sealed rule
/// consults.
///
/// Total, pure, and monotone: a later stamp never returns an earlier
/// epoch, so a chain cannot walk backwards through its own rules.
pub fn epoch_at(stamp: u64) -> Epoch {
    match stamp >= CONTINUOUS_FROM {
        true => Epoch::Continuous,
        false => Epoch::Genesis,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_boundary_is_exact_and_the_lookup_is_monotone() {
        assert_eq!(epoch_at(0), Epoch::Genesis);
        assert_eq!(epoch_at(CONTINUOUS_FROM - 1), Epoch::Genesis);
        assert_eq!(
            epoch_at(CONTINUOUS_FROM),
            Epoch::Continuous,
            "inclusive at the boundary"
        );
        assert_eq!(epoch_at(u64::MAX), Epoch::Continuous);

        // Monotone: no stamp ever returns an earlier epoch than a
        // smaller one. A chain cannot walk backwards through its rules.
        let mut seen = Epoch::Genesis;
        for stamp in [0u64, 1, 240, 99_999, 100_000, 100_001, 1 << 40] {
            let epoch = epoch_at(stamp);
            assert!(epoch >= seen, "stamp {stamp} went backwards to {epoch:?}");
            seen = epoch;
        }
    }
}
