//! docs/73 DECIDE, step 1: CAPTURE. Each decision's state is seen as a grid,
//! persistence (each row's last outcome) predicts before anything resolves,
//! the incumbent acts, and the resolved triple (state, action, outcome) is
//! booked. Behaviour is unchanged: no word acts yet.
//!
//! The grids (docs/73 "The grids"): rows are the entities by name, columns
//! the kind's features, a feature's hue its rank among the rows
//! (`rank × 10 / n` floored, ties sharing the lower rank). The outcome grid
//! has the state's frame, a row all hue 1 where that entity lost.

use cfe::ratio::Ratio;
use solver_verify::Grid;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;

/// The one dial (docs/73, DECLARED in docs/59): a book keeps at most a
/// WINDOW of triples and of persistence's scores.
pub const WINDOW: usize = 64;

/// One resolved decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Triple {
    pub state: Grid,
    /// The incumbent's act, one byte a row.
    pub action: Vec<u8>,
    pub outcome: Grid,
}

/// A (kind, scope)'s book.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Book {
    pub triples: VecDeque<Triple>,
    /// Each row's outcome at the subject's last resolved decision; `None`
    /// before the first.
    pub last: Option<BTreeMap<String, bool>>,
    /// Persistence's record, (correct rows, rows) a decision.
    pub persistence: VecDeque<(u32, u32)>,
}

/// What one resolution did, for the log.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Resolved {
    pub rows: u32,
    pub lost: u32,
    /// Persistence's correct rows and rows scored (none at a first decision).
    pub correct: u32,
    pub scored: u32,
}

/// A feature column's hues: each value's rank among the rows (the count of
/// strictly smaller values), times 10, over the rows, floored.
fn hues(values: &[Ratio]) -> Vec<u8> {
    let n = values.len().max(1);
    values.iter().map(|v| (values.iter().filter(|w| *w < v).count() * 10 / n) as u8).collect()
}

/// SEE: the state grid of rows (each a feature vector, all of one width).
pub fn see(rows: &[Vec<Ratio>]) -> Option<Grid> {
    let cols = rows.first()?.len();
    let columns: Vec<Vec<u8>> = (0..cols).map(|c| hues(&rows.iter().map(|r| r[c].clone()).collect::<Vec<_>>())).collect();
    let cells = (0..rows.len()).flat_map(|r| columns.iter().map(move |col| col[r])).collect();
    Grid::new(rows.len(), cols, cells)
}

impl Book {
    /// PREDICT (persistence) and RECORD in one: the rows by name, the state
    /// seen before the act, the incumbent's act and the rows that lost.
    pub fn resolve(&mut self, names: &[String], state: Grid, action: Vec<u8>, lost: &BTreeSet<String>) -> Resolved {
        let mut done = Resolved { rows: names.len() as u32, lost: names.iter().filter(|n| lost.contains(*n)).count() as u32, ..Resolved::default() };
        if let Some(last) = &self.last {
            done.scored = done.rows;
            done.correct = names.iter().filter(|n| last.get(*n).copied().unwrap_or(false) == lost.contains(*n)).count() as u32;
            self.persistence.push_back((done.correct, done.scored));
            if self.persistence.len() > WINDOW {
                self.persistence.pop_front();
            }
        }
        let cells = names.iter().flat_map(|n| std::iter::repeat_n(lost.contains(n) as u8, state.cols)).collect();
        if let Some(outcome) = Grid::new(state.rows, state.cols, cells) {
            self.triples.push_back(Triple { state, action, outcome });
            if self.triples.len() > WINDOW {
                self.triples.pop_front();
            }
        }
        self.last = Some(names.iter().map(|n| (n.clone(), lost.contains(n))).collect());
        done
    }

    /// Persistence's hit rate over its window, (correct, rows).
    pub fn persistence_record(&self) -> (u32, u32) {
        self.persistence.iter().fold((0, 0), |(c, r), (a, b)| (c + a, r + b))
    }
}

pub type Books = BTreeMap<(String, String), Book>;

fn put_text(text: &str, out: &mut Vec<u8>) {
    out.extend_from_slice(&(text.len() as u16).to_le_bytes());
    out.extend_from_slice(text.as_bytes());
}

fn get_text(value: &[u8], at: &mut usize) -> Option<String> {
    let len = get_u16(value, at)? as usize;
    let s = String::from_utf8(value.get(*at..*at + len)?.to_vec()).ok()?;
    *at += len;
    Some(s)
}

fn get_u16(value: &[u8], at: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(value.get(*at..*at + 2)?.try_into().ok()?);
    *at += 2;
    Some(v)
}

