//! N-CONDUCTOR + N-LINEAGE — the heartbeat and the lineage graph.
//!
//! Scripted civilizations play their colonies autonomously, each
//! through a CONTAINED AGENT (NS-6: granted scopes, every receipt
//! attributed `by#agent` — automation on a leash). Each heartbeat a
//! world: drains the steward inbox, plays its policy, ticks,
//! collects, grinds the forge (its OWN latest receipt digest seeds
//! the puzzle — CHARTER §9), and keeps the ledger.
//!
//! **TECH IS EARNED (the pyramid loop closed):** smelters unlock at
//! forge rung 3, synthesizers at rung 5 — for every temperament.
//! Research depth is the tech tree; surplus funds search; search
//! unlocks industry; industry funds deeper search.
//!
//! **GROWTH PER CIV IS A MINT:** a civilization at rung 5 with 20
//! confirmed solves CLEAVES once per era into TWO PEER BRANCHES —
//! irreversible dimensional growth, exactly `physics::FlowEvent::Mint`'s law worn
//! as gameplay. The mutation is chronicle-seeded: the parent's latest
//! receipt digest draws WHICH axis mutates and what the child is
//! called — the parent's own history decides what its children
//! become. Lineage is registry-recorded now, receipted at G1. A
//! branch founds a NEW world (binding #1: one chronicle per world —
//! a shared-chronicle counterpart planet waits on the multi-colony
//! engine).
//!
//! **LORE LIVES IN DATA, NOT SOURCE:** polity specs load from
//! `<var>/polities.json` when present; `write_default_polities` seeds that
//! file once so even the founding NPCs are operator-editable. NPC
//! entries carry `founder: "conductor"` — the observer marks them ⚙.
//!
//! **THE AUTARKIES FEEL THE STAR (BOT-THERM):** genesis commissions
//! every polity world's sun and sites its planet — spec-pinned, or
//! derived from the registry axes through the chronicle beacon (the
//! civilization's own genesis receipts choose its sky). Furnace-sited
//! polities answer heat with encasement shells and mint joules;
//! Frost-sited polities raise a cryo-lattice array, bank sink, and hold
//! expansion against the joule runway — honestly allowed to
//! freeze-lock; mesic polities keep their temperament untouched.
//!
//! Everything is deterministic: same era length → byte-identical
//! receipts. HONEST SEAM: worlds live in process memory; reload from
//! the exported chronicle is the G0 persistence seam (coder-02).

use cfe::dop::{Resource, ZoneKind};
use cfe::ipsal::Scope;
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::World;
use ledger::galaxy::Galaxy;
use ledger::registry::{lineage_domain, Civilization, Registry, DISPOSITIONS, METABOLISMS, PERCEPTIONS};
use ledger::{Directive, Disposition, Ledger};
use physics::Beacon;
use solver::{forge_task, solve_consulted, Difficulty, SolveRead};
use std::path::Path;

mod polities;
pub use polities::{founding_polities, load_polities, write_default_polities};

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
    let roster = root.join("polities.json");
    if roster.is_file() {
        // The archived copy carries the live name. (It was written as
        // `bots.json` under two earlier roster names; nothing reads it
        // back, so the archive name simply tracks the current one.)
        std::fs::copy(&roster, archive.join("polities.json"))?;
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

/// The reference model posts in WHOLE LOTS: amounts floor to the
/// 1/64 grid the melt law already speaks (MELT_PER_UNIT). This is
/// layer-2 prudence, not law — it keeps every book amount's
/// denominator a small power of 64, so exact arithmetic stays cheap
/// across an open, no-singles book (measured 2026-07-31: unquantized
/// compounding rationals stalled the beat by ~6 crossings deep).
fn lot(amount: &Ratio) -> Ratio {
    use num_bigint::BigInt;
    let grid = BigInt::from(64);
    Ratio::new(amount.numer() * &grid / amount.denom(), grid)
}

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
                    let give = lot(&half(sink));
                    let want = lot(&steep_want(&give, severity));
                    if give > ratio(0, 1) && want > ratio(0, 1) {
                        return Some((Resource::Sink, give, Resource::Joule, want));
                    }
                }
            }
            (Regime::Furnace, _) => {
                let joules = orb.banked_amount(Resource::Joule);
                if joules > ratio(TARGET_RESERVE.0, TARGET_RESERVE.1) {
                    let give = lot(&half(joules.clone()));
                    let want = lot(&relaxed_want(&give, &joules));
                    if give > ratio(0, 1) && want > ratio(0, 1) {
                        return Some((Resource::Joule, give, Resource::Sink, want));
                    }
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
            let give = lot(&half(stock));
            let want = lot(&steep_want(&give, severity));
            if give > ratio(0, 1) && want > ratio(0, 1) {
                return Some((*biggest, give, famined, want));
            }
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
        let give = lot(&half(stock.clone()));
        let want = lot(&relaxed_want(&give, &stock));
        if give > ratio(0, 1) && want > ratio(0, 1) {
            return Some((*biggest, give, *leanest, want));
        }
        return None;
    }
    None
}

