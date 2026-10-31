# Handoff — the pickup document

**Status:** process carrier (beside `BUILD_ORDER.md`,
`OPEN_QUESTIONS.md`, and `PICKUP.md`), for any engineering node
joining or resuming work. The carrier split: **JSON is never authority**
(`docs/JSON_DISPLAY_ONLY.md`); **state is never read
here** — the current landing and the landed ledger live in
`BUILD_ORDER.md` (the single state carrier), and the fast cold-start map (run
commands, landed table, lanes, dark spots) lives in `PICKUP.md`. This
document carries the practice: the discipline, the coordination
protocol as lived, the state-model laws, and the earned gotchas. It
changes only when a law or habit changes; the dated gap list in §9
refreshes at each major handoff.

### Stand-down marker (2026-08-08)

**Multi-day pause.** State snapshot is **`docs/archive/PICKUP.md` §0** and
`BUILD_ORDER.md` §Stand-down board. Tip **`fce10e0`** on clean `main`.
Resume: read Pickup §0 → smoke tests → approval before new lands.
Do not re-open dual-law hubs or u16 clade unless directed.

## 1. Reading order (before any change)

1. `docs/00_CHARTER.md` — every law binds; §8 terminology, §9
   stochastic, §10 the parallel-node protocol.
2. `docs/BUILD_ORDER.md` — the landed ledger + the boards. What to do
   next is decided here, on approval. (`docs/archive/PICKUP.md` is the fast
   map of the same state — orientation, never authority.)
