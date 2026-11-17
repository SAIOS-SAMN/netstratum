# 137 — The gravitational catch: axioms, theorems, proofs, and their traversal

**Status:** derived 2026-11-13; G1 built 2026-11-14 (`formation::settle_system`),
read 2026-11-15 (§5). T1–T8 hold for the code as built, witnessed by W1–W4. Nothing here is a new law: every axiom is a law already standing,
cited by its code.

**Operator 2026-11-13:** "the throat (universe or galactic) being the
gravitational pull on the star based on density that creates sub
gravitational grasp via manifold chains so that the orbit of the planetary
masses as they form are drawn to the star … without hardcoding this it
should be the emergent property based on the elements — do you see this as
orthogonal gravitational catch"; "we need derivations and formulas — axioms,
theorems, and to provide proofs that can get written in anastome as the
traversal of the hypha".

**Names** (nature's, where they exist): the *in-plane catch* is the orbit a
body's enclosed hold sets, the ring law. The *orthogonal catch* is the
differential rotation (tidal shear) across a body's own extent, which spins
its partner orbits on the planes orthogonal to the orbit. *Capture* is
membership of an enclosure, inside its rim.

## 0. The tiers

One grammar, coupled tiers (docs/78 design law 4): each tier is an
enclosure, and each body orbits in the enclosure of the tier above it.

| tier | its enclosure's base `B` | what orbits in it |
|---|---|---|
| universe | the universal core's throat | galaxies, the ocean's motes |
| galaxy | its core's throat (`breath::galaxy_enc`) | its motes and its stars |
| star system | its star core's kept matter, `M_kept/M_total` (no throat; docs/134 A10) | its disc's motes and its worlds |

## 1. Axioms

Each axiom is a law already standing.

- **A1 (the hold by place).** An enclosure is a base `B` and bodies `j`,
  each at its place `r_j` with holding `h_j`. The hold within reach `r` is
  the base and every body standing within it:

      H(<r) = B + Σ_{r_j < r} h_j

  Code: `flows::Enclosed::hold_within`; the star system's enclosure,
  `formation::system_frame_of`.
- **A2 (holding is matter).** A body's holding is the sum over the kinds it
  holds of each amount times its weight, `h = Σ_k a_k·w_k`. A kind `k` is
  its nucleon dipole, `Q_k` protons and `Z_k` neutrons (3Q and 3Z frames,
  R the electron's phase; docs/97, the inverse of Xylarium's letters). Its
  weight is `w_k = Q_k + Z_k`. No tier constant enters, and no kind is keyed
  by one letter. Code: `pulse::mass`,
  `flows::Mote::holding`.
- **A3 (the ring law).** A body at reach `r` turns on its plane with the
  rate, the generator's component, `b = H(<r)/r`. Its spin is
  `ℓ = r·H(<r)`, and its deficit is `δ = H/(r + H)`, so `b = δ/(1 − δ)`.
  Code: `ring`, `orientation::rate`, EQUATIONS E57.
- **A4 (the motion is the Cayley form).** One ring's motion on plane `E` is
  `R = (1 + b·E)(1 − b·E)⁻¹`, and the generator read back from `R` is `b`
  (FORMS-TURNING v1, served by the hypha).
- **A5 (the halves).** A body of extent share `e` at reach `r` has the shear

      σ = (2e·H(<r) − ΔH)/(2e·r),     ΔH = H(<r(1 + e)) − H(<r(1 − e))

  Its two halves turn at `b± = b ± σ/2`, with `σ ↦ −σ` on the vacuum. Each
  half's spin is the one standing at its rate's reach, `ℓ±* = R±·H(<R±)`
  with `b±·R± = H(<R±)`. The partner target is `ℓ_R* = (ℓ−* − ℓ+*)/2`,
  with the plane's sense. Code: `flows::{shear_rate, ring_volume,
  partner_target}`, `Enclosed::reach_at_rate`; docs/128 §10.3.
- **A6 (conformation, paid).** Each partner spin relaxes toward its target
  by the attractor's keep at its deficit, `ℓ_R′ = ℓ* + k·(ℓ_R − ℓ*)` (E38).
  The range meeting it pays the change by holding. Code:
  `flows::conform_partners`.
- **A7 (the rim).** An enclosure's rim is its whole hold, `rim = H_whole`
  (docs/78). A body whose reach passes it leaves the enclosure. Code:
  `formation::settle_system` (escaped past the rim).

## 2. Theorems and proofs

**T1 (the catch is the matter's).** In every tier the in-plane catch `b`
at reach `r` is a function of the kinds held within `r` and their amounts
alone.

*Proof.* By A3, `b = H(<r)/r`. By A1, `H(<r)` is `B` plus holdings. By A2,
every holding, and every base (a throat's hold or a star core's kept
matter), is `Σ a_k·w_k` over kinds `k = (Q_k, Z_k)`. No other term enters. ∎

**T2 (one ring of an orbit is a served motion).** One ring of a body's
orbit on plane `E` is the FORMS-TURNING motion of generator `b·E`, with
`b = H(<r)/r`. A court given the body's coordinate and its turning
reconstructs `b` exactly.

*Proof.* A3 and A4. The Cayley form is exact over ℚ, and its generator is
read back unchanged (FORMS-TURNING's statement). ∎

**T3 (at fixed places, the star's hold cancels in the shear's volume).**
With the bodies at fixed places, `ΔH` does not depend on the base `B`. So
the shear grows with the base at first degree:

    σ(B) = σ(0) + B/r

*Proof.* By A1 both `H(<r(1 + e))` and `H(<r(1 − e))` contain `B` once, so
`B` cancels in `ΔH`. Then
`σ = (2e·(B + I(<r)) − ΔI)/(2e·r) = (2e·I(<r) − ΔI)/(2e·r) + B/r`, with
`I` the bodies' part of the hold, fixed with their places. ∎

**T3′ (a heavier star draws a body of the same spin nearer).** A body known
by its spin `ℓ` stands at the least `r` with `ℓ = r·H(<r)` (A3; the code
holds bodies by spin, `Enclosed::reach`). Raising `B` lowers that `r` and
raises its deficit `δ = H/(r + H)`.

*Proof.* `H(<r)` grows with `B` at every `r` (A1) and never falls with `r`.
So `r·H(<r)` reaches `ℓ` at a smaller `r`, and there `δ` is larger,
because `H` is larger and `r` smaller. ∎

So a star whose core keeps more matter holds a body of a given spin nearer
and deeper (T3′), and at a given place it shears it harder (T3). By T1,
that matter is its elements. This is the "drawn to the star": the disc's
places are readings of the spins against the star's kept hold, never set.

**T4 (rigid gives none; a lone parcel gives its orbit's rate).** `σ = 0`
exactly when `ΔH = 2e·H(<r)`. Where nothing stands across the parcel
(`ΔH = 0`), `σ = b`.

*Proof.* The numerator of A5 vanishes exactly when `ΔH = 2e·H`. With
`ΔH = 0`, `σ = 2e·H/(2e·r) = H/r = b` by A3. ∎

**T5 (the orthogonal catch of a lone parcel is ⅔ of its orbit).** Let the
hold be one value `H₀` over the halves' reaches `[R+, R−]`, and let
nothing stand across the parcel. Then `R± = r·b/b±`, and

    ℓ_R* = (2/3)·ℓ,     ℓ = r·H₀,

with the plane's sense, whatever the extent `e > 0`.

*Proof.* By T4, `σ = b`, so `b+ = 3b/2` and `b− = b/2`. With the hold
`H₀` over both reaches, A5 gives `R± = H₀/b±`, so `R+ = (2/3)·r` and
`R− = 2r`, and `ℓ±* = R±·H₀`. Then
`ℓ_R* = (R− − R+)·H₀/2 = (2r − (2/3)·r)·H₀/2 = (2/3)·r·H₀`. ∎

The partner spin is a reach times a hold, first degree in each reading,
with no square. By T3 it grows with the star's kept matter.

**T6 (capture grows with the star).** A body stands in a star system iff
its reach is within the system's rim, `H_whole = B + Σ h_j`. Raising the
kept matter raises the rim. So a heavier star holds bodies to a greater
reach.

*Proof.* A7 and A1. ∎

**T7 (the orthogonal catch is paid, never made).** Across one
conformation, on every partner plane,

    Σ_j w_j·ℓ_R,j(after) − Σ_j w_j·ℓ_R,j(before) = ρ,

where `ρ` is the grain's remainder.

*Proof.* By A6, each change `w_i·Δℓ_R,i` is met by the range on reach,
which pays `−w_i·Δℓ_R,i` by holding. What the grain floors from those
payments is `ρ`, and `conform_partners` returns it. ∎

With `ρ = 0`, the sum of the payments around the range is a closed path
whose holonomy is trivial: the form MU-ANASTOME §4.3 holds a relation in.
`ρ` is the carrier's grain, not physics. **As built it is counted, not
booked:** the star system names it as the identity "system partners"
(`Settled::partners`), and the galaxy and the ocean discard it
(`let _ =`). Booking it by a law is G4 (§5), so the closed path holds to
the grain, not exactly.

**T8 (one law on every tier).** T3–T7 hold for any enclosure that
`Enclosed` represents, and the star system's enclosure is one
(`formation::system_frame_of` returns `Enclosed::with_lifts`). So the
orthogonal catch in a star's disc is the same law as in a galaxy, with
nothing new written.

*Proof.* A1–A7 are stated on `Enclosed` alone. ∎

**T9 (where it runs).** The conformation runs on the galaxy's motes
(`breath.rs`, after the torque), on the ocean's (`pulse.rs`) and, since G1,
on a star system's disc (`settle_system`, on the system's own enclosure).
This is a reading of the code, not a theorem of the physics.

## 3. The traversal through the hypha (MU-ANASTOME §9)

By the operator's direction relayed by Mycelium (2026-11-13), universal law
is served by the hypha and in-universe dynamics stay in the kernel. The
catch is in-universe. What crosses is the mathematics a mind perceives,
and each theorem's certificate is something a court reconstructs:

| theorem | what is posted | what a court reconstructs |
|---|---|---|
| T1, T2 | each body's coordinate `S:G:T:H:K` (LAW-COORDINATE) and its turning (FORMS-TURNING) | the generator `b` from the motion; one ring of one body as a single-body `Transition` (motion: the rotor of `b·E` about the attractor; admitted by the conformal, momentum and time laws, as `ns_embassy::probe::turn` is) |
| T3–T6 | the enclosure's holds as the posted bodies' coordinates | the identities, recomputed from the posted state (reconstruction, never replay) |
| T7 | the before and after states of the range | `Σ w·Δℓ_R = ρ`, a closed path of payments with trivial holonomy |

**What cannot cross as one motion, and why that is right.** A court's
`Transition` carries every body by one versor, and metric rigidity keeps
every pair's product. Differential rotation is not rigid. Posting a sheared
disc as one transition would be refused by rigidity, and correctly: the
shear is the departure from rigid (T4). So the orthogonal catch crosses as
its parts: each body's own ring (T2), and the payment identity (T7). A mind
perceives it as MU-ANASTOME §9.2 states: "orbits about those attractors,
sheared by gravity and precessing, never returning exactly".

## 4. Witnesses

- **W1 (T3), `the_stars_hold_cancels_in_the_shears_volume`:** two
  enclosures with the bodies at the same places and bases `B₁`, `B₂`: `ΔH`
  equal; `σ₂ − σ₁ = (B₂ − B₁)/r`. A first draft held the bodies at the same
  *spins* and failed (`15/16` against `3/4`). That failure is T3′.
- **W1′ (T3′), `a_heavier_star_draws_a_body_of_the_same_spin_nearer`:** the
  same spins under a heavier base stand nearer and deeper.
- **W2 (T4):** a point hold gives `σ = b`; a hold growing with the reach
  gives `σ = 0` (`the_shear_is_the_hold_across_the_parcel_against_the_rigid`).
- **W3 (T5), `a_lone_parcels_partner_spin_is_two_thirds_of_its_orbit`:** a point hold: `(ℓ−* − ℓ+*)/2 = (2/3)·r·B` for several `r`
  and `e`.
- **W4 (T7):** `the_partner_orbits_conform_to_the_shear_and_the_range_pays`.
- **W5 (G1, after the build):** in a star system, a rigidly turning disc
  takes no partner spin. A heavier star gives its disc more at the same
  reach. `Σ w·ℓ_R` stands on every partner plane but for the returned grain.

## 5. Open

- **G1 (built; read 2026-11-15).** `conform_partners` runs on a star
  system's motes in `settle_system`, on the system's own enclosure (T8),
  and its grain remainder is counted as the identity "system partners".
  Read: one checkpoint (`.testtmp/catch/l218`) to bell 222, the same tree
  with and without the catch.
  - Every star, world, disc and fuel line is identical.
  - The star's two system motes carry the same partner spins,
    `Σ h·ℓ_⊥ = −20;08·60⁵` in both.
  - The only difference is the payments' grain remainder, `45;50·60⁻⁷`,
    under one grain.
  At this checkpoint the disc is too sparse (one star, two motes) for the
  shear to conform anything. Until a disc with a range of motes is read, the
  law's working is witnessed by W5.
- **G2 (capture's frame).** Register K1: the frame scale `λ = W/g` changes
  as each capture grows the system, so a captured body's spin read back
  depends on when it is read. T6 states who is held; it does not fix `λ`.
- **G3 (the calibre).** The theorems hold on each tier's own ruler.
  Comparing a tier's catch with another tier's (the star's heat against the
  atom's tightness, docs/134 §10.3) needs the calibre between them.
- **G4 (the grain's remainder).** `ρ` of T7 is counted in the star system
  and discarded in the galaxy and the ocean. It must be booked by a law
  (docs/78: no remainder assignments), or shown to be nil on the
  conformation's own grain.
