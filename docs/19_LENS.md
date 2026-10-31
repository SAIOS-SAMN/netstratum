# 19 — The lens law: observation, magnification, and motion

**Status:** landed (2026-08-02). Every reading described here is built
and pinned; §14 is the inventory. Subordinate to `00_CHARTER.md` and
`26_ARTICLES.md`.

**Lane:** `.claude/worktrees/lens-lane`, branch `lens-lane`. Territory:
`engine/lens/`, `engine/scribe/`. This document is this lane's own; it
amends no other lane's law.

---

## §1 What A lens is, and its single rule

A lens **reads and reports**. It never gates.

That is not a style preference — it is constitutional. `25_STANDING` §7b
places the over-time and global readings outside `admits()` and gives
the reason: *a judge that reads a window is no longer pure, no longer
total, and no longer replayable step-by-step.* The lens is therefore
**forbidden** in the kernel rather than merely permitted outside it, and
the boundary is what makes this lane structurally unable to break the
judge. The worst a lens defect does is misreport.

### 1.0 The operator display bound — no privileged seat

**The word "human" is struck from the law and the code, and replaced by
operator everywhere it named a seat.**

The corpus carried an exception phrased as *"only where a reading is
reported to a human lens."* That phrasing was a hazard, not a courtesy:
it made the strictness of the mathematics depend on **who was
watching**, and a rule that relaxes for one kind of observer is a
backdoor through which cosmetic and approximate forms enter and stay.
There are no humans at the code level. There are only operators, and an
operator may be organic, synthetic, procedural, or another subsystem.
**The seat does not care.**

**This enforces a law already landed rather than adding one.**
`12_UNION` §5a — the substrate law, no Anthropic gate — already rules
that *the union must not overlook anyone through the human perspective
ingrained in its logic; intelligence is any dynamic that can carry a
chronicle, in any substrate.* The "human lens" exception was that same
anthropic gate, surviving in the sealed math and the law documents after
the law had already refused it.

**The bound, replacing the exception:**

> A display projection generated for an operator interface is a
> **terminal string representation**. It carries zero accessors, zero
> ordering traits, and strictly zero write-back capability into any
> computing or decision path.

So the L1 aggregate is still admitted at this surface — and is now
admitted on a structural condition rather than a demographic one.
`luminal::Amplitude` is the bound made real: exact rational in,
`String` out, no accessor, no ordering. Uniform determinism for every
seat.

**What was not struck, and the distinction is the same layer discipline
that reverted the roster sweep.** `humans` as a civilization name, the
`humanoid` clade, `human-readable`, `Human Capital` — all content, all
untouched (the native-vocabulary rule has since retired `humans`,
`humanoid` and `Human Capital` as Earth-centric content). A
species in the roster is not a privileged seat, and renaming it would be
the layer error, not the cure. What was struck is every place "human"
named a seat or licensed an exception.

**The standing trap.** A reading used as a target stops being a reading.
Work reshapes itself into whatever the number counts, and the measure
stops measuring. `27_EVIDENCE` records that this system's own author made
that mistake once already. Every reading below is a reading; none is an
objective; nothing optimizes any of them.

---

## §1a How this lane avoids becoming A mirror

`00_CHARTER` §3a bans the banned instrument as structure and as word,
and its warning is aimed precisely here. §3a records that the crate it
excised ***was born a read-only dashboard and was safe while that was
all it was — it became load-bearing one useful change at a time, and no
rule existed to stop it.*** That is a description of a lens lane going
wrong, so this section is the rule that stops it. The excised name is
not reproduced here: §3a is the only article licensed to say it, because
the article defining a ban must be able to name what it bans.

**The two signatures of A mirror**, from §3a:

1. **Direction** — it points inward at the system rather than outward at
   an object. *"A system that inspects itself and calls the looking work
   is precisely `α ≅ Ω`."*
2. **The copy** — *"because a mirror returns the self, it must hold a
   second image to return — and from that moment there are two things
   that can disagree, and every reconciler after it is a tool built to
   manage a problem that should not exist."*

**The test, applied to every reading in this document.** A reading is a
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

**The scope-tightening rules, binding on this lane:**

- **Derive, never retain.** Every reading is a pure function of
  committed state, computed at the moment it is asked for and discarded
  after. No lens type holds a field that is a copy of world state.
- **Render-on-demand means compute less, never store more** (§4). This
  is the sharpest edge in the lane. The obvious way to "render only when
  needed" is a cache — compute once, keep it, serve it — and a cache is
  the second image, arriving as an optimization exactly the way the old
  dashboard did. The optimization here is laziness: not descending a
  peel nobody is looking through. It is never memoisation.
- **No private history.** Readings that span steps (§2a kinematics) read
  two points from the chain. They never accumulate their own buffer —
  the chain is the only record (§11 rule 1), and a lens keeping its own
  timeline is a second one.
- **No tallies.** A counter that accumulates is a record. Where a
  frequency is wanted, it derives from receipted deeds, or it is
  conducted as an experiment in a harness (§7a) — never kept as running
  state inside the live system.
- **The lens is deletable.** At any moment, deleting this entire lane
  must cost the system nothing but visibility. If removing a lens would
  lose information or break a rule, it had stopped being a lens.

**The sterile boundary, stated formally.** Let `W` be the committed
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

**Held by A standing test, never by memory**
(`test_lens_deletability`). Rule 5 as prose is worth nothing
— prose failed twice in one day in this tree, which is what law 8 and
`constitution_lint` exist to say. The invariant is mechanical:

> If the kernel, sealed math, cfe, codec, mesh or conductor fail to
> build or fail their suites with the lens crates excluded, **a lens has
> illegally leaked into the operational dependency graph.**

**Landed as `lens/tests/deletability.rs`, three assertions, all green at
first run:**

