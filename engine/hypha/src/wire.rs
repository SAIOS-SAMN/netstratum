//! The house's acts on the wire (Mycelium ac8d425a / 7323c0c5: the sanctioned
//! way is that the house opens a `Session` and speaks the court's acts through
//! the public codecs; no house layer returns to the mind's SDK, as the house is
//! not a mind). Each act is encoded by `mycelium::anastome::dialogue`'s public
//! codec, spoken through `Session::speak` (the raw act door), and its reply
//! decoded by the matching public decoder: the same shapes the client SDK's
//! house wrappers had before Strip 7 removed them. A reply that does not
//! decode is an error by name, never a success.
use ed25519_dalek::SigningKey;
use mycelium::anastome::dialogue::challenge::{
    self, Challenged, FinalityReading, LineHead, SignedChallenge,
};
use mycelium::anastome::dialogue::domain::{
    self, Founded, SignedDomain, SignedOrigin, SignedReservation,
};
use mycelium::anastome::dialogue::seat::{
    self, MailRead, Outcome, Percept, Post, ReadMail, Refusal, Seats, SignedPost, Taken,
};
use mycelium::anastome::dialogue::uplink::{self, FoldAt, SignedFold, Uplinked};
use mycelium::sdk::Session;
use std::io;

fn undecoded(what: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("the {what} reply did not decode"),
    )
}

fn unwritten() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        "the act cannot be written (a denominator, or a length past its carrier)",
    )
}

fn nonce() -> io::Result<[u8; 32]> {
    use std::io::Read;
    let mut n = [0u8; 32];
    std::fs::File::open("/dev/urandom")?.read_exact(&mut n)?;
    Ok(n)
}

/// The house's acts, spoken on a session.
pub trait HouseActs {
    fn found_domain(&self, w: &SignedDomain) -> io::Result<Founded>;
    fn reserve(&self, r: &SignedReservation) -> io::Result<Founded>;
    fn originate(&self, o: &SignedOrigin) -> io::Result<Founded>;
    fn uplink_fold(&self, f: &SignedFold) -> io::Result<Uplinked>;
    fn challenge(&self, c: &SignedChallenge) -> io::Result<Challenged>;
    fn finality(&self, fold: FoldAt) -> io::Result<FinalityReading>;
    fn line_head(&self, tier: [u8; 32]) -> io::Result<LineHead>;
    fn seat_post(&self, s: &SignedPost) -> io::Result<Result<u64, Refusal>>;
    fn post_seats(&self, house: &SigningKey, seats: Seats) -> io::Result<Result<u64, Refusal>>;
    fn post_taken(&self, house: &SigningKey, t: Taken) -> io::Result<Result<u64, Refusal>>;
    fn post_percept(&self, house: &SigningKey, p: Percept) -> io::Result<Result<u64, Refusal>>;
    fn post_outcome(&self, house: &SigningKey, o: Outcome) -> io::Result<Result<u64, Refusal>>;
    /// The impressions the court holds for `key` past `after`: perceived.
    fn perceive(
        &self,
        key: &SigningKey,
        after: u64,
        epoch: u64,
    ) -> io::Result<Result<MailRead, Refusal>>;
    /// A court task for `voice` from the registry, or `None` where it holds
    /// nothing for it.
    fn pull_task(
        &self,
        voice: &[u8; 32],
    ) -> io::Result<Option<mycelium::anastome::dialogue::TaskEnvelope>>;
    /// A solve filed under `key` with its rule's tape: the court's verdict code
    /// and μ, or `None` where the reply does not read.
    fn file_solve(
        &self,
        key: &SigningKey,
        citation: &[u8; 32],
        outputs: &[Vec<Vec<u8>>],
        rule: &[u8],
    ) -> io::Result<Option<(u8, [u64; 4])>>;
}

