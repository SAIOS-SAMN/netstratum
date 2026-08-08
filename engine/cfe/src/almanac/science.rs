//! CHEMISTRY SCIENCE — civ utility, dark-7 gravity, isomorphic union.
//!
//! Pure derivations only. The science path is not a free mint of matter:
//! it scores what bonds *for this civilization*, names the dark pulls
//! that drag play toward immediate need, and defines when two gems form
//! an isomorphic union (same geometry family, different content).

use super::periodic::{periodic_of, Periodic, LANES};
use crate::ratio::{ratio, Ratio};

/// **THE DARK SEVEN** — gravity wells toward *immediate* need rather
/// than long-term survival. Codes 1..=7, closed vocabulary.
///
/// Not sins as flavour text: each reweights the chemistry score when
/// the matching condition is live (famine, vanity of surplus, envy of
/// intel, war, ease, glut, pride of depth). Survival utility and dark
/// utility are both exact rationals; the net score is their sum under
/// the active pull weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DarkPull {
    /// Growth greed — prefer shallow high-yield bonds (bank now).
    Green = 1,
    /// Display — prefer rare/high-knowledge gems for show.
    Vanity = 2,
    /// Rivalry — prefer what intel/spies have already touched.
    Envy = 3,
    /// Destruction — prefer high-entropy order-work products.
    Wrath = 4,
    /// Ease — prefer lowest spend / smallest arity.
    Sloth = 5,
    /// Mass — prefer max gem mass regardless of use.
    Gluttony = 6,
    /// Depth for its own sake — ignore civ metabolism, chase lines.
    Pride = 7,
}

impl DarkPull {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Green),
            2 => Some(Self::Vanity),
            3 => Some(Self::Envy),
            4 => Some(Self::Wrath),
            5 => Some(Self::Sloth),
            6 => Some(Self::Gluttony),
            7 => Some(Self::Pride),
            _ => None,
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Vanity => "vanity",
            Self::Envy => "envy",
            Self::Wrath => "wrath",
            Self::Sloth => "sloth",
            Self::Gluttony => "gluttony",
            Self::Pride => "pride",
        }
    }

    pub fn gloss(self) -> &'static str {
        match self {
            Self::Green => "growth now — shallow yield over depth",
            Self::Vanity => "display — rare knowledge for show",
            Self::Envy => "rivalry — chase what others already hold",
            Self::Wrath => "destruction — high-entropy order work",
            Self::Sloth => "ease — cheapest bond that saturates",
            Self::Gluttony => "mass — max gem yield regardless of use",
            Self::Pride => "depth for its own sake — ignore metabolism",
        }
    }
}

/// Sealed-axis inputs for survival scoring (disposition · metabolism ·
/// perception). Codes match genesis wire (1..=3 / 1..=4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CivAxes {
    pub disposition: u8,
    pub metabolism: u8,
    pub perception: u8,
}

/// **SURVIVAL UTILITY** — what this bond does for long-term standing of
/// *this* sealed civilization. Pure: same axes + members → same score.
///
/// - expansionist (1): width — more members / higher mass fuel growth
/// - empiricist (2): knowledge depth (sum of lines)
/// - epicurean (3): surplus enjoyment — mid knowledge, not hoard extremes
/// - mineral (1): prefer deeper lines (pressure work)
/// - biotic (2): prefer mid lines + higher phase diversity proxy
/// - plasmic (3): prefer refractory phase (4) / hot work
pub fn survival_utility(axes: CivAxes, members: &[[u8; 32]], yielded: &Ratio) -> Ratio {
    if members.is_empty() {
        return ratio(0, 1);
    }
    let seats: Vec<Periodic> = members.iter().map(periodic_of).collect();
    let knowledge: i64 = seats.iter().map(|s| i64::from(s.line)).sum();
    let arity = seats.len() as i64;
    let mean_line = if arity > 0 {
        knowledge / arity
    } else {
        0
    };
    let phase = seats.first().map(|s| s.phase).unwrap_or(1);

    // Base: knowledge (empiricist core) + mass yield.
    let mut score = ratio(knowledge, 1) + yielded.clone();
    let two = ratio(1, 1) + ratio(1, 1);

    match axes.disposition {
        1 => {
            // Expansionist — mass and width.
            score += yielded.clone() + ratio(arity, 1);
        }
        2 => {
            // Empiricist — knowledge doubled (1+1).
            score += ratio(knowledge, 1);
        }
        3 if (2..=4).contains(&mean_line) => {
            // Epicurean — prefer mid knowledge (not pride extremes).
            score += two.clone();
        }
        _ => {}
    }

    match axes.metabolism {
        1 => {
            // Mineral — deep lines pay.
            score += ratio(mean_line, 1);
        }
        2 if (2..=5).contains(&mean_line) => {
            // Biotic — mid band.
            score += ratio(1, 1);
        }
        3 if phase == 4 => {
            // Plasmic — refractory phase 4.
            score += two;
        }
        _ => {}
    }

    // Harmonic perception (3): multi-body compounds resonate.
    if axes.perception == 3 && arity > 2 {
        score += ratio(arity - 2, 1);
    }
    score
}

