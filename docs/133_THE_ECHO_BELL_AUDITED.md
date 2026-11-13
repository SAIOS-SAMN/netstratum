# 133 — The echo bell audited: the front, and what it does not explain

**Operator 2026-11-12:** "diagnose the echo bell as a multi-dimensional
object instead of flattening it and describe how that impacts the
topology"; then "scope what it doesn't explain — re-scope the audit and
derive".

**Status:** derived 2026-11-12, one law measured (§3). Read against
`ring.rs` (the front), `breath::breathe` (the echo bell), docs/132 (the
stride) and `observe ablate`.

## 1. The echo as a front: what it explains

The bounce's front is "the geodesic sphere of light-time about it, growing
from S⁰"; it reaches a place at reach r after `r + H(<r)` bells (`ring.rs`).
`breath::breathe` fires it once per galaxy, every law on every mote at one
bell. Read as the front, the echo has four dimensions the bell collapses:
reach (each mote at its own arrival), plane (each isoclinic plane its own
ring), face (outbound on the continuum, inbound on the vacuum) and time
(the shell is always somewhere). Collapsing them:

- makes the galaxy a return map sampled every `T_u` bells, frozen between
  (the identical census at bells 392 and 557);
- makes the drive isotropic: the planes cannot separate (#167's read: "a
  source of asymmetry in the drive is still missing");
- reduces the Möbius circulation the two faces' fronts would make through
  the throat to two flashes at once;
- takes the laws' locality away: every law sees every mote at once.

## 2. What the front does not explain

1. **The cost.** The echo bell on g557 did not finish in 40 minutes. The
   front spreads the home's arrivals over about 1–2.5 bells (reaches
   0.06–1.3 about a hold of 1–1.26), so it moves the work, it does not
   remove it.
2. **The stored widths.** The carriers at bell 557 (`breathe`'s fields
   line, in bits): amounts 220, agitation 108, lever place 231, reservoir
   261. Amounts are whole nuclei on the one nucleon denominator,
   `1024·60³⁰` (`pulse::nucleus`, about 187 bits), by design, and the
   carriers on the grain (`60⁶`, about 36 bits). These are bounded: no
   stored value is collapsing. A collapse of the stored state does not
   explain the cost.
3. **The stages never measured.** The ablation (docs/128 §14, `observe
   ablate`) timed six laws, each under 23 s. The echo bell runs about 22
   stages; sixteen were never timed (release, torque, emission, recycling,
   shear, fluid, luminal, exits, chains, bindings, births, merges, the
   balance, star collisions, systems, the beat).
4. **The composition.** The ablation applied each law to the same state;
   the bell applies each to the one before's output. Its cost is not the
   sum of the laws' costs taken alone.

## 3. The re-scoped audit: the cost as a product of dimensions

A law's cost on the bell is a product, never one number a stage:

    cost = (motes it touches) × (range terms each) × (rings) × (width of each term's product)

The width of a term is the sum of the bits of the readings it multiplies.
Readings are exact by ruling (operator 2026-10-23: "approximations leaking
into an exact system"; `Enclosed::reach`: "readings, never stored, so never
floored"): a deficit `δ = H/(R + H)` is a quotient of two wide integers, a
face `faces_on(δ)` another, and a holding `w = amount·weight` sits on the
nucleon denominator. A term that multiplies three or four of them is
kilobits wide.

**Measured (g557, `observe ablate 0`, one application):** `flows::emit`
(heat into the manifold, two rings) took **115.5 s**, the slowest law yet
timed, though it returns light of only 477 bits and leaves agitation at 249
bits. Its terms, `recycled·w_j·keep_j/whole`, multiply four exact
readings, each range's `whole` its own sum of such products; the range
terms are few (722 meetings across the disc). So width, not count, is its
cost.

## 4. The law the audit gives

**A coefficient enters a law's product on the grain; a reading stays exact
where it decides.** Each per-mote coefficient that multiplies into a stored
change (a holding share `w`, a face's keep, a transit) is taken on the grain
once at the law's read, as `ring_motions` already does ("the holding on the
grain, the transits already on it"), and the comparisons that decide (a
face, a crossing, a meeting) read the exact values. The stored change is on
the grain already; its remainder is booked where the law books it (for
`emit`, "each grain's floor radiates"), so every ledger identity stands
exactly.

This keeps the 2026-10-23 ruling: no reading is floored where it is read.
What is floored is the carrier a product is built from, the same carrier
the stored result is held on.

## 5. The audit to run

On one echo bell of a g557 copy, time-boxed, per stage: motes touched, range
terms, rings, the widest coefficient entering a product and the widest value
written, and time; and the work split by reach shell, plane and face (§1).
**Prediction:** time tracks the product width wherever coefficients enter
exact, and falls by an order of magnitude a law when they enter on the grain;
the ledger identities hold before and after.

## 6. Witnesses

1. Each law's ledger identity (`Σ a·m` and light for `emit`; `Σ m·ℓ` for
   the exchanges) exact before and after its coefficients enter on the
   grain.
2. The written values' widths bounded by the grain's, whatever the range.
3. The bell's decisions (faces, crossings, meetings) unchanged: read from
   the same exact values.
