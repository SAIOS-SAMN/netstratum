# 49_TAXATION — The craftable tax system

*Translated from operator directive 2026-08-13. A tax is not a hardcoded
drain — it is a rule a civilization crafts: what it draws, when, from whom,
at what rate, and where it flows. The engine already has one tax
(demurrage, a fixed decay on standing stock); this generalises taxation
into craftable data under diplomacy, with the continuity tax (#44) and the
estate tax as its first two instances.*

## 0. Why a system, not two more hardcoded drains

Demurrage (`DemurrageTouched`, tag 102) is a tax: it settles at the touch,
receipted, decaying idle stock. But it is the only tax, fixed in code. A
civilization-scale economy needs taxation that is targeted (knowledge,
estate, trade — not just stock), triggered (per-beat, per-turnover, per-
treaty), and crafted (the sovereign designs its levies). Building the
continuity and estate taxes as two more bespoke branches would repeat the
`disposition == N` pigeonhole this project just spent a session dismantling.
So taxation becomes data, one shape over many levies.

## 1. A tax is data (the practice / vocabulary precedent)

A levy is a factor-set over the landed channels, exactly as a practice is
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

A levy is minted like a vocabulary entry (one benefit, one friction,
banded, novelty-gated) and settles through the existing receipt seam — no
new conservation counterparty where the sink is `Void`, a transfer where it
is `Sovereign`/`Counterparty`.

## 2. The tax workbench — crafted under diplomacy

A new bench kind (`BenchKind::Levy`) on the workbench (`43_DISTRIBUTION §7`,
landed): the sovereign designs its levies and reads the standing tax
system — each levy's base, trigger, rate, sink; its measured yield; its
incidence (who pays). It sits under diplomacy because a tax is an imposition
between parties — sovereign upon subject, polity upon polity across a
treaty — and the treaty rails (`TreatyKind`, the diplomacy layer) already
model the party relation a `Treaty`-triggered levy rides.

## 3. The continuity tax (#44, `42_LIFESPAN §3`)

The first instance. A lineage's standing advantages — crystallized boons,
adopted practices — must be RE-confirmed within a lifespan or lapse. It is
demurrage applied to knowledge rather than stock, on the same settle seam:

- **base** `Knowledge`, **trigger** `Turnover`, **sink** `Void`.
- A long-lived line barely turns over, so it re-confirms cheaply and
  accumulates — the Custodian's deep, retained law.
- A short-lived line turns over fast; each generation must re-earn what the
  last knew or it decays — the Locust's fast, forgetful churn.

This couples to the `growth_of` continuity axes (time / testing): a line
that pays little continuity tax deepens them; one that pays much sees them
bleed.

## 4. The estate tax

The second instance, on endurance. When a generation turns over (a lifespan
cycle completes at the mortality settle), the estate handed to the next —
banked value plus standing advantages — is taxed at the handover:

- **base** `Estate`, **trigger** `Turnover`, **sink** `Sovereign`.
- A line that persists past its lifespan pays estate tax at each turnover —
  the recurring cost of endurance, a brake on unbounded accumulation.
- A short-churn line hands over small estates often; a long-lived one hands
  over large estates seldom. Lifespan sets the estate-tax cadence, so
  `42_LIFESPAN` is the clock for both §3 and §4.

Continuity tax (knowledge lapses to Void) and estate tax (value flows to the
Sovereign) are the two faces of turnover: what a generation keeps versus
what it owes on passing it on.

## 5. Settle discipline

- Every levy settles at its trigger through the existing receipt seam —
  `Touch` levies beside demurrage/rejuvenation, `Turnover` levies at the
  mortality settle (`42_LIFESPAN`), `Treaty` levies at the bond crossing.
- A `Void` sink owes no counterparty (demurrage's admitted decay); a
  `Sovereign`/`Counterparty` sink is a receipted transfer.
- A world with no crafted levy pays only demurrage, exactly as today —
  byte-identical. Taxes are additive craft, never a silent new drain.

## 6. Rulings owed

1. **The levy data model** — the exact `base` / `trigger` / `rate` / `sink`
   enums, and whether a levy is a minted vocabulary entry or its own deed.
2. **The crafting gate** — who may mint a levy (a seated sovereign? a T0
   progenitor, like a vocabulary mint?), and its fourfold rate band.
3. **The continuity decay** (§3) — how fast unconfirmed knowledge lapses and
   exactly what lapses (which boons/practices), the `42_LIFESPAN §5` ruling.
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
