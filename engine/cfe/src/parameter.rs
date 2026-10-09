//! **THE CHARTERED PARAMETERS** (P2, tag 103 — the mint path for the
//! balance placeholders, ruled 2026-08-06 alongside the dop.rs minting
//! design).
//!
//! A balance placeholder was a dial in a function: tunable only by a
//! commit, readable only as a constant. This registry makes each one
//! CHARTERABLE — a `ParameterChartered` receipt moves the value for
//! the world that receipted it, the fold reads seed ∪ charters in
//! chronicle order, and the live rule follows the chain instead of
//! the compiled tree. The seed IS the current dial values, moved here
//! whole (THE DEFINITION LAW: genesis reads no chain, so the founding
//! values are compiled — their census class stays DIAL until the
//! operator rules the registry's seed a class of its own; this module
//! changes what a value CAN do, not what it counts as).
//!
//! **Who may charter is the law**: chartering is a TIER 0 power under
//! the same standing gate as `mint_vocabulary` (12_UNION ruling 4 —
//! progenitors create the board everyone else plays on).
//!
//! **The band is derived, not chosen**: a chartered value must stand
//! inside the FOURFOLD BAND of its seed — `[seed/4, 4·seed]` — the
//! same band `factor_in_bounds` already rules for vocabulary factors,
//! anchored to the seed so successive charters cannot walk a rate
//! away one lawful step at a time. The band is decidable from the
//! value alone, so it lives at a DOOR ([`Chartered`]), not a judge
//! (29_CONSTRUCTION).

use crate::ratio::{ratio, Ratio};

/// The closed key vocabulary — wire codes, append-only. Each variant
/// cites the placeholder it charters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Parameter {
    /// THERMAL-1 (05 §1.2): joules one active encasement shell
    /// captures per tick.
    EncasementThroughput = 1,
    /// THERMAL-1 (05 §1.2): the orb's base radiator capacity.
    RadiatorBase = 2,
    /// THERMAL-1 (05 §1.2): the per-tick leak on accrued joules
    /// outside the cryo vault.
    JouleLeakPerTick = 3,
    /// ECON-J (05 §1.2): joules one active zone burns per tick for
    /// heating on a sub-mesic sited orb.
    HeatingPerZone = 4,
    /// SINK-1 (05 §1.4): sink one active cryo-lattice array captures
    /// per tick from vault-band headroom.
    SinkThroughput = 5,
    /// IDEO-1 (09 §3.2): the ideological friction rate φ.
    FrictionRate = 6,
    /// MASK-1a (14 §4): the veil's standing joule toll per collect.
    VeilToll = 7,
}

impl Parameter {
    pub fn from_code(code: u8) -> Option<Parameter> {
        match code {
            1 => Some(Parameter::EncasementThroughput),
            2 => Some(Parameter::RadiatorBase),
            3 => Some(Parameter::JouleLeakPerTick),
            4 => Some(Parameter::HeatingPerZone),
            5 => Some(Parameter::SinkThroughput),
            6 => Some(Parameter::FrictionRate),
            7 => Some(Parameter::VeilToll),
            _ => None,
        }
    }
    pub fn code(self) -> u8 {
        self as u8
    }
    /// The founding value — THE CHARTERED-PARAMETER SEED (P2, tag 103).
    ///
    /// **These are SEALED RATES, not open dials** (task #3, 2026-08-12).
    /// Each is the epoch-0 value every PRE-CHARTER beat read: the live
    /// value folds `seed ∪ charters` (`crate::parameter::of`), so with
    /// no charter the seed IS the rate, and editing it here would
    /// recompute every old chronicle's heat — breaking SEAL B on
    /// history. The lawful way to move one is a receipted
    /// `ParameterChartered` deed, never an edit. That is exactly the
    /// sealed-rate property (`21_KERNEL_PARAMETERS` §11): a rate whose
    /// change is history, not tuning. They leave the open-dial census
    /// wearing the seal, as the cliff and the flat sink rate do (the
    /// per-arm markers below carry the seal; this prose only explains).
    pub fn seed(self) -> Ratio {
        match self {
            Parameter::EncasementThroughput => ratio(2, 1), // SEALED-RATE: epoch-0 charter seed
            Parameter::RadiatorBase => ratio(2, 1), // SEALED-RATE: epoch-0 charter seed
            Parameter::JouleLeakPerTick => ratio(1, 8), // SEALED-RATE: epoch-0 charter seed
            Parameter::HeatingPerZone => ratio(1, 4), // SEALED-RATE: epoch-0 charter seed
            Parameter::SinkThroughput => ratio(2, 1), // SEALED-RATE: epoch-0 charter seed
            Parameter::FrictionRate => ratio(1, 8), // SEALED-RATE: epoch-0 charter seed
            Parameter::VeilToll => ratio(1, 4), // SEALED-RATE: epoch-0 charter seed
        }
    }
    pub const ALL: [Parameter; 7] = [
        Parameter::EncasementThroughput,
        Parameter::RadiatorBase,
        Parameter::JouleLeakPerTick,
        Parameter::HeatingPerZone,
        Parameter::SinkThroughput,
        Parameter::FrictionRate,
        Parameter::VeilToll,
    ];
}

/// **A LAWFUL CHARTER, proven at construction** (29_CONSTRUCTION):
/// the value stands inside the fourfold band of its key's seed. The
/// band anchors to the SEED, so charters do not compound — the whole
/// reachable space is `[seed/4, 4·seed]`, forever, by construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chartered {
    key: Parameter,
    value: Ratio,
}

impl Chartered {
    pub fn new(key: Parameter, value: Ratio) -> Option<Chartered> {
        let seed = key.seed();
        // The band reads the standing fourfold law — one home
        // (`vocabulary::fourfold_band`), anchored to the seed.
        let (low_factor, high_factor) = crate::vocabulary::fourfold_band();
        let low = seed.clone() * low_factor;
        let high = seed * high_factor;
        (value >= low && value <= high).then_some(Chartered { key, value })
    }
    pub fn key(&self) -> Parameter {
        self.key
    }
    pub fn value(&self) -> &Ratio {
        &self.value
    }
}

/// The literal door — a malformed literal is a build error in a
/// runtime coat.
pub fn chartered(key: Parameter, value: Ratio) -> Chartered {
    Chartered::new(key, value)
        .expect("a chartered literal must stand inside the fourfold band of its seed")
}

/// The whole table in ONE reverse walk — the tick's read (the
/// practice-channel precedent: one walk per tick, never one per key).
pub struct Table([Ratio; 7]);

impl Table {
    pub fn get(&self, key: Parameter) -> &Ratio {
        &self.0[key.code() as usize - 1]
    }
}

pub fn table_of(chronicle: &crate::vrp::Chronicle) -> Table {
    // The chronicle keeps each key's last charter as it is appended: no walk.
    Table(core::array::from_fn(|at| {
        chronicle.chartered(at as u8 + 1).cloned().unwrap_or_else(|| Parameter::ALL[at].seed())
    }))
}

/// **THE FOLD** — the value as the chain holds it: the LAST charter
/// for the key, or the seed where none stands. Same chain, same
/// value, on every node, forever.
pub fn of(chronicle: &crate::vrp::Chronicle, key: Parameter) -> Ratio {
    chronicle.chartered(key.code()).cloned().unwrap_or_else(|| key.seed())
}
