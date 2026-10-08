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

/// **THE CARRIED POSITION** (RECONSTRUCT v): beside the cursors, one record
/// a people — `positions-<tier>/<people with '/' as '~'>.pos`:
///
/// ```text
/// "netstratum census position v1" ‖ applied u64 ‖ head[32] ‖ bytes u64 ‖ codec::state_bytes
/// ```
///
/// written when every receipt read is folded: the census state after
/// `applied` receipts, the last of them hashing to `head` and ending at byte
/// `bytes` of the chronicle. A people is re-seated from it, reading only the
/// chronicle after `bytes` — never replayed from genesis.
const POSITION: &[u8] = b"netstratum census position v1";

fn position_path(home: &Path, people: &str) -> PathBuf {
    home.join("positions-colonies").join(format!("{}.pos", people.replace('/', "~")))
}

fn write_position(home: &Path, people: &str, f: &Followed) -> io::Result<()> {
    let path = position_path(home, people);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut bytes = POSITION.to_vec();
    bytes.extend_from_slice(&(f.stepper.applied() as u64).to_le_bytes());
    bytes.extend_from_slice(&f.head);
    bytes.extend_from_slice(&f.cursor.bytes.to_le_bytes());
    bytes.extend_from_slice(&codec::state_bytes(f.stepper.state()));
    let tmp = path.with_extension("pos.tmp");
    std::fs::write(&tmp, &bytes)?;
    std::fs::rename(&tmp, &path)
}

