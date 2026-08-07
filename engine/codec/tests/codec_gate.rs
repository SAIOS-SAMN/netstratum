//! CODEC-1 gate witnesses (§11 ladder #35): the whole committed state
//! round-trips byte-exact through the one canonical encoding; one
//! state, one encoding; the schema version pins the field set — a
//! future schema refuses to decode under this kernel (a schema bump
//! is a kernel amendment, never a silent widening); the empty world
//! encodes; and the encoding is total over every field the freeze
//! window snapshot holds (the MESH-1d inventory, all twenty).

use cfe::dop::ZoneKind;
use cfe::ratio::ratio;
use cfe::world::World;
use codec::{from_state_bytes, state_bytes, SCHEMA_V10};

/// A rich session: zones, ticks, a collection, a gauge shift, a raw
/// want — enough committed surface to exercise the families that a
/// played world reaches without multi-world choreography.
fn a_rich_world() -> World {
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("settles");
    world.shift_gauge(ratio(1, 3));
    world
}

#[test]
fn the_state_round_trips_byte_exact() {
    let committed = a_rich_world().committed();
    let bytes = state_bytes(&committed);
    // Wave-3/4: v6 leads since element routes joined the snapshot.
    // Every older schema still READS — a v5 state decodes with no
    // element lanes, which is what a world predating them held.
    assert_eq!(bytes[0], SCHEMA_V10, "the schema leads the bytes");
    let decoded = from_state_bytes(&bytes).expect("the canonical state decodes");
    assert_eq!(decoded, committed, "value round trip");
    assert_eq!(state_bytes(&decoded), bytes, "BYTE round trip — one state, one encoding");
}

#[test]
fn the_empty_world_encodes() {
    let committed = World::new().committed();
    let bytes = state_bytes(&committed);
    let decoded = from_state_bytes(&bytes).expect("the empty state decodes");
    assert_eq!(decoded, committed);
    assert_eq!(state_bytes(&decoded), bytes);
}

#[test]
fn a_future_schema_refuses() {
    let committed = World::new().committed();
    let mut bytes = state_bytes(&committed);
    bytes[0] = SCHEMA_V10 + 1;
    assert!(
        from_state_bytes(&bytes).is_none(),
        "a schema this kernel does not know refuses — never a guess"
    );
}

#[test]
fn a_truncated_state_refuses() {
    let committed = a_rich_world().committed();
    let bytes = state_bytes(&committed);
    for cut in [1usize, bytes.len() / 2, bytes.len() - 1] {
        assert!(
            from_state_bytes(&bytes[..cut]).is_none(),
            "truncation at {cut} refuses, never a partial state"
        );
    }
}

#[test]
fn distinct_states_encode_distinct_bytes() {
    let one = a_rich_world().committed();
    let mut world = a_rich_world();
    world.shift_gauge(ratio(1, 7));
    let other = world.committed();
    assert_ne!(one, other, "the sessions truly differ");
    assert_ne!(state_bytes(&one), state_bytes(&other), "different state, different bytes");
}
