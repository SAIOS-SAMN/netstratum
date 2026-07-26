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

    /// THE UNIVERSE MANIFOLD (contract §9): the base is a compact
    /// 3-torus — spatial coordinates S = (x, y, z) live in ℚ mod
    /// `span` per axis, so the universe has no edge and every
    /// direction comes back around. A world's full coordinate is
    /// TUniversal{S, G, T}: S the spatial triple, G the gauge (fiber
    /// phase), T the universal tick clock. The polynomial FIBER at
    /// every point is the physics carrier's graded dimensions
    /// (`physics::AxisVector`), and a Mint GROWS them — the universe
    /// gains dimensions as discoveries land. ("Calabi-Yau" is the
    /// lore word; the claim made here is compact toroidal base +
    /// polynomial fiber, nothing more.)
    ///
    /// THE L1 LAW (carried from the convergence doctrine): distance
    /// is L1 — the wrapped |Δ| summed per axis, the short way around.
    /// NO SQUARING: we do not square circles in a toroid; L2 would
    /// collapse the geometry the residual carries.
    ///
    /// The span (default 64) reads from `<root>/universe.json`,
    /// written on first chart.
    pub fn universe_span(&self) -> Ratio {
        fs::read_to_string(self.root.join("universe.json"))
            .ok()
            .and_then(|text| json::extract_string(&text, "span"))
            .and_then(|s| parse_ratio(&s))
            .unwrap_or_else(|| Ratio::from(num_bigint::BigInt::from(64)))
    }

    /// THE UNIVERSAL CLOCK — T of TUniversal{S, G, T}: a six-field
    /// base-60 odometer over the era's tick count. It advances PER
    /// TICK, never per wall-second (ambient time would kill SEAL B);
    /// at a 2-second cadence it happens to move every 2 seconds, but
    /// replay re-derives it exactly.
    pub fn set_universal_clock(&self, ticks: u64) -> std::io::Result<()> {
        fs::create_dir_all(&self.root)?;
        self.write_universe_card(ticks)
    }

    fn write_universe_card(&self, ticks: u64) -> std::io::Result<()> {
        let mut w = JsonWriter::new();
        w.open();
        w.field_str("topology", "torus");
        w.field_str("span", "64");
        w.field_str("tuniversal", &universal_clock(ticks));
        w.field_u64("ticks", ticks);
        w.field_str(
            "fiber",
            "polynomial (physics::AxisVector grades; minted dimensions grow it)",
        );
        w.close();
        let tmp = self.root.join("universe.json.tmp");
        fs::write(&tmp, w.finish())?;
        fs::rename(&tmp, self.root.join("universe.json"))
    }

    /// Chart a world onto the star-chart: exact-ℚ coordinates in
    /// `<root>/starchart.json` (merge-on-write; charting the same
    /// world again moves it — the chart is a card, the chronicle is
    /// the truth). Spatial coordinates WRAP onto the 3-torus: mod
    /// span, exactly; `g` is the gauge phase (fiber coordinate).
    pub fn chart_world(
        &self,
        world: &str,
        s: (&Ratio, &Ratio, &Ratio),
        g: &Ratio,
    ) -> std::io::Result<()> {
        fs::create_dir_all(&self.root)?;
        // The universe card, written once (the clock rewrites it).
        if !self.root.join("universe.json").exists() {
            self.write_universe_card(0)?;
        }
        let span = self.universe_span();
        let wrap = |v: &Ratio| -> Ratio { v - (v / &span).floor() * &span };
        let (x, y, z) = (wrap(s.0), wrap(s.1), wrap(s.2));
        let path = self.root.join("starchart.json");
        let mut entries: Vec<(String, [String; 4])> = Vec::new();
        if let Ok(text) = fs::read_to_string(&path) {
            for line in text.lines() {
                if let (Some(name), Some(px), Some(py)) = (
                    json::extract_string(line, "world"),
                    json::extract_string(line, "x"),
                    json::extract_string(line, "y"),
                ) {
                    let pz = json::extract_string(line, "z").unwrap_or_else(|| "0".into());
                    let pg = json::extract_string(line, "g").unwrap_or_else(|| "0".into());
                    entries.push((name, [px, py, pz, pg]));
                }
            }
        }
        let slug = sanitize(world);
        entries.retain(|(name, _)| name != &slug);
        entries.push((
            slug,
            [ratio_str(&x), ratio_str(&y), ratio_str(&z), ratio_str(g)],
        ));
        entries.sort();
        let mut out = String::new();
        for (name, [px, py, pz, pg]) in &entries {
            let mut w = JsonWriter::new();
            w.open();
            w.field_str("world", name);
            w.field_str("x", px);
            w.field_str("y", py);
            w.field_str("z", pz);
            w.field_str("g", pg);
            w.close();
            out.push_str(&w.finish());
            out.push('\n');
        }
        let tmp = self.root.join("starchart.json.tmp");
        fs::write(&tmp, out)?;
        fs::rename(&tmp, path)
    }

    /// Read one world's charted coordinate: (S = (x, y, z), G).
    pub fn charted_position(&self, world: &str) -> Option<((Ratio, Ratio, Ratio), Ratio)> {
        let text = fs::read_to_string(self.root.join("starchart.json")).ok()?;
        let slug = sanitize(world);
        for line in text.lines() {
            if json::extract_string(line, "world").as_deref() == Some(&slug) {
                return Some((
                    (
                        parse_ratio(&json::extract_string(line, "x")?)?,
                        parse_ratio(&json::extract_string(line, "y")?)?,
                        parse_ratio(&json::extract_string(line, "z").unwrap_or_else(|| "0".into()))?,
                    ),
                    parse_ratio(&json::extract_string(line, "g").unwrap_or_else(|| "0".into()))?,
                ));
            }
        }
        None
    }

    /// The L1 toroidal distance between two charted worlds — wrapped
    /// |Δ| summed over the spatial axes, exact. NO SQUARING (the L1
    /// law): the short way around, per axis, added whole.
    pub fn l1_distance(&self, a: &str, b: &str) -> Option<Ratio> {
        let ((ax, ay, az), _) = self.charted_position(a)?;
        let ((bx, by, bz), _) = self.charted_position(b)?;
        let span = self.universe_span();
        let leg = |u: &Ratio, v: &Ratio| -> Ratio { wrapped_abs(u, v, &span) };
        Some(leg(&ax, &bx) + leg(&ay, &by) + leg(&az, &bz))
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

/// The six-field base-60 odometer: T of TUniversal, a pure function
/// of the tick count — never of any wall clock.
pub fn universal_clock(ticks: u64) -> String {
    let mut fields = [0u64; 6];
    let mut rest = ticks;
    for slot in (0..6).rev() {
        fields[slot] = rest % 60;
        rest /= 60;
    }
    fields
        .iter()
        .map(|f| format!("{f:02}"))
        .collect::<Vec<_>>()
        .join(":")
}

/// The wrapped per-axis |Δ| — the short way around the torus.
fn wrapped_abs(a: &Ratio, b: &Ratio, span: &Ratio) -> Ratio {
    use num_traits::Signed;
    let mut d = (a - b).abs();
    // Both coordinates live in [0, span), so d < span already; take
    // the shorter arc.
    let around = span - &d;
    if around < d {
        d = around;
    }
    d
}

/// Parse the mirror's exact rendering back: `p/q` or a plain integer.
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
    for (resource, amount) in &world.accrued() {
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
        E::ColonyCharted { orb_id, source_orb, name } => (
            "colony_charted",
            format!("{name} (orb {orb_id}) from orb {source_orb}"),
        ),
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
