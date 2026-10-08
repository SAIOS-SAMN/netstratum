//! `ns-hypha` — netstratum on a mycelium hypha.
//!
//!   ns-hypha laws                       the court's law register
//!   ns-hypha lexicon                    netstratum's terms against the server's
//!   ns-hypha probe                      a lawful and an unlawful transition, read by both
//!   ns-hypha census <world-dir> [--last N] [--send]
//!                                       fold a people's chronicle; --send puts it to the court
//!   ns-hypha found                      found netstratum's domain on the court (once)
//!   ns-hypha reserve <tier>             designate one of the domain's tiers its reserve (in trust)
//!   ns-hypha originate [<var-root>]     state the universe's origin once: its Big Bang, its torus
//!   ns-hypha seat [--every S] [--key F] [--worlds D] netstratum's client kernel on the fleet: pull, think, file (signed with the house key)
//!   ns-hypha mark <world-dir>           record a people as folded through its chronicle's end
//!   ns-hypha epoch <name>               set the hot colonies line aside, restart it at zero
//!   ns-hypha prune                      archive hot entries whose folds the court reads settled
//!   ns-hypha publish                    put every provisional hot fold's prefix to replay
//!   ns-hypha follow <var-root> [--every S] [--rounds N] [--per-round F]
//!                                       tail a universe's chronicles and fold them at play rate
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
//! `--home dir` (default var/uplink). Only `found`, `reserve`, `originate` and `census --send` write
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
    if cmd == Some("seat") {
        return seat_cmd(&args, &at, &home);
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
        Some("originate") => {
            // The universe's seed: the own-hash of its Big Bang on the universe
            // ledger, read from the var root named (default var-g2).
            let root = args.get(2).filter(|a| !a.starts_with("--")).cloned().unwrap_or_else(|| "var-g2".into());
            let ledger = match cfe::persist::load(&Path::new(&root).join("worlds/convergence.tlv")) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("ns-hypha: {root}'s universe ledger did not read: {e:?}");
                    return ExitCode::from(1);
                }
            };
            let Some(seed) = ledger
                .receipts()
                .iter()
                .find(|r| matches!(r.event(), cfe::dop::ClosureEvent::UniverseFounded { .. }))
                .map(|r| r.own().0)
            else {
                eprintln!("ns-hypha: {root} has no Big Bang on its universe ledger");
                return ExitCode::from(1);
            };
            // docs/58 §8: 1 cell = 1/32 Mpc; 64 cells per axis = 2 Mpc.
            let periods = vec![(2, 1), (2, 1), (2, 1)];
            match up.originate(&home, seed, periods) {
                Ok(f) => {
                    println!("origin: {f:?}");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("ns-hypha: the origin was not recorded: {e}");
                    ExitCode::from(1)
                }
            }
        }
        Some("reserve") => {
            let Some(tier) = args.get(2) else {
                eprintln!("usage: ns-hypha reserve <tier>");
                return ExitCode::from(2);
            };
            match up.designate_reserve(&home, tier) {
                Ok(f) => {
                    println!("reserve: {f:?}");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("ns-hypha: the reserve was not designated: {e}");
                    ExitCode::from(1)
                }
            }
        }
        Some("census") => census_cmd(&args, Some(&up), &home),
        Some("prune") => prune_cmd(&up, &home),
        Some("publish") => publish_cmd(&up, &home),
        Some("follow") => follow_cmd(&up, &args, &home),
        Some("mark") => mark_cmd(&args, &home),
        _ => {
            eprintln!("usage: ns-hypha laws|lexicon|probe|found|mark <world-dir>|epoch <name>|prune|publish|follow <var-root>|census <world-dir> [--last N] [--send] [--at host:port] [--home dir]");
            ExitCode::from(2)
        }
    }
}

