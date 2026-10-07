//! Netstratum's uplink to a mycelium hypha (MU-RECURSION 78.62).
//!
//! The hypha holds the laws (L0) and adjudicates; netstratum keeps its own
//! game — colonies, peoples, the STAMP economy, the chronicle — and puts
//! what it settles to the court. Two acts carry it:
//!
//!   * VERIFY: a transition read by the court and by this kernel with the
//!     same verifier (law 12 — one reader), so the two can only disagree if
//!     the builds differ.
//!   * FOLD: a tier's net positions per identity per lane over a prefix of
//!     the tier's own line ([`line`]), signed by the tier, admitted by
//!     continuity (78.119). [`census`] derives those folds from a colony's
//!     chronicle exactly.
//!
//! Every call goes to a real `mycelium-server` (78.59, "NO MOCK").

use mycelium::court::identity::HyphaKey;
use mycelium::physics::laws::{self, Breach, Law, Transition, Verdict};
use mycelium::sdk::Session;
use mycelium::transducer::cga::Undecomposed;
use std::io;
use std::path::Path;

pub use mycelium;

pub mod census;
pub mod fold;
pub mod line;
pub mod probe;

/// The court netstratum speaks to unless told otherwise
/// (`--at` / `NS_HYPHA`).
pub const DEFAULT_AT: &str = "127.0.0.1:7520";

/// One session to one hypha, under netstratum's own voice.
pub struct Uplink {
    session: Session,
    pub addr: String,
    pub voice: [u8; 32],
}

/// A transition read twice: by this kernel and by the court.
pub struct Reading {
    pub local: Verdict,
    pub court: Verdict,
}

impl Reading {
    /// The court and the kernel read the same breaches, law by law.
    pub fn agree(&self) -> bool {
        laws_of(&self.local) == laws_of(&self.court)
    }
}

/// A spent stroke reads as `Undecomposed(Unbounded)`: not measured, never
/// breached.
pub fn unmeasured(b: &Breach) -> bool {
    matches!(b, Breach::Undecomposed(Undecomposed::Unbounded(_)))
}

/// The laws a verdict says were BREACHED, not counting unmeasured readings.
pub fn laws_of(v: &Verdict) -> Vec<Law> {
    let mut l: Vec<Law> = v.breaches.iter().filter(|(_, b)| !unmeasured(b)).map(|(law, _)| *law).collect();
    l.sort_by_key(|law| law.index());
    l.dedup();
    l
}

/// The same verifier the court's act runs, under the stroke the motion's
/// own modes need.
pub fn verify_local(t: &Transition) -> Verdict {
    laws::verify_sized(t)
}

/// The terms netstratum speaks on the network, each in the vocabulary it
/// means it in (MU-LEXICON). "World" is not among them: in netstratum it is
/// the informal name of a people's chronicle, never a wire name.
pub const TERMS: [(&str, Vocab); 13] = [
    ("Universe", Vocab::Physical),
    ("Galaxy", Vocab::Physical),
    ("System", Vocab::Physical),
    ("Star", Vocab::Physical),
    ("Body", Vocab::Physical),
    ("People", Vocab::Civic),
    ("Money", Vocab::Civic),
    ("Value channel", Vocab::Civic),
    ("Ledger tree", Vocab::Civic),
    ("Domain", Vocab::Network),
    ("Tier", Vocab::Network),
    ("μ", Vocab::Network),
    ("POW++", Vocab::Network),
];

/// The lexicon's three vocabularies, named here so [`TERMS`] is a constant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vocab {
    Physical,
    Civic,
    Network,
}

impl Uplink {
    /// Open a session to `addr` (`host:port`) under the voice at
    /// `home/voice.key`, minted there on first use.
    pub fn open(addr: &str, home: &Path) -> io::Result<Uplink> {
        std::fs::create_dir_all(home)?;
        let key_path = home.join("voice.key");
        let key_name = key_path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "the key path is not UTF-8"))?;
        let voice = HyphaKey::load_or_mint(key_name).vk_bytes();
        let kernel = mycelium::sdk::Kernel::from_voice(voice);
        Ok(Uplink { session: Session::open(addr, kernel), addr: addr.to_string(), voice })
    }

    /// The register the hypha holds: every law at its coordinate.
    pub fn laws(&self) -> io::Result<Vec<mycelium::physics::laws::codec::Served>> {
        self.session.laws()
    }

    /// The server's lexicon version, and every term in [`TERMS`] it does not
    /// serve in the vocabulary netstratum means it in.
    pub fn lexicon_check(&self) -> io::Result<(u8, Vec<String>)> {
        use mycelium::physics::laws::lexicon::Vocabulary;
        let (version, served) = self.session.lexicon()?;
        let theirs = |v: Vocab| match v {
            Vocab::Physical => Vocabulary::Physical,
            Vocab::Civic => Vocabulary::Civic,
            Vocab::Network => Vocabulary::Network,
        };
        let missing = TERMS
            .iter()
            .filter(|(name, v)| !served.iter().any(|t| t.name == *name && t.vocabulary == theirs(*v)))
            .map(|(name, v)| format!("{name} ({v:?})"))
            .collect();
        Ok((version, missing))
    }

    /// `t` read by this kernel and by the court, side by side.
    pub fn verify_both(&self, t: &Transition) -> io::Result<Reading> {
        let court = self.session.verify(t)?;
        Ok(Reading { local: verify_local(t), court })
    }
}
