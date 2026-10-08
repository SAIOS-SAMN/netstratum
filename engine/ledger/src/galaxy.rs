//! GALAXY-1a (13_GALAXY §3–§4): the observation deed and its readings.
//! One universe, structurally — galaxies are lazy REGIONS of the one
//! torus, never new tori. The universe ledger (`convergence.tlv`) is
//! the truth; `galaxies.card` is a TLV rendering; membership derives from
//! position and is never stored. The frontier is a distance, never an
//! outside: wrapped-L1 to every charted center against its reach.

use std::fs;
use std::path::PathBuf;

use cfe::dop::ClosureEvent;
use cfe::persist;
use cfe::ratio::{ratio, Ratio};
use cfe::vrp::Chronicle;

use crate::{card, ratio_str, sanitize, Ledger};

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

/// One dead line's word at a core: its own claim, citing the line and the
/// proofs it was minted on, and the universal stamp it was deposited at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreWord {
    pub world: String,
    pub orbit: [u8; 32],
    pub expansion: Vec<u8>,
    pub cited: Vec<[u8; 32]>,
    pub stamp: u64,
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
            .filter_map(|receipt| match receipt.event() {
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
        self.load_starchart_entries()
            .into_iter()
            .map(|(name, _)| name)
            .collect()
    }

    /// The observation deed (13_GALAXY §3): the anchor wraps onto the
    /// torus; the FOUNDING galaxy is chartered lazily by the first
    /// deed ever executed; a new galaxy is CREATED only when the
    /// anchor stands past the frontier — wrapped-L1 to EVERY charted
    /// center exceeding its reach (a far region, never an outside).
    /// Both receipts append to the universe ledger; `galaxies.card`
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

    /// **THE CORE A PLACE'S SINKS REACH** (the cores design, slice 1): its
    /// galaxy's — the charted centre nearest by the torus's own distance
    /// within reach (`galaxy_of`'s law) — and, outside every reach, still the
    /// NEAREST core: nothing lands nowhere. Ties go to the earlier name.
    /// `None` only before any galaxy is charted.
    pub fn core_of(&self, position: &(Ratio, Ratio, Ratio)) -> Option<String> {
        let span = self.universe_span();
        let ranked = self.galaxies().into_iter().map(|galaxy| {
            let distance = l1_wrapped(position, &galaxy.center, &span);
            (distance > galaxy.reach, distance, galaxy.name)
        });
        ranked.min().map(|(_, _, name)| name)
    }

    /// Append events to the universe ledger at `stamp`, in order, and sync.
    pub fn append_universe(&self, events: Vec<ClosureEvent>, stamp: u64) -> std::io::Result<()> {
        if events.is_empty() {
            return Ok(());
        }
        fs::create_dir_all(&self.root)?;
        let path = self.ledger_path();
        let mut ledger = match path.exists() {
            true => persist::load(&path).map_err(io_error)?,
            false => Chronicle::new(),
        };
        for event in events {
            ledger.append(event, stamp, vec![]);
        }
        persist::sync(&ledger, &path).map_err(io_error)?;
        Ok(())
    }

    /// What each core holds: its `CoreDeposited` receipts less its
    /// `CoreEmitted` ones, per holding (`None` = STAMP). A reading: nothing
    /// is written.
    pub fn core_holdings(&self) -> std::collections::BTreeMap<(String, Option<cfe::dop::Resource>), Ratio> {
        let mut held = std::collections::BTreeMap::new();
        let Ok(ledger) = persist::load(&self.ledger_path()) else {
            return held;
        };
        for receipt in ledger.receipts() {
            match receipt.event() {
                ClosureEvent::CoreDeposited { core, resource, amount, .. } => {
                    *held.entry((core.clone(), *resource)).or_insert_with(|| ratio(0, 1)) += amount.clone();
                }
                ClosureEvent::CoreEmitted { core, resource, amount, .. } => {
                    *held.entry((core.clone(), *resource)).or_insert_with(|| ratio(0, 1)) -= amount.clone();
                }
                _ => {}
            }
        }
        held
    }

    /// **THE CORES' LIBRARIES** (the cores design, slice 3): every word the
    /// dead deposited at each core, by core, in deposit order — each dead
    /// line's word its OWN claim, citing the line and its proofs (conflicting
    /// words stay distinct). A reading: nothing is written.
    pub fn core_libraries(&self) -> std::collections::BTreeMap<String, Vec<CoreWord>> {
        let mut libraries: std::collections::BTreeMap<String, Vec<CoreWord>> = std::collections::BTreeMap::new();
        let Ok(ledger) = persist::load(&self.ledger_path()) else {
            return libraries;
        };
        for receipt in ledger.receipts() {
            if let ClosureEvent::CoreWordDeposited { core, world, orbit, expansion, cited } = receipt.event() {
                libraries.entry(core.clone()).or_default().push(CoreWord {
                    world: world.clone(),
                    orbit: *orbit,
                    expansion: expansion.clone(),
                    cited: cited.clone(),
                    stamp: receipt.stamp(),
                });
            }
        }
        libraries
    }

    /// **THE UNIVERSE'S SEED** (docs/58 §3) — the own-hash of its Big Bang
    /// (`UniverseFounded`) on the universe ledger, or `None` before one is
    /// recorded. A reading: nothing is written.
    pub fn universe_seed(&self) -> Option<[u8; 32]> {
        let path = self.ledger_path();
        let ledger = persist::load(&path).ok()?;
        ledger
            .receipts()
            .iter()
            .find(|r| matches!(r.event(), ClosureEvent::UniverseFounded { .. }))
            .map(|r| r.own().0)
    }

    /// **THE BIG BANG** (docs/58 §3) — record this universe's origin ONCE on
    /// its universe ledger and return its seed. On an empty ledger it is the
    /// FIRST receipt; on a ledger that already holds history (a universe
    /// founded before this law) it is written once, late, as a RE-FOUNDING
    /// naming the head it was written over. Called again, it returns the seed
    /// already recorded and writes nothing.
    pub fn found_universe(&self, roster: [u8; 32], matter: [u8; 32], stamp: u64) -> std::io::Result<([u8; 32], bool)> {
        if let Some(seed) = self.universe_seed() {
            return Ok((seed, false));
        }
        fs::create_dir_all(&self.root)?;
        let path = self.ledger_path();
        let mut ledger = match path.exists() {
            true => persist::load(&path).map_err(io_error)?,
            false => Chronicle::new(),
        };
        let over = ledger.receipts().last().map(|r| r.own().0);
        ledger.append(
            ClosureEvent::UniverseFounded {
                topology: "torus".into(),
                span: self.universe_span(),
                roster,
                matter,
                over,
            },
            stamp,
            vec![],
        );
        let seed = ledger.receipts().last().map(|r| r.own().0).expect("just appended");
        persist::sync(&ledger, &path).map_err(io_error)?;
        Ok((seed, true))
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

    /// The card: TLV reflection of the ledger — deleting it loses nothing (SEAL C).
    fn render_galaxies_card(&self, galaxies: &[Galaxy]) -> std::io::Result<()> {
        let mut out = Vec::new();
        for galaxy in galaxies {
            card::put_str(card::galaxy::NAME, &galaxy.name, &mut out);
            card::put_str(card::galaxy::CX, &ratio_str(&galaxy.center.0), &mut out);
            card::put_str(card::galaxy::CY, &ratio_str(&galaxy.center.1), &mut out);
            card::put_str(card::galaxy::CZ, &ratio_str(&galaxy.center.2), &mut out);
            card::put_str(card::galaxy::REACH, &ratio_str(&galaxy.reach), &mut out);
            card::put(card::galaxy::END, &[], &mut out);
        }
        card::write_atomic(&self.root.join("galaxies.card"), &out)
    }
}

/// The L1 law on the torus: wrapped |Δ| summed per axis, the short way
/// around — NO SQUARING.
fn l1_wrapped(
    a: &(Ratio, Ratio, Ratio),
    b: &(Ratio, Ratio, Ratio),
    span: &Ratio,
) -> Ratio {
    crate::torus_distance(a, b, span)
}
