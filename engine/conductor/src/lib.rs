//! N-CONDUCTOR + N-LINEAGE — the heartbeat and the family tree.
//!
//! Scripted civilizations play their colonies autonomously, each
//! through a CONTAINED AGENT (NS-6: granted scopes, every receipt
//! attributed `by#agent` — automation on a leash). Each heartbeat a
//! world: drains the steward inbox, plays its policy, ticks,
//! collects, grinds the forge (its OWN latest receipt digest seeds
//! the puzzle — CHARTER §9), and mirrors everything.
//!
//! **TECH IS EARNED (the pyramid loop closed):** smelters unlock at
//! forge rung 3, synthesizers at rung 5 — for every temperament.
//! Research depth is the tech tree; surplus funds search; search
//! unlocks industry; industry funds deeper search.
//!
//! **SPECIATION IS A MINT:** a civilization at rung 5 with 20
//! confirmed solves spawns ONE daughter per era — irreversible
//! dimensional growth, exactly `physics::FlowEvent::Mint`'s law worn
//! as gameplay. The mutation is chronicle-seeded: the parent's latest
//! receipt digest draws WHICH axis mutates and what the child is
//! called — the parent's own history decides what its children
//! become. Lineage is registry-recorded now, receipted at G1. A
//! daughter founds a NEW world (binding #1: one chronicle per world —
//! a shared-chronicle sibling planet waits on the multi-colony
//! engine).
//!
//! **LORE LIVES IN DATA, NOT SOURCE:** egregore specs load from
//! `<var>/egregores.json` when present; `write_default_egregores` seeds that
//! file once so even the founding NPCs are operator-editable. NPC
//! entries carry `founder: "conductor"` — the observer marks them ⚙.
//!
//! **THE AUTARKIES FEEL THE STAR (BOT-THERM):** genesis commissions
//! every egregore world's sun and sites its planet — spec-pinned, or
//! derived from the registry axes through the chronicle beacon (the
//! civilization's own genesis receipts choose its sky). Furnace-sited
//! egregores answer heat with encasement shells and mint joules;
//! Frost-sited egregores raise a cryo-lattice array, bank sink, and hold
//! expansion against the joule runway — honestly allowed to
//! freeze-lock; mesic egregores keep their temperament untouched.
//!
//! Everything is deterministic: same era length → byte-identical
//! receipts. HONEST SEAM: worlds live in process memory; reload from
//! the mirrored chronicle is the G0 persistence seam (coder-02).

use cfe::dop::{Resource, ZoneKind};
use cfe::ipsal::Scope;
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::World;
use mirror::galaxy::Galaxy;
use mirror::registry::{lineage_domain, Civilization, Registry, DISPOSITIONS, METABOLISMS, PERCEPTIONS};
use mirror::{Directive, Disposition, Mirror};
use physics::Beacon;
use solver::{forge_task, solve, Difficulty, SolveRead};
use std::path::Path;

mod egregores;
pub use egregores::{founding_egregores, load_egregores, write_default_egregores};

/// AEON-1b (11 §5): THE FOLD — seal the live era and rotate it into
/// the archive sequence. `EraFolded` (tag 46) appends to the universe
/// ledger with the NEW base gauge (the next aeon seats on its own
/// G-sheet — placeholder advance: era/12, wrapped), then the live
/// `worlds/` and `registry/` move whole into `<base>-era<N>/` beside
/// the var root. Nothing is destroyed — the fold is archival by
/// construction; the next boot CONVERGES the sealed era, and the four
/// C's read which trajectories continue. The roster file STAYS: the
/// civilizations continue into the new aeon.
pub fn fold(var_root: &str) -> std::io::Result<(u64, Ratio)> {
    use cfe::vrp::Chronicle;
    let root = std::path::PathBuf::from(var_root);
    let base = root
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "var".into());
    let parent = match root.parent() {
        Some(p) if p.as_os_str().is_empty() => std::path::PathBuf::from("."),
        Some(p) => p.to_path_buf(),
        None => std::path::PathBuf::from("."),
    };
    // The next era ordinal: one past the highest archived.
    let mut era: u64 = 1;
    if let Ok(entries) = std::fs::read_dir(&parent) {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().into_owned();
            if let Some(n) = name.strip_prefix(&format!("{base}-era")) {
                if let Ok(held) = n.parse::<u64>() {
                    era = era.max(held + 1);
                }
            }
        }
    }
    let worlds = root.join("worlds");
    if !worlds.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no live worlds to fold",
        ));
    }
    // Seal: EraFolded onto the universe ledger, base G advanced.
    let ledger_path = worlds.join("convergence.tlv");
    let mut ledger = match cfe::persist::load(&ledger_path) {
        Ok(held) => held,
        Err(_) => Chronicle::new(),
    };
    let gauge = cfe::world::wrap_gauge(ratio(era as i64, 12));
    let stamp = ledger.receipts().iter().map(|r| r.stamp).max().unwrap_or(0);
    ledger.append(
        cfe::dop::ClosureEvent::EraFolded { era, gauge: gauge.clone() },
        stamp,
        vec![],
    );
    cfe::persist::sync(&ledger, &ledger_path)
        .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
    // Rotate: the archive sequence gains the sealed era, whole.
    let archive = parent.join(format!("{base}-era{era}"));
    std::fs::create_dir_all(&archive)?;
    std::fs::rename(&worlds, archive.join("worlds"))?;
    let registry = root.join("registry");
    if registry.is_dir() {
        std::fs::rename(&registry, archive.join("registry"))?;
    }
    let roster = root.join("egregores.json");
    if roster.is_file() {
        std::fs::copy(&roster, archive.join("bots.json"))?;
    }
    Ok((era, gauge))
}

/// SCIENCE-1 (10 §3d): the RELEVANCE table — which industries exist
/// in a science's own tree. A plasmic line carries no forge-of-iron
/// (smelting is meaningless to a current sheet); its advancement
/// crosses by the joint research venture instead. Placeholder table,
/// niche-first (the FUNCTION-1 lesson).
/// The AericDeckBand mapping (RELEVANCE ENTRY ruling, 2026-07-31):
/// an aeric home is a banded COLUMN — cloud-deck mesic over
/// hyper-thermal depths (12 §5a) — so relevance evaluation reads the
/// DECK's thermal seat, never the surface's. A reading past the
/// mesic ceiling displaces to the deck (the ceiling); colder stands
/// (a frozen giant's deck is honestly frozen). No third siting axis:
/// geometry stays clean, and every non-aeric body reads unchanged.
pub fn deck_band4(metabolism: &str, t4: Ratio) -> Ratio {
    if metabolism != "aeric" {
        return t4;
    }
    let (_, mesic_hi) = cfe::thermal::SubstrateBand::Mesic.bounds4();
    if t4 > mesic_hi { mesic_hi } else { t4 }
}

