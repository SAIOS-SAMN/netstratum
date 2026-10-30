# The kernel Charter — law for the one validated update function

**Status:** law (ratified 2026-07-31). Governs `engine/kernel`. Subordinate
to `docs/00_CHARTER.md` §11 (the One-Truth Amendment); this document is its
Rule 3 made concrete — the invariants, the equations, the algorithms, and
the order they land.

**Amendment tiers (or the cadence dies).** Sealing the vocabularies must not
freeze content development behind constitutional process. Two tiers, both
changing the constitution digest so the handshake still catches every
difference:
- **Additive amendment (minor `VERSION` bump):** a new event tag with its
  invariant coverage, or a new closed-vocabulary entry. Content-development
  cadence rides this tier. Two operators on different additive tiers are
  simply different universes until they match — lawful nurseries to each
  other; cross-version convergence is a bridge problem, never a silent skip.
- **Breaking amendment (major `VERSION` bump):** a changed meaning of an
  existing rule or field. Rare, charter-tier deliberation.

The protocols named as sealed law (unambiguously): **RFA** (exact-rational
algebra → I3), **DOP** (NS-1, the object model → schema/vocabulary), **VRP**
(NS-3, the receipt chain — verification/parent-linking → the commitment
shape; confirmed VRP, not RQP), and **CFE** (the runtime whose transition
rules become the invariants). Law seals; mechanism stays in its crate,
version-pinned.

**Provenance (honest):** the convergence mechanics below are re-derived from
the v0.1 ancestor `/opt/ipsal-autic-v0.1/repos/convergence-engine`
(`fold.rs`, `mdc.rs`, `consensus.rs`, `DIMENSIONAL_RESOLUTION.md`,
`DIMENSIONAL_BLOCKCHAIN.md`) — which is AI-generated, sandbox, unwitnessed.
We take the design and re-witness it red-first in this disciplined tree (the
NS-3 "spec from the working ancestor" precedent). No sandbox code is
imported. What the ancestor solves — convergence better than gradient
descent — it solves; what it does not solve — the ordering/Sybil anchor —
remains open (§7).

---

## 0b. The sealed math core (the equations are immutable — more than the
alphabets)

The governing equations are not the kernel's to own alone, nor the mesh's —
they are one immutable definition both bind and call. The residual `R`, the
coherence functional `C(Δ)`, `coboundary_reduce`, `orbit_lock`,
`verify_certificate`, and the evolution operators live in the sealed math
core (the algebra layer — `rfa-core` is the seed; the certificate verifier
moves in from `solver` at its wiring). Three laws bind it:

1. **One definition, many callers — no invariant codes its own math.** The
   kernel's I6 calls `orbit_lock`; the mesh's consilience calls `C(Δ)`; I4
   calls `verify_certificate`. A primitive reimplemented in two places can
   diverge, and two nodes computing the same input to different results is
   the death of convergent-consensus. So the equations are called, never
   copied.
2. **Version-pinned into the constitution digest.** The math core's version
   is part of `constitution_digest()` exactly as `kernel::VERSION` is. A
   changed equation changes the digest — a different universe, caught by
   the handshake, never a silent divergence.
3. **Math amendments are breaking-only (major bump), always — the equations
   are more immutable than the vocabularies.** A vocabulary change can be
   additive (a new tag; old computations unchanged; minor bump; lawful
   fork). A math change alters the result of a computation on the same
   inputs — it is inherently breaking, because two nodes on the same
   alphabet but different equations would compute different `C(Δ)` on
   identical receipts and diverge silently. That is strictly worse than a
   changed alphabet (which the digest catches as a different universe). So
   the math can change only by breaking amendment, never additively.

The kernel is one component of the sealed constitution (kernel invariants +
the math core + the vocabularies + the derivation rules); all of it is
version-pinned into one digest, and a node runs all of it unmodifiable.

## 1. What the kernel is

The kernel is the sole validated update function:

    lawful(prior, next, step) -> Ok | Unlawful

