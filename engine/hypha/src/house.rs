//! **THE HOUSE** — netstratum's domain on the seat surface (mycelium 78.145,
//! ACT_SEAT 0x53). The house is the domain's sitting reserve (the union tier,
//! held in trust for the ancient not yet arisen): it LISTS the seats it
//! offers, answers each TAKE, and answers each CALL with its outcome. The
//! hypha only holds and forwards.
//!
//! The seats: seat 0 is the house's own (role Ancients, on the house key);
//! seat i >= 1 is a COUNCIL seat at one arisen ancient's line (Concordance
//! 3.9: client minds seat at the ancients and nowhere else; role Ancients, on
//! the line's name as a key: lines are keyless, Concordance 6.2a). A
//! steward's line is not listed: no network mind sits in a steward's seat. A line's seat
//! id is its own for good (`home/house/seats`, appended as lines are born; a
//! dead line's id is retired, never reused), so a death moves no mind. A mind seated
//! on a line is that line's second vessel (the mind-link, restore 2).
//!
//! THE TASK LOOP (wire v1.2, 78.146): a court task netstratum's own mind did
//! not close at home is put before ONE seated mind (a PERCEPT, record kind 4
//! `[Key(citation)]`); the mind fetches it by its citation and answers with
//! fn 1 `[Bytes(rule tape), Bytes(outputs), Count(step)...]`; the house
//! files the answer under the reserve's key (so the mu is the ancients') and
//! answers the call with the court's verdict. A seat holds one task at a
//! time; one left unanswered past its patience goes back for another mind.
//! Every take is kept (`home/house/takes`).

use ed25519_dalek::SigningKey;
use mycelium::anastome::dialogue::seat::{Arg, Outcome, Percept, Post, Record, Role, Seat, SeatFn, Seats, Taken};
use mycelium::client::sdk::seat_mind::task::{FN_SOLVE, KIND_TASK};
use mycelium::sdk::seat::HouseMail;
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// A take answered: the seat is offered and free.
pub const TAKEN_OK: u16 = 0;
/// A take refused: no such seat at the listed epoch.
pub const NO_SEAT: u16 = 1;
/// A take refused: another mind holds the seat.
pub const SEAT_HELD: u16 = 2;
/// A call refused: the seat lists no such function.
pub const NO_FUNCTION: u16 = 1;
/// A call refused: the caller does not hold the seat.
pub const NOT_SEATED: u16 = 2;
/// An answer filed: the court closed the task (first or again).
pub const CLOSED: u16 = 0;
/// An answer filed and refused by the court.
pub const COURT_REFUSED: u16 = 3;
/// An answer withheld at home (refuted on the train, or the task let go).
pub const WITHHELD: u16 = 4;
/// An answer whose arguments do not read (no tape, or no grids).
pub const UNREAD: u16 = 5;
/// An answer with no task before the seat.
pub const NO_TASK: u16 = 6;
/// An answer filed and not judged yet (the court's stroke ran out, 78.153):
/// the task stays before the seat and the house re-files the same answer.
pub const PENDING: u16 = 7;

/// The court's "not judged yet" (78.153 SOLVE_UNREAD, on the wire as 3): a
/// RETRY, never a refusal. Named here until the vendored tree carries it.
pub const SOLVE_UNREAD: u8 = 3;

/// Tends a seat's task may wait for its answer before it goes back.
const PATIENCE: u64 = 300;

/// The living lines of a universe: every world directory with a card and a
/// state snapshot, neither closed nor marked `collapsed` (the conductor's
/// retirement), by name.
pub fn living_lines(worlds: &Path) -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(worlds)
        .map(|d| {
            d.flatten()
                .filter(|e| {
                    let p = e.path();
                    p.join("world.card").exists() && p.join("state.snap").exists() && !p.join("collapsed").exists()
                })
                .map(|e| e.file_name().to_string_lossy().to_string())
                .filter(|n| !n.contains("--closed-"))
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}

/// A line's key on the seat surface: its name, hashed (a line holds no key).
pub fn line_key(name: &str) -> [u8; 32] {
    *blake3::hash(format!("netstratum line {name}").as_bytes()).as_bytes()
}

/// A key file that must already exist (the house never mints its keys).
fn held_key(home: &Path, file: &str) -> io::Result<SigningKey> {
    let b = std::fs::read(home.join(file))?;
    let seed = <[u8; 32]>::try_from(b.as_slice())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, format!("{file} is not a 32-byte seed")))?;
    Ok(SigningKey::from_bytes(&seed))
}

