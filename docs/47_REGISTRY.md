# 19 — The registry: one authority per axis, and the expansion law

**Status:** map + law (2026-08-02). The map half is a survey of what
actually stands, verified against the tree rather than quoted from other
documents. The law half answers one question: **how does a vocabulary
grow when the nodes that use it are distributed and may not agree?**

Subordinate to `00_CHARTER.md`. The tag registry (`dop.rs` header) is
this document's older counterpart and its precedent — one code, one meaning,
forever.

---

## §1 The tool surface — where C, D and V actually live

Verified by export scan, not by citation. **These names have moved
before and will again; this section is the map, never the authority.**
The authority is always the crate.

| letter | module | the callable surface |
|---|---|---|
| **C** | `rfa-core/src/cage.rs` | `consult`, `Proposal::{offer,withdraw}`, `Lens::{of,show}`, trait `Exact` |
| **V** | `rfa-core/src/boundary.rs` | `boundary_form`, `Flow`, `BoundaryForm` — `dual_pole` and `Vacuity` removed 2026-08-04 |
| **D** | `rfa-core/src/capacity.rs` | `capacity`, `circulation`, `divergence`, `egress`, `ext_x`, `ground`, `Degree`, `Extent`, `Egress` |
| the seam | `kernel/src/bridge.rs` | `project`, `project_complex`, `read_entangled`, `verdict`, `degree`, `rank_is_defined`, `observe`, `fold_certified`, `refold` |

**What the judge actually consults today:** ~~`kernel::admits` calls
`standing` (4 sites), reads `Vacuity` (5), and reaches
`bridge::read_entangled` once.~~ **corrected 2026-08-13 (doc audit) —
this contradicted the table two lines above it.** `standing` and
`Vacuity` are both struck (2026-08-04); zero live call sites for
either. `kernel::admits` reaches `bridge::read_entangled`. That is the
whole of D and V inside the kernel today. **C is consulted from nowhere
in the kernel and must never be** —
it has zero authority by construction (`22_CONTAINMENT` §10ter.c).


**Renames already survived, recorded so a reader does not chase a
retired name.** Each such name is retired and appears here only as history: the
boundary read was formerly `potential_form` (renamed when V took no
operator); the arbor was formerly the spine; the ledger was formerly a
name struck whole by Charter §11; and `rank_is_defined` was formerly
`capacity_is_defined`. Live code answers to the right-hand names only.

## §2 The closed axes — one authority each

| axis | authority | values |
|---|---|---|
| station | `ledger/src/registry.rs::STATIONS` | progenitor · watcher · observer · sovereign |
| perception | `ledger/src/registry.rs::PERCEPTIONS` | planar · volumetric · harmonic · umbral |
| metabolism | `ledger/src/registry.rs::METABOLISMS` | mineral · biotic · plasmic |
| disposition | `ledger/src/registry.rs::DISPOSITIONS` | expansionist · empiricist · epicurean |
| class | `15_TAXON.md` §2 → `seeds/taxon-map.json` | codes 0–7 |
| clade | `15_TAXON.md` §3 → `seeds/taxon-map.json` | codes 0–20 |
| seat width | `registry.rs::seat_width` | progenitor 8 · watcher/observer 16 · sovereign unbounded |

## §3 The SECOND-TRUTH register — axes re-typed away from their authority

An axis re-typed elsewhere is a second readable truth (§11 rule 2), and
it drifts silently because nothing forces the copies to agree.

