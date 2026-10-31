//! MATTER-BALANCE-1 (34) — thermal dual gate and OrderWorked booking.
//!
//! Caps refine / compound / hull mint when entropy_vented outruns
//! syntropy_contained unless a Regenerator stands. Readings derive;
//! amounts for OrderWorked use 1+1 halves/quarters only.

use super::*;

/// OrderWorked kind codes (wire).
pub const ORDER_REFINE: u8 = 1;
pub const ORDER_COMPOUND: u8 = 2;
pub const ORDER_HULL: u8 = 3;
pub const ORDER_SALVAGE: u8 = 4;

impl World {
    /// Thermal balance on one orb: syntropy / (syntropy + entropy).
    /// `None` when both arms are zero (bootstrap — no gate).
    pub fn thermal_balance_of(&self, orb: u64) -> Option<Ratio> {
        let held = peek(&self.orbs, orb)?;
        let s = held.syntropy_contained.clone();
        let e = held.entropy_vented.clone();
        let whole = s.clone() + e.clone();
        if whole == ratio(0, 1) {
            return None;
        }
        Some(s / whole)
    }

    /// Does a Regenerator stand on this orb?
    pub fn regenerator_stands(&self, orb: u64) -> bool {
        peek(&self.orbs, orb)
            .is_some_and(|held| held.zones.iter().any(|z| z.kind == ZoneKind::Regenerator))
    }

    /// Half as 1/(1+1) — orderly floor.
    pub fn balance_orderly_floor() -> Ratio {
        ratio(1, 1) / (ratio(1, 1) + ratio(1, 1))
    }

    /// Gate for order-work: refine, compound, hull mint.
    /// Unbooked (None) passes; Regenerator passes; balance ≥ 1/2 passes.
    pub fn require_order_balance(&self, orb: u64) -> Result<(), ActionRefused> {
        let Some(held) = peek(&self.orbs, orb) else {
            return Err(ActionRefused::NoSuchOrb { orb });
        };
        if self.regenerator_stands(orb) {
            return Ok(());
        }
        let s = held.syntropy_contained.clone();
        let e = held.entropy_vented.clone();
        let whole = s.clone() + e.clone();
        if whole == ratio(0, 1) {
            return Ok(()); // bootstrap
        }
        let balance = s.clone() / whole;
        if balance >= Self::balance_orderly_floor() {
            return Ok(());
        }
        Err(ActionRefused::BalanceEntropyDominates {
            balance: Box::new(balance),
            syntropy: Box::new(s),
            entropy: Box::new(e),
        })
    }

    /// Book both thermal arms and receipt OrderWorked (no Resource move).
    pub(super) fn post_order_worked(
        &mut self,
        actor: u64,
        orb: u64,
        kind: u8,
        entropy: Ratio,
        syntropy: Ratio,
    ) {
        if entropy == ratio(0, 1) && syntropy == ratio(0, 1) {
            return;
        }
        if let Some(held) = peel(&mut self.orbs, orb) {
            held.entropy_vented += entropy.clone();
            held.syntropy_contained += syntropy.clone();
        }
        self.close(
            ClosureEvent::OrderWorked {
                orb_id: orb,
                kind,
                entropy,
                syntropy,
            },
            actor,
        );
    }

    /// Derive OrderWorked arms for a refine (spend, yielded).
    pub(super) fn order_arms_refine(
        &self,
        orb: u64,
        spend: &Ratio,
        yielded: &Ratio,
    ) -> (Ratio, Ratio) {
        let four = ratio(1, 1) + ratio(1, 1) + ratio(1, 1) + ratio(1, 1);
        let entropy = spend.clone() / four;
        let syntropy = if self.regenerator_stands(orb) {
            yielded.clone()
        } else {
            ratio(0, 1)
        };
        (entropy, syntropy)
    }

    /// Compound: pure syntropy of gem yield.
    pub(super) fn order_arms_compound(&self, yielded: &Ratio) -> (Ratio, Ratio) {
        (ratio(0, 1), yielded.clone())
    }

    /// Hull mint: structure syntropy + residual entropy.
    pub(super) fn order_arms_hull(&self, scrap_spent: &Ratio) -> (Ratio, Ratio) {
        let two = ratio(1, 1) + ratio(1, 1);
        let four = two.clone() + two.clone();
        (scrap_spent.clone() / four, scrap_spent.clone() / two)
    }

    /// Salvage: form-change residual books both arms equally (neutral
    /// recycle). Regenerator is not required for scrap→structure feed;
    /// it unlocks *order-work* under refine/vent strain (34 §2.2–2.3).
    pub(super) fn order_arms_salvage(&self, _orb: u64, amount: &Ratio) -> (Ratio, Ratio) {
        let eight = {
            let two = ratio(1, 1) + ratio(1, 1);
            let four = two.clone() + two.clone();
            four.clone() + four
        };
        let residual = amount.clone() / eight;
        (residual.clone(), residual)
    }
}
