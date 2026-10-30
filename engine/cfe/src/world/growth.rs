//! GROWTH-1 — deriving the six growth mechanisms from the record.
//!
//! Every axis reads the chronicle and the books. Nothing is stored,
//! nothing is compared against a dialed number, and nothing here can be
//! reached at stamp 0 except by a line that genuinely did everything at
//! stamp 0 — which is the point.

use super::*;
use crate::growth::{share, Growth};

impl World {
    /// **The GROWTH reading** — the six axes of one orb, derived whole.
    ///
    /// Postings carry no stamp on the account (the account holds
    /// totals), so when comes from the chronicle, which is where when
    /// belongs. The books hold how much; the chain holds how long.
    pub fn growth_of(&self, orb: u64) -> Growth {
        let zero = ratio(0, 1);
        let now = self.clock.now();

        // ── the postings, from the record ────────────────────────────
        let mut first: Option<u64> = None;
        let mut last: Option<u64> = None;
        let mut postings = 0u64;
        let mut empty = 0u64;
        for receipt in self.chronicle.history() {
            let ClosureEvent::BookPosted { orb_id, kept, lost, .. } = receipt.event() else {
                continue;
            };
            if *orb_id != orb {
                continue;
            }
            postings += 1;
            if *kept == zero && *lost == zero {
                // A posting that moved nothing. It happened, and it
                // taught nothing — triviality is what says so, and the
                // SPAN below refuses to count it at all.
                empty += 1;
                continue;
            }
            // Only real writing extends the span. Without this, a line
            // could post zeros across a long life and read as practised
            // — padding buying literacy, which is the leap re-entering
            // through the axis built to close it.
            first = Some(first.map_or(receipt.stamp(), |held| held.min(receipt.stamp())));
            last = Some(last.map_or(receipt.stamp(), |held| held.max(receipt.stamp())));
        }

        // Practice — the span the writing covers, against the age of the
        // line. Everything at one stamp spans nothing and reads 0.
        let span = match (first, last) {
            (Some(first), Some(last)) => last.saturating_sub(first),
            _ => 0,
        };
        let practice = share(ratio(span as i64, 1), ratio(now as i64, 1));

        // ── the accounts ─────────────────────────────────────────────
        let held = self.orb_by_id(orb);
        let accounts: Vec<(Ratio, bool)> = held
            .map(|orb| {
                orb.books()
                    .into_iter()
                    .filter(|view| view.posted())
                    .map(|view| {
                        let weight = view.kept.clone() + view.lost.clone();
                        // Tested: both sides posted. Only-ever-winning
                        // is a record, not a lesson.
                        let tested = view.kept > zero && view.lost > zero;
                        (weight, tested)
                    })
                    .collect()
            })
            .unwrap_or_default();

        let total: Ratio = accounts
            .iter()
            .fold(zero.clone(), |sum, (weight, _)| sum + weight.clone());
        let deepest = accounts
            .iter()
            .map(|(weight, _)| weight.clone())
            .max()
            .unwrap_or_else(|| zero.clone());
        let tested = accounts.iter().filter(|(_, tested)| *tested).count();

        // ── the line, for scope ──────────────────────────────────────
        let line: Ratio = self.orbs.iter().fold(zero.clone(), |sum, orb| {
            sum + orb
                .books()
                .into_iter()
                .filter(|view| view.posted())
                .fold(ratio(0, 1), |held, view| held + view.kept + view.lost)
        });

        Growth {
            practice,
            knowledge: accounts.len(),
            expertise: share(deepest, total.clone()),
            wisdom: share(ratio(tested as i64, 1), ratio(accounts.len() as i64, 1)),
            locality: share(total, line),
            triviality: share(ratio(empty as i64, 1), ratio(postings as i64, 1)),
        }
    }
}