pub fn industry_relevant(metabolism: &str, kind: ZoneKind) -> bool {
    !matches!(
        (metabolism, kind),
        ("plasmic", ZoneKind::Smelter | ZoneKind::Synthesizer)
    )
}

/// RATIO SYNTHESIS (ruling 2026-07-31): price is an emergent ratio,
/// never a system property. The reserve a line considers "enough" —
/// the denominator of its relaxed ask and the floor of its surplus.
pub const TARGET_RESERVE: (i64, i64) = (8, 1);

/// A famished line bids STEEP — it gives more than it wants, and
/// deeper famine bids steeper: want = give / (1 + severity).
fn steep_want(give: &Ratio, severity: u32) -> Ratio {
    give.clone() / ratio(1 + severity as i64, 1)
}

/// A surplus-rich line RELAXES — it asks a premium that emerges from
/// its own reserve fraction: want = give × (2·reserve − target)/
/// reserve. Exactly give at the target margin, approaching (never
/// reaching) 2× as the reserve deepens — so a steep famished bid
/// (severity ≥ 1 gives 2:1) always crosses it, while flat 1:1 asks
/// rest lawfully on the book; nothing forces a clear.
fn relaxed_want(give: &Ratio, reserve: &Ratio) -> Ratio {
    let target = ratio(TARGET_RESERVE.0, TARGET_RESERVE.1);
    give.clone() * (ratio(2, 1) * reserve.clone() - target) / reserve.clone()
}

/// OFFER-1a (BUILD_ORDER §3a): a world's TRADE INTENT, read pure from
/// its committed state. Ratios derive from the state itself (famine
/// severity, reserve depth) — there is no base price and none is
/// needed: no ratio is too high or too low, an unmatched one rests
/// (ruling 2026-07-31). Amounts are halves of the held stock, never
/// the whole (guardrail 1's spirit: no world corners itself). The
/// metabolism rides in for the relevance evaluation (the aeric deck).
pub fn trade_intent(
    world: &World,
    metabolism: &str,
) -> Option<(Resource, Ratio, Resource, Ratio)> {
    let orb = world.orb()?;
    let half = |amount: Ratio| amount / ratio(2, 1);
    let severity = orb.famine.len() as u32;
    // The regime specials first (niche before behavior — the
    // FUNCTION-1 order): the Furnace↔Frost fuel axis.
    if let Some(regime) = regime(world, metabolism) {
        match regime {
            (Regime::Frost, _) => {
                let starving = orb.famine.contains(&Resource::Joule);
                let sink = orb.banked_amount(Resource::Sink);
                if starving && sink > ratio(0, 1) {
                    let give = half(sink);
                    let want = steep_want(&give, severity);
                    return Some((Resource::Sink, give, Resource::Joule, want));
                }
            }
            (Regime::Furnace, _) => {
                let joules = orb.banked_amount(Resource::Joule);
                if joules > ratio(TARGET_RESERVE.0, TARGET_RESERVE.1) {
                    let give = half(joules.clone());
                    let want = relaxed_want(&give, &joules);
                    return Some((Resource::Joule, give, Resource::Sink, want));
                }
            }
        }
    }
    // THE FAMINE ASK: ANY famished line offers half its LARGEST bank
    // for the famined resource, bidding steep with its severity.
    if let Some(&famined) = orb.famine.first() {
        let biggest = Resource::ALL
            .iter()
            .filter(|r| **r != famined)
            .max_by_key(|r| orb.banked_amount(**r))?;
        let stock = orb.banked_amount(*biggest);
        if stock > ratio(0, 1) {
            let give = half(stock);
            let want = steep_want(&give, severity);
            return Some((*biggest, give, famined, want));
        }
        return None;
    }
    // THE SURPLUS OFFER: an unfamished line with a reserve past
    // target offers half of it at its relaxed premium — wanting the
    // leanest of the resources its OWN zones burn (demand grounded
    // in upkeep, never in an empty slot — measured 2026-07-30; a
    // resting offer blocks new intents, so wanting one's own upkeep
    // keeps the resting ask correct when famine later strikes it).
    let biggest = Resource::ALL
        .iter()
        .max_by_key(|r| orb.banked_amount(**r))?;
    let stock = orb.banked_amount(*biggest);
    if stock > ratio(TARGET_RESERVE.0, TARGET_RESERVE.1) {
        let burns: Vec<Resource> = orb
            .zones
            .iter()
            .flat_map(|z| z.kind.upkeep_per_tick())
            .map(|(resource, _)| resource)
            .filter(|resource| resource != biggest)
            .collect();
        let leanest = burns.iter().min_by_key(|r| orb.banked_amount(**r))?;
        let give = half(stock.clone());
        let want = relaxed_want(&give, &stock);
        return Some((*biggest, give, *leanest, want));
    }
    None
}

/// OFFER-1a: one market beat over the whole board — post fresh
/// intents (one open offer per world; the offer id is the chronicle
/// length at posting, deterministic), then clear the FIRST crossing
/// (price-time priority inside `clear_trade`). Returns the cleared
/// pairs (world names) for the narrator line.
pub fn market_step(worlds: &mut [Egregore], beat: u64) -> Vec<(String, String)> {
    // Post intents.
    for egregore in worlds.iter_mut() {
        if egregore.penned || !egregore.world.offers().is_empty() {
            continue;
        }
        if let Some((give, give_amount, want, want_amount)) =
            trade_intent(&egregore.world, &egregore.spec.civilization.metabolism)
        {
            let offer = egregore.world.chronicle().receipts().len() as u64;
            let orb = cfe::world::FIRST_ORB;
            let _ = egregore
                .world
                .post_offer(orb, give, give_amount, want, want_amount, offer);
        }
    }
    // Clear crossings, pairwise, first match per beat.
    let mut cleared = Vec::new();
    for a in 0..worlds.len() {
        for b in (a + 1)..worlds.len() {
            let (left, right) = worlds.split_at_mut(b);
            let one = &mut left[a];
            let two = &mut right[0];
            if one.penned || two.penned {
                continue;
            }
            let (Some(offer_a), Some(offer_b)) = (
                one.world.offers().first().map(|o| o.offer),
                two.world.offers().first().map(|o| o.offer),
            ) else {
                continue;
            };
            let deal = beat * 1_000 + (a as u64) * 10 + b as u64;
            if cfe::world::clear_trade(&mut one.world, offer_a, &mut two.world, offer_b, deal)
                .is_ok()
            {
                cleared.push((
                    one.spec.world_name.clone(),
                    two.spec.world_name.clone(),
                ));
            }
        }
    }
    cleared
}

/// A build temperament — the only thing that differs between egregores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    /// Two extractors per habitat, expand at the food knife-edge.
    Aggressive,
    /// One-to-one, build only with a full cost buffer banked.
    Steady,
    /// Extractor-heavy early, first in line for unlocked industry.
    Industry,
}

