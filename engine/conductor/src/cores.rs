//! **THE CORES** (docs/58 §6 rulings 5 and 9; the cores design approved by
//! the operator 2026-10-08) — slice (1): every sink reaches its galaxy's core.
//!
//! A charted galaxy's core is the throat at its centre, an account on the
//! universe ledger. What a world's deeds UNMAKE no longer vanishes: the judge
//! (`Polity::chain_beat`) already holds every deed's state before and after,
//! so each deed's sink is read there, exactly, from the world's own holdings,
//! and kept PENDING beside the world until the year's close deposits it at
//! the core (`CoreDeposited`, tag 193). Netstratum's core REFLECTS (operator
//! ruling, 2026-10-08): the re-emission is slice (2).
//!
//! What a sink is follows the universe's measured matter budget (docs/58 §7):
//!
//!   * the FEE AND LOSS kinds ([`LOSS_KINDS`]) — whatever the world's holdings
//!     fell by in that deed;
//!   * a RAZE — the half the world does not get back, which by the raze law
//!     equals the half it does (the refund the deed credits);
//!   * TRANSIT MELT — an aperture's two legs, paired by deal across the
//!     universe: what left the sender less what reached the receiver.
//!
//! A build is NOT a sink: its cost is embodied in the zone (the raze is where
//! half of it is lost). Holdings are banked resources and STAMP, exact.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::{ratio, Ratio};
use cfe::world::state::Committed;
use std::collections::BTreeMap;
use std::path::Path;

/// A holding: a banked resource, or STAMP (`None`).
pub type Holding = Option<Resource>;

/// What a world holds in all: every orb's banked resources and STAMP.
pub fn holdings(state: &Committed) -> BTreeMap<Holding, Ratio> {
    let mut held: BTreeMap<Holding, Ratio> = BTreeMap::new();
    for orb in &state.orbs {
        for (resource, amount) in &orb.banked {
            *held.entry(Some(*resource)).or_insert_with(|| ratio(0, 1)) += amount.clone();
        }
        *held.entry(None).or_insert_with(|| ratio(0, 1)) += orb.stamp.clone();
    }
    held
}

/// Every holding's change from `before` to `after`, zero changes omitted.
pub fn change(before: &BTreeMap<Holding, Ratio>, after: &BTreeMap<Holding, Ratio>) -> BTreeMap<Holding, Ratio> {
    let zero = ratio(0, 1);
    let mut d = BTreeMap::new();
    for key in before.keys().chain(after.keys()) {
        let delta = after.get(key).unwrap_or(&zero).clone() - before.get(key).unwrap_or(&zero).clone();
        if delta != zero {
            d.insert(*key, delta);
        }
    }
    d
}

/// The measured fee and loss kinds (docs/58 §7): what the world's holdings
/// fall by in one of these deeds is unmade.
pub const LOSS_KINDS: &[&str] = &[
    "UpkeepSettled",
    "FamineStruck",
    "TitheReceipted",
    "ScrapSalvaged",
    "ScienceSeated",
    "ChemCataloged",
    "EstateSurveyed",
    "ParcelConveyed",
];

/// How a deed's change reaches the core.
pub enum Sunk {
    /// Unmade outright: these amounts go to the core.
    Lost(Vec<(Holding, Ratio)>),
    /// One leg of an aperture deal (signed: a sent leg negative), held until
    /// its other leg pairs with it.
    Leg { deal: u64, holding: Holding, amount: Ratio },
    /// Nothing reaches the core.
    None,
}

fn kind(event: &ClosureEvent) -> String {
    let debug = format!("{event:?}");
    debug.split(|c: char| !c.is_ascii_alphanumeric()).next().unwrap_or("").to_string()
}

/// Does this deed's kind ever reach the core? (So the judge reads holdings
/// only for the deeds that can.)
pub fn reaches(event: &ClosureEvent) -> bool {
    use ClosureEvent as E;
    matches!(
        event,
        E::ApertureSent { .. } | E::ApertureReceived { .. } | E::StampApertureSent { .. } | E::StampApertureReceived { .. } | E::ZoneRazed { .. }
    ) || LOSS_KINDS.contains(&kind(event).as_str())
}

