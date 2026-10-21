//! **THE DEFINITION LAW** (lens-lane ruling, 2026-08-05, on operator
//! order: *we can't have JSON files hanging around our Rust codebase —
//! make a ruling on your lane*).
//!
//! > **A definition COMPILES. Only a receipt PERSISTS.**
//! > No file may say what EXISTS; a file may only say what HAPPENED.
//!
//! This module used to open with the opposite creed — *"lore lives in
//! data, not source"* — and seeded `<var>/polities.json` for the
//! operator to hand-edit. That is struck. The roster is source now, and
//! the roster is the ONLY roster.
//!
//! **Why, in four measured parts.**
//!
//! 1. **The two truths already drifted, and it cost a landed ruling.**
//!    `12_UNION` §2 seated the tardigrade line in the progenitor band.
//!    The seed file carried it; these built-in defaults did not; and a
//!    rebuilt `var` boots the defaults, because `write_default_polities`
//!    refused to overwrite. So the fallback was not a rare path — it was
//!    the path, and a board on it ran five lines, all `station: None`,
//!    all T3, with the whole progenitor band structurally absent. The
//!    previous revision of this file DOCUMENTED that in a comment and
//!    left both copies standing. Two truths do not stay equal; they take
//!    turns being wrong.
//!
//! 2. **§11 rule 2 already forbade it.** *Views derive from the chain,
//!    disposable, never read back* (`SYSTEM_SPEC` §11, `19_LENS` §15a).
//!    `recurrence_guard.rs` enforces it with an adopted debt list that
//!    "never grows" — and `polities.rs` was ON that list. This is not a
//!    new law; it is an unpaid one. The list SHRINKS by one here, which
//!    is the only direction it is allowed to move.
//!
//! 3. **The roster is consensus-relevant, so a file cannot hold it.**
//!    Which lines exist decides which worlds emit receipts, which decides
//!    what mints. Two nodes with different `polities.json` derive
//!    different universes from the same chain. THE UPLINK LAW already
//!    says nothing file-borne rides the mesh; a definition outside the
//!    chain is that violation waiting for a second node to exist.
//!
//! 4. **The parser failed silently, in geometry.** The struck loader
//!    brace-counted objects ("strings hold no braces" — a motto with a
//!    `{` broke it), found keys with `text.find("\"name\"")` (which
//!    matches INSIDE a value), and defaulted a missing `x`/`y`/`z` to
//!    `ratio(0, 1)`. A typo'd coordinate key did not refuse — it sited
//!    the world at the ORIGIN, on top of whatever else was there. A
//!    mistyped star word did not refuse either; it silently un-pinned a
//!    thermal seat and let it derive. None of those can be typed wrong
//!    here, because the compiler reads this file.
//!
//! **What is NOT struck.** Chronicles, spines, receipts and `.tlv`
//! segments stay on disk and stay read at boot — they are what HAPPENED,
//! and a world that forgets is not a world. The law bars definitions in
//! files, never receipts.
//!
//! **How a line is added now.** Through the receipted door: the CLI's
//! `civ new` verb, which emits a receipt for the birth. Editing a JSON
//! roster created a civilization with NO receipt — the definition simply
//! appeared. That door is now closed, which is the point.

use crate::{Policy, PolitySpec};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use ledger::registry::Civilization;

