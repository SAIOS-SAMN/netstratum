# Era convergence — annexing the archived timelines (CONVERGE-1)

**Status:** landing spec — the mechanic and its witnesses land with this
document (Charter §1). Constants here marked *placeholder* tune at balance
time; everything marked *law* binds.

> **Corrected 2026-08-13 (doc audit): this is design-stage, not landed.**
> Zero hits in `engine/` for `TUniversal`, `convergence_pass`, or era
> annexation — the "landing spec" label was never made good on.

**The problem.** Three archive eras exist
(`var-era1`: 22 worlds, `var-era2`: 38, `var`: prior runs of the current
root) whose receipts carry only local stamps — era1's `stamp 65` and era2's
`stamp 52` are incomparable numbers on unrelated timelines, and no era ever
persisted the universe card. Boot today is a blind genesis event beside a
archive: nothing reads the archives, every run mints fresh founding
reserves, and any naive merge would interleave same-stamp receipts from
different timelines — corruption by converging timelines.

**The resolution:** persist **TUniversal{S, G, T}** (the coordinate the lens
records: S the spatial triple on the 3-torus, G the gauge fiber phase, T
the universal tick odometer `00:00:00:00:00:00`), and replace blind genesis
with a boot-time **convergence pass** that adopts every intact archived
world onto one timeline and one chart — by annexation, never by merge.

---

## 1. The laws

1. **Annex, never merge (SEAL B).** Archived chronicles are sealed — not
   one byte is rewritten, reordered, or deleted. Convergence appends
   `EraAnnexed` receipts (tag 21) to a new universe-level ledger
   (`<root>/convergence.tlv`, cfe chronicle machinery: predecessor-linked,
   digest-chained, tamper-refusing). Replay of the present re-derives the
   convergence; history is never touched.
2. **Verify before adopting.** Each archived world re-verifies through the
   real chain law (`cfe::vrp` digests over the raw event bytes — unknown
   event tags still verify, SEAL A). A broken chain is **quarantined**:
   excluded from the timeline, left on disk, named in the read. Never
   merged, never deleted.
   **The torn-tail law (measured on the real archives, 2026-07-27):** a
   interrupted writer leaves a final line of NUL bytes or a half-written
   record — an append that never completed. By SEAL B (what has no
   receipt does not persist) the world annexes at its **verified
   prefix** and the tear is recorded in the read. Unparseable lines
   carry no receipt data and are skipped — if one ever hid a lost
   receipt, the chain law breaks at the next predecessor link and the world
   quarantines anyway. A content-bearing malformed line anywhere but
   the tail is interior corruption: quarantine.
3. **One T axis by epoch shift.** Eras are sequential, never concurrent, so
   a pure offset gives a valid global order:
   `epoch(era_1) = 0; epoch(era_{n+1}) = epoch(era_n) + max terminal T of era_n`,
   and `TUniversal_T(receipt) = epoch(era) + local stamp`. Lamport-style
   epoch ordering — the archival special case of NS-5 consensus, designed
   as annexation receipts so live mesh convergence (N6) subsumes it rather
   than fights it.
4. **Placement is a measurement, never a hash.** A world's S, G derive from
   its own history through the four C's (§2) — exact rationals end to end,
   no wall clock, no randomness. Re-running convergence on any machine
   places every world at identical coordinates.
5. **Idempotence.** `EraAnnexed` receipts key on (era, world, terminal
   digest); a second boot sees them and skips. Convergence is run-once by
   construction — no double-annexation drift.
6. **Spawns read the converged universe.** Genesis after convergence
   charts new worlds against the converged star-chart (occupied
   coordinates visible) instead of founding blind.

## 2. Placement — the four C's as the algorithm

The endpoint fold is refused (zeroth-order: it forgets the path). Placement
reads the trajectory, per the house verdict vocabulary:

- **Congruence (each receipt a transition):** every decoded event deposits
  an exact-rational shear onto the three axes (`physics::Triple`), graded:
  - *spatial* (the matter line): `YieldCollected +amount`,
    `UpkeepSettled −amount`, `FamineStruck −drawn`, at grade = resource
    code;
  - *temporal* (the pace line): every receipt `+1` at grade 0;
  - *gauge* (the authority phase): `EntityRegistered +1` @0,
    `ScopeGranted +1` / `ScopeRevoked −1` @1,
    `ColonyFounded/Charted +1` @2.
  Unknown event tags deposit nothing (SEAL A) but still verify (§1.2).
- **Coherence (what loops fail to close):** the per-axis residual R —
  strikes−reliefs (spatial), builds−razes (spatial structural),
  grants−revokes (gauge) — measured and kept, never minimized (the house
  law: the residual is structured information).
- **Cohesion (the fiber):** R rides the fiber — written to the world's
  `fiber.json` card. Two worlds with one endpoint but different loop
  content separate here; the fiber makes base-coordinate collision
  harmless, so there is no probing and no collision handling at all.
