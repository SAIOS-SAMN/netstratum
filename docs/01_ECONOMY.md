# The economy — design stage

**Status:** design — no economy mechanic ships before its protocol sections
and test vectors exist (Charter §1), and no settlement mechanic opens before
landing N7 seals (the anchor bridge is the only lawful money boundary).
All percentages and prices in this document are **placeholder parameters**,
not fixed law — they tune at balance time.

**2026-08-07 expansion (in-game money + element commerce):** the playable
unit of account is **STAMP** (`32_STAMP_MARKET.md`) — exact-rational,
receipted, NPC-sustained; instruments are Share · Note · Future · Perp
(Notes ≠ ideology standing bonds). Element trade routes and derived
refineries: `31_ELEMENT_COMMERCE.md` (**REFINE-1 landed Wave-1**; routes
Wave-2/3 partial). Offices T4–T9 and R-line: `30_OFFICES_AND_RAILS.md`
(T-line landed; R1–R2 type partial). Interactive trade desk, fleets,
alignment-priced routes, civ-local capital books: `33_TRADE_FLEET_AND_DESK.md`
+ `archive/TASKS_TRADE_FLEET_DESK.md` (5 phases). External trade of STAMP claims
uses the **R-line** (R3 strand, R4 isthmus/datum). NS-7 remains the
boundary for **real-world** settlement; STAMP is in-game first and only
optionally exits the court rail.

The economic architecture follows the charter's split: gameplay is
high-frequency, free, and local (CFE ticks, receipted by NS-3); value
settlement is low-frequency and external (NS-7 consensus roots to WWW3
rails). Players never pay per-action fees; the chain never sees a game tick.

---

## 1. Domain registration (the in-game DNS)

A domain (`netstratum://lantern.prime`, `netstratum://outer-rim.trade`) is a
human-readable address bound to a DOP orb — a planet, station, market hub, or
entity profile.

- **Root domains** carry a tiered annual lease (shorter/premium names price
  higher), settled through NS-7.
- **Subdomains are free and unlimited** once the root is held
  (`cinder.lantern.prime`, `station-4.lantern.prime`) — managed locally by
  the owner's ipsal-os entity, no settlement event.
- **Resolution is protocol work:** a domain resolves to an orb address via
  NS-1; ownership and transfer are receipted via NS-3; the registry root
  anchors via NS-7 so ownership survives any single node.
- Revenue split (placeholder): treasury for development/infrastructure vs. a
  host pool rewarding operators of stable NS-5 verification nodes.

**Bindings:** NS-1 (address → orb), NS-3 (ownership receipts), NS-5 (host
rewards), NS-7 (settlement). First consumer at spec time: registering
`lantern.prime` and resolving it from a second node.

## 2. Planet leasing and colony zoning

Owners (landlords) lease tiles, orbital slots, or industrial zones to tenants
rather than selling whole worlds.

| Structure | Shape |
|---|---|
| **Fixed-yield lease** | tenant pays a periodic fee for a designated tile; predictable both ways |
| **Revenue share** | zero upfront; a fixed fraction of tile output routes to the owner automatically via receipt logic — the low-barrier entry path for new players |
| **Development speculation** | barren worlds lease at a discount to worldshapers; tile value follows measured improvement (atmosphere, water) |

The RFA property that makes this trustworthy: splits compute in exact
rational arithmetic — a 5% share of 1,000,003 units divides bit-for-bit
identically on every node. No party can be shorted by rounding, and disputes
reduce to replaying receipts.

**Bindings:** NS-1 (tile/zone orbs), NS-3 (the split is receipt routing),
NS-6 (lease terms live in the entities' permission grants), NS-7 (only if a
lease settles in external value; in-game-currency leases never touch the
chain).

## 3. Trade route fees and hyperlane tariffs

Goods move along graph edges between orbs. Infrastructure owners earn from
throughput:

- **Gate tariffs:** a player who builds an orbital transit gate or holds a
  hyperlane waypoint sets its import/export tariff within protocol-bounded
  limits.
- **Automated micro-splits:** when cargo clears a gate, the CFE splits the
  tariff at the tick (operator share / protocol reserve — placeholder ratio),
  each leg receipted.
- **Off-grid routes:** flying outside mapped hyperlanes avoids tariffs at the
  cost of transit time, hazard exposure, and no receipt trail for insurance —
  fee avoidance is a gameplay tradeoff, not an exploit.

**Bindings:** NS-1 (edges between orbs), NS-2 (route discovery is scanner
work), NS-3 (tariff splits), NS-5 (both endpoints attest the crossing).

## 4. Value flow

