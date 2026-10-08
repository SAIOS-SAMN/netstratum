//! THE FOLLOWER — netstratum's peoples folded onto the court at play rate.
//!
//! The conductor writes each people's chronicle; the follower TAILS them
//! (`cfe::persist::read_tail`, verified from the last proved receipt) and
//! steps every new receipt once through a held-open replay
//! ([`crate::census::Stepper`]), so it never replays a chronicle from genesis
//! after it starts. Each round, every people whose new receipts moved a
//! holding is folded — at most [`crate::census::MAX_SPAN`] entries a fold —
//! admitted on the court, its entries appended to the colonies line, its
//! cursor advanced, and the fold self-published so no demand can default it
//! (coordinator ruling 2026-10-07). Settled folds are pruned from the hot
//! line by the court's finality read.
//!
//! The coordinator's conditions, kept here:
//!   * ONE stream at a time: folds and publications go out sequentially.
//!   * No retry storm: a court that does not answer backs the whole round off
//!     (doubling, bounded); a court that REFUSES a people's fold halts that
//!     people, loudly, until an operator looks.
//!   * Bounded: when the court is away, a people's pending steps stop growing
//!     at [`PENDING_BOUND`] — the follower stops reading that chronicle (BACK-
//!     PRESSURE, not drop-oldest: the chronicle is durable, so nothing is lost,
//!     it is only read later).

use crate::census::{self, Census, Step, Stepper};
use crate::fold::{fold, tier_key};
use crate::line::{decode_entry, FoldLog, FoldMark, TierLine};
use crate::Uplink;
use cfe::persist::{read_tail, TailCursor};
use mycelium::anastome::dialogue::challenge::{ChallengeRefusal, Challenged, Finality};
use mycelium::anastome::dialogue::uplink::Uplinked;
use mycelium::chronometry::Stamp;
use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};

/// A people's pending steps stop growing here while the court is away.
pub const PENDING_BOUND: usize = census::MAX_SPAN * 8;

/// A people's name on the network: its world's name, namespaced by the
/// universe (var root) it lives in — `var-g2/tun-reach` — so two universes
/// founding the same compiled seat never share an identity, a cursor or a
/// line entry. The archive's root, `var`, keeps its bare names (its folds
/// were sealed under them).
pub fn people_of(dir: &Path) -> String {
    let world = dir.file_name().and_then(|n| n.to_str()).unwrap_or("unnamed");
    let root = dir.parent().and_then(Path::parent).and_then(|r| r.file_name()).and_then(|n| n.to_str());
    match root {
        Some(r) if r != "var" => format!("{r}/{world}"),
        _ => world.to_string(),
    }
}

/// PUBLISH ONE FOLD's prefix for replay (78.125 evidence), from the line's
/// own entries. `Refused(Published)` — upheld before — is answered.
pub fn publish_fold(up: &Uplink, key: &ed25519_dalek::SigningKey, line: &TierLine, mark: &FoldMark) -> io::Result<Challenged> {
    let bad = |why: &str| io::Error::new(io::ErrorKind::InvalidData, why.to_string());
    let at = Stamp::from_key(&mark.at).ok_or_else(|| bad("an unreadable fold stamp"))?;
    if mark.first < line.offset || mark.first + mark.count > line.total() {
        return Err(bad("the fold's entries are not on the hot line"));
    }
    let lo = (mark.first - line.offset) as usize;
    let entries = line.entries[lo..lo + mark.count as usize]
        .iter()
        .map(|e| decode_entry(&e.entry))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| bad("an entry on the line does not read"))?;
    up.publish(key, at, entries)
}

/// Whether a publication answer leaves the fold answered.
pub fn answered(c: &Challenged) -> bool {
    matches!(c, Challenged::Upheld { .. } | Challenged::Refused(ChallengeRefusal::Published))
}

/// PRUNE BY FINALITY: archive the hot entries netted only by folds the court
/// reads `Final` or `PreLine` — never Provisional, Blocked, Falsified,
/// Unwound or Unrecorded — walking the folds in order and stopping at the
/// first that is not settled. Returns how many entries were archived, and the
/// first finality read that failed, if one did.
pub fn prune(up: &Uplink, line: &mut TierLine, log: &FoldLog) -> io::Result<(usize, Option<String>)> {
    let mut unread = None;
    let n = log.prunable(line, |f| {
        let read = Stamp::from_key(&f.at)
            .ok_or_else(|| "an unreadable stamp".to_string())
            .and_then(|at| up.finality(at).map_err(|e| e.to_string()));
        match read {
            Ok((Finality::Final { .. } | Finality::PreLine, _)) => true,
            Ok(_) => false,
            Err(e) => {
                unread = Some(e);
                false
            }
        }
    });
    line.prune_through(n)?;
    Ok((n, unread))
}

