# 40 — PLANETARY REJUVENATION (the regrowth law)

**Status:** DESIGN, ruled 2026-08-12 (operator, Gemini relay). Growth
form RULED: **(B) rational sigmoid on a grain** (§4) — preserve the
S-curve so collapse from near-zero is genuinely expensive to recover.
Charter-law translation of the relayed design; **not** a transcription.
Task #28.

**§6 addendum, 2026-08-13** (operator, direct): the ration this law
governs is generalized off biomass (landed, commit `f467876` — `K`/`r`/
`habitability` now read the sitting line's own metabolism and clade,
not a hardcoded biomass/Mesic pair). §6 below is a PROPOSED ruling,
five pillars pre-authorized, extending the same law to a ration that is
NOT privately banked but belongs to the planet itself — with the
public-externality and counter-play consequences that implies. Not yet
implemented; land as its own task once the open forks in §6.3 close.

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

## 6. THE MEDIUM RATION — public environmental externalities (PROPOSED)

### 6.0 Why

§1–§5 price a **private** ecological failure: a Locust who strips their
own world pays for it with their own recovery time, on their own
ledger. Nothing so far prices a **public** one. A plasmic line's food
is not a farmed crop — per `dop::metabolic_harvest(3)` it is captured
heat, and per `docs/36_DEFENSE_AXIOMS.md` §3.3 a plasmic line's own
"atmosphere" is its local field/density, not gas. Pushed one step
further, a plasmic line eating a gas giant's helium is drawing down a
stock that is not its infrastructure's harvest at all — it is the
**planet's own primordial matter**, the same matter every OTHER line
sited nearby reads for its own habitability.

That distinction is the design payoff: a Plasmic Locust doesn't only
ruin its own economy, it can freeze out a biotic neighbor who never
consented to the trade. This is meant to elevate extraction from a
solitaire optimization into the union's actual political axis —
treaties, embargoes, and wars fought over EXTRACTION RATES, not only
territorial borders.

### 6.1 RULED pillars — operator-authorized 2026-08-13

Five architectural pillars, pre-authorized before the counter-play
question was resolved (§6.2) and before any code lands:

1. **The Medium ration.** A new ration carrier — sibling to
   `dop::Resource` and the minted-element `Ration::Element` — that is
   read and written on the orb's own site record, never on a player's
   `banked`/`elements` ledger. Extracting it debits the PLANET; only
   the player's own cut (if any survives the physics) reaches their
   bank.
2. **Primordial `K`.** Where §3's `K` derives from BUILT footprint
   (`living_zones_of` — habitats and regenerators, infrastructure a
   player raised), the Medium's `K` derives from the orb's FORMATION
   physics — star class, orbital distance, and (new) some reading of
   the orb's own mass/kind (a "giant" vs a rock) — never from zones
   built on it. A world's primordial ceiling is not something a player
   grows.
3. **`r ≈ 0` — geological timescale.** The Medium's intrinsic rate is
   effectively zero within any playable era. `grained_logistic` needs
   no new form for this (an `r` near zero just barely creeps — see §4's
   surrogate); what changes is only the DERIVATION `r` reads, from a
   habitat's fast biotic yield to a near-null geological one. A
   stripped gas giant does not come back.
