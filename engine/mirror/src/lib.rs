//! The world mirror (substrate contract §7–§8).
//!
//! THE TOPOLOGY: a CIVILIZATION (registry identity, spans worlds)
//! settles COLONIES on PLANETS inside a WORLD — the world is the
//! simulation container: one clock, one chronicle, one receipt chain,
//! one future mesh unit. Layout:
//!
//! - `var/registry/<civ>.json` — who exists ([`registry::Registry`])
//! - `var/worlds/<world>/world.json` — the container's own card
//! - `var/worlds/<world>/chronicle.jsonl` — ONE receipt stream per
//!   world, append-only, carrying the digest chain and the normative
//!   TLV event bytes (hex)
//! - `var/worlds/<world>/planets/<planet>/state.json` — the colony
//!   snapshot on that planet (atomic replace per write)
//! - `var/worlds/<world>/inbox/` — steward directives awaiting the
//!   game loop; applied or refused ones are archived, never deleted
//! - `var/worlds/<world>/solves/` — proof-of-solution certificates
//!
//! Two laws:
//!
//! - **The mirror is a REFLECTION, never an authority.** The chronicle
//!   in memory is the world; the mirror re-renders it. Nothing ever
//!   reads state.json back into a world — replay grounds worlds.
//! - **Observation and intervention are different verbs.** Outside
//!   readers (the observer) only read. Outside writers (the steward)
//!   only write to `inbox/`, and a directive becomes real ONLY by
//!   passing through the game loop as a receipted, scope-checked
//!   NS-6 action.

use cfe::ratio::Ratio;
use cfe::vrp::Receipt;
use cfe::world::World;
use num_traits::One;
use std::fs;
use std::io::Write as _;
use std::path::PathBuf;

mod json;
pub mod registry;
use json::JsonWriter;

/// A steward directive: a requested verb, to be executed by the game
/// loop AS the steward entity under its granted scopes. The file is a
/// request, never a state change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directive {
    pub verb: String,
    pub args: Vec<String>,
    /// The entity handle the directive acts as (NS-6 containment).
    pub acting_as: String,
}

/// One pending directive: its file (for archiving) and its content.
#[derive(Debug, Clone)]
pub struct PendingDirective {
    pub path: PathBuf,
    pub directive: Directive,
}

/// The disposition the game loop reached for a directive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    Applied,
    Refused,
}

pub struct Mirror {
    root: PathBuf,
}

impl Mirror {
    /// A mirror rooted at `<root>/` (conventionally `var/worlds`).
    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// The directory for one world (created on first touch).
    pub fn world_dir(&self, world: &str) -> PathBuf {
        self.root.join(sanitize(world))
    }

    /// Reflect one colony: atomically replace the planet's
    /// `state.json`, refresh `world.json`, append any receipts the
    /// world's stream does not yet carry. Call after every command;
    /// appending is incremental.
    pub fn record(
        &self,
        world_name: &str,
        planet: &str,
        civilization: &str,
        world: &World,
    ) -> std::io::Result<()> {
        let dir = self.world_dir(world_name);
        fs::create_dir_all(dir.join("inbox"))?;
        fs::create_dir_all(dir.join("solves"))?;
        let planet_dir = dir.join("planets").join(sanitize(planet));
        fs::create_dir_all(&planet_dir)?;

        // state.json — write whole to a sibling, then rename (atomic
        // on POSIX): a reader never sees a torn snapshot.
        let state = render_state(world_name, planet, civilization, world);
        let tmp = planet_dir.join("state.json.tmp");
        fs::write(&tmp, state)?;
        fs::rename(&tmp, planet_dir.join("state.json"))?;

        // world.json — the container's card: name + settled planets.
        let mut planets: Vec<String> = fs::read_dir(dir.join("planets"))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        planets.sort();
        let mut w = JsonWriter::new();
        w.open();
        w.field_str("world", world_name);
        w.key("planets");
        w.array(planets.into_iter());
        w.close();
        let tmp = dir.join("world.json.tmp");
        fs::write(&tmp, w.finish())?;
        fs::rename(&tmp, dir.join("world.json"))?;

        // chronicle.jsonl — append receipts beyond what is mirrored.
        let path = dir.join("chronicle.jsonl");
        let already = if path.exists() {
            fs::read_to_string(&path)?.lines().count()
        } else {
            0
        };
        let receipts = world.chronicle().receipts();
        if receipts.len() > already {
            let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
            for receipt in &receipts[already..] {
                let mut line = render_receipt(receipt);
                line.push('\n');
                file.write_all(line.as_bytes())?;
            }
        }
        Ok(())
    }