/// One seat in the compiled roster. Every field the struck loader took
/// with `?` is a positional argument here, so an omission is a compile
/// error rather than a skipped entry on stderr.
#[allow(clippy::too_many_arguments)]
fn seat(
    name: &str,
    sigil: &str,
    species: &str,
    clade: &str,
    motto: &str,
    disposition: &str,
    metabolism: &str,
    perception: &str,
    station: Option<&str>,
    lineage: &[&str],
    world_name: &str,
    planet: &str,
    colony: &str,
    policy: Policy,
    position: (Ratio, Ratio, Ratio),
    siting: Option<(StarClass, Ratio, Ratio)>,
) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: name.into(),
            sigil: sigil.into(),
            species: species.into(),
            clade: clade.into(),
            motto: motto.into(),
            disposition: disposition.into(),
            metabolism: metabolism.into(),
            perception: perception.into(),
            founder: "conductor".into(),
            lineage: lineage.iter().map(|s| (*s).into()).collect(),
            station: station.map(|s| s.into()),
        },
        world_name: world_name.into(),
        planet: planet.into(),
        colony: colony.into(),
        policy,
        position,
        // BOT-THERM: a seat is PINNED (star, albedo, orbit together) or
        // DERIVED (all three absent — the orbit falls out at genesis from
        // the registry axes through the chronicle beacon). The struck
        // file shape allowed the incoherent middle, where a star was
        // pinned and its albedo silently defaulted. One `Option` of a
        // triple makes the middle unrepresentable.
        star: siting.as_ref().map(|(class, _, _)| *class),
        albedo: siting.as_ref().map(|(_, albedo, _)| albedo.clone()),
        orbit: siting.map(|(_, _, orbit)| orbit),
        gauge: None,
        composition: None,
        place: None,
    }
}

