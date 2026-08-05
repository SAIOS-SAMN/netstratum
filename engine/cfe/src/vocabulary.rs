//! **THE VOCABULARY AND ITS BINDINGS** (`28_VOCABULARY`, V0).
//!
//! A class of entity is not a word. It is a word, a mark, and a
//! MECHANISM — and this module holds the mechanism.
//!
//! **The vacuity this pays.** Measured 2026-08-05: nothing in the engine
//! branched on the sealed taxon class. `TaxonSealed` (tag 85) committed
//! it, the snapshot carried it, replay restored it, and no rule read it
//! to decide anything — while metabolism drove 8 branch sites,
//! disposition 7 and perception 6. By `27_EVIDENCE` LAW 8's own test —
//! *a term that computes is kept; a term that only resonates is
//! excised* — the class partition was a term that only resonated.
//!
//! It is kept rather than excised, because the fix is smaller than the
//! loss: give it the mechanism it always implied.
//!
//! **Why a binding is DATA and not code.** A minted entry (V2) will
//! carry one of these, authored by a progenitor. Data needs no sandbox,
//! no interpreter and no new capability class; exact rationals replay
//! bit-identically; and a closed channel set means a mint changes
//! REACHABILITY, never what the engine can express.

use crate::ratio::{ratio, Ratio};

/// Zone yield — `world::tick`'s `yield_factor`. More is better.
pub const CHANNEL_YIELD: u8 = 1;
/// Zone upkeep — `world::tick`'s `upkeep_factor`. Less is better.
pub const CHANNEL_UPKEEP: u8 = 2;
/// The survey draw — `world::survey`. Less is better.
pub const CHANNEL_SURVEY: u8 = 3;

/// The closed channel table.
///
/// **Only channels the engine ALREADY computes appear here.** Declaring
/// a channel nothing reads would manufacture exactly the vacuity this
/// module exists to pay off — so the table grows by LANDING a channel,
/// never by a binding naming one that is not there. Research rate and
/// route friction are the obvious next two and are deliberately absent
/// until they are wired.
pub const CHANNELS: [u8; 3] = [CHANNEL_YIELD, CHANNEL_UPKEEP, CHANNEL_SURVEY];

/// **DOES A FACTOR ABOVE 1 HELP ON THIS CHANNEL?**
///
/// The channels do not share a polarity, and the one-benefit-one-friction
/// law is meaningless without knowing which way each points: raising
/// yield helps, raising upkeep hurts, and raising the survey draw hurts
/// because the draw is a COST. A structural rule of "one factor above 1
/// and one below" would have called a class that raises upkeep and
/// lowers yield a fair trade when it is strictly worse on both.
pub fn channel_favours_increase(channel: u8) -> bool {
    match channel {
        CHANNEL_YIELD => true,
        CHANNEL_UPKEEP | CHANNEL_SURVEY => false,
        _ => false,
    }
}

pub fn channel_valid(channel: u8) -> bool {
    CHANNELS.contains(&channel)
}

/// A mechanism: exact rational factors over the closed channel set.
pub type Binding = [(u8, Ratio)];

/// The factor bounds (placeholder, operator-tunable). No entry may be a
/// hundredfold. The bound is on the FACTOR — a scalar coefficient — and
/// never on a directional quantity, so `24_FORM` §2b is untouched.
pub fn factor_in_bounds(factor: &Ratio) -> bool {
    *factor >= ratio(1, 4) && *factor <= ratio(4, 1)
}

/// The factor a binding applies on one channel — the PRODUCT of every
/// entry naming it, or 1 where it is silent.
///
/// A product rather than a lookup so a binding listing a channel twice
/// composes instead of shadowing; combined with [`canonical`] the two
/// readings always agree.
pub fn factor_on(binding: &Binding, channel: u8) -> Ratio {
    binding
        .iter()
        .filter(|(c, _)| *c == channel)
        .fold(ratio(1, 1), |acc, (_, f)| acc * f.clone())
}

/// The binding in canonical form: one entry per channel, ascending,
/// silent channels dropped.
///
/// **Novelty is compared on this form** (§6). Without it `[(1,a),(2,b)]`
/// and `[(2,b),(1,a)]` would read as two different mechanisms and the
/// novelty gate could be walked straight through by reordering a list.
pub fn canonical(binding: &Binding) -> Vec<(u8, Ratio)> {
    let mut out: Vec<(u8, Ratio)> = CHANNELS
        .iter()
        .map(|channel| (*channel, factor_on(binding, *channel)))
        .filter(|(_, factor)| *factor != ratio(1, 1))
        .collect();
    out.sort_by_key(|(channel, _)| *channel);
    out
}

