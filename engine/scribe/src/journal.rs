//! THE CHRONICLE FEED (08 — JOURNAL-1): civilizations SPEAK — one
//! journal entry per completed year, generated lens-side from the
//! chronicle + committed state ONLY.
//!
//! The laws (08 §1), binding here:
//!
//! - **A RENDERING, never state.** No new chronicle events, no
//!   receipts, no simulation reads. Deleting every journal changes
//!   nothing; regenerating reproduces it byte-identically.
//! - **Deterministic narrative.** Same chronicle ⇒ same journal.
//!   Flavor selection seeds from receipt digests already in the
//!   chronicle through the beacon (CHARTER §9) — no wall clock, no
//!   ambient randomness.
//! - **Exact rationals as `p/q` strings** (C2R): growth, joules, sink.
//!   Never floats, never truncated integers.
//! - **The terminology law holds:** Furnace (never Crucible); no
//!   set-letter glyphs; world-book names from `06` (Emberkin,
//!   Coherent, Glasskeeper, Marrow, Wanderer; physiology-register
//!   labels like "Radiotroph" ride beside them).
//! - **A year = 60 ticks** (placeholder, [`YEAR_TICKS`]) — one field
//!   of the TUniversal base-60 odometer; the entry header shows the
//!   odometer cycle of the WORLD's own clock (chronicle-derived, so
//!   regeneration never depends on any epoch outside the chronicle).
//!
//! THE YEAR WINDOW: year `y` (1-based) covers stamps in
//! `[(y−1)·YEAR_TICKS, y·YEAR_TICKS)` and completes when the clock
//! reaches `y·YEAR_TICKS`. The window is CLOSED under the clock law
//! (stamps never decrease), so a written year can never change
//! retroactively — append-only regeneration stays byte-identical.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::vrp::{Chronicle, Receipt};
use cfe::world::{replay, Committed, World};
use physics::Beacon;
use std::fs;
use std::io::Write as _;

use ledger::registry::Civilization;
use ledger::Ledger;
use cfe::ratio::ratio_str;
use lens::render::universal_clock;

/// A year = 60 ticks (08 §1.5, placeholder) — one field of the
/// TUniversal base-60 odometer.
pub const YEAR_TICKS: u64 = 60;

/// The identity facts a journal entry narrates from — a REFLECTION of
/// the registry entry (never an authority; the registry stays the
/// directory of receipted truth, 04 §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Voice {
    pub civilization: String,
    pub disposition: String,
    pub metabolism: String,
    pub perception: String,
}

impl Voice {
    /// The voice of one registered civilization.
    pub fn of(civilization: &Civilization) -> Voice {
        Voice {
            civilization: civilization.name.clone(),
            disposition: civilization.disposition.clone(),
            metabolism: civilization.metabolism.clone(),
            perception: civilization.perception.clone(),
        }
    }
}

/// The alert level of one entry (08 §2): Normal (cyan) · Warning
/// (red) · Milestone (blue).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alert {
    Normal,
    Warning,
    Milestone,
}

impl Alert {
    pub fn word(self) -> &'static str {
        match self {
            Alert::Normal => "normal",
            Alert::Warning => "warning",
            Alert::Milestone => "milestone",
        }
    }
}

/// The economic summary of one year — every quantity an EXACT string
/// (integers plain, fractions `p/q`; never a decimal).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicSummary {
    /// Banked joules at the year boundary, summed across orbs.
    pub banked_joules: String,
    /// The year's heating bill (ECON-J, 05 §1.2): heating-per-zone ×
    /// zones × [`YEAR_TICKS`] on sited sub-mesic orbs; 0 elsewhere.
    pub heating_need_year: String,
    /// Cumulative vented entropy at the boundary (VENT-1 ledger).
    pub entropy_vented: String,
    /// `VentSettled` amounts inside this year's window, summed.
    pub vented_this_year: String,
    /// Banked sink (`S_cool`) at the boundary, summed across orbs.
    pub banked_sink: String,
    /// Zones standing at the boundary, across orbs.
    pub zones: u64,
    /// `ZoneBuilt` receipts inside the window.
    pub zones_built: u64,
    /// `ZoneRazed` receipts inside the window.
    pub zones_razed: u64,
    /// Exact growth per year: (zones at end − zones at start) over
    /// max(zones at start, 1) — `p/q`, never a percentage float.
    pub growth_per_year: String,
}