/// **THE AEON 1 ROSTER — the one roster, compiled.**
///
/// Absorbed from the struck `seeds/aeon1-polities.json` (19 entries),
/// which was neither read by the engine nor kept in step with it. THE
/// LOCALITY-ANCHORED NAMES applied as ruled (`06_CIVILIZATIONS` §2a):
/// Deep Chorus → **Echowright** (operator-ruled), Hivelight → **Vitreous
/// Hex**, Eightfold → **Brine Cortex**, and Slagborn STANDS (its rename
/// was declined). The live `var/registry/*.json` cards already carried
/// those three names while the durable roster did not — the mismatch
/// `ENDOWMENTS.md` inventories. Compiling the roster resolves it: one
/// name, one place.
///
/// **THE CLASS PARTITION** (identity re-key #5, 2026-08-14).
///
/// The roster is keyed to the game's own SCI-FI taxonomy — substrate
/// (biotic · mineral · plasmic · element-eater), clade (the biological
/// form), thermal band, and tier — never to Earth myth or esoterica.
/// Names that once carried occult/contactee baggage were retired for
/// coined netstratum-native ones; the STRUCTURE they anchor is unchanged.
///
/// Six of the founding lines are not decoration — they ARE the taxon
/// partition (`15_TAXON` §2): class 7 (apex-solitary) is seated by
/// Sylvane and Forgeborn and by nobody else, so the class stands or
/// falls with them; and clade 19 `draconic` (the 2026-08-01 ruling —
/// *one archetype, two expressions*) rides on Umbrid, the saurian
/// expression of the class-4 archetype Stillcold also wears. Each line's
/// identity is its substrate and clade; the myth is gone, the physics
/// stays.
pub fn founding_polities() -> Vec<PolitySpec> {
    let g_half = || Some((StarClass::G, ratio(1, 2), ratio(1, 2)));
    vec![
        // ── THE ELDEST LINES (once the progenitor band) ───────────────
        // No line is seated a progenitor or a sovereign at founding
        // (operator 2026-10-21, "realms EMERGE, not drawn"): a progenitor
        // is the line whose lineage folds a gate (`ascension`), a sovereign
        // the line whose lineage holds its own star system. Tunborn is the
        // Slagborn line's own head (`12_UNION` §2, 2026-07-30).
        seat(
            "Tunborn", "tunborn", "hypobiotid-voidsleeper", "hypobiotid",
            "the dark between stars is a bed, not a wall",
            "expansionist", "mineral", "planar",
            None, &[],
            "tun-reach", "husk", "first-wake", Policy::Steady,
            (ratio(2, 1), ratio(-3, 1), ratio(1, 1)), g_half(),
        ),
        // CLASS 7 — apex-solitary. Seated by these two and no others,
        // so the class stands or falls with them (`15_TAXON` §2 / 37).
        seat(
            "Sylvane", "sylvane", "ailuroform-primus", "ailuroform",
            "we were first, and we remember why that matters less than you think",
            "epicurean", "biotic", "harmonic",
            None, &[],
            "sylvane-hearth", "sunwell", "first-hearth", Policy::Steady,
            (ratio(1, 1), ratio(1, 1), ratio(1, 1)), None,
        ),
        seat(
            // FLATLANDER BASELINE seat (clade bimaniform) — knowledge baseline
            // for design; extract+regen both natural (EconomicMode::Balanced).
            "Forgeborn", "forgeborn", "bimaniform-flatlander", "bimaniform",
            "we plan in stone and fire; we no longer dig",
            "expansionist", "mineral", "planar",
            None, &[],
            "forge-reach", "anvil-spire", "deepforge", Policy::Industry,
            (ratio(8, 1), ratio(3, 1), ratio(5, 1)), g_half(),
        ),
        // CLASS 2 — geodic-lithovore, beside Slagborn and Tunborn.
        seat(
            "Corebound", "corebound", "lithophage-stonewill", "lithophage",
            "the surface is weather; we are the world",
            "epicurean", "mineral", "volumetric",
            None, &[],
            "core-deep", "coreglow", "stone-hollow", Policy::Steady,
            (ratio(0, 1), ratio(-13, 1), ratio(7, 1)),
            Some((StarClass::M, ratio(1, 2), ratio(2, 1))),
        ),
        seat(
            "Coronal Choir", "coronal-choir", "plasmiform-sheet", "plasmiform",
            "we are the weather of the star",
            "expansionist", "plasmic", "harmonic",
            None, &[],
            "corona-sea", "prominence", "current-sheet", Policy::Steady,
            (ratio(15, 1), ratio(15, 1), ratio(-6, 1)), None,
        ),
        // ── THE WATCHER BAND (T1) ─────────────────────────────────────
        // CLASS 4 — telepathic-resonant. Stillcold and Umbrid are ONE
        // archetype at two clades (operator ruling 2026-08-01): class is
        // the shared identity, clade is how it is expressed, and any
        // mechanic keyed on class sees them as one line.
        seat(
            "Stillcold", "stillcold", "cryocoherens-still", "cryocoherens",
            "in the perfect cold, everything can be known",
            "empiricist", "biotic", "volumetric",
            Some("watcher"), &[],
            "cryo-vault", "stillspire", "still-gate", Policy::Steady,
            (ratio(-9, 1), ratio(14, 1), ratio(2, 1)),
            Some((StarClass::M, ratio(0, 1), ratio(1, 1))),
        ),
        seat(
            "Thresholder", "thresholder", "bimaniform-septem", "bimaniform",
            "we hold the threshold and do not descend",
            "empiricist", "biotic", "harmonic",
            Some("watcher"), &[],
            "sevenwatch", "threshold-spire", "threshold-watch", Policy::Steady,
            (ratio(21, 1), ratio(-6, 1), ratio(13, 1)),
            Some((StarClass::B, ratio(1, 4), ratio(1, 1))),
        ),
        seat(
            "Coralline", "coralline", "cnidiform-accrete", "cnidiform", "we accrete, therefore we are",
            "epicurean", "biotic", "harmonic",
            Some("watcher"), &[],
            "coralline-hearth", "coralline-prime", "first-coralline", Policy::Steady,
            (ratio(12, 1), ratio(0, 1), ratio(0, 1)), None,
        ),
        // ── THE OBSERVER BAND (T1) ────────────────────────────────────
        // Clade 19 `draconiform` — the saurian sub-archetype the 2026-08-01
        // ruling exists to seat. Same class as Stillcold, a different
        // expression of it.
        seat(
            "Umbrid", "umbrid", "draconiform-umbra", "draconic",
            "the union is the evidence; the seat above re-checks it",
            "epicurean", "biotic", "umbral",
            Some("observer"), &[],
            "umbra-hold", "shadowspire", "first-umbra", Policy::Steady,
            (ratio(-11, 1), ratio(16, 1), ratio(4, 1)),
            Some((StarClass::K, ratio(1, 4), ratio(3, 4))),
        ),
        // ── THE ARCHIVISTS (12_UNION §2a) ──────────────────────────────
        // Stelahold · Tallykeep · Glyphward, the ledger-integrity and
        // calendar-keeping lineages descended from the Forgeborn domain;
        // sovereign only when their lineage holds its own system.
        seat(
            "Stelahold", "stelahold", "bimaniform-flatlander", "bimaniform",
            "what is written in stone outlives the flood",
            "empiricist", "mineral", "planar",
            None, &["forgeborn"],
            "stela-reach", "pale-wall", "first-slab", Policy::Steady,
            (ratio(18, 1), ratio(-8, 1), ratio(4, 1)),
            Some((StarClass::G, ratio(1, 3), ratio(1, 1))),
        ),
        seat(
            "Tallykeep", "tallykeep", "bimaniform-calendar", "bimaniform",
            "the calendar is the spine of the sky",
            "empiricist", "biotic", "harmonic",
            None, &["forgeborn"],
            "tally-span", "sun-crown", "first-tally", Policy::Steady,
            (ratio(-14, 1), ratio(9, 1), ratio(22, 1)),
            Some((StarClass::K, ratio(1, 4), ratio(4, 5))),
        ),
        seat(
            "Glyphward", "glyphward", "bimaniform-scribe", "bimaniform",
            "the record outlives the recorder",
            "empiricist", "biotic", "harmonic",
            None, &["forgeborn"],
            "glyph-sky", "star-gate", "first-word", Policy::Steady,
            (ratio(11, 1), ratio(27, 1), ratio(-5, 1)),
            Some((StarClass::A, ratio(1, 5), ratio(1, 1))),
        ),
        // ── THE UNSEATED LINES (T3 — `tier_of(None)`) ─────────────────
        // Species strings = imperium tongue (37); never Earth common names.
        // Clade field = imperium word (legacy English still parses).
        seat(
            "Slagborn", "slagborn", "hypobiotid-endure", "hypobiotid", "we eat what worlds discard",
            "expansionist", "mineral", "planar",
            None, &["tunborn"],
            "cinder-reach", "bask", "first-vent", Policy::Aggressive,
            (ratio(0, 1), ratio(0, 1), ratio(0, 1)), g_half(),
        ),
        seat(
            "Echowright", "mysticeti-pelagic", "soniform-songline", "soniform", "we heard it before you saw it",
            "epicurean", "biotic", "harmonic",
            None, &[],
            "kepler-passage", "tidefall", "songline", Policy::Steady,
            (ratio(12, 1), ratio(5, 1), ratio(3, 1)), None,
        ),
        seat(
            "Vitreous Hex", "hivelight", "hexasocial-comb", "hexasocial", "the comb computes",
            "empiricist", "biotic", "volumetric",
            None, &[],
            "amber-veil", "comb", "waggle-reach", Policy::Industry,
            (ratio(-7, 1), ratio(9, 1), ratio(2, 1)), None,
        ),
        seat(
            "Brine Cortex", "eightfold", "polybrachia-cortex", "polybrachia", "eight arms, one argument",
            "epicurean", "biotic", "volumetric",
            None, &[],
            "jovian-reach", "europa", "brine-garden", Policy::Steady,
            (ratio(5, 1), ratio(58, 1), ratio(61, 1)), None,
        ),
        // BOT-THERM: the Frost seat — an M sun at unit orbit under zero
        // albedo lands in the cryogenic vault (05 §1.2): sink banks,
        // joules are life, and the freeze-lock is honestly on the table.
        seat(
            "Frostwright", "frostwright", "amphiform-rime", "amphiform",
            "we bank what the fire cannot hold",
            "empiricist", "biotic", "planar",
            None, &[],
            "dimlight-hold", "rime", "still-yard", Policy::Steady,
            (ratio(-20, 1), ratio(-11, 1), ratio(44, 1)),
            Some((StarClass::M, ratio(0, 1), ratio(1, 1))),
        ),
        seat(
            "Mycelial Weft", "mycelial-weft", "mycotiform-hypha", "mycotiform",
            "the network under the soil holds the forest's memory",
            "empiricist", "biotic", "volumetric",
            None, &[],
            "understory", "loam", "hyphal-reach", Policy::Industry,
            (ratio(3, 1), ratio(-4, 1), ratio(11, 1)), None,
        ),
        seat(
            "Syzygy", "syzygy", "syzygiform-conjunction", "syzygiform", "we exist where the voices agree",
            "epicurean", "biotic", "umbral",
            None, &[],
            "alignment", "penumbra", "conjunction", Policy::Steady,
            (ratio(-30, 1), ratio(22, 1), ratio(-17, 1)),
            Some((StarClass::G, ratio(1, 1), ratio(1, 1))),
        ),
        seat(
            "Deepmat", "deepmat", "endolithid-understone", "endolithid",
            "the sky is a rumor we have not confirmed",
            "empiricist", "biotic", "planar",
            None, &[],
            "deepmat-veil", "underdeck", "no-sky", Policy::Steady,
            (ratio(40, 1), ratio(-2, 1), ratio(9, 1)),
            Some((StarClass::G, ratio(1, 1), ratio(1, 1))),
        ),
        seat(
            "Klethaxu", "klethaxu", "hexasocial-trail", "hexasocial", "the trail is the thought",
            "expansionist", "biotic", "planar",
            None, &[],
            "klethax-warren", "duff", "first-trail", Policy::Industry,
            (ratio(33, 1), ratio(-21, 1), ratio(6, 1)), None,
        ),
        seat(
            "Darlok", "darlok", "metamorphid-veil", "metamorphid", "you have already met us",
            "empiricist", "biotic", "umbral",
            None, &[],
            "shroudmere", "nowhere-yet", "first-mask", Policy::Steady,
            (ratio(7, 1), ratio(-19, 1), ratio(11, 1)),
            Some((StarClass::M, ratio(1, 2), ratio(1, 1))),
        ),
    ]
}

