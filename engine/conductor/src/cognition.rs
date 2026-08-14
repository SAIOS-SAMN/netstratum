//! COGNITION-1 (operator ruling 2026-08-14): the first tier above a fixed
//! `Policy` if/else — utility scoring for the one decision `build_step`
//! actually has a genuine CHOICE in: Extractor vs Habitat, the two
//! settlement candidates, when BOTH are affordable and gated this beat.
//!
//! This does NOT replace Policy's caps, reserve gates, thermal regimes,
//! forge-starved hold, or industry priority — those encode measured
//! incidents (`build_step`'s own comments) and stay load-bearing exactly
//! as ruled. Utility scoring only breaks the tie that the old code
//! resolved by fixed priority (extractor always won when both wanted).
//!
//! Two axes already sealed elsewhere are reused rather than invented:
//! **disposition** (44_LINEAGES; expansionist/empiricist/epicurean) and
//! **economic mode** (15_TAXON/37_IMPERIUM_TAXON; extractive/balanced/
//! regenerative, from `World::taxon_economy_of`) — the clade weighting
//! the operator asked for is this mode, not a new invented table.
//!
//! The weight literals below (base affinities, the bottleneck pull, the
//! mode nudge, the balance pull) are new BALANCE dials — the same
//! already-acknowledged, operator-gated conductor debt bucket the
//! tuned-constant ratchet (`engine/cfe/tests/tuned_constant_ratchet.rs`)
//! already carries red for `dop.rs`/`vocabulary.rs`/conductor; not a
//! fresh regression, the same owed conversion.

use cfe::almanac::EconomicMode;
use cfe::dop::ZoneKind;
use cfe::ratio::{ratio, Ratio};

/// The settlement decision, given viability (affordability + gate) already
/// resolved upstream. `iron_gap`/`bio_gap` are how far short of the next
/// unlocked-industry gate the line sits (0 when the gate is already met or
/// no gate is pending) — the empiricist reads this as "which resource is
/// the bottleneck to the next discovery."
pub fn settlement_choice(
    extractor_viable: bool,
    habitat_viable: bool,
    disposition: u8,
    mode: EconomicMode,
    ext: i64,
    hab: i64,
    iron_gap: Ratio,
    bio_gap: Ratio,
) -> Option<ZoneKind> {
    match (extractor_viable, habitat_viable) {
        (false, false) => None,
        (true, false) => Some(ZoneKind::Extractor),
        (false, true) => Some(ZoneKind::Habitat),
        (true, true) => {
            let (extractor_u, habitat_u) =
                utility(disposition, mode, ext, hab, iron_gap, bio_gap);
            if extractor_u >= habitat_u {
                Some(ZoneKind::Extractor)
            } else {
                Some(ZoneKind::Habitat)
            }
        }
    }
}

/// COGNITION-2: scores an actual simulated OUTCOME (`lookahead::project`'s
/// `ext`/`hab`/gaps/`famished`, for the one candidate that was actually
/// built on the clone) with the same weight table `settlement_choice`
/// uses — a projection and a live read are never judged by two different
/// laws — plus one veto no static formula can express: an outcome that
/// starves the colony is always worse than one that doesn't, whatever
/// its other weights read. `kind` says which side's score this outcome
/// answers for (the projection already reflects having built it).
pub fn score_outcome(
    kind: ZoneKind,
    disposition: u8,
    mode: EconomicMode,
    ext: i64,
    hab: i64,
    iron_gap: Ratio,
    bio_gap: Ratio,
    famished: bool,
) -> Ratio {
    if famished {
        return ratio(-1_000_000, 1);
    }
    let (extractor_u, habitat_u) = utility(disposition, mode, ext, hab, iron_gap, bio_gap);
    match kind {
        ZoneKind::Extractor => extractor_u,
        ZoneKind::Habitat => habitat_u,
        _ => ratio(0, 1),
    }
}

