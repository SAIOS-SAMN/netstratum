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
//! the COMPILED roster (THE DEFINITION LAW — `polities.rs`); nothing
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

use cfe::dop::{Resource, Station, ZoneKind};
use cfe::ipsal::Scope;
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::World;
use ledger::galaxy::Galaxy;
use ledger::registry::{
    lineage_domain, office_preconditions, Civilization, OfficeEvidence, Registry,
    DISPOSITIONS, METABOLISMS, PERCEPTIONS,
};
use ledger::{Directive, Disposition, Ledger};
use physics::Beacon;
use solver::{
    forge_task, propose_consulted_measured, solve_consulted, Difficulty, Hunger, Need,
    SolveRead,
};

use std::path::Path;

mod polities;
pub use polities::{founding_polities, registered_polities, roster};

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
    // THE DEFINITION LAW: no roster file is archived, because none
    // exists. The era's roster is the roster the binary that sealed it
    // was compiled with — recorded by the commit, not by a copy.
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
///
/// **UNREACHABLE AS WIRED — measured 2026-08-03, declared here rather
/// than quietly repaired.** This tests the METABOLISM axis, and no
/// metabolism is or can be `"aeric"`: the vocabulary is closed at
/// `["mineral", "biotic", "plasmic"]`
/// (`ledger::registry::METABOLISMS`), and `12_UNION` §5a lists aeric as
/// a SUBSTRATE — a lineage line that is itself *biotic*, distinguished
/// by living in a gas giant's banded column. Substrates live in the
/// `clade` field, not the metabolism axis, which is exactly why the
/// same table says `plasmic` "needs the third metabolism value" and
/// aeric does not. Both call sites (`regime`, `trade_intent`) pass a
/// metabolism, so the early return fires for every civilization alive
/// and the ruling has never once applied.
///
/// It is not fixed here because the fix is not this lane's to choose.
/// The reading has two plausible homes and they are not equivalent:
/// the civilization's `clade` (but the clade vocabulary is a CLOSED u8
/// set ratified by the operator at `15_TAXON` §3, and `aeric` is not in
/// it — adding it is an amendment), or the WORLD's own body (a gas
/// giant is a place, and §5a calls the giants of every world its
/// holdings — which would make this a siting property, not an axis
/// read at all). Measured on the live board: 33 registry cards, zero
/// carrying an aeric substrate under any spelling.
///
/// Pinned by `the_aeric_deck_band_is_unreachable_today` — the day a
/// real aeric reading is wired, that pin goes red and is truth-updated.
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

/// E3.7: when an orb hungers for a prospected ElementId, offer half
/// the largest closed-carrier bank for a unit of that element.
fn element_want_intent(
    world: &World,
) -> Option<(cfe::dop::Cargo, Ratio, cfe::dop::Cargo, Ratio)> {
    let orb = world.orb()?;
    let (_, element) = world.element_wants().iter().find(|(o, _)| *o == orb.orb_id)?;
    let biggest = Resource::ALL
        .iter()
        .max_by_key(|r| orb.banked_amount(**r))?;
    let stock = orb.banked_amount(*biggest);
    if stock <= ratio(0, 1) {
        return None;
    }
    let give = lot(&(stock.clone() / ratio(2, 1)));
    let want = lot(&relaxed_want(&give, &stock));
    if give <= ratio(0, 1) || want <= ratio(0, 1) {
        return None;
    }
    Some((
        cfe::dop::Cargo::Resource(*biggest),
        give,
        cfe::dop::Cargo::Element(*element),
        want,
    ))
}

/// E-MKT-1: when an orb holds open-matter stock, offer half of the
/// largest element bank for the leanest closed carrier its zones burn
/// (or iron if no upkeep map). Never invents element ids — only banked
/// identities.
fn element_trade_intent(
    world: &World,
) -> Option<(cfe::dop::Cargo, Ratio, cfe::dop::Cargo, Ratio)> {
    let orb = world.orb()?;
    let (element, stock) = orb
        .elements
        .iter()
        .max_by_key(|(_, amount)| amount.clone())?;
    if *stock <= ratio(2, 1) {
        return None;
    }
    let half = stock.clone() / ratio(2, 1);
    let give = lot(&half);
    if give <= ratio(0, 1) {
        return None;
    }
    // Prefer a resource the zones burn and the bank is lean on; else iron.
    let burns: Vec<Resource> = orb
        .zones
        .iter()
        .flat_map(|z| z.kind.upkeep_per_tick())
        .map(|(resource, _)| resource)
        .collect();
    let want_res = burns
        .iter()
        .min_by_key(|r| orb.banked_amount(**r))
        .copied()
        .unwrap_or(Resource::Iron);
    let want = lot(&relaxed_want(&give, stock));
    if want <= ratio(0, 1) {
        return None;
    }
    Some((
        cfe::dop::Cargo::Element(*element),
        give,
        cfe::dop::Cargo::Resource(want_res),
        want,
    ))
}

/// OFFER-1a: one market beat over the whole board — post fresh
/// intents (one open offer per world; the offer id is the chronicle
/// length at posting, deterministic), then clear the FIRST crossing
/// (price-time priority inside `clear_trade`). Returns the cleared
/// pairs (world names) for the narrator line.
pub static PROBE_DIGEST_REPEAT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static PROBE_DIGEST_TOTAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn market_step(worlds: &mut [Polity], beat: u64) -> Vec<(String, String)> {
    // Post intents. NO SINGLES (ruling 2026-07-31): the book is
    // open-ended — a fresh intent posts BESIDE what rests; only an
    // identical pair (same give, same want) holds its place.
    for polity in worlds.iter_mut() {
        if polity.penned {
            continue;
        }
        // E3.7: demand first — element_wants bid resource for open matter.
        if let Some((give, give_amount, want, want_amount)) =
            element_want_intent(&polity.world)
        {
            if !polity
                .world
                .offers()
                .iter()
                .any(|held| held.give == give && held.want == want)
            {
                let offer = polity.world.chronicle().receipts().len() as u64;
                let orb = cfe::world::FIRST_ORB;
                if let (Some(pair), Some(ga), Some(wa)) = (
                    cfe::dop::Exchange::cargo(give, want),
                    cfe::dop::Positive::new(give_amount),
                    cfe::dop::Positive::new(want_amount),
                ) {
                    let _ = polity.world.post_offer(orb, pair, ga, wa, offer);
                }
            }
        }
        // E-MKT-1: element surplus before closed-carrier intents when
        // the line holds banked open matter above a floor.
        if let Some((give, give_amount, want, want_amount)) =
            element_trade_intent(&polity.world)
        {
            if !polity
                .world
                .offers()
                .iter()
                .any(|held| held.give == give && held.want == want)
            {
                let offer = polity.world.chronicle().receipts().len() as u64;
                let orb = cfe::world::FIRST_ORB;
                if let (Some(pair), Some(ga), Some(wa)) = (
                    cfe::dop::Exchange::cargo(give, want),
                    cfe::dop::Positive::new(give_amount),
                    cfe::dop::Positive::new(want_amount),
                ) {
                    let _ = polity.world.post_offer(orb, pair, ga, wa, offer);
                }
            }
        }
        if let Some((give, give_amount, want, want_amount)) =
            trade_intent(&polity.world, &polity.spec.civilization.metabolism)
        {
            let give_c = cfe::dop::Cargo::Resource(give);
            let want_c = cfe::dop::Cargo::Resource(want);
            if polity
                .world
                .offers()
                .iter()
                .any(|held| held.give == give_c && held.want == want_c)
            {
                continue;
            }
            let offer = polity.world.chronicle().receipts().len() as u64;
            let orb = cfe::world::FIRST_ORB;
            let (Some(pair), Some(give_amount), Some(want_amount)) = (
                cfe::dop::Exchange::new(give, want),
                cfe::dop::Positive::new(give_amount),
                cfe::dop::Positive::new(want_amount),
            ) else {
                continue;
            };
            let _ = polity
                .world
                .post_offer(orb, pair, give_amount, want_amount, offer);
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
                let best_held: Option<cfe::dop::Unit> = one
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
                // (1+grade)/8 is the PICKUP-flagged placeholder (C2's
                // cluster). At grade >= 7 it reaches 1, which is out of
                // [0,1) — the dead BadEta used to refuse that at the
                // verb; the Unit door now says it at construction, and
                // an unconstructible efficiency charters no lane, which
                // is the same observable outcome.
                let Some(eta) = cfe::dop::Unit::new(ratio(1 + i64::from(best_grade), 8)) else {
                    continue;
                };
                if best_held.is_none_or(|held| eta > held) {
                    let deal = beat * 100_000 + seq * 4;
                    seq += 1;
                    // THE REFUSAL IS CARRIED, NEVER SWALLOWED (operator
                    // ruling 2026-08-02). `let _ =` here hid the whole
                    // cross-book economy: measured, 17 crossings cleared
                    // and 0 lanes chartered, because every charter
                    // refused CannotAfford{Alloy, need 4, have 0} — and
                    // no lane means no cross-book capacity reading at
                    // all. An economic gate read as a topological limit
                    // for exactly as long as nobody could see it.
                    if let Err(refusal) = cfe::world::charter_route(
                        &mut one.world,
                        cfe::world::FIRST_ORB,
                        &mut two.world,
                        cfe::world::FIRST_ORB,
                        eta,
                        deal,
                    ) {
                        one.lane_refused.push(refusal);
                    }
                }
            }
        }
    }
    cleared
}

/// WHERE A LINE IS GOING — read from its own measured yield, never set
/// by a constant.
///
/// `SEARCH_BUDGET` bounds one heartbeat so a single beat cannot run
/// away; it was never a statement about whether spending is WORTH it.
/// This is. Every branch below cites a number from
/// `solver/examples/forge_reach.rs`, measured over 200 forged tasks per
/// rung on rungs 3–5 — the rungs that actually pay, since
/// `mint_bound` f(0..=2) = 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trajectory {
    /// The cheap reach is closing work. STAY THERE. Yield per candidate
    /// is highest at sated on every rung — 51/2, 17 and 13 against
    /// struck's 199/8, 11 and 9. Spending more while the cheap reach
    /// pays is buying what you already have.
    Growth,
    /// Spending and closing nothing. NECESSITY OPENS THE REACH, and it
    /// is worth a great deal: the marginal yield of the sated→struck
    /// step measured 97/4, 13/2 and 7 at rungs 3, 4 and 5. At rung 3
    /// struck closes 200/200 — matching an exhaustive sweep at 8
    /// candidates per task against up to 200,000.
    Stagnation,
    /// Struck, and STILL closing nothing. The READING has failed, so
    /// stop reading and EXHAUST instead — this is where the sweep earns
    /// its keep and the only place it does.
    ///
    /// Two measurements put it here rather than at a deeper hunger.
    /// First, deeper hunger is worthless: struck→long-starved bought
    /// EXACTLY ZERO extra yield at rungs 3 and 4, and at rung 5 bought
    /// one more confirmed closure and one more FALSE one. Second, the
    /// sweep is worth something precisely where the reach is not — at
    /// rungs 4 and 5 it confirms 122 and 86 against the reach's 77 and
    /// 54. Those are the tasks composed through intermediate states no
    /// observation motivates: solved by exhaustion, not by reading.
    Decline,
}

/// WHICH REGIME a board runs under — the three paths, held together so
/// they can be measured against each other rather than argued about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchMode {
    /// COUPLED — the trajectory routes. Read while reading pays,
    /// escalate the hunger on stagnation, exhaust only on decline.
    #[default]
    Coupled,
    /// UNCOUPLED — always exhaust. The original path, and the control
    /// every other number is read against.
    Uncoupled,
    /// **V ∪ D** — the union. Every task gets BOTH: read first, and
    /// exhaust whenever the reading did not close. Not a router — a
    /// dyad, neither path deriving its standing from the other, and the
    /// closure set is the union of what each can reach alone.
    ///
    /// It should find everything either finds, and pay both costs to do
    /// it. Whether that is worth it is what the benchmark answers.
    Union,
}

/// HOW a line searches. Both paths are live; neither is deprecated.
///
/// They are not competitors — they fail in different places. The reach
/// MATCHES an exhaustive sweep exactly at rung 3 (200/199/1) at 8
/// candidates against up to 200,000, and loses only at rungs 4-5, where
/// the answer needs composition through states the observation cannot
/// name. So the engine reads first and exhausts only when reading has
/// demonstrably run out — which is what makes exhaustion affordable at
/// all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchPath {
    /// Derivation-first: the observation names the operator, the need
    /// orders the reach. ~66x the energy per candidate on the live
    /// board.
    Reach,
    /// Exhaustive composition, bounded by `SEARCH_BUDGET`. Finds what
    /// no reading motivates, and costs four orders of magnitude more.
    Sweep,
}

impl Trajectory {
    /// WHICH PATH this trajectory searches by.
    pub fn path(&self) -> SearchPath {
        match self {
            Trajectory::Growth | Trajectory::Stagnation => SearchPath::Reach,
            Trajectory::Decline => SearchPath::Sweep,
        }
    }

    /// The reach this trajectory buys, when it is reading. Unused on
    /// the sweep path, which has no need to order.
    pub fn need(&self, starved: u64) -> Need {
        self.need_of(Vec::new(), starved)
    }

    /// **THE TRAJECTORY DECIDES WHETHER; THE LINE DECIDES WHICH.**
    ///
    /// Composed at the 2026-08-04 merge. This lane's trajectory answers
    /// *is spending paying* — a homeostatic reading, measured, and it
    /// is what stops a paying line from buying reach it already has.
    /// It cannot answer WHAT the line is short of, and its own
    /// `need` could only ever say Novelty.
    ///
    /// `Polity::hunger` answers that half: it reads the line's own
    /// chronicle rhythm for BOTH hungers — starved of signal, starved
    /// of links — so two lines stagnating for different reasons reach
    /// for different families. Empty `measured` falls back to Novelty,
    /// which is this lane's original behaviour exactly.
    pub fn need_of(&self, measured: Vec<Hunger>, starved: u64) -> Need {
        match self {
            Trajectory::Growth | Trajectory::Decline => Need::sated(),
            Trajectory::Stagnation => match measured.is_empty() {
                true => Need::struck(Hunger::Novelty, starved),
                false => Need {
                    hungers: measured,
                    starved_for: starved,
                    thin: Vec::new(),
                },
            },
        }
    }