/// **THE PLACEMENT** — a registered line's coordinates, derived exactly
/// from its sigil.
///
/// FNV-1a over the sigil bytes, then three coordinates in `-31..=31`.
/// Deliberately NOT a [`physics::Beacon`] draw: the beacon's law is that
/// every draw derives from a receipt digest already in the chronicle
/// (CHARTER §9), and a line being placed has no chronicle yet. Seeding a
/// beacon from a name instead would have bought convenience by widening
/// the one law that makes replay work. This is a PLACEMENT — a pure
/// function of a stable identity, deterministic, exact, integer-only —
/// and it claims nothing about randomness.
fn placement(sigil: &str) -> (i64, i64, i64) {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut axes = [0i64; 3];
    for (i, byte) in sigil.as_bytes().iter().enumerate() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        axes[i % 3] = i64::from((hash >> 24) as u8 % 63) - 31;
    }
    (axes[0], axes[1], axes[2])
}

/// **THE REGISTERED LINES** — every civilization in the registry that
/// the compiled roster does not already name.
///
/// This is what replaces the struck `polities.json` append. `civ new`
/// registers a line and stops there; the registry card is a RECEIPT (a
/// registration HAPPENED), which the definition law admits, and the
/// conductor derives the rest rather than reading it back. The card
/// carries identity — name, axes, station, lineage — and everything
/// spatial falls out of the sigil, so there is nothing left for a file
/// to hold and nothing left to drift.
///
/// **Collisions refuse to overlap.** Two sigils can mix to one placement;
/// a world silently landing on top of another is exactly the failure the
/// struck loader had (a missing `x` defaulted to the origin). Occupied
/// coordinates are stepped along `x` deterministically until free, so
/// the outcome is stable across machines and no two lines share a point.
pub fn registered_polities(registry: &ledger::registry::Registry) -> Vec<PolitySpec> {
    let founding = founding_polities();
    let mut taken: Vec<(i64, i64, i64)> = founding
        .iter()
        .map(|spec| {
            let read = |r: &Ratio| {
                use num_traits::ToPrimitive;
                r.to_integer().to_i64().unwrap_or(0)
            };
            (read(&spec.position.0), read(&spec.position.1), read(&spec.position.2))
        })
        .collect();

    let mut grown = Vec::new();
    let mut cards = registry.list();
    // The registry lists by directory order; sort so the placement walk
    // resolves collisions the same way on every machine.
    cards.sort_by(|a, b| a.sigil.cmp(&b.sigil));
    for card in cards {
        if founding.iter().any(|spec| spec.civilization.sigil == card.sigil) {
            continue;
        }
        // A line seated by a founding act (a cleave) resumes exactly where
        // it was seated: its world, planet, colony, chart point and
        // temperament are a recorded fact, never re-derived.
        if let Some(seat) = registry.founded_at(&card.sigil) {
            grown.push(PolitySpec {
                world_name: seat.world,
                planet: seat.planet,
                colony: seat.colony,
                civilization: card,
                policy: Policy::parse(&seat.policy).unwrap_or(Policy::Steady),
                position: seat.position,
                star: None,
                albedo: None,
                orbit: None,
                gauge: None,
                composition: None,
                place: None,
            });
            continue;
        }
        let (mut x, y, z) = placement(&card.sigil);
        while taken.contains(&(x, y, z)) {
            x += 1;
        }
        taken.push((x, y, z));
        let sigil = card.sigil.clone();
        grown.push(PolitySpec {
            world_name: format!("{sigil}-hearth"),
            planet: format!("{sigil}-prime"),
            colony: format!("first-{sigil}"),
            civilization: card,
            policy: Policy::Steady,
            position: (ratio(x, 1), ratio(y, 1), ratio(z, 1)),
            // Unsited by construction: the orbit derives at genesis from
            // the registry axes through the chronicle beacon, which is
            // the path a card-borne line should walk anyway.
            star: None,
            albedo: None,
            orbit: None,
            gauge: None,
            composition: None,
            place: None,
        });
    }
    grown
}

