# 19 — THE LENS LAW: observation, magnification, and motion

**Status:** LANDED (2026-08-02). Every reading described here is built
and pinned; §14 is the inventory. Subordinate to `00_CHARTER.md` and
`18_TORSION.md`.

**Lane:** `.claude/worktrees/lens-lane`, branch `lens-lane`. Territory:
`engine/lens/`, `engine/scribe/`. This document is this lane's own; it
amends no other lane's law.

---

## §1 WHAT A LENS IS, AND THE ONE RULE

A lens **reads and reports**. It never gates.

That is not a style preference — it is constitutional. `18_TORSION` §7b
places the over-time and global readings outside `lawful()` and gives
the reason: *a judge that reads a window is no longer pure, no longer
total, and no longer replayable step-by-step.* The lens is therefore
**forbidden** in the kernel rather than merely permitted outside it, and
the boundary is what makes this lane structurally unable to break the
judge. The worst a lens defect does is misreport.

### 1.0 THE OPERATOR DISPLAY BOUND — no privileged seat

**Ruled 2026-08-02: the word "human" is struck from the law and the
code, and replaced by OPERATOR everywhere it named a SEAT.**

The corpus carried an exception phrased as *"only where a reading is
reported to a human lens."* That phrasing was a hazard, not a courtesy:
it made the strictness of the mathematics depend on **who was
watching**, and a rule that relaxes for one kind of observer is a
backdoor through which cosmetic and approximate forms enter and stay.
There are no humans at the code level. There are only OPERATORS, and an
operator may be organic, synthetic, procedural, or another subsystem.
**The seat does not care.**

**This enforces a law already landed rather than adding one.**
`12_UNION` §5a — THE SUBSTRATE LAW, NO ANTHROPIC GATE — already rules
that *the union must not overlook anyone through the human perspective
ingrained in its logic; intelligence is any dynamic that can carry a
chronicle, in ANY substrate.* The "human lens" exception was that same
anthropic gate, surviving in the sealed math and the law documents after
the law had already refused it.

**THE BOUND, replacing the exception:**

> A display projection generated for an OPERATOR interface is a
> **terminal string representation**. It carries zero accessors, zero
> ordering traits, and strictly zero write-back capability into any
> computing or decision path.

So the L1 aggregate is still admitted at this surface — and is now
admitted on a STRUCTURAL condition rather than a demographic one.
`luminal::Amplitude` is the bound made real: exact rational in,
`String` out, no accessor, no ordering. Uniform determinism for every
seat.

**WHAT WAS NOT STRUCK, and the distinction is the same layer discipline
that reverted the roster sweep.** `humans` as a CIVILIZATION NAME, the
`humanoid` clade, `human-readable`, `Human Capital` — all CONTENT, all
untouched. A species in the roster is not a privileged seat, and
renaming it would be the layer error, not the cure. What was struck is
every place "human" named a SEAT or licensed an EXCEPTION.

**THE STANDING TRAP.** A reading used as a target stops being a reading.
Work reshapes itself into whatever the number counts, and the measure
stops measuring. `18_TORSION` records that this system's own author made
that mistake once already. Every reading below is a reading; none is an
objective; nothing optimizes any of them.

---

## §1a HOW THIS LANE AVOIDS BECOMING A MIRROR

`00_CHARTER` §3a bans the banned instrument as structure AND as word,
and its warning is aimed precisely here. §3a records that the crate it
excised ***was born a read-only dashboard and was safe while that was
all it was — it became load-bearing one useful change at a time, and no
rule existed to stop it.*** That is a description of a lens lane going
wrong, so this section is the rule that stops it. The excised name is
not reproduced here: §3a is the one article licensed to say it, because
the article defining a ban must be able to name what it bans.

**THE TWO SIGNATURES OF A MIRROR**, from §3a:

1. **DIRECTION** — it points INWARD at the system rather than OUTWARD at
   an object. *"A system that inspects itself and calls the looking work
   is precisely `α ≅ Ω`."*
2. **THE COPY** — *"because a mirror returns the self, it must hold a
   second image to return — and from that moment there are two things
   that can disagree, and every reconciler after it is a tool built to
   manage a problem that should not exist."*

**THE TEST, applied to every reading in this document.** A reading is a
lens only if it passes all three:

```text
    OUTWARD   it is turned ON something — a relation, a world, a pair —
              never on the lens's own workings.

    GENERATIVE  it returns something that DID NOT EXIST before the look.
              If the answer was already sitting somewhere retrievable,
              the instrument is a mirror handing it back.

    NO SECOND IMAGE   it DERIVES at read time and stores nothing. The
              moment a reading is retained, there are two records and a
              reconciler becomes necessary — which §11 rule 5 clause 1
              forbids outright.
```

**THE SCOPE-TIGHTENING RULES, binding on this lane:**

- **DERIVE, NEVER RETAIN.** Every reading is a pure function of
  committed state, computed at the moment it is asked for and discarded
  after. No lens type holds a field that is a copy of world state.
- **RENDER-ON-DEMAND MEANS COMPUTE LESS, NEVER STORE MORE** (§4). This
  is the sharpest edge in the lane. The obvious way to "render only when
  needed" is a cache — compute once, keep it, serve it — and a cache IS
  the second image, arriving as an optimization exactly the way the old
  dashboard did. The optimization here is LAZINESS: not descending a
  peel nobody is looking through. It is never MEMOISATION.
- **NO PRIVATE HISTORY.** Readings that span steps (§2a kinematics) read
  two points FROM THE CHAIN. They never accumulate their own buffer —
  the chain is the one record (§11 rule 1), and a lens keeping its own
  timeline is a second one.
- **NO TALLIES.** A counter that accumulates is a record. Where a
  frequency is wanted, it derives from receipted deeds, or it is
  conducted as an EXPERIMENT in a harness (§7a) — never kept as running
  state inside the live system.
- **THE LENS IS DELETABLE.** At any moment, deleting this entire lane
  must cost the system nothing but visibility. If removing a lens would
  lose information or break a rule, it had stopped being a lens.

**THE STERILE BOUNDARY, stated formally.** Let `W` be the committed
state and `L` the lens execution:

```text
    STATELESS DERIVATION     L : W → View,   with   Storage(L) = ∅
    STRICT EPHEMERALITY      ∂State(W)/∂L = 0   ∧   W \ L ≡ W

    ┌──────────────────────┐            ┌──────────────────────┐
    │  COMMITTED CHAIN  W  │ ──[read]──►│   LENS LAYER  L      │
    │  · world state       │            │  · derive on demand  │
    │  · receipted deeds   │            │  · zero stored fields│
    │  · canonical timeline│            │  · pure projection   │
    └──────────────────────┘            └──────────────────────┘
               ▲                                    │
               └──────────── BLOCKED ───────────────┘
                  no writeback · no second image
```

`W \ L ≡ W` is the whole lane in one line: **remove the lens and the
world is unchanged.**

**HELD BY A STANDING TEST, NEVER BY MEMORY** (operator ruling
2026-08-02, `test_lens_deletability`). Rule 5 as prose is worth nothing
— prose failed twice in one day in this tree, which is what LAW 8 and
`constitution_lint` exist to say. The invariant is mechanical:

> If the kernel, sealed math, cfe, codec, mesh or conductor fail to
> build or fail their suites with the lens crates excluded, **a lens has
> illegally leaked into the operational dependency graph.**

**Landed as `lens/tests/deletability.rs`, three assertions, all green at
first run:**

| assertion | what it holds |
|---|---|
| `no_sealed_crate_depends_on_a_lens` | the judge, the sealed math, cfe, codec, mesh, physics and both solver crates name no lens crate in ANY dependency table — dev included, since a law test that renders cannot run once the lens is gone |
| `the_ledger_never_depends_on_a_lens_at_runtime` | `#37`'s split holds: the ledger may build fixtures through the display writers in DEV only, never at runtime |
| `the_conductor_renders_but_never_decides_on_a_reading` | the one lawful edge, pinned by SHAPE rather than absence |

**THE DIRECTION IS THE LAW, not the dependency.** A CLIENT may render —
that is what a client is for, and `16_UNIT` §2 names the conductor a
client of units. The ban is on a lens standing UPSTREAM OF THE JUDGE,
because from that moment deleting it breaks a rule rather than dimming a
light.

