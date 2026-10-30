# 74 — The emergent economy: currencies, prices and trade from cognition

**Objective.** No money is issued by the physics. What a civilization
accepts in trade, at what ratio, and what it holds as money emerge from its
minds' decisions (docs/73 decide) over its own goods. This holds for a
civilization on planetary mass or in latent space. Value is learned, never
authored.

Status: scoped 2026-10-10 against the code; not built. It builds on docs/73
and completes the emergent-economy arc's unbuilt slices (NUM-1, CAPEX-1;
memory emergent-economy-arc).

## Facts (read from the code)

- **One fiat money.** STAMP is minted 1:1 from crystal
  (`cfe::world::mint_stamp_from_crystal`, `world/mod.rs:663`). Every line mints
  half its surplus crystal each beat (`stamp_market_step`). STAMP pays:
  - the hubs, the road and gate tolls;
  - gate costs (ascension), commission bounties;
  - the memory share (`assign_memory`).
- **Barter exists, with authored intents.** The book posts
  `OfferPosted { give: Cargo, give_amount, want: Cargo, want_amount }` (tag 49)
  and fills it with `OfferCleared` (tag 50). Cargo is any `Resource` or almanac
  `Element` (`dop.rs:608`). The intents are hardcoded: `element_want_intent`
  gives half the biggest stock (`lib.rs:442`). Every cleared ratio is a typed,
  chronicled receipt.
- **Elements are the planet's mass.** A species eats its staple from its veins
  (`staple_diet_of`, demand-driven, `StapleMined`); industry runs on
  non-staple veins and trade.
- **Latent space.** The substrata (docs/69, `substrata.rs`) are traversed by the
  ancients; no population lives there. Their "float" is authored: 1 for a
  first closure, a quarter after, spent only on opening tubes.
- **Rules.** The Concordance is the hypha's, for every kernel that joins it
  (`/opt/xylarium-kernel/docs/CONCORDANCE.md`, draft 3).
  - 5.4: tolls are charged per unit carried, on mass (S) and information
    (H), never on value.
  - 5.8: "Ordinary civilizations pay road tolls in their own money."
  - μ is the hypha's reserve, outside the universe (boundary 1).
  - Netstratum's "paid in STAMP" (docs/70 P2) was its own reading, not the
    Concordance's: the Concordance already asks for each civilization's own
    money.

## First principles

1. **Goods are what is scarce and held:** matter (the element and resource
   ledgers, measured in mass on the grain), hull capacity, memory (the minds'
   entry budget), and words (measured in `program_bytes`).
2. **A price is a cleared ratio** between two goods, read from `OfferCleared`
   receipts. Nothing else is a price.
3. **Money is a behaviour, not an issue.** A good is money for a civilization
   to the degree its lines accept it in order to give it on, not to consume
   or build with it. That is measured, never declared.
4. **Every trade decision is a decide kind** (docs/73): learned from receipts,
   scored before the outcome, with the current rule as the incumbent.

## Measures (typed, from the chronicle, no dials)

- **Price.** For each pair of goods, the last cleared ratio and its window of
  ratios, per hub.
- **Moneyness.** For each civilization and good, over the window: of the
  units received in trades, the share later given away in trades rather than
  consumed (`StapleMined`, build bills, hull scrap) or held.
- **A civilization's money** is its good of greatest moneyness, if that share
  exceeds every other good's. Two civilizations may hold different moneys.
- **The exchange rate between two moneys** is their cleared ratio at the hubs
  where both trade.

## The new decide kinds (docs/73's step, unchanged)

| kind | scope / subject | rows | features (ranked) | loss | incumbent | action while a word acts |
|---|---|---|---|---|---|---|
| offer | galaxy / line | candidate offers: each (give good, want good, ratio) whose ratio is one cleared for that pair in the window, or the line's own stock ratio | give stock, want need, the ratio's rank among the window's, the want good's moneyness | the offer rested unfilled to the window's end | `element_want_intent` and `stamp_market_step`'s halves | post the unmarked candidates; post none of the marked |
| accept | galaxy / line | goods offered to the line in payment this beat | the good's moneyness in the line's civilization, its price trend, the line's stock of it | accepted, then neither consumed nor given on within the window, or given on at a worse ratio | accept anything that clears | refuse marked goods in payment; accept unmarked |

The `offer` kind replaces docs/73's `amount` kind (which was written in
STAMP); `hull`, `envoy` and `gate` stay.

## Roles: the ancients think, the progenitors act (operator, 2026-10-13)

Free markets, with no intervention by decree. No class holds a market power
the others lack: every participant trades under the same laws, and influence
comes from position and holdings, never from a rule.

