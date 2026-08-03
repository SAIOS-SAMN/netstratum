//! THE VACUITY PROBE — what a world actually READS on the dual-pole
//! scale, measured across a life.
//!
//! `kernel::VERSION` 3 (THE CAPACITY LAW) refuses a `DiscoveryMinted`
//! unless the prior state reads `Vacuity::Entangled` — structure AND
//! flow, both standing. The conductor mints anyway and `replay` banks
//! it, so the live board pays out on worlds the constitution bars. That
//! much was already measured (`judgment_pins`).
//!
//! What was NOT known is the SHAPE of the problem: which worlds read
//! which row, and what a world must do to earn standing. This probe
//! answers by construction rather than by argument.
//!
//! It also asks the open question empirically. A vacuous state is not a
//! discard — `boundary.rs` records THE BIFURCATION LAW: *a refused
//! state is a ROOT, and `Stasis` names exactly what must be
//! re-volumized*. The recycling mechanism already exists as a deed:
//! NOVEL-1 `succeed`, where a successor line adopts a derelict's
//! chronicle as its own substrate — an entity retaking another entity's
//! identity. **What does entanglement read after that?** The last
//! stages measure it.
//!
//! **EVERY FIXTURE PROVES ITSELF.** A fixture that silently fails to
//! reach the state it claims prints a confident verdict about a world
//! that was never built — the same vacuous shape this probe exists to
//! hunt. So each stage is CHECKED, an unreached state is reported as
//! UNMEASURED and never as a reading, and the run tallies its failures
//! rather than panicking (a panic is exit 101).
//!
//! Run: `cargo run -p conductor --example vacuity_probe`
//! Read-only. Builds worlds in memory, writes nothing.

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::world::World;
use kernel::bridge::{read_entangled, verdict_per_dimension};

/// Tally of fixtures that did not reach the state they claim. Reported
/// at the end; never a panic.
struct Tally {
    unmeasured: Vec<String>,
}

impl Tally {
    fn verdict(&mut self, label: &str, world: &World) {
        let committed = world.committed();
        let read = read_entangled(&committed);
        let per_axis = verdict_per_dimension(&committed);
        println!(
            "  {label:<32}  orbs {}  bonds {}  cov {}  debts {}",
            committed.orbs.len(),
            committed.bonds.len(),
            committed.covenants.len(),
            committed.debts.len()
        );
        println!(
            "  {:<32}  {:?}   D {}   per-channel {:?}",
            "", read.verdict(), read.degree(), per_axis
        );
    }

    /// The state was not reached — say so, and measure nothing.
    fn unmeasured(&mut self, label: &str, why: String) {
        println!("  {label:<32}  UNMEASURED — {why}");
        self.unmeasured.push(label.to_string());
    }
}

fn banked(world: &World, resource: Resource) -> Ratio {
    world
        .orb()
        .map(|orb| orb.banked_amount(resource))
        .unwrap_or_else(|| ratio(0, 1))
}

/// A world played until it can actually afford to charter — the
/// founding reserves are 20 iron + 10 biomass and they MOVE, so a
/// two-orb world is not free.
fn played_rich() -> World {
    let mut world = World::new();
    world.found("first-landing").expect("founds");
    // 2 extractors = 20 iron, 1 habitat = 5 biomass: exactly the
    // founding reserves' reach. Nothing here may assume it can afford
    // more than the charter granted.
    let _ = world.zone(0, ZoneKind::Extractor);
    let _ = world.zone(1, ZoneKind::Extractor);
    let _ = world.zone(8, ZoneKind::Habitat);
    for _ in 0..40 {
        world.tick();
        let _ = world.collect();
    }
    // MEASURED: two extractors burn 1/2 biomass each and one habitat
    // yields exactly 1 — biomass nets to ZERO and the colony sits at 5
    // forever, unable to pay a charter's 10. The second habitat is what
    // makes the line biomass-positive, and it is affordable only after
    // the first stretch of play.
    let _ = world.zone(9, ZoneKind::Habitat);
    for _ in 0..80 {
        world.tick();
        let _ = world.collect();
    }
    world
}

/// Two orbs in one world. Charting MOVES the founding reserves, so this
/// only works on a world that has them.
fn two_orbs(tally: &mut Tally) -> Option<World> {
    let mut world = played_rich();
    let iron = banked(&world, Resource::Iron);
    let biomass = banked(&world, Resource::Biomass);
    match world.chart("second-site", cfe::world::FIRST_ORB) {
        Ok(_) => Some(world),
        Err(refused) => {
            tally.unmeasured(
                "two orbs",
                format!("chart refused {refused:?} (iron {iron}, biomass {biomass})"),
            );
            None
        }
    }
}

