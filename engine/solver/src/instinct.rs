//! **The evolved instinct** (COG-7 — the Workbench chooser).
//!
//! A civilization's history reshapes the probability space of its cognition.
//! Two layers, and they flow into one another with no new ledger — the fold is
//! over data the library already holds:
//!
//! - **Basic instinct (the genesis seed).** The species' irreducible prior — a
//!   base weight per generator family, from its disposition. It keeps every
//!   option minimally alive on beat 1 (the brainstem baseline), so a founding
//!   band already knows how to reach before it has minted anything.
//!
//! - **Evolved instinct (stochastic accretion).** Every word the line has
//!   minted was built from generators (operator families) and confirmed against
//!   obstructions (`cited`). The tally of that cited usage, per family, is the
//!   rut the line's own timeline has carved: a line that survived by minting one
//!   kind of geometry reaches first for that kind against the next problem, even
//!   an unrelated one. Its past has rewritten its instinct.
//!
//! ```text
//!   I_total(family) = I_genesis(family) + Σ_{w spells family}  |w.cited|
//! ```
//!
//! Early game the genesis seed is all there is; late game the accreted weight of
//! the minted words drowns it out, and behaviour is dominated by the idiosyncratic
//! structures the line invented to survive its own star system.
//!
//! **The law, unbroken** (`need.rs`): this only ORDERS/WEIGHTS the reach — a bias
//! on which family the Workbench tries first. The observation still supplies every
//! candidate; instinct never invents an operator the observation cannot support.

use crate::need::Family;

/// The four generator families, in declaration order — the axes an instinct
/// weights. (Mirrors `selfmap`'s private census order.)
const FAMILIES: [Family; 4] = [Family::Symmetry, Family::Recolor, Family::Tile, Family::Object];

/// **The workbench chooser** — a chronicle-seeded proportional draw over the
/// instinct weights: the family the Mind reaches for first this beat, sampled
/// from the distribution its whole history has shaped. Heavier families are
/// likelier; the same seed always draws the same family (replayable, divergent
/// per line). `None` only when every weight is zero (never, once the genesis
/// seed is present). The seed is the line's latest `chronicle_digest`, so the
/// draw carries the line's trauma and triumph — two lines with the same
/// vocabulary facing the same deficit still guess differently if their
/// histories differ (the digest is a hash chain of their exact past).
pub fn choose(weights: &[(Family, u64)], seed: [u8; 32]) -> Option<Family> {
    let total: u64 = weights.iter().fold(0u64, |sum, (_, w)| sum.saturating_add(*w));
    if total == 0 {
        return None;
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(b"instinct");
    let block = *hasher.finalize().as_bytes();
    let roll = u64::from_le_bytes(block[0..8].try_into().expect("32 >= 8")) % total;
    let mut acc = 0u64;
    for (family, w) in weights {
        acc = acc.saturating_add(*w);
        if roll < acc {
            return Some(*family);
        }
    }
    None
}

/// **The stress blend** (COG-7 — the Explore/Exploit tradeoff, thermodynamically
/// gated). Two opposing biases: the thick habit (`instinct`, reach for the rut)
/// and the thin curiosity (`curiosity`, the `knowledge_order` frontier — reach
/// for what is lacking). Curiosity is a high-entropy, expensive search that only a
/// line with surplus can afford; instinct is the low-energy path of least
/// homological resistance. So the composition is gated by thermodynamic stress:
///
/// ```text
///   W = σ · Î + (1 − σ) · Ĉ        (over NORMALISED distributions)
/// ```
///
/// σ → 1 (starving, pinned at the syntropic limit, existential γ): the draw
/// collapses onto instinct — a stressed colony desperately reaches for the tools
/// that saved it before, culturally rigid. σ → 0 (full granaries, banked
/// leisure): the draw favours curiosity — a prosperous empire has the metabolic
/// luxury to explore its frontier and diversify.
///
/// Exact-integer: σ arrives quantised as `sigma_num/sigma_den` (the caller
/// clamps `sigma_num ≤ sigma_den`). Each distribution is normalised to its own
/// total, then the blend is cleared to a common denominator so the returned
/// weights are exact integers — `choose` only needs their ratios, and the shared
/// denominator cancels in the draw.
pub fn blend(
    instinct: &[(Family, u64)],
    curiosity: &[Family],
    sigma_num: u64,
    sigma_den: u64,
) -> Vec<(Family, u64)> {
    let i_total: u64 = instinct.iter().map(|(_, w)| *w).sum::<u64>().max(1);
    let n = curiosity.len().max(1) as u64;
    // Curiosity ranks: thinnest-first (position 0) is the most peripheral, so it
    // takes the largest rank weight `n`; the sum of ranks is the triangular n.
    let c_total = (n * (n + 1) / 2).max(1);
    let sn = sigma_num.min(sigma_den); // clamp σ ≤ 1
    let inv = sigma_den - sn; // (1 − σ) numerator over the same denom
    FAMILIES
        .iter()
        .map(|f| {
            let i_f = instinct.iter().find(|(g, _)| g == f).map(|(_, w)| *w).unwrap_or(0);
            let c_f = curiosity.iter().position(|g| g == f).map(|p| n - p as u64).unwrap_or(0);
            // W_f × (σ_den · i_total · c_total) = σ_n·i_f·c_total + inv·c_f·i_total.
            // The bracket is a shared constant across families — it cancels in
            // the proportional draw, so these integers are the blended weights.
            let habit = sn.saturating_mul(i_f).saturating_mul(c_total);
            let frontier = inv.saturating_mul(c_f).saturating_mul(i_total);
            (*f, habit.saturating_add(frontier))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    

    #[test]
    fn stress_blends_exploit_against_explore() {
        use Family::*;
        // A line whose history is all recolor (the rut), but whose thin frontier
        // (curiosity) points at object (the family it lacks).
        let instinct = vec![(Symmetry, 1u64), (Recolor, 40), (Tile, 1), (Object, 1)];
        let curiosity = vec![Object, Tile, Symmetry, Recolor]; // thinnest-first

        // σ = 1 (starving): collapse onto the rut — Recolor must be the heaviest.
        let stressed = blend(&instinct, &curiosity, 1, 1);
        let top = |w: &[(Family, u64)]| w.iter().max_by_key(|(_, x)| *x).map(|(f, _)| *f).unwrap();
        assert_eq!(top(&stressed), Recolor, "a starving line reaches for its rut");

        // σ = 0 (sated): follow curiosity — the thin frontier Object leads.
        let sated = blend(&instinct, &curiosity, 0, 1);
        assert_eq!(top(&sated), Object, "a prosperous line explores its frontier");

        // σ = 1/2: the rut still shows through (Recolor's 40 is large), but the
        // frontier has pulled real weight onto Object — the blend is a genuine
        // mix, not a switch.
        let mixed = blend(&instinct, &curiosity, 1, 2);
        let w = |f| mixed.iter().find(|(g, _)| *g == f).map(|(_, x)| *x).unwrap();
        assert!(w(Object) > w(Tile), "at half stress the frontier lifts Object above its habit weight");
        assert!(w(Recolor) > 0, "the rut is still present in the blend");
    }

}