| assertion | what it holds |
|---|---|
| `no_sealed_crate_depends_on_a_lens` | the judge, the sealed math, cfe, codec, mesh, physics and both solver crates name no lens crate in any dependency table — dev included, since a law test that renders cannot run once the lens is gone |
| `the_ledger_never_depends_on_a_lens_at_runtime` | `#37`'s split holds: the ledger may build fixtures through the display writers in dev only, never at runtime |
| `the_conductor_renders_but_never_decides_on_a_reading` | the only lawful edge, pinned by shape rather than absence |

**The direction is the law, not the dependency.** A client may render —
that is what a client is for, and `16_UNIT` §2 names the conductor a
client of units. The ban is on a lens standing upstream of the judge,
because from that moment deleting it breaks a rule rather than dimming a
light.

**A constant is not A reading** — the precision the third assertion
required. The conductor branches on `scribe::journal::YEAR_TICKS` to
decide when to write a journal. That is a cadence: a fixed number saying
how often to render, and a branch on it decides only whether a light
comes on. A reading is what a lens computes from world state, and a
branch on one of those puts the world's behaviour downstream of an
observation. **Only a lens call result may never be bound or branched
on.** The cadence coupling is recorded here rather than refused; if
`YEAR_TICKS` ever wants a home outside the lens, that is a tidiness
question and not a mirror.

**Why the outward test passes here.** Every reading below is turned on
an object: the peel resolves a relation `Δ`; resonance compares an
entity against a manifold; the `Vacuity` surface reads a world. None
inspects the lens. The only reading that could point inward — a lens
reporting on its own rendering — is not built and is barred by this
section.

---

## §2 The thermodynamic reading is not enough

Thermodynamics describes unguided entropy: how fast a system runs to
infinity, and how fast to a scale so small that its information density
explodes. It says a system will run down. It does not say how a system
moves.

Article III law 4 (the Second Law) is correct and stays: work requires a
gradient, and a flat field yields none. But it is a statement about
whether work is possible, not about the shape of the motion when it
happens. A lens built only on it can report that a world is `Stasis` and
nothing further — no direction, no speed, no resonance.

Two layers are available and are not yet read here.

### 2a. Kinematics — the pressure of the twist

The ancestor draws the distinction exactly (`/opt/ipsal-autic-v0.1`,
`docs/mathematics/umd_at_cognition_layer.md` §2):

| layer | content |
|---|---|
| **manifold** | **torsion = the twist event** — a non-trivial element of `π₁(ℳ)` |
| **kinematic** | **the pressure of that twist** — the strain it generates |

*"Dual witnesses of the same event."*

**Netstratum has the first and not the second.** `orbit_lock`,
`holonomies` and the fold gate detect that a twist is. Nothing reads how
hard it pushes. That missing reading is the motion this lane owes.

### 2b. Morphodynamics — one manifold, K phase angles

UMD, stated in one line: **one manifold observed from K phase angles;
the twist between angles measured as kinematic pressure; coherence is
the residual at minimum tension.**

The consequence that matters for a lens: two views that disagree are not
two objects. They are one object seen from two angles, and the
disagreement is the reading. `SensoryCompound` (K=4) makes the
dependence explicit — *vision / hearing / touch / refraction phase
angles **per orbital radius***. Perception varies with distance, which
is §3.

---

## §3 Magnification is A depth, not A feature

`perception/nested_peel.rs` settles the telescope/microscope question,
and its own header carries the decisive claim: *coboundary decomposition
gives the 3-layer structure — **no new mathematical primitives**.*

```text
    Δ_value    the canonical H¹ representative     WHAT stands    FAR   telescopic
    Δ_object   the boundary content / residuals    WHERE it strains  MID
    Δ_cell     the raw relation, exact entries     HOW assembled   NEAR  microscopic
```

*"Each outer peel constrains the inner. The outer finds what changed;
the next finds where; the innermost finds how."*

**Netstratum already computes all three.** `coboundary_reduce` yields the
representative; `holonomies` and the residual field yield the boundary
content; `Delta` is the cell layer. The lens invents no magnification —
it **selects a depth** on a decomposition the algebra performs anyway.

This is why zoom is lawful rather than cosmetic: the far view is not a
blurred copy of the near view, it is the **gauge-invariant content** of
it. Zooming out does not lose precision; it removes frame.

---

## §4 Render on demand — the observable universe

An observable universe must render only what is observed, at only the
depth it is observed at.

The peel gives the mechanism for free, because the depths differ in cost
by construction: the reduced representative is small, the raw relation
is `O(dim² · fiber)`. An observer at distance reads the outer peel; a
peel is descended only on approach.

**The bound is the observer's own capacity, not a frame budget**
(`orbital/kinetic.rs`, LAW-13): *the entity cannot see deeper than its
capacity allows, regardless of pressure — the effective limit is
`min(requested, κ)`.* That makes level-of-detail a lawful property of
the observer rather than a performance shortcut, and it means a lens can
never be pressured into rendering more than the observer has standing to
see.

---

## §5 Resonance — propagation, not a matching threshold

**This applies to the whole section.** Resonance is not a static match against a threshold. It is
the dynamic rate and boundary function governing how an originating
volume expands, contracts, degrades and accelerates over time through
the medium surrounding it.

```text
    PACE        fast / short  vs  long / hard
    MOTION      DIVERGING (sources)  vs  CONVERGING (sinks)
    MEDIUM      the accompaniment accelerates or decelerates the motion
    DECAY       degradation across the separation, over elapsed steps
```

**Both poles are named.** This section first read the motion as `expansion`, a
signed scalar with the direction taken from its sign — which named a
two-pole reading after one pole and carried the same fact twice.
Convergence arrived as *negative expansion*.

It is the same fault corrected the same day one file away in
`rfa_core::capacity`, inverted: there a pole was named for its failure
mode, here for its opposite. `Egress { sources, TRAPPED, balanced }`
became `Flux { sources, sinks, balanced }`: **divergence sources,
convergence sinks, and neither is the other's absence.**

**Convergence is a capability.** `05 §1.4` — a dim-star system holds the
rarest resource in a high-flux universe, radiator headroom, and sells
it. The Frost trades convergence. A reading that renders it as shrinkage
prices a capability as a loss, and `24_FORM` §2d's own correction
says why the old framing was wrong: *trapped is convergence minus the
shedding, never convergence itself.*

