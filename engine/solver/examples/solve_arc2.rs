//! Measure the harness against the REAL ARC-AGI-2 dataset (official
//! JSON layout: {"train":[{input,output}...],"test":[{input,output}...]}).
//! A task is CONFIRMED only if one program closes every training pair
//! AND its render matches EVERY held-out test output — the official
//! scoring shape, no partial credit.
//!
//! Usage: cargo run -p solver --release --example solve_arc2 -- <dir> [max_depth]

use solver::{solve, Grid, SolveRead, Task, TaskPair};

/// A minimal JSON reader for exactly this machine-written schema:
/// objects, arrays, and small non-negative integers. No general JSON
/// claims — the files are the official dataset's own emissions.
struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            bytes: text.as_bytes(),
            at: 0,
        }
    }

    fn skip_ws(&mut self) {
        while self
            .bytes
            .get(self.at)
            .is_some_and(|b| b.is_ascii_whitespace() || *b == b',' || *b == b':')
        {
            self.at += 1;
        }
    }

    fn expect(&mut self, byte: u8) -> Option<()> {
        self.skip_ws();
        (self.bytes.get(self.at) == Some(&byte)).then(|| self.at += 1)
    }

    fn peek(&mut self) -> Option<u8> {
        self.skip_ws();
        self.bytes.get(self.at).copied()
    }

    fn key(&mut self) -> Option<String> {
        self.expect(b'"')?;
        let start = self.at;
        while self.bytes.get(self.at) != Some(&b'"') {
            self.at += 1;
        }
        let key = String::from_utf8_lossy(&self.bytes[start..self.at]).into_owned();
        self.at += 1;
        Some(key)
    }

    fn number(&mut self) -> Option<u8> {
        self.skip_ws();
        let start = self.at;
        while self.bytes.get(self.at).is_some_and(u8::is_ascii_digit) {
            self.at += 1;
        }
        std::str::from_utf8(&self.bytes[start..self.at])
            .ok()?
            .parse()
            .ok()
    }

    /// `[[1,2],[3,4]]` → Grid.
    fn grid(&mut self) -> Option<Grid> {
        self.expect(b'[')?;
        let mut rows: Vec<Vec<u8>> = Vec::new();
        while self.peek() == Some(b'[') {
            self.expect(b'[')?;
            let mut row = Vec::new();
            while self.peek() != Some(b']') {
                row.push(self.number()?);
            }
            self.expect(b']')?;
            rows.push(row);
        }
        self.expect(b']')?;
        let (r, c) = (rows.len(), rows.first()?.len());
        Grid::new(r, c, rows.concat())
    }

    /// `[{"input":G,"output":G}, ...]` → pairs (output optional on test).
    fn pairs(&mut self) -> Option<Vec<(Grid, Option<Grid>)>> {
        self.expect(b'[')?;
        let mut out = Vec::new();
        while self.peek() == Some(b'{') {
            self.expect(b'{')?;
            let (mut input, mut output) = (None, None);
            while self.peek() == Some(b'"') {
                match self.key()?.as_str() {
                    "input" => input = Some(self.grid()?),
                    "output" => output = Some(self.grid()?),
                    _ => return None,
                }
            }
            self.expect(b'}')?;
            out.push((input?, output));
        }
        self.expect(b']')?;
        Some(out)
    }
}

/// One official task file → (Task for the solver, ALL test pairs).
fn load(path: &std::path::Path) -> Option<(Task, Vec<(Grid, Grid)>)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut r = Reader::new(&text);
    r.expect(b'{')?;
    let (mut train, mut test) = (None, None);
    while r.peek() == Some(b'"') {
        match r.key()?.as_str() {
            "train" => train = Some(r.pairs()?),
            "test" => test = Some(r.pairs()?),
            _ => return None,
        }
    }
    let train: Vec<TaskPair> = train?
        .into_iter()
        .map(|(input, output)| {
            Some(TaskPair {
                input,
                output: output?,
            })
        })
        .collect::<Option<_>>()?;
    let tests: Vec<(Grid, Grid)> = test?
        .into_iter()
        .map(|(input, output)| Some((input, output?)))
        .collect::<Option<_>>()?;
    let name = path.file_stem()?.to_string_lossy().into_owned();
    let first_test = tests.first()?.0.clone();
    Some((
        Task {
            name,
            train,
            test_in: first_test,
            test_expected: None, // scored below across ALL tests, not by the solver
        },
        tests,
    ))
}

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = args.next().expect("usage: solve_arc2 <dir> [max_depth]");
    let max_depth: usize = args.next().map(|d| d.parse().expect("depth")).unwrap_or(3);

    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("dataset dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    paths.sort();

    let (mut attempted, mut closed, mut confirmed) = (0u32, 0u32, 0u32);
    for path in &paths {
        let Some((task, tests)) = load(path) else {
            eprintln!("skip {path:?}: unreadable");
            continue;
        };
        attempted += 1;
        if let SolveRead::Solved { program, .. } = solve(&task, max_depth) {
            closed += 1;
            let all_match = tests
                .iter()
                .all(|(input, want)| &solver::grid::run(&program, input) == want);
            if all_match {
                confirmed += 1;
                let word: Vec<String> = program.iter().map(|op| op.describe()).collect();
                println!("SOLVED {} via [{}]", task.name, word.join(" → "));
            }
        }
    }
    println!(
        "\nARC-AGI-2 record: {attempted} attempted · {closed} closed training · {confirmed} CONFIRMED on all held-out tests (depth {max_depth})"
    );
}