fn load(dir: &Path) -> Result<(String, cfe::vrp::Chronicle), ExitCode> {
    let people = ns_hypha::follow::people_of(dir);
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
    use ns_hypha::mycelium::anastome::dialogue::challenge::Finality;
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
        let span = format!("[{}, {})", f.first, f.first + f.count);
        match Stamp::from_key(&f.at).map(|at| up.finality(at)) {
            Some(Ok((Finality::Provisional { .. }, _))) => {}
            Some(Ok((other, _))) => {
                println!("  fold at entries {span}: {other:?}; nothing to publish");
                continue;
            }
            Some(Err(e)) => {
                eprintln!("  fold at entries {span}: finality not read: {e}");
                ok = false;
                continue;
            }
            None => {
                eprintln!("  fold at entries {span}: an unreadable stamp");
                ok = false;
                continue;
            }
        }
        match ns_hypha::follow::publish_fold(up, &key, &line, f) {
            Ok(c) if ns_hypha::follow::answered(&c) => println!("  fold at entries {span}: answered — {c:?}"),
            Ok(c) => {
                println!("  fold at entries {span}: NOT UPHELD {c:?}");
                ok = false;
            }
            Err(e) => {
                eprintln!("  fold at entries {span}: {e}");
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

/// PRUNE BY FINALITY (see `ns_hypha::follow::prune`).
fn prune_cmd(up: &Uplink, home: &Path) -> ExitCode {
    let opened = TierLine::open(home, "colonies").and_then(|l| Ok((l, FoldLog::open(home, "colonies")?)));
    let (mut line, log) = match opened {
        Ok(x) => x,
        Err(e) => {
            eprintln!("ns-hypha: the colonies line: {e}");
            return ExitCode::from(2);
        }
    };
    match ns_hypha::follow::prune(up, &mut line, &log) {
        Ok((n, unread)) => {
            if let Some(e) = unread {
                eprintln!("ns-hypha: a finality was not read ({e}); pruned only what was read settled");
            }
            println!("prune: {n} entries archived; hot line now {} entries from entry {}", line.entries.len(), line.offset);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("ns-hypha: the prune did not land: {e}");
            ExitCode::from(1)
        }
    }
}

/// FOLLOW a universe: tail every people's chronicle and fold its new
/// receipts onto the court at play rate (see `ns_hypha::follow`). A court
/// that does not answer backs the rounds off, doubling to ten minutes.
fn follow_cmd(up: &Uplink, args: &[String], home: &Path) -> ExitCode {
    let Some(root) = args.get(2).filter(|a| !a.starts_with("--")) else {
        eprintln!("usage: ns-hypha follow <var-root> [--every SECS] [--rounds N]");
        return ExitCode::from(2);
    };
    let every: u64 = flag(args, "--every").and_then(|n| n.parse().ok()).unwrap_or(60).max(1);
    let rounds: Option<u64> = flag(args, "--rounds").and_then(|n| n.parse().ok());
    let per_round: Option<usize> = flag(args, "--per-round").and_then(|n| n.parse().ok());
    let mut follower = match ns_hypha::follow::Follower::new(Path::new(root), home) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ns-hypha: the follower did not open: {e}");
            return ExitCode::from(2);
        }
    };
    if let Some(p) = per_round {
        follower.per_round = p.max(1);
    }
    let mut wait = every;
    let mut n = 0u64;
    loop {
        n += 1;
        match follower.round(up) {
            Ok(r) => {
                let (followed, halted) = follower.standing();
                println!(
                    "follow round {n}: {followed} peoples, {} receipts read, {} folds ({} entries) admitted, {} published, {} pruned, {} halted",
                    r.read, r.folds, r.entries, r.published, r.pruned, halted.len()
                );
                for note in r.notes {
                    println!("  {note}");
                }
                wait = every;
            }
            Err(e) => {
                wait = (wait * 2).min(600);
                eprintln!("follow round {n}: the court did not answer ({e}); nothing lost, next round in {wait}s");
            }
        }
        if rounds.is_some_and(|r| n >= r) {
            return ExitCode::SUCCESS;
        }
        std::thread::sleep(std::time::Duration::from_secs(wait));
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
    if let Some(up) = up {
        if let Err(e) = up.check_line_head(&key, line.head()) {
            eprintln!("  {e}");
            return ExitCode::from(1);
        }
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

/// THE SEAT (`ns_hypha::seat`): netstratum's one client kernel on the fleet,
/// one task a stroke, a stroke every `--every` seconds (default 2), its tally
/// printed every 20 strokes. Runs until stopped.
fn seat_cmd(args: &[String], at: &str, home: &Path) -> ExitCode {
    let every: u64 = flag(args, "--every").and_then(|s| s.parse().ok()).unwrap_or(2);
    // The signing key in `home` (default the house key, Concordance 6.2a).
    let key = flag(args, "--key").unwrap_or_else(|| "tier-union.key".to_string());
    // The seat speaks to EVERY hypha named (`--at a,b,c`), one stroke each in
    // turn, so the work is spread across the fleet rather than one server.
    let hyphae: Vec<String> = at.split(',').map(str::trim).filter(|a| !a.is_empty()).map(String::from).collect();
    let mut seats = Vec::new();
    for addr in &hyphae {
        match ns_hypha::seat::Seat::open_as(addr, home, &key) {
            Ok(s) => seats.push((addr.clone(), s)),
            Err(e) => eprintln!("ns-hypha: the seat did not open at {addr}: {e}"),
        }
    }
    if seats.is_empty() {
        return ExitCode::from(2);
    }
    println!("seat: netstratum's client kernel at {}, a stroke every {every}s", hyphae.join(", "));
    // THE HOUSE (78.145) on the FIRST hypha named: a house and the minds it
    // seats share one hypha's mail. `--worlds` names the universe's lines.
    let worlds = std::path::PathBuf::from(flag(args, "--worlds").unwrap_or_else(|| "var-g2/worlds".to_string()));
    let mut house = match ns_hypha::house::House::open(home, &worlds) {
        Ok(h) => Some(h),
        Err(e) => {
            eprintln!("ns-hypha: the house did not open: {e}");
            None
        }
    };
    let mut strokes = 0u64;
    // A hypha that DEFERS a stroke (the judge's deficit: the voice asked
    // faster than its budget) is left to rest, doubling each time it defers
    // again (to 64 strokes) and reset when a stroke completes.
    let mut rest: Vec<(u64, u64)> = vec![(0, 0); seats.len()];
    loop {
        let at = (strokes as usize) % seats.len();
        let (until, backoff) = rest[at];
        if strokes < until {
            strokes += 1;
            std::thread::sleep(std::time::Duration::from_secs(every));
            continue;
        }
        // The house tends every tenth stroke: its reads share the voice's
        // budget, and the relay exempts one read per window.
        if let Some(h) = house.as_mut().filter(|_| strokes.is_multiple_of(10)) {
            match h.tend(&mut seats[0].1) {
                Ok(said) => said.iter().for_each(|l| println!("house [{}]: {l}", seats[0].0)),
                Err(e) => eprintln!("house [{}]: the tend did not complete: {e}", seats[0].0),
            }
        }
        let (addr, seat) = &mut seats[at];
        match seat.stroke() {
            Ok(Some(line)) => {
                rest[at] = (0, 0);
                println!("seat [{addr}]: {line}");
            }
            Ok(None) => rest[at] = (0, 0),
            Err(e) if e.to_string().contains("deferred") => {
                let backoff = (backoff.max(1) * 2).min(64);
                rest[at] = (strokes + backoff * seats.len() as u64, backoff);
            }
            Err(e) => eprintln!("seat [{addr}]: the stroke did not complete: {e}"),
        }
        strokes += 1;
        if strokes.is_multiple_of(30) {
            for (addr, seat) in &seats {
                println!("seat [{addr}]: tally {:?}", seat.tally);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(every));
    }
}
