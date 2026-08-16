//! **SOC-1 — THE OPEN WORKBENCH** (operator directive 2026-08-16: *Statistical
//! Mechanics of Infrastructure*).
//!
//! The Mind's cognition has been free since COG-7 (`solver::instinct::choose` —
//! a chronicle-seeded proportional draw over generator families). Its CIVIC
//! development was not: `build_step` picked the next building by a fixed `.or()`
//! priority (industry over refine over settlement, always, for every
//! temperament), so a whale-society and a lithophage-society raised the same
//! city in the same order. SOC-1 points the same stochastic methodology at the
//! build workbench.
//!
//! **The partition function.** During SURPLUS the choice of infrastructure is a
//! probability distribution over the valid state space `S_valid` (the buildings
//! that pass their tech-unlock + affordability gate — computed upstream in
//! `build_step`, unchanged). Each candidate `s` carries a dispositional affinity
//! weight `w_s`; the selection probability is
//!
//! ```text
//!   P(s) = w_s / Z,     Z = Σ_{j ∈ S_valid} w_j
//! ```
//!
//! **The Heaviside veto.** Societal disposition never overrides biological
//! thermodynamics. When a critical deficit exists (famine, worn-vein ecology
//! collapse, thermal runaway, a Frost founding without its sink) `build_step`
//! collapses the space onto the survival response BEFORE this draw is ever
//! reached — those crisis arms stay deterministic. So the ensemble explores its
//! dispositional phase space in surplus and is strictly canalized by physical
//! law in crisis, exactly as `build_step` already gates.
//!
//! **Deterministic niche construction.** The draw is seeded by the line's
//! `chronicle_digest` — a hash of its exact timeline — so two identical species
//! under identical constraints construct identical environments, and a
//! beat-for-beat replay is invariant. Freedom that is still exact-rational and
//! replayable: the `ℤ/ℚ` requirement holds while the architecture grows
//! emergent and non-linear.

use cfe::dop::ZoneKind;

/// **THE BUILD-FAMILY AFFINITY** — the SOC-1 seed, a `genesis_instinct` analog
/// over the build workbench (mirrors `solver::instinct::genesis_instinct`'s
/// shape: a brainstem baseline of one unit on every viable option — *nothing
/// affordable is unthinkable* — plus one extra unit on the family this line's
/// DISPOSITION favours). The disposition codes are the sealed `44_LINEAGES`
/// axis, the same `u8` `cognition::utility` already reads:
///
/// - **1 expansionist** leans to SPRAWL — the Extractor that opens new ground.
/// - **2 empiricist** leans to DEPTH — the industry/refinement tier that turns
///   research into structure (Smelter / Synthesizer / ElementRefinery).
/// - **3 epicurean** leans to SECURITY — the Habitat and its Regenerator.
///
/// A disposition that names none keeps the flat baseline (every option weight
/// 1). No `ratio` dials — integer weights only; `choose_build` needs their
/// proportions, and the common baseline cancels in the draw.
pub fn build_affinity(disposition: u8, kind: ZoneKind) -> u64 {
    let favoured = match disposition {
        1 => matches!(kind, ZoneKind::Extractor),
        2 => matches!(
            kind,
            ZoneKind::Smelter | ZoneKind::Synthesizer | ZoneKind::ElementRefinery
        ),
        3 => matches!(kind, ZoneKind::Habitat | ZoneKind::Regenerator),
        _ => false,
    };
    1 + favoured as u64
}