**A CONSTANT IS NOT A READING** — the precision the third assertion
required. The conductor branches on `scribe::journal::YEAR_TICKS` to
decide WHEN to write a journal. That is a cadence: a fixed number saying
how often to render, and a branch on it decides only whether a light
comes on. A READING is what a lens COMPUTES FROM WORLD STATE, and a
branch on one of those puts the world's behaviour downstream of an
observation. **Only a lens CALL result may never be bound or branched
on.** The cadence coupling is recorded here rather than refused; if
`YEAR_TICKS` ever wants a home outside the lens, that is a tidiness
question and not a mirror.

**Why the outward test passes here.** Every reading below is turned on
an OBJECT: the peel resolves a relation `Δ`; resonance compares an
entity against a manifold; the `Vacuity` surface reads a world. None
inspects the lens. The one reading that could point inward — a lens
reporting on its own rendering — is not built and is barred by this
section.

---

## §2 THE THERMODYNAMIC READING IS NOT ENOUGH

**Operator ruling 2026-08-02:** *thermodynamics is unguided entropy —
how fast to infinity, how fast to so small its information density
explodes.* It says a system will run down. It does not say how a system
MOVES.

Article III LAW 4 (the Second Law) is correct and stays: work requires a
gradient, and a flat field yields none. But it is a statement about
whether work is POSSIBLE, not about the shape of the motion when it
happens. A lens built only on it can report that a world is `Stasis` and
nothing further — no direction, no speed, no resonance.

Two layers are available and are not yet read here.

### 2a. KINEMATICS — the pressure of the twist

The ancestor draws the distinction exactly (`/opt/ipsal-autic-v0.1`,
`docs/mathematics/umd_at_cognition_layer.md` §2):

| layer | content |
|---|---|
| **manifold** | **torsion = the TWIST event** — a non-trivial element of `π₁(ℳ)` |
| **kinematic** | **the PRESSURE of that twist** — the strain it generates |

*"Dual witnesses of the same event."*

**Netstratum has the first and not the second.** `orbit_lock`,
`holonomies` and the fold gate detect that a twist IS. Nothing reads how
hard it pushes. That missing reading is the motion this lane owes.

### 2b. MORPHODYNAMICS — one manifold, K phase angles

UMD, stated in one line: **ONE manifold observed from K phase angles;
the twist between angles measured as kinematic pressure; coherence is
the residual at minimum tension.**

The consequence that matters for a lens: two views that disagree are not
two objects. They are one object seen from two angles, and the
disagreement is the reading. `SensoryCompound` (K=4) makes the
dependence explicit — *vision / hearing / touch / refraction phase
angles **per orbital radius***. Perception varies with DISTANCE, which
is §3.

---

## §3 MAGNIFICATION IS A DEPTH, NOT A FEATURE

`perception/nested_peel.rs` settles the telescope/microscope question,
and its own header carries the decisive claim: *coboundary decomposition
gives the 3-layer structure — **no new mathematical primitives**.*

```text
    Δ_value    the canonical H¹ representative     WHAT stands    FAR   telescopic
    Δ_object   the boundary content / residuals    WHERE it strains  MID
    Δ_cell     the raw relation, exact entries     HOW assembled   NEAR  microscopic
```

*"Each outer peel constrains the inner. The outer finds WHAT changed;
the next finds WHERE; the innermost finds HOW."*

**Netstratum already computes all three.** `coboundary_reduce` yields the
representative; `holonomies` and the residual field yield the boundary
content; `Delta` is the cell layer. The lens invents no magnification —
it **selects a depth** on a decomposition the algebra performs anyway.

This is why zoom is lawful rather than cosmetic: the far view is not a
blurred copy of the near view, it is the **gauge-invariant content** of
it. Zooming out does not lose precision; it removes frame.

---

## §4 RENDER ON DEMAND — the observable universe

An observable universe must render only what is observed, at only the
depth it is observed at.

The peel gives the mechanism for free, because the depths differ in cost
by construction: the reduced representative is small, the raw relation
is `O(dim² · fiber)`. An observer at distance reads the outer peel; a
peel is descended only on approach.

**The bound is the observer's own capacity, not a frame budget**
(`orbital/kinetic.rs`, LAW-13): *the entity cannot see deeper than its
capacity allows, regardless of pressure — the effective limit is
`min(requested, κ)`.* That makes level-of-detail a LAWFUL property of
the observer rather than a performance hack, and it means a lens can
never be pressured into rendering more than the observer has standing to
see.

