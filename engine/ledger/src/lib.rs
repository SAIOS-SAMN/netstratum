//! The pre-chain LEDGER of record and its lens (Charter §11 — the
//! crate formerly named "mirror"; the name died 2026-07-31 by
//! operator order, and with it the pretense that this was ever one
//! thing).
//!
//! WHAT LIVES HERE, honestly inventoried:
//!
//! **The law (pre-chain interim — every file-borne path a Tier-A
//! debt, adopted in-chain at ladder #36; no NEW read-back path may
//! ever be added — the §11 standing test):**
//! - the steward seam ([`Directive`]/[`PendingDirective`]) — the ONE
//!   lawful inbound door: outside writers touch only `inbox/`, and a
//!   request becomes real solely through the game loop as a
//!   receipted, scope-checked NS-6 action;
//! - [`converge`] — the fold: era annexation, quarantines;
//! - [`registry`] — identity: civilizations, stations, sigils;
//! - [`galaxy`] — the sky: the observation deed, lazy founding.
//!
//! **The display (write-only exports, §11 rule 2 — never read back,
//! deletable at any moment, re-served from the chain at #37):**
//! - [`Ledger::record`] — the state snapshot + receipt-stream export;
//! - [`journal`] — the Chronicle Feed renderings.
//!
//! **THE UPLINK LAW (mesh authenticity):** nothing in this crate ever
//! rides the mesh. Only chain-verified bytes ascend — `chronicle.tlv`
//! today, spine commitments at #15. Exports are for eyes; ledger
//! files are debts awaiting adoption; the mesh carries neither.
//!
//! Layout under `var/`: `registry/<civ>.json` (identity),
//! `worlds/<world>/world.json` (the card), `chronicle.jsonl` (display
//! stream), `planets/<planet>/state.json` + `state.tlv` (snapshots),
//! `inbox/` (the steward door), `solves/` (proof-of-solution
//! certificates). The chronicle in memory is the world; nothing ever
//! reads a snapshot back into a world — replay grounds worlds.

use cfe::ratio::Ratio;
use cfe::vrp::Receipt;
use cfe::world::World;
use num_traits::One;
use std::fs;
use std::io::Write as _;
use std::path::PathBuf;

pub mod converge;
pub mod galaxy;
pub mod journal;
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

pub struct Ledger {
    root: PathBuf,
}

impl Ledger {
    /// The ledger rooted at `<root>/` (conventionally `var/worlds`).
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
        // ONE CHRONICLE PER WORLD PER ERA (the persist.rs divergence
        // law, ledger-side): founding names recur across eras, and an
        // incremental append onto a PRIOR era's file would interleave
        // two timelines in one chain. A divergent resident chronicle is
        // rotated aside as a closed sibling world — never clobbered —
        // and convergence later annexes it as its own timeline.
        self.rotate_divergent(&dir, world)?;
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

        // MIRROR-TLV (operator directive, 2026-07-30): the COMMITTED
        // state speaks the house wire — state.tlv is the orb's
        // canonical NS-1 bytes (equal state, equal bytes), the
        // machine layer every board may decode natively. JSON stays
        // strictly a rendering: readings, context, and voices — never
        // a second truth of the committed orb.
        if let Some(orb) = world.orb() {
            let tmp = planet_dir.join("state.tlv.tmp");
            fs::write(&tmp, orb.wire_bytes())?;
            fs::rename(&tmp, planet_dir.join("state.tlv"))?;
        }

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

