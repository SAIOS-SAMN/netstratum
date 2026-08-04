//! Lore lives in data, not source: polity specs load from
//! `<var>/polities.json`; `write_default_polities` seeds it once, and from
//! then on the operator owns the file — edit, add, replace freely.
//! The founding defaults below are the ONLY lore in Rust, and they
//! exist solely to write that first file.

use crate::{PolitySpec, Policy};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use ledger::registry::Civilization;
use std::path::Path;

/// The founding NPCs (founder "conductor" — the observer marks ⚙).
pub fn founding_polities() -> Vec<PolitySpec> {
    let seated = |name: &str,
                  sigil: &str,
                  species: &str,
                  motto: &str,
                  d: &str,
                  m: &str,
                  p: &str,
                  station: Option<&str>,
                  lineage: &[&str]| {
        Civilization {
            name: name.into(),
            sigil: sigil.into(),
            species: species.into(),
            clade: String::new(),
            motto: motto.into(),
            disposition: d.into(),
            metabolism: m.into(),
            perception: p.into(),
            founder: "conductor".into(),
            lineage: lineage.iter().map(|s| (*s).into()).collect(),
            station: station.map(|s| s.into()),
        }
    };
    let civ = |name: &str, sigil: &str, species: &str, motto: &str, d: &str, m: &str, p: &str| {
        seated(name, sigil, species, motto, d, m, p, None, &[])
    };
    vec![
        // BOT-THERM: the Furnace seat, pinned openly in the file — a G
        // sun at half orbit under half albedo (stellar load 2, past
        // the mesic ceiling): the shells answer, the joules mint.
        // **THE FIFTH SEAT** (`12_UNION` §2, operator ruling 2026-07-30):
        // *the tardigrade line sits in the progenitor band from this
        // aeon — Tunborn, the Slagborn line's OWN head.*
        //
        // **This roster contradicted that landed ruling.** The seed file
        // has carried Tunborn at `station: progenitor` since the ruling;
        // these built-in defaults did not, and they are what a board
        // boots with when `<var>/polities.json` is absent —
        // `write_default_polities` refuses to overwrite, so the fallback
        // is not a rare path, it is the path a rebuilt var takes. A
        // board running the fallback therefore ran the tardigrade line
        // with NO head and no seat above it: five lines, all
        // `station: None`, all T3, and the whole progenitor band
        // structurally absent from play.
        //
        // Measured 2026-08-04: the live var had exactly that shape.
        PolitySpec {
            civilization: seated(
                "Tunborn", "tunborn", "tardigrade progenitor",
                "the dark between stars is a bed, not a wall",
                "expansionist", "mineral", "planar",
                Some("progenitor"), &[],
            ),
            world_name: "tun-reach".into(),
            planet: "husk".into(),
            colony: "first-wake".into(),
            policy: Policy::Steady,
            position: (ratio(2, 1), ratio(-3, 1), ratio(1, 1)),
            star: Some(StarClass::G),
            albedo: Some(ratio(1, 2)),
            orbit: Some(ratio(1, 2)),
            gauge: None,
        },
        PolitySpec {
            civilization: seated(
                "Slagborn", "slagborn", "tardigrade", "we eat what worlds discard",
                "expansionist", "mineral", "planar",
                // The line RESOLVES to its head. Without this the
                // lineage law has nothing to read and Slagborn is a
                // tardigrade line with no origin.
                None, &["tunborn"],
            ),
            world_name: "cinder-reach".into(),
            planet: "bask".into(),
            colony: "first-vent".into(),
            policy: Policy::Aggressive,
            position: (ratio(0, 1), ratio(0, 1), ratio(0, 1)),
            star: Some(StarClass::G),
            albedo: Some(ratio(1, 2)),
            orbit: Some(ratio(1, 2)),
            gauge: None,
        },
        // The mesic seats carry NO thermal fields: their orbits derive
        // at genesis from the registry axes through the chronicle
        // beacon — the derived path stays exercised in every era.
        PolitySpec {
            civilization: civ(
                "Echowright", "deep-chorus", "blue whale", "we heard it before you saw it",
                "epicurean", "biotic", "harmonic",
            ),
            world_name: "kepler-passage".into(),
            planet: "tidefall".into(),
            colony: "songline".into(),
            policy: Policy::Steady,
            position: (ratio(12, 1), ratio(5, 1), ratio(3, 1)),
            star: None,
            albedo: None,
            orbit: None,
            gauge: None,
        },
        PolitySpec {
            civilization: civ(
                "Hivelight", "hivelight", "honeybee", "the comb computes",
                "empiricist", "biotic", "volumetric",
            ),
            world_name: "amber-veil".into(),
            planet: "comb".into(),
            colony: "waggle-reach".into(),
            policy: Policy::Industry,
            position: (ratio(-7, 1), ratio(9, 1), ratio(2, 1)),
            star: None,
            albedo: None,
            orbit: None,
            gauge: None,
        },
        PolitySpec {
            civilization: civ(
                "Eightfold", "eightfold", "octopus", "eight arms, one argument",
                "epicurean", "biotic", "volumetric",
            ),
            world_name: "jovian-reach".into(),
            planet: "europa".into(),
            colony: "brine-garden".into(),
            policy: Policy::Steady,
            position: (ratio(5, 1), ratio(58, 1), ratio(61, 1)),
            star: None,
            albedo: None,
            orbit: None,
            gauge: None,
        },
        // BOT-THERM: the Frost seat — an M sun at unit orbit lands in
        // the cryogenic vault (05 §1.2): sink banks, joules are life,
        // and the freeze-lock is honestly on the table.
        PolitySpec {
            civilization: civ(
                "Frostwright", "frostwright", "wood frog", "we bank what the fire cannot hold",
                "empiricist", "biotic", "planar",
            ),
            world_name: "dimlight-hold".into(),
            planet: "rime".into(),
            colony: "still-yard".into(),
            policy: Policy::Steady,
            position: (ratio(-20, 1), ratio(-11, 1), ratio(44, 1)),
            star: Some(StarClass::M),
            albedo: Some(ratio(0, 1)),
            orbit: Some(ratio(1, 1)),
            gauge: None,
        },
    ]
}