/// OFFER-1a: one market beat over the whole board — post fresh
/// intents (one open offer per world; the offer id is the chronicle
/// length at posting, deterministic), then clear the FIRST crossing
/// (price-time priority inside `clear_trade`). Returns the cleared
/// pairs (world names) for the narrator line.
pub fn market_step(worlds: &mut [Polity], beat: u64) -> Vec<(String, String)> {
    // Post intents. NO SINGLES (ruling 2026-07-31): the book is
    // open-ended — a fresh intent posts BESIDE what rests; only an
    // identical pair (same give, same want) holds its place.
    for polity in worlds.iter_mut() {
        if polity.penned {
            continue;
        }
        if let Some((give, give_amount, want, want_amount)) =
            trade_intent(&polity.world, &polity.spec.civilization.metabolism)
        {
            if polity
                .world
                .offers()
                .iter()
                .any(|held| held.give == give && held.want == want)
            {
                continue;
            }
            let offer = polity.world.chronicle().receipts().len() as u64;
            let orb = cfe::world::FIRST_ORB;
            let _ = polity
                .world
                .post_offer(orb, give, give_amount, want, want_amount, offer);
        }
    }
    // Clear crossings: EVERY resting combination on every pair — no
    // first-match single. Deal ids stride 4 (haul takes deal/deal+1,
    // the wear-charter deal+2) from a per-beat sequence.
    let mut cleared = Vec::new();
    let mut seq: u64 = 0;
    for a in 0..worlds.len() {
        for b in (a + 1)..worlds.len() {
            let (left, right) = worlds.split_at_mut(b);
            let one = &mut left[a];
            let two = &mut right[0];
            if one.penned || two.penned {
                continue;
            }
            // MARKET-2: the transit's spatial L1 — supply melts with
            // distance; the engine refuses walled pairs itself.
            let distance = spatial_l1(&one.spec.position, &two.spec.position);
            let offers_a: Vec<u64> = one.world.offers().iter().map(|o| o.offer).collect();
            let offers_b: Vec<u64> = two.world.offers().iter().map(|o| o.offer).collect();
            let mut crossed_here = false;
            for offer_a in &offers_a {
                for offer_b in &offers_b {
                    let deal = beat * 100_000 + seq * 4;
                    seq += 1;
                    if cfe::world::clear_trade(
                        &mut one.world,
                        *offer_a,
                        &mut two.world,
                        *offer_b,
                        deal,
                        distance.clone(),
                    )
                    .is_ok()
                    {
                        crossed_here = true;
                        cleared.push((
                            one.spec.world_name.clone(),
                            two.spec.world_name.clone(),
                        ));
                    }
                }
            }
            // ROUTE-1 + NO SINGLES: trade wears the path, and a
            // better lane ANNEXES the pair — chartered when no lane
            // stands, or when the pair's technology now supports a
            // deeper suppression than the best lane held.
            if crossed_here {
                let best_held: Option<Ratio> = one
                    .world
                    .routes()
                    .iter()
                    .filter(|(held, _)| {
                        two.world.routes().iter().any(|(other, _)| other == held)
                    })
                    .map(|(_, eta)| eta.clone())
                    .max();
                let best_grade = one
                    .world
                    .instruments()
                    .iter()
                    .chain(two.world.instruments().iter())
                    .map(|(_, _, grade)| *grade)
                    .max()
                    .unwrap_or(0);
                let eta = ratio(1 + i64::from(best_grade), 8);
                if best_held.is_none_or(|held| eta > held) {
                    let deal = beat * 100_000 + seq * 4;
                    seq += 1;
                    let _ = cfe::world::charter_route(
                        &mut one.world,
                        cfe::world::FIRST_ORB,
                        &mut two.world,
                        cfe::world::FIRST_ORB,
                        eta,
                        deal,
                    );
                }
            }
        }
    }
    cleared
}

