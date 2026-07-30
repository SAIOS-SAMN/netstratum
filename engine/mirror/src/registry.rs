//! The civilization registry (contract §8): WHO exists, across worlds.
//!
//! One JSON file per civilization under `var/registry/`. An entry is
//! IDENTITY ONLY until the genesis landings give the axes mechanics —
//! recorded, displayed, joined; changing no rule today. When G1 lands,
//! the entry is re-committed as a genesis receipt and becomes binding.

use crate::json::{extract_string, JsonWriter};
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

/// BAND-1 (12_UNION §2–§3): the class STATIONS — offices in the
/// union, never ranks. Append-only like the axes. A civilization with
/// no station is simply a people of the bands (the common, lawful
/// state).
pub const STATIONS: [&str; 4] = ["progenitor", "watcher", "observer", "sovereign"];

/// BAND-1 (12_UNION §1): a station's seat width — SPACE AVAILABLE,
/// not rank. `None` is unbounded (every charter-holding civilization
/// is a Sovereign). Progenitor width is PER LINEAGE DOMAIN (8 seats
/// per species, 12_UNION §2 — domains open as substrates elevate, so
/// the band grows by domains, not by widening). Constants are
/// placeholder parameters; the fiber-growth reading (capacity as a
/// function of minted planes) awaits AEON-1b's plane surface.
pub fn seat_width(station: &str) -> Option<u64> {
    match station {
        "progenitor" => Some(8),
        "watcher" | "observer" => Some(16),
        _ => None,
    }
}

/// One civilization: a species-level identity spanning worlds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Civilization {
    pub name: String,
    pub slug: String,
    /// Free text — the encyclopedia grows itself ("blue whale",
    /// "monarch butterfly", "human").
    pub species: String,
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
    AlreadyRegistered { slug: String },
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
        let path = self.root.join(format!("{}.json", civilization.slug));
        if path.exists() {
            return Err(RegistryRefused::AlreadyRegistered {
                slug: civilization.slug.clone(),
            });
        }
        fs::create_dir_all(&self.root).map_err(|e| RegistryRefused::Io(e.to_string()))?;
        let mut w = JsonWriter::new();
        w.open();
        w.field_str("name", &civilization.name);
        w.field_str("slug", &civilization.slug);
        w.field_str("species", &civilization.species);
        w.field_str("motto", &civilization.motto);
        w.key("axes");
        w.open();
        w.field_str("disposition", &civilization.disposition);
        w.field_str("metabolism", &civilization.metabolism);
        w.field_str("perception", &civilization.perception);
        w.close();
        w.field_str("founder", &civilization.founder);
        if !civilization.lineage.is_empty() {
            w.field_str("lineage", &civilization.lineage.join(" "));
        }
        if let Some(station) = &civilization.station {
            w.field_str("station", station);
        }
        w.close();
        let mut text = w.finish();
        text.push('\n');
        fs::write(&path, text).map_err(|e| RegistryRefused::Io(e.to_string()))?;
        Ok(())
    }

    /// Load one entry by slug.
    pub fn load(&self, slug: &str) -> Option<Civilization> {
        let text = fs::read_to_string(self.root.join(format!("{slug}.json"))).ok()?;
        Some(Civilization {
            name: extract_string(&text, "name")?,
            slug: extract_string(&text, "slug")?,
            species: extract_string(&text, "species").unwrap_or_default(),
            motto: extract_string(&text, "motto").unwrap_or_default(),
            disposition: extract_string(&text, "disposition")?,
            metabolism: extract_string(&text, "metabolism")?,
            perception: extract_string(&text, "perception")?,
            founder: extract_string(&text, "founder").unwrap_or_default(),
            lineage: extract_string(&text, "lineage")
                .map(|joined| joined.split_whitespace().map(String::from).collect())
                .unwrap_or_default(),
            station: extract_string(&text, "station"),
        })
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
                (path.extension().is_some_and(|x| x == "json"))
                    .then(|| path.file_stem().unwrap().to_string_lossy().into_owned())
            })
            .collect();
        slugs.sort();
        slugs.iter().filter_map(|s| self.load(s)).collect()
    }
}

/// Derive a slug the way the mirror sanitizes names.
pub fn slug_of(name: &str) -> String {
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
    pub fn seat(&self, slug: &str, station: &str) -> Result<(), RegistryRefused> {
        let mut card = self.load(slug).ok_or_else(|| RegistryRefused::Io(
            format!("no such line: {slug}"),
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
                    seated.slug != slug
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
        let path = self.root.join(format!("{slug}.json"));
        std::fs::remove_file(&path).map_err(|e| RegistryRefused::Io(e.to_string()))?;
        self.create(&card)
    }

    /// LINEAGE-1: resolve a civilization's lineage HEAD — the root of
    /// its pointer chain. A line whose pointer is absent (or dangles
    /// into the unrecorded past) resolves to the deepest entry that
    /// EXISTS: headless is a lawful state, not an error (12_UNION §5).
    /// A corrupt cycle stops deterministically at first revisit.
    pub fn resolve_head(&self, slug: &str) -> Option<String> {
        let mut current = self.load(slug)?;
        let mut visited = vec![current.slug.clone()];
        // LINEAGE-2: the PRIMARY chain — first parent per hop (append
        // order); the full DAG reads through `resolve_heads`.
        while let Some(parent) = current.lineage.first() {
            if visited.iter().any(|seen| seen == parent) {
                break;
            }
            match self.load(parent) {
                Some(ancestor) => {
                    visited.push(ancestor.slug.clone());
                    current = ancestor;
                }
                None => break,
            }
        }
        Some(current.slug)
    }

    /// LINEAGE-2 (12 §5): EVERY root of the ancestry DAG — a confluent
    /// hybrid descends from both parents' lines, so a line may carry
    /// several heads. A walk's terminus is a head when no unvisited
    /// recorded parent remains: empty lineage, dangling pointers, and
    /// cycles all stop there deterministically. Slug-sorted, deduped.
    pub fn resolve_heads(&self, slug: &str) -> Vec<String> {
        let Some(start) = self.load(slug) else {
            return Vec::new();
        };
        let mut visited = vec![start.slug.clone()];
        let mut frontier = vec![start];
        let mut heads = Vec::new();
        while let Some(current) = frontier.pop() {
            let mut pushed = 0;
            for parent in &current.lineage {
                if visited.iter().any(|seen| seen == parent) {
                    continue;
                }
                if let Some(ancestor) = self.load(parent) {
                    visited.push(ancestor.slug.clone());
                    frontier.push(ancestor);
                    pushed += 1;
                }
            }
            if pushed == 0 {
                heads.push(current.slug);
            }
        }
        heads.sort();
        heads.dedup();
        heads
    }
}
