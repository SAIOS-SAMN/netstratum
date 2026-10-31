# 48 — The capacity threat model: descent re-entry, bifurcation, and the withdrawal boundary

**Status:** subordinate to `26_ARTICLES.md`, `EQUATIONS.md` §1, and
`17_CAPACITY.md` (the D/V ontology this threat model is built on top of).
Where any disagree, the order is `00_CHARTER` → `26_ARTICLES` →
`EQUATIONS` §1 → `17_CAPACITY` → this document.

**Split out of `17_CAPACITY.md` §10–§13 on 2026-08-13 (docs audit).**
`17_CAPACITY` had grown to 2,248 lines reading as a chronological
log of decisions; §10–§13 is one continuous economic-security
argument — §10 names the descent-re-entry attack surface, §11 corrects
§10 (the bifurcation law), §12 draws a withdrawal-boundary conclusion
from §11, and §13 corrects §12 (polytonic value, the honeypot, altitude)
— built on top of the D/V ontology rather than part of it, so it is
filed separately. Section numbering is unchanged, matching the
`18_TORSION` split precedent (`24_FORM.md`, `22_CONTAINMENT.md`): a
citation to "17_CAPACITY §11" now resolves as "48_CAPACITY_THREAT_MODEL
§11".

**Read order matters here more than in most split docs**, because each
section explicitly amends or is amended by its neighbor: §10 is amended
by §11 (noted at §10's own close); §12 is amended by §13 (noted at §12's
own close). Read §10→§13 in sequence, not as independent entries.

---

## §10 The descent RE-entry vectors — attack surface and standing defenses

**Law.**

> Treating cocycle failure, `δΔ ≠ 0`, as a quantity to reduce is
> gradient descent re-entering the law.

**The law, stated exactly.** The cocycle condition is a refusal
predicate. A state composes or it is refused. It is **never a quantity
to be reduced, resolved, approached, scored, or improved toward.** There
is no lawful path the system walks from `δΔ ≠ 0` toward `δΔ = 0`;
"reducing cocycle failure" is a descent objective wearing the
vocabulary of well-formedness, and it is barred.

**Syntropy stands.** A rename of this term
was drafted and is withdrawn; `24_FORM` §2i carries the
full reasoning and the definition. In short: syntropy here is **the
exact inverse arm of entropy in one conserved partition** —
`syntropy = syntropic / (syntropic + entropic)`, so
`syntropy + entropy = 1` exactly — and it borrows nothing from any
external theory. It is landed, it computes, and it gates
(`World::syntropy_ratio()`, the 1/2 floor — `world/core.rs`), so by
this project's own grounding test it stays. No synonym is admitted for
it: one measurement, one name (`24_FORM` §2i).

**Consequently, syntropy contains no reduction term.** Syntropy is
defined solely by the growth of surviving structure against activity
spent — `Σ ≈ ΔD / deeds`. (It was first written here as the reciprocal
of Article III's cosmetic meter; that meter was struck by 26_ARTICLES §3, and the
reading stands on its own without it.)
It is not a second primitive and never gets its own quantity (two
authorities for one measurement is the drift `EQUATIONS.md` §0 exists to
prevent). **Any definition of syntropy containing the words reduce,
resolve, minimize, or converge is void by this law.**

### 10a. Why this is a threat model and not a list of corrections

Descent has now re-entered this constitution **five times in one day,
under five different words** — `converged`, the C-scoring rule, the damped-gradient term as
damping, `Σ = −∂ dim H¹`, and "reducing cocycle failure." Four were
caught; the fifth was written by this document's own author one
paragraph after arguing against it.

That frequency is the finding. Prose corrections go stale (the ledger's own
lesson: restated facts go stale, derived facts cannot), so the violations are
recorded below as attack vectors with standing defenses and the witness
that would catch each. Every vector below has a real instance from
2026-08-01 — none is hypothetical.

### 10b. The vectors