/// One journal entry — one completed year of one world, derived from
/// the chronicle + committed state only (08 §1.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntry {
    /// The completed year, 1-based.
    pub year: u64,
    /// The boundary tick (`year × YEAR_TICKS`).
    pub tick: u64,
    /// The base-60 odometer cycle of the boundary tick.
    pub cycle: String,
    pub world: String,
    pub civilization: String,
    /// The world-book register (06): people + physiology label.
    pub archetype: String,
    pub alert: Alert,
    pub headline: String,
    /// The narrative lines, joined by `\n`.
    pub body: String,
    pub economics: EconomicSummary,
    /// Resources under famine at the boundary, ascending.
    pub famine: Vec<String>,
    /// Wants and offers derived from deficits/surpluses (08 §2).
    pub trade_request: Option<String>,
}

/// The joule stance at a year boundary — the surplus/deficit narrative
/// flips on the EXACT boundary: covered at `banked == need`, deficit
/// strictly below it (08 §4, witnessed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JouleStance {
    Covered,
    Deficit,
}

/// Classify banked joules against the year's heating need — an exact
/// rational comparison, no rounding anywhere.
pub fn joule_stance(banked: &Ratio, need: &Ratio) -> JouleStance {
    match banked < need {
        true => JouleStance::Deficit,
        false => JouleStance::Covered,
    }
}

/// The banked-sink offer line (05 §1.4): absorption capacity offered
/// to Furnace traders — `None` when nothing is banked.
pub fn sink_offer(banked_sink: &Ratio) -> Option<String> {
    match *banked_sink > ratio(0, 1) {
        true => Some(format!(
            "Radiator headroom banked: {} S_cool on offer to Furnace traders — we absorb what the fire cannot hold.",
            ratio_str(banked_sink)
        )),
        false => None,
    }
}

// ---- the fixed flavor tables (08 §1.2: digest-seeded selection) ----

/// Leisure lines per disposition (registry axis) — indexed by beacon.
fn leisure_table(disposition: &str) -> [&'static str; 3] {
    match disposition {
        "expansionist" => [
            "The young race survey drones along the perimeter and wager rations on the fastest lap.",
            "Founding songs are sung at the edge of the built grid, facing the empty tiles.",
            "The holiday of First Iron is kept by racing ore carts down the extractor ramps.",
        ],
        "epicurean" => [
            "The long tables are set again; this year's vintage is judged against the ledger of every year before.",
            "Steam gardens open at dusk and the shift bells go unrung for a day.",
            "A festival of small comforts: lamplight, warm alloyglass, and no talk of upkeep.",
        ],
        // Empiricist — and the reference register for any future axis.
        _ => [
            "The observatory league published its measurements and argued them joyfully past midnight.",
            "Apprentices re-derived the founding ledgers by hand, for the pleasure of exactness.",
            "The year's puzzle tournament crowned a solver; the winning program is recited like verse.",
        ],
    }
}

/// A perception-flavored closing clause for the leisure line.
fn perception_clause(perception: &str) -> [&'static str; 2] {
    match perception {
        "volumetric" => [
            "The stacked halls echo it floor through floor.",
            "It is carved into the deep galleries, layer under layer.",
        ],
        "harmonic" => [
            "The chord of it hangs in the resonance halls.",
            "Every hull hums the refrain for a week after.",
        ],
        _ => [
            "The plaza mosaics record it tile by tile.",
            "It is chalked across the flat commons for all to read.",
        ],
    }
}

/// Milestone headlines — growth years.
const MILESTONE_HEADLINES: [&str; 3] = [
    "the skyline grows",
    "new zones rise on the grid",
    "the build ledger runs long",
];

/// Normal headlines — quiet years.
const NORMAL_HEADLINES: [&str; 3] = [
    "steady operations",
    "the year turns quietly",
    "all ledgers balance",
];

