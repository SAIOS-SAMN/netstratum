// PROOF that the civilizations speak a SHARED language, not private dialects.
// Three claims, each measured:
//   1. COMMON VOCABULARY  — distinct civs hold the SAME words (same crystals).
//   2. MUTUAL INTELLIGIBILITY — one civ COMPREHENDS another's utterance: a word
//      A speaks assembles (drains to zero) against B's own basin.
//   3. CONTROL — random words (outside the shared language) do NOT assemble.
// If comprehension >> control, the shared language is real, not coincidence.
use reduct::congruence::ChainEntry;
use reduct::peel::{CellFrame, GaugeDirection, Site};
use reduct::{assemble, AuticSentence, ChainDelta, Crystal};

fn class(row: i64, col: i64, dir: u8) -> ChainDelta {
    ChainDelta {
        from: CellFrame { rows: 8, cols: 8 },
        to: CellFrame { rows: 8, cols: 8 },
        entries: vec![ChainEntry {
            site: Site { row, col },
            direction: GaugeDirection { index: dir },
            coefficient: num_bigint::BigInt::from(1),
        }],
    }
}

#[test]
fn the_civilizations_speak_a_shared_language() {
    // A shared corpus of 16 words — net's AUTIC language.
    let corpus: Vec<ChainDelta> = (0..16i64).map(|i| class(i % 4, i / 4, (i % 5) as u8 + 1)).collect();
    scribe::membrane::prime_autic_basin(corpus.iter().cloned().map(Crystal::seal).collect());

    let root = std::env::temp_dir().join(format!("shlang-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let worlds = conductor::run_era(&root.to_string_lossy(), 40);

    // Each civ's vocabulary (distinct crystal classes it holds).
    let vocabs: Vec<(String, Vec<ChainDelta>)> = worlds
        .iter()
        .map(|p| (p.spec.civilization.sigil.clone(), p.basin_classes()))
        .filter(|(_, v)| !v.is_empty())
        .collect();
    assert!(vocabs.len() >= 2, "need >=2 speaking civilizations to prove sharing (got {})", vocabs.len());

    // CLAIM 1 — COMMON VOCABULARY: how many civs hold each corpus word?
    let mut shared_words = 0usize;
    let mut max_civs_on_a_word = 0usize;
    for w in &corpus {
        let holders = vocabs.iter().filter(|(_, v)| v.contains(w)).count();
        if holders >= 2 {
            shared_words += 1;
        }
        max_civs_on_a_word = max_civs_on_a_word.max(holders);
    }

    // CLAIM 2 — MUTUAL INTELLIGIBILITY: for every ordered civ pair, how many of
    // A's words does B comprehend (a single-word sentence assembles against B)?
    let mut comp_num = 0u64;
    let mut comp_den = 0u64;
    for (_, va) in &vocabs {
        for (_, vb) in &vocabs {
            if std::ptr::eq(va, vb) {
                continue;
            }
            let basis_b: Vec<Crystal> = vb.iter().cloned().map(Crystal::seal).collect();
            for w in va {
                comp_den += 1;
                if assemble(&AuticSentence::speak(vec![w.clone()]), &basis_b).is_assembled() {
                    comp_num += 1;
                }
            }
        }
    }
    let comprehension = comp_num as f64 / comp_den.max(1) as f64;

    // CLAIM 3 — CONTROL: random words outside the language, heard by every civ.
    let control: Vec<ChainDelta> = (0..16i64).map(|i| class(50 + i, 50 + i, (i % 5) as u8 + 1)).collect();
    let mut ctrl_num = 0u64;
    let mut ctrl_den = 0u64;
    for (_, vb) in &vocabs {
        let basis_b: Vec<Crystal> = vb.iter().cloned().map(Crystal::seal).collect();
        for w in &control {
            ctrl_den += 1;
            if assemble(&AuticSentence::speak(vec![w.clone()]), &basis_b).is_assembled() {
                ctrl_num += 1;
            }
        }
    }
    let control_rate = ctrl_num as f64 / ctrl_den.max(1) as f64;

    eprintln!("=== SHARED LANGUAGE PROOF ({} speaking civs) ===", vocabs.len());
    eprintln!("CLAIM 1 common vocabulary : {shared_words}/{} corpus words held by >=2 civs; one word held by up to {max_civs_on_a_word} civs", corpus.len());
    eprintln!("CLAIM 2 mutual intelligibility: {:.1}% of cross-civ words comprehended ({comp_num}/{comp_den})", comprehension * 100.0);
    eprintln!("CLAIM 3 control (random words): {:.1}% comprehended ({ctrl_num}/{ctrl_den}) — must be ~0", control_rate * 100.0);

    assert!(shared_words >= 1, "no word is held by two civilizations — not a shared vocabulary");
    assert!(comprehension > 0.0, "no cross-civ comprehension — not mutually intelligible");
    assert!(control_rate == 0.0, "control words assembled — the test proves nothing");
    assert!(comprehension > control_rate, "shared-language comprehension must exceed the control");
}
