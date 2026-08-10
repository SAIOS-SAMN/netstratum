//! The civilization registry (contract §8): WHO exists, across worlds.
//!
//! One JSON file per civilization under `var/registry/`. An entry is
//! IDENTITY ONLY until the genesis landings give the axes mechanics —
//! recorded, displayed, joined; changing no rule today. When G1 lands,
//! the entry is re-committed as a genesis receipt and becomes binding.

use crate::card;
use std::fs;
use std::path::PathBuf;

/// The three genesis axes (World-Book Ch. 2). Free strings validated
/// at the door — the vocabulary is closed, the registry enforces it.
/// The vocabulary GROWS append-only under the substrate law (12_UNION
/// §5a — the tag-registry discipline applied to the axes): SUBSTRATE-1
/// added `plasmic` (plasma intelligences — current sheets and confined
/// toroids of the stellar environs) and `umbral` (occlusion-reading —
/// the syzygial lines that exist where the voices align). A value is
/// never removed, never renamed.
pub const DISPOSITIONS: [&str; 3] = ["expansionist", "empiricist", "epicurean"];
pub const METABOLISMS: [&str; 3] = ["mineral", "biotic", "plasmic"];
pub const PERCEPTIONS: [&str; 4] = ["planar", "volumetric", "harmonic", "umbral"];

/// BAND-1 + T4–T15 (12_UNION §2a, 30_OFFICES_AND_RAILS): class STATIONS —
/// offices in the union, never ranks. Append-only like the axes. A
/// civilization with no station is simply a people of the bands (the
/// common, lawful state). T4–T15 are **deeper functions**, not
/// offices above the progenitors. **T10–T15 added 2026-08-08** for balance.
pub const STATIONS: [&str; 16] = [
    "progenitor",
    "watcher",
    "observer",
    "sovereign",
    "caravan",
    "crucible",
    "exchange",
    "assay",
    "vault",
    "anchor",
    "herald",
    "lattice",
    "chorus",
    "warden",
    "foundry",
    "nexus",
];

/// BAND-1 (12_UNION §1): a station's seat width — SPACE AVAILABLE,
/// not rank. `None` is unbounded. **+25 capacity** on bounded offices
/// (2026-08-08 expansion: 25 more taxons/seats per tier).
pub fn seat_width(station: &str) -> Option<u64> {
    let bonus = 25u64; // office_seat_bonus — 25 more per tier
    match station {
        "progenitor" => Some(8 + bonus),
        "watcher" | "observer" => Some(16 + bonus),
        "caravan" | "crucible" | "exchange" | "assay" | "vault" => Some(16 + bonus),
        "herald" | "lattice" | "chorus" | "warden" | "foundry" => Some(16 + bonus),
        "sovereign" | "anchor" | "nexus" => None,
        _ => None,
    }
}

/// THE TIER — the vertical band a station sits in (12_UNION §2a +
/// 30_OFFICES_AND_RAILS). Higher **numbers** are deeper functions, not
/// higher rank. T0 remains the top.
///
/// A DERIVED reading, never stored. The station is the committed fact;
/// the tier is what it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tier {
    /// THE BOARD-MAKERS — progenitors.
    T0,
    /// THE ARCHITECT OBSERVERS — watchers and observers.
    T1,
    /// THE RECORD KEEPERS — sovereigns.
    T2,
    /// THE BIOSPHERE NODES — the common, lawful state. Reached by
    /// holding no seat, never by holding an unrecognised one.
    T3,
    /// CARAVAN — trade routes (T4).
    T4,
    /// CRUCIBLE — elemental refineries (T5).
    T5,
    /// EXCHANGE — STAMP market-making (T6).
    T6,
    /// ASSAY — element identity court (T7).
    T7,
    /// VAULT — custody and collateral (T8).
    T8,
    /// ANCHOR — external settlement seats (T9).
    T9,
    /// HERALD — diplomacy mesh / information (T10).
    T10,
    /// LATTICE — research / knowledge office (T11).
    T11,
    /// CHORUS — culture / literature (T12).
    T12,
    /// WARDEN — defense / medium (T13).
    T13,
    /// FOUNDRY — deep industrial (T14).
    T14,
    /// NEXUS — interstellar / union coordination (T15).
    T15,
}

