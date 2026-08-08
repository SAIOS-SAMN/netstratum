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

// ── SEATED SCIENCE · LAB CREATION · EXPLOSION ─────────────────────

/// Science discipline seats — paid unlocks under the bench.
/// Not BAND-1 offices; capability seats on an orb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ScienceDiscipline {
    Chem = 1,
    Biology = 2,
    Physics = 3,
    Math = 4,
    Research = 5,
    /// Cybernetic / synthetic mods (matrix-risk path).
    Synthetic = 6,
    /// Bio clones — population control.
    Clone = 7,
    /// Skin / face mods (Tleilaxu-shaped: disguise + identity play).
    Skin = 8,
}

impl ScienceDiscipline {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Chem),
            2 => Some(Self::Biology),
            3 => Some(Self::Physics),
            4 => Some(Self::Math),
            5 => Some(Self::Research),
            6 => Some(Self::Synthetic),
            7 => Some(Self::Clone),
            8 => Some(Self::Skin),
            _ => None,
        }
    }

    pub fn as_wire(self) -> u8 {
        self as u8
    }

    pub fn word(self) -> &'static str {
        match self {
            Self::Chem => "chem",
            Self::Biology => "biology",
            Self::Physics => "physics",
            Self::Math => "math",
            Self::Research => "research",
            Self::Synthetic => "synthetic",
            Self::Clone => "clone",
            Self::Skin => "skin",
        }
    }

    pub fn from_word(word: &str) -> Option<Self> {
        match word {
            "chem" | "chemistry" | "alchemy" => Some(Self::Chem),
            "bio" | "biology" | "life" | "biologic" => Some(Self::Biology),
            "phys" | "physics" | "physical" => Some(Self::Physics),
            "math" | "maths" | "mathematics" | "ratios" => Some(Self::Math),
            "research" | "lab" | "constellation" => Some(Self::Research),
            "synth" | "synthetic" | "cyber" | "cybernetic" => Some(Self::Synthetic),
            "clone" | "clones" | "cloning" => Some(Self::Clone),
            "skin" | "face" | "tleilax" | "tleilaxu" | "thalaxu" | "mask-mod" => Some(Self::Skin),
            _ => None,
        }
    }

    pub fn gloss(self) -> &'static str {
        match self {
            Self::Chem => "matter bonds · catalog · iso-union",
            Self::Biology => "life · biomass · habitat · regen",
            Self::Physics => "star · thermal · medium · site",
            Self::Math => "ratios · thresholds · sealed books",
            Self::Research => "constellation · forge · practice",
            Self::Synthetic => "cybernetic mods · mesh integrity · rogue risk",
            Self::Clone => "bio clones · population control · identity strain",
            Self::Skin => "skin/face mods · disguise · unmask residual",
        }
    }

    /// Seat fee multiplier (× resonator eighth). Derived from wire code — no free dial.
    pub fn seat_weight(self) -> i64 {
        i64::from(self as u8)
    }

    /// Max rank (1 = read/basic act, 2 = creation with dual outcomes).
    pub fn max_rank(self) -> u8 {
        2
    }
}

/// Lab creation path — what you *make* once seats are paid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LabPath {
    /// Chemistry bond / catalog / iso (needs Chem seat).
    ChemWork = 1,
    /// Bio+chem hybrid work (needs Biology + Chem).
    BioChem = 2,
    /// Synthetic cybernetic mod (needs Synthetic + Math; Physics helps risk).
    SynthMod = 3,
    /// Bio clone for population (needs Clone + Biology).
    CloneWork = 4,
    /// Skin / face modification (needs Skin + Biology or Synthetic).
    SkinWork = 5,
}

