//! COST PROBE (remediations lane) — attribute the per-beat cost of the
//! chained beat against a REAL chronicle, so the remediation checklist
//! rests on measurement and not on reading.
//!
//! This is a measuring instrument, not a mechanic: it appends nothing,
//! writes nothing, and judges nothing. It replays what the conductor
//! already replays and reports where the time goes.
//!
//! ```text
//! cargo run --release -p conductor --example cost_probe -- <chronicle.tlv> [deeds_per_beat]
//! ```
//!
//! The shape it exists to expose is `Conductor::chain_beat`
//! (`conductor/src/lib.rs:1365`), whose own comment concedes it is
//! "O(deeds × history) per beat". The probe prices one such beat at the
//! CURRENT history length, so the cost can be re-measured after the
//! stepping replay lands and the curve compared.

use std::time::Instant;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = match args.next() {
        Some(held) => std::path::PathBuf::from(held),
        None => {
            println!("usage: cost_probe <chronicle.tlv> [deeds_per_beat]");
            return;
        }
    };
    let deeds: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(3);

    // (1) THE LOAD — what `persist::sync` pays on EVERY beat just to
    // find the tail it means to append (conductor/src/lib.rs:1289).
    let at = Instant::now();
    let chronicle = match cfe::persist::load(&path) {
        Ok(held) => held,
        Err(refused) => {
            println!("the chronicle refused: {refused:?}");
            return;
        }
    };
    let load = at.elapsed();
    let receipts = chronicle.receipts().to_vec();
    let n = receipts.len();

    // (2) THE CHAIN VERIFY alone — `from_receipts` re-hashes every link.
    let at = Instant::now();
    let verified = cfe::vrp::Chronicle::from_receipts(receipts.clone());
    let verify = at.elapsed();
    if verified.is_err() {
        println!("the chain did not re-verify");
        return;
    }

    // (3) ONE FULL REPLAY — the monolithic loop that cannot advance by a
    // single receipt.
    let at = Instant::now();
    let committed = cfe::world::replay(&chronicle);
    let replay = at.elapsed();

    // (4) ONE STATE ENCODING — paid once per beat at the commitment.
    let at = Instant::now();
    let bytes = codec::state_bytes(&committed);
    let encode = at.elapsed();

    // (5) ONE WHOLE BEAT, as `chain_beat` actually spends it: for each
    // new deed, rebuild the prefix chronicle (copy + verify) and replay
    // it whole, then judge the step.
    let first = n.saturating_sub(deeds);
    let at = Instant::now();
    let mut prior = cfe::world::replay(
        &cfe::vrp::Chronicle::from_receipts(receipts[..first].to_vec()).expect("intact"),
    );
    let mut judged = 0usize;
    for cut in first..n {
        let next = cfe::world::replay(
            &cfe::vrp::Chronicle::from_receipts(receipts[..=cut].to_vec()).expect("intact"),
        );
        let receipt = &receipts[cut];
        let step = kernel::Step {
            event: receipt.event.clone(),
            actor: receipt.provenance.first().copied().unwrap_or(cfe::world::SOVEREIGN),
            stamp: receipt.stamp,
        };
        // The verdict is not the probe's business — only its cost is.
        let _ = kernel::admits(&prior, &next, &step);
        judged += 1;
        prior = next;
    }
    let beat = at.elapsed();

    println!("chronicle           {}", path.display());
    println!("receipts            {n}");
    println!("state bytes         {}", bytes.len());
    println!("---");
    println!("persist::load       {load:?}   (paid per beat by persist::sync)");
    println!("chain verify only   {verify:?}");
    println!("one full replay     {replay:?}");
    println!("codec::state_bytes  {encode:?}");
    println!("---");
    println!("ONE CHAINED BEAT    {beat:?}   ({judged} deeds judged, {} replays)", judged + 1);
    println!(
        "per-deed cost       {:?}",
        beat.checked_div(judged.max(1) as u32).unwrap_or_default()
    );
}
