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
| an exchange among a range (shear, fluid, conform, drag): each step pays its neighbours by holding | an affine map `x ↦ A·x + b` on the range's components, `Σ w·x` kept | `Aⁿ·x + (Σ_{i<n} Aⁱ)·b` while the ranges stand (§1.1) |

`kⁿ` and `Dⁿ` are powers of exact rationals: compositions of one law with
itself, not squares of lengths (design law 2 refuses a norm read by its
square, not a law composed with itself). Each is held as its base and n and
put on the grain once, at the end of the stride.

**Why it is exact.** A relaxation keeps its target (the attractor) or its
mean (an exchange pays by holding, `Σ w·x` kept), so the target does not move
during the stride, and n compositions of `x ↦ x* + k(x − x*)` are
`x* + kⁿ(x − x*)` exactly. A motion's n-fold composition is its power. The
equations are the same equations, composed.

### 1.1 The exchanges are coupled

An exchange's step on one mote is paid by its range, so its n-bell form is
not `kⁿ` mote by mote: the payments move the neighbours off their own
targets, and they relax back in the bells after. Between events the map is
affine with a fixed matrix `A` (each row the mote's keep and its range's
payments), so n bells are `Aⁿ` and its geometric sum, exact. `A` is sparse
(the ranges) but its powers fill in across a connected range, so `Aⁿ` by
squaring costs the range's size squared a squaring. Per mote `kⁿ` holds
only for a law whose step pays no one (an attractor on its own kept target,
the motion, the flow law). Measured cost a single application on g557's
3,827 motes (`observe ablate`): conform_partners 5.4 s, conform_axes 2.2 s,
the magnet 16.8 s, ring_motions 15.8 s. So the exchanges' stride form is
the build's real cost, and is read before step 3 (§4).

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

## 5. The torque over a stride, read as probabilities

The open point was the torque: its response is read from the phase it sweeps
(`flows::mean_response`), so its coefficient moves with the state.

**Between echoes there is no push.** The torque is the echo's push (`p`, the
core's state against its throat, `pulse::home_push`; `breath::breathe` reads
it inside an arrival). Between arrivals `p` is not delivered, so the torque
does not enter a stride. At an arrival it acts over that bell's passage, as
built, and that bell is stepped singly (§2, event 5). A stride carries the
transport and the motion; the torque stands at its ends.

**Across echoes, the push is a draw on the phase.** Where an echo meets a
mote is fixed by the state, but over many echoes it is read by the share of
time the mote's phase spends in each part of its circle, its occupation
measure:

    ρ(ψ) = (1/ω(ψ)) / ∮ dψ/ω        (ω the phase's speed a bell)

- **The response is the sawtooth `1 − 2·frac(ψ)`:** odd about the half turn,
  `r(1 − f) = −r(f)`. Between echoes no torque moves the spin, so `ω` stands
  over the turn and `ρ` is uniform: the gaining hemisphere (`f < ½`) and the
  losing one are met with probability ½ each, and the expected response over
  a passage of any length is

      E[mean_response(ψ, len)] = ∮ ρ(ψ)·mean_response(ψ, len) dψ = 0

  So the echo's push carries **no expected spin**: across echoes it is a
  draw of mean zero, each draw bounded by `|Δℓ_k| ≤ p·len_k/2`. The push
  redistributes spin among motes and phases; it does not raise the spin of
  the matter on average.
- **At a resonance, a mean of `1/q` at most** (witnessed, `ring::stride_tests`).
  A mote turning `ν = T_u·ω` turns between echoes meets each echo
  `frac(ν)` further on. Where `frac(ν) = a/q` it is met at the same `q`
  phases, a `q`-th of a turn apart, and the push's mean response is exactly
  the response over `q` times the arc, read from `q` times the phase, over
  `q`:

      mean_q = mean_response(q·ψ₀, q·len) / q,      |mean_q| ≤ 1/q

  So every resonance has a mean, of size at most `1/q`: a low-order one
  (`q` small) drives its motes in or out steadily, a high-order one barely,
  and the fair draw (mean zero) is the limit `q → ∞`. This is an orbital
  resonance with the pulse, as nature's Kirkwood gaps are with Jupiter.
- **So the disc's spin cannot come from the echoes.** With the push fair
  on average, matter that stands inside the ring stays there under echoes
  alone. What moves spin outward on average is the transport (the shear and
  the fluid, by strides, §1) and what the bounce lays at release (docs/131
  §7). This is the measured state of g557 read as a law.

**The phase over a stride.** The transport moves the spin by closed forms
(`kⁿ`), so `ω` moves with it and the phase at the stride's end is
`ψ_n = ψ₀ + Σ_{i<n} ω(ℓ_i)`, a sum over a known sequence: each term is
exact and cheap (one reading a bell, no law), so it is summed, never
approximated. Between reads it is held as its base and its formula and
rendered on observation (the 2026-10-24 ruling).

**Witnesses** (adding to §3):
5. At a resonance `frac(ν) = a/q` the mean draw is
   `mean_response(q·ψ₀, q·len)/q`, bounded by `1/q`, at q = N and 4N
   (built: `ring::stride_tests`).

## 6. Open

1. The shear's rate on the vacuum face (`stream_delta`, `1 − δ`) rises with
   reach, so on the vacuum the pull toward the range's rate moves spin
   inward. With docs/131 step 5 (the stream a reading) the shear reads one
   rate on both faces.
