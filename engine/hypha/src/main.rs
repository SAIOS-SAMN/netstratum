//! `ns-hypha` — netstratum on a mycelium hypha.
//!
//!   ns-hypha laws                       the court's law register
//!   ns-hypha lexicon                    netstratum's terms against the server's
//!   ns-hypha probe                      a lawful and an unlawful transition, read by both
//!   ns-hypha census <world-dir> [--last N] [--send]
//!                                       fold a people's chronicle; --send puts it to the court
//!   ns-hypha found                      found netstratum's domain on the court (once)
//!   ns-hypha mark <world-dir>           record a people as folded through its chronicle's end
//!   ns-hypha epoch <name>               set the hot colonies line aside, restart it at zero
//!   ns-hypha prune                      archive hot entries whose folds the court reads settled
//!   ns-hypha publish                    put every provisional hot fold's prefix to replay
//!
//! `census --send` then publishes the folds it had admitted (`--no-publish`
//! to skip): an upheld fold cannot be demanded, so no deadline can default
//! it. One stream at a time, in fold order; `Refused(Published)` is answered.
//! (Coordinator ruling 2026-10-07: self-publication is the default.)
//!
//! A census resumes from the people's cursor (`var/uplink/cursor-colonies`),
//! so no receipt is ever netted twice; `--last N` bounds only a people's
//! first census.
//!
//! Flags: `--at host:port` (or `NS_HYPHA`, default 127.0.0.1:7520),
//! `--home dir` (default var/uplink). Only `found` and `census --send` write
//! anything on the court; every other command is a reading.

