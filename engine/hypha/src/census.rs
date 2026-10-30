//! The census fold — a people's chronicle, netted for the court.
//!
//! The chronicle is the record: an append-only, hash-chained run of
//! receipts, and the colony's state is its replay. A census over the window
//! `[from, to)` steps the replay one receipt at a time and reads every
//! colony orb's holdings before and after each receipt:
//!
//!   * Rows are each holding's net change over the window — one identity per
//!     (people, orb, holding), matter on [`lane::MATTER`], STAMP on
//!     [`lane::MONEY`], in units of the resource ledger's 1/1024 grain.
//!   * Declared is what the receipts themselves made or unmade: whatever a
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
use cfe::vrp::{Chronicle, Receipt};
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
    /// A people's lexicon: the words it has minted (information, lane H).
    Words,
}

impl Holding {
    fn lane(self) -> usize {
        match self {
            Holding::Matter(_) => lane::MATTER,
            Holding::Stamp => lane::MONEY,
            Holding::Words => lane::INFORMATION,
        }
    }

    fn name(self) -> String {
        match self {
            Holding::Matter(r) => format!("{r:?}"),
            Holding::Stamp => "STAMP".to_string(),
            Holding::Words => "words".to_string(),
        }
    }
}

/// Why a census could not be taken.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CensusRefused {
    /// The window does not lie inside the chronicle.
    Window { from: usize, to: usize, len: usize },
    /// A count passes the court's i128 carrier: refused, never wrapped.
    Overflow { orb: u64, holding: Holding },
    /// The per-receipt entries do not sum to the window's totals, so the
    /// line could not re-derive the fold. Never sent.
    EntriesDisagree,
}

/// One holding's count in grain units — the amount READ AT the resource
/// ledger's 1/1024 grain (floored), and whether it sat off the grain. The
/// grain is the ledger's own law (#138): sub-grain dust is not ledger value,
/// so a fold never carries it; but an off-grain holding means some fill site
/// broke that law, and the census reports every one (`Stepper::off_grain`)
/// instead of hiding it. Floored positions telescope exactly, so per-receipt
/// changes still sum to the window's change and continuity holds.
fn units(amount: &Ratio) -> (BigInt, bool) {
    let scaled = amount * Ratio::from_integer(BigInt::from(RESOURCE_LEDGER_GRAIN));
    (scaled.floor().to_integer(), !scaled.is_integer())
}

type Positions = BTreeMap<(u64, Holding), BigInt>;

/// An off-grain holding the census read: receipt index, orb, holding, amount.
pub type OffGrain = (usize, u64, Holding, String);

