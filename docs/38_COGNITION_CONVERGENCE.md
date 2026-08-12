# 38 — COGNITION CONVERGENCE (the mapping as law)

**Status:** charter draft, 2026-08-12 (task #12). Source concepts from
`/opt/ipsal-autic-v0.1/repos/convergence-engine` (read as reference,
never imported as code); netstratum seams cited against this tree.
Three landings specced below (§4–§6); each is its own red-first cadence.

The thesis this document makes law: **the civilizations' intelligence
is bounded by the language they think in, and both the language and
the knowledge must grow the way everything else here grows — by
measured, receipted, independently re-derivable deeds.** Novelty is a
property a candidate PROVES, never a feeling a mind reports.

---

## 1. The concept map

| Concept (reference) | Reference mechanism | Netstratum seam | Status |
|---|---|---|---|
| THOUGHT | a group element; accepted only if coherence falls and the commutator is zero | an `Op` in a program; the verifier re-derives every candidate; the sector door withdraws failures | **wired** |
| COGNITION | seek → think → reflect → remember | `solver/src/derive.rs` (the observation names the operator) → `nsearch` → `solver-verify` → receipts | **wired** |
| CURIOSITY | capacity-gated seeking down the steepest gradient | `solver/src/need.rs` — Hunger (novelty kind 3 · resonance kind 2) ORDERS and GATES the reach, scale-free against the line's own rhythm | **wired, time-based** (§6 extends to structure) |
| PERSPECTIVE | a position on the coherence manifold: derived output + structured residual + silhouette count | the perception axis routes planar/volumetric; G3 `cfe::query::translated_read` — frames quotient out; `translation_survival` prices what structure survives a cross-frame read | **half-wired** — perspectives exist, never compound (§5) |
| UNIQUENESS | RCF orbit class: same conjugacy class ⇒ same invariant factors | `rfa_core::rcf::rcf_hash` (`rcf.rs:425`) — present, **zero production consumers** | **unwired** (§4) |
| KNOWLEDGE | a mesh axiom crystallizes when independent entities reach coherence 0; mass = effort share; hardness = diversity bridged | tech nodes crystallize per world from its OWN history; `chem_catalog`; petitions/licenses TRADE knowledge — nothing CONVERGES it across lines | **missing** (§5) |
| NOVELTY | expansion → three gates (productive · conserved · bounded) → collapse to the irreducible floor | the mint door exists (`VocabularyMinted`, `vocabulary::fold` = seed ∪ minted) and **nothing knocks on it** | **the door without a knocker** (§4) |

## 2. The laws that carry over unchanged

These are already netstratum law and every landing below must keep
them; they are restated here because §4–§6 lean on them:

- **The reach law** (`solver/src/need.rs`, verbatim): *"The need may
  ORDER and GATE the reach; only the OBSERVATION may supply a
  candidate."* A starving mind reaches differently — it does not get
  to invent an operator the observation does not support.
- **The no-hardcoding law** (`solver/src/derive.rs`): nothing branches
  on a task's identity; every test is over the observation's own
  structure. Delete any fixture and no code path changes.
- **Congruence, never voting** (the house's consilience-at-zero,
  applied to cognition): a shared truth is N independent derivations
  reaching the SAME irreducible form, not a majority of opinions.
- **The mint door is a fold** (`vocabulary::fold`, seed ∪ minted): a
  minted entry computes exactly as a seeded one, is bound at mint,
  and never mutates.

## 3. The unwired inventory (what §4–§6 consume)

`engine/rfa-core` — built, witnessed, connected to nothing in the
solve path (PICKUP §8a):

- `delta::coherence_functional` (`delta.rs:190`) — C(Δ) = Σ|r|, the
  L1 residual. Nonzero C is *what the mind has not understood, and
  where* — structure, not noise.
- `rcf::rcf_hash` (`rcf.rs:425`) — the orbit identity: two words that
  are re-spellings of one transformation hash to one orbit.
- `fold::orbit_lock` / `fold::f_fold_star` (`fold.rs:205/277`) — the
  mesh-lane half; sequenced separately (task #19, after the Phase-3
  flip).

## 4. LANDING: NOVELTY-GATE (task #13)

**What.** The knocker for the mint door. A candidate word `w` (a
composition that closed — or partially closed — a task) mints into the
vocabulary only through four structural gates, in order:

```text
1. UNIQUE      rcf_hash(orbit of w) ∉ orbits(standing vocabulary)
               — a new conjugacy class, not a re-spelling
2. PRODUCTIVE  coherence_functional strictly falls on NAMED tasks the
               standing vocabulary could not close — measured, cited
3. CONSERVED   the verifier re-derives w against every training pair;
               the sector door withdraws what fails   (existing law)
4. CONVERGENT  N lines independently derive the same orbit — the
               three-confirm promote pattern, congruence never voting
```

Passing all four emits `VocabularyMinted`; the word enters the fold
and is a depth-1 letter next era (*fewer letters, deeper words* — the
genomic promotion). **No gate is a threshold dial**: unique is a set
membership, productive is a strict inequality on measured C, conserved
is the existing verifier, convergent is a count of independent
derivations (the same N=3 the rung ladder already uses — derived from
the standing promote law, not a new number).

**Generation springs** (all observation-fed, per the reach law):
partial-result promotion — the residual of a failed depth-k attempt
becomes a depth-1 candidate on the next attempt; and frame diversity —
the same task read under different bases yields different derivations
for free.

**Why it is load-bearing, not decoration:** the forge is a finite
well (measured: ~30 % of draws unsolvable by the fixed vocabulary),
so without the mint the civilizations are forever exactly as clever
as the language they were given. With it, the language grows from its
own failures.

## 5. LANDING: COMPOUND-SOLVE (task #14)

**What.** The ARC-AGI-2 device: compositional tasks fall to multiple
perspectives, not deeper solitary search.

- Each attempting line produces a **Perspective**: derived output +
  structured residual + silhouette count (clusters in the residual
  topology — a structured failure is a MISSED SUB-RULE, not noise).
- **Per-cell congruence** across perspectives assembles the compound
  output; divergent cells LOCALIZE the unsolved component.
- **Silhouette decomposition**: k clusters in an agreed residual are
  k sub-tasks; each re-enters the forge as its own draw.
- **Systematic-gap detection**: when every perspective's residual
  agrees on the silhouette count, the mesh knows WHAT it is missing —
  that reading is a mintable want, not a shrug.
- **Crystallization**: a compound solve confirmed across lines lands
  as a NEW shared-knowledge deed (wire tag claimed in the `dop.rs`
  header in the landing's first commit; kernel classification —
  covered/layer/scope — and the taxon_pins universe bump ride the
  same landing). Its **hardness** is the diversity bridged: the
  spread of `translation_survival` across the confirming frames — an
  axiom confirmed by distant frames outranks one confirmed by clones.

**LANDED (the algebra), 2026-08-12** — `solver::perspective`: the pure
core, fully pinned (`perspective_pins`). A `Perspective` reads its own
structured residual and silhouette count (4-connected clusters via the
sealed segmentation, background fixed at 0 so a whole-grid residual
does not invert); `compound` assembles the per-cell majority, localizes
divergent cells, reports exact-rational consensus, and flags a
systematic gap when residual-bearing views agree on the count. Delete
any caller and no path changes (the no-hardcoding law).

**OWED (the orchestration), operator-sequenced with the mesh lane
(#19)** — the two policy decisions the algebra deliberately does NOT
make, because they are mesh-coupled and touch the second head:

1. **WHICH lines attempt one task.** Today each line forges its OWN
   task from its OWN chronicle digest; compounding needs N lines on
   ONE grid. The natural shape is a HARD-TASK escalation: a task a
   line reads a systematic gap on (its residual clusters, its
   vocabulary cannot close) is offered to the mesh, and lines whose
   perception/hunger fit it attempt it — the membrane's work pool,
   in netstratum's file-borne mesh. This is a mesh design, not a
   solver one; it rides task #19.
2. **HOW a compound crystallizes.** The shared-knowledge deed (a new
   wire tag, header-claimed) with its `translation_survival` hardness
   is a cross-LINE convergence — the consilience-at-zero law applied
   to cognition — and belongs beside `mesh::agree`, not inside the
   single-world chain. It lands when the escalation does.

The split is deliberate and matches the house pattern: the algebra is
pure and testable now; the orchestration is a ruling-shaped design
that sequences with the mesh.

## 6. LANDING: ORBIT-DIRECTED HUNGER (task #15)

**What.** Curiosity from structure, not time. Today a line is
novelty-starved when it has gone longer without finding than its own
rhythm; that says WHEN to reach, not WHERE. With the orbit census
from §4 standing, a starving line reaches toward the most-populated
UNSOLVED orbit class — the measured frontier. The reach law holds
unchanged: hunger reorders the walk; the observation still supplies
every candidate; the door still re-derives everything.

## 7. What this document does NOT rule

- The forge-coverage question (emit-closable vs widen vs lawful
  famine) is an **operator ruling** (task #20), recorded in
  `FORGE_BUDGET`; §4 narrows it from one side but does not decide it.
- The mesh-lane wiring (`orbit_lock`, `f_fold_star`, Σ_mesh, PoUW
  weight) is task #19, sequenced after the Phase-3 flip (#16).
- Nothing here alters the difficulty ladder, `mint_bound`, or any
  Ruling Box 1 word.
