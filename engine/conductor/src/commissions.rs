//! Commissions (docs/66 Y2): the universe's real needs as tasks, each with its
//! own judge, bountied by the lines that need them. A commission declares its
//! judge's own measure as a generator of the language (operator 2026-10-09,
//! option 2), so the minds can compose it; it closes when a mind's word
//! reproduces the judge's exact answer on the commission's pairs.
//!
//! **The first commission: where a galaxy's hub stands.** The need is read off
//! the world: a galaxy's open lines on its disc plane (each line's offset from
//! its core, on the two disc axes, spaced so no two lines touch), one hue-1 body
//! a line. The judge marks the central bodies (`Select::Central`, the bodies
//! whose L1 distance to all the others is least) in hue 2. The train pairs are
//! other galaxies' planes with their judged answers; the test is this galaxy's.
//!
//! - **One attempt a beat** across the universe: the galaxies in turn, and in
//!   each its lines in turn. The line tries its own words first, then thinks.
//!   The train is every other galaxy's plane; an order closes only when exactly
//!   one (cut, extreme) survives the pairs (determined, never a tie-break).
//! - **The bounty**: a closure is paid by the galaxy's other open lines, the
//!   ladder's dearest closure price (the greatest `mint_bound`; a commission is
//!   not on the forge's ladder) shared among them, in STAMP over the iceberg (a
//!   walled line pays nothing).
//! - **The word decides**: while a word held by a line of the galaxy still
//!   closes the galaxy's commission, the hub stands where that word marks
//!   (`hub_of`); otherwise the commission reopens and the medoid stands in.
//!   The physics still bounds the trade.
//! - **The book** is a binary checkpoint (Concordance 1.13): per galaxy, the
//!   closer and its word.
use crate::Polity;
use cfe::ratio::{ratio, Ratio};
use ledger::Ledger;
use solver::{Difficulty, Grid, Program, SolveRead, Task, TaskPair};
use std::collections::BTreeMap;
use std::path::Path;

/// The widest a commission plane may be, in cells (an ARC frame).
const FRAME: i64 = 29;

/// A galaxy's disc plane: the grid, and each line's cell.
pub struct Plane {
    pub grid: Grid,
    pub cells: Vec<(usize, (usize, usize))>,
}

/// The wrapped offset of `x` from `c` on a torus axis of `span`, floored.
fn offset(x: &Ratio, c: &Ratio, span: &Ratio) -> i64 {
    let mut d = x.clone() - c.clone();
    let half = span.clone() / ratio(2, 1);
    while d > half {
        d -= span.clone();
    }
    while d <= -half.clone() {
        d += span.clone();
    }
    num_traits::ToPrimitive::to_i64(&d.floor().to_integer()).unwrap_or(0)
}

/// Read `members` of a galaxy centred at `centre` onto its disc plane. `None`
/// with fewer than three distinct cells (no centre to find).
pub fn plane(
    worlds: &[Polity],
    members: &[usize],
    centre: &(Ratio, Ratio, Ratio),
    span: &Ratio,
) -> Option<Plane> {
    let at: Vec<(usize, i64, i64)> = members
        .iter()
        .map(|&i| {
            let p = &worlds[i].spec.position;
            (
                i,
                offset(&p.0, &centre.0, span),
                offset(&p.1, &centre.1, span),
            )
        })
        .collect();
    let (min_r, max_r) = (at.iter().map(|a| a.1).min()?, at.iter().map(|a| a.1).max()?);
    let (min_c, max_c) = (at.iter().map(|a| a.2).min()?, at.iter().map(|a| a.2).max()?);
    // Spaced by two so no two lines touch; scaled down to fit the frame.
    let reach = (max_r - min_r).max(max_c - min_c).max(1);
    let scale = (2 * reach + FRAME - 1) / FRAME;
    let scale = scale.max(1);
    let cell = |v: i64, min: i64| usize::try_from(2 * ((v - min) / scale)).unwrap_or(0);
    let rows = cell(max_r, min_r) + 1;
    let cols = cell(max_c, min_c) + 1;
    let mut cells = vec![0u8; rows * cols];
    let mut placed = Vec::new();
    for (i, r, c) in at {
        let (r, c) = (cell(r, min_r), cell(c, min_c));
        cells[r * cols + c] = 1;
        placed.push((i, (r, c)));
    }
    let distinct: std::collections::BTreeSet<(usize, usize)> =
        placed.iter().map(|(_, rc)| *rc).collect();
    if distinct.len() < 3 {
        return None;
    }
    Some(Plane {
        grid: Grid::new(rows, cols, cells)?,
        cells: placed,
    })
}

/// The judge: the central bodies marked in hue 2.
pub fn judged(g: &Grid) -> Grid {
    use solver_verify::project::{apply, Act, ObjectRule, Select};
    apply(
        &ObjectRule {
            select: Select::Central,
            act: Act::Recolor(2),
        },
        g,
    )
}

