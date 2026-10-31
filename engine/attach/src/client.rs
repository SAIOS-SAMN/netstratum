//! The client an external agent or sandbox uses to attach to Netstratum
//! (docs/109 §3): it connects to a hypha with its own key, reads the house's
//! listing, calls `attach` on the door seat, and acts through the world seat
//! it is given. Every act is a signed seat post, spoken through Mycelium's
//! session with Mycelium's public seat codecs.

use crate::iface::*;
use ed25519_dalek::SigningKey;
use mycelium::anastome::dialogue::seat::{self, Arg, Call, Outcome, Percept, Post, ReadMail, Record, Refusal, Seats, SignedPost, Take, Taken};
use mycelium::sdk::{Kernel, Session};
use std::io;

fn undecoded(what: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("the {what} reply did not decode"))
}

fn unwritten() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "the post cannot be written")
}

fn nonce() -> io::Result<[u8; 32]> {
    use std::io::Read;
    let mut n = [0u8; 32];
    std::fs::File::open("/dev/urandom")?.read_exact(&mut n)?;
    Ok(n)
}

/// What a mind reads in its mail, typed for attachment.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// The house answered a take.
    Taken(Taken),
    /// The mind was admitted: its line's key and its world seat.
    Admitted { line: [u8; 32], seat: u32 },
    /// The mind was not admitted, with the reason (`NO_VIABLE_WORLD`).
    NotAdmitted { reason: u64 },
    /// A call's result: applied (`true`) or refused.
    Result { call: [u8; 32], applied: bool },
    /// The house's outcome for a call (`QUEUED`, or a refusal).
    Outcome(Outcome),
    /// Any other percept (for example a council task).
    Percept(Percept),
}

/// Type a percept's records as attachment events.
pub fn events_of(p: &Percept) -> Vec<Event> {
    let mut out = Vec::new();
    for r in &p.records {
        match r {
            Record::Plain { kind, args } if *kind == KIND_ADMITTED => {
                if let [Arg::Key(line), Arg::Count(seat), ..] = args.as_slice() {
                    out.push(Event::Admitted { line: *line, seat: u32::try_from(*seat).unwrap_or(u32::MAX) });
                }
            }
            Record::Plain { kind, args } if *kind == KIND_NOT_ADMITTED => {
                if let [Arg::Count(reason), ..] = args.as_slice() {
                    out.push(Event::NotAdmitted { reason: *reason });
                }
            }
            Record::Plain { kind, args } if *kind == KIND_RESULT => {
                if let [Arg::Key(call), Arg::Count(applied), ..] = args.as_slice() {
                    out.push(Event::Result { call: *call, applied: *applied == 1 });
                }
            }
            _ => out.push(Event::Percept(p.clone())),
        }
    }
    out
}

/// A mind attached, or attaching, to Netstratum's domain on one hypha.
pub struct Client {
    session: Session,
    key: SigningKey,
    domain: [u8; 32],
    after: u64,
    epoch: Option<u64>,
}

impl Client {
    /// Connect to the hypha at `addr` (`host:port`) as the mind holding `key`,
    /// for Netstratum's domain `domain`.
    pub fn open(addr: &str, key: SigningKey, domain: [u8; 32]) -> Client {
        let kernel = Kernel::from_key(key.clone());
        Client { session: Session::open(addr, kernel), key, domain, after: 0, epoch: None }
    }

    /// This mind's key.
    pub fn mind(&self) -> [u8; 32] {
        self.key.verifying_key().to_bytes()
    }

    fn post(&self, p: Post) -> io::Result<(Result<u64, Refusal>, [u8; 32])> {
        let signed: SignedPost = p.sign(&self.key).ok_or_else(unwritten)?;
        let reference = signed.reference().ok_or_else(unwritten)?;
        let reply = self.session.speak(seat::encode_post(&signed).ok_or_else(unwritten)?)?;
        Ok((seat::decode_post_reply(&reply).ok_or_else(|| undecoded("post"))?, reference))
    }

    /// The house's latest listing of its seats, or `None` where none is
    /// posted.
    pub fn listing(&self) -> io::Result<Option<Seats>> {
        let reply = self.session.speak(seat::encode_read_listing(&self.domain, None))?;
        let listed = seat::decode_listing_reply(&reply, None).ok_or_else(|| undecoded("listing"))?;
        Ok(listed.and_then(|s| match s.post {
            Post::Seats(seats) => Some(seats),
            _ => None,
        }))
    }

