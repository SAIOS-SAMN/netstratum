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
    /// The parent civilization's slug, for speciated descendants —
    /// registry-recorded now, receipted at G1 like all identity.
    pub lineage: Option<String>,
}

/// Typed refusals of registration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryRefused {
    /// The slug already exists — join it, or choose a new name.
    AlreadyRegistered { slug: String },
    /// An axis value outside the closed vocabulary, named.
    UnknownAxis { axis: &'static str, given: String },
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
        if let Some(lineage) = &civilization.lineage {
            w.field_str("lineage", lineage);
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
            lineage: extract_string(&text, "lineage"),
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
