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
//! lens. Law cards use binary TLV ([`card`]), never JSON.**
//!
//! **THE UPLINK LAW (mesh authenticity):** nothing in this crate ever
//! rides the mesh. Only chain-verified bytes ascend — `chronicle.tlv`
//! today, arbor commitments at #15. Exports are for eyes; ledger
//! files are debts awaiting adoption; the mesh carries neither.
//!
//! Layout under `var/`: `registry/<civ>.civ` (identity TLV),
//! `worlds/universe.card` · `starchart.card` · `galaxies.card`,
//! `worlds/<world>/world.card`, `chronicle.tlv` (authority),
//! `planets/<planet>/state.tlv` (machine mirror), `inbox/*.dir`
//! (steward directives TLV), `solves/*.sol` (certificates TLV).
//! See `docs/JSON_DISPLAY_ONLY.md`.

use cfe::ratio::Ratio;
use std::fs;
use std::path::PathBuf;

pub mod card;
pub mod converge;
pub mod galaxy;
pub mod registry;
pub(crate) use cfe::ratio::ratio_str;

/// THE UNIVERSAL CLOCK RENDERING — T of TUniversal{S, G, T} as the
/// six-field base-60 odometer string the law cards carry
/// (`universe.card` tuniversal). The law writes its own cards with
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
    /// The universe ledger read once per change (it is append-only, so its
    /// length is its version): the galaxies and the cores' libraries from one
    /// parse, and the span, which is fixed at the Big Bang.
    cache: std::sync::Mutex<galaxy::UniverseRead>,
    span: std::sync::OnceLock<Ratio>,
}