use ns_hypha::census::{self, grains};
use ns_hypha::fold::{fold, tier_key};
use ns_hypha::line::{Cursors, FoldLog, FoldMark, TierLine};
use ns_hypha::mycelium::anastome::dialogue::uplink::Uplinked;
use ns_hypha::{laws_of, probe, Uplink, DEFAULT_AT};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn flag(args: &[String], name: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].clone())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let at = flag(&args, "--at").or_else(|| std::env::var("NS_HYPHA").ok()).unwrap_or_else(|| DEFAULT_AT.to_string());
    let home = PathBuf::from(flag(&args, "--home").unwrap_or_else(|| "var/uplink".to_string()));
    let cmd = args.get(1).map(String::as_str);
    if cmd == Some("census") && !args.iter().any(|a| a == "--send") {
        return census_cmd(&args, None, &home);
    }
    if cmd == Some("mark") {
        return mark_cmd(&args, &home);
    }
    if cmd == Some("epoch") {
        return epoch_cmd(&args, &home);
    }
    let up = match Uplink::open(&at, &home) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("ns-hypha: cannot open a session to {at}: {e}");
            return ExitCode::from(2);
        }
    };
    let vk: String = up.voice.iter().take(4).map(|b| format!("{b:02x}")).collect();
    match cmd {
        Some("laws") => match up.laws() {
            Ok(served) => {
                println!("hypha {at} (netstratum voice {vk}…) holds {} laws:", served.len());
                for s in served {
                    println!("  [{}] {} — {}", s.law.index(), s.law.name(), s.statement);
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("ns-hypha: the register was not read: {e}");
                ExitCode::from(1)
            }
        },
        Some("lexicon") => match up.lexicon_check() {
            Ok((version, missing)) if missing.is_empty() => {
                println!("lexicon v{version}: every netstratum term is served as netstratum means it");
                ExitCode::SUCCESS
            }
            Ok((version, missing)) => {
                println!("lexicon v{version}: not served as netstratum means them: {}", missing.join(", "));
                ExitCode::from(1)
            }
            Err(e) => {
                eprintln!("ns-hypha: the lexicon was not read: {e}");
                ExitCode::from(1)
            }
        },
        Some("probe") => {
            let cases = [("lawful turn, height advancing", probe::lawful(1), true), ("the same turn, height still", probe::timeless(1), false)];
            let mut ok = true;
            for (name, t, want) in cases {
                let Some(t) = t else {
                    eprintln!("  {name}: the bodies could not be carried");
                    ok = false;
                    continue;
                };
                match up.verify_both(&t) {
                    Ok(r) => {
                        let admitted = r.court.admitted();
                        println!(
                            "  {name}: court {} {:?}, local {:?}, agree {}",
                            if admitted { "ADMITTED" } else { "REFUSED" },
                            laws_of(&r.court).iter().map(|l| l.name()).collect::<Vec<_>>(),
                            laws_of(&r.local).iter().map(|l| l.name()).collect::<Vec<_>>(),
                            r.agree()
                        );
                        ok &= r.agree() && admitted == want;
                    }
                    Err(e) => {
                        eprintln!("  {name}: the court did not answer: {e}");
                        ok = false;
                    }
                }
            }
            if ok {
                println!("probe: the court at {at} reads both transitions as netstratum does");
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Some("found") => match up.found_domain(&home) {
            Ok(f) => {
                println!("domain: {f:?}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("ns-hypha: the domain was not founded: {e}");
                ExitCode::from(1)
            }
        },
        Some("census") => census_cmd(&args, Some(&up), &home),
        Some("prune") => prune_cmd(&up, &home),
        Some("publish") => publish_cmd(&up, &home),
        Some("mark") => mark_cmd(&args, &home),
        _ => {
            eprintln!("usage: ns-hypha laws|lexicon|probe|found|mark <world-dir>|epoch <name>|prune|publish|census <world-dir> [--last N] [--send] [--at host:port] [--home dir]");
            ExitCode::from(2)
        }
    }
}

fn load(dir: &Path) -> Result<(String, cfe::vrp::Chronicle), ExitCode> {
    let people = dir.file_name().and_then(|n| n.to_str()).unwrap_or("unnamed").to_string();
    match cfe::persist::load(&dir.join("chronicle.tlv")) {
        Ok(c) => Ok((people, c)),
        Err(e) => {
            eprintln!("ns-hypha: {}: the chronicle did not load: {e:?}", dir.display());
            Err(ExitCode::from(1))
        }
    }
}

/// PUBLISH every hot fold the court still reads Provisional: its entries
/// streamed for replay, so it is upheld before anyone can demand it.
fn publish_cmd(up: &Uplink, home: &Path) -> ExitCode {
    use ns_hypha::mycelium::anastome::dialogue::challenge::{ChallengeRefusal, Challenged, Finality};
    use ns_hypha::mycelium::chronometry::Stamp;
    let (line, log, key) = match TierLine::open(home, "colonies").and_then(|l| Ok((l, FoldLog::open(home, "colonies")?, tier_key(home, "colonies")?))) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("ns-hypha: the colonies line: {e}");
            return ExitCode::from(2);
        }
    };
    let mut ok = true;
    for f in log.folds.iter().filter(|f| f.first >= line.offset && f.first + f.count <= line.total()) {
        let Some(at) = Stamp::from_key(&f.at) else {
            eprintln!("  fold at entry {}: an unreadable stamp", f.first);
            ok = false;
            continue;
        };
        match up.finality(at) {
            Ok(Finality::Provisional { .. }) => {}
            Ok(other) => {
                println!("  fold at entries [{}, {}): {other:?}; nothing to publish", f.first, f.first + f.count);
                continue;
            }
            Err(e) => {
                eprintln!("  fold at entry {}: finality not read: {e}", f.first);
                ok = false;
                continue;
            }
        }
        let lo = (f.first - line.offset) as usize;
        let entries: Option<Vec<_>> = line.entries[lo..lo + f.count as usize].iter().map(|e| ns_hypha::line::decode_entry(&e.entry)).collect();
        let Some(entries) = entries else {
            eprintln!("  fold at entry {}: an entry on the line does not read", f.first);
            ok = false;
            continue;
        };
        match up.publish(&key, at, entries) {
            Ok(c @ Challenged::Upheld { .. }) => println!("  fold at entries [{}, {}): {c:?}", f.first, f.first + f.count),
            // Upheld before: the court holds the replay, and no demand can
            // be made on it — the fold is answered.
            Ok(Challenged::Refused(ChallengeRefusal::Published)) => println!("  fold at entries [{}, {}): already upheld", f.first, f.first + f.count),
            Ok(c) => {
                println!("  fold at entries [{}, {}): NOT UPHELD {c:?}", f.first, f.first + f.count);
                ok = false;
            }
            Err(e) => {
                eprintln!("  fold at entry {}: the court did not answer: {e}", f.first);
                ok = false;
            }
        }
    }
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

/// PRUNE BY FINALITY: archive the hot entries netted only by folds the
/// court reads `Final` or `PreLine` — never Provisional, Blocked, Falsified,
/// Unwound or Unrecorded — walking the folds in order and stopping at the
/// first that is not settled.
fn prune_cmd(up: &Uplink, home: &Path) -> ExitCode {
    use ns_hypha::mycelium::anastome::dialogue::challenge::Finality;
    use ns_hypha::mycelium::chronometry::Stamp;
    let (mut line, log) = match TierLine::open(home, "colonies").and_then(|l| Ok((l, FoldLog::open(home, "colonies")?))) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("ns-hypha: the colonies line: {e}");
            return ExitCode::from(2);
        }
    };
    let mut unread = None;
    let n = log.prunable(&line, |f| {
        let read = Stamp::from_key(&f.at).ok_or_else(|| "an unreadable stamp".to_string()).and_then(|at| up.finality(at).map_err(|e| e.to_string()));
        println!("  fold at entries [{}, {}): {read:?}", f.first, f.first + f.count);
        match read {
            Ok(Finality::Final { .. } | Finality::PreLine) => true,
            Ok(_) => false,
            Err(e) => {
                unread = Some(e);
                false
            }
        }
    });
    if let Some(e) = unread {
        eprintln!("ns-hypha: a finality was not read ({e}); pruned only what was read settled");
    }
    match line.prune_through(n) {
        Ok(()) => {
            println!("prune: {n} entries archived; hot line now {} entries from entry {}", line.entries.len(), line.offset);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("ns-hypha: the prune did not land: {e}");
            ExitCode::from(1)
        }
    }
}

