# System spec — node, validator, game, convergence, mesh

**Status:** working spec (2026-07-31). The five components of the distributed
system, each with its state, its equations (extracted from the v0.1 predecessor
`convergence-engine`, exact-rational, not paraphrased), its interface, and
its mapping to the current tree. Subordinate to `docs/00_CHARTER.md` §11 and
`engine/kernel/CHARTER.md`; reasoning in `docs/KERNEL_RELAY.md`.

Notation: exact rational arithmetic throughout (the predecessor's `Q`,
Netstratum's `Ratio`; never a float). `Δ` is the state as an antisymmetric
relational tensor; `Δ_{ij,l}` is the entry between i and j on layer l.
Provenance: v0.1 is AI-generated sandbox — equations are taken, re-witnessed
red-first here (the NS-3 precedent). No sandbox code imported.

---

## 0. The foundation — D and V (capacity, not descent)

**Operator ruling 2026-08-01 — the foundation is dimensional capacity.**
Gradient descent is a failed topology: uni-directional, and it discards
the information the economy is made of. It is struck whole, not demoted
to a hint, and the uni-directional linear timeline is struck with it.
"The master equation" is refused with it — there is no one master; the
system runs many algorithms at many scales. The standing law is
`docs/EQUATIONS.md` §1 (**D0–D4**), summarized here:

    V     = the committed state, read whole (indivisible; the totality law)
            — the substance. Δ(V) is the relation it expresses.
    D(Ω)  = the space D spans — the room, and D's own side.
            Ω ≢ V: a room is not its furniture      ruled 2026-08-02
    D     = the structure surviving Π_G( Δ(V) )        ruled 2026-08-01

    measured:     D is a reading, never a quota — nothing is shrunk to fit
    growth:       orbit_lock ≠ ∅ → the fold refuses, the residue emits,
                  D rises by the plane, by receipt (earned, never assumed)
    contraction:  D falls only as Π_G strips a coboundary — never below
                  H¹; forbidden where orbit_lock ≠ ∅

**Capacity growth and the discovery mint are one event.** The index set
`(i,j,k,l)` and the arities `Delta{dim,fiber}` are the container, not
D — that reading is struck. A universe may hold enormous arity and near
zero D when its differences are all frame.

The v0.1 evolution equation (`engine.rs` T.FOUND.3) is kept below as
history only, with its gradient term struck:

    Struck:  Δ_{k+1} = B_S⁻¹( Π_G( F_fold*( Σ ω_i N_i( B_S(Δ_k) ) ) ) ) + ξ_k

Its surviving terms remain real machinery and keep their rows — the
composition is what was refused, not the parts:

| term | meaning | component | Netstratum |
|---|---|---|---|
| `B_S`, `B_S⁻¹` | read into the working frame, write back canonical | node | rfa-core basis |
| `N_i` | the operators — candidate transformations (the deeds) | game | `ClosureEvent` |
| `ω_i = weight_i · confidence_i` | operator weighting (MDC echelons) | validator | §5 |
| ~~the damped-gradient term~~ | **Struck 2026-08-01** — the gradient; a failed topology, never built | — | — |
| `F_fold*` | the orbit-lock fold gate — forbids collapsing torsion-locked states | convergence | kernel I6 |
| `Π_G` | gauge projection = `coboundary_reduce` to the H¹ core (the fold) | convergence | rfa-core |
| `ξ_k` | chronicle-seeded stochastic term — replayable (Charter §9) | node | `physics::beacon` |

**The kernel is this equation's judge, not its driver.** The game proposes
`N_i` (a deed); the validator scores and weights; convergence gates the fold;
the node stamps and chains. `admits(prior, next, step)` is the check that a
proposed `Δ_{k+1}` is a legal image of `Δ_k` under one term of this equation.
The equation generates candidates; the kernel admits or refuses. Never does
the kernel optimize the equation globally (KERNEL_RELAY §9).

The five per-step invariants (v0.1 `PhiK`, checked every K-step):

    I^(1) magnitude:      ‖Δ‖₁ = Σ |Δ_ij,l|   (the L1 mass — the proportional law)
    I^(2) conservation:   Σ_{i<j}|Δ_ij + Δ_ji| + Σ_i |Δ_ii|   (0 = exact)
    I^(3) rank:           dim of the relational structure
    I^(4) symmetry class: RCF canonical-form hash (bytes)
    I^(5) stamp:          T_k  (TUniversal coordinate)

Magnitude bound (validation ceiling, v0.1 C7):

    | ‖Δ_{k+1}‖₁ − ‖F̄‖₁ |  ≤  ε_mag · ‖F̄‖₁      (F̄ = the pre-projection sum)

(The proportional law — operator ruling 2026-07-31: the v0.1 squared forms
are carried into Netstratum as L1 — squares are barred as aggregators
of difference; every zero-set is unchanged. Squares survive only as
physical law — the T⁴ discipline of 05.)

---

## 1. Node — the individual, the walker

**Responsibility.** Hold one binary's state, advance it one K-step at a time,
produce the commitment, present the chain to the mesh. Every node runs an
identical kernel; there is no local-vs-network mode.

**State.**

    Node {
      chain:   Vec<Commitment>          // the path
      head:    Committed                // current state Δ_k (in memory)
      version: kernel::version          // the judging kernel version
      coord:   TUniversal { S, G, T }   // position; the vertical binds to G
    }

**Equations.**

- The commitment (VRP chaining law, §11-bound; spine-produced, kernel-blind):

      commit[k] = H( schema ‖ version ‖ bytes(Δ_k) ‖ commit[k-1] ‖ coord[k] )

- The stochastic term (Charter §9 — a reading of history, never an oracle):

      ξ_k = draw( seed = commit[k-1], domain_tag )        // exact, replayable

- The frame transform B_S: the D4⋉Z² spatial basis (rfa-core `canonical_chain_free`);
  read Δ into the working frame, write the result back canonical. `B_S⁻¹∘B_S = id`.

**Interface.**

    advance(node) -> node'          // one K-step: propose, judge, chain
    present(node) -> Vec<Commitment>+Vec<bytes>   // to the mesh
    resume(chain) -> node           // rebuild head by replay under version

**Landed / build.** Landed: `World`, `replay`, the coordinate, `physics::beacon`
(ξ), and the commitment (SPINE-1 + #36, 2026-07-31 — `mesh::spine::seal`
binds schema ‖ kernel::version ‖ state ‖ predecessor ‖ coordinate; the record
repeats the blob's leading byte, never a constant). Build: `advance` as
the kernel-gated step (Phase 3 — per-deed threading rides the flip by
operator ruling).

---

## 2. Validator — the kernel, the judge

**Responsibility.** The one sealed function every node runs to admit or refuse
a step. Pure, total, no I/O, encoding-blind. The full law is
`engine/kernel/CHARTER.md`; this is its role in the equation.

**Equations — the six invariants (`lawful` is their conjunction).**

    I1 conservation:  Σ_orb bank(next,r) == Σ_orb bank(prior,r)     [moves]
                      ≡ PhiK I^(2) = 0 on the value tensor
    I2 containment:   permits(entity(prior, actor), scope_of(event))
    I3 exactness:     every value canonical exact rational (coprime, den>0)
    I4 creation:      value(next) - value(prior) == mint(step),  gated ≥ 0
    I5 decay:         decay = min(rate · ease · elapsed, held),  rate flat
    I6 fold gate:     a fold is lawful  iff  orbit_lock(a,b) = ∅   (§4)

**The operator weighting (MDC, the `ω_i` of the evolution equation).** Each
proposed operator (deed) is scored across the decision echelons E; the
effective weight and the disagreement entropy:

    ω_e = weight_e · confidence_e
    Σ_ω = Σ_e ω_e
    winner = argmax_e ω_e
    entropy H = 1 - (ω_winner / Σ_ω)          // v0.1 mdc_entropy
    stasis  = H ≥ threshold                    // default threshold = 1/2

**Struck 2026-08-01** (operator ruling; `48_CAPACITY_THREAT_MODEL` §10 AV-1, §11):

    Struck:  Operators are scored by how much they reduce C(Δ).
    Struck:  the damped-gradient term as a damping term.

The first was the descent — a greedy minimization of a class-blind
scalar, and the last place in this document where an objective survived
after the gradient itself was struck at §0. The guard had been placed on
the damped-gradient term while the objective leaked through the scoring rule; both are now
gone. `∂Δ` is not deleted but uncaged — a directional operator, never
collapsed to a magnitude, never bounded above (`24_FORM` §2b).

**No quantity is scored, minimized, or descended.** An operator is
admitted or it roots: where a proposed step composes, it folds; where it
does not, the non-composition opens capacity by receipt rather than
being refused (`48_CAPACITY_THREAT_MODEL` §11, the bifurcation law). `C` survives
only as the component semantics, and only as a predicate — the
per-residual cocycle test, never the L1 sum, which is a lens reading
(`17_CAPACITY` §5).

Recorded, never silently dropped: the echelon arithmetic above stands as
the disagreement/stasis reading. It is what the MDC measures. What is
struck is its use as a selection objective.

**Interface.**

    lawful(prior, next, step) -> Ok | Unlawful
    score(prior, operator)    -> (confidence, orbit_target)   // per echelon
    version: u64                                              // charter-pinned

**Landed / build.** Landed: `lawful`, `VERSION`, and I1–I5 whole
(2026-07-31): I1 conservation (moves + aperture legs + mints/decays),
I2 containment, I3 exactness, I4 complete with the PoUW anchor —
`verify_work` the replay-complete judge, `mint_bound` sealed shut
until Ruling Box 1 (U1, tag 86) — and I5 complete with the flat decay
bound calling the sealed demurrage rule (U1f). Coverage: 59 of 64
deed types judged; the rest refuse `Unjudged` (the W-waves,
U_DIRECTIVES §5). Build: I6 (U2), the echelon scoring, the sealed
vocabularies + `constitution_digest()`.

---

## 3. Game — the client, the operator surface

**Responsibility.** Where operators act — organic, synthetic or procedural,
the seat does not care: propose deeds, run the
board, defend sovereign states. The game never edits state directly — it
submits a proposed `N_i` (deed) and the validator judges. This is the §11
"one input boundary" (the steward seam) generalized to all clients.

**State.** The operator's civilizations, worlds, strategy — all content, all
chained deeds. Nothing here is authority; it is proposals + views.

**Equations — the operators `N_i`.** A deed is an operator on Δ. The closed
set of lawful operator shapes is kernel vocabulary; the content they carry is
free (species is free text — bird people, tiger people — over the closed
axes). The 15 roots are the first 15 genesis deeds; a 16th is one more deed.

    propose:  N_i ∈ ClosureEvent (shape sealed) with free content
    submit:   request -> inbox -> validator.lawful -> chain | typed refusal

**Interface.**

    act(operator, deed) -> submitted | refused(cure)
    view(chain, facet)  -> projection            // a lens, read-only (§ below)
    defend(operator)    -> strategy              // AI agents; incentive layer

**Landed / build.** Landed: `clients/cli`, all deeds, the steward inbox,
lens-shaped reads (NS-2). Build: the client talks to the local kernel (not
`cfe::world` directly) — Phase 3; AI-agent defense — post-mesh.

**Observation is a lens, never a copy (§11 rule 2).** Views derive from the
chain at a granted facet depth, disposable, never read back. The observer,
wall, and journals rebuild as lens projections before the ledger's display
half deletes (the observation-continuity rule — the operator's window never
goes dark).

**The lens is the level (operator ruling 2026-08-01).** Mirror and
ANTI-MIRROR are deprecated as framings whole; `12_UNION` §2c carries the
ruling and what survives it. `engine/mirror` is already deleted (d606b01)
and `cfe::opposition` excised (c7cad9e), both held by standing tests in
`engine/kernel/tests/recurrence_guard.rs`. A lens derives from the one
chain and holds no copy, so it has nothing to reconcile — which is why it,
and not opposition, is what §11 leaves standing.

---

## 4. Convergence — dimensional resolution, the value engine

**Responsibility.** Merge two independently-built states, creating value from
genuine difference without discarding the residue. Not gradient descent —
descent-and-emission, bounded both ends.

**Equations — the core measurements (v0.1, exact).**

    residual (cocycle):   R_{ijk,l} = Δ_{ik,l} − Δ_{ij,l} − Δ_{jk,l}
    coherence functional:  C(Δ) = Σ |R|       (0 ⟺ the cocycle condition
                                               holds — the differences
                                               compose; a lens; the
                                               proportional law)
    coboundary reduce:     Π_G(Δ) = H¹ projection (strip the frame; the
                                               core survives)

**`C(Δ) = 0` never means "converged"** (operator correction 2026-08-01).
Zero says the relations compose, not that the states are identical and
not that difference was spent. The genuine difference is the H¹ class:
`Π_G` strips only the coboundary — the coordinate-frame artifact that
was never difference — and what survives is the cocycle class, which C
cannot see. A state may read zero and carry a large irreducible core.
Reduction removes contradiction and frame; **residuals are never
discarded** (kernel Charter I6, *no difference dies without account*).

**The dimensional-resolution cycle (replaces gradient descent).**

    loop:
      unfold:  if transmute(Δ): raise dimensionality (propose N that adds a
               plane); bounded above by the C7 magnitude gate.
      fold:    Δ ← Π_G(F_fold*(Δ)); bounded below by the H¹ floor.
      measure: C(Δ); the cycle rests when C(Δ)=0 and orbit_lock stays ∅
               — relations compose, no torsion binds. Resting is not
               emptiness: the H¹ core stands on the floor below.

**The fold gate F_fold* (I6, v0.1 `fold.rs`).** Before folding a,b:

    orbit_lock(a,b) := ∃ op ∈ Operators :
                       conservation-valid orbit  op^k(a) ≅_RCF b
    if orbit_lock(a,b) ≠ ∅:  forbid fold  →  emit the residue as a new plane
                             (the mint; the residue's life after)
    else:                    fold permitted (the difference was gauge)

**The transmutation trigger (when to unfold, MDC, v0.1 `mdc.rs`).**

    transmute = stasis and ∀e: confidence_e < low_threshold   // dimensional wall

**Convergence value (the economics).** Two nodes converge; value is minted on
irreducible genuine difference (the discovery mint, G4), gated by verified
work (§7, unbuilt). Guards: seam-conservation (I1, `opposition` = 0), fold
cadence (AEON), and the mint-rate/split (monetary policy — measured, not
assumed). Disagreement is held in stasis (preserved, per-dimension exact
distance), never voted away.

**Landed / build.** Landed: `coboundary_reduce`, `coherence_functional`,
CONVERGE-1 (quarantine/torn-tail), the AEON fold. (`cfe::opposition` was
listed here as landed; it is excised — c7cad9e — and its checks are
kernel I1/I6 now.)
Build (re-derived from v0.1, red-first): the unfold half, F_fold* as I6,
the transmutation trigger, the mint-rate/split experiment (§7).

---

## 5. MESH — replication, admission, agreement

**Responsibility.** The peer network. Replicate chains, re-judge every
incoming step (admission = verification), aggregate agreement, hold
disagreement in stasis. No central authority — a node with granting power
would be a second truth made of permission (the figure formerly read "a
mirror made of permission"; mirror is deprecated as a framing, 12 §2c).

**Equations (v0.1 `consensus.rs`, exact).**

- Mesh-aggregate sigma (weighted agreement; self weight 1):

      Σ_mesh = ( σ_self + Σ_j ω_j · σ_j ) / ( 1 + Σ_j ω_j )
      σ normalized to [0,1];  ω_j = sluice_weight(peer)

- Hard re-sync trigger (drift over K-steps):

      | dΣ_mesh / dK |  >  τ_Σ    ⇒   re-sync

- Resonance confirmation at torsion period m (mesh m=10, deep m=1024):
  agreement confirmed when RCF orbits align at the m sample points
  (`RESONANCE_M10_COS`, cos-symmetric: cos(k) = cos(m−k)).

**Admission (the trustless core).**

    receive(segment):
      for step in segment:
        lawful(prior, next, step)?      // this kernel re-judges
        commit[k] == H(...)?            // chain integrity
      accept | refuse-typed(break named)

**The open anchor (§7, the gap).** Σ_mesh measures agreement; it does not make
weight costly to fake. `ω_j` is assigned, not earned. The verification half
exists (`verify_work`, U1 — landed); the anchoring of ω to it — weight =
certificate-verified genuine difference — is unbuilt and waits on Ruling
Box 1 + cryptographic identity (N7). Until it lands the mesh governs
cooperating nodes only. This is the load-bearing gap for real value.

**Landed / build.** Landed: the single-chain integrity door (`receive`),
the pen (MESH-1b), the spine + adoption door + kernel-observed play (#36,
2026-07-31 — every conductor world walks its own chain, judged where
covered, holes stamped `judged=0`), and **the agreement law whole
(`mesh::agree`, 2026-07-31): Σ_mesh, the re-sync trigger, resonance m=10 —
re-derived exact from v0.1; the majority anchor not carried (unanimous or
refused, NS-5 §1)**. Build: the spine uplink (present/receive for
commitment segments); the ω anchoring (above); Byzantine fork resolution
(confluence at exact zero, or annex).

---

## 6. The dependency order (nothing skips) — validity vs build-queue

**The validity order (what may be trusted before what):**

    validator (kernel)  →  node (spine/chain)  →  game (client gated)
        →  convergence (fold gate + mint)  →  mesh (replication + anchor)

Per §11: the kernel seals first (I1→I6 + vocabularies + digest); the spine
gives it the chain; the client routes through it. Nothing replicates upward
until this universe is lawful — that is a trust dependency, and it holds.

**The build queue is not this order** (operator ruling, mesh-second,
2026-07-31; `docs/AUDIT_2026-07-31.md` §1). The game is live, so the
mesh is secondary to the kernel, not last in line:

    1. Kernel — seal the judge (I1–I5 landed; I6 = U2; the W-waves)
    2. MESH   — the door rises immediately behind it (#36 spine
                adoption; `receive` already stands; then Σ_mesh,
                re-sync, resonance, the verified-work anchor)
    3. everything else — absorbed, never free-standing:
         into the kernel: convergence's fold gate (I6), the PoUW mint
           bound (I4 via `verify_work`, landed U1), coverage extensions
         into the MESH:   Σ_mesh, admission, the anchor, fork law
         elsewhere:       client verbs, conductor policy, lenses

This is a positional correction, not a law change: the kernel Charter
§5a already dissolves convergence into the I4/I6 seams, and §5 makes
admission = re-judging under one's own kernel — the mesh half of each
axis is the same unit law re-run on received bytes (16_UNIT §1). The
economics (the rate) and the codec run in parallel lanes, gated where
they touch new transition semantics or committed fields.

## 7. Open — the honest map (unchanged, restated per component)

- **Validator/mesh:** the ordering mechanism. Carrot — the
  verified-work anchor (proof-of-useful-work): the kernel half is
  built (U1, 2026-07-31 — `verify_work` replay-complete, the mint
  bounded, sealed shut at rate zero); the rate and the mesh half
  (weight earned not assigned) await Ruling Box 1 and the mesh era —
  still the load-bearing gap for real value, but now one ruling and
  one wiring, not a build. Stick — the alignment covenant (Charter
  §11): open divergence lawful, covert divergence slashed (pen/work/
  stake forfeit, reusing `ForwardSlashed`). Carrot and stick are one
  economy; their three price rulings sit together with the operator.
- **Node/mesh:** identity binding — hands are self-declared integers (N7).
  No consensus weight is canonicalizing before it binds (kernel Charter §1).
- **Convergence:** the mint rate + reward split (monetary policy, measured);
  the residue composition law (how two residues compose into a generator —
  thesis, not spec).
- **Mesh:** Byzantine fork resolution beyond stasis.

## 8. The attacker experiment — deferred (not boarded; far too early)

The eventual falsifiability test for the "stable band" claim: two universes
under the two-node harness — one honest (genuine solver certificates), one
adversary (fabricated-difference puppets merge-farming the mint) — measuring
whether the verified-work anchor refuses or pays. It composes from pieces
that mostly do not exist yet (the anchor itself is unbuilt, §7). Recorded
here as the shape of the future test, but deferred and unboarded (operator
ruling): an attacker experiment before the kernel is sealed, the spine
lands, the mesh replicates, and the anchor exists has nothing to test. It revives
only when those prerequisites stand.
