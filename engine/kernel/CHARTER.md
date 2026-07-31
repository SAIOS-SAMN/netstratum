# THE KERNEL CHARTER — law for the one validated update function

**Status:** LAW (ratified 2026-07-31). Governs `engine/kernel`. Subordinate
to `docs/00_CHARTER.md` §11 (the One-Truth Amendment); this document is its
Rule 3 made concrete — the invariants, the equations, the algorithms, and
the order they land.

**Amendment tiers (or the cadence dies).** Sealing the vocabularies must NOT
freeze content development behind constitutional process. Two tiers, both
changing the constitution digest so the handshake still catches every
difference:
- **Additive amendment (minor `VERSION` bump):** a new event tag WITH its
  invariant coverage, or a new closed-vocabulary entry. Content-development
  cadence rides this tier. Two operators on different additive tiers are
  simply DIFFERENT UNIVERSES until they match — lawful nurseries to each
  other; cross-version convergence is a bridge problem, never a silent skip.
- **Breaking amendment (major `VERSION` bump):** a changed MEANING of an
  existing rule or field. Rare, charter-tier deliberation.

The protocols named as sealed law (unambiguously): **RFA** (exact-rational
algebra → I3), **DOP** (NS-1, the object model → schema/vocabulary), **VRP**
(NS-3, the RECEIPT CHAIN — verification/parent-linking → the commitment
shape; confirmed VRP, not RQP), and **CFE** (the runtime whose transition
rules become the invariants). Law seals; mechanism stays in its crate,
version-pinned.

**Provenance (honest):** the convergence mechanics below are re-derived from
the v0.1 ancestor `/opt/ipsal-autic-v0.1/repos/convergence-engine`
(`fold.rs`, `mdc.rs`, `consensus.rs`, `DIMENSIONAL_RESOLUTION.md`,
`DIMENSIONAL_BLOCKCHAIN.md`) — which is AI-generated, sandbox, unwitnessed.
We take the DESIGN and re-witness it red-first in this disciplined tree (the
NS-3 "spec from the working ancestor" precedent). No sandbox code is
imported. What the ancestor solves — convergence better than gradient
descent — it solves; what it does NOT solve — the ordering/Sybil anchor —
remains open (§7).

---

## 1. What the kernel is

The kernel is the sole validated update function:

    lawful(prior, next, step) -> Ok | Unlawful

- **prior, next**: full committed state, in memory.
- **step**: `{ event, actor, stamp }` — the deed CLAIMING to carry prior to next.
- **pure, total, no I/O, encoding-blind, network-blind.** It never
  serializes (the codec holds the byte monopoly, so kernel and chain cannot
  disagree about bytes), and it never reads the mesh, the uplink, or any
  peer state. All arrows point ONE way: `engine/kernel` imports `cfe` only
  — the audit is grep-simple, and it stays that way by law. The kernel
  judges MEANING; the codec fixes FORM; the mesh does DISTRIBUTION; none
  reaches back into the judge. My verbs call `lawful()`; `lawful()` calls
  nothing of the mesh's.

The kernel judges ONE step. It never optimizes a global quantity — a global
property (coherence, stability) is not a per-step fact and cannot enter
`lawful()` without destroying its nature. The design law: **enforce the
LOCAL invariants whose universal application yields the global band;
MEASURE the global with a lens; never enforce it in the judge.**

**THE SEALED DEFAULT ARM — refuse-unknown, never admit-unknown.** The
default arm of the judge REFUSES: a deed no invariant covers does not enter
(`Unlawful::Unjudged`). Coverage grows by amendment — each landing moves an
event from the default-refuse arm to an explicit judged arm — and the kernel
wires as the live gate (§11 Phase 3) only when `Unjudged` is unreachable,
every event covered. The distinction that guards this forever: **SEAL A is a
READER's law, never the JUDGE's.** Readers skip unknown TAGS so old software
survives new content (SEAL A, right and permanent). A judge that skips
unknown DEEDS is a hole exactly the size of every future tag. Two different
laws, one word — "unknown" — and conflating them silently rebuilds
default-admit. (Landed: the flip + witness `the_sealed_default_refuses_the_unjudged`.)

**No consensus weight before cryptographic identity.** Until identity binds
(N7), any confirmation-weight computation is ADVISORY ONLY — never
canonicalizing. Weight summed over self-declared integer hands IS the puppet
web, pre-built by us; this one line forbids a well-meaning seat from
implementing it early.

## 2. The invariant set (the whole of the law)

`lawful()` is the conjunction of six invariants. A step is admitted only if
all hold. Each lands red-first, one witness minimum, coverage growing by
amendment (never by silent omission — §11 clause 2).

**I1 — Conservation (closed moves).** A deed that only MOVES value may not
change the world total of that value.

    for a value-moving event on resource r:
        Σ_{orb} bank(prior, orb, r) == Σ_{orb} bank(next, orb, r)

