//! TAXON TABLE — class · clade ceilings, expansion slots, words.
//!
//! **RAISE (2026-08-08):**
//! - class ceiling 7 → **13** (+6 archetype tiers for balance)
//! - clade ceiling 20 → **254** (18 expansion taxons × 13 classes under u8)
//! - Target was 25/class; u8 wire caps at 18 (20+13×18=254). u16 reserved.
//!
//! Expansion layout:
//! ```text
//! code = 20 + (class − 1) × 18 + slot    // class 1..=13, slot 1..=18
//! codes 21..=254
//! ```

use crate::ratio::{ratio, Ratio};

/// Lawful class codes 1..=13 (0 = unclassed).
pub fn class_ceiling() -> u8 {
    13
}

/// Lawful clade codes 1..=254 (0 = unrecorded).
pub fn clade_ceiling() -> u8 {
    254
}

/// Legacy named clades (imperium 1..=20).
pub const CLADE_LEGACY_MAX: u8 = 20;

/// Expansion slots per class tier (u8 max fit for 13 classes).
pub const CLADE_EXPAND_SLOTS: u8 = 18;

/// Prefer 25; landed 18 under u8. Raise with u16 clade amendment.
pub const CLADE_EXPAND_TARGET: u8 = 25;

/// Expansion clade code for (class, slot). Slot is 1..=[`CLADE_EXPAND_SLOTS`].
pub fn expansion_clade(class: u8, slot: u8) -> Option<u8> {
    if class == 0 || class > class_ceiling() {
        return None;
    }
    if slot == 0 || slot > CLADE_EXPAND_SLOTS {
        return None;
    }
    let code =
        20u16 + u16::from(class - 1) * u16::from(CLADE_EXPAND_SLOTS) + u16::from(slot);
    if code > 254 {
        return None;
    }
    Some(code as u8)
}

/// Inverse: which class owns an expansion clade code.
pub fn expansion_class_of(clade: u8) -> Option<u8> {
    if clade <= CLADE_LEGACY_MAX {
        return None;
    }
    let idx = clade - CLADE_LEGACY_MAX - 1;
    let class = idx / CLADE_EXPAND_SLOTS + 1;
    if class == 0 || class > class_ceiling() {
        return None;
    }
    Some(class)
}

/// Slot within class for an expansion clade.
pub fn expansion_slot_of(clade: u8) -> Option<u8> {
    if clade <= CLADE_LEGACY_MAX {
        return None;
    }
    let idx = clade - CLADE_LEGACY_MAX - 1;
    Some(idx % CLADE_EXPAND_SLOTS + 1)
}

/// Class archetype sigil.
pub fn archetype_sigil(class: u8) -> &'static str {
    match class {
        0 => "∅",
        1 => "⬡",
        2 => "◈",
        3 => "◐",
        4 => "≋",
        5 => "⊚",
        6 => "∿",
        7 => "▲",
        8 => "⟺",  // corridor-nomad
        9 => "▼",  // abyssal-pressure
        10 => "◆", // crystal-mind
        11 => "※", // void-swarm
        12 => "❄", // thermal-banker
        13 => "☯", // mirror-form
        _ => "?",
    }
}

/// Class word.
pub fn archetype_word(class: u8) -> &'static str {
    match class {
        0 => "unclassed",
        1 => "biotic-swarm",
        2 => "geodic-lithovore",
        3 => "metashifter",
        4 => "telepathic-resonant",
        5 => "pelagic-aerial",
        6 => "plasma-field",
        7 => "apex-solitary",
        8 => "corridor-nomad",
        9 => "abyssal-pressure",
        10 => "crystal-mind",
        11 => "void-swarm",
        12 => "thermal-banker",
        13 => "mirror-form",
        _ => "unknown",
    }
}

/// Legacy clade 0..=20 words (imperium).
pub fn legacy_clade_word(code: u8) -> Option<&'static str> {
    Some(match code {
        0 => "unrecorded",
        1 => "ailuroform",
        2 => "bimaniform",
        3 => "cryocoherens",
        4 => "lithophage",
        5 => "hypobiotid",
        6 => "soniform",
        7 => "polybrachia",
        8 => "hexasocial",
        9 => "amphiform",
        10 => "mycotiform",
        11 => "cnidiform",
        12 => "plasmiform",
        13 => "syzygiform",
        14 => "endolithid",
        15 => "volatiform",
        16 => "actinoform",
        17 => "revenid",
        18 => "heliciform",
        19 => "draconiform",
        20 => "metamorphid",
        _ => return None,
    })
}

/// Pre-built expansion words for codes 21..=254 (234 entries).
const EXPAND_WORDS: [&str; 234] =
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/almanac/expand_clade_words.inc.rs"));

/// Full clade word.
pub fn clade_word(code: u8) -> &'static str {
    if let Some(w) = legacy_clade_word(code) {
        return w;
    }
    if code >= 21 {
        if let Some(w) = EXPAND_WORDS.get((code - 21) as usize) {
            return w;
        }
    }
    "unknown"
}

/// Parse clade wire from word (legacy + expansion + aliases).
pub fn clade_from_word(word: &str) -> Option<u8> {
    match word {
        "unrecorded" => Some(0),
        "ailuroform" | "felid" | "feliform" => Some(1),
        "bimaniform" | "humanoid" | "bimanus" | "flatlander" | "flatland" | "human"
        | "humans" | "baseline" => Some(2),
        "cryocoherens" | "cryo-coherent" | "cryo" => Some(3),
        "lithophage" | "lithic" | "lithovore" => Some(4),
        "hypobiotid" | "tardigrade" | "void-sleeper" => Some(5),
        "soniform" | "cetacean" | "ceticant" => Some(6),
        "polybrachia" | "cephalopod" | "octobranch" => Some(7),
        "hexasocial" | "hymenopteran" | "formicine" | "eusocialid" => Some(8),
        "amphiform" | "amphibian" => Some(9),
        "mycotiform" | "fungal" | "mycota" => Some(10),
        "cnidiform" | "cnidarian" => Some(11),
        "plasmiform" | "plasmoid" => Some(12),
        "syzygiform" | "syzygial" => Some(13),
        "endolithid" | "endolithic" => Some(14),
        "volatiform" | "avian" => Some(15),
        "actinoform" | "teleost" => Some(16),
        "revenid" | "revenant" => Some(17),
        "heliciform" | "gastropod" => Some(18),
        "draconiform" | "draconic" => Some(19),
        "metamorphid" | "metashifting" | "metashifter" => Some(20),
        other => {
            for (i, w) in EXPAND_WORDS.iter().enumerate() {
                if *w == other {
                    return Some((i + 21) as u8);
                }
            }
            None
        }
    }
}

/// How many expansion taxons a class exposes.
pub fn class_taxon_capacity(_class: u8) -> u8 {
    CLADE_EXPAND_SLOTS
}

/// Seat-width bonus (25 more civ capacity per office tier).
pub fn office_seat_bonus() -> u64 {
    25
}

/// Pure research weight grain for expansion class.
pub fn expansion_research_weight(class: u8) -> Ratio {
    ratio(i64::from(class.max(1)), 1) * ratio(1, 8)
}