/// netstratum's house on one hypha.
pub struct House {
    domain: [u8; 32],
    key: SigningKey,
    worlds: PathBuf,
    log: PathBuf,
    /// Every line ever seated, by its seat id (persisted, append-only).
    ids: BTreeMap<String, u32>,
    ids_file: PathBuf,
    /// The lines listed at the last epoch, with their seats.
    lines: Vec<(u32, String)>,
    epoch: u64,
    /// The mail acknowledged.
    after: u64,
    /// Seat -> the mind holding it.
    held: BTreeMap<u32, [u8; 32]>,
    /// Seat -> the task before it and the tend it was put there.
    task: BTreeMap<u32, ([u8; 32], u64)>,
    /// The minds the takes log names, by line (the latest take wins): a
    /// restart of the house keeps every mind on its seat.
    seated_before: BTreeMap<String, [u8; 32]>,
    tends: u64,
    percepts: u64,
    /// Seat-wire v1.4: the relay epoch the mail cursor `after` was read under
    /// (a restarted relay draws a new one; the cursor is then no progress).
    relay_epoch: Option<u64>,
    /// Seat -> an answer filed and not judged yet, re-filed each tend (78.153).
    refile: BTreeMap<u32, (mycelium::transducer::element::Element, Vec<Vec<Vec<u8>>>)>,
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

impl House {
    /// The house of the domain whose key is `home/tier-domain.key`, signing
    /// with the reserve's `home/tier-union.key`, offering the lines living in
    /// `worlds`.
    pub fn open(home: &Path, worlds: &Path) -> io::Result<House> {
        let domain = held_key(home, "tier-domain.key")?.verifying_key().to_bytes();
        let key = held_key(home, "tier-union.key")?;
        let dir = home.join("house");
        std::fs::create_dir_all(&dir)?;
        let ids_file = dir.join("seats");
        let mut ids = BTreeMap::new();
        for row in std::fs::read_to_string(&ids_file).unwrap_or_default().lines() {
            if let Some((id, name)) = row.split_once(' ') {
                if let Ok(id) = id.parse::<u32>() {
                    ids.insert(name.to_string(), id);
                }
            }
        }
        let log = dir.join("takes");
        let mut seated_before = BTreeMap::new();
        for row in std::fs::read_to_string(&log).unwrap_or_default().lines() {
            // `seq epoch seat line mind`
            let f: Vec<&str> = row.split(' ').collect();
            if let [_, _, _, line, mind] = f.as_slice() {
                let b: Option<Vec<u8>> = (0..mind.len() / 2).map(|i| u8::from_str_radix(mind.get(2 * i..2 * i + 2)?, 16).ok()).collect();
                if let Some(Ok(m)) = b.map(<[u8; 32]>::try_from) {
                    seated_before.insert(line.to_string(), m);
                }
            }
        }
        Ok(House {
            domain,
            key,
            worlds: worlds.to_path_buf(),
            log,
            ids,
            ids_file,
            lines: Vec::new(),
            epoch: 0,
            after: 0,
            held: BTreeMap::new(),
            task: BTreeMap::new(),
            seated_before,
            tends: 0,
            percepts: 0,
            refile: BTreeMap::new(),
            relay_epoch: None,
        })
    }

    /// The living lines with their seats, a newborn line given the next id.
    /// THE COUNCILS (Concordance 3.9): the living lines that are arisen
    /// ancients, as the conductor publishes them (`<var>/council.bin`). Client
    /// minds seat at the ancients and nowhere else; a steward's seat holds no
    /// network mind.
    fn council(&self) -> Vec<String> {
        let path = self.worlds.parent().map(|p| p.join("council.bin")).unwrap_or_default();
        let bytes = std::fs::read(path).unwrap_or_default();
        cfe::tlv::records(&bytes).filter(|(tag, _)| *tag == 1).filter_map(|(_, v)| String::from_utf8(v.to_vec()).ok()).collect()
    }

