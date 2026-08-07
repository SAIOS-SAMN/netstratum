//! **TREATIES — the political spectrum, as pairwise chartered law
//! (#42).**
//!
//! Councils, bonds and forwards already bind parties to QUANTITIES. A
//! treaty binds them to a RELATIONSHIP: it is a co-signed charter over
//! the channels that exist only between two worlds, and its kind is the
//! politics.
//!
//! **The four kinds, and why they are four.** They are not a taste;
//! they are the sign of what a pact does to the pair, crossed with
//! whether it opens or closes the way between them:
//!
//! ```text
//!                 EASES the crossing      BURDENS it
//!   binds terms   TRADE, DEFENSE          WAR (and WALLS it)
//!   binds none    PEACE (bars war)        —
//! ```
//!
//! TRADE and DEFENSE both ease the melt; what separates them is what
//! they cost to hold. A defense pact SWEARS OFF WAR with its partner —
//! you cannot declare war on a world you swore to defend — so it
//! spends an option that a trade pact leaves in your hand.
//!
//! TRADE and DEFENSE are CONCORD: every term must ease. WAR is
//! DISCORD: every term must burden. PEACE binds nothing at all — its
//! whole content is that it bars war, which is why it carries no terms
//! and cannot be made to carry any.
//!
//! **The law that makes it a treaty and not a decree** is that peace
//! and war are MUTUALLY EXCLUSIVE on a pair. Sealing peace over a
//! standing war ends it; sealing war over a standing peace BREAKS that
//! peace, and the breach is receipted against the breaker. Nothing else
//! in the engine lets one world's deed rewrite another's standing, and
//! that asymmetry is exactly what makes war worth refusing.

use crate::ratio::Ratio;

/// **THE SILENT FACTOR** — what a channel reads where nothing binds
/// it, DERIVED rather than written.
///
/// This is the multiplicative identity, and it is not a dial: writing
/// it as a literal three times over would raise the tuned-constant
/// count for a number that is not tuned. The vocabulary already
/// answers it — an empty binding's factor on any channel — so the one
/// home answers here too.
fn silent() -> Ratio {
    crate::vocabulary::factor_on(&[], PAIR_MELT)
}

/// **THE PAIR CHANNELS.** Deliberately a SEPARATE table from
/// `vocabulary::CHANNELS`, not an extension of it.
///
/// The world channels price what a world does to itself, and a class,
/// a practice or a minted building binds them. These price what two
/// worlds do to EACH OTHER, and only a treaty binds them. Keeping the
/// tables apart is what stops a civilization from minting itself a
/// cheaper melt with nobody's consent — a pair quantity may only be
/// moved by both parties, which is the whole idea of a treaty.
///
/// As with the world table, only channels the engine ALREADY computes
/// appear here: a channel is landed by wiring it, never by a pact
/// naming it.
/// The transit melt on this pair — the void's cut of every haul
/// between them (`world::MELT_PER_UNIT`, and `world::pair_melt` is its
/// one producer). LESS is better.
pub const PAIR_MELT: u8 = 1;

/// The closed pair table — ONE channel, and deliberately one.
///
/// The melt is the only quantity this engine currently computes
/// BETWEEN two worlds; the reciprocity window, the crisis books and
/// the friction rate all price what happens inside one. A second pair
/// channel enters here when a second pair quantity is wired, never
/// because a pact would like to name one — the world table's own rule,
/// applied to a smaller table.
///
/// One channel is enough for the whole spectrum because the kinds are
/// not distinguished by WHICH channel they bind. Trade and defense
/// both ease the crossing; what separates them is that a defense pact
/// also BARS you from declaring war on the world you swore it to.
pub const PAIR_CHANNELS: [u8; 1] = [PAIR_MELT];

pub fn pair_channel_valid(channel: u8) -> bool {
    PAIR_CHANNELS.contains(&channel)
}