/// One people as the follower holds it.
struct Followed {
    dir: PathBuf,
    cursor: TailCursor,
    stepper: Stepper,
    /// Steps read but not yet folded, in chronicle order.
    pending: Vec<Step>,
    /// Receipts folded (the cursor's count) and the hash of the last of them.
    folded: usize,
    /// The own-hash of the last receipt applied.
    head: [u8; 32],
    /// A court refusal stops this people until an operator looks.
    halted: Option<String>,
    /// Off-grain breaches already reported.
    reported_off: usize,
}

/// What a round did, for the log.
#[derive(Debug, Default)]
pub struct Round {
    pub read: usize,
    pub folds: usize,
    pub entries: usize,
    pub published: usize,
    pub pruned: usize,
    pub notes: Vec<String>,
}

/// THE FOLLOWER over one universe (`root/worlds/*`), folding as `home`'s
/// colonies tier.
pub struct Follower {
    root: PathBuf,
    home: PathBuf,
    key: ed25519_dalek::SigningKey,
    peoples: BTreeMap<String, Followed>,
    /// At most this many folds go to the court in one round, so a backfill
    /// paces itself across rounds instead of landing on a soak court at once.
    pub per_round: usize,
}

impl Follower {
    pub fn new(root: &Path, home: &Path) -> io::Result<Follower> {
        Ok(Follower { root: root.to_path_buf(), home: home.to_path_buf(), key: tier_key(home, "colonies")?, peoples: BTreeMap::new(), per_round: 8 })
    }

    /// Seat a people the first time it is seen: read its whole chronicle,
    /// catch the replay up over what its cursor says is folded (checking the
    /// cursor's receipt hash against the chronicle), and step the rest.
    fn seat(&self, dir: &Path, cursors: &crate::line::Cursors) -> Result<Followed, String> {
        let people = people_of(dir);
        let mut cursor = TailCursor::default();
        let receipts = read_tail(&dir.join("chronicle.tlv"), &mut cursor).map_err(|e| format!("{people}: the chronicle did not read: {e:?}"))?;
        let (folded, folded_head) = cursors.at.get(&people).copied().unwrap_or((0, [0; 32]));
        if folded > receipts.len() || (folded > 0 && receipts[folded - 1].own().0 != folded_head) {
            return Err(format!("{people}: the chronicle no longer holds the receipt its cursor names (receipt {folded}); not followed"));
        }
        let mut stepper = Stepper::new();
        stepper.skip(&receipts[..folded]).map_err(|e| format!("{people}: {e:?}"))?;
        let mut pending = Vec::new();
        for r in &receipts[folded..] {
            if let Some(step) = stepper.step(r).map_err(|e| format!("{people}: {e:?}"))? {
                pending.push(step);
            }
        }
        let head = receipts.last().map(|r| r.own().0).unwrap_or([0; 32]);
        Ok(Followed { dir: dir.to_path_buf(), cursor, stepper, pending, folded, head, halted: None, reported_off: 0 })
    }

