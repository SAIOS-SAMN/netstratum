//! GALAXY-1a (13_GALAXY §3–§4): the observation deed and its readings.
//! One universe, structurally — galaxies are lazy REGIONS of the one
//! torus, never new tori. The universe ledger (`convergence.tlv`) is
//! the truth; `galaxies.json` is a rendering; membership derives from
//! position and is never stored. The frontier is a distance, never an
//! outside: wrapped-L1 to every charted center against its reach.

use std::fs;
use std::path::PathBuf;

use cfe::dop::ClosureEvent;
use cfe::persist;
use cfe::ratio::{ratio, Ratio};
use cfe::vrp::Chronicle;

use crate::extract::extract_string;
use pen::JsonWriter;
use crate::{ratio_str, sanitize, wrapped_abs, Ledger};

/// A charted galaxy: a named region of the ONE torus — center S in
/// exact rationals, reach an L1 radius. Reach is a placeholder
/// parameter (13_GALAXY §4); naming refines under the 10 §2 law
/// (names are the line's own tongue) at GALAXY-1b.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Galaxy {
    pub name: String,
    pub center: (Ratio, Ratio, Ratio),
    pub reach: Ratio,
}

/// The placeholder reach — both the founding galaxy's and a newly
/// observed one's, until the fiber-growth reading lands.
fn reach_constant() -> Ratio {
    ratio(16, 1)
}

fn io_error<E: std::fmt::Debug>(e: E) -> std::io::Error {
    std::io::Error::other(format!("{e:?}"))
}

impl Ledger {
    fn ledger_path(&self) -> PathBuf {
        self.root.join("convergence.tlv")
    }

    /// Every charted galaxy, ledger order — read from the universe
    /// ledger's `GalaxyObserved` receipts. The receipt is the truth;
    /// this reading never needs the card, and a reader writes nothing
    /// (the lens discipline).
    pub fn galaxies(&self) -> Vec<Galaxy> {
        let path = self.ledger_path();
        if !path.exists() {
            return Vec::new();
        }
        let Ok(ledger) = persist::load(&path) else {
            return Vec::new();
        };
        ledger
            .receipts()
            .iter()
            .filter_map(|receipt| match &receipt.event {
                ClosureEvent::GalaxyObserved { name, center, reach, .. } => Some(Galaxy {
                    name: name.clone(),
                    center: center.clone(),
                    reach: reach.clone(),
                }),
                _ => None,
            })
            .collect()
    }