impl Policy {
    pub fn parse(word: &str) -> Option<Policy> {
        match word {
            "aggressive" => Some(Policy::Aggressive),
            "steady" => Some(Policy::Steady),
            "industry" => Some(Policy::Industry),
            _ => None,
        }
    }

    pub fn describe(&self) -> &'static str {
        match self {
            Policy::Aggressive => "aggressive",
            Policy::Steady => "steady",
            Policy::Industry => "industry",
        }
    }
}

/// One scripted civilization and where it settles.
#[derive(Debug, Clone)]
pub struct EgregoreSpec {
    pub civilization: Civilization,
    pub world_name: String,
    pub planet: String,
    pub colony: String,
    pub policy: Policy,
    /// The world's spatial coordinate S = (x, y, z) on the 3-torus.
    pub position: (Ratio, Ratio, Ratio),
    /// BOT-THERM: the world's sun, operator-pinned; `None` derives at
    /// genesis from the civilization's registry axes, beacon-drawn
    /// from its own genesis receipts (CHARTER §9 — never wall clock).
    pub star: Option<StarClass>,
    /// BOT-THERM: the orbit albedo, pinned; `None` derives (see `star`).
    pub albedo: Option<Ratio>,
    /// BOT-THERM: the orbit radius, pinned; `None` derives (see `star`).
    pub orbit: Option<Ratio>,
    /// AEON-1b: the world's opening gauge key, wrapped [0, 1); `None`
    /// founds at zero. A divergence branch carries its cleaved key.
    pub gauge: Option<Ratio>,
}

/// One live egregore world: the sim, its contained actor, its research rung.
pub struct Egregore {
    pub spec: EgregoreSpec,
    pub world: World,
    /// The egregore's contained agent (build/raze/collect granted).
    automaton: u64,
    /// The steward's contained entity (drains the operator's inbox).
    steward: u64,
    /// The forge rung this civilization currently attempts.
    pub level: u32,
    /// Confirms at the current rung (3 promote).
    streak: u32,
    /// Confirmed solves this era.
    pub solved: u32,
    /// One daughter per era — speciation is rare, not a flood.
    spawned: bool,
    /// MESH-1b: the pen is out — beats, saves, and the market wait.
    pub penned: bool,
}

fn banked(world: &World, resource: Resource) -> Ratio {
    world
        .orb()
        .and_then(|orb| {
            orb.banked
                .iter()
                .find(|(r, _)| *r == resource)
                .map(|(_, amount)| amount.clone())
        })
        .unwrap_or_else(|| ratio(0, 1))
}

fn zone_count(world: &World, kind: ZoneKind) -> usize {
    world
        .orb()
        .map(|orb| orb.zones.iter().filter(|z| z.kind == kind).count())
        .unwrap_or(0)
}

fn next_free_tile(world: &World) -> Option<u16> {
    let orb = world.orb()?;
    (0u16..64).find(|tile| !orb.zones.iter().any(|z| z.tile == *tile))
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// BOT-THERM: the Frost runway law — a sub-mesic colony expands only
/// when its banked joules cover the one-tick heating bill of the
/// GROWN city (ECON-J, 05 §1.2: `heating_per_zone × (zones + 1)`).
/// Exact rational comparison: meeting the bill exactly holds the
/// runway; one grain short refuses.
pub fn joule_runway_allows(banked_joules: &Ratio, zones: usize) -> bool {
    *banked_joules >= cfe::dop::heating_per_zone() * ratio(zones as i64 + 1, 1)
}

/// BOT-THERM: the thermal regime a egregore world lives in. The mesic band
/// is home (temperament unchanged — `None`); above its ceiling the
/// Furnace laws apply, below its floor the Frost laws. Unsited worlds
/// carry no regime (the engine's opt-in guard, mirrored here).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Regime {
    Furnace,
    Frost,
}

fn regime(world: &World, metabolism: &str) -> Option<(Regime, Ratio)> {
    let class = world.star()?;
    let site = world.orb()?.thermal.clone()?;
    let t4 = deck_band4(
        metabolism,
        cfe::thermal::t_eq4(&class.luminosity(), &site.albedo, &site.orbit_r),
    );
    let (mesic_lo, mesic_hi) = cfe::thermal::SubstrateBand::Mesic.bounds4();
    if t4 < mesic_lo {
        Some((Regime::Frost, t4))
    } else if t4 > mesic_hi {
        Some((Regime::Furnace, t4))
    } else {
        None
    }
}

/// BOT-THERM: the axis→regime preference (06 §1) for specs that pin no
/// siting — a mineral metabolism settles hot (the Furnace mints; 05
/// §3.1), a biotic one settles mesic. The exact seat inside the regime
/// is beacon-drawn from the civilization's OWN genesis receipts:
/// deterministic, replayable, never authored. A Frost seat is never
/// derived — settling the vault is a deliberate act (the operator's,
/// or the founding file's, which seeds one).
fn derived_siting(civilization: &Civilization, beacon: &Beacon) -> (StarClass, Ratio, Ratio) {
    match civilization.metabolism.as_str() {
        // A plasmic metabolism is of the stellar environs (12_UNION
        // §5a) — it shares the Furnace regime's seats; the mesic band
        // would starve a current sheet.
        "mineral" | "plasmic" => {
            // Furnace seats: a G sun at half orbit, stellar loads
            // 2, 11/5, 12/5 — every one past the mesic ceiling; the
            // encasement shells will answer.
            let albedos = [ratio(1, 2), ratio(9, 20), ratio(2, 5)];
            let drawn = beacon.uniform_index("genesis.furnace.albedo", 3) as usize;
            (StarClass::G, albedos[drawn].clone(), ratio(1, 2))
        }
        _ => {
            // Mesic seats: stellar loads 1, 9/10, 25/36 — every one
            // inside the mesic band (05 §2).
            let seats = [
                (ratio(0, 1), ratio(1, 1)),
                (ratio(1, 10), ratio(1, 1)),
                (ratio(0, 1), ratio(6, 5)),
            ];
            let drawn = beacon.uniform_index("genesis.mesic.seat", 3) as usize;
            (StarClass::G, seats[drawn].0.clone(), seats[drawn].1.clone())
        }
    }
}

