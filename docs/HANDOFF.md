# HANDOFF — the pickup document

**Status:** PROCESS CARRIER (beside `BUILD_ORDER.md` and
`OPEN_QUESTIONS.md`), for any engineering node joining or resuming
work. **State is never read here:** the current landing and the landed
ledger live in `BUILD_ORDER.md`, the ONE carrier. This document
carries the working knowledge around that state — the discipline, the
coordination protocol as lived, and the earned gotchas. Refresh the
dated §9 at each major handoff; everything else changes only when a
law or habit changes.

## 1. Reading order (before any change)

1. `docs/00_CHARTER.md` — every law binds; §8 terminology, §9
   stochastic, §10 the parallel-node protocol.
2. `docs/BUILD_ORDER.md` — the landed ledger + the boards. What to do
   next is decided HERE, on operator go.
3. `docs/OPEN_QUESTIONS.md` — rejected ideas (never silently revive)
   and recorded deferrals with their reopen triggers (Q4 = the
   ideology board's).
4. `docs/03_SUBSTRATE_CONTRACT.md` — mechanic → primitive → witness
   (coder-01 authority).
5. The design doc for whatever you touch (05 thermal, 09 ideology, 10
   tech, …) and `protocols/NS-1_DOP.md` §2.3 (the permanent tag
   registry).

## 2. The landing discipline (how every mechanic ships)

- **One landing at a time. Spec + mechanic + test vectors in the same
  landing (LAW XVII). Commit ONLY on explicit operator go.**
- **Red-first with recorded output:** write the witnesses in
  `engine/cfe/tests/<name>.rs` FIRST, run them, record the red (compile
  errors count), then implement to green.
- **Evidence blocks verbatim** in the BUILD_ORDER landed entry: clippy
  zero (`cargo clippy --workspace --all-targets -- -D warnings`), the
  witness count (N/N workspace witnesses green — sum every suite), the
  named exact measurements, and the CLI arc played end-to-end
  (`printf '…' | cargo run -p netstratum-cli -- --plain`).
- Every witness file ends its arcs with
  `assert_eq!(replay(world.chronicle()), world.committed())` — replay
  MATCHES is the SEAL B connectivity proof and it has caught real bugs.
- Refusals are typed, name the cure in the CLI, and are computed PURE
  before any mutation — a refusal orphans nothing (no receipts, no
  bank moves).
- Fractions render whole ("3/2", never 1.5). Placeholder parameters
  live in `dop.rs` as named fns beside the types they rule.

## 3. The parallel-node protocol (who touches what)

- **coder-02 (main tree):** `docs/` (except 03), `engine/cfe/` in
  practice (world.rs named in the charter; dop.rs/tests by usage),
  `clients/cli/`.
- **coder-01 (worktree):** `engine/rfa-core`, `engine/physics`,
  `engine/solver`, `engine/mirror`, `engine/conductor` (by usage),
  `docs/03`, workspace `Cargo.toml`, `tools/observer.py`.
- Cross-territory happens ONLY as a mechanical relay (the P1
  precedent): e.g. adding a renderer arm the compiler demands. Flag it
  in the landing entry and the commit message.
- **The other node commits mid-session.** Before EVERY commit: `git
  status` + `git log --oneline -5`; re-read shared append-points
  (BUILD_ORDER, the dop.rs registry header, NS-1 §2.3) before editing
  them. Doc numbers and tags are claimed by git history order — the
  earlier claim wins (the 08 collision was resolved by renumbering to
  09).

## 4. The tag registry (permanent — one tag, one meaning, forever)

Authoritative header: top of `engine/cfe/src/dop.rs`. Mirrors: NS-1
§2.3 and the BUILD_ORDER boards. Update all three at landing, never
before. As of 2026-07-27: events 1–7, 10–13, 21–25, 27, 29–38 and orb
entries 16–19, 26, 28, 41 are spent; 8–9 reserved (D2 terrain, D3
milestones); 20 receipt frame; **39–40 reserved (TECH-1)**; **next
free: 42**. Contiguity is not law.

## 5. The state model (get this right or replay breaks)

- **Committed vs workspace:** what has no receipt does not persist.
  Banked state never mutates silently. Accruals (yield, vent, friction,
  bond spans, demurrage) are WORKSPACE and settle by ONE receipt at a
  touch — the VentSettled shape. `World` workspace fields: `accrued`,
  `vent_pending`, `friction_pending`. Committed families beyond orbs:
  `debts` (ABSORB-1), `bonds` (BOND-1), `covenants` (COVENANT-1) — all
  three live in `Committed`, are cloned by `committed()`, rebuilt by
  `replay()`, and carried by `resume()`. **A new committed family must
  join all four places.**
- **The touch law:** `demurrage_due` computes PURE; affordability is
  judged post-decay; `touch()` applies, receipts, and anchors
  `last_touch` unconditionally — every settle is a touch (including
  `UpkeepSettled` on replay; that divergence was a real caught bug).
- **Flat-rate decay only** (the denominator-explosion ruling, commit
  7635eb7): decay = rate × elapsed capped at the store, NEVER
  proportional-compounding (banked × (31/32)^n grew denominators to
  ~2^1100 live).
- **Replay-fact discipline:** the live path and the `replay()` arm must
  make IDENTICAL reads in IDENTICAL order (crisis flags before bank
  moves, etc.). Shared helpers used verbatim by both paths:
  `settle_debts`, `credit_covenants`, `touch`. If a live event needs
  state replay can't derive, put the fact IN the receipt (the covenant
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
- **Empty-book readings (placeholder rulings):** reciprocity reads 1
  (fresh orbs trade frictionless); `cohesion_ratio` reads None ("no
  deeds yet"); the collateral curve reads full quota (the empty book is
  not a credential).

## 6. The cohesion ledger (the ideology board, LANDED whole)

Eight books on `ColonyOrb`, all committed, replay-rebuilt, wire tag 41
(emitted only when nonzero — zero-book orbs stay byte-identical to
pre-ideology ones):

| axis | syntropic | entropic | posts when |
|---|---|---|---|
| compassion | `given_in_crisis` | `drawn_in_crisis` | transfer/absorb while the counterparty is famine- or overheat-struck, uncleared, at the deed's stamp |
| love | `bond_flow` | `severance_load` | bond settles only (crisis books belong to deliberate deeds); load = rate × notice on a cut or default, zero on term expiry |
| integrity | `covenants_fulfilled` | `covenants_breached` | covenant release/slash, VALUE-weighted by collateral stake |
| morality | `returned_across` | `extracted_across` | reciprocity-window payments / lapses (windows open at absorption) |

One deed may post several axes (a transfer can post compassion, pay
absorption debts, credit a covenant, and accrue friction). Ratios
DERIVE at read (`dop::cohesion_ratio`), never stored. Friction taxes
reciprocity DIVERGENCE on transfers only — bonds and absorptions are
exempt by design (the standing loop is the friction-free channel). The
engine judges nothing; it measures.

## 7. Build, test, play

```
cargo test --workspace                                  # 158/158 green at handoff
cargo clippy --workspace --all-targets -- -D warnings   # must finish clean
cargo test -p cfe --test world_<name>                   # one witness file
printf 'found meridian\n…\nquit\n' | cargo run -p netstratum-cli -- --plain
```

The conductor's converge/egregore suites drive real archives and take
~3–4 minutes; the rest is instant. `var/` is runtime state, never
tracked, never a source of truth.

## 8. Compiler-enforced connectivity (expected breakage)

Adding a `ClosureEvent` variant intentionally breaks three exhaustive
matches until you add arms: `world.rs::replay`, the CLI chronicle
renderer (`clients/cli/src/main.rs`), and the mirror's
`render_receipt` (`engine/mirror/src/lib.rs` — coder-01 territory,
mechanical relay). `engine/mirror/src/converge.rs` and `journal.rs`
have catch-alls. Adding an `ActionRefused` variant breaks the CLI
`refuse()` match — the new arm must name the cure.

## 9. Where things stand and what's next (dated: 2026-07-27)

Landed this arc (see BUILD_ORDER §1b for the evidence blocks): the
full ideology board — TRADE-1 (transfers, compassion), ABSORB-1
(absorption + reciprocity windows, morality), BOND-1 (standing bonds,
love), COVENANT-1 (vitrified escrow + fidelity curve, integrity),
IDEO-1 (wire entry 41, observer uplink, friction 38, CLI `ideology`).
Design authority: `docs/09_IDEOLOGY.md` (status LANDED); deferrals:
OPEN_QUESTIONS Q4. Also landed by the coder node in the same span:
BOT-THERM (thermal genesis for the egregores), JOURNAL-1 (the
chronicle feed), the flat-demurrage and mirror-rotation fixes, and the
egregore/automaton vocabulary (ce5cd64 — "bot" is retired).

Candidate next work (the operator picks; nothing starts without go):

1. **Verification gaps named at handoff:** (a) an interleaving stress
   witness — two identical-remainder absorption debts on one lane
   lapsing at different ticks (the replay removal is first-match by
   pair+remainder; order-consistency is argued, not measured), and
   absorb + bond settle + covenant resolution on one lane at one
   stamp; coder-02 territory, red-first-able. (b) a `mirror_pins`
   assertion for the state.json `cohesion` block content — coder-01,
   relay.
2. **JOURNAL follow-on (Q4.6, relay):** the Chronicle Feed reads the
   cohesion books and friction receipts mirror-side.
3. **TECH-1** (`docs/10_TECH_EMERGENCE.md`, tags 39–40): carries the
   SINK-1 research/throttle gates.
4. **BOT-THERM phase 2:** trade execution against the wants the wall
   shows (boarded, not landed) — the egregores' automatons could now
   use `send`/`absorb`/`bond` and grow real cohesion books.
5. **Depth board D2/D3** (terrain tag 8, spoilage+milestones tag 9)
   and the protocol board N4 (scanner) onward.
6. **Q4 deferrals** when balance demands them (commensuration table,
   recency windows, covenant templates, target-side severance).

## 10. Session habits that kept this clean

- Read whole files before editing them; the other node's changes land
  on disk mid-task and edits fail or mislead on stale context.
- Prefer receipt-anchored asserts in witnesses over hand-computed bank
  totals — parameters retune (flat demurrage changed numbers once).
- When a witness fails, decide honestly whether the expectation or the
  engine is wrong before touching either (one witness typo was the
  author's: 1/2 + 1/2 asserted as 2).
- Never edit `docs/05/06/01` to point AT newer docs when a one-way
  reference from the new doc suffices — minimizes collision surface.
- The commit message is the landing record's twin: what landed, the
  rulings recorded, the relays flagged, the evidence.