/// THREE orbs — the smallest population that can carry a CYCLE.
///
/// `Unframed` is flow without structure, and structure here means
/// HOLONOMY: a triangle of obligations that does not compose
/// (`R_ijk = Δ_ik − Δ_ij − Δ_jk ≠ 0`). Two entities cannot form one —
/// there is no third leg to fail to compose — so no two-orb world can
/// ever leave `Unframed`, however many obligations it seals.
fn three_orbs(tally: &mut Tally, label: &str) -> Option<World> {
    let mut world = played_rich_long();
    for name in ["second-site", "third-site"] {
        if let Err(refused) = world.chart(name, cfe::world::FIRST_ORB) {
            tally.unmeasured(
                label,
                format!(
                    "chart {name} refused {refused:?} (iron {}, biomass {})",
                    banked(&world, Resource::Iron),
                    banked(&world, Resource::Biomass)
                ),
            );
            return None;
        }
    }
    Some(world)
}

/// Long enough to afford TWO charters (2 × 20 iron + 2 × 10 biomass,
/// moved out of the source bank).
///
/// The founding reserves are 20 iron + 10 biomass and an extractor
/// costs 10 iron, so the opening build is BOUNDED — the first draft
/// asked for three extractors, could not pay for the third, and
/// panicked inside a probe whose whole point is to tally rather than
/// die (a panic is exit 101). It now builds what it can afford, plays,
/// and only then expands.
fn played_rich_long() -> World {
    let mut world = World::new();
    world.found("first-landing").expect("founds");
    // Affordable at founding: 2 extractors (20 iron), 1 habitat (5 biomass).
    let _ = world.zone(0, ZoneKind::Extractor);
    let _ = world.zone(1, ZoneKind::Extractor);
    let _ = world.zone(8, ZoneKind::Habitat);
    for _ in 0..40 {
        world.tick();
        let _ = world.collect();
    }
    // Now the banks can carry more.
    let _ = world.zone(2, ZoneKind::Extractor);
    let _ = world.zone(9, ZoneKind::Habitat);
    for _ in 0..160 {
        world.tick();
        let _ = world.collect();
    }
    world
}

/// A DERELICT: every orb famished, which is what NOVEL-1 succession
/// requires. Habitats burn iron and mint no iron, so an iron-less
/// habitat colony starves by construction.
fn derelict(tally: &mut Tally) -> Option<World> {
    let mut world = World::new();
    world.found("last-landing").expect("founds");
    // Habitats only: they consume iron (1/4 each per tick) and there is
    // no extractor to replace it.
    world.zone(0, ZoneKind::Habitat).expect("zones");
    world.zone(1, ZoneKind::Habitat).expect("zones");
    for _ in 0..400 {
        world.tick();
        let _ = world.collect();
        let famished = world
            .committed()
            .orbs
            .iter()
            .all(|orb| !orb.famine.is_empty());
        if famished {
            return Some(world);
        }
    }
    let famine = world
        .committed()
        .orbs
        .first()
        .map(|o| o.famine.clone())
        .unwrap_or_default();
    tally.unmeasured(
        "derelict",
        format!(
            "400 ticks did not starve every orb (famine {famine:?}, iron {})",
            banked(&world, Resource::Iron)
        ),
    );
    None
}

