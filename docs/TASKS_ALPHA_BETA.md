# TASKS — ALPHA/BETA READINESS (doc-vs-code audit, 2026-08-13)

**Status:** AUDIT + PROPOSED PHASING. Produced by a full docs/-vs-`engine/`
sweep (5 parallel agents, one per subsystem cluster, each verifying code
directly rather than trusting doc prose) plus the standing-law findings
from this session's `40_REJUVENATION.md` §6 work. Not itself LAW — a
work-list to be ruled phase by phase, same discipline as `BUILD_ORDER.md`.

**Baseline measured, not asserted:** `cargo test --workspace` is GREEN
except one pre-existing debt (`tuned_constant_ratchet`, 165 literals vs a
ceiling of 112 — red before this session, unrelated to any work here).
The engine is not broken; the gaps below are missing law, not broken law.

---

## 0. Ground truth and hygiene — before any feature work

- [ ] **Reconcile `tuned_constant_ratchet`** (165 vs ceiling 112,
  `engine/cfe/tests/tuned_constant_ratchet.rs`; `PICKUP.md`'s own
  snapshot says 170, so the doc is already stale against the live
  count — someone has been paying this down uncredited). Either the
  ceiling is stale against work already converted, or literals crept
  in uncredited. This is the project's own dial-debt gate; it must be
  green before any phase below closes.
- [ ] **Truth-audit doc status lines against the task boards.**
  `32_STAMP_MARKET.md` and `TASKS_TIER_RAIL_MARKET.md` both claim
  "Phase-4 complete," but I5.1–I5.6 (Share/Note/Future/Perp lifecycle,
  `instrument_step`) are still unchecked `[ ]` on the same board — a
  live violation of `00_CHARTER.md`'s one-truth discipline. Every doc
  claiming LANDED/COMPLETE status this audit found PARTIAL needs its
  status line corrected, not just this list.
- [x] **The shippable surface for "alpha" — CONFIRMED, not open.**
  `clients/cli/src/main.rs` is a real play-through path (found → build
  → trade → diplomacy → judge), not a test harness — the N3 arc
  (agent refused → granted → collected → revoked → refused, 9
  receipts, replay MATCHES) is measured end to end. **Persistence is
  complete**: `chronicle.tlv` append-only, `World::resume` replays
  byte-exact across a real process boundary, double-boot and
  era-convergence (128 worlds annexed, torn-tail recovery) both
  measured. **Multiplayer is NOT ready**: `engine/mesh` carries
  two-node chronicle replication (`examples/ns5_two_nodes.rs`, forged
  chains refuse at the break) but **no cross-node consensus** —
  reconciliation, resync, and Sybil-resistant peer weighting are all
  still open, consistent with the standing "mesh-second" ruling.
  **Ruling: alpha ships single-node (one player or a trusted LAN
  replica), not networked multiplayer** — the engine underneath is
  ready for that scope, nothing here blocks it.
- [ ] **The kernel Phase-3 flip (`39_PHASE3_FLIP.md`) — a second,
  cheap, high-leverage gate found by this audit, ahead of founding.**
  The kernel currently runs BESIDE the chain (judges every deed, 70/71
  types covered, zero refusals measured on a 15-world live sample) but
  is not yet the enforcement gate — deeds still apply regardless of
  the judge's verdict. Promoting it needs exactly **one operator
  decision**: the addressing-law question in `39_PHASE3_FLIP.md` §3.
  This is unrelated to founding/tithe and can be ruled in parallel,
  but it should be ruled EARLY — every phase below plays differently
  once refusals are load-bearing instead of advisory.

## 1. The founding economy — hard blocker, first deed every player hits

`20_FOUNDING.md` is marked **LAW** (2026-08-02). Measured against code,
it is close to **zero-implemented**. This is the worst doc/code gap
found in the whole sweep, because founding is literally step one.

- [ ] **The tithe (§3, §6a) — currently absent entirely.** Zero `tithe`
  hits anywhere in `engine/cfe/src/world/`. No capture-capacity
  calculation, no recipient selection (existing planet → elder,
  new world → ancient), no value movement. `ward_elder()`
  (`genesis.rs:303`) sets a bare boolean, nothing more.
- [ ] **Wire the seat-scope ladder for real (`19_REGISTRY.md` §4a).**
  `Entity`/`Scope` (`ipsal.rs`) has no Elder/Ancient seat today —
  tithe routing needs one to exist before it can route anything.