    /// Read pending directives, inbox-name ascending (the steward
    /// numbers them; order is the steward's stated order).
    pub fn pending_directives(&self, world: &str) -> std::io::Result<Vec<PendingDirective>> {
        let inbox = self.world_dir(world).join("inbox");
        if !inbox.exists() {
            return Ok(Vec::new());
        }
        let mut paths: Vec<PathBuf> = fs::read_dir(&inbox)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "json"))
            .collect();
        paths.sort();
        let mut out = Vec::new();
        for path in paths {
            if let Some(directive) = parse_directive(&fs::read_to_string(&path)?) {
                out.push(PendingDirective { path, directive });
            }
        }
        Ok(out)
    }

    /// Archive a processed directive under `inbox/applied/` or
    /// `inbox/refused/` — dispositions are permanent, never deleted.
    pub fn archive(&self, pending: &PendingDirective, disposition: Disposition) -> std::io::Result<()> {
        let sub = match disposition {
            Disposition::Applied => "applied",
            Disposition::Refused => "refused",
        };
        let dir = pending.path.parent().expect("inbox file has a parent").join(sub);
        fs::create_dir_all(&dir)?;
        let name = pending.path.file_name().expect("inbox file has a name");
        fs::rename(&pending.path, dir.join(name))
    }

    /// Record a proof-of-solution: one JSON file per confirmed task.
    /// `depth` is the difficulty rung the solve was earned at (the
    /// observer's DEPTH column reads the maximum).
    pub fn record_solve(
        &self,
        world: &str,
        task: &str,
        program: &str,
        certificate_hex: &str,
        depth: u32,
    ) -> std::io::Result<()> {
        let dir = self.world_dir(world).join("solves");
        fs::create_dir_all(&dir)?;
        let mut w = JsonWriter::new();
        w.open();
        w.field_str("task", task);
        w.field_str("program", program);
        w.field_str("certificate", certificate_hex);
        w.field_u64("depth", depth as u64);
        w.close();
        fs::write(dir.join(format!("{}.json", sanitize(task))), w.finish())
    }
}

/// Filesystem-safe names (worlds, planets, civilizations, tasks):
/// lowercase alphanumerics and dashes, everything else becomes a dash.
pub(crate) fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| {
            let c = c.to_ascii_lowercase();
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

/// Exact rendering: integers plain, everything else `p/q` — never a
/// decimal, never a float.
fn ratio_str(r: &Ratio) -> String {
    if r.denom().is_one() {
        r.numer().to_string()
    } else {
        format!("{}/{}", r.numer(), r.denom())
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn resource_name(r: cfe::dop::Resource) -> &'static str {
    match r {
        cfe::dop::Resource::Iron => "iron",
        cfe::dop::Resource::Biomass => "biomass",
        cfe::dop::Resource::Alloy => "alloy",
        cfe::dop::Resource::Diamond => "diamond",
    }
}

fn zone_name(k: cfe::dop::ZoneKind) -> &'static str {
    match k {
        cfe::dop::ZoneKind::Extractor => "extractor",
        cfe::dop::ZoneKind::Habitat => "habitat",
        cfe::dop::ZoneKind::Smelter => "smelter",
        cfe::dop::ZoneKind::Synthesizer => "synthesizer",
    }
}

fn scope_name(s: cfe::ipsal::Scope) -> &'static str {
    match s {
        cfe::ipsal::Scope::Build => "build",
        cfe::ipsal::Scope::Raze => "raze",
        cfe::ipsal::Scope::Collect => "collect",
    }
}