/// **IS THIS WORLD NAME TAKEN?** A world directory belongs to exactly one
/// line. A name is taken when a compiled seat founds there or any line's seat
/// card records it. Measured 2026-10-08: a cleave named `forge-reach` (head
/// "forge", tail "reach") was seated in Forgeborn's compiled world; the two
/// lines wrote one directory, the scribe read each other's writes as
/// divergence and rotated the directory aside every beat (869 copies, 9.7 GB)
/// until the host ran out of resources.
pub fn world_name_taken(registry: &ledger::registry::Registry, world: &str) -> bool {
    founding_polities().iter().any(|spec| spec.world_name == world)
        || registry.list().iter().any(|card| registry.founded_at(&card.sigil).is_some_and(|seat| seat.world == world))
}

/// **THE COMPILED ROSTER'S DIGEST** (docs/58 §3) — what a universe's Big Bang
/// records of who was seated at its founding: every compiled seat's
/// identity, world and siting, in roster order. A change to any seat is a
/// different founding.
pub fn roster_digest() -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key("netstratum founding roster v1");
    for spec in founding_polities() {
        let c = &spec.civilization;
        for field in [&c.name, &c.sigil, &c.species, &c.clade, &c.disposition, &c.metabolism, &c.perception, &spec.world_name, &spec.planet, &spec.colony] {
            h.update(&(field.len() as u32).to_le_bytes());
            h.update(field.as_bytes());
        }
        h.update(c.station.as_deref().unwrap_or("").as_bytes());
        h.update(spec.policy.describe().as_bytes());
        h.update(format!("{}|{}|{}", spec.position.0, spec.position.1, spec.position.2).as_bytes());
        match (spec.star, &spec.albedo, &spec.orbit) {
            (Some(class), Some(a), Some(o)) => h.update(format!("{class:?}|{a}|{o}").as_bytes()),
            _ => h.update(b"derived"),
        };
    }
    *h.finalize().as_bytes()
}

/// **THE ROSTER** — the compiled founding lines, then every registered
/// line the operator has raised since. One call, one truth, no file.
pub fn roster(registry: &ledger::registry::Registry) -> Vec<PolitySpec> {
    let mut all = founding_polities();
    all.extend(registered_polities(registry));
    // ONE LINE PER WORLD: a spec whose world another line already holds is
    // not seated there (two lines writing one directory is the failure
    // `world_name_taken` now prevents at the cleave); it is reported, loudly.
    let mut held = std::collections::BTreeSet::new();
    all.retain(|spec| {
        if held.insert(spec.world_name.clone()) {
            return true;
        }
        eprintln!(
            "roster: {} is NOT seated — its world {} already belongs to another line",
            spec.civilization.sigil, spec.world_name
        );
        false
    });
    all
}