/// Seed `<path>` with the founding polities (refuses to overwrite — the
/// operator's edits are never clobbered).
pub fn write_default_polities(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        return Ok(());
    }
    let mut out = String::from("[\n");
    let polities = founding_polities();
    for (i, polity) in polities.iter().enumerate() {
        let c = &polity.civilization;
        // BOT-THERM: the optional siting fields — written only when
        // pinned, so derived seats stay derived in the seeded file.
        let mut thermal = String::new();
        if let Some(class) = polity.star {
            thermal.push_str(&format!(",\"star\":\"{}\"", star_word(class)));
        }
        if let Some(albedo) = &polity.albedo {
            thermal.push_str(&format!(",\"albedo\":\"{}\"", ratio_text(albedo)));
        }
        if let Some(orbit) = &polity.orbit {
            thermal.push_str(&format!(",\"orbit\":\"{}\"", ratio_text(orbit)));
        }
        out.push_str(&format!(
            "  {{\"name\":\"{}\",\"sigil\":\"{}\",\"species\":\"{}\",\"motto\":\"{}\",\
             \"disposition\":\"{}\",\"metabolism\":\"{}\",\"perception\":\"{}\",\
             \"world\":\"{}\",\"planet\":\"{}\",\"colony\":\"{}\",\"policy\":\"{}\",\
             \"x\":\"{}\",\"y\":\"{}\",\"z\":\"{}\"{}}}{}\n",
            c.name, c.sigil, c.species, c.motto,
            c.disposition, c.metabolism, c.perception,
            polity.world_name, polity.planet, polity.colony, polity.policy.describe(),
            ratio_text(&polity.position.0), ratio_text(&polity.position.1),
            ratio_text(&polity.position.2),
            thermal,
            if i + 1 < polities.len() { "," } else { "" },
        ));
    }
    out.push_str("]\n");
    std::fs::write(path, out)
}