Refuses `ConservationBroken { r, before, after }` with both totals whole.
(LANDED for `ResourceTransferred`; extends per event by amendment.)

**I2 — Containment.** The actor holds the scope the event requires, read
from `prior` (identity precedes the world, NS-6 §1.2).

    permits(entity(prior, step.actor), scope_of(step.event))  must hold

**I3 — Exactness.** Every value in `next` is a canonical exact rational
(coprime, positive denominator). No float, no wall-clock, no ambient
entropy ever entered (CHARTER §9). Structural: non-canonical cannot be
constructed, only refused.

**I4 — Bounded creation (syntropy).** Value is created ONLY by a lawful
mint, and only in the amount its rule permits. No fiat (05 §1.2, 01 §6.1:
joules only by encasement). At convergence scale the mint is the discovery
mint (G4), amount tied to VERIFIED difference (§7).

    value(next) - value(prior)  ==  mint(step),   mint >= 0, gated

**I5 — Bounded decay (entropy).** Value is destroyed ONLY by the flat,
denominator-bounded rule. NEVER proportional (the 2^1100 defect: proportional
decay collapses canonical denominators without bound).

    decay(step) = min(rate * ease * elapsed, held),   rate flat, bounded

**I6 — Conservation of difference (the fold gate).** Every irreducible
residual is REDUCED as gauge, MINTED as a new dimension, or REFUSED — never
silently discarded. Concretely, a fold (declaring two states the same and
collapsing them) is lawful only if no orbit-lock binds them (§4).