    /// Take `seat` (a world seat the house admitted this mind to).
    pub fn take(&self, at: u32) -> io::Result<Result<u64, Refusal>> {
        let t = Take { domain: self.domain, sub_origin: None, seat: at, mind: self.mind(), nonce: nonce()? };
        Ok(self.post(Post::Take(t))?.0)
    }

    /// Call `fn_id` on `seat` with `args`: the relay's answer and the call's
    /// reference, which its outcome and its result name.
    pub fn call(&self, at: u32, fn_id: u16, args: Vec<Arg>) -> io::Result<(Result<u64, Refusal>, [u8; 32])> {
        let c = Call { domain: self.domain, sub_origin: None, seat: at, mind: self.mind(), nonce: nonce()?, fn_id, args };
        self.post(Post::Call(c))
    }

    /// Ask to attach with `metabolism` (`mineral`, `biotic` or `plasmic`).
    pub fn attach(&self, metabolism: &str) -> io::Result<(Result<u64, Refusal>, [u8; 32])> {
        let code = metabolism_code(metabolism).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("no metabolism named {metabolism}")))?;
        self.call(DOOR, FN_ATTACH, vec![Arg::Count(code)])
    }

    /// Zone `tile` (0–63) with the zone kind whose wire code is `kind`.
    pub fn zone(&self, at: u32, tile: u64, kind: u64) -> io::Result<(Result<u64, Refusal>, [u8; 32])> {
        self.call(at, FN_ZONE, vec![Arg::Count(tile), Arg::Count(kind)])
    }

    /// Raze `tile`.
    pub fn raze(&self, at: u32, tile: u64) -> io::Result<(Result<u64, Refusal>, [u8; 32])> {
        self.call(at, FN_RAZE, vec![Arg::Count(tile)])
    }

    /// Collect the world's harvest.
    pub fn collect(&self, at: u32) -> io::Result<(Result<u64, Refusal>, [u8; 32])> {
        self.call(at, FN_COLLECT, Vec::new())
    }

    /// Open a sandbox (a sub-universe at the line's core).
    pub fn sandbox(&self, at: u32) -> io::Result<(Result<u64, Refusal>, [u8; 32])> {
        self.call(at, FN_SANDBOX, Vec::new())
    }

    /// Detach: the mind's authority in its world ends at the next beat.
    pub fn detach(&self, at: u32) -> io::Result<(Result<u64, Refusal>, [u8; 32])> {
        self.call(at, FN_DETACH, Vec::new())
    }

    /// The mind's new mail, typed: takes answered, outcomes, admissions and
    /// results. The cursor advances; a restarted relay is read from its start.
    pub fn mail(&mut self) -> io::Result<Result<Vec<Event>, Refusal>> {
        let act = seat::encode_read_mail(&ReadMail::sign_standing(&self.key, self.after, self.epoch.unwrap_or(0), nonce()?));
        let read = match seat::decode_mail_reply_standing(&self.session.speak(act)?).ok_or_else(|| undecoded("mail"))? {
            Ok(r) => r,
            Err(why) => return Ok(Err(why)),
        };
        // The relay's standing first: a restarted relay is read from its start.
        self.after = read.cursor(self.epoch, self.after);
        self.epoch = Some(read.epoch);
        let mut out = Vec::new();
        for m in &read.mail {
            self.after = self.after.max(m.seq);
            let Some(s) = m.post() else { continue };
            match s.post {
                Post::Taken(t) => out.push(Event::Taken(t)),
                Post::Outcome(o) => out.push(Event::Outcome(o)),
                Post::Percept(p) => out.extend(events_of(&p)),
                _ => {}
            }
        }
        Ok(Ok(out))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn percept(records: Vec<Record>) -> Percept {
        Percept { domain: [0; 32], sub_origin: None, seat: 7, mind: [1; 32], local_time: Arg::Count(0), seq: 1, records, point: None }
    }

    #[test]
    fn admission_and_result_percepts_type_as_events() {
        let p = percept(vec![
            Record::Plain { kind: KIND_ADMITTED, args: vec![Arg::Key([9; 32]), Arg::Count(12)] },
            Record::Plain { kind: KIND_RESULT, args: vec![Arg::Key([8; 32]), Arg::Count(1)] },
            Record::Plain { kind: KIND_NOT_ADMITTED, args: vec![Arg::Count(NO_VIABLE_WORLD as u64)] },
        ]);
        assert_eq!(
            events_of(&p),
            vec![
                Event::Admitted { line: [9; 32], seat: 12 },
                Event::Result { call: [8; 32], applied: true },
                Event::NotAdmitted { reason: NO_VIABLE_WORLD as u64 },
            ]
        );
    }
}