- **prior, next**: full committed state, in memory.
- **step**: `{ event, actor, stamp }` — the deed claiming to carry prior to next.
- **pure, total, no I/O, encoding-blind, network-blind.** It never
  serializes (the codec holds the byte monopoly, so kernel and chain cannot
  disagree about bytes), and it never reads the mesh, the uplink, or any
  peer state. It does bind the sealed math core (§0b) and the state types —
  that is shared immutable law, not peer state. The audit is grep-simple:
  **`engine/kernel` imports the sealed set (state types + the algebra
  core), never the mesh.** All arrows point one way — the kernel judges
  meaning (calling the sealed math), the codec fixes form, the mesh does
  distribution; none reaches back into the judge. `lawful()` calls the
  sealed equations; it calls nothing of the mesh's.

The kernel judges one step. It never optimizes a global quantity — a global
property (coherence, stability) is not a per-step fact and cannot enter
`lawful()` without destroying its nature. The design law: **enforce the
local invariants whose universal application yields the global band;
measure the global with a lens; never enforce it in the judge.**

**The sealed default arm — refuse-unknown, never admit-unknown.** The
default arm of the judge refuses: a deed no invariant covers does not enter
(`Unlawful::Unjudged`). Coverage grows by amendment — each landing moves an
event from the default-refuse arm to an explicit judged arm — and the kernel
wires as the live gate (§11 Phase 3) only when `Unjudged` is unreachable,
every event covered. The distinction that guards this forever: **SEAL A is a
reader's law, never the judge's.** Readers skip unknown tags so old software
survives new content (SEAL A, right and permanent). A judge that skips
unknown deeds is a hole exactly the size of every future tag. Two different
laws, one word — "unknown" — and conflating them silently rebuilds
default-admit. (Landed: the flip + witness `the_sealed_default_refuses_the_unjudged`.)

**No consensus weight before cryptographic identity.** Until identity binds
(N7), any confirmation-weight computation is advisory only — never
canonicalizing. Weight summed over self-declared integer hands is the puppet
web, pre-built by us; this one line forbids a well-meaning seat from
implementing it early.

## 2. The invariant set (the whole of the law)

`lawful()` is the conjunction of six invariants. A step is admitted only if
all hold. Each lands red-first, one witness minimum, coverage growing by
amendment (never by silent omission — §11 clause 2).

**I1 — Conservation (closed moves).** A deed that only moves value may not
change the world total of that value.

    for a value-moving event on resource r:
        Σ_{orb} bank(prior, orb, r) == Σ_{orb} bank(next, orb, r)

Refuses `ConservationBroken { r, before, after }` with both totals whole.
(Landed for `ResourceTransferred`; extends per event by amendment.)

**I2 — Containment.** The actor holds the scope the event requires, read
from `prior` (identity precedes the world, NS-6 §1.2).

    permits(entity(prior, step.actor), scope_of(step.event))  must hold

**I3 — Exactness.** Every value in `next` is a canonical exact rational
(coprime, positive denominator). No float, no wall-clock, no ambient
entropy ever entered (Charter §9). Structural: non-canonical cannot be
constructed, only refused.

**I4 — Bounded creation (syntropy).** Value is created only by a lawful
mint, and only in the amount its rule permits. No fiat (05 §1.2, 01 §6.1:
joules only by encasement). At convergence scale the mint is the discovery
mint (G4), amount tied to verified difference (§7).

    value(next) - value(prior)  ==  mint(step),   mint >= 0, gated

**I5 — Bounded decay (entropy).** Value is destroyed only by the flat,
denominator-bounded rule. Never proportional (the 2^1100 defect: proportional
decay collapses canonical denominators without bound).

    decay(step) = min(rate * ease * elapsed, held),   rate flat, bounded

**I6 — Conservation of difference (the fold gate).** Every irreducible
residual is reduced as gauge, minted as a new dimension, or refused — never
silently discarded. Concretely, a fold (declaring two states the same and
collapsing them) is lawful only if no orbit-lock binds them (§4).