```
players ──(domain leases, tariffs, rents)──► treasury + node-host pool
   ▲                                              │
   └── free, receipted gameplay (CFE local) ◄─────┘
                        │
              NS-7 consensus roots
                        ▼
             WWW3 settlement rails
```

## 5. Binding guardrails (these are law, not parameters)

1. **Fun-first (Charter §2):** the core loop — explore, zone, build, trade —
   is fully playable with zero real-money involvement. Monetization attaches
   to scarce names, land tenure, and infrastructure throughput, never to
   game-rule advantage. No pay-to-win.
2. **Onboarding stays law (Charter §3.5):** a new player never needs a
   wallet. Settlement rails are opt-in for players who choose to hold
   externally-anchored assets.
3. **No wealth promises.** Netstratum is a game with a real economy attached
   at the settlement boundary, not an income scheme. Docs and marketing never
   describe playing as "earning real wealth" — that framing invites
   speculation-first players, re-creates Dark Forest's bot arms race with
   monetary stakes, and misrepresents risk.
4. **Bots earn under containment.** Automated agents may operate gates,
   fleets, and leases — inside NS-6 permission grants, generation-tagged, and
   subject to the same tariffs. Automation is an economic participant, never
   an unmeasured extraction path.
5. **Regulatory review before rails go live.** Any mechanic that settles in
   external value (stablecoins, tokens, fiat) gets legal review for the
   operating jurisdictions before N7's adapter touches a public chain.
   Design-stage documents (this one) are not offers.
6. **Choke points must be routable-around.** Any monetized gate or hyperlane
   must have a lawful (if costlier) alternative path — no player may hold
   absolute toll power over another player's only route. Measured in the
   world graph at balance time.

## 6. Thermodynamic currency & settlement architecture

The physical substrate of this economy is the thermal balance loop —
specified in full in `05_THERMAL_BALANCE.md`, the crypto boundary at its
§7. This section holds the economic mechanics; the physics and the crypto boundary
rules live there.

### 6.1 Energy as native commodity money

The economy operates on a physical energy standard derived from the exact
thermodynamic state variable `T_eq⁴`.

- **Supply and minting:** energy (embers, tracked as exact rationals in
  the CFE chronicle) cannot be minted by fiat or player action; its total
  volume and local density are governed strictly by sector thermal balance
  (`05_THERMAL_BALANCE.md` §1, §3). The minting mechanism is **syntropic
  encasement** (05 §1.2): embers enter the world only as excess stellar
  and factory heat captured by encasement shells — the Furnace mints, the
  Frost banks losslessly, warm stores leak.
- **Thermal arbitrage:** the intrinsic price differential between
  energy-rich Furnace sectors and heat-sink Frost sectors is the core
  macro-economic trade loop and the fundamental monetary axis. Transporting
  coolant toward high-flux stars and energy toward cryogenic compute
  sectors constitutes the primary monetary exchange between thermal
  regimes.
- **Frictionless movement:** in-game energy transfers, market orders, and
  local consumption remain completely CFE-native — zero gas, no external
  wallet (Charter §3.5; guardrail 2 holds).
- **The hard line (05 §7.1):** no in-game unit is ever a per-unit crypto
  asset. This extends guardrail 1 — monetization attaches at the settlement
  boundary only.

### 6.2 Macro-assets and the anchoring boundary (NS-7)

Everyday energy flows freely inside the CFE with no on-chain footprint;
only high-value economic claims cross the settlement adapter, under the
trigger conditions defined in `05_THERMAL_BALANCE.md` §7.2:

1. **Provably scarce worlds (ERC-721):** planets or moons whose `T_eq⁴`
   falls inside a substrate's equilibrium band anchor as persistent deeds.
2. **Thermal entropy rights (ERC-1155):** sector-level allocations for heat
   dumping and radiator capacity — transferable, tradeable rights.
3. **Inter-sector route contracts:** high-throughput, long-duration supply
   agreements between Furnace and Frost regimes anchor externally so
   ownership survives across empire boundaries.

These ride the same rails as §1 domain leases and §2 land tenure, under the
same guardrails 3 and 5. Settlement uses an existing chain, never a new one
(05 §7.4): the mesh is the world; an existing chain is the notary.

### 6.3 Separation of physics and valuation

Per Charter §7 and SEAL C, contract bytecode is strictly passive and
declarative — external tokens mirror immutable facts verified from the
chronicle root (the exact `T_eq⁴` rationals and band-membership facts),
provable by simple rational-comparison ZK circuits. Game-loop determinism
and zero-latency tick execution are preserved: the chain never computes,
only notarizes.

## 7. Neutral lithic escrow (Litho-Chalcogen nodes)

