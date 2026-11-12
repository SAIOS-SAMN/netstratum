# 132 — The stride: the whole system faster, its equations whole

**Operator 2026-11-12:** "you need the ability to increase the velocity of
the entire system without fracturing the equations: which should be
allowable in a deterministic state".

**Status:** derived 2026-11-12; not built. Read against docs/117 §11.4
("over n bells the motion is that rotor composed n times … held as its exact
base and its formula: the generator and n"), docs/100 §8.1 (E38, the
attractor's keep) and `breath::breathe`.

## 0. What stands

- **A galaxy's laws run only on an echo's arrival** (`breath::breathe`, the
  loop over `arrivals`). The torque, the shear, the fluid, the luminal
  bound, the ring motions, the magnet, the exits and the bindings run once
  every `T_u` bells. On g557 the census at bells 392 and 557 is identical:
  two echoes in 557 bells, so the home's angular-momentum transport has run
  twice in its life.
- **Running every bell** would give the transport its time, at `T_u` times
  the cost a bell.
- **Stepping n bells as one bell scaled by n** fractures the equations: a
  rate times n leaves the flow law's domain, a keep `k` applied once is not
  `kⁿ`, and the order of events inside the n bells is lost.

## 1. The stride

The system's velocity is its **stride** `n`: the number of bells one
application of each law stands for. Each law is advanced n bells by its own
composition law, in closed form, never by a rate multiplied by n:

| law's form | one bell | n bells |
|---|---|---|
| a turning (Cayley motion) | `R` | `Rⁿ`, held as the generator and n (docs/117 §11.4) |
| a boost (flow law) | `D(u) = (1 + u)/(1 − u)` | `D(u)ⁿ`; its rate `(Dⁿ − 1)/(Dⁿ + 1)` |
| a relaxation toward a target the step keeps (an attractor, E38) | `x ↦ x* + k·(x − x*)` | `x ↦ x* + kⁿ·(x − x*)` |
| an exchange among a range toward its holding-weighted mean (shear, fluid, conform, drag) | the mean is kept, each departure kept by `k_i` | `kᵢⁿ` on each departure while the range stands |

`kⁿ` and `Dⁿ` are powers of exact rationals: compositions of one law with
itself, not squares of lengths (design law 2 refuses a norm read by its
square, not a law composed with itself). Each is held as its base and n and
put on the grain once, at the end of the stride.

**Why it is exact.** A relaxation keeps its target (the attractor) or its
mean (an exchange pays by holding, `Σ w·x` kept), so the target does not move
during the stride, and n compositions of `x ↦ x* + k(x − x*)` are
`x* + kⁿ(x − x*)` exactly. A motion's n-fold composition is its power. The
equations are the same equations, composed.

## 2. Events end a stride

The closed forms hold while the coefficients stand: the same range meeting
each mote, the same hold `H(<ℓ)` under it, the same face. Each of these
changes only at an **event**:

1. **an order change of spin:** two motes' spins cross, so `H(<ℓ)` steps
   (the enclosure's order, `Enclosed`);
2. **a range change:** two motes' extents start or stop meeting (`meeting`);
3. **a crossing:** a plane's span passes zero (the throat, docs/131 §4), a
   spin component passes zero (through the centre), the luminal bound;
4. **a binding:** a band's grip passes its hold (`band_pairs`);
5. **an echo's arrival** (every `T_u`, as now).

The state is deterministic, so the next event is a reading, not a guess:
each quantity's path over the stride is its closed form, and the least whole
`n` at which any event condition holds is found by bisection on `n`, each
test an exact comparison of the closed forms at that `n` (no logarithm, no
root). A stride runs to the bell before the first event; the event bell is
stepped singly; then the next stride is read.

**The velocity is a choice of observation, not of physics.** The
universe's course is the same at any stride; only how many bells pass
between reads changes (the 2026-10-24 ruling: render exact only under
observation). A cap on the stride (`n ≤ T_u`, so no echo is skipped) is the
only bound.

## 3. Witnesses

1. **Composition:** each law's n-bell closed form equals n one-bell
   applications held exact (ungrained), at `n = N` and `n = 4N`
   (AGENTS.md: work counters N against 4N).
2. **Conservation over a stride:** `Σ m·ℓ` per plane, the reservoir and the
   antipole stand as they do bell by bell.
3. **Events found, not passed:** a stride never carries two spins past each
   other, a range past its meeting, or a span past zero; the bisection's
   event bell is the first at which single steps reach it.
4. **Determinism:** two runs from one state at strides `n` and `n/2 + n/2`
   reach the same state (on the grain).

## 4. Build order

1. Each law's n-bell form beside its one-bell form, replacing it (one law:
   one bell is `n = 1`), with witness 1: the attractor keep (`conform_axes`,
   `conform_partners`), the flow law, the orientation's `advance_by` (it
   already composes `rings` motions), then the shear and the fluid.
2. The event reader: the least `n` per event kind, by bisection.
3. `breath::breathe` runs a galaxy's transport by strides between echoes,
   not only on arrivals; the echo stays the pulse's arrival.
4. Read on a copy of g557: matter between the ring and the rim, bands, and
   the cost a stride against a bell.

## 5. Open

1. Laws whose coefficient is read from the moving state itself, not from a
   kept target (the torque's response to the phase it passes, `mean_response`):
   their n-bell form is the integral the response already takes over a
   passage (docs/128 §9.6); whether a whole stride is one passage is to be
   derived.
2. The shear's rate on the vacuum face (`stream_delta`, `1 − δ`) rises with
   reach, so on the vacuum the pull toward the range's rate moves spin
   inward. With docs/131 step 5 (the stream a reading) the shear reads one
   rate on both faces.