impl LabPath {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::ChemWork),
            2 => Some(Self::BioChem),
            3 => Some(Self::SynthMod),
            4 => Some(Self::CloneWork),
            5 => Some(Self::SkinWork),
            _ => None,
        }
    }

    pub fn as_wire(self) -> u8 {
        self as u8
    }

    pub fn word(self) -> &'static str {
        match self {
            Self::ChemWork => "chem",
            Self::BioChem => "biochem",
            Self::SynthMod => "synth",
            Self::CloneWork => "clone",
            Self::SkinWork => "skin",
        }
    }

    pub fn from_word(word: &str) -> Option<Self> {
        match word {
            "chem" | "chemistry" | "bond" | "catalog" => Some(Self::ChemWork),
            "biochem" | "hybrid" | "bio-chem" | "biocross" => Some(Self::BioChem),
            "synth" | "synthetic" | "cyber" | "mod" => Some(Self::SynthMod),
            "clone" | "cloning" | "population" => Some(Self::CloneWork),
            "skin" | "face" | "mask" | "tleilax" => Some(Self::SkinWork),
            _ => None,
        }
    }

    /// Disciplines that must be seated (rank ≥ 1) to attempt this path.
    pub fn required_seats(self) -> &'static [ScienceDiscipline] {
        match self {
            Self::ChemWork => &[ScienceDiscipline::Chem],
            Self::BioChem => &[ScienceDiscipline::Biology, ScienceDiscipline::Chem],
            Self::SynthMod => &[ScienceDiscipline::Synthetic, ScienceDiscipline::Math],
            Self::CloneWork => &[ScienceDiscipline::Clone, ScienceDiscipline::Biology],
            Self::SkinWork => &[ScienceDiscipline::Skin, ScienceDiscipline::Biology],
        }
    }

    /// Base explosion risk weight (exact rational), before dark/survival.
    /// Eighth-derived steps — not free dials.
    pub fn base_risk(self) -> Ratio {
        match self {
            Self::ChemWork => ratio(1, 8),
            Self::BioChem => ratio(1, 4),
            Self::SynthMod => ratio(3, 8),
            Self::CloneWork => ratio(1, 4),
            Self::SkinWork => ratio(1, 4) + ratio(1, 8),
        }
    }

    /// Known positive outcome (always declared at creation).
    pub fn known_positive(self) -> (&'static str, &'static str) {
        match self {
            Self::ChemWork => ("gem/catalog progress", "known chemistry deepens the catalog"),
            Self::BioChem => ("hybrid yield", "bio+chem product feeds survival utility"),
            Self::SynthMod => ("cyber integrity", "detection/mesh tools improve"),
            Self::CloneWork => ("population seat", "clone count rises for habitat load"),
            Self::SkinWork => ("disguise grade", "skin mod aids veil / diplomatic mask"),
        }
    }

    /// Known negative outcome (always declared — the price of the work).
    pub fn known_negative(self) -> (&'static str, &'static str) {
        match self {
            Self::ChemWork => ("order-work entropy", "compounding vents entropy / crystal fee"),
            Self::BioChem => ("contamination toll", "biomass spent; contamination residual risk"),
            Self::SynthMod => ("identity shear", "synthetic graft taxes crystal + trust"),
            Self::CloneWork => ("identity dilution", "clone pool strains habitat / kinship"),
            Self::SkinWork => ("unmask debt", "disguise leaves a residual if pierced"),
        }
    }

    /// Hidden positive — not shown unless math seat or empiricist perception.
    pub fn hidden_positive(self) -> (&'static str, &'static str) {
        match self {
            Self::ChemWork => ("dark-well map", "successful work maps a dark pull for later"),
            Self::BioChem => ("metabolic fit", "biotic metabolism scores higher next bonds"),
            Self::SynthMod => ("mesh control", "hidden command channel on synthetic limbs"),
            Self::CloneWork => ("escort pool", "clones feed KIN escort readiness"),
            Self::SkinWork => ("face-dance license", "hidden leap toward diplomatic/espionage mask"),
        }
    }

    /// Hidden negative — revealed on explosion (or dark-led work).
    pub fn hidden_negative(self) -> (&'static str, &'static str) {
        match self {
            Self::ChemWork => ("runaway reaction", "explosion: gem loss + crystal slash"),
            Self::BioChem => ("cascade famine seed", "explosion: biomass slash + cascade pressure"),
            Self::SynthMod => ("rogue faction", "explosion: matrix residual — synth turns"),
            Self::CloneWork => ("clone revolt", "explosion: identity residual + want struck"),
            Self::SkinWork => ("unmask catastrophe", "explosion: veil fails; spy marks reverse"),
        }
    }

    pub fn explosion_kind(self) -> ExplosionKind {
        match self {
            Self::ChemWork => ExplosionKind::ChemRunaway,
            Self::BioChem => ExplosionKind::BioChemCross,
            Self::SynthMod => ExplosionKind::SynthRogue,
            Self::CloneWork => ExplosionKind::CloneRevolt,
            Self::SkinWork => ExplosionKind::SkinUnmask,
        }
    }
}

/// Typed explosion residual (wire code 1..=5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExplosionKind {
    ChemRunaway = 1,
    BioChemCross = 2,
    SynthRogue = 3,
    CloneRevolt = 4,
    SkinUnmask = 5,
}

