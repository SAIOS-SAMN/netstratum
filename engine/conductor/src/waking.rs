//! docs/82 step 3: a mind that wakes becomes a line in the conductor.
//!
//! - Its METABOLISM is its medium's, in the registry's closed vocabulary:
//!   geologic minds are mineral, chemical minds biotic, plasmic minds
//!   plasmic. Its disposition and perception, and its NAME, are spoken from
//!   the seed (flavour only; docs/82: no types).
//! - Its place is its system's stand-in place (until the sphere).
//! - A WAKING TAKES NO SEAT (operator 2026-10-21, "realms EMERGE, not
//!   drawn"): every tier is reached by what the line's lineage does
//!   (ancients by birth near the core, progenitors by folding a gate,
//!   sovereigns by holding their own system).
use crate::life::Waking;
use crate::pulse::{draw, Universe};
use crate::{Policy, PolitySpec};
use ledger::registry::{Civilization, Registry};

const ONSETS: [&str; 12] = ["k", "t", "v", "r", "s", "m", "n", "th", "z", "l", "dr", "sh"];
const VOWELS: [&str; 6] = ["a", "e", "i", "o", "u", "ae"];

fn spoken(seed: &[u8; 32], path: &[u64], salt: u32) -> String {
    let n = 2 + (draw(seed, path, "name.len", salt) % 2) as u32;
    (0..n)
        .map(|i| {
            let d = draw(seed, path, "name", salt * 8 + i);
            format!("{}{}", ONSETS[(d % 12) as usize], VOWELS[((d / 12) % 6) as usize])
        })
        .collect()
}

/// The line a waking founds, registered and seated, or None if no free name
/// (or the world already holds a line).
pub fn found(u: &Universe, seed: &[u8; 32], w: &Waking, registry: &Registry, world: Option<&crate::life::World>) -> Option<PolitySpec> {
    let world_name = format!("w{}", w.path.iter().map(|p| p.to_string()).collect::<Vec<_>>().join("-"));
    if crate::polities::world_name_taken(registry, &world_name) {
        return None;
    }
    let sigil = (0..8u32).map(|salt| spoken(seed, &w.path, salt)).find(|s| registry.load(s).is_none())?;
    let mut name = sigil.clone();
    name[..1].make_ascii_uppercase();
    let metabolism = match w.medium { "geologic" => "mineral", "plasmic" => "plasmic", _ => "biotic" };
    let pick = |field: &str, n: u64| (draw(seed, &w.path, field, 0) % n) as usize;
    let civilization = Civilization {
        name: name.clone(),
        sigil: sigil.clone(),
        species: format!("{}-born", w.medium),
        clade: w.medium.to_string(),
        motto: format!("we woke at bell {} in the {} of our world", w.bell, w.medium),
        disposition: ledger::registry::DISPOSITIONS[pick("disposition", 3)].to_string(),
        metabolism: metabolism.to_string(),
        perception: ledger::registry::PERCEPTIONS[pick("perception", 4)].to_string(),
        founder: "the ladder".into(),
        lineage: Vec::new(),
        station: None,
    };
    registry.create(&civilization).ok()?;
    let civilization = registry.load(&sigil)?;
    let position = crate::seating::place_of(u, &w.path)?;
    let spec = PolitySpec {
        civilization,
        world_name: world_name.clone(),
        planet: format!("{sigil}-world"),
        colony: format!("{sigil}-waking"),
        policy: Policy::Steady,
        position: position.clone(),
        star: None,
        albedo: None,
        orbit: None,
        gauge: None,
        composition: None,
        place: Some(w.path.clone()),
    };
    // Its own world's era-1 siting (the refactor onto era 1's physics).
    let mut spec = spec;
    if let Some((class, albedo, orbit)) = world.and_then(|x| crate::seating::site_of(&x.heat, &x.burn)) {
        spec.star = Some(class);
        spec.albedo = Some(albedo);
        spec.orbit = Some(orbit);
    }
    spec.composition = crate::seating::composition_of(u, &w.path);
    let seat = ledger::registry::Seat { world: world_name, planet: spec.planet.clone(), colony: spec.colony.clone(), position, policy: spec.policy.describe().to_string() };
    let _ = registry.write_founded_at(&sigil, &seat);
    Some(spec)
}
