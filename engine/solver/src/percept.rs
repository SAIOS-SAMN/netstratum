//! **RESTORE 10** (Concordance 1.10, the audit approved 2026-10-08): a mind
//! PERCEIVES objects, counts and extrema — the developer standard that kept
//! quantity out of perception (`reduct/peel.rs`: "never arithmetic content")
//! binds developers, not minds.
//!
//! A grid is perceived as BODIES (4-connected, one hue each, the grid's own
//! background left out) with the features the court's rule language selects
//! on (mycelium 78.143 SELECT, as announced): hue, cell count, shape up to
//! translation, contact with the frame. Counts are exact; an extremum names
//! its order and keeps its ties ("the largest" is every body of the largest
//! count — the rule states it, nothing is broken silently).
//!
//! On top of perception, THE SELECTION A TASK SHOWS: across every training
//! pair, which bodies changed and how (recoloured to one hue, or removed),
//! and the first predicate — in a fixed, stated order — that picks exactly
//! those bodies in every pair. The rule is then applied to the test input.
//! It is thought, judged at home by closing every pair (Pin A), and stated to
//! the court only through its rule language (the translator, once 78.143's
//! steps land); until then a closed object rule is counted, never sent.

pub use solver_verify::project::{apply, perceive, Act, Body, ObjectRule, Percept, Select};
use crate::grid::Grid;

/// What became of one body between a pair's input and output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fate {
    Kept,
    Recolored(u8),
    Removed,
}

/// Each input body's fate in one pair, or `None` where the pair changes
/// anything a body-wise reading cannot say (a frame change, a cell outside
/// every body, a body changed unevenly).
fn fates(input: &Grid, output: &Grid, p: &Percept) -> Option<Vec<Fate>> {
    if (input.rows, input.cols) != (output.rows, output.cols) {
        return None;
    }
    let mut owned = vec![false; input.cells.len()];
    let mut out = Vec::with_capacity(p.bodies.len());
    for b in &p.bodies {
        let now: Vec<u8> = b.cells.iter().map(|&(r, c)| output.cells[r * input.cols + c]).collect();
        for &(r, c) in &b.cells {
            owned[r * input.cols + c] = true;
        }
        let first = now[0];
        if now.iter().any(|&h| h != first) {
            return None;
        }
        out.push(if first == b.hue {
            Fate::Kept
        } else if first == p.background {
            Fate::Removed
        } else {
            Fate::Recolored(first)
        });
    }
    // Every cell outside the bodies stands still.
    if owned.iter().enumerate().any(|(i, &own)| !own && input.cells[i] != output.cells[i]) {
        return None;
    }
    Some(out)
}

/// The predicates a mind tries, in order, built from what the pairs show.
fn candidates(seen: &[(Percept, Vec<Fate>)]) -> Vec<Select> {
    let mut base = vec![Select::Largest, Select::Smallest, Select::Central, Select::Border(true), Select::Border(false), Select::TwinOfOtherHue];
    for h in 0..10u8 {
        base.push(Select::Hue(h));
    }
    let mut sizes: Vec<usize> = Vec::new();
    let mut shapes: Vec<Vec<(usize, usize)>> = Vec::new();
    for (p, fate) in seen {
        for (b, f) in p.bodies.iter().zip(fate) {
            if *f != Fate::Kept {
                if !sizes.contains(&b.size()) {
                    sizes.push(b.size());
                }
                if !shapes.contains(&b.shape) {
                    shapes.push(b.shape.clone());
                }
            }
        }
    }
    sizes.sort_unstable();
    base.extend(sizes.into_iter().map(Select::Size));
    base.extend(shapes.into_iter().map(Select::Shape));
    let negated: Vec<Select> = base.iter().cloned().map(|s| Select::Not(Box::new(s))).collect();
    let singles: Vec<Select> = base.iter().cloned().chain(negated).collect();
    // Then every pair of them, conjoined — tried after every single one.
    let mut all = singles.clone();
    for (i, a) in singles.iter().enumerate() {
        for b in &singles[i + 1..] {
            all.push(Select::And(Box::new(a.clone()), Box::new(b.clone())));
        }
    }
    all
}

