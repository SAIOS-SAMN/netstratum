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
use cfe::world::World;
use std::fs;
use std::io::Write as _;
use std::path::PathBuf;

pub mod converge;
pub mod galaxy;
pub mod journal;
mod json;
pub mod registry;
use json::JsonWriter;
use lens::render::{hex, render_receipt, render_state};
pub(crate) use lens::json::ratio_str;
/// Re-exported at the #37 split (moved to `lens`); existing callers
/// (conductor's clock renderings) keep their path.
pub use lens::render::universal_clock;

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

        // state.tlv (landed as "MIRROR-TLV", 2026-07-30): the COMMITTED
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

        // chronicle.jsonl — append receipts beyond what is exported.
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

    /// The divergence check behind [`Ledger::record`]: compare the
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

/// Parse the ledger's exact rendering back: `p/q` or a plain integer.
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
