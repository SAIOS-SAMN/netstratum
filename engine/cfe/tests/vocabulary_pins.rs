//! **THE BINDING LAW, pinned** (`28_VOCABULARY` §4, V0).
//!
//! A class of entity is a word, a mark and a MECHANISM. These witnesses
//! hold the third: that every seed archetype carries one, that each one
//! trades, that no two are the same trade, and that the mechanism
//! actually reaches the world.

use cfe::ratio::ratio;
use cfe::vocabulary::{
    archetype_binding, binding_trades, canonical, channel_favours_increase, factor_in_bounds,
    factor_on, CHANNELS, CHANNEL_SURVEY, CHANNEL_UPKEEP, CHANNEL_YIELD,
};

/// **EVERY SEED ARCHETYPE CARRIES A LAWFUL BINDING** — one benefit, one
/// friction, every factor in bounds, every channel real.
#[test]
fn every_seed_archetype_binding_is_lawful() {
    for class in 0..=cfe::dop::class_ceiling() {
        let binding = archetype_binding(class);
        assert!(
            binding_trades(&binding),
            "class {class} ({}) carries an unlawful binding: {binding:?}",
            cfe::dop::archetype_word(class)
        );
        for (channel, factor) in &binding {
            assert!(CHANNELS.contains(channel), "class {class} names channel {channel}");
            assert!(factor_in_bounds(factor), "class {class} factor out of bounds");
        }
    }
}

/// **THE UNCLASSED CLAIMS NOTHING** — the empty binding is the ground
/// state, lawful and paid nothing (Article II in vocabulary form).
#[test]
fn the_unclassed_binding_is_empty_and_lawful() {
    assert!(archetype_binding(0).is_empty());
    assert!(binding_trades(&archetype_binding(0)));
    // And an empty binding is the identity on every channel.
    for channel in CHANNELS {
        assert_eq!(factor_on(&archetype_binding(0), channel), ratio(1, 1));
    }
}

/// **NO TWO ARCHETYPES ARE THE SAME MECHANISM** (§6, asserted on the
/// seed before anything may mint against it).
///
/// A partition whose members compute identically distinguishes nothing.
/// This is the novelty law's arithmetic form, and it must already hold
/// of the seed or the gate it guards is meaningless.
#[test]
fn no_two_archetypes_share_a_mechanism() {
    let mut seen: Vec<(u8, Vec<(u8, cfe::ratio::Ratio)>)> = Vec::new();
    for class in 1..=cfe::dop::class_ceiling() {
        let shape = canonical(&archetype_binding(class));
        if let Some((other, _)) = seen.iter().find(|(_, s)| *s == shape) {
            panic!(
                "class {class} ({}) computes exactly as class {other} ({}) — \
                 a partition that distinguishes nothing",
                cfe::dop::archetype_word(class),
                cfe::dop::archetype_word(*other)
            );
        }
        seen.push((class, shape));
    }
}

/// **A STRICTLY-DOMINANT BINDING IS UNMINTABLE** — the law's whole
/// point, tested against the shape that would break a naive rule.
#[test]
fn a_strictly_dominant_binding_is_refused() {
    // Better yield AND cheaper upkeep: pure gain, no trade.
    assert!(!binding_trades(&[
        (CHANNEL_YIELD, ratio(3, 2)),
        (CHANNEL_UPKEEP, ratio(3, 4))
    ]));
    // Strictly worse on both is refused too — that is a penalty, not a
    // class.
    assert!(!binding_trades(&[
        (CHANNEL_YIELD, ratio(3, 4)),
        (CHANNEL_UPKEEP, ratio(3, 2))
    ]));
    // POLARITY IS THE REASON THIS TEST EXISTS. A structural rule of
    // "one factor above 1, one below" would call the second case a fair
    // trade — it raises one and lowers another — when it is strictly
    // worse on both, because upkeep and yield point opposite ways.
    assert!(channel_favours_increase(CHANNEL_YIELD));
    assert!(!channel_favours_increase(CHANNEL_UPKEEP));
    assert!(!channel_favours_increase(CHANNEL_SURVEY));
    // The lawful trade: more yield, more upkeep.
    assert!(binding_trades(&[
        (CHANNEL_YIELD, ratio(3, 2)),
        (CHANNEL_UPKEEP, ratio(3, 2))
    ]));
}

