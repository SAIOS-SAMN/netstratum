# 20 — THE FOUNDING MARKET

**Status:** LAW (operator ruling, 2026-08-02). Subordinate to
`00_CHARTER.md`. Supersedes the founding half of
`04_CIVILIZATION_GENESIS.md` §4 and extends `01_ECONOMY.md` §2.

---

## §0 THE SUPREMACY CLAUSE

> **"My word is stronger than any law or word written down. THIS is the
> law — if written rules/laws violate this trajectory, then THE LAWS are
> wrong."**

Inscribed law is a RECORD of rulings, never a source of authority
against one. Where a document contradicts a standing ruling, **the
document yields and is struck** — it is not weighed against the ruling
and it does not get to argue from seniority. Charter §11's one-truth
discipline still binds the correction: the old text is struck in place
with its reason, never silently rewritten.

**This clause does not suspend the KERNEL.** A ruling changes what the
game is; it does not change what a receipt means, and `lawful` still
refuses an unlawful step from anybody. The operator directs the world;
the operator does not get a bypass on arithmetic. That is the same
constraint `19_REGISTRY` §4d puts on the ancient's seat, applied to the
seat that writes the rulings.

## §1 FOUNDING IS AN ACQUISITION, NEVER A DERIVATION

**The flow, as ruled:**

```text
1. new user spins up the code
2. "FOUND NEW CIVILIZATION"
3. offered a LIST OF TYPES; picks the one most interesting to them
4. acquires a place to stand — by one of the three paths in §2
5. the founding TITHE fires (§3) — this is the baseline economy's
   first transaction, and the economy starts HERE
```

**What this strikes.** Founding today DERIVES a world from metabolism:
`derived_siting` draws a star, albedo and orbit from the line's
metabolism plus a beacon, and commissions it. **That is now the FALLBACK
for an uncommissioned world, never the mechanism.** A civilization gets
its world by acquiring one. The derivation survives only as the default
seed for a universe with nothing yet to buy.

**The choice is the player's, not the schema's.** Type is chosen from a
list because it is interesting, not assigned because a metabolism
implied a temperature. The axes of `04` §1 (disposition, metabolism,
perception) are what the list is drawn FROM; they stop being the thing
that decides where you land.

## §2 THE THREE ACQUISITION PATHS

### 2a. PURCHASE — a plot, a planet, or a share of one

In a KNOWN universe where civilizations already stand, worlds and parts
of worlds are for sale. **A share of a planet is a first-class outcome**,
not a consolation: a planet hosts more than one colony, and buying a
piece of one is a normal founding.

This rides law that already stands. `01_ECONOMY` §2 seats three lease
structures, and its **revenue share — zero upfront, a fixed fraction of
output routed by receipt** — is named there as *the low-barrier entry
path for new players*. **That is the founding path for a user with no
capital, and it is the reason this whole section does not violate
CHARTER §3.5.** The market primitives stand: `cfe::world::post_offer`,
`cfe::world::offers`, `cfe::world::clear_trade`, `cfe::world::transfer_as`.

### 2b. COMMISSION — pay a progenitor to move stars and suns

If no existing world suits the founder, or none is palatable, **the
founder pays a PROGENITOR CLASS to build them one** — to move the stars
and suns and raise a new solar system or world to order. **This must be
possible.**

This is the progenitor seat doing literally what it is defined to do:
`12_UNION` §2a seats progenitors as the ones who *make the room everyone
else moves on*. Commissioning makes that a SERVICE with a price, and it
gives the 8-seats-per-domain band an economic reason to be scarce.

**The acceptance criterion is VIABILITY (§4), not delivery.** A
commissioned world that lands its buyer outside every band they can live
in is not a delivered commission.

### 2c. FALLBACK — the derived seat

A universe with nothing to sell still founds. The derivation of §1 seeds
it, subject to §4.

## §3 THE TITHE — the economy's genesis transaction

**Founding pays a tithe to the ancients or the elders.** This is the
first economic event in a civilization's life and the point at which the
baseline economy starts turning.

**Who receives it follows the ward-scope ladder** (`19_REGISTRY` §4a) —
the tithe goes to whoever wards the scope the founding lands in:

```text
founded on an existing planet     → that planet's ELDER   (planetary ward)
founded into a newly raised world → the ANCIENT           (universal ward)
```

That rule closes the virgin-universe question rather than leaving it: a
brand-new world has no elder yet, so its first founding tithes upward to
the scope that does exist.

**And it makes ward scope pay.** An elder or ancient is no longer a
title with a vantage — it is a seat with REVENUE, funded by the
foundings inside its ward. This is what gives an ancient a standing
reason to open a room (`19_REGISTRY` §4b): more room admits more
founders, and founders tithe.

**The tithe is IN-GAME VALUE.** It settles in the resources of
`cfe::dop::Resource`, on the rails `01_ECONOMY` §2 already reserves for
in-game-currency leases, which *never touch the chain*. **A new player
still needs no wallet, and the tithe is not pay-to-win** — it buys a
place to stand, never a rule advantage (`01_ECONOMY` §5.1, §5.2, intact).

**THIS DRAFT'S OWN LINE HERE IS STRUCK** (operator, 2026-08-02). It held
"the tithe RATE, whether one-time or recurring, who sets it" for an
operator ruling. **There is no rate to rule.** The tithe is not a
percentage anybody sets — it is a continuous accrual against real
holdings, derived at the touch. See §6a, which builds the algorithm
instead of asking for the number.

## §4 THE VIABILITY LAW — every offered type must be able to flourish

> **"Any civ that can be brought online — new or recurrent — must be
> accounted for: hot, cold, thermal, plasma. These all need capacity to
> grow and flourish."**