/// The station → tier reading, and THE ENGINE AUTHORITY for it.
///
/// **`None` is a REFUSAL, not a default.** An unrecognised station does
/// NOT fall through to `T3`: the common is the absence of a seat, never
/// the presence of an unreadable one.
///
/// **Exhaustive by construction.** Every value in [`STATIONS`] is named
/// here, and `tier_covers_every_station` fails the build if a station
/// is added without deciding its band.
pub fn tier_of(station: Option<&str>) -> Option<Tier> {
    match station {
        None => Some(Tier::T3),
        Some("progenitor") => Some(Tier::T0),
        Some("watcher") | Some("observer") => Some(Tier::T1),
        Some("sovereign") => Some(Tier::T2),
        Some("caravan") => Some(Tier::T4),
        Some("crucible") => Some(Tier::T5),
        Some("exchange") => Some(Tier::T6),
        Some("assay") => Some(Tier::T7),
        Some("vault") => Some(Tier::T8),
        Some("anchor") => Some(Tier::T9),
        Some("herald") => Some(Tier::T10),
        Some("lattice") => Some(Tier::T11),
        Some("chorus") => Some(Tier::T12),
        Some("warden") => Some(Tier::T13),
        Some("foundry") => Some(Tier::T14),
        Some("nexus") => Some(Tier::T15),
        Some(_) => None,
    }
}

/// Is this line seated in an office that serves everyone?
///
/// Any seated office (T0–T2, T4–T9) is a public function — not a
/// lifestyle instrument. **Only T3 (no seat) is non-serving.**
///
/// Returns `None` when the tier cannot be read, so a gate refuses
/// instead of guessing.
pub fn holds_a_serving_seat(station: Option<&str>) -> Option<bool> {
    tier_of(station).map(|tier| tier != Tier::T3)
}

/// T1.2 — typed preconditions for economic offices (facts of the
/// chain, not doors). Callers pass what the world has already done;
/// this function never invents history.
///
/// Returns `Ok(())` when the office may be claimed, or `Err(reason)`
/// naming the cure (never silent).
pub fn office_preconditions(
    station: &str,
    evidence: &OfficeEvidence,
) -> Result<(), &'static str> {
    match station {
        "progenitor" | "watcher" | "observer" | "sovereign" => Ok(()),
        "caravan" => {
            if evidence.element_kinds > 0 || evidence.routes_chartered > 0 {
                Ok(())
            } else {
                Err("open a vein (prospect/assay) or charter a route first")
            }
        }
        "crucible" => {
            if evidence.element_kinds > 0 {
                Ok(())
            } else {
                Err("bank at least one almanac element before claiming the crucible")
            }
        }
        "exchange" => {
            if evidence.market_clears > 0 || evidence.element_kinds > 0 {
                Ok(())
            } else {
                Err("clear a market trade or hold element inventory first")
            }
        }
        "assay" => {
            if evidence.assays_struck > 0 {
                Ok(())
            } else {
                Err("seal at least one AssayStruck before claiming the assay court")
            }
        }
        "vault" => {
            if evidence.element_kinds > 0 || evidence.market_clears > 0 {
                Ok(())
            } else {
                Err("hold matter or clear a trade before claiming the vault")
            }
        }
        "anchor" => {
            if evidence.has_crossed {
                Ok(())
            } else {
                Err("cross the void (aperture) before claiming the anchor")
            }
        }
        "herald" => {
            if evidence.routes_chartered > 0 || evidence.has_crossed {
                Ok(())
            } else {
                Err("charter a route or cross the void before claiming the herald")
            }
        }
        "lattice" => {
            if evidence.assays_struck > 0 || evidence.element_kinds > 0 {
                Ok(())
            } else {
                Err("strike an assay or hold element before claiming the lattice")
            }
        }
        "chorus" => Ok(()), // culture path — open after found
        "warden" => {
            if evidence.element_kinds > 0 {
                Ok(())
            } else {
                Err("hold matter before claiming the warden")
            }
        }
        "foundry" => {
            if evidence.element_kinds > 0 {
                Ok(())
            } else {
                Err("bank element before claiming the foundry")
            }
        }
        "nexus" => {
            if evidence.has_crossed {
                Ok(())
            } else {
                Err("cross the void before claiming the nexus")
            }
        }
        _ => Err("unknown station"),
    }
}