    /// The divergence check behind [`Mirror::record`]: compare the
    /// resident chronicle's line `k` digest against the live world's
    /// receipt `k` (k = last shared index). Equal ⇒ same timeline
    /// (incremental append proceeds). Different ⇒ a PRIOR era's world
    /// lives here: the whole world dir rotates to
    /// `<name>--closed-<digest8>` (its terminal digest names it).
    fn rotate_divergent(&self, dir: &std::path::Path, world: &World) -> std::io::Result<()> {
        let path = dir.join("chronicle.jsonl");
        let receipts = world.chronicle().receipts();
        if !path.exists() || receipts.is_empty() {
            return Ok(());
        }
        let text = fs::read_to_string(&path)?;
        let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
        if lines.is_empty() {
            return Ok(());
        }
        let check = lines.len().min(receipts.len()) - 1;
        let resident = lines
            .get(check)
            .and_then(|line| json::extract_string(line, "digest"))
            .unwrap_or_default();
        let live = hex(&receipts[check].own.0);
        if resident == live {
            return Ok(());
        }
        let terminal = lines
            .last()
            .and_then(|line| json::extract_string(line, "digest"))
            .unwrap_or_else(|| "unknown".into());
        let tag: String = terminal.chars().take(8).collect();
        let base = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let mut closed = dir.with_file_name(format!("{base}--closed-{tag}"));
        let mut bump = 1u32;
        while closed.exists() {
            bump += 1;
            closed = dir.with_file_name(format!("{base}--closed-{tag}-{bump}"));
        }
        fs::rename(dir, closed)
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
    /// 3-torus — spatial coordinates S = (x, y, z) live in the exact rationals mod
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

    /// Chart a world onto the star-chart: exact-rational coordinates in
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
pub(crate) fn ratio_str(r: &Ratio) -> String {
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
        cfe::dop::Resource::Joule => "joule",
        cfe::dop::Resource::Sink => "sink",
        cfe::dop::Resource::Crystal => "crystal",
    }
}

fn zone_name(k: cfe::dop::ZoneKind) -> &'static str {
    match k {
        cfe::dop::ZoneKind::Extractor => "extractor",
        cfe::dop::ZoneKind::Habitat => "habitat",
        cfe::dop::ZoneKind::Smelter => "smelter",
        cfe::dop::ZoneKind::Synthesizer => "synthesizer",
        cfe::dop::ZoneKind::Encasement => "encasement",
        cfe::dop::ZoneKind::CryoArray => "cryo-array",
        cfe::dop::ZoneKind::Resonator => "resonator",
    }
}

