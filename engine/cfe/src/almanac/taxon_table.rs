//! TAXON table — open-ended class · clade ceilings (no u8 wire cap).
//!
//! **Open (2026-08-08):** clade is `u16` end-to-end. Expansion =
//! **25 taxons × 13 class tiers** + 20 legacy = ceiling **345**.
//! Class ceiling **13** (u8 still holds; raised freely by amendment).
//!
//! ```text
//! code = 20 + (class − 1) × 25 + slot    // class 1..=13, slot 1..=25
//! codes 21..=345
//! ```

/// Lawful class codes 1..=13 (0 = unclassed). u8 today; raise by amendment.
pub fn class_ceiling() -> u8 {
    13
}

/// Lawful clade codes 1..=[`clade_ceiling`] (0 = unrecorded). **u16 — open.**
pub fn clade_ceiling() -> u16 {
    CLADE_LEGACY_MAX + u16::from(class_ceiling()) * u16::from(CLADE_EXPAND_SLOTS)
}

/// Legacy named clades (imperium 1..=20).
pub const CLADE_LEGACY_MAX: u16 = 20;

/// Expansion slots per class tier (full target; no u8 truncation).
pub const CLADE_EXPAND_SLOTS: u8 = 25;

/// Alias of [`CLADE_EXPAND_SLOTS`] — target met.
pub const CLADE_EXPAND_TARGET: u8 = 25;

/// Is this clade code in the lawful range (legacy or expansion layout)?
pub fn is_lawful_clade(clade: u16) -> bool {
    clade > 0 && clade <= clade_ceiling()
}

/// Expansion clade code for (class, slot). Slot is 1..=25.
pub fn expansion_clade(class: u8, slot: u8) -> Option<u16> {
    if class == 0 || class > class_ceiling() {
        return None;
    }
    if slot == 0 || slot > CLADE_EXPAND_SLOTS {
        return None;
    }
    Some(CLADE_LEGACY_MAX + u16::from(class - 1) * u16::from(CLADE_EXPAND_SLOTS) + u16::from(slot))
}

/// Inverse: which class owns an expansion clade code.
pub fn expansion_class_of(clade: u16) -> Option<u8> {
    if clade <= CLADE_LEGACY_MAX {
        return None;
    }
    let idx = clade - CLADE_LEGACY_MAX - 1;
    let class = (idx / u16::from(CLADE_EXPAND_SLOTS)) + 1;
    if class == 0 || class > u16::from(class_ceiling()) {
        return None;
    }
    Some(class as u8)
}

/// Slot within class for an expansion clade.
pub fn expansion_slot_of(clade: u16) -> Option<u8> {
    if clade <= CLADE_LEGACY_MAX {
        return None;
    }
    let idx = clade - CLADE_LEGACY_MAX - 1;
    Some((idx % u16::from(CLADE_EXPAND_SLOTS) + 1) as u8)
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
        8 => "⟺",
        9 => "▼",
        10 => "◆",
        11 => "※",
        12 => "❄",
        13 => "☯",
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
pub fn legacy_clade_word(code: u16) -> Option<&'static str> {
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

/// Pre-built expansion words for codes 21..=345 (325 entries).
const EXPAND_WORDS: [&str; 325] = include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/almanac/expand_clade_words.inc.rs"
));

/// Full clade word.
pub fn clade_word(code: u16) -> &'static str {
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
pub fn clade_from_word(word: &str) -> Option<u16> {
    match word {
        "unrecorded" => Some(0),
        "ailuroform" => Some(1),
        "bimaniform" | "bimanus" | "flatlander" | "flatland" | "baseline" => Some(2),
        "cryocoherens" | "cryo-coherent" | "cryo" => Some(3),
        "lithophage" | "lithic" | "lithovore" => Some(4),
        "hypobiotid" | "void-sleeper" => Some(5),
        "soniform" | "ceticant" => Some(6),
        "polybrachia" | "octobranch" => Some(7),
        "hexasocial" | "eusocialid" => Some(8),
        "amphiform" => Some(9),
        "mycotiform" => Some(10),
        "cnidiform" => Some(11),
        "plasmiform" | "plasmoid" => Some(12),
        "syzygiform" | "syzygial" => Some(13),
        "endolithid" | "endolithic" => Some(14),
        "volatiform" => Some(15),
        "actinoform" => Some(16),
        "revenid" | "revenant" => Some(17),
        "heliciform" => Some(18),
        "draconiform" => Some(19),
        "metamorphid" | "metashifting" | "metashifter" => Some(20),
        other => {
            for (i, w) in EXPAND_WORDS.iter().enumerate() {
                if *w == other {
                    return Some((i as u16) + 21);
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