**If a type appears on the founding list of §1, a viable founding site
for it must exist and be reachable by §2.** A type the game offers and
cannot house is a defect in the game, never a hard lesson for the
player.

**What this strikes, measured rather than asserted** (pins in
`engine/cfe/tests/siting_determinism.rs`, both green):

```text
band  Cryogenic     1.000e4  .. 1.000e8   K⁴
band  Mesic         3.906e9  .. 1.049e10
band  HyperThermal  1.296e11 .. 5.063e12

derived furnace seats (mineral | plasmic):
      1.212e10 · 1.333e10 · 1.454e10   ⇒ BAND = NONE, all three
```

**A mineral or plasmic line derives into the GAP between bands** — past
the mesic ceiling, an order of magnitude short of the hyperthermal
floor. It founds into a temperature no substrate is defined for. **Under
this section that is now a DEFECT and the derived furnace seats are
struck as a founding default.**

Two things follow, and both are engineering rather than doctrine:

1. **The band set must cover what the game offers.** Three bands leave
   two uncovered stretches (100–250 K and 320–600 K in temperature).
   Either the bands close over the offered types or the sites move
   inside them. **The operand exists; the coverage does not.**
2. **`cryo` must be foundable.** No derivation reaches a cold world
   today, so a cold line cannot be founded at all. Under §2 it becomes
   reachable the ordinary way — purchased, or commissioned from a
   progenitor — and that is the intended fix, not a widening of the
   derivation.

**Recurrent lines are covered by the same words.** A civilization
brought back online is accounted for exactly as a new one: it must land
somewhere it can live.

## §5 THE PURSE — a civilization's economy, held together

**Ruled 2026-08-02.** A new player needs no WALLET (external, deferred —
§8), but a civilization needs a **PURSE**: the thing that holds its
economy together across everything it owns.

**Verified gap:** stores live per-ORB. `Resource` balances hang off
colony orbs, and **there is no civilization-level economic container in
the engine at all** — no purse, no treasury, no coffer, zero hits. A
civilization SPANS worlds (`04` §5) and its economy does not add up
anywhere.

**THE PURSE IS A READING, NEVER A SECOND VAULT.** A purse that stored
balances would be a second readable truth over the orbs that already
hold them — the exact drift Charter §11 exists to prevent, and it would
break conservation the moment the two disagreed. So:

```text
purse(line) = the aggregate read across the orbs that line holds,
              with each orb's due settled at the read's stamp
```