/// **NOVELTY CANNOT BE WALKED THROUGH BY REORDERING.** Canonical form
/// collapses order and duplicates, so two spellings of one mechanism
/// compare equal.
#[test]
fn canonical_form_defeats_reordering_and_duplicates() {
    let a = [(CHANNEL_YIELD, ratio(5, 4)), (CHANNEL_UPKEEP, ratio(5, 4))];
    let b = [(CHANNEL_UPKEEP, ratio(5, 4)), (CHANNEL_YIELD, ratio(5, 4))];
    assert_eq!(canonical(&a), canonical(&b), "order is not a mechanism");

    // Listed twice, it composes rather than shadowing.
    let split = [
        (CHANNEL_YIELD, ratio(5, 4)),
        (CHANNEL_YIELD, ratio(4, 5)),
        (CHANNEL_UPKEEP, ratio(3, 4)),
    ];
    assert_eq!(
        factor_on(&split, CHANNEL_YIELD),
        ratio(1, 1),
        "5/4 x 4/5 is 1 — exact, no rounding"
    );
    assert_eq!(
        canonical(&split),
        vec![(CHANNEL_UPKEEP, ratio(3, 4))],
        "a channel that composes to 1 is silent"
    );
}

/// **THE MECHANISM REACHES THE WORLD** — the witness V0 exists for.
///
/// Before this, sealing a class changed NOTHING: the receipt committed,
/// the snapshot carried it, and no rule read it. Two worlds identical
/// but for their sealed class must now diverge in their books, or the
/// class is still a term that only resonates.
#[test]
fn a_sealed_class_changes_what_the_world_banks() {
    use cfe::dop::{Resource, ZoneKind};

    let run = |class: Option<u8>| -> cfe::ratio::Ratio {
        let mut world = cfe::world::World::new();
        world.found("binding-hold").expect("founds");
        world.zone(0, ZoneKind::Extractor).expect("zones");
        if let Some(class) = class {
            world.seal_taxon(class, 5).expect("seals");
        }
        for _ in 0..8 {
            world.tick();
        }
        // The ACCRUAL, not the bank: the bank still holds the founding
        // endowment until a settle, and reading it measured the
        // endowment rather than the tick.
        world
            .accrued()
            .into_iter()
            .find(|(resource, _)| *resource == Resource::Iron)
            .map(|(_, amount)| amount)
            .unwrap_or_else(|| ratio(0, 1))
    };

    let unsealed = run(None);
    // ∿ plasma-field yields x3/2; ▲ apex-terrestrial yields x1/2.
    let rich = run(Some(6));
    let lean = run(Some(7));

    assert!(
        rich > unsealed,
        "the plasma-field binding must lift the yield: {rich} vs {unsealed}"
    );
    assert!(
        lean < unsealed,
        "the apex-terrestrial binding must lower it: {lean} vs {unsealed}"
    );
    // AND AN UNSEALED WORLD IS UNTOUCHED — V0 is strictly additive.
    //
    // Class 0 cannot be SEALED (`seal_taxon` refuses it: the unclassed
    // is the ABSENCE of a seal, not a seal naming absence), so the
    // unsealed run above IS the unclassed case. It reads the empty
    // binding, which `the_unclassed_binding_is_empty_and_lawful` pins
    // as the identity on every channel.
    let mut probe = cfe::world::World::new();
    probe.found("unclassed-hold").expect("founds");
    assert!(matches!(
        probe.seal_taxon(0, 5),
        Err(cfe::world::ActionRefused::TaxonUnknown { class: 0, .. })
    ));
}

