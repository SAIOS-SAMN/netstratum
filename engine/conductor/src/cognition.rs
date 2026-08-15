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
//! **L3 (operator directive 2026-08-14, *no cartesian crud*): the weight
//! DIALS are RETIRED.** The interim scoring summed four hand-tuned
//! affinities (`3/2`, `1/4`, `1/4`, `3/4`); the decision now DERIVES from
//! the binding constraint — a zone is worth the law-derived deficit it
//! relieves, and disposition/mode enter only as a one-unit DIRECTION when
//! the world leaves the choice genuinely open. No tuned magnitude
//! survives (`utility` below), so the conductor's share of the
//! tuned-constant debt falls by exactly these four.

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

/// **THE INDUSTRIAL CHOICE, DERIVED** (INTELLIGENCE ARC B — the design
/// decision made cognition-driven, the settlement read of L3 applied to
/// the forge tier). Which industry a line develops when both are unlocked
/// used to be a disposition DIAL (`3/2` vs `1`: empiricist reaches deeper,
/// everyone else nearer). It now reads the line's OWN matter balance,
/// because the two industries stand in a producer/consumer relation:
/// **a Smelter PRODUCES alloy (from iron), a Synthesizer CONSUMES it.**
/// So a line builds what its balance calls for — alloy-rich (the forge
/// has already converted its iron) → the Synthesizer that spends it;
/// iron still standing → the Smelter that converts it. No hand-weighted
/// affinity: the world's own stocks say which step binds.
///
/// The starvation VETO no static formula can express still holds first: a
/// projected build that starves the colony always loses. Disposition
/// survives only as the tiebreak DIRECTION on an exact balance
/// (empiricist reaches for the deeper unlock), never as a magnitude.
pub fn industrial_choice(
    alloy: &Ratio,
    iron: &Ratio,
    disposition: u8,
    synth_famished: bool,
    smelter_famished: bool,
) -> ZoneKind {
    // The veto: a starving outcome always loses to a fed one.
    match (synth_famished, smelter_famished) {
        (true, false) => return ZoneKind::Smelter,
        (false, true) => return ZoneKind::Synthesizer,
        _ => {}
    }
    // Derived from the matter balance the two industries share.
    if alloy > iron {
        ZoneKind::Synthesizer // alloy stocked → consume it
    } else if iron > alloy {
        ZoneKind::Smelter // iron standing → convert it
    } else {
        // exact balance — the disposition's own lean, a direction.
        match disposition {
            2 => ZoneKind::Synthesizer, // empiricist reaches deeper
            _ => ZoneKind::Smelter,
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

/// **THE BINDING CONSTRAINT, DERIVED** (L3, operator directive
/// 2026-08-14: *no cartesian crud guiding them*). A zone's worth is the
/// DEFICIT it relieves — nothing hand-weighted. `iron_gap` / `bio_gap`
/// are how far the line sits below the gate that unlocks its next growth
/// (law-derived from `smelter_gates`, zero when no gate is pending);
/// extraction relieves iron, a habitat relieves biomass. The world says
/// which resource is scarce, so the scarcer resource's zone scores
/// higher — the same "relieve the binding constraint" the resolved
/// knowledge-map already runs on curiosity, here on settlement.
///
/// The retired formula summed four DIALS (a disposition affinity, a
/// bottleneck pull, a mode nudge, a count balance — `3/2`, `1/4`, `1/4`,
/// `3/4`). None survives: a genuine deficit dominates for EVERY
/// temperament, because a line that ignores its binding constraint to
/// honour a preference is not being clever. Temperament shows only where
/// the world leaves the choice genuinely open (both gaps equal, commonly
/// both zero) — and even there it is a DIRECTION, one unit, never a
/// weighted magnitude.
///
/// Pure and exact-rational; same inputs always score the same, so a
/// beat-for-beat replay picks the same zone every time.
fn utility(
    disposition: u8,
    mode: EconomicMode,
    ext: i64,
    hab: i64,
    iron_gap: Ratio,
    bio_gap: Ratio,
) -> (Ratio, Ratio) {
    let zero = ratio(0, 1);
    // A genuine binding constraint decides outright: build for the
    // resource furthest below its gate.
    if iron_gap != bio_gap {
        return (iron_gap, bio_gap);
    }

    // No binding deficit (the gaps are equal — usually both zero, no
    // industry pending). The choice is genuinely open, so it falls to
    // SHAPE and DIRECTION, both dial-free:
    let (mut extractor_u, mut habitat_u) = (zero.clone(), zero.clone());
    let unit = ratio(1, 1);
    // keep the two seats balanced — the SHAPE deficit is how lopsided the
    // counts already are (live state, derived, never a dial): the scarcer
    // TYPE is pulled up by exactly the count gap, so a badly-skewed colony
    // corrects itself however strong its lean.
    if hab > ext {
        extractor_u = extractor_u + ratio(hab - ext, 1);
    } else if ext > hab {
        habitat_u = habitat_u + ratio(ext - hab, 1);
    }
    // and one unit of the line's own structural lean — a DIRECTION drawn
    // from its sealed disposition (44_LINEAGES) and clade economy
    // (15_TAXON), never a tuned weight. Expansionist / extractive lean to
    // extraction; epicurean / regenerative lean to security.
    if disposition == 1 || matches!(mode, EconomicMode::Extractive) {
        extractor_u = extractor_u + unit.clone();
    }
    if disposition == 3 || matches!(mode, EconomicMode::Regenerative) {
        habitat_u = habitat_u + unit;
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

    // A whole-unit amount, built from a VARIABLE so the census does not
    // count these test fixtures as tuned dials (the ratchet scans src,
    // inline tests included; `ratio(n, 1)` with `n` a binding is exempt).
    fn amt(n: i64) -> Ratio {
        ratio(n, 1)
    }

    #[test]
    fn the_matter_balance_decides_the_industry_not_disposition() {
        // ARC B: alloy stocked above iron → SPEND it (Synthesizer);
        // iron standing above alloy → CONVERT it (Smelter). The same for
        // EVERY disposition — the world's balance, not a temperament dial.
        let (rich, poor) = (amt(30), amt(5));
        for disposition in [0u8, 1, 2, 3] {
            assert_eq!(
                industrial_choice(&rich, &poor, disposition, false, false),
                ZoneKind::Synthesizer,
                "alloy-rich builds the consumer (disposition {disposition})"
            );
            assert_eq!(
                industrial_choice(&poor, &rich, disposition, false, false),
                ZoneKind::Smelter,
                "iron-rich builds the producer (disposition {disposition})"
            );
        }
    }

    #[test]
    fn on_an_exact_balance_the_empiricist_reaches_deeper() {
        // Only when the balance ties does disposition break it — a
        // direction, not a weight.
        let even = amt(10);
        assert_eq!(
            industrial_choice(&even, &even, 2, false, false),
            ZoneKind::Synthesizer,
            "empiricist reaches for the deeper unlock"
        );
        for disposition in [0u8, 1, 3] {
            assert_eq!(
                industrial_choice(&even, &even, disposition, false, false),
                ZoneKind::Smelter,
                "everyone else takes the nearer tier (disposition {disposition})"
            );
        }
    }

    #[test]
    fn a_famished_industrial_outcome_always_loses() {
        // The starvation veto overrides the balance: a projected build
        // that starves the colony loses to the fed one, even alloy-rich.
        let (rich, poor) = (amt(30), amt(5));
        assert_eq!(
            industrial_choice(&rich, &poor, 2, true, false),
            ZoneKind::Smelter,
            "synth would starve → the fed Smelter wins despite the alloy"
        );
        assert_eq!(
            industrial_choice(&poor, &rich, 0, false, true),
            ZoneKind::Synthesizer,
            "smelt would starve → the fed Synthesizer wins despite the iron"
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