---

## §5 RESONANCE — PROPAGATION, not a matching threshold

**THE CORRECTION (operator, 2026-08-02), and it reaches the whole
section.** Resonance is NOT a static match against a threshold. It is
the dynamic RATE AND BOUNDARY function governing how an originating
volume expands, contracts, degrades and accelerates over time through
the medium surrounding it.

```text
    PACE        fast / short  vs  long / hard
    DIRECTION   outward expansion  vs  inward contraction
    MEDIUM      the accompaniment accelerates or decelerates the motion
    DECAY       degradation from the origin, over elapsed steps and
                over distance
```

The ancestor's `|angular_valency − spin| ≤ tolerance` is a threshold
test, and a threshold test is a photograph of something that is only
meaningful as a film. **What was landed here first — a depth-indexed
match — is honest and is NOT resonance**; it is renamed to what it does
(`Alignment`, §13) and resonance is rebuilt as propagation.

### 5a. THE PROPAGATION PHYSICS, IN DISCRETE EXACT FORM

**The relayed form is continuous and cannot be transcribed:**

```text
    RELAYED (not exact-rational):
        V(t) = V0 · exp( ∫ (expand − contract) dt )
        a(t) = d/dt ( velocity · η_accompaniment )
        D(t,d) = decay_consumed(t) · distance_from_origin(d)
```

`exp` is TRANSCENDENTAL and `∫` / `d/dt` are continuous. This substrate
admits neither: exactness is a type-level guarantee, and **the chain is
discrete — one commitment per step, which is `c` itself** (LAW 5).
Transcribing the relayed form would put an approximation into the
substrate, which is the fault this document has now caught four times.

**The physics survives whole. Steps ARE the clock, so every derivative
is a difference and every integral is a sum:**

```text
    VOLUME          the extent standing at a step
    EXPANSION       V(k) − V(k−1), DIRECTION PRESERVED
                    positive = outward, negative = inward
    VELOCITY        expansion per chain step — the step is the unit
    ACCELERATION    the SECOND difference, modulated by the medium
    DEGRADATION     decay consumed over elapsed steps × distance
                    from the origin (wrapped L1, `key_distance`)
```

Nothing is collapsed to a rate-scalar; the direction survives every
reading, because *outward* and *inward* are the whole content and a
magnitude discards exactly that.

### 5b. THE MEDIUM — the accompaniment that speeds or damps

A volume does not propagate through nothing. The surrounding economic
medium **modulates the velocity**: it either accelerates the motion
(coherence building) or decelerates it (variance damping). The medium is
not a metaphor — it is the standing economic surround: what decays, what
is owed, what capacity stands nearby.

```text
    η > 1   the accompaniment ACCELERATES — the volume propagates
            faster than it would alone
    η = 1   inert — the medium neither helps nor hinders
    η < 1   the accompaniment DECELERATES — the medium damps
```

**η is DERIVED, never dialed** (LAW 6), for the reason §5c gives.

### 5c. WHY NO TOLERANCE CONSTANT SURVIVES

The earlier draft here proposed a fixed ruled tolerance, then a
depth-derived one. **Both are superseded**: under propagation there is no
threshold at all to set. A reading of resonance reports a PACE, a
DIRECTION and a DEGRADATION — three quantities carried whole — and never
a verdict of *matched / not matched*.

That removes the attack surface entirely rather than shrinking it.
Article III struck `α`/`β` as *"the bounded dynamic parameters a descent
needs to be tuned"*; a reading with no parameter has nothing to tune,
and two observers reading the same chain get the same pace because a
difference of two committed states is not a matter of opinion.

## §6 THE APERTURE — per-observer, without belief

`perception/harmonic_tuning.rs` carries the per-observer bias: an
11-channel exact-rational weighting under which an entity reads what it
sees. *"Each entity is the same kernel viewing the same observation
through its own chromatic spectrum."* The compound is a bounded
running average, `h[k]' = (h[k]·n + signal)/(n+1)`, exact throughout.

This is how a lens becomes per-observer without violating LAW 9. **No
belief enters any rule.** The substrate is identical for every observer;
what differs is the aperture through which it is read, and an aperture
is a measured weighting rather than a conviction. Two observers may
report differently and both be right, because they are reporting
different projections of one manifold — which is §2b restated at the
observer.

---