/// The world-book register (06 §1–§2): the thermal band — and, for
/// unsited worlds, the metabolism axis — fixes which people speaks.
/// Only band and function are mechanically real; this label is the
/// physiology register riding the entry header.
fn archetype(committed: &Committed, voice: &Voice) -> &'static str {
    use cfe::thermal::{band_of, t_eq4, SubstrateBand};
    let site = committed
        .orbs
        .iter()
        .find_map(|orb| orb.thermal.as_ref());
    if let (Some(class), Some(site)) = (committed.star, site) {
        let t4 = t_eq4(&class.luminosity(), site.albedo.value(), site.orbit_r.value());
        return match band_of(&t4) {
            Some(SubstrateBand::HyperThermal) => "Emberkin (Radiotroph)",
            Some(SubstrateBand::Cryogenic) => "Coherent (Cryo-Superconductor)",
            Some(SubstrateBand::Mesic) => "Wanderer (Mesic Generalist)",
            None => match t4 < SubstrateBand::Cryogenic.bounds4().0 {
                true => "Coherent (Cryo-Superconductor)",
                false => "Emberkin (Radiotroph)",
            },
        };
    }
    // Unsited worlds speak from the deep regimes (06 §1.1, §1.3).
    match voice.metabolism.as_str() {
        "mineral" => "Glasskeeper (Litho-Chalcogen)",
        _ => "Marrow (Endolith)",
    }
}

fn resource_word(resource: Resource) -> &'static str {
    match resource {
        Resource::Iron => "iron",
        Resource::Biomass => "biomass",
        Resource::Alloy => "alloy",
        Resource::Diamond => "diamond",
        Resource::Joule => "joule",
        Resource::Sink => "sink",
        Resource::Crystal => "crystal",
    }
}

fn zone_word(kind: ZoneKind) -> &'static str {
    match kind {
        ZoneKind::Extractor => "extractor",
        ZoneKind::Habitat => "habitat",
        ZoneKind::Smelter => "smelter",
        ZoneKind::Synthesizer => "synthesizer",
        ZoneKind::Encasement => "encasement shell",
        ZoneKind::CryoArray => "cryo-lattice array",
        ZoneKind::Regenerator => "regenerator",
        ZoneKind::Resonator => "resonator",
    }
}

/// Generate the full journal of a world: one entry per completed year
/// (08 §1.5), derived from the chronicle + committed prefixes only.
/// Pure and deterministic — same chronicle ⇒ same entries, bytes and
/// all. A zero `year_ticks` measures no year and yields no entries.
pub fn journal(world_name: &str, world: &World, voice: &Voice, year_ticks: u64) -> Vec<JournalEntry> {
    if year_ticks == 0 {
        return Vec::new();
    }
    let receipts = world.chronicle().receipts();
    let years = world.now() / year_ticks;
    let mut out = Vec::new();
    // The committed prefix replays through the REAL chain machinery:
    // events re-append into a partial chronicle and `replay` grounds
    // the boundary state — no shadow fold to drift from `cfe::world`.
    let mut partial = Chronicle::new();
    let mut fed = 0usize;
    let mut zones_before: u64 = 0;
    for year in 1..=years {
        let boundary = year * year_ticks;
        let window_start = fed;
        while fed < receipts.len() && receipts[fed].stamp < boundary {
            let held = &receipts[fed];
            partial.append(held.event.clone(), held.stamp, held.provenance.clone());
            fed += 1;
        }
        let window = &receipts[window_start..fed];
        // A year with no history before it has no one to speak.
        let Some(seed) = receipts[..fed].last() else {
            continue;
        };
        let committed = replay(&partial);
        let entry = narrate(
            world_name,
            voice,
            year,
            year_ticks,
            &committed,
            window,
            zones_before,
            seed,
        );
        zones_before = committed.orbs.iter().map(|orb| orb.zones.len() as u64).sum();
        out.push(entry);
    }
    out
}

