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
   **Chapter three (THE MISSED SIXTH, arch-02's coverage audit,
   2026-07-31):** every document said 63 deed types; the enum held 64,
   and the remainder arithmetic (58 covered + 5 named = 63) BALANCED —
   against the wrong total — so a live deed (GaugeShiftedAt) sat
   silently uncovered, deadlocking the Phase 3 precondition it never
   appeared in. The lesson: **an arithmetic that checks out against an
   uncounted universe is the failure mode, not the safeguard.** A
   count used as law must be DERIVED from the one authority (count
   the enum, never quote a doc), and a balancing remainder proves
   nothing unless the total itself was measured.
   **Chapter four (THE ZERO THAT MEANT EMPTY, operator correction
   2026-08-01):** this ledger glossed `C(Δ) = 0` as "converged", and
   the kernel charter wrote "zero iff converged" two sections after
   stating I6 as *no difference dies without account* — a constitution
   contradicting itself about its own central claim. The sealed code
   never drifted: `coherence_functional` says zero iff the COCYCLE
   CONDITION holds, and `coboundary_reduce` says what survives IS the
   cocycle class. Only the gloss drifted — and a gloss is how a
   CONSERVATIVE law comes to read as a DESTRUCTIVE one, because
   "converged" invites minimization and minimization discards exactly
   the residual the economy mints on. The lesson beyond the fix: **a
   summary word attached to an equation is a restatement, and rots like
   one.** Where this ledger names a zero point it states what the zero
   MEASURES, never what the zero is worth.
   All drift chapters share the root: restated facts rot; derived
   facts cannot.
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

**E2 · The singularity law (the angle-free collision).** LANDED (U2e)
— `rfa_core::judged_fold` (Π_G then F_fold*) with THE ANGLE-FREE
WITNESS green (`fold_pins.rs`): the verdict is invariant under every
permutation frame of the entities and under gauge-coboundary shifts
of either input; the veto is frame-invariant too. Authority: 16_UNIT
§4a over SYSTEM_SPEC §0.

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

**I1 · Conservation, TOTAL.** LANDED WHOLE (the value-law wave,
operator directive 2026-07-31) — `kernel::conservation` +
`kernel::lawful_deltas`. Per-resource, every resource, every covered
deed; an empty declaration holds every bank still (smuggling under a
neutral deed refuses). THE SIGN LAW rides the declaration: every
amount non-negative, direction from the DEED (`AmountNotPositive` —
the negative-transfer/aperture inversion hole is CLOSED). Stateful
declarations (the council's resource, the razed zone's kind) read
PRIOR state. THE ESCROW LAW LANDED (dispatch C7, 2026-08-01):
conservation reads the CONSERVED total = banked + escrow
(`kernel::total_escrow`, `kernel::total_conserved`) — a covenant lock
or return moves collateral BETWEEN the two ledgers, so its declared
net is ZERO and a bank debit that escrows nothing refuses. Council
CLOSE is held out: its atomic multi-event settle is per-deed
threading, gated on the Phase 3 flip.

    ∀ r ∈ Resources:  conserved(next, r) == conserved(prior, r) + declared(r)
      where conserved(s, r) = Σ_orb bank(s, r) + Σ_pact escrow(s, r)
    declared: move 0 · aperture ∓ · mint + · decay − · upkeep/draw −
              · covenant lock/return 0 (banked↔escrow) · council close +
              · founding = the chartered reserves · zone = ∓cost/refund
              vectors · (all amounts ≥ 0, sign-guarded)

**I2 · Containment.** LANDED — `kernel::containment`.

    permits( entity(prior, actor), scope_of(event) )

**I3 · Exactness.** LANDED — `kernel::exactness` calls the sealed
`cfe::ratio::is_canonical` (never reimplements).

    every value a canonical exact rational (coprime, den > 0); no float exists

**I1e · Element conservation.** LANDED (operator ruling 2026-08-01) —
`kernel::element_law`: the element ledger judged with strict symmetry
to `banked`. An assay mints `richness · elapsed` (sign-guarded,
bounded by the PRIOR vein — deterministic from `(prior, step)`);
every other deed holds every element key still. Refusals
`ElementNotPositive` · `AssayExceedsVein` · `ElementBroken`.

    AssayStruck:  amount ≥ 0  ∧  amount ≤ richness · (stamp − last_assay)
    ∀ element e:  total(next, e) == total(prior, e) + declared(e)