- **Convergence (placed where it was going, not where it stopped):** the
  terminal-window net (last `W = 8` receipts, *placeholder*) is added once
  to the fold — one step of `𝒟_T` forward. An interrupted ascending world
  lands ahead of its endpoint; a collapsed one lands where the collapse
  was taking it.

The chart projection (*law — fixed so placement is reproducible*):
`x = Σ` spatial even grades, `y = Σ` spatial odd grades, `z = Σ` temporal,
`g = Σ` gauge — each plus its terminal-window net, wrapped mod span by
`chart_world` (L1 torus; no squaring).

## 3. What convergence buys (the recovery arc)

- **Triage:** the trajectory separates *interrupted* (positive net flows,
  receipts just stop — the era's process died, not the world) from
  *collapsed* (famine cascade). The first resumes live; the second lands
  as a derelict — salvage site and archaeology, not noise.
- **Recovery is the existing law, fed by neighbors:** placement lands a
  world near its structural kin — the economies fittest to supply it.
  Famine relief (banked recovers to one-tick upkeep → zones wake) and
  `ColonyCharted` (reserves move, conservation) are already landed and
  witnessed; convergence connects them to the dead.
- **Nothing minted, identity preserved:** archived banks stay real matter;
  registries (`cryochoir`, `deep-chorus`…) are rejoined, never re-founded —
  one civilization, one identity, across timelines.
- **Future landings (recorded, not landed here):** salvage/charter into
  annexed derelicts; fiber-read founding hints; thermal coupling
  (05 §3) once thermal lands.

### 3.1 INCURSION-1 — the archived-sheet door (specced 2026-07-30; 1a landed 2026-07-31)

**Layer: universal** (`kernel::layer_of`, `17_CAPACITY` §3d). The deed
receipts `era · world · line` and binds none of them — replay does
nothing with it and no live path consumes it. **A layer is what a deed
binds, not what it mentions:** `EraAnnexed` names a world and binds no
world either. The three names in this record made it look as though it
spanned all three layers at once; it does not, and the criterion is
what needed sharpening.

**INCURSION-1a landed** (tag 81 `IncursionStruck`, universe ledger
only): `conductor::incursion(era_root, world)` wakes a sealed
derelict — the archive is never edited (byte-identical after the
wake, witnessed); the living are never raided (any unfamished orb
refuses typed); the sealed chronicle is copied forward and continued
live under the woken line (succession law — the tree drops to the
kernel, the record rides whole); the revenant line registers
chronicle-seeded (axes beacon-drawn from the terminal digest, clade
`revenant`, policy Aggressive — the entropic leg); the door receipts
era · world · line. One wake per seal. The aeon-voice trigger stays
the orchestration's (deferred, recorded). The original spec:

The archive is frontier read actively: sealed eras are not scenery,
they are a door that opens both ways. The lawful Antaran shape:

- **What crosses.** Nothing crosses that is not receipted. An incursion
  is a succession-class deed executed from the live side against an
  annexed derelict chart: a live line claims salvage rights on a sealed
  world (the NOVEL-1 pattern — a chronicle inherited, never edited), or
  a derelict's dormant automata wake as an incursion polity — a
  conductor-seeded line whose spec derives from the sealed chronicle's
  terminal state (chronicle-seeded, Charter §9; no dice).
- **The door's gate.** Incursions open only at aeon-voice alignments
  (11 §1 — board-scale events read at board-scale cadence) and only
  onto worlds whose four-C trajectory settled in the retirement region
  — the living are never raided; the shield law binds (a pre-crossing
  world cannot be touched from the archive side either).
- **What it stresses.** The maintainer surface: an incursion polity
  plays the entropic leg (claims derelicts, absorbs vents, rides
  demurrage — the archived fold sheet's side of the manifold made
  playable in the small; infinity is disallowed, the torus wraps).
  Every deed receipted, conservation-exact; never record-destruction.
- **Lands with:** a conductor incursion step (the sweep pattern), the
  incursion spec fields, and vectors — one landing, tag claimed at its
  cadence. Constants placeholder.

## 4. Wire and disk

- **Tag 21 `EraAnnexed`** (permanent, NS-1 §2.3 registry):
  `LE64(epoch offset) ‖ terminal digest[32] ‖ LE16(era len) ‖ era utf8 ‖ world utf8`.
- **`<root>/convergence.tlv`** — the universe ledger: framed (tag 20)
  `EraAnnexed` receipts, loaded through the verification door, synced
  append-only (`cfe::persist`).
- **`<root>/universe.json`** — the card, now actually persisted every
  beat and at convergence: topology, span, `tuniversal` odometer, ticks.
- **`<root>/starchart.json`** — annexed worlds charted beside live ones.
- **`worlds/<w>/fiber.json`** — the world's residual card (reflection,
  never authority).

## 5. Witnesses (land with this document)

jsonl→receipt round-trip re-verifies Intact; a tampered line quarantines
the world with its break named; epoch offsets stack eras without overlap;
placement is deterministic across re-runs and separates same-endpoint
worlds by fiber; a second converge appends nothing (idempotent); the
universe card carries the odometer; conductor boot converges before
genesis.