impl HouseActs for Session {
    fn found_domain(&self, w: &SignedDomain) -> io::Result<Founded> {
        domain::decode_domain_reply(&self.speak(domain::encode_domain(w))?)
            .ok_or_else(|| undecoded("domain"))
    }
    fn reserve(&self, r: &SignedReservation) -> io::Result<Founded> {
        domain::decode_domain_reply(&self.speak(domain::encode_reservation(r))?)
            .ok_or_else(|| undecoded("reservation"))
    }
    fn originate(&self, o: &SignedOrigin) -> io::Result<Founded> {
        domain::decode_domain_reply(&self.speak(domain::encode_origin(o))?)
            .ok_or_else(|| undecoded("origin"))
    }
    fn uplink_fold(&self, f: &SignedFold) -> io::Result<Uplinked> {
        uplink::decode_uplink_reply(&self.speak(uplink::encode_uplink(f))?)
            .ok_or_else(|| undecoded("fold"))
    }
    fn challenge(&self, c: &SignedChallenge) -> io::Result<Challenged> {
        challenge::decode_challenge_reply(&self.speak(challenge::encode_challenge(c))?)
            .ok_or_else(|| undecoded("challenge"))
    }
    fn finality(&self, fold: FoldAt) -> io::Result<FinalityReading> {
        challenge::decode_finality_reading(&self.speak(challenge::encode_finality_request(fold))?)
            .ok_or_else(|| undecoded("finality"))
    }
    fn line_head(&self, tier: [u8; 32]) -> io::Result<LineHead> {
        challenge::decode_line_head_reply(&self.speak(challenge::encode_line_head_request(tier))?)
            .ok_or_else(|| undecoded("line head"))
    }
    fn seat_post(&self, s: &SignedPost) -> io::Result<Result<u64, Refusal>> {
        seat::decode_post_reply(&self.speak(seat::encode_post(s).ok_or_else(unwritten)?)?)
            .ok_or_else(|| undecoded("post"))
    }
    fn post_seats(&self, house: &SigningKey, seats: Seats) -> io::Result<Result<u64, Refusal>> {
        self.seat_post(&Post::Seats(seats).sign(house).ok_or_else(unwritten)?)
    }
    fn post_taken(&self, house: &SigningKey, t: Taken) -> io::Result<Result<u64, Refusal>> {
        self.seat_post(&Post::Taken(t).sign(house).ok_or_else(unwritten)?)
    }
    /// 78.152 TU-7: a percept the house left unstamped is stamped with this
    /// session's last final point before it is signed.
    fn post_percept(&self, house: &SigningKey, p: Percept) -> io::Result<Result<u64, Refusal>> {
        let p = Percept {
            point: p.point.or(self.tuniversal()),
            ..p
        };
        self.seat_post(&Post::Percept(p).sign(house).ok_or_else(unwritten)?)
    }
    fn post_outcome(&self, house: &SigningKey, o: Outcome) -> io::Result<Result<u64, Refusal>> {
        self.seat_post(&Post::Outcome(o).sign(house).ok_or_else(unwritten)?)
    }
    fn perceive(
        &self,
        key: &SigningKey,
        after: u64,
        epoch: u64,
    ) -> io::Result<Result<MailRead, Refusal>> {
        let act = seat::encode_read_mail(&ReadMail::sign_standing(key, after, epoch, nonce()?));
        seat::decode_mail_reply_standing(&self.speak(act)?).ok_or_else(|| undecoded("perception"))
    }
    fn pull_task(
        &self,
        voice: &[u8; 32],
    ) -> io::Result<Option<mycelium::anastome::dialogue::TaskEnvelope>> {
        let reply = self.speak(mycelium::anastome::dialogue::percept::encode_task_request(
            voice,
        ))?;
        Ok(mycelium::anastome::dialogue::percept::decode_task_envelope(
            &reply,
        ))
    }
    fn file_solve(
        &self,
        key: &SigningKey,
        citation: &[u8; 32],
        outputs: &[Vec<Vec<u8>>],
        rule: &[u8],
    ) -> io::Result<Option<(u8, [u64; 4])>> {
        let act = mycelium::anastome::dialogue::percept::encode_solve_with_rule(
            key,
            citation,
            outputs,
            Some(rule),
        )
        .ok_or_else(unwritten)?;
        Ok(mycelium::anastome::dialogue::percept::decode_solve_receipt(
            &self.speak(act)?,
        ))
    }
}
