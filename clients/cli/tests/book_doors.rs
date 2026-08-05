//! BOOK-1 HAS DOORS — the deeds are reachable (`19_LENS` §16).
//!
//! The registry landed committed, replayable and wire-encoded with
//! **zero callers**: `mint_book`, `post_book` and `archive_book`
//! appeared only at their own definitions. A feature nothing can reach
//! does not exist for anyone outside a test suite.
//!
//! These pins drive the deeds the way a door does — through the world
//! surface a CLI verb calls — and prove the round trip a civilization
//! actually makes: mint, post, read back, close, and find it still
//! readable.

use cfe::books::{book_name, kind_of};
use cfe::world::{replay, ActionRefused, World};
use cfe::dop::non_negative;
use cfe::ratio::ratio;

fn founded() -> World {
    let mut world = World::default();
    world.found("home").expect("founds");
    world
}

fn orb_id(world: &World) -> u64 {
    world.orb().expect("founded").orb_id
}

/// **THE WHOLE JOURNEY A LINE MAKES.** Mint an account of its own
/// making, post to it, read it back, then close it — and find every
/// entry still standing.
#[test]
fn a_line_mints_posts_reads_and_closes_its_own_book() {
    let mut world = founded();
    let orb = orb_id(&world);

    // The door opens on a name, not a digest — that is the whole point
    // of addressing a kind by its normalized name.
    let kind = world.mint_book(orb, &book_name("Deep Valor")).expect("mints");
    assert_eq!(kind, kind_of("  deep   VALOR "));

    world
        .post_book(orb, kind, non_negative(3, 1), non_negative(1, 1))
        .expect("posts");

    // Read back through the uniform view the display uses.
    let view = world.orb().expect("orb").books();
    let held = view.iter().find(|held| held.kind == kind).expect("present");
    assert_eq!(held.name, "Deep Valor", "the declared name is what shows");
    assert!(!held.chartered);
    assert_eq!(held.standing(), Some(ratio(3, 4)));

    world.archive_book(orb, kind).expect("closes");
    let view = world.orb().expect("orb").books();
    let held = view.iter().find(|held| held.kind == kind).expect("still there");
    assert!(held.archived);
    assert_eq!(held.standing(), Some(ratio(3, 4)), "closing loses nothing");
}

/// The display surface shows the charter's four before anything is
/// minted, and both kinds afterwards — a reader never learns which
/// storage a book came from.
#[test]
fn the_display_view_carries_charter_and_minted_alike() {
    let mut world = founded();
    let orb = orb_id(&world);
    assert_eq!(world.orb().expect("orb").books().len(), 4);

    world.mint_book(orb, &book_name("Valor")).expect("mints");
    world.mint_book(orb, &book_name("Ardor")).expect("mints");
    let view = world.orb().expect("orb").books();
    assert_eq!(view.len(), 6);
    assert_eq!(view.iter().filter(|held| held.chartered).count(), 4);
    // Charter books lead, so a display never interleaves the two kinds.
    assert!(view[..4].iter().all(|held| held.chartered));
}

/// **THE DOOR CARRIES THE REFUSALS, not just the successes.** Each one
/// a player can actually hit from the verb, hit through the same path
/// the verb takes.
#[test]
fn the_door_refuses_the_way_the_law_says() {
    let mut world = founded();
    let orb = orb_id(&world);

    // `book mint` with nothing after it.
    assert!(matches!(
        cfe::books::BookName::new(""),
        Err(cfe::books::BookNameFault::NormalizesToNothing)
    ));
    // `book mint TRADE-1`
    assert!(matches!(
        cfe::books::BookName::new("trade-1"),
        Err(cfe::books::BookNameFault::Charter { .. })
    ));
    // `book post` before minting.
    assert!(matches!(
        world.post_book(orb, kind_of("valor"), non_negative(1, 1), non_negative(0, 1)),
        Err(ActionRefused::NoSuchBook)
    ));

    let kind = world.mint_book(orb, &book_name("Valor")).expect("mints");
    // `book mint Valor` twice.
    assert!(matches!(
        world.mint_book(orb, &book_name("  valor  ")),
        Err(ActionRefused::BookAlreadyMinted { .. })
    ));
    // `book archive COVENANT-1`
    assert!(matches!(
        world.archive_book(orb, kind_of("COVENANT-1")),
        Err(ActionRefused::CharterBookNotArchivable { .. })
    ));
    // `book post` into a closed account.
    world.archive_book(orb, kind).expect("closes");
    assert!(matches!(
        world.post_book(orb, kind, non_negative(1, 1), non_negative(0, 1)),
        Err(ActionRefused::BookClosedToWriting { .. })
    ));
}

/// **THE DEEDS A DOOR MAKES ARE REAL RECEIPTS.** Everything driven
/// through the door replays from the chronicle alone — a door that
/// wrote unreceipted state would be a second timeline.
#[test]
fn everything_the_door_does_survives_replay() {
    let mut world = founded();
    let orb = orb_id(&world);
    let kept = world.mint_book(orb, &book_name("Deep Valor")).expect("mints");
    let closed = world.mint_book(orb, &book_name("Ardor")).expect("mints");
    world
        .post_book(orb, kept, non_negative(5, 2), non_negative(1, 4))
        .expect("posts");
    world.archive_book(orb, closed).expect("closes");

    let rebuilt = replay(world.chronicle());
    assert_eq!(
        rebuilt.orbs.first().expect("orb").books,
        world.orb().expect("orb").books,
        "the door's deeds rebuild from receipts alone"
    );
    let held = rebuilt.orbs.first().expect("orb");
    assert_eq!(held.book(kept).expect("kept").kept, ratio(5, 2));
    assert!(held.book(closed).expect("closed").archived);
}

/// The agent door (`as <handle> book …`) runs the SAME deeds under a
/// scope check, so an unpermitted entity is refused rather than
/// silently succeeding.
#[test]
fn the_agent_door_is_scope_checked() {
    let mut world = founded();
    let orb = orb_id(&world);
    // An entity id that was never registered holds no scope at all.
    let stranger = 9_999_u64;
    assert!(
        world.mint_book_as(stranger, orb, &book_name("Valor")).is_err(),
        "an unregistered actor may not mint"
    );
    // And the sovereign path still works, so the refusal above is the
    // scope check rather than the deed being broken.
    assert!(world.mint_book(orb, &book_name("Valor")).is_ok());
}
