//! THE TRANSLATOR (mycelium 78.142 `sdk::rule`): a netstratum program stated
//! as the court's rule through mycelium's own constructors, judged by the
//! judge's own steps (`would_judge`) — offline, nothing sent.

use ns_hypha::mycelium::sdk::rule::would_judge;
use ns_hypha::mycelium::transducer::family::NoCrystal;
use ns_hypha::seat::{translate, Untranslated};
use solver::grid::run;
use solver::{Grid, Op};

fn rows(g: &Grid) -> Vec<Vec<u8>> {
    g.cells.chunks(g.cols).map(|r| r.to_vec()).collect()
}

fn family(program: &[Op], inputs: &[Grid]) -> Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)> {
    inputs.iter().map(|g| (rows(g), rows(&run(&program.to_vec(), g)))).collect()
}

fn inputs() -> Vec<Grid> {
    vec![
        Grid::new(3, 2, vec![1, 0, 2, 3, 0, 4]).unwrap(),
        Grid::new(3, 2, vec![0, 5, 6, 0, 7, 0]).unwrap(),
        Grid::new(3, 2, vec![2, 2, 0, 1, 3, 0]).unwrap(),
    ]
}

#[test]
fn each_symmetry_states_a_rule_the_judge_closes() {
    for op in [Op::Rotate90, Op::Rotate180, Op::Rotate270, Op::FlipH, Op::FlipV, Op::Transpose] {
        let program = vec![op.clone()];
        let fam = family(&program, &inputs());
        let rule = translate(&program, &fam, &NoCrystal).unwrap_or_else(|why| panic!("{op:?}: {why:?}"));
        assert!(would_judge(&rule, &fam, &|_| None, &NoCrystal).is_ok(), "{op:?}: the judge's own steps close it");
    }
}

#[test]
fn a_composed_word_with_a_recolor_states_one_rule() {
    let mut map = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    map.swap(1, 2);
    let program = vec![Op::Rotate90, Op::Recolor(map)];
    let fam = family(&program, &inputs());
    let rule = translate(&program, &fam, &NoCrystal).expect("translates");
    assert!(would_judge(&rule, &fam, &|_| None, &NoCrystal).is_ok());
}

#[test]
fn a_tile_or_object_op_is_not_a_rule_and_is_counted_as_such() {
    let program = vec![Op::Tile { down: 2, across: 1, odd_row: solver::TileMirror::None, odd_col: solver::TileMirror::None }];
    let fam = family(&program, &inputs());
    assert_eq!(translate(&program, &fam, &NoCrystal).unwrap_err(), Untranslated::NotARule);
}