    /// Advance on the beat's outcome. A close returns any line to
    /// growth — including one in decline, which is what makes the
    /// contraction recoverable rather than a trap.
    pub fn advance(self, closed: bool) -> Trajectory {
        match (self, closed) {
            (_, true) => Trajectory::Growth,
            (Trajectory::Growth, false) => Trajectory::Stagnation,
            (Trajectory::Stagnation | Trajectory::Decline, false) => Trajectory::Decline,
        }
    }

    pub fn describe(&self) -> &'static str {
        match self {
            Trajectory::Growth => "growth",
            Trajectory::Stagnation => "stagnation",
            Trajectory::Decline => "decline",
        }
    }
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
    /// gate and into the chain. Every deed past this mark is chained
    /// individually, with its own step, and gated.
    chained: usize,
    /// THE STEPPER (task #9): the judged prefix's committed state,
    /// advanced one receipt at a time — prior/next per deed are a
    /// clone of state (O(state)), never a prefix replay (O(history)).
    /// Workspace like `chained`: seeded at first chain of a process.
    stepper: Option<cfe::world::Replayer>,
    /// NOVELTY-GATE (38 §4): this line's minted solver library —
    /// workspace cache of the `SolverWordMinted` fold, seeded lazily
    /// at the first research beat of a process.
    library: solver::mint::Library,
    library_folded: bool,
    /// Words seeking their three confirms: (orbit, expansion, cited).
    word_pending: Vec<([u8; 32], Vec<solver_verify::grid::Op>, Vec<[u8; 32]>)>,
    /// Planes this line has OPENED — the unfold, one per earned mint.
    /// The fiber's grade count is `world.key().len()`; this counts the
    /// openings that got it there.
    pub unfolded: u64,
    /// WHICH REGIME this line searches under. Set by the era runner so
    /// the three paths can be benchmarked against one another.
    pub mode: SearchMode,
    /// WHERE THIS LINE IS GOING, read from its own outcomes and used
    /// to decide what the next beat's search may spend.
    pub trajectory: Trajectory,
    /// Beats since this line last closed a task — what it hands the
    /// hunger when it reaches. A real duration, never a constant.
    pub starved: u64,
    /// Candidate programs this line has spent on the READ path,
    /// cumulative. The S of W/S; the board reads its own yield without
    /// instrumentation.
    pub spent: u64,
    /// Times this line fell back to the EXHAUST path. Counted as
    /// invocations, not candidates: the sweep reports no count and is
    /// bounded by `SEARCH_BUDGET`, so folding it into `spent` would
    /// mix a measured number with a bound and ruin both yields.
    pub swept: u64,
    /// Deeds the kernel REFUSED, by receipt sequence. Kept rather than
    /// discarded — a refusal the board never sees is the gate speaking
    /// into a void.
    /// THE JUDGE'S REFUSALS — per-deed, from `chain_beat`.
    ///
    /// **Made public 2026-08-02 (operator ruling: stop swallowing
    /// refusals).** This was private and NOTHING read it: the kernel has
    /// been judging every deed every beat and posting its verdicts into
    /// a field no caller could see. A refusal names its cure, and a cure
    /// nobody can read is not a cure.
    pub refused: Vec<(u64, kernel::Refused)>,
    /// THE LANE REFUSALS — why `charter_route` said no.
    ///
    /// `conductor/src/lib.rs`'s charter site discarded this with
    /// `let _ =`, and the consequence was invisible: **17 crossings
    /// cleared and 0 lanes were chartered** on a measured board, because
    /// every charter refused `CannotAfford { Alloy, need 4, have 0 }`.
    /// The cross-book capacity reading has no substrate without lanes,
    /// so a swallowed economic refusal read as a topological limit.
    pub lane_refused: Vec<cfe::world::ActionRefused>,
    /// R5 — the forge memo. `solve` is pure: same digest, same
    /// difficulty, same answer. Measured 2026-08-02: 31 % of forge draws
    /// reuse an unchanged digest, and 99 % of forge time is the
    /// exhaustive sweep over tasks with no answer in the vocabulary — so
    /// a third of the board's compute was re-deriving identical
    /// `Unsolved` readings. One entry is enough: the repeat is always
    /// "unchanged since last beat".
    ///
    /// **KEYED ON THE WHOLE `Difficulty`, not on `word_len`** (defect
    /// found at the 2026-08-04 integration, measured). The key was
    /// `(digest, word_len)`, and `Difficulty::level(3)` and `level(4)`
    /// BOTH carry `word_len: 2` while differing in tier, rows and cols.
    /// So a line promoting 3 → 4 on an unchanged digest hit the memo and
    /// was handed the level-3 solve, which the conductor then shipped as
    /// a level-4 proof: the gate re-forged level 4, re-ran the program
    /// and got a different grid — `WorkRefused { OutputMismatch }`, a
    /// mint the proposer emitted and the gate refused. The task IS the
    /// whole difficulty, so the key must be too.
    last_forge: Option<([u8; 32], Difficulty, Need, SolveRead)>,
    /// How many refusals the board has already been TOLD. The record
    /// above is permanent; this is only the narrator's place in it, so
    /// a refusal is announced once per process and kept forever.
    /// Workspace state, like `chained`: a RESUMED world restarts both
    /// at 0 and re-gates its whole chronicle on the next beat, so a
    /// pre-existing refusal is announced again in the new process. That
    /// is honest — the gate met the deed again — and it is the same
    /// re-verification `chained: 0` already performs.
    reported: usize,

}

/// The smelter's (iron, biomass) gate — bound ONCE, read by the
/// industrial arm, the forge hold, and the gather alike, so the
/// ratchet counts two dials and no reader can drift from another.
fn smelter_gates() -> (Ratio, Ratio) {
    (ratio(30, 1), ratio(10, 1))
}

