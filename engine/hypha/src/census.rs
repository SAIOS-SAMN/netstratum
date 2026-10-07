//! THE CENSUS FOLD — a people's chronicle, netted for the court.
//!
//! The chronicle is the record: an append-only, hash-chained run of
//! receipts, and the colony's state is its replay. A census over the window
//! `[from, to)` steps the replay one receipt at a time and reads every
//! colony orb's holdings before and after each receipt:
//!
//!   * ROWS are each holding's net change over the window — one identity per
//!     (people, orb, holding), matter on [`lane::MATTER`], STAMP on
//!     [`lane::MONEY`], in units of the resource ledger's 1/1024 grain.
//!   * DECLARED is what the receipts themselves made or unmade: whatever a
//!     receipt's net change on a holding is, summed across every orb, is
//!     emitted by that receipt's KIND — one identity per (people, kind,
//!     holding). A transfer between two orbs nets to zero and declares
//!     nothing; a harvest declares a source, an upkeep a sink.
//!
//! Nothing is classified by hand: which kinds are sources and which are
//! sinks is read off the replay, so a kind that makes value where the game's
//! law says it only moves value shows up by name in the declared list.
//! Continuity holds by construction (both lists sum the same per-receipt
//! changes); what the court's admission adds is a signed, sealed commitment
//! to the window's positions on the `colonies` tier's line, whose record
//! cites the chronicle head they were read at ([`crate::line`]).

use crate::fold::lane;
use cfe::dop::{ColonyOrb, Resource};
use cfe::ratio::Ratio;
use cfe::vrp::Chronicle;
use cfe::world::replay::Replayer;
use cfe::world::RESOURCE_LEDGER_GRAIN;
use mycelium::anastome::dialogue::uplink::Net;
use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};
use std::collections::BTreeMap;

/// What a colony orb holds: a banked resource, or its STAMP.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Holding {
    Matter(Resource),
    Stamp,
}

impl Holding {
    fn lane(self) -> usize {
        match self {
            Holding::Matter(_) => lane::MATTER,
            Holding::Stamp => lane::MONEY,
        }
    }

    fn name(self) -> String {
        match self {
            Holding::Matter(r) => format!("{r:?}"),
            Holding::Stamp => "STAMP".to_string(),
        }
    }
}

/// Why a census could not be taken.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CensusRefused {
    /// The window does not lie inside the chronicle.
    Window { from: usize, to: usize, len: usize },
    /// A holding is not on the 1/1024 grain, so it has no integer count.
    OffGrain { receipt: usize, orb: u64, holding: Holding, amount: String },
    /// A count passes the court's i128 carrier: refused, never wrapped.
    Overflow { orb: u64, holding: Holding },
}

/// One holding's count in grain units.
fn units(amount: &Ratio) -> Option<BigInt> {
    let scaled = amount * Ratio::from_integer(BigInt::from(RESOURCE_LEDGER_GRAIN));
    scaled.is_integer().then(|| scaled.to_integer())
}

type Positions = BTreeMap<(u64, Holding), BigInt>;

fn positions(orbs: &[ColonyOrb], receipt: usize) -> Result<Positions, CensusRefused> {
    let mut at = Positions::new();
    for orb in orbs {
        let held = orb.banked.iter().map(|(r, a)| (Holding::Matter(*r), a)).chain(std::iter::once((Holding::Stamp, &orb.stamp)));
        for (holding, amount) in held {
            let n = units(amount).ok_or_else(|| CensusRefused::OffGrain {
                receipt,
                orb: orb.orb_id,
                holding,
                amount: amount.to_string(),
            })?;
            if !n.is_zero() {
                at.insert((orb.orb_id, holding), n);
            }
        }
    }
    Ok(at)
}

/// Every key's change from `before` to `after`, zero changes omitted.
fn change(before: &Positions, after: &Positions) -> BTreeMap<(u64, Holding), BigInt> {
    let zero = BigInt::zero();
    let mut d = BTreeMap::new();
    for key in before.keys().chain(after.keys()) {
        let delta = after.get(key).unwrap_or(&zero) - before.get(key).unwrap_or(&zero);
        if !delta.is_zero() {
            d.insert(*key, delta);
        }
    }
    d
}

/// The receipt's kind: its event's variant name.
fn kind_of(event: &cfe::dop::ClosureEvent) -> String {
    let debug = format!("{event:?}");
    debug.split(|c: char| !c.is_ascii_alphanumeric()).next().unwrap_or("").to_string()
}

/// A census over one window of one people's chronicle.
#[derive(Debug, Clone)]
pub struct Census {
    /// The people's name: the namespace of every identity below.
    pub people: String,
    pub from: usize,
    pub to: usize,
    /// The chronicle head at `to` — the receipts the census was read from.
    pub head: [u8; 32],
    /// (orb, holding, net change in grain units).
    pub rows: Vec<(u64, Holding, i128)>,
    /// (receipt kind, holding, net emission in grain units).
    pub declared: Vec<(String, Holding, i128)>,
}

fn small(n: &BigInt, orb: u64, holding: Holding) -> Result<i128, CensusRefused> {
    n.to_i128().ok_or(CensusRefused::Overflow { orb, holding })
}