fn render_state(world_name: &str, planet: &str, civilization: &str, world: &World) -> String {
    let mut w = JsonWriter::new();
    w.open();
    w.field_str("civilization", civilization);
    w.field_str("world", world_name);
    w.field_str("planet", planet);
    w.field_u64("clock", world.now());
    w.field_u64("receipts", world.chronicle().receipts().len() as u64);

    if let Some(orb) = world.orb() {
        w.field_str("colony", &orb.name);
        w.key("banked");
        w.open();
        for (resource, amount) in &orb.banked {
            w.field_str(resource_name(*resource), &ratio_str(amount));
        }
        w.close();
        w.key("famine");
        w.array(orb.famine.iter().map(|r| resource_name(*r).to_string()));
        w.key("zones");
        w.open_array();
        for zone in &orb.zones {
            w.open();
            w.field_u64("tile", zone.tile as u64);
            w.field_str("kind", zone_name(zone.kind));
            w.close();
        }
        w.close_array();
    }

    w.key("accrued");
    w.open();
    for (resource, amount) in world.accrued() {
        w.field_str(resource_name(*resource), &ratio_str(amount));
    }
    w.close();

    w.key("entities");
    w.open_array();
    for entity in world.entities() {
        w.open();
        w.field_u64("id", entity.entity_id);
        w.field_str("handle", &entity.handle);
        match entity.principal {
            None => w.field_raw("principal", "null"),
            Some(p) => w.field_u64("principal", p),
        }
        w.key("grants");
        w.array(entity.grants.iter().map(|s| scope_name(*s).to_string()));
        w.close();
    }
    w.close_array();
    w.close();
    w.finish()
}

fn render_receipt(receipt: &Receipt) -> String {
    use cfe::dop::ClosureEvent as E;
    let mut w = JsonWriter::new();
    w.open();
    w.field_u64("seq", receipt.sequence);
    w.field_u64("stamp", receipt.stamp);
    w.field_str("parent", &hex(&receipt.parent.0));
    w.field_str("digest", &hex(&receipt.own.0));
    w.key("by");
    w.array(receipt.provenance.iter().map(|p| p.to_string()));
    let (name, detail): (&str, String) = match &receipt.event {
        E::ColonyFounded { name, .. } => ("colony_founded", name.clone()),
        E::ZoneBuilt { tile, kind, .. } => ("zone_built", format!("{} @ {tile}", zone_name(*kind))),
        E::YieldCollected { resource, amount, .. } => (
            "yield_collected",
            format!("{} {}", ratio_str(amount), resource_name(*resource)),
        ),
        E::UpkeepSettled { resource, amount, .. } => (
            "upkeep_settled",
            format!("{} {}", ratio_str(amount), resource_name(*resource)),
        ),
        E::FamineStruck { resource, .. } => ("famine_struck", resource_name(*resource).into()),
        E::FamineRelieved { resource, .. } => ("famine_relieved", resource_name(*resource).into()),
        E::ZoneRazed { tile, .. } => ("zone_razed", format!("tile {tile}")),
        E::EntityRegistered { handle, .. } => ("entity_registered", handle.clone()),
        E::ScopeGranted { agent, scope } => {
            ("scope_granted", format!("{} to #{agent}", scope_name(*scope)))
        }
        E::ScopeRevoked { agent, scope } => {
            ("scope_revoked", format!("{} from #{agent}", scope_name(*scope)))
        }
    };
    w.field_str("event", name);
    w.field_str("detail", &detail);
    w.field_str("tlv", &hex(&receipt.event.wire_bytes()));
    w.close();
    w.finish()
}

/// Minimal directive parsing (flat JSON: `verb`, `args`, `as`). The
/// game loop re-validates EVERYTHING — this only reads the request.
fn parse_directive(text: &str) -> Option<Directive> {
    let verb = json::extract_string(text, "verb")?;
    let acting_as = json::extract_string(text, "as").unwrap_or_else(|| "steward".into());
    let args = json::extract_string_array(text, "args").unwrap_or_default();
    Some(Directive {
        verb,
        args,
        acting_as,
    })
}
