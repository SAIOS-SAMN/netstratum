# 25 — STANDING: build debt, the decision surface, and the D > 0 proof

**Status:** LAW, split out of the retired `18_TORSION.md` on 2026-08-03 by operator
ruling. It carries §6, §7 and §8 — what the engine cannot yet express,
what of the law enters the kernel, and the measured proof.

**Why these three travel together.** Each is a statement about the law's
STANDING rather than its content: what is owed, what is binding, and
what has been demonstrated. None of them rules on physics; all of them
report on it.

Section numbering is UNCHANGED — `kernel/tests/vacuity_pins.rs` cites §8
and `rfa-core/src/capacity.rs` cites §6.2, and both still resolve.

---

## §6 BUILD DEBT — what the engine cannot yet express

Recorded honestly, per Article III's own standard: naming a gap is
computation, hiding one behind vocabulary is cosmetic.

1. **CLEARED, and it was pointing at STRUCK law** (lens lane report,
   2026-08-02). This debt read *"the cosmetic meter is not measured …
   first build"*. It must not be built: Article III was struck whole
   and rewritten as THE SCIENCE FOUNDATION, and its own strike list
   names that meter — `deeds(w)/(1 + ΔD(w))` — together with "cosmetic
   non-utility" as a legal term. The reasoning is the article's own:
   *physics needs no penalty; a closed loop at uniform potential does
   no work, not because a law forbids it but because there is no free
   energy in it.*

   **LAW 4 now carries that ground**, thermodynamically rather than
   morally, through `dual_pole` / `Vacuity::Stasis` — landed, and
   already consulted by `kernel::capacity_law`. Building the window
   arithmetic would have implemented a corpse. What the lens owed
   instead was a SURFACE for the row a world stands in, which nothing
   displayed; that is `lens::standing` (`19_LENS` §12).

   **CORRECTED 2026-08-13 (doc audit):** both citations above are dead.
   `dual_pole` / `Vacuity` were STRUCK by operator ruling 2026-08-04
   (`engine/rfa-core/src/boundary.rs:36-45` — "THE DUAL-POLE LOCK IS
   STRUCK... both the type and its verdict are removed"; measured
   60 beats / 374 solves / 0 mints / 124 withheld before the strike).
   The four-row classification (Void/Unframed/Stasis/Entangled) no
   longer exists as a type; `boundary_form`'s divergence reading
   survives as `24_FORM §2d`'s sign law input. `lens::standing` /
   `19_LENS §12` is also gone — that lane's own doc marks §12 REMOVED
   2026-08-04. This paragraph's claim is superseded, not current law.
