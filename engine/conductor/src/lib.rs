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
//! **GROWTH PER CIV IS A MINT:** a civilization at the TOP of its own
//! ladder (where `Difficulty`/`VolDifficulty` saturates — rung 8 today) with
//! 20 confirmed solves CLEAVES once per era into TWO PEER BRANCHES —
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
    lineage_domain, office_preconditions, Civilization, OfficeEvidence, Registry, DISPOSITIONS,
    METABOLISMS, PERCEPTIONS,
};
use ledger::{Directive, Disposition, Ledger};
use physics::Beacon;
use solver::{forge_task, Difficulty, Hunger, Need, SolveRead};
use solver::{forge_volume, think_volume_task, VolDifficulty};

use std::path::Path;

/// R4 (demand delivery): a solved program that CHANGES FRAME — a tiling
/// or a non-square rotation — hides same-frame sub-transforms the mesh
/// *can* found. The hypha's `on_sentence` founding is same-frame only
/// (a reshape carries no chain-form residual); but every maximal run of
/// dimension-PRESERVING ops inside the program is a genuine same-frame
/// `(before, after)` pair the hypha founds as a generator. Walk the
/// program forward on each example, emit each same-frame run as its own
/// proved offer, and leave the frame-changing steps (tile/reshape) as a
/// known operator — never founded. No-op when the whole program is
/// already same-frame (the primary `offer_proved` already carries that
/// pair). Frame membership is decided by DIMENSIONS, not op type, so a
/// square transpose/rotate (in-frame, content-permuting) is founded while
/// a non-square rotate (frame-changing) is skipped as structural.
fn same_frame_founding_offers(
    name: &str,
    program: &[solver_verify::Op],
    inputs: &[solver_verify::Grid],
) {
    fn rows(g: &solver_verify::Grid) -> Vec<Vec<u8>> {
        g.cells.chunks(g.cols).map(|r| r.to_vec()).collect()
    }
    // The program is the SAME for every example, so segment k is the
    // same sub-word in each — group by k and offer ONE FAMILY per
    // segment (all examples' pairs), not one thin pair per example.
    // A single observation is exactly what the tongue's canonical
    // readers refuse as under-determined; the family pins it.
    let mut segs: Vec<(String, Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>)> = Vec::new();
    for input in inputs.iter() {
        let mut g = input.clone();
        let mut seg_start = g.clone();
        let mut seg_ops: Vec<solver_verify::Op> = Vec::new();
        let mut k = 0usize;
        let mut saw_change = false;
        let mut push = |k: usize,
                        seg_ops: &[solver_verify::Op],
                        a: &solver_verify::Grid,
                        b: &solver_verify::Grid,
                        segs: &mut Vec<(String, Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>)>| {
            let word = seg_ops
                .iter()
                .map(|o| o.describe())
                .collect::<Vec<_>>()
                .join(" → ");
            if segs.len() <= k {
                segs.push((word, Vec::new()));
            }
            segs[k].1.push((rows(a), rows(b)));
        };
        for op in program {
            let g2 = op.apply(&g);
            if g2.rows == g.rows && g2.cols == g.cols {
                seg_ops.push(op.clone());
                g = g2;
            } else {
                saw_change = true;
                if !seg_ops.is_empty() {
                    push(k, &seg_ops, &seg_start, &g, &mut segs);
                    k += 1;
                }
                g = g2;
                seg_start = g.clone();
                seg_ops.clear();
            }
        }
        if saw_change && !seg_ops.is_empty() {
            push(k, &seg_ops, &seg_start, &g, &mut segs);
        }
    }
    for (k, (word, family)) in segs.into_iter().enumerate() {
        if !family.is_empty() {
            scribe::membrane::offer_proved(&format!("{name}#s{k}"), &word, family);
        }
    }
}

