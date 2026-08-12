//! **ZONE VOCABULARY (#41) — the operator's three-way ruling as one
//! resolution ladder.**
//!
//! A minted building's binding is a set of SCALE FACTORS, and factors
//! need something to scale. The ruling: a declared base wins; failing
//! that the base is inferred from the channels the binding touches;
//! failing that the entry is lawful vocabulary that simply never
//! builds. Making the third outcome LAWFUL is what lets inference stay
//! ambiguous without being dangerous — nothing has to be guessed.

use cfe::dop::{ClosureEvent, ZoneKind, ZONE_KINDS};
use cfe::ratio::ratio;
use cfe::vocabulary::{
    self, zone_base, Axis, TradingBinding, ZoneBase, CHANNEL_CONVERTER_YIELD,
    CHANNEL_EXTRACTOR_UPKEEP, CHANNEL_HABITAT_UPKEEP, CHANNEL_HABITAT_YIELD, CHANNEL_UPKEEP,
    CHANNEL_YIELD,
};

#[test]
fn the_zone_seed_is_the_standing_table_so_the_next_code_cannot_collide() {
    let seed = vocabulary::seed(Axis::Zone);
    assert_eq!(
        seed.len(),
        usize::from(ZONE_KINDS),
        "every compiled kind enters the fold"
    );
    for entry in &seed {
        let kind = ZoneKind::from_wire(entry.code).expect("a seed code is a compiled kind");
        assert_eq!(entry.word, cfe::dop::zone_word(kind), "the word is the kind's own");
        assert!(entry.binding.is_empty(), "a compiled kind claims nothing — the ground state");
        assert_eq!(entry.base, None, "and modulates nothing");
    }
    // THE PROPERTY THE SEED EXISTS FOR: the fold's ceiling is the last
    // compiled code, so the first minted kind takes the next wire code
    // and can never shadow a compiled one.
    let ceiling = seed.iter().map(|entry| entry.code).max().unwrap_or(0);
    assert_eq!(ceiling, ZONE_KINDS);
    // Free of the COMPILED table, which is what "free" has to mean once
    // minted codes exist: `from_wire` answers for them by design (that
    // is how a minted building rides one byte), so the question is
    // whether the table claims the code, and `from_wire_const` is the
    // reading that says.
    assert_eq!(ZoneKind::from_wire_const(ceiling + 1), None, "the next code is free");
    assert_eq!(
        ZoneKind::from_wire(ceiling + 1),
        Some(ZoneKind::Minted(ceiling + 1)),
        "and past the table the wire reads a MINTED kind"
    );
    assert!(ZoneKind::Minted(ceiling + 1).wire() == ceiling + 1, "which rides as its own code");
}

#[test]
fn a_declared_base_wins() {
    // RUNG 1. The mint named it; nothing is inferred.
    for code in 1..=ZONE_KINDS {
        let kind = ZoneKind::from_wire(code).expect("compiled");
        assert_eq!(
            zone_base(Some(code), &[(CHANNEL_YIELD, ratio(5, 4))]),
            ZoneBase::Declared(kind)
        );
    }
    // A declared base OUTRANKS what the channels would have inferred:
    // habitat channels on a mint that declares the extractor.
    assert_eq!(
        zone_base(
            Some(ZoneKind::Extractor.wire()),
            &[(CHANNEL_HABITAT_YIELD, ratio(5, 4)), (CHANNEL_HABITAT_UPKEEP, ratio(5, 4))]
        ),
        ZoneBase::Declared(ZoneKind::Extractor),
        "a declaration is not a hint"
    );
}

#[test]
fn a_base_is_inferred_from_the_channels_the_binding_touches() {
    // RUNG 2. The channel table already says which family each channel
    // prices; nothing is restated by the inference.
    assert_eq!(
        zone_base(None, &[(CHANNEL_HABITAT_YIELD, ratio(5, 4)), (CHANNEL_UPKEEP, ratio(5, 4))]),
        ZoneBase::Inferred(ZoneKind::Habitat)
    );
    assert_eq!(
        zone_base(None, &[(CHANNEL_EXTRACTOR_UPKEEP, ratio(3, 4)), (CHANNEL_YIELD, ratio(3, 4))]),
        ZoneBase::Inferred(ZoneKind::Extractor)
    );
    assert_eq!(
        zone_base(None, &[(CHANNEL_CONVERTER_YIELD, ratio(5, 4))]),
        ZoneBase::Inferred(ZoneKind::Smelter),
        "the converter channel prices the chain; its first rung stands for it"
    );
    // Two habitat channels still name ONE family.
    assert_eq!(
        zone_base(
            None,
            &[(CHANNEL_HABITAT_YIELD, ratio(5, 4)), (CHANNEL_HABITAT_UPKEEP, ratio(3, 4))]
        ),
        ZoneBase::Inferred(ZoneKind::Habitat)
    );
}