/// **THE BINDING LAW — one benefit, one friction** (`28_VOCABULARY`
/// §4a; PRACTICE-1 generalized).
///
/// PRACTICE-1 rules that a discipline carries *one benefit, one
/// friction, exact, per discipline*, and all five landed practices obey
/// it. Promoted here from a property of those five to the law every
/// vocabulary entry must satisfy, so a strictly-dominant archetype is
/// UNMINTABLE BY CONSTRUCTION rather than by a balance patch applied
/// after someone finds it.
///
/// **The empty binding is lawful and claims nothing.** That is class 0,
/// the unclassed — and it is Article II's ground state in vocabulary
/// form: *it holds its place, keeps its structural existence, is never
/// deleted and never barred — and it is paid nothing.* A binding is
/// either empty, or it trades.
pub fn binding_trades(binding: &Binding) -> bool {
    let entries = canonical(binding);
    if entries.is_empty() {
        return true;
    }
    let mut benefit = false;
    let mut friction = false;
    for (channel, factor) in &entries {
        if !channel_valid(*channel) || !factor_in_bounds(factor) || *factor <= ratio(0, 1) {
            return false;
        }
        let helps = (*factor > ratio(1, 1)) == channel_favours_increase(*channel);
        match helps {
            true => benefit = true,
            false => friction = true,
        }
    }
    benefit && friction
}

/// **THE ARCHETYPE BINDINGS** — the seed mechanism of each class.
///
/// Placeholder magnitudes, operator-tunable at balance; the SHAPE is the
/// law. Each states the class's own trade in the two or three channels
/// that express it, and no two are the same mechanism (pinned) — a
/// partition whose members computed identically would distinguish
/// nothing, which is the novelty law (§6) asserted on the seed before
/// anything is allowed to mint against it.
pub fn archetype_binding(class: u8) -> Vec<(u8, Ratio)> {
    match class {
        // The unclassed: no trade, no claim. The ground state.
        0 => vec![],
        // ⬡ biotic-swarm — many bodies, one lattice: it produces well
        // and eats well.
        1 => vec![(CHANNEL_YIELD, ratio(5, 4)), (CHANNEL_UPKEEP, ratio(5, 4))],
        // ◈ geodic-lithovore — the stone-eater: cheap to keep, slow to
        // yield.
        2 => vec![(CHANNEL_YIELD, ratio(3, 4)), (CHANNEL_UPKEEP, ratio(3, 4))],
        // ◐ metashifter — form unfixed: it sees for half the draw and
        // builds poorly while doing it.
        3 => vec![(CHANNEL_YIELD, ratio(3, 4)), (CHANNEL_SURVEY, ratio(1, 2))],
        // ≋ telepathic-resonant — phases in agreement: the lens is
        // cheap, coherence is not.
        4 => vec![(CHANNEL_UPKEEP, ratio(5, 4)), (CHANNEL_SURVEY, ratio(3, 4))],
        // ⊚ pelagic-aerial — free inside a medium: it gathers well and
        // surveys badly, having no fixed vantage.
        5 => vec![(CHANNEL_YIELD, ratio(5, 4)), (CHANNEL_SURVEY, ratio(3, 2))],
        // ∿ plasma-field — energy-rich and hard to hold.
        6 => vec![(CHANNEL_YIELD, ratio(3, 2)), (CHANNEL_UPKEEP, ratio(3, 2))],
        // ▲ apex-terrestrial — the solitary peak: it needs little and
        // makes little.
        7 => vec![(CHANNEL_YIELD, ratio(1, 2)), (CHANNEL_UPKEEP, ratio(1, 2))],
        _ => vec![],
    }
}

// ─────────────────────────────────────────────────────────────────────
// THE FOLD (`28_VOCABULARY` §8, V1)
// ─────────────────────────────────────────────────────────────────────

/// Which vocabulary an entry belongs to. Wire codes; append-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Axis {
    Disposition = 1,
    Metabolism = 2,
    Perception = 3,
    Class = 4,
    Clade = 5,
    Practice = 6,
}

impl Axis {
    pub fn from_code(code: u8) -> Option<Axis> {
        match code {
            1 => Some(Axis::Disposition),
            2 => Some(Axis::Metabolism),
            3 => Some(Axis::Perception),
            4 => Some(Axis::Class),
            5 => Some(Axis::Clade),
            6 => Some(Axis::Practice),
            _ => None,
        }
    }
    pub fn code(self) -> u8 {
        self as u8
    }
    /// **STATIONS ARE ABSENT AND THAT IS THE LAW** (`28_VOCABULARY`
    /// §10). The four offices are chartered by `12_UNION` §3 as stations
    /// of FUNCTION; minting one would let a T0 seat manufacture the seat
    /// that gates it. Offices change by amendment and by nothing else,
    /// so there is no axis code for them to be minted under.
    pub const ALL: [Axis; 6] = [
        Axis::Disposition,
        Axis::Metabolism,
        Axis::Perception,
        Axis::Class,
        Axis::Clade,
        Axis::Practice,
    ];
}

/// One vocabulary entry: a code, a mark, a word, and a mechanism.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub code: u8,
    pub sigil: String,
    pub word: String,
    pub binding: Vec<(u8, Ratio)>,
}