/// Does a factor above one HELP the pair on this channel? Melt is a
/// COST, so raising it hurts and lowering it helps. Without this the
/// concord law cannot tell an easing from a burden, and every pact
/// would read the same.
pub fn pair_favours_increase(channel: u8) -> bool {
    // No pair channel yet favours an increase; the melt is a cost and
    // is currently the only one. Written as a match so a channel that
    // DOES gets an arm rather than an inverted default.
    match channel {
        PAIR_MELT => false,
        _ => false,
    }
}

/// **THE KINDS.** Wire codes are append-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreatyKind {
    /// Eases the way goods travel — the melt on the pair falls.
    Trade = 1,
    /// Eases the crossing AND swears the pair off war: a world cannot
    /// declare war on one it holds a defense pact with. The strongest
    /// bond, because it spends the option to attack.
    Defense = 2,
    /// Binds nothing and bars war. The absence of a burden IS the pact.
    Peace = 3,
    /// Burdens the pair, and WALLS it: no aperture crosses between
    /// worlds at war.
    War = 4,
}

impl TreatyKind {
    pub fn from_wire(code: u8) -> Option<TreatyKind> {
        match code {
            1 => Some(TreatyKind::Trade),
            2 => Some(TreatyKind::Defense),
            3 => Some(TreatyKind::Peace),
            4 => Some(TreatyKind::War),
            _ => None,
        }
    }
    pub fn wire(self) -> u8 {
        self as u8
    }
    pub fn word(self) -> &'static str {
        match self {
            TreatyKind::Trade => "trade",
            TreatyKind::Defense => "defense",
            TreatyKind::Peace => "peace",
            TreatyKind::War => "war",
        }
    }
    /// Does this pact EASE the pair, burden it, or neither?
    pub fn concord(self) -> bool {
        matches!(self, TreatyKind::Trade | TreatyKind::Defense)
    }
    /// **THE WALL.** A pair at war exchanges nothing: the aperture
    /// refuses in both directions, exactly as the elder's ward does.
    /// War is expensive because it costs the trade, not because a
    /// number was chosen to make it so.
    pub fn walls(self) -> bool {
        matches!(self, TreatyKind::War)
    }
    /// The kind this one displaces where both would stand. Peace ends a
    /// war; war breaks a peace. Trade and defense displace nothing —
    /// a pair may hold both, and hold them alongside a peace.
    pub fn displaces(self) -> Option<TreatyKind> {
        match self {
            TreatyKind::Peace => Some(TreatyKind::War),
            TreatyKind::War => Some(TreatyKind::Peace),
            _ => None,
        }
    }
}

/// The named fault a refused pact carries — each names the term that
/// broke it, because the way up is the point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreatyFault {
    /// The channel is not one the engine computes between two worlds.
    ChannelUnknown { channel: u8 },
    /// The factor falls outside the band.
    FactorOutOfBounds { channel: u8 },
    /// A concord that burdens, or a discord that eases — the pact's
    /// terms contradict the pact's kind.
    TermsContradictKind { channel: u8 },
    /// A peace carrying terms. Peace binds NOTHING; a peace with terms
    /// is a trade pact wearing peace's name, and it would let a pair
    /// bar war for free.
    PeaceBindsNothing,
    /// A concord or discord carrying no terms at all — a pact that
    /// changes nothing between two worlds is not a pact. (Peace is the
    /// deliberate exception, and it is the only one.)
    TermsEmpty,
}

/// **TERMS THAT MATCH THEIR KIND, proven at construction** (P1,
/// 29_CONSTRUCTION — the `TradingBinding` precedent).
///
/// Every way a pact can contradict itself dies here, so the verb that
/// seals one can never ask again: a trade pact that raises the melt, a
/// war that eases it, a peace that smuggles terms, a pact that binds
/// nothing at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreatyTerms {
    kind: TreatyKind,
    terms: Vec<(u8, Ratio)>,
}