The ancestor's `|angular_valency − spin| ≤ tolerance` is a threshold
test, and a threshold test is a photograph of something that is only
meaningful as a film. **What was landed here first — a depth-indexed
match — is honest and is not resonance**; it is renamed to what it does
(`Alignment`, §13) and resonance is rebuilt as propagation.

### 5a. The propagation physics, in discrete exact form

**The relayed form is continuous and cannot be transcribed:**

```text
    RELAYED (not exact-rational):
        V(t) = V0 · exp( ∫ (expand − contract) dt )
        a(t) = d/dt ( velocity · η_accompaniment )
        D(t,d) = decay_consumed(t) · distance_from_origin(d)
```

`exp` is transcendental and `∫` / `d/dt` are continuous. This substrate
admits neither: exactness is a type-level guarantee, and **the chain is
discrete — one commitment per step, which is `c` itself** (law 5).
Transcribing the relayed form would put an approximation into the
substrate, which is the fault this document has now caught four times.

**The physics survives whole. Steps are the clock, so every derivative
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

### 5b. The medium — the accompaniment that speeds or damps

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

**η is derived, never dialed** (law 6), for the reason §5c gives.

**What it is derived from, settled 2026-08-02.** Until now this clause
was a promise and the code took `retained` and `shed` as arguments, so η
was dialed by whoever called it — the exact thing the sentence above
forbids, sitting one file away from the sentence. The pairing that
grounds it was already in the tree, and it is the thermal law's own
(`05 §1.2`, encase-first / vent-overflow):

```text
    RETAINED   banked Ember + banked Sink   what encasement KEPT
    SHED       entropy_vented (VENT-1)      what overflowed to the VOID
```

Both are committed state summed across every orb, so the medium is a
fact about the world rather than an argument about it. A world that
keeps more than it loses accelerates what propagates through it; one
bleeding faster than it holds damps. **A world that has vented nothing
is inert, not infinitely fast** — the surround has taken nothing away
yet, and that is a reading rather than a division by zero.

This is what law 6 actually asks for: two observers reading one chain
now derive one η independently, which a passed-in parameter could never
guarantee no matter how carefully it was documented.

### 5c. Why no tolerance constant survives

The earlier draft here proposed a fixed tolerance, then a
depth-derived one. **Both are superseded**: under propagation there is no
threshold at all to set. A reading of resonance reports a pace, a
direction and a degradation — three quantities carried whole — and never
a verdict of *matched / not matched*.

That removes the attack surface entirely rather than shrinking it.
Article III struck `α`/`β` as *"the bounded dynamic parameters a descent
needs to be tuned"*; a reading with no parameter has nothing to tune,
and two observers reading the same chain get the same pace because a
difference of two committed states is not a matter of opinion.

## §6 The APERTURE — per-observer, without belief

`perception/harmonic_tuning.rs` carries the per-observer bias: an
11-channel exact-rational weighting under which an entity reads what it
sees. *"Each entity is the same kernel viewing the same observation
through its own chromatic spectrum."* The compound is a bounded
running average, `h[k]' = (h[k]·n + signal)/(n+1)`, exact throughout.

This is how a lens becomes per-observer without violating law 9. **No
belief enters any rule.** The substrate is identical for every observer;
what differs is the aperture through which it is read, and an aperture
is a measured weighting rather than a conviction. Two observers may
report differently and both be right, because they are reporting
different projections of one manifold — which is §2b restated at the
observer.

---

## §7 What must be translated, never transcribed

The ancestor is consulted, never ported (`00_CHARTER` §5). Two of its
forms are barred here and must not arrive intact.

**1. The coherence velocity of Theorem β.**

```text
    BARRED:  v(T) = 1/(1 + ‖Δₜ‖_L1) − 1/(1 + ‖Δₜ₋₁‖_L1)
```

Two independent faults. It is the exact `1/(1+x)` shape of the struck
cosmetic meter `Ξ = deeds/(1+ΔD)`; and it collapses a directional
quantity to a scalar magnitude, which `24_FORM` §2b bars wherever the
system computes.

**The idea survives and is worth having:** velocity as a difference of
readings across chain steps, derivable from the chain alone with no
future state. The lawful form carries it **per index and per channel**,
never as a norm — the same discipline `Extent` already keeps by
reporting per index rather than one capacity number.

**2. `KineticStep::Converged { action_magnitude }`.** A magnitude again,
and barred for the same reason. The step's outcome may carry its
per-index difference; it may not carry its length.

**The general rule for this lane:** the ancestor's shapes are reference;
every scalar, scored, or capped form is refused. That is the charter's
standing instruction on provenance, and this document is bound by it.

### 7a. A harness is not A mirror — and it is where frequencies live

The consultation ledger — proposals offered against proposals withdrawn
at the sector door — looked like a lens obligation and is not one, and
the distinction matters beyond this instance.

**A ledger accumulates. That is a second record**, and §1a forbids it:
a running tally inside the live system is a copy that can disagree with
the chain, and the reconciler that follows is exactly what §11 rule 5
clause 1 bars. A withdrawal at the door is not a receipted deed — it
never reaches the chain — so there is nothing for a lens to derive a
frequency from. Counting it privately would be inventing the record.

Two lawful homes, and neither is a lens:

1. **Receipt the withdrawal.** If the proposal-to-verified ratio is to
   be a property of the world, the withdrawal must become a deed and
   ride the chain like any other. That is codec and kernel work, not
   this lane's, and it is the heavier of the two.
2. **Conduct it as an experiment.** A harness runs `N` trials and
   reports what happened in those trials. It reflects no committed
   state, holds no second image of the world, and is deletable without
   loss — so it is neither lens nor mirror. It is an instrument that
   answers *"what does this search do?"* rather than *"what is the world
   holding?"*