/// Evidence a world presents when ascending an economic office.
#[derive(Debug, Clone, Copy, Default)]
pub struct OfficeEvidence {
    pub element_kinds: u64,
    pub assays_struck: u64,
    pub routes_chartered: u64,
    pub market_clears: u64,
    pub has_crossed: bool,
}

/// One civilization: a species-level identity spanning worlds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Civilization {
    pub name: String,
    pub sigil: String,
    /// Free text — the encyclopedia grows itself ("blue whale",
    /// "monarch butterfly", "human").
    pub species: String,
    /// The CLADE tier (06 register law, ruled 2026-07-30): the taxon
    /// line between species and the axes — free text, register-only,
    /// never a mechanic and never the station tier. Biological lines
    /// carry their clade (cetacean, cephalopod, gastropod,
    /// hymenopteran…); substrate lines carry the 12 §5a substrate
    /// (plasmoid, lithic, syzygial…). Empty = unrecorded, lawful.
    pub clade: String,
    pub motto: String,
    pub disposition: String,
    pub metabolism: String,
    pub perception: String,
    pub founder: String,
    /// LINEAGE-2 (12 §5): the parent lines, append order — a DAG,
    /// never a single thread. Empty = headless-of-record (lawful). A
    /// divergence branch carries one parent; a confluent hybrid
    /// carries BOTH. Wire form: space-joined slugs in the one
    /// `lineage` field — a single-parent write is byte-identical to
    /// the LINEAGE-1 form, and old files read unchanged.
    pub lineage: Vec<String>,
    /// BAND-1: the class station held, if any (12_UNION §2–§3) —
    /// validated against `STATIONS`, seated only through an open seat.
    pub station: Option<String>,
}

/// Typed refusals of registration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryRefused {
    /// The slug already exists — join it, or choose a new name.
    AlreadyRegistered { sigil: String },
    /// An axis value outside the closed vocabulary, named.
    UnknownAxis { axis: &'static str, given: String },
    /// BAND-1 — a station outside the closed office vocabulary.
    UnknownStation { given: String },
    /// BAND-1 — the space-available law: the station's seats stand
    /// full for this lineage domain. Not a rank refused — a room
    /// filled; the band grows as the universe does.
    NoSeatOpen { station: String, domain: String, held: u64, width: u64 },
    Io(String),
}

pub struct Registry {
    root: PathBuf,
}

impl Registry {
    /// A registry rooted at `<root>/` (conventionally `var/registry`).
    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Register a new civilization. Axes are checked against the
    /// closed vocabulary; the slug derives from the name.
    pub fn create(&self, civilization: &Civilization) -> Result<(), RegistryRefused> {
        for (axis, given, allowed) in [
            ("disposition", &civilization.disposition, &DISPOSITIONS[..]),
            ("metabolism", &civilization.metabolism, &METABOLISMS[..]),
            ("perception", &civilization.perception, &PERCEPTIONS[..]),
        ] {
            if !allowed.contains(&given.as_str()) {
                return Err(RegistryRefused::UnknownAxis {
                    axis,
                    given: given.clone(),
                });
            }
        }
        // BAND-1: a claimed station must be a known office with an
        // open seat (the space-available law) — checked at the door.
        if let Some(station) = &civilization.station {
            if !STATIONS.contains(&station.as_str()) {
                return Err(RegistryRefused::UnknownStation { given: station.clone() });
            }
            let domain = lineage_domain(&civilization.metabolism, None);
            if let Some(width) = seat_width(station) {
                let held = self
                    .list()
                    .iter()
                    .filter(|seated| {
                        seated.station.as_deref() == Some(station.as_str())
                            && (station != "progenitor"
                                || lineage_domain(&seated.metabolism, None) == domain)
                    })
                    .count() as u64;
                if held >= width {
                    return Err(RegistryRefused::NoSeatOpen {
                        station: station.clone(),
                        domain: domain.into(),
                        held,
                        width,
                    });
                }
            }
        }
        let path = self.root.join(format!("{}.civ", civilization.sigil));
        if path.exists() {
            return Err(RegistryRefused::AlreadyRegistered {
                sigil: civilization.sigil.clone(),
            });
        }
        fs::create_dir_all(&self.root).map_err(|e| RegistryRefused::Io(e.to_string()))?;
        self.write_civ_tlv(&path, civilization)
            .map_err(|e| RegistryRefused::Io(e.to_string()))?;
        Ok(())
    }

