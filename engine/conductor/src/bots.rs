//! Lore lives in data, not source: bot specs load from
//! `<var>/bots.json`; `write_default_bots` seeds it once, and from
//! then on the operator owns the file — edit, add, replace freely.
//! The founding defaults below are the ONLY lore in Rust, and they
//! exist solely to write that first file.

use crate::{BotSpec, Policy};
use mirror::registry::Civilization;
use std::path::Path;

/// The founding NPCs (founder "conductor" — the observer marks ⚙).
pub fn founding_bots() -> Vec<BotSpec> {
    let civ = |name: &str, slug: &str, species: &str, motto: &str, d: &str, m: &str, p: &str| {
        Civilization {
            name: name.into(),
            slug: slug.into(),
            species: species.into(),
            motto: motto.into(),
            disposition: d.into(),
            metabolism: m.into(),
            perception: p.into(),
            founder: "conductor".into(),
            lineage: None,
        }
    };
    vec![
        BotSpec {
            civilization: civ(
                "Slagborn", "slagborn", "tardigrade", "we eat what worlds discard",
                "expansionist", "mineral", "planar",
            ),
            world_name: "cinder-reach".into(),
            planet: "bask".into(),
            colony: "first-vent".into(),
            policy: Policy::Aggressive,
        },
        BotSpec {
            civilization: civ(
                "Deep Chorus", "deep-chorus", "blue whale", "we heard it before you saw it",
                "epicurean", "biotic", "harmonic",
            ),
            world_name: "kepler-passage".into(),
            planet: "tidefall".into(),
            colony: "songline".into(),
            policy: Policy::Steady,
        },
        BotSpec {
            civilization: civ(
                "Hivelight", "hivelight", "honeybee", "the comb computes",
                "empiricist", "biotic", "volumetric",
            ),
            world_name: "amber-veil".into(),
            planet: "comb".into(),
            colony: "waggle-reach".into(),
            policy: Policy::Industry,
        },
    ]
}

/// Seed `<path>` with the founding bots (refuses to overwrite — the
/// operator's edits are never clobbered).
pub fn write_default_bots(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        return Ok(());
    }
    let mut out = String::from("[\n");
    let bots = founding_bots();
    for (i, bot) in bots.iter().enumerate() {
        let c = &bot.civilization;
        out.push_str(&format!(
            "  {{\"name\":\"{}\",\"slug\":\"{}\",\"species\":\"{}\",\"motto\":\"{}\",\
             \"disposition\":\"{}\",\"metabolism\":\"{}\",\"perception\":\"{}\",\
             \"world\":\"{}\",\"planet\":\"{}\",\"colony\":\"{}\",\"policy\":\"{}\"}}{}\n",
            c.name, c.slug, c.species, c.motto,
            c.disposition, c.metabolism, c.perception,
            bot.world_name, bot.planet, bot.colony, bot.policy.describe(),
            if i + 1 < bots.len() { "," } else { "" },
        ));
    }
    out.push_str("]\n");
    std::fs::write(path, out)
}

/// Load bot specs from a bots.json written by `write_default_bots`
/// (or hand-edited to the same flat shape). Unparseable entries are
/// skipped with a stderr note, never silently.
pub fn load_bots(path: &Path) -> Option<Vec<BotSpec>> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut specs = Vec::new();
    for object in split_objects(&text) {
        let field = |key: &str| extract_string(&object, key);
        let parsed = (|| {
            Some(BotSpec {
                civilization: Civilization {
                    name: field("name")?,
                    slug: field("slug")?,
                    species: field("species").unwrap_or_default(),
                    motto: field("motto").unwrap_or_default(),
                    disposition: field("disposition")?,
                    metabolism: field("metabolism")?,
                    perception: field("perception")?,
                    founder: "conductor".into(),
                    lineage: field("lineage"),
                },
                world_name: field("world")?,
                planet: field("planet")?,
                colony: field("colony")?,
                policy: Policy::parse(&field("policy")?)?,
            })
        })();
        match parsed {
            Some(spec) => specs.push(spec),
            None => eprintln!("conductor: skipping malformed bot entry: {object}"),
        }
    }
    (!specs.is_empty()).then_some(specs)
}

/// Split a JSON array text into its top-level `{...}` objects by
/// brace counting (our own writer's shape; strings hold no braces).
fn split_objects(text: &str) -> Vec<String> {
    let mut objects = Vec::new();
    let mut depth = 0usize;
    let mut start = None;
    for (i, c) in text.char_indices() {
        match c {
            '{' => {
                if depth == 0 {
                    start = Some(i);
                }
                depth += 1;
            }
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    if let Some(s) = start.take() {
                        objects.push(text[s..=i].to_string());
                    }
                }
            }
            _ => {}
        }
    }
    objects
}

/// Extract `"key":"value"` from one flat object.
fn extract_string(text: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let after = &text[text.find(&needle)? + needle.len()..];
    let after = after.trim_start().strip_prefix(':')?.trim_start();
    let inner = after.strip_prefix('"')?;
    Some(inner[..inner.find('"')?].to_string())
}