/// The book: per galaxy, the closer's sigil and its word.
fn read_book(var_root: &Path) -> BTreeMap<String, (String, Program)> {
    let mut book = BTreeMap::new();
    let bytes = std::fs::read(var_root.join("commissions.bin")).unwrap_or_default();
    for (tag, value) in cfe::tlv::records(&bytes) {
        if tag != 1 {
            continue;
        }
        let text = |at: &mut usize| -> Option<String> {
            let len = u16::from_le_bytes(value.get(*at..*at + 2)?.try_into().ok()?) as usize;
            let s = String::from_utf8(value.get(*at + 2..*at + 2 + len)?.to_vec()).ok()?;
            *at += 2 + len;
            Some(s)
        };
        let mut at = 0;
        let (Some(galaxy), Some(sigil)) = (text(&mut at), text(&mut at)) else {
            continue;
        };
        if let Some(program) = value.get(at..).and_then(solver_verify::program_from_bytes) {
            book.insert(galaxy, (sigil, program));
        }
    }
    book
}

fn write_book(var_root: &Path, book: &BTreeMap<String, (String, Program)>) -> std::io::Result<()> {
    let mut out = Vec::new();
    for (galaxy, (sigil, program)) in book {
        let Some(bytes) = solver_verify::program_bytes(program) else {
            continue;
        };
        let mut value = Vec::new();
        for t in [galaxy, sigil] {
            value.extend_from_slice(&(t.len() as u16).to_le_bytes());
            value.extend_from_slice(t.as_bytes());
        }
        value.extend_from_slice(&bytes);
        cfe::tlv::put(1, &value, &mut out);
    }
    let path = var_root.join("commissions.bin");
    let staged = path.with_extension("staged");
    std::fs::write(&staged, out)?;
    std::fs::rename(staged, path)
}

/// The open lines of every galaxy, by core.
fn galaxies(worlds: &[Polity], book: &Ledger) -> BTreeMap<String, Vec<usize>> {
    let mut out: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, p) in worlds.iter().enumerate() {
        if p.penned {
            continue;
        }
        if let Some(core) = book.core_of(&p.spec.position) {
            out.entry(core).or_default().push(i);
        }
    }
    out
}

/// Where the word says the hub stands: the line at a cell the galaxy's
/// commission word marks, while that word still closes the commission on this
/// plane and is held by a line of the galaxy. `None` hands the hub back to the
/// medoid.
pub fn hub_of(
    worlds: &[Polity],
    var_root: &Path,
    core: &str,
    members: &[usize],
    centre: &(Ratio, Ratio, Ratio),
    span: &Ratio,
) -> Option<usize> {
    let (sigil, program) = read_book(var_root).remove(core)?;
    if !members
        .iter()
        .any(|&i| worlds[i].spec.civilization.sigil == sigil)
    {
        return None;
    }
    let plane = plane(worlds, members, centre, span)?;
    let out = solver_verify::grid::run(&program, &plane.grid);
    if out != judged(&plane.grid) {
        return None;
    }
    plane
        .cells
        .iter()
        .filter(|(_, (r, c))| out.at(*r, *c) == 2)
        .map(|(i, _)| *i)
        .min_by(|a, b| worlds[*a].spec.world_name.cmp(&worlds[*b].spec.world_name))
}

/// What a beat of commissions did.
#[derive(Debug, Clone, Default)]
pub struct Commissioned {
    /// (galaxy, the closer's world, the bounty arrived, the predicate that
    /// closed it, how many orders the train admitted).
    pub closed: Option<(String, String, Ratio, String, usize)>,
    /// A closure withheld: (galaxy, how many orders its pairs still admit).
    pub ambiguous: Option<(String, usize)>,
}

