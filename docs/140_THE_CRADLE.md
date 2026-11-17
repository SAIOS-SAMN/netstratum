# 140 — The cradle: the progenitor's machine for foundings

**Operator, 2026-11-17:** "instead of working on a single founding: should
we produce the machinery one level up at the foundry level that can produce
foundations or modify live foundations — from a progenitor's perspective,
not a sovereign's." Named **the cradle**. "Foundry" and "crucible" are
taken in this repository (docs/108 O7, docs/30 T14), and "uplink" and
"hypha" are other nodes' names.

**Status:** derived 2026-11-17. Not built. Tracked as #246, milestone
v0.1.0.

## 0. What the record holds

- **The progenitors are tier 0, the Board-Makers** (docs/12 §2, §2a). "The
  conductor's own operations are their deeds, named": seeding, expanding the
  board, contracting it (the fold), and creating spaces, stars, worlds and
  galaxies. Every deed is "receipted, conservation-lawful, none an edit".
- **A sovereign** holds and governs chartered worlds from inside them
  (docs/12 §3). The cradle is the progenitor's instrument: it makes and
  extends the board, and governs nothing on it.
- **Today the progenitor's work is loose tools:**

| Tool | What it does | Its deed |
|---|---|---|
| `pulse <dir> found 2^n` (`pulse::founding`, `deploy`, `orient`, `write`) | a universe from a count, the table rebuilt each time (~15 min at 2^40, #233) | found |
| `conductor --var <dir> --cycles 1` | era 0's universe ledger and the Big Bang receipt | found |
| `ns-embassy refound` | the domain, origin, reserve, manifest and ledger on the court | found, its network half |
| keys minted by hand, then the above (the trial, #240) | a test domain | found |
| `ns-embassy apply`, growth manifests | a grant applied to a live universe | extend |
| an inbound crossing (#109, not built) | matter arriving at a throat | extend |
| `conductor --fold` (`conductor::fold`) | the era sealed | seal |

## 1. Axioms

- **C1 (one machine, three deeds).** Every progenitor act on a universe is
  one of three deeds: **found**, **extend** or **seal**. Each is receipted,
  conserves what it touches, and edits nothing.
- **C2 (a founding is a function of its place).** A founding is
  `found(count, place, seed)`:
  - the table the climb makes at that place (docs/97 §14–§15, E79), with
    the place's held share g;
  - deployed;
  - oriented by the seed.

  The universal core's founding is the case `place = the throat, g = ½`.
  Every singularity founds its own table at its own bounce (V6; Xylarium's
  "tables per singularity", S138), so a galactic core's founding is the
  same deed one tier down.
- **C3 (extension is a deed on a live universe).** A live universe receives
  matter only by a deed that names the matter, its place and its source:
  - a growth grant;
  - an inbound crossing, its whole state carried (docs/123 §3b);
  - a core's own bounce.

  The universe's books (nucleons, 3Z, μ on its nuclei) are conserved
  across the deed. The universe is reconstructed after it from the deed's
  receipt, never replayed.
- **C4 (the network half is the embassy's).** What crosses to the court
  (genesis, origin, reserve, manifest, ledger, apply, mouth, traverse) is
  sent through `ns-embassy`, signed by the domain's tiers. The cradle
  decides; the embassy speaks.

## 2. Derived

- **D1 (the deployment is held, not rebuilt).** `found` is a function of
  `(count, place)` up to the seed's orientation. The table is cheap; the
  cost is `deploy`, which reads only the table. So the deployed, un-oriented
  universe is computed once per `(count, g)` and held (built in #248,
  `cradle::found`). It is used only where its total is the table
  (conservation), and each founding orients it by its own seed. Re-founding,
  test domains and galactic cores at the same `(count, g)` reuse it. That
  answers #233's repetition. The filter's all-pairs cost (#233) is paid
  once per table, and is derived separately.
- **D2 (extension is a delta).** An extend deed changes the throat's mix
  by its parcel. The release, the capture phases and the flows read the new
  mix from that bell on. Nothing before the deed is re-run.
- **D3 (a test domain is a found deed with fresh keys; built, #249,
  `ns-embassy found-test`).** The trial's steps
  become one call: `found` with a seed, then the embassy's refound, then
  `apply`. The keys are minted by the cradle for a test domain, and never
  for a live domain ("a new key is a new domain").
- **D4 (closure is the done-when, and assay can verify it).** After every
  deed the universe's books close on every axis: nucleons, 3Z, μ and the
  light ledger. Assay's question ("does the flux through the boundary cancel
  on every axis?") is that check made portable. The cradle can submit a
  deed's closure for verification, as minds' work settles.

## 3. What it replaces

| Today | Becomes |
|---|---|
| `pulse found` rebuilding the table each time | `cradle::found(count, place, seed)`, the table held per `(count, g)` |
| the hand-made trial domain | `cradle::found` plus the embassy, in one deed |
| apply, and growth by hand | `cradle::extend` with its receipt |
| `conductor --fold` | `cradle::seal` |
| the universal core's founding alone | founding at every singularity (V6) |

## 4. Witnesses

1. A table held per `(count, g)` equals the one `pulse::founding` makes
   fresh, with nucleons conserved.
2. Found then refound gives the same manifest, the court reading it back
   (1495 entries for da34b0b5's table).
3. Extend conserves nucleons, 3Z and μ: the universe after equals the
   universe before plus the parcel, reconstructed from the receipt.
4. Seal leaves the era readable and nothing reverted.
5. Found at a galactic core's place gives that core's own table, read
   beside the universal core's.

## 5. Build order (#246: steps #248, #249, #250, #251, #252)

1. `cradle::found` in place of the `pulse` binary's body, with the table
   held per `(count, g)` (D1). Witnesses 1 and 2.
2. The test-domain deed (D3), retiring the hand-made trial steps.
3. `cradle::extend` for growth, then for the inbound crossing when #109
   derives the throat's keep formula. Witness 3.
4. `cradle::seal` over `conductor::fold`. Witness 4.
5. Founding at a galactic core (V6, #213), witness 5. This is where the
   table's reach can grow by the places that hold heavy matter.
