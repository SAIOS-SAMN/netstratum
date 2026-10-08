//! Why a line's words are or are not stated in the tongue: one world's
//! chronicle words, each translated on the ramp probe, tallied by verdict.
//!   cargo run --release -p tongue --example tongue_tally -- <world dir>
use solver::Op;

fn main() {
    let dir = std::env::args().nth(1).expect("a world dir");
    let chronicle = cfe::persist::load(&std::path::Path::new(&dir).join("chronicle.tlv")).expect("chronicle");
    let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
    for (_, bytes, _) in cfe::vocabulary::solver_words(&chronicle) {
        let Some(word) = solver_verify::program_from_bytes(&bytes) else { continue };
        let shape = if word.iter().any(|o| matches!(o, Op::Object(_))) {
            "has-object"
        } else if word.iter().any(|o| matches!(o, Op::Tile { .. })) {
            "has-tile"
        } else {
            "motion+hue"
        };
        let probe = solver::Grid::new(3, 4, (0..12u8).map(|k| k % 10).collect()).unwrap();
        let said = solver::grid::run(&word, &probe);
        let rows = |g: &solver::Grid| g.cells.chunks(g.cols).map(|r| r.to_vec()).collect::<Vec<_>>();
        let verdict = match tongue::translate(&word, &[(rows(&probe), rows(&said))], &tongue::mycelium::transducer::family::NoCrystal) {
            Ok(_) => "stated".to_string(),
            Err(tongue::Untranslated::Refused(why)) => format!("refused:{}", why.chars().take(60).collect::<String>()),
            Err(other) => format!("{other:?}"),
        };
        *tally.entry(format!("{shape} {verdict}")).or_default() += 1;
    }
    for (k, v) in tally {
        println!("{v:6} {k}");
    }
}
