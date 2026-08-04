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
/// **SIX LINES ARE HELD OUT, pending an operator ruling** — Lyran,
/// Anunnaki, Archon, Agarthan, Draco and Pleiadian. They are not
/// declined on taste; they fail two standing laws at once. The operator
/// ordered the occult out of the build (and struck `Anunnaki` from
/// `18_TORSION` by name); and `06_CIVILIZATIONS` §2a's LOCALITY-ANCHORED
/// NAMING law requires a name to carry its PLACE, MEDIUM and
/// MANIFESTATION — which contactee and gnostic names do not, being
/// mythological references rather than descriptions of anywhere.
/// Compiling them into the binary would have moved what was struck from
/// a data file INTO the source, so they wait for a name instead.
///
/// The cost is stated rather than hidden: those six held five of the
/// eight station seats, so the **observer band is EMPTY** until one is
/// renamed or a lawful line takes the seat.
pub fn founding_polities() -> Vec<PolitySpec> {
    let g_half = || Some((StarClass::G, ratio(1, 2), ratio(1, 2)));
    vec![
        // ── THE PROGENITOR BAND (T0) ──────────────────────────────────
        // **THE FIFTH SEAT** (`12_UNION` §2, operator ruling 2026-07-30):
        // *the tardigrade line sits in the progenitor band from this
        // aeon — Tunborn, the Slagborn line's OWN head.* It is compiled
        // now, so no boot path can lose it again.
        seat(
            "Tunborn", "tunborn", "tardigrade progenitor", "tardigrade",
            "the dark between stars is a bed, not a wall",
            "expansionist", "mineral", "planar",
            Some("progenitor"), &[],
            "tun-reach", "husk", "first-wake", Policy::Steady,
            (ratio(2, 1), ratio(-3, 1), ratio(1, 1)), g_half(),
        ),
        seat(
            "Coronal Choir", "coronal-choir", "current sheet", "plasmoid",
            "we are the weather of the star",
            "expansionist", "plasmic", "harmonic",
            Some("progenitor"), &[],
            "corona-sea", "prominence", "current-sheet", Policy::Steady,
            (ratio(15, 1), ratio(15, 1), ratio(-6, 1)), None,
        ),
        // ── THE WATCHER BAND (T1) ─────────────────────────────────────
        seat(
            "Coralline", "coralline", "reef polyp", "cnidarian", "we accrete, therefore we are",
            "epicurean", "biotic", "harmonic",
            Some("watcher"), &[],
            "coralline-hearth", "coralline-prime", "first-coralline", Policy::Steady,
            (ratio(12, 1), ratio(0, 1), ratio(0, 1)), None,
        ),
        // ── THE UNSEATED LINES (T3 — `tier_of(None)`) ─────────────────
        // The line RESOLVES to its head: without this lineage the law has
        // nothing to read and Slagborn is a tardigrade line with no origin.
        seat(
            "Slagborn", "slagborn", "tardigrade", "tardigrade", "we eat what worlds discard",
            "expansionist", "mineral", "planar",
            None, &["tunborn"],
            "cinder-reach", "bask", "first-vent", Policy::Aggressive,
            (ratio(0, 1), ratio(0, 1), ratio(0, 1)), g_half(),
        ),
        seat(
            "Echowright", "deep-chorus", "blue whale", "cetacean", "we heard it before you saw it",
            "epicurean", "biotic", "harmonic",
            None, &[],
            "kepler-passage", "tidefall", "songline", Policy::Steady,
            (ratio(12, 1), ratio(5, 1), ratio(3, 1)), None,
        ),
        seat(
            "Vitreous Hex", "hivelight", "honeybee", "hymenopteran", "the comb computes",
            "empiricist", "biotic", "volumetric",
            None, &[],
            "amber-veil", "comb", "waggle-reach", Policy::Industry,
            (ratio(-7, 1), ratio(9, 1), ratio(2, 1)), None,
        ),
        seat(
            "Brine Cortex", "eightfold", "octopus", "cephalopod", "eight arms, one argument",
            "epicurean", "biotic", "volumetric",
            None, &[],
            "jovian-reach", "europa", "brine-garden", Policy::Steady,
            (ratio(5, 1), ratio(58, 1), ratio(61, 1)), None,
        ),
        // BOT-THERM: the Frost seat — an M sun at unit orbit under zero
        // albedo lands in the cryogenic vault (05 §1.2): sink banks,
        // joules are life, and the freeze-lock is honestly on the table.
        seat(
            "Frostwright", "frostwright", "wood frog", "amphibian",
            "we bank what the fire cannot hold",
            "empiricist", "biotic", "planar",
            None, &[],
            "dimlight-hold", "rime", "still-yard", Policy::Steady,
            (ratio(-20, 1), ratio(-11, 1), ratio(44, 1)),
            Some((StarClass::M, ratio(0, 1), ratio(1, 1))),
        ),
        seat(
            "Mycelial Weft", "mycelial-weft", "fungal mycelium", "fungal",
            "the network under the soil holds the forest's memory",
            "empiricist", "biotic", "volumetric",
            None, &[],
            "understory", "loam", "hyphal-reach", Policy::Industry,
            (ratio(3, 1), ratio(-4, 1), ratio(11, 1)), None,
        ),
        seat(
            "Syzygy", "syzygy", "alignment dweller", "syzygial", "we exist where the voices agree",
            "epicurean", "biotic", "umbral",
            None, &[],
            "alignment", "penumbra", "conjunction", Policy::Steady,
            (ratio(-30, 1), ratio(22, 1), ratio(-17, 1)),
            Some((StarClass::G, ratio(1, 1), ratio(1, 1))),
        ),
        seat(
            "Deepmat", "deepmat", "endolith mat", "endolithic",
            "the sky is a rumor we have not confirmed",
            "empiricist", "biotic", "planar",
            None, &[],
            "deepmat-veil", "underdeck", "no-sky", Policy::Steady,
            (ratio(40, 1), ratio(-2, 1), ratio(9, 1)),
            Some((StarClass::G, ratio(1, 1), ratio(1, 1))),
        ),
        seat(
            "Klethaxu", "klethaxu", "ant", "hymenopteran", "the trail is the thought",
            "expansionist", "biotic", "planar",
            None, &[],
            "klethax-warren", "duff", "first-trail", Policy::Industry,
            (ratio(33, 1), ratio(-21, 1), ratio(6, 1)), None,
        ),
        seat(
            "Darlok", "darlok", "veil-native, form unfixed", "metashifting", "you have already met us",
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
        });
    }
    grown
}

/// **THE ROSTER** — the compiled founding lines, then every registered
/// line the operator has raised since. One call, one truth, no file.
pub fn roster(registry: &ledger::registry::Registry) -> Vec<PolitySpec> {
    let mut all = founding_polities();
    all.extend(registered_polities(registry));
    all
}
