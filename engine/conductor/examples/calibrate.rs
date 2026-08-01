//! #17 CALIBRATION — the empirical evidence Ruling Box 1 waits on.
//!
//! U_DIRECTIVES §7: after #36 + U1 the economy runs CHAINED with
//! certificate-bounded minting REFUSING until ruled. We measure THERE,
//! then the operator rules `f` against observation, never assumption.
//!
//! Three readings, all deterministic (seeded — same run, same numbers
//! on every machine):
//!
//!   A. SOLVES-PER-RUNG — the honest solver's hit rate at each
//!      difficulty rung (a solve counts ONLY when the rendered test
//!      output matches the forge's expected grid — the conductor's own
//!      criterion), the certificate work-payload size, and the
//!      closed-but-mismatched tail (training closed, test render
//!      diverged — real work that verify_work rightly refuses).
//!   B. MINT PRESSURE vs THE RULED BOUND — real certificates encoded
//!      as work payloads, run as DiscoveryMinted through the kernel.
//!      Every outcome TALLIED by kind, never assumed. Under Ruling
//!      Box 1 (2026-08-01: f = 0·0·0·1·4·7 on the ladder) a 1-mint
//!      refuses on the saturated rungs 0–2 and admits on 3–5; a zero
//!      mint always admits. Any other outcome is COUNTED and reported
//!      — this harness measures, it does not panic.
//!   C. A BOUNDED REAL ERA — the conductor plays; solves recorded and
//!      spine growth counted, the board dynamics in miniature.
//!
//! Run: `cargo run -p conductor --example calibrate --release`

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ipsal::Entity;
use cfe::ratio::ratio;
use cfe::world::{Committed, World, SOVEREIGN};
use solver::{forge_task, solve, Difficulty, SolveRead};
use solver_verify::work::{encode_work, WorkProof};

/// Deterministic per-index digest — a spread of forge seeds with no
/// wall clock (CHARTER §9: randomness is a reading of history). A plain
/// LCG: the forge needs varied seeds, not cryptographic ones.
fn digest_for(rung: u32, i: u32) -> [u8; 32] {
    let mut state = 0x243F_6A88u32
        ^ rung.wrapping_mul(2_654_435_761)
        ^ i.wrapping_mul(40_503)
        ^ 1;
    let mut d = [0u8; 32];
    for byte in d.iter_mut() {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        *byte = (state >> 24) as u8;
    }
    d
}

/// The prior state a mint judges against: one orb holding iron, the
/// sovereign seated (holds every scope by construction).
fn mint_prior() -> Committed {
    let mut committed = World::new().committed();
    let mut orb = ColonyOrb { orb_id: 1, name: "forge".into(), ..Default::default() };
    orb.bank(Resource::Iron, ratio(5, 1));
    committed.orbs = vec![orb];
    committed.entities = vec![Entity {
        entity_id: SOVEREIGN,
        handle: "sovereign".into(),
        principal: None,
        grants: vec![],
    }];
    committed
}

/// The per-rung tally — every judged outcome counted by kind.
#[derive(Default)]
struct RungTally {
    solves_honest: u32,
    closed_mismatched: u32,
    unsolved: u32,
    encode_failed: u32,
    cert_bytes_total: usize,
    // Positive-mint outcomes at bound 0:
    refused_exceeds: u32,
    refused_work: u32,
    refused_other: u32,
    admitted_positive: u32, // would be a seal breach — counted, reported
    // Zero-mint outcomes:
    zero_admitted: u32,
    zero_refused: u32,
}

fn main() {
    println!("# ============================================================");
    println!("# #17 CALIBRATION — PoUW economics, empirical (Ruling Box 1)");
    println!("# deterministic, seeded; judged under the kernel's ruled f");
    println!("# (Box 1, 2026-08-01: 0·0·0·1·4·7 on the ladder)");
    println!("# ============================================================");
    println!();

    part_a_and_b();
    part_c_bounded_era();

    println!();
    println!("# ============================================================");
    println!("# READING: the SUPPLY per rung is the honest-solves column;");
    println!("# the ruled f maps supply to issuance — 1-mints refuse on the");
    println!("# saturated rungs 0-2 and admit on 3-5. Re-run after any");
    println!("# ladder or rate amendment; the densities are the evidence.");
    println!("# ============================================================");
}