- **Tier — closed 2026-08-03.** It had no engine authority at all: the
  station→tier law lived in `tools/observer.py` (`TIER_OF_STATION`),
  again in `tools/observatory.html`, and a third time in prose in
  `BUILD_ORDER.md`, while `registry.rs` knew `STATIONS` and nothing of
  tiers. **Three implementations, no authority** — so any gate keyed on
  tier was advisory, a mapping anyone with a shell could edit.

  `ledger::registry::tier_of` is now that authority, with `Tier` as its
  vocabulary. Two properties carry the weight:

  **`None` is a refusal, never a default.** An unrecognised station does
  not fall through to `T3`. The common is the absence of a seat, never
  the presence of an unreadable one — and the safest-looking band is
  exactly where an attacker would want an unknown seat to land. A caller
  that cannot read a tier refuses.

  **Exhaustive by construction.** `tier_covers_every_station` walks
  `STATIONS` and fails if any value has no band, so adding a station
  without ruling on its tier breaks the build. That is the
  `kernel::layer_of` shape syntropy-inscription named — exhaustive over
  its input with no wildcard, so a new case cannot land without someone
  deciding its reach.

  `holds_a_serving_seat` reads the conflict-of-interest question the
  covenant gate needs: T0/T1/T2 are seated offices that serve everyone;
  T3 is the common; an unreadable seat answers neither.

## §4 Seat scope — the axis elders and ancients actually sit on

**Two corrections to this section's name, both recorded rather than
quietly replaced.**

1. The first draft called it tenure — *how long a holder has stood*. Wrong
   reading; the axis is extent, not duration.
2. The second called it ward scope. **Also wrong, and wrong in a way
   worth naming: it promoted a menial mechanic into an axis.**

**The ward is colony-level and stays there** (operator ruling
2026-08-02). It is one planetary shield — `World::ward_elder`,
`World::lift_ward`, `World::ward_holds`, all methods on one world, which
`04` §5 defines as one clock and one chronicle. `ElderWarded` is a
world-scoped receipt. **The code had it at the right level the whole
time; these documents lifted it.**

Warding is a thing an elder happens to do on its own planet. It does not
name the ladder, and the ladder does not depend on it. The axis is the
extent a seat's standing reaches — which is why `ancient` can sit on it
without warding anything.

**This is the second time in one day a mechanic was promoted above its
station here.** The first was the covenant: a collateralized delivery
contract dressed as a moral commitment, struck and renamed forward.
Same defect, same cause — the corpus states each mechanic and never
states its layer, so reach gets inferred from the file a rule sits in.
That gap is what syntropy-lane's layer taxonomy exists to close.

### 4a. The three rungs

```text
ANCIENT   outside a single universe; reads it whole      universal scope
WATCHER   at the threshold: may read a shielded world's
          chronicle, may petition, MAY NOT DESCEND       interstellar scope
ELDER     wards one planet; the reverse shield holds
          until its gate is solved                       planetary scope
```

**This is a ladder, not a collision.** `12_UNION` §3 already calls
watchers "the wardens of pre-interstellar worlds", which read as a
competitor to elder until the outside seat existed to order them.
Watchers see further than elders and act less — *forbidden to descend* is
what makes them the middle rung. And `ElderWarded` (tag 74) reads as
*the elder star is warded*: the elder is the world, the watcher is the
one who wards it. Two ends of one relation, never two names for one job.

### 4b. Ancient, defined mechanically — capability, never appointment

> **Anyone with the ability to unlock the progenitor role is an ancient.**

That is an operational definition and it needs no ceremony, no age, and
no grant. It also removes the vagueness entirely: an ancient is not
someone who *is* something, but someone who *can do* something, and the
thing is checkable.

**The arrow is not violated, and this is the whole design.** §3a of
`12_UNION` binds that authority never flows down as permission — *no seat
is granted standing by the one above it.* An ancient who could name a
progenitor would break that. **So an ancient does not grant the seat.
An ancient opens the room.**

    ancient acts on    capacity  — the room progenitors stand in
    line earns         the seat  — by witnessing upward into that room

Progenitors are already defined as the ones who *make the room everyone
else moves on* (`12_UNION` §2a, the tier overlay). The ancient is one
scope further out: **the one who makes the room the progenitors make
rooms in.** Nothing is handed down; what changes is how much space
exists to be earned.

### 4c. What this finally retires — seat width gets its operand