    /// Derived membership (13_GALAXY §4): the galaxy whose center
    /// stands L1-nearest within reach, ties broken by name; outside
    /// every reach the world is intergalactic (None) — lawful, exactly
    /// as headless and pre-solar are lawful.
    pub fn galaxy_of(&self, world: &str) -> Option<String> {
        let ((x, y, z), _) = self.charted_position(world)?;
        let span = self.universe_span();
        self.galaxies()
            .into_iter()
            .map(|galaxy| {
                let distance = l1_wrapped(&(x.clone(), y.clone(), z.clone()), &galaxy.center, &span);
                (distance, galaxy)
            })
            .filter(|(distance, galaxy)| distance <= &galaxy.reach)
            .min_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.name.cmp(&b.1.name)))
            .map(|(_, galaxy)| galaxy.name)
    }

    /// Every charted world on the star-chart, chart order — the
    /// S-side population a galaxy reading sweeps.
    pub fn charted_worlds(&self) -> Vec<String> {
        let Ok(text) = fs::read_to_string(self.root.join("starchart.json")) else {
            return Vec::new();
        };
        text.lines()
            .filter_map(|line| extract_string(line, "world"))
            .collect()
    }

    /// The observation deed (13_GALAXY §3): the anchor wraps onto the
    /// torus; the FOUNDING galaxy is chartered lazily by the first
    /// deed ever executed; a new galaxy is CREATED only when the
    /// anchor stands past the frontier — wrapped-L1 to EVERY charted
    /// center exceeding its reach (a far region, never an outside).
    /// Both receipts append to the universe ledger; `galaxies.json`
    /// re-renders after (SEAL C). Returns the created galaxy, or None
    /// when the anchor lies inside a charted sky.
    ///
    /// `observer` is the crossing civilization; `creator` the
    /// progenitor seat executing the deed (12 §2a — the board-maker
    /// function); `stamp` is the caller's universal T (the conductor's
    /// clock at the year voice).
    pub fn observe_galaxy(
        &self,
        observer: &str,
        creator: &str,
        anchor: (&Ratio, &Ratio, &Ratio),
        stamp: u64,
    ) -> std::io::Result<Option<Galaxy>> {
        fs::create_dir_all(&self.root)?;
        let span = self.universe_span();
        let wrap = |v: &Ratio| -> Ratio { v - (v / &span).floor() * &span };
        let anchor = (wrap(anchor.0), wrap(anchor.1), wrap(anchor.2));

        let path = self.ledger_path();
        let mut ledger = match path.exists() {
            true => persist::load(&path).map_err(io_error)?,
            false => Chronicle::new(),
        };
        let mut galaxies = self.galaxies();

        let charter = |ledger: &mut Chronicle, galaxy: &Galaxy| {
            ledger.append(
                ClosureEvent::GalaxyObserved {
                    observer: observer.into(),
                    creator: creator.into(),
                    name: galaxy.name.clone(),
                    center: galaxy.center.clone(),
                    reach: galaxy.reach.clone(),
                },
                stamp,
                vec![],
            );
        };

        if galaxies.is_empty() {
            let founding = Galaxy {
                name: "the-founding".into(),
                center: (ratio(0, 1), ratio(0, 1), ratio(0, 1)),
                reach: reach_constant(),
            };
            charter(&mut ledger, &founding);
            galaxies.push(founding);
        }

        let inside = galaxies
            .iter()
            .any(|galaxy| l1_wrapped(&anchor, &galaxy.center, &span) <= galaxy.reach);
        let created = if inside {
            None
        } else {
            let galaxy = Galaxy {
                // Deterministic placeholder name: replay re-derives it
                // from the ledger alone (count is monotone).
                name: format!("{}-sky-{}", sanitize(observer), galaxies.len()),
                center: anchor,
                reach: reach_constant(),
            };
            charter(&mut ledger, &galaxy);
            galaxies.push(galaxy.clone());
            Some(galaxy)
        };

        persist::sync(&ledger, &path).map_err(io_error)?;
        self.render_galaxies_card(&galaxies)?;
        Ok(created)
    }

    /// INCURSION-1a (07 §3.1): the door's receipt on the universe
    /// ledger — era, world, and the woken line, beside EraAnnexed and
    /// GalaxyObserved.
    pub fn strike_incursion(
        &self,
        era: &str,
        world: &str,
        line: &str,
        stamp: u64,
    ) -> std::io::Result<()> {
        let path = self.ledger_path();
        let mut ledger = match path.exists() {
            true => persist::load(&path).map_err(io_error)?,
            false => Chronicle::new(),
        };
        ledger.append(
            ClosureEvent::IncursionStruck {
                era: era.into(),
                world: world.into(),
                line: line.into(),
            },
            stamp,
            vec![],
        );
        persist::sync(&ledger, &path).map_err(io_error)?;
        Ok(())
    }

    /// The card: one line per galaxy, a REFLECTION of the ledger —
    /// deleting it loses nothing (SEAL C).
    fn render_galaxies_card(&self, galaxies: &[Galaxy]) -> std::io::Result<()> {
        let mut out = String::new();
        for galaxy in galaxies {
            let mut w = JsonWriter::new();
            w.open();
            w.field_str("galaxy", &galaxy.name);
            w.field_str("x", &ratio_str(&galaxy.center.0));
            w.field_str("y", &ratio_str(&galaxy.center.1));
            w.field_str("z", &ratio_str(&galaxy.center.2));
            w.field_str("reach", &ratio_str(&galaxy.reach));
            w.close();
            out.push_str(&w.finish());
            out.push('\n');
        }
        let tmp = self.root.join("galaxies.json.tmp");
        fs::write(&tmp, out)?;
        fs::rename(&tmp, self.root.join("galaxies.json"))
    }
}

/// The L1 law on the torus: wrapped |Δ| summed per axis, the short way
/// around — NO SQUARING.
fn l1_wrapped(
    a: &(Ratio, Ratio, Ratio),
    b: &(Ratio, Ratio, Ratio),
    span: &Ratio,
) -> Ratio {
    wrapped_abs(&a.0, &b.0, span) + wrapped_abs(&a.1, &b.1, span) + wrapped_abs(&a.2, &b.2, span)
}