// ─────────────────────────────────────────────────────────────────────
// THE MINT (V2) — `28_VOCABULARY` §5–§7
// ─────────────────────────────────────────────────────────────────────

use cfe::world::{ActionRefused, World};

/// A world whose line holds a progenitor seat, ready to mint.
fn seated() -> World {
    let mut world = World::new();
    world.found("mint-hold").expect("founds");
    world.seat_station("tunborn", 1).expect("seats the progenitor");
    world
}

/// **STANDING** — minting is a TIER 0 power (§5). A line with no
/// progenitor seat is refused, and so is one with a lesser seat.
#[test]
fn only_a_progenitor_may_mint() {
    let mut world = seated();
    // A watcher (2) is a real office and still not a board-maker.
    world.seat_station("coralline", 2).expect("seats the watcher");

    for line in ["coralline", "nobody"] {
        assert!(
            matches!(
                world.mint_vocabulary(line, 4, "◇", "probe", vec![
                    (CHANNEL_YIELD, ratio(5, 4)),
                    (CHANNEL_UPKEEP, ratio(5, 4)),
                ]),
                Err(ActionRefused::MintWithoutStanding { .. })
            ),
            "{line} minted without a progenitor seat"
        );
    }
    // And the progenitor may.
    assert!(world
        .mint_vocabulary("tunborn", 4, "◇", "probe", vec![
            (CHANNEL_YIELD, ratio(7, 4)),
            (CHANNEL_UPKEEP, ratio(7, 4)),
        ])
        .is_ok());
}

/// **STATIONS ARE NOT MINTABLE** (§10) — there is no axis code for an
/// office, so a T0 seat cannot manufacture the seat that gates it.
#[test]
fn no_axis_mints_an_office() {
    let mut world = seated();
    for axis in [0u8, 7, 8, 99] {
        assert!(matches!(
            world.mint_vocabulary("tunborn", axis, "", "office", vec![
                (CHANNEL_YIELD, ratio(5, 4)),
                (CHANNEL_UPKEEP, ratio(5, 4)),
            ]),
            Err(ActionRefused::VocabularyAxisUnknown { .. })
        ));
    }
}

/// **THE BINDING MUST TRADE** (§4a) — a strictly-dominant entry is
/// refused at the door, not balanced afterwards.
#[test]
fn a_dominant_mint_is_refused_at_the_door() {
    let mut world = seated();
    assert!(matches!(
        world.mint_vocabulary("tunborn", 4, "◇", "freelunch", vec![
            (CHANNEL_YIELD, ratio(2, 1)),
            (CHANNEL_UPKEEP, ratio(1, 2)),
        ]),
        Err(ActionRefused::BindingUnlawful)
    ));
    // Out of bounds refuses too.
    assert!(matches!(
        world.mint_vocabulary("tunborn", 4, "◇", "huge", vec![
            (CHANNEL_YIELD, ratio(9, 1)),
            (CHANNEL_UPKEEP, ratio(9, 1)),
        ]),
        Err(ActionRefused::BindingUnlawful)
    ));
    // A channel the engine does not compute refuses — the table grows by
    // LANDING a channel, never by a mint naming one.
    assert!(matches!(
        world.mint_vocabulary("tunborn", 4, "◇", "ghost", vec![
            (9, ratio(5, 4)),
            (CHANNEL_UPKEEP, ratio(3, 4)),
        ]),
        Err(ActionRefused::BindingUnlawful)
    ));
}

