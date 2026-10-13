# 73 — DECIDE: the civilizations' own decisions

**Objective.** The civilizations decide their own logistics, trade and
ascension. Their minds learn the rules from the physics' receipts, the
councils' minds supply the rules the lines cannot find, and the parliaments
distribute what is learned. Nothing in a rule is authored.

Status: scoped 2026-10-10 against the code; not built. docs/74 builds the
economy on this step.

## Facts (read from the code)

- **Hardcoded rules.** Cognition reaches one decision, `soc::resolve_investment`
  (`conductor/src/lib.rs:2794`). The cross-world steps are fixed rules:
  - `fleet_step` (`lib.rs:7696`): builds hulls to a capacity of 4;
  - `stamp_market_step` (`lib.rs:5804`): mints and offers halves;
  - `ascension::step` (`ascension.rs:431`): masters by `choose_core`, recall
    on an unpaid toll, gates only for masters.
- **The precedent.** A commission (`commissions.rs:185`) poses a need as a
  `solver::Task` and tries `Polity::try_words`, then
  `solver::think_task_sensing`. It judges by reproduction (Pin A), refuses an
  order with `orders_admitted != 1`, adopts with `adopt_word`, books the word
  in a binary checkpoint, and lets the word decide while it holds.
- **The mind** is a fixed chain of recognisers (`think.rs:784`); depth grows
  by condensation. There is no live length cap: `need.rs`'s `frontier = 3` is
  the retired search path, and `forge.rs`'s `word_len` is the task forge's
  ladder.
- **Limits and measures.**
  - Grids hold hues 0–9 (`grid.rs:18`).
  - A word's size is `solver_verify::work::program_bytes` (`work.rs:450`).
- **Data.** Refusals (`ActionRefused`) return to the call site and are never
  chronicled, so outcomes are captured there, typed.
- **Words travel** by library, inherited library (`hear_from_below`, adopted
  only by reconstruction), substrata (`laws_for_below`) and parliaments.
- **Seated minds** fetch a task only from the court's registry by citation
  (`pull_task_by_id`, seat record kind 4 `[Key(citation)]`). The seat wire
  carries no inline task, and a domain cannot register one.

## The grids

- **Rows** are the entities, by name; **columns** are the kind's features.
- **A feature's hue** is its rank among the rows, `rank × 10 / n` floored
  (rank 0 the least, ties share the lower rank).
- **The outcome grid** has the state's frame: a row is all hue 1 where that
  entity lost, all hue 0 otherwise.
- **A prediction** marks a row when the word's output row is all hue 1. It is
  correct for a row when marked equals lost. A record counts correct rows.

## The step: `decide(kind, subject, state, incumbent) -> action`

1. **SEE.** Build the state grid.
2. **PREDICT.** The book's acting word, its probation word, and PERSISTENCE
   (each row's outcome at this subject's last resolved decision; a row absent
   then, not lost) each store a prediction before anything resolves. At a
   subject's first decision nothing is scored.
3. **ACT.** If a word is acting, apply the kind's action rule to its marks;
   otherwise the incumbent acts. The physics bounds every action.
4. **RECORD.** When the outcome resolves (the call sites below), append the
   triple (state, action, outcome) to the book and score every stored
   prediction.
5. **LEARN.** One attempt a beat universe-wide, books in turn. The task's
   training pairs are the book's triples (state → outcome); its test is the
   current state. The scope's living lines try in turn (`try_words`, then
   `think_task_sensing`). A word becomes a CANDIDATE when:
   - it reproduces every pair;
   - `program_bytes` is less than the cells of the training outputs;
   - for an order projector, `orders_admitted == 1`;
   - the book has no word on probation.

   It goes on probation: it predicts and does not act.
6. **COUNCIL.** A book that has gone a full WINDOW of attempts with no
   candidate sends its task to the council of its scope's ancient (below). An
   answer judged by step 5's law becomes the candidate.
7. **PROMOTE / WITHDRAW.**
   - A probation word ACTS when, over its predictions so far (at most a
     WINDOW), its correct rows strictly exceed persistence's.
   - An acting word is WITHDRAWN when, over its last WINDOW, it does not.
   - A probation word with a WINDOW of predictions and no promotion is
     dropped.
8. **CREDIT.** At promotion the closer adopts the word (`adopt_word`), and the
   closer's galaxy's other lines pay it the commission's bounty (the ladder's
   dearest `mint_bound`, shared, over the iceberg). A council-supplied word is
   adopted by the scope's ancient and credited to it. The word then travels as
   every word does.