impl Egregore {
    /// Found the era: registry entry (join if present), the colony,
    /// the contained egregore actor, the steward entity, and the world's
    /// place on the star-chart.
    pub fn genesis(spec: EgregoreSpec, registry: &Registry, mirror: &Mirror) -> Egregore {
        let _ = registry.create(&spec.civilization);
        // BLOCKER A1 (the G0 persistence seam, CLOSED 2026-07-30):
        // the boot RESUMES before it founds. A verifying resume
        // carrier means this world already lived — its receipts,
        // techs, famines, keys, and trades continue; a missing or
        // broken carrier founds fresh (a broken one rotates aside as
        // the mirror's divergence law already provides). NPC progress
        // survives every restart without an operator fold.
        let carrier = mirror.world_dir(&spec.world_name).join("chronicle.tlv");
        if let Ok(chronicle) = cfe::persist::load(&carrier) {
            let world = World::resume(chronicle);
            if world.orb().is_some() {
                let handle = format!("{}-conductor", spec.civilization.sigil);
                let by_handle = |name: &str| {
                    world
                        .committed()
                        .entities
                        .iter()
                        .find(|held| held.handle == name)
                        .map(|held| held.entity_id)
                };
                if let (Some(automaton), Some(steward)) =
                    (by_handle(&handle), by_handle("steward"))
                {
                    // MESH-1b: the walk decides whose pen this is.
                    // Lawful conductor hand → held and stamped from
                    // here on; a pen out across a restart STAYS out;
                    // pre-mesh None stays unheld (law starts at the
                    // first seal); a violating spine waits penned —
                    // writes never land on a refused chain.
                    let mut world = world;
                    let penned = match mesh::authority(world.chronicle()) {
                        Ok(Some(hand)) if hand == cfe::vrp::CONDUCTOR_HAND => {
                            world.hold_pen(Some(cfe::vrp::CONDUCTOR_HAND));
                            false
                        }
                        Ok(Some(_)) | Err(_) => true,
                        Ok(None) => false,
                    };
                    let _ = mirror.chart_world(
                        &spec.world_name,
                        (&spec.position.0, &spec.position.1, &spec.position.2),
                        world.gauge(),
                    );
                    // The forge state is not yet receipted — it
                    // reconstructs from the solves artifacts (an
                    // honest approximation until solves become
                    // receipts): three confirms promote a rung.
                    let solved = std::fs::read_dir(
                        mirror.world_dir(&spec.world_name).join("solves"),
                    )
                    .map(|entries| {
                        entries
                            .filter_map(|e| e.ok())
                            .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
                            .count() as u32
                    })
                    .unwrap_or(0);
                    let level = (1 + solved / 3).min(5);
                    // One divergence per era: a resumed continuation
                    // is the same era — the receipt remembers.
                    let spawned = world.chronicle().receipts().iter().any(|receipt| {
                        matches!(
                            receipt.event,
                            cfe::dop::ClosureEvent::Diverged { .. }
                        )
                    });
                    return Egregore {
                        spec,
                        world,
                        automaton,
                        steward,
                        level,
                        streak: solved % 3,
                        solved,
                        spawned,
                        penned,
                    };
                }
            }
        }
        let opening = spec
            .gauge
            .clone()
            .map(cfe::world::wrap_gauge)
            .unwrap_or_else(|| ratio(0, 1));
        let _ = mirror.chart_world(
            &spec.world_name,
            (&spec.position.0, &spec.position.1, &spec.position.2),
            // AEON-1b: a divergence branch charts at its cleaved key;
            // everyone else founds neutral and G moves by receipt.
            &opening,
        );
        let mut world = World::new();
        world.found(&spec.colony).expect("a fresh world founds");
        if opening != ratio(0, 1) {
            world.shift_gauge(opening.clone());
        }
        let automaton = world
            .register_agent(&format!("{}-conductor", spec.civilization.sigil))
            .expect("registers");
        let steward = world.register_agent("steward").expect("registers");
        for scope in [Scope::Build, Scope::Raze, Scope::Collect] {
            world.grant(automaton, scope).expect("grants the automaton");
            world.grant(steward, scope).expect("grants the steward");
        }
        // PRACTICE-1 (operator-ruled: NPCs adopt at genesis): the
        // disposition names the founding discipline — expansionist
        // keeps the Vigil, epicurean raises the Icon, empiricist keeps
        // the Fast. Receipted like every deed; the Hoard and the
        // Athanor are earned choices, never seeded.
        let founding_practice = match spec.civilization.disposition.as_str() {
            "expansionist" => Some(3),
            "epicurean" => Some(2),
            "empiricist" => Some(1),
            _ => None,
        };
        if let Some(practice) = founding_practice {
            let _ = world.adopt_practice(practice);
        }
        // BOT-THERM: thermal genesis — every egregore world founds under a
        // REAL sun. Spec-pinned fields rule; absent ones derive from
        // the registry axes, beacon-seeded by the world's own genesis
        // receipt digest (the chronicle-seeded discipline, CHARTER §9).
        let digest = world
            .chronicle()
            .receipts()
            .last()
            .expect("genesis receipts exist")
            .own
            .0;
        let beacon = Beacon::from_digest(digest);
        let (derived_star, derived_albedo, derived_orbit) =
            derived_siting(&spec.civilization, &beacon);
        let class = spec.star.unwrap_or(derived_star);
        let albedo = spec.albedo.clone().unwrap_or(derived_albedo);
        let orbit = spec.orbit.clone().unwrap_or(derived_orbit);
        world.commission_star(class).expect("a fresh world has no sun yet");
        // An unlawful operator-pinned siting (albedo ≥ 1, orbit ≤ 0)
        // leaves the orb thermally DORMANT (the engine's opt-in guard)
        // instead of killing the board; derived seats are always lawful.
        let _ = world.site_planet(cfe::world::FIRST_ORB, albedo, orbit);
        Egregore {
            spec,
            world,
            automaton,
            steward,
            level: 1,
            streak: 0,
            solved: 0,
            spawned: false,
            penned: false,
        }
    }