/// Disposition base weight + economic-mode nudge + bottleneck read.
/// Pure and exact-rational — same inputs always score the same, so a
/// beat-for-beat replay picks the same zone every time.
fn utility(
    disposition: u8,
    mode: EconomicMode,
    ext: i64,
    hab: i64,
    iron_gap: Ratio,
    bio_gap: Ratio,
) -> (Ratio, Ratio) {
    // Base: disposition's own affinity (44_LINEAGES — expansionist
    // reads yield/growth, empiricist reads the discovery bottleneck,
    // epicurean reads food/biomass security).
    let (mut extractor_u, mut habitat_u) = match disposition {
        1 => (ratio(3, 2), ratio(1, 1)), // expansionist: extraction leads
        3 => (ratio(1, 1), ratio(3, 2)), // epicurean: security leads
        _ => (ratio(1, 1), ratio(1, 1)), // empiricist (2) and unminted: even base
    };

    // Empiricist (and, mildly, everyone) weight whichever resource is
    // presently the bottleneck to the next unlocked-industry gate —
    // the same "what is this line starved for" read `hunger()` already
    // uses for forge task selection, applied here to settlement.
    let bottleneck_pull = if disposition == 2 { ratio(1, 1) } else { ratio(1, 4) };
    if iron_gap > ratio(0, 1) {
        extractor_u = extractor_u + bottleneck_pull.clone();
    }
    if bio_gap > ratio(0, 1) {
        habitat_u = habitat_u + bottleneck_pull;
    }

    // Economic mode (clade-derived taxon economy): extractive lines
    // lean further into extraction, regenerative lines lean away from
    // it, balanced (the flatlander baseline) stays neutral — a soft
    // nudge, not a hard cap, so it never overrides an outright shortage.
    let mode_nudge = ratio(1, 4);
    match mode {
        EconomicMode::Extractive => extractor_u = extractor_u + mode_nudge,
        EconomicMode::Regenerative => habitat_u = habitat_u + mode_nudge,
        EconomicMode::Balanced => {}
    }

    // A standing lopsided count pulls the other way — the same
    // ext/hab ratio the policy caps already read, so utility never
    // fights the shape a temperament was built around, only the beats
    // where the temperament itself left the choice open.
    if ext > hab {
        habitat_u = habitat_u + ratio(3, 4);
    } else if hab > ext {
        extractor_u = extractor_u + ratio(3, 4);
    }

    (extractor_u, habitat_u)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_one_viable_ignores_utility_entirely() {
        assert_eq!(
            settlement_choice(true, false, 3, EconomicMode::Regenerative, 0, 0, ratio(0, 1), ratio(0, 1)),
            Some(ZoneKind::Extractor)
        );
        assert_eq!(
            settlement_choice(false, true, 1, EconomicMode::Extractive, 0, 0, ratio(0, 1), ratio(0, 1)),
            Some(ZoneKind::Habitat)
        );
    }

    #[test]
    fn neither_viable_is_none() {
        assert_eq!(
            settlement_choice(false, false, 1, EconomicMode::Balanced, 0, 0, ratio(0, 1), ratio(0, 1)),
            None
        );
    }

    #[test]
    fn expansionist_breaks_an_even_tie_toward_extractor() {
        assert_eq!(
            settlement_choice(true, true, 1, EconomicMode::Balanced, 0, 0, ratio(0, 1), ratio(0, 1)),
            Some(ZoneKind::Extractor)
        );
    }

    #[test]
    fn epicurean_breaks_an_even_tie_toward_habitat() {
        assert_eq!(
            settlement_choice(true, true, 3, EconomicMode::Balanced, 0, 0, ratio(0, 1), ratio(0, 1)),
            Some(ZoneKind::Habitat)
        );
    }

    #[test]
    fn empiricist_follows_the_bottleneck_not_a_fixed_side() {
        // iron short of the smelter gate: the discovery bottleneck is
        // extraction, so the empiricist builds toward it.
        assert_eq!(
            settlement_choice(true, true, 2, EconomicMode::Balanced, 0, 0, ratio(5, 1), ratio(0, 1)),
            Some(ZoneKind::Extractor)
        );
        // biomass short instead: the same empiricist now builds habitat.
        assert_eq!(
            settlement_choice(true, true, 2, EconomicMode::Balanced, 0, 0, ratio(0, 1), ratio(5, 1)),
            Some(ZoneKind::Habitat)
        );
    }

    #[test]
    fn extractive_mode_nudges_an_even_disposition_toward_extractor() {
        assert_eq!(
            settlement_choice(true, true, 2, EconomicMode::Extractive, 0, 0, ratio(0, 1), ratio(0, 1)),
            Some(ZoneKind::Extractor)
        );
    }

    #[test]
    fn regenerative_mode_nudges_an_even_disposition_toward_habitat() {
        assert_eq!(
            settlement_choice(true, true, 2, EconomicMode::Regenerative, 0, 0, ratio(0, 1), ratio(0, 1)),
            Some(ZoneKind::Habitat)
        );
    }

    #[test]
    fn a_lopsided_count_pulls_utility_back_toward_balance() {
        // Expansionist base favours extractor, but ext already far
        // outnumbers hab — the balance pull should be enough to flip it.
        assert_eq!(
            settlement_choice(true, true, 1, EconomicMode::Balanced, 10, 1, ratio(0, 1), ratio(0, 1)),
            Some(ZoneKind::Habitat)
        );
    }

    #[test]
    fn a_famished_outcome_always_scores_worst() {
        // Expansionist base heavily favours extractor, but an outcome
        // that starves the colony must never win regardless.
        let starved = score_outcome(
            ZoneKind::Extractor, 1, EconomicMode::Extractive, 99, 0, ratio(0, 1), ratio(0, 1), true,
        );
        let fed = score_outcome(
            ZoneKind::Habitat, 1, EconomicMode::Extractive, 0, 1, ratio(0, 1), ratio(0, 1), false,
        );
        assert!(fed > starved);
    }

    #[test]
    fn score_outcome_matches_settlement_choice_on_the_same_inputs() {
        // The two entry points must never disagree given the same read
        // — one static formula, whichever candidate is asked about.
        let extractor_u = score_outcome(
            ZoneKind::Extractor, 2, EconomicMode::Balanced, 0, 0, ratio(0, 1), ratio(0, 1), false,
        );
        let habitat_u = score_outcome(
            ZoneKind::Habitat, 2, EconomicMode::Balanced, 0, 0, ratio(0, 1), ratio(0, 1), false,
        );
        let picked = if extractor_u >= habitat_u { ZoneKind::Extractor } else { ZoneKind::Habitat };
        assert_eq!(
            settlement_choice(true, true, 2, EconomicMode::Balanced, 0, 0, ratio(0, 1), ratio(0, 1)),
            Some(picked)
        );
    }
}