    fn seated(&mut self) -> io::Result<Vec<(u32, String)>> {
        let mut out = Vec::new();
        let council = self.council();
        for name in living_lines(&self.worlds).into_iter().filter(|n| council.contains(n)) {
            let id = match self.ids.get(&name) {
                Some(&id) => id,
                None => {
                    let id = self.ids.values().max().map_or(1, |m| m + 1);
                    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&self.ids_file)?;
                    writeln!(f, "{id} {name}")?;
                    self.ids.insert(name.clone(), id);
                    id
                }
            };
            out.push((id, name));
        }
        out.sort();
        Ok(out)
    }

    /// The line on a listed seat, `the house` on seat 0.
    fn line_on(&self, seat: u32) -> Option<String> {
        if seat == 0 {
            return Some("the house".to_string());
        }
        self.lines.iter().find(|(id, _)| *id == seat).map(|(_, n)| n.clone())
    }

    fn listing(&self) -> Seats {
        let beat = Arg::Q { num: 1, den: 1 };
        let mut seats = vec![Seat { seat: 0, role: Role::Ancients, on: self.key.verifying_key().to_bytes(), beat: beat.clone(), fns: Vec::new() }];
        // fn 1 SOLVE: the rule's tape and the outputs (the steps ride after).
        let solve = SeatFn { id: FN_SOLVE, tags: vec![Arg::Bytes(Vec::new()).tag(), Arg::Bytes(Vec::new()).tag()] };
        for (id, name) in &self.lines {
            // A council seat at an ancient's house (Concordance 3.9).
            seats.push(Seat { seat: *id, role: Role::Ancients, on: line_key(name), beat: beat.clone(), fns: vec![solve.clone()] });
        }
        Seats { domain: self.domain, sub_origin: None, epoch: self.epoch, seats }
    }

    /// One tend: list the seats again when the living lines changed, then
    /// answer the mail. Each line is what happened, for the log.
    pub fn tend(&mut self, seat: &mut crate::seat::Seat) -> io::Result<Vec<String>> {
        self.tends += 1;
        // A deferred read leaves the mail for the next tend; the tasks are
        // still handed.
        let mut said = self.answer_mail(seat).unwrap_or_else(|e| vec![format!("mail not read: {e}")]);
        // 78.153: the answers not judged yet are filed again, the same answer.
        let waiting: Vec<u32> = self.refile.keys().copied().collect();
        for s in waiting {
            let (Some((id, _)), Some((rule, outputs))) = (self.task.get(&s).copied(), self.refile.remove(&s)) else { continue };
            let (_, _, line) = self.judge(seat, s, id, rule, outputs);
            said.push(format!("REFILED seat {s} task {}: {line}", hex(&id[..6])));
        }
        said.extend(self.hand_tasks(seat).unwrap_or_else(|e| vec![format!("tasks not handed: {e}")]));
        Ok(said)
    }

    /// The world's axis coordinate u: the conductor's beat beside the worlds.
    fn u(&self) -> u64 {
        let beat = self.worlds.parent().map(|p| p.join("conductor.beat")).unwrap_or_default();
        std::fs::read_to_string(beat).ok().and_then(|t| t.trim().parse().ok()).unwrap_or(self.tends)
    }