    /// The policy's build decision — at most one zone per beat, never
    /// during famine, always through the contained actor. Industry is
    /// GATED ON RESEARCH for every temperament: smelters need rung 3,
    /// synthesizers rung 5.
    /// RQP/FUNCTION wiring (operator directive 2026-07-30): the
    /// Pathfinder directs the drills. A mature colony (rung 5 — the
    /// synthesizer tier that can have refined a crystal) pays the
    /// scan's crystal draw; the CrystalDrawn receipt IS the survey
    /// record, and placement follows the reading: a single-orb world
    /// CHARTERS the surveyed sector (P1) and opens its first drill; a
    /// multi-orb world places the next extractor on the scan's first
    /// resonant orb with room to grow. Unaffordable or starving, the
    /// survey idles lawfully and siting draws rule as before.
    fn survey_step(&mut self) {
        if self.level < 5 {
            return;
        }
        if self.world.orb().is_none_or(|orb| !orb.famine.is_empty()) {
            return;
        }
        let primary = cfe::world::FIRST_ORB;
        let Ok(read) = self.world.scan_as(self.automaton, primary, primary) else {
            return; // no crystal, no survey — the draw gates the lens
        };
        let orbs: Vec<u64> = self.world.committed().orbs.iter().map(|o| o.orb_id).collect();
        if orbs.len() == 1 {
            // The expansion: the survey confirms the home sigil and
            // the charter plants its next sector — reserves permitting
            // (20 iron + 10 biomass move at the charter, P1).
            if banked(&self.world, Resource::Iron) >= ratio(40, 1)
                && banked(&self.world, Resource::Biomass) >= ratio(20, 1)
            {
                let site_name = format!("{}-survey", self.spec.colony);
                if let Ok(site) = self.world.chart_as(self.automaton, &site_name, primary) {
                    let _ = self.world.zone_as(self.automaton, site, 0, ZoneKind::Extractor);
                }
            }
            return;
        }
        // Guided drilling: the first resonant hit that is not the
        // prober and still has room takes the next drill.
        let room_of = |orb_id: u64| {
            self.world
                .orb_by_id(orb_id)
                .map(|orb| orb.zones.len())
                .unwrap_or(usize::MAX)
        };
        let primary_zones = room_of(primary);
        let Some(&target) = read
            .hits
            .iter()
            .find(|id| **id != primary && room_of(**id) < primary_zones)
        else {
            return;
        };
        let taken: Vec<u16> = self
            .world
            .orb_by_id(target)
            .map(|orb| orb.zones.iter().map(|z| z.tile).collect())
            .unwrap_or_default();
        let Some(tile) = (0..64u16).find(|t| !taken.contains(t)) else {
            return;
        };
        let _ = self.world.zone_as(self.automaton, target, tile, ZoneKind::Extractor);
    }

    fn build_step(&mut self) {
        let world = &mut self.world;
        if world.orb().is_none_or(|orb| !orb.famine.is_empty()) {
            return; // starving colonies build nothing
        }
        let (ext, hab) = (
            zone_count(world, ZoneKind::Extractor),
            zone_count(world, ZoneKind::Habitat),
        );
        let iron = banked(world, Resource::Iron);
        let bio = banked(world, Resource::Biomass);
        let alloy = banked(world, Resource::Alloy);
        let total = world.orb().map(|o| o.zones.len()).unwrap_or(0);
        let smelters = zone_count(world, ZoneKind::Smelter);

        // The earned tech tree.
        let can_smelt = self.level >= 3;
        let can_synth = self.level >= 5;

        // Unlocked industry takes priority for every temperament —
        // that is what research is FOR. SCIENCE-1: only inside the
        // line's OWN science (relevance table) — a plasmic line never
        // forges iron; its advancement crosses by joint venture.
        let metabolism = self.spec.civilization.metabolism.clone();
        let industrial: Option<ZoneKind> = if can_synth
            && alloy >= ratio(20, 1)
            && bio >= ratio(15, 1)
            && industry_relevant(&metabolism, ZoneKind::Synthesizer)
        {
            Some(ZoneKind::Synthesizer)
        } else if can_smelt
            && smelters < 3
            && iron >= ratio(30, 1)
            && bio >= ratio(10, 1)
            && industry_relevant(&metabolism, ZoneKind::Smelter)
        {
            Some(ZoneKind::Smelter)
        } else {
            None
        };

        let settlement: Option<ZoneKind> = match self.spec.policy {
            Policy::Aggressive => {
                // One extractor SHORT of the knife-edge: a standing
                // +1/tick biomass surplus, so industry stays fundable.
                if total >= 16 {
                    None
                } else if ext + 1 < 2 * hab.max(1) && iron >= ratio(10, 1) {
                    Some(ZoneKind::Extractor)
                } else if bio >= ratio(5, 1) {
                    Some(ZoneKind::Habitat)
                } else {
                    None
                }
            }
            Policy::Steady => {
                if total >= 12 {
                    None
                } else if ext <= hab && iron >= ratio(20, 1) {
                    Some(ZoneKind::Extractor)
                } else if hab < ext && bio >= ratio(10, 1) {
                    Some(ZoneKind::Habitat)
                } else {
                    None
                }
            }
            Policy::Industry => {
                if total >= 20 {
                    None
                } else if ext < 2 * hab.max(1) && iron >= ratio(25, 1) {
                    Some(ZoneKind::Extractor)
                } else if bio >= ratio(10, 1) {
                    Some(ZoneKind::Habitat)
                } else {
                    None
                }
            }
        };

        // BOT-THERM: thermal temperament. A Furnace-sited egregore answers
        // rising heat with encasement shells BEFORE anything else —
        // minted joules are what the regime is FOR (05 §1.2); a
        // Frost-sited egregore raises its sink charter (one cryo-lattice
        // array, 05 §1.4), then holds every further build against the
        // joule runway — famine avoidance, honestly allowed to fail
        // (a freeze-lock is legitimate drama, not a bug). Mesic-sited
        // and unsited egregores keep their temperament untouched.
        let affords = |world: &World, kind: ZoneKind| {
            kind.cost()
                .into_iter()
                .all(|(resource, need)| banked(world, resource) >= need)
        };
        let wanted: Option<ZoneKind> =
            match regime(world, &self.spec.civilization.metabolism) {
            Some((Regime::Furnace, t4)) => {
                let shells = zone_count(world, ZoneKind::Encasement) as i64;
                // Famine-free here (guarded above), so every zone is
                // active and this factory sum matches the tick law's.
                let factory = world
                    .orb()
                    .map(|orb| {
                        orb.zones
                            .iter()
                            .fold(ratio(0, 1), |sum, zone| sum + zone.kind.factory_heat())
                    })
                    .unwrap_or_else(|| ratio(0, 1));
                let heat_net =
                    t4 / cfe::thermal::thermal_k() + factory - cfe::dop::radiator_base();
                let throughput = cfe::dop::encasement_throughput() * ratio(shells, 1);
                if heat_net > throughput {
                    // The shell answer: joule collection outranks
                    // sprawl — raise the next shell, or HOARD the ore
                    // toward it (expanding deeper into uncaptured heat
                    // only degrades every yield, 05 §1.2). Shells come
                    // from SURPLUS: the colony waits until the shell
                    // AND the next extractor are both funded, so one
                    // purchase cannot flip its compounding arm.
                    let iron_for = |kind: ZoneKind| {
                        kind.cost()
                            .into_iter()
                            .find(|(resource, _)| *resource == Resource::Iron)
                            .map(|(_, need)| need)
                            .unwrap_or_else(|| ratio(0, 1))
                    };
                    let shell_line =
                        iron_for(ZoneKind::Encasement) + iron_for(ZoneKind::Extractor);
                    (iron >= shell_line).then_some(ZoneKind::Encasement)
                } else if can_smelt
                    && smelters < 3
                    && iron >= ratio(30, 1)
                    && bio < ratio(10, 1)
                {
                    // Industry outranks new settlement on a Furnace
                    // seat: when ONLY the biomass line still blocks an
                    // unlocked smelter, hold the rations for it — a
                    // habitat spent now would push the forge past the
                    // horizon again.
                    industrial.or(match settlement {
                        Some(ZoneKind::Habitat) => None,
                        held => held,
                    })
                } else {
                    industrial.or(settlement)
                }
            }
            Some((Regime::Frost, _)) => {
                if zone_count(world, ZoneKind::CryoArray) == 0 {
                    // The sink charter: a Frost colony exists to bank
                    // the cold; its first array is founded on the
                    // founding iron, before any joule flows.
                    affords(world, ZoneKind::CryoArray).then_some(ZoneKind::CryoArray)
                } else if joule_runway_allows(&banked(world, Resource::Joule), total) {
                    industrial.or(settlement)
                } else {
                    None // the runway is short: STOP expanding
                }
            }
            None => industrial.or(settlement),
        };
        if let (Some(kind), Some(tile)) = (wanted, next_free_tile(world)) {
            let _ = world.zone_as(self.automaton, cfe::world::FIRST_ORB, tile, kind);
        }
    }

