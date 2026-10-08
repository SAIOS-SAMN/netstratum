//! **THE PROJECTOR** (docs/62 L7) — a selection is a projector onto the
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

/// WHICH BODIES A RULE SELECTS — the predicates, in the order a mind tries
/// them (a stated order; the first that holds on every pair is the rule).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Select {
    Hue(u8),
    Size(usize),
    /// Every body of the largest count (ties included).
    Largest,
    /// Every body of the smallest count (ties included).
    Smallest,
    /// Touching the frame, or not.
    Border(bool),
    /// Of the same shape (up to translation) as some OTHER body of another
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
            Select::Border(on) => b.border == *on,
            Select::TwinOfOtherHue => p.bodies.iter().enumerate().any(|(j, o)| j != i && o.hue != b.hue && o.shape == b.shape),
            Select::Shape(s) => &b.shape == s,
            Select::Not(inner) => !inner.picks(p, i),
            Select::And(a, b) => a.picks(p, i) && b.picks(p, i),
        }
    }
}

/// WHAT A RULE DOES to the bodies it selects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Act {
    Recolor(u8),
    Remove,
}

/// An object rule: SELECT, then ACT.
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