/// Parts A + B fused: forge N tasks per rung, solve each, and for every
/// honest solve push a real DiscoveryMinted through the kernel, tallying
/// every outcome against the sealed bound.
fn part_a_and_b() {
    const SAMPLES: u32 = 40;
    println!("## PART A/B — solves-per-rung + mint pressure vs the sealed bound");
    println!("## ({SAMPLES} forged tasks per rung; every honest solve is a would-be mint)");
    println!();
    println!(
        "{:<5} {:<5} {:<5} {:<7} {:<9} {:<7} {:<6} {:<11} {:<9}",
        "rung", "tier", "word", "honest", "mismatch", "unsolv", "cert", "mint@1", "zero-mint"
    );
    println!("{}", "-".repeat(78));

    let prior = mint_prior();
    let orb_id = prior.orbs.first().map(|o| o.orb_id).unwrap_or(0);

    let mut grand = RungTally::default();

    for rung in 0..=5u32 {
        let difficulty = Difficulty::level(rung);
        let mut tally = RungTally::default();

        for i in 0..SAMPLES {
            let digest = digest_for(rung, i);
            let task = forge_task(digest, difficulty);
            let (program, test_output, certificate) =
                match solve(&task, difficulty.word_len) {
                    SolveRead::Solved { program, test_output, matches_expected, certificate } => {
                        if matches_expected == Some(true) {
                            (program, test_output, certificate)
                        } else {
                            // Training closed, test render diverged —
                            // real search work verify_work rightly
                            // refuses (OutputMismatch). Counted, not
                            // minted.
                            tally.closed_mismatched += 1;
                            continue;
                        }
                    }
                    SolveRead::Unsolved => {
                        tally.unsolved += 1;
                        continue;
                    }
                };
            tally.solves_honest += 1;

            // Encode the REAL proof payload (solver-verify's monopoly).
            let proof = WorkProof {
                forge_digest: digest,
                difficulty,
                program,
                output: test_output,
                certificate,
            };
            let work = match encode_work(&proof) {
                Some(bytes) => bytes,
                None => {
                    tally.encode_failed += 1;
                    continue;
                }
            };
            tally.cert_bytes_total += work.len();

            // MINT PRESSURE: a POSITIVE 1-mint against this verified
            // work, judged under the ruled f; every outcome is
            // tallied, none assumed.
            let mut minted = prior.clone();
            if let Some(orb) = minted.orbs.first_mut() {
                orb.bank(Resource::Iron, ratio(1, 1));
            }
            let positive = kernel::Step {
                event: ClosureEvent::DiscoveryMinted {
                    orb_id,
                    resource: Resource::Iron,
                    amount: ratio(1, 1),
                    work: work.clone(),
                },
                actor: SOVEREIGN,
                stamp: 1,
            };
            match kernel::lawful(&prior, &minted, &positive) {
                Err(kernel::Unlawful::MintExceedsWork { .. }) => tally.refused_exceeds += 1,
                Err(kernel::Unlawful::WorkRefused { .. }) => tally.refused_work += 1,
                Err(_) => tally.refused_other += 1,
                Ok(()) => tally.admitted_positive += 1,
            }

            // THE DEGENERATE MINT: amount 0 with valid work admits (the
            // U1 witness-5 law).
            let zero = kernel::Step {
                event: ClosureEvent::DiscoveryMinted {
                    orb_id,
                    resource: Resource::Iron,
                    amount: ratio(0, 1),
                    work,
                },
                actor: SOVEREIGN,
                stamp: 1,
            };
            match kernel::lawful(&prior, &prior, &zero) {
                Ok(()) => tally.zero_admitted += 1,
                Err(_) => tally.zero_refused += 1,
            }
        }

        let avg_cert = match tally.solves_honest {
            0 => 0,
            n => tally.cert_bytes_total / n as usize,
        };
        println!(
            "{:<5} {:<5} {:<5} {:<7} {:<9} {:<7} {:<6} {:<11} {}",
            rung,
            difficulty.tier,
            difficulty.word_len,
            tally.solves_honest,
            tally.closed_mismatched,
            tally.unsolved,
            avg_cert,
            format!(
                "{}x/{}w/{}o/{}A",
                tally.refused_exceeds,
                tally.refused_work,
                tally.refused_other,
                tally.admitted_positive
            ),
            format!("{}ok/{}ref", tally.zero_admitted, tally.zero_refused),
        );

        grand.solves_honest += tally.solves_honest;
        grand.closed_mismatched += tally.closed_mismatched;
        grand.unsolved += tally.unsolved;
        grand.encode_failed += tally.encode_failed;
        grand.refused_exceeds += tally.refused_exceeds;
        grand.refused_work += tally.refused_work;
        grand.refused_other += tally.refused_other;
        grand.admitted_positive += tally.admitted_positive;
        grand.zero_admitted += tally.zero_admitted;
        grand.zero_refused += tally.zero_refused;
    }

    println!("{}", "-".repeat(78));
    println!("## legend mint@1: x=MintExceedsWork w=WorkRefused o=other-unlawful A=admitted");
    println!("## (ruled f: a 1-mint refuses on rungs 0-2 — bound 0 — and admits on 3-5)");
    let attempts = 6 * SAMPLES;
    println!(
        "TOTAL: {} honest solves / {} closed-mismatched / {} unsolved of {} attempts; \
         encode failures {}.",
        grand.solves_honest, grand.closed_mismatched, grand.unsolved, attempts, grand.encode_failed
    );
    println!(
        "BOUND: positive 1-mints — {} refused MintExceedsWork, {} WorkRefused, {} other, \
         {} admitted. Zero mints — {} admitted, {} refused.",
        grand.refused_exceeds,
        grand.refused_work,
        grand.refused_other,
        grand.admitted_positive,
        grand.zero_admitted,
        grand.zero_refused,
    );
    println!();
}