    fn write_civ_tlv(
        &self,
        path: &std::path::Path,
        c: &Civilization,
    ) -> std::io::Result<()> {
        let mut out = Vec::new();
        card::put_str(card::civ::NAME, &c.name, &mut out);
        card::put_str(card::civ::SIGIL, &c.sigil, &mut out);
        card::put_str(card::civ::SPECIES, &c.species, &mut out);
        if !c.clade.is_empty() {
            card::put_str(card::civ::CLADE, &c.clade, &mut out);
        }
        card::put_str(card::civ::MOTTO, &c.motto, &mut out);
        card::put_str(card::civ::DISPOSITION, &c.disposition, &mut out);
        card::put_str(card::civ::METABOLISM, &c.metabolism, &mut out);
        card::put_str(card::civ::PERCEPTION, &c.perception, &mut out);
        card::put_str(card::civ::FOUNDER, &c.founder, &mut out);
        if !c.lineage.is_empty() {
            card::put_str(card::civ::LINEAGE, &c.lineage.join(" "), &mut out);
        }
        if let Some(station) = &c.station {
            card::put_str(card::civ::STATION, station, &mut out);
        }
        card::write_atomic(path, &out)
    }

    /// Load one entry by sigil (`.civ` TLV only).
    pub fn load(&self, sigil: &str) -> Option<Civilization> {
        let data = card::read(&self.root.join(format!("{sigil}.civ")))?;
        parse_civ_tlv(&data)
    }

    /// Every registered civilization, slug-ascending — the directory a
    /// new player joins from.
    pub fn list(&self) -> Vec<Civilization> {
        let Ok(entries) = fs::read_dir(&self.root) else {
            return Vec::new();
        };
        let mut slugs: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let path = e.path();
                if path.extension()?.to_str()? != "civ" {
                    return None;
                }
                Some(path.file_stem()?.to_string_lossy().into_owned())
            })
            .collect();
        slugs.sort();
        slugs.dedup();
        slugs.iter().filter_map(|s| self.load(s)).collect()
    }
}

fn parse_civ_tlv(data: &[u8]) -> Option<Civilization> {
    let mut c = Civilization {
        name: String::new(),
        sigil: String::new(),
        species: String::new(),
        clade: String::new(),
        motto: String::new(),
        disposition: String::new(),
        metabolism: String::new(),
        perception: String::new(),
        founder: String::new(),
        lineage: Vec::new(),
        station: None,
    };
    for (tag, value) in card::records(data) {
        match tag {
            card::civ::NAME => c.name = card::str_of(value).unwrap_or_default(),
            card::civ::SIGIL => c.sigil = card::str_of(value).unwrap_or_default(),
            card::civ::SPECIES => c.species = card::str_of(value).unwrap_or_default(),
            card::civ::CLADE => c.clade = card::str_of(value).unwrap_or_default(),
            card::civ::MOTTO => c.motto = card::str_of(value).unwrap_or_default(),
            card::civ::DISPOSITION => c.disposition = card::str_of(value).unwrap_or_default(),
            card::civ::METABOLISM => c.metabolism = card::str_of(value).unwrap_or_default(),
            card::civ::PERCEPTION => c.perception = card::str_of(value).unwrap_or_default(),
            card::civ::FOUNDER => c.founder = card::str_of(value).unwrap_or_default(),
            card::civ::LINEAGE => {
                c.lineage = card::str_of(value)
                    .map(|j| j.split_whitespace().map(String::from).collect())
                    .unwrap_or_default();
            }
            card::civ::STATION => c.station = card::str_of(value),
            _ => {}
        }
    }
    if c.sigil.is_empty() || c.disposition.is_empty() {
        return None;
    }
    Some(c)
}

/// Derive a slug the way the ledger sanitizes names.
pub fn sigil_of(name: &str) -> String {
    crate::sanitize(name)
}