    /// Drain the operator's inbox: each directive applies as the
    /// steward entity (scope-checked in-engine) or refuses; archived
    /// either way, never deleted.
    fn drain_inbox(&mut self, mirror: &Mirror) {
        let Ok(pending) = mirror.pending_directives(&self.spec.world_name) else {
            return;
        };
        for item in pending {
            let applied = self.apply_directive(&item.directive);
            let _ = mirror.archive(
                &item,
                if applied {
                    Disposition::Applied
                } else {
                    Disposition::Refused
                },
            );
        }
    }

    fn apply_directive(&mut self, directive: &Directive) -> bool {
        let parse_tile = |word: &str| -> Option<u16> {
            if let Some((r, c)) = word.split_once(':') {
                let (r, c): (u16, u16) = (r.parse().ok()?, c.parse().ok()?);
                (r < 8 && c < 8).then_some(r * 8 + c)
            } else {
                word.parse().ok().filter(|t| *t < 64)
            }
        };
        let parse_kind = |word: &str| match word {
            "ext" => Some(ZoneKind::Extractor),
            "hab" => Some(ZoneKind::Habitat),
            "smelt" => Some(ZoneKind::Smelter),
            "synth" => Some(ZoneKind::Synthesizer),
            _ => None,
        };
        match (directive.verb.as_str(), directive.args.as_slice()) {
            ("zone", [tile, kind]) => match (parse_tile(tile), parse_kind(kind)) {
                (Some(tile), Some(kind)) => self.world.zone_as(self.steward, cfe::world::FIRST_ORB, tile, kind).is_ok(),
                _ => false,
            },
            ("raze", [tile]) => match parse_tile(tile) {
                Some(tile) => self.world.raze_as(self.steward, cfe::world::FIRST_ORB, tile).is_ok(),
                None => false,
            },
            ("collect", []) => self.world.collect_as(self.steward, cfe::world::FIRST_ORB).is_ok(),
            // #40: the t0 gift petition — a receipted grant under
            // steward containment; the crown law holds in the engine.
            ("gift", [condition]) => match condition.parse::<u8>() {
                Ok(condition) => self
                    .world
                    .grant_tech_as(self.steward, cfe::world::FIRST_ORB, condition)
                    .is_ok(),
                Err(_) => false,
            },
            _ => false,
        }
    }

    /// The research arm: the world's own latest receipt digest seeds
    /// one forge attempt at the current rung. Three confirms promote.
    ///
    /// PERCEPTION ROUTES THE RESEARCH (the axis has teeth): a
    /// volumetric civilization natively attempts VOLUMETRIC tasks —
    /// stacked grids, stack operators — while planar and harmonic
    /// civilizations work the planar forge (the harmonic carrier is a
    /// named gap; it lands with its own vocabulary, not as a skin).
    fn research_step(&mut self, mirror: &Mirror) {
        let Some(receipt) = self.world.chronicle().receipts().last() else {
            return;
        };
        let digest = receipt.own.0;
        let solved = if self.spec.civilization.perception == "volumetric" {
            let difficulty = solver::VolDifficulty::level(self.level);
            let task = solver::forge_volume(digest, difficulty);
            match solver::solve_volume(&task, difficulty.word_len) {
                solver::VolumeSolveRead::Solved {
                    program,
                    matches_expected: Some(true),
                    certificate,
                    ..
                } => {
                    let word: Vec<String> = program.iter().map(|op| op.describe()).collect();
                    Some((task.name, word.join(" → "), hex(&certificate)))
                }
                _ => None,
            }
        } else {
            let difficulty = Difficulty::level(self.level);
            let task = forge_task(digest, difficulty);
            match solve(&task, difficulty.word_len) {
                SolveRead::Solved {
                    program,
                    matches_expected: Some(true),
                    certificate,
                    ..
                } => {
                    let word: Vec<String> = program.iter().map(|op| op.describe()).collect();
                    Some((task.name, word.join(" → "), hex(&certificate.output)))
                }
                _ => None,
            }
        };
        match solved {
            Some((name, word, cert_hex)) => {
                let _ = mirror.record_solve(
                    &self.spec.world_name,
                    &name,
                    &word,
                    &cert_hex,
                    self.level,
                );
                self.solved += 1;
                self.streak += 1;
                if self.streak >= 3 && self.level < 5 {
                    self.level += 1;
                    self.streak = 0;
                }
            }
            None => self.streak = 0,
        }
    }