/// **DARK UTILITY** under one pull — how attractive the bond is *as a
/// gravity well*, not as survival. Pure.
pub fn dark_utility(
    pull: DarkPull,
    members: &[[u8; 32]],
    yielded: &Ratio,
    knowledge: u16,
) -> Ratio {
    if members.is_empty() {
        return ratio(0, 1);
    }
    let seats: Vec<Periodic> = members.iter().map(periodic_of).collect();
    let arity = seats.len() as i64;
    let mean_line: i64 = seats.iter().map(|s| i64::from(s.line)).sum::<i64>() / arity.max(1);
    let spend: i64 = seats.iter().map(|s| i64::from(s.binds)).sum();
    match pull {
        DarkPull::Green => {
            // Shallow yield: invert mean line (7 - mean) + mass.
            let shallow = (7i64 - mean_line).max(0);
            ratio(shallow, 1) + yielded.clone()
        }
        DarkPull::Vanity => ratio(i64::from(knowledge), 1) + ratio(mean_line, 1),
        DarkPull::Envy => {
            // Prefers mid-high rarity without needing peer state here;
            // world layer boosts when intel is live.
            ratio(mean_line, 1) + ratio(arity, 1)
        }
        DarkPull::Wrath => {
            // High order-work signature: high spend + multi-body.
            ratio(spend, 1) + ratio(arity, 1)
        }
        DarkPull::Sloth => {
            // Inverse of spend and arity (cheapest wins).
            let ease = (LANES as i64 + 2) - spend.min(LANES as i64) - arity.min(5);
            ratio(ease.max(0), 1)
        }
        DarkPull::Gluttony => yielded.clone() + yielded.clone(),
        DarkPull::Pride => ratio(i64::from(knowledge), 1) + ratio(mean_line * 2, 1),
    }
}

/// Net science score: survival + Σ (weight_p × dark_p).
pub fn net_utility(
    axes: CivAxes,
    members: &[[u8; 32]],
    yielded: &Ratio,
    knowledge: u16,
    pulls: &[(DarkPull, Ratio)],
) -> (Ratio, Ratio, Ratio) {
    let survival = survival_utility(axes, members, yielded);
    let mut dark = ratio(0, 1);
    for (pull, weight) in pulls {
        if *weight > ratio(0, 1) {
            dark += dark_utility(*pull, members, yielded, knowledge) * weight.clone();
        }
    }
    let net = survival.clone() + dark.clone();
    (survival, dark, net)
}

/// **ISOMORPHISM** — two banked gems (or any ids) sit in the same
/// geometry family when shell and phase match. Same family ⇒ they may
/// form an isomorphic union (chemical creation of a higher product
/// without inventing a free periodic cell).
pub fn isomorphic(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let pa = periodic_of(a);
    let pb = periodic_of(b);
    pa.shell == pb.shell && pa.phase == pb.phase
}

/// Product identity of an isomorphic union — content-addressed, order-free.
pub fn iso_union_product(a: &[u8; 32], b: &[u8; 32]) -> Option<[u8; 32]> {
    if !isomorphic(a, b) || a == b {
        return None;
    }
    let mut pair = [*a, *b];
    pair.sort_unstable();
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"netstratum.almanac.iso.v1");
    hasher.update(&pair[0]);
    hasher.update(&pair[1]);
    Some(*hasher.finalize().as_bytes())
}

/// Knowledge won by an iso-union: sum of mean-line proxies (each gem's
/// line) + 1 (the union step). Not free mint of knowledge dial — derived
/// from seats.
pub fn iso_union_knowledge(a: &[u8; 32], b: &[u8; 32]) -> Option<u16> {
    isomorphic(a, b).then(|| {
        let la = u16::from(periodic_of(a).line);
        let lb = u16::from(periodic_of(b).line);
        la.saturating_add(lb).saturating_add(1)
    })
}

/// Recipe for iso-union: spend unit of each gem, yield two of product
/// (mass-exact: 1+1 in → 2 out).
pub fn iso_union_recipe(a: &[u8; 32], b: &[u8; 32]) -> Option<(Ratio, Ratio, [u8; 32])> {
    let product = iso_union_product(a, b)?;
    let one = ratio(1, 1);
    Some((one.clone(), one.clone() + one, product))
}