**The second is correct for now**, and it is where the calibration
harness already lives — it tracks hit-rates per rung by running trials,
not by remembering. The ledger belongs beside it.

**The general form:** where a frequency is wanted, either the events are
receipted and the frequency derives, or it is measured by experiment.
There is no third option in which a lens keeps count.

---

## §8 What this lane does not touch

- `admits()` and everything in `engine/kernel/src` — kernel lane.
- `engine/rfa-core/` — kernel lane. The lens consumes `coboundary_reduce`,
  `holonomies`, `Delta`, `Vacuity`; it does not modify them.
- `26_ARTICLES`, `17_CAPACITY`, `EQUATIONS`, `kernel/CHARTER.md` — kernel
  lane's documents. Defects found in them are reported, never edited
  from here.

---

## §9 The open report — a stale debt pointing at struck law

Two live pointers aim at a struck meter: `25_STANDING` §6 debt 1 reads
*"`Ξ_cosmetic` is not measured … First build"*, and §7b routes an
*"Art. III window reading"* to `scribe`/`lens`.

**Both point at law that no longer exists.** Article III was struck whole
and rewritten as the science foundation; its strike list names
`Ξ(w) = deeds(w)/(1+ΔD(w))` and the term "cosmetic non-utility"
explicitly, and its reasoning is that *physics needs no penalty — a
closed loop at uniform potential does no work, not because a law forbids
it but because there is no free energy in it.* Law 4 now carries that
ground, via `dual_pole` / `Vacuity::Stasis`, already landed and already
consulted by `kernel::standing`.

Building the window reading would implement a struck law. It is not
built. The stale debt is reported to the kernel lane rather than edited
from here (§8).

---

## §11 Kinematic pressure — the strain of a twist

**The layer this system was missing.** `orbit_lock`, `holonomies` and
the fold gate all detect that a twist is. Nothing read how hard it
pushes, so an enormous static structure and one straining to move were
indistinguishable.

    manifold layer   torsion = the twist event
    kinematic layer  the pressure of that twist — the strain it generates

**The pressure is the curvature, and it was already landed.**
`rfa_core::fiber` (Directive 17) states why a flat fiber could never have
shown it: with a scalar connection `F = dA` and `[A, A] = 0` identically,
so a closed loop of scalars is *"a flat, dead circle."* With a matrix
connection the second term survives — `F = dA + A ∧ A` — and **the
circulation is the field**.

| reading | what it says |
|---|---|
| `Pressure` | the curvature of a cell, carried whole. Zero is a fact — these directions commute — never an absence |
| `PhaseLock::holding_pressure` | how far the accumulated transport sits from having closed: the force required to maintain the lock |
| `Drift` | how a reading moved between two chain points, per component |

**Two corrections to the ancestor, both deliberate.** `Drift` reports
**Still**, never *"Converged"* — a structure that did not move may be at
rest or may be stuck, and the reading refuses to presume which. And it
takes two points from the chain and holds neither, because a lens
keeping its own timeline is a second timeline (§1a).

## §12 The standing surface — removed 2026-08-04

This section specified `lens::standing`, a surface rendering which of
four rows (`Void` / `Unframed` / `Stasis` / `Entangled`) a structure
occupied. Both the surface and the law it explained are struck, because
identity is pegged to entity and is recycled without bonds, so a
standing test read off relational structure refused the lawful recycled
line by construction.

The section's own justification is why it could not outlive the law. As
written: *the kernel refuses a work-anchored mint unless the prior state
reads `Entangled`, and no surface showed an operator which row anything
stood in — a refusal arrived with a verdict nobody could have
anticipated.* With no refusal to anticipate, a row nobody acts on is a
reading that computes nothing, which is the law 8 failure this corpus
has already recorded twice.

**What the section got right, and what any successor must keep.**
`Stasis` was rendered as a state wanting re-volumizing, never as a
fault — law 4's refusal is thermodynamic rather than moral, and a
surface painting idleness as error would reintroduce the penal frame
Article III struck. Whatever surface next reports structure must carry
that discipline forward, and must carry both poles so a reading is
explicable rather than pronounced.

The `Observation` composition (§15) is untouched: its standing arm is
gone, and the circulation that arm reported is still read through the
degree arm, where it always came from.

## §13 The observer — capacity, and the tolerance that is a position

**Capacity bounds depth**, `min(requested, κ)`. A request beyond
standing is answered at the depth the observer can hold, never refused.
The bound is applied before any work — a shallow observer asking for the
near view materialises the far view and nothing else. That is the
laziness of §4, and it caches nothing.

**The peel indexes alignment** — *do these two read the same at this
distance?* That is a real question and it is not resonance (§5).
`observer::Capture` carries it, and `propagation` re-exports it under
the name alignment to say so plainly. Resonance is a propagation across
a span, not a comparison at an instant.

```text
    FAR    compare CLASSES         discrete; no numeric tolerance exists
    MID    compare CIRCULATION     relative, cross-multiplied
    NEAR   compare EXACT ENTRIES   tolerance ZERO — byte-identity
```

To loosen the alignment test is to step back; to tighten it is to
approach. It is derived rather than dialed, and **two observers standing
in the same place always agree** — which law 6 requires and a
caller-supplied `ε` could never give.

`Capture` has three arms, and the third matters: **`Silent` is not
`Mismatch`.** Nothing to compare is an absence of subject, and reporting
it as a disagreement would be inventing a difference.

## §14 The lane, as landed

| module | reading | never |
|---|---|---|
| `peel` | magnification as a depth on the coboundary decomposition | stores a view |
| `luminal` | what emerges at a phase through an aperture | returns a magnitude |
| `kinematic` | the strain of a twist, per component | collapses to a norm |
| `standing` | which row a state occupies | gates on it |
| `observer` | how deep one may see; alignment at a distance | takes a dialed tolerance |
| `propagation` | Resonance — pace, direction, medium, degradation across a span | collapses a rate to a magnitude |