/// Part C — a bounded real era: the conductor plays a short span; count
/// the solves it records and the spine records it chains.
fn part_c_bounded_era() {
    println!("## PART C — a bounded real era (the board in miniature)");
    println!();

    let var = std::env::temp_dir().join(format!("calibrate-era-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&var);
    if std::fs::create_dir_all(&var).is_err() {
        println!("  (era skipped: scratch dir unavailable)");
        return;
    }

    const CYCLES: u32 = 12;
    let worlds = conductor::run_era(var.to_string_lossy().as_ref(), CYCLES);

    let total_solved: u32 = worlds.iter().map(|e| e.solved).sum();
    let max_level = worlds.iter().map(|e| e.level).max().unwrap_or(0);

    let mut spine_records = 0usize;
    let mut spined_worlds = 0usize;
    if let Ok(entries) = std::fs::read_dir(var.join("worlds")) {
        for entry in entries.flatten() {
            let spine_root = entry.path().join("spine");
            if let Ok(spine) = mesh::spine::Spine::open(&spine_root) {
                if !spine.records().is_empty() {
                    spined_worlds += 1;
                    spine_records += spine.records().len();
                }
            }
        }
    }

    println!("  worlds:          {}", worlds.len());
    println!("  cycles (beats):  {CYCLES}");
    println!("  solves recorded: {total_solved}");
    println!("  rung reached:    up to level {max_level}");
    println!(
        "  spine records:   {spine_records} across {spined_worlds} chained worlds \
         (judged=0 — observed play)"
    );
    let _ = std::fs::remove_dir_all(&var);
}
