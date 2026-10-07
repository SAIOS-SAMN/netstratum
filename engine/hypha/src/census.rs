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
//! changes). Each receipt's own change is one entry on the `colonies` tier's
//! line ([`crate::line`]), so the court's seal commits to the batches the
//! fold nets, and a challenge re-derives the fold from them.

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
    /// The per-receipt entries do not sum to the window's totals, so the
    /// line could not re-derive the fold. Never sent.
    EntriesDisagree,
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

/// ONE RECEIPT'S NET CHANGE — a line entry (78.123's tier line: an entry is
/// the underlying batch a fold nets, never the fold itself). Receipts that
/// move no holding write no entry.
#[derive(Debug, Clone)]
pub struct Step {
    /// The receipt's index in the chronicle.
    pub receipt: usize,
    /// The receipt's own hash — the line record's local cite.
    pub cite: [u8; 32],
    /// (orb, holding, change in grain units).
    pub rows: Vec<(u64, Holding, i128)>,
    /// (receipt kind, holding, net emission in grain units).
    pub declared: Vec<(String, Holding, i128)>,
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
    /// The window's totals: (orb, holding, net change in grain units).
    pub rows: Vec<(u64, Holding, i128)>,
    /// The window's totals: (receipt kind, holding, net emission).
    pub declared: Vec<(String, Holding, i128)>,
    /// Every receipt that moved a holding, in chronicle order.
    pub steps: Vec<Step>,
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
    let mut steps = Vec::new();
    for (i, r) in receipts.iter().enumerate().take(to).skip(from) {
        replay.apply(r);
        let after = positions(&replay.state().orbs, i)?;
        let moved = change(&before, &after);
        if moved.is_empty() {
            continue;
        }
        let mut net: BTreeMap<Holding, BigInt> = BTreeMap::new();
        for ((_, holding), d) in &moved {
            *net.entry(*holding).or_default() += d;
        }
        let kind = kind_of(r.event());
        let mut declared = Vec::new();
        for (holding, d) in net.into_iter().filter(|(_, d)| !d.is_zero()) {
            declared.push((kind.clone(), holding, small(&d, 0, holding)?));
            *emitted.entry((kind.clone(), holding)).or_default() += d;
        }
        let rows = moved
            .iter()
            .map(|((orb, holding), d)| Ok((*orb, *holding, small(d, *orb, *holding)?)))
            .collect::<Result<Vec<_>, _>>()?;
        steps.push(Step { receipt: i, cite: r.own().0, rows, declared });
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
    let census = Census { people: people.to_string(), from, to, head: receipts[to - 1].own().0, rows, declared, steps };
    if !census.entries_sum_to_totals() {
        return Err(CensusRefused::EntriesDisagree);
    }
    Ok(census)
}

/// Rows and declared as the court reads them: one identity per (people,
/// orb, holding) and per (people, kind, holding), each value on its
/// holding's lane, each list in its identities' ascending order.
fn nets(people: &str, rows: &[(u64, Holding, i128)], declared: &[(String, Holding, i128)]) -> (Vec<Net>, Vec<Net>) {
    let net = |who: [u8; 32], holding: Holding, v: i128| {
        let mut lanes = [0i128; 4];
        lanes[holding.lane()] = v;
        (who, lanes)
    };
    let mut r: Vec<Net> = rows
        .iter()
        .map(|(orb, h, v)| net(crate::fold::identity("holding", &format!("{people}/{orb}/{}", h.name())), *h, *v))
        .collect();
    let mut d: Vec<Net> = declared
        .iter()
        .map(|(kind, h, v)| net(crate::fold::identity("emission", &format!("{people}/{kind}/{}", h.name())), *h, *v))
        .collect();
    r.sort_by_key(|(who, _)| *who);
    d.sort_by_key(|(who, _)| *who);
    (r, d)
}

/// THE BOUND ON A FOLD'S SPAN (coordinator ruling 2026-10-07): an
/// availability demand makes the tier publish every entry a fold nets, and
/// the court replays them in one act, so no fold nets more than this many
/// (~1.1 MB at ~270 B an entry). History is backfilled as successive folds.
pub const MAX_SPAN: usize = 4096;

impl Census {
    /// This window cut into successive censuses of at most `max` steps each,
    /// in chronicle order — each its own fold. A chunk's totals are its own
    /// steps' sums; it ends after its last step's receipt (the final chunk
    /// at the window's end, so receipts that moved nothing are passed over
    /// with it). One chunk when the window already fits.
    pub fn chunks(self, max: usize) -> Vec<Census> {
        if self.steps.len() <= max.max(1) {
            return vec![self];
        }
        let Census { people, from, to, head, steps, .. } = self;
        let parts: Vec<Vec<Step>> = steps.chunks(max.max(1)).map(<[Step]>::to_vec).collect();
        let last = parts.len() - 1;
        let mut start = from;
        parts
            .into_iter()
            .enumerate()
            .map(|(n, steps)| {
                let (end, end_head) = match (n == last, steps.last()) {
                    (false, Some(s)) => (s.receipt + 1, s.cite),
                    _ => (to, head),
                };
                let mut rows: BTreeMap<(u64, Holding), i128> = BTreeMap::new();
                let mut declared: BTreeMap<(String, Holding), i128> = BTreeMap::new();
                for s in &steps {
                    for (orb, h, v) in &s.rows {
                        *rows.entry((*orb, *h)).or_default() += v;
                    }
                    for (k, h, v) in &s.declared {
                        *declared.entry((k.clone(), *h)).or_default() += v;
                    }
                }
                let c = Census {
                    people: people.clone(),
                    from: start,
                    to: end,
                    head: end_head,
                    rows: rows.into_iter().filter(|(_, v)| *v != 0).map(|((o, h), v)| (o, h, v)).collect(),
                    declared: declared.into_iter().filter(|(_, v)| *v != 0).map(|((k, h), v)| (k, h, v)).collect(),
                    steps,
                };
                start = end;
                c
            })
            .collect()
    }