**I4 · Bounded creation.** LANDED WHOLE (U1, tag 86) —
`kernel::creation` + `solver_verify::verify_work` (the
REPLAY-COMPLETE law: re-forge ∧ re-apply every pair ∧ re-chain; an
intact hash chain over a non-solution refuses). **THE EMISSION LAW —
RULED (Ruling Box 1, operator 2026-08-01; the kernel VERSION 1→2
amendment; evidence: the #17 calibration, 5898351):** `mint_bound`
pays the ruled `f` in exact ladder integers — L1/ladder terms, no
norms, no roots (§0.3 chapter two stands).

    mint(step) ≥ 0   ∧   verify_work(work) = Ok(d)   ∧   mint ≤ f(d)
    f on the ladder rungs 0–5:  0 · 0 · 0 · 1 · 4 · 7
    (saturated rungs pay zero; OFF-LADDER difficulty = 0 — a payload
     never names its own price)
    no work → no discovery mint; the refusal carries bound and claim whole

**I5 · Bounded decay.** LANDED WHOLE (U1f) — `kernel::decay` CALLS
the sealed rule `cfe::world::demurrage_due` (call-not-copy — one
computation, two judges). `last_touch` was ALREADY committed on the
orb wire (tag 28, VENT-1) — no schema was spent; the U_DIRECTIVES §3
SCHEMA_V3 premise died at execution.

    decay ≤ min( rate · ease · elapsed, banked )   // rate FLAT, never
    elapsed = stamp − last_touch; thermally gated  // proportional (the
    an un-owed decay refuses, both sides whole     // 2^1100 lesson)

**I6 · The fold gate.** KERNEL HALF LANDED (U2d) — `kernel::gauge_law`:
a fold transition's only lawful LOCAL effect is its declared key move
(gauge declaration==effect, every grade, every covered deed — a key
smuggled under a neutral deed refuses `GaugeBroken`). The PAIR half —
`orbit_lock` between the two MERGING chains via the sealed
`rfa_core::judged_fold` (C4/C4a, E2) — is the merge site's law
(converge/mesh admission), the aperture-pair seam pattern: kernel
enforces the local half, the seam enforces the pair.

    LOCAL:  next.gauge == declared(prior.gauge, deed)   ∀ grades
    PAIR:   a fold is lawful  ⟺  orbit_lock(Π_G(a), Π_G(b)) = ∅

## §3 Convergence — the value engine

Authority: SYSTEM_SPEC §4, 07_ERA_CONVERGENCE.

**C1 · The residual (cocycle).** LANDED — `rfa_core::for_each_residual`.

    R_{ijk,l} = Δ_{ik,l} − Δ_{ij,l} − Δ_{jk,l}

**C2 · The coherence functional.** LANDED — `rfa_core::coherence_functional`.
(THE PROPORTIONAL LAW — §0.3 chapter two lives here: the witnessed values
moved 24 → 12 in the RFA vectors and 29 → 7 in the NS-2 scanner, and
the tag-58 wire vector was re-extracted from the running engine.)

    C(Δ) = Σ |R|        (0 ⟺ THE COCYCLE CONDITION HOLDS — the
                         differences COMPOSE; a LENS, never a driver;
                         the scalar in the SAME UNITS as its payload)

**`C(Δ) = 0` IS NOT "CONVERGED"** (operator correction 2026-08-01 — the
row's earlier gloss is STRUCK). Zero says the relations compose,
`Δ_ik = Δ_ij + Δ_jk` for every triple; that is what the sealed
`rfa_core::coherence_functional` states and all it states. It does NOT
say the states are identical, and it does NOT say difference was spent.
Genuine difference is the H¹ class — C3 PRESERVES it by construction
(see that row) and C cannot see it, so a state may read `C(Δ) = 0` and
carry a large irreducible core. **We never discard residuals** (I6:
*no difference dies without account*).

**C3 · The coboundary reduction.** LANDED — `rfa_core::coboundary_reduce`.
The reduction is CONSERVATIVE, and this row is the reason C2's zero can
never mean emptiness: it strips ONLY the coboundary — the
coordinate-frame distortion that was never difference — and what
survives IS the cocycle class (the sealed `rfa_core::reduce` header
states exactly this). Two Deltas differing only by a frame reduce to
the same representative; a PURE coboundary reduces to zero because it
carried no structural content to begin with. Nothing irreducible is
ever removed here.

    Π_G(Δ) = the H¹ projection (strip the frame; the core SURVIVES)

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
fold (Π_G ∘ F_fold*, H¹-floored below) → measure. The cycle RESTS at
`C(Δ) = 0` with `orbit_lock = ∅`: the relations compose and no torsion
binds the pair. Resting is not emptiness — the H¹ core STANDS on the
floor the loop cannot fold past. What the cycle removes is
contradiction and frame; what it cannot reduce, it EMITS (C4).

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