Diplomacy needs a neutral place to hold a treaty and its collateral —
somewhere neither party can quietly rewrite. In the world-book, the
Glasskeepers (Litho-Chalcogens, `06 §1.1`) provide it: deep-crust crystalline
matrices that vitrify under stress, freezing their held state. This section
fixes the *mechanic* behind that lore; the physiology stays flavour (`06`
register note).

### 7.1 Non-falsifiable state contracts

Multi-party treaties — a multi-decade coolant/energy supply pact, a
non-aggression zone, an entropy-headroom lease — may be committed to a
Litho-Chalcogen escrow node rather than a mutable surface store.

- **Vitrified execution:** on mutual signing, the treaty's transition
  function is encoded as an exact-rational state machine and the node vitrifies —
  the escrowed state becomes immutable in-fiction.
- **Exact-rational validation:** conditions evaluate strictly by exact rational
  comparison over CFE chronicle data (e.g. "Empire A delivered ≥ 1,000,000
  embers to Sector B within N ticks"). No float, no rounding, no subjective dispute
  window (Charter §3.2).
- **Dual-key collateral lock:** parties deposit collateral — in-game embers,
  coolant, or entropy rights, or NS-7-bridged ERC-721/1155 assets. While
  locked in the vitrified matrix it cannot be spent, moved, or re-pledged.
- **Settlement:** condition true → release to the recipient; thermodynamic
  breach (choking an agreed coolant stream, exceeding a sector's `T_eq⁴`
  heat-dump threshold) → `BREACH_AND_SLASH`, collateral forfeits to the
  aggrieved party.

### 7.2 The two binding guardrails (these bind the mechanic)

The escrow is powerful enough to break two charter walls if built naively;
both are closed here:

1. **The CFE is the arbiter, not the chain (Charter §7, SEAL C).** All
   release/slash logic runs in the CFE state machine. Any ERC-721/1155 asset
   in the pot is a *passive ownership record* only; the external contract
   verifies a proof and moves a token, it never evaluates treaty terms or
   game physics in bytecode.
2. **Destroying a node cannot erase truth (SEAL B).** The vitrified lattice
   holds a *copy* of the escrow state and the collateral claim, never an
   alternate ground truth. Physical destruction of an active node defaults to
   the recorded `BREACH_AND_SLASH` clause *in the chronicle* — it does not
   delete history or let a party escape a settled obligation. The chronicle
   stays sole ground truth; the lattice is a vault and a tripwire, not a
   second ledger.

### 7.3 Cross-boundary settlement (NS-7)

For treaties settling on an external chain (inter-empire deeds, tokenized
macro-routes), the vitrified escrow outcome anchors through NS-7 as a
succinct proof over the chronicle root — the *real* proof-system version, per
`05_THERMAL_BALANCE.md` §7.3 (a stark / multilinear prover over exact-rational
inequalities, not a boundary-stitch or topological invariant). The adapter
verifies one succinct proof of fulfilment-or-breach and executes the token
transfer; the chain never sees the treaty history or the physics.

### 7.4 Guardrail alignment

Escrowed choke mechanics (entropy-lease leverage, priority-proving swaps)
remain bound by §5 guardrail 6: no treaty may hand one party *absolute* toll
power over another's only route — a squeezed party always keeps a lawful,
costlier alternative. Escrow enforces agreements; it does not manufacture
inescapable monopolies.

## 8. Coolant arbitrage & sink capacity export (design stage)

The cold half of the thermal axis, dual to §6.1's ember minting — physics
specified at `05_THERMAL_BALANCE.md` §1.4; lands with the VENT-1/ECON-E
landing.

- **The commodity:** a dim-star system's export is **Banked Sink**
  (`S_cool`) — heat-absorption capacity in ember-commensurable units —
  shipped physically (coolant out, entropic fluid back, re-armed against
  the void) or leased as the §6.2 thermal entropy rights.
- **The books balance per side (05 §1.4):** importing embers increments
  banked embers; absorbing a neighbour's waste consumes banked sink and,
  once shed to the void, settles into the absorber's `entropy_vented`
  ledger (`VentSettled`, collect-settled — nothing un-receipted).
- **Earned, never passive (05 §1.4):** both captures gate through
  research → infrastructure → throttle `α ∈ [0,1]`; the uncaptured
  `(1−α)` settles as vented loss. Guardrails 1 and 6 hold: no gate is
  pay-to-win and no sink monopoly may be absolute.
- **The equilibrium:** the `S_cool / J` price ratio is the Furnace↔Frost
  exchange rate — cold systems import embers to live (ECON-E heating
  upkeep) and in exchange absorb the Furnace's thermal death. Placeholder
  pricing; tunes at balance.