    /// What a challenge checks: the steps, summed key by key, are exactly the
    /// window's rows and declared.
    pub fn entries_sum_to_totals(&self) -> bool {
        let mut rows: BTreeMap<(u64, Holding), i128> = BTreeMap::new();
        let mut declared: BTreeMap<(String, Holding), i128> = BTreeMap::new();
        for s in &self.steps {
            for (orb, h, v) in &s.rows {
                *rows.entry((*orb, *h)).or_default() += v;
            }
            for (k, h, v) in &s.declared {
                *declared.entry((k.clone(), *h)).or_default() += v;
            }
        }
        rows.retain(|_, v| *v != 0);
        declared.retain(|_, v| *v != 0);
        let want_rows: BTreeMap<_, _> = self.rows.iter().map(|(o, h, v)| ((*o, *h), *v)).collect();
        let want_declared: BTreeMap<_, _> = self.declared.iter().map(|(k, h, v)| ((k.clone(), *h), *v)).collect();
        rows == want_rows && declared == want_declared
    }

    /// The window's totals as the court reads them — the fold's rows.
    pub fn nets(&self) -> (Vec<Net>, Vec<Net>) {
        nets(&self.people, &self.rows, &self.declared)
    }

    /// The line entries this window writes, one per receipt that moved a
    /// holding: `put_nets(rows) ‖ put_nets(declared)` of that receipt alone,
    /// with the receipt's hash as the local cite. Their sum is [`Census::nets`].
    pub fn entries(&self) -> Vec<(Vec<u8>, [u8; 32])> {
        self.steps
            .iter()
            .map(|s| {
                let (rows, declared) = nets(&self.people, &s.rows, &s.declared);
                (crate::line::entry_bytes(rows, declared), s.cite)
            })
            .collect()
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

    fn step(receipt: usize, orb: u64, v: i128) -> Step {
        let iron = Holding::Matter(Resource::Iron);
        Step { receipt, cite: [receipt as u8; 32], rows: vec![(orb, iron, v)], declared: vec![("YieldCollected".into(), iron, v)] }
    }

    #[test]
    fn a_long_window_cuts_into_bounded_folds_that_sum_to_it_and_tile_it() {
        let iron = Holding::Matter(Resource::Iron);
        let steps: Vec<Step> = (0..10).map(|i| step(i * 2, 1, 3)).collect();
        let whole = Census { people: "p".into(), from: 0, to: 25, head: [99; 32], rows: vec![(1, iron, 30)], declared: vec![("YieldCollected".into(), iron, 30)], steps };
        assert!(whole.entries_sum_to_totals());
        let parts = whole.chunks(4);
        assert_eq!(parts.iter().map(|c| c.steps.len()).collect::<Vec<_>>(), vec![4, 4, 2]);
        assert_eq!(parts.iter().map(|c| (c.from, c.to)).collect::<Vec<_>>(), vec![(0, 7), (7, 15), (15, 25)]);
        assert_eq!(parts[0].head, [6; 32]);
        assert_eq!(parts[2].head, [99; 32]);
        assert!(parts.iter().all(Census::entries_sum_to_totals));
        assert_eq!(parts.iter().map(|c| c.rows[0].2).sum::<i128>(), 30);
    }

    #[test]
    fn grains_read_signed_whole_and_part() {
        assert_eq!(grains(0), "0");
        assert_eq!(grains(2048), "2");
        assert_eq!(grains(-(11 * 1024 + 383)), "-11 383/1024");
        assert_eq!(grains(-1), "-0 1/1024");
    }

    fn census(rows: Vec<(u64, Holding, i128)>, declared: Vec<(String, Holding, i128)>) -> Census {
        Census { people: "test".into(), from: 0, to: 1, head: [7; 32], rows, declared, steps: Vec::new() }
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
        let lanes_of = |who: [u8; 32]| rows.iter().find(|(w, _)| *w == who).map(|(_, l)| *l);
        assert_eq!(lanes_of(identity("holding", "test/1/Iron")), Some([1, 0, 0, 0]));
        assert_eq!(lanes_of(identity("holding", "test/1/STAMP")), Some([0, 1, 0, 0]));
        assert!(rows.windows(2).all(|w| w[0].0 < w[1].0), "rows in their identities' ascending order");
        let mut b = a.clone();
        b.people = "other".into();
        assert!(b.nets().0.iter().all(|(who, _)| lanes_of(*who).is_none()));
        assert_ne!(identity("holding", "a/1/Iron"), identity("emission", "a/1/Iron"));
    }
}