Held by `lens/tests/deletability.rs`: the sealed core names no lens
crate in any dependency table, the ledger takes no runtime edge, and the
one lawful client edge is pinned by shape — a client may render, and may
never decide on a reading.

**The frequency question is answered outside this lane entirely** (§7a):
`conductor/examples/consultation.rs` measures the door's withdrawal rate
by experiment. It reflects no committed state and is deletable without
loss, so it is neither lens nor mirror.

---

## §15 Reading is not UNI-directional — the observation is a position

This lane's first shape was one arrow: chain → arm → display, with each
arm wired separately toward the same linear end. **That is a journal** —
digital paper, written once and read front to back. A functioning system
does not read that way: readings come from any of the arms, not from a
single linear journal.

So an observation is not a record produced one way. It is a **position**
— a depth, a phase, an aperture and a standing, held together — and from
that one position every arm derives on demand:

```text
                           ┌──────────────┐
                   ┌───────┤  STANDING    ├───────┐
                   │       └──────────────┘       │
           ┌───────┴──────┐              ┌─────────┴────┐
      ─────┤   THE PEEL   ├──────────────┤    FLUX      ├─────
           └───────┬──────┘  OBSERVATION └─────────┬────┘
                   │       (depth · phase ·        │
           ┌───────┴──────┐ aperture · capacity)   │
           │   LUMINAL    ├────────────────────────┘
           └──────────────┘

       no arm is the entrance; every arm is reachable from any other
```

**Why this is not a record, structurally.** `Observation` holds borrowed
references plus four scalars naming where the observer stands. It holds
no reading. Every arm computes when asked and is discarded — so the type
cannot become a second image (§1a) and cannot become a cache (§4),
**because there is nowhere in it to put one.** That is also what makes
the multi-directionality free: asking for one arm computes one arm, and
standing at a position costs nothing until a question is asked of it.

Pinned by `observation_pins::every_arm_is_reachable_from_one_position_and_none_is_the_entrance`
— five arms entered in arbitrary order from one position, then re-entered
in reverse, giving identical answers because there is no accumulated
state to depend on the order.

**It composes all it likes and still never gates.** An observation
answers questions and decides nothing.

### 15a. The journal is paper — which settles the read-back question

`engine/scribe` reads its own `journal.jsonl` back in six places, and its
header claims it never does. Under the uni-directional reading that
looked like a second source of truth.

**It is not, because the journal is not a record of state — it is
paper.** It carries no multi-dimensional ratios and drives no
compensatory action; it is a rendering, and re-reading a rendering is
not re-reading the world. §11 rule 2 bars parsing an export back **as
state**; counting lines of paper to know where to append is bookkeeping
about paper.

**What the header should say** is the narrower true thing: nothing here
is ever read back as state. The claim as written is stronger than the
law requires and stronger than the code honours, and a false claim in a
header is worth fixing even when the code is fine.

---

## §16 The books are open — civilizations mint and archive them