**The container law (resolves I6 × sealed-schema; codec-side precision is
the mesh operator's).** I6's emission "mints a new fiber plane," but §11 says
state-schema changes are amendments — runtime dimension-growth and a sealed
schema collide UNLESS the schema seals CONTAINERS, not fixed dimensions.
Inscribe what the tree already does: the gauge key is a VARIABLE-LENGTH
container inside schema v1, grown only by receipt (`GaugeShiftedAt` opens
planes); banked elements are likewise a container. So: **the schema seals the
containers; the fibers grow inside them lawfully at runtime; new container
KINDS are amendments.** Without this line I6 either shatters the codec gate
or becomes a schema mutation — the codec is what would break, so its
byte-level precision belongs to the mesh/codec lane.

Together I1/I4/I5 are the full value law: **value moves conserved, is
created only by bounded lawful mint, destroyed only by bounded lawful
decay.** I6 is the convergence law: **no difference dies without account.**

## 3. The measured quantities (lenses, never enforced)

Read to KNOW the state; never targets the kernel drives.

- **Coherence functional** (the residual measure; zero iff converged):

        C(Δ) = Σ_i R_i^2        (exact rationals; squaring lawful on RFA
                                 residuals, NOT on torus distance — that
                                 stays L1, the short way around)

- **Constitution digest** (the link handshake — same universe?):

        D_const = H( VERSION ‖ vocabularies ‖ derivation_rules ‖ schema )

  Two nodes link only if `D_const` matches. The canonical genesis roster is
  a NAMED deed-set the kernel hashes but does not contain — start canonical
  (same digest) or fork your own (different digest, still lawful).

- **The commitment** (the spine binds it; the kernel never reads it):

        commit[n] = H( schema ‖ VERSION ‖ state_bytes[n]
                       ‖ commit[n-1] ‖ coordinate[n] )
        coordinate[n] = TUniversal{ S, G, T }[n]

## 4. Convergence — dimensional resolution, not gradient descent

Gradient descent minimizes to a point and discards the residual — uni-
directional, lossy. It is REJECTED as the convergence method (kept only as a
direction hint). Convergence is **descent-and-emission**, bounded on both
ends (the ancestor's `DIMENSIONAL_RESOLUTION`):

    loop:
        unfold:  if the residual needs dimensions the state cannot express,
                 propose an operator that RAISES dimensionality
                 (trigger §5); bounded above by validation gates.
        fold:    project to the irreducible H^1 core (coboundary_reduce);
                 bounded below by the cohomological floor — cannot collapse
                 past H^1.
        measure: C(Δ). converged if C(Δ) == 0 AND no orbit-lock remains.

**The fold gate (I6, from the ancestor's `F_fold*`).** Before folding Δa and
Δb, test for a torsion cycle:

    orbit_lock(Δa, Δb) := ∃ op ∈ Operators, k ≤ K_max :
                          conservation-valid orbit  op^k(Δa) ≅ Δb

    if orbit_lock(Δa, Δb) ≠ ∅:
        FORBID the fold — Δa, Δb are genuinely distinct perspectives;
        the orbit IS the symmetry the residue forms; collapsing it
        destroys information.  ⇒ the residue is EMITTED as a new
        dimension (the mint, I4), not discarded.
    else:
        fold permitted — the difference was gauge, lawfully reduced.

This is the residue's "life after": genuine difference (a torsion orbit)
becomes a new fiber plane (the fold's up-arrow); gauge difference reduces
(the down-arrow). The convergence creates the shared class AND preserves the
genuine difference as new symmetry — syntropy and entropy in one step,
conserving the total (I6).

## 5. When to unfold — the transmutation trigger (MDC)

The state grows a dimension only on a measured wall, never freely (the
ancestor's `mdc_resolve`). Over the decision echelons e ∈ E each with weight
w_e and confidence c_e:

    entropy    = mdc_entropy( max_e(w_e·c_e), Σ_e(w_e·c_e) )
    stasis     = entropy ≥ stasis_threshold
    transmute  = stasis AND ∀ e: c_e < low_threshold   (the dimensional wall)

`transmute` true ⇒ unfold (raise dimensionality). Otherwise hold in
**stasis** — disagreement is PAUSED and PRESERVED for resolution, never
discarded to a majority. This is convergence-not-consensus: the mesh records
WHERE in relational space nodes disagree and by what exact rational, not
merely THAT a vote passed.

## 5a. The two axes — where each half lives (POW++ accounting)

POW++ is two axes, and each splits into a LOCAL half (kernel) and a
CROSS-NODE half (mesh). The split is forced by network-blindness: the
kernel judges one node, one step; anything spanning two nodes is the mesh's.

- **Proof of Convergence = I6 (kernel) + consilience (mesh).** Local: the
  fold gate, `orbit_lock(a,b) = ∅` — a single-state exact check, I6, lands
  WITH the spine. Cross-node: `C(Δ) = 0` consilience between two chronicles
  — outside the kernel (a LENS the mesh reads; §3), never enforced in the
  judge.
- **Proof of Useful Work = I4's mint-bound (kernel) + work-verification
  (mesh).** Local: the anchor on I4 — a deed claiming work carries a
  solver certificate, and the mint may not exceed what `verify_certificate`
  proves (you cannot mint faster than you produce certified intelligence;
  anti-inflation and anti-Sybil in one bound). Cross-node: the mesh receive
  door replays embedded certificates before adopting a spine.
- **Seam conservation = I1 (kernel) + opposition (mesh).** Local: aperture
  legs sum zero within one book — one unconditional line (why the
  void-share ruling mattered). Cross-node: `cfe::opposition` at a merge.
- **I3 exactness** is the substrate BOTH axes stand on: `C(Δ)=0` must be
  EXACTLY zero and certificates verify EXACTLY — impossible with floats.

**Not baked now, and why.** Consilience in the kernel breaks
network-blindness (the audit `engine/kernel` imports `cfe` only would
fail). `verify_certificate` in the kernel bakes unwired dead code (solves
enter no chronicle today) AND breaks the dependency law — the verifier is
in `engine/solver`; wiring it first requires moving a certificate-verify
primitive to a shared base. The I4 anchor lands on **Ruling Box 1** (what
work earns — pen weight, minting rights, receipt-coupled crystallization);
I6 lands with the spine. Both are named hooks now, wired at their phase.

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

## 7. What this charter does NOT settle (open, flagged)

- **The ordering / Sybil anchor.** The mesh MEASURES agreement exactly; it
  does not yet make influence costly to fake. Proof-of-useful-work — weight
  earned by certificate-verified genuine difference — is the anchor, and it
  is UNBUILT. Until it lands, the mesh governs cooperating nodes only.
- **Identity binding.** Hands are self-declared integers (N7 deferral).
  Adversarial multi-operator linking needs unforgeable identity first.
- **The mint rate and the reward split** (I4 at convergence scale). Monetary
  policy: the amount minted per resolved difference, and its split by
  marginal genuine contribution. These are TUNED and MEASURED in the
  prototype (attacker-universe vs honest-universe), never assumed.

## 8. The order of development (Phase 1, unstuck)

The kernel seals in this order; each invariant red-first, one witness
minimum, nothing wired as the live gate until the spine forms (§11 Phase 3):

1. I1 conservation — LANDED (transfers); extend per event.
2. I2 containment.
3. I3 exactness.
4. I4 bounded creation / I5 bounded decay (the value law completes).
5. The sealed vocabularies + derivation rules + `constitution_digest()`.
   **Dependency direction (mesh-operator ruling): manifest-with-witness NOW,
   invert at Phase 3.** The kernel holds a vocabulary manifest and a witness
   proves `cfe`'s enums match it; the enums stay in `cfe` for now. The
   inversion (kernel owns the vocabularies, `cfe` depends up) lands at Phase
   3, when wiring the gate buys its cost back — the constitution digest makes
   the sync-witness structural in the meantime.
6. I6 the fold gate + dimensional resolution (lands with the spine, §11
   Phase 2-3), re-derived red-first from the v0.1 ancestor.

When 1-6 hold, the kernel IS the standard: the thing released, that every
operator runs and none may edit. Only then does wiring it as the sole gate
(Phase 3) make sense, and only then does the mesh (Phase 5) replicate it.
