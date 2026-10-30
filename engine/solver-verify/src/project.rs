//! **The projector** (docs/62 L7) — a selection is a projector onto the
//! bodies a predicate picks, and an act on what it picked. The verifier
//! carries it whole (`Op::Project`), so a learned selection is a word like
//! any other: certified, minted, replayed. Moved here from the solver's
//! restore-10 percepts, which keep the learning.

use crate::grid::Grid;
use crate::objects::{background, segment};

/// One perceived body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body {
    pub cells: Vec<(usize, usize)>,
    pub hue: u8,
    /// Its shape up to translation: cells relative to its top-left, sorted.
    pub shape: Vec<(usize, usize)>,
    /// Whether it touches the grid's frame.
    pub border: bool,
}

impl Body {
    pub fn size(&self) -> usize {
        self.cells.len()
    }
}

/// A grid perceived: its background and its bodies.
#[derive(Debug, Clone)]
pub struct Percept {
    pub background: u8,
    pub bodies: Vec<Body>,
}

/// Perceive a grid as bodies.
pub fn perceive(g: &Grid) -> Percept {
    let bg = background(g);
    let bodies = segment(g, bg, true)
        .into_iter()
        .map(|o| {
            let hue = o.cells[0].2;
            let mut shape: Vec<(usize, usize)> = o.cells.iter().map(|&(r, c, _)| (r - o.top, c - o.left)).collect();
            shape.sort_unstable();
            let border = o.top == 0 || o.left == 0 || o.bottom + 1 == g.rows || o.right + 1 == g.cols;
            Body { cells: o.cells.iter().map(|&(r, c, _)| (r, c)).collect(), hue, shape, border }
        })
        .collect();
    Percept { background: bg, bodies }
}

/// Which bodies A RULE selects — the predicates, in the order a mind tries
/// them (a stated order; the first that holds on every pair is the rule).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Select {
    Hue(u8),
    Size(usize),
    /// Every body of the largest count (ties included).
    Largest,
    /// Every body of the smallest count (ties included).
    Smallest,
    /// The order A relation shows (docs/66 Y2, the faculty; coordinator ruling
    /// 2026-10-09): every body extreme (`Extreme`) in the integral of its
    /// sorted L1 relations to the other bodies, read at a `Cut`. Deduced by
    /// `solver::percept`, never supplied: `Central` is its case
    /// `Ordered { cut: All, extreme: Least }`.
    Ordered { cut: Cut, extreme: Extreme },
    /// Supplied (coordinator audit 2026-10-09): the hub-siting commission's
    /// measure handed to the language before the minds could deduce it. Kept so
    /// the words minted with it still read; the learner no longer offers it.
    /// The central bodies (docs/66 Y2, the first commission's generator): every
    /// body whose L1 distance to all the others, summed, is least (ties
    /// included). A body's distance to another is the least L1 distance
    /// between their cells. Declared by the hub-siting commission as its
    /// judge's own measure; in the algebra from then on, for every line.
    Central,
    /// Touching the frame, or not.
    Border(bool),
    /// Of the same shape (up to translation) as some other body of another
    /// hue in the same grid — a template it copies.
    TwinOfOtherHue,
    /// Of this exact shape.
    Shape(Vec<(usize, usize)>),
    Not(Box<Select>),
    /// Both predicates hold.
    And(Box<Select>, Box<Select>),
}

impl Select {
    /// Does the predicate pick body `i` of `p`?
    pub fn picks(&self, p: &Percept, i: usize) -> bool {
        let b = &p.bodies[i];
        match self {
            Select::Hue(h) => b.hue == *h,
            Select::Size(k) => b.size() == *k,
            Select::Largest => p.bodies.iter().map(Body::size).max() == Some(b.size()),
            Select::Smallest => p.bodies.iter().map(Body::size).min() == Some(b.size()),
            Select::Ordered { cut, extreme } => {
                let values: Vec<usize> = relations(p).iter().map(|r| integral(r, *cut)).collect();
                let target = match extreme {
                    Extreme::Least => values.iter().min(),
                    Extreme::Most => values.iter().max(),
                };
                target == Some(&values[i])
            }
            Select::Central => {
                let spread: Vec<usize> = (0..p.bodies.len()).map(|k| spread(p, k)).collect();
                spread.iter().min() == Some(&spread[i])
            }
            Select::Border(on) => b.border == *on,
            Select::TwinOfOtherHue => p.bodies.iter().enumerate().any(|(j, o)| j != i && o.hue != b.hue && o.shape == b.shape),
            Select::Shape(s) => &b.shape == s,
            Select::Not(inner) => !inner.picks(p, i),
            Select::And(a, b) => a.picks(p, i) && b.picks(p, i),
        }
    }
}

/// The L1 distance between two bodies: the least over their cells.
fn gap(a: &Body, b: &Body) -> usize {
    a.cells
        .iter()
        .flat_map(|&(r, c)| b.cells.iter().map(move |&(s, d)| r.abs_diff(s) + c.abs_diff(d)))
        .min()
        .unwrap_or(0)
}

/// Where an ordered reading is cut: the integral of a body's sorted relations
/// taken whole, over its `k` nearest, or over its `k` farthest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cut {
    All,
    Nearest(u16),
    Farthest(u16),
}

/// Which end of an order a rule selects (ties kept).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Extreme {
    Least,
    Most,
}

/// The sense (coordinator ruling 2026-10-09): each body's raw L1 relations to
/// every other body, as measured, sorted nearest first. Never pre-summed.
pub fn relations(p: &Percept) -> Vec<Vec<usize>> {
    (0..p.bodies.len())
        .map(|i| {
            let mut r: Vec<usize> = p.bodies.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, o)| gap(&p.bodies[i], o)).collect();
            r.sort_unstable();
            r
        })
        .collect()
}

/// The integral (the partial sum, the coboundary run backwards) of one body's
/// sorted relations, read at `cut`.
pub fn integral(relation: &[usize], cut: Cut) -> usize {
    match cut {
        Cut::All => relation.iter().sum(),
        Cut::Nearest(k) => relation.iter().take(usize::from(k)).sum(),
        Cut::Farthest(k) => relation.iter().rev().take(usize::from(k)).sum(),
    }
}

/// Body `i`'s L1 distance to every other body, summed (`Select::Central`).
pub fn spread(p: &Percept, i: usize) -> usize {
    p.bodies.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, o)| gap(&p.bodies[i], o)).sum()
}

/// What A RULE does to the bodies it selects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Act {
    Recolor(u8),
    Remove,
}

/// An object rule: select, then act.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectRule {
    pub select: Select,
    pub act: Act,
}

/// Apply a rule to a grid.
pub fn apply(rule: &ObjectRule, g: &Grid) -> Grid {
    let p = perceive(g);
    let mut out = g.clone();
    for i in 0..p.bodies.len() {
        if rule.select.picks(&p, i) {
            let hue = match rule.act {
                Act::Recolor(h) => h,
                Act::Remove => p.background,
            };
            for &(r, c) in &p.bodies[i].cells {
                out.cells[r * g.cols + c] = hue;
            }
        }
    }
    out
}

