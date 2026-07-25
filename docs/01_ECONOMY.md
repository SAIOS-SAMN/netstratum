# THE ECONOMY — design stage

**Status:** DESIGN — no economy mechanic ships before its protocol sections
and test vectors exist (CHARTER §1), and no settlement mechanic opens before
landing N7 seals (the ANCHOR bridge is the only lawful money boundary).
All percentages and prices in this document are **placeholder parameters**,
not inscribed law — they tune at balance time.

The economic architecture follows the charter's split: gameplay is
high-frequency, free, and local (CFE ticks, receipted by NS-3); value
settlement is low-frequency and external (NS-7 consensus roots to WWW3
rails). Players never pay per-action fees; the chain never sees a game tick.

---

## 1. Domain registration (the in-game DNS)

A domain (`netstratum://sol.prime`, `netstratum://outer-rim.trade`) is a
human-readable address bound to a DOP orb — a planet, station, market hub, or
entity profile.

- **Root domains** carry a tiered annual lease (shorter/premium names price
  higher), settled through NS-7.
- **Subdomains are free and unlimited** once the root is held
  (`mars.sol.prime`, `station-4.sol.prime`) — managed locally by the owner's
  ipsal-os entity, no settlement event.
- **Resolution is protocol work:** a domain resolves to an orb address via
  NS-1; ownership and transfer are receipted via NS-3; the registry root
  anchors via NS-7 so ownership survives any single node.
- Revenue split (placeholder): treasury for development/infrastructure vs. a
  host pool rewarding operators of stable NS-5 verification nodes.

**Bindings:** NS-1 (address → orb), NS-3 (ownership receipts), NS-5 (host
rewards), NS-7 (settlement). First consumer at spec time: registering
`sol.prime` and resolving it from a second node.

## 2. Planet leasing and colony zoning

Owners (landlords) lease tiles, orbital slots, or industrial zones to tenants
rather than selling whole worlds.

| Structure | Shape |
|---|---|
| **Fixed-yield lease** | tenant pays a periodic fee for a designated tile; predictable both ways |
| **Revenue share** | zero upfront; a fixed fraction of tile output routes to the owner automatically via receipt logic — the low-barrier entry path for new players |
| **Development speculation** | barren worlds lease at a discount to terraformers; tile value follows measured improvement (atmosphere, water) |

The RFA property that makes this trustworthy: splits compute in exact
rational arithmetic — a 5% share of 1,000,003 units divides bit-for-bit
identically on every node. No party can be shorted by rounding, and disputes
reduce to replaying receipts.

**Bindings:** NS-1 (tile/zone orbs), NS-3 (the split IS receipt routing),
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

## 5. INSCRIBED GUARDRAILS (these are law, not parameters)

1. **Fun-first (CHARTER §2):** the core loop — explore, zone, build, trade —
   is fully playable with zero real-money involvement. Monetization attaches
   to scarce names, land tenure, and infrastructure throughput, never to
   game-rule advantage. No pay-to-win.
2. **Onboarding stays law (CHARTER §3.5):** a new player never needs a
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