/// LINEAGE-1 (12_UNION §5) — the lineage-domain reading: which
/// progenitor domain a civilization elevates toward, DERIVED from
/// metabolism + the substrate band its seat lives in (never stored,
/// never chosen). The four founding domains plus the visible-next
/// (12_UNION §2, §5a); an unsited line reads by metabolism alone.
pub fn lineage_domain(
    metabolism: &str,
    band: Option<cfe::thermal::SubstrateBand>,
) -> &'static str {
    use cfe::thermal::SubstrateBand as Band;
    match (metabolism, band) {
        ("plasmic", _) => "plasmic",
        ("mineral", Some(Band::HyperThermal)) => "thermal",
        ("mineral", _) => "lithic",
        ("biotic", Some(Band::Cryogenic)) => "cryo",
        _ => "mesic",
    }
}

impl Registry {
    /// BAND-1 fruition (the second-look audit, 2026-07-30): CLAIM a
    /// STATION SEAT for an existing line — the deed upward advancement
    /// was missing. Same door as creation: the office must be known,
    /// the room must stand open (progenitors per lineage domain); the
    /// card rewrites with the seat taken. Elevation's gates are the
    /// CALLER's evidence (the world's receipts); the registry checks
    /// the room.
    pub fn seat(&self, sigil: &str, station: &str) -> Result<(), RegistryRefused> {
        let mut card = self.load(sigil).ok_or_else(|| RegistryRefused::Io(
            format!("no such line: {sigil}"),
        ))?;
        if !STATIONS.contains(&station) {
            return Err(RegistryRefused::UnknownStation { given: station.into() });
        }
        let domain = lineage_domain(&card.metabolism, None);
        if let Some(width) = seat_width(station) {
            let held = self
                .list()
                .iter()
                .filter(|seated| {
                    seated.sigil != sigil
                        && seated.station.as_deref() == Some(station)
                        && (station != "progenitor"
                            || lineage_domain(&seated.metabolism, None) == domain)
                })
                .count() as u64;
            if held >= width {
                return Err(RegistryRefused::NoSeatOpen {
                    station: station.into(),
                    domain: domain.into(),
                    held,
                    width,
                });
            }
        }
        card.station = Some(station.into());
        // Rewrite the TLV card whole.
        let path = self.root.join(format!("{sigil}.civ"));
        let _ = fs::remove_file(&path);
        self.create(&card)
    }

    /// LINEAGE-1: resolve a civilization's lineage HEAD — the root of
    /// its pointer chain. A line whose pointer is absent (or dangles
    /// into the unrecorded past) resolves to the deepest entry that
    /// EXISTS: headless is a lawful state, not an error (12_UNION §5).
    /// A corrupt cycle stops deterministically at first revisit.
    pub fn resolve_head(&self, sigil: &str) -> Option<String> {
        let mut current = self.load(sigil)?;
        let mut visited = vec![current.sigil.clone()];
        // LINEAGE-2: the PRIMARY chain — first parent per hop (append
        // order); the full DAG reads through `resolve_heads`.
        while let Some(parent) = current.lineage.first() {
            if visited.iter().any(|seen| seen == parent) {
                break;
            }
            match self.load(parent) {
                Some(ancestor) => {
                    visited.push(ancestor.sigil.clone());
                    current = ancestor;
                }
                None => break,
            }
        }
        Some(current.sigil)
    }

    /// LINEAGE-2 (12 §5): EVERY root of the provenance DAG — a confluent
    /// hybrid descends from both parents' lines, so a line may carry
    /// several heads. A walk's terminus is a head when no unvisited
    /// recorded parent remains: empty lineage, dangling pointers, and
    /// cycles all stop there deterministically. Slug-sorted, deduped.
    pub fn resolve_heads(&self, sigil: &str) -> Vec<String> {
        let Some(start) = self.load(sigil) else {
            return Vec::new();
        };
        let mut visited = vec![start.sigil.clone()];
        let mut frontier = vec![start];
        let mut heads = Vec::new();
        while let Some(current) = frontier.pop() {
            let mut pushed = 0;
            for parent in &current.lineage {
                if visited.iter().any(|seen| seen == parent) {
                    continue;
                }
                if let Some(ancestor) = self.load(parent) {
                    visited.push(ancestor.sigil.clone());
                    frontier.push(ancestor);
                    pushed += 1;
                }
            }
            if pushed == 0 {
                heads.push(current.sigil);
            }
        }
        heads.sort();
        heads.dedup();
        heads
    }
}