/// Set the whole hot colonies line aside as an epoch and restart it at the
/// zero head (78.125: a tier's first fold under the challenge law starts its
/// line at zero; every fold before reads PreLine).
fn epoch_cmd(args: &[String], home: &Path) -> ExitCode {
    let Some(name) = args.get(2).filter(|a| !a.starts_with("--")) else {
        eprintln!("usage: ns-hypha epoch <name>");
        return ExitCode::from(2);
    };
    match TierLine::open(home, "colonies").and_then(|mut l| {
        let (n, head) = (l.entries.len(), l.head());
        l.restart(name).map(|kept| (n, head, kept, l.offset))
    }) {
        Ok((n, head, kept, offset)) => {
            let h: String = head.iter().take(6).map(|b| format!("{b:02x}")).collect();
            println!("colonies line: {n} entries (head {h}…) set aside as {}; hot line restarted at the zero head, next entry {offset}", kept.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("ns-hypha: the epoch was not started: {e}");
            ExitCode::from(1)
        }
    }
}

/// Mark a people's chronicle as folded through its current end, without
/// folding it — for history already netted before the cursor existed.
fn mark_cmd(args: &[String], home: &Path) -> ExitCode {
    let Some(dir) = args.get(2).filter(|a| !a.starts_with("--")) else {
        eprintln!("usage: ns-hypha mark <world-dir>");
        return ExitCode::from(2);
    };
    let (people, chronicle) = match load(Path::new(dir)) {
        Ok(x) => x,
        Err(code) => return code,
    };
    let Some(last) = chronicle.receipts().last() else {
        eprintln!("ns-hypha: {people}: an empty chronicle has nothing to mark");
        return ExitCode::from(1);
    };
    let n = chronicle.receipts().len();
    match Cursors::open(home, "colonies").and_then(|mut c| c.set(&people, n, last.own().0)) {
        Ok(()) => {
            println!("{people}: marked folded through {n} receipts");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("ns-hypha: the colonies cursors: {e}");
            ExitCode::from(2)
        }
    }
}

fn census_cmd(args: &[String], up: Option<&Uplink>, home: &Path) -> ExitCode {
    let Some(dir) = args.get(2).filter(|a| !a.starts_with("--")) else {
        eprintln!("usage: ns-hypha census <world-dir> [--last N] [--send]");
        return ExitCode::from(2);
    };
    let (people, chronicle) = match load(Path::new(dir)) {
        Ok(x) => x,
        Err(code) => return code,
    };
    let opened = Cursors::open(home, "colonies").and_then(|c| Ok((c, TierLine::open(home, "colonies")?, FoldLog::open(home, "colonies")?)));
    let (mut cursors, mut line, mut folds) = match opened {
        Ok(x) => x,
        Err(e) => {
            eprintln!("ns-hypha: the colonies line: {e}");
            return ExitCode::from(2);
        }
    };
    let receipts = chronicle.receipts();
    let len = receipts.len();
    // Resume where this people was last folded, never netting a receipt twice.
    let from = match cursors.at.get(&people) {
        Some(&(n, last)) => {
            if n > len || n == 0 || receipts[n - 1].own().0 != last {
                eprintln!("ns-hypha: {people}: the chronicle no longer holds the receipt its cursor names (receipt {n}); refused");
                return ExitCode::from(1);
            }
            n
        }
        None => {
            let last = flag(args, "--last").and_then(|n| n.parse::<usize>().ok()).unwrap_or(len);
            len.saturating_sub(last)
        }
    };
    if from == len {
        println!("census of {people}: folded through all {len} receipts; nothing new");
        return ExitCode::SUCCESS;
    }
    let c = match census::take(&people, &chronicle, from, len) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ns-hypha: {people}: no census: {e:?}");
            return ExitCode::from(1);
        }
    };
    let head: String = c.head.iter().take(6).map(|b| format!("{b:02x}")).collect();
    println!("census of {people}: receipts [{}, {}) of {len}, head {head}…, {} moved a holding", c.from, c.to, c.steps.len());
    println!("  holdings moved ({}):", c.rows.len());
    for (orb, h, v) in &c.rows {
        println!("    orb {orb:>4} {:<10} {:>14}", format!("{h:?}").replace("Matter(", "").replace(')', ""), grains(*v));
    }
    println!("  emitted by kind ({}):", c.declared.len());
    for (kind, h, v) in &c.declared {
        let side = if *v > 0 { "source" } else { "sink" };
        println!("    {side:<6} {kind:<24} {:<10} {:>14}", format!("{h:?}").replace("Matter(", "").replace(')', ""), grains(*v));
    }
    let key = match tier_key(home, "colonies") {
        Ok(k) => k,
        Err(e) => {
            eprintln!("ns-hypha: the colonies tier key: {e}");
            return ExitCode::from(2);
        }
    };
    // Each chunk of at most MAX_SPAN entries is its own fold: it nets those
    // per-receipt entries, its prefix is the line's head after the last of
    // them, and its entries are appended and the cursor advanced only once
    // the court admits it. A refusal stops the run where it stands, so the
    // cursor always names the last admitted fold.
    let chunks = c.chunks(census::MAX_SPAN);
    if chunks.len() > 1 {
        println!("  {} folds of at most {} entries", chunks.len(), census::MAX_SPAN);
    }
    for (n, chunk) in chunks.into_iter().enumerate() {
        let batch = chunk.entries();
        let (rows, declared) = chunk.nets();
        let signed = fold(&key, line.peek_all(&batch), rows, declared);
        if let Err(d) = signed.fold.continuity() {
            println!("  fold {n}: local continuity BROKEN {d:?}");
            return ExitCode::from(1);
        }
        println!("  fold {n}: receipts [{}, {}), {} entries onto a colonies line of {}; continuity holds", chunk.from, chunk.to, batch.len(), line.entries.len());
        let Some(up) = up else {
            continue;
        };
        if batch.is_empty() {
            // Nothing moved: no entry, no fold, only the cursor advances.
            if let Err(e) = cursors.set(&people, chunk.to, chunk.head) {
                eprintln!("  the cursor did not advance: {e}");
                return ExitCode::from(1);
            }
            println!("  nothing moved; cursor advanced to {}", chunk.to);
            continue;
        }
        match up.uplink_fold(&signed) {
            Ok(Uplinked::Admitted { at, .. }) => {
                let mark = FoldMark { at: at.key(), first: line.total(), count: batch.len() as u64 };
                let held = line.append_all(batch).and_then(|_| folds.push(mark)).and_then(|()| cursors.set(&people, chunk.to, chunk.head));
                if let Err(e) = held {
                    eprintln!("  fold {n}: ADMITTED BUT NOT HELD: the line or cursor did not take it: {e}");
                    return ExitCode::from(1);
                }
                let h: String = line.head().iter().take(6).map(|b| format!("{b:02x}")).collect();
                println!("  fold {n}: admitted at h {}; line {} entries, head {h}…; {people} folded through {}", at.h, line.entries.len(), chunk.to);
            }
            Ok(u) => {
                println!("  fold {n}: court {u:?}");
                return ExitCode::from(1);
            }
            Err(e) => {
                eprintln!("  fold {n}: the court did not answer: {e}");
                return ExitCode::from(1);
            }
        }
    }
    match up {
        None => {
            println!("  (not sent: pass --send to put the folds to the court)");
            ExitCode::SUCCESS
        }
        Some(up) if !args.iter().any(|a| a == "--no-publish") => publish_cmd(up, home),
        Some(_) => ExitCode::SUCCESS,
    }
}
