//! **THE PROPOSER** — netstratum's mind speaking the court's own rule
//! language (mycelium 78.143 `sdk::rule`): one language for minds and court,
//! so nothing is translated. From the task's own perception (the hues it
//! shows, whether the frame holds), the mind proposes rules — a selection,
//! then an act (recolour, erase, fill, halo, extend, copy, crop, scale, a
//! symmetry) — mints each through the one mint (`rule::sentence`), and
//! checks it with THE JUDGE'S OWN STEPS on the training pairs
//! (`rule::would_judge`). The first that closes every pair is the answer,
//! spoken on the test inputs. Nothing is claimed that the judge's steps did
//! not close; the court judges again when it is filed.

use mycelium::sdk::rule::{
    sentence, would_judge, Cmp, Coord, Create, Extent, Order, Paint, Pred, Region, Step, Stop, Symmetry, Tie, Value,
};
use mycelium::transducer::element::Element;
use mycelium::transducer::family::{self, NoCrystal};

type G = Vec<Vec<u8>>;

/// A closed proposal: its steps, its minted rule, its test outputs.
pub struct Proposal {
    pub steps: Vec<Step>,
    pub rule: Element,
    pub outputs: Vec<G>,
}

fn hues(grids: &[&G]) -> Vec<u8> {
    let mut seen = [false; 10];
    for g in grids {
        for row in g.iter() {
            for &h in row {
                if (h as usize) < 10 {
                    seen[h as usize] = true;
                }
            }
        }
    }
    (0u8..10).filter(|&h| seen[h as usize]).collect()
}

fn shape(g: &G) -> (usize, usize) {
    (g.len(), g.first().map_or(0, Vec::len))
}

/// The selections a mind tries, in a stated order: everything; each hue it
/// sees; each hue's complement; the largest and smallest bodies (ties held);
/// then COUNT and COMPARE (78.143 slice 2): the extremum of each body
/// coordinate in both orders, the bodies whose shape is unique in the scene
/// or that carry several hues, and the bodies touching each hue.
fn selections(input_hues: &[u8]) -> Vec<Pred> {
    let mut out = vec![Pred::All];
    for &h in input_hues.iter().filter(|&&h| h != 0) {
        out.push(Pred::Hue(h));
    }
    for &h in input_hues.iter().filter(|&&h| h != 0) {
        out.push(Pred::Not(Box::new(Pred::Hue(h))));
    }
    out.push(Pred::extremum(Coord::Tally, Order::Greatest, Tie::Hold));
    out.push(Pred::extremum(Coord::Tally, Order::Least, Tie::Hold));
    for coord in [Coord::Shell, Coord::Height, Coord::Width, Coord::Enclosed, Coord::Contacts, Coord::Hues, Coord::Peers] {
        for order in [Order::Greatest, Order::Least] {
            out.push(Pred::extremum(coord, order, Tie::Hold));
        }
    }
    out.push(Pred::count(Coord::Peers, 1));
    out.push(Pred::Cmp { coord: Coord::Hues, cmp: Cmp::Greater, to: Value::Integer(1) });
    for &h in input_hues.iter().filter(|&&h| h != 0) {
        out.push(Pred::Touches(Box::new(Pred::Hue(h))));
    }
    out
}

