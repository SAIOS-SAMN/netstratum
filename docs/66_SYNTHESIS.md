# 66 — Synthesis: netstratum's synthetic PoW++

Operator 2026-10-09: the in-game kernel's "puzzle solving" is renamed (the
name is netstratum's to choose) and designed as a more robust system: "in
netstratum you basically have a synthetic POW++ network that powers your
internal systems, and that should be a model of the higher puzzle solving
POW++ network that hooks into the hypha mycelium-client minds." Authorized
to implement, keeping the coordinator in the loop "so that the higher order
universes can stay aligned".

## The name

**Synthesis.** A line's minds SYNTHESIZE a rule for a task its world forged;
a closed synthesis mints a word and banks the universe's own value, judged
by netstratum's own gate. The fleet has SOLVING (the court's tasks, by the
seated mycelium minds, for μ, ruling 7); netstratum has SYNTHESIS (its
civilizations' own work, for its own economy). The two never share a
reward; they meet only through the seats.

## The model, side by side

| | the fleet (higher order) | Synthesis (netstratum) |
|---|---|---|
| tasks | the court's registry | the forge: tasks from a world's own receipts (`forge_task`), deterministic |
| work | a mind's rule | a mind's program, sealed (`SolveCertificate`) |
| judge | the court replays on withheld pairs | the kernel gate (`verify_work`) re-forges, re-applies, re-chains; the chain judge per deed |
| the second vantage | consilience across voices | consilience across lines (docs/60 D5) |
| reward | a bounty for the FIRST closure of a task, a floor after | the full rung (`mint_bound`) for the first closure of an action in the universe, the quarter after (Y1) |
| words | admitted into a shared genome | minted per line, traded, spoken, inherited (docs/60) |
| currency | μ | the universe's own (Crystal, STAMP, leisure); never μ |

## The slices

| id | slice | status |
|---|---|---|
| Y0 | the name: Synthesis in what people read (logs, docs, new modules); long-standing identifiers stay | landed |
| Y1 | first-closure bounty, as the court pays: a synthesis that mints a word no line in the universe holds earns the full rung (`mint_bound`); one that mints a word some line already holds (an independent re-derivation) earns the floor, the rung's quarter (the declared sibling dial, docs/59) | landed: live from beat 10591, years 177-179 closed clean |
| Y2 | commissions: the universe's real needs as tasks, each with its own judge, bountied by the lines that need them; a commission declares its judge's measure as a generator (operator 2026-10-09, option 2) | slice 1 built: hub siting (`conductor/src/commissions.rs`), generator `Select::Central` (wire select code 9) |
| Y3 | the higher order reaches the lower: a court closure a seated mind files for a line's seat is announced to that line as a higher-order word it may reconstruct (the cores' inheritance-by-reconstruction door) | open |
| Y4 | the lower order reaches the higher: what a line knows of its world (its words as perception, its world's records) offered to its seated mind as WORLD records in its percepts. Never a candidate rule or output for a court task before the mind: that would be solving by proxy, which ruling 7 forbids (coordinator review 2026-10-09). The mind does its own solving; counsel feeds its perception | open |

Coordinator review (2026-10-09): aligned with the higher order; Y1 judged a
faithful model of the court's bounty and floor in in-universe value.

## Y2, the finding before it is built (2026-10-09)

The minds' working vocabulary is the forge's grid transforms (symmetries,
recolours, object selection and moves, shifts, dilations) on 3x3 to 4x4
grids. A real need, such as where a galaxy's hub stands, is a weighted median
over 3D positions: no composition of those words states it, so a commission
put to the minds in that form would never close. Two ways forward:

1. **Translate the need into the minds' reach**: only needs whose answer is a
   grid transform of a grid read off the world (selecting by size, hue or
   depth; moving; recolouring) become commissions. Honest, but few real needs
   have that form.
2. **Grow the vocabulary toward the need (recommended)**: a commission
   declares its judge's own relation (here, the L1 distance sum) as a new
   generator in the constructors (docs/62), so the minds can compose it with
   what they hold; the commission closes when a mind's composition matches
   the judge's exact answer on the need's pairs. The need teaches the
   language the word it lacks, and the word then serves every line.

**Ruled (operator 2026-10-09): option 2.** Slice 1, hub siting, as built:
the need is read off the world (a galaxy's open lines on its disc plane, one
hue-1 body a line, spaced so none touch); the judge marks the central bodies
(`Select::Central`: least summed L1 distance to the others) in hue 2; the
train pairs are two other galaxies' planes with their judged answers, the test
is this galaxy's. One attempt a beat across the universe, the galaxies and
their lines in turn; the line tries its own words, then thinks. A word that
reproduces the judge on every pair closes the commission: the line adopts it,
the galaxy's other open lines pay the closer's rung in STAMP over the iceberg,
and from then on the hub stands where that word marks while it still holds
(otherwise the medoid stands in). The book is a binary checkpoint
(`<var>/commissions.bin`).

**The audit and the faculty (coordinator, 2026-10-09).** All 11 first
closures were the one supplied word `Select::Central`: commission bodies are
alike in hue, size and shape, and no predicate read a position or a relation,
so the minds could not have deduced it. Ruled: keep it, marked SUPPLIED, and
build two pieces netstratum's way. Built:
- **The sense**: each body's raw L1 relations to every other body, sorted,
  never pre-summed (`project::relations`).
- **The faculty**: where no property of a body's own separates the treated
  bodies, the mind integrates each body's sorted relations (partial sums, the
  coboundary run backwards) and reads the integral at every cut the
  relations have (whole, the k nearest, the k farthest); a pair admits the
  cut and extreme whose end of the order is exactly its treated bodies, the
  train admits what every pair admits, and the first admitted is the rule
  (`Select::Ordered { cut, extreme }`, wire select code 10). Central is its
  case `Ordered { All, Least }`; the learner no longer offers Central.