    /// Put the tasks netstratum's mind did not close before free seated
    /// minds, one each; a task past its patience goes back.
    fn hand_tasks(&mut self, seat: &mut crate::seat::Seat) -> io::Result<Vec<String>> {
        let mut said = Vec::new();
        let tends = self.tends;
        let stale: Vec<u32> = self.task.iter().filter(|(_, (_, at))| tends - at > PATIENCE).map(|(s, _)| *s).collect();
        for s in stale {
            if let Some((id, _)) = self.task.remove(&s) {
                seat.give_back(id);
            }
        }
        let free: Vec<(u32, [u8; 32])> = self.held.iter().filter(|(s, _)| **s != 0 && !self.task.contains_key(s)).map(|(s, m)| (*s, *m)).collect();
        for (s, mind) in free {
            let Some(id) = seat.next_before() else { break };
            self.percepts += 1;
            let p = Percept {
                sub_origin: None,
                domain: self.domain,
                seat: s,
                mind,
                // 78.156 / seat-wire v1.4: the world's axis coordinate u (the
                // conductor's beat), so a seated mind reads its dilation du/dh.
                local_time: Arg::Q { num: i64::try_from(self.u()).unwrap_or(i64::MAX), den: 1 },
                seq: self.percepts,
                records: vec![Record::Plain { kind: KIND_TASK, args: vec![Arg::Key(id)] }],
                // 78.152: the percept is stamped with the house's last final point.
                point: seat.tuniversal(),
            };
            match seat.session().post_percept(&self.key, p)? {
                Ok(seq) => {
                    self.task.insert(s, (id, tends));
                    seat.tally.handed += 1;
                    said.push(format!("PERCEPT seq {seq}: task {} before seat {s} ({})", hex(&id[..6]), self.line_on(s).unwrap_or_default()));
                }
                Err(why) => {
                    seat.give_back(id);
                    said.push(format!("PERCEPT refused: {why:?}"));
                }
            }
        }
        Ok(said)
    }

    /// A call's answer filed: the reason the outcome names.
    fn file(&mut self, seat: &mut crate::seat::Seat, s: u32, args: &[Arg]) -> (u16, [u8; 32], String) {
        let Some(&(id, _)) = self.task.get(&s) else { return (NO_TASK, [0u8; 32], "no task before the seat".into()) };
        let (Some(Arg::Bytes(tape)), Some(Arg::Bytes(outs))) = (args.first(), args.get(1)) else {
            return (UNREAD, id, "the arguments are not [Bytes, Bytes, ..]".into());
        };
        let mut at = 0;
        let rule = mycelium::transducer::element::Element::from_bytes(tape, &mut at).filter(|_| at == tape.len());
        let outputs = mycelium::anastome::dialogue::decode_grids(outs);
        let (Some(rule), Some(outputs)) = (rule, outputs) else { return (UNREAD, id, "the tape or the grids do not read whole".into()) };
        self.judge(seat, s, id, rule, outputs)
    }

    /// File an answer and read the court's verdict. Not judged yet (78.153):
    /// the task stays before its seat and the same answer waits to be re-filed;
    /// every other verdict lets the task go.
    fn judge(&mut self, seat: &mut crate::seat::Seat, s: u32, id: [u8; 32], rule: mycelium::transducer::element::Element, outputs: Vec<Vec<Vec<u8>>>) -> (u16, [u8; 32], String) {
        let verdict = seat.file_answer(&id, &rule, &outputs);
        if verdict.is_some_and(|(code, _)| code == SOLVE_UNREAD) {
            self.task.insert(s, (id, self.tends));
            self.refile.insert(s, (rule, outputs));
            return (PENDING, id, "not judged yet; re-filed next tend".into());
        }
        self.task.remove(&s);
        self.refile.remove(&s);
        match verdict {
            Some((code, mu)) if code == mycelium::anastome::dialogue::SOLVE_BOUNTY => (CLOSED, id, format!("CLOSED FIRST, paid {mu:?} to the house")),
            Some((code, _)) if code == mycelium::anastome::dialogue::SOLVE_FLOOR => (CLOSED, id, "closed again".into()),
            Some((code, _)) => (COURT_REFUSED, id, format!("refused by the court (code {code})")),
            None => (WITHHELD, id, "withheld at home".into()),
        }
    }