/// Narrate one completed year from its boundary state and window
/// receipts — the 08 §2 state→text mapping, nothing invented.
#[allow(clippy::too_many_arguments)]
fn narrate(
    world_name: &str,
    voice: &Voice,
    year: u64,
    year_ticks: u64,
    committed: &Committed,
    window: &[Receipt],
    zones_before: u64,
    seed: &Receipt,
) -> JournalEntry {
    use cfe::thermal::{cryo_vault_max4, t_eq4, SubstrateBand};
    let zero = ratio(0, 1);
    let beacon = Beacon::from_digest(seed.own.0);

    // ---- the boundary reads (committed state only) ----
    let banked_joules: Ratio = committed
        .orbs
        .iter()
        .fold(zero.clone(), |sum, orb| sum + orb.banked_amount(Resource::Joule));
    let banked_sink: Ratio = committed
        .orbs
        .iter()
        .fold(zero.clone(), |sum, orb| sum + orb.banked_amount(Resource::Sink));
    let entropy_vented: Ratio = committed
        .orbs
        .iter()
        .fold(zero.clone(), |sum, orb| sum + orb.entropy_vented.clone());
    let zones_now: u64 = committed.orbs.iter().map(|orb| orb.zones.len() as u64).sum();
    let overheat = committed.orbs.iter().any(|orb| orb.overheat);

    // The thermal siting read: first sited orb carries the band.
    let sited = committed
        .orbs
        .iter()
        .find_map(|orb| orb.thermal.as_ref().map(|site| (orb, site.clone())));
    let t4 = match (committed.star, &sited) {
        (Some(class), Some((_, site))) => {
            Some(t_eq4(&class.luminosity(), site.albedo.value(), site.orbit_r.value()))
        }
        _ => None,
    };
    let sub_mesic = t4
        .as_ref()
        .is_some_and(|t4| *t4 < SubstrateBand::Mesic.bounds4().0);
    let in_vault = t4.as_ref().is_some_and(|t4| *t4 <= cryo_vault_max4());

    // ECON-J (05 §1.2): the year's heating bill on sub-mesic orbs.
    let heating_need: Ratio = match sub_mesic {
        true => {
            cfe::dop::heating_per_zone()
                * ratio(zones_now as i64, 1)
                * ratio(year_ticks as i64, 1)
        }
        false => zero.clone(),
    };
    let stance = joule_stance(&banked_joules, &heating_need);
    let freeze_locked = sub_mesic
        && committed
            .orbs
            .iter()
            .any(|orb| orb.famine.contains(&Resource::Joule));

    // ---- the window reads (this year's receipts) ----
    let mut built: u64 = 0;
    let mut razed: u64 = 0;
    let mut built_kinds: Vec<&'static str> = Vec::new();
    let mut vented_this_year = zero.clone();
    let mut chartered: Vec<String> = Vec::new();
    let mut registered: u64 = 0;
    for receipt in window {
        match &receipt.event {
            ClosureEvent::ZoneBuilt { kind, .. } => {
                built += 1;
                let word = zone_word(*kind);
                if !built_kinds.contains(&word) {
                    built_kinds.push(word);
                }
            }
            ClosureEvent::ZoneRazed { .. } => razed += 1,
            ClosureEvent::VentSettled { amount, .. } => vented_this_year += amount.clone(),
            ClosureEvent::ColonyCharted { name, .. } => chartered.push(name.clone()),
            ClosureEvent::EntityRegistered { .. } => registered += 1,
            _ => {}
        }
    }

    // Growth as an exact rational per year (08 §2): net over the
    // standing count at the year's start (floor 1 — a founding year
    // grows over one notional plot, never divides by zero).
    let net = ratio(zones_now as i64, 1) - ratio(zones_before as i64, 1);
    let growth = net / ratio(zones_before.max(1) as i64, 1);

    // The famine list, resource-ascending, deduplicated across orbs.
    let mut famine: Vec<String> = Vec::new();
    for orb in &committed.orbs {
        for resource in &orb.famine {
            let word = resource_word(*resource).to_string();
            if !famine.contains(&word) {
                famine.push(word);
            }
        }
    }
    famine.sort();

    // ---- the narrative (08 §2, top to bottom of the mapping) ----
    let mut lines: Vec<String> = Vec::new();

    // Joules vs heating need.
    match (stance, heating_need > zero) {
        (JouleStance::Deficit, _) => lines.push(format!(
            "RED — the joule bank holds {} against a heating bill of {} this year: fuel is sought before freeze-lock.",
            ratio_str(&banked_joules),
            ratio_str(&heating_need),
        )),
        (JouleStance::Covered, true) => lines.push(format!(
            "The joule bank stands at {} against a heating bill of {} — the surplus goes to building and expanding.",
            ratio_str(&banked_joules),
            ratio_str(&heating_need),
        )),
        (JouleStance::Covered, false) => {
            if banked_joules > zero {
                lines.push(format!(
                    "The joule bank stands at {} with no heating bill — the surplus goes to building and expanding.",
                    ratio_str(&banked_joules),
                ));
            }
        }
    }

    // Famine / freeze-lock, RED per resource with dormant zones named.
    if freeze_locked {
        lines.push(
            "RED — FREEZE-LOCK: the joule stores ran dry on a sub-mesic orbit; every flow stands still until fuel arrives."
                .to_string(),
        );
    }
    for orb in &committed.orbs {
        for resource in &orb.famine {
            let dormant: Vec<&'static str> = orb
                .zones
                .iter()
                .filter(|zone| !orb.zone_active(zone))
                .map(|zone| zone_word(zone.kind))
                .fold(Vec::new(), |mut kinds, word| {
                    if !kinds.contains(&word) {
                        kinds.push(word);
                    }
                    kinds
                });
            let dormant_note = match dormant.is_empty() {
                true => String::new(),
                false => format!("; dormant: {}", dormant.join(", ")),
            };
            lines.push(format!(
                "RED — famine on {}: the {} stores of {} ran dry{}.",
                resource_word(*resource),
                resource_word(*resource),
                orb.name,
                dormant_note,
            ));
        }
    }

    // Thermal stress vs pristine syntropic operations.
    if t4.is_some() {
        if overheat {
            lines.push(format!(
                "Thermal stress: the radiators run past their rating; {} entropy vented this year ({} on the cumulative ledger).",
                ratio_str(&vented_this_year),
                ratio_str(&entropy_vented),
            ));
        } else if vented_this_year > zero {
            lines.push(format!(
                "The overflow cleared, but {} entropy vented this year ({} cumulative) — the shells want widening.",
                ratio_str(&vented_this_year),
                ratio_str(&entropy_vented),
            ));
        } else {
            lines.push(
                "Pristine syntropic operations: nothing vented this year; every excess joule was encased."
                    .to_string(),
            );
        }
    }

    // Banked sink + vault band (05 §1.4).
    if let Some(offer) = sink_offer(&banked_sink) {
        lines.push(offer);
    }
    if in_vault && banked_joules > zero {
        lines.push(
            "The vault band holds: banked stores sit lossless in the deep cold.".to_string(),
        );
    }

    // What they BUILT, with exact growth.
    if built > 0 {
        let raze_note = match razed {
            0 => String::new(),
            n => format!(", {n} razed"),
        };
        lines.push(format!(
            "Built this year: {} zone(s) ({}){} — {} standing; growth {} per year.",
            built,
            built_kinds.join(", "),
            raze_note,
            zones_now,
            ratio_str(&growth),
        ));
    } else if razed > 0 {
        lines.push(format!(
            "Ground cleared: {} zone(s) razed — {} standing; growth {} per year.",
            razed,
            zones_now,
            ratio_str(&growth),
        ));
    } else {
        lines.push(format!(
            "No new construction — {} zone(s) standing; growth {} per year.",
            zones_now,
            ratio_str(&growth),
        ));
    }

    // Expansion and population notes.
    for charted in &chartered {
        lines.push(format!(
            "A new colony was chartered: {charted} — the line is seeking space.",
        ));
    }
    if registered > 0 {
        lines.push(format!(
            "{registered} entit{} joined the rolls this year.",
            match registered {
                1 => "y",
                _ => "ies",
            }
        ));
    }

    // Leisure & culture flavor — digest-seeded (08 §1.2), tables fixed.
    let leisure = leisure_table(&voice.disposition);
    let clause = perception_clause(&voice.perception);
    lines.push(format!(
        "{} {}",
        leisure[beacon.uniform_index("journal.leisure", leisure.len() as u64) as usize],
        clause[beacon.uniform_index("journal.perception", clause.len() as u64) as usize],
    ));

    // Trade requests derive from deficits and surpluses (08 §2).
    let mut trade: Vec<String> = Vec::new();
    if stance == JouleStance::Deficit || freeze_locked {
        trade.push("seeking joules (fuel before freeze-lock)".to_string());
    }
    for resource in &famine {
        if resource != "joule" {
            trade.push(format!("seeking {resource}"));
        }
    }
    if banked_sink > zero {
        trade.push(format!("offering {} S_cool", ratio_str(&banked_sink)));
    }
    let trade_request = match trade.is_empty() {
        true => None,
        false => Some(trade.join("; ")),
    };
    if let Some(request) = &trade_request {
        lines.push(format!("To the trade lanes: {request}."));
    }

    // ---- alert + headline ----
    let alert = if !famine.is_empty() || freeze_locked || overheat || stance == JouleStance::Deficit
    {
        Alert::Warning
    } else if built > 0 || !chartered.is_empty() || registered > 0 {
        Alert::Milestone
    } else {
        Alert::Normal
    };
    let headline = match alert {
        Alert::Warning => {
            if freeze_locked {
                "RED ALERT — freeze-lock: joule stores dry against the cold".to_string()
            } else if let Some(first) = famine.first() {
                format!("RED ALERT — famine: the {first} stores ran dry")
            } else if stance == JouleStance::Deficit {
                "RED ALERT — joule deficit against the heating bill".to_string()
            } else {
                "RED ALERT — thermal stress: the radiators run past their rating".to_string()
            }
        }
        Alert::Milestone => MILESTONE_HEADLINES
            [beacon.uniform_index("journal.headline", MILESTONE_HEADLINES.len() as u64) as usize]
            .to_string(),
        Alert::Normal => NORMAL_HEADLINES
            [beacon.uniform_index("journal.headline", NORMAL_HEADLINES.len() as u64) as usize]
            .to_string(),
    };

    JournalEntry {
        year,
        tick: year * year_ticks,
        cycle: universal_clock(year * year_ticks),
        world: world_name.to_string(),
        civilization: voice.civilization.clone(),
        archetype: archetype(committed, voice).to_string(),
        alert,
        headline,
        body: lines.join("\n"),
        economics: EconomicSummary {
            banked_joules: ratio_str(&banked_joules),
            heating_need_year: ratio_str(&heating_need),
            entropy_vented: ratio_str(&entropy_vented),
            vented_this_year: ratio_str(&vented_this_year),
            banked_sink: ratio_str(&banked_sink),
            zones: zones_now,
            zones_built: built,
            zones_razed: razed,
            growth_per_year: ratio_str(&growth),
        },
        famine,
        trade_request,
    }
}