// COG (doc 52): the homological economy — the deficit peeled through the same
// `overlay` the cognition uses; coefficients drive production, residual drives
// research. Replaces the cartesian build cascade with the peel.
pub mod civic;
pub mod lookahead;
pub mod ocular;
mod polities;
// DIO-1 (operator-ruled 2026-08-16): planetary siting on exact rational
// Diophantine tracks (r = M·k², P = M·k³ off the (M,M) base). See siting.rs.
mod siting;
// SOC-1 (operator directive 2026-08-16): the open build workbench — the surplus
// phase of `build_step` draws its next building from a disposition-weighted
// partition function seeded by the chronicle digest, in place of the fixed
// `.or()` priority. Crisis stays canalized; surplus explores. See soc.rs.
pub mod soc;
// RCP-2 (operator directive 2026-08-16): the emergent recipe space — a buildable
// is a hashed transformation from open-almanac elements into the universal roles,
// discovered against a deficit and gated by TOPOLOGICAL reach, never a ZoneKind
// enum variant or an integer level. See recipe.rs.
pub mod recipe;
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
    let stamp = ledger
        .receipts()
        .iter()
        .map(|r| r.stamp())
        .max()
        .unwrap_or(0);
    ledger.append(
        cfe::dop::ClosureEvent::EraFolded {
            era,
            gauge: gauge.clone(),
        },
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
    if t4 > mesic_hi {
        mesic_hi
    } else {
        t4
    }
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

/// **THE FOUNDING BAND** (COG Slice 2 activation, operator ruling
/// 2026-08-15 "seed per habitat"). When the conductor raises a
/// colony's FIRST habitat, a founding band of hunter-gatherers takes
/// up residence — the settlement is founded, and from that moment the
/// population is LIVE: it grows toward the thermodynamic K_pop on the
/// syntropic wild flow, and the beat's `collect` drives the logistic
/// every cycle. Subsequent habitats raise living capacity (K), they do
/// not reseed the band. This is APPROVED SEED — the founding endowment
/// of a settlement, definition not tuning (same class as
/// `founding_reserves`), and it seats through `settle_population`
/// (whole-integer, so it carries no fractional dial). It is small: a
/// band well below any world's syntropic limit, so a fresh settlement
/// GROWS up to its limit rather than being culled down to it.
pub const FOUNDING_BAND: i64 = 4;

/// **THE FOUNDING BAND, BOUNDED BY THE LAND** (G2). How many of the band the
/// chosen site feeds before anything is built: `⌊K_pop⌋` read off the world
/// at genesis (the syntropic wild flow over the per-capita ration), clamped
/// to `[1, FOUNDING_BAND]`. A rate-0 capture line (a plasmoid draws no
/// ration) has no ration-bound ceiling and arrives whole.
pub fn founding_band(world: &World) -> i64 {
    use num_traits::ToPrimitive;
    if world.per_capita_ration() <= ratio(0, 1) {
        return FOUNDING_BAND;
    }
    let k = world.population_ceiling(cfe::world::FIRST_ORB).floor().to_integer();
    k.to_i64().unwrap_or(FOUNDING_BAND).clamp(1, FOUNDING_BAND)
}

/// The archetype CLASS a founding line seals at genesis (DIO-3a). The roster
/// declares each line's CLADE (the species — soniform, cnidiform) but no
/// class; 7 is the neutral baseline `science_lab` already reads for an
/// unsealed world, and it falls through `economic_mode` to let the clade
/// colour the economy. A definition, not a dial (a bare integer, and the
/// class ceiling is 13); a per-clade archetype table is a later refinement.
pub const NEUTRAL_ARCHETYPE_CLASS: u8 = 7;

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
pub fn trade_intent(world: &World, metabolism: &str) -> Option<(Resource, Ratio, Resource, Ratio)> {
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
fn element_want_intent(world: &World) -> Option<(cfe::dop::Cargo, Ratio, cfe::dop::Cargo, Ratio)> {
    let orb = world.orb()?;
    let (_, element) = world
        .element_wants()
        .iter()
        .find(|(o, _)| *o == orb.orb_id)?;
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
fn element_trade_intent(world: &World) -> Option<(cfe::dop::Cargo, Ratio, cfe::dop::Cargo, Ratio)> {
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

/// COGNITION-1/2 measurement (2026-08-14): how often `build_step`
/// actually reaches a genuine two-candidate settlement choice, and how
/// often the lookahead (as opposed to the bootstrap hold or a missing
/// projection) is what decided it. Mirrors `PROBE_DIGEST_REPEAT`'s own
/// pattern — a measured count, never asserted from the code alone.
pub static COGNITION_CHOICE_POINTS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub static COGNITION_LOOKAHEAD_DECIDED: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

/// **PEEL, NOT SWEEP** (operator ruling 2026-07-30: no cartesian find-scan
/// repetition). `clear_trade`'s own FIRST check refuses any pair whose
/// cargos are not exact mirrors (`held_a.give != held_b.want || held_a.want
/// != held_b.give` — `cfe::world::mod.rs`) — that mirror condition IS the
/// market's own partition, not an approximation of one. Bucket every open
/// offer by its `(give, want)` key in ONE pass (O(total offers)), then a
/// civ pair is a candidate ONLY if one holds a key the other holds the
/// mirror of — a pair sharing no complementary key can be PROVEN to never
/// cross, for any combination of their offers, without touching it. The
/// output is a `BTreeSet`, so its iteration order is always ascending
/// `(a, b)` regardless of the (randomized) `HashMap` build order — the
/// same relative visiting order the retired cartesian sweep walked, so the
/// per-beat `seq`/`deal` numbering this beat assigns stays internally
/// deterministic (replay never re-derives a deal id — it reads whatever
/// this beat actually recorded — so this only has to be deterministic
/// with itself, not bit-identical to the sweep it replaces).
pub fn crossing_candidates(worlds: &[Polity]) -> Vec<(usize, usize)> {
    use std::collections::{BTreeSet, HashMap};
    let cargo_bytes = |c: &cfe::dop::Cargo| -> Vec<u8> {
        let mut b = Vec::new();
        c.encode(&mut b);
        b
    };
    let mut by_key: HashMap<(Vec<u8>, Vec<u8>), Vec<usize>> = HashMap::new();
    for (idx, polity) in worlds.iter().enumerate() {
        if polity.penned {
            continue;
        }
        for offer in polity.world.offers() {
            let key = (cargo_bytes(&offer.give), cargo_bytes(&offer.want));
            let bucket = by_key.entry(key).or_default();
            if !bucket.contains(&idx) {
                bucket.push(idx);
            }
        }
    }
    let mut pairs: BTreeSet<(usize, usize)> = BTreeSet::new();
    for ((give, want), holders) in &by_key {
        let mirror = (want.clone(), give.clone());
        let Some(others) = by_key.get(&mirror) else {
            continue;
        };
        for &a in holders {
            for &b in others {
                if a == b {
                    continue;
                }
                pairs.insert(if a < b { (a, b) } else { (b, a) });
            }
        }
    }
    pairs.into_iter().collect()
}

pub fn market_step(worlds: &mut [Polity], beat: u64) -> Vec<(String, String)> {
    // Post intents. NO SINGLES (ruling 2026-07-31): the book is
    // open-ended — a fresh intent posts BESIDE what rests; only an
    // identical pair (same give, same want) holds its place.
    for polity in worlds.iter_mut() {
        if polity.penned {
            continue;
        }
        // E3.7: demand first — element_wants bid resource for open matter.
        if let Some((give, give_amount, want, want_amount)) = element_want_intent(&polity.world) {
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
        if let Some((give, give_amount, want, want_amount)) = element_trade_intent(&polity.world) {
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
    // Clear crossings: EVERY resting combination on every CANDIDATE pair —
    // no first-match single. Deal ids stride 4 (haul takes deal/deal+1, the
    // wear-charter deal+2) from a per-beat sequence. Candidates are peeled
    // by their own (give, want) key (`crossing_candidates`), never swept
    // cartesian over every pair the roster could form.
    let mut cleared = Vec::new();
    let mut seq: u64 = 0;
    for (a, b) in crossing_candidates(worlds) {
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
                    cleared.push((one.spec.world_name.clone(), two.spec.world_name.clone()));
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
                .filter(|(held, _)| two.world.routes().iter().any(|(other, _)| other == held))
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

// SearchMode / SearchPath (the Coupled/Uncoupled/Union regimes and the
// Reach/Sweep split) were RETIRED with the Cartesian sweep (operator ruling
// 2026-08-16, "no enums — that is just more cartesian slop"): with the
// exhaustive path gone there is one path (Reach, the SNF derivation), so the
// mode/path enums were dead discrete-case dispatch. The line searches by
// deriving; a miss asserts a cokernel failure.

impl Trajectory {
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
                    reach_bonus: 0,
                    capacity: 0,
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
/// PERF — the ceiling on a vessel's grown basin. A mind is finite: absorption and
/// practice stop growing it past this, so memory stays bounded over a long run
/// (the basin is live workspace, never pruned by replay). Far above any real
/// working vocabulary, so cognition is unaffected; it only stops a runaway.
const AUTIC_BASIN_CAP: usize = 4096;

/// PERF — how often (in world beats) a vessel PRACTISES a mesh wall and SPEAKS a
/// sentence. Every beat would put two extra reduce ops per civ on top of the
/// existing think_task, dominating a 107-civ global beat. Periodic keeps both
/// axes flowing at a fraction of the cost. Absorption stays every beat (it is
/// already O(new) via the high-water mark).
const AUTIC_STRIDE: u64 = 4;

/// ENTITY-ALIGN-2 (docs/56 §4, piece a) — how often (in world beats) the
/// SIBLING forges its OWN task off its OWN basin instead of confirming the
/// automaton's. Rare (once every 4 beats, like `AUTIC_STRIDE`): confirmation
/// is what actually mints words today (COMM-1), so self-selection must stay
/// a minority of beats or consilience collapses (neither vessel ever closes
/// the SAME orbit the other did). Occasional is the whole point of this
/// slice — the first beat a vessel's OWN choice, not the civ's, decides what
/// it thinks about.
const SIBLING_AUTONOMY_STRIDE: u64 = 4;

/// BEAT-PARALLEL — serialises the one non-isolated per-civ op (`registry.seat`,
/// which reads the whole roster and enforces a global seat-width limit). Rare
/// (once per line), so it does not gate the parallel cognition.
static SEAT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub struct Polity {
    pub spec: PolitySpec,
    pub world: World,
    /// The polity's contained agent (build/raze/collect granted).
    automaton: u64,
    /// The steward's contained entity (drains the operator's inbox) — DIRECTIVES
    /// ONLY. The inbox is not a mind; the steward is no longer a thinker (N-I10).
    steward: u64,
    /// N-I10 — the SIBLING office: the line's second COGNITIVE vessel. It thinks
    /// its own basin toward consilience (N-I8) and hears offered words; distinct
    /// from the steward so culture never requires the player's directive vessel
    /// to think.
    sibling: u64,
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
    /// **THE PHASE-3 FLIP** (`39_PHASE3_FLIP` §5, operator ruling
    /// 2026-08-13): how many receipts have an ARBOR commitment, not
    /// just a judgment. Distinct from `chained` — `chained` advances
    /// on every deed judged (lawful or not, purely for the refusal
    /// report); `arbor_committed` advances only past deeds the kernel
    /// ADMITTED, since a refused deed gets no commitment at all. The
    /// two marks diverge exactly when a refusal happens, which is the
    /// live measure of "replay demoted to proposer."
    arbor_committed: usize,
    /// NOVELTY-GATE (38 §4): this line's minted solver library —
    /// workspace cache of the `SolverWordMinted` fold, seeded lazily
    /// at the first research beat of a process.
    library: solver::mint::Library,
    /// The line's resolved knowledge map, cached against its library's size
    /// (the library only ever grows by `adopt`, so its word count is an exact
    /// fingerprint). `solver::knowledge_map` is exact algebra over every word;
    /// recomputed for every line every beat it was 99% of a G2 beat (154 s at
    /// beat 1005). It is recomputed now only when the library grew.
    knowledge: Option<(usize, solver::SelfMap)>,
    library_folded: bool,
    /// N-I8 — words seeking CONSILIENCE: (orbit, expansion, deposits). Each
    /// deposit is `(vessel_id, digest)`; the word mints only when two DISTINCT
    /// vessels have independently closed the orbit. One vessel confirming many
    /// digests is Horizon, not an install (C−) — pending is a wait.
    word_pending: Vec<([u8; 32], Vec<solver_verify::grid::Op>, Vec<(u64, [u8; 32])>)>,
    /// Planes this line has OPENED — the unfold, one per earned mint.
    /// The fiber's grade count is `world.key().len()`; this counts the
    /// openings that got it there.
    pub unfolded: u64,
    /// WHERE THIS LINE IS GOING, read from its own outcomes and used
    /// to decide what the next beat's search may spend.
    pub trajectory: Trajectory,
    /// COG-DIRECTION Slice 3 — the line's ORDER state at the last beat it read
    /// itself: `(syntropy_contained, entropy_vented, families)`. The books are
    /// cumulative, so the per-beat DELTA is the order-capture FLOW — whether the line
    /// captured or shed order THIS beat — and the family delta is whether its material
    /// self is gaining or collapsing structure. Direction is derived from those FLOWS
    /// (evolution — the line vs its own past), not a lifetime-level threshold. `None`
    /// until the first material read.
    prev_order: Option<(Ratio, Ratio, usize)>,
    /// COG-DIRECTION Slice 3b — the POPULATION's cohesion band this beat:
    /// `(lower_third, upper_third)`, the tertile cuts of the live cohesion
    /// distribution across every warm-booked line. Set once per beat by
    /// [`direction_census_step`] before the cycle loop; read by `research_step` so
    /// a line's direction is its fitness RELATIVE TO THE POPULATION (true natural
    /// selection: the top third grows, the bottom third declines) rather than an
    /// absolute 1/2 dial. `None` before the first census or when fewer than three
    /// warm lines stand (no population to be relative to → hold). MEASURED
    /// (2026-08-21, 90-civ live journal): the cohesion LEVEL spreads 0..0.93,
    /// median 0.070, 72/90 distinct — a clean partition; the per-beat FLOW is ~75%
    /// flat at EVERY window 1..13, so selection reads the discriminating LEVEL,
    /// population-relative, never the flat flow (Slice 3a refuted by measurement).
    pop_cohesion_band: Option<(Ratio, Ratio)>,
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
    /// N-I·VESSEL — THE COGNITIVE BASIN, PER VESSEL (THE INTELLIGENCE PORT).
    /// Each VESSEL (person) grows its OWN basin of crystallized residuals: a
    /// wall `think_task` mints and adopts joins THAT vessel's mouth, and the
    /// same vessel's next think DRAINS against it (Pin B) — thinking changes the
    /// thinker, one thinker at a time. A sibling on the same puzzle walls afresh;
    /// it never inherits the wall (isolation IS the map key).
    ///
    /// **Law II — the map is an INDEX, not the thought.** `Vec<Crystal>` is the
    /// basin the organ reduces; `BTreeMap<vessel_id, _>` only ADDRESSES which
    /// basin — ordered by `vessel_id` (`Ord`), so iteration is a replay-stable
    /// walk. NEVER `HashMap` (hash addressing throws away the fiber); `Q/ℤ` lives
    /// INSIDE the crystals, never in the key. Non-committed workspace like
    /// `library` — the committed wire (`Entity`) stays empty, so replay of a
    /// pre-grown receipt still matches. Seed vessel is the line's `automaton`;
    /// the map admits many (population individuation is a later cut).
    grown: std::collections::BTreeMap<u64, Vec<solver::Crystal>>,
    /// AUTIC-1 — how many AUTIC crystals this line's vessels have absorbed from
    /// the mesh-learned basin (cumulative, workspace like `grown`). The live
    /// measure of the shared tongue entering entity cognition.
    pub autic_absorbed: usize,
    /// AUTIC-2 — sentences one vessel has SPOKEN to the other (composed
    /// utterances), and how many carried a DemandUnmet leftover the hearer LEARNED
    /// (adopted as a new crystal). Cumulative workspace. The live measure of the
    /// COMPOSE move: multi-word thought exchanged and comprehended between minds.
    pub autic_spoke: usize,
    pub autic_learned: usize,
    /// AUTIC-1 curriculum axis — how many mesh walls this line's vessels have
    /// PRACTISED (heard the mesh's own problem and re-derived it, growing the
    /// basin where it walled). Cumulative workspace. The live measure of the
    /// entities facing the network's real problems, not only forged ones.
    pub autic_practiced: usize,
    /// PERF — per-vessel high-water mark into the shared AUTIC basin: how many
    /// learned crystals this vessel has already been offered. Absorption examines
    /// only crystals PAST this mark, so it is O(new) per think, never O(basin).
    autic_seen: std::collections::BTreeMap<u64, usize>,
    /// COG-AUTHOR (A3) — variant words this line has AUTHORED: cited a word it
    /// holds, rebound it under a seeded symmetry, and offered the result to the
    /// mesh door. The live measure of the rung from hearing to authoring — a
    /// reader becoming a speaker.
    pub autic_authored: usize,
    /// How many refusals the board has already been TOLD. The record
    /// above is permanent; this is only the narrator's place in it, so
    /// a refusal is announced once per process and kept forever.
    /// Workspace state, like `chained`: a RESUMED world restarts both
    /// at 0 and re-gates its whole chronicle on the next beat, so a
    /// pre-existing refusal is announced again in the new process. That
    /// is honest — the gate met the deed again — and it is the same
    /// re-verification `chained: 0` already performs.
    reported: usize,
    /// #157 — this process's proof that the resume carrier
    /// (`chronicle.tlv`) matches this world up to some point: lets
    /// `persist_sync` skip re-reading and re-verifying the whole file
    /// every beat, the dominant share of the per-beat floor on a live
    /// era (measured ~3s/civ by beat 11, growing every beat since).
    /// Workspace like `chained`/`stepper`: a RESUMED world starts cold
    /// (`Default`) and pays one full check on its first beat, exactly
    /// like today.
    sync_cursor: cfe::persist::SyncCursor,
}

/// The smelter's (iron, biomass) gate — bound ONCE, read by the
/// industrial arm, the forge hold, and the gather alike, so the
/// ratchet counts two dials and no reader can drift from another.
fn smelter_gates() -> (Ratio, Ratio) {
    (ratio(30, 1), ratio(10, 1))
}

// ── #146 THE ENTITY-ATLAS (doc 55 §9) — value-preserving live cognition ──
//
// A civilization's collective cognition = the manifold ΔH over its VESSELS
// (minds). A crystal's key is its FULL CANONICAL CLASS (`ChainDelta` — frame +
// exact entries), NEVER a u64 digest: the digest would collapse the frame-morphism
// and the Q/ℤ coefficients — the very value (the per-vessel analog of a line's
// `rcf_hash`). Two vessels SHARE a class iff their canonical classes match.

/// The value-preserving key of a crystal — its full canonical class, lossless.
fn atlas_crystal_key(c: &solver::Crystal) -> String {
    format!("{:?}", c.class)
}
/// A splitmix seed from bytes — for the CONNECTION SAMPLE only, never the value key.
fn atlas_seed(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}
fn atlas_draw(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    (z ^ (z >> 31)) % 1000
}
/// A contractible house-chart (one mind).
fn atlas_point() -> reduct::sheaf::CellSheaf {
    reduct::sheaf::CellSheaf::new(vec![vec![1]], vec![]).expect("point chart")
}
/// `n` shared-class contacts as an overlap.
fn atlas_contacts(n: usize) -> reduct::sheaf::CellSheaf {
    reduct::sheaf::CellSheaf::new(vec![vec![1; n.max(1)]], vec![]).expect("contact overlap")
}
fn atlas_restr(n: usize) -> Vec<Vec<Vec<num_bigint::BigInt>>> {
    vec![(0..n.max(1))
        .map(|_| vec![num_bigint::BigInt::from(1)])
        .collect()]
}
/// A cohesion ratio in `[0,1]` as a per-mille connection coherence, clamped.
fn atlas_permille(c: &Ratio) -> u64 {
    let scaled = (c.numer() * num_bigint::BigInt::from(1000)) / c.denom();
    scaled
        .to_string()
        .parse::<i64>()
        .unwrap_or(0)
        .clamp(0, 1000) as u64
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
        match receipt.event() {
            // ONT-2b: eating-mining is still mining — a staple pull counts
            // toward the ground evidence beside the geometric assay.
            ClosureEvent::AssayStruck { .. } | ClosureEvent::StapleMined { .. } => {
                evidence.assays_struck += 1
            }
            ClosureEvent::RouteChartered { .. } => evidence.routes_chartered += 1,
            ClosureEvent::OfferCleared { .. } => evidence.market_clears += 1,
            _ => {}
        }
    }
    evidence.has_crossed = world.has_crossed();
    evidence
}

/// N-I8 — CONSILIENCE deposit (pure). Record `vessel`'s closure of `orbit` in the
/// pending board (idempotent per vessel+digest), and return the cited digests IFF
/// two DISTINCT vessels have now closed it. Does NOT remove the entry — the caller
/// removes it only on a SUCCESSFUL (affordable) mint, so a consilient-but-unpaid
/// word stays pending (fasting). One vessel confirming many digests never reaches
/// two distinct vessel_ids (C−).
///
/// **BOUNDED BY VESSEL, NOT BY ATTEMPT** (fixed 2026-08-24 — a real, measured
/// leak): a vessel that keeps re-depositing the same unconfirmed orbit across
/// beats carries a NEW digest every time (`research_step`'s digest is the
/// beat's own receipt hash), so `(vessel, digest)` never repeats even though
/// it is the same vessel re-confirming the same finding. Appending one entry
/// per attempt made this grow WITHOUT BOUND for any orbit only one vessel
/// ever visits — exactly the shape `pin_c_two_vessels_mint...`'s own fixture
/// exercises (one vessel deposits three distinct digests before giving up).
/// Only vessel DIVERSITY ever gates the mint, never the attempt history, so
/// each vessel gets ONE slot, updated in place — bounded by the vessel
/// count (two, in practice), never by beats elapsed.
///
/// **IDENTITY AND CONTENT ARE SEPARATE AXES.** `mint_solver_word_as` gates on
/// `cited.len()` alone, so this list's LENGTH must track vessel-diversity — the
/// identity axis two distinct minds prove by both showing up — never digest
/// agreement, the content axis. A digest is deterministic on WHAT was closed
/// (`research_step` stamps every deposit this beat with the SAME chronicle
/// digest), so two distinct vessels closing the same orbit in the same beat
/// legitimately carry the identical digest — real consilience that a naive
/// digest-dedup would collapse to one entry and silently starve forever
/// (`SolverWordUnconfirmed` on every attempt). One digest PER DISTINCT VESSEL,
/// never per distinct digest VALUE — the digest itself rides along only as
/// per-vessel provenance for the receipt, exactly as `entity_id` rides beside
/// (never inside) a content hash in the mesh's own orbit-congruence law.
pub fn consilience_deposit(
    pending: &mut Vec<([u8; 32], Vec<solver_verify::grid::Op>, Vec<(u64, [u8; 32])>)>,
    orbit: [u8; 32],
    expansion: &[solver_verify::grid::Op],
    vessel: u64,
    digest: [u8; 32],
) -> Option<Vec<[u8; 32]>> {
    let entry = match pending.iter_mut().find(|(held, _, _)| *held == orbit) {
        Some(e) => e,
        None => {
            pending.push((orbit, expansion.to_vec(), Vec::new()));
            pending.last_mut().expect("just pushed")
        }
    };
    match entry.2.iter_mut().find(|(v, _)| *v == vessel) {
        Some((_, held)) => *held = digest,
        None => entry.2.push((vessel, digest)),
    }
    let mut vessels: Vec<u64> = entry.2.iter().map(|(v, _)| *v).collect();
    vessels.sort_unstable();
    vessels.dedup();
    if vessels.len() < 2 {
        return None; // Horizon — one vantage waiting, not a mint (C−)
    }
    // One digest per DISTINCT VESSEL (first deposit wins) — `cited.len()` must
    // equal `vessels.len()`, the identity count, never collapse on shared content.
    let mut cited: Vec<[u8; 32]> = Vec::with_capacity(vessels.len());
    for &v in &vessels {
        if let Some((_, d)) = entry.2.iter().find(|(ev, _)| *ev == v) {
            cited.push(*d);
        }
    }
    Some(cited)
}

/// ENTITY-ALIGN-1 (docs/56 §3, pure) — mirrors `solver::consilience`'s
/// ALREADY-DEPLOYED hardness measure (`orders.dedup().len()`,
/// `selfmap.rs:225-231`) at vessel scope instead of line scope: the count
/// of DISTINCT knowledge-readings among `vessels`, each read from `grown`
/// and reduced to a SET of classes (order and duplicate count ignored — a
/// basin is a proven set, not a sequence; `ChainDelta` derives
/// `PartialEq, Eq` but not `Hash`, so this is a linear `contains` scan,
/// bounded by `AUTIC_BASIN_CAP` per vessel). Identical knowledge from
/// every vessel bridges nothing (hardness 1); the more genuinely
/// different the vessels' own bases, the harder-won the agreement.
pub fn confirmation_hardness(
    grown: &std::collections::BTreeMap<u64, Vec<solver::Crystal>>,
    vessels: &[u64],
) -> u32 {
    fn basin_classes_of(
        grown: &std::collections::BTreeMap<u64, Vec<solver::Crystal>>,
        vessel: u64,
    ) -> Vec<reduct::ChainDelta> {
        let mut classes: Vec<reduct::ChainDelta> = Vec::new();
        if let Some(basin) = grown.get(&vessel) {
            for crystal in basin {
                if !classes.contains(&crystal.class) {
                    classes.push(crystal.class.clone());
                }
            }
        }
        classes
    }
    fn same_basin_knowledge(a: &[reduct::ChainDelta], b: &[reduct::ChainDelta]) -> bool {
        a.len() == b.len() && a.iter().all(|c| b.contains(c))
    }
    let readings: Vec<Vec<reduct::ChainDelta>> =
        vessels.iter().map(|&v| basin_classes_of(grown, v)).collect();
    let mut distinct: Vec<&Vec<reduct::ChainDelta>> = Vec::new();
    for reading in &readings {
        if !distinct.iter().any(|held| same_basin_knowledge(held, reading)) {
            distinct.push(reading);
        }
    }
    distinct.len() as u32
}

/// ENTITY-ALIGN-2 (docs/56 §4, piece a, pure) — the digest a vessel chooses
/// its own problem from: a hash of the LATEST crystal in its own basin, never
/// the civ's chronicle digest. `None` for an empty or absent basin — nothing
/// grown yet means nothing to derive a problem from, so autonomy is not yet
/// earned.
pub fn basin_digest(basin: Option<&Vec<solver::Crystal>>) -> Option<[u8; 32]> {
    let latest = basin?.last()?;
    let mut hasher = blake3::Hasher::new();
    hasher.update(atlas_crystal_key(latest).as_bytes());
    Some(*hasher.finalize().as_bytes())
}

/// ENTITY-ALIGN-2 (docs/56 §4, piece a, pure) — whether THIS beat is one
/// where the sibling self-selects rather than confirms: hash-seeded per
/// (vessel, beat) so it is emergent (a different vessel's self-select beats
/// land on different world beats) rather than a synchronized global tick like
/// `AUTIC_STRIDE`'s `is_multiple_of`. Fires on roughly `1/stride` of beats.
pub fn sibling_self_select_beat(sibling: u64, beat: u64, stride: u64) -> bool {
    let seed = sibling
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(beat)
        .wrapping_mul(0xD1B5_4A32_D192_ED03);
    seed % stride == 0
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
        cfe::thermal::t_eq4(
            &class.luminosity(),
            site.albedo.value(),
            site.orbit_r.value(),
        ),
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
                let sibling0 = by_handle("sibling");
                if let (Some(automaton), Some(steward)) = (by_handle(&handle), by_handle("steward"))
                {
                    // MESH-1b: the walk decides whose pen this is.
                    // Lawful conductor hand → held and stamped from
                    // here on; a pen out across a restart STAYS out;
                    // pre-mesh None stays unheld (law starts at the
                    // first seal); a violating arbor waits penned —
                    // writes never land on a refused chain.
                    let mut world = world;
                    // N-I10 — the SIBLING is the second cognitive vessel; a world
                    // founded before N-I10 lacks it, so register it on resume.
                    let sibling = sibling0
                        .unwrap_or_else(|| world.register_agent("sibling").expect("registers"));
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
                    // The ladder resumes from the solve cards
                    // themselves: the highest rung any solve in this
                    // world was EARNED at (sol::DEPTH). The .json
                    // count that stood here died with the format
                    // migration to .sol — it read 0 forever and
                    // silently reset every resumed world to rung 1,
                    // which is why eras re-proved D4 singles for
                    // hours after each restart.
                    let level = scribe::resume_level(book, &spec.world_name)
                        .unwrap_or(1)
                        .max(1);
                    // The tally keeps its own honest count (.sol, the
                    // format actually written) for streak/solved.
                    let solved = std::fs::read_dir(book.world_dir(&spec.world_name).join("solves"))
                        .map(|entries| {
                            entries
                                .filter_map(|e| e.ok())
                                .filter(|e| e.path().extension().is_some_and(|x| x == "sol"))
                                .count() as u32
                        })
                        .unwrap_or(0);
                    // One divergence per era: a resumed continuation
                    // is the same era — the receipt remembers.
                    let cleaved = world.chronicle().receipts().iter().any(|receipt| {
                        matches!(receipt.event(), cfe::dop::ClosureEvent::Diverged { .. })
                    });
                    return Polity {
                        chained: 0,
                        arbor_committed: 0,
                        stepper: None,
                        library: solver::mint::Library::new(),
                        knowledge: None,
                        library_folded: false,
                        word_pending: Vec::new(),
                        refused: Vec::new(),
                        unfolded: 0,
                        lane_refused: Vec::new(),
                        grown: std::collections::BTreeMap::new(),
                        autic_absorbed: 0,
                        autic_spoke: 0,
                        autic_learned: 0,
                        autic_practiced: 0,
                        autic_seen: std::collections::BTreeMap::new(),
                        autic_authored: 0,
                        reported: 0,
                        sync_cursor: cfe::persist::SyncCursor::default(),
                        trajectory: Trajectory::Growth,
                        prev_order: None,
                        pop_cohesion_band: None,
                        starved: 0,
                        spent: 0,
                        swept: 0,
                        spec,
                        world,
                        automaton,
                        steward,
                        sibling,
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
        // GALACTIC-1: seal the regional resource-weighting seed from the
        // charted position — derived here (the caller holds the ledger's
        // position), recorded into the chronicle as the DERIVED seed only,
        // so replay never needs the starchart back.
        let region_seed =
            cfe::dop::region_seed_of(&spec.position.0, &spec.position.1, &spec.position.2);
        let _ = world.chart_region(region_seed);
        if opening != ratio(0, 1) {
            world.shift_gauge(opening.clone());
        }
        let automaton = world
            .register_agent(&format!("{}-conductor", spec.civilization.sigil))
            .expect("registers");
        let steward = world.register_agent("steward").expect("registers");
        // N-I10 — the SIBLING: the second cognitive vessel (distinct from the
        // steward, which stays on directives). Thinks toward consilience + hears.
        let sibling = world.register_agent("sibling").expect("registers");
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
            .own()
            .0;
        let beacon = Beacon::from_digest(digest);
        // THE BAND CHOOSES WHERE IT SETTLES (G2, operator 2026-10-07: "only
        // give them options, freedom, and agency"). A pinned seat is the
        // line's compiled HOME — one option, not an order — beside every
        // continuous draw of its regime; the physics reads each option's
        // viability and the band draws among them, viability-weighted and
        // chronicle-seeded (`siting::choose_site`). Better land is likelier,
        // never guaranteed. A home the physics refuses is REPORTED, never
        // silently left unsited.
        let home = match (spec.star, spec.albedo.clone(), spec.orbit.clone()) {
            (Some(class), Some(albedo), Some(orbit)) => Some((class, albedo, orbit)),
            _ => None,
        };
        let (options, refused) = siting::founding_options(&spec.civilization, &beacon, home);
        if let Some(why) = refused {
            eprintln!(
                "genesis {}: the compiled home ({why}) is refused by the physics; the band chooses among {} derived sites",
                spec.civilization.sigil,
                options.len()
            );
        }
        let pick = siting::choose_site(&options, digest).unwrap_or(0);
        let chosen = options.get(pick).cloned().unwrap_or_else(|| {
            let (class, albedo, orbit) = siting::diophantine_siting(&spec.civilization, &beacon);
            siting::SiteOption { class, albedo, orbit, viability: ratio(0, 1), home: false }
        });
        eprintln!(
            "genesis {}: the band settles {} — {:?} orbit {} albedo {} (viability {}, one of {} options)",
            spec.civilization.sigil,
            if chosen.home { "its compiled home" } else { "a derived site" },
            chosen.class,
            chosen.orbit,
            chosen.albedo,
            chosen.viability,
            options.len()
        );
        let (class, albedo, orbit) = (chosen.class, chosen.albedo, chosen.orbit);
        world
            .commission_star(class)
            .expect("a fresh world has no sun yet");
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
        match (cfe::dop::Unit::new(albedo.clone()), cfe::dop::Positive::new(orbit.clone())) {
            (Some(albedo), Some(orbit)) => {
                if let Err(refusal) = world.site_planet(cfe::world::FIRST_ORB, albedo, orbit, None, None) {
                    eprintln!("genesis {}: the planet was NOT sited — {refusal:?}", spec.civilization.sigil);
                }
            }
            _ => eprintln!(
                "genesis {}: the planet was NOT sited — albedo {albedo} / orbit {orbit} fail the physics' doors",
                spec.civilization.sigil
            ),
        }
        // **DIO-3a: SEAL THE CLADE AT GENESIS** (operator ruling 2026-08-16). Until
        // sealed, every founding line reads the flatlander default (clade 2,
        // Mesic), so DIO-2's species-relative η is LATENT — a whale, a coral,
        // and a plasmoid all read the same temperate optimum. Translate the
        // spec's DECLARED clade string to its code and seal it, so
        // `rejuvenation_clade` returns the real species and `habitability_flux`
        // reads its OWN band. Class 7 is the neutral baseline `science_lab`
        // already assumes when unsealed (so `economic_mode` falls through to the
        // clade — the identity, not the archetype, colours the economy); a
        // per-clade archetype class is a later refinement. Sealed AFTER siting so
        // the orbit draw (which reads the genesis digest) is unchanged: DIO-3a
        // isolates to "the clade goes live". A clade the table does not know
        // stays flatlander rather than refusing the founding.
        if let Some(clade) = cfe::almanac::table_clade_from_word(&spec.civilization.clade) {
            let _ = world.seal_taxon(NEUTRAL_ARCHETYPE_CLASS, clade);
        }
        // THE FOUNDING BAND ARRIVES WITH THE COLONY, BOUNDED BY THE LAND (G2).
        // The band founds the settlement; it is not a reward for raising a
        // Habitat (that coupling let a first non-habitat build's famine lock
        // the band out forever — every G2 cleave was born empty). Its size is
        // the physics' reading of what this land feeds wild, before anything
        // is built (`population_ceiling`, the syntropic flow over the ration),
        // never more than FOUNDING_BAND and never less than one: a band does
        // arrive, and what follows is the physics'.
        let band = founding_band(&world);
        world.settle_population(cfe::world::FIRST_ORB, band);
        eprintln!("genesis {}: a founding band of {band} settles", spec.civilization.sigil);
        let world_name_for_level = spec.world_name.clone();
        Polity {
            chained: 0,
            arbor_committed: 0,
            stepper: None,
            library: solver::mint::Library::new(),
            knowledge: None,
            library_folded: false,
            word_pending: Vec::new(),
            refused: Vec::new(),
            unfolded: 0,
            lane_refused: Vec::new(),
            grown: std::collections::BTreeMap::new(),
            autic_absorbed: 0,
            autic_spoke: 0,
            autic_learned: 0,
            autic_practiced: 0,
            autic_seen: std::collections::BTreeMap::new(),
            autic_authored: 0,
            reported: 0,
            sync_cursor: cfe::persist::SyncCursor::default(),
            trajectory: Trajectory::Growth,
            prev_order: None,
            pop_cohesion_band: None,
            starved: 0,
            spent: 0,
            swept: 0,
            spec,
            world,
            automaton,
            steward,
            sibling,
            // The climb resumes from the kernel's own paper — a
            // restart is a reboot, not an amnesia (scribe::resume_level;
            // the ladder saturates on its own, no cap needed here).
            level: scribe::resume_level(book, &world_name_for_level)
                .unwrap_or(1)
                .max(1),
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

        // ONT-2b: the STAPLE is an ORB invariant — the law partitions the vein
        // book by orb, so resolve each orb's staple ONCE (a peel by the orb
        // plane), never per vein. Recomputing `staple_diet_of` (an O(prospects)
        // scan + morphogen projection) inside the assay walk would be a
        // cartesian re-scan of the book per vein — the sweep the standing
        // ruling forbids. `orbs` is the polity's few orbs, so this is one
        // projection per orb and the assay walk below stays linear.
        let staples: Vec<(u64, Option<[u8; 32]>)> = orbs
            .iter()
            .map(|&orb| (orb, self.world.staple_diet_of(orb)))
            .collect();

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
            // The STAPLE vein is the population's larder — it is mined by EATING
            // at the collect touch (demand-driven `StapleMined`), not pumped here
            // on the geometric span. Pre-mining it would strip the vein's
            // richness before `K_pop` reads it (the deadlock the reverted first
            // attempt hit). Every NON-staple vein (industry, trade, trace) still
            // assays on its geometric span. The staple is read from the per-orb
            // peel above, an O(1) lookup in the polity's small orb set.
            let staple = staples
                .iter()
                .find(|(o, _)| *o == orb)
                .and_then(|(_, s)| *s);
            if staple == Some(element) {
                continue;
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
        // **THE OFFICE DERIVES FROM THE OPPORTUNITY** (INTELLIGENCE ARC B,
        // business-development made cognition-driven). A line no longer
        // takes the office a fixed `disposition → office` dial assigns it;
        // it seats the vocation its OWN economy is strongest in — a router
        // turns caravan, a market-maker turns exchange. The FIT is the
        // COMMENSURATE activity each vocation is measured by: routes
        // chartered vs. market clears, both event counts on the same
        // scale (element_kinds is inventory BREADTH, a precondition to
        // seat, never a fit — comparing it against a trade count seats
        // every element-holder crucible by an accident of units).
        let want = if evidence.routes_chartered != evidence.market_clears {
            // the stronger demonstrated activity wins outright.
            if evidence.routes_chartered > evidence.market_clears {
                "caravan"
            } else {
                "exchange"
            }
        } else {
            // no distinguishing activity yet — fall to the disposition's
            // own structural LEAN (a direction from temperament,
            // 44_LINEAGES, never a tuned magnitude).
            match self.spec.civilization.disposition.as_str() {
                "empiricist" => "exchange",
                _ => "caravan",
            }
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
        // BEAT-PARALLEL — `registry.seat` reads the WHOLE roster (`self.list()`)
        // and enforces a global seat-width constraint, so it is the one op in the
        // per-civ cycle that is not per-world isolated. It fires at most once per
        // line's lifetime (guarded above by `held.is_some()`), so serialising it
        // under a global lock keeps the width invariant exact without touching the
        // parallel cognition. Everything else in `cycle` is per-world (`book` is
        // filesystem-keyed by world) or Mutex-guarded (membrane/worth/profile).
        let seated = {
            let _guard = SEAT_LOCK.lock().unwrap_or_else(|p| p.into_inner());
            registry.seat(&self.spec.civilization.sigil, want)
        };
        if seated.is_ok() {
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
        let dis = self.world.genesis().map(|g| g.disposition).unwrap_or(1);
        // disposition wire: 1 expansionist · 2 empiricist · 3 epicurean
        // (and others default expansionist-ish yield path).
        let chem = cfe::almanac::ScienceDiscipline::Chem;

        // THE BASE CHEM SEAT IS SURVIVAL INFRASTRUCTURE, not a discretionary
        // luxury. Measurement (EMERGENCE-2b) showed the base lab is load-bearing:
        // its disposition-ordered survival compounds feed food-margin lines, so
        // the roster seats it universally. Folding the BASE seat into the surplus
        // tier starved the very lines with no surplus to invest — so it stays
        // here, in the survival phase. The DISCRETIONARY chemistry — advancing the
        // lab deeper — is the abundance resolver's Synthesize act. (The full
        // gate-removal waits for `express_survival` to name a chem deed itself.)
        if self.world.science_rank(primary, chem) < 1 {
            if self.world.seat_science_as(automaton, primary, chem).is_ok() {
                return; // one act per beat
            }
            return; // cannot seat yet — no chemistry below the lab door
        }

        // Disposition-ordered chemistry acts (production, once seated).
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
            if self.world.compound_as(actor, orb, &members).is_ok() {
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
            let can_pay = ZoneKind::Habitat
                .cost()
                .into_iter()
                .all(|(resource, need)| banked(&self.world, resource) >= need);
            if can_pay {
                if let Some(tile) = next_free_tile(&self.world) {
                    // The founding band arrived with the colony at genesis
                    // (`founding_band`); a first Habitat raises living
                    // capacity, it does not seed people.
                    let _ = self.world.zone_as(self.automaton, primary, tile, ZoneKind::Habitat);
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
                    if self.world.iso_union_as(actor, orb, ids[i], ids[j]).is_ok() {
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
        let orbs: Vec<u64> = self
            .world
            .committed()
            .orbs
            .iter()
            .map(|o| o.orb_id)
            .collect();
        if orbs.len() == 1 {
            // The expansion: the survey confirms the home sigil and
            // the charter plants its next sector. DERIVED (P2): the
            // charter MOVES the founding reserves, so the gate is
            // double them — a line charters only from a position that
            // keeps a second founding in the bank. The dead 40/20
            // restated exactly this.
            let launches = cfe::dop::founding_reserves()
                .into_iter()
                .all(|(resource, moved)| banked(&self.world, resource) >= moved * ratio(2, 1));
            if launches {
                let site_name = format!("{}-survey", self.spec.colony);
                if let Ok(site) = self.world.chart_as(self.automaton, &site_name, primary) {
                    let _ = self
                        .world
                        .zone_as(self.automaton, site, 0, ZoneKind::Extractor);
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
        let _ = self
            .world
            .zone_as(self.automaton, target, tile, ZoneKind::Extractor);
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
            .find_map(|receipt| match receipt.event() {
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
            // The ledger moves on its grain: the ask is smelted to it.
            let ask = cfe::world::smelt_resource(&surplus.min(need.clone()));
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
    /// **COG-8 — BUILD-TO-THINK.** A line invests in a bigger BRAIN when it has
    /// thinkers to deepen and can afford the apparatus. Seating a cognitive
    /// science (Math/Physics/Research) lifts `cognitive_capacity` 2→3, so the Mind
    /// composes one rung deeper — COG-6's depth-3 frontier that the conductor,
    /// seating only Chem, never unlocked. A COSTED investment: `seat_science_as`
    /// pays the science-seat bill from the orb's banks, so a poor or starving line
    /// cannot buy depth — intelligence is EARNED, not gifted (true-wealth). The
    /// discipline is drawn STOCHASTICALLY — a partition draw over the unseated
    /// cognitive sciences, weighted by the species' own seat affinity (a
    /// comb-compute hexasocial reaches for Math, a plasmiform Physics), seeded by
    /// the line's latest receipt digest so replay is exact (the SOC
    /// stochastic-freedom principle). One seat per beat.
    /// **THE ABUNDANCE TENSIONS** (COG-EMERGENCE-2) — the three γ the surplus
    /// resolver weighs, each a saturation on the melt lattice `[0, L]`, read from
    /// live committed state. The material VOID (`Σ|γ|`, the peel's unspanned
    /// residual) admits TWO cures: DEEPEN the mind to invent past it (while the
    /// mind has depth headroom — `civic::cognitive_deficit`, zero once a
    /// hard-cognitive science is seated), or seat CHEMISTRY to synthesise the
    /// missing structure (while the lab is unseated). Both read the SAME void, so
    /// the draw picks the cure. EXPANSION reads a different axis — the spatial
    /// room left in the star system. No affordability enters: these are tensions,
    /// not budgets.
    fn abundance_tensions(&self) -> (u64, u64, u64, u64, u64) {
        use cfe::almanac::ScienceDiscipline::{Chem, Math, Physics, Research};
        const FRONTIER: usize = 3; // compose-depth ceiling; cognitive_capacity caps here
        const SYSTEM_CAP: u64 = 4; // a star system holds a handful of worlds (expand_step)
        let l = civic::LATTICE as u64;
        let primary = cfe::world::FIRST_ORB;
        let ext = self
            .civic_peel()
            .map(|p| civic::residual_extent(&p))
            .unwrap_or(0);
        // DEEPEN: the void weighed against the mind's remaining depth. The base
        // γ saturates at the FRONTIER (by design — the no-overfit ceiling), but
        // that saturation triggers the instant ONE hard science seats, long
        // before the Deepen ACT itself runs out of room (`invest_cognition` can
        // still seat/deepen the other two, or elevate any of the three past rank
        // 1). THE SECOND, CAPACITY-SCALED CONSUMER `cognitive_capacity`'s comment
        // calls for: while any hard-cognitive science stands seated but below
        // its own `max_rank()`, the same material void keeps tilting Deepen —
        // the exact shape `synthesis` already uses to deepen Chem past its base
        // seat, applied by symmetry to Math/Physics/Research.
        let capacity = self.world.cognitive_capacity(primary);
        let base_deficit = civic::cognitive_deficit(ext, capacity, FRONTIER);
        let deepenable = [Math, Physics, Research].into_iter().any(|d| {
            let r = self.world.science_rank(primary, d);
            r >= 1 && r < d.max_rank()
        });
        let beyond_frontier = if deepenable { ext.min(l) } else { 0 };
        let cognitive = (base_deficit + beyond_frontier).min(l);
        // SYNTHESIZE: the DISCRETIONARY deepening of chemistry — advancing the lab
        // BEYOND the survival base seat (chemistry_step seats rank 1 as survival
        // infra). Once the lab stands, the SAME material void can be cured further
        // by deepening it — a luxury, so it competes in the abundance fold. Zero
        // before the base seat (survival's job) and once chemistry is maxed.
        let chem_rank = self.world.science_rank(primary, Chem);
        let synthesis = if chem_rank >= 1 && chem_rank < Chem.max_rank() {
            ext.min(l)
        } else {
            0
        };
        // EXPAND: the spatial room left in the system, as a lattice fraction.
        let orbs = self.world.committed().orbs.len() as u64;
        let expansion = SYSTEM_CAP.saturating_sub(orbs).saturating_mul(l) / SYSTEM_CAP;
        // CULTURE — the practices this line's own chronicle has EARNED into the
        // fold (seed ∪ minted, `28_VOCABULARY`) but not yet taken up. Adoption
        // itself stays physics-gated (a tempered discipline still needs its
        // hardship receipts, `adopt_practice_as`) — this tension is only the
        // WILL: how much unclaimed discipline stands in the fold, capped like
        // every other γ.
        let culture = self.earnable_practices().len() as u64;
        let culture = culture.min(l);
        // CRAFT (RCP-2 F4) — the doing-layer's own unclaimed headroom: how
        // many of the five universal roles this line's held almanac could
        // fill (`recipe::discover`) but hasn't yet raised a recipe for.
        // Reads the SAME "unclaimed but earnable" shape Culture already
        // uses, applied to the material doing-layer instead of vocabulary —
        // a role no held element can fill never counts (the discovery
        // search itself stays honest).
        let craft = self.craftable_roles().len() as u64;
        let craft = craft.min(l);
        (cognitive, expansion, synthesis, culture, craft)
    }

    /// The fold's practices this line has not yet adopted — the raw material
    /// CULTURE's tension reads and `invest_culture` draws from. Order is the
    /// fold's own (append order, `28_VOCABULARY`), never re-sorted, so the
    /// seed-draw below is the only source of which one a beat picks.
    fn earnable_practices(&self) -> Vec<u8> {
        let held = &self.world.committed().practices;
        cfe::vocabulary::fold(self.world.chronicle(), cfe::vocabulary::Axis::Practice)
            .into_iter()
            .map(|entry| entry.code)
            .filter(|code| !held.contains(code))
            .collect()
    }

    /// This line's own species manifold (metabolism archetype) and topological
    /// reach, as `recipe::discover` needs them — the SAME `civ_reach`-derived
    /// `AlgebraicRank` shape `abundance_step` already builds for the resolver's
    /// own `reach` argument, kept local here since `discover` wants it doubly
    /// (once to size the tension, again inside `invest_craft` to act on it).
    fn species_and_reach(&self) -> (cfe::morphogen::Metabolism, soc::AlgebraicRank) {
        let species = cfe::morphogen::archetype_named(&self.spec.civilization.metabolism);
        let reach = soc::AlgebraicRank {
            components: (self.world.civ_reach() as u64).max(1),
            cycles: 0,
            relations: Vec::new(),
        };
        (species, reach)
    }

    /// The universal roles (`morphogen::FOOD..SINK`) this line's held almanac
    /// could fill via `recipe::discover` but hasn't yet raised a recipe for —
    /// the raw material CRAFT's tension reads and `invest_craft` draws from.
    /// A role the almanac cannot fill (no held element projects into it) is an
    /// honest hole, never counted — `discover` itself is the single source of
    /// truth on workability, never re-derived here.
    fn craftable_roles(&self) -> Vec<usize> {
        let held: Vec<[u8; 32]> = self
            .world
            .committed()
            .prospects
            .iter()
            .map(|p| p.element)
            .collect();
        if held.is_empty() {
            return Vec::new();
        }
        let built: std::collections::HashSet<usize> =
            cfe::vocabulary::recipes_built(self.world.chronicle())
                .into_iter()
                .map(|r| r.role)
                .collect();
        let (species, reach) = self.species_and_reach();
        (0..cfe::morphogen::ROLE_DIM)
            .filter(|role| !built.contains(role))
            .filter(|&role| recipe::discover(role, &held, &species, &reach).is_some())
            .collect()
    }

    /// **THE ABUNDANCE STEP** (COG-EMERGENCE-2) — above the survival floor
    /// (`express_survival`, the sole zone-raiser — N-I6), the line makes ONE
    /// surplus choice per beat, not three authored gates. It FEELS the three
    /// tensions and draws one act — deepen, expand, synthesise, or hoard — off
    /// its own chronicle digest. NO affordability is consulted: the will is
    /// topological; the chosen act pays its own bill or is REFUSED
    /// (`seat_science_as`/`chart_as` return `Err` on an unpayable bill, or the
    /// founding on toxic leverage) AFTER the choice. Two lines, same surplus,
    /// different histories → different acts. Replay-exact.
    fn abundance_step(&mut self) {
        let primary = cfe::world::FIRST_ORB;
        // A world must exist to act upon — a precondition, not a decision.
        if self.world.orb().is_none() || self.world.star().is_none() {
            return;
        }
        // NO surplus precondition, NO affordability. The fold runs every beat a
        // world stands and draws by tension + instinct ALONE; a chosen act pays
        // its own resource bill or is REFUSED by physics after the fact (cognition
        // seats in Iron/Crystal, never in food, so a lean line's Deepen attempt is
        // rejected, not a famine). Measurement (EMERGENCE-2b) confirmed: with the
        // base chem seat kept in survival, the fold needs no leisure gate — the
        // settlement law holds and cognition seats at baseline parity.
        let (cognitive, expansion, synthesis, culture, craft) = self.abundance_tensions();
        // `reach` is the line's survey depth — its accumulated history —
        // perturbing the draw. It scales every weight equally, so it never biases
        // WHICH act, only how this line's own digest lands.
        let reach = soc::AlgebraicRank {
            components: (self.world.civ_reach() as u64).max(1),
            cycles: 0,
            relations: Vec::new(),
        };
        let Some(receipt) = self.world.chronicle().receipts().last() else {
            return;
        };
        let mut hasher = blake3::Hasher::new();
        hasher.update(&receipt.own().0);
        hasher.update(b"abundance-resolve");
        let seed = *hasher.finalize().as_bytes();
        // SOC-5 — THE COLLECTIVE RESOLVE. The society's self-organisation
        // (SOC-4's live caste census) sets how decisively it spends its surplus:
        // a workforce concentrated on one shared deficit commits (a low Hoard
        // floor), a covered or scattered one rests. The concentration is a fold
        // of the distributed caste draws — the collective converging on its own
        // deepest void, no central plan — and it descends the residual: a sharp
        // deficit focuses the surplus that closes it, and a closed deficit
        // dissolves the caste that formed on it.
        let convergence = soc::caste_convergence(&self.caste_partition());
        match soc::resolve_investment(
            &reach,
            cognitive,
            expansion,
            synthesis,
            culture,
            craft,
            convergence,
            seed,
        ) {
            soc::Investment::Deepen => self.invest_cognition(primary, seed),
            soc::Investment::Expand => self.invest_expansion(),
            soc::Investment::Synthesize => {
                let _ = self.world.seat_science_as(
                    self.automaton,
                    primary,
                    cfe::almanac::ScienceDiscipline::Chem,
                );
            }
            soc::Investment::Culture => self.invest_culture(seed),
            soc::Investment::Craft => self.invest_craft(primary, seed),
            soc::Investment::Hoard => {}
        }
    }

    /// The CULTURE act — take up one earned-but-unclaimed practice from the
    /// fold. Which one is a seed-draw over `earnable_practices()` (the same
    /// chronicle-digest discipline every other draw in this file uses), so two
    /// lines with the same unclaimed set but different histories pick
    /// differently. NO pre-filtering by the tempering gate: the will names a
    /// practice, `adopt_practice_as` is the physics that refuses an untempered
    /// one AFTER the choice — the same law `invest_cognition`'s unaffordable
    /// seat already keeps.
    fn invest_culture(&mut self, seed: [u8; 32]) {
        let earnable = self.earnable_practices();
        if earnable.is_empty() {
            // CLOSING THE LOOP: this line has adopted every practice its own
            // fold contains — the earned path alone would leave it idle
            // forever (the fold's "seed ∪ minted" was, until now, ALWAYS just
            // "seed": `mint_vocabulary_as`'s only live caller was the
            // operator's CLI, never a civ's own derivation). If a real void
            // still stands, invent one — the same "read the residual, tilt
            // toward closing it" law the whole abundance fold already runs
            // on, applied to vocabulary itself instead of zones/science.
            self.invent_practice(seed);
            return;
        }
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed);
        hasher.update(b"culture");
        let draw = *hasher.finalize().as_bytes();
        let index =
            (u64::from_le_bytes(draw[..8].try_into().expect("8 bytes")) as usize) % earnable.len();
        let _ = self
            .world
            .adopt_practice_as(self.automaton, earnable[index]);
    }

    /// **THE CRAFT ACT** (RCP-2 F4) — raise one earned-but-unbuilt recipe:
    /// which ROLE is a seed-draw over `craftable_roles()` (the same
    /// chronicle-digest discipline every other draw in this file uses), then
    /// `recipe::discover` names the exact transformation (which held element,
    /// how much it yields) and `build_recipe_as` raises it. No pre-filtering
    /// beyond `craftable_roles()`'s own honesty check — a role the almanac
    /// truly cannot fill was never a candidate, so `discover` inside here
    /// always succeeds for a role this same beat's tension already vetted.
    fn invest_craft(&mut self, primary: u64, seed: [u8; 32]) {
        let roles = self.craftable_roles();
        if roles.is_empty() {
            return;
        }
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed);
        hasher.update(b"craft");
        let draw = *hasher.finalize().as_bytes();
        let index =
            (u64::from_le_bytes(draw[..8].try_into().expect("8 bytes")) as usize) % roles.len();
        let role = roles[index];
        let held: Vec<[u8; 32]> = self
            .world
            .committed()
            .prospects
            .iter()
            .map(|p| p.element)
            .collect();
        let (species, reach) = self.species_and_reach();
        if let Some(minted) = recipe::discover(role, &held, &species, &reach) {
            let _ = self.world.build_recipe_as(
                self.automaton,
                primary,
                minted.inputs,
                minted.role,
                minted.yield_of_role,
            );
        }
    }

    /// **INVENT A PRACTICE** — mint a new discipline into THIS line's own
    /// fold, derived from what it has actually lived, never a random draw:
    /// - the FRICTION (`CHANNEL_TEMPER`) is the line's own weathered hardship
    ///   count — an invented discipline demands proof of exactly the trial
    ///   this line has already survived, not an arbitrary number;
    /// - the BENEFIT (`CHANNEL_DEMURRAGE`) relieves decay in proportion to
    ///   how deep the still-unspanned residual runs — the same shape the
    ///   seeded Hoard practice already wears (`CHANNEL_DEMURRAGE` < 1), so
    ///   an invented discipline is recognizably the SAME kind of thing a
    ///   progenitor would have authored, not a foreign mechanic.
    /// Refuses (no-op) when there is no residual to name a discipline after —
    /// a covered economy has nothing left to invent a cure for.
    fn invent_practice(&mut self, seed: [u8; 32]) {
        let Some(peel) = self.civic_peel() else {
            return;
        };
        let ext = civic::residual_extent(&peel);
        if ext == 0 {
            return;
        }
        let hardships = self
            .world
            .chronicle()
            .receipts()
            .iter()
            .filter(|receipt| {
                matches!(
                    receipt.event(),
                    cfe::dop::ClosureEvent::FamineStruck { .. }
                        | cfe::dop::ClosureEvent::OverheatStruck { .. }
                        | cfe::dop::ClosureEvent::VentSettled { .. }
                        | cfe::dop::ClosureEvent::DemurrageSettled { .. }
                        | cfe::dop::ClosureEvent::DemurrageTouched { .. }
                )
            })
            .count();
        let temper = ratio(hardships as i64, 1);
        // Bounded relief: deeper residual buys more, but the well-formed
        // channel table (`vocabulary::channel_valid`'s own trading law) keeps
        // any single channel inside its lawful range — never overflows past
        // full retention, only approaches it.
        let relief = ratio(1, 1) - ratio(1, (ext.min(15) + 2) as i64);
        let Ok(binding) = cfe::vocabulary::TradingBinding::new(vec![
            (cfe::vocabulary::CHANNEL_DEMURRAGE, relief),
            (cfe::vocabulary::CHANNEL_TEMPER, temper),
        ]) else {
            return;
        };
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed);
        hasher.update(b"invent-practice");
        let name_seed = *hasher.finalize().as_bytes();
        let word_text = cfe::dop::tech_node_name(7, &name_seed);
        let Some(word) = cfe::dop::Name::new(&word_text) else {
            return;
        };
        let sigil = format!("prac-{}", &hex(&name_seed)[..6]);
        let _ = self.world.mint_vocabulary_as(
            self.automaton,
            &self.spec.civilization.sigil,
            cfe::vocabulary::Axis::Practice,
            &sigil,
            word,
            binding,
        );
    }

    /// The DEEPEN act — seat one hard-cognitive science (Math, Physics, Research)
    /// that has headroom, the sub-draw weighted by the species' seat affinity
    /// (Primary 4 · Secondary 2 · Alien 1). Called only once the abundance
    /// resolver has CHOSEN to deepen; the seat pays its bill or is refused
    /// (unaffordable ⇒ the line willed a bigger brain it cannot yet build). Was
    /// `cognitive_investment_step` — its leisure/famine/capacity gates dissolved
    /// into the abundance tension.
    ///
    /// **BEYOND THE BASE SEAT**: the candidate set is any discipline below its
    /// OWN `max_rank()`, not just the unseated ones — `seat_science_as` already
    /// elevates rank on a repeat call (1→2), so the same action that seats a
    /// science from cold also deepens one already standing, exactly mirroring
    /// how Chemistry's `Synthesize` arm deepens `chem_rank` past its base seat
    /// (`abundance_tensions`). Without this, Deepen's own action space died the
    /// moment all three sciences reached rank 1, independent of any tension math
    /// — the plateau `cognitive_capacity`'s doc comment names.
    fn invest_cognition(&mut self, primary: u64, seed: [u8; 32]) {
        use cfe::almanac::ScienceDiscipline::{Math, Physics, Research};
        let met = self.world.genesis().map(|g| g.metabolism).unwrap_or(2);
        let clade = Some(self.world.rejuvenation_clade());
        let open: Vec<cfe::almanac::ScienceDiscipline> = [Math, Physics, Research]
            .into_iter()
            .filter(|d| self.world.science_rank(primary, *d) < d.max_rank())
            .collect();
        if open.is_empty() {
            return;
        }
        let weights: Vec<u64> = open
            .iter()
            .map(|d| {
                let fee = cfe::almanac::science_seat_affinity(met, *d, clade).fee_mult();
                (4 / fee).max(1) as u64
            })
            .collect();
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed);
        hasher.update(b"which-science");
        let sub = *hasher.finalize().as_bytes();
        if let Some(i) = soc::partition_draw(&weights, sub) {
            let _ = self.world.seat_science_as(self.automaton, primary, open[i]);
        }
    }

    /// The EXPAND act — charter one new world in the system. Called only once the
    /// abundance resolver has CHOSEN to expand; the system cap is a physical
    /// bound, and the TRUE-WEALTH solvency (operator ruling 2026-08-15: no
    /// toxic-leverage founding off floored cash) is the post-choice PHYSICS bound,
    /// NOT a pre-decision gate. Was `expand_step`'s charter branch; the
    /// supply-ring weave stays every-beat in `expand_step`.
    fn invest_expansion(&mut self) {
        const SYSTEM_CAP: usize = 4;
        // A line cannot PROJECT settlement to a new world before it has matured
        // (level ≥ 3) — a physical progression bound, kept post-choice (the line
        // may WILL to expand young; it simply cannot yet reach). Founding early
        // would split the food base and tip the home orb into famine.
        if self.level < 3 {
            return;
        }
        let orbs: Vec<u64> = self
            .world
            .committed()
            .orbs
            .iter()
            .map(|o| o.orb_id)
            .collect();
        if orbs.len() >= SYSTEM_CAP {
            return; // the system is full — a physical cap
        }
        let home = *orbs.first().unwrap_or(&cfe::world::FIRST_ORB);
        let accrued = self.world.accrued_of(home);
        let liquid = |resource| {
            let pending: Ratio = accrued
                .iter()
                .filter(|(r, _)| *r == resource)
                .fold(ratio(0, 1), |sum, (_, a)| sum + a.clone());
            banked(&self.world, resource) + pending
        };
        let solvent = cfe::dop::founding_reserves()
            .into_iter()
            .all(|(resource, moved)| liquid(resource) >= moved);
        if solvent {
            let name = format!("{}-orb{}", self.spec.world_name, orbs.len());
            let _ = self.world.chart_as(self.automaton, &name, home);
        }
    }

    fn build_step(&mut self) {
        // N-I6 — build_step NO LONGER RAISES ZONES. The survival express
        // (`express_survival`, post-tick) is the SOLE zone-raiser; Policy and
        // think must not both build. `gather_for_forge` stays pre-tick — it
        // stocks the forge, it does not settle. The old settlement / industrial
        // / regenerator / forge-hold arms and every decider they called
        // (`settlement_choice` / `industrial_choice` / `score_outcome` /
        // `lookahead::project` / `soc::choose_build`) are OFF the live path.
        self.gather_for_forge();
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
                (Some(tile), Some(kind)) => self
                    .world
                    .zone_as(self.steward, cfe::world::FIRST_ORB, tile, kind)
                    .is_ok(),
                _ => false,
            },
            ("raze", [tile]) => match parse_tile(tile) {
                Some(tile) => self
                    .world
                    .raze_as(self.steward, cfe::world::FIRST_ORB, tile)
                    .is_ok(),
                None => false,
            },
            ("collect", []) => self
                .world
                .collect_as(self.steward, cfe::world::FIRST_ORB)
                .is_ok(),
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
    // N-I4: idle on the beat. This shaped the retired SEARCH's beam; the one
    // reduce organ observes the forged task's train pairs directly, with no
    // beam to tilt. Preserved for the survival seat (N-I5), where Need/instinct
    // may ORDER which leftover or book a starving line observes first — an
    // ordering of observation, never a widening or a search.
    #[allow(dead_code)]
    fn hunger(&self) -> Need {
        let now = self.world.now();
        let receipts = self.world.chronicle().receipts();
        let since = |matches: &dyn Fn(&cfe::dop::ClosureEvent) -> bool| -> u64 {
            receipts
                .iter()
                .rev()
                .find(|r| matches(r.event()))
                .map_or(now, |r| now.saturating_sub(r.stamp()))
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
            receipts.iter().filter(|r| matches(r.event())).count() as u64
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
        Need {
            hungers,
            starved_for,
            thin: Vec::new(),
            reach_bonus: 0,
            capacity: 0,
        }
    }

    /// **MULTI-ORB EXPANSION** (INTELLIGENCE ARC A1) — a matured
    /// civilization grows into its star system, and that growth is what
    /// gives its mind something to stand on.
    ///
    /// Once a line has climbed its research ladder (rung ≥ 3 — cognition
    /// EARNED, not a clock) and its home is fed, it CHARTS a colony orb
    /// and weaves its orbs into an internal SUPPLY RING (standing
    /// inter-orb bonds, each orb streaming to the next and the last
    /// closing back). That circular internal economy is exactly what the
    /// relational algebra reads as RANK (`bridge::read_entangled`): a
    /// civilization whose logistics form obligation cycles no simple
    /// hierarchy can explain. It is the structure B-E1's depth coupling
    /// was blocked on — every book held ONE orb, `rank 0`, so the mind
    /// was pinned at the floor. A2 reads this rank into search depth, so
    /// expansion itself deepens the mind.
    ///
    /// Re-sealing an existing bond refuses and is ignored, so this simply
    /// MAINTAINS the ring as it grows. Nothing is minted — `chart_as`
    /// MOVES founding reserves, and the bonds settle conserved at the
    /// touch.
    fn expand_step(&mut self) {
        // A line expands only once it has climbed, and only when fed.
        if self.level < 3 || self.world.orb().is_none_or(|o| !o.famine.is_empty()) {
            return;
        }
        // THE CHARTER DECISION MOVED to the abundance resolver (Expand arm,
        // EMERGENCE-2 — `invest_expansion`, which keeps the TRUE-WEALTH solvency
        // as its post-choice bound). This step now only weaves the internal
        // supply ring — maintenance that runs every beat a climbed, fed line
        // holds, exactly as before.
        // Weave / maintain the internal supply ring over every orb. An
        // edge whose source cannot yet stream simply settles to nothing
        // (settle_bond floors delivery at zero) — never a negative
        // transfer — so the ring is safe to seal whole; the cycles it
        // closes are the rank the mind stands on.
        let orbs: Vec<u64> = self
            .world
            .committed()
            .orbs
            .iter()
            .map(|o| o.orb_id)
            .collect();
        if orbs.len() >= 2 {
            if let Some(rate) = cfe::dop::Positive::new(ratio(1, 1)) {
                for i in 0..orbs.len() {
                    let (from, to) = (orbs[i], orbs[(i + 1) % orbs.len()]);
                    if let Some(pair) = cfe::dop::Between::new(from, to) {
                        let _ = self.world.seal_bond_as(
                            self.automaton,
                            pair,
                            cfe::dop::Resource::Biomass,
                            rate.clone(),
                            0,
                        );
                    }
                }
            }
        }
    }

    /// **COGNITION STEP** (COG-1) — bank the beat's LEISURE. `cascade_reading`
    /// reads the colony's stratified demand from live committed state and pours
    /// its food flow through the survival tiers (metabolic, then cognitive);
    /// whatever surplus survives is leisure, the budget the Mind spends to
    /// think. A line at or below its survival floor banks nothing — a mind with
    /// no cognitive surplus cannot afford a thought (COG-3). Settled on the
    /// primary orb, where the founding population sits.
    fn cognition_step(&mut self) {
        let primary = cfe::world::FIRST_ORB;
        // No people, no mind: an unpopulated colony has no one to hold leisure,
        // even where its untended wild flow runs a nominal surplus. Cognition
        // is a property of a living population, so a line with none banks
        // nothing (and the cascade's "surplus with no demand" is not leisure).
        if self.world.population_of(primary) == 0 {
            return;
        }
        // COG-3 FASTING: the accumulator's inflow is the cascade's leisure PLUS
        // a base trickle off the syntropic wild flow, so a zero-leisure line is
        // never permanently deadlocked — it thinks slowly rather than never.
        let leisure = civic::cascade_reading(&self.world, primary).leisure;
        let fill = civic::fasting_fill(leisure, self.world.syntropic_flow(primary));
        // COG-5 / ONT-4 COGNITIVE DEMURRAGE: unused leisure dissipates each beat
        // (you cannot bank a nap for ten years). The rate is now DERIVED from the
        // planet's orbit and vitals — `thermal(t_eq4) · dry(albedo) ·
        // orbital_tempo(P)` (`World::leisure_decay_rate`) — not a dial: a hot, dry,
        // fast-orbit world spends its surplus quickly, a cold, wet, distant one
        // hoards it. PROPORTIONAL (`bank × rate`), so zero at an empty bank (a poor
        // line still banks its trickle — COG-3); the collapse it once caused is
        // cured in `settle_leisure` by flooring the FLOW. Net settle = fill − decay.
        let decay = self.world.leisure_bank_of(primary) * self.world.leisure_decay_rate(primary);
        let delta = fill - decay;
        if delta != ratio(0, 1) {
            self.world.settle_leisure(primary, delta);
        }
    }

    /// **THE DEFECT HANDOFF** (COG-12) — run the civic peel over live state and
    /// read the irreducible void BACKWARDS: its SHAPE (`classify_defect` — which
    /// family class can bridge it, from the SNF free/torsion character) and its
    /// SIZE (`mint_cost` — the homological pressure). The Mind reaches toward the
    /// shape and as deep as the size demands. Empty shape / zero pressure when
    /// the economy is covered — then there is no void, and no search.
    /// **THE LIVE CIVIC PEEL** — overlay the population's material deficit on the
    /// standing generators and read the residual γ (`EconomicPeel`), the void no
    /// known structure spans. `None` when there is no world to read. This is the
    /// organ N-I6 orphaned (its live caller `build_step` was removed); EMERGENCE-2
    /// reads it live again — for the abundance TENSIONS, not as a second builder
    /// (`express_survival` stays the sole zone-raiser).
    fn civic_peel(&self) -> Option<civic::EconomicPeel> {
        let world = &self.world;
        let orb = world.orb()?;
        let metabolism = world
            .committed()
            .genesis
            .as_ref()
            .map(|g| g.metabolism)
            .unwrap_or(2);
        let fold = cfe::vocabulary::fold(world.chronicle(), cfe::vocabulary::Axis::Zone);
        let standing: Vec<cfe::dop::ZoneKind> = orb.zones.iter().map(|z| z.kind).collect();
        let primary = cfe::world::FIRST_ORB;
        let pop = world.population_of(primary);
        let demand: Vec<(cfe::dop::Resource, cfe::ratio::Ratio)> = if pop > 0 {
            let carrier = cfe::dop::metabolic_ration(metabolism);
            vec![(carrier, ratio(pop as i64, 1) * world.per_capita_ration())]
        } else {
            Vec::new()
        };
        Some(civic::civic_reading(&fold, metabolism, &standing, &demand, [0u8; 32]).peel)
    }

    /// **WHAT THE SOCIETY SEES** (OCULAR-2, doc 54) — the same live economy the
    /// civic peel reads, but through the ocular organ instead of the lattice: the
    /// exact-rational DIVERGENCE FIELD of the economic connection. `field[i] =
    /// demand_i - supply_i`, the signed per-axis deficit, exact and unrounded — a
    /// positive entry is unmet need, a negative one surplus. A graded field, never
    /// summed or normed. `None` when there is no world. This is the loss-free
    /// reading the abundance drives migrate onto (OCULAR-3): no `quantize`, no
    /// `residual_extent`. A DERIVED read (pure over committed state), so SEAL-B is
    /// untouched while it stands alongside the lattice peel.
    pub fn ocular_deficit(&self) -> Option<Vec<cfe::ratio::Ratio>> {
        let world = &self.world;
        let orb = world.orb()?;
        let metabolism = world
            .committed()
            .genesis
            .as_ref()
            .map(|g| g.metabolism)
            .unwrap_or(2);
        let axes = civic::resource_axes();
        let standing: Vec<cfe::dop::ZoneKind> = orb.zones.iter().map(|z| z.kind).collect();
        let primary = cfe::world::FIRST_ORB;
        let pop = world.population_of(primary);
        let demand_loads: Vec<(cfe::dop::Resource, cfe::ratio::Ratio)> = if pop > 0 {
            let carrier = cfe::dop::metabolic_ration(metabolism);
            vec![(carrier, ratio(pop as i64, 1) * world.per_capita_ration())]
        } else {
            Vec::new()
        };
        let demand = civic::demand_exact(&demand_loads, &axes);
        let supply = civic::standing_supply_exact(&standing, metabolism, &axes);
        Some(ocular::seen_deficit(&demand, &supply))
    }

    /// **THE LIVE CASTE CENSUS** (SOC-4b) — the society self-organises this beat.
    /// Partition the working population across the unspanned residual axes the
    /// live civic peel reads (`civic_peel().residual`, the void no standing
    /// structure spans): each member is DRAWN toward the work most unmet, off
    /// the line's own committed digest, never assigned by a table. A DERIVED read
    /// — pure over committed state, exactly like `ration_survives` — so replay
    /// reads the same partition and SEAL-B is untouched. Empty when there is no
    /// world; all-zero over the residual axes when the population is nil or the
    /// economy is covered (no deficit ⇒ no caste, a homogeneous society). This is
    /// the pure `soc::caste_census` organ finally reading the LIVE deficit rather
    /// than a fixture; the shared objective SOC-5 will converge on is this census.
    pub fn caste_partition(&self) -> Vec<u64> {
        let Some(peel) = self.civic_peel() else {
            return Vec::new();
        };
        let members = self.world.labor_force(cfe::world::FIRST_ORB);
        if members == 0 {
            return vec![0u64; peel.residual.len()];
        }
        // The line's OWN history salts the draw (SOC-2's emergence law): two
        // societies with the same deficit but different pasts organise
        // differently. The last committed digest is that history, deterministic.
        let seed = self
            .world
            .chronicle()
            .receipts()
            .last()
            .map(|receipt| receipt.own().0)
            .unwrap_or([0u8; 32]);
        soc::caste_census(&peel.residual, members, seed)
    }

    #[allow(dead_code)]
    fn civic_defect(&self) -> (Vec<solver::need::Family>, u64) {
        let Some(peel) = self.civic_peel() else {
            return (Vec::new(), 0);
        };
        let shape = civic::classify_defect(&peel.residual, &peel.torsion);
        let pressure =
            num_traits::ToPrimitive::to_u64(&civic::mint_cost(&peel.residual, &peel.torsion))
                .unwrap_or(0);
        (shape, pressure)
    }

    /// N-I5 — THE RATION BOOK'S DIRECTION: the species's owed fiber index. A
    /// Resource ration is its discriminant (1..=7, distinct per metabolism); an
    /// element-eater owes the reserved element slot. Never `UNFED` (0), so the
    /// unfed mouth stays distinguishable from a fed one.
    fn ration_direction(ration: &cfe::dop::Ration) -> u8 {
        match ration {
            cfe::dop::Ration::Resource(r) => *r as u8,
            cfe::dop::Ration::Element(_) => 8,
        }
    }

    /// N-I5 — THE SURVIVAL OBSERVATION. Peel the ration book of one orb into
    /// HAVE|OWE (mouths as sites over the species-ration fiber) and read it with
    /// the ONE organ (`solver::ration_reads` → `reduce_pair`). `true` iff the
    /// larder feeds every working mouth this beat. No mouths — or a rate-0 line
    /// with no farmed ration (plasmoid, #29) — trivially survives: there is no
    /// book to peel. A pure deterministic read (labor, sustenance, ration), so
    /// replay reads the same verdict and SEAL-B is untouched.
    pub fn ration_survives(&self, orb: u64) -> bool {
        let mouths = self.world.labor_force(orb);
        if mouths == 0 {
            return true;
        }
        let per_capita = self.world.per_capita_ration();
        if per_capita <= ratio(0, 1) {
            return true;
        }
        let stamp = self.world.now();
        let available: Ratio = self
            .world
            .sustenance_sources(orb, stamp)
            .iter()
            .map(|source| source.available())
            .fold(ratio(0, 1), |sum, amount| sum + amount);
        // Mouth-granular coverage — how many mouths the larder feeds this beat.
        // `to_integer` truncates toward zero, which IS floor since the quotient
        // is non-negative; capped at the mouths (surplus feeds no phantom).
        let fed = num_traits::ToPrimitive::to_u64(&(available / per_capita).to_integer())
            .unwrap_or(mouths)
            .min(mouths);
        let ration_dir = Self::ration_direction(&self.world.ration_of(orb));
        matches!(
            solver::ration_reads(mouths, fed, ration_dir),
            solver::RationRead::Fed
        )
    }

    /// N-I6 — peel the survival book for one orb: the exact integer readings the
    /// solver seat thinks (mouths, fed, accrued, housing, matter), all
    /// mouth-/unit-equivalents. The conductor peels the world; the solver thinks
    /// the measurement. A pure read (SEAL-B-safe).
    fn survival_book(&self, orb: u64) -> solver::SurvivalBook {
        let world = &self.world;
        let ration = world.ration_of(orb);
        let ration_dir = Self::ration_direction(&ration);
        let metabolism = world
            .committed()
            .genesis
            .as_ref()
            .map(|g| g.metabolism)
            .unwrap_or(2);
        let ration_resource = match &ration {
            cfe::dop::Ration::Resource(r) => Some(*r),
            cfe::dop::Ration::Element(_) => None,
        };
        let yields_ration = |kind: ZoneKind| match ration_resource {
            Some(r) => cfe::dop::zone_yield(kind, metabolism).0 == r,
            None => kind == ZoneKind::Extractor, // an element-eater mines its ration
        };
        let ration_source_sites = world
            .orb()
            .map(|o| o.zones.iter().filter(|z| yields_ration(z.kind)).count() as u64)
            .unwrap_or(0);
        let mouths = world.labor_force(orb);
        let per_capita = world.per_capita_ration();
        let to_u64 =
            |r: Ratio| num_traits::ToPrimitive::to_u64(&r.to_integer()).unwrap_or(u64::MAX);
        let (fed, accrued, habitat_support) = if per_capita <= ratio(0, 1) {
            (mouths, mouths, mouths) // a rate-0 line (plasmoid, #29) has no farmed ration gate
        } else {
            let available: Ratio = world
                .sustenance_sources(orb, world.now())
                .iter()
                .map(|s| s.available())
                .fold(ratio(0, 1), |a, b| a + b);
            let fed = to_u64(available / per_capita.clone()).min(mouths);
            let accrued_amt = ration_resource
                .and_then(|r| {
                    world
                        .accrued_of(orb)
                        .into_iter()
                        .find(|(res, _)| *res == r)
                        .map(|(_, a)| a)
                })
                .unwrap_or_else(|| ratio(0, 1));
            let accrued = to_u64(accrued_amt / per_capita.clone());
            let support = to_u64(world.population_capacity(orb) / per_capita);
            (fed, accrued, support)
        };
        solver::SurvivalBook {
            ration_source_sites,
            mouths,
            fed,
            ration_dir,
            accrued,
            habitat_support,
            habitats: world.habitat_count(orb),
            extractors: zone_count(world, ZoneKind::Extractor) as u64,
            matter_opened: false,
            alloy: to_u64(banked(world, Resource::Alloy)),
            iron: to_u64(banked(world, Resource::Iron)),
        }
    }

    /// N-I6 — the ZoneKind whose `zone_yield` is this line's ration (charter §3
    /// founding): Habitat for a biomass line, Extractor for a mineral/element
    /// line. Resolved from `zone_yield`, never hardcoded.
    fn ration_source_zone(&self, orb: u64) -> ZoneKind {
        let metabolism = self
            .world
            .committed()
            .genesis
            .as_ref()
            .map(|g| g.metabolism)
            .unwrap_or(2);
        if let cfe::dop::Ration::Resource(r) = self.world.ration_of(orb) {
            for kind in [ZoneKind::Habitat, ZoneKind::Extractor] {
                if cfe::dop::zone_yield(kind, metabolism).0 == r {
                    return kind;
                }
            }
        }
        ZoneKind::Extractor
    }

    /// N-I6 — THE SURVIVAL EXPRESS. Peel the book, think the deed, express ≤1
    /// deed through a door that already exists (`zone_as` / `raze_as`; Collect and
    /// Rest express nothing extra — the cycle collect IS the deed). The SOLE
    /// zone-raiser now (`build_step` no longer builds). Runs in the post-tick
    /// window so it reads the food that feeds the mouths, not the drained larder.
    /// `industry_relevant` / affordability stay RENDER gates — an unaffordable or
    /// off-metabolism deed is rest, not a score.
    fn express_survival(&mut self, orb: u64, matter_opened: bool) {
        let mut book = self.survival_book(orb);
        book.matter_opened = matter_opened;
        let metabolism = self.spec.civilization.metabolism.clone();
        match solver::think_survival(&book) {
            solver::SurvivalDeed::Rest | solver::SurvivalDeed::Collect => {}
            solver::SurvivalDeed::SeatRationSource => {
                let kind = self.ration_source_zone(orb);
                self.seat_zone(orb, kind);
            }
            solver::SurvivalDeed::Habitat => self.seat_zone(orb, ZoneKind::Habitat),
            solver::SurvivalDeed::RazeExtractor => self.raze_zone(orb, ZoneKind::Extractor),
            solver::SurvivalDeed::Synthesizer => {
                if industry_relevant(&metabolism, ZoneKind::Synthesizer) {
                    self.seat_zone(orb, ZoneKind::Synthesizer);
                }
            }
            solver::SurvivalDeed::Smelter => {
                if industry_relevant(&metabolism, ZoneKind::Smelter) {
                    self.seat_zone(orb, ZoneKind::Smelter);
                }
            }
        }
    }

    /// Seat one zone if a tile is free and its cost is banked (the render gate —
    /// an unaffordable deed is rest). One zone per beat is the metabolic bound.
    ///
    /// BOT-THERM (N-I6 follow-up, 2026-08-23): `joule_runway_allows` was
    /// written against `build_step`'s zone-raising and orphaned when
    /// N-I6 moved zone-raising here (`express_survival`) without
    /// carrying the gate — a sub-mesic (Frost) colony built with no
    /// runway check at all, which is why the runway pin's own frost-
    /// vs-mesic comparison inverted (a cold colony's PER-ZONE cost
    /// gate alone is not the same law as its ONE-TICK HEATING BILL
    /// gate). Checked here, ADDITIONALLY to the ordinary cost gate —
    /// never in place of it.
    fn seat_zone(&mut self, orb: u64, kind: ZoneKind) {
        let can_pay = kind
            .cost()
            .into_iter()
            .all(|(res, need)| banked(&self.world, res) >= need);
        if !can_pay {
            return;
        }
        if let Some((Regime::Frost, _)) = regime(&self.world, &self.spec.civilization.metabolism) {
            let zones_now = self.world.orb().map(|o| o.zones.len()).unwrap_or(0);
            if !joule_runway_allows(&banked(&self.world, Resource::Joule), zones_now) {
                return;
            }
        }
        if let Some(tile) = next_free_tile(&self.world) {
            // The founding band arrived with the colony at genesis
            // (`founding_band`); a Habitat raises capacity, never seeds people.
            let _ = self.world.zone_as(self.automaton, orb, tile, kind);
        }
    }

    /// Raze one zone of the given kind — the typed inverse of build (Pin E).
    fn raze_zone(&mut self, orb: u64, kind: ZoneKind) {
        let tile = self
            .world
            .orb()
            .and_then(|o| o.zones.iter().find(|z| z.kind == kind).map(|z| z.tile));
        if let Some(tile) = tile {
            let _ = self.world.raze_as(self.automaton, orb, tile);
        }
    }

    /// N-I7 — THE LOOK PRICE: the cheapest thought (a single degree-1
    /// observation), billed in leisure through the SEATED cost law
    /// (`civic::mint_cost × metabolic_exchange`) — no new rate table. A constant,
    /// strictly cheaper than any composed word (`mint_cost` is monotone in
    /// residual extent); it moves automatically if the seated cost law moves.
    pub fn observe_price() -> Ratio {
        let bill = civic::mint_cost(&[num_bigint::BigInt::from(1)], &[]);
        ratio(
            num_traits::ToPrimitive::to_i64(&bill).unwrap_or(i64::MAX),
            1,
        ) * civic::metabolic_exchange()
    }

    /// AUTIC-1 Slice 1b — THE VESSEL ABSORBS THE SHARED TONGUE. Draw a
    /// hash-seeded subset of net's learned AUTIC basin (the crystals net acquired
    /// from the mycelium network's walls) into this `vessel`'s grown basin, so it
    /// REDUCES against the shared language in `think_task`. This is the entity-scale
    /// end of the recursion: net learns AUTIC at the membrane, the entities come to
    /// think in it. EMERGENT, not forced — a per-vessel hash gate admits a different
    /// ~1/3 subset per vessel, so dialects persist where derivations differ (operator:
    /// "let it emerge, measure it"). Idempotent: dedup by class, so re-drawing each
    /// beat only picks up newly learned crystals. Replay-safe: `grown` is live
    /// workspace, never chronicled, and replay folds recorded deeds rather than
    /// re-running think — so this changes what the LIVE beat discovers (recorded as
    /// deeds), never the replay. In headless `run_era` the membrane is inert (empty
    /// basin), so this is a no-op and SEAL B holds unchanged.
    fn absorb_autic(&mut self, vessel: u64) -> usize {
        let depth = scribe::membrane::autic_basin_depth();
        let mark = self.autic_seen.get(&vessel).copied().unwrap_or(0);
        // O(new): only crystals learned since this vessel last looked. Nothing new
        // (run_era off-wire, or no fresh mesh learning) => cheap exit, no clone.
        if mark >= depth {
            return 0;
        }
        let fresh = scribe::membrane::autic_basin_from(mark);
        let basin = self.grown.entry(vessel).or_default();
        let mut added = 0;
        for (k, crystal) in fresh.into_iter().enumerate() {
            if basin.len() >= AUTIC_BASIN_CAP {
                break; // a mind is bounded — do not grow the basin without end
            }
            // Per-vessel stochastic gate, deterministic in (vessel, index): two odd
            // mixing constants spread the low bits so `% 3` admits ~1/3 evenly. Each
            // crystal is offered to a vessel exactly once (past its mark), so no
            // `contains` scan is needed — the mark IS the dedup.
            let index = (mark + k) as u64;
            let seed = vessel
                .wrapping_mul(0x9E3779B97F4A7C15)
                .wrapping_add(index)
                .wrapping_mul(0xD1B54A32D192ED03);
            if seed % 3 == 0 {
                basin.push(crystal);
                added += 1;
            }
        }
        self.autic_seen.insert(vessel, depth);
        self.autic_absorbed += added;
        added
    }

    /// AUTIC-2 — THE COMPOSE MOVE, LIVE. The `speaker` splices its grown words
    /// into an `AuticSentence` (a composed multi-word utterance) and the `hearer`
    /// ASSEMBLES it against ITS OWN basin: the pieces it recognises are accounted,
    /// and any leftover (`DemandUnmet`) is exactly the structure it lacked — which
    /// it LEARNS by adopting the remainder as a new crystal. This is genuine
    /// knowledge transfer between two minds: one composes a thought from what it
    /// knows, the other comprehends what it can and grows by precisely what it
    /// could not. Bounded to a real sentence (up to AUTIC_SENTENCE_LEN of the
    /// speaker's freshest words), not the whole basin. Replay-safe: `grown` is
    /// live workspace, folded deeds carry replay. A no-op until a speaker holds
    /// two words, so early/isolated minds are silent, not fabricating speech.
    /// Observability — the largest vessel basin this line holds (AUTIC-2 probe).
    pub fn max_basin(&self) -> usize {
        self.grown.values().map(|v| v.len()).max().unwrap_or(0)
    }

    /// Observability — this civilization's whole VOCABULARY: the distinct crystal
    /// classes held across all its vessels' basins (the words it can speak and
    /// hear). Used to prove a shared language across civilizations.
    pub fn basin_classes(&self) -> Vec<reduct::ChainDelta> {
        let mut classes: Vec<reduct::ChainDelta> = Vec::new();
        for basin in self.grown.values() {
            for crystal in basin {
                if !classes.contains(&crystal.class) {
                    classes.push(crystal.class.clone());
                }
            }
        }
        classes
    }

    /// Observability — up to `limit` distinct crystal classes from this civ's
    /// vocabulary. Bounded so the O(civs^2 * words) shared-language census stays
    /// cheap on capped basins (the full `basin_classes` dedup is O(basin^2)).
    pub fn basin_sample(&self, limit: usize) -> Vec<reduct::ChainDelta> {
        let mut classes: Vec<reduct::ChainDelta> = Vec::new();
        for basin in self.grown.values() {
            for crystal in basin {
                if classes.len() >= limit {
                    return classes;
                }
                if !classes.contains(&crystal.class) {
                    classes.push(crystal.class.clone());
                }
            }
        }
        classes
    }

    /// AUTIC-1 CURRICULUM AXIS, LIVE. The vessel PRACTISES the mesh's own
    /// problems: draw a hash-seeded wall from net's published AUTIC walls (the
    /// mesh's real ARC-AGI-2 demand, heard through the membrane) and HEAR it
    /// through this vessel's own organ (`solver::hear`) — a second, INDEPENDENT
    /// vantage on the mesh's problem, growing the basin where the vessel walls.
    /// This is the entities facing the network's problems, not only forged ones;
    /// where `absorb_autic` copies net's ANSWERS, this practises net's QUESTIONS.
    /// Emergent (a different wall per vessel by hash), replay-safe (`grown` is
    /// workspace; replay folds deeds). A no-op with no published walls (run_era
    /// off-wire), so it is silent, not dormant.
    fn practise_autic_wall(&mut self, vessel: u64) {
        // PERF — practice is periodic, not every think. A global beat already
        // steps 107 civs through think_task; adding a full solver::hear per civ
        // per beat dominates the beat. Every AUTIC_STRIDE beats keeps the mesh
        // curriculum flowing without tripling the per-beat cognition cost.
        if !self.world.now().is_multiple_of(AUTIC_STRIDE) {
            return;
        }
        // A bounded mind does not practise into an unbounded basin.
        if self.grown.get(&vessel).map(|b| b.len()).unwrap_or(0) >= AUTIC_BASIN_CAP {
            return;
        }
        // Hash-seeded pick: a different wall per (vessel, beat), deterministic.
        // Clones ONE pair (the picker indexes under the lock), not the whole ring.
        let seed = vessel
            .wrapping_mul(0x9E3779B97F4A7C15)
            .wrapping_add(self.world.now())
            .wrapping_mul(0xD1B54A32D192ED03);
        if let Some((a, b)) = scribe::membrane::autic_wall_pick(seed) {
            let _ = solver::hear(&a, &b, self.grown.entry(vessel).or_default());
            self.autic_practiced += 1;
        }
    }

    fn speak_and_hear(&mut self, speaker: u64, hearer: u64) {
        const AUTIC_SENTENCE_LEN: usize = 4;
        // PERF — speech is periodic (see practise): one compose+assemble per civ
        // per beat would dominate the global beat. Every AUTIC_STRIDE beats.
        if !self.world.now().is_multiple_of(AUTIC_STRIDE) {
            return;
        }
        // Compose the sentence from the speaker's freshest words (borrow released
        // before the hearer's basin is taken mutably).
        let words: Vec<reduct::ChainDelta> = match self.grown.get(&speaker) {
            Some(basin) if basin.len() >= 2 => basin
                .iter()
                .rev()
                .take(AUTIC_SENTENCE_LEN)
                .map(|c| c.class.clone())
                .collect(),
            _ => return, // nothing composed to say — silence, not fabricated speech
        };
        let sentence = reduct::AuticSentence::speak(words);
        let hearer_basin = self.grown.entry(hearer).or_default();
        match reduct::assemble(&sentence, hearer_basin) {
            reduct::AssemblyRead::Assembled { .. } => {}
            reduct::AssemblyRead::DemandUnmet { remainder, .. } => {
                // LEARN — the hearer adopts exactly the structure it lacked (bounded).
                if hearer_basin.len() < AUTIC_BASIN_CAP {
                    hearer_basin.push(reduct::Crystal::seal(remainder));
                    self.autic_learned += 1;
                }
            }
        }
        self.autic_spoke += 1;
    }

    /// COG-AUTHOR (A3) — THE VARIATION VERB. The rung between hearing and
    /// authoring: a line CITES a word it already holds, REBINDS it (composes the
    /// cited program with a seeded D4 symmetry — role/quantifier re-aim), PROVES
    /// the variant on a probe grid, and OFFERS it to the mesh door, which judges
    /// it like any word (installs the real ones, withholds noise). The civ's own
    /// beat seeds the choice (replayable, divergent across lines), so the variant
    /// is AUTHORED, not searched — cheaper than deriving from scratch, more than
    /// hearing. This is net CLAIMING (the judge/claimant seam): net authors and
    /// proves; mycelium's grammar is the sole judge. Periodic (AUTIC_STRIDE) and
    /// only when the library is non-empty — a line with nothing to cite is silent.
    fn author_variation(&mut self, vessel: u64) {
        if !self.world.now().is_multiple_of(AUTIC_STRIDE) {
            return;
        }
        let words = self.library.words();
        if words.is_empty() {
            return; // nothing to cite — no authorship without a vocabulary
        }
        // Seed on the civ's own beat + vessel: replayable, divergent across lines.
        let seed = self
            .world
            .now()
            .wrapping_mul(0x9E3779B97F4A7C15)
            .wrapping_add(vessel)
            .wrapping_mul(0xD1B54A32D192ED03);
        // ADOPT BY WORTH — cite the word of GREATEST measured utility (distinct
        // walls it has closed, per the mesh's lexicon.worth), not a hash-random one.
        // A line authors variations of what actually WORKS, so its offers are worth
        // the mesh's judgment. A library with no worth yet (young line, off-wire)
        // falls back to the seeded pick, so authorship never stalls.
        let worthiest = words
            .iter()
            .max_by_key(|w| scribe::membrane::word_worth(&w.orbit))
            .filter(|w| scribe::membrane::word_worth(&w.orbit) > 0);
        let word = worthiest.unwrap_or(&words[(seed as usize) % words.len()]);
        // The six D4 rebinds; the seed picks one.
        use solver_verify::grid::Op;
        const D4: [Op; 6] = [
            Op::Rotate90,
            Op::Rotate180,
            Op::Rotate270,
            Op::FlipH,
            Op::FlipV,
            Op::Transpose,
        ];
        let g = D4[((seed >> 8) as usize) % D4.len()].clone();
        let mut variant = word.expansion.clone();
        variant.push(g);
        let name = format!("net_var_{:016x}", seed);
        let program: String = variant
            .iter()
            .map(|op| op.describe())
            .collect::<Vec<_>>()
            .join(" -> ");
        // Prove the variant on a canonical probe grid — the authored word is a
        // lawful (input -> output) claim the mesh door can replay. Build the
        // sealed-math Grid, run, then cast both to the membrane's row-vectors.
        let Some(probe) = solver_verify::grid::Grid::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        else {
            return;
        };
        let out = solver_verify::grid::run(&variant, &probe);
        let rows = |g: &solver_verify::grid::Grid| -> Vec<Vec<u8>> {
            (0..g.rows)
                .map(|r| g.cells[r * g.cols..(r + 1) * g.cols].to_vec())
                .collect()
        };
        scribe::membrane::offer_proved(&name, &program, vec![(rows(&probe), rows(&out))]);
        self.autic_authored += 1;
    }

    /// N-I8 — deposit `vessel`'s closure of `orbit` toward a shared word, and
    /// MINT it iff two DISTINCT vessels have now closed it (consilience). The
    /// mint bills `mint_cost` in leisure (COG-2/4/5, unchanged) — an unaffordable
    /// consilient word stays pending (fasting), a starving line may burn people
    /// (COG-3). A lone vessel never reaches two (C−): pending is a wait, not an
    /// install. The mint DEED (`mint_solver_word_as`) is unchanged.
    /// ENTITY-ALIGN-1 (docs/56 §3) — the vessel ids that have deposited
    /// toward `orbit` so far, read from `word_pending`'s own per-vessel
    /// slots (see `consilience_deposit`'s doc comment: one slot per
    /// distinct vessel, never per attempt).
    fn vessels_confirming(&self, orbit: [u8; 32]) -> Vec<u64> {
        self.word_pending
            .iter()
            .find(|(held, _, _)| *held == orbit)
            .map(|(_, _, deposits)| deposits.iter().map(|(v, _)| *v).collect())
            .unwrap_or_default()
    }

    fn deposit_toward_word(
        &mut self,
        orbit: [u8; 32],
        expansion: &[solver_verify::grid::Op],
        vessel: u64,
        digest: [u8; 32],
    ) {
        let Some(cited) =
            consilience_deposit(&mut self.word_pending, orbit, expansion, vessel, digest)
        else {
            return; // one vantage waiting — Horizon, not a mint (C−)
        };
        let expansion = expansion.to_vec();
        let bill = civic::mint_cost(&[num_bigint::BigInt::from(expansion.len())], &[]);
        let primary = cfe::world::FIRST_ORB;
        // COG-5: price the homological bill in LEISURE via the exchange rate.
        // COG-11: an EVOLVED line thinks more on less — the price is discounted by
        // `cognitive_efficiency(evolution_stage)` (1/(1+stage), exactly 1 at the
        // founding stage so an un-evolved line pays the same as before).
        let price = ratio(
            num_traits::ToPrimitive::to_i64(&bill).unwrap_or(i64::MAX),
            1,
        ) * civic::metabolic_exchange()
            * civic::cognitive_efficiency(self.world.evolution_stage());
        // COG-13 EROI gate; `afford` survives only as the desperation-burn floor.
        let afford = self.world.leisure_bank_of(primary) >= price;
        let commit = civic::cognitive_investment(&self.world, primary, &price);
        // COG-3 desperation burn: a starving line may spend people for the Eureka.
        let burn = civic::desperation_price(&price);
        let head = ratio(self.world.population_of(primary) as i64, 1);
        let starving = matches!(
            civic::cascade_reading(&self.world, primary).binding,
            Some(civic::Drive::Metabolic)
        );
        let can_burn = !afford && starving && head > burn.clone();
        // COMM-1 — AGREEMENT IS NEVER DISCARDED. Two minds have independently closed
        // the same orbit (consilience reached above) — that shared truth is the rare,
        // precious event, and its cognitive cost was ALREADY paid by the two looks.
        // Minting the word is the RECEIPT of the agreement (a banked-neutral deed,
        // 28_VOCABULARY §7), not a fresh investment to EROI-gate. Discarding an agreed
        // word because the starved economy can't clear the bar is the deepest waste:
        // the civilization reached a shared word and threw it away, guaranteeing it
        // never builds a language. So the agreed word ALWAYS mints; the leisure cost
        // is settled only when a solvent line can pay it (COG-13 EROI) or a starving
        // line burns for the eureka (COG-3) — otherwise the truth is recorded free.
        // The society keeps every shared word it earns; the economy prices the
        // ONGOING draw, never the record of agreement.
        if let Some(bytes) = solver_verify::program_bytes(&expansion) {
            if self
                .world
                .mint_solver_word_as(self.automaton, orbit, bytes, cited.clone())
                .is_ok()
            {
                if commit {
                    self.world.settle_leisure(primary, -price);
                } else if can_burn {
                    self.world.settle_population_exact(primary, -burn);
                }
                // ENTITY-ALIGN-1 (docs/56 §3) — read the confirming vessels'
                // basins BEFORE the pending entry is retained below, then
                // close the hardness reading as its own receipt alongside
                // the mint. Recorded only; the mint above already succeeded
                // on identity alone (cited.len() >= 2), unchanged.
                let vessels = self.vessels_confirming(orbit);
                let hardness = confirmation_hardness(&self.grown, &vessels);
                let _ = self
                    .world
                    .record_word_hardness_as(self.automaton, orbit, hardness);
                // ENTITY-ALIGN-1 — WITNESSED ON THE WIRE, not only in the
                // receipt: the same live signal COG-DIR/COG-ATLAS/COG-LEX
                // already print every beat, so the graded-consolidation
                // measure is directly observable on a running board.
                eprintln!(
                    "ENTITY-ALIGN-1 clock {} {}: orbit={} hardness={} vessels={:?}",
                    self.world.now(),
                    self.spec.civilization.sigil,
                    hex(&orbit)[..8].to_string(),
                    hardness,
                    vessels,
                );
                self.library.adopt(solver::mint::MintedWord {
                    orbit,
                    expansion,
                    cited,
                });
                self.word_pending.retain(|(held, _, _)| *held != orbit);
            }
        }
    }

    /// N-I8 — the SIBLING office's independent deposit toward consilience. It
    /// thinks `task` on ITS OWN basin (the line's second registered office,
    /// `steward`), pays its OWN look (N-I7), and on a verified close deposits
    /// `(steward, digest)` toward the shared word. Deposit-only: no value mint
    /// (the automaton is the line's value producer). It looks only if the bank
    /// can pay AFTER the automaton's look — affordability-staggered, so a line
    /// with one look's surplus funds the automaton first and the sibling fasts.
    fn sibling_deposit(&mut self, task: &solver_verify::loader::Task, digest: [u8; 32]) {
        let orb = cfe::world::FIRST_ORB;
        // COMM-1 — THE CONFIRMING LOOK IS CHEAPER THAN THE DISCOVERING LOOK. The
        // sibling is not searching the orbit the automaton already found; it is
        // CONFIRMING it — verification, not discovery. Communication (a second
        // vantage on the same wall) is the FOUNDATION of a society, not a surplus
        // luxury paid only after discovery, so it must not cost a full second look:
        // priced at full `observe_price` the sibling always fasts under scarcity,
        // consilience never reaches two vessels, and no shared word ever mints
        // (measured: 0 VocabularyMinted / 0 KnowledgeConverged across 8 mature
        // worlds — isolated, non-composing civilizations). Priced as a fraction, a
        // line that can afford ONE discovery can always afford the confirmation, so
        // two vessels close the same orbit and the word mints. The discount is the
        // ratio of verifying a known program to searching for it.
        let price = Self::observe_price() * cfe::ratio::ratio(1, 4);
        if self.world.leisure_bank_of(orb) < price {
            return; // the sibling fasts — even the confirming look is not free (N-I7)
        }
        self.world.settle_leisure(orb, -price);
        let sibling = self.sibling;
        self.absorb_autic(sibling); // think in the shared tongue net has learned
        let read = solver::think_task(task, self.grown.entry(sibling).or_default());
        if let SolveRead::Solved {
            program,
            matches_expected: Some(true),
            ..
        } = read
        {
            if program.len() >= 2 {
                let standing = solver::mint::standing_orbits(&solver::mint::sweep_vocabulary(task));
                if let Some(orbit) = solver::mint::consider(&program, &self.library, &standing) {
                    self.deposit_toward_word(orbit, &program, sibling, digest);
                }
            }
        }
        // N-I10 — after thinking, the sibling HEARS one shared word (culture in
        // motion). Affordability-gated: it hears only if it can still pay.
        self.sibling_hear();
        // AUTIC-2 — THE COMPOSE MOVE, LIVE. Above single-word hearing: the
        // automaton SPEAKS a composed sentence (several of its words spliced) and
        // the sibling ASSEMBLES it against its basin, learning exactly the leftover
        // it lacked. Multi-word thought, not one invariant at a time.
        self.speak_and_hear(self.automaton, self.sibling);
        // COG-AUTHOR (A3) — and the automaton AUTHORS: cites a word, rebinds it,
        // offers the variant to the mesh door. The rung from hearing to authoring.
        self.author_variation(self.automaton);
    }

    /// ENTITY-ALIGN-2 (docs/56 §4, piece a) — occasionally, the SIBLING
    /// chooses its OWN problem instead of confirming the automaton's: it
    /// forges a task off `basin_digest`'s reading of ITS OWN basin, never the
    /// civ's chronicle digest.
    ///
    /// **CORRECTED AFTER LIVE MEASUREMENT** (2026-08-24): originally priced
    /// at the FULL `observe_price`, reasoning that a self-selected problem is
    /// a genuine discovery and should cost what the automaton's own discovery
    /// costs. Run live against 22 founding civs for 480 beats — this NEVER
    /// once fired (zero `ENTITY-ALIGN-2` witness lines; every mint's hardness
    /// stayed 1). The full price is exactly the bar COMM-1's own doc comment
    /// already names as unaffordable in practice ("priced at full
    /// observe_price the sibling always fasts under scarcity... measured: 0
    /// VocabularyMinted / 0 KnowledgeConverged across 8 mature worlds") — I
    /// re-introduced the identical dead-code bar COMM-1 exists to avoid.
    /// Priced at the SAME discount as confirming (COMM-1's `1/4`): the
    /// distinction this slice adds is WHICH problem the sibling pursues, not
    /// a stiffer economic bar on pursuing one — a look the sibling could
    /// already afford to spend confirming, it can spend choosing instead.
    /// Returns `false` (self-select declined; the caller falls back to
    /// confirming) when the sibling has nothing grown to derive from yet, or
    /// cannot afford even the discounted look — autonomy is earned by having
    /// thought something first, never granted free.
    fn sibling_self_select(&mut self) -> bool {
        let sibling = self.sibling;
        let Some(seed_digest) = basin_digest(self.grown.get(&sibling)) else {
            return false;
        };
        let orb = cfe::world::FIRST_ORB;
        let price = Self::observe_price() * cfe::ratio::ratio(1, 4);
        if self.world.leisure_bank_of(orb) < price {
            return false;
        }
        self.world.settle_leisure(orb, -price);
        // ENTITY-ALIGN-2 — WITNESSED ON THE WIRE: the sibling chose its own
        // problem THIS beat, off ITS OWN basin, not the civ's dispatch.
        eprintln!(
            "ENTITY-ALIGN-2 clock {} {}: sibling {} self-selected off its own basin (seed={}), not confirming",
            self.world.now(),
            self.spec.civilization.sigil,
            sibling,
            hex(&seed_digest)[..8].to_string(),
        );
        self.absorb_autic(sibling);
        self.practise_autic_wall(sibling);
        let difficulty = Difficulty::level(self.level);
        let task = forge_task(seed_digest, difficulty);
        let read = solver::think_task(&task, self.grown.entry(sibling).or_default());
        if let SolveRead::Solved {
            program,
            matches_expected: Some(true),
            ..
        } = read
        {
            if program.len() >= 2 {
                let standing = solver::mint::standing_orbits(&solver::mint::sweep_vocabulary(&task));
                if let Some(orbit) = solver::mint::consider(&program, &self.library, &standing) {
                    self.deposit_toward_word(orbit, &program, sibling, seed_digest);
                }
            }
        }
        // Culture propagation is orthogonal to WHICH problem the sibling
        // chose — a self-selecting vessel still hears and speaks.
        self.sibling_hear();
        self.speak_and_hear(self.automaton, self.sibling);
        self.author_variation(self.automaton);
        true
    }

    /// N-I10 — the sibling HEARS one folded word from the sun each beat it can
    /// afford: it drains the word (understood) or WALLS and adopts it (learns a
    /// shared/inherited word its basin lacked). Hearing is looking — it pays
    /// `observe_price` (N-I7); a broke sibling fasts the hear. Bounded to one
    /// word per beat, rotated by the beat clock so the whole vocabulary is heard
    /// over time. No copy — the sibling reaches each word through its own reduce.
    fn sibling_hear(&mut self) {
        let words = self.library.words();
        if words.is_empty() {
            return;
        }
        let orb = cfe::world::FIRST_ORB;
        let price = Self::observe_price();
        if self.world.leisure_bank_of(orb) < price {
            return; // the sibling cannot afford to hear this beat
        }
        let idx = (self.world.now() as usize) % words.len();
        let expansion = words[idx].expansion.clone();
        self.world.settle_leisure(orb, -price);
        let sibling = self.sibling;
        let _ = solver::hear_word(&expansion, self.grown.entry(sibling).or_default());
    }

    /// #146 THE EMERGENT ENTITY-ATLAS — this civilization's collective cognition,
    /// measured live through the manifold. VESSELS (minds) are houses; a class held
    /// by ≥ 2 vessels is a shared idea — a VALUE-PRESERVING overlap (crystals
    /// compared by their full canonical class, never a lossy digest). The
    /// connection is sampled PROBABILISTICALLY (seeded by civ+beat+class, replay-
    /// exact — `instinct::choose` in the small, weighted by the civ's own cohesion
    /// coherence), so collective cognition EMERGES from which shared ideas transport
    /// this beat rather than any dialed knob. ΔH is the exact `reduct::atlas`
    /// measure (the Ignition Law, dh = flat − 1). Returns
    /// `(vessels, contacts, flat, ΔH)`. OBSERVATION ONLY — no committed state, no
    /// receipt, no replay effect.
    fn cognition_atlas(&self, coherence_permille: u64, beat: u64) -> (usize, usize, usize, usize) {
        let houses: Vec<std::collections::BTreeSet<String>> = self
            .grown
            .values()
            .map(|cs| cs.iter().map(atlas_crystal_key).collect())
            .filter(|s: &std::collections::BTreeSet<String>| !s.is_empty())
            .collect();
        let vessels = houses.len();
        if vessels < 2 {
            return (vessels, 0, 0, 0);
        }
        // Emergent overlap: the value-preserving classes held by ≥ 2 vessels.
        let mut counts: std::collections::BTreeMap<&String, usize> =
            std::collections::BTreeMap::new();
        for h in &houses {
            for k in h {
                *counts.entry(k).or_default() += 1;
            }
        }
        let contacts: Vec<&String> = counts
            .into_iter()
            .filter(|(_, c)| *c >= 2)
            .map(|(k, _)| k)
            .collect();
        let n_contacts = contacts.len();
        // Probabilistic connection: each shared class transports flatly this beat
        // with the civ's coherence (seeded by civ+beat+the class's own value).
        let civ_seed = atlas_seed(self.spec.civilization.sigil.as_bytes());
        let flat = contacts
            .iter()
            .filter(|k| {
                atlas_draw(civ_seed ^ (beat << 20) ^ atlas_seed(k.as_bytes())) < coherence_permille
            })
            .count();
        let dh = if flat == 0 {
            0
        } else {
            reduct::atlas::delta_h(
                &atlas_point(),
                &atlas_point(),
                &atlas_contacts(flat),
                &atlas_restr(flat),
                &atlas_restr(flat),
            )
        };
        (vessels, n_contacts, flat, dh)
    }

    /// MANIFOLD-1a: the world's own reduce-organ occupancy this beat,
    /// encoded raw (`row: i64 LE ‖ col: i64 LE ‖ index: u8`, entries in
    /// occupancy order) — never re-derived by a receipt reader, never the
    /// `reduct::GaugeDirection` type itself (`cfe` does not depend on
    /// `reduct`; this crate does, so the encoding lives here). Withheld
    /// (chronicle's gauge cache stays whatever it already was — empty
    /// before any beat has one) for a line with nothing material to read
    /// yet, per this codebase's honest-withhold law.
    fn gauge_step(&mut self) {
        let (Some(orb), Some(class)) = (self.world.orb(), self.world.star()) else {
            return;
        };
        let occupancy = soc::gauge_occupancy(&orb.elements, class);
        let mut sample = Vec::with_capacity(occupancy.len() * 17);
        for (site, direction) in &occupancy {
            sample.extend_from_slice(&site.row.to_le_bytes());
            sample.extend_from_slice(&site.col.to_le_bytes());
            sample.push(direction.index);
        }
        self.world.observe_gauge(sample);
    }

    fn research_step(&mut self, book: &Ledger, survives: bool) {
        let Some(receipt) = self.world.chronicle().receipts().last() else {
            return;
        };
        let digest = receipt.own().0;
        // CAPTURED BEFORE THE BEAT MOVES ANYTHING. Promotion reads both,
        // and reading either afterwards would be vacuous: a close always
        // advances the trajectory to Growth.
        let trajectory_before = self.trajectory;

        // COG-DIRECTION (Slices 1+2) — THE CIVILIZATION READS ITSELF AND THAT
        // READING IS ITS DIRECTION. The SAME `reduct::peel` organ the grid seat runs,
        // turned on this line's own material self: element holdings projected into the
        // five almanac families (the ONT lens, past iron+biomass), combined with its
        // thermodynamic books (`cohesion_ratio`), to DERIVE a direction — no
        // `if biomass then food`. Slice 2: this derived read now DRIVES the trajectory
        // (the `direction_override` below), retiring the boolean `advance(closed)` FSM
        // for any line with a material self; the FSM survives only as the fallback for
        // a pre-material line (no star, no elements — nothing yet to read). Emitted to
        // STDERR to be witnessed live on the network, not in a test.
        //
        // The mapping is derived from the organ's own readings, not a puzzle boolean:
        //   ordering (captures order AND ≥2 families)     -> Growth   (composing itself)
        //   dissipating but ≥2 families (venting order)   -> Stagnation (necessity opens the reach)
        //   collapsed to <2 families (a hole)             -> Decline  (material collapse)
        // Read the material self as OWNED values so the borrow of `self.world` ends
        // before we mutate `self.prev_order` below.
        let self_now: Option<(soc::DirectionRead, cfe::ratio::Ratio, cfe::ratio::Ratio)> =
            match (self.world.orb(), self.world.star()) {
                (Some(orb), Some(class)) => soc::observe_direction(
                    &orb.elements,
                    class,
                    &orb.syntropy_contained,
                    &orb.entropy_vented,
                )
                .map(|read| {
                    (
                        read,
                        orb.syntropy_contained.clone(),
                        orb.entropy_vented.clone(),
                    )
                }),
                _ => None,
            };
        let direction_override: Option<Trajectory> = if let Some((read, syn, ent)) = self_now {
            // WITHHOLD LAW (operator ruling 2026-08-21): a direction verdict requires a
            // SIGNAL. The absence of a reading HOLDS the prior trajectory — it is never
            // mapped to a negative verdict (the honest-withhold principle enforced
            // everywhere in this project). Two consequences:
            //
            // (a) The DRIVER is the lifetime LEVEL read, not the 1-beat FLOW. Measured:
            //     the flow reads ~72% zeros because the entropy/syntropy books are
            //     CUMULATIVE, so a single beat's delta is usually 0. Driving on a
            //     72%-zero instrument maps "unmeasured" to a verdict for most lines —
            //     the withhold sin through a coarse thermometer. The level drives until
            //     Slice 3a's WINDOWED flow is PROVEN to discriminate (flat-fraction well
            //     below 72%); the flow is emitted for witness only, and flow=0 is a
            //     HOLD, never a verdict.
            // (b) Cold books (no thermodynamic reading yet) HOLD the prior trajectory.
            //
            // `families < 2` is a MEASURED collapse (the peel sees the mono-family hole),
            // a real signal, so it verdicts Decline even under the withhold law.
            //
            // SLICE 3b — POPULATION-RELATIVE SELECTION. Fitness is a line's cohesion
            // RELATIVE TO THE LIVE POPULATION (the tertile band set by
            // `direction_census_step`), not an absolute 1/2 dial: the top third GROWS
            // (fittest — its mind climbs the difficulty ladder), the bottom third
            // DECLINES (selected against), the middle STAGNATES. This is evolution over
            // the syntropic value — the same reduce organ reading the civilization's
            // place in its cohort. The absolute 1/2 threshold is the FALLBACK for a
            // sparse cohort (< 3 warm lines, no band). Withhold law intact: cold books
            // (cohesion None) hold the prior trajectory, never a verdict from absence.
            let derived_solo = if read.families < 2 {
                Trajectory::Decline
            } else {
                match (read.cohesion.as_ref(), self.pop_cohesion_band.as_ref()) {
                    (None, _) => trajectory_before, // cold books — HOLD prior
                    (Some(c), Some((lo, hi))) => {
                        if *c >= *hi {
                            Trajectory::Growth // top third of the living cohort
                        } else if *c < *lo {
                            Trajectory::Decline // bottom third — selection presses
                        } else {
                            Trajectory::Stagnation // the middle band holds station
                        }
                    }
                    // Sparse cohort (no population band): the absolute break-even holds.
                    (Some(c), None) if *c > cfe::ratio::ratio(1, 2) => Trajectory::Growth,
                    (Some(_), None) => Trajectory::Stagnation,
                }
            };
            // #146 MANIFOLD-5 FOLLOW-UP — CONNECTIVITY RELIEVES THE TRAP (doc 55 §9's
            // own design consequence: "raise CONNECTIVITY... NOT the private minds —
            // which is what the instinct::choose connection sampler + 3b selection
            // already push toward"). dH>0 is not a guess at fitness — it is the
            // Mayer-Vietoris connecting-map rank PROVING a genuinely shared class
            // exists this beat across ≥2 of this civ's own vessels (the Ignition Law's
            // ΔH≈connectivity−1: a mind alone or merely adjacent never sees dH>0). A
            // proven collective idea is a strictly positive signal that must never
            // DOWNGRADE the cohesion-tertile verdict, only promote it one tier
            // (Decline→Stagnation→Growth) — cohesion stays the primary read; the
            // atlas is the tie-break connectivity already earns, not a second dial.
            let coherence = read.cohesion.as_ref().map(atlas_permille).unwrap_or(0);
            let (vessels, contacts, flat, dh) = self.cognition_atlas(coherence, self.world.now());
            let derived = if dh > 0 {
                match derived_solo {
                    Trajectory::Decline => Trajectory::Stagnation,
                    Trajectory::Stagnation => Trajectory::Growth,
                    Trajectory::Growth => Trajectory::Growth,
                }
            } else {
                derived_solo
            };
            // The per-beat FLOW — WITNESSED ONLY (Slice 3a's input; drives nothing yet).
            // A zero flow is NO SIGNAL (a hold), printed `0` for the discrimination
            // measurement, never a verdict here.
            let zero = cfe::ratio::ratio(0, 1);
            let flow = match self.prev_order.as_ref() {
                Some((psyn, pent, _)) => {
                    let d = (syn.clone() - psyn.clone()) - (ent.clone() - pent.clone());
                    if d > zero {
                        "+"
                    } else if d < zero {
                        "-"
                    } else {
                        "0"
                    }
                }
                None => "?",
            };
            let cohesion = read
                .cohesion
                .as_ref()
                .map(|c| format!("{}/{}", c.numer(), c.denom()))
                .unwrap_or_else(|| "cold".to_string());
            let band = self
                .pop_cohesion_band
                .as_ref()
                .map(|(lo, hi)| {
                    format!(
                        "[{}/{}..{}/{}]",
                        lo.numer(),
                        lo.denom(),
                        hi.numer(),
                        hi.denom()
                    )
                })
                .unwrap_or_else(|| "solo".to_string());
            eprintln!(
                "COG-DIR clock {} {}: elems={} families={}/5 cohesion={} pop-band={} flow={} -> drives={} (relative; was FSM={})",
                self.world.now(),
                self.spec.civilization.sigil,
                read.elements,
                read.families,
                cohesion,
                band,
                flow,
                derived.describe(),
                trajectory_before.describe(),
            );
            // COG-ATLAS — the emergent, value-preserving entity-atlas cognition:
            // this civilization's collective ΔH over its own minds, the connection
            // sampled probabilistically at its cohesion coherence (computed above,
            // where dH already fed the Slice 3b trajectory promotion — not
            // observation-only anymore, see the comment there).
            eprintln!(
                "COG-ATLAS clock {} {}: vessels={} contacts={} flat={} dH={} coherence={}permille (emergent)",
                self.world.now(),
                self.spec.civilization.sigil,
                vessels,
                contacts,
                flat,
                dh,
                coherence,
            );
            // COG-LEX — the emergent VOCABULARY made visible: shared words this
            // civilization has minted (two vantages agreed and the word landed) and
            // the agreements still in flight (one vantage waiting on the pending
            // board for a second). A growing lexicon IS communication — a society
            // composing a shared language. Observation only; the mint happens on the
            // research beat, this only reports the standing count.
            // AUTIC-1 — alongside the civ's PRIVATE lexicon, the size of the
            // SHARED AUTIC language net has learned from the mycelium network
            // (net_basin). words=/pending= is the parochial dialect; autic= is the
            // shared tongue the entities will come to think in. Watching the two
            // together is the emergent measure of the recursion closing.
            eprintln!(
                "COG-LEX clock {} {}: words={} pending={} autic={} absorbed={} practiced={} spoke={} learned={} authored={} worth={} (private | shared | curriculum | sentences | authorship | measured-utility)",
                self.world.now(),
                self.spec.civilization.sigil,
                self.library.words().len(),
                self.word_pending.len(),
                scribe::membrane::autic_basin_depth(),
                self.autic_absorbed,
                self.autic_practiced,
                self.autic_spoke,
                self.autic_learned,
                self.autic_authored,
                scribe::membrane::worth_loaded(),
            );
            self.prev_order = Some((syn, ent, read.families));
            Some(derived)
        } else {
            None
        };

        // N-I5 — THE SURVIVAL OBSERVATION (Pin G, THE INTELLIGENCE PORT). Peel
        // the ration book: does the larder feed the working mouths? A line that
        // cannot does NOT think ARC this beat — it rests (the `None` arm below,
        // exactly as a barren beat rests; not a sweep). The owe column is
        // labor_force × the line's ration (mouths eat — buildings' upkeep is not
        // the pair), the have is the derived sustenance, and the owed DIRECTION
        // is the species's ration (a plasmoid owes joule, not biomass — no
        // hardcode). The SAME reduce organ the grid seat runs. `survives` was
        // read in `cycle` after regrowth and before collect drained the larder.
        let solved = if !survives {
            // The book did not close — withhold the beat, call no think_task.
            None
        } else if self.spec.civilization.perception == "volumetric" {
            // VOLUMETRIC-1d — THE VOLUMETRIC SEAT, WIRED. `think_volume_task`
            // is the SAME reduce-organ discipline as `think_task` (peel, glue
            // through `reduct::think`, render, seal — no search; `solve_volume`
            // stays the retired prover-side sweep, never called live). The
            // SAME N-I7 leisure gate applies — the look's price is a function
            // of "one degree-1 observation" (`Self::observe_price()`), not of
            // the seat's geometry, so a volumetric line pays exactly what a
            // planar one does for the same look.
            //
            // MINT/WORD-MINT DEFERRED, deliberately, not by oversight: the wire
            // machinery for a volumetric PoUW deed already exists whole
            // (`WorkClass::Volumetric(VolDifficulty)`, `encode_volume_work` —
            // `mint_discovery_as` takes raw bytes, so it is reachable), but the
            // word-mint/vocabulary system (`solver::mint::Library`, `consider`,
            // `sweep_vocabulary`) is typed to the planar `Program`/`Op`
            // exclusively — extending it to `VProgram` is its own typed-system
            // landing. This wiring's own measured goal does not need it: `think`
            // grows the basin on ANY wall (Pin B, N-I2), Solved or not — so
            // `self.grown` (the vessel cognition COG-ATLAS/ΔH reads) fills for a
            // volumetric line exactly the way it does for a planar one, which is
            // the actual thing #173 exists to unblock.
            //
            // A CONFIRMED VOLUMETRIC CLOSE COUNTS (G2, operator 2026-10-07: "only
            // give them options, freedom, and agency"). The thought used to be
            // discarded here, so a volumetric line could never close, climb or
            // cleave — a dead end written into the dispatch, not an outcome. A
            // `Solved` whose certificate `think_volume_task` already verified
            // (`verify_volume_certificate`, Pin A on every train pair) and whose
            // test output matches now closes exactly as a planar solve does: the
            // solve is recorded, counted, and the line climbs on mastery. The
            // MINT stays deferred as documented above (no `WorkProof` — the
            // proof slot is `None`), and the membrane's planar pairs are empty.
            let orb = cfe::world::FIRST_ORB;
            let price = Self::observe_price();
            if self.world.leisure_bank_of(orb) >= price {
                self.world.settle_leisure(orb, -price);
                let vessel = self.automaton;
                self.absorb_autic(vessel);
                self.practise_autic_wall(vessel);
                let vdifficulty = VolDifficulty::level(self.level);
                let vtask = forge_volume(digest, vdifficulty);
                match think_volume_task(&vtask, self.grown.entry(vessel).or_default()) {
                    solver::VolumeSolveRead::Solved { program, matches_expected: Some(true), certificate, .. } => {
                        let word: Vec<String> = program.iter().map(|op| op.describe()).collect();
                        Some((vtask.name.clone(), word.join(" → "), hex(&certificate.output), None, Vec::new()))
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else {
            let difficulty = Difficulty::level(self.level);
            let task = forge_task(digest, difficulty);
            // NOVELTY-GATE (38 §4): the library folds once per process
            // from this line's own receipts — the chronicle is the
            // register, this cache only spares the re-read.
            if !self.library_folded {
                for (orbit, bytes, cited) in cfe::vocabulary::solver_words(self.world.chronicle()) {
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
                let certificate = solver_verify::SolveCertificate::seal(&task, &expansion, &output);
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
            // THE PRODUCER IS THE ONE REDUCE ORGAN (N-I4, THE INTELLIGENCE
            // PORT). On a pre-reach MISS the line THINKS: `think_task` peels the
            // forged task's train pairs and GLUES a frame, or WITHHOLDS. Search
            // has LEFT the live beat — `propose_consulted_measured`, `solve`,
            // `sweep_vocabulary` are no longer in this stack (search.rs survives
            // only as a novelty instrument on the mint path below, never as how
            // this mind solves the K-step). A withhold is REST: the `None` arm
            // declines the trajectory exactly as a barren beat always did. It is
            // NOT a sweep — `swept` does not move on a think withhold (that
            // cokernel-failure count was the search's, and the search is gone).
            // The basin persists across beats: a wall `think_task` mints and
            // adopts this beat DRAINS the next (Pin B), so the thinker is
            // changed by what it thought.
            let read = match pre_reached {
                // `pre_reach` is a REDUCE against crystallized words — a library
                // hit, not a fresh look. It stays UNCHARGED (N-I7).
                Some(hit) => hit,
                None => {
                    // N-I7 — THE LOOK COSTS LEISURE. The line is Fed here (Pin G
                    // passed above, else `solved` is already None). The fresh
                    // peel-and-reduce is the priced act: pay the observe price
                    // FIRST, then drain. An unaffordable look is FASTING — rest
                    // (this None arm), not a sweep; the bank refills from surplus
                    // (`cognition_step`) and a later beat can pay.
                    let orb = cfe::world::FIRST_ORB;
                    let price = Self::observe_price();
                    if self.world.leisure_bank_of(orb) >= price {
                        self.world.settle_leisure(orb, -price);
                        // N-I·VESSEL — drain THIS vessel's mouth, not the line pool.
                        let vessel = self.automaton;
                        self.absorb_autic(vessel); // think in the learned AUTIC tongue
                        self.practise_autic_wall(vessel); // practise the mesh's own problems
                        solver::think_task(&task, self.grown.entry(vessel).or_default())
                    } else {
                        SolveRead::Unsolved // the bank cannot pay the look — fast this beat
                    }
                }
            };
            let auto = match read {
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
                        // N-I8 — a confirmed composite is a WORD candidate; the
                        // AUTOMATON deposits its closure. The word mints only on
                        // CONSILIENCE (a second vessel independently closes the
                        // same orbit) — `deposit_toward_word` gates that and bills
                        // the mint (`mint_cost`, COG-2/4/5, unchanged).
                        let standing =
                            solver::mint::standing_orbits(&solver::mint::sweep_vocabulary(&task));
                        if let Some(orbit) =
                            solver::mint::consider(&program, &self.library, &standing)
                        {
                            self.deposit_toward_word(orbit, &program, self.automaton, digest);
                        }
                    }
                    let word: Vec<String> = program.iter().map(|op| op.describe()).collect();
                    let cert_hex = hex(&certificate.output);
                    // Law II in full: the pairs the solve was PROVED
                    // on cross the membrane whole — the train family
                    // plus the confirmed test — so the mesh judges by
                    // replay, not by a synthesized stand-in. Cloned
                    // here because the proof takes `test_output` next.
                    let rows = |g: &solver_verify::Grid| -> Vec<Vec<u8>> {
                        g.cells.chunks(g.cols).map(|r| r.to_vec()).collect()
                    };
                    let mut proved: Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)> = task
                        .train
                        .iter()
                        .map(|p| (rows(&p.input), rows(&p.output)))
                        .collect();
                    proved.push((rows(&task.test_in), rows(&test_output)));
                    // R4: if this program changes frame, also offer its
                    // same-frame sub-transforms so the mesh founds the base
                    // ops the reshape hides (the tiling itself stays a known
                    // operator, never founded).
                    let mut sf_inputs: Vec<solver_verify::Grid> =
                        task.train.iter().map(|p| p.input.clone()).collect();
                    sf_inputs.push(task.test_in.clone());
                    same_frame_founding_offers(&task.name, &program, &sf_inputs);
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
                        task.name.clone(),
                        word.join(" → "),
                        cert_hex,
                        solver_verify::encode_work(&proof).map(|bytes| (bytes, difficulty)),
                        proved,
                    ))
                }
                _ => None,
            };
            // N-I8 — the SIBLING office's independent vantage: its OWN basin
            // (N-I·VESSEL), its OWN look (N-I7), deposit-only (no value mint —
            // the automaton is the line's value producer). Affordability-
            // staggered: it looks only if the bank can still pay AFTER the
            // automaton, so a line with one look's surplus funds the automaton
            // first and the sibling fasts. When both close the same orbit, the
            // second deposit reaches two distinct vessels and the word mints.
            //
            // ENTITY-ALIGN-2 (docs/56 §4, piece a) — a MINORITY of beats,
            // hash-seeded per (sibling, beat) so it is emergent rather than a
            // synchronized global tick, the sibling instead CHOOSES its own
            // problem (`sibling_self_select`) — the first entity-initiated
            // agency in this civ's cognition, rather than dispatch deciding
            // "we think about X now" for both vessels. It falls back to
            // confirming when self-select declines (no basin yet, or cannot
            // afford even the discounted look), so this is additive: no
            // beat that used to confirm loses its confirmation entirely.
            let self_select_beat = sibling_self_select_beat(
                self.sibling,
                self.world.now(),
                SIBLING_AUTONOMY_STRIDE,
            );
            if !self_select_beat || !self.sibling_self_select() {
                self.sibling_deposit(&task, digest);
            }
            auto
        };
        match solved {
            Some((name, word, cert_hex, proof, proved_pairs)) => {
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
                // Law II (mycelium 23_RESIDENCY): the solve offers
                // ITSELF — the proved pairs, same moment, non-blocking;
                // the resident membrane thread speaks MY-K while the
                // world beat moves on. The mesh judges by replay, so
                // every program family the forge proves is speakable.
                scribe::membrane::offer_proved(&name, &word, proved_pairs);
                self.solved += 1;
                self.streak += 1;
                // THE COUPLING CLOSES HERE. A close returns the line to
                // growth from wherever it was, and clears the hunger it
                // reached with.
                // Slice 2: the civilization's DIRECTION is its self-reading when it
                // has a material self; the boolean close only sets direction for a
                // pre-material line (the FSM fallback). The close still clears the
                // hunger below either way.
                self.trajectory =
                    direction_override.unwrap_or_else(|| self.trajectory.advance(true));
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
                let ladder_still_climbs = self.ladder_climbs(self.level);
                if mastered && ladder_still_climbs {
                    self.level += 1;
                    self.streak = 0;
                }
            }
            None => {
                self.streak = 0;
                // Growth → stagnation → decline, one step per barren
                // beat. What each state SPENDS is `Trajectory::need`.
                // Slice 2: the derived self-reading is the direction; the FSM's
                // barren-beat step only stands for a pre-material line.
                self.trajectory =
                    direction_override.unwrap_or_else(|| self.trajectory.advance(false));
                self.starved = self.starved.saturating_add(1);
            }
        }
    }

    /// DIVERGENCE (AEON-1b, 11 §3): at the top of its own ladder (rung 8
    /// today, read off the ladder's saturation) with 20 confirmed
    /// solves — sustained pressure — the line CLEAVES into two PEER
    /// branches: no parent, no child. The origin's own key moves +delta
    /// and receipts `Diverged` naming the departing branch; the branch
    /// founds its own world at the opposite phase (−delta, wrapped).
    /// The mutation stays chronicle-seeded: the line's own history
    /// decides where its branches land on the manifold. The lineage
    /// pointer records LINE membership (LINEAGE-1), never parenthood.
    /// Does THIS line's ladder still differentiate above `level`? A planar
    /// line climbs the planar forge ladder, a volumetric line the volumetric
    /// one — each read off its own saturation, never a remembered top.
    fn ladder_climbs(&self, level: u32) -> bool {
        if self.spec.civilization.perception == "volumetric" {
            VolDifficulty::level(level) != VolDifficulty::level(u32::MAX)
        } else {
            Difficulty::climbs(level)
        }
    }

    fn divergence_step(&mut self, registry: &Registry) -> Option<PolitySpec> {
        // Divergence waits for the TOP of the ladder — derived from
        // the ladder, not from a remembered 5.
        if self.cleaved || self.ladder_climbs(self.level) || self.solved < 20 {
            return None;
        }
        let receipt = self.world.chronicle().receipts().last()?;
        let beacon = Beacon::from_digest(receipt.own().0);
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
                false => others[beacon.uniform_index(domain, others.len() as u64) as usize].clone(),
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
        let stamp = receipt.stamp();
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
        let start =
            beacon.uniform_index("growth.name", (heads.len() * tails.len()) as u64) as usize;
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
        let step = |domain: &str| -> Ratio { ratio(beacon.uniform_index(domain, 9) as i64 - 4, 2) };
        let (mut dx, dy, dz) = (step("chart.dx"), step("chart.dy"), step("chart.dz"));
        if dx == ratio(0, 1) && dy == ratio(0, 1) && dz == ratio(0, 1) {
            dx = ratio(1, 1);
        }
        let spec = PolitySpec {
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
        };
        // THE SEAT, recorded once beside the card: the boot roster reads it,
        // so a restart resumes this branch in the world it was founded in,
        // at the same chart point under the same temperament — never a
        // second founding under a derived `{sigil}-hearth`. (The gauge is
        // read only at founding; the resumed chronicle carries it.)
        let seat = ledger::registry::Seat {
            world: spec.world_name.clone(),
            planet: spec.planet.clone(),
            colony: spec.colony.clone(),
            position: spec.position.clone(),
            policy: spec.policy.describe().to_string(),
        };
        if let Err(e) = registry.write_founded_at(&spec.civilization.sigil, &seat) {
            eprintln!("divergence: {} seated without a seat card ({e}); a restart would found it again", spec.civilization.sigil);
        }
        Some(spec)
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
        let beacon = Beacon::from_digest(receipt.own().0);
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
        // MANIFOLD-1a (TU-1 / G, doc 55 §1/§3): stamp THIS beat's gauge
        // sample first, before any deed below can close a receipt — every
        // receipt this beat reads the SAME snapshot (TU-1c: gauge settles
        // at the beat boundary, never mid-beat). The sample is the SAME
        // reduce-organ occupancy `soc::observe_direction` already peels
        // (the civilization's own material self, `G_cognition =
        // reduct::GaugeDirection`, doc 55 §1 — "already live"), read here
        // raw rather than re-derived, so the receipt carries the organ's
        // actual reading. A pre-material line (no orb/star yet) has
        // nothing to read — withhold, not a manufactured zero.
        profile::timed("gauge_step", || self.gauge_step());
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
        // N-I5 — THE SURVIVAL OBSERVATION (Pin G), read AFTER the beat's
        // regrowth (tick) and BEFORE collect consumes it: can the vein/flow feed
        // the working mouths this beat? Read post-collect it would see the
        // ONT-2b larder drained to ~0 (the staple is MINED vein → mouths at
        // collect, the bank bypassed) and falsely starve a flow-fed line. Pure
        // read — replay reads the same verdict, so SEAL-B is untouched.
        let survives = profile::timed("survival_read", || {
            self.ration_survives(cfe::world::FIRST_ORB)
        });
        // N-I6 — THE SURVIVAL EXPRESS: the residual names ≤1 deed, expressed in
        // this same post-tick / pre-collect window (the sole zone-raiser now).
        // A Fed book may open matter (Pin F); a Short book still thinks — that is
        // how it names the cure (Pin G withholds only ARC, below).
        profile::timed("survival_express", || {
            self.express_survival(cfe::world::FIRST_ORB, survives)
        });
        // Every settled orb collects — a chartered survey site left
        // uncollected would starve at its own upkeep.
        profile::timed("collect", || {
            let orbs: Vec<u64> = self
                .world
                .committed()
                .orbs
                .iter()
                .map(|o| o.orb_id)
                .collect();
            for orb in orbs {
                let _ = self.world.collect_as(self.automaton, orb);
            }
        });
        // COG-1: the collect settled the people's food; the surplus above their
        // survival drives banks as leisure — the Mind's budget for this beat.
        profile::timed("cognition_step", || self.cognition_step());
        profile::timed("survey_step", || self.survey_step());
        profile::timed("expand_step", || self.expand_step());
        // COG-EMERGENCE-2 THE ABUNDANCE STEP: above the survival floor, the line
        // makes ONE surplus choice — deepen, expand, synthesise, or hoard — by a
        // single fold-draw over its tensions, not three authored gates. Runs
        // BEFORE research so a cognitive seat this beat deepens this beat's mint;
        // the chosen act pays its own bill or is refused by physics AFTER the
        // choice (the will stays the line's own). Replaces cognitive_investment_
        // step + expand_step's charter + chemistry_step's seat.
        profile::timed("abundance_step", || self.abundance_step());
        profile::timed("research_step", || self.research_step(book, survives));
        let newborn = profile::timed("divergence_step", || self.divergence_step(registry));
        // THE RESUME CARRIER (task 25, 2026-07-30): every polity
        // world syncs its tlv chronicle beside the ledger exports — an
        // append-only resume copy (cfe::persist grows the file, never
        // rewrites). The player's OBSERVING hand can resume a copy;
        // the WRITING hand stays refused at the CLI (two writers on
        // one clock is the split-brain NS-5 exists to arbitrate) —
        // intervention still goes through the steward inbox.
        // #157 — `sync_from` skips re-reading/re-verifying the prefix
        // this process already proved in an earlier beat (`sync_cursor`),
        // reading only the unwritten tail. A cold cursor (first beat)
        // pays the same full check `sync` always did.
        let synced_clean = profile::timed("persist_sync", || {
            cfe::persist::sync_from(
                self.world.chronicle(),
                &book.world_dir(&self.spec.world_name).join("chronicle.tlv"),
                &mut self.sync_cursor,
            )
            .is_ok()
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
            // #157 — a clean `sync_from` this SAME beat already proved
            // chronicle.tlv consistent; `record`'s own divergence check
            // (`rotate_divergent`) would re-read and re-parse the whole
            // file again to answer a question just answered. Skip it
            // only then — `record` stays the correct door whenever the
            // sync didn't run clean (a fresh/foreign world, or a
            // refused sync this beat).
            let _ = if synced_clean {
                scribe::record_synced(
                    book,
                    &self.spec.world_name,
                    &self.spec.planet,
                    &self.spec.civilization.sigil,
                    &self.world,
                )
            } else {
                scribe::record(
                    book,
                    &self.spec.world_name,
                    &self.spec.planet,
                    &self.spec.civilization.sigil,
                    &self.world,
                )
            };
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
        // Borrowed, not cloned: the whole history used to be copied here
        // EVERY beat (O(history) per beat per world, growing without bound);
        // only the unjudged tail is ever walked below.
        let receipts: &[cfe::vrp::Receipt] = self.world.chronicle().receipts();
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
                sub: 0,
            };
            arbor.adopt(coordinate, &bytes)?;
            self.chained = receipts.len();
            self.arbor_committed = receipts.len();
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
                        if let Some(at) = receipts.iter().position(|r| r.sequence() == sequence) {
                            let prior = cfe::world::replay(
                                &cfe::vrp::Chronicle::from_receipts(receipts[..at].to_vec())
                                    .map_err(|_| mesh::arbor::ArborRefused::StateUndecodable)?,
                            );
                            let next = cfe::world::replay(
                                &cfe::vrp::Chronicle::from_receipts(receipts[..=at].to_vec())
                                    .map_err(|_| mesh::arbor::ArborRefused::StateUndecodable)?,
                            );
                            let receipt = &receipts[at];
                            let step = kernel::Step {
                                event: receipt.event().clone(),
                                actor: receipt
                                    .provenance()
                                    .first()
                                    .copied()
                                    .unwrap_or(cfe::world::SOVEREIGN),
                                stamp: receipt.stamp(),
                            };
                            if let Err(unlawful) = kernel::admits(&prior, &next, &step) {
                                self.refused.push((receipt.sequence(), unlawful));
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
            let verified = cfe::vrp::Chronicle::from_receipts(receipts[..self.chained].to_vec())
                .map_err(|_| mesh::arbor::ArborRefused::StateUndecodable)?;
            for receipt in verified.receipts() {
                stepper.apply(receipt);
            }
            self.stepper = Some(stepper);
        }
        let stepper = self.stepper.as_mut().expect("seeded above");
        // **THE PHASE-3 FLIP** (`39_PHASE3_FLIP` §5, operator ruling
        // 2026-08-13): one arbor commitment PER DEED, each carrying its
        // own step, so `mesh::arbor::extend` gates it live — the sub-
        // coordinate ruling (§3, option A) is what makes this possible;
        // several deeds sharing one tick address distinctly by `sub`.
        //
        // `still_admitting` tracks whether every deed so far THIS CALL
        // has committed to the arbor. Judgment (for the refusal report,
        // `self.refused`) runs over every new deed unconditionally, same
        // as before — but the arbor only ever extends along its own
        // head, so the first refusal stops arbor commits for the rest
        // of this call. Later deeds stay judged-and-reported but
        // uncommitted; `self.arbor_committed` simply does not advance
        // past them, so the NEXT call retries from the same point. This
        // is the live meaning of "replay demoted to a proposer": the
        // world keeps playing regardless, but the arbor — the live gate
        // — only ever holds what the kernel actually admitted.
        let mut still_admitting = true;
        for receipt in &receipts[self.chained..] {
            let step = kernel::Step {
                event: receipt.event().clone(),
                actor: receipt
                    .provenance()
                    .first()
                    .copied()
                    .unwrap_or(cfe::world::SOVEREIGN),
                stamp: receipt.stamp(),
            };
            let prior = stepper.state().clone();
            stepper.apply(receipt);
            let next = stepper.state();
            match kernel::admits(&prior, next, &step) {
                Ok(()) => {
                    if still_admitting {
                        let sub = match arbor.head("") {
                            Some(head) if head.coordinate.t == receipt.stamp() => {
                                head.coordinate.sub + 1
                            }
                            _ => 0,
                        };
                        let coordinate = mesh::arbor::Coordinate {
                            branch: String::new(),
                            s: seat.clone(),
                            g: self.world.gauge().clone(),
                            t: receipt.stamp(),
                            sub,
                        };
                        match arbor.extend(coordinate, &codec::state_bytes(next), Some(&step)) {
                            Ok(_) => self.arbor_committed += 1,
                            // The judge admitted it live but the arbor's
                            // own re-derivation (from ITS head, which may
                            // sit behind `next` if an earlier deed this
                            // call never reached its extend()) disagrees
                            // — stop committing, keep judging/reporting.
                            Err(_) => still_admitting = false,
                        }
                    }
                }
                Err(unlawful) => {
                    self.refused.push((receipt.sequence(), unlawful));
                    still_admitting = false;
                }
            }
        }
        self.chained = receipts.len();
        write_judged_sidecar(
            &book.world_dir(&self.spec.world_name),
            self.chained,
            &self.refused,
        );
        Ok(())
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
        if d < ratio(0, 1) {
            -d
        } else {
            d
        }
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
    // #136 MEMOIZATION RIDER — the exporter's TOP element is a per-WORLD reading
    // (max over its element ledger, up to ~11.5k kinds). Reading it per (a, b)
    // pair was the O(kinds·pairs) SWEEP that dominated the beat once the melt
    // grain landed. Peel it ONCE per world here and refresh only the two a clear
    // moves — same element, same tie-break (`max_by` keeps the last of equals),
    // same choices — exactly the bargain the mass cache above already strikes.
    let top_of = |p: &Polity| -> Option<([u8; 32], Ratio)> {
        p.world.orb().and_then(|o| {
            o.elements
                .iter()
                .max_by(|x, y| x.1.cmp(&y.1))
                .map(|(id, amt)| (*id, amt.clone()))
        })
    };
    let mut top: Vec<Option<([u8; 32], Ratio)>> = worlds.iter().map(top_of).collect();
    for a in 0..n {
        for b in (a + 1)..n {
            if worlds[a].penned || worlds[b].penned {
                continue;
            }
            let distance = spatial_l1(&worlds[a].spec.position, &worlds[b].spec.position);
            let melt = cfe::world::route_melt(&worlds[a].world, &worlds[b].world, &distance);
            if melt >= ratio(1, 1) {
                continue; // walled direct — merchant territory, not element haul
            }
            // Prefer shipping from the side with more total element mass.
            let (exporter, importer) = if masses[a] >= masses[b] {
                (a, b)
            } else {
                (b, a)
            };
            let Some((element, have)) = top[exporter].clone() else {
                continue;
            };
            // Ship half (derive 2 as 1+1 — no tuned dial).
            // #136 — the dispatched amount lands on the 1/64 SMELT grain too, so
            // the sender's stock stays `denom | 64` after the debit (the giver
            // keeps the sub-grain remainder, un-shipped). A ship below 1/64 smelts to
            // zero and the guards below skip it — a fill smelted to zero refuses.
            let ship = cfe::world::smelt(&(have.clone() / (ratio(1, 1) + ratio(1, 1))));
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
                cleared.push((ex.spec.world_name.clone(), im.spec.world_name.clone()));
                // The clear moved matter: refresh exactly the two
                // touched readings (the cache's whole bargain).
                masses[exporter] = mass_of(&worlds[exporter]);
                masses[importer] = mass_of(&worlds[importer]);
                top[exporter] = top_of(&worlds[exporter]);
                top[importer] = top_of(&worlds[importer]);
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
                    let _ = worlds[i]
                        .world
                        .grant_tech_as(steward, cfe::world::FIRST_ORB, 0);
                }
            }
            let distance = spatial_l1(&worlds[a].spec.position, &worlds[b].spec.position);
            let melt = cfe::world::route_melt(&worlds[a].world, &worlds[b].world, &distance);
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
            clears.push((buy.spec.world_name.clone(), sell.spec.world_name.clone()));
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
                let Some(ask_a) = worlds[a].world.offers().first().cloned() else {
                    continue;
                };
                let Some(ask_b) = worlds[b].world.offers().first().cloned() else {
                    continue;
                };
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
                            let d = spatial_l1(&worlds[x].spec.position, &worlds[y].spec.position);
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
                let Some(want_a) = ask_a.want.as_resource() else {
                    continue;
                };
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
                    .post_offer(
                        cfe::world::FIRST_ORB,
                        pair,
                        give_amount,
                        want_amount,
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
                        &mut merchant.world,
                        counter,
                        &mut seller.world,
                        ask_a.offer,
                        deal,
                        d_am,
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
                    .post_offer(
                        cfe::world::FIRST_ORB,
                        pair,
                        give_amount,
                        want_amount,
                        counter,
                    )
                    .is_err()
                {
                    continue;
                }
                let sold = {
                    let (merchant, buyer) = two_mut(worlds, m, b);
                    cfe::world::clear_trade(
                        &mut merchant.world,
                        counter,
                        &mut buyer.world,
                        ask_b.offer,
                        deal + 2,
                        d_mb,
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

/// **CROSS-LINE CONSILIENCE** (38 §5, the mesh half of the intelligence
/// arc) — the board's once-per-beat convergence pass.
///
/// Each line resolves its OWN knowledge into a crystallized invariant
/// (`solver::knowledge_map`, the same resolve the reach consults). When
/// INDEPENDENT lines land on the SAME invariant, that agreement is a
/// shared axiom no single line owns — and each converged line records its
/// OWN leg on its OWN chain (`crystallize_knowledge_as`), one leg per
/// book, the way a lane spans two worlds without a second copy of one
/// truth. It is idempotent by construction: a line already carrying a
/// signature's axiom refuses, so a standing consilience records once and
/// then rests.
///
/// Libraryless lines are skipped — a line that has minted no vocabulary
/// has resolved no knowledge to share — so the pass costs nothing until
/// lines begin to learn. Returns the crystallizations struck THIS beat,
/// each `(world, mass, hardness)`.
pub fn consilience_step(worlds: &mut [Polity], _beat: u64) -> Vec<(String, u32, u32)> {
    // Each learning line's crystallized knowledge, keyed by board seat —
    // resolved again only for a line whose library grew since its last map.
    for polity in worlds.iter_mut() {
        let words = polity.library.words().len();
        if words > 0 && polity.knowledge.as_ref().map(|(n, _)| *n) != Some(words) {
            polity.knowledge = Some((words, solver::knowledge_map(&polity.library)));
        }
    }
    let consiliences = {
        let refs: Vec<(u64, &solver::SelfMap)> = worlds
            .iter()
            .enumerate()
            .filter(|(_, polity)| !polity.library.words().is_empty())
            .filter_map(|(seat, polity)| polity.knowledge.as_ref().map(|(_, map)| (seat as u64, map)))
            .collect();
        if refs.len() < 2 {
            return Vec::new(); // one learning line is no agreement
        }
        solver::consilience(&refs)
    };

    let mut recorded = Vec::new();
    for consilience in consiliences {
        for seat in &consilience.lines {
            let polity = &mut worlds[*seat as usize];
            if polity
                .world
                .crystallize_knowledge_as(
                    polity.automaton,
                    consilience.signature,
                    consilience.mass as u32,
                    consilience.hardness as u32,
                )
                .is_ok()
            {
                // **L4: CONSILIENCE PAYS.** Reaching a shared truth IS a
                // discovery, so it relieves the line's novelty hunger and
                // lifts its trajectory to growth exactly as a solve does —
                // the collective payoff spent through the SAME trajectory
                // that drives every line's economy, no new matter minted,
                // no dial. It is gated on HARDNESS ≥ 2 (the truth was
                // bridged by genuinely UNLIKE lines): a hard-won shared
                // insight prospers the minds that reached it; a trivial
                // agreement between near-twins is recorded but earns
                // nothing. Idempotent by the crystallize door above, so
                // one convergence pays once.
                if consilience.hardness >= 2 {
                    polity.trajectory = polity.trajectory.advance(true);
                    polity.starved = 0;
                }
                recorded.push((
                    polity.spec.world_name.clone(),
                    consilience.mass as u32,
                    consilience.hardness as u32,
                ));
            }
        }
    }
    recorded
}

/// **THE KNOWLEDGE MARKET** (INTELLIGENCE ARC C — trade minds, not just
/// matter) — the board's once-per-beat circulation of vocabulary.
///
/// Every word any line has minted is a listing in the roster's pool. A
/// line that LACKS a word may LICENSE it into its own library — knowledge
/// spreads from the minds that forged it to the minds that can use it,
/// the way a design or a business idea spreads once it exists. This FIRST
/// cut prices the listing by its DEPTH (the length of the program it
/// spells — a longer, harder-won word costs more) and gates the buy on
/// the line's own PROSPERITY: it licenses only a word it can afford
/// against its banked Crystal. So a rich, developed civilization acquires
/// the ideas it lacks; a poor one cannot yet reach the deep ones. One
/// license per line per beat, the cheapest it can afford — the market
/// clears gradually, not in a flood.
///
/// Banked-neutral: the word is identity, and this cut does not yet MOVE
/// the payment to the seller (the conserved buyer→seller settlement is
/// the two-sided follow-up). The prosperity gate is the throttle.
pub fn knowledge_market_step(worlds: &mut [Polity], _beat: u64) -> Vec<(String, usize)> {
    // The roster's pool of words, deduped by orbit.
    let mut pool: Vec<([u8; 32], Vec<u8>, Vec<[u8; 32]>)> = Vec::new();
    let mut seen: std::collections::HashSet<[u8; 32]> = std::collections::HashSet::new();
    for polity in worlds.iter() {
        for (orbit, expansion, cited) in cfe::vocabulary::solver_words(polity.world.chronicle()) {
            if seen.insert(orbit) {
                pool.push((orbit, expansion, cited));
            }
        }
    }
    if pool.is_empty() {
        return Vec::new(); // nothing minted yet — no market
    }
    // The price of a listing: the DEPTH of the word it spells (program
    // length), derived from the word itself, never a dial.
    let price_of = |expansion: &[u8]| -> usize {
        solver_verify::program_from_bytes(expansion)
            .map(|program| program.len())
            .unwrap_or(0)
            .max(1)
    };
    let mut licensed = Vec::new();
    for buyer in 0..worlds.len() {
        let own: std::collections::HashSet<[u8; 32]> =
            cfe::vocabulary::solver_words(worlds[buyer].world.chronicle())
                .into_iter()
                .map(|(orbit, _, _)| orbit)
                .collect();
        let crystal = worlds[buyer]
            .world
            .orb()
            .map(|orb| orb.banked_amount(cfe::dop::Resource::Crystal))
            .unwrap_or_else(|| ratio(0, 1));
        // The cheapest lacked word this line can afford — its first
        // reachable idea.
        let pick = pool
            .iter()
            .filter(|(orbit, _, _)| !own.contains(orbit))
            .map(|(orbit, expansion, cited)| (price_of(expansion), orbit, expansion, cited))
            .filter(|(price, _, _, _)| crystal >= ratio(*price as i64, 1))
            .min_by_key(|(price, _, _, _)| *price);
        if let Some((price, orbit, expansion, cited)) = pick {
            if worlds[buyer]
                .world
                .license_knowledge_as(
                    worlds[buyer].automaton,
                    *orbit,
                    expansion.clone(),
                    cited.clone(),
                )
                .is_ok()
            {
                licensed.push((worlds[buyer].spec.world_name.clone(), price));
            }
        }
    }
    licensed
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
            match worlds.iter_mut().find(|e| e.spec.world_name == world_name) {
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
                        Err(refused) => {
                            log.push(format!("HANDOFF REFUSED — {world_name}: {refused:?}"))
                        }
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
        .map(|receipt| receipt.own().0)
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
    // N-I10 — seed the SIBLING cognitive vessel too; `genesis` re-reads it by
    // handle on the resume below (the binding is only for the register side-effect).
    let _sibling = match world.register_agent("sibling") {
        Ok(entity) => entity,
        Err(_) => world
            .committed()
            .entities
            .iter()
            .find(|held| held.handle == "sibling")
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
    let coordinate = |axis: &str| -> Ratio { ratio(beacon.uniform_index(axis, 64) as i64, 1) };
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
        if let Ok(Some(galaxy)) = book.observe_galaxy(
            &polity.spec.civilization.sigil,
            &creator,
            (&x, &y, &z),
            stamp,
        ) {
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

/// **COG-DIRECTION Slice 3b — THE POPULATION CENSUS (true natural selection).**
/// Once per beat, BEFORE the cycle loop, read every warm-booked line's cohesion
/// (its syntropy fraction, `thermal_balance_of` — `None` when the books are cold,
/// so a cold line is not in the distribution and holds) and stamp each line's
/// `pop_cohesion_band` with the TERTILE CUTS of that live distribution. A line
/// then reads its own direction RELATIVE TO THE POPULATION: the top third grows
/// (fittest), the bottom third declines (selected against), the middle stagnates.
/// Fitness is relative to peers, not an absolute dial — evolution over the
/// syntropic value. Fewer than three warm lines → no band (nothing to be relative
/// to; the absolute read holds). Grounded by measurement (2026-08-21): the
/// cohesion LEVEL spreads 0..0.93 (median 0.070, 72/90 distinct) — a clean
/// partition — while the per-beat FLOW is ~75% flat at every window, so selection
/// reads the discriminating level, not the flat flow. This mutates only ephemeral
/// conductor state (`trajectory`/`level`), never the chronicle — no replay risk.
pub fn direction_census_step(worlds: &mut [Polity]) {
    let mut cohesions: Vec<Ratio> = worlds
        .iter()
        .filter_map(|p| {
            let id = p.world.orb()?.orb_id;
            p.world.thermal_balance_of(id)
        })
        .collect();
    if cohesions.len() < 3 {
        for p in worlds.iter_mut() {
            p.pop_cohesion_band = None;
        }
        return;
    }
    cohesions.sort();
    let n = cohesions.len();
    let band = (cohesions[n / 3].clone(), cohesions[(2 * n) / 3].clone());
    for p in worlds.iter_mut() {
        p.pop_cohesion_band = Some(band.clone());
    }
}

/// **THE EMERGENT CENSUS, LIVE** — make the wired-but-silent civilization visible
/// on the wire (mesh relay #69 + the audit's invisible layers). COG-ECON reports
/// the ECONOMY moving (chartered lanes, banked primitives, Alloy, zones built) and
/// the COGNITIVE FLOOR (minds whose cognitive_capacity has lifted off survival to
/// 3 — a seated science). COG-SOC reports the SELF-ORGANIZATION (caste count +
/// how many lines have a multi-caste society) that until now steered investment
/// entirely unseen. Observation only; reads committed/derived state, writes
/// nothing. Bounded to the live (unpenned) roster.
pub fn emergent_census(worlds: &[Polity], beat: u64) {
    let live: Vec<&Polity> = worlds.iter().filter(|p| !p.penned).collect();
    if live.is_empty() {
        return;
    }
    let mut routes = 0usize;
    let mut zones = 0usize;
    let mut minds_at_cap = 0usize;
    let mut caste_lines = 0usize;
    let mut castes_total = 0usize;
    let mut banked_bic = ratio(0, 1);
    let mut alloy = ratio(0, 1);
    for p in &live {
        routes += p.world.routes().len();
        zones += p.world.orb().map(|o| o.zones.len()).unwrap_or(0);
        if p.world.cognitive_capacity(cfe::world::FIRST_ORB) >= 3 {
            minds_at_cap += 1;
        }
        for r in [Resource::Biomass, Resource::Iron, Resource::Crystal] {
            banked_bic += banked(&p.world, r);
        }
        alloy += banked(&p.world, Resource::Alloy);
        let census = p.caste_partition();
        if census.len() >= 2 {
            caste_lines += 1;
            castes_total += census.len();
        }
    }
    eprintln!(
        "COG-ECON beat {beat}: civs={} routes={routes} banked_BIC={} alloy={} zones={zones} minds_at_cap3={minds_at_cap} (trade | building | cognitive floor)",
        live.len(),
        banked_bic.to_integer(),
        alloy.to_integer(),
    );
    eprintln!(
        "COG-SOC beat {beat}: caste_societies={caste_lines}/{} castes_total={castes_total} (self-organization made visible)",
        live.len(),
    );
}

/// **THE SHARED-LANGUAGE CENSUS, LIVE** — prove on the wire that the
/// civilizations speak ONE tongue, learned from the AUTIC network, not private
/// dialects. Three measured claims over the live population's own basins:
///   1. COMMON VOCABULARY — crystals held by >= 2 civilizations.
///   2. MUTUAL INTELLIGIBILITY — a word one civ speaks ASSEMBLES (drains to zero,
///      `reduct::assemble`) against ANOTHER civ's independently-grown basin.
///   3. CONTROL — synthetic words OUTSIDE the language must NOT assemble, or the
///      census proves nothing. Comprehension >> control is the proof.
/// Emits COG-PROOF. Bounded: samples up to `SAMPLE` speaking civs so the
/// O(civs^2 * words) cross-civ pass stays cheap on a 100+ roster. Observation
/// only — reads basins, writes nothing.
pub fn shared_language_census(worlds: &[Polity], beat: u64) {
    const SAMPLE: usize = 8;
    const WORDS: usize = 40; // bounded vocabulary per civ — keeps the pass cheap
    let vocabs: Vec<Vec<reduct::ChainDelta>> = worlds
        .iter()
        .filter(|p| !p.penned)
        .map(|p| p.basin_sample(WORDS))
        .filter(|v| !v.is_empty())
        .take(SAMPLE)
        .collect();
    if vocabs.len() < 2 {
        return; // not enough speaking civilizations yet — no claim to make
    }
    // CLAIM 1 — common vocabulary: how many crystals are held by >= 2 civs.
    let mut seen: Vec<(reduct::ChainDelta, usize)> = Vec::new();
    for v in &vocabs {
        for w in v {
            match seen.iter_mut().find(|(c, _)| c == w) {
                Some(entry) => entry.1 += 1,
                None => seen.push((w.clone(), 1)),
            }
        }
    }
    let shared = seen.iter().filter(|(_, n)| *n >= 2).count();
    let max_holders = seen.iter().map(|(_, n)| *n).max().unwrap_or(0);
    // CLAIM 2 — mutual intelligibility across every sampled ordered pair.
    let (mut num, mut den) = (0u64, 0u64);
    for (i, va) in vocabs.iter().enumerate() {
        for (j, vb) in vocabs.iter().enumerate() {
            if i == j {
                continue;
            }
            let basis_b: Vec<reduct::Crystal> =
                vb.iter().cloned().map(reduct::Crystal::seal).collect();
            for w in va {
                den += 1;
                if reduct::assemble(&reduct::AuticSentence::speak(vec![w.clone()]), &basis_b)
                    .is_assembled()
                {
                    num += 1;
                }
            }
        }
    }
    // CLAIM 3 — control: synthetic words guaranteed outside the language (varied
    // by beat so the census cannot accidentally lock onto a real crystal).
    let control: Vec<reduct::ChainDelta> = (0..8i64)
        .map(|i| reduct::probe_class(1_000_000 + beat as i64 * 131 + i, (i % 9 + 1) as u8))
        .collect();
    let (mut cnum, mut cden) = (0u64, 0u64);
    for vb in &vocabs {
        let basis_b: Vec<reduct::Crystal> = vb.iter().cloned().map(reduct::Crystal::seal).collect();
        for w in &control {
            cden += 1;
            if reduct::assemble(&reduct::AuticSentence::speak(vec![w.clone()]), &basis_b)
                .is_assembled()
            {
                cnum += 1;
            }
        }
    }
    let permille = |n: u64, d: u64| if d > 0 { n * 1000 / d } else { 0 };
    eprintln!(
        "COG-PROOF beat {beat}: civs={} shared_words={shared} max_holders={max_holders} comprehension={}permille control={}permille (intelligibility {num}/{den} vs control {cnum}/{cden})",
        vocabs.len(),
        permille(num, den),
        permille(cnum, cden),
    );
}

/// Run one era: CONVERGE the archived eras (07 §1.6), then genesis for
/// every polity, then `cycles` heartbeats; branches born mid-era join the
/// beat. Returns the live worlds.
pub fn run_era(var_root: &str, cycles: u32) -> Vec<Polity> {
    run_era_inner(var_root, cycles, None, None)
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
    run_era_inner(var_root, cycles, None, Some(&arrived))
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
    run_era_inner(var_root, cycles, Some(seats), Some(&arrived))
}

fn run_era_inner(
    var_root: &str,
    cycles: u32,
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
            Polity::genesis(spec, &registry, &book)
        })
        .collect();
    for beat in 0..cycles {
        // TUniversal T = epoch base + this era's beat (07 §1.3).
        let _ = book.set_universal_clock(epoch_base + u64::from(beat) + 1);
        // Slice 3b: the population census BEFORE the cycle loop, so each line's
        // direction read this beat is relative to the live cohort.
        direction_census_step(&mut worlds);
        // BEAT-PARALLEL (#152) — the same per-civ fan-out the live loop uses, here
        // so the SEAL-B pins PROVE it bit-identical to sequential: each civ is
        // deterministically self-seeded and mutates only its own world. Balanced
        // by MEASURED cost (`spent + swept`, greedy LPT bin-packing) rather than
        // civ count — one heavy civ no longer stalls a whole thread. Determinism
        // rides on each result's ORIGINAL index (sorted back before use), not on
        // chunk/spawn order, so replay == committed regardless of assignment.
        let newborns: Vec<PolitySpec> = {
            let threads = std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
                .min(16);
            let book = &book;
            let registry = &registry;
            let mut indexed: Vec<(usize, &mut Polity)> = worlds.iter_mut().enumerate().collect();
            indexed.sort_by_key(|(_, p)| std::cmp::Reverse(p.spent + p.swept));
            let mut buckets: Vec<Vec<(usize, &mut Polity)>> =
                (0..threads).map(|_| Vec::new()).collect();
            let mut bucket_cost: Vec<u64> = vec![0; threads];
            for (idx, polity) in indexed {
                let lightest = bucket_cost
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, &c)| c)
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                bucket_cost[lightest] += polity.spent + polity.swept;
                buckets[lightest].push((idx, polity));
            }
            let mut results: Vec<(usize, PolitySpec)> = std::thread::scope(|s| {
                let handles: Vec<_> = buckets
                    .into_iter()
                    .map(|bucket| {
                        s.spawn(move || {
                            let mut local = Vec::new();
                            for (idx, polity) in bucket {
                                // 50_WIN_CONDITIONS: a collapsed line (every
                                // orb at zero population) already refuses
                                // every scoped action (`permitted`) — but
                                // nothing stopped the beat loop from still
                                // paying its FULL pipeline forever on a world
                                // that can no longer act. THE ONLY CLOSURE
                                // THIS GAME HAS gets no exit from the roster:
                                // retire it from cycling the instant it
                                // closes, symmetric to how `divergence_step`
                                // ADDS a line the instant one is born.
                                if polity.world.collapsed().is_some() {
                                    continue;
                                }
                                if let Some(spec) = polity.cycle(book, registry) {
                                    local.push((idx, spec));
                                }
                            }
                            local
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .flat_map(|h| h.join().unwrap_or_default())
                    .collect()
            });
            results.sort_by_key(|(idx, _)| *idx);
            results.into_iter().map(|(_, spec)| spec).collect()
        };
        // A line that collapsed this cycle leaves the roster before the
        // cross-world steps can name it as a party (`retire_collapsed`).
        retire_collapsed(&mut worlds, &book);
        for spec in newborns {
            worlds.push(Polity::genesis(spec, &registry, &book));
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
            &book
                .world_dir(&polity.spec.world_name)
                .join("chronicle.tlv"),
        );
    }
    worlds
}

/// **THE ONE CLOSURE'S EXIT** (50_WIN_CONDITIONS). A collapsed line (every
/// orb at zero population, `CivilizationCollapsed`) refuses every scoped act,
/// but a dead world left in the roster is still a PARTY to every cross-world
/// step: measured in G2's baseline, dimlight-hold traded 4,485 apertures after
/// it collapsed at receipt 112. Retiring it here — its chronicle synced whole
/// first, so the carrier holds every receipt it ever took — removes it from
/// every step at once, symmetric to how `divergence_step` ADDS a line the
/// instant one is born. Returns the retired world names, in roster order.
pub fn retire_collapsed(worlds: &mut Vec<Polity>, book: &Ledger) -> Vec<String> {
    let mut retired = Vec::new();
    worlds.retain(|polity| {
        if polity.world.collapsed().is_none() {
            return true;
        }
        let _ = cfe::persist::sync(
            polity.world.chronicle(),
            &book.world_dir(&polity.spec.world_name).join("chronicle.tlv"),
        );
        retired.push(polity.spec.world_name.clone());
        false
    });
    retired
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

fn write_judged_sidecar(
    dir: &std::path::Path,
    watermark: usize,
    refused: &[(u64, kernel::Refused)],
) {
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
    static TRACED: OnceLock<bool> = OnceLock::new();
    static BOOK: Mutex<Vec<(&'static str, Duration)>> = Mutex::new(Vec::new());

    fn armed() -> bool {
        *ARMED.get_or_init(|| std::env::var_os("NETSTRATUM_PROFILE").is_some())
    }

    /// **THE BOUNDARY TRACE** (task #136) — armed by `NETSTRATUM_TRACE`, emits a
    /// line on stderr AS each step is entered and left, so a step that never
    /// returns (a beat that will not advance) is named the instant it is entered:
    /// the last `-> name` with no `<- name` is the stall. Unarmed it is one
    /// `OnceLock` read. Distinct from the aggregate profile, which cannot report
    /// a beat that never completes.
    fn traced() -> bool {
        *TRACED.get_or_init(|| std::env::var_os("NETSTRATUM_TRACE").is_some())
    }

    pub fn timed<T>(name: &'static str, run: impl FnOnce() -> T) -> T {
        if traced() {
            use std::io::Write;
            eprintln!("trace: -> {name}");
            let _ = std::io::stderr().flush();
        }
        if !armed() {
            let out = run();
            if traced() {
                use std::io::Write;
                eprintln!("trace: <- {name}");
                let _ = std::io::stderr().flush();
            }
            return out;
        }
        let at = Instant::now();
        let out = run();
        let spent = at.elapsed();
        if traced() {
            use std::io::Write;
            eprintln!("trace: <- {name} {}ms", spent.as_millis());
            let _ = std::io::stderr().flush();
        }
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
                h.assigned_route == 0 && HullRole::from_wire(h.role) == Some(HullRole::Escort)
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
                    h.assigned_route == 0 && HullRole::from_wire(h.role) == Some(HullRole::Escort)
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
        && (orb.banked_amount(Resource::Crystal) > ratio(0, 1) || orb.stamp_amount() > ratio(0, 1))
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
            if let (Some(u), Some(p)) = (Positive::new(ratio(1, 1)), Positive::new(ratio(1, 1))) {
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
    let has_yard = orb
        .zones
        .iter()
        .any(|z| z.kind == cfe::dop::ZoneKind::ShipYard);
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
    if !orb
        .zones
        .iter()
        .any(|z| z.kind == cfe::dop::ZoneKind::ShipYard)
    {
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
