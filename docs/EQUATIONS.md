# EQUATIONS — the one ledger of equations and algorithms

**Status:** DRAFT for operator ratification (2026-07-31). The running
ledger: every equation and algorithm the system lives by, found HERE
first, never by searching the tree.

## §0 The law of this ledger

1. **This is the map, not a second authority.** Each row carries the
   canonical rendering AND NAMES its authority: the law document (§)
   where its prose lives, and the sealed implementation (crate::fn)
   where its code lives. For a LANDED row the sealed code is the
   deepest authority (call-not-copy, kernel CHARTER §0b); for an
   INSCRIBED row the named document is. A disagreement between this
   ledger and an authority is a DEFECT — inscribe it, never shrug.
2. **The row-claim convention** (the tag-claim convention's sibling):
   a landing that inscribes a NEW equation or algorithm claims its row
   here in the SAME landing, before code uses it. After any merge,
   verify your row survived.
3. **The founding lesson.** Equation drift has already happened once:
   the ancestor's README said the coherence was L1 while the code
   computed SUM-OF-SQUARES (recorded at BUILD_ORDER §1a). Scattered
   restatement is how equations diverge; one ledger with named
   authority is the standing cure — same tier of protection as the
   dop.rs tag header. **Chapter two (THE PROPORTIONAL LAW, operator ruling
   2026-07-31):** the ruling reversed which side of that drift was
   law — squares are BARRED as aggregators of difference everywhere
   (they are the mathematics of minimization; ours is measurement);
   the L1 forms below are the standing law. Squares and quartics
   survive ONLY as physical law (the T⁴ discipline, T1).
4. Documents KEEP their equations inline where the mechanic lives
   (spec + mechanic + vectors in one landing stands). This ledger adds
   the map; it strips nothing.

Legend: **LANDED** — sealed in code, witnessed · **INSCRIBED** — law
written, wiring unbuilt · **DRAFT** — awaiting ratification.

---

## §1 The spine — one walk

**E1 · The evolution equation (the master).** INSCRIBED (parts landed
severally — see their rows).
Authority: SYSTEM_SPEC §0. Implementation: the composition of E2–E6,
I1–I6, G1.

    Δ_{k+1} = B_S⁻¹( Π_G( F_fold*( Σ_i ω_i N_i( B_S(Δ_k) ) − ζ∇V ) ) ) + ξ_k

**E2 · The singularity law (the angle-free collision).** DRAFT.
Authority: 16_UNIT §4a (draft) over SYSTEM_SPEC §0. Implementation:
U2 (the angle-free witness).

    B_S⁻¹( Π_G( F_fold*( … B_S(Δ) … ) ) )
    orbit_lock = ∅  →  fold closes to zero (the difference was angle)
    orbit_lock ≠ ∅  →  the collapse refuses; the residue EMITS as a
                       new plane (the bounce — emission, never
                       annihilation); the verdict is invariant to
                       frame and gauge of approach

**E3 · The five per-step invariants (PhiK).** INSCRIBED (re-derive
red-first with the spine; L1 forms per the proportional law). Authority:
SYSTEM_SPEC §0.

    I^(1) magnitude ‖Δ‖₁ = Σ|Δ| · I^(2) conservation Σ_{i<j}|Δ_ij+Δ_ji| + Σ_i|Δ_ii|
    I^(3) rank · I^(4) symmetry-class hash (RCF bytes) · I^(5) stamp T_k

**E4 · The magnitude bound (C7).** Gate LANDED in L1 —
`rfa_core::validate_step` (the bounded-magnitude gate over
`rfa_core::magnitude`); the spine-level wiring INSCRIBED. Authority:
SYSTEM_SPEC §0.

    | ‖Δ_{k+1}‖₁ − ‖F̄‖₁ |  ≤  ε_mag · ‖F̄‖₁      (F̄ = pre-projection sum)

**E5 · The commitment.** LANDED (tag 84). Authority: kernel CHARTER §3,
NS-5 §9. Implementation: `mesh::spine::seal`.

    commit[k] = H( schema ‖ VERSION ‖ bytes(Δ_k) ‖ commit[k−1] ‖ coord[k] )

**E6 · The stochastic term.** LANDED. Authority: 00_CHARTER §9,
SYSTEM_SPEC §1. Implementation: `physics::beacon`.

    ξ_k = draw( seed = commit[k−1], domain_tag )    // exact, replayable

## §2 The kernel — the judge (`lawful` is the conjunction)

Authority: engine/kernel/CHARTER.md §1–§6. Implementation:
`kernel::lawful` and its invariant functions.

**I1 · Conservation.** LANDED — `kernel::conservation`.

    Σ_orb bank(next, r) == Σ_orb bank(prior, r) + lawful_delta(step)
    (move: 0 · aperture out: −amount · in: +amount · mint: +amount · decay: −amount)

**I2 · Containment.** LANDED — `kernel::containment`.

    permits( entity(prior, actor), scope_of(event) )

**I3 · Exactness.** LANDED — `kernel::exactness` calls the sealed
`cfe::ratio::is_canonical` (never reimplements).

    every value a canonical exact rational (coprime, den > 0); no float exists

**I4 · Bounded creation.** LANDED WHOLE (U1, tag 86) —
`kernel::creation` + `solver_verify::verify_work` (the
REPLAY-COMPLETE law: re-forge ∧ re-apply every pair ∧ re-chain; an
intact hash chain over a non-solution refuses). The bound mechanism
is live and SEALED SHUT: `mint_bound = 0` until Ruling Box 1 opens
the rate by amendment.

    mint(step) ≥ 0   ∧   verify_work(work) = Ok(d)   ∧   mint ≤ mint_bound(d)
    no work → no discovery mint; the refusal carries bound and claim whole

**I5 · Bounded decay.** LANDED WHOLE (U1f) — `kernel::decay` CALLS
the sealed rule `cfe::world::demurrage_due` (call-not-copy — one
computation, two judges). `last_touch` was ALREADY committed on the
orb wire (tag 28, VENT-1) — no schema was spent; the U_DIRECTIVES §3
SCHEMA_V3 premise died at execution.

    decay ≤ min( rate · ease · elapsed, banked )   // rate FLAT, never
    elapsed = stamp − last_touch; thermally gated  // proportional (the
    an un-owed decay refuses, both sides whole     // 2^1100 lesson)

**I6 · The fold gate.** INSCRIBED (U2). Authority: kernel CHARTER §6.

    a fold is lawful  ⟺  orbit_lock(a, b) = ∅

## §3 Convergence — the value engine

Authority: SYSTEM_SPEC §4, 07_ERA_CONVERGENCE.

**C1 · The residual (cocycle).** LANDED — `rfa_core::for_each_residual`.

    R_{ijk,l} = Δ_{ik,l} − Δ_{ij,l} − Δ_{jk,l}

**C2 · The coherence functional.** LANDED — `rfa_core::coherence_functional`.
(THE PROPORTIONAL LAW — §0.3 chapter two lives here: the witnessed values
moved 24 → 12 in the RFA vectors and 29 → 7 in the NS-2 scanner, and
the tag-58 wire vector was re-extracted from the running engine.)

    C(Δ) = Σ |R|        (0 ⟺ converged; a LENS, never a driver; the
                         scalar in the SAME UNITS as its payload)

**C3 · The coboundary reduction.** LANDED — `rfa_core::coboundary_reduce`.

    Π_G(Δ) = the H¹ projection (strip gauge distortion to the core)

**C4 · The orbit lock & the fold gate.** LANDED in rfa-core (U2b) —
`rfa_core::orbit_lock` + `rfa_core::f_fold_star` (`Operator` = a
`GL_n⁺(Q)` conjugation `A·Δ·A⁻¹`; each orbit step kept only while
`verify_conservation` holds; conjugacy tested by the exact C4a
signature). The kernel-side I6 wiring (fold-class deeds into
`covered()`) is U2d, still INSCRIBED. Direction (SYSTEM_SPEC §4): a
lock FORBIDS the fold.

    orbit_lock(a, b) := ∃ op ∈ Operators, k ≤ K_max :
                        conservation-valid orbit  op^k(a) ≅_RCF b
    f_fold_star:  lock ≠ ∅ → TorsionVeto (emit)  ·  lock = ∅ → GaugeFold (lawful)

**C4a · The RCF conjugacy invariant (`≅_RCF`).** LANDED (U2a) —
`rfa_core::rcf_signature` (exact, the judge) + `rfa_core::rcf_hash`
(blake3 fingerprint, commitment-binding only). Two states are `≅_RCF`
iff their characteristic matrices `(xI − A)` share invariant factors —
the Smith-Normal-Form signature over `Q[x]`, computed per fiber layer.
The angle-free heart of the singularity law (16_UNIT §4a): a frame
change (`A ↦ PAP⁻¹`) leaves the signature identical, so the fold
verdict cannot depend on the frame of approach. THE PROPORTIONAL LAW
does not touch it (a canonical form, not an aggregator of difference);
blake3, never the ancestor's Keccak.

    A ≅_RCF B  ⟺  invariant_factors(xI − A) = invariant_factors(xI − B)

**C5 · The dimensional-resolution cycle (algorithm).** INSCRIBED.
Authority: SYSTEM_SPEC §4. unfold (raise a plane, C7-bounded above) →
fold (Π_G ∘ F_fold*, H¹-floored below) → measure (done at C(Δ) = 0
with orbit_lock ∅).

**C6 · The four-C placement (algorithm).** LANDED (CONVERGE-1).
Authority: 07 §2. Implementation: `ledger::converge`.

## §4 Governance — the MDC lineage

Authority: kernel CHARTER §5b–§5c, SYSTEM_SPEC §2. INSCRIBED (sealed
math; wiring Phase 3; confidence anchors to work at U1/U4).

    ω_e = weight_e · confidence_e        Σ_ω = Σ_e ω_e
    winner = argmax_e ω_e                H = 1 − (ω_winner / Σ_ω)
    stasis = H ≥ 1/2                     transmute = stasis ∧ ∀e: confidence_e < low

## §5 The mesh — replication and agreement

Authority: SYSTEM_SPEC §5. All INSCRIBED (re-derive as law when the
mesh era opens; the unit law of §2 is what admission re-runs).

    Σ_mesh = ( σ_self + Σ_j ω_j·σ_j ) / ( 1 + Σ_j ω_j )     // σ ∈ [0,1]
    | dΣ_mesh / dK | > τ_Σ   ⇒   re-sync
    resonance at torsion period m (mesh m=10, deep m=1024; cos(k) = cos(m−k))

**M1 · Pair conservation (the aperture seam).** Local half LANDED (I1);
the pair half is an N6 reading. Authority: NS-4 §1–§3.

    near leg + far leg = 0 exactly, across two books

## §6 Economy and thermal

**T1 · The thermal master equation.** LANDED — `cfe::thermal`.
Authority: 05 §1. The combined constant is ONE rational parameter;
`T_eq⁴` is exact; `T_eq` itself is display-only (roots never taken).

    T_eq⁴ = L(1−A) / (16πσr²)

**T2 · The difficulty vector (FORGE-2a).** LANDED as a read —
`Difficulty::from_key` (the vector from the discrete moduli; the
float/continuous form BARRED at 14 §2.5). Wiring N6-gated.