/// Render one entry as its journal.jsonl line (08 §3) — flat JSON via
/// the ledger's own writer, one line, no trailing newline.
pub fn render_entry(entry: &JournalEntry) -> String {
    let mut w = pen::JsonWriter::new();
    w.open();
    w.field_u64("year", entry.year);
    w.field_u64("tick", entry.tick);
    w.field_str("cycle", &entry.cycle);
    w.field_str("world", &entry.world);
    w.field_str("civilization", &entry.civilization);
    w.field_str("archetype", &entry.archetype);
    w.field_str("alert", entry.alert.word());
    w.field_str("headline", &entry.headline);
    w.field_str("body", &entry.body);
    w.key("economics");
    w.open();
    w.field_str("banked_joules", &entry.economics.banked_joules);
    w.field_str("heating_need_year", &entry.economics.heating_need_year);
    w.field_str("entropy_vented", &entry.economics.entropy_vented);
    w.field_str("vented_this_year", &entry.economics.vented_this_year);
    w.field_str("banked_sink", &entry.economics.banked_sink);
    w.field_u64("zones", entry.economics.zones);
    w.field_u64("zones_built", entry.economics.zones_built);
    w.field_u64("zones_razed", entry.economics.zones_razed);
    w.field_str("growth_per_year", &entry.economics.growth_per_year);
    w.close();
    w.key("famine");
    w.array(entry.famine.iter().cloned());
    if let Some(request) = &entry.trade_request {
        w.field_str("trade_request", request);
    }
    w.close();
    w.finish()
}

