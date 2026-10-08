//! **THE TONGUE** (docs/60 D10b) — netstratum's words stated in the fleet's
//! language: a solver program as a mycelium v0.1.0 rule element, built ONLY
//! through mycelium's own constructors (`sdk::rule`), never forged bytes.
//! The seat states its answers through it; the conductor states each line's
//! words through it, so a line's dialect can be read against the shared
//! tongue.

pub use mycelium;

use mycelium::sdk::rule::{sentence, Step, Symmetry};
use mycelium::transducer::element::Element;
use mycelium::transducer::family::{Crystal, NoCrystal};
use solver::Op;

/// A fleet grid read as a netstratum grid, or `None` (ragged, empty, or a hue
/// past the ten-hue ladder).
fn grid(g: &[Vec<u8>]) -> Option<solver::Grid> {
    let rows = g.len();
    let cols = g.first()?.len();
    if g.iter().any(|r| r.len() != cols) {
        return None;
    }
    solver::Grid::new(rows, cols, g.concat())
}

fn rows(g: &solver::Grid) -> Vec<Vec<u8>> {
    g.cells.chunks(g.cols.max(1)).map(|r| r.to_vec()).collect()
}

/// THE TRANSLATOR (Concordance 1.9/1.10; mycelium 78.142 `sdk::rule`): a
/// netstratum program as the court's rule. Each of the six symmetries is the
/// centred rigid motion `Step::grid_motion` builds for the shapes it acts
/// between (traced through the program on the family's first input);
/// `Recolor` is a hue step naming every hue it moves. A program with an
/// object op or a tile has no form yet: `Err(NotARule)`. The steps are minted
/// through the one mint over the family (`rule::sentence`).
pub fn translate(
    program: &solver::Program,
    train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)],
    crystal: &dyn Crystal,
) -> Result<Element, Untranslated> {
    let first = train.first().and_then(|(a, _)| grid(a)).ok_or(Untranslated::NoFamily)?;
    let mut at = first;
    let mut steps = Vec::with_capacity(program.len());
    for op in program {
        let next = solver::grid::run(&vec![op.clone()], &at);
        let kind = match op {
            Op::Rotate90 => Some(Symmetry::Rotate90),
            Op::Rotate180 => Some(Symmetry::Rotate180),
            Op::Rotate270 => Some(Symmetry::Rotate270),
            Op::FlipH => Some(Symmetry::FlipH),
            Op::FlipV => Some(Symmetry::FlipV),
            Op::Transpose => Some(Symmetry::Transpose),
            Op::Recolor(_) | Op::Object(_) | Op::Tile { .. } | Op::Shift { .. } | Op::Dilate { .. } | Op::Project(_) => None,
        };
        match (kind, op) {
            (Some(kind), _) => {
                let step = Step::grid_motion(kind, (at.rows, at.cols), (next.rows, next.cols)).ok_or(Untranslated::Shape)?;
                steps.push(step);
            }
            // docs/62 L4: a dilator is the court's Scale; a translator its
            // motion by the cell vector (the grid's own sites, no centring).
            (None, Op::Dilate { up, down }) => steps.push(Step::Scale { up: *up, down: *down }),
            (None, Op::Shift { dr, dc }) => {
                use num_rational::BigRational;
                let d = [BigRational::from_integer(num_bigint::BigInt::from(*dr)), BigRational::from_integer(num_bigint::BigInt::from(*dc))];
                steps.push(Step::Motion(mycelium::transducer::cga::translator_at(&d)));
            }
            // docs/62 L7: a projector is the court's SELECT and a hue step
            // on what it selects (a hue sent to 0 under a selection is erased).
            (None, Op::Project(rule)) => {
                steps.push(Step::Select(court_pred(&rule.select).ok_or(Untranslated::NotARule)?));
                let to = match rule.act {
                    solver::percept::Act::Recolor(h) => h,
                    solver::percept::Act::Remove => 0,
                };
                steps.push(Step::Hue((1u8..10).filter(|&k| k != to).map(|k| (k, to)).collect()));
            }
            // The court's sensor reads hue 0 as the background, not a body:
            // a map that moves 0, or sends a hue to 0, creates or erases
            // bodies and is no gauge on them (mycelium, confirmed at
            // family/perceive.rs). Painting the background is a creation;
            // erasing is a selection's complement.
            (None, Op::Recolor(map)) if map[0] != 0 || map[1..].contains(&0) => {
                return Err(Untranslated::Background);
            }
            (None, Op::Recolor(map)) => {
                let moved: Vec<(u8, u8)> = (0u8..10).filter(|&h| map[h as usize] != h).map(|h| (h, map[h as usize])).collect();
                steps.push(Step::Hue(moved));
            }
            (None, _) => return Err(Untranslated::NotARule),
        }
        at = next;
    }
    sentence(&steps, train, crystal).map_err(|why| Untranslated::Refused(format!("{why:?}")))
}

/// A projector's predicate in the court's SELECT language, where it has one:
/// size is the body's cell count (the court's Shell); the frame-contact,
/// twin and exact-shape predicates have no court form yet.
fn court_pred(select: &solver::percept::Select) -> Option<mycelium::sdk::rule::Pred> {
    use mycelium::sdk::rule::{Coord, Order, Pred, Tie};
    use solver::percept::Select;
    Some(match select {
        Select::Hue(h) => Pred::Hue(*h),
        Select::Size(k) => Pred::count(Coord::Shell, i64::try_from(*k).ok()?),
        Select::Largest => Pred::extremum(Coord::Shell, Order::Greatest, Tie::Hold),
        Select::Smallest => Pred::extremum(Coord::Shell, Order::Least, Tie::Hold),
        Select::Not(inner) => Pred::Not(Box::new(court_pred(inner)?)),
        Select::And(a, b) => Pred::And(vec![court_pred(a)?, court_pred(b)?]),
        Select::Border(_) | Select::TwinOfOtherHue | Select::Shape(_) => return None,
    })
}

/// Why a program could not be stated as a court rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Untranslated {
    /// An object op or a tile: no form in the court's language yet.
    NotARule,
    /// The family has no first pair to trace shapes on.
    NoFamily,
    /// A symmetry met shapes it cannot act between.
    Shape,
    /// A hue map that moves the background (hue 0) or sends a hue to it:
    /// a creation or an erasure, not a gauge on bodies.
    Background,
    /// The seam refused it, by name.
    Refused(String),
}

/// A LINE'S WORD IN THE SHARED TONGUE: the word stated on its own action (the
/// ramp probe, every hue present, and what the word makes of it), answered
/// by the blake3 of the element's address, or `None` where the tongue cannot
/// state it (an object op or a tile, or the mint refused).
pub fn state_word(word: &[Op]) -> Option<[u8; 32]> {
    let probe = solver::Grid::new(3, 4, (0..12u8).map(|k| k % 10).collect())?;
    let said = solver::grid::run(&word.to_vec(), &probe);
    let train = vec![(rows(&probe), rows(&said))];
    let element = translate(&word.to_vec(), &train, &NoCrystal).ok()?;
    Some(*blake3::hash(element.address().as_bytes()).as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_motion_and_hue_word_is_stated_and_an_object_word_is_not() {
        let mut map = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        map.swap(1, 2);
        assert!(state_word(&[Op::Rotate180, Op::Recolor(map)]).is_some());
        assert!(state_word(&[Op::FlipH, Op::FlipV]).is_some());
        assert!(state_word(&[Op::Object(solver::ObjOp::KeepLargest), Op::FlipH]).is_none());
    }
}
