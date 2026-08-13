# 49_TAXATION — THE CRAFTABLE TAX SYSTEM

*Translated from operator directive 2026-08-13. A tax is not a hardcoded
drain — it is a RULE a civilization crafts: what it draws, when, from whom,
at what rate, and where it flows. The engine already has ONE tax
(demurrage, a fixed decay on standing stock); this generalises taxation
into craftable data under diplomacy, with the continuity tax (#44) and the
estate tax as its first two instances.*

## 0. Why a system, not two more hardcoded drains

Demurrage (`DemurrageTouched`, tag 102) is a tax: it settles at the touch,
receipted, decaying idle stock. But it is the ONLY tax, fixed in code. A
civilization-scale economy needs taxation that is TARGETED (knowledge,
estate, trade — not just stock), TRIGGERED (per-beat, per-turnover, per-
treaty), and CRAFTED (the sovereign designs its levies). Building the
continuity and estate taxes as two more bespoke branches would repeat the
`disposition == N` pigeonhole this project just spent a session dismantling.
So taxation becomes DATA, one shape over many levies.

## 1. A TAX IS DATA (the practice / vocabulary precedent)

A LEVY is a factor-set over the landed channels, exactly as a practice is
(`28_VOCABULARY`, the practice-channel ruling). One levy declares:

- **base** — what it draws on: `Stock` (like demurrage), `Estate` (banked
  value + standing advantages at turnover), `Knowledge` (crystallized boons
  / adopted practices), `Trade` (flows across a treaty).
- **trigger** — when it fires: `Touch` (the settle seam), `Turnover` (a
  lifespan cycle completing — the `42_LIFESPAN` mortality settle), `Treaty`
  (a bond/transfer crossing a border).
- **rate** — the fraction drawn, fourfold-banded like every chartered factor.
- **sink** — where it flows: `Sovereign` (to the seated authority),
  `Void` (it simply lapses — demurrage's own sink), `Counterparty` (the
  other side of a treaty).

A levy is MINTED like a vocabulary entry (one benefit, one friction,
banded, novelty-gated) and settles through the existing receipt seam — no
new conservation counterparty where the sink is `Void`, a transfer where it
is `Sovereign`/`Counterparty`.

## 2. THE TAX WORKBENCH — crafted under diplomacy

A new bench kind (`BenchKind::Levy`) on the workbench (`43_DISTRIBUTION §7`,
landed): the sovereign DESIGNS its levies and READS the standing tax
system — each levy's base, trigger, rate, sink; its measured yield; its
incidence (who pays). It sits under DIPLOMACY because a tax is an imposition
between parties — sovereign upon subject, polity upon polity across a
treaty — and the treaty rails (`TreatyKind`, the diplomacy layer) already
model the party relation a `Treaty`-triggered levy rides.

## 3. THE CONTINUITY TAX (#44, `42_LIFESPAN §3`)

The first instance. A lineage's standing advantages — crystallized boons,
adopted practices — must be RE-CONFIRMED within a lifespan or LAPSE. It is
demurrage applied to KNOWLEDGE rather than stock, on the same settle seam:

- **base** `Knowledge`, **trigger** `Turnover`, **sink** `Void`.
- A LONG-lived line barely turns over, so it re-confirms cheaply and
  ACCUMULATES — the Custodian's deep, retained law.
- A SHORT-lived line turns over fast; each generation must re-earn what the
  last knew or it decays — the Locust's fast, forgetful churn.

This couples to the `growth_of` continuity axes (TIME / TESTING): a line
that pays little continuity tax deepens them; one that pays much sees them
bleed.

## 4. THE ESTATE TAX

The second instance, on ENDURANCE. When a generation turns over (a lifespan
cycle completes at the mortality settle), the ESTATE handed to the next —
banked value plus standing advantages — is taxed at the handover:

- **base** `Estate`, **trigger** `Turnover`, **sink** `Sovereign`.
- A line that PERSISTS past its lifespan pays estate tax at each turnover —
  the recurring cost of endurance, a brake on unbounded accumulation.
- A short-churn line hands over small estates often; a long-lived one hands
  over large estates seldom. Lifespan sets the estate-tax cadence, so
  `42_LIFESPAN` is the clock for both §3 and §4.

Continuity tax (knowledge lapses to Void) and estate tax (value flows to the
Sovereign) are the two faces of turnover: what a generation KEEPS versus
what it OWES on passing it on.

## 5. SETTLE DISCIPLINE

- Every levy settles at its trigger through the existing receipt seam —
  `Touch` levies beside demurrage/rejuvenation, `Turnover` levies at the
  mortality settle (`42_LIFESPAN`), `Treaty` levies at the bond crossing.
- A `Void` sink owes no counterparty (demurrage's admitted decay); a
  `Sovereign`/`Counterparty` sink is a receipted transfer.
- A world with NO crafted levy pays only demurrage, exactly as today —
  byte-identical. Taxes are additive craft, never a silent new drain.

## 6. RULINGS OWED

1. **The levy data model** — the exact `base` / `trigger` / `rate` / `sink`
   enums, and whether a levy is a minted vocabulary entry or its own deed.
2. **The crafting gate** — who may mint a levy (a seated sovereign? a T0
   progenitor, like a vocabulary mint?), and its fourfold rate band.
3. **The continuity decay** (§3) — how fast unconfirmed knowledge lapses and
   exactly WHAT lapses (which boons/practices), the `42_LIFESPAN §5` ruling.
4. **The estate base and rate** (§4) — what counts as the taxed estate
   (banked stock only, or advantages too) and the handover rate.
5. **The workbench scope** — the `Levy` bench this layer, vs a broader
   diplomacy bench (treaties + levies together) up front.

## 7. What this composes with

- `DemurrageTouched` (tag 102) — the settle-at-touch tax precedent and the
  `Void`-sink model.
- `28_VOCABULARY` / practices — the levy-as-craftable-data mechanism.
- `43_DISTRIBUTION §7` workbench — the crafting/inspection surface.
- `TreatyKind` / diplomacy — the party relation a `Treaty` levy rides.
- `42_LIFESPAN` — the turnover clock for the continuity and estate taxes.
- `growth_of` — the continuity axes the continuity tax feeds.