- **The ancients are pure intelligence.** They keep their cores' throats (the
  G3 contest, open to any lineage), seat the councils whose minds earn μ for
  the reserve, open the substrata, and speak the words the parliaments carry.
  They do not trade. An ancient line posts no offers, takes no bids, and
  carries no cargo of its own.
- **The progenitors act on the ancients' behalf.** The ancients' lineages, as
  their parliaments, are the ancient civilizations' hands in the economy:
  - they build and keep the gates and roads, and collect the road tolls;
  - they trade, from the holdings the ancients' standing earns;
  - at the hypha they gain μ from their toll shares (Concordance 5.8).

  What the ancients' intelligence decides reaches the markets only through
  them, as participants, under the same laws as every line.
- **Consequence for today's code.** Payments now made to an ancient (a gate's
  cost, `ascension.rs`) go instead to its lineage's progenitors, who hold them
  for it. An ancient line drops out of the hub's buyers and sellers. Both are
  changes to build, each with a done-when (below).

## The tiered hierarchy (operator, 2026-10-13; proposed for the Concordance)

One shared ladder of tiers for every universe the nodes create, so that a tier
can bridge between universes, and any two games can be interlinked through
wormholes or galactic cores. Proposed to the coordinator for the Concordance
charter; Netstratum builds to it.