/// A people's carried position: (applied, head, bytes, state).
fn read_position(home: &Path, people: &str) -> Option<(usize, [u8; 32], u64, cfe::world::state::Committed)> {
    let bytes = std::fs::read(position_path(home, people)).ok()?;
    let rest = bytes.strip_prefix(POSITION)?;
    let applied = u64::from_le_bytes(rest.get(..8)?.try_into().ok()?) as usize;
    let head: [u8; 32] = rest.get(8..40)?.try_into().ok()?;
    let at = u64::from_le_bytes(rest.get(40..48)?.try_into().ok()?);
    let state = codec::from_state_bytes(rest.get(48..)?)?;
    Some((applied, head, at, state))
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
    /// The receipt count the carried position record last stood at.
    recorded: usize,
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
        if let Some(f) = self.seat_carried(dir, &people, cursors) {
            return Ok(f);
        }
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
        Ok(Followed { dir: dir.to_path_buf(), cursor, stepper, pending, folded, head, halted: None, reported_off: 0, recorded: 0 })
    }

    /// Seat a people from its CARRIED POSITION: the census state the record
    /// holds, and only the chronicle after it (verified from the record's
    /// head). `None` — and the whole read above, once — when there is no
    /// record, or it stands past the cursor (the line rolled back behind it),
    /// or the tail after it does not read or does not hold the cursor's
    /// receipt where the cursor names it.
    fn seat_carried(&self, dir: &Path, people: &str, cursors: &crate::line::Cursors) -> Option<Followed> {
        let (applied, head, at, state) = read_position(&self.home, people)?;
        let (folded, folded_head) = cursors.at.get(people).copied().unwrap_or((0, [0; 32]));
        if applied == 0 || applied > folded {
            return None;
        }
        let mut cursor = TailCursor { bytes: at, receipts: applied as u64, head: Some(cfe::vrp::ReceiptHash(head)) };
        let tail = read_tail(&dir.join("chronicle.tlv"), &mut cursor).ok()?;
        let caught = folded - applied;
        let folded_at = match caught {
            0 => head,
            n => tail.get(n - 1)?.own().0,
        };
        if folded_at != folded_head {
            return None;
        }
        let mut stepper = Stepper::from_state(state, applied);
        stepper.skip(&tail[..caught]).ok()?;
        let mut pending = Vec::new();
        for r in &tail[caught..] {
            if let Some(step) = stepper.step(r).ok()? {
                pending.push(step);
            }
        }
        let last = tail.last().map(|r| r.own().0).unwrap_or(head);
        Some(Followed { dir: dir.to_path_buf(), cursor, stepper, pending, folded, head: last, halted: None, reported_off: 0, recorded: applied })
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
        // on. A court that is away backs the round off (`Err`). A court whose
        // head stands EARLIER on this line (it restarted without its latest
        // admissions) is the authority: the line rolls back to it and the
        // receipts behind the lost entries fold again (`resync`). A head that
        // is not on this line at all halts the round, loudly.
        let court = up.line_head(self.key.verifying_key().to_bytes())?;
        if court != line.head() {
            match self.resync(court, &mut line, &mut log, &mut cursors)? {
                Some(note) => round.notes.push(note),
                None => {
                    let hex = |h: &[u8; 32]| h.iter().take(8).map(|b| format!("{b:02x}")).collect::<String>();
                    round.notes.push(format!(
                        "NOT FOLDING — the court's line head {}… is not on this line (head {}…); an operator must look",
                        hex(&court),
                        hex(&line.head())
                    ));
                    return Ok(round);
                }
            }
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

        // CARRY each fully folded people's position forward (RECONSTRUCT v).
        for (people, f) in self.peoples.iter_mut() {
            if f.halted.is_none() && f.pending.is_empty() && f.folded == f.stepper.applied() && f.folded > f.recorded {
                match write_position(&self.home, people, f) {
                    Ok(()) => f.recorded = f.folded,
                    Err(e) => round.notes.push(format!("{people}: position not carried: {e}")),
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

    /// ROLL BACK TO THE COURT'S HEAD when it stands earlier on this line:
    /// truncate the hot line to it, drop the logged folds after it, and send
    /// every people whose receipts those entries netted back to its first
    /// such receipt (cursor rolled back, held replay dropped, re-seated next
    /// round). `None` when the court's head is not on the hot line.
    fn resync(
        &mut self,
        court: [u8; 32],
        line: &mut TierLine,
        log: &mut FoldLog,
        cursors: &mut crate::line::Cursors,
    ) -> io::Result<Option<String>> {
        let keep = if court == line.base {
            0
        } else {
            match line.entries.iter().position(|e| e.head == court) {
                Some(i) => i + 1,
                None => return Ok(None),
            }
        };
        let cut = line.offset + keep as u64;
        let removed = line.truncate_to(keep)?;
        let folds_before = log.folds.len();
        log.truncate_from(cut)?;
        let dropped = folds_before - log.folds.len();
        let lost: std::collections::BTreeSet<[u8; 32]> = removed.iter().map(|e| e.cite).collect();
        // Which people wrote each lost receipt, and where it sits in its chronicle.
        let mut rolled = Vec::new();
        for dir in std::fs::read_dir(self.root.join("worlds"))?.flatten().map(|e| e.path()) {
            if dir.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.contains("--closed-")) {
                continue;
            }
            let Ok(chronicle) = cfe::persist::load(&dir.join("chronicle.tlv")) else { continue };
            let receipts = chronicle.receipts();
            if let Some(first) = receipts.iter().position(|r| lost.contains(&r.own().0)) {
                let people = people_of(&dir);
                if first == 0 {
                    cursors.clear(&people)?;
                } else {
                    cursors.set(&people, first, receipts[first - 1].own().0)?;
                }
                self.peoples.remove(&people);
                rolled.push(format!("{people}@{first}"));
            }
        }
        Ok(Some(format!(
            "RESYNC — the court's head stands {} entries back on this line (it restarted without them); rolled back, {} folds dropped, refolding: {}",
            removed.len(),
            dropped,
            rolled.join(", ")
        )))
    }

    /// Peoples followed, and those halted with why.
    pub fn standing(&self) -> (usize, Vec<(String, String)>) {
        let halted = self.peoples.iter().filter_map(|(p, f)| f.halted.clone().map(|why| (p.clone(), why))).collect();
        (self.peoples.len(), halted)
    }
}

#[cfg(test)]
mod tests {
    use super::{people_of, position_path, write_position, Follower};
    use crate::line::Cursors;
    use cfe::dop::ZoneKind;
    use cfe::world::World;
    use std::path::Path;

    #[test]
    fn a_people_reseats_from_its_carried_position_reading_only_the_tail() {
        let base = std::env::temp_dir().join(format!("hypha-position-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let (root, home) = (base.join("u1"), base.join("home"));
        let dir = root.join("worlds").join("meridian");
        std::fs::create_dir_all(&dir).unwrap();
        let carrier = dir.join("chronicle.tlv");
        let mut world = World::new();
        world.found("meridian").expect("founds");
        world.zone(4, ZoneKind::Extractor).expect("builds");
        for _ in 0..3 {
            world.tick();
        }
        world.collect().expect("settles");
        cfe::persist::sync(world.chronicle(), &carrier).unwrap();

        let follower = Follower::new(&root, &home).unwrap();
        let people = people_of(&dir);
        let mut cursors = Cursors::open(&home, "colonies").unwrap();
        let total = world.chronicle().total() as usize;
        let head = world.chronicle().receipts().last().unwrap().own().0;
        cursors.set(&people, total, head).unwrap();
        let seated = follower.seat(&dir, &cursors).unwrap();
        assert_eq!(seated.recorded, 0, "no record yet: the whole read, once");
        write_position(&home, &people, &seated).unwrap();

        // The world plays on; a re-seat resumes from the record.
        world.zone(9, ZoneKind::Habitat).expect("builds");
        world.tick();
        world.collect().expect("settles");
        cfe::persist::sync(world.chronicle(), &carrier).unwrap();
        let carried = follower.seat(&dir, &cursors).unwrap();
        assert_eq!(carried.recorded, total, "seated from the carried position");
        std::fs::remove_file(position_path(&home, &people)).unwrap();
        let whole = follower.seat(&dir, &cursors).unwrap();
        assert_eq!(whole.recorded, 0);
        assert_eq!(carried.stepper.applied(), whole.stepper.applied());
        assert_eq!(carried.stepper.state(), whole.stepper.state());
        assert_eq!(carried.head, whole.head);
        assert_eq!(carried.cursor, whole.cursor);
        let cites = |f: &super::Followed| f.pending.iter().map(|s| (s.receipt, s.cite, s.rows.clone())).collect::<Vec<_>>();
        assert_eq!(cites(&carried), cites(&whole));
        assert!(!carried.pending.is_empty(), "the new receipts moved holdings");

        // A record standing past the cursor (the line rolled back) is not used.
        write_position(&home, &people, &whole).unwrap();
        let prior = world.chronicle().receipts()[total - 2].own().0;
        cursors.set(&people, total - 1, prior).unwrap();
        assert_eq!(follower.seat(&dir, &cursors).unwrap().recorded, 0);
    }

    #[test]
    fn a_people_is_namespaced_by_its_universe_and_the_archive_keeps_bare_names() {
        assert_eq!(people_of(Path::new("var/worlds/tun-reach")), "tun-reach");
        assert_eq!(people_of(Path::new("/opt/netstratum/var-g2/worlds/tun-reach")), "var-g2/tun-reach");
        assert_eq!(people_of(Path::new("var-g2/worlds/tun-reach/")), "var-g2/tun-reach");
    }
}