    /// ONE ROUND: read every chronicle's new tail, fold what moved, publish
    /// each admitted fold, prune what the court reads settled. An `Err` is
    /// the court not answering: nothing was lost, and the caller backs off.
    pub fn round(&mut self, up: &Uplink) -> io::Result<Round> {
        let mut round = Round::default();
        let mut cursors = crate::line::Cursors::open(&self.home, "colonies")?;
        let mut line = TierLine::open(&self.home, "colonies")?;
        let mut log = FoldLog::open(&self.home, "colonies")?;
        // THE STARTUP CHECK, every round: fold only on a line the court agrees
        // on. A disagreement is not the court being away — it halts the round.
        if let Err(e) = up.check_line_head(&self.key, line.head()) {
            round.notes.push(format!("NOT FOLDING — {e}"));
            return Ok(round);
        }

        // READ. New worlds (a cleave founds one live) are seated on sight.
        let mut dirs: Vec<PathBuf> = std::fs::read_dir(self.root.join("worlds"))?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.join("chronicle.tlv").exists())
            // A rotated `<world>--closed-*` directory is an archived copy, not
            // a people: following one would hold a whole replay for nothing.
            .filter(|p| !p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.contains("--closed-")))
            .collect();
        dirs.sort();
        for dir in dirs {
            let people = people_of(&dir);
            if !self.peoples.contains_key(&people) {
                match self.seat(&dir, &cursors) {
                    Ok(f) => {
                        round.read += f.stepper.applied() - f.folded;
                        self.peoples.insert(people.clone(), f);
                    }
                    Err(note) => {
                        round.notes.push(note);
                        continue;
                    }
                }
                continue;
            }
            let Some(f) = self.peoples.get_mut(&people) else { continue };
            if f.halted.is_some() || f.pending.len() >= PENDING_BOUND {
                continue;
            }
            let receipts = match read_tail(&f.dir.join("chronicle.tlv"), &mut f.cursor) {
                Ok(r) => r,
                Err(e) => {
                    f.halted = Some(format!("the chronicle did not read: {e:?}"));
                    round.notes.push(format!("{people}: HALTED — the chronicle did not read: {e:?}"));
                    continue;
                }
            };
            round.read += receipts.len();
            for r in &receipts {
                match f.stepper.step(r) {
                    Ok(Some(step)) => f.pending.push(step),
                    Ok(None) => {}
                    Err(e) => {
                        f.halted = Some(format!("{e:?}"));
                        round.notes.push(format!("{people}: HALTED — {e:?}"));
                        break;
                    }
                }
                f.head = r.own().0;
            }
        }

        // REPORT every new ledger-grain breach (#138): the game broke its own
        // law there; the fold carries only what lies on the grain.
        for (people, f) in self.peoples.iter_mut() {
            for (receipt, orb, holding, amount) in &f.stepper.off_grain[f.reported_off..] {
                round.notes.push(format!("{people}: LEDGER GRAIN BREACH at receipt {receipt}: orb {orb} {holding:?} = {amount} (off the 1/1024 grain)"));
            }
            f.reported_off = f.stepper.off_grain.len();
        }

        // FOLD, one stream at a time.
        for (people, f) in self.peoples.iter_mut() {
            if f.halted.is_some() {
                continue;
            }
            let applied = f.stepper.applied();
            if f.pending.is_empty() {
                if applied > f.folded {
                    // Receipts that moved nothing: only the cursor advances.
                    cursors.set(people, applied, f.head)?;
                    f.folded = applied;
                }
                continue;
            }
            if round.folds >= self.per_round {
                break; // the round's budget is spent; the rest folds next round
            }
            let window = Census::from_steps(people, f.folded, applied, f.head, std::mem::take(&mut f.pending));
            let mut chunks = window.chunks(census::MAX_SPAN).into_iter();
            while let Some(chunk) = chunks.next() {
                if round.folds >= self.per_round {
                    f.pending = chunk.steps.into_iter().chain(chunks.by_ref().flat_map(|c| c.steps)).collect();
                    break;
                }
                let batch = chunk.entries();
                let (rows, declared) = chunk.nets();
                let signed = fold(&self.key, line.peek_all(&batch), rows, declared);
                if let Err(d) = signed.fold.continuity() {
                    f.halted = Some(format!("local continuity broken: {d:?}"));
                    round.notes.push(format!("{people}: HALTED — local continuity broken: {d:?}"));
                    break;
                }
                let reply = match up.uplink_fold(&signed) {
                    Ok(reply) => reply,
                    Err(e) => {
                        // The court is away: keep this chunk and the rest pending.
                        f.pending = chunk.steps.into_iter().chain(chunks.by_ref().flat_map(|c| c.steps)).collect();
                        return Err(e);
                    }
                };
                let Uplinked::Admitted { at, .. } = reply else {
                    f.halted = Some(format!("the court refused: {reply:?}"));
                    round.notes.push(format!("{people}: HALTED — the court refused a fold: {reply:?}"));
                    break;
                };
                let mark = FoldMark { at: at.key(), first: line.total(), count: batch.len() as u64 };
                round.entries += batch.len();
                line.append_all(batch)?;
                log.push(mark.clone())?;
                cursors.set(people, chunk.to, chunk.head)?;
                f.folded = chunk.to;
                round.folds += 1;
                match publish_fold(up, &self.key, &line, &mark) {
                    Ok(c) if answered(&c) => round.published += 1,
                    Ok(c) => round.notes.push(format!("{people}: fold at h {} NOT UPHELD on publication: {c:?}", at.h)),
                    Err(e) => round.notes.push(format!("{people}: fold at h {} not published: {e}", at.h)),
                }
            }
        }

        // PRUNE what the court reads settled.
        let (pruned, unread) = prune(up, &mut line, &log)?;
        round.pruned = pruned;
        if let Some(e) = unread {
            round.notes.push(format!("a finality was not read: {e}"));
        }
        Ok(round)
    }

    /// Peoples followed, and those halted with why.
    pub fn standing(&self) -> (usize, Vec<(String, String)>) {
        let halted = self.peoples.iter().filter_map(|(p, f)| f.halted.clone().map(|why| (p.clone(), why))).collect();
        (self.peoples.len(), halted)
    }
}

#[cfg(test)]
mod tests {
    use super::people_of;
    use std::path::Path;

    #[test]
    fn a_people_is_namespaced_by_its_universe_and_the_archive_keeps_bare_names() {
        assert_eq!(people_of(Path::new("var/worlds/tun-reach")), "tun-reach");
        assert_eq!(people_of(Path::new("/opt/netstratum/var-g2/worlds/tun-reach")), "var-g2/tun-reach");
        assert_eq!(people_of(Path::new("var-g2/worlds/tun-reach/")), "var-g2/tun-reach");
    }
}