impl Ledger {
    /// The ledger rooted at `<root>/` (conventionally `var/worlds`).
    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into(), cache: Default::default(), span: std::sync::OnceLock::new() }
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
    /// Carriers: `*.dir` TLV only.
    pub fn pending_directives(&self, world: &str) -> std::io::Result<Vec<PendingDirective>> {
        let inbox = self.world_dir(world).join("inbox");
        if !inbox.exists() {
            return Ok(Vec::new());
        }
        let mut paths: Vec<PathBuf> = fs::read_dir(&inbox)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "dir"))
            .collect();
        paths.sort();
        let mut out = Vec::new();
        for path in paths {
            if let Some(directive) = parse_directive_tlv(&fs::read(&path)?) {
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
    /// The span (default 64) reads from `<root>/universe.card` (TLV).
    pub fn universe_span(&self) -> Ratio {
        if let Some(span) = self.span.get() {
            return span.clone();
        }
        match self.read_universe_span() {
            // Fixed once written: held from the first reading on.
            Some(span) => self.span.get_or_init(|| span).clone(),
            None => Ratio::from(num_bigint::BigInt::from(64)),
        }
    }

    fn universe_card_path(&self) -> PathBuf {
        self.root.join("universe.card")
    }

    fn starchart_path(&self) -> PathBuf {
        self.root.join("starchart.card")
    }

    fn read_universe_span(&self) -> Option<Ratio> {
        let data = card::read(&self.universe_card_path())?;
        for (tag, value) in card::records(&data) {
            if tag == card::uni::SPAN {
                return card::str_of(value).and_then(|s| parse_ratio(&s));
            }
        }
        None
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
        // Span carried forward — never re-asserted as a hard-coded 64.
        let span = self
            .read_universe_span()
            .map(|s| ratio_str(&s))
            .unwrap_or_else(|| "64".into());
        self.write_universe_card_inner(&span, ticks)
    }

    fn write_universe_card_inner(&self, span: &str, ticks: u64) -> std::io::Result<()> {
        let mut out = Vec::new();
        card::put_str(card::uni::TOPOLOGY, "torus", &mut out);
        card::put_str(card::uni::SPAN, span, &mut out);
        card::put_str(card::uni::TUNIVERSAL, &universal_clock(ticks), &mut out);
        card::put_u64(card::uni::TICKS, ticks, &mut out);
        card::put_str(
            card::uni::FIBER,
            "polynomial (physics::AxisVector grades; minted dimensions grow it)",
            &mut out,
        );
        card::write_atomic(&self.universe_card_path(), &out)
    }

    /// Chart a world onto the star-chart: exact-rational coordinates in
    /// `<root>/starchart.card` (TLV merge-on-write). Spatial coordinates
    /// WRAP onto the 3-torus; `g` is the gauge phase.
    pub fn chart_world(
        &self,
        world: &str,
        s: (&Ratio, &Ratio, &Ratio),
        g: &Ratio,
    ) -> std::io::Result<()> {
        fs::create_dir_all(&self.root)?;
        if !self.universe_card_path().exists() {
            self.write_universe_card(0)?;
        }
        let span = self.universe_span();
        let wrap = |v: &Ratio| -> Ratio { v - (v / &span).floor() * &span };
        let (x, y, z) = (wrap(s.0), wrap(s.1), wrap(s.2));
        let mut entries = self.load_starchart_entries();
        let slug = sanitize(world);
        entries.retain(|(name, _)| name != &slug);
        entries.push((
            slug,
            [ratio_str(&x), ratio_str(&y), ratio_str(&z), ratio_str(g)],
        ));
        entries.sort();
        self.write_starchart_entries(&entries)
    }

    fn load_starchart_entries(&self) -> Vec<(String, [String; 4])> {
        match card::read(&self.starchart_path()) {
            Some(data) => parse_starchart_tlv(&data),
            None => Vec::new(),
        }
    }

    fn write_starchart_entries(
        &self,
        entries: &[(String, [String; 4])],
    ) -> std::io::Result<()> {
        let mut out = Vec::new();
        for (name, [px, py, pz, pg]) in entries {
            card::put_str(card::chart::WORLD, name, &mut out);
            card::put_str(card::chart::X, px, &mut out);
            card::put_str(card::chart::Y, py, &mut out);
            card::put_str(card::chart::Z, pz, &mut out);
            card::put_str(card::chart::G, pg, &mut out);
            card::put(card::chart::END, &[], &mut out);
        }
        card::write_atomic(&self.starchart_path(), &out)
    }

    /// Read one world's charted coordinate: (S = (x, y, z), G).
    pub fn charted_position(&self, world: &str) -> Option<((Ratio, Ratio, Ratio), Ratio)> {
        let slug = sanitize(world);
        for (name, [px, py, pz, pg]) in self.load_starchart_entries() {
            if name == slug {
                return Some((
                    (
                        parse_ratio(&px)?,
                        parse_ratio(&py)?,
                        parse_ratio(&pz)?,
                    ),
                    parse_ratio(&pg)?,
                ));
            }
        }
        None
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

/// The wrapped per-axis |Δ| — the short way around the torus. Either
/// coordinate may sit outside `[0, span)` (an uncharted spec position): the
/// difference is wrapped onto `[0, span)` first, then the shorter arc taken.
pub(crate) fn wrapped_abs(a: &Ratio, b: &Ratio, span: &Ratio) -> Ratio {
    let raw = a - b;
    let d = &raw - (&raw / span).floor() * span;
    let around = span - &d;
    if around < d {
        around
    } else {
        d
    }
}

/// **THE TORUS DISTANCE** — the one distance between two points of the
/// universe: the L1 law on the 3-torus (wrapped |Δ| per axis, the short way
/// around, summed), against the universe's carried `span`. Every distance in
/// the game (melt, logistics, routes, the desk) reads THIS; a sum of unwrapped
/// |Δ| flattens the torus into a board with edges.
pub fn torus_distance(a: &(Ratio, Ratio, Ratio), b: &(Ratio, Ratio, Ratio), span: &Ratio) -> Ratio {
    wrapped_abs(&a.0, &b.0, span) + wrapped_abs(&a.1, &b.1, span) + wrapped_abs(&a.2, &b.2, span)
}

/// Parse the ledger's exact rendering back: `p/q` or a plain integer.
pub fn parse_ratio(text: &str) -> Option<Ratio> {
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

/// Parse starchart TLV (concatenated entries ended by tag END).
fn parse_starchart_tlv(data: &[u8]) -> Vec<(String, [String; 4])> {
    let mut entries = Vec::new();
    let mut cur: Option<(String, [String; 4])> = None;
    for (tag, value) in card::records(data) {
        match tag {
            card::chart::WORLD => {
                if let Some(prev) = cur.take() {
                    entries.push(prev);
                }
                let name = card::str_of(value).unwrap_or_default();
                cur = Some((name, ["0".into(), "0".into(), "0".into(), "0".into()]));
            }
            card::chart::X => {
                if let Some((_, ref mut c)) = cur {
                    if let Some(s) = card::str_of(value) {
                        c[0] = s;
                    }
                }
            }
            card::chart::Y => {
                if let Some((_, ref mut c)) = cur {
                    if let Some(s) = card::str_of(value) {
                        c[1] = s;
                    }
                }
            }
            card::chart::Z => {
                if let Some((_, ref mut c)) = cur {
                    if let Some(s) = card::str_of(value) {
                        c[2] = s;
                    }
                }
            }
            card::chart::G => {
                if let Some((_, ref mut c)) = cur {
                    if let Some(s) = card::str_of(value) {
                        c[3] = s;
                    }
                }
            }
            card::chart::END => {
                if let Some(prev) = cur.take() {
                    entries.push(prev);
                }
            }
            _ => {}
        }
    }
    if let Some(prev) = cur.take() {
        entries.push(prev);
    }
    entries
}

/// Steward directive TLV.
fn parse_directive_tlv(data: &[u8]) -> Option<Directive> {
    let mut verb = None;
    let mut args = Vec::new();
    let mut acting_as = "steward".into();
    for (tag, value) in card::records(data) {
        match tag {
            card::dir::VERB => verb = card::str_of(value),
            card::dir::ARG => {
                if let Some(a) = card::str_of(value) {
                    args.push(a);
                }
            }
            card::dir::ACTING_AS => {
                if let Some(a) = card::str_of(value) {
                    acting_as = a;
                }
            }
            _ => {}
        }
    }
    Some(Directive {
        verb: verb?,
        args,
        acting_as,
    })
}

pub fn write_directive_tlv(path: &std::path::Path, d: &Directive) -> std::io::Result<()> {
    let mut out = Vec::new();
    card::put_str(card::dir::VERB, &d.verb, &mut out);
    for a in &d.args {
        card::put_str(card::dir::ARG, a, &mut out);
    }
    card::put_str(card::dir::ACTING_AS, &d.acting_as, &mut out);
    card::write_atomic(path, &out)
}

#[cfg(test)]
mod torus_tests {
    use super::torus_distance;
    use cfe::ratio::ratio;

    #[test]
    fn the_torus_has_no_edge_the_short_way_is_taken_on_every_axis() {
        let span = ratio(64, 1);
        let p = |x, y, z| (ratio(x, 1), ratio(y, 1), ratio(z, 1));
        // opposite faces are neighbours
        assert_eq!(torus_distance(&p(1, 0, 0), &p(63, 0, 0), &span), ratio(2, 1));
        // a coordinate outside [0, span) wraps first
        assert_eq!(torus_distance(&p(-1, 0, 0), &p(1, 0, 0), &span), ratio(2, 1));
        assert_eq!(torus_distance(&p(65, 0, 0), &p(0, 0, 0), &span), ratio(1, 1));
        // the long way is never taken: at most half the span per axis
        assert_eq!(torus_distance(&p(0, 0, 0), &p(32, 40, 10), &span), ratio(32 + 24 + 10, 1));
        // symmetric, zero on itself, exact on rationals
        let a = (ratio(1, 3), ratio(127, 2), ratio(0, 1));
        let b = (ratio(63, 1), ratio(1, 2), ratio(5, 7));
        assert_eq!(torus_distance(&a, &b, &span), torus_distance(&b, &a, &span));
        assert_eq!(torus_distance(&a, &a, &span), ratio(0, 1));
        assert_eq!(torus_distance(&a, &b, &span), ratio(4, 3) + ratio(1, 1) + ratio(5, 7));
    }
}

#[cfg(test)]
mod core_tests {
    use super::Ledger;
    use cfe::ratio::ratio;

    #[test]
    fn every_place_reaches_a_core_nearest_by_the_torus() {
        let root = std::env::temp_dir().join(format!("ledger-cores-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let book = Ledger::at(&root);
        let p = |x: i64, y: i64, z: i64| (ratio(x, 1), ratio(y, 1), ratio(z, 1));
        assert_eq!(book.core_of(&p(0, 0, 0)), None, "no core before any galaxy is charted");
        // the founding galaxy at the origin, then a far one at (32, 0, 0)
        book.observe_galaxy("a", "seat", (&ratio(0, 1), &ratio(0, 1), &ratio(0, 1)), 1).unwrap();
        book.observe_galaxy("b", "seat", (&ratio(32, 1), &ratio(0, 1), &ratio(0, 1)), 2).unwrap();
        assert_eq!(book.core_of(&p(30, 0, 0)).as_deref(), Some("b-sky-1"));
        // across the torus's edge, the origin's core is the near one
        assert_eq!(book.core_of(&p(62, 0, 0)).as_deref(), Some("the-founding"));
        // outside every reach (16), still the nearest core: nothing lands nowhere
        assert_eq!(book.core_of(&p(14, 30, 0)).as_deref(), Some("the-founding"));
        book.append_universe(
            vec![cfe::dop::ClosureEvent::CoreDeposited { core: "b-sky-1".into(), world: "w".into(), resource: None, amount: ratio(3, 2) }],
            3,
        )
        .unwrap();
        assert_eq!(book.core_holdings().get(&("b-sky-1".to_string(), None)), Some(&ratio(3, 2)));
        book.append_universe(
            vec![cfe::dop::ClosureEvent::CoreEmitted { core: "b-sky-1".into(), world: "w".into(), resource: None, amount: ratio(1, 2) }],
            4,
        )
        .unwrap();
        assert_eq!(book.core_holdings().get(&("b-sky-1".to_string(), None)), Some(&ratio(1, 1)), "deposits less emissions");
        // the dead's library: each line's word its own claim, conflicts kept
        let word = |world: &str, orbit: u8| cfe::dop::ClosureEvent::CoreWordDeposited {
            core: "b-sky-1".into(),
            world: world.into(),
            orbit: [orbit; 32],
            expansion: vec![1, 2],
            cited: vec![[orbit; 32]],
        };
        book.append_universe(vec![word("dead-a", 7), word("dead-b", 7), word("dead-b", 8)], 5).unwrap();
        let library = &book.core_libraries()["b-sky-1"];
        assert_eq!(library.len(), 3, "two lines' same word stays two claims");
        assert_eq!(library[0].world, "dead-a");
        assert_eq!(library[0].stamp, 5);
    }
}