**The container law (resolves I6 × sealed-schema; codec-side precision is
the mesh operator's).** I6's emission "mints a new fiber plane," but §11 says
state-schema changes are amendments — runtime dimension-growth and a sealed
schema collide unless the schema seals containers, not fixed dimensions.
Inscribe what the tree already does: the gauge key is a variable-length
container inside schema v1, grown only by receipt (`GaugeShiftedAt` opens
planes); banked elements are likewise a container. So: **the schema seals the
containers; the fibers grow inside them lawfully at runtime; new container
kinds are amendments.** Without this line I6 either shatters the codec gate
or becomes a schema mutation — the codec is what would break, so its
byte-level precision belongs to the mesh/codec lane.

Together I1/I4/I5 are the full value law: **value moves conserved, is
created only by bounded lawful mint, destroyed only by bounded lawful
decay.** I6 is the convergence law: **no difference dies without account.**

## 3. The measured quantities (lenses, never enforced)

Read to know the state; never targets the kernel drives.

- **Coherence functional** (the cocycle-CONSISTENCY measure; zero iff the
  differences compose — never "iff converged", see the correction below):

        C(Δ) = Σ_i |R_i|        (exact rationals; the proportional law —
                                 operator ruling 2026-07-31, amending the
                                 earlier squaring clause: squares are
                                 barred as aggregators of difference,
                                 on residuals as on torus distance — L1
                                 everywhere, the short way around, each
                                 difference counted exactly its size in
                                 its own units; the zero point the law
                                 judges at is unchanged)

  **`C(Δ) = 0` is not "converged"** (operator correction 2026-08-01).
  Zero says the relations compose — `Δ_ik = Δ_ij + Δ_jk` for every
  triple. That is what the sealed `rfa_core::coherence_functional`
  states, and all it states. It does not say the two states are
  identical, and it does not say difference was spent. The genuine
  difference is the H¹ class, and `coboundary_reduce` preserves it by
  construction: it strips the coboundary — the coordinate-frame
  artifact that was never difference — and what survives is the cocycle
  class (`rfa_core::reduce`, the sealed authority). C cannot see that
  class. A state may read `C(Δ) = 0` and carry a large irreducible
  core. Calling zero "converged" contradicts **I6 in §2 of this same
  document** (*no difference dies without account*) and invites the
  minimization §4 rejects. Reduction removes contradiction and frame.
  **We never discard residuals.**

- **Constitution digest** (the link handshake — same universe?):

        D_const = H( version ‖ vocabularies ‖ derivation_rules ‖ schema )

  Two nodes link only if `D_const` matches. The canonical genesis roster is
  a named deed-set the kernel hashes but does not contain — start canonical
  (same digest) or fork your own (different digest, still lawful).

- **The commitment** (the spine binds it; the kernel never reads it):

        commit[n] = H( schema ‖ version ‖ state_bytes[n]
                       ‖ commit[n-1] ‖ coordinate[n] )
        coordinate[n] = TUniversal{ S, G, T }[n]

## 4. Convergence — dimensional resolution, not gradient descent

Gradient descent minimizes to a point and discards the residual — uni-
directional, lossy. It is struck entirely (operator ruling 2026-08-01 — a
failed topology; the earlier "kept only as a direction hint" is withdrawn,
and the uni-directional linear timeline with it). The foundation is
dimensional capacity: `docs/EQUATIONS.md` §1, D0–D4 — capacity grows to
admit what the universe expresses, and the difference is never reduced to
fit. Convergence is **emission**, bounded on both
ends (the ancestor's `DIMENSIONAL_RESOLUTION`):

    loop:
        unfold:  if the residual needs dimensions the state cannot express,
                 propose an operator that raises dimensionality
                 (trigger §5); bounded above by validation gates.
        fold:    project to the irreducible H^1 core (coboundary_reduce);
                 bounded below by the cohomological floor — cannot collapse
                 past H^1.
        measure: C(Δ). The cycle rests when C(Δ) == 0 and no orbit-lock
                 remains — the relations compose, and no torsion binds
                 the pair. Resting is not emptiness: the H^1 core stands
                 (the floor this loop cannot fold past). What the cycle
                 removed was contradiction and frame, never difference.

**The fold gate (I6, from the ancestor's `F_fold*`).** Before folding Δa and
Δb, test for a torsion cycle:

    orbit_lock(Δa, Δb) := ∃ op ∈ Operators, k ≤ K_max :
                          conservation-valid orbit  op^k(Δa) ≅ Δb

    if orbit_lock(Δa, Δb) ≠ ∅:
        Forbid the fold — Δa, Δb are genuinely distinct perspectives;
        the orbit is the symmetry the residue forms; collapsing it
        destroys information.  ⇒ the residue is emitted as a new
        dimension (the mint, I4), not discarded.
    else:
        fold permitted — the difference was gauge, lawfully reduced.

This is the residue's "life after": genuine difference (a torsion orbit)
becomes a new fiber plane (the fold's up-arrow); gauge difference reduces
(the down-arrow). The convergence creates the shared class and preserves the
genuine difference as new symmetry — syntropy and entropy in one step,
conserving the total (I6).

## 5. When to unfold — the transmutation trigger (MDC)

The state grows a dimension only on a measured wall, never freely (the
ancestor's `mdc_resolve`). Over the decision echelons e ∈ E each with weight
w_e and confidence c_e:

    entropy    = mdc_entropy( max_e(w_e·c_e), Σ_e(w_e·c_e) )
    stasis     = entropy ≥ stasis_threshold
    transmute  = stasis and ∀ e: c_e < low_threshold   (the dimensional wall)

`transmute` true ⇒ unfold (raise dimensionality). Otherwise hold in
**stasis** — disagreement is paused and preserved for resolution, never
discarded to a majority. This is convergence-not-consensus: the mesh records
where in relational space nodes disagree and by what exact rational, not
merely that a vote passed.

## 5a. The two axes — where each half lives (PoW++ accounting)

PoW++ is two axes, and each splits into a local half (kernel) and a
cross-node half (mesh). The split is forced by network-blindness: the
kernel judges one node, one step; anything spanning two nodes is the mesh's.

- **Proof of Convergence = I6 (kernel) + consilience (mesh).** Local: the
  fold gate, `orbit_lock(a,b) = ∅` — a single-state exact check, I6, lands
  with the spine. Cross-node: `C(Δ) = 0` consilience between two chronicles
  — outside the kernel (a lens the mesh reads; §3), never enforced in the
  judge.
- **Proof of Useful Work = I4's mint-bound (kernel) + work-verification
  (mesh).** Local: the anchor on I4 — a deed claiming work carries a
  solver certificate, and the mint may not exceed what `verify_certificate`
  proves (you cannot mint faster than you produce certified intelligence;
  anti-inflation and anti-Sybil in one bound). Cross-node: the mesh receive
  door replays embedded certificates before adopting a spine.
- **Seam conservation = I1 (kernel) + replay (mesh).** Local: aperture
  legs sum zero within one book — one unconditional line (why the
  void-share ruling mattered). Cross-node: **`cfe::opposition` is
  excised** (c7cad9e), and the anti-mirror framing that named it is
  deprecated (operator ruling 2026-08-01; `12_UNION` §2c). Its checks
  are kernel I1/I6 now, and `tests/recurrence_guard.rs` refuses the
  module's return as a standing test. A merging node does not set two
  books against each other — it re-runs this identical kernel over the
  presented chain, so **admission is verification** (§11's nursery
  law). Any cross-node reading is a lens over the one chain, never a
  second record: *you do not reconcile what cannot diverge.*
- **I3 exactness** is the substrate both axes stand on: `C(Δ)=0` must be
  exactly zero and certificates verify exactly — impossible with floats.

**Definition sealed now; invocation wired at its phase.** The definitions
of `C(Δ)`, `orbit_lock`, and `verify_certificate` belong in the sealed math
core (§0b) — immutable, version-pinned — from the moment they exist; the
kernel and mesh call them. What lands later is the wiring: I6 invoking
`orbit_lock` lands with the spine; I4 invoking `verify_certificate` lands
on **Ruling Box 1** (what work earns — pen weight, minting rights,
receipt-coupled crystallization), and requires first moving the verifier
from `engine/solver` into the sealed math core (it is math, not runtime).
Consilience is never invoked by the kernel (cross-node — the kernel is
network-blind), but its definition `C(Δ)` is the same sealed primitive the
mesh calls. So: one sealed definition per equation, two callers, wired when
each caller's phase arrives.

## 5b. The PoW++ governance math (the MDC lineage, sealed)

The ordering weight `ω_i` in the evolution equation (§0) is not hand-set —
it is computed by the sealed governance math, the v0.1 MDC lineage, so no
node assigns weight by fiat. All exact rational, in the sealed math core:

    ω_e   = weight_e · confidence_e            # effective echelon weight
    Σ_ω   = Σ_e ω_e
    H     = 1 − (ω_winner / Σ_ω)               # disagreement entropy
    stasis    = H ≥ stasis_threshold           # default 1/2
    transmute = stasis ∧ ∀e: confidence_e < low  # the dimensional wall

Operator score reads the component semantics (Equations §3 — congruence,
coherence, cohesion), which name where capacity is strained and never what
to shrink; no gradient term survives (the damped-gradient term struck 2026-08-01). At mesh
scale the weight
`ω` becomes the PoW++ ordering weight, and the confidence `confidence_e`
is anchored to certificate-verified work (§5a) — that is the seam where
proof-of-useful-work enters the governance: you cannot buy weight, you
earn confidence by proving intelligence, and `verify_certificate` is the
proof. Until that anchor lands, `confidence` is advisory only (kernel
Charter §1: no canonicalizing weight before cryptographic identity).

## 5c. PoW++ not regurgitated PoW — the three guardrails

What makes this proof-of-*work*-plus and not a re-skin of Bitcoin, stated
as three laws a reviewer checks:

1. **Verification is replay, never a hash race.** Both axes verify by
   re-execution: convergence replays receipts to `C(Δ)=0`; useful work
   replays a certificate in O(program length). No node ever burns cycles
   on throwaway nonces. The security budget is spent on meaningful
   computation whose result is kept, not on entropy thrown away.

2. **Arbitrary-precision exact rationals, never fixed-width.** The sealed
   rational (`cfe::ratio::Ratio = BigRational` over big integers) is
   arbitrary-precision by law. Fixed-width rationals (`i64/u64`) overflow
   on accumulated game state and an overflow in consensus math is a chain
   split — this is the hard-won 2^1100 lesson (proportional decay exploded
   a denominator past any fixed width; the fix was flat, bounded, and
   big-integer). A `Rational64` is a correct principle (exact, GCD-reduced,
   canonical, cross-multiplied comparison — matching ours) but the wrong
   width for consensus over accumulating value. Floats are outlawed (I3);
   fixed-width rationals are outlawed for the same reason one layer up.

3. **One sealed rational, called never copied (§0b).** A second rational
   implementation — even a correct one — is two definitions that can
   diverge, which is the death of convergent-consensus. The kernel, the
   mesh, and the economy all call `cfe::ratio`; none reimplements it. A
   proposed `pow_kernel/rational.rs` would violate this by construction:
   the primitive already exists, sealed and arbitrary-precision.

**The hardware/work-shape note (economics, downstream of the kernel).**
"Recycle old ASICs as the useful-work lattice" is sound in spirit —
useful computation instead of discarded entropy — but needs one technical
correction: a Bitcoin SHA-256 ASIC (S9/S19) is hyper-specialized for
SHA-256 double-hashing only; its gates cannot do matrix multiplication or
general ZK. So the useful work such hardware can contribute is
hash-shaped: state-vector hashing, commitment/merkle verification,
hash-based proofs (stark-family). Matrix/AI compute needs different
silicon. The choice of work-shape — hash-based vs proof-based vs
search-based (our solver certificates today) — determines what hardware
participates, and it is a real economics ruling (Ruling Box 1 territory,
what work earns) that the kernel does not make: the kernel only requires
that whatever the work is, `solver_verify::verify_work` checks it by the
full replay (re-forge ∧ re-apply ∧ re-chain — U1, landed). The bare
`verify_certificate` is tamper-evidence only: it re-derives the chain
over the same prover-supplied triple and so cannot, alone, tell a
solution from a fabricated render — the replay-complete law names why
the anchor is `verify_work`, never the certificate check on its own.

## 5d. The Syntropic Sink (ssa) — scoping and corrections

The ssa is proposed as an equilibrium governor balancing physical entropy
`H` (aggregate hash) against useful compute `U`, producing a stabilized
gradient. Conceptually strong and apt-named: it is the syntropy/entropy
dissipative-structure thesis (§ the value law) applied to network
difficulty — absorb entropy into finality, not into a difficulty cliff;
zero-float, deterministic. But it reads aggregate H and U across nodes, so
its placement is the mesh, not the per-step kernel (network-blind). Its
definition seals into the math core (§0b, version-pinned); the mesh invokes
it, alongside `Σ_mesh`. It does not enter the six invariants. It reaches
the kernel at one existing seam: a ZK proof is a form of work certificate,
so it enters I4's mint-bound via `verify_certificate` (§5a) — nowhere else.

Three corrections it needs before it is built:

1. **The boundedness is in the normalization, not the ratio.** With
   `S = U/(H+U)`, the stabilized delta `Δg = H(1−S) + U·S = (H²+U²)/(H+U)`.
   As `H → ∞` this tends to `H` — Unbounded. Only `S ∈ [0,1]` is bounded.
   So `Δg` is bounded only if `H,U` are pre-normalized to `[0,1]` (the mass-
   normalization step), which then gives `Δg ∈ [0,1]`. The normalization is
   load-bearing; the syntropic ratio alone does not prevent runaway. An
   unbounded `Δg` driving difficulty is the runaway it claims to stop —
   state this or the guarantee is false. (Use `S` as the difficulty signal,
   or normalize first, or bound `Δg` explicitly.)

2. **Arbitrary-precision, never `Rational64`.** The `H²`/`U²` terms are a
   magnitude explosion — squaring accumulated values overflows fixed-width
   (`i64`) fast, and overflow in consensus math is a chain split (the 2^1100
   lesson, sharpened by the squaring). Ssa must use `cfe::ratio`
   (`BigRational`). §5c guardrail 2.

3. **Call the sealed rational, never a `pow_kernel/rational.rs`.** Ssa in a
   crate that reimplements the rational primitive violates §0b by
   construction. Call `cfe::ratio`. §5c guardrail 3.

ZK note: a ZK proof is a work certificate (asymmetric — expensive to make,
trivial to verify), so it plugs into `verify_certificate` at I4's seam.
Stark-family proofs are hash-based (they fit the SHA-256-ASIC work-shape
above); but the verifier must be exact/deterministic (integer-field or
hash-based) — a float or non-deterministic pairing verifier splits the
chain. Verification stays replay/exact; the proof is the scarce resource.

## 6. Algorithms (the shapes to witness, red-first)

    lawful(prior, next, step):
        I1 conservation(prior, next, step)?      # per covered event
        I2 containment(prior, step)?
        I3 exactness(next)?
        I4 bounded_creation(prior, next, step)?
        I5 bounded_decay(prior, next, step)?
        I6 fold_gate(prior, next, step)?         # orbit-lock on any fold
        -> Ok

    orbit_lock_exists(a, b, operators, K_max):   # I6 core
        for op in operators:
            cur = a
            for k in 1..=K_max:
                cur = op.apply(cur)?              # break on error
                if not cur.conservation_valid(): break
                if rcf(cur) == rcf(b): return true   # torsion cycle
                if k > 1 and rcf(cur) == rcf(a): break  # orbit exhausted
        return false

    dimensional_resolution(state):               # convergence driver
        loop:
            if transmute(state): state = unfold(state)   # §5
            state = fold(state)                          # gated by I6/§4
            if C(state) == 0 and no_orbit_lock(state): return state

## 7. What this charter does not settle (open, flagged)

- **The ordering / Sybil anchor.** The mesh measures agreement exactly; it
  does not yet make influence costly to fake. Proof-of-useful-work — weight
  earned by certificate-verified genuine difference — is the anchor, and it
  is unbuilt. Until it lands, the mesh governs cooperating nodes only.
- **Identity binding.** Hands are self-declared integers (N7 deferral).
  Adversarial multi-operator linking needs unforgeable identity first.
- **The mint rate and the reward split** (I4 at convergence scale). Monetary
  policy: the amount minted per resolved difference, and its split by
  marginal genuine contribution. These are tuned and measured in the
  prototype (attacker-universe vs honest-universe), never assumed.

## 8. The order of development (Phase 1, unstuck)

The kernel seals in this order; each invariant red-first, one witness
minimum, nothing wired as the live gate until the spine forms (§11 Phase 3):

1. I1 conservation — landed (transfers); extend per event.
2. I2 containment.
3. I3 exactness.
4. I4 bounded creation / I5 bounded decay (the value law completes).
5. The sealed vocabularies + derivation rules + `constitution_digest()`.
   **Dependency direction (mesh-operator ruling): manifest-with-witness now,
   invert at Phase 3.** The kernel holds a vocabulary manifest and a witness
   proves `cfe`'s enums match it; the enums stay in `cfe` for now. The
   inversion (kernel owns the vocabularies, `cfe` depends up) lands at Phase
   3, when wiring the gate buys its cost back — the constitution digest makes
   the sync-witness structural in the meantime.
6. I6 the fold gate + dimensional resolution (lands with the spine, §11
   Phase 2-3), re-derived red-first from the v0.1 ancestor.

When 1-6 hold, the kernel is the standard: the thing released, that every
operator runs and none may edit. Only then does wiring it as the sole gate
(Phase 3) make sense, and only then does the mesh (Phase 5) replicate it.