4. **Dual-arm conservation.** The mutating settle-at-touch deed (the
   piece task #28 already stopped short of, biomass or not) gains a
   SECOND kernel conservation arm here: it must prove the debit off the
   orb's site record and the credit onto the extractor's bank as one
   matter-conserving transfer, not two independently-balanced mints.
   This is stricter than the biomass case, where the grown stock
   mints into existence at the world's OWN account with one arm.
5. **Thermal drift feedback.** The depleted Medium level feeds back
   into the region computation, so extraction can shift a world's own
   `ThermalRegion` (per the ATM axiom: strip the medium, lose the
   insulation, the band drifts). This composes for free with today's
   landing: `almanac::habitability(clade, region)` already reads
   region generically (commit `f467876`), so once a region can drift,
   every clade's own habitability re-derives from the SAME read with
   no new cross-wiring.

### 6.2 The counter-play — RESOLVED 2026-08-13: system-level physics coupling

The open question before any of §6.1 can be implemented: `World` is
one sovereign's entire domain (`state.rs`) — one `star`, one
`orbs: Vec<ColonyOrb>`, one taxon, one genesis metabolism. Other
sovereigns are separate `World`s, reachable only through the typed
`Treaty` layer (`world/diplomacy.rs`, `partner: &str`). Orbs are
**single-tenant** — there is no shared orb today, so pillar 5's drift,
taken alone, only ever touches the extracting sovereign's OWN
habitability. That is still just the private case with extra steps.

Two more literal readings were considered and set aside for THIS
ruling:

- **Trade/treaty-mediated only** — cheapest, reuses `world_quarantined`
  and `treaty_elements` almost verbatim, but the harm to a neighbor
  stays purely economic/legal (lost supply, breach grounds), never a
  number moving on their own world. Rejected as too weak a payoff for
  what was pitched.
- **True shared/condominium orbs** — the literal reading (rival species
  standing on the same rock), but it requires zone-level ownership,
  per-tile taxon/metabolism, and multi-actor `K`/habitability reads —
  a foundational rewrite of `ColonyOrb` and every `taxon_economy_of`
  call site. Rejected as out of scope for this ruling; may be worth
  its own SEAL later.

**RULED: system-level physics coupling.** Orbs stay single-tenant, but
a star system is widened to host **multiple sovereigns' orbits around
one shared star** (a concept that does not exist yet — today's `star`
is private to one `World`). A Medium strip on one sovereign's orbit
contributes to a shared SYSTEM perturbation term (dust, radiation,
magnetospheric disruption — the ATM domain's own vocabulary), and that
term is read into `t_eq4` for every OTHER orbit around the same star,
sovereign or not. A neighbor's `ThermalRegion` can drift from a
rival's action with **no consent and no treaty required** — which is
what gives the political layer a genuine, involuntary grievance to
organize around, not just a bilateral inconvenience either side could
have simply declined to sign.

Once the harm is real and chronicle-visible, the counter-play composes
from precedent already standing:

- **Diplomatic** — a new treaty annex, sibling to `treaty_elements`
  (E3.9): a rate-CAP annex on a live Trade pact between co-signed
  system-mates, capping a partner's Medium extraction rate the same
  way an element allow-list caps what crosses. Consensual, dies with
  the pact it rides, composes by union — the existing annex shape,
  reused, not reinvented.
- **Legal** — breaching a live rate-cap annex is a treaty breach, and
  the existing War/Peace displacement law (§1 of `12_UNION`-adjacent
  diplomacy code, `TreatyKind::War` vs `Trade`) already makes a breach
  lawful grounds for war. No new pact kind needed.
- **Economic** — a declared war already walls trade (peace binds
  nothing; war walls everything, the `element_barred`/displacement
  shape). That alone is real teeth: the offender loses market access
  to everyone it angered.
- **Kinetic — OPEN, not ruled here.** Whether a declared war should
  unlock a NEW mechanic to strike an enemy's Medium-extraction zones
  directly (extending `strike_interdiction`'s cargo-in-transit
  precedent, `defense.rs`, to standing infrastructure) is real scope:
  today `raze`/`raze_as` (`world/core.rs`) is self-service only — no
  mechanic anywhere lets one `World` destroy another `World`'s zone.
  "Wars fought over extraction rates" wants more than a trade wall to
  have real teeth; this needs its own ruling and is NOT authorized by
  this ruling.

### 6.3 What is NOT ruled yet — open forks before code

- **The system perturbation term itself.** How a stripped orbit's
  Medium level enters a sibling orbit's `t_eq4` needs its own
  derivation, the same weight of design `t_eq4`'s luminosity/albedo/
  orbit_r composition already got — not a bare tuned literal (the
  tuned-constant ratchet, `tests/tuned_constant_ratchet.rs`, is
  already red against 112 and only falls; this cannot add to it).
- **Where a multi-sovereign system lives in the data model.** `World`
  currently owns its `star` privately. A shared star spanning several
  `World`s is new state that belongs to neither `World` alone — likely
  a mesh-layer concern given the standing mesh-second build order, and
  needs its scope pinned (kernel-local proxy vs true mesh object)
  before any code starts.
- **The rate-cap annex's wire shape.** Its `ClosureEvent` tag and any
  new `Ration`/`TreatyKind` wire codes are shared registers (the
  numbered manifest at the top of `dop.rs`) and must be CLAIMED there
  before landing, per standing register-claim discipline.
- **The kinetic escalation** flagged in §6.2 — its own ruling.
- **`r`'s exact near-zero value.** "Effectively zero" needs a cited
  anchor (`cfe::anchor`, typed, no bare `measured()`), not a hand-picked
  fraction — per the anchor-provenance ruling, every number derives,
  measures, or mints; this one still needs to pick which.

### 6.4 What this composes with (in addition to §5)

- `36_DEFENSE_AXIOMS` §3.3 and the ATM domain table — the
  species-relative "atmosphere" and the "vacuum strip → band shift"
  consequence this ruling implements in engine terms.
- `37_IMPERIUM_TAXON` — `habitability(clade, region)` is the read-through
  channel for §6.1 pillar 5; no new cross-wiring needed, it already
  reads region generically.
- `world/diplomacy.rs` (`treaty_elements`/`element_barred`) — the direct
  precedent for the rate-cap annex in §6.2.
- `world_quarantined` (`science_lab.rs`) — the self-scoped-signal
  precedent this ruling extends into a genuinely public one.
- The tuned-constant ratchet and the anchor-provenance ruling — gate
  every literal §6.3 still owes.