/// What one judged deed sinks, from the world's holdings `before` and `after`.
pub fn sunk(event: &ClosureEvent, before: &BTreeMap<Holding, Ratio>, after: &BTreeMap<Holding, Ratio>) -> Sunk {
    use ClosureEvent as E;
    let zero = ratio(0, 1);
    let d = change(before, after);
    match event {
        E::ApertureSent { deal, resource, .. } | E::ApertureReceived { deal, resource, .. } => {
            let amount = d.get(&Some(*resource)).cloned().unwrap_or(zero);
            Sunk::Leg { deal: *deal, holding: Some(*resource), amount }
        }
        E::StampApertureSent { deal, .. } | E::StampApertureReceived { deal, .. } => {
            let amount = d.get(&None).cloned().unwrap_or(zero);
            Sunk::Leg { deal: *deal, holding: None, amount }
        }
        // The raze returns half; the half it keeps is equal to it.
        E::ZoneRazed { .. } => Sunk::Lost(d.into_iter().filter(|(_, v)| *v > zero).collect()),
        _ if LOSS_KINDS.contains(&kind(event).as_str()) => {
            Sunk::Lost(d.into_iter().filter(|(_, v)| *v < zero).map(|(h, v)| (h, -v)).collect())
        }
        _ => Sunk::None,
    }
}

/// A world's sinks not yet deposited at its core: amounts unmade, and the
/// aperture legs still waiting for their pair.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Pending {
    pub lost: BTreeMap<Holding, Ratio>,
    pub legs: Vec<(u64, Holding, Ratio)>,
    /// The absolute receipt count these sinks are read through: a judge that
    /// restarts behind it (its watermark written after this) skips the deeds
    /// already counted, so none is counted twice.
    pub through: usize,
}

