# 40 — PLANETARY REJUVENATION (the regrowth law)

**Status:** DESIGN, ruled 2026-08-12 (operator, Gemini relay). Growth
form RULED: **(B) rational sigmoid on a grain** (§4) — preserve the
S-curve so collapse from near-zero is genuinely expensive to recover.
Charter-law translation of the relayed design; **not** a transcription.
Task #28.

## 0. Why

Pure extraction makes every civilization a zero-sum player against the
universe's starting stock, and the only mathematically viable endgame
is planet-jumping — a single meta, the **Locust**. Regrowth toward a
carrying capacity opens the divergent strategies the taxon economy
already names:

- **The Custodian** keeps per-beat extraction below the growth rate
  `r`. Slower early industry, but core worlds never degrade — a system
  held indefinitely.
- **The Locust** extracts far past `r`. The ecosystem collapses toward
  zero for an immediate surge, and the dead world takes a very long
  time to climb back — so the Locust must keep jumping.

The mechanic must produce this split without a single tuned constant
outside the derivations below, and without breaking the exact-rational,
byte-exact-replay engine.

## 1. The one law over this chapter — deterministic physics get NO deed

The ledger records **decisions** — a player action, a treaty, an
extraction — never the passage of time. Logistic regrowth is a
mathematical certainty given `(P0, K, r, Δt)`, so the chain needs no
row to agree on what it did. This is not a new principle: it is exactly
how **famine, wants, and demurrage** already work — replay-derived
state, recomputed identically on every node, never a stored authority.

**Rejuvenation is demurrage run backwards.** `world::rules::demurrage_due`
(rules.rs:121) already settles a FLAT decay lazily *at the touch*:
`elapsed = stamp − orb.last_touch`, the decay applied only when a deed
touches the orb, capped so it never overshoots. Regrowth is the mirror
image — a settlement toward `K` instead of toward `0` — and it reuses
the same `last_touch` stamp the demurrage organ already carries.

## 2. Lazy evaluation — settle Δt only at an audit

No per-beat recompute. A world's natural stock is governed by the
growth curve (§3–§4); the engine evaluates the area under it only when
an event forces an audit.

**Mutating audit** (extraction, bombardment, quarantine) — the deed
that interrupts natural growth is the only thing written. When the
Replayer reaches it:

1. read `Δt = T_current − T_last` (`T_current` is the deed's stamp;
   `T_last` is the zone's last settled stamp);
2. run the growth law over `Δt`, crediting the grown stock to the
   zone's memory;
3. apply the deed's own mutation (e.g. subtract the extracted amount);
4. set `T_last = T_current`.

**The receipt proves the math.** The deed's receipt carries the
resolved outcome — the grown amount and the new baseline — so an
auditing node reconstructs the same state without a growth deed:

    ExtractionResolved { extracted, physics_yield_applied, new_baseline }

(This is the existing receipt-digest idiom — deeds already carry their
computed outcome, e.g. `NoteRedeemed { paid, seized }`.)

**Read-only audit** (a scan) runs the identical Δt calculation in
memory and serves the value to the client. It writes nothing, mints no
receipt, and does **not** advance `T_last`.

**Snapshot** (`state.tlv`, task #18) resolves every zone up to the
checkpoint beat, writes the hard absolute stock, and resets every
zone's `T_last` to the checkpoint. A conductor booting from the
snapshot starts from absolute numbers and ignores all prior history —
the O(bytes) boot the linear stepper already gives us.

## 3. The carrying capacity and the rate — derived, not dialed

- **`K` (carrying capacity)** — the natural ceiling of a resource on a
  world, read from what the world already IS: its `ZoneKind`, its
  `StarClass` luminosity, and its orbital position (the same three the
  thermal and locality laws already read). A lush habitable-band world
  carries a large `K` for biomass; a barren rock, near zero. No new
  table — `K` composes the standing readings.
- **`r` (intrinsic growth rate)** — how fast the environment recovers,
  derived from the same place. `r` is the axis the archetypes are
  measured against: extract below it and the Custodian holds; above it
  and the Locust collapses the world.

Both are exact rationals, both fall out of readings that already exist,
so neither is a tuned dial.

## 4. THE OPEN FORK — the growth curve's exact-rational form

The relayed model is the logistic closed form

    P(t) = K / (1 + ((K − P0)/P0) · e^(−r·Δt))

and this **cannot be transcribed**. `e^(−r·Δt)` is transcendental;
netstratum is exact-rational with no floats. Demurrage's own comment
records the wall directly: a proportional/geometric shape
`banked × (31/32)^n` "explodes the canonical denominator without bound
(measured live: ~2^1100 after 220 beats)", which is why the flat law
replaced it. The logistic's *differential* `dP/dt = r·P·(1 − P/K)` is
fully rational; only its closed-form *solution* carries the exponential.

Two netstratum-legal translations, each a permanent SEAL-B choice:

- **(A) Flat-toward-K.** Regrowth is a flat rate per elapsed beat,
  capped at `K` — demurrage's proven idiom, mirrored. O(1) per audit,
  denominators bounded by the rate's. **Cost:** it loses the S-curve —
  a world at near-zero recovers at the same rate as one at mid-stock,
  so "dead worlds take thousands of beats" becomes "dead worlds take a
  fixed many beats." The Locust/Custodian split still holds (extraction
  vs `r`), but collapse is less punishing.

- **(B) Rational sigmoid on a grain.** A rational approximant of the
  logistic (a Padé-style surrogate) evaluated on a fixed grain — the
  same move `grained_t_eq4`/`ORBIT_GRAIN` made to keep exact heat
  bounded. O(1) per audit, denominators bounded by the grain, and it
  **preserves the slow-near-zero S-shape** so collapse is genuinely
  expensive to recover from. **Cost:** more design and more witnesses;
  the surrogate's shape is a chosen approximation.

**RULED (B), 2026-08-12** — the S-curve is the whole point of the
Locust/Custodian tension (slow recovery from collapse is what forces
the jump), and the grain precedent shows exact arithmetic can carry it
bounded. The rational surrogate and its grain are to be designed as the
first landing slice of task #28; its shape becomes SEAL-B law once a
receipt reads it, so it lands red-first with witnesses pinning the
S-shape (slow near 0, fast at mid-stock, saturating at K) and the
bounded-denominator property, exactly as `grained_t_eq4` pinned heat.

## 5. What this composes with

- **Demurrage** (`rules.rs`, `01_ECONOMY`) — the same `last_touch`
  settle-at-touch organ; regrowth is its mirror.
- **Extraction / yield** (`world/tick.rs`, `dop::zone_yield`) — the
  mutating audits that trigger settlement.
- **The snapshot** (`state.tlv`, task #18) — resolves and resets
  `T_last` at the checkpoint.
- **The taxon economy** (`37_IMPERIUM_TAXON`) — extractive vs
  regenerative vs balanced modes are the archetypes this law prices.
