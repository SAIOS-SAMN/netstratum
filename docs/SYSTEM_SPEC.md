# SYSTEM SPEC — node, validator, game, convergence, mesh

**Status:** WORKING SPEC (2026-07-31). The five components of the distributed
system, each with its state, its equations (extracted from the v0.1 ancestor
`convergence-engine`, exact-rational, not paraphrased), its interface, and
its mapping to the current tree. Subordinate to `docs/00_CHARTER.md` §11 and
`engine/kernel/CHARTER.md`; reasoning in `docs/KERNEL_RELAY.md`.

Notation: exact rational arithmetic throughout (the ancestor's `Q`,
netstratum's `Ratio`; NEVER a float). `Δ` is the state as an antisymmetric
relational tensor; `Δ_{ij,l}` is the entry between i and j on layer l.
Provenance: v0.1 is AI-generated sandbox — equations are TAKEN, re-witnessed
red-first here (the NS-3 precedent). No sandbox code imported.

---

## 0. THE EVOLUTION EQUATION (the spine all five share)

The master equation (v0.1 `engine.rs` T.FOUND.3, "the evolution equation —
not Markov") — ONE lawful step from Δ_k to Δ_{k+1}:

    Δ_{k+1} = B_S⁻¹( Π_G( F_fold*( Σ_i ω_i N_i( B_S(Δ_k) ) − ζ∇V ) ) ) + ξ_k

Term by term — each is a component of THIS spec:

| term | meaning | component | netstratum |
|---|---|---|---|
| `B_S`, `B_S⁻¹` | read into the working frame, write back canonical | node | rfa-core basis |
| `N_i` | the operators — candidate transformations (the DEEDS) | game | `ClosureEvent` |
| `ω_i = weight_i · confidence_i` | operator weighting (MDC echelons) | validator | §5 |
| `ζ∇V` | potential-gradient DAMPING — gradient demoted to a hint, ζ small | validator | — (to build) |
| `F_fold*` | the orbit-lock fold gate — forbids collapsing torsion-locked states | convergence | kernel I6 |
| `Π_G` | gauge projection = `coboundary_reduce` to the H¹ core (the fold) | convergence | rfa-core |
| `ξ_k` | chronicle-seeded stochastic term — replayable (CHARTER §9) | node | `physics::beacon` |

**The kernel is this equation's judge, not its driver.** The game proposes
`N_i` (a deed); the validator scores and weights; convergence gates the fold;
the node stamps and chains. `lawful(prior, next, step)` is the check that a
proposed `Δ_{k+1}` is a legal image of `Δ_k` under one term of this equation.
The equation GENERATES candidates; the kernel ADMITS or REFUSES. Never does
the kernel optimize the equation globally (KERNEL_RELAY §9).

The five per-step invariants (v0.1 `PhiK`, checked every K-step):

    I^(1) magnitude:      ‖Δ‖²
    I^(2) conservation:   Σ_{i<j}(Δ_ij + Δ_ji)² + Σ_i (Δ_ii)²   (0 = exact)
    I^(3) rank:           dim of the relational structure
    I^(4) symmetry class: RCF canonical-form hash (bytes)
    I^(5) stamp:          T_k  (TUniversal coordinate)

Magnitude bound (validation ceiling, v0.1 C7):

    | ‖Δ_{k+1}‖² − ‖F̄‖² |  ≤  ε_mag · ‖F̄‖²      (F̄ = the pre-projection sum)

---

## 1. NODE — the individual, the walker

**Responsibility.** Hold one binary's state, advance it one K-step at a time,
produce the commitment, present the chain to the mesh. Every node runs an
identical kernel; there is no local-vs-network mode.

**State.**

    Node {
      chain:   Vec<Commitment>          // the path
      head:    Committed                // current state Δ_k (in memory)
      version: kernel::VERSION          // the judging kernel version
      coord:   TUniversal { S, G, T }   // position; the vertical binds to G
    }

**Equations.**

- The commitment (VRP chaining law, §11-bound; spine-produced, kernel-blind):

      commit[k] = H( schema ‖ VERSION ‖ bytes(Δ_k) ‖ commit[k-1] ‖ coord[k] )

- The stochastic term (CHARTER §9 — a reading of history, never an oracle):

      ξ_k = draw( seed = commit[k-1], domain_tag )        // exact, replayable

- The frame transform B_S: the D4⋉Z² spatial basis (rfa-core `canonical_chain_free`);
  read Δ into the working frame, write the result back canonical. `B_S⁻¹∘B_S = id`.

**Interface.**

    advance(node) -> node'          // one K-step: propose, judge, chain
    present(node) -> Vec<Commitment>+Vec<bytes>   // to the mesh
    resume(chain) -> node           // rebuild head by replay under VERSION

**Landed / build.** Landed: `World`, `replay`, the coordinate, `physics::beacon`
(ξ). Build: the commitment (coder-04 codec + spine), `advance` as the
kernel-gated step (Phase 3).

---

## 2. VALIDATOR — the kernel, the judge

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

Operators are scored by how much they reduce C(Δ) (§4). `ζ∇V` is the
gradient of the potential V (v0.1 D.SAMN.2) used ONLY as a damping term
(ζ small) — never the driver.

**Interface.**

    lawful(prior, next, step) -> Ok | Unlawful
    score(prior, operator)    -> (confidence, orbit_target)   // per echelon
    VERSION: u64                                              // charter-pinned

**Landed / build.** Landed: `lawful`, I1 (transfers), `VERSION`. Build:
I2–I6, the echelon scoring, the sealed vocabularies + `constitution_digest()`.

---

## 3. GAME — the client, the operator surface

**Responsibility.** Where operators (AI or human) act: propose deeds, run the
board, defend sovereign states. The game NEVER edits state directly — it
submits a proposed `N_i` (deed) and the validator judges. This is the §11
"one input boundary" (the steward seam) generalized to all clients.

**State.** The operator's civilizations, worlds, strategy — all CONTENT, all
chained deeds. Nothing here is authority; it is proposals + views.

**Equations — the operators `N_i`.** A deed is an operator on Δ. The closed
set of lawful operator SHAPES is kernel vocabulary; the CONTENT they carry is
free (species is free text — bird people, tiger people — over the closed
axes). The 15 roots are the first 15 genesis deeds; a 16th is one more deed.

    propose:  N_i ∈ ClosureEvent (shape sealed) with free content
    submit:   request -> inbox -> validator.lawful -> chain | typed refusal

**Interface.**

    act(operator, deed) -> submitted | refused(cure)
    view(chain, facet)  -> projection            // a LENS, read-only (§ below)
    defend(operator)    -> strategy              // AI agents; incentive layer

**Landed / build.** Landed: `clients/cli`, all deeds, the steward inbox,
lens-shaped reads (NS-2). Build: the client talks to the local kernel (not
`world.rs` directly) — Phase 3; AI-agent defense — post-mesh.

**Observation is a lens, never a copy (§11 rule 2).** Views DERIVE from the
chain at a granted facet depth, disposable, never read back. The observer,
wall, and journals rebuild as lens projections BEFORE the mirror deletes
(the observation-continuity rule).

---

## 4. CONVERGENCE — dimensional resolution, the value engine

**Responsibility.** Merge two independently-built states, creating value from
genuine difference without discarding the residue. NOT gradient descent —
descent-and-emission, bounded both ends.

**Equations — the core measurements (v0.1, exact).**

    residual (cocycle):   R_{ijk,l} = Δ_{ik,l} − Δ_{ij,l} − Δ_{jk,l}
    coherence functional:  C(Δ) = Σ R²        (0 ⟺ converged; a LENS)
    coboundary reduce:     Π_G(Δ) = H¹ projection (strip distortion to core)

**The dimensional-resolution cycle (replaces gradient descent).**

    loop:
      unfold:  if transmute(Δ): raise dimensionality (propose N that adds a
               plane); bounded above by the C7 magnitude gate.
      fold:    Δ ← Π_G(F_fold*(Δ)); bounded below by the H¹ floor.
      measure: C(Δ); done when C(Δ)=0 AND orbit_lock stays ∅.

**The fold gate F_fold* (I6, v0.1 `fold.rs`).** Before folding a,b:

    orbit_lock(a,b) := ∃ op ∈ Operators, k ≤ K_max :
                       conservation-valid orbit  op^k(a) ≅_RCF b
    if orbit_lock(a,b) ≠ ∅:  FORBID fold  →  emit the residue as a NEW plane
                             (the mint; the residue's life after)
    else:                    fold permitted (the difference was gauge)

**The transmutation trigger (when to unfold, MDC, v0.1 `mdc.rs`).**

    transmute = stasis AND ∀e: confidence_e < low_threshold   // dimensional wall

**Convergence value (the economics).** Two nodes converge; value is minted on
irreducible genuine difference (the discovery mint, G4), gated by verified
work (§7, UNBUILT). Guards: seam-conservation (I1, `opposition` = 0), fold
cadence (AEON), and the mint-rate/split (monetary policy — MEASURED, not
assumed). Disagreement is held in STASIS (preserved, per-dimension exact
distance), never voted away.

**Landed / build.** Landed: `coboundary_reduce`, `coherence_functional`,
CONVERGE-1 (quarantine/torn-tail), `cfe::opposition` (seam), the AEON fold.
Build (re-derived from v0.1, red-first): the unfold half, F_fold* as I6,
the transmutation trigger, the mint-rate/split experiment (§7).

---

## 5. MESH — replication, admission, agreement

**Responsibility.** The peer network. Replicate chains, re-judge every
incoming step (admission = verification), aggregate agreement, hold
disagreement in stasis. NO central authority — a node with granting power
would be a mirror made of permission.

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
        lawful(prior, next, step)?      // MY kernel re-judges
        commit[k] == H(...)?            // chain integrity
      accept | refuse-typed(break named)

**The open anchor (§7, THE gap).** Σ_mesh MEASURES agreement; it does not make
weight COSTLY to fake. `ω_j` is assigned, not earned. The ordering/Sybil
resistance — weight = certificate-verified genuine difference
(proof-of-useful-work) — is UNBUILT. Until it lands the mesh governs
COOPERATING nodes only. This is the load-bearing gap for real value.

**Landed / build.** Landed (in-tree, partial): NS-5 consilience, the
resume/replication seam, the pen (MESH-1b). Build: Σ_mesh + re-sync +
resonance as re-derived law; the verified-work anchor; Byzantine fork
resolution (confluence at exact zero, or annex).

---

## 6. THE DEPENDENCY ORDER (nothing skips)

    validator (kernel)  →  node (spine/chain)  →  game (client gated)
        →  convergence (fold gate + mint)  →  mesh (replication + anchor)

Per §11: the kernel seals first (I1→I6 + vocabularies + digest); the spine
gives it the chain; the client routes through it; convergence lands with the
spine; the mesh is LAST. Nothing replicates upward until this universe is
lawful. The economics (coder-01) and the codec (coder-04) run in parallel
lanes, gated where they touch new transition semantics or committed fields.

## 7. OPEN — the honest map (unchanged, restated per component)

- **Validator/mesh:** the ordering mechanism, BOTH halves. Carrot — the
  verified-work anchor (proof-of-useful-work), weight earned not assigned,
  UNBUILT, the load-bearing gap. Stick — the alignment covenant (CHARTER
  §11): open divergence lawful, covert divergence slashed (pen/work/stake
  forfeit, reusing `CovenantSlashed`). Carrot and stick are one economy;
  their three price rulings sit together with the operator.
- **Node/mesh:** identity binding — hands are self-declared integers (N7).
  No consensus weight is canonicalizing before it binds (kernel CHARTER §1).
- **Convergence:** the mint rate + reward split (monetary policy, MEASURED);
  the residue composition law (how two residues compose into a generator —
  thesis, not spec).
- **Mesh:** Byzantine fork resolution beyond stasis.

## 8. PUPPET-1 — the attacker experiment (mesh lane, after the spine)

The deliverable that makes the "stable band" claim FALSIFIABLE. Two universes
under the landed two-node acceptance harness: one honest (genuine solver
certificates), one adversary (fabricated-difference puppets merge-farming the
mint). Measure whether the anchor REFUSES or PAYS. Composes entirely from
existing pieces (the acceptance example, the codec, branch coordinates,
solver certificates, the I6 fold gate). Owner: the mesh operator, built when
the spine lands. Inscribing it now is what turns "measure, don't assume" from
a slogan into a boarded test.