2. **`D` is not measured.** `PhiK I^(3)` ("rank: dim of the relational
   structure") specifies it and no code reads it. Until `D` is
   measured, Articles II, III and IV have no operand.
3. **The torsion test has no caller.** `orbit_lock` / `f_fold_star`
   are landed and witnessed in `rfa-core` and are invoked nowhere.
   Article I's mandate is unenforced until the merge site calls them.
3a. **The kernel admits gaps silently** (§2g). At an entangled seam it
   checks an inequality and lets everything below the bound through
   unaccounted. It must instead hold both sides and DISPOSITION the
   gap — reduce, emit, or refuse carrying both whole. No `==` anywhere;
   the equality would discard the very quantity that tells a forgery
   from a discovery. Breaking amendment (major `VERSION`). First bite
   is verified present in the tree:
   `engine/kernel/tests/discovery_pins.rs:266` — *"The degenerate mint
   is lawful: 0 ≤ bound 0"* — plus its header claim at line 10. Land
   red-first, and the refusal must name both sides.
4. **Continuous-domain forms need exact realizations.** The exterior
   derivative, the sup over a manifold, and a real principal eigen-axis
   have no exact-rational form. (`det U* → ∞` was the fourth entry and
   is REMOVED with §2c's strike — `U*` no longer names anything, and a
   realization owed to a struck object is a debt owed to nobody.) Infinity is
   already barred by standing ruling (the T∞ purge: the torus wraps
   finitely). These land as: finite differences over the committed
   index set; max over a finite committed set; a wrapped finite locus;
   and the SNF invariant-factor axis of Article I. **The articles bind
   as written; these are the realizations that satisfy them.**

5. **THE UNWIRED SEALED LAYER — the registry** (operator ratification
   2026-08-01). Eleven public items in the sealed math and the bridge
   are landed, witnessed by pins, and consumed by NO `src/` caller.
   They are not the `GROUND` fault — that was a constant declared and
   never computed, and these compute and are tested. They are
   CAPABILITY AWAITING A CALLER, and the absence is recorded here
   rather than left implied:

   | item | crate | what it waits on |
   |---|---|---|
   | `verdict_per_dimension` | kernel::bridge | **its own doc says "THIS belongs in the judge" — and it is not in the judge** |
   | `rank_is_defined` | kernel::bridge | the Article II / IV gate (debt 5a) |
   | `fold_certified`, `refold`, `observe`, `occupied_dimensions` | kernel::bridge | the merge site and the observation resolver |
   | `capacity_anchored` | rfa-core::capacity | kept as the drift record against `capacity` |
   | `is_pure_circulation` | rfa-core::capacity | THE SIGN LAW's enforcement point (§2d) |
   | `expanded_from` | rfa-core::reduce | the support-expansion mint rule |
   | `commutes_with`, `congruate`, `is_identity` | rfa-core::fiber | the non-abelian fiber's caller |
   | `is_admitted` | rfa-core::boundary | the `Vacuity` gate — **DEAD, see the 2026-08-13 correction at §1** (struck 2026-08-04) |

5a. **`rank_is_defined` is a recorded obligation, not enforcement.**
   Its doc-comment claimed callers "MUST" consult it while no caller
   existed. The word is downgraded and the obligation stands: the
   Article II / IV gate consults it when built, because an unresolved
   absence lets phantom topology mint phantom rank.

6. **THE STANDING CHECK — LAW 8 made mechanical.** The `GROUND` fault
   (declared, documented, asserted, never computed) was found by
   sweeping the sealed layer for public items no `src/` code consumes.
   That sweep is cheap, it would have caught the fault the day it
   landed, and it is the only mechanical enforcement LAW 8 has. Run it
   at each landing that touches `rfa-core` or `kernel`.

---

## §7 THE DECISION SURFACE — what of this law enters the KERNEL

This section exists to be READ before anything is built. It sorts every
claim in this document by the one test that decides kernel membership,
and it is the intended reading order for that decision.

**THE TEST** (kernel CHARTER §1, KERNEL_RELAY §9 — the trap everyone
hits): the kernel judges ONE step, locally, exactly, totally. A LOCAL
per-step fact derivable from `(prior, next, step)` can enter
`lawful()`. A GLOBAL, dynamical, or over-time property CANNOT, at any
strength of conviction — *enforce the local invariants whose universal
application yields the global band; measure the global with a lens;
never make the kernel optimize a global quantity.*

**THE GROUNDING TEST** (added 2026-08-01 after §2h was struck): a claim
may be written as law here only with a file and line naming what
BEHAVIOUR it drives. A doc-comment is not behaviour; a vocabulary entry
is not a mechanism.

### 7a. KERNEL — local, per-step, admissible in `lawful()`

| claim | why local | what it touches |
|---|---|---|
| **§2f entanglement** — no capacity without potential motion | reads `(prior, step)` only | `creation()` in `kernel/src/lib.rs` |
| **§2g gap disposition** — hold both sides, dispose the gap | one deed, one comparison | `creation()`; new `Unlawful` arm carrying both sides |
| **Art. IV monotone** — a step may not lower the rank outside lawful reduction | `prior`→`next` comparison | new invariant, the `gauge_law` shape |
| **Art. II non-vacuous** — `D = GROUND` claims no ALLOCATION (`Degree::is_above_ground`, never a zero; standing is admission's, §2c′) | property of `prior` | `containment()`, or a new gate beside it |
| **`D` itself** — the structure surviving `Π_G(Δ(V))` | pure function of one state | kernel gains its `rfa-core` dep (anticipated at U_DIRECTIVES §4) |

### 7b. LENS — global or over-time, MUST NOT enter the kernel

| claim | why not local | where it lives |
|---|---|---|
| **Art. III standing surface** — which row a state occupies (Void / Unframed / Stasis / Entangled) | a property of one state, read for display and never gating | `lens::standing` — **DEAD, see the 2026-08-13 correction at §1** (`19_LENS §12` REMOVED 2026-08-04) |
| **Art. III allocation** — the Eclipse, as claim-revocation | a policy over many actors | conductor / mesh |
| **§2e traverse congruence** — `α ≅ Ω` | spans a whole chain | `lens` over the spine |
| **§2c union** — one `Δ`, two complementary projections | a property of one state's algebra, already computed | `rfa_core::boundary` + `capacity` (landed) |
| **§2d THE SIGN LAW** — egress / ingress / balanced | a per-vertex reading of one state | `rfa_core::capacity` (`divergence`, `egress`) |
| **§2j field force** — friction, melt, key gap | already landed OUTSIDE the judge, priced per deed | `cfe/src/world/` (`economy.rs::transfer_as`, `mod.rs::pair_melt`, `mod.rs::key_distance`) |

**§2i is a THIRD category** — neither kernel nor lens but a
CLASSIFICATION obligation on the vocabularies: every element declared
syntropic or entropic, with an UNMEASURED tripwire for the remainder.
It binds the codec and the element grammar, not `lawful()`.

Putting any 7b row into `lawful()` destroys the kernel's nature. That
is not a style preference — a judge that reads a window is no longer
pure, no longer total, and no longer replayable step-by-step.

### 7c. NOT LAW — framing, unbuilt, or struck

- **§2h eclipse/vantage** — STRUCK; ungrounded, needed by nothing.
- **Luminal, holonomic, egress naming** — useful description of the
  mint and the chain; drives no behaviour on its own.
- **`Ξ`/`ΔD` as objectives** — barred outright (Art. III).

### 7d. THE ORDER TO DECIDE IN

`D` (7a row 5) gates the other four kernel rows and both Article III
readings — nothing above it can be judged until a state can be measured.
It is one caller over the landed `rfa_core::coboundary_reduce`, and
§2f binds its shape: **the capacity read and the potential read must
happen in one pass over one prior state**, or the entanglement is
broken before it is built.

The remaining open rulings, none of which the author may decide:
whether `I5`'s decay and the assay bound are entangled seams (§2g);
what congruence relation `α ≅ Ω` uses (§2e); and whether the breaking
`VERSION` bump for gap disposition lands alone or with the ARC corpus
work-shape.

---

## §8 PROOF — `D > 0`, measured on a real universe

The whole construction is worthless if the rank is always zero. It was,
for the entire life of this design, and this section is the record that
it no longer is. **Reproduce it with `cargo test -p kernel --test
bridge_pins`.**

### 8a. The claim

There exists a lawful committed state whose relational structure has
rank greater than zero, read through the kernel by the sealed math.

### 8b. The construction

Four orbs. Four standing bonds forming a ring — 1→2, 2→3, 3→4, 4→1 —
each of rate 3. **No chords:** orbs 1 and 3 have no obligation between
them, and neither do 2 and 4.

    engine/kernel/tests/bridge_pins.rs
      a_universe_with_an_unclosed_obligation_ring_reads_capacity

### 8c. Why the rank is non-zero

Two facts, and both are needed:

1. **It is a cocycle.** The cocycle condition is checked only on FILLED
   triangles — a triple whose three edges are all present. Every triple
   here is missing an edge, so no condition applies and no residual
   stands. Nothing fails to compose.
2. **It is not a coboundary.** Walking the ring does not return where it
   started, so no assignment of per-orb potentials reproduces these
   obligations. `rfa_core::holonomies` reduces against a spanning forest
   and finds one edge the forest cannot explain, carrying charge.

Cocycle and not coboundary is exactly non-trivial `H¹`. The measured
value is `rank = 1`, with `rank_is_defined()` true and `residuals`
empty.

### 8d. The negative controls, which fix its meaning

- **A ring that closes consistently reads rank 0.** Same four orbs, but
  the obligations balance around the loop. Structure is the FAILURE to
  close, never the loop itself.
- **Adding the missing 1—3 chord fills the triangles** and the same
  economic situation re-reads as strain rather than topology. One fact,
  seen through the support.
- **A world with no obligations reads rank 0**, defined — a true zero,
  not an unmeasurable one.
- **A new orb changes nothing.** Chartering adds a vertex and moves the
  rank not at all: growth is ALLOWED, never forced.

### 8e. What had to be true first

Each of these was a live defect, and any one of them alone forces
`D ≡ 0` or `D` undefined:

| defect | effect if uncorrected |
|---|---|
| the algebra had no caller | `Δ(V)` is not a computation; nothing to measure |
| projecting banks differenced | a pure coboundary; reduction annihilates it |
| a dense complex | every cocycle is a coboundary; rank always zero |
| absent read as zero | phantom triangles report phantom strain |
| unverified absence | phantom cycles mint rank never earned |
| residuals summed to a scalar | the emission source is destroyed |
| the residual clause on the gate | non-composing states barred from being read |

### 8f. Honest limits

The rank is measured and **used by nothing.** `lawful()` does not read
it, Article II does not gate on it, Article IV does not defend it, and
no mint is bounded by it. The proof is that the number is real, not
that the engine acts on it.
