# OPEN QUESTIONS — unresolved research vectors

**Status:** RESEARCH / NOT LAW. Everything here is an *open* direction,
recorded so a good idea is not lost and a rejected one is not silently
revived as if settled. Nothing in this file is inscribed law; a vector leaves
this file only by landing under the BUILD_ORDER process, or by being formally
closed with its verdict. If a proposal here conflicts with an inscribed law
(CHARTER, the SEALs, an NS spec), the inscribed law wins until a landing says
otherwise.

---

## Q1. Holographic-GPU field engine & Proof-of-Useful-Work mesh

**The proposal (2026-07-26).** Replace the DAG/chronicle + NS-5/NS-7 stack
with a GPU-accelerated "holonomic holographic field" engine: treat the
spatial-thermal continuum as a continuously differentiable manifold, compute
sector sub-volumes on GPU tensor cores, reach consensus by *boundary
stitching* between neighbouring sectors, and make the physics simulation
itself the Proof-of-Useful-Work. Settlement proofs would be "holonomy /
sheaf-cohomology" checks over sector boundaries instead of succinct proofs
over a trace.

**Verdict: REJECTED as an architecture replacement; kernels salvaged
elsewhere.** It cannot replace the inscribed stack, for reasons that are
load-bearing, not tunable:

1. **GPU float breaks exact-rational determinism (CHARTER §3.2) and SEAL B.**
   IEEE-754 addition is not associative; two GPUs — or two schedules on one
   GPU — diverge in rounding. That is fatal to bit-exact cross-node replay.
   "Spectral rational grids (Chebyshev/Fourier over integer fields)" is
   self-contradictory: those are floating-point approximation methods. Exact
   arithmetic on GPU reduces to finite-field bignum work — which is what real
   ZK provers already do.
2. **Holonomy ≠ tamper-evidence.** A flat connection (Ω=0) gives
   path-independence, not a tamper-evident history. Determinism (same replay)
   and tamper-evidence (no undetected rewrite) are different guarantees; the
   second needs commitments — i.e. the chronicle. Discarding block history
   violates SEAL B directly.
3. **Boundary-stitching is a security hole.** Matching neighbours on a shared
   boundary proves *consistency*, not *interior integrity* — a node can
   compute a false interior while keeping its edges clean. "Cheat inside and
   the mesh rejects you" is false in general.
4. **AdS/CFT misapplied.** The holographic principle is a specific
   gravity/CFT duality; it does not make arbitrary game-state bookkeeping
   reconstructable at O(1) from a 2D boundary.
5. **Homology proves structure, not history.** Betti numbers can show "a
   route exists" or "a loop is closed" but cannot prove "≥ N joules delivered
   by tick T". Persistent homology is also expensive; the "beats ZK on
   throughput, independent of tick count" claim compares different problems.
6. **PoUW's open problem is verification.** "The simulation is the useful
   work" is an old, attractive idea; it stalls because useful work must be
   cheaply verifiable and hard to fake, which returns you to needing a
   succinct proof. Boundary-stitching does not close this.

**Kernels kept (already inscribed):**

- Domain decomposition — sectors to nodes, exchange boundary data — is
  standard and already the spirit of NS-5. Kept as engineering, not as
  "holography" or free consensus.
- Succinct settlement proofs over exact-rational inequalities (`A/B ≥ C/D`) via a
  real STARK / multilinear prover (Lasso, Jolt, HyperPlonk) — inscribed at
  `05_THERMAL_BALANCE.md` §7.3 as the buildable version of the "topological
  proof" idea.

**What could reopen this vector, honestly:** a deterministic exact-rational GPU
arithmetic path (finite-field, reproducible across hardware) *plus* a real
succinct proof of each interior transition (not boundary agreement). At that
point it is no longer "holographic field magic" — it is GPU-accelerated ZK
over the existing chronicle, which is a legitimate performance project that
does not touch the SEALs. That is the form in which it may return.

---

## Q2. Reopening paths for the sink-dual rejections (05 §1.4)

Three rejections from the sink-dual draft carry lawful ways back in;
recorded so a flat "no" doesn't bury a good tuning lever.

1. **Passive sink minting — reopened by distinction.** `radiator_base`
   is already a passive, ungated sink (every orb self-cools; landed in
   THERMAL-1). What stays EARNED is *bankable/exportable* capacity
   (`S_cool`). Baseline passive, tradeable earned. Reopen trigger:
   balance shows the cold start too punishing without a baseline.
2. **Banked-store decay — reopenable as receipted demurrage.** Banked
   state never mutates silently (SEAL B), but decay MAY accumulate as
   workspace and settle in one receipt at collect/transfer (the
   VentSettled shape), or be held off by storage upkeep. Reopen trigger:
   balance wants banked joules not to be a free eternal store outside
   Glasskeeper vaults.
3. **A global conservation sheet — reopenable at N6.** Per-side
   invariants (05 §1.4) stay the orb-level ground truth; they compose
   into a universe-level audit (Σ encased + Σ vented = Σ excess;
   Σ absorbed = Σ sink-consumed) checked at mesh consensus (NS-5), not
   imposed as an orb law.

Closed for good: un-receipted anything (SEAL B — spills are workspace
until `VentSettled` seals them, which is already the design); the
terminology rulings (charter §8); section numbering (mechanical).