/// Take the census of `chronicle[from..to]`. The replay runs from the first
/// receipt (the state at `from` is the replay of everything before it).
pub fn take(people: &str, chronicle: &Chronicle, from: usize, to: usize) -> Result<Census, CensusRefused> {
    let receipts = chronicle.receipts();
    if from > to || to > receipts.len() || to == 0 {
        return Err(CensusRefused::Window { from, to, len: receipts.len() });
    }
    let mut replay = Replayer::new();
    for r in &receipts[..from] {
        replay.apply(r);
    }
    let start = positions(&replay.state().orbs, from)?;
    let mut before = start.clone();
    let mut emitted: BTreeMap<(String, Holding), BigInt> = BTreeMap::new();
    for (i, r) in receipts.iter().enumerate().take(to).skip(from) {
        replay.apply(r);
        let after = positions(&replay.state().orbs, i)?;
        let mut net: BTreeMap<Holding, BigInt> = BTreeMap::new();
        for ((_, holding), d) in change(&before, &after) {
            *net.entry(holding).or_default() += d;
        }
        let kind = kind_of(r.event());
        for (holding, d) in net.into_iter().filter(|(_, d)| !d.is_zero()) {
            *emitted.entry((kind.clone(), holding)).or_default() += d;
        }
        before = after;
    }
    let rows = change(&start, &before)
        .into_iter()
        .map(|((orb, holding), d)| Ok((orb, holding, small(&d, orb, holding)?)))
        .collect::<Result<Vec<_>, _>>()?;
    let declared = emitted
        .into_iter()
        .filter(|(_, d)| !d.is_zero())
        .map(|((kind, holding), d)| Ok((kind, holding, small(&d, 0, holding)?)))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Census { people: people.to_string(), from, to, head: receipts[to - 1].own().0, rows, declared })
}

impl Census {
    /// The rows and declared emission as the court reads them: one identity
    /// per (people, orb, holding) and per (people, kind, holding), each
    /// value on its holding's lane.
    pub fn nets(&self) -> (Vec<Net>, Vec<Net>) {
        let net = |who: [u8; 32], holding: Holding, v: i128| {
            let mut lanes = [0i128; 4];
            lanes[holding.lane()] = v;
            (who, lanes)
        };
        let rows = self
            .rows
            .iter()
            .map(|(orb, h, v)| net(crate::fold::identity("holding", &format!("{}/{orb}/{}", self.people, h.name())), *h, *v))
            .collect();
        let declared = self
            .declared
            .iter()
            .map(|(kind, h, v)| net(crate::fold::identity("emission", &format!("{}/{kind}/{}", self.people, h.name())), *h, *v))
            .collect();
        (rows, declared)
    }
}

/// A count in grain units, read as the amount it is.
pub fn grains(v: i128) -> String {
    let g = i128::from(RESOURCE_LEDGER_GRAIN);
    let sign = if v < 0 { "-" } else { "" };
    let (whole, part) = (v.unsigned_abs() / g as u128, v.unsigned_abs() % g as u128);
    if part == 0 {
        format!("{sign}{whole}")
    } else {
        format!("{sign}{whole} {part}/{g}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fold::{fold, identity};
    use mycelium::anastome::dialogue::uplink::Discontinuous;

    #[test]
    fn grains_read_signed_whole_and_part() {
        assert_eq!(grains(0), "0");
        assert_eq!(grains(2048), "2");
        assert_eq!(grains(-(11 * 1024 + 383)), "-11 383/1024");
        assert_eq!(grains(-1), "-0 1/1024");
    }

    fn census(rows: Vec<(u64, Holding, i128)>, declared: Vec<(String, Holding, i128)>) -> Census {
        Census { people: "test".into(), from: 0, to: 1, head: [7; 32], rows, declared }
    }

    #[test]
    fn a_declared_harvest_balances_and_value_from_nowhere_does_not() {
        let key = ed25519_dalek::SigningKey::from_bytes(&[3; 32]);
        let iron = Holding::Matter(Resource::Iron);
        let harvested = census(vec![(1, iron, 2048), (1, Holding::Stamp, 5)], vec![("YieldCollected".into(), iron, 2048), ("StampApertureReceived".into(), Holding::Stamp, 5)]);
        let (rows, declared) = harvested.nets();
        assert_eq!(fold(&key, harvested.head, rows, declared).fold.continuity(), Ok(()));

        let nowhere = census(vec![(1, iron, 2048)], vec![]);
        let (rows, declared) = nowhere.nets();
        assert_eq!(fold(&key, nowhere.head, rows, declared).fold.continuity(), Err(Discontinuous::Unbalanced { lane: lane::MATTER }));
    }

    #[test]
    fn matter_and_money_never_share_a_lane_and_peoples_never_share_an_identity() {
        let a = census(vec![(1, Holding::Matter(Resource::Iron), 1), (1, Holding::Stamp, 1)], vec![]);
        let (rows, _) = a.nets();
        assert_eq!(rows[0].1, [1, 0, 0, 0]);
        assert_eq!(rows[1].1, [0, 1, 0, 0]);
        let mut b = a.clone();
        b.people = "other".into();
        assert_ne!(a.nets().0[0].0, b.nets().0[0].0);
        assert_ne!(identity("holding", "a/1/Iron"), identity("emission", "a/1/Iron"));
    }
}
