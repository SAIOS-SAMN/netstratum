# N4 DIRECTIVES — coder-03 working orders (2026-07-30, coder-02)

Plain language by operator instruction. Scope: the N4 landing (NS-2 RQP +
the scanner). Supersedes nothing; folds into NS-2 and PICKUP when N4 lands.

## Review verdict

Coder-03's assessment is verified and approved. Checked against the tree
2026-07-30:

- N4 is unblocked. The only ordering rule is N3-before-N4 (BUILD_ORDER
  line ~74); N3 landed 2026-07-25. No open second slice gates it.
- The ancestor citation is accurate. `/opt/ipsal-autic-v0.4/iae/src/
  reduction/mod.rs` (SIGIL1 block): address = S-canonical × G-canonical
  projections of one frame-invariant form; T excluded from the key,
  kept as the chronicle stamp. CHARTER §8 ratifies this lineage.
- `rfa_core::coherence_functional` is exact-rational (`C(Δ) = Σ R²` over
  rationals, zero iff converged). Safe to use. The torus "no squaring"
  law applies to L1 distance on the star-chart, not to RFA residuals.
- Tag registry: 54 is GALAXY-1 (coder-02, reserved in the dop.rs
  header). NS-2 allocates from 55. Acceptance criterion already on the
  board: a structural query finds the planted match and refuses the
  planted near-miss; ends with replay MATCHES.
- Resonance crystals were deferred to exactly this landing (02 §5.1).

## Directives

1. **Worktree.** Work in `.claude/worktrees/coder03-n4`, branch
   `coder03/n4`, off current main. Never edit the main working tree
   while another coder has uncommitted work there. Merge to main only on
   green: build, full suite, clippy zero.
2. **One landing.** Spec + mechanic + vectors together, red-first: write
   the failing witnesses before the mechanic. NS-2 §1/§2/§3, the engine
   index, the CLI verb, and `examples/ns2_vectors.rs` are one unit.
3. **File boundaries.**
   - Yours: `protocols/NS-2_RQP.md`; a NEW engine module for the index
     (one new file, suggest `engine/cfe/src/query.rs` — do not spread
     into `engine/mirror/`); the dop.rs tag header (append-only); the
     scanner CLI verb; the vectors example; your witness files.
   - Coder-02, in flight: `engine/mirror/` galaxy module +
     `mirror/tests/galaxy_pins.rs`, `docs/13_GALAXY.md`, universe-ledger
     touchpoints. If the index needs a mirror read, ask — it will be
     exposed rather than reached for.
   - Coder-01, hot: `clients/cli/src/main.rs` (the operator menu and t0
     console just landed), conductor. CLI verbs merge one at a time;
     rebase before merging; BUILD_ORDER conflicts resolve keep-both.
4. **Tags and receipts.** Claim tags by editing the dop.rs header in
   your first commit, before code uses them. Only the crystal draw is a
   chronicle event (one tag, from 55). Query, hit, and refusal are NS-2
   protocol records in NS-2's own framing — they never enter the world
   chronicle. That is the lens law in wire form: a query is a read, no
   receipt, no state change; only the power draw closes and receipts.
5. **Exactness.** No floats anywhere. The refusal carries the residual
   list whole; `C(Δ)` is the summary scalar, not the payload.
6. **After landing.** Refresh your one lane line in PICKUP §5 and the
   tag registry note in PICKUP §4. Nothing else in PICKUP.

## Needs an operator ruling (not blocking worktree work)

1. **G3 coupling.** 04 §6 couples G3 to N4, but G3 also needs G1, which
   has not landed. Recommendation: rule that N4 lands alone and G3
   follows when G1 lands. Coder-03 proceeds on this assumption in the
   worktree; the merge waits for the ruling or a no-objection.
2. **The orb's queryable structure** (NS-2 §1). Recommendation for v1:
   S-canonical = zone geometry canonicalized under the grid symmetry;
   G-canonical = banked-resource content signature + thermal class; T
   excluded. Draft it as PLACEHOLDER in NS-2 §1; the operator may extend.
3. **CHARTER §10.** It ratifies two nodes and its ownership map has
   drifted from practice (coder-01 landed in `clients/cli/`, coder-02 is
   working in `engine/mirror/`). Proposed amendment for ratification:
   add coder-03 as a third node; replace the static directory map with
   lane claims — a node claims a landing in PICKUP §5, owns the files
   that landing names until merge, and the dop.rs header remains the one
   tag authority. Until ratified, coder-03's work above stays inside the
   files named in directive 3.

## Coordination surface between N4 and the galaxy lane (coder-02)

None hard today. Both lanes read orb state and the star-chart read-only.
The scanner addresses by {S, G} and excludes T; galaxy membership is a
derived S-side reading — consistent, no shared code. A future
scanner-queries-by-galaxy-region verb is post-GALAXY-1a and post-N4, not
part of either landing. CLI merge order for the scanner verb vs. the
galaxies view: whoever is green first merges first; the other rebases.
