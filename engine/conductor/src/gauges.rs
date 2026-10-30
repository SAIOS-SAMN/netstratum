//! The four gauges of A people (Xylarium 11.0 (37); ruling of 2026-10-21/22:
//! never boolean; a gauge, not a boolean or a straight ladder, reversible by
//! the actions of the species; adopted and read from Netstratum's own
//! universe). The law is Xylarium's (xyl-cosmos `peoples.rs`); Netstratum
//! reads each term from what its own universe and its own lines already
//! hold. Every gauge is a share in [0, 1], re-read from the line's present
//! state whenever it is read (each year's close), never latched, gating
//! nothing.
//!
//! - **Instinct** (eating to survive, reproducing to have kin):
//!   intake over intake and need, `E_in/(E_in + E_need)`. Intake is what its
//!   world renews for it (the land's wild flow, its star's light taken up by
//!   its medium, `syntropic_flow`) plus what its works add
//!   (`cultivated_yield`); need is its people times the ration each eats.
//! - **Intelligence** (barely understanding what it is): its record over
//!   what is knowable, the distinct kinds of matter it holds against those on
//!   its world and on the worlds within its horizon (their compositions).
//! - **Sentience** (aware, without the capability to act):
//!   its horizon over its sky. Its sky is its own world, its star's other
//!   worlds and its galaxy's stars; its horizon the targets it reaches by what
//!   it does this term (its world, and the worlds and stars of the peoples it
//!   trades with), so a lapse of trade narrows it.
//! - **Consciousness** (conscious of its planet and resources, acting
//!   accordingly): `A·k`, A the share of its world's flow its works direct,
//!   `works/(works + renewal)`, and k its draw in step with its world's
//!   renewal, `1 − |d − ρ|/(d + ρ)`.
//! - **The singular gauge**: `Γ = [(1 + g_I)(1 + g_N)(1 + g_S)(1 + g_C) − 1]/15`.
use crate::Polity;
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;
use std::collections::BTreeSet;

/// A people's four gauges and its singular gauge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gauges {
    pub instinct: Ratio,
    pub intelligence: Ratio,
    pub sentience: Ratio,
    pub consciousness: Ratio,
    pub gamma: Ratio,
}

/// `a/(a + b)`, zero where both are zero.
fn share(a: &Ratio, b: &Ratio) -> Ratio {
    let both = a.clone() + b.clone();
    if both.is_zero() {
        return ratio(0, 1);
    }
    a.clone() / both
}

/// `a/b` held within one, zero where b is zero.
fn within(a: &Ratio, b: &Ratio) -> Ratio {
    if b.is_zero() {
        return ratio(0, 1);
    }
    (a.clone() / b.clone()).min(ratio(1, 1))
}

/// The distinct Zs of a composition.
fn kinds(composition: Option<&Vec<(u16, Ratio)>>) -> BTreeSet<u16> {
    composition.map(|c| c.iter().filter(|(_, a)| *a > ratio(0, 1)).map(|(z, _)| *z).collect()).unwrap_or_default()
}

/// Read a line's gauges: `partners`, the era-1 world paths of the lines it
/// traded with this term.
pub fn read(p: &Polity, partners: &[Vec<u64>]) -> Gauges {
    let w = &p.world;
    let orb = cfe::world::FIRST_ORB;
    let need = ratio(w.population_of(orb) as i64, 1) * w.per_capita_ration();
    let renewal = w.syntropic_flow(orb);
    let works = w.cultivated_yield(orb);
    let intake = renewal.clone() + works.clone();
    let instinct = share(&intake, &need);
    // Its draw: what it eats, never more than its intake.
    let draw = need.clone().min(intake.clone());
    let both = draw.clone() + renewal.clone();
    let k = if both.is_zero() {
        ratio(0, 1)
    } else {
        let gap = draw.clone() - renewal.clone();
        ratio(1, 1) - if gap < ratio(0, 1) { -gap } else { gap } / both
    };
    let consciousness = share(&works, &renewal) * k;
    // Its sky and its horizon, read from where it stands.
    let (sentience, knowable) = match p.spec.place.as_deref() {
        Some(here) if here.len() > 2 => {
            let (g, s) = (here[0], here[1]);
            let sky = crate::places::sky(here);
            let mut reached: BTreeSet<Vec<u64>> = BTreeSet::new();
            reached.insert(here.to_vec());
            let mut known = kinds(p.spec.composition.as_ref());
            for other in partners.iter().filter(|o| o.len() > 2 && o.first() == Some(&g)) {
                reached.insert(other.clone());
                if other[1] != s {
                    reached.insert(other[..2].to_vec());
                }
                known.extend(kinds(crate::places::site(other).and_then(|site| site.composition).as_ref()));
            }
            let horizon = ratio(i64::try_from(reached.len()).unwrap_or(i64::MAX), 1);
            (within(&horizon, &ratio(i64::try_from(sky).unwrap_or(i64::MAX), 1)), known.len())
        }
        _ => (ratio(0, 1), kinds(p.spec.composition.as_ref()).len()),
    };
    let record = w.orb().map_or(0, |o| o.elements.iter().filter(|(_, a)| *a > ratio(0, 1)).count());
    let intelligence = within(&ratio(record as i64, 1), &ratio(knowable as i64, 1));
    let one = ratio(1, 1);
    let product = (one.clone() + instinct.clone()) * (one.clone() + intelligence.clone()) * (one.clone() + sentience.clone()) * (one.clone() + consciousness.clone());
    let gamma = (product - one) / ratio(15, 1);
    Gauges { instinct, intelligence, sentience, consciousness, gamma }
}

#[cfg(test)]
mod tests {
    use super::{share, within};
    use cfe::ratio::ratio;

    #[test]
    fn a_gauge_is_a_share_that_falls_as_well_as_rises() {
        // Instinct: intake 3 against need 1 reads 3/4; a famine that halves
        // intake lowers it, never latched.
        assert_eq!(share(&ratio(3, 1), &ratio(1, 1)), ratio(3, 4));
        assert_eq!(share(&ratio(3, 2), &ratio(1, 1)), ratio(3, 5));
        assert_eq!(share(&ratio(0, 1), &ratio(0, 1)), ratio(0, 1));
        // A record past what is knowable reads full, never past it.
        assert_eq!(within(&ratio(5, 1), &ratio(4, 1)), ratio(1, 1));
    }
}
