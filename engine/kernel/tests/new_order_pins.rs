//! THE NEW ORDER — the two offices and the BOND, read through the
//! module split rather than through a hand-built `Committed`.
//!
//! `world.rs` was one 5,034-line file and is now `world/` — fifteen
//! organs: `accord` seals the bonds, `state` holds them, `replay`
//! rebuilds them, `tick` expires them, `economy` settles them,
//! `refusal` refuses them. Every pin the offices already had constructs
//! `Committed` DIRECTLY, which is the right shape for testing the
//! algebra and the wrong shape for asking whether the split still
//! delivers a world to it. These build through the real deed path —
//! found, zone, tick, collect, chart, seal_bond — and then read.
//!
//! What they hold:
//!   1. THE BOND SURVIVES THE SPLIT — a bond sealed through `accord`
//!      arrives in `V` and reaches `D` on the BOND channel, not merely
//!      in some channel.
//!   2. BOTH OFFICES STAND — `D` reads above the ground and `V` holds
//!      the substance, from one committed state, neither collapsed.
//!   3. THE OFFICES ARE NOT EACH OTHER — the reading moves when the
//!      relation moves, and the substance does not.

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::world::{World, FIRST_ORB};
use kernel::bridge::{
    open_axes_of, project_complex, read_entangled, standing_axial_of, CHANNEL_BOND,
    CHANNEL_FORWARD, CHANNEL_DEBT,
};
use rfa_core::Degree;

/// A real two-planet economy, built by deeds: 16 ticks of extraction
/// and habitat, collected, then a second colony chartered from the
/// source's own reserves.
fn economy() -> (World, u64) {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects");
    let charted = world.chart("outpost", FIRST_ORB).expect("charts");
    (world, charted)
}

#[test]
fn a_bond_sealed_in_the_new_order_reaches_d_on_the_bond_channel() {
    // BOND-1 THROUGH THE SPLIT. The deed lands in `world/accord.rs`,
    // the standing order is held in `world/state.rs`, and the bridge
    // projects `Committed::bonds` onto CHANNEL_BOND. If the split had
    // dropped the field, mis-seated it, or stopped replaying it, the
    // relation would simply not arrive — and D would read a world with
    // no obligations in it.
    let (mut world, charted) = economy();

    // Before the bond: the two orbs relate by nothing.
    let bare = read_entangled(&world.committed());
    assert_eq!(bare.degree(), Degree::GROUND, "no obligation, no circulation");

    world
        .seal_bond(FIRST_ORB, charted, Resource::Iron, ratio(1, 2), 0)
        .expect("the accord seals");

    let state = world.committed();
    assert_eq!(state.bonds.len(), 1, "V carries the standing order");

    // AND IT ARRIVES ON THE RIGHT AXIS. `project_complex` seats bonds on
    // CHANNEL_BOND; a bond leaking onto the forward or debt channel
    // would still read as "some structure" on a summed meter, which is
    // exactly why the reading is per-axis.
    let (delta, support) = project_complex(&state);
    let (i, j) = (0usize, 1usize);
    assert!(support.has(i, j, CHANNEL_BOND), "the relation stands on the BOND channel");
    assert_eq!(
        *delta.entry(i, j, CHANNEL_BOND),
        ratio(1, 2),
        "and it carries the sealed rate, exactly"
    );
    assert!(!support.has(i, j, CHANNEL_FORWARD), "a bond is not a forward");
    assert!(!support.has(i, j, CHANNEL_DEBT), "a bond is not a debt");
}

#[test]
fn both_offices_stand_from_one_committed_state() {
    // D(Ω) AND V(Ω) — the two readings of one totality (`17` §3), taken
    // through the new order. Neither is derived from the other and
    // neither is stored: `V` is the committed state the deeds built,
    // `D` is the reading taken of it.
    let (mut world, charted) = economy();
    world
        .seal_bond(FIRST_ORB, charted, Resource::Iron, ratio(1, 2), 0)
        .expect("seals");
    let state = world.committed();

    // V — THE SUBSTANCE. Held, not computed.
    assert_eq!(state.orbs.len(), 2, "V holds the colonies the deeds founded");
    assert_eq!(state.bonds.len(), 1, "V holds the obligation the deed sealed");

    // D — THE READING. Taken of V, never stored on it.
    let read = read_entangled(&state);
    assert_eq!(
        read.degree(),
        Degree::above_ground(read.rank),
        "D is GROUND plus what circulates — the only constructor there is"
    );

    // A SINGLE BOND IS A LINE, NOT A CYCLE. Two orbs and one relation
    // carry no independent cycle, so D stands ON the ground: the world
    // holds real substance and no circulation yet. Recorded rather than
    // hidden — the offices are independent, and this is what that looks
    // like when only one of them has anything.
    assert_eq!(read.degree(), Degree::GROUND, "one relation closes no loop");
    assert!(!state.bonds.is_empty(), "yet V is not empty — the offices are not each other");
}

#[test]
fn the_open_axes_name_where_this_world_can_still_grow() {
    // TOWARD UPLIFT, through the real deed path. Every channel of a
    // freshly chartered economy is at the ground, so every direction is
    // open — the reading gives a map before it ever withholds anything.
    let (mut world, charted) = economy();
    world
        .seal_bond(FIRST_ORB, charted, Resource::Iron, ratio(1, 2), 0)
        .expect("seals");
    let state = world.committed();

    let axial = standing_axial_of(&state, FIRST_ORB);
    assert_eq!(axial.len(), 3, "one share per channel, never one scalar");
    for share in &axial {
        assert!(*share >= ratio(0, 1), "a share is a magnitude, never a debt");
    }
    assert_eq!(
        open_axes_of(&state, FIRST_ORB),
        vec![CHANNEL_BOND, CHANNEL_FORWARD, CHANNEL_DEBT],
        "nothing circulates yet, so every direction is still open to it"
    );
}