| tier | who | where | responsible for |
|---|---|---|---|
| Ancients | the civilizations that ascend | the planets and systems nearest the galactic core | folding space and time at the galactic cores; designing the inner-galaxy wormholes; assigning progenitors. Pure intelligence: they act on the economy only through their progenitors |
| Progenitors | assigned by the ancients | the passage networks | building, keeping and operating passage. No hyperspace traversal without A progenitor assigned from the ancients. They collect the tolls, trade on the ancients' behalf, and hear the realms' reports |
| Stewards | govern realms | the boundaries between civilizations | diplomacy, trade, and sovereign disputes between civilizations; the realm is the authority over the moneys its peoples' trade produces, settling between them at their cleared ratios; they report up to the progenitors |
| Sovereigns | the realms' directives and operatives | the planets | acting on their planets as the realm directs |
| Captains | the lower level | fleets, states (a planet's), stardocks | the fleets, the planet states and the stardocks |

- **Interaction is tier to adjacent tier.** Captains ↔ sovereigns ↔ stewards ↔
  progenitors ↔ ancients; stewards interact with progenitors, progenitors with
  ancients. Reports and disputes go up; directives come down.
- **Every tier is a seat for a mind** (Concordance 3.8). A seat decides by
  decide (docs/73), by learned words, never by authored rules; the physics
  bounds every act.
- **Currencies, tokens and local economies** emerge from the local
  civilizations (above). The realm is the hierarchical authority over them;
  no tier issues money by decree.

### Against the Concordance as it stands (for the coordinator)

- **3.2.** "Progenitors … build the networks of highways and wormholes" versus
  the ancients design the inner-galaxy wormholes and assign the progenitors who
  build and operate them.
- **3.3.** A realm as "the shell of a sphere of influence that a passage network
  reached in one steward's term" versus a realm as the boundary between
  civilizations. The two may agree, with the boundary as the shell; the
  coordinator's to reconcile.
- **3.4.** Sovereigns "rule one people's worlds by that people's own lights"
  versus sovereigns act on planets as the realms' operatives.
- **3.5.** Captains "act from ships" versus captains control fleets, planet
  states and stardocks.
- **3.6.** Conflict goes up the chain: unchanged.

### Against Netstratum's code

| tier | today | to build |
|---|---|---|
| ancients | lines arisen at their core's bulge (by birth, or ascended by deed); they trade at hubs, and gates are folded by the parliaments (docs/72) | the ancients choose the inner-galaxy wormholes; their lines leave the markets (Roles, below) |
| progenitors | the keeping lineage's lines as parliaments; envoys cross without an assigned progenitor | every traversal names a progenitor the ancients assigned, or it is refused |
| stewards / realms | none (`Polity.steward` is a per-line actor, not a realm's keeper) | realms derived from the boundaries between civilizations; a steward seat per realm; realm settlement and reports |
| sovereigns | the lines (peoples) | sovereign seats acting on planets under their realm |
| captains | the hulls and offices | captain seats over fleets, planet states and stardocks |

## The constants, and their way out

Every static constant in the economy is either derived from physics or
learned by decide (docs/73). Until then it is declared, and visible here and
in docs/59.

| constant | where | way out |
|---|---|---|
| STAMP minted 1:1 from crystal | `mint_stamp_from_crystal` | retired (below); money emerges |
| "half" in offers, bids and the mint | `stamp_market_step`, `element_want_intent` | learned: the `offer` and `accept` kinds |
| one uniform clearing price per hub | `stamp_market_step` | derived: each pair's cleared ratio on the barter book |
| fleet capacity 4 | `fleet_step` | learned: the `hull` kind |
| gate cost 7 STAMP | `ascension::gate_cost` | learned: the price the progenitors' offer for passage clears at |
| gate melt and passage toll, span × 1/64 | `ascension::gate_melt` | derived: the melt from distance and technology, as roads already are (`route_melt`); the toll learned |
| gate freshness 1 cell | `ascension::FRESHNESS_CELLS` | derived: the bulge, one cell |
| Window 64 | docs/73 | declared; a memory bound |
| the float's 1 and ¼ | `substrata` | learned, when a population lives below |

## θ, μ and the free market (operator, 2026-10-13; docs/81)

- **Traversal inside the universe** (wormholes and tubes, the ancients'
  folding of space and time) is paid in θ, the phase, in turns. Galactic
  traversal between universes over the hypha is paid in μ. Nothing inside
  the universe is paid in μ.
- **The progenitors hold both μ and θ:** μ from their toll shares at the
  hypha, θ from the passage they carry.
- **Local economies earn θ** by converging their timelines and reducing
  their collisions. θ is minted only by convergence and lost by collision.
- **The free market stands.**
  - θ may be used everywhere, and is required only for traversal.
  - Each civilization's currency remains an emergent property of its own
    trade (above).
  - θ is obtained by earning it or by trading for it at cleared ratios,
    never by decree.
- **Underneath:** realms' and stewards' resource utilization, the trade
  networks of element harvesting on local star systems, feeds up the chain.

## STAMP's retirement, and what pays obligations

1. **Settle in kind.** The road toll on mass is paid as a share of the mass
   carried, to the road's keeper or builders, in the cargo itself (5.4's "on
   mass" kept; 5.8 already says "in their own money").
2. **Denominate in the payee's money.** Gate costs, bounties and passage
   tolls are owed in the payee civilization's money, converted from their
   current STAMP value at the hub's cleared rate. Until decide learns a price
   for an obligation, that conversion is its incumbent.
3. **The memory share** reads wealth as the sum of a line's holdings valued at
   cleared prices in its civilization's money, not STAMP.
4. **The mint stops** once no obligation is denominated in STAMP: the reading
   is STAMP minted per beat → 0. Existing balances stay as one good among
   others; their moneyness decides whether STAMP stays a money anywhere.

## Latent space

The same law applies to any population that holds goods and trades. In the
substrata the scarce goods are traversals (tasks closed) and words. When a
population lives there (none does today), its float is whatever its lines
accept in order to give on, measured by the same moneyness. Until then, the
float's authored 1 and ¼ stand as the incumbent, and are named as authored.

## Build order and done-when (the live log, before → after)

Builds after docs/73 steps 1–3 (capture, learn, act).

1. **Measures.** Price, moneyness, and each civilization's money computed
   from receipts and logged each bounce; behaviour unchanged.
   - Done-when: moneyness per civilization and good: no reading → a reading.
   - Witness: the prices agree with the `OfferCleared` receipts' ratios.
2. **offer.**
   - Done-when: barter clears per beat (`OfferCleared`), today's count →
     rising.
3. **accept.**
   - Done-when: civilizations whose money is not STAMP, 0 → > 0; distinct
     moneys across the universe, 1 → > 1.
4. **Obligations.** Tolls in kind and obligations in the payee's money (as
   Concordance 5.8 already asks).
   - Done-when: STAMP minted per beat → 0, with tolls and bounties still
     paid. Witness: the payees' holdings.
5. **Roles.** Ancient lines leave the hub's buyers and sellers, and payments
   to ancients go to their progenitors.
   - Done-when: ancient lines trading at hubs, today's count → 0; payments
     received by ancient lines → 0, and the progenitors' receipts rise by the
     same amounts. Witness: the chronicles' receipts.
6. **The hierarchy.** Realms derived; steward seats; every traversal with an
   assigned progenitor; the ancients choosing the inner-galaxy wormholes.
   - Done-when: realms per universe 0 → > 0, each with its settlement and its
     report; traversals without an assigned progenitor → 0 (refused).
     Witness: the progenitors' receipt of the realms' reports.
7. **Latent space**, when a population lives in a substratum.
   - Done-when: its float's moneyness → a reading.

## Authored, stated plainly

- What counts as a good.
- The moneyness measure.
- The candidate ratios: those cleared in the window, plus the line's own
  stock ratio.
- The two action rules.
- The incumbents until replaced.
- docs/73's window.

## Dependencies

- **docs/73 steps 1–3.**
- **docs/70 P2 corrected:** tolls in the civilization's own money, as
  Concordance 5.8 states.
- **μ stays outside the universe** (boundary 1): nothing here converts to or
  from μ.