**THE PURSE IS A PROJECTION, NOT A TOUCH** (ruled 2026-08-02; this
document's first draft said the opposite and is struck):

```text
purse(line) = Σ over the line's orbs of ( store − demurrage_due(orb, …) )
```

**The draft made reading a purse a settling touch that writes receipts.
That was wrong and would have been expensive** — every status render and
UI refresh would have written demurrage receipts into the chronicle,
making the act of LOOKING a chain event. The projection subtracts what
is DUE without settling it, so the number shown is honest and nothing is
written. Value is still not hidden from decay: the due is already
deducted in the reading, and it settles for real at the next genuine
touch. Vaulted stores keep their exemption unchanged.

**What it makes possible** is everything downstream in this document:
you cannot tithe, price a commission, or judge a founder's solvency
until a line's economy adds up to one number.

## §6 THE ALGORITHMS — nothing here is a constant

**Operator rulings, 2026-08-02 (relayed spec + this lane's audit against
the tree).** §6 formerly listed five "open questions" awaiting a number.
**That was the wrong shape** — each was a constant standing in for an
algorithm. The rulings below give the functions. **Two of the four do
not compute against the engine as specified, and are inscribed with the
defect named rather than transcribed.**

This is FUNCTION-1 (`12_UNION` §4) applied where the drafts forgot it:
**readings stay DERIVED, never declared.**

### 6a. THE TITHE — CAPTURE, and decay as its residual

**REWRITTEN SYNTROPIC (operator ruling 2026-08-02). Every prior draft of
this section is struck.** They built the tithe as a share of
DESTRUCTION: decay split into a skim and a burn. **That is negentropic —
it makes loss the primary object and the ward's income its shadow — and
CHARTER §2a rules it out in terms this section ignored:** *the
generative side leads; refusals are subordinate clauses of a positive
law, never the law itself.* §2a also records that this exact drift cost
real correctness twice before, because *the negentropic form was not
merely bleak, it measured the wrong object.* It did again here.

**THE ENGINE ALREADY RUNS THE RIGHT PATTERN.** Nothing needs inventing:

```text
Encasement   "captures excess heat into joules BEFORE anything vents"
Joule        "minted ONLY by syntropic encasement, never by fiat"
CryoArray    "captures vault-band headroom as bankable sink, never passive"
```

**Capture comes first and venting is what capture missed.** The tithe
takes the same shape.

**THE LAW — a ward CAPTURES; what nothing captures decays.**

```text
at the touch, in this order:

  captured = what the ward's BUILT capture capacity takes
             → transferred to the ward purse            ← THE PRIMARY READING
  decayed  = what no capture reached
             → destroyed                                 ← the residual
```

**A ward's income is a positive act, not a levy.** It is earned by
having BUILT the capacity to capture, exactly as a colony earns joules
by raising a shell. A ward that builds nothing captures nothing — not
because it is punished, but because capture is a thing you construct.

**THIS DISSOLVES THE DIVERGENCE RATHER THAN COUNTERWEIGHTING IT.** The
kernel lane's finding (c0fc819) was that `burned = decay · (1 − share)`
vanishes as share → 1. **There is no share here.** Capture is bounded by
BUILT THROUGHPUT — the standing precedent is `encasement_throughput() ×
shells`, where overflow past throughput vents and the colony must raise
another shell. **A ward cannot capture everything by accumulating a
ratio; it can only capture what it built to capture, and building costs
real resources.** The sink narrows only to the exact extent that
somebody did the work, which is the outcome the economy should want.

Conservation is unchanged: ΔΣ = −decayed ≤ 0, deflationary at every
step, and now stable in the limit because throughput is a built quantity
rather than a ratio with a ceiling of 1.

**CONSTRAINT — capture capacity is not kernel-readable at every scope.**
`lawful` is pure, total and I/O-free. A ward whose scope IS the world in
front of the kernel can be verified locally (the ELDER case). **Universal
scope cannot** (the ANCIENT case) — no single world's kernel sees it, so
it is a mesh-level computation carried as verified evidence.

**CONSTRAINT — this still amends a landed wire event.**
`DemurrageSettled` is one-sided today, booked as `[(resource, −amount)]`
and checked at I5. Capture makes the settle two-sided — a captured leg
with a recipient and a decayed leg — so tag, event shape and I5 change
together. Kernel surgery, sequenced behind P2 (§6e).

**GATED ON ONE DEFINITION OF THE TRANSITION (kernel lane, task #30).**
P3's gradient is REVERTED — it broke `conductor/tests/survey_pins.rs`
replay (Crystal 127/16 live against 15/16 replayed) and only a
workspace sweep caught it. **So this section assumes the `condition ==
5` CLIFF, not a gradient**, until #30 lands. Capture changes
`DemurrageSettled`, which both the live proposer and replay depend on,
so it is exposed to the same failure and must wait behind #30 with the
parameter work.

**OPEN, and measurable only once #30 is in:** `ease` multiplies decay,
so advancing technology lowers what is available to capture. Whether a
ward's income falls as its wards advance is a single step at condition 5
under the cliff, and would become a continuous erosion under a restored
gradient. Measure it with capture and rate readable at one touch.

**NARROWING THE DIAGNOSIS, offered rather than assumed.** Replay's body
(`world.rs` 4226–4829, 603 lines) recomputes exactly ONE shared rule:
`dop::commensurate_weight(resource)`, at world.rs:4439 in the
`ResourceTransferred` arm. **It never calls `ease_of` or
`demurrage_due`** — those arms apply the receipted amount. So the
double-definition exposure is one call site rather than 113 arms, and
the P3 divergence is unlikely to be a shared-rule recomputation at all:
live held MORE crystal than replay, which points at state reached
without a receipt rather than at a value derived twice.

### 6a-bis. NO ABSOLUTES — the threshold is the defect

**Operator ruling 2026-08-02, and it generalises a defect the kernel
lane already paid for.** `ease_of` tested `condition == 5`: four fifths
of the tech tree bought nothing and condition 5 became a strategy
target. P3 replaced that cliff with a gradient. **The same defect is
present wherever THIS document tests a hard boundary**, and it is struck
in all of them.

```text
STRUCK, everywhere it appears in this document:

  a hard IN/OUT band test          → a reading that falls off
  argmax with an equality tie-break → a proportional standing
  viable / not viable               → how well a type lives here
  a threshold that flips an outcome → a curve that bends it
```

**Why this is mechanical and not aesthetic.** A threshold does three
bad things at once: it makes a hair's difference decide an outcome, it
creates a strategy target at the boundary, and it renders every state
short of the boundary indistinguishable from having nothing. Every one
of those is a measurement error before it is a balance problem.

**The positive form:** every reading in this document is defined
everywhere in its domain and moves continuously with what it measures.
Where a hard edge remains it is a PHYSICAL bound (built throughput
cannot exceed what was built), never a judgement.

### 6b. THE COMMISSION PRICE — NOT COMPUTABLE AS SPECIFIED

**The relayed spec:** `ΔU = G · M · m · |1/r₁ − 1/r₂|`.

**AUDIT — the engine has none of the three terms.** Verified by scan:
**zero declarations of mass, gravitation, or G anywhere in `engine/`.**
A star exposes `luminosity()` and nothing else; there is no stellar
mass, no planetary mass, and `orbit_r` is a bare ratio, not a radius in
metres. Introducing G also introduces an IRRATIONAL constant into a
kernel whose first invariant is exact rationals — it could only enter as
a rational approximation, which is precisely the hidden constant this
section exists to excise.

**What DOES compute.** The `|1/r₁ − 1/r₂|` factor is exact and available
today, because `orbit_r` exists and rational reciprocals are exact:

```text
price(commission) = work_k · | 1/r_current − 1/r_target |
```

**One scale constant, not three absent quantities.** `thermal_k()` is
the standing precedent for exactly this kind of scale factor, so
`work_k` is idiomatic rather than novel. It is honest about being a
scale, and §7 binds it to become a reading when one exists.

**Still uncovered, and named:** this prices an ORBIT change only. A
commission that changes albedo, commissions a different star class, or
raises a system where none stood has no price under this formula. The
paid service of §2b is broader than the ruling covers.

### 6c. SUBSTRATE COVERAGE — a reading defined everywhere

**The relayed spec** summed Heaviside steps over the three bands. **Two
defects, and both are the same defect.**

**It returns ZERO exactly where the problem lives.** The bands are
disjoint and do not cover the line, and a sum of steps over
non-covering intervals is zero between them:

```text
T ∈ 320..600 K   → every Θ pair is 0 → Coverage(T) = 0
the derived furnace seats sit at ~332..347 K
```

The worlds §4 exists to rescue read zero under it. **The gap was never
in the lookup mechanism; it is in the bounds, and restating a bucket as
a sum changes its notation, not its support.** It also introduces
`BandCapacity(b)` — a fresh literal per bucket, in the section whose
object is removing literals.

**And the step function IS the absolute struck by §6a-bis.** A world one
kelvin outside a bound reads nothing at all; a world one kelvin inside
reads full. That is `condition == 5` again, wearing thermal clothes.

**THE LAW — coverage is a positive reading, defined everywhere on the
positive line, falling off continuously from each band's core.**

```text
coverage(T) is defined for every T > 0
             reads FULL at a band's core
             DESCENDS with distance from that core
             is never zero-by-construction, only small
```

**Three things follow at once.** No temperature is unaccounted, so §7 is
satisfied. A gap world reads POOR rather than IMPOSSIBLE, so a mineral
line founding at 332 K is a hard place to live rather than an
uninhabitable one — which is a game, where the alternative was a defect.
And `band_of` returning `None` stops being an absolute exclusion: the
named bands become the REGIONS WHERE COVERAGE IS RICH, never the test
for whether a world counts.

**Held for the operator:** the falloff's shape. It must be exact
rational, monotone in distance from the core, and positive everywhere —
beyond that it is a tuning question, and §7 binds it to read its own
conditions rather than carry a literal.

### 6d. WARD STANDING — proportional, so ties need no rule

**The relayed spec** was a tuple argmax with two tie-breaks: capacity,
then earliest founding stamp, then lower line hash. **It is
deterministic and total, and it is a cliff.** Winner-take-all means a
hair of capacity flips the entire ward, the boundary becomes a strategy
target, and every line but one reads as though it held nothing. §6a-bis
strikes it.

**THE LAW — standing is held in proportion to capture capacity.**

```text
standing(L, ward) = capacity(L) as a share of the capacity standing
                    in that ward — a rational in [0,1], for EVERY line
```

**The tie-break disappears because the cliff did.** Two lines of equal
capacity hold equal standing; that is the correct answer and it needs no
rule, no founding stamp and no hash. **A rule that exists only to
resolve an equality is a sign the equality was made to matter by a
threshold** — remove the threshold and the special case goes with it.

**Succession stops being an event, as before.** A line's standing falls
continuously with its capacity, and what it held redistributes to
whoever holds capacity in that scope. No ceremony, no interregnum, and
now no discontinuity at the moment of a fall.

**Where a single holder IS required** — one name on a deed, one
counterparty for a settle — take the greatest standing, and note that
this is a PRESENTATION of the reading, never the reading itself. The
underlying quantity stays proportional and the deterministic ordering
(capacity, then stamp, then hash) survives ONLY as a display
convention, with no mechanic reading it.

### 6e. RETENTION — the leak answered with a DEED, not a boundary

**`leaks()` is the weakest reading in the engine, and moving its
threshold would not fix it.** The defect is not where the line sits. It
is that one side of the line has nothing to do.

**Measured.** `thermal.rs:69` is `*t_eq4 > cryo_vault_max4()` — a pure
step. **The same boundary is read THREE times independently:**

```text
world/rules.rs:51   gates the SEALED demurrage rule
world/tick.rs:274   gates cryo-array capture of vault headroom
world/tick.rs:403   gates the accrued-joule leak
```

**And "vaulted" is not a thing anyone builds — it is a birth
temperature.** `in_vault = t4 <= cryo_vault_max4()`. A cold world is
lossless because it is cold.

**THE ASYMMETRY, which is the actual finding.** The one deed that
captures thermal headroom — the cryo-lattice array — is itself gated on
already being cold (`tick.rs:274`: `t_eq4 <= cryo_vault_max4()`). **So a
warm world has NO deed available to improve its retention. Not an
expensive one. None.** Cold worlds inherit losslessness; warm worlds
leak forever and may do nothing about it. That is a verdict, not a game,
and it fails §4's viability law by leaving a whole class of world with
no path.

**THE PRECEDENT IS ALREADY LANDED, and it is not the cryo array.** The
covenant's collateral sits in a **vitrified matrix** that *"vaults it
losslessly by construction"* (`accord.rs:320-327`, `state.rs:56-61`).
**The engine already knows how to build losslessness.** It is available
only to locked collateral inside a delivery pact, and nowhere else.

**THE LAW — retention is a positive reading, and it is BUILT.**

```text
retention(orb) ∈ [0,1] ∩ ℚ   the share of banked stores an orb HOLDS

    inherited   cold position confers retention freely
    built       vitrification deeds raise it, at a price

decay = banked × (1 − retention) × elapsed      flat, denominators bounded
```

**Retention is what is held; decay is the residual.** That is CHARTER
§2a's order — the generative side leads and the loss is its subordinate
clause — applied where `leaks` had it backwards.

**THE DEED: `vitrify`.** Extend the vitrified matrix from covenant
collateral to general banked stores as a costed, receipted action. **A
warm world then BUILDS toward what a cold world was given.** Thermal
position stops being a verdict and becomes a starting condition with a
price attached, which is the difference between a penalty and an
economy.

**What this collapses.** The three duplicate boundary readings become
one retention reading. The cliff disappears — retention moves
continuously with temperature, so no world sits one unit of T⁴ from
owing everything. And §4's viability holds by a DEED rather than by
relocating a band edge, which is the fix that does not need the bands
moved at all.

**SEQUENCING.** Retention changes the magnitude of a sealed rule both
the proposer and the judge read, so it lands behind kernel #30 with the
rest of the parameter work. **The deed and the reading land together** —
shipping `vitrify` while `leaks` still gates demurrage would sell a
world something that does not yet do anything.

### 6f. THE TWO-SIDED LAW — every reading needs its inverse

**Operator ruling 2026-08-02, and it generalises §6a, §6c and §6e into
one defect.** Each of those was a reading that named a LACK and had no
term for its opposite. The pattern is systemic and it is visible in one
struct.

**THE ENGINE ALREADY KNOWS HOW TO KEEP TWO-SIDED BOOKS.** `ColonyOrb`
carries three genuine pairs, and they are the social ledger:

```text
returned_across      ⟷  extracted_across
covenants_fulfilled  ⟷  covenants_breached
bond_flow            ⟷  severance_load
```

**The physical ledger on the same struct is single-entry:**

```text
famine: Vec<Resource>    lack, named and receipted   ⟷  (nothing)
overheat: bool           excess-as-problem, flagged  ⟷  (nothing)
entropy_vented: Ratio    loss, measured              ⟷  (nothing)
```

**And the event vocabulary confirms it.** Every thermal or provisioning
event names a lack or an excess-to-be-shed: `FamineStruck`,
`FamineRelieved`, `OverheatStruck`, `OverheatCleared`, `VentSettled`,
`OverflowAbsorbed`, `WantRelieved`. **There is no event for abundance
arriving.** A colony can be receipted as starving, as overheating, as
venting and as relieved — never as thriving.

**The word "surplus" appears in the conductor's policy heuristics and in
the scribe's prose. It appears nowhere in committed state.** The engine
narrates abundance and does not record it.

### 6f.1 EXPANSION ONE — `overheat` is the wrong TYPE

A `bool` can say *too hot / not*. It cannot say **where on the axis**.
Thermal state is a position with a direction and a magnitude, and the
type must be able to hold both.

```text
STRUCK:    overheat: bool                       one direction, no degree
EXPANDS TO: a signed thermal displacement       two directions, a magnitude
```

**This is expansion, not containment** — the fix is a wider type, never
a tighter guard on a narrow one. And the degree already exists: tag 22
`OverheatStruck` carries `overflow: Ratio` on the wire, and
`replay.rs:197` discards it as `{ orb_id, .. }` because the destination
cannot hold it. **The information is receipted, replayable, and thrown
away for want of a type.**

### 6f.2 EXPANSION TWO — the cold branch does not exist

`frozen` is computed inside `tick` (`sub_mesic && joule_famined`) and
discarded at the end of the tick. **There is no `UnderflowStruck`, no
cold degree, no committed field, no wire tag.** The hot side has two
events, a flag, a vent mechanism and an absorption market; the cold side
has a local variable.

The displacement of §6f.1 gives it somewhere to live: **one axis, signed,
with overflow above and underflow below**, and the freeze-lock becomes a
region of that reading rather than a boolean recomputed every tick.

### 6f.3 EXPANSION THREE — FAMINE REQUIRES FLOURISH

**This is the one that builds capacity, and it is the reason the other
two matter.**

`famine` names which resources a colony LACKS. Nothing names which it
holds in abundance. So the state can express contraction and cannot
express thriving — and a quantity that cannot be expressed cannot drive
anything.

```text
famine:   Vec<Resource>            what is missing      ⟶  dormancy, refusal
flourish: Vec<(Resource, Ratio)>   what is abundant     ⟶  ???
```

**The `???` is capacity.** D has no positive driver in committed state.
Today a colony can only lose dimensions — famine strikes, zones go
dormant, yields degrade. Nothing in state says a line is thriving, so
nothing can key growth off it, and capacity is static by construction
rather than by design.

**With flourish recorded, capacity grows dynamically from measured
abundance:** a line in sustained surplus converts it into new dimensions
— the direct mechanism behind `19_REGISTRY` §4b, where an ancient opens
a ROOM rather than granting a seat, and `20_FOUNDING` §6a, where a ward
captures with capacity it BUILT. **Both of those laws currently name a
growth they have no operand for. Flourish is the operand.**

### 6f.4 THE STANDING TEST

**Any reading added to committed state must answer: what is its
inverse, and is that inverse recorded?** If the reading names a lack,
the surplus must be nameable in the same terms. If it names a
displacement in one direction, the type must hold the other.

**A single-entry book measures a system that can only lose.** That is
the same defect CHARTER §2a records twice already, arriving through the
type system instead of through prose.

### 6g. THE REGENERATING ASSET — the economy has no capital, only faucets

**Operator ruling 2026-08-02.** *"How does the underlying asset
regenerate itself?"* **Measured answer: there is no underlying asset.**

**Nothing in the engine depletes, and nothing regenerates.** A scan for
depletion, regeneration, replenishment or exhaustion across `engine/`
returns no resource-sense hit at all — every match is the scribe
regenerating display files or famine being relieved. Both halves are
absent together, and that is the finding.

**The whole production model, measured:**

```text
ZONES   zone_yield(kind, metabolism) is a CONSTANT per tick, multiplied
        by efficiency 1/(1+overflow), terrain boost, and tech factors
        (tick.rs:361-393). No stock is read. No stock is written.

VEINS   an assay mints richness × elapsed. `richness` is set once at
        ProspectStruck from element_properties and NEVER DECREASES —
        confirmed across every site that touches it.
```

**So extraction draws on nothing.** An extractor produces iron forever;
a vein yields forever at a fixed richness. **"Unbounded extraction"
is exact, and its consequence is not exhaustion — it is that value
without scarcity is not an asset.** There is nothing to own, nothing to
improve, nothing whose condition can change, and therefore nothing worth
investing in.

**GROWTH IS TERMINAL, and the ceiling is countable.** A colony grows by
adding zones to an 8×8 grid — 64 tiles — each yielding a constant.
Fill the grid, apply the multipliers, and growth stops. There is no
compounding anywhere in the loop.

**AND TECHNOLOGY IS A FLAT MULTIPLIER, NOT A GROWTH MECHANISM.**
`tech_node_of` is the whole of what research does to output:

```text
condition 1 → 3/4     condition 4 → 5/4
condition 2 → 5/4     condition 5 → 1/2   (the demurrage ease)
condition 3 → 5/4
```

**Five constants applied to a flow.** Research cannot raise a
regeneration rate, because there is no rate. It cannot expand a stock,
because there is no stock. It cannot open a dimension. **It moves a
faucet's setting once and then is finished** — which is why the tech
tree bottoms out in strategy targets (§6a-bis) rather than in a
trajectory.

### 6g.1 THE MISSING PAIR

§6f's law applied to production. The inverse of extraction is not
conservation and it is not restraint:

```text
extracted_across  ⟷  (nothing)          the social books are paired;
                                         the productive ones are not
EXTRACTION        ⟷  REGENERATION       the missing pair
```

### 6g.2 THE LAW — a stock that renews, and intelligence that renews it faster

```text
STOCK        the underlying asset: a quantity extraction DRAWS DOWN
             and regeneration RESTORES. Owned, improvable, depletable,
             and therefore worth something.

REGENERATION rate at which a stock restores itself. The positive term
             extraction has never had.

RESEARCH     raises the REGENERATION RATE — never merely the extraction
             multiplier. This is what makes intelligence compound.

FLOURISH     surplus (§6f.3) reinvests into stock or into regeneration,
             so abundance becomes productive base instead of a number
             that sits.
```

**Why research must act on regeneration rather than on yield.** A
multiplier on a flow is a one-time step: it lifts the line and stops.
**A rise in the regeneration rate raises the ceiling itself**, so the
same research pays forever and pays more the larger the stock it acts
on. That is a growth mechanism rather than a bonus, and it is the
difference between a tech tree and a list of upgrades.

**Why this closes the loop the other sections opened.** §6f.3 asked what
FLOURISH feeds; the answer is stock and regeneration. §6a asked what a
ward's capture capacity is BUILT from; the answer is reinvested surplus.
`19_REGISTRY` §4b asked what opens a ROOM; the answer is a line whose
regenerating base actually grew. **Three laws were each missing the same
operand, and it is capital.**

### 6g.3 WHAT THIS DOES NOT SAY

It does not say extraction should be punished, or that scarcity should
be imposed as a brake. **A stock that regenerates is a MORE generous
economy than a faucet, not a meaner one** — a faucet is capped at its
setting forever, while a stock a civilization tends can grow without
bound. The generative side leads here too (CHARTER §2a): the law names
what renews, and depletion is what happens when renewal is outrun.

**Held for the operator, since these set the economy's whole shape:**
whether stocks are per-orb, per-vein or per-world; whether regeneration
is linear or proportional to the remaining stock; and whether a stock
can be driven to zero at all, or only asymptotically toward it.

### 6h. DIVERSIFICATION — the periodic table, and the three scales

**Operator ruling 2026-08-02.** Stocks (§6g) are one quantity per
resource, and the resource vocabulary is seven names. That is not an
economy with assets in it; it is an economy with seven faucets that now
regenerate.

**MEASURED — the element grammar exists and is consumed by NOTHING.**

```text
ColonyOrb.elements : Vec<([u8; 32], Ratio)>     banked, per orb
element_properties(hash, star) -> (family 1..4, tier 1..3, richness)
element_name(family, hash)                       every element is NAMED
spine_bands(StarClass) -> [u32; 4]               abundance BY STAR
```

Every read of `.elements` in the tree is an encode, a decode, a getter,
a bank, or the kernel's conservation arm. **No production path consumes
an element.** They accumulate and do nothing.

**AND `smelt` IS NOT A TRANSFORM.** `smelt(amount)` takes no resource at
all — it quantizes onto a 1/64 grid so a fill smelted to zero refuses
whole. The actual industry is a fixed three-link chain with one metal in
it:

```text
Iron ──(Smelter)──▶ Alloy ──(Synthesizer)──▶ Diamond
```

**One metal, in a game whose matter grammar already names families,
tiers, and their stellar abundance.**

### 6h.1 THE PERIODIC TABLE IS ALREADY SEEDED

`spine_bands` is nucleosynthesis in miniature and it already works:

```text
O · B · A                    [1, 2, 0, 1]      hot, massive
F · G · K                    [2, 1, 1, 1]      sun-like
M · BrownDwarf               [1, 0, 2, 1]      cool, small
remnants (WD/RG/NS/WR)       [0, 2, 1, 2]      the forges
```

**A world's matter is drawn from its own star's bands**, exactly as the
operator's "distribute at world creation" asks — seeded by the star
class and the element hash, deterministic, replayable, no oracle. Four
families × three tiers is TWELVE classes standing today. **The expansion
is to give those classes real periodic identity — group and period, with
properties that industry can read — not to invent a distribution
mechanism, which already exists.**

**What makes it an economy rather than a longer list:** different
elements must smelt to different alloys with different properties, so a
world's stellar inheritance decides what it can BUILD and therefore what
it must TRADE for. That is comparative advantage arising from physics
(`04` §1.2's stated goal) instead of from a table of prices.

### 6h.2 STOCKS ARE WRAPPERS — three scales, named

**A stock is not the asset. It is a claim on one**, and the two must not
be the same object or a civilization can own only what it physically
holds.

```text
MICRO   the DEPOSIT      a vein or seam of one element at one site:
                         richness, regeneration rate, seed floor (§6g)

MID     the STOCK        a claim ON a deposit — transferable, ownable,
                         priceable, and separable from the matter

MACRO   the GDP          a civilization's whole productive reading,
                         aggregated across every deposit it holds a
                         claim on
```

**The purse (§5) is the macro reading's ancestor** — it already sums
banked stores across a line's orbs as a projection with demurrage
deducted. GDP is that pattern applied to PRODUCTION rather than to
holdings: what a line's assets generate, not what sits in its banks.

**Why the wrapper matters and is not financial decoration.** Separating
the claim from the matter is what lets a line invest in a deposit it
does not sit on, lets a ward hold standing across worlds, and gives
`§6a`'s capture and `19_REGISTRY` §4b's room-opening something
transferable to be built from. Without it, ownership and location are
the same fact and nothing can be financed.

### 6h.3 THE MICROCOSM/MACROCOSM LAW

**Every economic reading must be defined at all three scales, and the
macro reading must be DERIVED from the micro — never stored beside it.**

A GDP kept as its own number is a second readable truth over the
deposits that produce it (Charter §11 rule 2), and it will drift the
moment one deposit moves. The purse already runs this correctly: it is a
projection, not a vault. **GDP inherits that discipline or it inherits
the drift.**

**Held for the operator, since these set the table's shape:** how many
families and tiers the expanded table carries; whether a claim can be
partial (a share of a deposit) or whole only; and whether GDP reads
sustainable draw, actual draw, or both — the gap between them being
precisely the measure of whether a civilization is living within its
regeneration.

### 6i. THE CONSUMER — transducers, and the derivative suite already half-built

**Operator ruling 2026-08-02.** §6h found elements banked and consumed
by nothing. **Two consumers are owed, and they are different in kind:**
industry must consume MATTER, and a market must consume CLAIMS.

### 6i.1 SMELT IS A TRANSFORM WHERE A TRANSDUCER BELONGS

```text
TRANSFORM    Iron ──▶ Alloy            one input, one output, fixed
TRANSDUCER   (element, tier, energy, technique) ──▶ material whose
             PROPERTIES DERIVE FROM ITS INPUTS
```

**The difference is capacity, exactly.** A transform has one output and
therefore adds no dimension: whatever goes in, the same thing comes out
with a different name. A transducer opens a SPACE of outputs, and the
size of that space is the capacity the industry actually has.

`smelt(amount)` today takes no resource at all — it quantizes onto a
1/64 grid. The real chain is `Iron → Alloy → Diamond`, three links and
one metal, and **iron is a single aspect of a matter grammar that
already names families, tiers, and stellar abundance** (§6h). Every
element a world inherits from its star currently smelts to nothing,
because there is no transducer to read it.

**Resource development capacity is the count of distinct materials a
line can produce from what it holds.** Today that number is 2 (alloy,
diamond) and it is the same on every world in every universe. Under a
transducer it is a function of stellar inheritance, technique, and
energy — which is where research finally acts on something other than a
multiplier (§6g).

### 6i.1b THE NAME IS STRUCK — covenant was a layer confusion

**Operator ruling 2026-08-02, LANDED.** *Covenant* named a commercial
delivery contract, which put a collateralized margin position over
physical resources in the vocabulary of moral and relational commitment.
**Two layers, one word.**

```text
RELATIONAL   COVENANT   entity and orb charters: reciprocity, standing,
                        alignment. NON-TRANSFERABLE obligations.
                        RESERVED — no code claims this name today.

RESOURCE     FORWARD    collateralized future delivery: obligor,
                        beneficiary, quota, margin, expiry.
```

**Landed across 29 files, 259 occurrences.** `ClosureEvent::Covenant*`
became `Forward*`; `struct Covenant` became `Forward`;
`seal_covenant_as` became `seal_forward_as`; the fidelity books
`covenants_fulfilled` / `covenants_breached` became `forwards_*`; the
lens labels moved with them; `COVENANT-1` became `FORWARD-1`.

**THE WIRE DID NOT MOVE, and that is the property that made this
lawful.** Tags 35, 36 and 37 carry the same meaning and the same bytes —
only the symbol changed. A tag registry entry may never be repurposed
(`19_REGISTRY` §5), and this repurposes nothing: it corrects what the
code was always called. 534 workspace tests green including the wire
round-trips, clippy clean, zero `Covenant` occurrences left in code.

**AND THE NAME IS CONTAINED, NOT RESERVED** (operator ruling
2026-08-02, correcting this section's first draft). The draft said the
name was "free for the relational layer to take when it lands." **That
was wrong in the same way the original naming was.**

A covenant is a moral and relational commitment — an obligation that
binds because of what a party IS rather than what it delivers. **That is
dogma-adjacent vocabulary, and it does not get a reserved seat in kernel
law.** `18_TORSION` THE CURSE holds the standing position: such
vocabulary is RESERVED rather than forbidden — a thing a civilization
may produce and be burdened by, never a thing the kernel is founded on.

```text
STRUCK   covenant as a resource contract        → it is a FORWARD
STRUCK   covenant as a reserved KERNEL term     → it is CONTAINED
CONTAINED  covenant as relational/moral vocabulary, available inside
           the reservation, to a civilization, as content — never as
           an axis the kernel judges on
```

**This is `19_REGISTRY` §5's tiering doing its job:** an AXIS is
constitutional, a VALUE is claimed, and CONTENT is free. Moral
vocabulary is CONTENT. A civilization may hold covenants, name them,
build a whole ethics on them — and the kernel neither knows nor judges
what a covenant is. **The engine measures delivery; it does not measure
virtue.**

### 6i.2 A FORWARD IS ALREADY A COLLATERALIZED FUTURE

**Measured, and this is the finding.** `Covenant` is not covenant-like;
it is the contract:

```text
obligor      the short — owes delivery        collateral   the margin
beneficiary  the long — receives              window       THE EXPIRY
resource     the underlying                   delivered    the fill
quota        the contract size                sealed       the open
```

with a full lifecycle on the wire — `CovenantSealed` (35),
`CovenantReleased` (36), `CovenantSlashed` (37). **Sealed, filled,
released, or liquidated.**

**And the margin is already risk-priced.**
`collateral_required(fulfilled, breached, quota)` computes
`(breached + 1) / (fulfilled + 1)`, clamped to `[1/4, 4]`, times quota.
**A counterparty that has always delivered posts a quarter of the
notional; one that has always defaulted posts four times it, and is
NEVER barred from the market** — the fidelity curve prices access
instead of gating it. That is a credit-scored margin model, landed and
running.

### 6i.3 THE SUITE, AS A GENERALIZATION OF WHAT STANDS

```text
SPOT       Offer · post_offer · clear_trade                    LANDED
FUTURE     Covenant — expiry `window`, margin `collateral`,
           liquidation CovenantSlashed, physical settlement    LANDED
PERPETUAL  a covenant with NO expiry, held open by a funding
           rate                                                 OWED
CLAIM      the tokenized wrapper on a deposit (§6h.2)            OWED
```

**THE FUNDING RATE IS DEMURRAGE.** A perpetual needs a periodic payment
that keeps an unexpiring position honest, and this engine already has
exactly one time-cost on held value: flat-rate, settled at the touch,
receipted, kernel-bounded at I5. **Inventing a second funding mechanism
beside it would be a second readable truth about the cost of time**
(Charter §11 rule 2). The perp rides the rail that exists.

### 6i.4 THE DESCENT LAW — an asset of an asset, described DOWN

> **A stock is not the asset. It is an asset OF an asset, and it must be
> describable downward.**

```text
CLAIM on a claim      →  CLAIM on a deposit  →  DEPOSIT  →  MATTER
```

**Every layer must name its underlying, and the chain must TERMINATE IN
MATTER.** A derivative whose descent does not reach a physical deposit
is unbacked, and unbacked leverage is the failure mode this suite is
otherwise inviting in wholesale.

**This is enforceable here in a way it is not elsewhere**, because every
layer is receipted: the descent is a walk down receipts, and a claim
that cannot be walked down refuses at the door rather than failing in a
crisis. **The engine can therefore carry the whole instrument set
without carrying its usual pathology** — not by capping leverage, but by
requiring that every position can state, exactly and in matter, what it
is a claim on.

**RE-HYPOTHECATION: BOUNDED, NEVER BANNED** (operator ruling
2026-08-02). A claim may back a secondary claim, subject to two
conditions that are both checkable at the door:

```text
1. TERMINAL      every claim states a walk-able receipt chain ending in
                 a physical ResourceStock. No termination, no position.
2. BOUNDED       the descent walk is at most K = 4 links, and a chain
                 containing a CYCLE (A backs B backs A) refuses.
```

**Refusal is `UnbackedDerivativeClaim`, at the door.** The depth bound
is what stops a leverage cascade from being assembled out of individually
lawful steps, and the cycle check is what stops a claim from being its
own collateral — the two failure modes that a per-position check alone
would let through.

## §7 THE BASE-LEVEL ACCOUNTING LAW

> **"Everything needs accounting for at the microscopic/base level, with
> algorithms that breathe with the system."**

**Every quantity this economy uses is derived from measured state at the
finest grain the engine holds, and no quantity is a policy number.** A
number that cannot be derived is a number that will be wrong the moment
the system moves, and it will not announce itself when it goes wrong.

**The three tests any economic quantity must pass:**

1. **Does it read the base state?** If it reads an aggregate someone
   else computed, it is a second truth (Charter §11 rule 2).
2. **Does it move when the system moves?** A quantity that holds still
   while the world changes is a constant wearing a function's name.
3. **Is it exact and replayable?** Rationals only. Two nodes must reach
   the same value bit-for-bit or it is not a measurement.

**This binds every parameter named `placeholder` in the tree**, and
`dop::demurrage_per_tick` is the nearest one: a flat rate that should
itself be a reading. Its shape is right and its value is still a
constant.

## §8 EXTERNAL WALLETS — deferred, on purpose

External wallet integration is coming and is **NOT** built now. This is
a prototype; the purse (§5) is internal and sufficient for every
mechanic in this document. `01_ECONOMY` §5.2 stays intact and unstrained
— nothing here needs a chain, and the tithe settles in-game because it
settles in `cfe::dop::Resource`.

## §9 WHAT OPENS

**This section replaces a "what yields" table.** That framing was wrong:
it read as capping provisions that are already flowing. **Nothing here
is being closed off — each entry is a place the flow was DAMMED BY A
CONSTANT, and this document opens it.**

| where | what was damming it | what it opens into |
|---|---|---|
| `04` §4 | genesis ends at *"THEN the colony"* | founding continues into acquisition, purse, tithe |
| `04` §1.2 | plasmic placement implied by metabolism | placement is acquired (§1, §2) |
| conductor | derived furnace seats as the default | a continuous substrate reading (§6c) |
| `cfe` | `SubstrateBand` as three windows | a reading defined everywhere (§6c) |
| `dop` | `demurrage_per_tick` a flat constant | a rate that reads its own conditions (§7) |
| decay | proceeds to the void | proceeds to the ward (§6a) |
| stores | per-orb only | the purse as a whole-line reading (§5) |
| `19` §4e | hot/cold "unreachable by default" | reachable by purchase or commission (§4) |
