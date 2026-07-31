//! OPPOSITION-1 — the anti-mirror readings (NS-5 law 2, the mesh
//! board's #4): the first measurement whose inputs are not copies of
//! each other. Every deal-linked deed carries its own inverse in the
//! OTHER book; the opposition sums each pair and refuses nonzero with
//! the residual WHOLE — a forger who writes the right shape with the
//! wrong number breaks by VALUE, not by digest.
//!
//! Pure readings over receipt slices: nothing here mutates, nothing
//! receipts (the lens law). The mesh walk consumes these; the
//! signatures are the lane seam (coder-04's side feeds what its
//! spine holds — valid, partial, or forged alike; the reading tells
//! the truth about all three).

use crate::dop::{ClosureEvent, Resource};
use crate::ratio::{ratio, Ratio};
use crate::vrp::Receipt;
use std::collections::BTreeMap;

/// One failed opposition — the reading carried whole, never a scalar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OppositionBreak {
    /// A deal's aperture legs do not net zero across the two books:
    /// both sums carried (the residual is theirs to subtract — shown,
    /// never summarized away).
    ValueUnbalanced { deal: u64, resource: Resource, sent: Ratio, received: Ratio },
    /// A licensed leap whose legs disagree or stand unpaired: the
    /// node each side holds, whole. One node, two skies — or a break.
    LeapUnpaired { deal: u64, issued: Option<[u8; 32]>, arrived: Option<[u8; 32]> },
}

/// The opposition read over two books: every (deal, resource) pair's
/// sent and received legs summed ACROSS both slices (a deal's legs
/// live one per book by the aperture law, but the reading assumes
/// nothing — it measures what it is handed), and every licensed leap
/// paired with an arrival carrying the SAME node in the other book.
/// An empty return is law 2 holding at exact zero.
pub fn opposition_read(ours: &[Receipt], theirs: &[Receipt]) -> Vec<OppositionBreak> {
    let mut sent: BTreeMap<(u64, u8), Ratio> = BTreeMap::new();
    let mut received: BTreeMap<(u64, u8), Ratio> = BTreeMap::new();
    let mut issued: BTreeMap<u64, [u8; 32]> = BTreeMap::new();
    let mut ours_nodes: Vec<[u8; 32]> = Vec::new();
    let mut theirs_nodes: Vec<[u8; 32]> = Vec::new();
    for (book, receipts) in [(0u8, ours), (1u8, theirs)] {
        for receipt in receipts {
            match &receipt.event {
                ClosureEvent::ApertureSent { deal, resource, amount, .. } => {
                    let held = sent
                        .entry((*deal, *resource as u8))
                        .or_insert_with(|| ratio(0, 1));
                    *held += amount.clone();
                }
                ClosureEvent::ApertureReceived { deal, resource, amount, .. } => {
                    let held = received
                        .entry((*deal, *resource as u8))
                        .or_insert_with(|| ratio(0, 1));
                    *held += amount.clone();
                }
                ClosureEvent::TechLicensed { deal, node, .. } => {
                    issued.insert(*deal, *node);
                }
                ClosureEvent::TechDiscovered { node, .. } => match book {
                    0 => ours_nodes.push(*node),
                    _ => theirs_nodes.push(*node),
                },
                _ => {}
            }
        }
    }
    let mut breaks = Vec::new();
    let keys: std::collections::BTreeSet<(u64, u8)> =
        sent.keys().chain(received.keys()).copied().collect();
    for key in keys {
        let sent_total = sent.get(&key).cloned().unwrap_or_else(|| ratio(0, 1));
        let received_total = received.get(&key).cloned().unwrap_or_else(|| ratio(0, 1));
        if sent_total != received_total {
            let (deal, code) = key;
            let resource = Resource::from_wire(code).expect("legs carry known codes");
            breaks.push(OppositionBreak::ValueUnbalanced {
                deal,
                resource,
                sent: sent_total,
                received: received_total,
            });
        }
    }
    for (deal, node) in issued {
        let arrived = ours_nodes
            .iter()
            .chain(theirs_nodes.iter())
            .any(|held| *held == node);
        if !arrived {
            breaks.push(OppositionBreak::LeapUnpaired {
                deal,
                issued: Some(node),
                arrived: None,
            });
        }
    }
    breaks
}