/// A build temperament — the only thing that differs between polities.
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
pub struct PolitySpec {
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

/// One live polity world: the sim, its contained actor, its research rung.
pub struct Polity {
    pub spec: PolitySpec,
    pub world: World,
    /// The polity's contained agent (build/raze/collect granted).
    automaton: u64,
    /// The steward's contained entity (drains the operator's inbox).
    steward: u64,
    /// The forge rung this civilization currently attempts.
    pub level: u32,
    /// Confirms at the current rung (3 promote).
    streak: u32,
    /// Confirmed solves this era.
    pub solved: u32,
    /// One CLEAVE per era — growth is rare, not a flood. The cleave
    /// yields two PEER branches (11 §3); this bounds how OFTEN the
    /// line grows, never how many branches a growth produces.
    cleaved: bool,
    /// MESH-1b: the pen is out — beats, saves, and the market wait.
    pub penned: bool,
    /// #36: the world's own chain, held open across beats (opened
    /// lazily at the first chain; re-opening per beat would re-verify
    /// the whole stream every cycle — quadratic over a live era).
    arbor: Option<mesh::arbor::Arbor>,
    /// DIRECTIVE 19 — how many receipts have been threaded through the
    /// judge and into the chain. Every deed past this mark is chained
    /// individually, with its own step, and judged.
    chained: usize,
    /// Deeds the kernel REFUSED, by receipt sequence. Kept rather than
    /// discarded — a refusal the board never sees is the judge speaking
    /// into a void.
    refused: Vec<(u64, kernel::Refused)>,
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

/// BOT-THERM: the thermal regime a polity world lives in. The mesic band
/// is home (temperament unchanged — `None`); above its ceiling the
/// Furnace laws apply, below its floor the Frost laws. Unsited worlds
/// carry no regime (the engine's opt-in guard, repeated here).
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

impl Polity {
    /// Found the era: registry entry (join if present), the colony,
    /// the contained polity actor, the steward entity, and the world's
    /// place on the star-chart.
    pub fn genesis(spec: PolitySpec, registry: &Registry, book: &Ledger) -> Polity {
        let _ = registry.create(&spec.civilization);
        // BLOCKER A1 (the G0 persistence seam, CLOSED 2026-07-30):
        // the boot RESUMES before it founds. A verifying resume
        // carrier means this world already lived — its receipts,
        // techs, famines, keys, and trades continue; a missing or
        // broken carrier founds fresh (a broken one rotates aside as
        // the ledger's divergence law already provides). NPC progress
        // survives every restart without an operator fold.
        let carrier = book.world_dir(&spec.world_name).join("chronicle.tlv");
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
                    // first seal); a violating arbor waits penned —
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
                    let _ = book.chart_world(
                        &spec.world_name,
                        (&spec.position.0, &spec.position.1, &spec.position.2),
                        world.gauge(),
                    );
                    // The forge state is not yet receipted — it
                    // reconstructs from the solves artifacts (an
                    // honest approximation until solves become
                    // receipts): three confirms promote a rung.
                    let solved = std::fs::read_dir(
                        book.world_dir(&spec.world_name).join("solves"),
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
                    let cleaved = world.chronicle().receipts().iter().any(|receipt| {
                        matches!(
                            receipt.event,
                            cfe::dop::ClosureEvent::Diverged { .. }
                        )
                    });
                    return Polity {
                        chained: 0,
                        refused: Vec::new(),
                        spec,
                        world,
                        automaton,
                        steward,
                        level,
                        streak: solved % 3,
                        solved,
                        cleaved,
                        penned,
                        arbor: None,
                    };
                }
            }
        }
        let opening = spec
            .gauge
            .clone()
            .map(cfe::world::wrap_gauge)
            .unwrap_or_else(|| ratio(0, 1));
        let _ = book.chart_world(
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
        // BOT-THERM: thermal genesis — every polity world founds under a
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
        // SITED WITHOUT AN ANGLE, deliberately. Passing ANY slot here
        // makes conductor/tests/survey_pins.rs fail with a live/replay
        // divergence in Crystal — measured at slots 0, 1, 7 and 30, so
        // it is the PRESENCE of the angle and not its value. See
        // `21_KERNEL_PARAMETERS` §17: the capability is landed and
        // witnessed, and the conductor does not use it until #30
        // closes the divergence it exposes.
        let _ = world.site_planet(cfe::world::FIRST_ORB, albedo, orbit, None, None);
        Polity {
            chained: 0,
            refused: Vec::new(),
            spec,
            world,
            automaton,
            steward,
            level: 1,
            streak: 0,
            solved: 0,
            cleaved: false,
            penned: false,
            arbor: None,
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

        // BOT-THERM: thermal temperament. A Furnace-sited polity answers
        // rising heat with encasement shells BEFORE anything else —
        // minted joules are what the regime is FOR (05 §1.2); a
        // Frost-sited polity raises its sink charter (one cryo-lattice
        // array, 05 §1.4), then holds every further build against the
        // joule runway — famine avoidance, honestly allowed to fail
        // (a freeze-lock is legitimate drama, not a bug). Mesic-sited
        // and unsited polities keep their temperament untouched.
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
    fn drain_inbox(&mut self, book: &Ledger) {
        let Ok(pending) = book.pending_directives(&self.spec.world_name) else {
            return;
        };
        for item in pending {
            let applied = self.apply_directive(&item.directive);
            let _ = book.archive(
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
    fn research_step(&mut self, book: &Ledger) {
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
                    // The volumetric tier is DECORATIVE (operator C8):
                    // no volumetric deed mints, so it carries no proof.
                    Some((task.name, word.join(" → "), hex(&certificate), None))
                }
                _ => None,
            }
        } else {
            let difficulty = Difficulty::level(self.level);
            let task = forge_task(digest, difficulty);
            // THE SECTOR DOOR is the live forge path. The search only
            // PROPOSES; nothing crosses until every training pair has
            // been re-derived exactly, which is the same re-derivation
            // `verify_work` performs at the kernel. An unproved
            // candidate reads `Unsolved` and the line simply rests that
            // beat — so the search inside may become as dirty as it
            // likes (float scoring, batching, sampled beam) and nothing
            // downstream changes.
            match solve_consulted(&task, difficulty.word_len) {
                SolveRead::Solved {
                    program,
                    matches_expected: Some(true),
                    certificate,
                    test_output,
                } => {
                    let word: Vec<String> = program.iter().map(|op| op.describe()).collect();
                    let cert_hex = hex(&certificate.output);
                    // THE PROOF, riding WHOLE (the opaque-payload law):
                    // the deed carries what the kernel re-runs.
                    let proof = solver_verify::WorkProof {
                        forge_digest: digest,
                        difficulty,
                        program: program.clone(),
                        output: test_output,
                        certificate,
                    };
                    Some((
                        task.name,
                        word.join(" → "),
                        cert_hex,
                        solver_verify::encode_work(&proof).map(|bytes| (bytes, difficulty)),
                    ))
                }
                _ => None,
            }
        };
        match solved {
            Some((name, word, cert_hex, proof)) => {
                // A1 — CLOSE THE PoUW LOOP. Cognition proved out; now it
                // MINTS. The amount is `kernel::mint_bound` — the ruled
                // ladder read from the same function the judge enforces,
                // never a number chosen here.
                if let Some((work, difficulty)) = proof {
                    let earned = kernel::mint_bound(&difficulty);
                    if earned > cfe::ratio::ratio(0, 1) {
                        let _ = self.world.mint_discovery_as(
                            self.automaton,
                            cfe::world::FIRST_ORB,
                            cfe::dop::Resource::Crystal,
                            earned,
                            work,
                            // The emitter now demands a warrant, and the
                            // kernel's is the only one that reads a
                            // payload. The bound is checked twice by one
                            // function: here and at I4.
                            &kernel::SealedWarrant,
                        );
                    }
                }
                let _ = scribe::record_solve(
                    book,
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
    fn divergence_step(&mut self, registry: &Registry) -> Option<PolitySpec> {
        if self.cleaved || self.level < 5 || self.solved < 20 {
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
        // MULTI-AXIAL (operator ruling 2026-08-02: *unpin unary, and
        // make it multi-axis*). `11_AEON` §3 retired "Charted" for
        // importing terrestrial unary budding — ONE PARENT, MUTATE ONE
        // AXIS — and this draw was still the second half of that: one
        // of three coordinates moved and the other two were held.
        //
        // A cleave is a WALK ON THE MANIFOLD
        // {disposition × metabolism × perception × G} (11 §3), and a
        // walk confined to one coordinate at a time is not a walk on a
        // manifold, it is a walk on its axes. The mask draws a NONEMPTY
        // SUBSET — 1..=7 over three bits — so every combination is
        // reachable and the single-axis move survives as one case among
        // seven rather than as the only case.
        //
        // Nonempty by construction: a cleave always diverges, so the
        // branches can never be clones. `mutate` draws from the options
        // EXCLUDING the current value, so every flagged axis genuinely
        // moves.
        let axes = 1 + beacon.uniform_index("growth.axes", 7);
        let (mut disposition, mut metabolism, mut perception) = (
            parent.disposition.clone(),
            parent.metabolism.clone(),
            parent.perception.clone(),
        );
        if axes & 1 != 0 {
            disposition = mutate(&disposition, &DISPOSITIONS, "growth.disposition");
        }
        if axes & 2 != 0 {
            metabolism = mutate(&metabolism, &METABOLISMS, "growth.metabolism");
        }
        if axes & 4 != 0 {
            perception = mutate(&perception, &PERCEPTIONS, "growth.perception");
        }

        // THEMED NAMING (operator ruling): the child's name derives
        // from the parent's dominant CAPABILITIES — metabolism names
        // the head, perception names the tail. Fresh two-part names,
        // never concatenations of provenance (lineage lives in the
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
        let start = beacon.uniform_index("growth.name", (heads.len() * tails.len()) as u64)
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
        // retired the branch).
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
        self.cleaved = true;
        // The origin's key moves and the cleave is receipted, naming
        // the departing branch (tag 43).
        self.world.diverge(&sigil, delta.clone());

        // The new branch settles ADJACENT: origin position plus a small
        // beacon-drawn displacement in the exact rationals per spatial axis (half-unit
        // grain, never the zero vector).
        let step = |domain: &str| -> Ratio {
            ratio(beacon.uniform_index(domain, 9) as i64 - 4, 2)
        };
        let (mut dx, dy, dz) = (step("chart.dx"), step("chart.dy"), step("chart.dz"));
        if dx == ratio(0, 1) && dy == ratio(0, 1) && dz == ratio(0, 1) {
            dx = ratio(1, 1);
        }
        Some(PolitySpec {
            civilization: branch,
            // Fresh themed world and planet — provenance is a registry
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

    /// One full heartbeat. A growth returns the newborn's spec —
    /// the era loop gives it a world of its own.
    pub fn cycle(&mut self, book: &Ledger, registry: &Registry) -> Option<PolitySpec> {
        self.drain_inbox(book);
        self.build_step();
        self.survey_step();
        self.world.tick();
        // Every settled orb collects — a chartered survey site left
        // uncollected would starve at its own upkeep.
        let orbs: Vec<u64> = self.world.committed().orbs.iter().map(|o| o.orb_id).collect();
        for orb in orbs {
            let _ = self.world.collect_as(self.automaton, orb);
        }
        self.research_step(book);
        let newborn = self.divergence_step(registry);
        // THE RESUME CARRIER (task 25, 2026-07-30): every polity
        // world syncs its tlv chronicle beside the ledger exports — an
        // append-only resume copy (cfe::persist grows the file, never
        // rewrites). The player's OBSERVING hand can resume a copy;
        // the WRITING hand stays refused at the CLI (two writers on
        // one clock is the split-brain NS-5 exists to arbitrate) —
        // intervention still goes through the steward inbox.
        let _ = cfe::persist::sync(
            self.world.chronicle(),
            &book
                .world_dir(&self.spec.world_name)
                .join("chronicle.tlv"),
        );
        // #36 KERNEL-OBSERVED PLAY: the beat chains. The committed
        // state extends this world's arbor at the world clock —
        // adopted once at first sight (the pre-chain founding stamped
        // a named debt), extended every beat after, judged=0 stamped
        // (the hole visible, never hidden; the Phase 3 flip threads
        // the steps). A penned world skips the beat entirely, so its
        // arbor stalls with its pen (MESH-1b). Best-effort like the
        // counterpart writes at this observed phase: a refused chain
        // leaves the arbor short, a visible stall, never a crash.
        let _ = self.chain_beat(book);
        let _ = scribe::record(
            book,
            &self.spec.world_name,
            &self.spec.planet,
            &self.spec.civilization.sigil,
            &self.world,
        );
        newborn
    }

    /// One beat onto the world's own arbor (#36): adopt-or-extend the
    /// committed state at `{S: charted position, G: base gauge, T:
    /// world tick}` on the mainline. The tick advanced this cycle, so
    /// the arrow ascends structurally; the adoption door shuts itself
    /// after the first pass.
    fn chain_beat(&mut self, book: &Ledger) -> Result<(), mesh::arbor::ArborRefused> {
        // The handle opens once and holds: the open's full re-verify
        // is paid at first sight (or on resume, where it re-verifies
        // the prior life's chain), then every beat is one append.
        if self.arbor.is_none() {
            self.arbor = Some(mesh::arbor::Arbor::open(
                book.world_dir(&self.spec.world_name).join("spine"),
            )?);
        }
        let arbor = self.arbor.as_mut().expect("held or just opened");
        let receipts = self.world.chronicle().receipts().to_vec();
        let seat = [
            self.spec.position.0.clone(),
            self.spec.position.1.clone(),
            self.spec.position.2.clone(),
        ];

        // THE ADOPTION DOOR (#36): a pre-chain world enters ONCE, its
        // current state seated as the unjudged genesis — the debt named,
        // never hidden. Everything after walks through the judge.
        if arbor.head("").is_none() {
            let bytes = codec::state_bytes(&self.world.committed());
            let coordinate = mesh::arbor::Coordinate {
                branch: String::new(),
                s: seat.clone(),
                g: self.world.gauge().clone(),
                t: self.world.now(),
            };
            arbor.adopt(coordinate, &bytes)?;
            self.chained = receipts.len();
            return Ok(());
        }

        // DIRECTIVE 19 — PER-DEED JUDGMENT. One commitment per DEED, not
        // one per beat, and each carries its own step so
        // `mesh::arbor::extend` runs `kernel::lawful` on it. Batching
        // was refused by ruling: nine deeds in one composite hides an
        // un-inspected interior where a local unlawful step shelters
        // behind a net-positive whole.
        //
        // Prior/next are taken by replaying the chronicle prefix, which
        // is O(deeds × history) per beat. That is honest and bounded at
        // this scale; the cure when it bites is a stepping replay in
        // cfe (replay() is one monolithic loop today and cannot advance
        // by a single receipt).
        let mut prior = cfe::world::replay(
            &cfe::vrp::Chronicle::from_receipts(receipts[..self.chained].to_vec())
                .map_err(|_| mesh::arbor::ArborRefused::StateUndecodable)?,
        );
        for at in self.chained..receipts.len() {
            let receipt = &receipts[at];
            let next = cfe::world::replay(
                &cfe::vrp::Chronicle::from_receipts(receipts[..=at].to_vec())
                    .map_err(|_| mesh::arbor::ArborRefused::StateUndecodable)?,
            );
            let step = kernel::Step {
                event: receipt.event.clone(),
                actor: receipt.provenance.first().copied().unwrap_or(cfe::world::SOVEREIGN),
                stamp: receipt.stamp,
            };
            if let Err(unlawful) = kernel::admits(&prior, &next, &step) {
                self.refused.push((receipt.sequence, unlawful));
            }
            prior = next;
        }
        self.chained = receipts.len();

        // THE COMMITMENT STAYS ONE PER BEAT — and that is NOT batching
        // the judgment, which happened per deed above. It is the arbor's
        // ADDRESSING law: `t` must strictly increase and coordinates must
        // be unique, so the store is addressed one record per clock tick
        // and several deeds share a tick (measured: per-deed commitment
        // refuses `Occupied` on the second deed of any beat). A finer
        // address — a sub-index on the coordinate — is a WIRE amendment,
        // since the coordinate is hashed into every commit. Operator
        // ruling, not a conductor change. Recorded, not worked around.
        let coordinate = mesh::arbor::Coordinate {
            branch: String::new(),
            s: seat,
            g: self.world.gauge().clone(),
            t: self.world.now(),
        };
        arbor
            .extend(coordinate, &codec::state_bytes(&self.world.committed()), None)
            .map(|_| ())
    }
}

/// MARKET-2: plain L1 between two charted positions — the transit
/// distance the iceberg law melts against.
pub fn spatial_l1(a: &(Ratio, Ratio, Ratio), b: &(Ratio, Ratio, Ratio)) -> Ratio {
    let leg = |p: &Ratio, q: &Ratio| {
        let d = p.clone() - q.clone();
        if d < ratio(0, 1) { -d } else { d }
    };
    leg(&a.0, &b.0) + leg(&a.1, &b.1) + leg(&a.2, &b.2)
}

/// Two disjoint mutable seats out of one board.
fn two_mut<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i != j);
    if i < j {
        let (left, right) = slice.split_at_mut(j);
        (&mut left[i], &mut right[0])
    } else {
        let (left, right) = slice.split_at_mut(i);
        (&mut right[0], &mut left[j])
    }
}

/// MERCHANT-1 (the MARKET-2 sequence) — THE ARBITRAGE PASS, the
/// reference model's merchant instinct (layer-2 POLICY over lawful
/// deeds; SEAM-1 will put a trait boundary here).
///
/// v1 law: the merchant is STRICTLY ADDITIVE — it bridges only
/// pairs the iceberg has WALLED (pair_melt ≥ 1 direct), creating
/// trade where none could exist and never front-running the open
/// market. The triangle: A rests give G want Q; B rests give Q
/// want G; M between them pays Q to A, hauls G, sells to B for Q —
/// profit iff what comes back exceeds what was paid after BOTH
/// legs' melt (the operator's test: P_B·(1−φD) > P_A + fees; fees
/// are ROUTE-1 upkeep, already receipted). One crossing per beat;
/// a merchant whose second leg refuses HOLDS inventory — that risk
/// is the trade. Every leg is an ordinary clear_trade: melt, void
/// share, and both books' receipts all land by the one law.
pub fn merchant_step(worlds: &mut [Polity], beat: u64) -> Vec<(String, String, String)> {
    let one = || ratio(1, 1);
    let n = worlds.len();
    // NO SINGLES (ruling 2026-07-31): every profitable triangle runs
    // this beat, and a merchant posts BESIDE its resting book — an
    // unsold leg-2 counter stays on the open market like any offer.
    let mut crossings = Vec::new();
    let mut seq: u64 = 0;
    for m in 0..n {
        if worlds[m].penned {
            continue;
        }
        for a in 0..n {
            if a == m || worlds[a].penned {
                continue;
            }
            for b in 0..n {
                if b == m || b == a || worlds[b].penned {
                    continue;
                }
                let Some(ask_a) = worlds[a].world.offers().first().cloned() else { continue };
                let Some(ask_b) = worlds[b].world.offers().first().cloned() else { continue };
                // The triangle in one quote: B wants what A sells,
                // and pays in what A asks.
                if ask_b.want != ask_a.give || ask_b.give != ask_a.want {
                    continue;
                }
                // Strictly additive: the direct pair must be walled.
                let d_ab = spatial_l1(&worlds[a].spec.position, &worlds[b].spec.position);
                if cfe::world::pair_melt(&worlds[a].world, &worlds[b].world, &d_ab) < one() {
                    continue;
                }
                let d_am = spatial_l1(&worlds[a].spec.position, &worlds[m].spec.position);
                let d_mb = spatial_l1(&worlds[m].spec.position, &worlds[b].spec.position);
                let melt_am = cfe::world::pair_melt(&worlds[a].world, &worlds[m].world, &d_am);
                let melt_mb = cfe::world::pair_melt(&worlds[m].world, &worlds[b].world, &d_mb);
                if melt_am >= one() || melt_mb >= one() {
                    continue;
                }
                // The profit test, exact: bought must fill B's ask,
                // and the quote coming back must exceed the quote
                // paid — else the caravan stays home.
                let bought = ask_a.give_amount.clone() * (one() - melt_am);
                if bought < ask_b.want_amount {
                    continue;
                }
                let back = ask_b.give_amount.clone() * (one() - melt_mb);
                if back <= ask_a.want_amount {
                    continue;
                }
                let held = worlds[m]
                    .world
                    .orb()
                    .map(|orb| orb.banked_amount(ask_a.want))
                    .unwrap_or_else(|| ratio(0, 1));
                if held < ask_a.want_amount {
                    continue;
                }
                // Leg 1: M counters A exactly — fills at A's rest.
                let counter = worlds[m].world.chronicle().receipts().len() as u64;
                if worlds[m]
                    .world
                    .post_offer(
                        cfe::world::FIRST_ORB,
                        ask_a.want,
                        ask_a.want_amount.clone(),
                        ask_a.give,
                        ask_a.give_amount.clone(),
                        counter,
                    )
                    .is_err()
                {
                    continue;
                }
                let deal = beat * 100_000 + 90_000 + seq * 4;
                seq += 1;
                {
                    let (merchant, seller) = two_mut(worlds, m, a);
                    if cfe::world::clear_trade(
                        &mut merchant.world, counter, &mut seller.world, ask_a.offer, deal, d_am,
                    )
                    .is_err()
                    {
                        continue;
                    }
                }
                // Leg 2: M counters B — fills at B's rest. A refusal
                // here leaves M holding cargo: merchant risk, lawful.
                let counter = worlds[m].world.chronicle().receipts().len() as u64;
                if worlds[m]
                    .world
                    .post_offer(
                        cfe::world::FIRST_ORB,
                        ask_b.want,
                        ask_b.want_amount.clone(),
                        ask_b.give,
                        ask_b.give_amount.clone(),
                        counter,
                    )
                    .is_err()
                {
                    continue;
                }
                let sold = {
                    let (merchant, buyer) = two_mut(worlds, m, b);
                    cfe::world::clear_trade(
                        &mut merchant.world, counter, &mut buyer.world, ask_b.offer, deal + 2, d_mb,
                    )
                    .is_ok()
                };
                if sold {
                    crossings.push((
                        worlds[m].spec.world_name.clone(),
                        worlds[a].spec.world_name.clone(),
                        worlds[b].spec.world_name.clone(),
                    ));
                }
            }
        }
    }
    crossings
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
    worlds: &mut [Polity],
    registry: &Registry,
    book: &Ledger,
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
                Some(polity) if !polity.penned => {
                    match mesh::yield_authority(
                        polity.world.chronicle_mut(),
                        cfe::vrp::CONDUCTOR_HAND,
                        hand,
                        stamp,
                    ) {
                        Ok(sequence) => {
                            polity.world.hold_pen(None);
                            let _ = cfe::persist::sync(
                                polity.world.chronicle(),
                                &book.world_dir(world_name).join("chronicle.tlv"),
                            );
                            let _ = scribe::record(
                                book,
                                &polity.spec.world_name,
                                &polity.spec.planet,
                                &polity.spec.civilization.sigil,
                                &polity.world,
                            );
                            polity.penned = true;
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
    for polity in worlds.iter_mut().filter(|e| e.penned) {
        let carrier = book
            .world_dir(&polity.spec.world_name)
            .join("chronicle.tlv");
        let Ok(chronicle) = cfe::persist::load(&carrier) else {
            continue;
        };
        if mesh::authority(&chronicle) == Ok(Some(cfe::vrp::CONDUCTOR_HAND)) {
            let name = polity.spec.world_name.clone();
            *polity = Polity::genesis(polity.spec.clone(), registry, book);
            log.push(format!("HANDOFF — {name}: the pen returns; beats resume"));
        }
    }
    log
}

/// INCURSION-1a typed refusals (07 §3.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncursionRefused {
    /// No sealed carrier stands at that era and world.
    NoSuchSeal,
    /// The living are never raided: an orb stands unfamished.
    TheLivingStand,
    /// The continuation already walks the live board.
    AlreadyWoken,
}

/// INCURSION-1a (07 §3.1): the archived-sheet door. A sealed DERELICT
/// (every orb famished — the cascade law) is continued LIVE under a
/// woken line: the sealed chronicle is copied forward (the archive is
/// never edited), the succession law executes on the continuation
/// (NOVEL-1 — the tree drops to the kernel, the record stands), fresh
/// contained agents register, the world charts at a chronicle-seeded
/// anchor, and the door receipts on the universe ledger
/// (IncursionStruck — era, world, line). The aeon-voice trigger is
/// the orchestration's (deferred, recorded); this is the deed in the
/// operator's and conductor's hand. The woken line plays the entropic
/// leg (policy Aggressive — the T∞ pole in the small).
pub fn incursion(
    era_root: &std::path::Path,
    world_name: &str,
    registry: &Registry,
    book: &Ledger,
    stamp: u64,
) -> Result<Polity, IncursionRefused> {
    let sealed = era_root
        .join("worlds")
        .join(world_name)
        .join("chronicle.tlv");
    let Ok(chronicle) = cfe::persist::load(&sealed) else {
        return Err(IncursionRefused::NoSuchSeal);
    };
    let mut world = World::resume(chronicle);
    let orbs = world.committed().orbs;
    if orbs.is_empty() || orbs.iter().any(|orb| orb.famine.is_empty()) {
        return Err(IncursionRefused::TheLivingStand);
    }
    let woken_world = format!("{world_name}-returned");
    let live_carrier = book.world_dir(&woken_world).join("chronicle.tlv");
    if live_carrier.exists() {
        return Err(IncursionRefused::AlreadyWoken);
    }
    // The woken line, chronicle-seeded from the terminal digest.
    let terminal = world
        .chronicle()
        .receipts()
        .last()
        .map(|receipt| receipt.own.0)
        .unwrap_or([0u8; 32]);
    let beacon = Beacon::from_digest(terminal);
    let name = format!("{world_name} Returned");
    let sigil = ledger::registry::sigil_of(&name);
    let civilization = Civilization {
        name,
        sigil: sigil.clone(),
        species: "revenant automaton".into(),
        clade: "revenant".into(),
        motto: "we finish what the famine started".into(),
        disposition: DISPOSITIONS
            [beacon.uniform_index("incursion.disposition", DISPOSITIONS.len() as u64) as usize]
            .into(),
        metabolism: METABOLISMS
            [beacon.uniform_index("incursion.metabolism", METABOLISMS.len() as u64) as usize]
            .into(),
        perception: PERCEPTIONS
            [beacon.uniform_index("incursion.perception", PERCEPTIONS.len() as u64) as usize]
            .into(),
        founder: "conductor".into(),
        lineage: Vec::new(),
        station: None,
    };
    let _ = registry.create(&civilization);
    // The succession on the CONTINUATION — the archive stays sealed.
    world
        .succeed(&sigil)
        .map_err(|_| IncursionRefused::TheLivingStand)?;
    let automaton = world
        .register_agent(&format!("{sigil}-conductor"))
        .map_err(|_| IncursionRefused::AlreadyWoken)?;
    // The canonical steward handle: register it, or reuse the sealed
    // world's own (genesis resumes by exactly these handles).
    let steward = match world.register_agent("steward") {
        Ok(entity) => entity,
        Err(_) => world
            .committed()
            .entities
            .iter()
            .find(|held| held.handle == "steward")
            .map(|held| held.entity_id)
            .ok_or(IncursionRefused::AlreadyWoken)?,
    };
    for scope in [Scope::Build, Scope::Raze, Scope::Collect] {
        let _ = world.grant(automaton, scope);
        let _ = world.grant(steward, scope);
    }
    // The continuation persists under the woken name; genesis resumes
    // it as the incursion polity.
    let _ = std::fs::create_dir_all(live_carrier.parent().expect("world dir"));
    let _ = cfe::persist::sync(world.chronicle(), &live_carrier);
    let coordinate = |axis: &str| -> Ratio {
        ratio(beacon.uniform_index(axis, 64) as i64, 1)
    };
    let spec = PolitySpec {
        civilization,
        world_name: woken_world.clone(),
        planet: format!("{sigil}-prime"),
        colony: format!("first-{sigil}"),
        policy: Policy::Aggressive,
        position: (
            coordinate("incursion.x"),
            coordinate("incursion.y"),
            coordinate("incursion.z"),
        ),
        star: None,
        albedo: None,
        orbit: None,
        gauge: None,
    };
    let polity = Polity::genesis(spec, registry, book);
    // The door receipts on the universe ledger (ledger-side, the
    // GalaxyObserved pattern).
    let era = era_root
        .file_name()
        .map(|held| held.to_string_lossy().into_owned())
        .unwrap_or_default();
    let _ = book.strike_incursion(&era, world_name, &sigil, stamp);
    Ok(polity)
}

/// GALAXY-1b (13_GALAXY §3, §5): the year-voice observation sweep.
/// Every crossed, charted world OBSERVES; an anchor past the frontier
/// creates the galaxy it observes, executed as the progenitor's deed
/// (12 §2a — the board-maker function). Idempotent by geometry: a
/// created galaxy encloses its observer, so the next sweep reads
/// inside-reach and creates nothing.
pub fn observe_crossings(
    worlds: &[Polity],
    registry: &Registry,
    book: &Ledger,
    stamp: u64,
) -> Vec<Galaxy> {
    let mut created = Vec::new();
    for polity in worlds {
        if !polity.world.has_crossed() {
            continue;
        }
        let Some(((x, y, z), _)) = book.charted_position(&polity.spec.world_name) else {
            continue;
        };
        let creator = galaxy_creator(registry, &polity.spec.civilization);
        if let Ok(Some(galaxy)) =
            book.observe_galaxy(&polity.spec.civilization.sigil, &creator, (&x, &y, &z), stamp)
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
/// every polity, then `cycles` heartbeats; branches born mid-era join the
/// beat. Returns the live worlds.
pub fn run_era(var_root: &str, cycles: u32) -> Vec<Polity> {
    let registry = Registry::at(format!("{var_root}/registry"));
    let book = Ledger::at(format!("{var_root}/worlds"));
    let eras = ledger::converge::default_era_roots(var_root);
    let epoch_base = ledger::converge::converge(&book, &eras)
        .map(|read| read.epoch_base)
        .unwrap_or(0);
    // 11 §5.2: the live aeon's G-sheet (see main.rs — same law).
    let live_base = cfe::world::wrap_gauge(ratio(eras.len() as i64, 12));
    let mut worlds: Vec<Polity> = polities::founding_polities()
        .into_iter()
        .map(|mut spec| {
            let opening = spec.gauge.take().unwrap_or_else(|| ratio(0, 1));
            spec.gauge = Some(cfe::world::wrap_gauge(opening + live_base.clone()));
            Polity::genesis(spec, &registry, &book)
        })
        .collect();
    for beat in 0..cycles {
        // TUniversal T = epoch base + this era's beat (07 §1.3).
        let _ = book.set_universal_clock(epoch_base + u64::from(beat) + 1);
        let mut newborns = Vec::new();
        for polity in &mut worlds {
            if let Some(spec) = polity.cycle(&book, &registry) {
                newborns.push(spec);
            }
        }
        for spec in newborns {
            worlds.push(Polity::genesis(spec, &registry, &book));
        }
        // JOURNAL-1 (08 §3): on year boundaries every civilization
        // speaks — the lens renders journals + wall, write-only display
        // of the chronicle, never a state change.
        if (u64::from(beat) + 1).is_multiple_of(scribe::journal::YEAR_TICKS) {
            for polity in &worlds {
                let _ = scribe::record_journal(
                    &book,
                    &polity.spec.world_name,
                    &polity.world,
                    &scribe::journal::Voice::of(&polity.spec.civilization),
                );
            }
            // GALAXY-1b: the observation sweep rides the year voice
            // (11 §1 — board-scale shifts read yearly).
            let _ = observe_crossings(&worlds, &registry, &book, epoch_base + u64::from(beat) + 1);
        }
    }
    worlds
}