impl Pending {
    pub fn take(&mut self, s: Sunk) -> bool {
        match s {
            Sunk::Lost(amounts) => {
                let mut moved = false;
                for (h, v) in amounts {
                    *self.lost.entry(h).or_insert_with(|| ratio(0, 1)) += v;
                    moved = true;
                }
                moved
            }
            // A leg of nothing carries no melt and can never pair (pairing
            // needs a sent leg, below zero): it is not kept. Kept, they grew
            // without bound, 9.7M of 9.7M+2,968 legs (optimization sweep,
            // 2026-10-13), rewritten every beat.
            Sunk::Leg { amount, .. } if amount == ratio(0, 1) => false,
            Sunk::Leg { deal, holding, amount } => {
                self.legs.push((deal, holding, amount));
                true
            }
            Sunk::None => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lost.is_empty() && self.legs.is_empty()
    }

    /// Clear what was deposited (the amounts unmade; the unpaired legs stay).
    pub fn deposited(&mut self) {
        self.lost.clear();
    }

    /// Plain text beside the world (`core.pending`), durable across a restart:
    /// `W <through>`, then `L <code> <amount>` per amount unmade and
    /// `A <deal> <code> <amount>` per leg (code 0 = STAMP, else the resource's
    /// wire code).
    pub fn to_text(&self) -> String {
        let code = |h: &Holding| h.map_or(0, |r| r as u8);
        let mut out = format!("W {}\n", self.through);
        for (h, v) in &self.lost {
            out.push_str(&format!("L {} {}\n", code(h), cfe::ratio::ratio_str(v)));
        }
        for (deal, h, v) in &self.legs {
            out.push_str(&format!("A {deal} {} {}\n", code(h), cfe::ratio::ratio_str(v)));
        }
        out
    }

    pub fn from_text(text: &str) -> Option<Pending> {
        let holding = |s: &str| -> Option<Holding> {
            match s.parse::<u8>().ok()? {
                0 => Some(None),
                c => Some(Some(Resource::from_wire(c)?)),
            }
        };
        let mut p = Pending::default();
        for line in text.lines().filter(|l| !l.is_empty()) {
            let f: Vec<&str> = line.split(' ').collect();
            match f.as_slice() {
                ["W", n] => p.through = n.parse().ok()?,
                ["L", c, v] => {
                    p.lost.insert(holding(c)?, ledger::parse_ratio(v)?);
                }
                ["A", deal, c, v] => {
                    let amount = ledger::parse_ratio(v)?;
                    // Zero legs written before the fix are dropped on read.
                    if amount != ratio(0, 1) {
                        p.legs.push((deal.parse().ok()?, holding(c)?, amount));
                    }
                }
                _ => return None,
            }
        }
        Some(p)
    }

    pub fn read(dir: &Path) -> Pending {
        std::fs::read_to_string(dir.join("core.pending")).ok().and_then(|t| Pending::from_text(&t)).unwrap_or_default()
    }

    pub fn write(&self, dir: &Path) -> std::io::Result<()> {
        let tmp = dir.join("core.pending.tmp");
        std::fs::write(&tmp, self.to_text())?;
        std::fs::rename(&tmp, dir.join("core.pending"))
    }
}

/// THE PAIRING: over every world's held legs, each deal whose sent legs and
/// received legs are both present for a holding melts `-(sum)` — deposited
/// at the SENDER's core — and its legs leave the pending sets. Returns
/// `(sender index, holding, melt)`.
pub fn pair_legs(pending: &mut [&mut Pending]) -> Vec<(usize, Holding, Ratio)> {
    let zero = ratio(0, 1);
    // deal, holding -> (sum, sender, has_sent, has_received)
    let mut deals: BTreeMap<(u64, Holding), (Ratio, Option<usize>, bool, bool)> = BTreeMap::new();
    for (i, p) in pending.iter().enumerate() {
        for (deal, h, v) in &p.legs {
            let e = deals.entry((*deal, *h)).or_insert_with(|| (zero.clone(), None, false, false));
            e.0 += v.clone();
            if *v < zero {
                e.1 = Some(i);
                e.2 = true;
            } else {
                e.3 = true;
            }
        }
    }
    let mut melts = Vec::new();
    // The paired deals, then ONE pass over each line's legs (was a pass over
    // every line's legs per paired deal: a year's ~10^5 deals times ~10^5
    // legs at every year close, docs/64).
    let mut paired: std::collections::BTreeSet<(u64, Holding)> = std::collections::BTreeSet::new();
    for ((deal, h), (sum, sender, sent, received)) in deals {
        if !(sent && received) {
            continue;
        }
        paired.insert((deal, h));
        let melt = -sum;
        if melt > zero {
            melts.push((sender.expect("a sent leg names its sender"), h, melt));
        }
    }
    if !paired.is_empty() {
        for p in pending.iter_mut() {
            p.legs.retain(|(d, held, _)| !paired.contains(&(*d, *held)));
        }
    }
    melts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn held(pairs: &[(Holding, i64)]) -> BTreeMap<Holding, Ratio> {
        pairs.iter().map(|(h, v)| (*h, ratio(*v, 1))).collect()
    }

    #[test]
    fn a_loss_sinks_what_fell_a_raze_sinks_the_half_it_keeps_a_build_sinks_nothing() {
        let iron = Some(Resource::Iron);
        let upkeep = ClosureEvent::UpkeepSettled { orb_id: 1, resource: Resource::Iron, amount: ratio(3, 1) };
        assert!(reaches(&upkeep));
        let Sunk::Lost(lost) = sunk(&upkeep, &held(&[(iron, 10)]), &held(&[(iron, 7)])) else { panic!("a loss") };
        assert_eq!(lost, vec![(iron, ratio(3, 1))]);
        // a loss kind that gained nothing sinks nothing on the gain
        let Sunk::Lost(lost) = sunk(&upkeep, &held(&[(iron, 7)]), &held(&[(iron, 9)])) else { panic!("a loss") };
        assert!(lost.is_empty());

        let raze = ClosureEvent::ZoneRazed { orb_id: 1, tile: 4 };
        let Sunk::Lost(lost) = sunk(&raze, &held(&[(iron, 0)]), &held(&[(iron, 5)])) else { panic!("a raze") };
        assert_eq!(lost, vec![(iron, ratio(5, 1))], "the refund's twin");

        let build = ClosureEvent::ZoneBuilt { orb_id: 1, tile: 4, kind: cfe::dop::ZoneKind::Extractor, base: None };
        assert!(!reaches(&build), "a build's cost is embodied, not unmade");
        assert!(matches!(sunk(&build, &held(&[(iron, 10)]), &held(&[(iron, 0)])), Sunk::None));
    }

    #[test]
    fn aperture_legs_pair_by_deal_and_the_melt_reaches_the_senders_core() {
        let iron = Some(Resource::Iron);
        let (mut a, mut b, mut c) = (Pending::default(), Pending::default(), Pending::default());
        a.legs.push((7, iron, ratio(-10, 1))); // sent
        b.legs.push((7, iron, ratio(8, 1))); // received: 2 melted
        c.legs.push((9, None, ratio(-4, 1))); // its receiver not yet judged
        let melts = pair_legs(&mut [&mut a, &mut b, &mut c]);
        assert_eq!(melts, vec![(0, iron, ratio(2, 1))]);
        assert!(a.legs.is_empty() && b.legs.is_empty());
        assert_eq!(c.legs.len(), 1, "an unpaired leg waits");
        // a lossless deal melts nothing and still clears
        let (mut d, mut e) = (Pending::default(), Pending::default());
        d.legs.push((3, None, ratio(-5, 1)));
        e.legs.push((3, None, ratio(5, 1)));
        assert!(pair_legs(&mut [&mut d, &mut e]).is_empty());
        assert!(d.legs.is_empty() && e.legs.is_empty());
    }

    #[test]
    fn the_pending_record_reads_back_whole() {
        let mut p = Pending { through: 41, ..Pending::default() };
        p.take(Sunk::Lost(vec![(Some(Resource::Biomass), ratio(7, 3)), (None, ratio(1, 2))]));
        p.take(Sunk::Leg { deal: 900_017, holding: Some(Resource::Alloy), amount: ratio(-9, 4) });
        assert_eq!(Pending::from_text(&p.to_text()), Some(p.clone()));
        p.deposited();
        assert!(p.lost.is_empty() && p.legs.len() == 1, "a deposit clears the amounts, never the waiting legs");
    }

    #[test]
    fn the_bounce_arrives_lawfully_and_a_forged_one_is_refused() {
        use cfe::world::{Replayer, World};
        let mut world = World::new();
        world.found("meridian").expect("founds");
        for resource in [Some(Resource::Iron), None] {
            let prior = world.committed();
            world.receive_from_core("the-founding", resource, cfe::dop::positive(7, 4)).expect("arrives");
            let receipt = world.chronicle().receipts().last().unwrap().clone();
            // the replay of the receipt is the live credit
            let mut stepper = Replayer::from_state(prior.clone());
            stepper.apply(&receipt);
            assert_eq!(stepper.state(), &world.committed());
            assert_eq!(change(&holdings(&prior), &holdings(&world.committed())), [(resource, ratio(7, 4))].into_iter().collect());
            // the kernel admits it as a declared source
            let step = kernel::Step { event: receipt.event().clone(), actor: cfe::world::SOVEREIGN, stamp: receipt.stamp() };
            assert_eq!(kernel::admits(&prior, stepper.state(), &step), Ok(()));
            // a credit larger than it declares is refused
            let mut forged = stepper.state().clone();
            match resource {
                Some(r) => forged.orbs[0].bank(r, ratio(1, 1)),
                None => forged.orbs[0].bank_stamp(ratio(1, 1)),
            }
            assert!(kernel::admits(&prior, &forged, &step).is_err());
        }
    }

    #[test]
    fn a_core_deposit_crosses_the_wire_whole() {
        let events = [None, Some(Resource::Crystal)].into_iter().flat_map(|resource| {
            [
                ClosureEvent::CoreDeposited { core: "the-founding".into(), world: "forge-reach".into(), resource, amount: ratio(3535, 1024) },
                ClosureEvent::CoreEmitted { core: "the-founding".into(), world: "forge-reach".into(), resource, amount: ratio(3, 1024) },
                ClosureEvent::CoreReturned { orb_id: 1, core: "corebound-sky-1".into(), resource, amount: ratio(9, 2) },
                ClosureEvent::CoreWordDeposited { core: "the-founding".into(), world: "tun-reach".into(), orbit: [4; 32], expansion: vec![2, 0, 1, 4], cited: vec![[5; 32], [6; 32]] },
            ]
        });
        for event in events {
            let mut c = cfe::vrp::Chronicle::new();
            c.append(event.clone(), 1, vec![]);
            let r = &c.receipts()[0];
            let bytes = r.wire_bytes();
            let (tag, value) = cfe::tlv::records(&bytes).next().expect("one frame");
            let back = cfe::vrp::Receipt::from_wire(tag, value).expect("reads");
            assert_eq!(back.event(), &event);
        }
    }
}
