//! The corpus loader — pure IO, no cognition (v0.4 doc 17 §1: the
//! loader never knows a rule). Parses the provenance-verified pair
//! format:
//!
//! ```text
//! name: arc_00576224
//! kind: pair
//! grid: train1_in 2 2 7 9 4 3
//! grid: train1_out 6 6 ...
//! grid: test_in ...
//! grid: test_expected ...
//! ```

use crate::grid::Grid;
use std::path::Path;

/// One training pair: the observed transformation instance.
#[derive(Debug, Clone)]
pub struct TaskPair {
    pub input: Grid,
    pub output: Grid,
}

/// One task: training pairs + the held-out test.
#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub train: Vec<TaskPair>,
    pub test_in: Grid,
    /// Present in the archived corpus; the solver never reads it
    /// during search — it is the verification target only.
    pub test_expected: Option<Grid>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    Io(String),
    Malformed(String),
}

fn parse_grid(words: &[&str], label: &str) -> Result<Grid, LoadError> {
    let bad = |what: &str| LoadError::Malformed(format!("{label}: {what}"));
    if words.len() < 2 {
        return Err(bad("missing dimensions"));
    }
    let rows: usize = words[0].parse().map_err(|_| bad("rows"))?;
    let cols: usize = words[1].parse().map_err(|_| bad("cols"))?;
    let cells: Result<Vec<u8>, _> = words[2..].iter().map(|w| w.parse::<u8>()).collect();
    let cells = cells.map_err(|_| bad("cells"))?;
    if cells.iter().any(|&h| h > 9) {
        return Err(bad("hue out of range"));
    }
    Grid::new(rows, cols, cells).ok_or_else(|| bad("dimension mismatch"))
}

/// Load one task file.
pub fn load_task(path: &Path) -> Result<Task, LoadError> {
    let text =
        std::fs::read_to_string(path).map_err(|e| LoadError::Io(format!("{path:?}: {e}")))?;
    let mut name = String::new();
    let mut grids: Vec<(String, Grid)> = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("name:") {
            name = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("grid:") {
            let words: Vec<&str> = rest.split_whitespace().collect();
            let (label, dims) = words
                .split_first()
                .ok_or_else(|| LoadError::Malformed("empty grid line".into()))?;
            grids.push((label.to_string(), parse_grid(dims, label)?));
        }
    }

    let take = |label: &str| grids.iter().find(|(l, _)| l == label).map(|(_, g)| g.clone());
    let mut train = Vec::new();
    for i in 1.. {
        match (take(&format!("train{i}_in")), take(&format!("train{i}_out"))) {
            (Some(input), Some(output)) => train.push(TaskPair { input, output }),
            (None, None) => break,
            _ => return Err(LoadError::Malformed(format!("train{i} pair incomplete"))),
        }
    }
    if train.is_empty() {
        return Err(LoadError::Malformed("no training pairs".into()));
    }
    let test_in = take("test_in").ok_or_else(|| LoadError::Malformed("no test_in".into()))?;

    Ok(Task {
        name,
        train,
        test_in,
        test_expected: take("test_expected"),
    })
}
