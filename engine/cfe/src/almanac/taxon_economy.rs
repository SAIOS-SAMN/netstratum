//! TAXON ECONOMY — extractive · regenerative · balanced modes.
//!
//! Pure derivations from sealed class/clade + planet thermal region.
//! No free dials. Flatlander baseline = bimaniform (clade 2).
//!
//! Authority: `15_TAXON` · `37_IMPERIUM_TAXON`.

use crate::almanac::thermal::ThermalRegion;
use crate::ratio::{ratio, Ratio};

/// How a line takes and returns matter on a world.
///
/// All three are **generative** (produce play and receipts); they differ
/// in *how* matter moves — mine first, regrow first, or both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EconomicMode {
    /// Extractor-primary: vein draw, slag, foundry. Regen is secondary tech.
    Extractive = 1,
    /// Regenerator-primary: restore, hypha, reef. Extraction is measured.
    Regenerative = 2,
    /// Both seats natural — the **flatlander baseline** (bimaniform).
    Balanced = 3,
}

impl EconomicMode {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Extractive),
            2 => Some(Self::Regenerative),
            3 => Some(Self::Balanced),
            _ => None,
        }
    }

    pub fn as_wire(self) -> u8 {
        self as u8
    }

    pub fn word(self) -> &'static str {
        match self {
            Self::Extractive => "extractive",
            Self::Regenerative => "regenerative",
            Self::Balanced => "balanced",
        }
    }

    pub fn gloss(self) -> &'static str {
        match self {
            Self::Extractive => "mine and refine first — regenerators are hard-won tech",
            Self::Regenerative => "restore and cycle first — extractors are measured",
            Self::Balanced => "flatlander baseline — extract and regenerate both seat naturally",
        }
    }
}

/// Planet–species fit: rates and gates scale by this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlanetFit {
    /// Preferred band for this clade.
    Native = 1,
    /// Survivable; three-quarters yield.
    Tolerant = 2,
    /// Hostile; half yield; extractive pressure rises.
    Hostile = 3,
}

impl PlanetFit {
    pub fn word(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Tolerant => "tolerant",
            Self::Hostile => "hostile",
        }
    }

    /// Exact rational scale on zone yields (derived fractions).
    pub fn yield_scale(self) -> Ratio {
        match self {
            Self::Native => ratio(1, 1),
            Self::Tolerant => ratio(3, 4),
            Self::Hostile => ratio(1, 2),
        }
    }
}

/// **FLATLANDER BASELINE** — clade 2 `bimaniform`.
///
/// The knowledge baseline for design and play: planar tool-makers whose
/// extract+regen suite is what Earth-trained operators already know.
/// Imperium species string may read `bimaniform-flatlander` or
/// `flatlander-baseline`; the wire is always clade 2.
pub const FLATLANDER_CLADE: u8 = 2;

pub fn is_flatlander_baseline(clade: u8) -> bool {
    clade == FLATLANDER_CLADE
}

pub fn flatlander_word() -> &'static str {
    "flatlander"
}

pub fn flatlander_species() -> &'static str {
    "bimaniform-flatlander"
}

/// Preferred thermal region family for a clade (imperium codes).
/// Used only to colour PlanetFit — not a hard exile.
pub fn clade_preferred_region(clade: u8) -> ThermalRegion {
    // Expansion clades follow owning class preference via first legacy of that class.
    if let Some(owner) = crate::almanac::taxon_table::expansion_class_of(clade) {
        return match owner {
            6 | 12 => ThermalRegion::HyperThermal,
            3 | 9 => ThermalRegion::Cryogenic,
            8 => ThermalRegion::Torrid, // corridor-nomad
            _ => ThermalRegion::Mesic,
        };
    }
    match clade {
        // cryocoherens · draconiform cold-coherence
        3 | 19 => ThermalRegion::Cryogenic,
        // hypobiotid · lithophage · endolithid — rock / any, prefer mesic-stone
        4 | 5 | 14 => ThermalRegion::Mesic,
        // plasmiform · syzygiform — field / hot
        12 | 13 => ThermalRegion::HyperThermal,
        // soniform · polybrachia · amphiform · actinoform — volumetric mesic/cold
        6 | 7 | 9 | 16 => ThermalRegion::Mesic,
        // volatiform — atmosphere mesic/torrid
        15 => ThermalRegion::Torrid,
        // flatlander / ailuroform / hexasocial / mycotiform / cnidiform — mesic
        1 | 2 | 8 | 10 | 11 => ThermalRegion::Mesic,
        // metamorphid · revenid · heliciform — flexible mesic
        _ => ThermalRegion::Mesic,
    }
}