impl TreatyTerms {
    pub fn new(kind: TreatyKind, terms: Vec<(u8, Ratio)>) -> Result<TreatyTerms, TreatyFault> {
        if let Some((channel, _)) = terms.iter().find(|(c, _)| !pair_channel_valid(*c)) {
            return Err(TreatyFault::ChannelUnknown { channel: *channel });
        }
        // PEACE BINDS NOTHING — and nothing is what it must bind.
        if kind == TreatyKind::Peace {
            return match terms.is_empty() {
                true => Ok(TreatyTerms { kind, terms }),
                false => Err(TreatyFault::PeaceBindsNothing),
            };
        }
        if terms.is_empty() {
            return Err(TreatyFault::TermsEmpty);
        }
        // THE BAND — the same fourfold pair every chartered factor in
        // this engine sits inside (`vocabulary::fourfold_band`, one
        // home), so a treaty cannot walk a rate anywhere a class or a
        // practice could not.
        let (low, high) = crate::vocabulary::fourfold_band();
        for (channel, factor) in &terms {
            if *factor < low || *factor > high {
                return Err(TreatyFault::FactorOutOfBounds { channel: *channel });
            }
            // THE POLITICS, ENFORCED AS ARITHMETIC. A term EASES when
            // it moves the channel the way the pair wants it; a
            // concord's every term must ease, a discord's every term
            // must burden.
            let eases = match pair_favours_increase(*channel) {
                true => *factor > silent(),
                false => *factor < silent(),
            };
            if eases != kind.concord() {
                return Err(TreatyFault::TermsContradictKind { channel: *channel });
            }
        }
        Ok(TreatyTerms { kind, terms })
    }
    pub fn kind(&self) -> TreatyKind {
        self.kind
    }
    pub fn terms(&self) -> &[(u8, Ratio)] {
        &self.terms
    }
    pub fn into_terms(self) -> Vec<(u8, Ratio)> {
        self.terms
    }
}

/// One pact as a world's book holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Treaty {
    /// The pairing key — the aperture's deal-id pattern, so both books
    /// name the same pact without either owning it.
    pub deal: u64,
    pub kind: TreatyKind,
    /// The world on the other side, by name.
    pub partner: String,
    pub terms: Vec<(u8, Ratio)>,
    /// The stamp it was sealed at, and how long it stands. A term of
    /// zero stands until broken — the bond's own shape.
    pub sealed: u64,
    pub term: u64,
}

impl Treaty {
    /// Has this pact run out? A termless pact never does.
    pub fn expired(&self, now: u64) -> bool {
        self.term > 0 && now > self.sealed + self.term
    }
}

/// **THE FACTOR A PAIR'S STANDING PACTS APPLY ON ONE CHANNEL** — the
/// product of every live pact naming it, or 1 where they are silent.
///
/// A product rather than a lookup, so two pacts compose instead of one
/// shadowing the other: a trade pact and a defense pact between the
/// same worlds both bite.
pub fn factor_on(treaties: &[Treaty], partner: &str, channel: u8, now: u64) -> Ratio {
    treaties
        .iter()
        .filter(|held| held.partner == partner && !held.expired(now))
        .flat_map(|held| held.terms.iter())
        .filter(|(held, _)| *held == channel)
        .fold(silent(), |acc, (_, factor)| acc * factor.clone())
}

/// Does a live pact of this kind stand with this partner?
pub fn stands(treaties: &[Treaty], partner: &str, kind: TreatyKind, now: u64) -> bool {
    treaties
        .iter()
        .any(|held| held.partner == partner && held.kind == kind && !held.expired(now))
}

/// **IS THIS PAIR AT WAR?** The reading every crossing consults.
pub fn walled(treaties: &[Treaty], partner: &str, now: u64) -> bool {
    treaties
        .iter()
        .any(|held| held.partner == partner && held.kind.walls() && !held.expired(now))
}