/// Reflect one world's journal (08 §3): append the years the file
/// does not yet carry to `<world>/journal.jsonl` — idempotent (a
/// re-record appends NOTHING; a deleted file regenerates
/// byte-identically, 08 §1.1) — then regenerate the wall. Returns
/// how many years were appended. (Moved from `Ledger::record_journal`
/// at the inversion — a scribe function over the world-dir paths.)
pub fn record_journal(
    book: &Ledger,
    world_name: &str,
    world: &World,
    voice: &Voice,
) -> std::io::Result<usize> {
    let entries = journal(world_name, world, voice, YEAR_TICKS);
    let dir = book.world_dir(world_name);
        fs::create_dir_all(&dir)?;
        let path = dir.join("journal.jsonl");
    let already = match path.exists() {
        true => fs::read_to_string(&path)?.lines().count(),
        false => 0,
    };
    let mut appended = 0usize;
    if entries.len() > already {
        let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
        for entry in &entries[already..] {
            let mut line = render_entry(entry);
            line.push('\n');
            file.write_all(line.as_bytes())?;
            appended += 1;
        }
    }
    render_wall(book)?;
    Ok(appended)
}

/// Regenerate the Terminal Wall (08 §3): one static, self-contained
/// page over every world's journal.jsonl — inline CSS/JS only, no
/// external requests; content is static, animation is cosmetic.
/// Atomic replace, like every lens card. (Moved from
/// `Ledger::render_wall` at the inversion.)
pub fn render_wall(book: &Ledger) -> std::io::Result<()> {
    let mut cards: Vec<WallCard> = Vec::new();
    if let Ok(entries) = fs::read_dir(book.root()) {
            let mut world_dirs: Vec<_> = entries
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.is_dir())
                .collect();
            world_dirs.sort();
            for dir in world_dirs {
                let Ok(text) = fs::read_to_string(dir.join("journal.jsonl")) else {
                    continue;
                };
                for line in text.lines() {
                    if let Some(card) = parse_wall_card(line) {
                        cards.push(card);
                    }
                }
            }
        }
    // Newest first; ties break by world then civilization so the
    // wall is a pure function of the journals (deterministic).
    cards.sort_by(|a, b| {
        b.tick
            .cmp(&a.tick)
            .then_with(|| a.world.cmp(&b.world))
            .then_with(|| a.civilization.cmp(&b.civilization))
    });
    let html = wall_html(&cards);
    fs::create_dir_all(book.root())?;
    let tmp = book.root().join("wall.html.tmp");
    fs::write(&tmp, html)?;
    fs::rename(&tmp, book.root().join("wall.html"))
}