/// **THE COMPILED SEED.** Genesis reads no chain — there is none yet —
/// so the founding vocabulary is a compiled definition exactly as the
/// founding roster is (`conductor::polities`). Everything after genesis
/// is minted.
pub fn seed(axis: Axis) -> Vec<Entry> {
    let plain = |code: u8, word: &str| Entry {
        code,
        sigil: String::new(),
        word: word.into(),
        binding: Vec::new(),
    };
    match axis {
        Axis::Disposition => vec![
            plain(1, "expansionist"),
            plain(2, "empiricist"),
            plain(3, "epicurean"),
        ],
        Axis::Metabolism => vec![plain(1, "mineral"), plain(2, "biotic"), plain(3, "plasmic")],
        Axis::Perception => vec![
            plain(1, "planar"),
            plain(2, "volumetric"),
            plain(3, "harmonic"),
            plain(4, "umbral"),
        ],
        Axis::Class => (0..=7)
            .map(|code| Entry {
                code,
                sigil: crate::dop::archetype_sigil(code).into(),
                word: crate::dop::archetype_word(code).into(),
                binding: archetype_binding(code),
            })
            .collect(),
        Axis::Clade => (0..=20).map(|code| plain(code, clade_word(code))).collect(),
        Axis::Practice => (1..=5).map(|code| plain(code, practice_word(code))).collect(),
    }
}

/// `15_TAXON` §3, compiled. Code 0 is unrecorded and lawful.
pub fn clade_word(code: u8) -> &'static str {
    match code {
        0 => "unrecorded",
        1 => "felid",
        2 => "humanoid",
        3 => "cryo-coherent",
        4 => "lithic",
        5 => "tardigrade",
        6 => "cetacean",
        7 => "cephalopod",
        8 => "hymenopteran",
        9 => "amphibian",
        10 => "fungal",
        11 => "cnidarian",
        12 => "plasmoid",
        13 => "syzygial",
        14 => "endolithic",
        15 => "avian",
        16 => "teleost",
        17 => "revenant",
        18 => "gastropod",
        19 => "draconic",
        20 => "metashifting",
        _ => "unknown",
    }
}

/// PRACTICE-1's five disciplines, by their in-value codes.
pub fn practice_word(code: u8) -> &'static str {
    match code {
        1 => "austerity",
        2 => "showpiece",
        3 => "restraint",
        4 => "hoard",
        5 => "overdrive",
        _ => "unknown",
    }
}

/// **THE FOLD** — `seed ∪ minted`, in chronicle order.
///
/// Same chain, same vocabulary, on every node, forever. This is the
/// property a file could never have (`conductor::polities`, THE
/// DEFINITION LAW) and the whole reason a minted definition is safe
/// where a filed one was not.
pub fn fold(chronicle: &crate::vrp::Chronicle, axis: Axis) -> Vec<Entry> {
    let mut out = seed(axis);
    for receipt in chronicle.receipts() {
        if let crate::dop::ClosureEvent::VocabularyMinted {
            axis: on,
            code,
            sigil,
            word,
            binding,
        } = &receipt.event
        {
            if Axis::from_code(*on) == Some(axis) {
                out.push(Entry {
                    code: *code,
                    sigil: sigil.clone(),
                    word: word.clone(),
                    binding: binding.clone(),
                });
            }
        }
    }
    out
}

/// The highest lawful code on an axis, read from the fold.
pub fn ceiling(chronicle: &crate::vrp::Chronicle, axis: Axis) -> u8 {
    fold(chronicle, axis)
        .iter()
        .map(|entry| entry.code)
        .max()
        .unwrap_or(0)
}

/// **THE ORDERING LAW** (`28_VOCABULARY` §9) — the vocabulary AS IT
/// STOOD at a given stamp, never as it stands today.
///
/// A draw taken at stamp `t` must read the vocabulary of stamp `t`. Read
/// today's instead and replaying an old era produces lines wearing
/// classes that did not exist yet — a SEAL B violation that fails
/// SILENTLY, since the replay still terminates and still looks like a
/// world. The fold is ordered, so honouring it is a matter of reading
/// it correctly rather than of a new rule.
pub fn fold_at(chronicle: &crate::vrp::Chronicle, axis: Axis, stamp: u64) -> Vec<Entry> {
    let mut out = seed(axis);
    for receipt in chronicle.receipts() {
        if receipt.stamp > stamp {
            break;
        }
        if let crate::dop::ClosureEvent::VocabularyMinted {
            axis: on,
            code,
            sigil,
            word,
            binding,
        } = &receipt.event
        {
            if Axis::from_code(*on) == Some(axis) {
                out.push(Entry {
                    code: *code,
                    sigil: sigil.clone(),
                    word: word.clone(),
                    binding: binding.clone(),
                });
            }
        }
    }
    out
}