/// WHY a task has no object rule, by its first stage that fails — the
/// diagnostic face of [`learn`].
pub fn why_not(train: &[(Grid, Grid)]) -> &'static str {
    let mut act: Option<Act> = None;
    for (input, output) in train {
        if (input.rows, input.cols) != (output.rows, output.cols) {
            return "the frame changes";
        }
        let p = perceive(input);
        let Some(fate) = fates(input, output, &p) else {
            return "a body changes unevenly, or a cell outside every body changes";
        };
        for f in fate {
            let this = match f {
                Fate::Kept => continue,
                Fate::Recolored(h) => Act::Recolor(h),
                Fate::Removed => Act::Remove,
            };
            match &act {
                None => act = Some(this),
                Some(a) if *a == this => {}
                Some(_) => return "the bodies change in more than one way",
            }
        }
    }
    if act.is_none() {
        return "no body changes";
    }
    if learn(train).is_some() {
        return "closed";
    }
    "no predicate picks the changed bodies"
}

/// THE SELECTION A TASK SHOWS: the object rule every training pair agrees
/// on, closing every pair exactly, or `None`.
pub fn learn(train: &[(Grid, Grid)]) -> Option<ObjectRule> {
    if train.is_empty() {
        return None;
    }
    let mut seen = Vec::with_capacity(train.len());
    let mut act: Option<Act> = None;
    for (input, output) in train {
        let p = perceive(input);
        let fate = fates(input, output, &p)?;
        for f in &fate {
            let this = match f {
                Fate::Kept => continue,
                Fate::Recolored(h) => Act::Recolor(*h),
                Fate::Removed => Act::Remove,
            };
            match &act {
                None => act = Some(this),
                Some(a) if *a == this => {}
                Some(_) => return None, // two different acts: not one rule
            }
        }
        seen.push((p, fate));
    }
    let act = act?; // nothing changed anywhere: no rule to learn
    for select in candidates(&seen) {
        let holds = seen.iter().all(|(p, fate)| {
            (0..p.bodies.len()).all(|i| select.picks(p, i) == (fate[i] != Fate::Kept))
        });
        if holds {
            let rule = ObjectRule { select, act: act.clone() };
            // Pin A: the rule closes every pair exactly.
            if train.iter().all(|(i, o)| apply(&rule, i) == *o) {
                return Some(rule);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(rows: usize, cols: usize, cells: &[u8]) -> Grid {
        Grid::new(rows, cols, cells.to_vec()).expect("a grid")
    }

    #[test]
    fn the_largest_body_is_recoloured_and_the_rule_states_its_ties() {
        // the largest body (4 cells) turns 2; the smaller ones stay
        let a = g(4, 5, &[1, 1, 0, 3, 0, 1, 1, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0]);
        let a2 = apply(&ObjectRule { select: Select::Largest, act: Act::Recolor(2) }, &a);
        let b = g(4, 5, &[0, 0, 0, 0, 0, 0, 5, 5, 5, 0, 0, 0, 5, 0, 0, 6, 0, 0, 0, 0]);
        let b2 = apply(&ObjectRule { select: Select::Largest, act: Act::Recolor(2) }, &b);
        let rule = learn(&[(a, a2), (b, b2)]).expect("learned");
        assert_eq!(rule.act, Act::Recolor(2));
        assert_eq!(rule.select, Select::Largest);
    }

    #[test]
    fn the_central_body_is_learned_as_a_commission_marks_it() {
        // docs/66 Y2: lone hue-1 cells spaced apart; the judge marks the one
        // whose L1 distance to the others is least. Learned, and it crosses
        // the wire whole.
        let mark = |x: &Grid| apply(&ObjectRule { select: Select::Central, act: Act::Recolor(2) }, x);
        let a = g(5, 5, &[1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0]);
        let b = g(5, 7, &[0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let (a2, b2) = (mark(&a), mark(&b));
        assert_eq!(a2.at(2, 2), 2);
        let rule = learn(&[(a, a2), (b, b2)]).expect("learned");
        assert_eq!(rule.select, Select::Central);
        let program = vec![solver_verify::grid::Op::Project(rule)];
        let bytes = solver_verify::program_bytes(&program).expect("encodes");
        assert_eq!(solver_verify::program_from_bytes(&bytes), Some(program));
    }

    #[test]
    fn a_shape_matching_its_template_is_selected() {
        // a 1-hue template and a same-shape 5-hue copy: the copy turns 2
        let a = g(4, 6, &[1, 1, 0, 5, 5, 0, 1, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 5]);
        let a2 = g(4, 6, &[1, 1, 0, 2, 2, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 5]);
        let rule = learn(&[(a.clone(), a2.clone()), (a, a2)]).expect("learned");
        assert_eq!(rule.act, Act::Recolor(2));
    }

    #[test]
    fn two_different_acts_are_not_one_rule() {
        let a = g(1, 3, &[1, 0, 2]);
        let out = g(1, 3, &[3, 0, 0]);
        assert!(learn(&[(a, out)]).is_none());
    }
}
