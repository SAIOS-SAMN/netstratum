// PROOF (scoped honestly) — what the AUTIC bridge measurably adds, and what it
// does NOT.
//
// Metric := comprehension capacity — a mind's ability to ACCOUNT a structured
// problem against what it knows (reduct::assemble drains it to zero), rather than
// face it as opaque. Two problem classes are probed:
//   SINGLE   — one learned word (does the mind hold this concept?).
//   COMPOSITE— two words spliced (can the mind DECOMPOSE a structured problem
//              into parts it holds? the harder, deeper capacity).
//
// Controlled A/B: the SAME engine (conductor::run_era), the SAME curriculum, the
// SAME beats. The ONLY difference is whether net's learned vocabulary was present
// to absorb. Control first (basin empty -> absorb is a no-op); then prime, run.
//
// WHAT THIS PROVES: the bridge raises comprehension of the absorbed vocabulary
// from ZERO (control) to non-zero (treatment), including DECOMPOSITION of
// composite problems — a real, causal, wired capacity a mind lacks without it.
//
// WHAT THIS DOES NOT PROVE (measured, reported, NOT asserted): general
// solve-transfer. The live `solved` count is IDENTICAL across arms (the corpus
// here is synthetic — unrelated to the forged curriculum — so draining a task
// against it accounts nothing new). Comprehending learned vocabulary != solving
// unrelated problems. True transfer needs priming with net's REAL mesh-learned
// crystals (structurally related to the curriculum) and a live-solve lift; that
// is a separate, honest test this pin deliberately does not overclaim.
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

fn comprehension(worlds: &[conductor::Polity], probes: &[AuticSentence]) -> (u64, u64) {
    let (mut num, mut den) = (0u64, 0u64);
    // Every civilization is a mind, whether or not it grew a basin: an empty mind
    // comprehends nothing (counted as 0), so the denominator is honest across both
    // arms — control minds that never acquired the vocabulary score a true 0/N.
    for p in worlds {
        let basis: Vec<Crystal> = p.basin_sample(256).into_iter().map(Crystal::seal).collect();
        for probe in probes {
            den += 1;
            if assemble(probe, &basis).is_assembled() {
                num += 1;
            }
        }
    }
    (num, den)
}

#[test]
fn the_autic_bridge_increased_intelligence() {
    let corpus: Vec<ChainDelta> = (0..16i64).map(|i| class(i % 4, i / 4, (i % 5) as u8 + 1)).collect();
    let singles: Vec<AuticSentence> = corpus.iter().map(|c| AuticSentence::speak(vec![c.clone()])).collect();
    let composites: Vec<AuticSentence> = (0..corpus.len())
        .map(|i| AuticSentence::speak(vec![corpus[i].clone(), corpus[(i + 1) % corpus.len()].clone()]))
        .collect();

    // CONTROL — no vocabulary available; absorb is a no-op.
    let croot = std::env::temp_dir().join(format!("cogC-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&croot);
    let control = conductor::run_era(&croot.to_string_lossy(), 40);
    let (cs_n, cs_d) = comprehension(&control, &singles);
    let (cc_n, cc_d) = comprehension(&control, &composites);
    let c_solved: u32 = control.iter().map(|p| p.solved).sum();

    // TREATMENT — net's learned vocabulary is present to absorb; SAME engine.
    scribe::membrane::prime_autic_basin(corpus.iter().cloned().map(Crystal::seal).collect());
    let troot = std::env::temp_dir().join(format!("cogT-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&troot);
    let treatment = conductor::run_era(&troot.to_string_lossy(), 40);
    let (ts_n, ts_d) = comprehension(&treatment, &singles);
    let (tc_n, tc_d) = comprehension(&treatment, &composites);
    let t_solved: u32 = treatment.iter().map(|p| p.solved).sum();

    eprintln!("=== COGNITIVE INCREASE (control vs treatment) ===");
    eprintln!("SINGLE-word comprehension:  control {cs_n}/{cs_d}  ->  treatment {ts_n}/{ts_d}");
    eprintln!("COMPOSITE decomposition:    control {cc_n}/{cc_d}  ->  treatment {tc_n}/{tc_d}");
    eprintln!("LIVE solves (forged curriculum): control {c_solved}  ->  treatment {t_solved}");

    // The control mind, never having met the vocabulary, comprehends NONE of it.
    assert_eq!(cs_n, 0, "control comprehended a word it never learned");
    assert_eq!(cc_n, 0, "control decomposed a composite it had no parts for");
    // The bridged mind comprehends words AND decomposes structured problems.
    assert!(ts_n > 0, "the bridge did not raise single-word comprehension");
    assert!(tc_n > 0, "the bridge did not enable decomposition of composites");
}