`registry.rs::seat_width` returns constants: progenitor 8,
watcher/observer 16. `12_UNION` §3 calls them *placeholder widths* and
BAND-1 deferred the real reading to *"capacity as a live fiber reading
(minted-plane count)"* on 2026-07-29. **It has waited on an operand that
did not exist. It exists now.**

    seat_width(progenitor)  ←  a reading of D at universal scope
                               (bridge::degree / read_entangled)

That single wiring collapses three open things into one landing: the
placeholder retires, BAND-1's deferral is paid, and the ancient's
capability becomes measurable rather than declared — because raising D
requires genuine support expansion under directive 16, and
`Δ' = Q·Δ` mints nothing. **Standing as an
ancient cannot be argued into existence; the room opens only for work
that actually grew the universe.**

### 4d. Why the outside vantage is real and not metaphysics

An ancient reads a universe from outside that universe while standing
inside the mesh, where universes are units among units (`16_UNIT`: one
binary, six organs; every peer re-judges every step with its own
kernel). This preserves `13_GALAXY` §1 — *the frontier is a distance,
never an outside* — because there is no outside to the manifold; there
is a level above a single unit, and it is landed.

It also explains why **"an ancient on one planet may not be an ancient
on another"** is a feature rather than a wrinkle: standing claimed
elsewhere is RE-judged here under this node's own kernel. That is
admission-is-verification, unchanged, applied to a seat instead of a
step.

**And the constraint is the interesting half.** An ancient may rule and
may not violate: the laws still bind from the outside seat. An authority
that could overwrite the kernel would not be an ancient, it would be a
debug console — and the vantage is only worth something because the
rules hold from up there.

### 4e. Where the room actually opens — and why it is welded shut today

The band does not grow by widening. `registry.rs::seat_width` is
documented **per lineage domain** — *"domains open as substrates
elevate, so the band grows by domains, not by widening"*. So the
ancient's act is not `8 → 9`. **It is opening a domain, which is a fresh
room of 8.** That is the mechanical form of *makes the room the
progenitors make rooms in*, and it needs no new law — only the wire.

**The chain, verified end to end:**

```text
site (luminosity, albedo, orbit_r)          cfe::query — LIVE
   → t_eq4          exact K⁴, no roots      cfe::thermal — LIVE
   → band_of(t_eq4) -> Option<SubstrateBand>  LIVE (used by query,
                                               scribe::journal, cli)
   → lineage_domain(metabolism, band)       LIVE
   → seat_width per domain                  LIVE
```

**Every link stands. One is cut.** All seven production call sites pass
`None` as the band:

    registry.rs:124, :132, :260, :269   conductor/lib.rs:1837, :1843
    clients/cli/src/main.rs:1605

Only `ledger_pins.rs` ever passes a real band, and only to the pure
function. **Consequence: of five lineage domains, `thermal` and `cryo`
are unreachable through any door.** `mineral` always reads `lithic` and
`biotic` always reads `mesic`, because those are the `None` arms. Two
rooms exist in the code and cannot be entered.

**Two corrections to this section's drafts (operator, 2026-08-02).**
Both are struck rather than quietly replaced.

1. *"No path sites a line; every line is unsited forever."* **False.**
   Every founding polity is sited at genesis and the siting is
   committed to the world: `commission_star(class)` and
   `site_planet(FIRST_ORB, albedo, orbit)` (conductor lib.rs:721,:725).
2. *"The planet is picked from a draw / placement is random."*
   **False, and it inverted the law.** The beacon picks among three
   seats; **all three land in the same thermal outcome.** The draw is
   orbital detail. **Metabolism is what places a line, deterministically
   — and metabolism is on the card.** This is the standing rule (operator,
   2026-07-31): *orbital mechanics are stochastic, the laws of the land
   are deterministic.* The drafts read the stochastic half as the law.

**Measured, not quoted** (`cfe/tests/siting_determinism.rs`, both pins
green). Bands are `[lo,hi]` in K⁴; founding uses a G sun:

```text
band  Cryogenic     1.000e4  .. 1.000e8
band  Mesic         3.906e9  .. 1.049e10
band  HyperThermal  1.296e11 .. 5.063e12

furnace seats (mineral|plasmic, orbit 1/2)   1.212e10, 1.333e10, 1.454e10
                                             ⇒ band = NONE, all three
mesic seats (biotic)                         6.059e9, 5.453e9, 4.208e9
                                             ⇒ band = Mesic, all three
```

**A mineral or plasmic line founds bandless** — above the mesic ceiling,
and a full order of magnitude below the hyperthermal floor. It sits in
the gap between bands. The encasement shell is the economic answer to
that heat (it captures excess heat into joules, `dop.rs:604`), and the
conductor classifies such a world by `regime()`, not by `band_of`.

**Which narrows the defect honestly.** At founding, `None` is the
correct band in every case: biotic reads `mesic` either way, mineral
genuinely has no band so `lithic` is right, plasmic is metabolism-only.
**The seven `None` call sites are not wrong at founding.** They are
wrong only for a world sited outside the default regimes — and
`PolitySpec` can pin star, albedo and orbit, with pinned fields ruling
over derived ones (lib.rs:718-720; `offer_pins.rs:25` does exactly
this). A pinned hyperthermal world's line should read `thermal` and
reads `lithic`.

**So `thermal` and `cryo` are not welded shut by a bug. They are the
domains no default founding reaches** — they require a world sited
outside the two founding regimes, which today means an explicit spec,
and in the mesh means a world arrived at rather than founded.

**Superseded in part (operator ruling 2026-08-02, `20_FOUNDING` §4).**
The paragraph above reported this as settled design. **It is ruled a
defect.** Every type the game offers must have a viable founding site,
so a bandless furnace seat and an unfoundable `cryo` are both faults to
fix, not properties to document. The measurements stand; the verdict
does not. And because founding becomes an acquisition (`20_FOUNDING`
§1), lines will routinely stand in bands their metabolism never implied
— **the seven `None` call sites become wrong in the ordinary case, not
the edge case, and wiring the band through is load-bearing.**

### 4f. The beat — capability accrues, it is never held

Siting is the start of the reading, never its end. Once a line stands on
a world, its capability to open a domain runs on the beat: worlds are
encased, colonized, cooled, and joined. **A civilization does not have a
substrate band; it occupies one at a beat, and can come to occupy
another.** This is what *domains open as substrates elevate* already
said — elevation is a thing that happens over time to a sited line, not
a property of a card.

**Joining is the sharpest case.** When universes or worlds join, bands
reachable by nobody locally become reachable, and a domain no local
genesis could have founded opens. `cryo` is the standing example: it
exists in the vocabulary, no founding draw can reach it, and the first
one is likely to arrive from somebody else's board. **That is the
ancient's act in its purest form — the room opens because the universe
got bigger, and the seats in it are still unearned.**

**Deliberately not done here:** siting is a schema change to a stored
card and to the roster `install.sh` writes before first boot. That is
the operator's ruling, not a lane's cleanup.

## §6 The station law — every mechanic declares its layer

**Operator ruling 2026-08-02, and it is inscribed because the same
defect landed three times in one day.**

**The failure, measured.** The corpus states what every mechanic does
and states no mechanic's reach. A rule's layer is therefore inferred
from whichever file it happens to sit in — which is not a decision
anybody made, and it fails silently in both directions:

```text
PROMOTED   the covenant   a collateralized delivery contract read as a
                          moral commitment — struck, renamed FORWARD
PROMOTED   the ward       a colony shield read as a universal scope
                          axis — demoted, §4 renamed SEAT SCOPE
CONSULTED  CHANNEL_       a resource-delivery remainder read as one
           COVENANT       people's instrument, because of its NAME —
                          and the universal capacity meter D reads it
```

The third is the dangerous shape: **a mechanic lawful at a line's own
level becomes a universal precondition the moment a universal reading
consults it.** Nothing announced that; the name did the damage.