/// **ECONOMIC MODE** from class + clade (pure, total for lawful codes).
///
/// | Mode | Typical clades |
/// |------|----------------|
/// | Extractive | lithophage, hypobiotid, endolithid, geodic class, plasmiform |
/// | Regenerative | mycotiform, cnidiform, amphiform, hexasocial, soniform |
/// | Balanced | **bimaniform (flatlander)**, ailuroform, polybrachia, default |
pub fn economic_mode(class: u8, clade: u8) -> EconomicMode {
    // Flatlander baseline always balanced.
    if is_flatlander_baseline(clade) {
        return EconomicMode::Balanced;
    }
    // Expansion clades (21+) inherit their owning class colour.
    if let Some(owner) = crate::almanac::taxon_table::expansion_class_of(clade) {
        return economic_mode(owner, 0); // re-enter on class only
    }
    // Class-primary colour (includes +6 balance tiers 8..=13).
    match class {
        2 | 9 => return EconomicMode::Extractive, // geodic · abyssal-pressure
        6 | 12 => return EconomicMode::Extractive, // plasma · thermal-banker
        1 | 11 => return EconomicMode::Regenerative, // biotic-swarm · void-swarm
        10 => return EconomicMode::Balanced,         // crystal-mind
        8 | 13 => return EconomicMode::Balanced,     // corridor-nomad · mirror-form
        _ => {}
    }
    match clade {
        // Extractive bodies
        4 | 5 | 14 | 12 => EconomicMode::Extractive,
        // Regenerative bodies
        8 | 9 | 10 | 11 | 6 => EconomicMode::Regenerative,
        // Balanced / hybrid
        1 | 7 | 2 | 15 | 16 | 20 | 3 | 19 => EconomicMode::Balanced,
        _ => EconomicMode::Balanced,
    }
}

/// Planet fit from preferred region vs actual (pure).
pub fn planet_fit(clade: u8, region: Option<ThermalRegion>) -> PlanetFit {
    let Some(region) = region else {
        return PlanetFit::Tolerant; // unsited — unknown world
    };
    let pref = clade_preferred_region(clade);
    if region == pref {
        return PlanetFit::Native;
    }
    // Adjacent bands = tolerant; far = hostile.
    let adj = matches!(
        (pref, region),
        (ThermalRegion::Mesic, ThermalRegion::Sublime | ThermalRegion::Torrid)
            | (ThermalRegion::Cryogenic, ThermalRegion::Quenched | ThermalRegion::Sublime)
            | (ThermalRegion::HyperThermal, ThermalRegion::Torrid | ThermalRegion::Incandescent)
            | (ThermalRegion::Torrid, ThermalRegion::Mesic | ThermalRegion::HyperThermal)
            | (ThermalRegion::Sublime, ThermalRegion::Cryogenic | ThermalRegion::Mesic)
    );
    if adj {
        PlanetFit::Tolerant
    } else {
        PlanetFit::Hostile
    }
}

/// Scale for **extractor** zone yield (including Extractor base yield).
/// Derived: mode bias × planet fit.
pub fn extract_yield_scale(mode: EconomicMode, fit: PlanetFit) -> Ratio {
    let mode_scale = match mode {
        EconomicMode::Extractive => ratio(5, 4), // +1/4
        EconomicMode::Regenerative => ratio(3, 4), // −1/4
        EconomicMode::Balanced => ratio(1, 1),
    };
    mode_scale * fit.yield_scale()
}

/// Scale for **regenerator** zone yield.
pub fn regen_yield_scale(mode: EconomicMode, fit: PlanetFit) -> Ratio {
    let mode_scale = match mode {
        EconomicMode::Regenerative => ratio(5, 4),
        EconomicMode::Extractive => ratio(3, 4),
        EconomicMode::Balanced => ratio(1, 1),
    };
    mode_scale * fit.yield_scale()
}

/// Habitat / farm scale — regenerative lines farm better; extractive less.
pub fn habitat_yield_scale(mode: EconomicMode, fit: PlanetFit) -> Ratio {
    let mode_scale = match mode {
        EconomicMode::Regenerative => ratio(5, 4),
        EconomicMode::Extractive => ratio(7, 8),
        EconomicMode::Balanced => ratio(1, 1),
    };
    mode_scale * fit.yield_scale()
}

/// Tech condition required to place a regenerator (normally 3 = DEEPENING).
/// Regenerative lines may seat at condition 2 (EASE path); extractive stay at 3.
pub fn regenerator_tech_need(mode: EconomicMode) -> u8 {
    match mode {
        EconomicMode::Regenerative => 2,
        EconomicMode::Balanced => 3,
        EconomicMode::Extractive => 3,
    }
}

/// Whether an extra extractor is still in the "natural" count before
/// pressure residual (regenerative lines feel extract pressure sooner).
pub fn extractor_soft_cap(mode: EconomicMode) -> u64 {
    match mode {
        EconomicMode::Extractive => 6,
        EconomicMode::Balanced => 4,
        EconomicMode::Regenerative => 2,
    }
}

/// Science research affinity for information work (gates research seat fee).
/// Flatlander baseline = primary research; extractive = secondary; etc.
pub fn research_affinity_mult(mode: EconomicMode) -> i64 {
    match mode {
        EconomicMode::Balanced => 1,      // flatlander: knowledge path open
        EconomicMode::Regenerative => 1,  // cycle-knowledge
        EconomicMode::Extractive => 2,    // research costs more (secondary)
    }
}

/// Full reading for CLI / pins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaxonEconomy {
    pub mode: EconomicMode,
    pub fit: PlanetFit,
    pub flatlander: bool,
    pub class: u8,
    pub clade: u8,
}

pub fn taxon_economy(
    class: u8,
    clade: u8,
    region: Option<ThermalRegion>,
) -> TaxonEconomy {
    TaxonEconomy {
        mode: economic_mode(class, clade),
        fit: planet_fit(clade, region),
        flatlander: is_flatlander_baseline(clade),
        class,
        clade,
    }
}