    /// DIVERGENCE (AEON-1b, 11 §3): at rung 5 with 20 confirmed
    /// solves — sustained pressure — the line CLEAVES into two PEER
    /// branches: no parent, no child. The origin's own key moves +delta
    /// and receipts `Diverged` naming the departing branch; the branch
    /// founds its own world at the opposite phase (−delta, wrapped).
    /// The mutation stays chronicle-seeded: the line's own history
    /// decides where its branches land on the manifold. The lineage
    /// pointer records LINE membership (LINEAGE-1), never parenthood.
    fn divergence_step(&mut self, registry: &Registry) -> Option<EgregoreSpec> {
        if self.spawned || self.level < 5 || self.solved < 20 {
            return None;
        }
        let receipt = self.world.chronicle().receipts().last()?;
        let beacon = Beacon::from_digest(receipt.own.0);
        let parent = &self.spec.civilization;
        // The cleave IS a key change (11 §3): delta in 1/12..=5/12,
        // beacon-drawn; the branches take opposite gauge phases.
        let delta = ratio(1 + beacon.uniform_index("diverge.delta", 5) as i64, 12);
        let origin_key = self.world.gauge().clone();

        // Which axis mutates, and to what (never to itself).
        let mutate = |current: &str, options: &[&str], domain: &str| -> String {
            let others: Vec<&&str> = options.iter().filter(|o| **o != current).collect();
            others[beacon.uniform_index(domain, others.len() as u64) as usize].to_string()
        };
        let axis = beacon.uniform_index("speciation.axis", 3);
        let (mut disposition, mut metabolism, mut perception) = (
            parent.disposition.clone(),
            parent.metabolism.clone(),
            parent.perception.clone(),
        );
        match axis {
            0 => disposition = mutate(&disposition, &DISPOSITIONS, "speciation.disposition"),
            1 => metabolism = mutate(&metabolism, &METABOLISMS, "speciation.metabolism"),
            _ => perception = mutate(&perception, &PERCEPTIONS, "speciation.perception"),
        }

        // THEMED NAMING (operator ruling): the child's name derives
        // from the parent's dominant CAPABILITIES — metabolism names
        // the head, perception names the tail. Fresh two-part names,
        // never concatenations of ancestry (lineage lives in the
        // registry field, not in ever-growing strings).
        let heads: &[&str] = match parent.metabolism.as_str() {
            "mineral" => &["ember", "slag", "basalt", "vein", "cinder", "forge"],
            _ => &["bloom", "grove", "reef", "moss", "fern", "spore"],
        };
        let tails: &[&str] = match perception.as_str() {
            "harmonic" => &["choir", "chord", "hum", "call"],
            "volumetric" => &["vault", "deep", "well", "hollow"],
            _ => &["reach", "field", "span", "march"],
        };
        // Deterministic collision walk: start at the drawn combo,
        // scan forward; if every themed name is taken, the line waits.
        let start = beacon.uniform_index("speciation.name", (heads.len() * tails.len()) as u64)
            as usize;
        let mut chosen: Option<(String, &str, &str)> = None;
        for offset in 0..heads.len() * tails.len() {
            let k = (start + offset) % (heads.len() * tails.len());
            let (head, tail) = (heads[k / tails.len()], tails[k % tails.len()]);
            let sigil = format!("{head}{tail}");
            if registry.load(&sigil).is_none() {
                chosen = Some((sigil, head, tail));
                break;
            }
        }
        let (sigil, head, tail) = chosen?;
        let mut name = sigil.clone();
        name[..1].make_ascii_uppercase();

        let root_species = parent
            .species
            .strip_suffix(" (descendant)")
            .unwrap_or(&parent.species);
        // A PEER, not a child: the species is the LINE's species,
        // whole — no "(descendant)" suffix, no "child of" (11 §3
        // retired the daughter).
        let branch = Civilization {
            name,
            sigil: sigil.clone(),
            species: root_species.to_string(),
            // The cleave keeps the clade — one line, one taxon.
            clade: parent.clade.clone(),
            motto: format!("one line, two songs — cleaved from {}", parent.name),
            disposition,
            metabolism,
            perception,
            founder: "conductor".into(),
            lineage: vec![parent.sigil.clone()],
            station: None,
        };
        registry.create(&branch).ok()?;
        self.spawned = true;
        // The origin's key moves and the cleave is receipted, naming
        // the departing branch (tag 43).
        self.world.diverge(&sigil, delta.clone());

        // The daughter settles ADJACENT: parent position plus a small
        // beacon-drawn displacement in the exact rationals per spatial axis (half-unit
        // grain, never the zero vector).
        let step = |domain: &str| -> Ratio {
            ratio(beacon.uniform_index(domain, 9) as i64 - 4, 2)
        };
        let (mut dx, dy, dz) = (step("chart.dx"), step("chart.dy"), step("chart.dz"));
        if dx == ratio(0, 1) && dy == ratio(0, 1) && dz == ratio(0, 1) {
            dx = ratio(1, 1);
        }
        Some(EgregoreSpec {
            civilization: branch,
            // Fresh themed world and planet — ancestry is a registry
            // fact and a star-chart adjacency, never a name chain.
            world_name: format!("{head}-{tail}"),
            planet: format!("{tail}fall"),
            colony: format!("{sigil}-landing"),
            policy: self.spec.policy,
            position: (
                &self.spec.position.0 + dx,
                &self.spec.position.1 + dy,
                &self.spec.position.2 + dz,
            ),
            // BOT-THERM: a branch's seat is never inherited — it is
            // drawn from its OWN genesis chronicle (the mutated axes
            // may prefer another regime entirely).
            star: None,
            albedo: None,
            orbit: None,
            // The opposite phase of the cleave (11 §3), wrapped.
            gauge: Some(cfe::world::wrap_gauge(origin_key - delta)),
        })
    }

    /// One full heartbeat. A speciation returns the newborn's spec —
    /// the era loop gives it a world of its own.
    pub fn cycle(&mut self, mirror: &Mirror, registry: &Registry) -> Option<EgregoreSpec> {
        self.drain_inbox(mirror);
        self.build_step();
        self.survey_step();
        self.world.tick();
        // Every settled orb collects — a chartered survey site left
        // uncollected would starve at its own upkeep.
        let orbs: Vec<u64> = self.world.committed().orbs.iter().map(|o| o.orb_id).collect();
        for orb in orbs {
            let _ = self.world.collect_as(self.automaton, orb);
        }
        self.research_step(mirror);
        let newborn = self.divergence_step(registry);
        // THE RESUME CARRIER (task 25, 2026-07-30): every egregore
        // world syncs its tlv chronicle beside the mirror — an
        // append-only resume copy (cfe::persist grows the file, never
        // rewrites). The player's OBSERVING hand can resume a copy;
        // the WRITING hand stays refused at the CLI (two writers on
        // one clock is the split-brain NS-5 exists to arbitrate) —
        // intervention still goes through the steward inbox.
        let _ = cfe::persist::sync(
            self.world.chronicle(),
            &mirror
                .world_dir(&self.spec.world_name)
                .join("chronicle.tlv"),
        );
        let _ = mirror.record(
            &self.spec.world_name,
            &self.spec.planet,
            &self.spec.civilization.sigil,
            &self.world,
        );
        newborn
    }
}