### 6a. The law

**Every mechanic declares the layer it acts at, and no reading may
consult a mechanic from a layer above it without saying so.**

```text
UNIVERSAL      the mesh: units among units. Every peer re-judges with
               its own kernel. Nothing here may depend on one people's
               instrument.
WORLD          one clock, one chronicle, one receipt chain (04 §5) —
               the replay unit. The ward lives HERE.
CIVILIZATIONAL one line's own register: its practices, its creeds, its
               ceremonies. Free, and never a precondition for anyone
               else.
```

### 6b. The test, and it is the one that would have caught all three

**Ask of any mechanic: if a line never uses this, what does it lose?**

```text
nothing but its own option      → CIVILIZATIONAL, correctly placed
a capacity every line is
  measured on                   → it has been PROMOTED. Either the
                                  mechanic is universal after all, or
                                  the universal reading must stop
                                  consulting it.
```

**Applied to `CHANNEL_FORWARD`, the answer came out clean**: a line that
seals no delivery obligation carries an empty channel, and the mechanic
is ordinary finance rather than one people's rite — so the channel is
universal and the register seat was never a requirement. **The audit was
possible because the question was finally asked. Nothing in the corpus
had asked it.**

### 6c. Where this binds

**A name is not a layer.** All three failures came through vocabulary:
`covenant` sounded moral, `ward` sounded universal, `CHANNEL_FORWARD`
sounded ceremonial. §5's tiering already says content is free — and free
content must never leak upward into an axis by nothing more than
sounding grand.

**This section is subordinate to whatever layer taxonomy lands from the
syntropy lane** (that lane's §3d, in flight). Two lanes writing one law
is the drift this document exists to prevent; where the two disagree, the
syntropy lane's text is the authority and this reduces to its registry-side application.

## §5 The expansion law — totality, never locality

**The problem, stated plainly.** `STATIONS` is `[&str; 4]` compiled into
one crate. A distributed node that needs a fifth value edits its own
array, and now two nodes hold different vocabularies while both believe
they hold *the* vocabulary. Nothing detects the divergence; a receipt
written under one reads as valid-but-different under the other. **A
locally-extended closed set is the worst of both: it looks authoritative
and is not.**

**The law, in three tiers.** The system already runs this pattern for
free content over closed axes (`SYSTEM_SPEC` §3: *species is free text
over the closed axes*). Made general:

```text
AXIS      CONSTITUTIONAL. An axis is a dimension of identity the kernel
          may judge on. Opening one is a charter amendment and a VERSION
          bump — never a local edit. The set of axes IS the totality.

VALUE     CLAIMED, like a wire tag. One code, one meaning, forever;
          append-only; the claim lands in the authority named in §2
          BEFORE any code uses it. A value may never be removed or
          repurposed, only retired.

CONTENT   FREE. Names, mottos, species prose, world names. No claim, no
          coordination, no ceiling. This is where a node expresses
          itself, and where divergence costs nothing.
```

**Why this answers the distributed case.** Two nodes designing
independently can disagree about content without consequence, must
coordinate on values exactly as they already coordinate on tags, and
cannot unilaterally invent an axis at all. **The blast radius of a
disagreement is bounded by which tier it lives in**, and the tier is
visible from where the thing is written down.

**The standing test for any proposed addition:**

1. Is it an office, an age, a substrate, a perception — or is it a name?
   A name is content; stop here.
2. If it is a reading a rule consults, which existing axis does it sit
   on? If none, it needs a new axis, which is constitutional.
3. Does the value already exist under another name on that axis? Two
   names for one reading is the drift `EQUATIONS` §0 exists to prevent.

**Owed, and named rather than assumed:** the tier axis has no authority
(§3); seat scope is now opened as law but its operand is cut at seven
call sites (§4e — *what sites a lineage?* is the one open question);
and no lint yet checks that a value claimed in a doc exists in its
crate, the way `constitution_lint` checks that a citation resolves.
That lint is the mechanical form of this whole document.