fn get_u32(value: &[u8], at: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(value.get(*at..*at + 4)?.try_into().ok()?);
    *at += 4;
    Some(v)
}

fn put_grid(g: &Grid, out: &mut Vec<u8>) {
    out.extend_from_slice(&(g.rows as u16).to_le_bytes());
    out.extend_from_slice(&(g.cols as u16).to_le_bytes());
    out.extend_from_slice(&g.cells);
}

fn get_grid(value: &[u8], at: &mut usize) -> Option<Grid> {
    let (rows, cols) = (get_u16(value, at)? as usize, get_u16(value, at)? as usize);
    let cells = value.get(*at..*at + rows * cols)?.to_vec();
    *at += rows * cols;
    Grid::new(rows, cols, cells)
}

fn decode(v: &[u8]) -> Option<((String, String), Book)> {
    let mut at = 0;
    let (kind, scope) = (get_text(v, &mut at)?, get_text(v, &mut at)?);
    let mut book = Book::default();
    if *v.get(at)? == 1 {
        at += 1;
        let mut last = BTreeMap::new();
        for _ in 0..get_u16(v, &mut at)? {
            let name = get_text(v, &mut at)?;
            last.insert(name, *v.get(at)? == 1);
            at += 1;
        }
        book.last = Some(last);
    } else {
        at += 1;
    }
    for _ in 0..get_u16(v, &mut at)? {
        book.persistence.push_back((get_u32(v, &mut at)?, get_u32(v, &mut at)?));
    }
    for _ in 0..get_u16(v, &mut at)? {
        let state = get_grid(v, &mut at)?;
        let action = v.get(at..at + state.rows)?.to_vec();
        at += state.rows;
        let outcome = get_grid(v, &mut at)?;
        book.triples.push_back(Triple { state, action, outcome });
    }
    Some(((kind, scope), book))
}

fn encode(kind: &str, scope: &str, book: &Book, out: &mut Vec<u8>) {
    let mut v = Vec::new();
    put_text(kind, &mut v);
    put_text(scope, &mut v);
    match &book.last {
        Some(last) => {
            v.push(1);
            v.extend_from_slice(&(last.len() as u16).to_le_bytes());
            for (name, lost) in last {
                put_text(name, &mut v);
                v.push(*lost as u8);
            }
        }
        None => v.push(0),
    }
    v.extend_from_slice(&(book.persistence.len() as u16).to_le_bytes());
    for (c, r) in &book.persistence {
        v.extend_from_slice(&c.to_le_bytes());
        v.extend_from_slice(&r.to_le_bytes());
    }
    v.extend_from_slice(&(book.triples.len() as u16).to_le_bytes());
    for t in &book.triples {
        put_grid(&t.state, &mut v);
        v.extend_from_slice(&t.action);
        put_grid(&t.outcome, &mut v);
    }
    cfe::tlv::put(1, &v, out);
}

/// The books, from `decide-books.bin` (a book that does not decode is
/// dropped, never guessed).
pub fn read(var_root: &Path) -> Books {
    let bytes = std::fs::read(var_root.join("decide-books.bin")).unwrap_or_default();
    cfe::tlv::records(&bytes).filter(|(t, _)| *t == 1).filter_map(|(_, v)| decode(v)).collect()
}

pub fn write(var_root: &Path, books: &Books) {
    let mut out = Vec::new();
    for ((kind, scope), book) in books {
        encode(kind, scope, book, &mut out);
    }
    let path = var_root.join("decide-books.bin");
    let staged = path.with_extension("staged");
    if std::fs::write(&staged, out).is_ok() {
        let _ = std::fs::rename(staged, path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cfe::ratio::ratio;

    #[test]
    fn a_book_survives_its_file() {
        let names = vec!["a".to_string(), "b".to_string()];
        let state = see(&[vec![ratio(1, 2), ratio(3, 1)], vec![ratio(1, 2), ratio(1, 1)]]).unwrap();
        assert_eq!(state.cells, vec![0, 5, 0, 0]);
        let mut book = Book::default();
        let lost: BTreeSet<String> = ["b".to_string()].into();
        assert_eq!(book.resolve(&names, state.clone(), vec![0, 1], &lost).scored, 0);
        let again = book.resolve(&names, state, vec![0, 0], &BTreeSet::new());
        assert_eq!((again.correct, again.scored), (1, 2));
        let dir = std::env::temp_dir().join(format!("decide-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let books: Books = [(("hull".to_string(), "g".to_string()), book)].into();
        write(&dir, &books);
        assert_eq!(read(&dir), books);
        let _ = std::fs::remove_dir_all(dir);
    }
}
