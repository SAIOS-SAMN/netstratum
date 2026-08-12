//! **THE KNOWN-RED LEDGER** — every suite the board currently fails,
//! each wearing its owning task. The ratchet's discipline applied to
//! test coverage: a red is a COUNTED debt entering by reviewed edit,
//! never an accident, and `cargo board` fails only on reds NOT in
//! this list. Remove an entry in the same change that cures it.
//!
//! Measured 2026-08-12: full workspace at `3cdba22` with
//! `--no-fail-fast` (211 suites green, 14 red), minus the three cured
//! the same day (world_market · world_absorb · world_tech_tree).

pub const KNOWN_REDS: &[(&str, &str)] = &[
    ("tuned_constant_ratchet", "task #3 — 170 dials vs 112 ceiling; R20 map"),
    ("measured_bounds", "task #10 — settlement footprint moved; re-measure"),
    ("merchant_pins", "task #10 — caravan bridge refuses; triangle diagnosis owed"),
    ("mint_loop_pins", "task #10 — judge refusals reached a chronicle"),
    ("offer_pins", "task #10 — famine-ask price cross fails post-arc"),
    ("polity_therm_pins", "task #10 — Frost sink charter count moved"),
    ("incursion_pins", "task #10 — undiagnosed (baseline sweep)"),
    ("pen_pins", "task #10 — undiagnosed (baseline sweep)"),
    ("recurrence_guard", "task #10 — undiagnosed (baseline sweep)"),
    ("resume_pins", "task #10 — undiagnosed (baseline sweep)"),
    ("roster_claim_pins", "task #10 — undiagnosed (baseline sweep)"),
    ("world_tech_arcs", "task #10 — undiagnosed (baseline sweep)"),
];