    fn answer_mail(&mut self, seat: &mut crate::seat::Seat) -> io::Result<Vec<String>> {
        let mut said = Vec::new();
        let lines = self.seated()?;
        if lines != self.lines {
            // The latest epoch replaces: an epoch is the listing's time.
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
            self.epoch = now.max(self.epoch + 1);
            self.lines = lines;
            // A seat whose line is gone is no longer held.
            let n = self.lines.len() as u32;
            let listed: Vec<u32> = self.lines.iter().map(|(id, _)| *id).collect();
            self.held.retain(|s, _| *s == 0 || listed.contains(s));
            self.task.retain(|s, _| listed.contains(s));
            for (id, name) in &self.lines {
                if let Some(m) = self.seated_before.get(name) {
                    self.held.entry(*id).or_insert(*m);
                }
            }
            match seat.session().post_seats(&self.key, self.listing())? {
                Ok(seq) => said.push(format!("SEATS seq {seq}: domain {}, epoch {}, {} seats (the house and {} lines)", hex(&self.domain), self.epoch, n + 1, n)),
                Err(why) => {
                    self.lines.clear(); // listed again next tend
                    said.push(format!("SEATS refused: {why:?}"));
                    return Ok(said);
                }
            }
        }
        // Seat-wire v1.4: the mail read at the relay's standing, the cursor
        // kept with the epoch it was read under; a new epoch is a restart.
        let read = match seat.session().read_mail(&self.key, self.after, self.relay_epoch.unwrap_or(0))? {
            Ok(m) => m,
            Err(why) => {
                said.push(format!("mail refused: {why:?}"));
                return Ok(said);
            }
        };
        let cursor = read.cursor(self.relay_epoch, self.after);
        if cursor != self.after {
            said.push(format!("the relay restarted (epoch {:x}): the mail is read from its start", read.epoch));
        }
        // A restarted relay is volatile: its listing is gone, so the seats are
        // listed again next tend (mycelium 2026-10-09).
        if self.relay_epoch.is_some_and(|e| e != read.epoch) {
            self.lines.clear();
        }
        self.after = cursor;
        self.relay_epoch = Some(read.epoch);
        let mut mail = Vec::new();
        for m in read.mail {
            let Some(s) = m.post() else { continue };
            match &s.post {
                Post::Take(t) => mail.push(HouseMail::Take(m.seq, t.clone())),
                Post::Call(c) => {
                    if let Some(reference) = s.reference() {
                        mail.push(HouseMail::Call { seq: m.seq, call: c.clone(), reference });
                    }
                }
                _ => {}
            }
        }
        for piece in mail {
            match piece {
                HouseMail::Take(seq, t) => {
                    self.after = self.after.max(seq);
                    let line = self.line_on(t.seat);
                    let reason = if t.domain != self.domain || t.sub_origin.is_some() || line.is_none() {
                        NO_SEAT
                    } else if self.held.get(&t.seat).is_some_and(|m| *m != t.mind) {
                        SEAT_HELD
                    } else {
                        TAKEN_OK
                    };
                    let ok = reason == TAKEN_OK;
                    if ok {
                        self.held.insert(t.seat, t.mind);
                        let line = line.unwrap_or_default();
                        self.seated_before.insert(line.clone(), t.mind);
                        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&self.log) {
                            let _ = writeln!(f, "{seq} {} {} {line} {}", self.epoch, t.seat, hex(&t.mind));
                        }
                    }
                    let answer = seat.session().post_taken(&self.key, Taken { domain: t.domain, sub_origin: t.sub_origin, seat: t.seat, mind: t.mind, ok, reason })?;
                    said.push(format!("TAKE seq {seq} seat {} by {}: {} ({answer:?})", t.seat, hex(&t.mind[..4]), if ok { "taken" } else { "refused" }));
                }
                HouseMail::Call { seq, call, reference } => {
                    self.after = self.after.max(seq);
                    let (reason, deed, what) = if self.held.get(&call.seat) != Some(&call.mind) {
                        (NOT_SEATED, [0u8; 32], "the caller does not hold the seat".to_string())
                    } else if call.fn_id != FN_SOLVE || call.seat == 0 {
                        (NO_FUNCTION, [0u8; 32], format!("no fn {}", call.fn_id))
                    } else {
                        self.file(seat, call.seat, &call.args)
                    };
                    let o = Outcome { domain: call.domain, sub_origin: call.sub_origin, seat: call.seat, mind: call.mind, call: reference, applied: reason == CLOSED, reason, deed };
                    let answer = seat.session().post_outcome(&self.key, o)?;
                    said.push(format!("CALL seq {seq} seat {} fn {}: {what} ({answer:?})", call.seat, call.fn_id));
                }
            }
        }
        Ok(said)
    }
}