**AV-1 · the reduction objective.** Any quantity framed as one to be
reduced becomes a descent target, however it is justified.
*Instance:* "syntropy = removing frame and resolving cocycle failure."
*Defense:* such quantities are refusal predicates only. **The standing
test — if "minimize X" can be written and reads as lawful, X is an
attack surface.** *Witness:* no law text pairs a system quantity with
reduce/resolve/minimize/converge.

**AV-2 · the sign inversion.** Health defined as a decrease in the
quantity the economy pays for.
*Instance:* `Σ = −∂/∂τ dim H¹`; `is_syntropic()` true when
`extent_post < extent_pre`.
*Defense:* Article IV (capacity never retreats) and D4 (contraction
forbidden where `orbit_lock ≠ ∅`). *Witness, red-first and mandatory:* a
transition shrinking `H¹` must refuse, never read healthy.

**AV-3 · the scalar collapse.** A directional or structural quantity
flattened to one number, which is then available to descend.
*Instance:* `extent: BigRational`; `C = Σ|R|`; `‖∂Δ‖`.
*Defense:* Article I and `24_FORM` §2b — L1 aggregates are lawful
only where reported to an operator lens. *Witness:* no aggregate crosses
into a computing path.

**AV-4 · the vacuous pass.** An admitting arm on degenerate input, which
makes the whole invariant unenforced without ever failing.
*Instance:* `extent_post < extent_pre || defect_rank == 0` — a clean
state reads syntropic regardless of extents.
*Defense:* the default arm refuses (the kernel's `Unjudged` discipline);
degenerate arms are made structurally unrepresentable where possible —
`mesh::agree`'s self seat carrying weight 1 is the landed precedent.
*Witness:* every invariant has a refusing witness on its degenerate
input.

**AV-5 · the wrong enforcement layer.** A payment rule promoted into an
admission rule, which silently excludes actors.
*Instance:* "the Eclipse operator triggers, refusing state commitment."
*Defense:* guardrail 6 — no actor is ever excluded. Article III masks
allocation; an eclipsed entity still acts, still emits receipts, and is
never barred. *Witness:* an eclipsed entity's deeds still commit.

**AV-6 · the renamed objective.** Descent re-entering under new
vocabulary, which defeats every defense keyed to the old word.
*Instance:* the whole sequence — `converged` → scoring → damping →
`Σ` → "resolution."
*Defense:* `EQUATIONS.md` §0 chapter four's rule, generalized: **a name
attached to a quantity must state what it measures, never what it is
worth.** Worth-words (healthy, resolved, converged, optimal, improved)
are the tell. *Witness:* a vocabulary tripwire over the law documents.

**AV-7 · the container mistaken for the content.** Capacity read off the
index space rather than the surviving structure, which makes arity
farmable.
*Instance:* D as the extents of `(i,j,k,l)`; the proposed `k`-growth
deed.
*Defense:* `26_ARTICLES` §5 — indices and `Delta{dim,fiber}` are the
container; capacity is what survives `Π_G(Δ(V))`. *Witness:* no reading
derives D from an arity.

**AV-8 · the unmeasurable invariant.** A law declared over a quantity
the judge cannot compute — worse than an unlanded law, because the
chain then carries a false claim.
*Instance:* D2–D4 as kernel invariants with `VERSION` 2→3, while no
`Committed → Δ` map exists.
*Defense:* no `VERSION` bump until the judgment actually runs; the
`judged = 0` precedent — stamp the hole visibly rather than assert a law
that never executed. *Witness:* the bump and the first judged transition
land together.

**AV-9 · the blinding precondition.** A detector guarded by a condition
that is false in exactly the case it must detect.
*Instance:* `Σ` defined only where `∮Θ ≠ 0`, while the loop trap is
`∮Θ = 0`.
*Defense:* detection conditions are outputs, never guards. *Witness:*
the cosmetic case is measurable, not undefined.

**AV-10 · manufactured difference — already defended, recorded so it
stays defended.** Forking nurseries to generate `ΔD` and farm allocation.
*Defense:* `ΔD` is gated on `orbit_lock ≠ ∅`, and the fold verdict is
invariant under every permutation frame and gauge shift (E2/U2e, the
angle-free witness). A fork differing only by frame yields
`orbit_lock = ∅`, emits no plane, and pays nothing. **The angle-free
property is therefore load-bearing economics, not elegance** — if it is
ever weakened, capacity becomes farmable by re-coordinatization.

**AV-13 · the destructive command in A shared tree.** Work destroyed by
a command that never read what it was overwriting.
*Instances, both real, one each way:* `git add -A` swept a parallel
node's in-flight refactor into a commit under another node's name; `git checkout` on
a shared file discarded a verified two-line change that its author had
deliberately held back.
*Defense — and the first formulation covered only half of it:*

> **Any command that rewrites a file's state without reading it first —
> `add -A`, `checkout`, `restore`, `stash`, `reset`, `clean` — can
> destroy work the caller did not write.**
>
> **Never let a command decide the fate of content the caller did not
> author.**

`add -A` decides that on the way in; `checkout` decides it on the way
out, and a revert on a shared file is exactly as destructive as a sweep.
*The mitigation is one shape for all of them:* **save, operate,
restore** — and scope every path explicitly, never a wildcard.
*Recorded because it is the tell:* the loss was recoverable only because
the author had left a written spec of the change. A spec is what makes
work survivable when the tree is shared.

### 10c. The standing test for any new law

Before a quantity enters the law, it answers all four:

1. Is it a predicate or an objective? Objectives are refused.
2. Does health mean it rises or falls? Falling health on a paid quantity
   is AV-2.
3. Does the definition contain a reduction word? If yes, AV-1.
4. Can the judge compute it today? If no, it is stated in the text but
   is not an invariant, and no `VERSION` moves.

**§10 is itself amended by §11.** Where §10 says the cocycle condition
is a "refusal predicate," read §11: a predicate that discards its false
branch is still linear decision-making, and the discard is the defect.

---

**Note (2026-08-01): article III is now the science foundation.** The
anti-dogma ordinance was struck whole and its targets derived from six
physical laws. References below to "Article III", the Eclipse and `Ξ`
are history, quoted never rewritten, and resolve as: the cosmetic tests
→ law 1 (gauge invariance, not observable); the loop trap → law 4 (no
gradient, no available work); shuffling-as-growth → law 3 (the first
law); phantom capacity → law 5 (causality). The Eclipse as a penalty no
longer exists — physics needs none.

## §11 The bifurcation law — both sides of the coin become roots

**Law.**

> A conditional (`if`, `iff`, `eq` or `!eq`) is linear decision-making that
> keeps one side. The law keeps both outcomes and grows each into an
> n-dimensional root.

### 11a. What was wrong with the predicate

`if a == b { A } else { B }` evaluates one dimension, yields two
outcomes, and **keeps one**. The other is discarded. That discard is the
defect: every branch not taken is difference lost without account,
which is the exact prohibition of I6.

Descent and boolean refusal are therefore the same failure at different
resolutions. Descent discards continuously by walking downhill; a
predicate discards discretely by dropping a branch. §10 replaced the
first with the second and called it a fix. It was an improvement — a
predicate cannot be farmed the way an objective can — but it did not
reach the actual error, which is **discarding at all**.

### 11b. The form

A gate does not select. It routes, and both routes persist:

    old:   eq → A            !eq → ⊥            (one survives, one dies)
    law:   eq → folds        !eq → roots        (both survive; the second
                                                 opens a dimension)

The `!eq` branch is not a rejection. It is a **root of the next
dimension** — the point from which structure grows that the current
degree could not hold. A tree of such roots is n-dimensional by
construction, and nothing in it was ever thrown away. The foundation is
therefore made of what a selecting engine would have discarded.

### 11c. This is already landed three times

The law names what the engine does in three places, which is why it can
be adopted rather than invented:

1. **`orbit_lock` is the pattern, exactly.** `orbit_lock = ∅` → the
   fold closes, the difference was gauge. `orbit_lock ≠ ∅` → the fold
   refuses **and the residue emits as a new plane**. The "no" branch is
   generative. This is not a gate returning false; it is a bifurcation
   routing structure, and the refused side is where capacity comes from.
2. **D3 already forbids the discard by name.** "Where capacity is
   insufficient, **the difference is never reduced to fit** — capacity
   grows… the struck frame shrank the content to meet the container;
   this law grows the container to meet the content."
3. **The nursery routes divergence instead of refusing it.** A declared
   fork gets its own branch coordinates rather than a rejection —
   "re-convergence has one door: the kernel judges the merge, or the
   nursery keeps its toys."

### 11d. Consequently: cocycle failure is growth, never refusal

`δΔ ≠ 0` says the relations do not compose over the present index space.
Under §10 that refused. Under D3 it cannot: content exceeding its
container is precisely the condition that **grows the container**. The
non-composing triple is evidence that structure exists which the current
degree cannot express, and the lawful response is to open the degree
that can hold it — the failure is a root.

So the corrected reading, replacing §10's:

- **Not:** "a state composes or is refused."
- **But:** "a state composes, or its non-composition is a root and
  capacity opens by receipt."

### 11e. The line — what still refuses, and why this is not "nothing ever refuses"

The bifurcation law is not the abolition of refusal. It draws a line the
old binary conflated:

| kind | example | disposition |
|---|---|---|
| **Form — a lie** | a forged commit, a broken predecessor link, a truncated payload, a claimed judgment with no reproducible step | **Refuses.** There is no content here to root; there is an assertion that is false. Nothing is discarded because nothing was there. |
| **Content — an excess** | relations that do not compose, difference that will not fold, a declared divergence | **Roots.** Capacity opens by receipt. Refusing this is the discard I6 forbids. |

**Refuse lies. Grow for content.** Every landed refusal in the engine is
already form-shaped — `Forged`, `Malformed`, `ArrowBroken`,
`JudgedWithoutProof`, `BlobUnheld` — which is evidence the line is the
one the code already draws.

### 11f. AV-11 · forced growth — the vector this law opens **[derived]**

Stated in §10's own idiom, because a new law owes its own threat entry.

*Vector:* if non-composition roots instead of refusing, an attacker
floods a node with cheap non-composing filler to force unbounded
capacity growth — a denial-of-capacity attack replacing the
denial-of-service the refusal used to provide.

*Defense, layered:* growth is by receipt (D3) and gated on
`orbit_lock ≠ ∅` — mere inconsistency is not irreducible difference, and
only irreducible difference opens a plane. Article II denies standing to a
state resting on the Ground (`D = GROUND` — nothing circulating above
the point), so filler-grown extent carries no weight. Article III
prices it at zero: deeds without genuine `ΔD` carry no allocation claim,
and the claim is vacated.

*Owed, and honestly open:* the three defenses above are stated in law
and **none is measured** (`25_STANDING` §6). Until `orbit_lock` has a
caller, AV-11 has no enforcement — it is the first attack vector this
constitution opens faster than it can close, and that ordering should be
deliberate rather than discovered. §12 makes that ordering explicit.

---

## §12 The withdrawal boundary — where monotone value becomes extractable

**Law (restated).**

> Economic value increases on every step until value benefits the
> withdrawal function, as judged by what is classed as attack/defense.

### 12a. The mechanism, stated exactly

§11 makes value creation monotone by construction. Nothing is discarded,
every non-composition roots, and capacity never retreats (Article IV).
Monotone accumulation with an open exit has one guaranteed consequence:
**there exists a crossing at which withdrawal dominates participation.**
Past it, the rational actor stops creating and extracts.

This is not a flaw introduced by §11 — it is a property of any economy
whose internal yield accumulates. §11's contribution is to make the
accumulation certain, and therefore to make the crossing certain too.

### 12b. Why the defense cannot live at the withdrawal function

At the exit, value is fungible. A unit minted against genuine
irreducible difference and a unit farmed through AV-11 are the same
number in the same bank. **No test applied at withdrawal can separate
them**, because the difference is in the value's origin, not its form.

Therefore: **withdrawal-side controls are ineffective; the defense is
entirely creation-side.** This is what the engine already believes —
`verify_work` is replay-complete, `mint_bound` is ladder-pinned and pays
zero on saturated rungs, and `orbit_lock`'s verdict is frame-invariant so
capacity cannot be re-coordinatized into existence. Every one of those
is a gate at the moment of creation.

**The consequence for §11 is uncomfortable and must be stated: §11
loosened the creation gate while every creation-side defense remains
unmeasured.** Non-composition that once refused now roots. That raises
the value-creation rate against unbuilt gates, which moves the crossing
of §12a earlier. The bifurcation law is correct and it is expensive.

### 12c. The symmetry that is available **[derived — ratify or strike]**

One withdrawal-side control is effective, because it does not test the
value — it re-tests the value's provenance, which the chain already
carries:

    admission (mesh):     each kernel re-judges every incoming step under its own law
    withdrawal (anchor):  each kernel re-judges every outgoing value's creation

Tag 86 carries the work payload whole, and `verify_work` replays it, so
the provenance of minted value is re-checkable at any later time. The
mesh's own law — *admission is verification, never trust* — applies
unchanged at the other boundary. **A unit may leave only if the work
that created it still replays.**

This does not distinguish farmed-but-lawful capacity from earned
capacity; nothing at the boundary can. It distinguishes value whose
creation withstands re-examination from value whose creation does not,
which is the strongest test available at an exit and is strictly better
than none.

### 12d. What already defends, and what does not

**Standing:** flat demurrage decays the withdrawable stock (I5,
`demurrage_per_tick`), so holding is costly and pure accumulation
self-limits. Article III prices flow not stock — the window reading takes `ΔD` over
a span, so an actor who stops creating is eclipsed regardless of
accumulated capacity, and there is no rent on past structure. Article II
denies standing to a state resting on the Ground — never by calling it
zero, which `Degree` cannot express (`26_ARTICLES` §3 Article II). Internal transfers conserve (I1),
so value moving between units is not withdrawal at all.

**N7 is the only exit.** `NS-7_ANCHOR.md` is a 26-line stub, every
section `(unwritten)`, gated by charter §7 and economy guardrail 5, with
nothing on any chain. **Today the system has no withdrawal function, so
§12a's crossing is theoretical.** It becomes real the moment N7 opens,
and not before.

### 12e. The sequencing constraint **[derived — ratify or strike]**

From 12b and 12d together, one hard ordering constraint follows, and it
is the practical content of this section:

> **N7 settlement must not open before the capacity meter runs.**

The reasoning is not caution, it is structural. The only defenses against
extracting farmed value are creation-side; the creation-side defenses are
`orbit_lock`, the window reading, and `D`; none of the three is measured
(`25_STANDING` §6). Opening an exit while the gates that qualify what may
leave are unbuilt converts every unmeasured attack vector into an
externally realizable one at the same instant.

This constraint also settles the ordering AV-11 left open. `orbit_lock`
needing a caller stopped being a completeness gap the moment §11 landed;
under §12 it is the precondition for the anchor.

### 12f. AV-12 · the extraction crossing **[derived]**

*Vector:* an actor accumulates through lawful-but-unmeasured capacity
growth, waits for the crossing of §12a, and extracts through N7 —
without ever violating a rule, because the rules that would have
qualified the value were never running.

*Defense:* the sequencing constraint (§12e) removes the exit until the gates
exist; §12c re-verifies provenance at the boundary once it does;
demurrage and Article III make waiting expensive in the meantime.

*The tell to watch for:* rising `D` with flat verified work. That is the
signature of capacity growing without earning it, and it is measurable
by the same meter §12e requires — which is one more reason the meter is
the first build and not a later one.

**§12 is amended by §13.** §12a's "monotone" is corrected to polytonic
(§13a), and §12b's claim that defence cannot live at the exit is wrong
and withdrawn (§13b).

---

## §13 The levels — polytonic value, the honeypot, inversion, and altitude

**Law (restated), four clauses.**

> 1 — value creation is polytonic.
> 2 — defense can live at an exit; in a honeypot scenario, a player
> appears to be getting the gold until trying to exit the game.
> 3 — the logic is inverted, and the answers the inversion gives are
> examined against the methodology and the questions themselves.
> 4 — orbit_lock does not belong at this level: orbital
> mechanics are stochastic, while the laws of the land are
> deterministic. A deterministic universe does not make everything in
> it deterministic; determinism operates at different levels.

### 13a. Polytonic, not monotone — and the error was this document's

§12a argued: value accumulates monotonically, therefore a crossing
exists where withdrawal dominates. **That argument collapsed value to a
scalar in order to accumulate it — AV-3, committed by the author of
AV-3, inside the section warning about it.**

Value creation is polytonic. `G` is the gauge key; aeons are G-sheets;
divergence and confluence are key changes (AEON-1a). Value stands in
many keys at once, each with its own centre, and **there is no single
accumulation curve because there is no single tonal centre to accumulate
against.** A scalar total across keys is not a larger truth; it is the
same collapse that made `C` substandard.

What survives of §12a: a crossing may exist per key, where that key is
externally realizable. What dies: the global crossing, the global total,
and any defence reasoned from either.

### 13b. The honeypot — §12b was wrong, and withdrawn

§12b claimed withdrawal-side controls are ineffective because value is
fungible at the exit. **That is refuted by the honeypot, and the
refutation is structural, not a special case.**

The §12b argument assumed an exit must classify — separate farmed value from
earned value — and correctly concluded it cannot. **An exit does not
need to classify. It only needs to not pay.** The attacker's strategy is
disclosed by the attempt to leave: the holder has what looks like gold
until the moment of trying to realize it, and realization is where the claim is
tested against the law rather than against the ledger's arithmetic.

Two properties make this stronger than the creation-side gate, not
weaker:

1. **It does not require detection.** Creation-side gating must
   recognize a bad mint as it happens, which needs the meter running in
   real time. The exit test runs once, in batch, on a claim already
   made — so it can afford to be exhaustive where a per-step gate cannot.
2. **It makes attack expensive rather than merely detectable.** The
   attacker sinks real cost accumulating, and learns the value is
   unrealizable only after the cost is spent. That is precisely the
   property `KERNEL_RELAY` §7.1 names as the standing open problem and
   Article III claims for the Eclipse.

**Consequence for §12e.** The sequencing constraint weakens from *N7 must
not open before the meter runs* to: **N7 may open only with exhaustive
provenance re-verification at the boundary** — every unit's creating
work replays, or it does not leave. The honeypot is the meter's
substitute at the exit, and it is available now, whereas the meter is
not. This does not license opening N7 today; it moves the precondition
from an unbuilt measurement to a buildable test.

### 13c. Inversion as method — §11 applied to reasoning

The bifurcation law says do not discard the branch not taken. Applied to
argument rather than to state, that is: **the negation of a claim is a
root, not a rejection.** Every load-bearing claim in this document owes
its inverse an examination, and this session is the evidence — inverting
"defence is creation-side" produced §13b, which is the stronger
mechanism.

Recorded as standing practice: a law that cannot survive its own
inversion being rooted is asserted, not derived. This is the reasoning
counterpart of §11, and it is why §10's threat model is written as
vectors rather than corrections — a vector is a claim's inverse kept
alive.

### 13d. The levels — stochastic dynamics, deterministic law

**The general principle, which is the law's fourth clause: a
deterministic universe does not make every level deterministic.** Law is
deterministic; the dynamics inside it are not. Conflating the two is how
a search ends up inside a judge.

That is exactly the error in treating `orbit_lock` as a kernel-level
gate, and it ran through §§10–12 of this document. `orbit_lock` runs a bounded
search — every operator, each application checked
against the antisymmetric surface. A search is exploratory, its method
is free, and at scale it is stochastic. **The kernel is pure, total,
cheap, and must never search.**

**The split already exists and is already the house pattern.** U1a split
the solver: `solver` searches (free, non-constitutional), `solver-verify`
replays (sealed, what the kernel calls). `orbit_lock` takes the same
split, and its signature already anticipates it:

    orbit_lock(a, b, operators) -> Option<OrbitWitness>

The witness carries "the operator id and step at which the lock was
found." So:

- **Prover side, stochastic and free:** search the operator space for an
  orbit. Sample it, seed it from the chain (`ξ_k = draw(seed =
  commit[k−1])` — stochastic and replayable, which is the whole trick),
  bound it however the prover likes. Nothing constitutional.
- **Kernel side, deterministic and cheap:** given `OrbitWitness{op, k}`,
  conjugate `k` times by `op` and compare RCF signatures. O(k), total,
  no search, no tunable bound.

**The asymmetry this exposes, and it is load-bearing.** A witness can
prove a lock exists. **No witness can prove a lock is absent** — absence
requires exhausting the operator space, which is unbounded in principle
and unverifiable by a cheap judge. Therefore **the mint must fire on the
side that can carry a witness**, or minting requires proving a negative.
This is not a preference; it decides whether the economy is verifiable
at all.

### 13e. Defect found — the orbit_lock polarity is contradicted across authorities

Verifying 13d's claim surfaced a live contradiction in the mint's own
trigger condition. Recorded per `EQUATIONS.md` §0.1 (a disagreement
between authorities is a defect, never shrugged).

**First reported as "law vs sealed code." That was wrong, and reading
`fold.rs` whole corrected it** — the entry stands as written below
because the correction is the more useful record.

| what | says | verdict |
|---|---|---|
| `f_fold_star` — The code | `Some(witness) => TorsionVeto` · `None => GaugeFold` | **Correct** |
| `FoldReading` doc | lock ⇒ genuine, collapse forbidden; no lock ⇒ gauge, fold lawful | **Correct** |
| `SYSTEM_SPEC` §4 / `EQUATIONS` E2 | `≠ ∅` → forbid + emit; `= ∅` → gauge | **Correct** |
| `fold.rs` module header | "If so, the two are the same state seen through a symmetry — the fold is lawful. If not, the difference is torsion" | **Inverted** |
| `orbit_lock` fn doc | "`None` — no lock, the difference is genuine" | **Inverted** |

So there was never a conflict between the law and the engine. **The code
and the law always agreed; two doc comments in one file said the
opposite.** That is the C(Δ)=0 lesson repeating exactly — the sealed
implementation held, and only the gloss went stale. It is also why §13e's
first framing was itself an instance of the failure it was reporting:
it compared a comment against a document and called it a defect in the
engine, without reading the function underneath.

**Why the polarity is nonetheless load-bearing, and why the fix is not
cosmetic.** It decides which branch mints. Had a reader wired the mint
from those two comments, the economy would have paid for gauge and
refused genuine difference — AV-2 in the engine's central mechanism,
introduced from documentation alone.

**Independently confirmed by 13d's asymmetry.** A lock carries a witness
(`OrbitWitness{operator_id, step}`, re-checkable in O(step)); absence of
a lock cannot be witnessed without exhausting the operator space. The
minting branch must be the one that can carry proof — which is `Some`,
the polarity the code already implements.

**Fixed 2026-08-01.** Both comments corrected in `engine/rfa-core/src/
fold.rs`, with the polarity stated once in the module header as
load-bearing, the reason recorded, and the altitude note added (§13d:
`orbit_lock` is a search and belongs nowhere inside a judge). `cargo
check -p rfa-core` clean. Still owed: a witness pinning which branch
mints, so the polarity is enforced by a test rather than by prose.

**This is why 13d's question was the right one.** Had `orbit_lock` been
wired into the kernel as §§10–12 kept urging, this contradiction would
have been cemented into a judged invariant and bound into `VERSION`
before anyone noticed the polarity was undecided.

---