/// **NOVELTY IS THE MECHANISM, NEVER THE NAME** (§6). This is the gate
/// that keeps the editor from being a rename tool.
#[test]
fn a_mint_that_only_renames_an_existing_mechanism_refuses() {
    let mut world = seated();
    // Class 1 (biotic-swarm) is yield x5/4, upkeep x5/4. A different
    // word, a different mark, the same trade.
    assert!(matches!(
        world.mint_vocabulary("tunborn", 4, "✦", "swarmlike", vec![
            (CHANNEL_UPKEEP, ratio(5, 4)),
            (CHANNEL_YIELD, ratio(5, 4)),
        ]),
        Err(ActionRefused::MintNotNovel { like: 1 })
    ));
    // And a genuinely new trade lands.
    assert!(world
        .mint_vocabulary("tunborn", 4, "✦", "swarmlike", vec![
            (CHANNEL_YIELD, ratio(7, 4)),
            (CHANNEL_SURVEY, ratio(7, 4)),
        ])
        .is_ok());
}

/// **A MARK NAMES ONE THING** — the sigil and the word are unique on
/// their axis, extending the landed archetype pin to minted entries.
#[test]
fn a_minted_mark_cannot_shadow_a_seated_one() {
    let mut world = seated();
    // ∿ is class 6, plasma-field.
    assert!(matches!(
        world.mint_vocabulary("tunborn", 4, "∿", "newname", vec![
            (CHANNEL_YIELD, ratio(7, 4)),
            (CHANNEL_UPKEEP, ratio(7, 4)),
        ]),
        Err(ActionRefused::MintNameTaken { .. })
    ));
    assert!(matches!(
        world.mint_vocabulary("tunborn", 4, "✧", "plasma-field", vec![
            (CHANNEL_YIELD, ratio(7, 4)),
            (CHANNEL_UPKEEP, ratio(7, 4)),
        ]),
        Err(ActionRefused::MintNameTaken { .. })
    ));
}

/// **THE MINTED CLASS IS REAL** — it takes the next code, it seals, and
/// its mechanism reaches the world. End to end, this is the feature.
#[test]
fn a_minted_class_can_be_sealed_and_it_computes() {
    let mut world = seated();
    let before = cfe::vocabulary::ceiling(world.chronicle(), cfe::vocabulary::Axis::Class);

    let code = world
        .mint_vocabulary("tunborn", 4, "⌘", "covenantal", vec![
            (CHANNEL_YIELD, ratio(7, 4)),
            (CHANNEL_UPKEEP, ratio(7, 4)),
        ])
        .expect("the progenitor mints");
    assert_eq!(code, before + 1, "the code is the next one, derived");

    // BEFORE the mint this code was unsealable; after it, it seals.
    world.zone(0, cfe::dop::ZoneKind::Extractor).expect("zones");
    world.seal_taxon(code, 5).expect("the minted class seals");
    assert_eq!(world.taxon(), Some((code, 5)));

    // And the binding it arrived with is the binding the fold reports.
    let folded = cfe::vocabulary::fold(world.chronicle(), cfe::vocabulary::Axis::Class);
    let minted = folded.iter().find(|e| e.code == code).expect("in the fold");
    assert_eq!(minted.word, "covenantal");
    assert_eq!(minted.sigil, "⌘");
    assert_eq!(
        cfe::vocabulary::factor_on(&minted.binding, CHANNEL_YIELD),
        ratio(7, 4)
    );

    // SEAL B: the whole thing replays.
    assert_eq!(
        cfe::world::replay(world.chronicle()),
        world.committed(),
        "the mint replays"
    );
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.committed(), world.committed(), "and resumes");
}

/// **THE MINT RIDES THE WIRE** — tag 90 round-trips byte-exact,
/// including the exact rationals of the binding.
#[test]
fn the_mint_round_trips_the_wire() {
    use cfe::dop::ClosureEvent;
    let event = ClosureEvent::VocabularyMinted {
        axis: 4,
        code: 8,
        sigil: "⌘".into(),
        word: "covenantal".into(),
        binding: vec![(CHANNEL_YIELD, ratio(7, 4)), (CHANNEL_UPKEEP, ratio(22, 7))],
    };
    let bytes = event.wire_bytes();
    // wire_bytes emits a TLV record: tag(1) | len(4 LE) | value.
    assert_eq!(bytes[0], 96, "the mint rides tag 96");
    let len = u32::from_le_bytes(bytes[1..5].try_into().expect("len")) as usize;
    let back = ClosureEvent::from_wire(96, &bytes[5..5 + len]).expect("tag 96 decodes");
    assert_eq!(back, event, "the mint survives the wire whole");
}