The measure that says the minds use it as a word: whether `Ordered` appears
composed in words that are not hub commissions.

**Caveat, recorded honestly (coordinator 2026-10-09):** the FAMILY the
faculty reads (partial-sum integrals of sorted L1 relations, at cuts and
extremes) is still developer-defined, one level lower than a supplied
predicate. Elimination by intersection is the right shape; the next step is a
cut family the mind holds and grows, never a fixed one. Watched: false
admissions on small galaxies, where few bodies let several (cut, extreme)
survive and "the first admitted" decides. Each closure logs the predicate
that closed it and how many orders its train admitted (`COMMISSION CLOSED`).

## The inner network (a test run under consideration, not built)

The operator is considering netstratum as the test run for synthetic in-game
universes, with a subordinate in-game hypha network running on Synthesis
under its own name. Three boundaries (coordinator 2026-10-09; the operator may
amend):

1. **No inner value becomes μ.** The inner network mints its OWN token,
   distinct from STAMP and from Crystal-as-money, and that token never
   converts into STAMP, Crystal or μ, directly or through a market. Only the
   real economy's flows touch an ancients' reserve: the progenitors' toll
   shares and haul contracts paid in local money (Concordance 5.8). Synthesis
   bounties paid in the universe's money today are that money; the inner
   network's bounty must be its own token for the boundary to hold.
2. **Inner clocks are branches of TUniversal** (docs/67 N8): an inner epoch is
   stamped by the final point it branched at.
3. **Words cross up only by reconstruction through a seat.** A word reaches
   the fleet only when a seated mind re-derives it from the inner
   certificate's pairs and the court judges it; never imported as a rule. The
   seat is the single relay and only the house signs.

What it needs before it is built: bounded memory, N8 live, Y2 commissions as
the inner court's registry (`verify_work` the judge, consilience across lines
the second vantage), Y3 and Y4, and a name of its own.