/// **THE WORKBENCH CHOOSER** — the civic partition-function draw, the `ZoneKind`
/// analog of `solver::instinct::choose`. Over the surplus-phase candidate set
/// `S_valid` (every viable building this beat, membership already resolved by
/// `build_step`'s gates), weight each by `build_affinity` and sample `P(s) =
/// w_s / Z` from the line's `chronicle_digest`. Heavier families are likelier;
/// the same seed always draws the same building (replayable, divergent per
/// line). `None` only when the candidate set is empty — a beat with nothing
/// viable builds nothing, exactly as the old `industrial.or(refine).or(base)`
/// returned `None`.
///
/// The domain tag `b"workbench"` separates this draw from the Mind's
/// `b"instinct"` draw on the same digest, so a line's building choice and its
/// next thought are independent samples of its history, not the same coin.
pub fn choose_build(candidates: &[ZoneKind], disposition: u8, seed: [u8; 32]) -> Option<ZoneKind> {
    let total: u64 = candidates
        .iter()
        .fold(0u64, |sum, k| sum.saturating_add(build_affinity(disposition, *k)));
    if total == 0 {
        return None;
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(b"workbench");
    let block = *hasher.finalize().as_bytes();
    let roll = u64::from_le_bytes(block[0..8].try_into().expect("32 >= 8")) % total;
    let mut acc = 0u64;
    for kind in candidates {
        acc = acc.saturating_add(build_affinity(disposition, *kind));
        if roll < acc {
            return Some(*kind);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed(n: u8) -> [u8; 32] {
        *blake3::hash(&[n]).as_bytes()
    }

    #[test]
    fn the_baseline_leaves_nothing_affordable_unthinkable() {
        // Every build carries at least one unit of weight for every
        // disposition — the brainstem baseline, so a viable option is never
        // impossible, only less likely.
        for disposition in 0u8..4 {
            for kind in [
                ZoneKind::Extractor,
                ZoneKind::Habitat,
                ZoneKind::Smelter,
                ZoneKind::Synthesizer,
                ZoneKind::ElementRefinery,
                ZoneKind::Regenerator,
            ] {
                assert!(build_affinity(disposition, kind) >= 1);
            }
        }
    }

    #[test]
    fn disposition_leans_to_its_family_but_only_by_a_direction() {
        // Empiricist weights DEPTH above sprawl; expansionist the reverse;
        // epicurean weights security. The lean is exactly one unit — a
        // direction, never a dial that makes the other unthinkable.
        assert!(build_affinity(2, ZoneKind::Smelter) > build_affinity(2, ZoneKind::Extractor));
        assert!(build_affinity(1, ZoneKind::Extractor) > build_affinity(1, ZoneKind::Smelter));
        assert!(build_affinity(3, ZoneKind::Habitat) > build_affinity(3, ZoneKind::Extractor));
        assert_eq!(
            build_affinity(2, ZoneKind::Smelter) - build_affinity(2, ZoneKind::Extractor),
            1
        );
    }

    #[test]
    fn a_single_candidate_is_always_taken() {
        // S_valid of size one collapses the partition function to certainty.
        for n in 0u8..8 {
            assert_eq!(
                choose_build(&[ZoneKind::Habitat], 1, seed(n)),
                Some(ZoneKind::Habitat)
            );
        }
    }

    #[test]
    fn an_empty_surface_builds_nothing() {
        assert_eq!(choose_build(&[], 2, seed(0)), None);
    }

    #[test]
    fn the_draw_is_replayable_same_digest_same_build() {
        let surface = [ZoneKind::Extractor, ZoneKind::Habitat, ZoneKind::Smelter];
        let s = seed(7);
        assert_eq!(
            choose_build(&surface, 2, s),
            choose_build(&surface, 2, s),
            "same digest → same building"
        );
    }

    #[test]
    fn the_workbench_is_open_disposition_biases_but_does_not_gate() {
        // THE SOC-1 LAW, WITNESSED: an empiricist line facing the same open
        // surface (industry vs sprawl) reaches for industry MORE OFTEN than an
        // expansionist line does — the workbench is weighted, not gated. And
        // neither is locked out of the other's family: across a spread of
        // histories both buildings occur. This is the freedom-with-a-lean the
        // fixed `.or()` priority never had.
        let surface = [ZoneKind::Smelter, ZoneKind::Extractor];
        let empiricist_industry = (0..90u8)
            .filter(|i| choose_build(&surface, 2, seed(*i)) == Some(ZoneKind::Smelter))
            .count();
        let expansionist_industry = (0..90u8)
            .filter(|i| choose_build(&surface, 1, seed(*i)) == Some(ZoneKind::Smelter))
            .count();
        assert!(
            empiricist_industry > expansionist_industry,
            "the empiricist reaches for industry more often ({empiricist_industry} vs {expansionist_industry})"
        );
        // The workbench is genuinely OPEN, not a switch: even the expansionist,
        // whose lean is against industry, still builds a Smelter sometimes.
        assert!(
            expansionist_industry > 0 && empiricist_industry < 90,
            "both families remain reachable for both dispositions ({expansionist_industry}, {empiricist_industry})"
        );
    }
}
