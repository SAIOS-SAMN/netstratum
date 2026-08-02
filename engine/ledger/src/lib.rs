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
//! **The display half MOVED (the #37 inversion): the writers —
//! record, record_solve, the journals, the wall — live in `scribe`,
//! ABOVE the lens. This crate is ledger-LAW in fact: it imports no
//! lens, renders no display, and the shared JSON pen (`pen::JsonWriter`)
//! writes only the Tier-A law cards (universe, starchart, registry,
//! galaxy, fiber).**
//!
//! **THE UPLINK LAW (mesh authenticity):** nothing in this crate ever
//! rides the mesh. Only chain-verified bytes ascend — `chronicle.tlv`
//! today, arbor commitments at #15. Exports are for eyes; ledger
//! files are debts awaiting adoption; the mesh carries neither.
//!
//! Layout under `var/`: `registry/<civ>.json` (identity),
//! `worlds/<world>/world.json` (the card), `chronicle.jsonl` (display
//! stream), `planets/<planet>/state.json` + `state.tlv` (snapshots),
//! `inbox/` (the steward door), `solves/` (proof-of-solution
//! certificates). The chronicle in memory is the world; nothing ever
//! reads a snapshot back into a world — replay grounds worlds.

use cfe::ratio::Ratio;
use std::fs;
use std::path::PathBuf;

pub mod converge;
pub mod galaxy;
pub mod extract;
pub mod registry;
use pen::JsonWriter;
pub(crate) use cfe::ratio::ratio_str;

/// THE UNIVERSAL CLOCK RENDERING — T of TUniversal{S, G, T} as the
/// six-field base-60 odometer string the law cards carry
/// (universe.json's `tuniversal`). The law writes its own cards with
/// its own pen (the inversion dropped the lens dependency); the
/// lens's display copy renders the same law for the eye.
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

    /// The ledger's root directory — pub for the SCRIBE (the display
    /// writers above the lens), which pens its wall and journals under
    /// the same roots the law keeps. Read-only access; the law's own
    /// writes stay in this crate.
    pub fn root(&self) -> &std::path::Path {
        &self.root
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
            .and_then(|text| extract::extract_string(&text, "span"))
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
                    extract::extract_string(line, "world"),
                    extract::extract_string(line, "x"),
                    extract::extract_string(line, "y"),
                ) {
                    let pz = extract::extract_string(line, "z").unwrap_or_else(|| "0".into());
                    let pg = extract::extract_string(line, "g").unwrap_or_else(|| "0".into());
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
            if extract::extract_string(line, "world").as_deref() == Some(&slug) {
                return Some((
                    (
                        parse_ratio(&extract::extract_string(line, "x")?)?,
                        parse_ratio(&extract::extract_string(line, "y")?)?,
                        parse_ratio(&extract::extract_string(line, "z").unwrap_or_else(|| "0".into()))?,
                    ),
                    parse_ratio(&extract::extract_string(line, "g").unwrap_or_else(|| "0".into()))?,
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

}

/// Filesystem-safe names (worlds, planets, civilizations, tasks):
/// lowercase alphanumerics and dashes, everything else becomes a dash.
/// Pub: the scribe pens its displays under the same roots the law
/// keeps — one naming rule, one place.
pub fn sanitize(name: &str) -> String {
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
    let verb = extract::extract_string(text, "verb")?;
    let acting_as = extract::extract_string(text, "as").unwrap_or_else(|| "steward".into());
    let args = extract::extract_string_array(text, "args").unwrap_or_default();
    Some(Directive {
        verb,
        args,
        acting_as,
    })
}
