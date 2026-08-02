//! THE CONSULTATION EXPERIMENT — how often does the door refuse?
//! (`19_LENS` §7a, task 4.)
//!
//! **This is a HARNESS, not a ledger, and the distinction is the whole
//! design.** A ledger accumulates, and accumulation is a second record
//! (`00_CHARTER` §3a). Worse: a withdrawal at the sector door is never
//! receipted — it does not reach the chain — so there is nothing for a
//! lens to derive a frequency FROM. Counting it privately would be
//! inventing the record.
//!
//! So the frequency is MEASURED BY EXPERIMENT. This runs `N` trials and
//! reports those trials. It reflects no committed state, holds no second
//! image, and is deletable without loss — which makes it neither lens
//! nor mirror. It answers *"what does this search do?"* rather than
//! *"what is the world holding?"*
//!
//! **WHY THE WITHDRAWAL RATE IS THE READING THAT MATTERS.** A degrading
//! search and a genuinely unsolvable task look identical once
//! `solve_consulted` collapses them — both just rest. Only
//! `consult_observed` separates them:
//!
//! ```text
//!     CROSSED    proposed a program; the door re-derived it and it held
//!     SILENT     proposed nothing — lawful, claims nothing
//!     WITHDRAWN  proposed a program the door REFUSED
//! ```
//!
//! A rising withdrawal count means approximation is proposing wrong
//! programs and the door is catching them. That is the door working —
//! and it is worth knowing *before* it stops.
//!
//! **A READING, NEVER A TARGET.** Nothing may optimise this ratio. A
//! search tuned to lower its withdrawal rate would learn to propose
//! LESS rather than propose BETTER, and the measure would stop
//! measuring — the standing trap of `19_LENS` §1.
//!
//! Run: `cargo run -p conductor --example consultation`

use solver::{consult_observed, forge_task, Consultation, Difficulty};

/// One rung's tally over `trials` forged tasks. Held on the stack for
/// the length of the run and reported — never stored, never returned to
/// the system.
struct Tally {
    crossed: u32,
    silent: u32,
    withdrawn: u32,
}

impl Tally {
    fn measure(level: u32, trials: u32) -> Self {
        let difficulty = Difficulty::level(level);
        let mut tally = Tally { crossed: 0, silent: 0, withdrawn: 0 };
        for seed in 0..trials {
            // Deterministic digests: the experiment repeats exactly for
            // every operator, which is what makes it evidence (LAW 6).
            let digest = [(seed % 251) as u8; 32];
            let task = forge_task(digest, difficulty);
            match consult_observed(&task, difficulty.word_len) {
                Consultation::Crossed(_) => tally.crossed += 1,
                Consultation::Silent => tally.silent += 1,
                Consultation::Withdrawn => tally.withdrawn += 1,
            }
        }
        tally
    }

    /// Proposals offered — the two arms where the search actually
    /// proposed a program.
    fn proposed(&self) -> u32 {
        self.crossed + self.withdrawn
    }

    /// The withdrawal rate as an EXACT ratio, rendered. No float, no
    /// percentage, no rounding — `withdrawn / proposed`, or `-` when
    /// nothing was proposed and the question does not arise.
    fn withdrawal_rate(&self) -> String {
        match self.proposed() {
            0 => "-".to_string(),
            n => format!("{}/{}", self.withdrawn, n),
        }
    }
}

fn main() {
    let trials: u32 = std::env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(40);

    println!("THE CONSULTATION EXPERIMENT — {trials} trials per rung");
    println!("a harness, not a ledger: nothing here is recorded or returned\n");
    println!("  rung   crossed   silent   withdrawn   withdrawal rate");
    println!("  ────   ───────   ──────   ─────────   ───────────────");

    for level in 0..=5u32 {
        let t = Tally::measure(level, trials);
        println!(
            "  {:>4}   {:>7}   {:>6}   {:>9}   {:>15}",
            level,
            t.crossed,
            t.silent,
            t.withdrawn,
            t.withdrawal_rate()
        );
    }

    println!("\nWITHDRAWN is the reading that matters: a proposal the door");
    println!("REFUSED. A degrading search and an unsolvable task both rest,");
    println!("and only this column tells them apart.");
    println!("\nIt is a reading, never a target. A search tuned to lower this");
    println!("would learn to propose LESS rather than propose BETTER.");
}