## §7 WHAT MUST BE TRANSLATED, NEVER TRANSCRIBED

The ancestor is consulted, never ported (`00_CHARTER` §5). Two of its
forms are BARRED here and must not arrive intact.

**1. The coherence velocity of Theorem β.**

```text
    BARRED:  v(T) = 1/(1 + ‖Δₜ‖_L1) − 1/(1 + ‖Δₜ₋₁‖_L1)
```

Two independent faults. It is the exact `1/(1+x)` shape of the struck
cosmetic meter `Ξ = deeds/(1+ΔD)`; and it collapses a directional
quantity to a scalar magnitude, which `18_TORSION` §2b bars wherever the
system computes.

**The idea survives and is worth having:** velocity as a DIFFERENCE OF
READINGS ACROSS CHAIN STEPS, derivable from the chain alone with no
future state. The lawful form carries it **per index and per channel**,
never as a norm — the same discipline `Extent` already keeps by
reporting per index rather than one capacity number.

**2. `KineticStep::Converged { action_magnitude }`.** A magnitude again,
and barred for the same reason. The step's outcome may carry its
per-index difference; it may not carry its length.

**The general rule for this lane:** the ancestor's SHAPES are reference;
every scalar, scored, or capped form is refused. That is the charter's
standing instruction on ancestry, and this document is bound by it.

### 7a. A HARNESS IS NOT A MIRROR — and it is where frequencies live

The consultation ledger — proposals offered against proposals withdrawn
at the sector door — looked like a lens obligation and is not one, and
the distinction matters beyond this instance.

**A ledger ACCUMULATES. That is a second record**, and §1a forbids it:
a running tally inside the live system is a copy that can disagree with
the chain, and the reconciler that follows is exactly what §11 rule 5
clause 1 bars. A withdrawal at the door is not a receipted deed — it
never reaches the chain — so there is nothing for a lens to derive a
frequency FROM. Counting it privately would be inventing the record.

Two lawful homes, and neither is a lens:

1. **RECEIPT THE WITHDRAWAL.** If the proposal-to-verified ratio is to
   be a property of the world, the withdrawal must become a deed and
   ride the chain like any other. That is codec and kernel work, not
   this lane's, and it is the heavier of the two.
2. **CONDUCT IT AS AN EXPERIMENT.** A harness runs `N` trials and
   reports what happened in those trials. It reflects no committed
   state, holds no second image of the world, and is deletable without
   loss — so it is neither lens nor mirror. It is an instrument that
   answers *"what does this search do?"* rather than *"what is the world
   holding?"*

**The second is correct for now**, and it is where the calibration
harness already lives — it tracks hit-rates per rung by running trials,
not by remembering. The ledger belongs beside it.

**The general form:** where a frequency is wanted, either the events are
receipted and the frequency DERIVES, or it is measured by EXPERIMENT.
There is no third option in which a lens keeps count.

---

## §8 WHAT THIS LANE DOES NOT TOUCH

- `lawful()` and everything in `engine/kernel/src` — kernel lane.
- `engine/rfa-core/` — kernel lane. The lens CONSUMES `coboundary_reduce`,
  `holonomies`, `Delta`, `Vacuity`; it does not modify them.
- `18_TORSION`, `17_CAPACITY`, `EQUATIONS`, `kernel/CHARTER.md` — kernel
  lane's documents. Defects found in them are REPORTED, never edited
  from here.

---

## §9 THE OPEN REPORT — a stale debt pointing at struck law

Two live pointers aim at a STRUCK meter: `18_TORSION` §6 debt 1 reads
*"`Ξ_cosmetic` is not measured … First build"*, and §7b routes an
*"Art. III window reading"* to `scribe`/`lens`.

**Both point at law that no longer exists.** Article III was struck whole
and rewritten as THE SCIENCE FOUNDATION; its strike list names
`Ξ(w) = deeds(w)/(1+ΔD(w))` and the term "cosmetic non-utility"
explicitly, and its reasoning is that *physics needs no penalty — a
closed loop at uniform potential does no work, not because a law forbids
it but because there is no free energy in it.* LAW 4 now carries that
ground, via `dual_pole` / `Vacuity::Stasis`, already landed and already
consulted by `kernel::capacity_law`.

Building the window reading would implement a struck law. It is not
built. The stale debt is reported to the kernel lane rather than edited
from here (§8).