/// One wall card — the display fields of one journal line.
struct WallCard {
    tick: u64,
    year: u64,
    cycle: String,
    world: String,
    civilization: String,
    archetype: String,
    alert: String,
    headline: String,
    body: String,
    trade_request: Option<String>,
}

fn parse_wall_card(line: &str) -> Option<WallCard> {
    Some(WallCard {
        tick: ledger::extract::extract_u64(line, "tick")?,
        year: ledger::extract::extract_u64(line, "year")?,
        cycle: ledger::extract::extract_string(line, "cycle")?,
        world: ledger::extract::extract_string(line, "world")?,
        civilization: ledger::extract::extract_string(line, "civilization")?,
        archetype: ledger::extract::extract_string(line, "archetype")?,
        alert: ledger::extract::extract_string(line, "alert")?,
        headline: ledger::extract::extract_string(line, "headline")?,
        body: ledger::extract::extract_string(line, "body")?,
        trade_request: ledger::extract::extract_string(line, "trade_request"),
    })
}

/// Escape text for HTML — the wall shows journal text verbatim.
fn html_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    out
}

/// The Terminal Wall page: cyber-noir, single-column, newest-first,
/// with a client-side filter bar (world / archetype / alert). Every
/// asset is inline; nothing is fetched.
fn wall_html(cards: &[WallCard]) -> String {
    let mut worlds: Vec<&str> = Vec::new();
    let mut archetypes: Vec<&str> = Vec::new();
    let mut alerts: Vec<&str> = Vec::new();
    for card in cards {
        if !worlds.contains(&card.world.as_str()) {
            worlds.push(&card.world);
        }
        if !archetypes.contains(&card.archetype.as_str()) {
            archetypes.push(&card.archetype);
        }
        if !alerts.contains(&card.alert.as_str()) {
            alerts.push(&card.alert);
        }
    }
    worlds.sort();
    archetypes.sort();
    alerts.sort();
    let options = |items: &[&str]| -> String {
        items
            .iter()
            .map(|item| format!("<option value=\"{0}\">{0}</option>", html_escape(item)))
            .collect::<String>()
    };

    let mut page = String::new();
    page.push_str("<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n");
    page.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    page.push_str("<title>THE TERMINAL WALL — the chronicle feed</title>\n<style>\n");
    page.push_str(
        "*{box-sizing:border-box;margin:0;padding:0}\n\
         body{background:#0a0a10;color:#00ffbf;font-family:monospace;padding:1.5rem 0.75rem}\n\
         .wall{max-width:44rem;margin:0 auto}\n\
         h1{color:#00f3ff;letter-spacing:0.25em;text-align:center;font-size:1.1rem;\
            text-shadow:0 0 8px #00f3ff;animation:hum 4s ease-in-out infinite alternate}\n\
         @keyframes hum{from{opacity:0.82}to{opacity:1}}\n\
         .filters{display:flex;gap:0.5rem;flex-wrap:wrap;justify-content:center;margin:1rem 0}\n\
         .filters select{background:#0a0a10;color:#00f3ff;border:1px solid #00f3ff;\
            font-family:monospace;padding:0.25rem 0.5rem}\n\
         .card{border:1px solid #00f3ff44;border-left:3px solid #00f3ff;\
            margin:1rem 0;padding:0.75rem 1rem;background:#0d0d16}\n\
         .card.warning{border-left-color:#ff0055}\n\
         .card.warning .headline{color:#ff0055;text-shadow:0 0 6px #ff005588}\n\
         .card.milestone .headline{color:#00f3ff}\n\
         .meta{color:#00f3ff;font-size:0.75rem;letter-spacing:0.08em}\n\
         .headline{color:#00ffbf;font-weight:bold;margin:0.35rem 0;text-transform:uppercase}\n\
         .body{white-space:pre-line;font-size:0.85rem;line-height:1.5}\n\
         .body .red{color:#ff0055}\n\
         .trade{color:#00f3ff;font-size:0.8rem;margin-top:0.5rem}\n\
         .empty{text-align:center;color:#00f3ff;opacity:0.6;margin-top:3rem}\n",
    );
    page.push_str("</style>\n</head>\n<body>\n<div class=\"wall\">\n");
    page.push_str("<h1>THE TERMINAL WALL</h1>\n");
    page.push_str("<div class=\"filters\">\n");
    page.push_str(&format!(
        "<select id=\"f-world\"><option value=\"\">every world</option>{}</select>\n",
        options(&worlds)
    ));
    page.push_str(&format!(
        "<select id=\"f-archetype\"><option value=\"\">every archetype</option>{}</select>\n",
        options(&archetypes)
    ));
    page.push_str(&format!(
        "<select id=\"f-alert\"><option value=\"\">every alert</option>{}</select>\n",
        options(&alerts)
    ));
    page.push_str("</div>\n<div id=\"feed\">\n");
    if cards.is_empty() {
        page.push_str("<div class=\"empty\">no journals yet — the worlds are young</div>\n");
    }
    for card in cards {
        let body = card
            .body
            .lines()
            .map(|line| match line.starts_with("RED") {
                true => format!("<span class=\"red\">{}</span>", html_escape(line)),
                false => html_escape(line),
            })
            .collect::<Vec<_>>()
            .join("\n");
        let trade = match &card.trade_request {
            Some(request) => format!(
                "<div class=\"trade\">TRADE » {}</div>",
                html_escape(request)
            ),
            None => String::new(),
        };
        page.push_str(&format!(
            "<div class=\"card {alert}\" data-world=\"{world}\" data-archetype=\"{archetype}\" data-alert=\"{alert}\">\n\
             <div class=\"meta\">{world} · {civilization} · {archetype} · year {year} · cycle {cycle} · {alert}</div>\n\
             <div class=\"headline\">{headline}</div>\n\
             <div class=\"body\">{body}</div>\n{trade}</div>\n",
            alert = html_escape(&card.alert),
            world = html_escape(&card.world),
            archetype = html_escape(&card.archetype),
            civilization = html_escape(&card.civilization),
            year = card.year,
            cycle = html_escape(&card.cycle),
            headline = html_escape(&card.headline),
        ));
    }
    page.push_str("</div>\n</div>\n<script>\n");
    page.push_str(
        "for (const id of ['f-world','f-archetype','f-alert']) {\n\
           document.getElementById(id).addEventListener('change', apply);\n\
         }\n\
         function apply() {\n\
           const world = document.getElementById('f-world').value;\n\
           const archetype = document.getElementById('f-archetype').value;\n\
           const alert = document.getElementById('f-alert').value;\n\
           for (const card of document.querySelectorAll('.card')) {\n\
             const show = (!world || card.dataset.world === world)\n\
               && (!archetype || card.dataset.archetype === archetype)\n\
               && (!alert || card.dataset.alert === alert);\n\
             card.style.display = show ? '' : 'none';\n\
           }\n\
         }\n",
    );
    page.push_str("</script>\n</body>\n</html>\n");
    page
}
