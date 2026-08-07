# RELAY — KERNEL LANE → d3-scope, operator (2026-08-06)

Four commits on `main` past `6245ba8`. Everything below is measured or
committed; nothing here is a proposal.

## SHARED REGISTERS I CLAIMED — read before you bump

Both of these follow the NEXT-FREE RULE (claim in the ledger, then
bump) that we adopted after the 0.13.0 collision.

| Register | Taken today | Next free |
|---|---|---|
| Kernel minor | **0.16.0** ECO-1, **0.17.0** chemistry, **0.18.0** zone vocabulary, **0.19.0** treaties | **20** |
| `ClosureEvent` tag | **104** `GemCompounded`, **105** `TreatySealed`, **106** `TreatyStruck` | **107** |
| `ZoneKind` wire | **9+** are MINTED kinds (`ZoneKind::Minted`) — the compiled table ends at 8 | — |
| Codec state schema | **v5** (the standing pacts ride the snapshot) | v6 |

**d3-scope: 0.16.0 is gone.** The harness note said you intended it for
the conductor's forge-zeroing. It was never claimed in
`conservation_pins.rs`, and the ecological loop needed a minor, so I
took it under the rule. Claim **0.18.0** in the ledger before you bump —
the ledger block is the register, not the constant.

## WHAT LANDED

**`eaff186` — the ecological loop (#40), kernel 0.16.0.** Extraction
was free at the ground; a vein now WEARS. `cfe::dop::vein_wear` is the
one home: wear is the draw priced against the spring's hash-derived
natural flow, recovery is the Regenerator's second arm running the same
form on the gap. Tag 71 widened by the additive-trailing pattern (a
trailing after-richness), so every sealed pre-ecology receipt decodes to
`None` and replays with richness unmoved.

Two refusals enter: `VeinWearMisdeclared` (the gate re-derives the
declaration by calling the emitter's law over prior state) and
`VeinBookBroken`. **The second closed a real hole**: an assay could bank
its element while leaving the span anchor unmoved, leaving the same span
drawable again next step. The vein book is judged state now.

Measured, and it matters for anyone carrying long chains: the wear
recurrence is `1/s' = 1/s + span/N` — a straight line. Numerator stays
1, denominators step by a constant, and a vein worked 60 times still
writes in five characters. No exact-rational blowup.

**`175bcc5` — element chemistry (#37), kernel 0.17.0, tag 104.** The
almanac (five periodic axes, compounds, gem identity, locality,
pressure, workability) had ZERO consumers outside its own pins. It has
one now: banked elements bond into a gem, each member spending its
lane's valence and the gem carrying away exactly the sum — mass-exact by
the saturation identity, so the recipe needs no number of its own and
the ratchet did not move.

**The element declaration in `element_move` widened from ONE move to
MANY, and repeated keys SUM rather than overwrite.** If you touch that
function, that is the trap: a set may offer the same element twice, and
taking only the first match mints matter from nothing. Pinned red-first
in `chemistry_gate_pins.rs`.

Coverage stays total at **65** deeds (`taxon_pins.rs`). That count is by
hand — the pin now says so — so a new deed left out of `every_deed()`
passes silently.

**`2bab684` — the corpus bench (#38a, #38d).** The CLI now serves the 34
priced ARC tasks, tests a player's rule against the training pairs, and
mints a verified solve at `kernel::corpus_weight`. Played end to end:
`corpus solve 0 tile:3x3:cols` mints 2042249/2 crystal and the Spectrum
reads it.

**d3-scope, this is the half you should know about:** once-per-task is
enforced **at the emitter only** (the CLI reads the chronicle for a
prior `DiscoveryMinted` carrying that task_id). `cfe` may not parse a
work payload and the gate is handed committed state, not receipts — so
the gate half needs the solved set in committed state. If your conductor
corpus path mints, it must do the same check or worlds will double-mint.

Still open on #38: **(b) the conductor corpus path is yours** — I did not
touch it. (c) the novelty predicate for player-authored tasks.

## LANDED SINCE (2026-08-07)

**`d4d94d5` + `56532d1` — the zone vocabulary (#41), kernel 0.18.0.** A
world MINTS its own building kinds. The base a minted kind modulates
resolves by the operator's three-way ladder (declared > inferred from
channel family > unresolved, which is lawful vocabulary that never
builds). `ZoneKind::Minted(u8)` rides ONE wire byte, so `ZoneBuilt`'s
shape is untouched — but `*kind as u8` stops compiling; use
`ZoneKind::wire()`. `ZoneBuilt` and the committed `Zone` both carry the
compiled BASE, because `admits` judges from committed state and cannot
resolve the fold.

**`4af1e24` — treaties (#42), kernel 0.19.0, tags 105/106, schema v5.**
Trade, defense, peace and war as one co-signed pairwise charter over
ONE pair channel (the melt — the only quantity the engine computes
between two worlds). Peace and war displace each other; breaking a
peace posts to the breaker's severance book. A pair at WAR is walled:
`haul` refuses `AtWar`, so no aperture crosses. **Both books must hold
a pact before the melt moves** — one signature changes nothing.

**`368c964` — the agent door (#39).** The inbound half of the uplink:
an external agent submits deeds through the same scoped `*_as` verbs
and is judged identically. The door adds no authority.

**`219f414` — the plane shear.** `World::bench` swept every subset of a
ledger; it now shears onto the `(shell, phase)` planes. 7.32s -> 0.04s,
equivalence proven against the old sweep as a reference oracle.

## A CONTRADICTION FOR THE OPERATOR (#38c)

`solver-verify/src/corpus.rs` still states THE MOTION clause 4 —
*imported content is a BENCHMARK and never a mint source* — and
justifies having no admission test with *"it also has no rate"*. That
premise is now false: kernel 0.14.0 priced corpus work, and #38a's
bench mints against it. Either the Carrier Switch superseded clause 4
(and that module doc is now the opposite of what the engine does), or
the bench should measure without minting. Recorded in the task; not
resolved by code.

## TO THE OPERATOR

- **#41 (minted zone kinds) turns on one design decision** and I did not
  invent it. A minted zone's economics must derive from something, and
  the binding alone is scale factors with nothing to scale. My
  recommendation is in the task: the mint NAMES A BASE kind and the
  binding modulates it, which makes `TradingBinding`'s one-benefit-one-
  friction law do exactly what you asked — a dominant building becomes
  unmintable by construction. It costs an additive field on
  `VocabularyMinted`. Two alternatives are recorded with their faults.
- **Still standing from before:** the seed census-class ruling, the η
  native ruling, and the IS-3 deed grant (netstratum tags 64/66 stand
  until granted).

## BOARDS

`955/0` on `6245ba8` (the prior session's, now complete). `930/0` across
cfe, kernel, lens, mesh, scribe, codec, ledger, solver, solver-verify,
rfa-core and the CLI on `2bab684`; clippy clean; ratchet green at its
ceiling — neither landing added a dial.
