//! X7.4 — Anchor import/export deeds (STAMP only via typed proof).

use super::*;
use crate::dop::Rail;
use crate::ratio::Ratio;
use crate::xrail::{
    decode_isthmus, decode_strand, encode_stamp_packet,
    verify_court_claim, StrandKind,
};

fn strand_enabled() -> bool {
    cfg!(any(feature = "deep-rail-strand", feature = "stamp-rail-strand"))
}

fn court_enabled() -> bool {
    cfg!(any(feature = "deep-rail-court", feature = "stamp-rail-court"))
}

impl World {
    fn require_anchor(&self) -> Result<(), ActionRefused> {
        if !self
            .stations
            .iter()
            .any(|(_, s)| *s == crate::dop::Station::Anchor as u8)
        {
            return Err(ActionRefused::AnchorRequired {
                rail: Rail::Strand as u8,
            });
        }
        Ok(())
    }

    /// X7.1 export: burn STAMP, emit strand stamp packet (feature-gated).
    pub fn export_stamp_strand(
        &mut self,
        orb: u64,
        amount: crate::dop::Positive,
    ) -> Result<Vec<u8>, ActionRefused> {
        self.export_stamp_strand_as(SOVEREIGN, orb, amount)
    }

    pub fn export_stamp_strand_as(
        &mut self,
        actor: u64,
        orb: u64,
        amount: crate::dop::Positive,
    ) -> Result<Vec<u8>, ActionRefused> {
        if !strand_enabled() {
            return Err(ActionRefused::DeepRailGated {
                rail: Rail::Strand as u8,
                feature: "deep-rail-strand",
            });
        }
        self.permitted(actor, Scope::Build)?;
        self.require_anchor()?;
        let amt = amount.value().clone();
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.stamp_amount();
        if have < amt {
            return Err(ActionRefused::CannotAffordStamp {
                need: Box::new(amt),
                have: Box::new(have),
            });
        }
        held.bank_stamp(-amt.clone());
        let packet = encode_stamp_packet(&amt);
        let claim_hash = decode_strand(&packet)
            .map(|p| p.claim_hash)
            .unwrap_or([0u8; 32]);
        self.close(
            ClosureEvent::StrandExported {
                orb_id: orb,
                amount: amt,
                claim_hash,
                kind: StrandKind::Stamp as u8,
            },
            actor,
        );
        Ok(packet)
    }

    /// X7.4 import strand STAMP packet — credit only on valid decode.
    pub fn import_anchor_strand(&mut self, orb: u64, bytes: &[u8]) -> Result<Ratio, ActionRefused> {
        self.import_anchor_strand_as(SOVEREIGN, orb, bytes)
    }

    pub fn import_anchor_strand_as(
        &mut self,
        actor: u64,
        orb: u64,
        bytes: &[u8],
    ) -> Result<Ratio, ActionRefused> {
        if !strand_enabled() {
            return Err(ActionRefused::DeepRailGated {
                rail: Rail::Strand as u8,
                feature: "deep-rail-strand",
            });
        }
        self.permitted(actor, Scope::Build)?;
        self.require_anchor()?;
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        let Some(packet) = decode_strand(bytes) else {
            self.close(
                ClosureEvent::AnchorImportRefused {
                    orb_id: orb,
                    reason: 1,
                },
                actor,
            );
            return Err(ActionRefused::AnchorProofInvalid { reason: 1 });
        };
        if packet.kind != StrandKind::Stamp {
            self.close(
                ClosureEvent::AnchorImportRefused {
                    orb_id: orb,
                    reason: 2,
                },
                actor,
            );
            return Err(ActionRefused::AnchorProofInvalid { reason: 2 });
        }
        let amt = packet.amount.clone();
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        held.bank_stamp(amt.clone());
        self.close(
            ClosureEvent::AnchorImported {
                orb_id: orb,
                amount: amt.clone(),
                claim_hash: packet.claim_hash,
                kind: packet.kind as u8,
            },
            actor,
        );
        Ok(amt)
    }

    /// X7.2–X7.4 court path: isthmus frame carrying datum multi-axial claim.
    pub fn import_anchor_court(&mut self, orb: u64, bytes: &[u8]) -> Result<Ratio, ActionRefused> {
        self.import_anchor_court_as(SOVEREIGN, orb, bytes)
    }

    pub fn import_anchor_court_as(
        &mut self,
        actor: u64,
        orb: u64,
        bytes: &[u8],
    ) -> Result<Ratio, ActionRefused> {
        if !court_enabled() {
            return Err(ActionRefused::DeepRailGated {
                rail: Rail::Court as u8,
                feature: "deep-rail-court",
            });
        }
        self.permitted(actor, Scope::Build)?;
        self.require_anchor()?;
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        let Some(frame) = decode_isthmus(bytes) else {
            self.close(
                ClosureEvent::AnchorImportRefused {
                    orb_id: orb,
                    reason: 3,
                },
                actor,
            );
            return Err(ActionRefused::AnchorProofInvalid { reason: 3 });
        };
        let Some(claim) = verify_court_claim(&frame.payload) else {
            self.close(
                ClosureEvent::AnchorImportRefused {
                    orb_id: orb,
                    reason: 4,
                },
                actor,
            );
            return Err(ActionRefused::AnchorProofInvalid { reason: 4 });
        };
        let amt = claim.amount.clone();
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        held.bank_stamp(amt.clone());
        self.close(
            ClosureEvent::AnchorImported {
                orb_id: orb,
                amount: amt.clone(),
                claim_hash: frame.envelope_hash,
                kind: 4,
            },
            actor,
        );
        Ok(amt)
    }

}
