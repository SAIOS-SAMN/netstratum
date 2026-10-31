//! Netstratum's seat tiers. Mycelium carries a seat's role as an opaque tier,
//! `Role(0..=9)`, and judges nothing by it; the game names its tiers. The
//! bytes are the ones Netstratum's houses have always offered.
use mycelium::anastome::dialogue::seat::Role;

/// The progenitor's seat, tier 0.
pub const PROGENITOR: Role = Role(0);
/// The house's own seat, the ancients', tier 1.
pub const ANCIENTS: Role = Role(1);
/// A steward's seat, tier 2.
pub const STEWARD: Role = Role(2);
/// A sovereign's seat, tier 3.
pub const SOVEREIGN: Role = Role(3);