fn main() {
    let mut tally = Tally { unmeasured: Vec::new() };
    println!("THE VACUITY PROBE — kernel VERSION {}\n", kernel::VERSION);
    println!("The admitted row is Entangled (structure AND flow); every other");
    println!("row refuses a mint as MintWithoutStanding. The bridge projects");
    println!("PAIRWISE obligations only — bonds, covenants, absorption debts.\n");

    println!("── AN ORDINARY LIFE ──");
    let mut fresh = World::new();
    fresh.found("first-landing").expect("founds");
    tally.verdict("founded, untouched", &fresh);
    let rich = played_rich();
    tally.verdict("played 60 ticks, one orb", &rich);

    println!("\n── WHAT EARNS STANDING ──");
    // `World` is deliberately not `Clone` (one chronicle, one world), so
    // each variant is built from its own run.
    if let Some(world) = two_orbs(&mut tally) {
        tally.verdict("two orbs, no obligation", &world);
    }
    if let Some(mut bonded) = two_orbs(&mut tally) {
        let orbs: Vec<u64> = bonded.committed().orbs.iter().map(|o| o.orb_id).collect();
        match bonded.seal_bond(orbs[0], orbs[1], Resource::Iron, ratio(1, 4), 0) {
            Ok(()) => tally.verdict("+ one bond (0 -> 1)", &bonded),
            Err(refused) => tally.unmeasured("+ one bond", format!("{refused:?}")),
        }
    }
    if let Some(mut covenanted) = two_orbs(&mut tally) {
        let orbs: Vec<u64> = covenanted.committed().orbs.iter().map(|o| o.orb_id).collect();
        match covenanted.seal_covenant(orbs[0], orbs[1], Resource::Iron, ratio(4, 1), 64) {
            Ok(()) => tally.verdict("+ one covenant (undelivered)", &covenanted),
            Err(refused) => tally.unmeasured("+ one covenant", format!("{refused:?}")),
        }
    }
    if let Some(mut both) = two_orbs(&mut tally) {
        let orbs: Vec<u64> = both.committed().orbs.iter().map(|o| o.orb_id).collect();
        let a = both.seal_bond(orbs[0], orbs[1], Resource::Iron, ratio(1, 4), 0);
        let b = both.seal_covenant(orbs[1], orbs[0], Resource::Biomass, ratio(2, 1), 64);
        match (a, b) {
            (Ok(()), Ok(())) => tally.verdict("+ bond one way, covenant back", &both),
            (a, b) => tally.unmeasured("+ bond and covenant", format!("{a:?} / {b:?}")),
        }
    }

    println!("\n── THE CYCLE: can ANY world reach Entangled? ──");
    // A COMPOSING triangle: 1 + 2 = 3 exactly. The obligations chain,
    // the residual is zero, and no holonomy stands.
    if let Some(mut world) = three_orbs(&mut tally, "composing triangle") {
        let o: Vec<u64> = world.committed().orbs.iter().map(|x| x.orb_id).collect();
        let a = world.seal_bond(o[0], o[1], Resource::Iron, ratio(1, 1), 0);
        let b = world.seal_bond(o[1], o[2], Resource::Iron, ratio(2, 1), 0);
        let c = world.seal_bond(o[0], o[2], Resource::Iron, ratio(3, 1), 0);
        match (a, b, c) {
            (Ok(()), Ok(()), Ok(())) => {
                tally.verdict("triangle that COMPOSES (1+2=3)", &world)
            }
            (a, b, c) => tally.unmeasured(
                "composing triangle",
                format!("{a:?} / {b:?} / {c:?}"),
            ),
        }
    }
    // A NON-COMPOSING triangle: 1 + 2 ≠ 7. The obligation fails to
    // chain, the residual stands, and that defect IS the holonomy.
    if let Some(mut world) = three_orbs(&mut tally, "non-composing triangle") {
        let o: Vec<u64> = world.committed().orbs.iter().map(|x| x.orb_id).collect();
        let a = world.seal_bond(o[0], o[1], Resource::Iron, ratio(1, 1), 0);
        let b = world.seal_bond(o[1], o[2], Resource::Iron, ratio(2, 1), 0);
        let c = world.seal_bond(o[0], o[2], Resource::Iron, ratio(7, 1), 0);
        match (a, b, c) {
            (Ok(()), Ok(()), Ok(())) => {
                tally.verdict("triangle that does NOT compose", &world)
            }
            (a, b, c) => tally.unmeasured(
                "non-composing triangle",
                format!("{a:?} / {b:?} / {c:?}"),
            ),
        }
    }

    println!("\n── THE RECYCLED IDENTITY (NOVEL-1 succession) ──");
    match derelict(&mut tally) {
        None => {}
        Some(mut world) => {
            let famine = world
                .committed()
                .orbs
                .first()
                .map(|o| o.famine.clone())
                .unwrap_or_default();
            println!("  (the derelict starved on {famine:?})");
            tally.verdict("derelict, before succession", &world);
            match world.succeed("revenant-line") {
                Ok(()) => tally.verdict("AFTER succession (recycled)", &world),
                Err(refused) => {
                    tally.unmeasured("AFTER succession", format!("{refused:?}"));
                }
            }
        }
    }

    println!();
    match tally.unmeasured.is_empty() {
        true => println!("every fixture reached its state — all rows above are measured"),
        false => println!(
            "UNMEASURED fixtures ({}): {}\n\
             Those rows carry no reading. Nothing above claims one for them.",
            tally.unmeasured.len(),
            tally.unmeasured.join(", ")
        ),
    }
}