impl ExplosionKind {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::ChemRunaway),
            2 => Some(Self::BioChemCross),
            3 => Some(Self::SynthRogue),
            4 => Some(Self::CloneRevolt),
            5 => Some(Self::SkinUnmask),
            _ => None,
        }
    }

    pub fn as_wire(self) -> u8 {
        self as u8
    }

    pub fn word(self) -> &'static str {
        match self {
            Self::ChemRunaway => "chem-runaway",
            Self::BioChemCross => "biochem-cross",
            Self::SynthRogue => "synth-rogue",
            Self::CloneRevolt => "clone-revolt",
            Self::SkinUnmask => "skin-unmask",
        }
    }

    pub fn gloss(self) -> &'static str {
        match self {
            Self::ChemRunaway => "bad chemistry — runaway bond",
            Self::BioChemCross => "bio+chem contamination cascade",
            Self::SynthRogue => "synthetic turns — rogue faction (matrix)",
            Self::CloneRevolt => "clone identity revolt",
            Self::SkinUnmask => "skin mod pierced — face revealed",
        }
    }
}

/// **SEAT FEE** — resonator eighth × discipline weight × rank.
/// Pure: same inputs → same fee. No free dial.
pub fn science_seat_fee(discipline: ScienceDiscipline, rank: u8) -> Ratio {
    let eighth = ratio(1, 8); // same grain as resonator yield law; world multiplies if needed
    let w = discipline.seat_weight();
    let r = i64::from(rank.max(1));
    eighth * ratio(w, 1) * ratio(r, 1)
}

/// **EXPLOSION RISK** — exact rational, pure.
///
/// ```text
/// risk = path.base
///      + max(0, dark − survival)          // dark-led work is unstable
///      + unseated_cross / 8               // missing support seats
///      + (order_thin ? 1/8 : 0)
/// ```
///
/// Explodes when `risk ≥ 1/2` (half — identity of two resonator eighths).
pub fn explosion_risk(
    path: LabPath,
    dark: &Ratio,
    survival: &Ratio,
    unseated_cross: u8,
    order_thin: bool,
) -> Ratio {
    let mut risk = path.base_risk();
    if dark > survival {
        risk += dark.clone() - survival.clone();
    }
    if unseated_cross > 0 {
        risk += ratio(i64::from(unseated_cross), 8);
    }
    if order_thin {
        risk += ratio(1, 8);
    }
    risk
}

/// Half threshold — two resonator eighths (derived identity).
pub fn explosion_threshold() -> Ratio {
    ratio(1, 2)
}

pub fn explodes(risk: &Ratio) -> bool {
    *risk >= explosion_threshold()
}

/// Whether hidden positive poles are revealed to the operator *before* create.
/// Empiricist perception (2) or Math seat rank ≥ 1.
pub fn reveals_hidden_positive(perception: u8, math_rank: u8) -> bool {
    perception == 2 || math_rank >= 1
}

/// Whether hidden negative poles are previewed (not only post-explosion).
/// Dark leading, or Envy/Wrath pull live (caller passes flag).
pub fn reveals_hidden_negative(dark_led: bool, envy_or_wrath: bool) -> bool {
    dark_led || envy_or_wrath
}

/// Full pole card for UI / receipts (known always; hidden conditional).
#[derive(Debug, Clone)]
pub struct LabPoleCard {
    pub path: LabPath,
    pub known_pos: (&'static str, &'static str),
    pub known_neg: (&'static str, &'static str),
    pub hidden_pos: Option<(&'static str, &'static str)>,
    pub hidden_neg: Option<(&'static str, &'static str)>,
    pub risk: Ratio,
    pub will_explode: bool,
}

pub fn lab_pole_card(
    path: LabPath,
    dark: &Ratio,
    survival: &Ratio,
    unseated_cross: u8,
    order_thin: bool,
    perception: u8,
    math_rank: u8,
    envy_or_wrath: bool,
) -> LabPoleCard {
    let risk = explosion_risk(path, dark, survival, unseated_cross, order_thin);
    let will_explode = explodes(&risk);
    let dark_led = dark > survival;
    let show_hp = reveals_hidden_positive(perception, math_rank);
    let show_hn = reveals_hidden_negative(dark_led, envy_or_wrath) || will_explode;
    LabPoleCard {
        path,
        known_pos: path.known_positive(),
        known_neg: path.known_negative(),
        hidden_pos: if show_hp {
            Some(path.hidden_positive())
        } else {
            None
        },
        hidden_neg: if show_hn {
            Some(path.hidden_negative())
        } else {
            None
        },
        risk,
        will_explode,
    }
}
