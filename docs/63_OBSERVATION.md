# 63 — Observation: compact until watched

This document specifies how a world's state is compacted while it is
unobserved and expanded when it is observed, so that memory is held only for
worlds being read and every compacted world remains exactly reconstructible.

A world's deterministic components (orbits, times) are held in compact form
while no observer is present. When an observer is present (a seated mind,
contact, the operator's watch, or the progenitor line), the world is
expanded and its stochastic draws are evaluated until no observer remains.

## Two compactions

1. **History as indices, not objects (landed).** A world's drained history
   is held as its wire frames (the snapshot's own encoding), decoded only
   where a reader walks. What the live rules ask of the history is kept as
   small indices on the chronicle, updated as receipts are appended and
   rebuilt in one pass when a chronicle is reconstituted: the parameter
   table, the words minted and held, the knowledge converged, the vocabulary
   mints, the recipes built, the count of word receipts, the last estate
   survey, the last drained handoff seal. Readers bounded to a recent window
   (the AP window, the journal's year) walk backward from the head and stop.
   Everything stays reconstructible from the receipts on disk.
2. **Worlds compact until observed** (Concordance 1.12: "a living people is
   always observed (its own life reads its world); a world no one reads is
   carried compact"). Every living line is observed and runs; what compacts
   is a world with no people. Beyond that, a world is observed when a mind holds
   its seat (the house's takes log), the operator watches it (`<var>/watch`,
   one world name a line), it is the progenitor candidate (the largest
   living lineage), or it is the nearest line to one of those (a people
   present: contact). An unobserved line is compact: its snapshot and
   chronicle on disk, its spec in memory at its founding place; it does not
   advance. When observed it is unrolled from its snapshot and its life runs
   until no observer remains, then it is carried and unloaded again. The
   observed set is read every 10 beats; boot unrolls only the observed.

What a compact line keeps: its committed state, receipts and pending sinks
(on disk). What it lets go: workspace (minds' basins, held conjectures,
heard words, words in flight on the bell), as a restart always has; the
bell schedules the dead's words again at the next year close.

## Task list

| id | slice | status |
|---|---|---|
| O1 | kept history as wire frames, decoded on demand | landed |
| O2 | chronicle indices: parameters, words, knowledge, vocabulary, recipes, word count, last survey, last seal | landed |
| O3 | windowed readers walk backward (AP window, journal year) | landed |
| O4 | the wall rebuilt once per year; the ledger read once per change; lineage only on a birth or death; owned words and office evidence folded incrementally | landed |
| O5 | observation (1.12): every living people observed; seated, watched, progenitor, contact; compact and unroll every 10 beats | landed; a first build froze 60 living lines (misread 'a people present' as contact), corrected the same hour |
| O7 | worlds without people (uninhabited planets, the dead's worlds) carried compact and worked out at the coordinate an observer reads, in closed form (1.12) | open |
| O8 | the beat and year driven from TUniversal (Concordance 1.11, Mycelium 78.152) | open, when 78.152 lands |
| O6 | the remaining whole-history readers (growth_of, heat_encased_of, the axis folds at boot) as indices | open |
