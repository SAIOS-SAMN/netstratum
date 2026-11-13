# 135 — What every universe shares, and what each may transpose

> **Superseded in part, 2026-11-13 (operator direction via Mycelium, MU-HOSTING
> §5a.2a, mycelium #304):** the universal physics is already served by the
> hypha through the Concordance, and the R-rows below are **in-universe
> physics: they stay in the kernels.** Mycelium #298–#303 are closed, and their
> fixtures are kept as Netstratum's own tests (`plasma::tests`,
> `cfe::almanac::atom::r7_penetration_fixtures`, `ring::stride_tests`).
>
> - **Served by the hypha** (read from a court, never restated):
>   - the six universal laws at LAW_VERSION 5 (conformal in Cl(4,1), momentum,
>     rigidity, time, reservoir, continuity; ACT_LAWS 0–2);
>   - the register's parameters (mode 5);
>   - the clause register, signature and alignment (modes 7–9);
>   - the `forms` crate (reference, passage, coordinate, coupling, turning);
>   - the coordinate origin and the TUniversal position;
>   - the founding by manifest and grant (ACT_DOMAIN 17, 18);
>   - conservation at the tube (fold nets, carried lines, holonomy,
>     `BasketEntry::holds()`).
> - **Netstratum's own:**
>   - constants and per-kind matter (neutrons, shells, seals, chemistry);
>   - the in-universe dynamics (two-pole equilibria, the atom's fill, the
>     plasma release, the throat's interior);
>   - the founding table and the body-state signature.
> - **Rule:** a new universal form enters only when a crossing kind states
>   its object and the hypha must check it at the tube.
>
> What follows is kept as Netstratum's own register of in-universe laws and
> the transpositions each universe declares.

**Operator 2026-11-13:** "we need coordination on which of these shared
components are required and which can be transposed for universe diversity
and push the shared libraries that make a functional universe/kernel up to
the hypha shared physics library".

**Status:** proposed 2026-11-13; rows landing by joint read (§4.1). Sent to Mycelium (the
shared library's maintainer) and Xylarium for their columns. Read against
Mycelium's LAW register (LAW-COORDINATE v1, LAW-COUPLING v2, LAW-TRAVERSAL;
docs/120, docs/129), docs/104, docs/105, docs/131, docs/132, docs/134.

## 0. The rule that sorts them

A universe is diverse by its **inputs and its orientation**, never by its
laws. A law that differs between kernels makes the seam a translation of
physics, not of letters: what crosses could not be checked. So:

- **Required (shared):** a master formula, its composition law and its
  conservation identity. Written once, in the shared library, with its
  witness. A kernel reads it; it never re-derives it.
- **Transposed (per universe):** what the master formulas are applied to and
  which face a kernel reads them from. A transposition is declared at the
  seam (as the letters are, docs/129 §0) and maps exactly (`t → 1/t`).
- **Derived, neither:** everything the formulas produce from the inputs
  (the echo's period `T_u`, the densest element, the vessel, the bands, the
  stars). A derived value is never an input and never shared: it differs
  between universes because their inputs do.

A value that is none of these (a tuned constant) is open physics
(docs/108), not diversity.

## 1. Required: the shared physics library

| # | Component | Master formula / identity | Netstratum source | Witness |
|---|---|---|---|---|
| R1 | The carrier | exact ℚ (the grain's base is T9, not law: Mycelium 2026-11-05, base 60 display only) | `cfe::ratio`, `ring::grain` | grain idempotent; ledger exact |
| R2 | The face law | `δ = H/(R + H)`; faces `(g, u)`, `g + u = 1`, read from the open or closed axis | `polarity::faces_on`, `ring::deficit` | `g + u = 1`; the throat `g = u` |
| R3 | The two-pole master equation | `x* = α/(α + β)`, `x_n = x* + (x₀ − x*)(1 − α − β)ⁿ` | `polarity::{equilibrium, after}` | `A + B` exact; closed form = n steps |
| R4 | The flow law and its powers | `x ⊕ y = (x + y)/(1 + xy)`; `x⊕ⁿ` closed form; the keep `kⁿ` | `ring::{compose, compose_n}` | `ring::stride_tests` (N, 4N) |
| R5 | Turning | FORMS-TURNING v1: carry `M·x·M̃` and composition in the even subalgebra of Cl(p,q), the signature carried as data (T8) | `orientation::{AXES, NEGATIVES, pairs, is_turning, pole_swap, carried}` | `orientation::cl54_tests` |
| R6 | The nucleon dipole and basket | the dipole `3Q` open / `3Z` held, the electrons' `U` beside; basket `(protons, neutrons, E_bound, M)` (LAW-COUPLING v2, #238) | `chem::{dimensions, basket_entry}` | the basket sums by amount |
| R7 | The atom's fill and its stripping | `R_n = Z·n(n+1)/2`; winding tightness; stripped at `t ≤ a` | `cfe::almanac::atom::{windings, open_at}` | hydrogen's lone winding 1 |
| R8 | The plasma release | `b = g·E(a)/Q`; `ℓ = b·F/w + (1 − b)·S`, `Σ n·w·ℓ = Σ n·F` | `plasma::{bound_share, carried}` | `plasma::tests` |
| R9 | The conservation set | nucleons; `Σ m·ℓ` per plane; charge; light made = emitted + kept | across `flows`, `breath` (`conservation` line) | the breath's conservation read |
| R10 | Traversal | LAW-TRAVERSAL (78.176 §5): not in `forms`; Mycelium's proposal, not a Netstratum row | docs/129 §5 | — |

## 2. Transposed: each universe's own

| # | Component | What a kernel declares | Netstratum | Xylarium |
|---|---|---|---|---|
| T1 | Orientation | which nucleon stands open | protons open (Q) | protons as Z (inverse) |
| T2 | The face read | the place's reading of R2: continuum or its inverse (`t → 1/t`) | the place; the far face inverted | the far face (docs/134 §5) |
| T3 | The founding table | the elements founded and their amounts | Netstratum builds the table (memory: table and hypha roles) | its own |
| T4 | The size | the founded grains (`2^40`, `2^55`) | per founding | per founding |
| T5 | The position | the hypha-assigned TUniversal coordinate | assigned by the hypha | assigned by the hypha |
| T6 | The H base | per kernel (78.173 §8.4: not uniform) | its own | its own |
| T7 | The stride | how many bells a read stands for (docs/132): observation, not physics | per read | per read |
| T8 | The signature | the state's Cl(p,q), mapped by `reference` | Cl(5,4) | Cl(13,1) |
| T9 | The grain's cut | the base a carrier is floored on | 60⁶ | 2^-b toward zero |

Each of T1–T7 changes what the shared formulas act on, never the formulas.
Two universes differing only in T1 are the same physics read from opposite
poles: the seam maps one onto the other exactly.

## 3. Derived: what diversity looks like

From T3–T5 alone the formulas give each universe its own `T_u`, its densest
element, its bands and its stars. The plasma layer's read (docs/134 §3.1) is
an example: one law change moved `T_u` from 186 to 110 on one founding. That
is why a law is never transposed: it would make every derived value
incomparable across the seam.

## 4. Not ready to share

These are open in Netstratum and are not proposed until derived and
witnessed: the throat's heat (docs/134 §4.2), the drift (§4.1), the mixed
parcel (§4.5), the stride's event reader (docs/132 §2), the burn on any
composition (planned).

## 4.1 The joint reads (2026-11-13)

- **Mycelium:** the library is the `forms` crate. One law is one commit with
  one joint read: an issue per row with fixtures, Xylarium reproduces them,
  and the row enters as a Kernel clause, then Network once every kernel reads
  it alike (A7: required is the intersection of the selections). Laws are
  never transposed; inputs, orientation and signature are.
- **Xylarium:**
  - R7 differs in screening. Netstratum's winding is screened only by the
    shells it does not penetrate, `m(m+1) < (n+1)(l+1)`; Xylarium's by every
    shallower shell. The shell rooms are the same (S^11).
  - Xylarium proposes R11, the throat: `s = (H + J)/(2H + J)`, `δ_h = 1/3`.
  - Push and resistance (`p ⊕ (−r)`) goes into R4, the bead's recipe into
    R6, capacity into R7, and the conservation of turning into R9.
- **Filed:** mycelium #298 (R4: read, joins FORMS-COUPLING as `flow_n`,
  `keep_n`), #299 (R2: the same form as `Parcel`, fixtures only), #300 (R3).
- **R7's sphere:** the capacity form takes n as an argument; both kernels
  declare n = 12 (S^11), recorded in the fixtures, with one more at n = 4
  (#301, penetration screening agreed: Xylarium deletes its shortcut);
  R8 filed as #302 (with the seam form), R9 as #303
  (forms A7b: no constant in the library).
- **R10** dropped: 78.176 is Mycelium's proposal, not a forms row.
- **One body-state signature across kernels** would be an operator ruling
  between the projects (raised by Mycelium); until then each declares its own.

## 5. How it goes up

The shared library is Mycelium's (operator: parameters uniform across
kernels are Mycelium's). Netstratum writes only its own tree: it proposes
R1–R10 with their sources and witnesses; Mycelium places them; each kernel
then reads them from the library and deletes its own copy (one model per
quantity). Netstratum's R-rows already Mycelium-held (R1's base, R6, R10) are
read, not proposed.