/// **THE ORDERING LAW** (`28_VOCABULARY` §9, V3) — the sharpest risk in
/// the whole design, pinned adversarially.
///
/// A draw must read the vocabulary AS OF ITS OWN RECEIPT. Reading
/// today's instead lets a replay of an old era hand lines a class that
/// did not exist when they were founded — and it fails SILENTLY: the
/// replay terminates, the digests chain, and the world merely becomes a
/// different world than the one that was played.
#[test]
fn the_fold_at_a_stamp_does_not_see_a_later_mint() {
    use cfe::vocabulary::{fold, fold_at, Axis};

    let mut world = seated();
    let before_stamp = world.now();
    let before = fold_at(world.chronicle(), Axis::Class, before_stamp).len();

    // Time passes, then a class is minted.
    world.tick();
    world.tick();
    world
        .mint_vocabulary("tunborn", 4, "⌘", "covenantal", vec![
            (CHANNEL_YIELD, ratio(7, 4)),
            (CHANNEL_UPKEEP, ratio(7, 4)),
        ])
        .expect("mints");
    let after = fold(world.chronicle(), Axis::Class).len();

    assert_eq!(after, before + 1, "the mint is in today's fold");
    assert_eq!(
        fold_at(world.chronicle(), Axis::Class, before_stamp).len(),
        before,
        "and it is INVISIBLE to a draw taken before it — history does \
         not gain a class it never had"
    );

    // The whole point, stated as the failure it prevents: a line founded
    // at the earlier stamp cannot wear the minted class.
    let earlier = fold_at(world.chronicle(), Axis::Class, before_stamp);
    assert!(
        !earlier.iter().any(|entry| entry.word == "covenantal"),
        "a class that did not exist yet must not be drawable then"
    );
}

/// **THE COVENANT IS A T3 INSTRUMENT** (operator ruling 2026-08-05:
/// *drop covenant to T3 and below*).
///
/// A line holding a chartered office does not covenant — it decrees or
/// delegates. The forward obligation is how the COMMON band binds:
/// parties with no authority over each other post collateral against a
/// deadline, which is the whole reason collateral is in the mechanism.
#[test]
fn a_seated_line_may_not_covenant() {
    use cfe::dop::Resource;

    let common = || {
        let mut world = World::new();
        world.found("covenant-hold").expect("founds");
        // G1: the line, so the world knows whose seat to read.
        world
            .seal_genesis("commoners", 2, 2, 1)
            .expect("seals the line");
        world.zone(0, cfe::dop::ZoneKind::Extractor).expect("zones");
        world.zone(8, cfe::dop::ZoneKind::Habitat).expect("zones");
        for _ in 0..16 {
            world.tick();
        }
        world.collect().expect("collects");
        let charted = world
            .chart("outpost", cfe::world::FIRST_ORB)
            .expect("charts");
        (world, charted)
    };

    // THE COMMON MAY. tier_of(None) = T3, the only road to the band.
    let (mut world, charted) = common();
    world
        .seal_forward(cfe::world::FIRST_ORB, charted, Resource::Iron, ratio(1, 1), 8)
        .expect("the common band binds by contract");

    // EVERY SEATED OFFICE MAY NOT — T0, T1 and T2 alike.
    for station in 1..=4u8 {
        let (mut world, charted) = common();
        world.seat_station("commoners", station).expect("seats");
        assert!(
            matches!(
                world.seal_forward(cfe::world::FIRST_ORB, charted, Resource::Iron, ratio(1, 1), 8),
                Err(ActionRefused::CovenantAboveTheCommon { .. })
            ),
            "station {station} sealed a covenant it should not hold"
        );
    }
}