/// One attempt A BEAT: the galaxy whose turn it is, if its commission is open,
/// put to the line whose turn it is.
pub fn step(worlds: &mut [Polity], ledger: &Ledger, var_root: &Path, beat: u64) -> Commissioned {
    let mut done = Commissioned::default();
    let span = ledger.universe_span();
    let centres: BTreeMap<String, (Ratio, Ratio, Ratio)> = ledger
        .galaxies()
        .into_iter()
        .map(|g| (g.name, g.center))
        .collect();
    let planes: Vec<(String, Vec<usize>, Plane)> =
        crate::profile::timed("commission:planes", || {
            galaxies(worlds, ledger)
                .into_iter()
                .filter_map(|(core, members)| {
                    let centre = centres.get(&core)?;
                    let plane = plane(worlds, &members, centre, &span)?;
                    Some((core, members, plane))
                })
                .collect()
        });
    if planes.len() < 3 {
        return done; // two train pairs and a test, or no commission
    }
    let turn = (beat as usize) % planes.len();
    let (core, members, plane) = &planes[turn];
    let expected = judged(&plane.grid);
    let mut book = read_book(var_root);
    if let Some((_, program)) = book.get(core) {
        if solver_verify::grid::run(program, &plane.grid) == expected
            && hub_of(worlds, var_root, core, members, &centres[core], &span).is_some()
        {
            return done; // closed, and its word still holds here
        }
    }
    // The train: every other galaxy's plane and its judged answer (as many
    // pairs as the universe shows, so the order is determined, not chosen).
    let train: Vec<TaskPair> = (1..planes.len())
        .map(|k| &planes[(turn + k) % planes.len()].2)
        .map(|p| TaskPair {
            input: p.grid.clone(),
            output: judged(&p.grid),
        })
        .collect();
    let task = Task {
        name: format!("commission:hub:{core}"),
        train,
        test_in: plane.grid.clone(),
        test_expected: Some(expected.clone()),
    };
    let mut sorted = members.clone();
    sorted.sort_by(|a, b| worlds[*a].spec.world_name.cmp(&worlds[*b].spec.world_name));
    let at = sorted[(beat as usize / planes.len()) % sorted.len()];
    let program = crate::profile::timed("commission:think", || {
        worlds[at].try_words(&task).or_else(|| {
            let (mut basin, mut held) = (Vec::new(), Vec::new());
            match solver::think_task_sensing(&task, &mut basin, &mut held, worlds[at].senses()) {
                SolveRead::Solved { program, .. } => Some(program),
                _ => None,
            }
        })
    });
    // The judge: the word must reproduce the exact answer on every pair.
    let Some(program) = program else { return done };
    let holds = task
        .train
        .iter()
        .all(|p| solver_verify::grid::run(&program, &p.input) == p.output)
        && solver_verify::grid::run(&program, &plane.grid) == expected;
    if !holds {
        return done;
    }
    // Determined, not chosen (2026-10-09): an order closes the commission only
    // when exactly one (cut, extreme) survives the pairs; while several do, the
    // learner's first-admitted rule would be the developer's tie-break, not
    // the minds' deduction, and the commission stays open.
    let pairs: Vec<(Grid, Grid)> = task
        .train
        .iter()
        .map(|p| (p.input.clone(), p.output.clone()))
        .collect();
    let admitted = solver::percept::orders_admitted(&pairs).unwrap_or(0);
    let ordered = matches!(program.as_slice(), [solver_verify::grid::Op::Project(r)] if matches!(r.select, solver_verify::project::Select::Ordered { .. }));
    if ordered && admitted != 1 {
        done.ambiguous = Some((core.clone(), admitted));
        return done;
    }
    worlds[at].adopt_word(solver::mint::MintedWord {
        orbit: solver::mint::word_orbit(&program),
        expansion: program.clone(),
        cited: Vec::new(),
    });
    book.insert(
        core.clone(),
        (worlds[at].spec.civilization.sigil.clone(), program.clone()),
    );
    let _ = write_book(var_root, &book);
    // The bounty: the closer's rung, shared by the galaxy's other open lines.
    // A commission is not on the forge's ladder: it pays the ladder's dearest
    // closure (the greatest `mint_bound`, read off the table).
    let bounty = (0..=Difficulty::top())
        .map(|r| kernel::mint_bound(Some(r)))
        .max()
        .unwrap_or_else(|| ratio(0, 1));
    let payers: Vec<usize> = members.iter().copied().filter(|&i| i != at).collect();
    let share = bounty / ratio(payers.len().max(1) as i64, 1);
    let mut arrived = ratio(0, 1);
    for (k, &i) in payers.iter().enumerate() {
        let have = worlds[i]
            .world
            .orb()
            .map(|o| o.stamp_amount())
            .unwrap_or_else(|| ratio(0, 1));
        let pay = cfe::world::smelt_resource(&share.clone().min(have));
        let Some(pay) = cfe::dop::Positive::new(pay) else {
            continue;
        };
        let distance =
            crate::spatial_l1(&worlds[i].spec.position, &worlds[at].spec.position, &span);
        let melt = cfe::world::route_melt(&worlds[i].world, &worlds[at].world, &distance);
        if melt >= ratio(1, 1) {
            continue;
        }
        let deal = beat
            .saturating_mul(100_000)
            .saturating_add(90_000 + k as u64);
        let before = worlds[at]
            .world
            .orb()
            .map(|o| o.stamp_amount())
            .unwrap_or_else(|| ratio(0, 1));
        let (from, to) = crate::two_mut(worlds, i, at);
        if cfe::world::stamp_aperture_transfer(
            &mut from.world,
            cfe::world::FIRST_ORB,
            &mut to.world,
            cfe::world::FIRST_ORB,
            pay,
            deal,
            &melt,
        )
        .is_ok()
        {
            arrived += worlds[at]
                .world
                .orb()
                .map(|o| o.stamp_amount())
                .unwrap_or_else(|| ratio(0, 1))
                - before;
        }
    }
    let by = match program.as_slice() {
        [solver_verify::grid::Op::Project(rule)] => format!("{:?}", rule.select),
        other => format!("a word of {} steps", other.len()),
    };
    done.closed = Some((
        core.clone(),
        worlds[at].spec.world_name.clone(),
        arrived,
        by,
        admitted,
    ));
    done
}