#[test]
fn what_resolves_to_nothing_is_vocabulary_that_never_builds() {
    // RUNG 3, and the reason the ladder is safe. Each of these is a
    // case inference CANNOT answer, and none of them has to be guessed.
    //
    // Only the general channels — they price every zone alike.
    assert_eq!(
        zone_base(None, &[(CHANNEL_YIELD, ratio(5, 4)), (CHANNEL_UPKEEP, ratio(5, 4))]),
        ZoneBase::Unresolved
    );
    // Two families at once — a binding that speaks to both names
    // neither.
    assert_eq!(
        zone_base(
            None,
            &[(CHANNEL_HABITAT_YIELD, ratio(5, 4)), (CHANNEL_EXTRACTOR_UPKEEP, ratio(5, 4))]
        ),
        ZoneBase::Unresolved,
        "ambiguity resolves to NOTHING, never to a guess"
    );
    // Nothing at all.
    assert_eq!(zone_base(None, &[]), ZoneBase::Unresolved);
    // A declared base that names no compiled kind.
    assert_eq!(zone_base(Some(0), &[]), ZoneBase::Unresolved);
    assert_eq!(zone_base(Some(ZONE_KINDS + 1), &[]), ZoneBase::Unresolved);
    // And an unresolved base builds nothing — the third outcome stated
    // as the thing it means.
    assert_eq!(ZoneBase::Unresolved.kind(), None);
    assert_eq!(ZoneBase::Declared(ZoneKind::Habitat).kind(), Some(ZoneKind::Habitat));
    assert_eq!(ZoneBase::Inferred(ZoneKind::Habitat).kind(), Some(ZoneKind::Habitat));
}

#[test]
fn a_minted_kind_may_never_dominate_the_kind_it_varies() {
    // WHY RUNG 1 IS THE RECOMMENDED ONE: with a base to modulate,
    // `TradingBinding`'s one-benefit-one-friction law makes a strictly
    // better building UNMINTABLE — not checked, unconstructable.
    let dominant = TradingBinding::new(vec![
        (CHANNEL_YIELD, ratio(2, 1)),   // more yield
        (CHANNEL_UPKEEP, ratio(1, 2)),  // and less upkeep
    ]);
    assert_eq!(
        dominant,
        Err(vocabulary::BindingFault::NoFriction),
        "a strictly-dominant building does not construct"
    );
    // The honest trade does.
    assert!(
        TradingBinding::new(vec![(CHANNEL_YIELD, ratio(2, 1)), (CHANNEL_UPKEEP, ratio(2, 1))])
            .is_ok(),
        "more yield bought with more upkeep is a building"
    );
}

#[test]
fn the_mint_receipt_carries_the_base_and_the_sealed_form_decodes_without_it() {
    let deed = ClosureEvent::VocabularyMinted {
        axis: Axis::Zone.code(),
        code: ZONE_KINDS + 1,
        sigil: "⌂".into(),
        word: "deepworks".into(),
        binding: vec![(CHANNEL_YIELD, ratio(2, 1)), (CHANNEL_UPKEEP, ratio(2, 1))],
        base: Some(ZoneKind::Extractor.wire()),
        ration: None,
    };
    let widened = deed.wire_bytes();
    let mut records = cfe::tlv::records(&widened);
    let (tag, wide_value) = records.next().expect("one record");
    assert_eq!(tag, 96);
    assert_eq!(ClosureEvent::from_wire(tag, wide_value).expect("decodes"), deed);

    // THE SEALED FORM — every mint that landed before the zone axis
    // carries no base, and its value is a byte-PREFIX of the widened
    // one, so it decodes to `None` forever.
    let sealed = ClosureEvent::VocabularyMinted {
        axis: Axis::Zone.code(),
        code: ZONE_KINDS + 1,
        sigil: "⌂".into(),
        word: "deepworks".into(),
        binding: vec![(CHANNEL_YIELD, ratio(2, 1)), (CHANNEL_UPKEEP, ratio(2, 1))],
        base: None,
        ration: None,
    };
    let sealed_bytes = sealed.wire_bytes();
    let sealed_value = cfe::tlv::records(&sealed_bytes).next().expect("one record").1;
    assert_eq!(
        ClosureEvent::from_wire(96, sealed_value).expect("decodes"),
        sealed
    );
    assert!(
        wide_value.len() == sealed_value.len() + 1 && wide_value.starts_with(sealed_value),
        "the base rides as one trailing byte and never re-cuts the sealed form"
    );
}