3. `docs/OPEN_QUESTIONS.md` — rejected ideas (never silently revive)
   and recorded deferrals with their reopen triggers (Q4 = the
   ideology board's).
4. `docs/03_SUBSTRATE_CONTRACT.md` — mechanic → primitive → witness
   (coder-01 authority).
5. The design doc for whatever is touched (05 thermal, 09 ideology, 10
   tech, …) and `protocols/NS-1_DOP.md` §2.3 (the permanent tag
   registry).

## 2. The landing discipline (how every mechanic ships)

- **One landing at a time. Spec + mechanic + test vectors in the same
  landing (law xvii). Commit only on explicit approval.**
- **Red-first with recorded output:** write the witnesses in
  `engine/cfe/tests/<name>.rs` first, run them, record the red (compile
  errors count), then implement to green.
- **Evidence blocks verbatim** in the BUILD_ORDER landed entry: clippy
  zero (`cargo clippy --workspace --all-targets -- -D warnings`), the
  witness count (N/N workspace witnesses green — sum every suite), the
  named exact measurements, and the CLI arc played end-to-end
  (`printf '…' | cargo run -p netstratum-cli -- --plain`).
- Every witness file ends its arcs with
  `assert_eq!(replay(world.chronicle()), world.committed())` — replay
  matches is the SEAL B connectivity proof and it has caught real bugs.
- Refusals are typed, name the cure in the CLI, and are computed pure
  before any mutation — a refusal orphans nothing (no receipts, no
  bank moves).
- Fractions render whole ("3/2", never 1.5). Placeholder parameters
  live in `dop.rs` as named fns beside the types they rule.

## 3. The parallel-node protocol (who touches what)

**Superseded 2026-08-04 — the two-node model below is history.** Work
is now organised as named lanes, one git worktree and branch each
(`kernel`, `lens`, `capacity`, `record`, `harness`, `cognition`,
`remediations`, `syntropy`, …). **The live ownership table is not in
this file** — keeping a second copy here is how the old one went stale.
Read the lane map before claiming any territory, run `pwd` and
`git branch --show-current` at session start and after every merge, and
never share a worktree: two nodes in one working tree is a state git
cannot untangle.

`/opt/netstratum` (root) is the landing zone and is nobody's workspace.

Two naming traps that have already cost a session each: **"Charter"**
names both `docs/00_CHARTER.md` (the record lane's) and
`engine/kernel/CHARTER.md` (the kernel lane's); **"harness"** names both
the harness lane (the conductor's) and `engine/solver/tests/
harness_pins.rs` (the ARC solve harness, the cognition lane's). One
word, two lanes — do not let it mean whichever is convenient.

The struck two-node model, kept not rewritten:

- ~~**coder-02 (main tree):** `docs/` (except 03), `engine/cfe/` in
  practice (world.rs named in the charter; dop.rs/tests by usage),
  `clients/cli/`.~~
- ~~**coder-01 (worktree):** `engine/rfa-core`, `engine/physics`,
  `engine/solver`, `engine/mirror`, `engine/conductor` (by usage),
  `docs/03`, workspace `Cargo.toml`, `tools/observer.py`.~~
  **`engine/mirror` has not existed since 2026-07-31** — the crate is
  `engine/ledger` (the mirror name is excluded; `docs/00_CHARTER.md`
  §3a) and the
  display half is `engine/lens`.
- Cross-territory happens only as a mechanical relay (the P1
  precedent): e.g. adding a renderer arm the compiler demands. Flag it
  in the landing entry and the commit message.
- **The other node commits mid-session.** Before every commit: `git
  status` + `git log --oneline -5`; re-read shared append-points
  (BUILD_ORDER, the dop.rs registry header, NS-1 §2.3) before editing
  them. Doc numbers and tags are claimed by git history order — the
  earlier claim wins (the 08 collision was resolved by renumbering to
  09).

## 4. The tag registry (permanent — one tag, one meaning, never reassigned)

Authoritative header: top of `engine/cfe/src/dop.rs`. Mirrors: NS-1
§2.3, the BUILD_ORDER boards, and Pickup §4 (the current spent/
reserved/next-free numbers live there, not here). Update the header +
NS-1 + BUILD_ORDER at landing, never before. Contiguity is not law.

## 5. The state model (get this right or replay breaks)

- **Committed vs workspace:** what has no receipt does not persist.
  Banked state never mutates silently. Accruals (yield, vent, friction,
  bond spans, demurrage) are workspace and settle by one receipt at a
  touch — the VentSettled shape. `World` workspace fields: `accrued`,
  `vent_pending`, `friction_pending`. Committed families beyond orbs:
  `debts` (ABSORB-1), `bonds` (BOND-1), `covenants` (FORWARD-1) — all
  three live in `Committed`, are cloned by `committed()`, rebuilt by
  `replay()`, and carried by `resume()`. **A new committed family must
  join all four places.**
- **The touch law:** `demurrage_due` computes pure; affordability is
  judged post-decay; `touch()` applies, receipts, and anchors
  `last_touch` unconditionally — every settle is a touch (including
  `UpkeepSettled` on replay; that divergence was a real caught bug).
- **Flat-rate decay only** (the denominator-explosion fix, commit
  7635eb7): decay = rate × elapsed capped at the store, never
  proportional-compounding (banked × (31/32)^n grew denominators to
  ~2^1100 live).
- **Replay-fact discipline:** the live path and the `replay()` arm must
  make identical reads in identical order (crisis flags before bank
  moves, etc.). Shared helpers used verbatim by both paths:
  `settle_debts`, `credit_forwards`, `touch`. If a live event needs
  state replay can't derive, put the fact in the receipt (the covenant
  collateral rides the seal receipt so the fidelity curve never
  re-runs on replay; the bond severance load rides tag 34).
- **Stamps:** receipts appended during `tick()` carry the PRE-walk
  clock value. World-driven resolutions (absorption lapse, bond term
  expiry, covenant release/slash) fire at the tick whose opening `now`
  crosses the boundary — witness deadline arithmetic counts ticks
  accordingly. Fulfilment wins the covenant deadline tie.
- **Bond re-anchoring:** non-ending settles always have boundary ==
  stamp (proven at the BOND-1 landing); replay re-anchors at
  `receipt.stamp`. `BondSettled` is its own tag precisely because a
  manual transfer must stay distinguishable from bond flow.
- **Empty-book readings (placeholder values):** reciprocity reads 1
  (fresh orbs trade frictionless); `cohesion_ratio` reads None ("no
  deeds yet"); the collateral curve reads full quota (the empty book is
  not a credential).

## 6. The cohesion ledger (the ideology board, landed whole)

Eight books on `ColonyOrb`, all committed, replay-rebuilt, wire tag 41
(emitted only when nonzero — zero-book orbs stay byte-identical to
pre-ideology ones):

| axis | syntropic | entropic | posts when |
|---|---|---|---|
| compassion | `given_in_crisis` | `drawn_in_crisis` | transfer/absorb while the counterparty is famine- or overheat-struck, uncleared, at the deed's stamp |
| love | `bond_flow` | `severance_load` | bond settles only (crisis books belong to deliberate deeds); load = rate × notice on a cut or default, zero on term expiry |
| integrity | `forwards_fulfilled` | `forwards_breached` | covenant release/slash, value-weighted by collateral stake |
| morality | `returned_across` | `extracted_across` | reciprocity-window payments / lapses (windows open at absorption) |

One deed may post several axes (a transfer can post compassion, pay
absorption debts, credit a covenant, and accrue friction). Ratios
derive at read (`dop::cohesion_ratio`), never stored. Friction taxes
reciprocity divergence on transfers only — bonds and absorptions are
exempt by design (the standing loop is the friction-free channel). The
engine judges nothing; it measures.

## 7. Build, test, play

```
cargo test --workspace                                  # 809/809 green at 8067f60 (2026-08-04)
cargo clippy --workspace --all-targets -- -D warnings   # must finish clean
cargo test -p cfe --test world_<name>                   # one witness file
printf 'found meridian\n…\nquit\n' | cargo run -p netstratum-cli -- --plain
```

The conductor's converge/polity suites drive real archives and take
~3–4 minutes; the rest is instant. `var/` is runtime state, never
tracked, never a source of truth.

## 8. Compiler-enforced connectivity (expected breakage)

Adding a `ClosureEvent` variant intentionally breaks three exhaustive
matches until arms are added. **Paths corrected 2026-08-04** — all three
had moved, and the crate the old text named no longer exists:

| match site | was |
|---|---|
| `engine/cfe/src/world/replay.rs` | `world.rs::replay` (the file was split) |
| `clients/cli/src/main.rs` | unchanged |
| `engine/lens/src/render.rs` (`render_receipt`) | `engine/mirror/src/lib.rs` |

`engine/ledger/src/converge.rs` and the scribe's `journal.rs` have
catch-alls. Adding an `ActionRefused` variant breaks the CLI `refuse()`
match — the new arm must name the cure.

**And A fourth site the compiler does not enforce, which is the whole
lesson of this section.** `kernel::covered()` decides which deeds the
judge judges at all, and it is a `matches!` list — adding a variant
does not break it, so a new deed silently becomes `Unjudged`. That has
already happened: `BookPosted` is live and uncovered, and the pin meant
to prove totality enumerates a hand list of 64 against an enum of 71
(Pickup §0). **When adding a variant, add it to `covered()` by hand,
or make the pin exhaustiveness-checked so the compiler starts
enforcing what this section assumes it does.**

**Gotcha (CLI planet cursor):**
`chart` automatically advances the active planet cursor to the newly
created charted. Chained `chart` operations must explicitly re-pin the
source (`chart <name> from <source>`) or each charter drains the
previous charted's reserves instead of the intended source — leaving a
row of empty orbs. Found live at the N4 landing: the scanner truthfully
reported three drained colonies as resonant (empty ≅ empty), which is
the lens working and the arc broken.

## 9. Verification gaps on record (dated: 2026-07-27; **refreshed 2026-08-04**)

The landed state, the lanes, and the next-work candidates live in
`PICKUP.md` §5 and the BUILD_ORDER boards. What lives here is the
honest debt the witnesses have not yet measured:

1. ~~The interleaving stress witness~~ **struck 2026-07-30** —
   measured in `world_interleave.rs` (coder-02): the identical-remainder
   pair distinguishes by the survivor's `opened` stamp mid-flight and
   replay removes the oldest match, matching live; the one-stamp pileup
   (lapse → bond expiry → covenant slash in one tick prologue, fixed
   order) replays and resumes identical. The engine held; both first
   expectations were the witness author's (a tick off-by-one, an
   overpriced collateral) — the §10 habit, exercised.
2. **The state.json `cohesion` block content pin** ~~in `mirror_pins`
   (coder-01, relay)~~: the block is emitted mechanically and the
   workspace compiles green, but nothing asserts its bytes. **The named
   witness file no longer exists** — nothing matching `mirror` survives
   anywhere in the tree (renamed 2026-07-31). Whoever picks this up must
   first find where the pin should now live (`engine/lens`), which is
   itself the debt: a gap recorded against a path outlives the path.
3. **Journal follow-on (Q4.6, relay):** the Chronicle Feed reads the
   cohesion books and friction receipts mirror-side under its
   reflection law.

4. **`covered()` is not total** (2026-08-04): `BookPosted` is a live,
   value-moving deed the kernel does not judge, and
   `kernel/tests/taxon_pins.rs` cannot see it because it enumerates a
   hand list of 64 against an enum of 71. The Phase 3 flip's
   precondition is therefore unmet, and the mint escrow cannot be
   generalised until it closes. Full record in Pickup §0.
5. **`tools/` has no test harness** (2026-08-04): the observer's
   intelligence fix is pinned by measurement (705 receipts on the live
   board) rather than by a test, because no Python witness runner
   exists. Standing one up is a lane decision nobody holds.

Strike entries here when their witnesses land; add new ones when a
claim ships proven-by-argument.

**A habit this refresh earned:** three of the five entries above went
stale by naming a path or a count that the tree later moved. When
recording a gap, anchor it to a symbol or a law rather than a line
number where possible — line numbers go stale silently and take the finding with
them.

## 10. Session habits that kept this clean

- Read whole files before editing them; the other node's changes land
  on disk mid-task and edits fail or mislead on stale context.
- Prefer receipt-anchored asserts in witnesses over hand-computed bank
  totals — parameters retune (flat demurrage changed numbers once).
- When a witness fails, decide honestly whether the expectation or the
  engine is wrong before touching either (one witness typo was the
  author's: 1/2 + 1/2 asserted as 2).
- Never edit `docs/05/06/01` to point at newer docs when a one-way
  reference from the new doc suffices — minimizes collision surface.
- The commit message is the landing record's twin: what landed, the
  decisions recorded, the relays flagged, the evidence.