/// Load polity specs from a polities.json written by `write_default_polities`
/// (or hand-edited to the same flat shape). Unparseable entries are
/// skipped with a stderr note, never silently.
pub fn load_polities(path: &Path) -> Option<Vec<PolitySpec>> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut specs = Vec::new();
    for object in split_objects(&text) {
        let field = |key: &str| extract_string(&object, key);
        let parsed = (|| {
            Some(PolitySpec {
                civilization: Civilization {
                    name: field("name")?,
                    sigil: field("sigil").or_else(|| field("slug"))?,
                    species: field("species").unwrap_or_default(),
                    clade: field("clade").unwrap_or_default(),
                    motto: field("motto").unwrap_or_default(),
                    disposition: field("disposition")?,
                    metabolism: field("metabolism")?,
                    perception: field("perception")?,
                    founder: "conductor".into(),
                    lineage: field("lineage")
                        .map(|joined| joined.split_whitespace().map(String::from).collect())
                        .unwrap_or_default(),
                    station: field("station"),
                },
                world_name: field("world")?,
                planet: field("planet")?,
                colony: field("colony")?,
                policy: Policy::parse(&field("policy")?)?,
                position: (
                    field("x").and_then(|v| parse_ratio(&v)).unwrap_or_else(|| ratio(0, 1)),
                    field("y").and_then(|v| parse_ratio(&v)).unwrap_or_else(|| ratio(0, 1)),
                    field("z").and_then(|v| parse_ratio(&v)).unwrap_or_else(|| ratio(0, 1)),
                ),
                // BOT-THERM compat: files without these fields (the
                // pre-thermal shape) load with `None` — the seat then
                // derives at genesis. An unparseable value reads as
                // absent, never as a refusal of the whole entry.
                star: field("star").and_then(|word| parse_star(&word)),
                albedo: field("albedo").and_then(|v| parse_ratio(&v)),
                orbit: field("orbit").and_then(|v| parse_ratio(&v)),
                gauge: field("gauge").and_then(|v| parse_ratio(&v)),
            })
        })();
        match parsed {
            Some(spec) => specs.push(spec),
            None => eprintln!("conductor: skipping malformed polity entry: {object}"),
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

/// BOT-THERM: the star words of the operator's file — one word per
/// StarClass (NS-6 §7 wire codes stay the protocol truth; these are
/// the human spellings).
fn star_word(class: StarClass) -> &'static str {
    match class {
        StarClass::O => "o",
        StarClass::B => "b",
        StarClass::A => "a",
        StarClass::F => "f",
        StarClass::G => "g",
        StarClass::K => "k",
        StarClass::M => "m",
        StarClass::BrownDwarf => "brown-dwarf",
        StarClass::WhiteDwarf => "white-dwarf",
        StarClass::RedGiant => "red-giant",
        StarClass::NeutronStar => "neutron-star",
        StarClass::WolfRayet => "wolf-rayet",
        StarClass::LuminalWhiteHole => "luminal-white-hole",
    }
}

fn parse_star(word: &str) -> Option<StarClass> {
    match word {
        "o" => Some(StarClass::O),
        "b" => Some(StarClass::B),
        "a" => Some(StarClass::A),
        "f" => Some(StarClass::F),
        "g" => Some(StarClass::G),
        "k" => Some(StarClass::K),
        "m" => Some(StarClass::M),
        "brown-dwarf" => Some(StarClass::BrownDwarf),
        "white-dwarf" => Some(StarClass::WhiteDwarf),
        "red-giant" => Some(StarClass::RedGiant),
        "neutron-star" => Some(StarClass::NeutronStar),
        "wolf-rayet" => Some(StarClass::WolfRayet),
        "luminal-white-hole" => Some(StarClass::LuminalWhiteHole),
        _ => None,
    }
}

/// The lens's exact rendering: whole if whole, else p/q.
fn ratio_text(r: &Ratio) -> String {
    use num_traits::One;
    if r.denom().is_one() {
        r.numer().to_string()
    } else {
        format!("{}/{}", r.numer(), r.denom())
    }
}

/// Parse `p/q` or a plain integer.
fn parse_ratio(text: &str) -> Option<Ratio> {
    use num_bigint::BigInt;
    use std::str::FromStr;
    let (numer, denom) = match text.split_once('/') {
        Some((n, d)) => (n, d),
        None => (text, "1"),
    };
    Some(Ratio::new(
        BigInt::from_str(numer.trim()).ok()?,
        BigInt::from_str(denom.trim()).ok()?,
    ))
}