fn positions(orbs: &[ColonyOrb], receipt: usize, off: &mut Vec<OffGrain>, held_off: &mut std::collections::BTreeSet<(u64, Holding)>) -> Positions {
    let mut at = Positions::new();
    let mut now_off = std::collections::BTreeSet::new();
    for orb in orbs {
        let held = orb.banked.iter().map(|(r, a)| (Holding::Matter(*r), a)).chain(std::iter::once((Holding::Stamp, &orb.stamp)));
        for (holding, amount) in held {
            let (n, off_grain) = units(amount);
            if off_grain {
                // Reported once, where the breach begins; a holding that stays
                // off the grain is not re-reported on every receipt.
                if !held_off.contains(&(orb.orb_id, holding)) {
                    off.push((receipt, orb.orb_id, holding, amount.to_string()));
                }
                now_off.insert((orb.orb_id, holding));
            }
            if !n.is_zero() {
                at.insert((orb.orb_id, holding), n);
            }
        }
    }
    *held_off = now_off;
    at
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

/// The word A receipt issues, if it mints one — its label on the information
/// lane. A solver word is named by its content-addressed orbit (the same word
/// minted by two lines is one word); a civic vocabulary word by its axis and
/// spelling. Domain-local names: a word that may ever cross domains is named by
/// the hash of its Mycelium address instead (coordinator ruling 2026-10-07),
/// and none crosses yet.
fn word_issued(event: &cfe::dop::ClosureEvent) -> Option<String> {
    use cfe::dop::ClosureEvent as E;
    match event {
        E::SolverWordMinted { orbit, .. } => Some(format!("SolverWordMinted:{}", orbit.iter().map(|b| format!("{b:02x}")).collect::<String>())),
        E::VocabularyMinted { axis, word, .. } => Some(format!("VocabularyMinted:{axis}:{word}")),
        _ => None,
    }
}

/// One receipt's net change — a line entry (78.123's tier line: an entry is
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

/// The incremental census — a people's replay held open across reads, so a
/// follower that tails a live chronicle steps each new receipt once and
/// never replays from genesis again. `skip` catches up over receipts
/// already folded (no step emitted); `step` emits the receipt's own change.
pub struct Stepper {
    replay: Replayer,
    before: Positions,
    applied: usize,
    /// Every off-grain breach read so far, newest last — where a holding left
    /// the ledger's grain (a fill site broke #138; reported, never folded).
    pub off_grain: Vec<OffGrain>,
    held_off: std::collections::BTreeSet<(u64, Holding)>,
}

impl Stepper {
    /// The empty book, before the first receipt.
    pub fn new() -> Stepper {
        Stepper { replay: Replayer::new(), before: Positions::new(), applied: 0, off_grain: Vec::new(), held_off: std::collections::BTreeSet::new() }
    }

    /// A stepper resumed from carried state (reconstruct v): the committed
    /// state after `applied` receipts, read from a position record — never
    /// rebuilt by replaying the receipts before it.
    pub fn from_state(state: cfe::world::state::Committed, applied: usize) -> Stepper {
        let mut stepper = Stepper { replay: Replayer::from_state(state), ..Stepper::new() };
        stepper.applied = applied;
        stepper.before = positions(&stepper.replay.state().orbs, applied.saturating_sub(1), &mut stepper.off_grain, &mut stepper.held_off);
        stepper
    }

    /// The committed state after the receipts applied.
    pub fn state(&self) -> &cfe::world::state::Committed {
        self.replay.state()
    }

    /// How many receipts this stepper has applied.
    pub fn applied(&self) -> usize {
        self.applied
    }

    /// Apply receipts already folded, emitting nothing; the holdings are
    /// read once at the end, so a long catch-up pays one census, not one
    /// per receipt.
    pub fn skip(&mut self, receipts: &[Receipt]) -> Result<(), CensusRefused> {
        for r in receipts {
            self.replay.apply(r);
        }
        self.applied += receipts.len();
        self.before = positions(&self.replay.state().orbs, self.applied.saturating_sub(1), &mut self.off_grain, &mut self.held_off);
        Ok(())
    }

    /// Apply one receipt and return its own net change, or `None` when it
    /// moved no holding (no line entry is written for it).
    pub fn step(&mut self, r: &Receipt) -> Result<Option<Step>, CensusRefused> {
        let i = self.applied;
        self.replay.apply(r);
        self.applied += 1;
        let after = positions(&self.replay.state().orbs, i, &mut self.off_grain, &mut self.held_off);
        let moved = change(&self.before, &after);
        self.before = after;
        let word = word_issued(r.event());
        if moved.is_empty() && word.is_none() {
            return Ok(None);
        }
        let mut net: BTreeMap<Holding, BigInt> = BTreeMap::new();
        for ((_, holding), d) in &moved {
            *net.entry(*holding).or_default() += d;
        }
        let kind = kind_of(r.event());
        let declared = net
            .into_iter()
            .filter(|(_, d)| !d.is_zero())
            .map(|(holding, d)| Ok((kind.clone(), holding, small(&d, 0, holding)?)))
            .collect::<Result<Vec<_>, _>>()?;
        let mut rows = moved
            .iter()
            .map(|((orb, holding), d)| Ok((*orb, *holding, small(d, *orb, *holding)?)))
            .collect::<Result<Vec<_>, _>>()?;
        let mut declared = declared;
        if let Some(word) = word {
            // Issued: the word is a declared source of one unit of information;
            // delivered: the people's lexicon (orb 0, the line itself) takes it.
            declared.push((word, Holding::Words, 1));
            rows.push((0, Holding::Words, 1));
        }
        Ok(Some(Step { receipt: i, cite: r.own().0, rows, declared }))
    }
}

impl Default for Stepper {
    fn default() -> Self {
        Self::new()
    }
}

/// Take the census of `chronicle[from..to]`. The replay runs from the first
/// receipt (the state at `from` is the replay of everything before it). The
/// window's totals are read twice — as the start-to-end change of holdings
/// and as the sum of its steps — and a census whose two readings disagree is
/// refused, so a fold is never signed over entries that cannot re-derive it.
pub fn take(people: &str, chronicle: &Chronicle, from: usize, to: usize) -> Result<Census, CensusRefused> {
    let receipts = chronicle.receipts();
    if from > to || to > receipts.len() || to == 0 {
        return Err(CensusRefused::Window { from, to, len: receipts.len() });
    }
    let mut stepper = Stepper::new();
    stepper.skip(&receipts[..from])?;
    let start = stepper.before.clone();
    let mut steps = Vec::new();
    for r in &receipts[from..to] {
        if let Some(step) = stepper.step(r)? {
            steps.push(step);
        }
    }
    let census = Census::from_steps(people, from, to, receipts[to - 1].own().0, steps);
    let held: BTreeMap<(u64, Holding), i128> = change(&start, &stepper.before)
        .into_iter()
        .map(|((orb, holding), d)| Ok(((orb, holding), small(&d, orb, holding)?)))
        .collect::<Result<_, CensusRefused>>()?;
    // Words are issued by receipts, not held in a colony's bank: the
    // start-to-end check reads the banked holdings only.
    let summed: BTreeMap<(u64, Holding), i128> =
        census.rows.iter().filter(|(_, h, _)| *h != Holding::Words).map(|(o, h, v)| ((*o, *h), *v)).collect();
    if held != summed {
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

/// The bound on A fold's SPAN (coordinator ruling 2026-10-07): an
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
                let c = Census::from_steps(&people, start, end, end_head, steps);
                start = end;
                c
            })
            .collect()
    }

    /// A census whose totals are the sums of `steps`, key by key, zero nets
    /// dropped — the window `[from, to)` ending at chronicle head `head`.
    pub fn from_steps(people: &str, from: usize, to: usize, head: [u8; 32], steps: Vec<Step>) -> Census {
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
        Census {
            people: people.to_string(),
            from,
            to,
            head,
            rows: rows.into_iter().filter(|(_, v)| *v != 0).map(|((o, h), v)| (o, h, v)).collect(),
            declared: declared.into_iter().filter(|(_, v)| *v != 0).map(|((k, h), v)| (k, h, v)).collect(),
            steps,
        }
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

    #[test]
    fn a_holding_is_read_at_the_grain_and_an_off_grain_one_is_flagged() {
        let on = cfe::ratio::ratio(2547, 512); // 5094/1024
        assert_eq!(units(&on), (BigInt::from(5094), false));
        let off = cfe::ratio::ratio(161293, 8192); // 20161.625/1024
        assert_eq!(units(&off), (BigInt::from(20161), true), "floored to the grain, and flagged");
    }

    #[test]
    fn a_minted_word_is_an_information_source_delivered_to_the_lexicon() {
        let word = "SolverWordMinted:ab".to_string();
        let c = census(vec![(0, Holding::Words, 1)], vec![(word, Holding::Words, 1)]);
        let (rows, declared) = c.nets();
        assert_eq!(rows[0].1, [0, 0, 0, 1], "the lexicon row rides H");
        assert_eq!(declared[0].1, [0, 0, 0, 1], "the issued word is an H source");
        let key = ed25519_dalek::SigningKey::from_bytes(&[3; 32]);
        assert_eq!(fold(&key, c.head, rows, declared).fold.continuity(), Ok(()));
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