/// The base-7 stack's concept words (10 §2: the condition fixes the
/// CONCEPT; the node hash draws the voice — the uplink carries the
/// concept, the chronicle carries the voice).
fn condition_word(condition: u8) -> &'static str {
    match condition {
        0 => "solar",
        1 => "insulation",
        2 => "discipline",
        3 => "deepening",
        4 => "doctrine",
        5 => "stores",
        6 => "terraforming",
        _ => "working",
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

    if let Some(class) = world.star() {
        w.field_u64("star_class", class as u8 as u64);
    }
    // MARKET-1 / observer uplink (BOARD-1): the resting offers, data
    // fields the boards render — never prose to parse.
    if !world.offers().is_empty() {
        w.key("offers");
        w.raw_array(world.offers().iter().map(|offer| {
            let mut entry = JsonWriter::new();
            entry.open();
            entry.field_u64("offer", offer.offer);
            entry.field_str("give", resource_name(offer.give));
            entry.field_str("give_amount", &ratio_str(&offer.give_amount));
            entry.field_str("want", resource_name(offer.want));
            entry.field_str("want_amount", &ratio_str(&offer.want_amount));
            entry.close();
            entry.finish()
        }));
    }
    // CONSTEL-1 / observer uplink: the base-7 sky — a derived
    // reading (held/progress/threshold per condition), never stored.
    if let Some(orb) = world.orb() {
        w.key("constellation");
        w.raw_array(world.constellation(orb.orb_id).into_iter().map(
            |(condition, held, progress, threshold)| {
                let mut entry = JsonWriter::new();
                entry.open();
                entry.field_u64("condition", u64::from(condition));
                entry.field_u64("held", u64::from(held));
                entry.field_u64("progress", progress);
                entry.field_u64("threshold", threshold);
                entry.close();
                entry.finish()
            },
        ));
    }
    // MANU-1 / observer uplink: the forged instruments — the reach
    // law reads the best grade (reach = base + grade, 10 §3f).
    if !world.instruments().is_empty() {
        w.key("instruments");
        w.raw_array(world.instruments().iter().map(|(orb, node, grade)| {
            let mut entry = JsonWriter::new();
            entry.open();
            entry.field_u64("orb", *orb);
            entry.field_u64("grade", u64::from(*grade));
            entry.field_str(
                "node",
                &format!("{:02x}{:02x}{:02x}{:02x}", node[0], node[1], node[2], node[3]),
            );
            entry.close();
            entry.finish()
        }));
    }
    // WANT-1 / observer uplink: the struck cognitive wants.
    if !world.wants().is_empty() {
        w.key("wants");
        w.array(world.wants().iter().map(|(_, kind)| want_name(*kind).to_string()));
    }
    if let Some(orb) = world.orb() {
        w.field_str("colony", &orb.name);
        // VENT-1 / observer uplink: the dual ledger and the derived
        // (never stored) syntropy ratio.
        w.field_str("entropy_vented", &ratio_str(&orb.entropy_vented));
        let banked_j = orb.banked_amount(cfe::dop::Resource::Joule);
        let denominator = banked_j.clone() + orb.entropy_vented.clone();
        if denominator > Ratio::from(num_bigint::BigInt::from(0)) {
            w.field_str("syntropy_ratio", &format!(
                "{}/{}",
                ratio_str(&banked_j),
                ratio_str(&denominator)
            ));
        }
        // IDEO-1 / observer uplink: the eight committed cohesion books
        // and the four derived (never stored) axis ratios — emitted
        // only when a book is nonzero, like the wire entry.
        let axes: [(&str, &str, &Ratio, &str, &Ratio); 4] = [
            ("compassion", "given_in_crisis", &orb.given_in_crisis,
             "drawn_in_crisis", &orb.drawn_in_crisis),
            ("love", "bond_flow", &orb.bond_flow,
             "severance_load", &orb.severance_load),
            ("integrity", "covenants_fulfilled", &orb.covenants_fulfilled,
             "covenants_breached", &orb.covenants_breached),
            ("morality", "returned_across", &orb.returned_across,
             "extracted_across", &orb.extracted_across),
        ];
        let zero = Ratio::from(num_bigint::BigInt::from(0));
        if axes.iter().any(|(_, _, syn, _, ent)| **syn != zero || **ent != zero) {
            w.key("cohesion");
            w.open();
            for (axis, syn_name, syn, ent_name, ent) in axes {
                w.field_str(syn_name, &ratio_str(syn));
                w.field_str(ent_name, &ratio_str(ent));
                if let Some(reading) = cfe::dop::cohesion_ratio(syn, ent) {
                    w.field_str(&format!("{axis}_ratio"), &ratio_str(&reading));
                }
            }
            w.close();
        }
        // CONSTEL-1 / observer uplink (the deferred export, paid):
        // the base-7 stack per orb — condition · held · progress ·
        // threshold, the crystallizer's OWN deposit-sheet reading
        // (never a second bookkeeping). Counters up, ratios derived
        // surface-side; emitted only when something has begun.
        let constellation = world.constellation(orb.orb_id);
        if constellation.iter().any(|(_, held, progress, _)| *held || *progress > 0) {
            w.key("constellation");
            w.open_array();
            for (condition, held, progress, threshold) in constellation {
                w.open();
                w.field_str("condition", condition_word(condition));
                w.field_u64("held", u64::from(held));
                w.field_u64("progress", progress);
                w.field_u64("threshold", threshold);
                w.close();
            }
            w.close_array();
        }
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

/// WANT-1: the kind table's names (tag 62).
fn want_name(kind: u8) -> &'static str {
    match kind {
        1 => "epistemological",
        2 => "resonance",
        3 => "novelty",
        _ => "unnamed",
    }
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
        E::EraAnnexed { era, world, offset, .. } => (
            "era_annexed",
            format!("{world} of {era} at offset {offset}"),
        ),
        E::GalaxyObserved { observer, name, .. } => (
            "galaxy_observed",
            format!("{name} observed by {observer}"),
        ),
        E::IncursionStruck { era, world, line } => (
            "incursion_struck",
            format!("{world} of {era} wakes as {line}"),
        ),
        E::ElderWarded {} => ("elder_warded", "the reverse shield seats".into()),
        E::WardLifted { claimer } => (
            "ward_lifted",
            format!("the depth gate opened by {claimer}"),
        ),
        E::VeilDrawn {} => ("veil_drawn", "the renders cloud; the chronicle never lies".into()),
        E::VeilLifted { starved } => (
            "veil_lifted",
            if *starved { "STARVED — the toll went unpaid".into() } else { "by deed".into() },
        ),
        E::PracticeAdopted { practice } => (
            "practice_adopted",
            format!("discipline {practice} bound by receipt"),
        ),
        E::PracticeReleased { practice } => (
            "practice_released",
            format!("discipline {practice} released — the lesson re-emerges"),
        ),
        E::InstrumentForged { orb_id, grade, .. } => (
            "instrument_forged",
            format!("orb {orb_id} forges its grade-{grade} instrument — the horizon grows"),
        ),
        E::VoidShare { deal, amount, .. } => (
            "void_share",
            format!("deal {deal}: the void's exact share — {} melted in transit", ratio_str(amount)),
        ),
        E::RouteChartered { deal, eta } => (
            "route_chartered",
            format!("lane {deal} chartered at η {eta} — the path is paid for"),
        ),
        E::RouteLapsed { deal } => (
            "route_lapsed",
            format!("lane {deal} LAPSES — raw space takes the path back"),
        ),
        E::WorldCharted { seed } => (
            "world_charted",
            format!(
                "terrain seed {:02x}{:02x}{:02x}{:02x}",
                seed[0], seed[1], seed[2], seed[3]
            ),
        ),
        E::OverheatStruck { overflow, .. } => {
            ("overheat_struck", format!("overflow {}", ratio_str(overflow)))
        }
        E::OverheatCleared { .. } => ("overheat_cleared", String::new()),
        E::StarCommissioned { class } => ("star_commissioned", format!("class {class}")),
        E::PlanetSited { orb_id, albedo, orbit_r } => (
            "planet_sited",
            format!("orb {orb_id} A={} r={}", ratio_str(albedo), ratio_str(orbit_r)),
        ),
        E::VentSettled { amount, .. } => ("vent_settled", ratio_str(amount)),
        E::DemurrageSettled { resource, amount, .. } => (
            "demurrage_settled",
            format!("{} {}", ratio_str(amount), resource_name(*resource)),
        ),
        E::ResourceTransferred { from_orb, to_orb, resource, amount } => (
            "resource_transferred",
            format!(
                "{} {} orb {from_orb} → orb {to_orb}",
                ratio_str(amount),
                resource_name(*resource)
            ),
        ),
        E::TechDiscovered { orb_id, condition, name, .. } => (
            "tech_discovered",
            format!("{name} — orb {orb_id} condition {condition}"),
        ),
        E::TechLicensed { deal, from_orb, council, condition, name, .. } => (
            "tech_licensed",
            format!(
                "{name} licensed out — orb {from_orb}, deal {deal}, council {council}, condition {condition}"
            ),
        ),
        E::CouncilSealed { council, resource, quota, window, collateral, members } => (
            "council_sealed",
            format!(
                "council {council}: {} members bound alike — {} {} quota, {} collateral, {window} ticks",
                members.len(),
                ratio_str(quota),
                resource_name(*resource),
                ratio_str(collateral)
            ),
        ),
        E::CouncilClosed { council, member, fulfilled, banked } => (
            "council_closed",
            format!(
                "council {council}: orb {member} {} — {} banked home",
                match fulfilled {
                    true => "kept faith",
                    false => "breached",
                },
                ratio_str(banked)
            ),
        ),
        E::WantStruck { orb_id, kind, reading } => (
            "want_struck",
            format!(
                "the {} want strikes orb {orb_id} — {} ticks starved",
                want_name(*kind),
                ratio_str(reading)
            ),
        ),
        E::WantRelieved { orb_id, kind } => (
            "want_relieved",
            format!("the {} want relieved — orb {orb_id}'s mind is fed", want_name(*kind)),
        ),
        E::OfferPosted { offer, give, give_amount, want, want_amount, .. } => (
            "offer_posted",
            format!(
                "offer {offer} rests: {} {} for {} {}",
                ratio_str(give_amount),
                resource_name(*give),
                ratio_str(want_amount),
                resource_name(*want)
            ),
        ),
        E::OfferCleared { offer, deal } => (
            "offer_cleared",
            format!("offer {offer} cleared — the crossing filled it (deal {deal})"),
        ),
        E::GaugeShiftedAt { grade, gauge } => (
            "gauge_shifted_at",
            format!("the key moves at grade {grade} — {}", ratio_str(gauge)),
        ),
        E::GaugeShifted { gauge } => {
            ("gauge_shifted", format!("the key moves to {}", ratio_str(gauge)))
        }
        E::Diverged { branch, gauge } => (
            "diverged",
            format!("cleaved — branch {branch} departs; the key moves to {}", ratio_str(gauge)),
        ),
        E::Confluent { partner, gauge } => (
            "confluent",
            format!("combined with {partner}; the key settles at {}", ratio_str(gauge)),
        ),
        E::StarAged { class } => ("star_aged", format!("the star moves — class {class}")),
        E::EraFolded { era, gauge } => (
            "era_folded",
            format!("aeon {era} folds; the new sheet seats at {}", ratio_str(gauge)),
        ),
        E::ApertureSent { deal, from_orb, resource, amount } => (
            "aperture_sent",
            format!(
                "{} {} orb {from_orb} → deal {deal}",
                ratio_str(amount),
                resource_name(*resource)
            ),
        ),
        E::ApertureReceived { deal, to_orb, resource, amount } => (
            "aperture_received",
            format!(
                "{} {} deal {deal} → orb {to_orb}",
                ratio_str(amount),
                resource_name(*resource)
            ),
        ),
        E::OverflowAbsorbed { source_orb, absorber_orb, amount } => (
            "overflow_absorbed",
            format!(
                "{} heat orb {source_orb} → orb {absorber_orb}'s sink",
                ratio_str(amount)
            ),
        ),
        E::AbsorptionLapsed { debtor_orb, creditor_orb, amount } => (
            "absorption_lapsed",
            format!(
                "{} unpaid orb {debtor_orb} → orb {creditor_orb}",
                ratio_str(amount)
            ),
        ),
        E::BondSealed { source_orb, target_orb, resource, rate, term } => (
            "bond_sealed",
            format!(
                "{} {}/tick orb {source_orb} → orb {target_orb} (term {term})",
                ratio_str(rate),
                resource_name(*resource)
            ),
        ),
        E::BondSettled { source_orb, target_orb, resource, amount, severed, load } => (
            "bond_settled",
            format!(
                "{} {} orb {source_orb} → orb {target_orb}{}",
                ratio_str(amount),
                resource_name(*resource),
                match severed {
                    true => format!(" SEVERED (load {})", ratio_str(load)),
                    false => String::new(),
                }
            ),
        ),
        E::CovenantSealed { obligor_orb, beneficiary_orb, resource, quota, window, collateral } => (
            "covenant_sealed",
            format!(
                "{} {} orb {obligor_orb} → orb {beneficiary_orb} in {window} (collateral {})",
                ratio_str(quota),
                resource_name(*resource),
                ratio_str(collateral)
            ),
        ),
        E::CovenantReleased { obligor_orb, beneficiary_orb, resource, collateral } => (
            "covenant_released",
            format!(
                "{} {} home to orb {obligor_orb} (pact with orb {beneficiary_orb})",
                ratio_str(collateral),
                resource_name(*resource)
            ),
        ),
        E::CovenantSlashed { obligor_orb, beneficiary_orb, resource, collateral } => (
            "covenant_slashed",
            format!(
                "{} {} forfeit orb {obligor_orb} → orb {beneficiary_orb}",
                ratio_str(collateral),
                resource_name(*resource)
            ),
        ),
        E::FrictionSettled { orb_id, amount } => (
            "friction_settled",
            format!("{} entropy on orb {orb_id}", ratio_str(amount)),
        ),
        E::CrystalDrawn { orb_id, amount } => (
            "crystal_drawn",
            format!("{} crystal held the lens open on orb {orb_id}", ratio_str(amount)),
        ),
        E::LineSucceeded { successor } => (
            "line_succeeded",
            format!("{successor} adopts this chronicle as its own substrate"),
        ),
        E::GenesisSealed { civilization, disposition, metabolism, perception } => (
            "genesis_sealed",
            format!(
                "{civilization} seals its axes ({disposition}/{metabolism}/{perception}) — the registry is a cache from here on"
            ),
        ),
        E::HandoffSealed { from, to } => (
            "handoff_sealed",
            format!("hand {from} yields the pen to hand {to}"),
        ),
        E::ProspectStruck { orb_id, stratum, name, family, tier, .. } => (
            "prospect_struck",
            format!(
                "{name} read from stratum {stratum} on orb {orb_id} (family {family}, tier {tier}) — what the seed always held"
            ),
        ),
        E::AssayStruck { orb_id, amount, .. } => (
            "assay_struck",
            format!("{} drawn from the opened vein on orb {orb_id}", ratio_str(amount)),
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