---

## §11 KINEMATIC PRESSURE — the strain of a twist

**The layer this system was missing.** `orbit_lock`, `holonomies` and
the fold gate all detect that a twist IS. Nothing read how hard it
pushes, so an enormous static structure and one straining to move were
indistinguishable.

    manifold layer   torsion = the TWIST event
    kinematic layer  the PRESSURE of that twist — the strain it generates

**The pressure is the curvature, and it was already landed.**
`rfa_core::fiber` (DIRECTIVE 17) states why a flat fiber could never have
shown it: with a scalar connection `F = dA` and `[A, A] = 0` identically,
so a closed loop of scalars is *"a flat, dead circle."* With a matrix
connection the second term survives — `F = dA + A ∧ A` — and **the
circulation IS the field**.

| reading | what it says |
|---|---|
| `Pressure` | the curvature of a cell, carried whole. Zero is a FACT — these directions commute — never an absence |
| `PhaseLock::holding_pressure` | how far the accumulated transport sits from having closed: the force required to maintain the lock |
| `Drift` | how a reading moved between two chain points, per component |

**Two corrections to the ancestor, both deliberate.** `Drift` reports
**STILL**, never *"Converged"* — a structure that did not move may be at
rest or may be stuck, and the reading refuses to presume which. And it
takes two points FROM THE CHAIN and holds neither, because a lens
keeping its own timeline is a second timeline (§1a).

## §12 THE STANDING SURFACE — which row a state occupies

The kernel refuses a work-anchored mint unless the prior state reads
`Entangled`, and no surface showed an operator which row anything stood
in — a refusal arrived with a verdict nobody could have anticipated.

`lens::standing` reads it and carries BOTH poles, so the row is
explicable rather than pronounced. **`Stasis` is rendered as a state
wanting re-volumizing, never as a fault**: LAW 4 is explicit that the
refusal is thermodynamic rather than moral, and a surface that painted
it as an error would reintroduce the penal frame Article III struck.

## §13 THE OBSERVER — capacity, and the tolerance that is a position

**Capacity bounds depth**, `min(requested, κ)`. A request beyond
standing is ANSWERED at the depth the observer can hold, never refused.
The bound is applied BEFORE any work — a shallow observer asking for the
near view materialises the far view and nothing else. That is the
laziness of §4, and it caches nothing.

**The peel indexes ALIGNMENT** — *do these two read the same at this
distance?* That is a real question and it is NOT resonance (§5).
`observer::Capture` carries it, and `propagation` re-exports it under
the name ALIGNMENT to say so plainly. Resonance is a propagation across
a span, not a comparison at an instant.

```text
    FAR    compare CLASSES         discrete; no numeric tolerance exists
    MID    compare CIRCULATION     relative, cross-multiplied
    NEAR   compare EXACT ENTRIES   tolerance ZERO — byte-identity
```

To loosen the alignment test IS to step back; to tighten it IS to
approach. It is derived rather than dialed, and **two observers standing
in the same place always agree** — which LAW 6 requires and a
caller-supplied `ε` could never give.

`Capture` has three arms, and the third matters: **`Silent` is not
`Mismatch`.** Nothing to compare is an absence of subject, and reporting
it as a disagreement would be inventing a difference.

## §14 THE LANE, AS LANDED

| module | reading | never |
|---|---|---|
| `peel` | magnification as a depth on the coboundary decomposition | stores a view |
| `luminal` | what emerges at a phase through an aperture | returns a magnitude |
| `kinematic` | the strain of a twist, per component | collapses to a norm |
| `standing` | which row a state occupies | gates on it |
| `observer` | how deep one may see; ALIGNMENT at a distance | takes a dialed tolerance |
| `propagation` | RESONANCE — pace, direction, medium, degradation across a span | collapses a rate to a magnitude |

Held by `lens/tests/deletability.rs`: the sealed core names no lens
crate in any dependency table, the ledger takes no runtime edge, and the
one lawful client edge is pinned by SHAPE — a client may render, and may
never decide on a reading.

**The frequency question is answered outside this lane entirely** (§7a):
`conductor/examples/consultation.rs` measures the door's withdrawal rate
by EXPERIMENT. It reflects no committed state and is deletable without
loss, so it is neither lens nor mirror.