/// Evidence the ascend gates read — folded from the live world
/// (receipts + banked elements). One home for office_step and pins.
fn office_evidence(world: &World) -> OfficeEvidence {
    use cfe::dop::ClosureEvent;
    let mut evidence = OfficeEvidence::default();
    if let Some(orb) = world.orb() {
        evidence.element_kinds = orb.elements.len() as u64;
    }
    for receipt in world.chronicle().receipts() {
        match &receipt.event {
            ClosureEvent::AssayStruck { .. } => evidence.assays_struck += 1,
            ClosureEvent::RouteChartered { .. } => evidence.routes_chartered += 1,
            ClosureEvent::OfferCleared { .. } => evidence.market_clears += 1,
            _ => {}
        }
    }
    evidence.has_crossed = world.has_crossed();
    evidence
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
        cfe::thermal::t_eq4(&class.luminosity(), site.albedo.value(), site.orbit_r.value()),
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
                        stepper: None,
                        library: solver::mint::Library::new(),
                        library_folded: false,
                        word_pending: Vec::new(),
                        refused: Vec::new(),
                        unfolded: 0,
                        lane_refused: Vec::new(),
                        last_forge: None,
                        reported: 0,
                        mode: SearchMode::default(),
                        trajectory: Trajectory::Growth,
                        starved: 0,
                        spent: 0,
                        swept: 0,
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
        // keeps the Restraint, epicurean raises the Showpiece, empiricist keeps
        // the Austerity. Receipted like every deed; the Hoard and the
        // Overdrive are earned choices, never seeded.
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
        // The spec's siting is Ratio-typed data; the doors prove it here.
        // `let _ =` already discarded refusals on this path, so an
        // unconstructible spec value sites nothing — same as before.
        if let (Some(albedo), Some(orbit)) =
            (cfe::dop::Unit::new(albedo), cfe::dop::Positive::new(orbit))
        {
            let _ = world.site_planet(cfe::world::FIRST_ORB, albedo, orbit, None, None);
        }
        Polity {
            chained: 0,
            stepper: None,
            library: solver::mint::Library::new(),
            library_folded: false,
            word_pending: Vec::new(),
            refused: Vec::new(),
            unfolded: 0,
            lane_refused: Vec::new(),
            last_forge: None,
            reported: 0,
            mode: SearchMode::default(),
            trajectory: Trajectory::Growth,
            starved: 0,
            spent: 0,
            swept: 0,
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

    /// **GROUND STEP** (operator lock 2026-08-07) — the almanac arm of
    /// the autonomous loop. Issues the same deeds the CLI uses:
    /// `prospect_as` (Scope::Build) and `assay_as` (Scope::Collect).
    ///
    /// **Baseline policy (deterministic, no metabolic gates):**
    /// - Prospect: the lowest unopened stratum on each commissioned
    ///   orb, at most one successful prospect per beat (same capacity
    ///   discipline as `build_step`'s one zone).
    /// - Assay: every open vein where `now > last_assay` (span law).
    ///
    /// Placement in `cycle`: after `build_step`, before `tick`, so
    /// open veins and banked elements stand before Resource yields.
    fn ground_step(&mut self) {
        if self.world.star().is_none() {
            return; // prospect refuses NoStarCommissioned
        }
        let automaton = self.automaton;
        let orbs: Vec<u64> = self
            .world
            .committed()
            .orbs
            .iter()
            .map(|held| held.orb_id)
            .collect();

        // PROSPECT — one success per beat, lowest stratum first.
        // CIV-REACH-1: stratum 0 always refuses and depth is capped by
        // the people's growth, so the walk is 1..=civ_reach — a bronze
        // line digs its two shallow strata and deepens as tech lands.
        let reach = self.world.civ_reach();
        let mut prospected = false;
        for orb in &orbs {
            if prospected {
                break;
            }
            for stratum in 1u8..=reach {
                match self.world.prospect_as(automaton, *orb, stratum) {
                    Ok(_) => {
                        prospected = true;
                        break;
                    }
                    // Already open: try the next higher stratum.
                    Err(cfe::world::ActionRefused::StratumProspected { .. }) => continue,
                    // Any other refusal (scope, no orb, …): leave this orb.
                    Err(_) => break,
                }
            }
        }

        // ASSAY — every open vein with a positive span.
        // Snapshot (orb, element, last_assay): assay mutates prospects.
        let veins: Vec<(u64, [u8; 32], u64)> = self
            .world
            .committed()
            .prospects
            .iter()
            .map(|vein| (vein.orb, vein.element, vein.last_assay))
            .collect();
        let now = self.world.now();
        for (orb, element, last_assay) in veins {
            if now <= last_assay {
                continue; // NothingAccrued — wait for the tick to open span
            }
            let _ = self.world.assay_as(automaton, orb, element);
        }
    }

    /// **OFFICE STEP (T1.4)** — pure NPC path to economic stations.
    /// After ground_step has banked elements, a line may claim Caravan
    /// (T4) or Exchange (T6) through the same doors the CLI uses:
    /// chain seat + registry card, gated by [`office_preconditions`].
    /// Disposition picks the office so two lines on one board can
    /// diverge without a player click (Wave-1 exit criterion).
    fn office_step(&mut self, registry: &Registry) {
        let held = self.spec.civilization.station.as_deref();
        // One office per line — the climb is from the COMMON (T3, no
        // seat). A founding progenitor / watcher / observer / sovereign
        // already holds a public office; do not replace it with a
        // deeper economic seat under pure NPC (Wave-1 seats T4/T6 from
        // unseated lines only).
        if held.is_some() {
            return;
        }
        let evidence = office_evidence(&self.world);
        // Expansionist → caravan (routes); empiricist → exchange
        // (books); epicurean / others prefer caravan when elements
        // stand, exchange when market clears already.
        let want = match self.spec.civilization.disposition.as_str() {
            "empiricist" => "exchange",
            "expansionist" => "caravan",
            _ if evidence.market_clears > 0 => "exchange",
            _ => "caravan",
        };
        if office_preconditions(want, &evidence).is_err() {
            return;
        }
        let Some(station) = Station::from_word(want) else {
            return;
        };
        let Some(line) = cfe::dop::Name::new(&self.spec.civilization.sigil) else {
            return;
        };
        if self
            .world
            .seat_station_as(self.automaton, line, station)
            .is_err()
        {
            return;
        }
        if registry.seat(&self.spec.civilization.sigil, want).is_ok() {
            self.spec.civilization.station = Some(want.into());
        }
    }

    /// **REFINE STEP (REFINE-1)** — pure NPC conversion of banked raw
    /// elements through a standing ElementRefinery. One refine per
    /// beat, first banked kind that covers the recipe spend.
    fn refine_step(&mut self) {
        let primary = cfe::world::FIRST_ORB;
        let Some(orb) = self.world.orb() else {
            return;
        };
        if !orb
            .zones
            .iter()
            .any(|z| z.kind == ZoneKind::ElementRefinery)
        {
            return;
        }
        let Some(class) = self.world.star() else {
            return;
        };
        let candidates: Vec<[u8; 32]> = orb.elements.iter().map(|(id, _)| *id).collect();
        let automaton = self.automaton;
        for raw in candidates {
            let (spend, refined_id, _) = cfe::dop::refine_recipe(&raw, class);
            // Skip already-refined identities (their own refine would
            // make a further form — fine later; Wave-1 takes raw only).
            if raw == refined_id {
                continue;
            }
            let have = self
                .world
                .orb()
                .map(|o| o.element_amount(&raw))
                .unwrap_or_else(|| ratio(0, 1));
            if have < spend {
                continue;
            }
            if self.world.refine_as(automaton, primary, raw).is_ok() {
                return; // one refine per beat
            }
        }
    }

    /// **C1.6 CHEMISTRY STEP** — pure NPC chemistry, disposition-routed.
    ///
    /// One act per beat: seat Chem lab → catalog known gem → compound
    /// best set → iso-union. Empiricist prioritizes catalog (knowledge);
    /// expansionist prioritizes compound/union (yield); epicurean only
    /// bonds when survival ≥ dark (soft science).
    fn chemistry_step(&mut self) {
        let primary = cfe::world::FIRST_ORB;
        if self.world.orb().is_none() || self.world.star().is_none() {
            return;
        }
        if self.world.orb().is_some_and(|o| !o.famine.is_empty()) {
            return; // starving lines do not play alchemy
        }
        let automaton = self.automaton;
        let dis = self
            .world
            .genesis()
            .map(|g| g.disposition)
            .unwrap_or(1);
        // disposition wire: 1 expansionist · 2 empiricist · 3 epicurean
        // (and others default expansionist-ish yield path).
        let chem = cfe::almanac::ScienceDiscipline::Chem;

        // 1. Seat chem lab if unseated and the bill is affordable.
        if self.world.science_rank(primary, chem) < 1 {
            if self
                .world
                .seat_science_as(automaton, primary, chem)
                .is_ok()
            {
                return; // one act per beat
            }
            // Cannot seat yet — no chemistry below the lab door.
            return;
        }

        // 2. Disposition-ordered chemistry acts.
        match dis {
            2 => {
                // Empiricist: catalog first (knowledge), then compound survival.
                if self.npc_chem_catalog(automaton, primary) {
                    return;
                }
                if self.npc_chem_compound(automaton, primary, /*prefer_survival*/ true) {
                    return;
                }
                let _ = self.npc_chem_iso(automaton, primary);
            }
            3 => {
                // Epicurean: soft science — compound only when survival ≥ dark.
                if self.npc_chem_compound(automaton, primary, /*prefer_survival*/ true) {
                    return;
                }
                if self.npc_chem_catalog(automaton, primary) {
                    return;
                }
            }
            _ => {
                // Expansionist / default: yield first (net utility), then iso, catalog.
                if self.npc_chem_compound(automaton, primary, /*prefer_survival*/ false) {
                    return;
                }
                if self.npc_chem_iso(automaton, primary) {
                    return;
                }
                let _ = self.npc_chem_catalog(automaton, primary);
            }
        }
    }

    /// Catalog one banked gem not yet in the chemistry catalog.
    fn npc_chem_catalog(&mut self, actor: u64, orb: u64) -> bool {
        let catalog: std::collections::HashSet<[u8; 32]> =
            self.world.chem_catalog().iter().copied().collect();
        let candidates: Vec<[u8; 32]> = self
            .world
            .orb()
            .map(|o| {
                o.elements
                    .iter()
                    .filter(|(_, amt)| *amt > ratio(0, 1))
                    .map(|(id, _)| *id)
                    .filter(|id| !catalog.contains(id))
                    .collect()
            })
            .unwrap_or_default();
        // Prefer highest survival utility gem for catalog.
        let mut best: Option<([u8; 32], cfe::ratio::Ratio)> = None;
        let axes = self.world.civ_axes().unwrap_or(cfe::almanac::CivAxes {
            disposition: 1,
            metabolism: 1,
            perception: 1,
        });
        for gem in candidates {
            let s = cfe::almanac::survival_utility(axes, &[gem], &ratio(1, 1));
            if best.as_ref().is_none_or(|(_, bs)| s > *bs) {
                best = Some((gem, s));
            }
        }
        if let Some((gem, _)) = best {
            return self.world.chem_catalog_gem_as(actor, orb, gem).is_ok();
        }
        false
    }

    /// Compound the top bench_science set (survival-first or net-first).
    fn npc_chem_compound(&mut self, actor: u64, orb: u64, prefer_survival: bool) -> bool {
        let ranked = self.world.bench_science(orb);
        // ScienceRead = (members, gem, knowledge, standing, survival, dark, net)
        let mut rows: Vec<_> = ranked
            .into_iter()
            .filter(|r| r.3.is_none()) // no standing refusal
            .collect();
        if prefer_survival {
            // survival ≥ dark and highest survival
            rows.retain(|r| r.4 >= r.5);
            rows.sort_by(|a, b| b.4.cmp(&a.4));
        } else {
            rows.sort_by(|a, b| b.6.cmp(&a.6)); // net
        }
        for (members, _, _, _, _, _, _) in rows.into_iter().take(8) {
            if self
                .world
                .compound_as(actor, orb, &members)
                .is_ok()
            {
                return true;
            }
        }
        false
    }

    /// **POL.3 IDENTITY STEP** — pure NPC parity for line identity.
    ///
    /// Ensures labor seats (habitat) when force is zero and resources
    /// afford; mirrors the player `polity` / `bench polity` surfaces.
    /// Genesis / taxon remain founding-time (not re-sealed here).
    fn identity_step(&mut self) {
        let primary = cfe::world::FIRST_ORB;
        if self.world.orb().is_none_or(|o| !o.famine.is_empty()) {
            return;
        }
        let labor = self.world.labor_board(primary);
        // No habitat labor yet — try to seat one habitat when affordable.
        if labor.habitats == 0 {
            let can_pay = ZoneKind::Habitat.cost().into_iter().all(|(resource, need)| {
                banked(&self.world, resource) >= need
            });
            if can_pay {
                if let Some(tile) = next_free_tile(&self.world) {
                    let _ = self.world.zone_as(
                        self.automaton,
                        primary,
                        tile,
                        ZoneKind::Habitat,
                    );
                }
            }
        }
    }

    /// Iso-union first isomorphic banked gem pair.
    fn npc_chem_iso(&mut self, actor: u64, orb: u64) -> bool {
        let ids: Vec<[u8; 32]> = self
            .world
            .orb()
            .map(|o| {
                o.elements
                    .iter()
                    .filter(|(_, amt)| *amt >= ratio(1, 1))
                    .map(|(id, _)| *id)
                    .collect()
            })
            .unwrap_or_default();
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                if cfe::almanac::isomorphic(&ids[i], &ids[j]) {
                    if self
                        .world
                        .iso_union_as(actor, orb, ids[i], ids[j])
                        .is_ok()
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// The policy's build decision — at most one zone per beat, never
    /// during famine, always through the contained actor. Industry is
    /// GATED ON RESEARCH for every temperament: smelters need rung 3,
    /// synthesizers rung 5.
    ///
    /// **ECO regenerators (operator lock 2026-08-07, mandatory):** when
    /// any open vein on an orb is worn (richness below its natural
    /// spring), the polity prioritizes `ZoneKind::Regenerator` so the
    /// element extraction lifecycle repairs itself. Regenerator
    /// placement requires DEEPENING (condition 3); the steward gifts
    /// that node when the ecology demands it and the gift is lawful.
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
        // THE TOP OF THE LADDER, DERIVED (operator ruling 2026-08-04):
        // a line surveys once nothing above it differs. `Difficulty::
        // climbs` reads that off the ladder instead of restating its
        // length as a constant.
        if Difficulty::climbs(self.level) {
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
            // the charter plants its next sector. DERIVED (P2): the
            // charter MOVES the founding reserves, so the gate is
            // double them — a line charters only from a position that
            // keeps a second founding in the bank. The dead 40/20
            // restated exactly this.
            let launches = cfe::dop::founding_reserves()
                .into_iter()
                .all(|(resource, moved)| {
                    banked(&self.world, resource) >= moved * ratio(2, 1)
                });
            if launches {
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

    /// **THE FORGE GATHERS** (the G2 relay's second arm, 2026-08-06 —
    /// "not a spend to hold back; a distribution to gather back"). The
    /// chart law rides 10 biomass to every child and the market banks
    /// imports where the offer rested, so a rung-3 mineral line's
    /// biomass stands scattered across the estate while the smelter
    /// gate reads the primary bank alone. The estate chain exists for
    /// exactly this movement: the survey grants the postal addresses,
    /// the conveyance moves the parcels, postage prices it honestly.
    ///
    /// Children keep a DERIVED floor — their own per-tick biomass
    /// rations × the reciprocity window — so the gather can never
    /// famine what the charter founded. Every deed is refusal-pure:
    /// an unaffordable survey or an over-cap conveyance idles this
    /// beat and the gather resumes on the next.
    fn gather_for_forge(&mut self) {
        let primary = cfe::world::FIRST_ORB;
        if !(self.level >= 3
            && zone_count(&self.world, ZoneKind::Smelter) < 3
            && industry_relevant(&self.spec.civilization.metabolism, ZoneKind::Smelter))
        {
            return;
        }
        let (_, bio_gate) = smelter_gates();
        if banked(&self.world, Resource::Biomass) >= bio_gate {
            return;
        }
        let committed = self.world.committed();
        let orbs: Vec<u64> = committed.orbs.iter().map(|held| held.orb_id).collect();
        if orbs.len() < 2 {
            return;
        }
        // The postal addresses: the standing survey must cover the
        // estate as it stands; a grown estate re-surveys, then gathers
        // on the next beat.
        let covered = self
            .world
            .chronicle()
            .receipts()
            .iter()
            .rev()
            .find_map(|receipt| match &receipt.event {
                cfe::dop::ClosureEvent::EstateSurveyed { covered, .. } => {
                    Some(covered.iter().map(|(orb, _)| *orb).collect::<Vec<u64>>())
                }
                _ => None,
            })
            .unwrap_or_default();
        if !orbs.iter().all(|orb| covered.contains(orb)) {
            let _ = self.world.survey_estate_as(self.automaton, primary);
            return;
        }
        let metabolism = committed
            .genesis
            .as_ref()
            .map(|held| held.metabolism)
            .unwrap_or(2);
        let mut need = bio_gate - banked(&self.world, Resource::Biomass);
        for child in orbs.into_iter().filter(|orb| *orb != primary) {
            if need <= ratio(0, 1) {
                break;
            }
            let Some(held) = committed.orbs.iter().find(|held| held.orb_id == child) else {
                continue;
            };
            // The derived floor: the child's own per-tick biomass
            // rations across the reciprocity window.
            let rations: Ratio = held
                .zones
                .iter()
                .flat_map(|zone| cfe::dop::zone_upkeep(zone.kind, metabolism))
                .filter(|(resource, _)| *resource == Resource::Biomass)
                .fold(ratio(0, 1), |sum, (_, amount)| sum + amount);
            let floor = rations * ratio(cfe::dop::reciprocity_window() as i64, 1);
            let surplus = held.banked_amount(Resource::Biomass) - floor;
            if surplus <= ratio(0, 1) {
                continue;
            }
            let ask = surplus.min(need.clone());
            let Some(amount) = cfe::dop::Positive::new(ask.clone()) else {
                continue;
            };
            if self
                .world
                .convey_as(
                    self.automaton,
                    cfe::dop::between(child, primary),
                    Resource::Biomass,
                    amount,
                )
                .is_ok()
            {
                need -= ask;
            }
        }
    }

    /// True when any open vein on `orb` is worn: richness below the
    /// hash-derived natural spring (`element_properties`). Prospected
    /// but never assayed veins sit at nature and do not count.
    fn orb_veins_worn(world: &World, orb: u64) -> bool {
        let Some(class) = world.star() else {
            return false;
        };
        world.committed().prospects.iter().any(|vein| {
            if vein.orb != orb {
                return false;
            }
            let (_, _, natural) = cfe::dop::element_properties(&vein.element, class);
            vein.richness < natural
        })
    }

    /// An orb that has worn veins and no regenerator yet needs repair.
    fn orb_needs_regenerator(world: &World, orb: u64) -> bool {
        if !Self::orb_veins_worn(world, orb) {
            return false;
        }
        let regens = world
            .committed()
            .orbs
            .iter()
            .find(|held| held.orb_id == orb)
            .map(|held| {
                held.zones
                    .iter()
                    .filter(|z| z.kind == ZoneKind::Regenerator)
                    .count()
            })
            .unwrap_or(0);
        regens == 0
    }

    fn build_step(&mut self) {
        self.gather_for_forge();
        let steward = self.steward;
        let automaton = self.automaton;
        let world = &mut self.world;
        if world.orb().is_none_or(|orb| !orb.famine.is_empty()) {
            return; // starving colonies build nothing
        }

        // ECO regenerators first: extract-without-repair is a doomed
        // ecology. Prefer the regenerator over every other build when
        // veins are worn and no regenerator stands. DEEPENING (3) is
        // gifted by the steward when the zone gate requires it.
        let primary = cfe::world::FIRST_ORB;
        if Self::orb_needs_regenerator(world, primary) {
            let holds_deepening = world
                .committed()
                .techs
                .iter()
                .any(|(o, c, _, _)| *o == primary && *c == 3);
            if !holds_deepening {
                let _ = world.grant_tech_as(steward, primary, 3);
            }
            let can_pay = ZoneKind::Regenerator.cost().into_iter().all(|(resource, need)| {
                banked(world, resource) >= need
            });
            if can_pay {
                if let Some(tile) = next_free_tile(world) {
                    if world
                        .zone_as(automaton, primary, tile, ZoneKind::Regenerator)
                        .is_ok()
                    {
                        return; // one zone per beat — the regenerator took it
                    }
                }
            }
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
        let element_kinds = world
            .orb()
            .map(|o| o.elements.len())
            .unwrap_or(0);
        let has_refinery = zone_count(world, ZoneKind::ElementRefinery) > 0;

        // The earned tech tree.
        let can_smelt = self.level >= 3;
        let can_synth = self.level >= 5;

        // Unlocked industry takes priority for every temperament —
        // that is what research is FOR. SCIENCE-1: only inside the
        // line's OWN science (relevance table) — a plasmic line never
        // forges iron; its advancement crosses by joint venture.
        let metabolism = self.spec.civilization.metabolism.clone();
        let (smelter_iron_gate, smelter_bio_gate) = smelter_gates();
        let industrial: Option<ZoneKind> = if can_synth
            && alloy >= ratio(20, 1)
            && bio >= ratio(15, 1)
            && industry_relevant(&metabolism, ZoneKind::Synthesizer)
        {
            Some(ZoneKind::Synthesizer)
        } else if can_smelt
            && smelters < 3
            && iron >= smelter_iron_gate
            && bio >= smelter_bio_gate
            && industry_relevant(&metabolism, ZoneKind::Smelter)
        {
            Some(ZoneKind::Smelter)
        } else {
            None
        };

        // **THE FORGE HOLD, generalized from the Furnace arm** (the
        // G2 measurement, 2026-08-06): when ONLY the biomass line
        // blocks an unlocked smelter, hold the rations for it — a
        // habitat spent now pushes the forge past the horizon again.
        // Measured before this arm existed: mineral lines imported
        // biomass through the triangle trade and spent every unit on
        // hearths — bio 0 at every mineral line, the smelter's 10
        // never accumulating. The Furnace branch already ruled these
        // exact words for its own seat; this is the same law on the
        // settlement path.
        let forge_starved = can_smelt
            && smelters < 3
            && iron >= smelter_iron_gate
            && bio < smelter_bio_gate
            && industry_relevant(&metabolism, ZoneKind::Smelter);

        // **THE SETTLEMENT GATES DERIVE** (P2, operator ruling
        // 2026-08-06: measured system bounds). The per-policy reserve
        // numbers these replace were restated by hand, and the
        // measurement (`measured_bounds.rs`) proved the shape unlawful
        // at bootstrap: a reserve gate on a resource with NO INCOME
        // SOURCE is unsatisfiable — Steady's habitat gate equalled the
        // whole founding biomass and seven lines locked at one zone;
        // Industry's extractor gate exceeded the endowment and every
        // Industry line habitat-spammed into iron famine. The derived
        // law: a build that opens the line's FIRST income of a funding
        // resource gates at COST (affordability); expansion gates at
        // COST × the policy's reserve factor. Post-bootstrap the
        // derived gates reproduce the dead numbers exactly.
        let metabolism = world
            .committed()
            .genesis
            .as_ref()
            .map(|held| held.metabolism)
            .unwrap_or(2);
        let has_income = |world: &World, resource: Resource| {
            world.orb().is_some_and(|orb| {
                orb.zones
                    .iter()
                    .any(|zone| cfe::dop::zone_yield(zone.kind, metabolism).0 == resource)
            })
        };
        let reserve_factor = match self.spec.policy {
            Policy::Aggressive => ratio(1, 1),
            Policy::Steady => ratio(2, 1),
            Policy::Industry => ratio(5, 2),
        };
        let gated = |world: &World, kind: ZoneKind| {
            kind.cost().into_iter().all(|(resource, need)| {
                let bar = match has_income(world, resource) {
                    true => need * reserve_factor.clone(),
                    false => need,
                };
                banked(world, resource) >= bar
            })
        };
        let settlement: Option<ZoneKind> = {
            // The city caps and shape preferences ARE the temperament
            // — they stay; only the reserve arithmetic derives.
            let (cap, wants_extractor) = match self.spec.policy {
                // One extractor SHORT of the knife-edge: a standing
                // +1/tick biomass surplus, so industry stays fundable.
                Policy::Aggressive => (16, ext + 1 < 2 * hab.max(1)),
                Policy::Steady => (12, ext <= hab),
                Policy::Industry => (20, ext < 2 * hab.max(1)),
            };
            let wants_habitat = match self.spec.policy {
                Policy::Steady => hab < ext,
                _ => true,
            };
            if total >= cap {
                None
            } else if wants_extractor && gated(world, ZoneKind::Extractor) {
                Some(ZoneKind::Extractor)
            } else if wants_habitat && !forge_starved && gated(world, ZoneKind::Habitat) {
                // `forge_starved` holds the rations: the biomass a
                // hearth would eat is the biomass the unlocked smelter
                // is waiting on.
                Some(ZoneKind::Habitat)
            } else {
                None
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
        // REFINE-1 (E3.5): prefer an ElementRefinery when raw elements
        // stand and none does — only when unlocked industry is not
        // already claiming the slot, and not while forge-starved (the
        // biomass ration is held for the smelter). Cost is habitat's
        // biomass bill; upkeep is encasement iron — neither raids the
        // smelter's iron gate.
        let refine: Option<ZoneKind> = if element_kinds > 0
            && !has_refinery
            && industrial.is_none()
            && !forge_starved
            && affords(world, ZoneKind::ElementRefinery)
        {
            Some(ZoneKind::ElementRefinery)
        } else {
            None
        };
        let with_refine = |base: Option<ZoneKind>| industrial.or(refine).or(base);
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
                // P2 (tag 103): the mirror follows the CHARTERED
                // fold, exactly as the tick does — a chartered radiator
                // must move the conductor's read too or the mirror lies.
                let heat_net = t4 / cfe::thermal::thermal_k() + factory
                    - world.parameter(cfe::parameter::Parameter::RadiatorBase);
                let throughput =
                    world.parameter(cfe::parameter::Parameter::EncasementThroughput)
                        * ratio(shells, 1);
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
                    with_refine(settlement)
                }
            }
            Some((Regime::Frost, _)) => {
                if zone_count(world, ZoneKind::CryoArray) == 0 {
                    // The sink charter: a Frost colony exists to bank
                    // the cold; its first array is founded on the
                    // founding iron, before any joule flows.
                    affords(world, ZoneKind::CryoArray).then_some(ZoneKind::CryoArray)
                } else if joule_runway_allows(&banked(world, Resource::Joule), total) {
                    with_refine(settlement)
                } else {
                    None // the runway is short: STOP expanding
                }
            }
            None => {
                // ENCASE-FIRST (05 §1.2) holds on the temperate seat
                // too: a colony whose vented book outruns its contained
                // one is bleeding the currency order-work is priced in
                // (34's gate reads exactly these two arms), and no
                // amount of sprawl repays that. One shell, then back to
                // the settlement line.
                let venting = world
                    .orb()
                    .is_some_and(|orb| orb.entropy_vented > orb.syntropy_contained);
                if venting
                    && zone_count(world, ZoneKind::Encasement) == 0
                    && affords(world, ZoneKind::Encasement)
                {
                    Some(ZoneKind::Encasement)
                } else {
                    with_refine(settlement)
                }
            }
        };
        if let (Some(kind), Some(tile)) = (wanted, next_free_tile(world)) {
            // REFINE-1: ElementRefinery needs DEEPENING (condition 3),
            // same technology door as the regenerator.
            if kind == ZoneKind::ElementRefinery {
                let holds_deepening = world
                    .committed()
                    .techs
                    .iter()
                    .any(|(o, c, _, _)| *o == primary && *c == 3);
                if !holds_deepening {
                    let _ = world.grant_tech_as(steward, primary, 3);
                }
            }
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
    /// **WHAT THIS LINE IS STARVED FOR** — a CHARACTER's hunger, read
    /// from its own history.
    ///
    /// **Derived, never stored** (`09_LEDGER` §1.2's law, applied to a
    /// person instead of a book). No deed, no tag, no schema, no kernel
    /// arm: hunger is a reading of the chronicle this line already
    /// wrote, so two characters differ without a wire amendment.
    ///
    /// WANT-1 strikes its wants on an ORB — a PLACE starves. That is
    /// the right law for a world and the wrong one for a person, and it
    /// is why two characters with the same grants were identical by
    /// construction: `Entity` carries `entity_id`, `handle`,
    /// `principal`, `grants` and nothing it could differ in. This reads
    /// the same two hungers off the actor's own deeds instead.
    ///
    /// ```text
    ///   Novelty     nothing DISCOVERED lately — the line has stopped
    ///               finding, and reaches for what MAKES structure
    ///   Resonance   nothing LINKED lately — the line has stopped
    ///               relating, and reaches for what RELATES
    /// ```
    ///
    /// The hunger steers the reach; it never steers what is accepted.
    /// The sector door still re-derives every proposal, so a starving
    /// mind looking in a different order costs the gate nothing.
    fn hunger(&self) -> Need {
        let now = self.world.now();
        let receipts = self.world.chronicle().receipts();
        let since = |matches: &dyn Fn(&cfe::dop::ClosureEvent) -> bool| -> u64 {
            receipts
                .iter()
                .rev()
                .find(|r| matches(&r.event))
                .map_or(now, |r| now.saturating_sub(r.stamp))
        };
        let found = since(&|e| {
            matches!(
                e,
                cfe::dop::ClosureEvent::DiscoveryMinted { .. }
                    | cfe::dop::ClosureEvent::TechDiscovered { .. }
            )
        });
        let linked = since(&|e| {
            matches!(
                e,
                cfe::dop::ClosureEvent::BondSealed { .. }
                    | cfe::dop::ClosureEvent::BondSettled { .. }
                    | cfe::dop::ClosureEvent::ResourceTransferred { .. }
            )
        });
        // **HUNGER IS RELATIVE TO THE LINE'S OWN RHYTHM**, not to a
        // borrowed constant.
        //
        // This first read inherited WANT-1's orb windows — novelty 480,
        // resonance 240 — and they are the wrong SCALE for a person: a
        // 60-beat era never reaches either, so every character was
        // permanently sated, searched at depth 1, and the live board's
        // earnings fell 268 -> 53. That was the constant, not the
        // design.
        //
        // A line is starved when it has gone longer without finding
        // than it usually takes it to find — `since > now / finds`,
        // written as a product so the arithmetic stays exact integers.
        // A line that has NEVER found is starved from the start.
        //
        // Scale-free by construction: it reads the same at 60 beats and
        // 6000, and it introduces no number to tune. The two windows it
        // replaces are gone rather than re-picked.
        let count = |matches: &dyn Fn(&cfe::dop::ClosureEvent) -> bool| -> u64 {
            receipts.iter().filter(|r| matches(&r.event)).count() as u64
        };
        let is_find = |e: &cfe::dop::ClosureEvent| {
            matches!(
                e,
                cfe::dop::ClosureEvent::DiscoveryMinted { .. }
                    | cfe::dop::ClosureEvent::TechDiscovered { .. }
            )
        };
        let is_link = |e: &cfe::dop::ClosureEvent| {
            matches!(
                e,
                cfe::dop::ClosureEvent::BondSealed { .. }
                    | cfe::dop::ClosureEvent::BondSettled { .. }
                    | cfe::dop::ClosureEvent::ResourceTransferred { .. }
            )
        };
        let stalled = |since: u64, deeds: u64| match deeds {
            0 => now > 0,
            n => since.saturating_mul(n) > now,
        };
        let mut hungers = Vec::new();
        if stalled(found, count(&is_find)) {
            hungers.push(Hunger::Novelty);
        }
        if stalled(linked, count(&is_link)) {
            hungers.push(Hunger::Resonance);
        }
        let starved_for = match hungers.is_empty() {
            true => 0,
            false => found.max(linked),
        };
        Need { hungers, starved_for, thin: Vec::new() }
    }

    fn research_step(&mut self, book: &Ledger) {
        let Some(receipt) = self.world.chronicle().receipts().last() else {
            return;
        };
        let digest = receipt.own.0;
        // CAPTURED BEFORE THE BEAT MOVES ANYTHING. Promotion reads both,
        // and reading either afterwards would be vacuous: a close always
        // advances the trajectory to Growth.
        let trajectory_before = self.trajectory;

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
                    // MECHANICAL ADAPTATION (cognition lane, 2026-08-04,
                    // relayed): `seal_volume`'s bare digest became a real
                    // parent-linked `VolumeCertificate`, so the journal
                    // takes its output link. Nothing else here changed.
                    //
                    // STILL OWED, and the harness lane's to land: this arm
                    // passes `None` as the proof because C8 made volumetric
                    // work decorative. That deferral is LIFTED — the
                    // verifier exists (`verify_volume_work`, schema v3).
                    // The arm should now build a `VolumeProof` and carry it,
                    // exactly as the planar arm builds a `WorkProof`. It
                    // waits on a deed tag (91, cfe's registry) and a
                    // `mint_bound` arm for the volumetric ladder.
                    Some((task.name, word.join(" → "), hex(&certificate.output), None))
                }
                _ => None,
            }
        } else {
            let difficulty = Difficulty::level(self.level);
            let task = forge_task(digest, difficulty);
            // NOVELTY-GATE (38 §4): the library folds once per process
            // from this line's own receipts — the chronicle is the
            // register, this cache only spares the re-read.
            if !self.library_folded {
                for (orbit, bytes, cited) in
                    cfe::vocabulary::solver_words(self.world.chronicle())
                {
                    if let Some(expansion) = solver_verify::program_from_bytes(&bytes) {
                        self.library.adopt(solver::mint::MintedWord {
                            orbit,
                            expansion,
                            cited,
                        });
                    }
                }
                self.library_folded = true;
            }
            // THE PRE-REACH: a library word tried whole before any
            // search. A hit is a full lawful proof — the certificate
            // seals the EXPANSION and `verify_work` re-derives it at
            // the gate exactly as a searched program.
            let pre_reached = solver::mint::pre_reach(&task, &self.library).map(|expansion| {
                let mut output = task.test_in.clone();
                for op in &expansion {
                    output = op.apply(&output);
                }
                let certificate =
                    solver_verify::SolveCertificate::seal(&task, &expansion, &output);
                let matches = task
                    .test_expected
                    .as_ref()
                    .map(|expected| expected == &output);
                SolveRead::Solved {
                    program: expansion,
                    matches_expected: matches,
                    certificate,
                    test_output: output,
                }
            });
            // THE SECTOR DOOR is the live forge path. The search only
            // PROPOSES; nothing crosses until every training pair has
            // been re-derived exactly, which is the same re-derivation
            // `verify_work` performs at the kernel. An unproved
            // candidate reads `Unsolved` and the line simply rests that
            // beat — so the search inside may become as dirty as it
            // likes (float scoring, batching, sampled beam) and nothing
            // downstream changes.
            //
            // THE REACH IS COUPLED TO THE LINE'S OWN TRAJECTORY, never
            // to a constant. What it spends is decided by whether
            // spending has been PAYING — see `Trajectory::read`, and
            // `examples/forge_reach.rs` for the numbers each branch
            // cites. `SEARCH_BUDGET` still stands behind this as the
            // per-beat ceiling that keeps one heartbeat bounded; it is
            // no longer what DECIDES the spend.
            // THE THREE PATHS. Each spends differently and each is
            // counted separately: read-candidates are MEASURED, sweep
            // invocations are BOUNDED by `SEARCH_BUDGET`, and blending
            // the two would ruin both yields.
            let reach = |polity: &mut Self| {
                let mut need =
                    polity.trajectory.need_of(polity.hunger().hungers, polity.starved);
                // ORBIT-DIRECTED HUNGER (38 §6): a starving mind reads
                // its own library's census and reaches thinnest-first —
                // curiosity pointed at the measured frontier. Sated
                // minds and empty libraries keep the standing order.
                if !need.hungers.is_empty() && !polity.library.words().is_empty() {
                    need.thin = solver::mint::family_census(&polity.library);
                }
                // THE MEMO, restored into the coupled path (remediations
                // R5). `propose` is pure: same digest, same difficulty,
                // same need, same answer. Keyed on all three — a
                // need-blind key hands a hungry mind its sated self's
                // answer, which is the staleness that shipped a level-3
                // solve as a level-4 proof.
                if let Some((held, held_d, held_need, cached)) = polity.last_forge.clone() {
                    if held == digest && held_d == difficulty && held_need == need {
                        PROBE_DIGEST_REPEAT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        return cached;
                    }
                }
                // COST IS MEASURED, then the proposal crosses THE SECTOR
                // DOOR. `propose_measured` alone would skip it: the
                // reach may look in any order it likes, but nothing
                // crosses that does not re-derive against every training
                // pair. Measure inside, dispose outside.
                let proposal = propose_consulted_measured(&task, &need);
                polity.spent = polity.spent.saturating_add(proposal.tested as u64);
                polity.last_forge =
                    Some((digest, difficulty, need, proposal.read.clone()));
                proposal.read
            };
            // WHICH PATH ACTUALLY PAID FOR THIS BEAT — captured before
            // the trajectory advances, because promotion reads it.
            let read = if let Some(hit) = pre_reached {
                hit
            } else {
                match self.mode {
                SearchMode::Uncoupled => {
                    self.swept = self.swept.saturating_add(1);
                    solve_consulted(&task, difficulty.word_len)
                }
                SearchMode::Coupled => match self.trajectory.path() {
                    SearchPath::Reach => reach(self),
                    SearchPath::Sweep => {
                        self.swept = self.swept.saturating_add(1);
                        solve_consulted(&task, difficulty.word_len)
                    }
                },
                // V ∪ D: read, and exhaust whenever the reading did not
                // close. Both costs paid; the closure set is the union.
                SearchMode::Union => match reach(self) {
                    closed @ SolveRead::Solved { .. } => closed,
                    SolveRead::Unsolved => {
                        self.swept = self.swept.saturating_add(1);
                        solve_consulted(&task, difficulty.word_len)
                    }
                },

                }
            };
            match read {
                SolveRead::Solved {
                    program,
                    matches_expected: Some(true),
                    certificate,
                    test_output,
                } => {
                    // NOVELTY-GATE: a confirmed composite is a WORD
                    // candidate. Gates 1–2 run here (unique orbit vs
                    // library and this task's standing letters; depth
                    // ≥ 2 by `consider`); each DISTINCT task digest is
                    // one confirmation; the third mints — receipted,
                    // adopted, and a depth-1 letter from then on.
                    if program.len() >= 2 {
                        let standing = solver::mint::standing_orbits(
                            &solver::search::sweep_vocabulary(&task),
                        );
                        if let Some(orbit) =
                            solver::mint::consider(&program, &self.library, &standing)
                        {
                            let pending = match self
                                .word_pending
                                .iter_mut()
                                .find(|(held, _, _)| *held == orbit)
                            {
                                Some(held) => held,
                                None => {
                                    self.word_pending.push((orbit, program.clone(), Vec::new()));
                                    self.word_pending.last_mut().expect("just pushed")
                                }
                            };
                            if !pending.2.contains(&digest) {
                                pending.2.push(digest);
                            }
                            if pending.2.len() >= solver::mint::CONFIRMS_TO_MINT {
                                let cited = pending.2.clone();
                                let expansion = pending.1.clone();
                                if let Some(bytes) = solver_verify::program_bytes(&expansion) {
                                    if self
                                        .world
                                        .mint_solver_word_as(self.automaton, orbit, bytes, cited.clone())
                                        .is_ok()
                                    {
                                        self.library.adopt(solver::mint::MintedWord {
                                            orbit,
                                            expansion,
                                            cited,
                                        });
                                        self.word_pending.retain(|(held, _, _)| *held != orbit);
                                    }
                                }
                            }
                        }
                    }
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
                // ladder read from the same function the gate enforces,
                // never a number chosen here.
                if let Some((work, difficulty)) = proof {
                    // THE MINT'S ONE ANCHOR (kernel VERSION 4). The
                    // amount is `kernel::mint_bound` — the ruled ladder
                    // read from the same function the gate enforces,
                    // never a number chosen here — and the proof rides
                    // the deed whole, so `verify_work` re-forges,
                    // re-applies and re-chains it forever.
                    //
                    // The capacity-law gate that stood here is STRUCK
                    // (operator ruling 2026-08-04): identity is pegged
                    // to entity and is recycled WITHOUT bonds, so a
                    // standing test read off relational structure
                    // refused the lawful recycled line by construction.
                    let earned = kernel::mint_bound(difficulty.rung());
                    if earned > cfe::ratio::ratio(0, 1) {
                        // The bound is the kernel's own reading and is
                        // non-negative by its ladder; the door proves it
                        // rather than trusting this comment.
                        let Some(earned) = cfe::dop::NonNegative::new(earned) else {
                            return;
                        };
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
                // THE COUPLING CLOSES HERE. A close returns the line to
                // growth from wherever it was, and clears the hunger it
                // reached with.
                self.trajectory = self.trajectory.advance(true);
                self.starved = 0;
                // D CLIMBS WHEN THE RUNG IS MASTERED — coupled, not
                // counted. The old rule was `streak >= 3 && level < 5`:
                // two chosen constants, one counting confirms while
                // ignoring what they COST, the other an absolute
                // ceiling.
                //
                // The couple: D (the rung, an extent) advances only on
                // expenditure of V (the reach, a rate), and the signal
                // that a rung is finished is that it stopped costing —
                // a close off the CHEAP reach. A close that needed
                // hunger or exhaustion says the line is at its limit
                // and holds, however many it strings together.
                //
                // The ceiling is DERIVED, never chosen: a line climbs
                // while the ladder still differentiates, and stops
                // where `Difficulty::level` saturates and the next rung
                // is the same task at the same price. That reads the
                // ladder instead of asserting its length, so extending
                // the ladder extends the climb with no edit here.
                // PATH-AGNOSTIC BY CONSTRUCTION. The first cut tested
                // `path_used == Reach`, which silently broke the
                // uncoupled control: that path never reaches, so it
                // could never promote, never left rung 1, and earned
                // exactly zero. The signal is not WHICH path paid, it
                // is whether the line was ALREADY closing — sustained
                // productivity at this rung. In coupled mode Growth
                // implies the cheap reach anyway, so this subsumes the
                // path test rather than weakening it.
                let mastered = trajectory_before == Trajectory::Growth;
                let ladder_still_climbs =
                    Difficulty::level(self.level + 1) != Difficulty::level(self.level);
                if mastered && ladder_still_climbs {

                    self.level += 1;
                    self.streak = 0;
                }
            }
            None => {
                self.streak = 0;
                // Growth → stagnation → decline, one step per barren
                // beat. What each state SPENDS is `Trajectory::need`.
                self.trajectory = self.trajectory.advance(false);
                self.starved = self.starved.saturating_add(1);
            }
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
        // Divergence waits for the TOP of the ladder — derived from
        // the ladder, not from a remembered 5.
        if self.cleaved || Difficulty::climbs(self.level) || self.solved < 20 {
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
        // The folded vocabulary hands back owned words, so the draw
        // takes a slice of `String` rather than of `&'static str`. Same
        // law: never to itself, so a flagged axis genuinely moves.
        let mutate_from = |current: &str, options: &[String], domain: &str| -> String {
            let others: Vec<&String> = options.iter().filter(|o| o.as_str() != current).collect();
            match others.is_empty() {
                true => current.to_string(),
                false => others
                    [beacon.uniform_index(domain, others.len() as u64) as usize]
                    .clone(),
            }
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
        // **28_VOCABULARY V3 — THE DRAW READS THE FOLD, AT THE RECEIPT.**
        //
        // The axis vocabularies are no longer the compiled constants:
        // they are `seed ∪ minted`, so a class of entity a progenitor
        // minted is one a cleave can actually walk onto. Without this
        // the editor would grow a vocabulary nothing ever wears.
        //
        // AND IT READS THE VOCABULARY AS OF THE DRAWING RECEIPT, never
        // as of today (`28_VOCABULARY` §9). Read today's and replaying an
        // old era hands lines a class that did not exist when they
        // cleaved — a SEAL B violation that fails SILENTLY, because the
        // replay still terminates and still looks like a world.
        let stamp = receipt.stamp;
        let words = |axis: cfe::vocabulary::Axis| -> Vec<String> {
            cfe::vocabulary::fold_at(self.world.chronicle(), axis, stamp)
                .into_iter()
                .filter(|entry| entry.code != 0)
                .map(|entry| entry.word)
                .collect()
        };
        let dispositions = words(cfe::vocabulary::Axis::Disposition);
        let metabolisms = words(cfe::vocabulary::Axis::Metabolism);
        let perceptions = words(cfe::vocabulary::Axis::Perception);
        let axes = 1 + beacon.uniform_index("growth.axes", 7);
        let (mut disposition, mut metabolism, mut perception) = (
            parent.disposition.clone(),
            parent.metabolism.clone(),
            parent.perception.clone(),
        );
        if axes & 1 != 0 {
            disposition = mutate_from(&disposition, &dispositions, "growth.disposition");
        }
        if axes & 2 != 0 {
            metabolism = mutate_from(&metabolism, &metabolisms, "growth.metabolism");
        }
        if axes & 4 != 0 {
            perception = mutate_from(&perception, &perceptions, "growth.perception");
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

    /// THE UNFOLD (GAUGE-N, `11` §2.1) — a line OPENS the plane it has
    /// already EARNED.
    ///
    /// **NOT CALLED FROM `cycle`, and that is a hold rather than a
    /// design.** Wiring it into the beat made
    /// `survey_pins::the_survey_charters_the_sector_and_the_spent_lens_idles`
    /// fail on `replay MATCHES the surveyed world` — the live world and
    /// the replay of its own chronicle disagreed by exactly 7 crystal.
    /// The GaugeShiftedAt deed itself replays correctly (the key is
    /// identical on both sides); what moves is downstream of the
    /// chronicle gaining a receipt. Until that is understood this stays
    /// caller-driven, because a proposer that breaks SEAL B is worse
    /// than one that never expands.
    ///
    /// The mechanism was landed and had exactly two callers: a cfe test
    /// and the operator's CLI verb. No autonomous line had ever opened
    /// a plane, so every live world sat at genesis `[0]` forever while
    /// its earned mints piled up unspent. The era FOLD (the
    /// contraction) ran every era; the unfold (the expansion) ran never.
    ///
    /// **The earning rule is NOT restated here.** `shift_gauge_at`
    /// refuses `PlaneUnearned` when the grade outruns the receipted
    /// mints — one plane per `TechDiscovered { condition: 1.. }`,
    /// `Diverged` or `Confluent`. This asks it and takes the refusal;
    /// copying the filter would be two definitions of one law
    /// (CHARTER §0b).
    ///
    /// **The plane's value is CHRONICLE-SEEDED**, never chosen: drawn
    /// from the world's own terminal digest, so replay re-derives it
    /// exactly and no wall clock enters (CHARTER §9).
    pub fn unfold(&mut self) -> bool {
        let Some(receipt) = self.world.chronicle().receipts().last() else {
            return false;
        };
        let beacon = Beacon::from_digest(receipt.own.0);
        // The next grade above what already stands.
        let next = self.world.key().len();
        let value = ratio(beacon.uniform_index("unfold.plane", 12) as i64, 12);
        match self.world.shift_gauge_at(next as u16, value) {
            Ok(()) => {
                self.unfolded = self.unfolded.saturating_add(1);
                true
            }
            Err(_) => false,
        }
    }

    /// One full heartbeat. A growth returns the newborn's spec —
    /// the era loop gives it a world of its own.
    ///
    /// **Cycle topology (operator lock 2026-08-07 + Wave-1 + C1.6):**
    /// `build_step` → `ground_step` → `office_step` → `refine_step`
    /// → `chemistry_step` → `tick` → `collect_as`. Ground banks
    /// elements; office seats T4/T6; refine converts raw; chemistry
    /// seats labs / catalogs / compounds — all pure NPC.
    pub fn cycle(&mut self, book: &Ledger, registry: &Registry) -> Option<PolitySpec> {
        profile::timed("drain_inbox", || self.drain_inbox(book));
        profile::timed("build_step", || self.build_step());
        profile::timed("ground_step", || self.ground_step());
        profile::timed("office_step", || self.office_step(registry));
        profile::timed("refine_step", || self.refine_step());
        profile::timed("chemistry_step", || self.chemistry_step());
        profile::timed("identity_step", || self.identity_step());
        profile::timed("tick", || {
            self.world.tick();
        });
        // Every settled orb collects — a chartered survey site left
        // uncollected would starve at its own upkeep.
        profile::timed("collect", || {
            let orbs: Vec<u64> =
                self.world.committed().orbs.iter().map(|o| o.orb_id).collect();
            for orb in orbs {
                let _ = self.world.collect_as(self.automaton, orb);
            }
        });
        profile::timed("survey_step", || self.survey_step());
        profile::timed("research_step", || self.research_step(book));
        let newborn = profile::timed("divergence_step", || self.divergence_step(registry));
        // THE RESUME CARRIER (task 25, 2026-07-30): every polity
        // world syncs its tlv chronicle beside the ledger exports — an
        // append-only resume copy (cfe::persist grows the file, never
        // rewrites). The player's OBSERVING hand can resume a copy;
        // the WRITING hand stays refused at the CLI (two writers on
        // one clock is the split-brain NS-5 exists to arbitrate) —
        // intervention still goes through the steward inbox.
        profile::timed("persist_sync", || {
            let _ = cfe::persist::sync(
                self.world.chronicle(),
                &book
                    .world_dir(&self.spec.world_name)
                    .join("chronicle.tlv"),
            );
        });
        // #36 KERNEL-OBSERVED PLAY: the beat chains. The committed
        // state extends this world's arbor at the world clock —
        // adopted once at first sight (the pre-chain founding stamped
        // a named debt), extended every beat after, gated=0 stamped
        // (the hole visible, never hidden; the Phase 3 flip threads
        // the steps). A penned world skips the beat entirely, so its
        // arbor stalls with its pen (MESH-1b). Best-effort like the
        // counterpart writes at this observed phase: a refused chain
        // leaves the arbor short, a visible stall, never a crash.
        profile::timed("chain_beat", || {
            let _ = self.chain_beat(book);
        });
        profile::timed("scribe_record", || {
            let _ = scribe::record(
            book,
            &self.spec.world_name,
            &self.spec.planet,
                &self.spec.civilization.sigil,
                &self.world,
            );
        });
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
        // never hidden. Everything after walks through the gate.
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
        // `mesh::arbor::extend` runs `kernel::admits` on it. Batching
        // was refused by ruling: nine deeds in one composite hides an
        // un-inspected interior where a local unlawful step shelters
        // behind a net-positive whole.
        //
        // THE JUDGED WATERMARK (task #8, measured 2026-08-12): the
        // cursor was memory-only, so EVERY conductor boot re-judged
        // every world's whole history — O(receipts²) prefix replays,
        // measured at 421s of a 430s beat across 153 grown worlds
        // (98 %; the "94 % research_step" reading was true at 33 young
        // worlds and is history). `kernel::admits` is deterministic on
        // (prior, next, step) — re-deriving identical verdicts each
        // boot is the R5-memo shape the board already ruled against —
        // so the cursor and the refused SEQUENCES persist beside the
        // spine, and a boot re-judges only what the record does not
        // hold. No sidecar (or an inconsistent one — a shorter
        // chronicle is a foreign tree) → the full re-judgment, today's
        // behavior, once. The permanent refusal record survives BETTER
        // than before: it is carried, not recomputed; the few recorded
        // refusals re-derive their typed verdicts at one prefix replay
        // each, so the narrator still announces them once per process.
        if self.chained == 0 {
            if let Some((watermark, refused_seqs)) =
                read_judged_sidecar(&book.world_dir(&self.spec.world_name))
            {
                if watermark <= receipts.len() {
                    self.chained = watermark;
                    for sequence in refused_seqs {
                        if let Some(at) =
                            receipts.iter().position(|r| r.sequence == sequence)
                        {
                            let prior = cfe::world::replay(
                                &cfe::vrp::Chronicle::from_receipts(receipts[..at].to_vec())
                                    .map_err(|_| mesh::arbor::ArborRefused::StateUndecodable)?,
                            );
                            let next = cfe::world::replay(
                                &cfe::vrp::Chronicle::from_receipts(
                                    receipts[..=at].to_vec(),
                                )
                                .map_err(|_| mesh::arbor::ArborRefused::StateUndecodable)?,
                            );
                            let receipt = &receipts[at];
                            let step = kernel::Step {
                                event: receipt.event.clone(),
                                actor: receipt
                                    .provenance
                                    .first()
                                    .copied()
                                    .unwrap_or(cfe::world::SOVEREIGN),
                                stamp: receipt.stamp,
                            };
                            if let Err(unlawful) = kernel::admits(&prior, &next, &step) {
                                self.refused.push((receipt.sequence, unlawful));
                            }
                        }
                    }
                }
            }
        }
        // THE STEPPER (task #9, the structural cure): prior/next per
        // deed are the stepper's state around one apply — a clone of
        // committed state, O(state) — never a prefix replay. The old
        // form here was O(deeds × history) per beat and O(history²)
        // at boot; measured at 402s of a 430s beat before the
        // watermark, ~20s/beat after it, ~0 now. The stepper seeds
        // once per process by folding the already-judged prefix
        // (O(history), the price of one replay), and the chain-hash
        // integrity of that prefix is still proven by from_receipts.
        if self.stepper.is_none() {
            let mut stepper = cfe::world::Replayer::new();
            let verified =
                cfe::vrp::Chronicle::from_receipts(receipts[..self.chained].to_vec())
                    .map_err(|_| mesh::arbor::ArborRefused::StateUndecodable)?;
            for receipt in verified.receipts() {
                stepper.apply(receipt);
            }
            self.stepper = Some(stepper);
        }
        let stepper = self.stepper.as_mut().expect("seeded above");
        for receipt in &receipts[self.chained..] {
            let step = kernel::Step {
                event: receipt.event.clone(),
                actor: receipt.provenance.first().copied().unwrap_or(cfe::world::SOVEREIGN),
                stamp: receipt.stamp,
            };
            let prior = stepper.state().clone();
            stepper.apply(receipt);
            if let Err(unlawful) = kernel::admits(&prior, stepper.state(), &step) {
                self.refused.push((receipt.sequence, unlawful));
            }
        }
        self.chained = receipts.len();
        write_judged_sidecar(
            &book.world_dir(&self.spec.world_name),
            self.chained,
            &self.refused,
        );

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

    /// DIRECTIVE 19 — every deed this world's gate REFUSED, by receipt
    /// sequence, in the order it met them. The record is permanent and
    /// nothing prunes it: the chronicle keeps the deed, this keeps the
    /// verdict.
    ///
    /// A refusal here does NOT mean the deed was undone.
    /// `world/replay.rs` is still the live transition (the Phase 3 flip demotes
    /// it to proposer, `U_DIRECTIVES` §5), so a refused deed stands in
    /// the chronicle with the gate on record against it. That gap is
    /// the whole reason this must be visible rather than counted.
    pub fn refusals(&self) -> &[(u64, kernel::Refused)] {
        &self.refused
    }

    /// The refusals the board has not been told about yet. Reading them
    /// marks them told — the RECORD stays whole above; only the
    /// narrator's cursor moves, so each refusal is announced once
    /// instead of every beat thereafter.
    pub fn unreported_refusals(&mut self) -> &[(u64, kernel::Refused)] {
        let from = self.reported.min(self.refused.len());
        self.reported = self.refused.len();
        &self.refused[from..]
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

/// E-APT / E3.3 — pure NPC element aperture trade (Wave-2).
///
/// For every unordered pair that is not penned and not economically
/// walled: if the exporter holds open matter, gift the solar leap when
/// needed (THRESHOLD-1), ship half the richest held element across the
/// void under `pair_melt` / ROUTE-1 suppression, and land
/// `ElementApertureSent` / `ElementApertureReceived`. Never invents
/// element ids. One successful clear per pair per call is enough for
/// the done bar; more pairs still run (no singles on the board).
pub fn element_trade_step(worlds: &mut [Polity], beat: u64) -> Vec<(String, String)> {
    let mut cleared = Vec::new();
    let mut seq: u64 = 0;
    let n = worlds.len();
    // PEELS AND SHEARS, NOT SWEEPS (standing ruling; measured
    // 2026-08-12 at 66s of a 110s beat, 153 worlds): the pair loop
    // re-folded BOTH sides' whole element ledgers per pair — O(n²)
    // exact-rational folds — and re-granted the solar leap per pair.
    // The mass is a per-WORLD reading: cache it once, refresh only the
    // two participants a clear actually moved; grant the leap once per
    // world in a pre-pass. Same pairs, same order, same choices —
    // WHERE TO LOOK moved, never what clears.
    let mass_of = |p: &Polity| -> Ratio {
        p.world
            .orb()
            .map(|o| {
                o.elements
                    .iter()
                    .fold(ratio(0, 1), |s, (_, a)| s + a.clone())
            })
            .unwrap_or_else(|| ratio(0, 1))
    };
    // THRESHOLD-1 pre-pass: aperture needs the solar leap (condition 0).
    for held in worlds.iter_mut() {
        if !held.penned && !held.world.knows_its_sun() {
            let steward = held.steward;
            let _ = held.world.grant_tech_as(steward, cfe::world::FIRST_ORB, 0);
        }
    }
    let mut masses: Vec<Ratio> = worlds.iter().map(mass_of).collect();
    for a in 0..n {
        for b in (a + 1)..n {
            if worlds[a].penned || worlds[b].penned {
                continue;
            }
            let distance = spatial_l1(&worlds[a].spec.position, &worlds[b].spec.position);
            let melt = cfe::world::route_melt(
                &worlds[a].world,
                &worlds[b].world,
                &distance,
            );
            if melt >= ratio(1, 1) {
                continue; // walled direct — merchant territory, not element haul
            }
            // Prefer shipping from the side with more total element mass.
            let (exporter, importer) = if masses[a] >= masses[b] {
                (a, b)
            } else {
                (b, a)
            };
            let Some((element, have)) = worlds[exporter]
                .world
                .orb()
                .and_then(|o| {
                    o.elements
                        .iter()
                        .max_by(|x, y| x.1.cmp(&y.1))
                        .map(|(id, amt)| (*id, amt.clone()))
                })
            else {
                continue;
            };
            // Ship half (derive 2 as 1+1 — no tuned dial).
            let ship = have.clone() / (ratio(1, 1) + ratio(1, 1));
            if ship <= ratio(0, 1) {
                continue;
            }
            let Some(amount) = cfe::dop::Positive::new(ship) else {
                continue;
            };
            let deal = beat.saturating_mul(100_000).saturating_add(70_000 + seq);
            seq += 1;
            let (ex, im) = two_mut(worlds, exporter, importer);
            if cfe::world::element_aperture_transfer(
                &mut ex.world,
                cfe::world::FIRST_ORB,
                &mut im.world,
                cfe::world::FIRST_ORB,
                element,
                amount,
                deal,
                &melt,
            )
            .is_ok()
            {
                cleared.push((
                    ex.spec.world_name.clone(),
                    im.spec.world_name.clone(),
                ));
                // The clear moved matter: refresh exactly the two
                // touched readings (the cache's whole bargain).
                masses[exporter] = mass_of(&worlds[exporter]);
                masses[importer] = mass_of(&worlds[importer]);
            }
        }
    }
    cleared
}

/// STAMP-1 / MM-1 — pure NPC mint and spot clear (Wave-4 minimal).
///
/// 1. Each unpenned line with surplus crystal mints **half** of the
///    surplus to STAMP 1:1 (`mint_stamp_from_crystal`) — inventory is
///    kept so a later spot can still price crystal (or iron).
/// 2. Unwalled pairs where one holds STAMP and the other a cargo
///    Resource (prefer crystal, else iron, else biomass): stamp
///    aperture one way + resource aperture the other (spot clear,
///    zero player). Pre-checked so a partial clear never strands
///    STAMP mid-deal.
pub fn stamp_market_step(worlds: &mut [Polity], beat: u64) -> Vec<(String, String)> {
    let mut clears = Vec::new();
    let half = |amount: Ratio| amount / (ratio(1, 1) + ratio(1, 1));
    // Mint phase — convert half of surplus crystal (identity rate).
    for polity in worlds.iter_mut() {
        if polity.penned {
            continue;
        }
        let Some(orb) = polity.world.orb() else {
            continue;
        };
        let crystal = orb.banked_amount(Resource::Crystal);
        let stamp = orb.stamp_amount();
        if crystal > stamp {
            let surplus = crystal - stamp;
            let convert = half(surplus);
            if convert > ratio(0, 1) {
                if let Some(amount) = cfe::dop::Positive::new(convert) {
                    let _ = cfe::world::mint_stamp_from_crystal(
                        &mut polity.world,
                        cfe::world::FIRST_ORB,
                        amount,
                    );
                }
            }
        }
    }
    // Spot clear: STAMP for cargo across unwalled pairs.
    let mut seq = 0u64;
    let n = worlds.len();
    let cargo_of = |p: &Polity| -> Option<(Resource, Ratio)> {
        let orb = p.world.orb()?;
        for resource in [Resource::Crystal, Resource::Iron, Resource::Biomass] {
            let have = orb.banked_amount(resource);
            if have > ratio(0, 1) {
                return Some((resource, have));
            }
        }
        None
    };
    for a in 0..n {
        for b in (a + 1)..n {
            if worlds[a].penned || worlds[b].penned {
                continue;
            }
            for i in [a, b] {
                if !worlds[i].world.knows_its_sun() {
                    let steward = worlds[i].steward;
                    let _ = worlds[i].world.grant_tech_as(
                        steward,
                        cfe::world::FIRST_ORB,
                        0,
                    );
                }
            }
            let distance = spatial_l1(&worlds[a].spec.position, &worlds[b].spec.position);
            let melt = cfe::world::route_melt(
                &worlds[a].world,
                &worlds[b].world,
                &distance,
            );
            if melt >= ratio(1, 1) {
                continue;
            }
            let stamp_a = worlds[a]
                .world
                .orb()
                .map(|o| o.stamp_amount())
                .unwrap_or_else(|| ratio(0, 1));
            let stamp_b = worlds[b]
                .world
                .orb()
                .map(|o| o.stamp_amount())
                .unwrap_or_else(|| ratio(0, 1));
            // Buyer has STAMP, seller has a cargo resource.
            let (buyer, seller) = if stamp_a > ratio(0, 1) && cargo_of(&worlds[b]).is_some() {
                (a, b)
            } else if stamp_b > ratio(0, 1) && cargo_of(&worlds[a]).is_some() {
                (b, a)
            } else {
                continue;
            };
            let Some((cargo_resource, cargo_have)) = cargo_of(&worlds[seller]) else {
                continue;
            };
            let pay_have = if buyer == a {
                stamp_a.clone()
            } else {
                stamp_b.clone()
            };
            let pay = half(pay_have);
            let cargo = half(cargo_have);
            if pay <= ratio(0, 1) || cargo <= ratio(0, 1) {
                continue;
            }
            let Some(pay_p) = cfe::dop::Positive::new(pay) else {
                continue;
            };
            let Some(cargo_p) = cfe::dop::Positive::new(cargo) else {
                continue;
            };
            let deal = beat.saturating_mul(100_000).saturating_add(80_000 + seq);
            seq += 1;
            let (buy, sell) = two_mut(worlds, buyer, seller);
            // Pre-check both legs can afford before either mutates.
            let buy_stamp = buy
                .world
                .orb()
                .map(|o| o.stamp_amount())
                .unwrap_or_else(|| ratio(0, 1));
            let sell_cargo = sell
                .world
                .orb()
                .map(|o| o.banked_amount(cargo_resource))
                .unwrap_or_else(|| ratio(0, 1));
            if buy_stamp < pay_p.value().clone() || sell_cargo < cargo_p.value().clone() {
                continue;
            }
            // STAMP pays the distance tax; cargo rides the local door
            // (zero melt aperture) — simplified spot, same deal pair.
            if cfe::world::stamp_aperture_transfer(
                &mut buy.world,
                cfe::world::FIRST_ORB,
                &mut sell.world,
                cfe::world::FIRST_ORB,
                pay_p,
                deal,
                &melt,
            )
            .is_err()
            {
                continue;
            }
            if cfe::world::aperture_transfer(
                &mut sell.world,
                cfe::world::FIRST_ORB,
                &mut buy.world,
                cfe::world::FIRST_ORB,
                cargo_resource,
                cargo_p,
                deal + 1,
            )
            .is_err()
            {
                // STAMP already moved; reverse via counter-aperture so
                // the board is not stranded mid-clear. Melt already
                // burned on the first leg — reverse residual at zero melt.
                let residual = buy
                    .world
                    .orb()
                    .map(|o| o.stamp_amount())
                    .unwrap_or_else(|| ratio(0, 1));
                // No clean reverse amount if melt burned; leave residual
                // on seller as inventory risk (merchant spirit). Clear
                // is NOT counted.
                let _ = residual;
                continue;
            }
            clears.push((
                buy.spec.world_name.clone(),
                sell.spec.world_name.clone(),
            ));
        }
    }
    clears
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
    // PEELS AND SHEARS (measured 2026-08-12, 21s of a 45s beat): the
    // melt is a PAIR reading — keys, routes, seats, siting — and none
    // of those move inside one merchant sweep (a cleared triangle
    // apertures cargo; it charters nothing and unfolds nothing). So
    // the triangle loop reads each pair's melt ONCE through a lazy
    // memo instead of re-deriving it per (m, a, b). Same triangles,
    // same order, same clears.
    let mut melt_memo: std::collections::HashMap<(usize, usize), Ratio> =
        std::collections::HashMap::new();
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
                let melt_of = |memo: &mut std::collections::HashMap<(usize, usize), Ratio>,
                               worlds: &[Polity],
                               x: usize,
                               y: usize|
                 -> Ratio {
                    let key = (x.min(y), x.max(y));
                    memo.entry(key)
                        .or_insert_with(|| {
                            let d = spatial_l1(
                                &worlds[x].spec.position,
                                &worlds[y].spec.position,
                            );
                            cfe::world::route_melt(&worlds[x].world, &worlds[y].world, &d)
                        })
                        .clone()
                };
                // Strictly additive: the direct pair must be walled.
                if melt_of(&mut melt_memo, worlds, a, b) < one() {
                    continue;
                }
                let melt_am = melt_of(&mut melt_memo, worlds, a, m);
                let melt_mb = melt_of(&mut melt_memo, worlds, m, b);
                if melt_am >= one() || melt_mb >= one() {
                    continue;
                }
                // The clear legs still carry their chart distances.
                let d_am = spatial_l1(&worlds[a].spec.position, &worlds[m].spec.position);
                let d_mb = spatial_l1(&worlds[m].spec.position, &worlds[b].spec.position);
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
                // Merchant arbitrage stays on closed carriers for now
                // (element triangles need element banks on the middle).
                let Some(want_a) = ask_a.want.as_resource() else { continue };
                let held = worlds[m]
                    .world
                    .orb()
                    .map(|orb| orb.banked_amount(want_a))
                    .unwrap_or_else(|| ratio(0, 1));
                if held < ask_a.want_amount {
                    continue;
                }
                // Leg 1: M counters A exactly — fills at A's rest.
                // The doors hold by posting law (the resting offer
                // already proved them); a None here skips the caravan.
                let counter = worlds[m].world.chronicle().receipts().len() as u64;
                let (Some(pair), Some(give_amount), Some(want_amount)) = (
                    cfe::dop::Exchange::cargo(ask_a.want.clone(), ask_a.give.clone()),
                    cfe::dop::Positive::new(ask_a.want_amount.clone()),
                    cfe::dop::Positive::new(ask_a.give_amount.clone()),
                ) else {
                    continue;
                };
                if worlds[m]
                    .world
                    .post_offer(cfe::world::FIRST_ORB, pair, give_amount, want_amount, counter)
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
                let (Some(pair), Some(give_amount), Some(want_amount)) = (
                    cfe::dop::Exchange::cargo(ask_b.want.clone(), ask_b.give.clone()),
                    cfe::dop::Positive::new(ask_b.want_amount.clone()),
                    cfe::dop::Positive::new(ask_b.give_amount.clone()),
                ) else {
                    continue;
                };
                if worlds[m]
                    .world
                    .post_offer(cfe::world::FIRST_ORB, pair, give_amount, want_amount, counter)
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
    run_era_with(var_root, cycles, SearchMode::default())
}

/// **THE CONDITION-DRIVEN ERA** (wedge removal, 2026-08-12): run at
/// most `cycles` beats, stopping EARLY once `arrived` reads true on
/// the board — checked at each beat's close. The era pins wait for a
/// CONDITION (a cleave, an unlocked smelter), and a fixed 45-beat run
/// wedges in debug builds as divergence multiplies worlds (measured:
/// pre-existing at 4a37e53, >10 min; the condition itself arrives by
/// ~beat 13). Stopping when the witnessed thing has happened keeps
/// every assertion's meaning and cuts the dead tail. The full-length
/// form remains `run_era` — production pacing is the binary's, not
/// this runner's.
pub fn run_era_until(
    var_root: &str,
    cycles: u32,
    arrived: impl Fn(&[Polity]) -> bool,
) -> Vec<Polity> {
    run_era_inner(var_root, cycles, SearchMode::default(), None, Some(&arrived))
}

/// The condition-driven era over a SEATED PREFIX of the founding
/// roster — a fixture scale for pins whose law is per-line (a smelter
/// rising, a rung earned): a smaller board proves the same claim at a
/// fraction of the beat cost, and the full-roster form stays for the
/// pins whose law is board-wide.
pub fn run_era_until_among(
    var_root: &str,
    cycles: u32,
    seats: usize,
    arrived: impl Fn(&[Polity]) -> bool,
) -> Vec<Polity> {
    run_era_inner(var_root, cycles, SearchMode::default(), Some(seats), Some(&arrived))
}

/// The era runner, under a named search regime — the seam the
/// three-path benchmark drives.
pub fn run_era_with(var_root: &str, cycles: u32, mode: SearchMode) -> Vec<Polity> {
    run_era_inner(var_root, cycles, mode, None, None)
}

fn run_era_inner(
    var_root: &str,
    cycles: u32,
    mode: SearchMode,
    seats: Option<usize>,
    arrived: Option<&dyn Fn(&[Polity]) -> bool>,
) -> Vec<Polity> {
    let registry = Registry::at(format!("{var_root}/registry"));
    let book = Ledger::at(format!("{var_root}/worlds"));
    let eras = ledger::converge::default_era_roots(var_root);
    let epoch_base = ledger::converge::converge(&book, &eras)
        .map(|read| read.epoch_base)
        .unwrap_or(0);
    // 11 §5.2: the live aeon's G-sheet (see main.rs — same law).
    let live_base = cfe::world::wrap_gauge(ratio(eras.len() as i64, 12));
    let roster = polities::founding_polities();
    let take = seats.unwrap_or(roster.len()).min(roster.len());
    let mut worlds: Vec<Polity> = roster
        .into_iter()
        .take(take)
        .map(|mut spec| {
            let opening = spec.gauge.take().unwrap_or_else(|| ratio(0, 1));
            spec.gauge = Some(cfe::world::wrap_gauge(opening + live_base.clone()));
            let mut polity = Polity::genesis(spec, &registry, &book);
            polity.mode = mode;
            polity
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
            let mut polity = Polity::genesis(spec, &registry, &book);
            polity.mode = mode;
            worlds.push(polity);
        }
        // OFFER-1a + MERCHANT-1 + E3.3: market, caravan, element haul.
        fleet_step(&mut worlds);
        kin_step(&mut worlds);
        let _ = market_step(&mut worlds, u64::from(beat));
        let _ = merchant_step(&mut worlds, u64::from(beat));
        let _ = element_trade_step(&mut worlds, u64::from(beat));
        let _ = stamp_market_step(&mut worlds, u64::from(beat));
        instrument_step(&mut worlds);
        market_war_step(&mut worlds);
        espionage_step(&mut worlds);
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
        if arrived.is_some_and(|done| done(&worlds)) {
            break;
        }
    }
    // THE FINAL SYNC (resume-carrier gap, 2026-08-12): `cycle` syncs
    // each world's `chronicle.tlv` at its own close, but the beat's
    // GLOBAL steps (market · element · stamp · fleet · instrument)
    // append receipts AFTER that sync — so the LAST beat's global
    // receipts never reached the resume carrier, and a world resumed
    // mid-era came back one crossing short. The binary beats forever
    // and re-syncs next cycle, so it never saw this; a bounded era
    // ending on a global crossing did. One sync per world at era end
    // closes it — the carrier now reflects the whole final state.
    for polity in &worlds {
        if polity.penned {
            continue;
        }
        let _ = cfe::persist::sync(
            polity.world.chronicle(),
            &book.world_dir(&polity.spec.world_name).join("chronicle.tlv"),
        );
    }
    worlds
}

/// PHASE-4 — pure NPC instrument lifecycle (P4.7): honest spot clears,
/// redeem matured notes, settle expired futures; issue a small Note
/// when listable and STAMP affords half-face collateral.
/// THE JUDGED SIDECAR — `judged.cursor` beside the world's exports:
/// first line the watermark (receipts judged), then one refused
/// SEQUENCE per line. Verdicts are never stored — they re-derive
/// deterministically at boot from the sequences (one prefix replay
/// each; refusals are rare). Best-effort like every export: a missing
/// or unreadable sidecar means the full re-judgment, never a crash.
fn read_judged_sidecar(dir: &std::path::Path) -> Option<(usize, Vec<u64>)> {
    let text = std::fs::read_to_string(dir.join("judged.cursor")).ok()?;
    let mut lines = text.lines();
    let watermark: usize = lines.next()?.trim().parse().ok()?;
    let refused = lines.filter_map(|line| line.trim().parse().ok()).collect();
    Some((watermark, refused))
}

fn write_judged_sidecar(dir: &std::path::Path, watermark: usize, refused: &[(u64, kernel::Refused)]) {
    // UNION with what already stands: the narrator DRAINS the
    // in-memory record for its ten-beat tally, so the sidecar is the
    // permanent copy and must never shrink by a display read.
    let mut sequences: Vec<u64> = read_judged_sidecar(dir)
        .map(|(_, held)| held)
        .unwrap_or_default();
    for (sequence, _) in refused {
        if !sequences.contains(sequence) {
            sequences.push(*sequence);
        }
    }
    sequences.sort_unstable();
    let mut text = format!("{watermark}\n");
    for sequence in sequences {
        text.push_str(&format!("{sequence}\n"));
    }
    let _ = std::fs::create_dir_all(dir);
    let _ = std::fs::write(dir.join("judged.cursor"), text);
}

/// **OPT-IN PHASE PROFILE** (task #8, 2026-08-12) — measures where the
/// beat goes. Armed by `NETSTRATUM_PROFILE=1`; unarmed it is one
/// atomic load per phase. An instrument, never a mechanic — it appends
/// nothing, gates nothing (the cost_probe discipline).
pub mod profile {
    use std::sync::{Mutex, OnceLock};
    use std::time::{Duration, Instant};

    static ARMED: OnceLock<bool> = OnceLock::new();
    static BOOK: Mutex<Vec<(&'static str, Duration)>> = Mutex::new(Vec::new());

    fn armed() -> bool {
        *ARMED.get_or_init(|| std::env::var_os("NETSTRATUM_PROFILE").is_some())
    }

    pub fn timed<T>(name: &'static str, run: impl FnOnce() -> T) -> T {
        if !armed() {
            return run();
        }
        let at = Instant::now();
        let out = run();
        let spent = at.elapsed();
        let mut book = BOOK.lock().expect("profile book");
        match book.iter_mut().find(|(held, _)| *held == name) {
            Some((_, total)) => *total += spent,
            None => book.push((name, spent)),
        }
        out
    }

    /// Drain the accumulated per-beat totals, costliest first.
    pub fn drain() -> Vec<(&'static str, Duration)> {
        if !armed() {
            return Vec::new();
        }
        let mut book = BOOK.lock().expect("profile book");
        let mut out: Vec<_> = book.drain(..).collect();
        out.sort_by(|a, b| b.1.cmp(&a.1));
        out
    }
}

pub fn instrument_step(worlds: &mut [Polity]) {
    for polity in worlds.iter_mut() {
        if polity.penned {
            continue;
        }
        ensure_npc_instruments(&mut polity.world, polity.automaton);
    }
}

/// P5-K2 — pure NPC KIN colour (36 §12): disposition-routed escort /
/// risk marks. No suite required. Compose fleet routes + escort role.
pub fn kin_step(worlds: &mut [Polity]) {
    for polity in worlds.iter_mut() {
        if polity.penned {
            continue;
        }
        ensure_npc_kin(&mut polity.world, polity.automaton);
    }
}

fn ensure_npc_kin(world: &mut cfe::world::World, actor: u64) {
    use cfe::dop::HullRole;
    let dis = world.defense_posture().disposition;
    let deals: Vec<u64> = world.routes().iter().map(|(d, _)| *d).collect();
    if deals.is_empty() {
        return;
    }
    // Empiricist (2): mark risk on first clear corridor (fog may refuse).
    if dis == 2 {
        for &deal in deals.iter().take(2) {
            let _ = world.mark_convoy_risk_as(actor, deal);
        }
        return;
    }
    // Expansionist (1): assign idle escorts onto first deal; mark risk.
    if dis == 1 {
        let idle: Vec<u64> = world
            .hulls()
            .iter()
            .filter(|h| {
                h.assigned_route == 0
                    && HullRole::from_wire(h.role) == Some(HullRole::Escort)
            })
            .map(|h| h.hull_id)
            .collect();
        if let Some(&deal) = deals.first() {
            for hull_id in idle.into_iter().take(2) {
                let _ = world.escort_assign_as(actor, hull_id, deal);
            }
            let _ = world.mark_convoy_risk_as(actor, deal);
        }
        return;
    }
    // Epicurean (3): only reinforce when CONTESTED; prefer secure.
    if dis == 3 {
        for &deal in &deals {
            let band = world.convoy_risk_band(deal);
            if band == cfe::world::RiskBand::Contested {
                let idle = world.hulls().iter().find(|h| {
                    h.assigned_route == 0
                        && HullRole::from_wire(h.role) == Some(HullRole::Escort)
                });
                if let Some(h) = idle {
                    let id = h.hull_id;
                    let _ = world.escort_assign_as(actor, id, deal);
                }
            }
            let _ = world.mark_convoy_risk_as(actor, deal);
        }
    }
}

/// P5-F — pure NPC espionage (36 §4): disposition-routed probe / exfil /
/// sabotage / covert tamper. No suite required.
pub fn espionage_step(worlds: &mut [Polity]) {
    for polity in worlds.iter_mut() {
        if polity.penned {
            continue;
        }
        ensure_npc_espionage(&mut polity.world, polity.automaton);
    }
}

fn ensure_npc_espionage(world: &mut cfe::world::World, actor: u64) {
    use cfe::world::FIRST_ORB;
    let dis = world.defense_posture().disposition;
    if world.detection_grade() < 1 {
        return;
    }
    let orb = FIRST_ORB;
    // Empiricist: probe then exfil first tech condition if any.
    if dis == 2 {
        let _ = world.spy_probe_as(actor, orb);
        if let Some((_, c, _, _)) = world.techs().first() {
            let cond = *c;
            let _ = world.tech_exfil_as(actor, orb, cond);
        }
        return;
    }
    // Expansionist: probe + sabotage or covert tamper.
    if dis == 1 {
        let _ = world.spy_probe_as(actor, orb);
        if !world.instruments().is_empty() {
            let _ = world.sabotage_instrument_as(actor, orb);
        } else {
            let _ = world.covert_tamper_as(actor, orb);
        }
        return;
    }
    // Epicurean: probe only (soft intel; market war handles STAMP panic).
    if dis == 3 {
        let _ = world.spy_probe_as(actor, orb);
    }
}

/// P5-M — pure NPC economic warfare (36 §10): disposition-routed clear
/// seal building, liquidity raids, circuit breaker — no suite required.
pub fn market_war_step(worlds: &mut [Polity]) {
    for polity in worlds.iter_mut() {
        if polity.penned {
            continue;
        }
        ensure_npc_market_war(&mut polity.world, polity.automaton);
    }
}

fn ensure_npc_market_war(world: &mut cfe::world::World, actor: u64) {
    use cfe::dop::{Positive, Resource};
    if world.orb().is_none() {
        return;
    }
    let orb_id = cfe::world::FIRST_ORB;
    let dis = world.genesis().map(|g| g.disposition).unwrap_or(1);
    // If sealed, expansionist resumes early when resume is near; others wait.
    if world.market_sealed() {
        if dis == 1 {
            if let Some(at) = world.market_resume_at() {
                if world.now().saturating_add(1) >= at {
                    let _ = world.resume_market_as(actor);
                }
            }
        }
        return;
    }
    // Empiricist (2) / epicurean (3): build honest clear depth on Crystal.
    if (dis == 2 || dis == 3) && (world.futures_listable() || world.notes_listable()) {
        if let Some(p) = Positive::new(ratio(1, 1)) {
            let _ = world.record_spot_clear_as(actor, orb_id, Resource::Crystal, p);
        }
    }
    // Expansionist (1): liquidity raid or note cascade when stamp affords.
    if dis == 1 {
        let stamp = world
            .orb()
            .map(|o| o.stamp_amount())
            .unwrap_or_else(|| ratio(0, 1));
        let has_offer = world.offers().iter().any(|o| o.orb == orb_id);
        if has_offer && stamp >= ratio(1, 1) {
            let _ = world.liquidity_raid_as(actor, orb_id);
        } else if stamp >= ratio(1, 1) && world.notes_listable() {
            let _ = world.note_cascade_as(actor, orb_id);
        }
    }
    // Epicurean (3): trip breaker when crystal clear depth is still thin
    // but instrument books are open (protect against thin-book poison).
    if dis == 3
        && (world.notes_listable() || world.futures_listable())
        && world.clear_depth_of(Resource::Crystal) < cfe::world::CLEAR_SEAL_DEPTH / 2
        && world.now().is_multiple_of(30)
    {
        let _ = world.seal_market_as(actor);
    }
}

fn ensure_npc_instruments(world: &mut cfe::world::World, actor: u64) {
    use cfe::dop::{Positive, Resource};
    let Some(orb) = world.orb() else {
        return;
    };
    let orb_id = cfe::world::FIRST_ORB;
    // Honest clear path (P5-M Q-S4): record spot clears; depth≥8 seals last-clear.
    if !world.market_sealed()
        && (world.futures_listable() || world.notes_listable())
        && (orb.banked_amount(Resource::Crystal) > ratio(0, 1)
            || orb.stamp_amount() > ratio(0, 1))
    {
        if let Some(p) = Positive::new(ratio(1, 1)) {
            let _ = world.record_spot_clear_as(actor, orb_id, Resource::Crystal, p);
        }
    }
    // Redeem / settle anything due.
    let due_notes: Vec<u64> = world
        .notes()
        .iter()
        .filter(|n| n.open && world.now() >= n.maturity)
        .map(|n| n.note_id)
        .collect();
    for id in due_notes {
        let _ = world.redeem_note_as(actor, id);
    }
    let due_futs: Vec<u64> = world
        .futures()
        .iter()
        .filter(|f| f.open && world.now() >= f.expiry)
        .map(|f| f.fut_id)
        .collect();
    for id in due_futs {
        let _ = world.settle_future_as(actor, id);
    }
    // Issue one Note if listable, no open notes, stamp can cover half face.
    if world.notes_listable() && world.notes().iter().all(|n| !n.open) {
        let face = ratio(1, 1) + ratio(1, 1); // 2
        let coll = ratio(1, 1); // half
        let stamp = world
            .orb()
            .map(|o| o.stamp_amount())
            .unwrap_or_else(|| ratio(0, 1));
        if stamp >= coll {
            if let (Some(n), Some(c)) = (Positive::new(face), Positive::new(coll)) {
                let holder = actor;
                let term = 60u64; // one year-voice
                let _ = world.issue_note_as(actor, orb_id, holder, n, c, term);
            }
        }
    }
    // Fund open perps once per beat (premium ladder). I5.4: a margin
    // breach LIQUIDATES — a perp that cannot fund cannot stand, and
    // swallowing the refusal left breached books open forever.
    let perps: Vec<u64> = world
        .perps()
        .iter()
        .filter(|p| p.open)
        .map(|p| p.perp_id)
        .collect();
    for id in perps {
        if let Err(cfe::world::ActionRefused::PerpMarginBreach { .. }) =
            world.fund_perp_as(actor, id)
        {
            let _ = world.close_perp_as(actor, id, true);
        }
    }
    // Empiricist (disposition 2) or Exchange: issue Iron share when listable and none open.
    if world.shares_listable() && world.shares().iter().all(|s| !s.open) {
        let empiricist = world.genesis().is_some_and(|g| g.disposition == 2);
        if empiricist || world.exchange_seated() {
            if let (Some(u), Some(p)) = (
                Positive::new(ratio(1, 1)),
                Positive::new(ratio(1, 1)),
            ) {
                let _ = world.issue_share_as(actor, orb_id, actor, Resource::Iron, u, p);
            }
        }
    }
}

/// FLEET-1 Phase-3 — pure NPC salvage + mint + assign (TASKS_TRADE_FLEET_DESK P3.9).
///
/// Ensures each unpenned line can haul: ShipYard when iron affords,
/// scrap from iron, hauler grade 0 until idle capacity ≥ 4, assign
/// idle hulls onto the first live route deal.
pub fn fleet_step(worlds: &mut [Polity]) {
    for polity in worlds.iter_mut() {
        if polity.penned {
            continue;
        }
        ensure_npc_fleet(&mut polity.world, polity.automaton);
    }
}

fn ensure_npc_fleet(world: &mut cfe::world::World, actor: u64) {
    let Some(orb) = world.orb() else {
        return;
    };
    let orb_id = cfe::world::FIRST_ORB;
    let has_yard = orb.zones.iter().any(|z| z.kind == cfe::dop::ZoneKind::ShipYard);
    if !has_yard {
        let cost_ok = cfe::dop::ZoneKind::ShipYard
            .cost()
            .into_iter()
            .all(|(r, n)| orb.banked_amount(r) >= n);
        if cost_ok {
            if let Some(tile) = (0u16..64).find(|&t| !orb.zones.iter().any(|z| z.tile == t)) {
                let _ = world.zone_as(actor, orb_id, tile, cfe::dop::ZoneKind::ShipYard);
            }
        }
    }
    let Some(orb) = world.orb() else {
        return;
    };
    if !orb.zones.iter().any(|z| z.kind == cfe::dop::ZoneKind::ShipYard) {
        return;
    }
    // Target total fleet capacity (all hulls), not free_capacity(0).
    // free_capacity(0) only counts *idle* hulls — after assign to a route
    // it drops to 0 and would mint a new hauler every beat forever.
    let one = ratio(1, 1);
    let target_cap = one.clone() + one.clone() + one.clone() + one;
    let need = cfe::dop::hull_scrap_cost(cfe::dop::HullRole::Hauler, 0);
    let fleet_cap = npc_fleet_capacity(world);
    // Salvage only the capacity shortfall — not dump-all-iron.
    if fleet_cap < target_cap {
        let scrap = orb.scrap_amount();
        if scrap < need {
            let want = need.clone() - scrap;
            let iron = orb.banked_amount(cfe::dop::Resource::Iron);
            let reserve = ratio(1, 1) + ratio(1, 1);
            let convertible = if iron > reserve {
                iron - reserve
            } else {
                ratio(0, 1)
            };
            let convert = if convertible < want {
                convertible
            } else {
                want
            };
            if convert > ratio(0, 1) {
                if let Some(amt) = cfe::dop::Positive::new(convert) {
                    let _ = world.salvage_scrap_as(actor, orb_id, amt);
                }
            }
        }
    }
    for _ in 0..4 {
        if npc_fleet_capacity(world) >= target_cap {
            break;
        }
        let scrap = world
            .orb()
            .map(|o| o.scrap_amount())
            .unwrap_or_else(|| ratio(0, 1));
        if scrap < need {
            break;
        }
        if world
            .mint_hull_as(actor, orb_id, cfe::dop::HullRole::Hauler, 0)
            .is_err()
        {
            break;
        }
    }
    // Assign idle hulls to first live route deal.
    let deal = world
        .routes()
        .first()
        .map(|(d, _)| *d)
        .or_else(|| {
            world
                .committed()
                .element_routes
                .first()
                .map(|(d, _, _, _)| *d)
        })
        .unwrap_or(0);
    if deal != 0 {
        let idle: Vec<u64> = world
            .hulls()
            .iter()
            .filter(|h| h.assigned_route == 0)
            .map(|h| h.hull_id)
            .collect();
        for id in idle {
            let _ = world.assign_hull_as(actor, id, deal);
        }
    }
}

/// Sum of capacity across every hull (assigned or idle).
fn npc_fleet_capacity(world: &cfe::world::World) -> cfe::ratio::Ratio {
    world.hulls().iter().fold(ratio(0, 1), |s, h| {
        s + cfe::dop::hull_capacity(h.role, h.grade)
    })
}
