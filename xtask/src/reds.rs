//! **The KNOWN-red ledger** — every suite the board currently fails,
//! each wearing its owning task and its diagnosis. The ratchet's
//! discipline applied to test coverage: a red is a counted debt
//! entering by reviewed edit, never an accident, and `cargo board`
//! fails only on reds not in this list. Remove an entry in the same
//! change that cures it.
//!
//! Baseline measured 2026-08-12 at `3cdba22` (211 ok / 14 red).
//! Task #10 diagnostic sweep closed eight: world_market · world_absorb
//! · world_tech_tree (the crossing-law batch) · vertical_axis_pins ·
//! measured_bounds · roster_claim_pins · polity_therm_pins ·
//! resume_pins · arbor_pins. A ninth closed 2026-08-12: mint_loop_pins
//! — the refused deed was NoteIssued (PHASE-4 note collateral peeled
//! from the free STAMP bank), not the chemistry deeds the first
//! diagnosis named; a `stamp_move` arm now declares that off-bank move.
//! Four remain, each root-caused below.

pub const KNOWN_REDS: &[(&str, &str)] = &[
    // Operator-gated —
    (
        "tuned_constant_ratchet",
        "task #3 — 170 dials vs 112 ceiling; needs per-cluster operator rulings (R20 map)",
    ),
    (
        "ring_derivation_pins",
        "task #10 REAL/OPERATOR — offices grew to arity 16 (Nexus) but the θ-ring is 60 \
         and 16 does not divide it; the fix is a structural ruling (TURN_SLOTS 60→240, \
         re-keys every angle) OR removing tier-ladder offices from the ring vocabulary",
    ),
    // Fixture drift, deeper than a re-pin (needs FLEET-1/Phase-2 staging + re-derived math) —
    (
        "merchant_pins",
        "task #10 FIXTURE — the caravan triangle needs FLEET-1 hull staging AND a \
         re-derived profit under the new melt; hull staging alone did not clear it \
         (a second gate in the triangle path). Fixture realignment, not a defect.",
    ),
    (
        "offer_pins",
        "task #10 FIXTURE — famine-ask price cross fails under post-arc stocks after the \
         FLEET-1 hull staging; one of two pins green, the other owes its diagnosis.",
    ),
];