/// MESH-1b (#2, the consumer slice): the conductor's side of the
/// receipted handoff. `yield <world> <seat>` lines in the control
/// file ask for the pen; the beat boundary is the one lawful moment
/// to seal it over (no beat is mid-flight). Consumed lines leave the
/// file — a refusal is answered once, never retried forever. Penned
/// worlds are then polled on their FILE: when the taking hand yields
/// back, the conductor re-resumes from the receipts (SEAL B — the
/// file is the world) and beats continue.
pub fn pen_service(
    worlds: &mut [Egregore],
    registry: &Registry,
    mirror: &Mirror,
    control_path: &Path,
    stamp: u64,
) -> Vec<String> {
    let mut log = Vec::new();
    // The yield door.
    if let Ok(text) = std::fs::read_to_string(control_path) {
        let mut kept = Vec::new();
        let mut consumed = false;
        for line in text.lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            let ["yield", world_name, seat] = words[..] else {
                kept.push(line.to_string());
                continue;
            };
            consumed = true;
            let hand = cfe::vrp::hand_of(seat);
            match worlds
                .iter_mut()
                .find(|e| e.spec.world_name == world_name)
            {
                Some(egregore) if !egregore.penned => {
                    match mesh::yield_authority(
                        egregore.world.chronicle_mut(),
                        cfe::vrp::CONDUCTOR_HAND,
                        hand,
                        stamp,
                    ) {
                        Ok(sequence) => {
                            egregore.world.hold_pen(None);
                            let _ = cfe::persist::sync(
                                egregore.world.chronicle(),
                                &mirror.world_dir(world_name).join("chronicle.tlv"),
                            );
                            let _ = mirror.record(
                                &egregore.spec.world_name,
                                &egregore.spec.planet,
                                &egregore.spec.civilization.sigil,
                                &egregore.world,
                            );
                            egregore.penned = true;
                            log.push(format!(
                                "HANDOFF — {world_name}: the pen passes to {seat} (seal {sequence}); beats wait"
                            ));
                        }
                        Err(refused) => log.push(format!(
                            "HANDOFF REFUSED — {world_name}: {refused:?}"
                        )),
                    }
                }
                Some(_) => log.push(format!(
                    "HANDOFF REFUSED — {world_name}: the pen is already out"
                )),
                None => log.push(format!(
                    "HANDOFF REFUSED — {world_name}: no such world on this board"
                )),
            }
        }
        if consumed {
            let mut out = kept.join("\n");
            if !out.is_empty() {
                out.push('\n');
            }
            let _ = std::fs::write(control_path, out);
        }
    }
    // The return poll.
    for egregore in worlds.iter_mut().filter(|e| e.penned) {
        let carrier = mirror
            .world_dir(&egregore.spec.world_name)
            .join("chronicle.tlv");
        let Ok(chronicle) = cfe::persist::load(&carrier) else {
            continue;
        };
        if mesh::authority(&chronicle) == Ok(Some(cfe::vrp::CONDUCTOR_HAND)) {
            let name = egregore.spec.world_name.clone();
            *egregore = Egregore::genesis(egregore.spec.clone(), registry, mirror);
            log.push(format!("HANDOFF — {name}: the pen returns; beats resume"));
        }
    }
    log
}

/// GALAXY-1b (13_GALAXY §3, §5): the year-voice observation sweep.
/// Every crossed, charted world OBSERVES; an anchor past the frontier
/// creates the galaxy it observes, executed as the progenitor's deed
/// (12 §2a — the board-maker function). Idempotent by geometry: a
/// created galaxy encloses its observer, so the next sweep reads
/// inside-reach and creates nothing.
pub fn observe_crossings(
    worlds: &[Egregore],
    registry: &Registry,
    mirror: &Mirror,
    stamp: u64,
) -> Vec<Galaxy> {
    let mut created = Vec::new();
    for egregore in worlds {
        if !egregore.world.has_crossed() {
            continue;
        }
        let Some(((x, y, z), _)) = mirror.charted_position(&egregore.spec.world_name) else {
            continue;
        };
        let creator = galaxy_creator(registry, &egregore.spec.civilization);
        if let Ok(Some(galaxy)) =
            mirror.observe_galaxy(&egregore.spec.civilization.sigil, &creator, (&x, &y, &z), stamp)
        {
            created.push(galaxy);
        }
    }
    created
}

/// The creator seat (13_GALAXY §3): the observer's lineage head when
/// it holds a progenitor seat; else the first seated progenitor of the
/// observer's domain (the founding head stands creator for a headless
/// line — lawful, 12 §5); else the conductor — tier 0's own operations
/// ARE the progenitors' deeds, named (12 §2a).
fn galaxy_creator(registry: &Registry, civilization: &Civilization) -> String {
    if let Some(head) = registry.resolve_head(&civilization.sigil) {
        if registry.load(&head).and_then(|c| c.station).as_deref() == Some("progenitor") {
            return head;
        }
    }
    let domain = lineage_domain(&civilization.metabolism, None);
    registry
        .list()
        .into_iter()
        .find(|seated| {
            seated.station.as_deref() == Some("progenitor")
                && lineage_domain(&seated.metabolism, None) == domain
        })
        .map(|seated| seated.sigil)
        .unwrap_or_else(|| "conductor".into())
}

/// Run one era: CONVERGE the archived eras (07 §1.6), then genesis for
/// every egregore, then `cycles` heartbeats; daughters born mid-era join the
/// beat. Returns the live worlds.
pub fn run_era(var_root: &str, cycles: u32) -> Vec<Egregore> {
    let registry = Registry::at(format!("{var_root}/registry"));
    let mirror = Mirror::at(format!("{var_root}/worlds"));
    let eras = mirror::converge::default_era_roots(var_root);
    let epoch_base = mirror::converge::converge(&mirror, &eras)
        .map(|read| read.epoch_base)
        .unwrap_or(0);
    // 11 §5.2: the live aeon's G-sheet (see main.rs — same law).
    let live_base = cfe::world::wrap_gauge(ratio(eras.len() as i64, 12));
    let mut worlds: Vec<Egregore> = egregores::founding_egregores()
        .into_iter()
        .map(|mut spec| {
            let opening = spec.gauge.take().unwrap_or_else(|| ratio(0, 1));
            spec.gauge = Some(cfe::world::wrap_gauge(opening + live_base.clone()));
            Egregore::genesis(spec, &registry, &mirror)
        })
        .collect();
    for beat in 0..cycles {
        // TUniversal T = epoch base + this era's beat (07 §1.3).
        let _ = mirror.set_universal_clock(epoch_base + u64::from(beat) + 1);
        let mut newborns = Vec::new();
        for egregore in &mut worlds {
            if let Some(spec) = egregore.cycle(&mirror, &registry) {
                newborns.push(spec);
            }
        }
        for spec in newborns {
            worlds.push(Egregore::genesis(spec, &registry, &mirror));
        }
        // JOURNAL-1 (08 §3): on year boundaries every civilization
        // speaks — the mirror renders journals + wall, a REFLECTION
        // of the chronicle, never a state change.
        if (u64::from(beat) + 1).is_multiple_of(mirror::journal::YEAR_TICKS) {
            for egregore in &worlds {
                let _ = mirror.record_journal(
                    &egregore.spec.world_name,
                    &egregore.world,
                    &mirror::journal::Voice::of(&egregore.spec.civilization),
                );
            }
            // GALAXY-1b: the observation sweep rides the year voice
            // (11 §1 — board-scale shifts read yearly).
            let _ = observe_crossings(&worlds, &registry, &mirror, epoch_base + u64::from(beat) + 1);
        }
    }
    worlds
}