- [ ] **Fix the `seat_width()` None-placeholder defect.** 7 call sites
  (`ledger/src/registry.rs:124,132,260,269`; `conductor/lib.rs:1837,1843`;
  `cli/main.rs:1605`) pass `None`, leaving `thermal`/`cryo` domains
  unreachable by founding — marked a defect by the operator on
  2026-08-02 and still open.
- [ ] **Rule the scope of PURCHASE (§2a) and COMMISSION (§2b) for
  alpha.** Today: `post_offer`/`clear_trade` trade `Resource`s between
  orbs, never a planetary ownership stake — no cross-World share
  purchase exists. `commission_star` (`thermal.rs:17`) is a bare stub:
  it sets `StarClass` and fires a receipt with **no price formula**
  (§6b's `work_k · |1/r_current − 1/r_target|` is nowhere in code) and
  **no progenitor gate at all** — any sovereign calls it solo. Decide:
  ship alpha on FALLBACK (derived siting) + a real tithe only, with
  PURCHASE/COMMISSION explicitly deferred and labeled as such in the
  doc — or scope them in now. Either is fine; leaving the doc silent
  on which is not.

**Definition of done:** a fresh founding (existing-planet and
new-world cases, both) pays a receipted tithe to a real seat, replays
byte-exact, and the doc's status line matches what shipped.

## 2. Close the rejuvenation loop — task #28's already-scoped tail

`40_REJUVENATION.md` §1–5 (the regrowth curve, K/r derivation, the
generic-ration read) is **landed and green** (commits `100b0a7`,
`06c5ff7`, `e76989d`, `f467876`). One piece was scoped from the start
and never built:

- [ ] **The mutating settle-at-touch deed.** Bank the grown stock at a
  real extraction/collect touch, emit the receipt digest (the
  `ExtractionResolved`-shaped outcome `40_REJUVENATION.md` §2 already
  names), advance `last_touch`. Today only the read-only scan
  (`World::regrown_ration`) exists — a player's grown stock is never
  actually banked. Needs a kernel conservation arm for the mint (same
  class of fix as task #21's `ConservationBroken` repair).

**Why this gates other work:** per standing project memory, tasks #29
(element economy famine/relief) and #30 (quarantine decay/relief) both
wait on this same settle-deed keystone. Landing it here unblocks three
lanes at once, not just rejuvenation.

## 3. Truth-align the market/instrument layer

- [ ] **Land I5.1–I5.6 or demote the claim.** Share dividend engine,
  Note coupon/default, Futures margin, Perp funding rate,
  `instrument_step` lifecycle, default-integrity hooks — all six are
  unchecked on `TASKS_TIER_RAIL_MARKET.md` despite `32_STAMP_MARKET.md`
  reading "Phase-4 complete." A beta player told they can trade real
  instruments, when none of the six lifecycle tasks exist, is a
  shipped inaccuracy, not a missing feature — fix the claim even if
  the feature waits.
- [ ] **E3.9 treaty element allow-lists** — open on the same board;
  also the direct precedent this session's Medium-ration ruling
  (`40_REJUVENATION.md` §6.2) depends on for its rate-cap annex.
- [ ] **`element_trade_step` NPC automation** (`31_ELEMENT_COMMERCE.md`)
  — refinery/route/market machinery is landed; the pure-NPC trade loop
  that's supposed to run without a player is not wired in `conductor`.

## 4. The progenitor/delegation foundation — unlocks cross-sovereign play

This is new scope this session surfaced, not a pre-existing task-board
item, so it needs its own ruling before work starts. `12_UNION.md`
§2/§2a (`Station::progenitor` as Tier-0 star/planet creator, planets
DELEGATED to sovereigns under council-gated diplomacy) is marked
DESIGN and is, independently confirmed by two audits this sweep,
**almost entirely unimplemented**:

- `commission_star` is the only star-creation path, is entirely
  sovereign-private, and has **no progenitor gate of any kind** — not
  even a self-check. The station value `"progenitor"` exists only as a
  registry/tier constant; the *only* progenitor-gated deed anywhere in
  the codebase is vocabulary minting.
- `GalaxyObserved` (tag 54, `13_GALAXY.md`) is computed
  (`conductor::observe_crossings` does the L1 frontier math) but **never
  emitted to the universe ledger** — no receipt, no `galaxies.json`,
  no replay persistence. The concept doesn't survive a restart today.

Phased:

- [ ] **4a. Progenitor-gated star creation.** Add the missing gate
  itself before anything else — today `commission_star` doesn't even
  check station, so "progenitor power" is currently unenforced law.
- [ ] **4b. The delegation deed.** A progenitor creates a star and
  delegates distinct orbits to distinct sovereign `World`s,
  council-gated per `12_UNION.md` §2 (or refused on breach/war, per
  the standing displacement law).
- [ ] **4c. Emit `GalaxyObserved` for real** — close the tag-54 gap so
  galaxy membership is a receipted, replayable fact, not a transient
  computation.
- [ ] **4d. Only after 4a–4b: land `40_REJUVENATION.md` §6** (the
  Medium ration, public externality, and its four-lever counter-play
  — diplomatic, legal, economic, delegation-gate) exactly as already
  ruled in that document. Not before — the ruling's own text names
  this dependency.

## 5. Defense teeth and remaining design-only law

- [ ] **Rule the kinetic lever.** Confirmed (independently, by this
  session's Medium-ration counter-play work and by this audit's
  defense sweep): `raze`/`raze_as` is self-service only
  (`Scope::Raze` checks the actor owns the zone); cargo-in-transit
  interdiction exists (`strike_interdiction`) but **no mechanism lets
  one `World` destroy another `World`'s standing zone**. Multiple
  design docs gesture at war having real teeth beyond a trade wall;
  today it doesn't. Needs its own ruling, referenced but not decided
  by `40_REJUVENATION.md` §6.2.
- [ ] Triage the remaining confirmed design-only items into
  needed-for-beta vs. post-beta: `05_THERMAL_BALANCE.md` §1.3–§1.4/§7
  (sink export, cryo-vault lossless banking, the crypto/settlement
  layer, symbolic-π extension); `35_ORBIT_DYNAMO_AND_ESTATE.md` Q-O1/
  Q-O2 (ψ precession, hull/route as Holding 6–7); species-specific
  mechanics named in `06_CIVILIZATIONS.md` (Glasskeeper escrow,
  Coherent proving-node role, Marrow terraforming) that are flavor
  text today with no mechanical hook.

---

## Dependency sketch

```text
Phase 0 (hygiene + the Phase-3 flip decision) ──► Phase 1 (founding/tithe)
                                                    ──► every later phase
                                                        (founding is step one)
Phase 2 (settle-deed) ──► tasks #29, #30 (already blocked on it, per memory)
Phase 4a ──► 4b ──► 4c
                └──► 4d (40_REJUVENATION §6, already ruled, blocked here)
Phase 3 and Phase 5 are independent of the others and of each other —
schedule by priority, not by dependency.
The kernel Phase-3 flip (§0) is independent of founding/tithe (§1) and
can be ruled in parallel — it changes how EVERY later phase behaves
(refusals become load-bearing) so ruling it early, even if it lands
later, avoids re-deriving downstream work against the wrong assumption.
```

**Scope ruling this audit makes explicitly:** alpha targets a single
node (one player, or a trusted LAN replica of the same chronicle) —
the engine's persistence and CLI are alpha-ready today; the mesh's
cross-node consensus is not, and is correctly out of scope per the
standing mesh-second ruling. Multiplayer is a beta-or-later target,
gated on `engine/mesh` work (§36 spine adoption, Σ_mesh resync, a real
Sybil-resistant peer weighting) not named further in this list because
it was already correctly deferred.

## Definition of done (program)

1. A cold-start player can found (paying a real tithe to a real seat),
   build, trade, and see the doc's own status lines match what
   actually runs — no LANDED/COMPLETE claim contradicted by an open
   checkbox on its own task board.
2. `cargo test --workspace` green, `tuned_constant_ratchet` included.
3. Rejuvenation's settle deed lands, unblocking #29/#30.
4. Every DESIGN-status doc this audit touched carries an explicit
   alpha/beta/post-beta triage, not silence.
5. The kernel Phase-3 flip is ruled (landed or explicitly deferred with
   a reason) rather than left an ambient assumption.
6. The progenitor/delegation slice (4a–4c) lands before
   `40_REJUVENATION.md` §6 is attempted — the ruling already says so.