/// Every candidate rule for a family, in order: same-frame acts first when
/// the frame holds, sized acts when it does not.
pub fn candidates(train: &[(G, G)]) -> Vec<Vec<Step>> {
    let ins: Vec<&G> = train.iter().map(|(a, _)| a).collect();
    let outs: Vec<&G> = train.iter().map(|(_, b)| b).collect();
    let in_hues = hues(&ins);
    let out_hues = hues(&outs);
    let new_hues: Vec<u8> = out_hues.iter().copied().filter(|h| !in_hues.contains(h)).collect();
    let paints: Vec<u8> = if new_hues.is_empty() { out_hues.clone() } else { new_hues.clone() };
    let same_frame = train.iter().all(|(a, b)| shape(a) == shape(b));
    let mut out = Vec::new();
    let sels = selections(&in_hues);
    if same_frame {
        for p in &sels {
            for &c in &paints {
                let recolor: Vec<(u8, u8)> = (1u8..10).filter(|&k| k != c).map(|k| (k, c)).collect();
                out.push(vec![Step::Select(p.clone()), Step::Hue(recolor)]);
                out.push(vec![Step::Select(p.clone()), Step::Region(Region::Interior), Step::Hue(vec![(0, c)])]);
                out.push(vec![Step::Select(p.clone()), Step::Region(Region::Halo), Step::Hue(vec![(0, c)])]);
            }
            out.push(vec![Step::Select(p.clone()), Step::Hue((1u8..10).map(|k| (k, 0)).collect())]);
            for ray in [(-1i8, 0i8), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)] {
                for stop in [Stop::Border, Stop::Contact] {
                    out.push(vec![Step::Select(p.clone()), Step::Create(Create::Extend { ray, paint: Paint::Own, stop: stop.clone() })]);
                    for &c in &paints {
                        out.push(vec![Step::Select(p.clone()), Step::Create(Create::Extend { ray, paint: Paint::Hue(c), stop: stop.clone() })]);
                    }
                }
            }
            for q in &sels {
                if q != p {
                    out.push(vec![Step::Select(p.clone()), Step::Create(Create::Copy { template: q.clone() })]);
                }
            }
        }
    } else {
        for p in &sels {
            out.push(vec![Step::Select(p.clone()), Step::Crop(Extent::Body)]);
            out.push(vec![Step::Select(p.clone()), Step::Crop(Extent::Interior)]);
        }
        for k in 2u8..=4 {
            out.push(vec![Step::Scale { up: k, down: 1 }]);
            out.push(vec![Step::Scale { up: 1, down: k }]);
        }
    }
    // The six symmetries, where the shapes fit them.
    if let Some((a, b)) = train.first() {
        for kind in [Symmetry::Rotate90, Symmetry::Rotate180, Symmetry::Rotate270, Symmetry::FlipH, Symmetry::FlipV, Symmetry::Transpose] {
            if let Some(step) = Step::grid_motion(kind, shape(a), shape(b)) {
                out.push(vec![step]);
            }
        }
    }
    out
}

/// Speak a minted rule on one input.
fn speak(rule: &Element, input: &G) -> Option<G> {
    let held = mycelium::transducer::element::Held::of(rule.clone());
    let form = held.form()?.clone();
    let spoken = family::speak_rule(&mycelium::transducer::sentence::rule_of_form(form), input, &NoCrystal)?;
    spoken.grid()
}

/// THE MIND'S PROPOSAL for a family: the first candidate the judge's own
/// steps close on every training pair, spoken on `tests`; at most `tries`
/// candidates are minted.
pub fn propose(train: &[(G, G)], tests: &[G], tries: usize) -> Option<Proposal> {
    propose_within(train, tests, tries, std::time::Duration::MAX)
}

/// [`propose`] stopping once `budget` of wall time is spent — a mind's look
/// is bounded; a candidate the clock cuts off is not tried.
pub fn propose_within(train: &[(G, G)], tests: &[G], tries: usize, budget: std::time::Duration) -> Option<Proposal> {
    let started = std::time::Instant::now();
    for steps in candidates(train).into_iter().take(tries) {
        if started.elapsed() >= budget {
            return None;
        }
        let Ok(rule) = sentence(&steps, train, &NoCrystal) else { continue };
        if would_judge(&rule, train, &|_| None, &NoCrystal).is_err() {
            continue;
        }
        let outputs: Option<Vec<G>> = tests.iter().map(|t| speak(&rule, t)).collect();
        if let Some(outputs) = outputs {
            return Some(Proposal { steps, rule, outputs });
        }
    }
    None
}
