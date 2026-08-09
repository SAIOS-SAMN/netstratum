//! C1.7 — stellar capacity game (`lines_of` as play).
//!
//! Pure board over almanac locality; deeds stay prospect / refine.

use cfe::almanac::{lines_of, LINES};
use cfe::dop::{positive, unit, ZoneKind};
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB};

fn founded(class: StarClass) -> World {
    let mut w = World::new();
    w.found("cap-home").expect("found");
    w.commission_star(class).expect("star");
    w.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("site");
    w.zone(0, ZoneKind::Extractor).expect("zone");
    w
}

#[test]
fn capacity_board_matches_lines_of() {
    for class in [
        StarClass::M,
        StarClass::G,
        StarClass::O,
        StarClass::BrownDwarf,
    ] {
        let w = founded(class);
        let b = w.stellar_capacity_board(FIRST_ORB);
        assert_eq!(b.star, Some(class));
        assert_eq!(b.reach, lines_of(class));
        assert_eq!(b.table_lines, LINES);
        assert_eq!(b.lines.len(), LINES as usize);
        for row in &b.lines {
            assert_eq!(row.reachable, row.line <= lines_of(class));
        }
        assert_eq!(b.open_strata, 0);
        assert!(
            b.next_move.contains("prospect"),
            "empty ground should prompt prospect: {}",
            b.next_move
        );
    }
}

#[test]
fn prospect_advances_capacity_board() {
    let mut w = founded(StarClass::G);
    let reach = lines_of(StarClass::G);
    let before = w.stellar_capacity_board(FIRST_ORB);
    assert_eq!(before.open_strata, 0);

    let struck = w.prospect(FIRST_ORB, 1).expect("prospect");
    let after = w.stellar_capacity_board(FIRST_ORB);
    assert_eq!(after.open_strata, 1);
    assert_eq!(after.deepest_open, 1);
    assert_eq!(after.veins.len(), 1);
    assert_eq!(after.veins[0].element, struck.element);
    // G reach is 5 — deeper prospect still available under the sun.
    if reach > 1 {
        assert!(
            after.next_move.contains("deeper") || after.next_move.contains("refine") || after.next_move.contains("prospect"),
            "expected next play after first vein: {}",
            after.next_move
        );
    }
    assert_eq!(replay(w.chronicle()), w.committed(), "SEAL B capacity board");
}

#[test]
fn m_dwarf_marks_deep_lines_unreachable() {
    let w = founded(StarClass::M);
    let b = w.stellar_capacity_board(FIRST_ORB);
    assert_eq!(b.reach, 3);
    let deep = b.lines.iter().filter(|r| r.line > 3).count();
    let shallow = b.lines.iter().filter(|r| r.line <= 3 && r.reachable).count();
    assert_eq!(shallow, 3);
    assert_eq!(deep, (LINES - 3) as usize);
    assert!(b.lines.iter().filter(|r| r.line > 3).all(|r| !r.reachable));
}

#[test]
fn no_star_capacity_is_empty_reach() {
    let mut w = World::new();
    w.found("dark").expect("found");
    let b = w.stellar_capacity_board(FIRST_ORB);
    assert!(b.star.is_none());
    assert_eq!(b.reach, 0);
    assert!(b.next_move.contains("commission") || b.next_move.contains("star"));
}