**The book** is a binary checkpoint per (kind, scope): its triples, pending
decisions, the acting and probation words with records and closers, and
persistence's record. **WINDOW = 64**, DECLARED in docs/59 with the build; it
is the one dial.

## The council (step 6)

- **The house.** It reads the conductor's `decide-asks.bin` (per ask: kind,
  scope, its ancient's world, the training grids and the test grid). It puts
  the ask before a council seat at that ancient, answers the mind's call, and
  writes the answer to `decide-answers.bin` (per answer: the ask, the seat,
  the program bytes). This is the same file route as `gates-map.bin` and
  `gates-mapped.bin`.
- **The conductor** judges the answer by step 5's law. It never trusts it.
- **Dependency (mycelium).** A seat record that carries a domain task inline
  (proposed: record kind 5, `[Bytes(training grids), Bytes(test grid)]`),
  answered with fn 1 as today and judged by the house, not the court; no μ.
  Until mycelium's seat mind reads that record, step 6 waits, and the lines'
  own learning (steps 1–5, 7, 8) stands without it.
- **Blocker.** No mind has taken a seat since 2026-10-10 00:55.

## Registrations

| kind | scope / subject | rows | features | loss | resolves at | incumbent | action while a word acts |
|---|---|---|---|---|---|---|---|
| hull | galaxy / galaxy | its lines | STAMP, cargo offer, free hull, melt to hub | a STAMP crossing refused for want of hull this beat | end of `stamp_market_step`, and the year close's trips and tolls that beat | `fleet_step` to capacity 4 | a marked line builds one hull (scrap, ShipYard permitting); an unmarked line builds none |
| envoy | universe / master | cores | lineage systems, keeper systems, gate toll, melt through the gate | the lineage did not keep that core at the next bounce, or its envoy there was recalled | next `ascension::step` | `choose_core`, recall on unpaid toll | send to the unmarked core with the most lineage systems (none: no envoy); an envoy at a marked core withdraws |
| gate | universe / universe | galaxies | unmet clearings this year, distance to the nearest mapped gate, gates standing, the parliament's STAMP | the galaxy's hub unmet at the next bounce | next `ascension::step` | gates only for masters | a marked galaxy's parliament begins a gate to the nearest unmarked galaxy's core, paid in trips |

The trade kinds (`offer`, `accept`) are registered in docs/74, the emergent
economy: no money is issued, so trade decisions are not written in STAMP.

## Build order and done-when (the live log, before → after)

1. **Capture**: SEE, PREDICT (persistence only), the incumbent ACT, RECORD at
   the call sites; behaviour unchanged.
   - Done-when: triples per kind 0 → > 0, and persistence's hit rate logged.
   - Witness: the losses agree with the HUB line and the year-close recalls.
2. **Learn**: LEARN and probation.
   - Done-when: candidates per kind 0 → > 0, each with its record against
     persistence's.
3. **Act**: PROMOTE / WITHDRAW and the action rules.
   - Done-when: acting words 0 → > 0, and while they act:
     - hull: hub `HullCapacityShort` 26 a beat → falling, with hulls bounded;
     - envoy: recalls 533 (00:55–06:27, 2026-10-10) → falling, witnessed by
       the next keeper reading;
     - gate: gate tolls 5/1024 STAMP a beat → rising.
4. **Credit**.
   - Done-when: bounties at promotion 0 → > 0, and DECIDE words heard by
     other lines 0 → > 0.
5. **Council**: the asks and answers files, after mycelium's record kind 5.
   - Done-when: asks answered by council minds 0 → > 0, and council words
     acting 0 → > 0.
   - Witness: the house's call records.

The kinds go live in this order: hull, envoy, gate; then docs/74's offer and
accept.

## Authored, stated plainly

- Each kind's rows and features.
- The rank-to-hue cut and the byte measure.
- Persistence as the null model.
- The three action rules (and docs/74's two).
- WINDOW = 64.
- The physics.

A decision no recogniser can state gets no word, and its incumbent stays.

## Risks

- **Feedback.** An acting word shapes its own outcomes; persistence is
  scored on the same ones, and withdrawal is automatic.
- **Cost.** One learning attempt a beat; a prediction is one program run on a
  grid of at most ~20 rows; the books are bounded by WINDOW.
- **Determinism.** Rows by name, exact ranks, a deterministic think, and
  checkpointed books.