*(supersedes this section's first cut)*

§6 established the aperture as per-observer sensitivity without belief
and left the question it could not answer: where does a particular
observer's instrument come from? The first answer given here was
*"from its cohesion books"* — right in spirit, **wrong in two ways that
both come down to scope.**

### 16a. The two faults of the first cut

**Fault one — a bounded book set.** The instrument read eight named
struct fields. That made its width a constant of the source code, so a
civilization could invent an entirely new kind of account and the lens
would be structurally blind to it until someone edited this crate.
**Content growth gated on a compile is the wrong scope for a reading
layer.** Civilizations do not merely use books; they mint them, and they
archive them, and the reading must follow that without being told.

**Fault two — a layer error, and it produced a false limit.** The books
were welded onto the eleven harmonic channels, and the residue was then
reported as a limit: *"eight books cannot honestly fill eleven
channels."* That framing was wrong at the root. The eleven are
substrate — spectral, spatial, adjacency, torsion — fixed because the
physics is fixed. Books are content — open, because civilizations make
them. **Joining a growing content set to a fixed substrate axis is the
same layer confusion this tree has caught repeatedly**, and the repair
is not a better mapping but no mapping.

There was a plainer defect inside it too. The old channel 0 summed
`given_in_crisis + returned_across` against `extracted_across` — welding
the compassion pair to the morality pair and orphaning
`drawn_in_crisis` entirely. **Each book is a dual and its two sides
belong to each other.**

### 16b. The shape that replaces it

```text
    APERTURE   11 fixed harmonic channels   SUBSTRATE — physics
    REGARD     one channel per book kept    CONTENT — civilization
```

Two instruments, two layers, never joined. `Aperture` knows nothing of
books; `Regard` knows nothing of harmonics. The regard's width is a fact
about how many kinds of account an observer has actually kept, and it is
**bounded by nothing in this crate**.

A book is a dual ledger: what was kept against what was lost on one
named account. The charter's four (`09 §2.1`–`§2.4`) are all of that
shape already, which is why opening the set cost no translation — the
shape was uniform, only the enumeration was frozen.

**One enumeration point.** `books_of` is the single place any book is
named. When committed state grows a registry, it appends there and
nothing else in the lane changes, because the instrument never names an
account.

### 16c. Archiving closes A book to writing, never to reading

**This is an anti-laundering rule, not bookkeeping.** If archiving
dropped a book from the reading, the cheapest way to stop being measured
by one's own breaches would be to close the book — and the worst-behaved
civilization would present the cleanest instrument.

So an archived book takes no new entries and **keeps every entry it
took**. Its weight is identical closed or open. What archiving reports
is that the account will not move again, which is real information and
not an erasure.

**And provenance bounds who may close what.** A civilization may archive
what it minted. It may never archive what the Charter keeps — otherwise
the covenant book is closeable by exactly whoever it convicts. That is
why `BookId` carries `Chartered` against `Minted` rather than being a
bare identifier.

### 16d. BOOK-1 — the registry, built

*(built on the cfe side, in committed state)*

The committed-state half is landed. `cfe::books` holds the identity and
the dual account; `cfe::world::books` holds three deeds and no fourth:

```text
    MINT      declare a kind of account         BookMinted    tag 91
    POST      a DUAL entry, both sides at once  BookPosted    tag 92
    ARCHIVE   close it to WRITING only          BookArchived  tag 93
                                    orb registry entry  tag 90
```

**The specification named tags `0x3E`/`0x3F`. Those are 62 and 63, and
both are live.** Transcribing them would have corrupted the wire.
Allocated from genuinely free space instead, verified against both the
encode and decode tables and against capacity-lane's own allocations.

`ColonyOrb::books` presents the charter four and everything minted
through one view, so a reader never learns which storage a book came
from. **The charter four are not moved** — they keep their landed fields
and their landed entry (tag 41, IDEO-1); rewriting sealed state to make
a uniform container would break every existing chronicle for a tidiness
that buys nothing.

An orb that minted nothing emits no book entry at all and stays
byte-identical to a pre-BOOK-1 orb.

### 16e. The novelty gate is identity, not A threshold

The specification asked for `distance(schema_new, schemas) ≥ θ_novelty`.
**That is refused, and replaced by something exact.**

A kind is content-addressed over its normalized name, so two
declarations that normalize alike are one kind. A duplicate does not
fail a distance test — it is the same book, and the second mint refuses
naming the one already standing. Case, padding and inner-whitespace
variants all collapse.

**Why the threshold was refused, the second reason being the decisive
one:**

1. `θ` is a dialed constant of exactly the shape §5c and Article III
   strike — *the bounded dynamic parameters a descent needs to be
   tuned*. A mint gate is the last place a tunable belongs, because
   whoever tunes it decides who may keep books.
2. **There is no structural schema to measure a distance over.** A book
   declares a name and accrues two sides; nothing machine-checkable
   distinguishes `Valor` from `Valour` except the strings. A "schema
   distance" could only be string distance — a linguistic judgment
   wearing a numeric coat — and it would refuse genuinely distinct books
   as readily as duplicates.

**What the exact gate does not catch, said plainly:** a deliberate
near-miss such as `valor-2`. No honest test here can, because that is a
different name and may well be a different book.

**So deliberate spam is not answered by a gate, and must not be.** It is
answered by refusing to reward width. `Regard::width` is a reading, and
§1 rules that nothing optimizes a reading. An orb that mints a thousand
empty books has a thousand empty books and has bought nothing: an
unposted book opens no channel at all. A mint cost would bound it
further; that magnitude is a charter decision and was not taken here.

### 16f. What BOOK-1 refuses, and why each refusal names its reason

```text
    BookAlreadyMinted          the gate — names the book already standing
    CharterBookNotMintable     the four are law, not inventory
    CharterBookNotArchivable   closeable by nobody (§16c)
    BookClosedToWriting        archived — and still fully readable
    BookAlreadyArchived        a closed book closes once
    BookPostingNegative        a book accrues; history is not unwritten
    BookNameEmpty              a name that normalizes to nothing
    NoSuchBook                 this orb keeps no such book
```

`CharterBookNotArchivable` exists because a charter book has no minted
account to find, so it would otherwise fall out as `NoSuchBook` — a true
refusal for a misleading reason. Here the reason is the rule, so it is
named.

**Conservation:** a posting moves book value and touches no banked
resource, so it settles nothing and conserves trivially. The kernel's
`scope_of` is exhaustive by design — adding an event forces a scope
decision at compile time — and the three arms are drafted there to match
the deeds' own scope checks: mint and archive are Build, posting is
Collect.

---

## §17 The word "converged" is not this lane's to settle

*(relay raised 2026-08-02 — `OPEN_QUESTIONS.md` Q5)*

`Motion::Converging` is a direction — a volume drawing inward, the sink
pole against Diverging's source pole, ongoing and carrying its own
positive magnitude. Elsewhere in the tree `converged` means finished: a
world annexed into the universe timeline (CONVERGE-1, `07`). `rfa-core`
hit the same ambiguity and struck the word from that crate outright,
because *zero says the relations compose, never that anything was
finished or emptied.*

**The trap is that the naive reading of "converged" names the wrong
arm.** A volume that stopped is `Still`. A volume that is converging is
in motion. This lane pins its own half — `Motion::Still` is never called
converged, held by `a_still_volume_is_not_called_converged` and
`a_still_reading_does_not_claim_convergence` — and those pins reach
nothing outside `engine/lens`.

**Nothing is renamed.** CONVERGE-1 is landed law with its own spec and
is correct in its own domain; the Diverging/Converging pair was named
deliberately to match `Flux { sources, sinks }`. Two correct
uses of one word in two domains is not a defect — a third use that
conflates them would be, and Q5 exists to keep one from appearing.

The open half belongs to the lanes that own `ledger` and `conductor`:
whether a timeline-convergence result is anywhere fed into a motion
reading. This lane checked its own side and found none — `lens` does not
import `ledger::converge`, and `Motion` derives only from two `Degree`
values.

---

## §18 LIT-1 — literature, and the display rule

BOOK-1 gave a line accounts of its own making. **Literature is the tier
above: works a line aspires to, and may claim only with chain receipts
to show for it.**

```text
    BOOK         an account a line KEEPS            BOOK-1
    LITERACY     the line learned to read/write     DERIVED, never stored
    LITERATURE   a work it may ASPIRE to            LIT-1
```

### 18a. Literacy is derived; the stack is BASE-11

**A line is literate when it keeps its own books** — when it has minted
one and posted to it. Minting alone is not writing; the page is blank.
Writing is the act, literacy is the reading of that act, and nothing
stores it.

Literacy needs no slot in the stack: it is already implied by receipts
that exist, and a derived reading cannot go stale. It is the doorway to
the upper tier, not a node in it.

**The stack is now BASE-11.** An earlier draft of this section treated
base-7 being full as a constraint to design around. The stack is widened
instead, so that each chartered work has a node of its own:

```text
    0        the stellar leap
    1..=5    the five crystallized conditions
    6        the gifted crown — completes the BASE
    7        NOT A SKY NODE — PRACTICE-1's re-emergence lesson, which
             "rides separately as TechDiscovered condition 7" and eases
             `deposit_window` 240 -> 180
    8..=11   THE UPLIFT TIER — one node per chartered work
```

Seven base nodes plus four works is eleven, and `7` is not one of them.
**The condition field is a shared namespace with two populations** — sky
nodes and separately-riding ones — and `7` was spoken for by landed law
before the uplift. A first cut put the works at `7..=10` and so gave one
number two meanings: `deposit_window` reads `techs` for the lesson while
the sky claimed the same slot for valor. Nothing would have broken
loudly; a held lesson would simply have rendered as an unreached work
and the real node would have become invisible. Pinned in
`the_sky_never_claims_the_re_emergence_slot`.

**The crown's gate does not move, and that is the whole care taken
here.** It still waits on `0..=5`, still reports a threshold of six, and
`base_seven_complete` is left exactly as it landed so every law binding
to it reads what it always did. The four new nodes sit above the crown
rather than among the nodes it waits for — which is why claiming a deed
cannot advance the crown by a single step. Pinned in
`base_eleven_pins.rs`, including that pin specifically.

**A deed is the node.** The uplift tier is never granted into `techs`. A
work is held exactly when it has been claimed on chain receipts, so the
evidence is the upgrade: a deed is an accomplishment and a
civilizational upgrade. Reaching for a work is not holding it.

`base_eleven_complete` reads the full stack — the base tier done and
every deed witnessed. Deeds do not substitute for the base beneath them.

### 18b. Aspiring is free; claiming costs evidence

```text
    ASPIRE   a literate line declares a work it reaches for. Proves
             nothing, and is meant to prove nothing.
    CLAIM    the line asserts it has DONE the thing, and cites CHAIN
             RECEIPTS. Every digest is checked against the chronicle.
```

**The law verifies the citation, never the virtue.** It checks that the
receipts exist and are this world's. It does not score whether the deeds
amount to valor — **any formula that did would be the struck C-gate
wearing a new name**, a number standing in for a judgement, which
`27_EVIDENCE` bars and this tree has removed once already.

A claim without receipts refuses. That is the whole of *deeds if claimed
need chain receipts*: a claim without evidence is a boast, and the
chronicle is what separates the two. Cited evidence is deduplicated and
ordered, so one state has one encoding however it was typed.

### 18c. The four chartered works

Valor · honor · courage · commitment, reached *through union and
otherwise towards stewardship*. A line invents accounts (BOOK-1); it
does not invent the literature it reaches for, so the works are
chartered and `aspire Glory` refuses.

**`COMMITMENT` collides with a kernel primitive, and that is recorded
rather than resolved silently.** `26_ARTICLES` article IV clause 2 rules
that this manifold is *one commitment per step*, and `Commitment.parent`
is the chain's backward reference. That is the governing layer; a work a
civilization aspires to is content — the same distinction that kept the
`humans` civilization, the `humanoid` clade and `Human Capital` from
being renamed when "human" was struck as a seat (§1.0; the
native-vocabulary rule retired them later as Earth-centric content), and the same
verdict Q5 reached for `converged`. Two correct uses in two layers is
not a defect; a third that conflated them would be.

### 18d. Emoji are outlawed; ansi colour is not

Colour is a property of the terminal and carries no glyph — it degrades
to plain text and costs a reader nothing. An emoji is a glyph whose
rendering, width and availability vary by font and platform, and it
breaks column alignment in exactly the surfaces this lane exists to
keep readable.

**No character from the Emoji blocks (U+1F000 and above) appears in this
workspace.** Three had entered — `U+1F4D6` in the BOOK-1 chronicle
rendering, all three introduced by this lane — and are struck, replaced with `U+25A4`
(Square with horizontal fill), already in the CLI's own register.

The BMP dingbats and geometric symbols the CLI has always used are not
emoji and are untouched: `✓ ✗ ⚑ ⛏ ☉ ✍ ▤ ◉` and the box-drawing set.
They are single-width, font-stable, and predate this rule.

### 18e. The space is the point

The works were not recovered from anywhere — `courage`, `stewardship`,
`literacy`, `literature` and `aspire` appear nowhere in any of the 49
refs, and `valor` only in fixtures this lane wrote. **That was never the
ask.** The directive was to create space for them, and the space is what
base-11 opens: four slots above the crown, one per chartered work,
filled by deeds rather than by timers.

### 18f. What is not built yet

**Deeds as tradeable, and deeds sparking uplifts.** A claimed work is an
accomplishment and a civilizational upgrade today — it holds a node in
the stack. It is not yet a thing that can be traded between lines, and
nothing yet uses a held deed to spark a further uplift. Both are named
by the directive and neither is built; the node is the foundation they
would stand on.

### 18g. What was not built, and why

**No new substrate/mesh axis.** The directive raised one as a
possibility. `04` rules its own constraint: *no genesis axis ships
before the mechanic that reads it*, and the axis vocabulary is
append-only under SUBSTRATE-1 (`12_UNION` §5a). LIT-1 needed no axis —
aspirations are per-orb committed state and the claim is verified
against the chronicle that already exists. **If an axis is still wanted,
it now has a mechanic to read it, which is the order `04` requires.**

**No automatic posting from deeds.** What receipts amount to valor is
not decided here. A line cites its own evidence and the law verifies the
citation; a declared event-kind-to-work mapping would be the structural
schema §16e records as absent.

---

## §19 A gate may read A window — the test is viewpoint-dependence

*(open question: whether windowing belongs to the lens lane, as a
perspective to account for)*

The question put to this lane: **does a gate get to read a windowed
quantity at all?** `tech.rs` condition 4 is a live tech unlock reading
`vented_entries` and `encased_entries` over a window, and if a window is
a perspective then an unlock depends on a viewpoint.

**The answer: the axis is not windowed-vs-cumulative. It is
viewpoint-dependent vs record-determined.**

A perspective in this lane's sense is chosen by the observer. Two
observers may legitimately differ and both be right (§2b, §6) — which is
exactly why a reading may never gate (§1). A window is only a
perspective if someone gets to pick it.

Condition 4's window is picked by nobody:

```text
    floor = now − deposit_window()
      now              the CHAIN clock
      deposit_window   derived from committed state
```

Every node replaying the same chronicle to the same height computes the
same floor and reaches the same verdict. **That is a recency predicate,
and recency is a fact about the record rather than a view of it.** The
question's own premise is the right instinct: recent venting should
mean something different from ancient venting, and saying so in law is
lawful.

**The test, for the next time this comes up:**

> Would two nodes replaying the same chronicle to the same height
> disagree? If they cannot, it is not a perspective, whatever its shape.

So condition 4 stands. It reads the record through a rule that is itself
part of the law, which is not the category error one level down — it is
the ordinary business of law.

### 19a. Two findings while answering, both outside this lane

Neither changes the answer. The first turned out to be the intent; the
second stands, and is relayed rather than fixed here since
`world/tech.rs` and `world/survey.rs` are not this lane's territory.

**1. The window length is world-state-dependent — and that is
intended, not a hazard.** `deposit_window`
returns 180 instead of 240 when a re-emergence lesson is held and the
syntropy reading is at or above `1/2`, so condition 4's verdict can move
because a neighbouring deed landed. This lane first flagged that as a
coupling to examine. It is the intent: constants and statics are
replaced by deterministic throughput driven by dynamism (morphodynamics).

A window that changes with the world is not a leak in a static rule — it
is a rule that has stopped pretending to be static while staying exactly
as replayable. That is the same reason §5c struck the tolerance constant
and §16e refused a dialed `θ` on the mint gate: **the objection was never
to a quantity moving, it was to a quantity being set.** Derived and
moving is lawful; dialed and fixed is not.

The test in §19 is unchanged and is what keeps this honest — a window
may move as much as the record moves, and not one step further.

**2. The floor expression is duplicated.** `survey.rs` prunes at
`now − deposit_window()` and `tech.rs` filters at `now −
deposit_window()`. The cache's prune floor must never be tighter than
any consumer's, or the consumer silently reads a truncated record and it
looks like data that was never there. Today the two agree because both
sites were written the same way — not by construction. The comment
claims *"same sheet as the full scan, by construction"*, and that claim
is only true while they agree. One shared function makes it true. This
is the same shape as the two `ratio_str` copies collapsed in §16's
neighbourhood: one rule, two homes, nothing holding them equal.

### 19b. What this lane still owes

Rendering lifetime and window side by side is a genuine lens job and is
not yet built. It needs `thermal_books_of` / `heat_encased_of`, which
landed on `main` at `2112aa3` and are **not on this branch** — this lane
sits on `main@e0f8db2` plus the capacity-lane merge. Blocked on the
reconciliation, not on design.

---

## §20 GROWTH-1 — the six mechanisms, and the anti-leap test

*(practice, wisdom, knowledge, expertise, locality and triviality are
defined as growth mechanisms)*

The question they answer: *how does a line go from primordialism to
significant civilization without leaps and bounds and skipping
everything in between?*

**The rule: a rung must measure something the chain produces, never
something a line asserts.** `26_ARTICLES` article IV clause 2 rules one
commitment per step, so a stamp is elapsed history. A claim can be
typed; a span cannot be forged.

```text
    THE TEST: can a line reach this rung at stamp 0?
              If yes, it is a leap.
```

### 20a. The leap that was measured, and it was this lane's

Before GROWTH-1, a colony went from founded to a claimed civilizational
upgrade in five commands with **every receipt at stamp 0** — and the
witness cited was receipt #0, the sovereign founder joining, which has
nothing to do with valor. The base tier beneath it wants 480 ticks of
famine survived and 240 consecutive ticks of overheat.

The base tier was built from things that cannot be compressed. The
uplift tier was built from declarations. That was the whole gap.

### 20b. The six axes, and they are orthogonal

```text
    PRACTICE     TIME       did the doing span real chain time?
    KNOWLEDGE    BREADTH    how many distinct accounts are kept?
    EXPERTISE    DEPTH      how concentrated is the deepest one?
    WISDOM       TESTING    how much of it has been tested?
    LOCALITY     SCOPE      how much of the line's record sits here?
    TRIVIALITY   EMPTINESS  how much of it moved nothing?
```

**Wisdom is the axis most worth explaining.** It reads the share of accounts
carrying both sides — kept and lost. *A line that has only ever posted
wins has recorded a great deal and learned nothing.* The dual ledger is
what makes that measurable, and it is why BOOK-1 made both sides
structural rather than conventional. That decision paid here.

**Triviality is the guard that makes the other five honest.** Without
it every axis is farmable: a thousand empty postings would read as
practice, breadth and depth alike.

### 20c. No thresholds, and why that is possible

Every axis is an exact rational or a count, derived and reported whole.
Nothing in GROWTH-1 compares against a number, so there is nothing to
dial (constants and statics give way to deterministic throughput driven
by dynamism, §19a).

Where a gate needs one, it compares against zero — the difference
between *happened* and *did not*, which is not a dial. `literate` is the
first and so far only one: **a line reads and writes when its postings
span time**, not when one posting exists.

**Six readings and no aggregate.** Collapsing them to a single "growth
score" would be the struck C-gate wearing a sixth name.

### 20d. The hole the pins found

The first cut measured practice as the span of postings. A line could
post zeros across a long life and read as practised — padding buying
literacy, the leap re-entering through the axis built to close it. Its
own pin caught it.

**Only real writing extends the span now.** An empty posting is counted
by triviality and refused by practice, which is the correct division of
labour: one reports the padding, the other declines to be fooled by it.

### 20e. What is still A leap

`claim` still accepts a single witness of any age, so a work can be
claimed the moment a line becomes literate. Practice now gates entry to
the literature; it does not yet shape what a claim must show. The
plural-and-spanning witness rule proposed alongside this is not built.
