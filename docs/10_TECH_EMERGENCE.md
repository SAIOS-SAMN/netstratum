# TECH EMERGENCE — the crystallized tree (TECH-1 spec, design stage)

**Status:** DESIGN — lands as TECH-1 after BOT-THERM; the landing that opens
SINK-1's deferred research gate. Constants are placeholders.

**The ruling (operator, 2026-07-27):** no static external tech tree — a
hardcoded ladder makes every civilization climb the same line. Technology
CRYSTALLIZES: nodes emerge deterministically from each world's own chronicle
under sustained pressure or surplus. No two worlds need share a tree.

## 1. The dual model

- **Kernel (fixed, already landed):** the physics laws (05) and the forge
  rung ladder (conductor: rung 3 smelters, rung 5 synthesizers). The forge
  IS the research work — solves fund discovery. Kernel nodes are the only
  hardcoded ones.
- **Emergent nodes:** crystallize when a condition holds over a sustained
  window, measured FROM EXISTING RECEIPTS wherever possible (famine
  duration = now − strike stamp; vent pressure = `entropy_vented` delta per
  year; surplus = banked vs the heating bill). No shadow counters unless a
  condition is underivable — then a committed counter with receipts.

## 2. The laws

1. **Seeded synthesis (SEAL B):** a node's identity, name, cost, and exact
   multiplier derive from
   `BLAKE3(discovery receipt digest ‖ world ‖ condition kind)` — the
   chronicle root at the moment of crystallization. Replay re-derives the
   identical node at the identical tick. No runtime randomness, no
   external calls, no LLM in the loop.
2. **Discovery is a closure:** `TechDiscovered` (permanent tag 39) —
   `LE64(orb) ‖ node_id[32] ‖ condition u8 ‖ modifier u8 ‖ ratio (magnitude) ‖ name utf8`.
   Costs and magnitudes are exact rationals. Tag 40 RESERVED for
   `TechLicensed` (blueprint trade over TRADE-1 — a later landing).
3. **Modifiers touch only landed parameters:**
   `HeatingReduction(Ratio)` · `EncasementBoost(Ratio)` ·
   `SinkBoost(Ratio)` · `RadiatorBoost(Ratio)` · `DemurrageEase(Ratio)`.
   Applied multiplicatively in the tick's heat law; the orb carries its
   discovered node list (codec entry, byte-exact). `TradeRangeExpansion`
   REFUSED for now — no range mechanic exists.
4. **Names are renderings:** deterministic word-table composition from the
   node hash (the journal precedent) — flavor at the surface, hash as the
   identity underneath.
5. **The gates open here:** TECH-1 is the research system SINK-1 §1.4
   awaits — sink capture's research gate binds to a crystallized node;
   the throttle-α policy action lands beside it.

## 3. Condition table (v1, placeholder thresholds)

| condition (derived from receipts) | crystallizes | modifier |
|---|---|---|
| joule famine survived ≥ 8 years total | Cryo-Resonant Insulation | HeatingReduction 3/4 |
| overheat held ≥ 4 consecutive years | Overflow Discipline | EncasementBoost 5/4 |
| vault sink banked ≥ threshold with zero demurrage | Lattice Deepening | SinkBoost 5/4 |
| high vent ratio sustained (vented > encased, 4 years) | Radiator Doctrine | RadiatorBoost 5/4 |
| surplus held (banked ≥ 4× heating bill, 8 years) | Long Stores | DemurrageEase 1/2 |

One node per condition kind per orb (idempotent); prerequisites = the
condition's own history, not a parent-pointer web (parents emerge naturally:
you cannot survive 8 famine-years without first being cold).

## 4. Deferred, recorded

- **Synergy/shared nodes** (co-discovery across worlds): needs BOND-1's
  cross-world link — separate chronicles cannot co-sign a discovery yet.
- **Tech licensing/diffusion** (tag 40): rides TRADE-1 + escrow (01 §7)
  once blueprints have a transfer law.
- **Ideology input:** post-IDEO-1, the 09 cohesion counters may enter the
  condition table — read as landed state, never invented here.
- **Journal integration:** discoveries surface on the Terminal Wall as
  Milestone entries (mirror-side rendering, JOURNAL-1's law — automatic
  once the receipts exist).
