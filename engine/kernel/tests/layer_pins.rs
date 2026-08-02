//! THE LAYER LAW, MECHANISED (`17_CAPACITY` §3d).
//!
//! An earlier landing wrote the layer assignment as a DOC TABLE and
//! called "declare your layer when you land" a standing rule. In a
//! distributed economy every node validates independently, so a reach
//! recorded only in prose is a reach no node can enforce — the honour
//! system does not survive consensus, and Article III LAW 8 excises a
//! term that only resonates. The operator refused it, correctly.
//!
//! `kernel::layer_of` is the layer. These pin the two properties that
//! make it real rather than decorative: it is EXHAUSTIVE, so a new deed
//! cannot land without a layer decision; and `None` means NOT RULED, so
//! a gate refuses rather than assuming a reach.

use cfe::dop::{ClosureEvent, Office, Resource};
use cfe::ratio::ratio;
use kernel::{layer_of, Layer};
use std::fs;

#[test]
fn the_layer_law_has_no_wildcard_arm() {
    // THE WHOLE GUARANTEE. `layer_of` is exhaustive over `ClosureEvent`
    // with no `_` arm, so adding a variant BREAKS THE BUILD until
    // somebody decides its reach. A wildcard would restore exactly the
    // inference this function exists to abolish — the rule whose reach
    // is read off whichever arm happens to catch it.
    //
    // Editing this test is the amendment (the `recurrence_guard` and
    // `cage_arrow` shape): an author who wants a default layer must
    // come here and say so, in the open.
    let source = fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"),
    )
    .expect("kernel source readable");
    let start = source.find("pub fn layer_of").expect("the layer law exists");
    let body = &source[start..start + source[start..].find("\n}\n").expect("body ends")];

    for line in body.lines() {
        let t = line.trim_start();
        if t.starts_with("//") {
            continue;
        }
        assert!(
            !(t.starts_with("_ =>") || t.starts_with("_=>")),
            "layer_of grew a wildcard arm — a new deed can now inherit a \
             reach nobody decided:\n  {line}"
        );
    }
}

#[test]
fn a_ceremonial_deed_and_a_universal_one_are_told_apart() {
    // THE RULED CASE (§3d.1a). The covenant RULING binds the CEREMONY;
    // the forward mechanism it was confused with is ordinary finance
    // open to any line, and stays universal. Both readings survive in
    // one table only because the layer is per-DEED and computed.
    assert_eq!(
        layer_of(&ClosureEvent::TaxonSealed { class: 4, clade: 19 }),
        Some(Layer::Civilizational),
        "class and clade are a line's own identity — the draconic seat"
    );
    assert_eq!(
        layer_of(&ClosureEvent::CovenantSealed {
            obligor_orb: 1,
            beneficiary_orb: 2,
            resource: Resource::Iron,
            quota: ratio(3, 1),
            window: 10,
            collateral: ratio(1, 1),
        }),
        Some(Layer::Universal),
        "the FORWARD is quota - delivered: ordinary finance, open to any line"
    );
    assert_eq!(
        layer_of(&ClosureEvent::ZoneRazed { orb_id: 1, tile: 0 }),
        Some(Layer::World),
        "a zone is one committed state's own"
    );
    assert_eq!(
        layer_of(&ClosureEvent::SovereignUnionSealed {
            progenitor: Office::Progenitor,
            sovereign: Office::Sovereign,
            term_stamps: 1500,
        }),
        Some(Layer::Universal),
        "the offices bind everyone"
    );
}

#[test]
fn not_ruled_is_a_value_and_never_a_default() {
    // `None` MEANS NOT YET RULED. Three deeds are genuinely contested
    // (§3d.3) and a false assignment would be worse than an absent one,
    // so they answer `None` and any gate consulting the layer must
    // REFUSE on it rather than assume a reach. Same discipline as
    // `scope_of`'s None and `covered()`'s Unjudged.
    //
    // This pin SHRINKS as the contested three are ruled. It must never
    // grow: a new deed answering None means somebody skipped the
    // decision the exhaustive match forced on them.
    let contested = [
        ClosureEvent::FrictionSettled { orb_id: 1, amount: ratio(1, 1) },
        ClosureEvent::IncursionStruck {
            era: "e".into(),
            world: "w".into(),
            line: "l".into(),
        },
    ];
    for event in contested {
        assert_eq!(layer_of(&event), None, "{event:?} is contested, not defaulted");
    }
}
