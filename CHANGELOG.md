# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html). Until 1.0.0, minor versions
may change public APIs, the wire, the ledger and the model's laws; every such
change is named here.

## Unreleased

### Changed

- Mycelium SDK pinned to main 15beb6ea (wire 0x0300), the commit Mycelium's
  standing mesh runs (127.0.0.1:8500–8502); the rc.1 mesh is gone and no
  v0.5.0 tag exists. Contract changes taken in place: every universe act
  (ledger, manifest, map) is signed over its crossing record (X1, #160),
  netstratum's acts carrying none across the tube (`Crossing::still` at the
  act's TUniversal point); tasks read as `TaskFamily`; a local throat's
  sides are `poses`. Read: `ns-hypha laws --at 127.0.0.1:8500` returns the six
  served laws.

### Added

- The plasma layer (docs/134; operator 2026-11-13, "an additional layer of
  physics"): a kind's electrons' bound share `b = g·E(a)/Q` and every
  release shared through it, `ℓ = b·F/w + (1 − b)·S`, `Σ n·w·ℓ` exact,
  replacing the per-kind carry `p·w̄/w` at seven sites. Read on one 2^40
  founding to its first echo, before → after: T_u 186 → 110; the ocean's
  hydrogen 18;27·60^5 grains → none; 106 stars → 35, holding hydrogen 1 → 3.
- The stride (docs/132; operator 2026-11-12, "increase the velocity of the
  entire system without fracturing the equations"): derived. A galaxy's
  laws run only on an echo's arrival (twice in g557's 557 bells); the stride
  advances each law by its own composition up to the next event. Built so
  far: the flow law over n bells (`ring::compose_n`) and the echo's draw as
  probabilities: between echoes there is no push, and across echoes the
  push's mean response at a resonance `a/q` is `mean_response(qψ₀, q·len)/q`,
  at most `1/q`, so the echoes carry no expected spin but at low-order
  resonances; the disc's spin is the transport's and the release's.

- The state on Cl(5,4) over ℚ (docs/131; operator 2026-11-12, "uniform
  phase states as defined by fluid hydromagnetics"): the nine axes, each
  pair's character read from its squares (16 turnings, 20 boosts), the pole
  swap and the carry `M·x·M̃`. A body's orientation holds its motion as the
  product of three commuting factors, the continuum's (the six built
  planes), the vacuum's (the spans' turnings, opposite hand) and the
  scale's (the conformal dilator, flow law), each in closed form; the save
  record carries the vacuum and the scale where they move. The general
  Cayley form on Cl(5,4) measured 43 ms an advance against 0.19 ms, and did
  not finish with boosts in it, so the other boosts act by carry (docs/131
  §9 step 2). Nothing moves the new factors yet: no change to how the
  universe moves.
- The spin the disc needs, read against the spin the bounce gives
  (`observe streams`, docs/128 §14.1): on g557 the ring and the rim
  coincide at 1.256, and the vacuum carries 0.10–0.37 of the spin needed.

- The jetstream's distribution measured (docs/128 §14; operator 2026-11-11,
  zero stars in the home): `observe streams G` reads each stream by side of
  its ring, the throat crossings of one bell's torque and the bands the
  galaxy would bind. On g557 the home's disc stands on the vacuum stream
  inside the core's ring (3,773 of 3,827 motes, median `R/H` 0.06) and binds
  no band. The throat test now reads the R side's passage signed, with its
  windings (`flows::r_side`, `flows::enters_throat`): since the R side is
  turned by the shear either way, the wrapped reading counted forward moves
  as crossings (2,139 against 215 in one bell). The exit across the passage
  was measured and not adopted: every exit landed beyond the galaxy's rim.
- The ablation probe, `observe ablate G [N]` (operator 2026-11-11: "why are
  we not doing localized formula ablations"): each law alone on a galaxy's
  real disc from a checkpoint (its first N motes, N against 4N), with its
  work counters, its invariant's remainder and its effect. Its first runs
  found and fixed: the vacuum stream's face inverted past the luminal bound
  (rates ~900 against ~1); `hold_phase` holding exact readings, so `G`
  carried 75-digit denominators by bell 557 (the scalar collapse; readings
  now on the grain, records read onto it); exact holding sums in the
  magnet, the manifold's twist and the range sums (terms on the grain); and
  `Enclosed::reach_at_rate`'s scan of every step, now a bisection on the
  running least of `H_k/place_k`, the same reach.
- Tilt from quantum torsion (docs/128 §13, EQUATIONS E70; operator
  2026-11-11, approved). Each bell a mote's spins precess about the core's
  wound axis by the twist at its own place (`φ = δ·Σ t_c·t·ν`, falling with
  reach) and tip about the twist's pressure on what its tube holds; the
  spin moved between the planes is the core's antipole's (`S_k + A_k`
  kept), restoring the wound plane's lead that §12 starved. The ring reads
  the chiral split between its halves' axes. With §4b and §12.3 the disc
  settles on a warp, not a flat plane.
- The echo weighed (docs/130, EQUATIONS E69; operator 2026-11-11). At each
  echo's arrival every mote takes a beat: the universal bounce's push
  carried through the layers between (`p·ν`, its nested clock), its
  reverberation between the two rims summed in closed form (`a/(1 − ρ)`,
  `ρ = ¼`), the fluid's drag answering its departure from the medium's beat
  (`Σ w·A` kept), applied to its form as an automorphism about its own spin:
  unitary, its twist and spin unchanged (`flows::echo_beats`,
  `Orientation::beat`).
- The ring in every dimension, and the shear out of the plane (docs/128
  §12, EQUATIONS E68; operator 2026-11-11). Each isoclinic plane is its own
  ring with its capacity `W_k = 3W·α_k`, tube, volume and shear
  (`flows::plane_rings`), and the partner orbits' target is read on each
  plane's own ring. A ring tilted against the rings its tube meets is
  conformed toward their axis, keeping its sense and size, the range paying
  (`flows::conform_axes`): the disc forms by contact, the warp's shear.
  `settle_disc`'s global pull and the target laid by the tilt retire.
- The rulings of 2026-11-11 adopted (docs/129; Mycelium 78.182, operator:
  "if mycelium sends you rulings its because i ruled for mycelium - adopt").
  LAW-COUPLING v2 built: a body's dipole is the push `π = p·δ/w` at its place
  and the bond ends' strength `U/3w` (`flows::dipole`), each isoclinic plane
  coupling at its own reach weighted by `α_p = W_p/3W` (`flows::magnet`), a
  dipole of no nucleons refused `UnbodiedDipole { pole_plane }`; the shear
  reads the same dipole. v1's pole `(Z − Q)/w` (`coupling::pole`,
  `Dimensions::pole`) and fluidity (`Dimensions::open_share`) retire. The
  basket entry, the reserve's faces, the pole-plane names and the traversal
  envelope are stated for netstratum (docs/129 §2–§5).
- The ring with volume, axis and tilt (docs/128 §11, EQUATIONS E67;
  operator 2026-11-11). A half's motion sweeps a solid torus on S³: its
  tube's width (the face) and content (the holding it encloses, which the
  shear reads), its axis in space (its spin carried by its half) and its
  tilt against the manifold's invariable axis, read from the product of the
  axes (inclination share, sense, line of nodes). Readings, exact
  (`strip::half_rings`, `strip::invariable_axes`, `orientation::tilt`).
- Drag's orthogonal axis (docs/128 §4b; operator 2026-11-11). Across the
  traversal, on the ring's axis, the medium's pressure repels the ring's
  crossing at the inverse velocity of pressure, `M(b_P) = −1/b_P` (share
  `1 − P`), by the same drag law that acts along the traversal; the crossing
  is kept by the exchange. Readings (`strip::ring_pressure`,
  `strip::along_and_crossing`).
- Each half moves at its ring's own transit (docs/128 §9.6,
  `flows::ring_motions`): the face from its tube, the twist's pressure, and
  drag on both axes (the counterflow along, the repulsion across), read for
  the manifold at the bell's start. The crossing's loss turns a ring toward
  the invariable plane. Each half reads its transit on the face it stands
  on, so the stream flag no longer sets the deficit and no transit passes ½
  (the first build inverted the vacuum's face and broke the luminal bound;
  found by the ablation probe).
- The unitary morphodynamic state (docs/128 §10, EQUATIONS E66; operator
  2026-11-11). A body's turning state factors exactly into its form, an
  automorphism holding R (its orbits), and its twist, one isoclinic turning
  against R, both unit quaternions (`Orientation::form_twist`). The R side
  is orbits sheared by the manifold, never a driven loop.
- The bell on the group (docs/128 §9, EQUATIONS E65; operator 2026-11-11,
  "rewrite entirely in non-euclidean space"). A mote carries its spin on all
  six planes of its orientation (`Mote::spins_orth`, three partner spins in
  place of one loop spin). Each half of the spin moves by one Cayley vector
  a ring at its own reach and rate, composed on S³ × S³; the tilt's
  per-plane turning, the loop laid on every partner alike and §8 step 3's
  per-plane reach retire. The bond test (`flows::collections`) decides
  pairs on certified whole-number bounds, with a work counter.
- Drag, the jetstream's counterflow (docs/128 §4a, EQUATIONS E64; operator
  2026-11-11). The forward traversal's rate read through the strip's pass is
  `−1/b`, so drag is the same jetstream on the other face, share `1 − s`. It
  takes from the parcel's departure from the medium's transit by the
  medium's crowding and gives it to the medium, keeping `Σ w·σ·s`. Readings
  in `conductor::strip` with their witnesses.
- `conductor::strip`, the pass per plane (docs/128, EQUATIONS E63; operator
  2026-11-11). Each plane's own velocity on its own circle, its face as the
  share of the parcel across the ring (continuous, one per plane), the
  transit's share and sense, and the medium's pressure resisting before the
  ring and aiding past it. Readings with their witnesses; the laws follow.
- docs/127, the manifold deformed, conformed and restored (operator
  2026-11-11; EQUATIONS E59–E62, register K16). Exuded properties are the
  manifold's deformation, never a body's holding. What restoration removes is
  dissipated into the manifold, radiated at the place's unlocked share and
  recycled at its held share to the bodies conforming there, so the energy
  book closes by construction. Its history: the half twist's read exposed
  heat leaking at every body boundary.
- docs/108 §1a, gaps against what is measured, with G1 Kepler's third law:
  `ring::period` reads `P ∝ R` far out, where the solar system measures
  `P ∝ R^{3/2}`. It is open physics, tied to the per-plane reach and the
  manifold's deformation. docs/78's design law 2 now carries the rationale
  for Netstratum's own first-degree rulers, with no claim that
  inverse-square physics is wrong.
- docs/125, the equations at their base: every era-1 equation sorted by the
  side of matter it acts on (proton, neutron, electron, nucleons, none) and
  by whether it stands at its base under the nucleon inversion (docs/97,
  docs/104 `I`, docs/95 §0): 17 at base, 34 not, by cause, three points the
  record disagrees with itself on, and the order of derivation.
- `conductor::orientation` (#117 step 2a, docs/117 §11.7, EQUATIONS E57):
  the orientation held as the generator `G` over six elliptic planes with
  windings `K` per plane; the rate a ring `b = δ/(1 − δ)`; the whole
  advance through `conductor::turning`; the L1 reading `g/(1 + |g|)` the
  laws act on. Not yet carried by motes or bodies.
- `conductor::turning` (#117 step 1, docs/117 §11.2–§11.3): the exact
  Clifford value over ℚ with the signature as data, and the Cayley motion
  `R = (1 + B)(1 − B)⁻¹`, its generator and the composition `R₁R₂`. An
  interim mirror of Mycelium's FORMS-TURNING v1, conformed by its fixtures
  (11b-i to 11b-iii, the one-plane closed forms, the refusals) value for
  value, to be replaced by the import at Mycelium v0.4.0.
- `conductor::dispersion` (#143, read only): heat by component, the
  dispersion of a body's parts about its motion in step on each plane's
  phase, each spatial plane's spin, between planes and on the orthogonal
  orbit, by kind on the phases, never folded into one number; `breathe`
  prints it for every collapsing binding (`heat by component`). docs/124 §3
  restates heat by component.
- `conductor::energy`, the energy book read only (docs/122 step 1,
  docs/106 S1): heat (motes, stars, worlds), potential `m·(1 − δ)` and bond
  (open frames) over every mote and body, and an `energy` line per
  universal echo and over the run in `breathe`, beside the light made,
  received and drained. Witnesses: reading leaves the universe as it was; a
  mote split in two leaves the book as it was; a fusion pair lowers the bond
  by its rung light. Read (2^40, 580 bells): heat 0 → 0.0265 unit masses,
  1.2 % of it light received, the rest unbooked; potential 0 → 0.123; the
  bond rose 39 % while 0.0037 of light was made.
- docs/124, States and phases (derived, not built): each form of energy
  on one level of the coordinate and named by nature's term (register
  H12): motion and heat on the phase level (heat is motion out of step, the
  spread of a body's turning, answering docs/120 §9 item 5), bond energy on
  the quantum level, potential on the state level, light in transit on no
  lane, the sink where it drains. States of matter (molecular, atomic gas,
  plasma) read heat against bonds. The star core's heat is derived from
  contraction (Kelvin–Helmholtz), added to docs/122 as build step 5; it
  retires the constant heats at ignition and at the outburst re-cut.
  GLOSSARY: heat, temperature, state of matter, transition, emission,
  absorption and irradiance, sink, contraction heating.
- `horizons::read` and a `horizon` line per echo in `breathe` (docs/117
  §8–§9, readings only): for each galaxy mote, which planes stand inside
  their horizons (`|R·t_k| < H`), against the whole enclosure's hold and each
  plane's own; where a turning plane is inside while the whole is outside
  (where the passage per plane would move a mote the built luminal bound
  does not); tilt ties; and where the two holds disagree. Read (2^40, 580
  bells): the universe unchanged; every mote's three planes inside; the
  whole outside with a plane inside for 21, 49 and 89 motes at the three
  echoes (about 4%); no ties; the holds never disagree.
- docs/117: the seven base axes and their signature (each frame plane
  hyperbolic, `Z² = +1`, `Q² = −1`; LAW-DILATION is that plane's boost),
  degrees counted on many axes, and the tower `S³ → S⁷ → S⁴` with
  `S¹¹ = S⁷ ∗ S³`: the machine on S⁷ and the quantum state riding on it.
  The nuclide table's degrees on S¹¹ split exactly over the machine's and
  the state's (witness: `the_degrees_on_s11_split_over_the_base_s7_and_the_state_s3`).
- docs/114 §9 (T6): the passage through a throat derived as a quarter turn,
  left multiplication by a unit quaternion on the body's state `q = H +
  R·(t_x·i + t_y·j + t_z·k)`. It is an L1 isometry, exact over ℚ, of order
  four and non-abelian (Q₈). Both reach maps (`2H − R`, `H²/R`) and the
  scalar Möbius reading are ruled out (operator 2026-11-05). Derived, not
  built; open points in §9.6. docs/113 §4 now points to it.
- The founding is oriented (docs/100 §4.1; operator 2026-11-04): the
  universal core is founded a grain off isotropic, the seed choosing which
  plane leads and which lags (`pulse::orient`; `pulse … found <grains>
  [seed phrase]`). Until now every law and the founding treated the three
  planes alike, so they carried one disc's state three times. Read: the lean
  grows from 1 grain to 910 by the third bounce, and 1,930 of 2,246 motes
  stand apart in phase; a seed leaning other planes reads the same universe
  permuted. `breathe` prints the lean at each echo.
- The front's periods read live at each universal echo (E51–E53): the
  contained front per plane per cell, the rim `R_rim = H(<R_rim)`, and the
  established period `T_u = 4·R_rim`, so `T_u/4` is the rim (docs/112 §7.1
  closed). Read: `T_u` 4.54 and 5.00 bells at the first two echoes against
  the stored 186. The schedule does not use them yet.
- docs/116 and EQUATIONS §8.5 (E47–E50): every placeholder in PHYSICS §5
  read in its code and deduced where a law closes (the isotropic body,
  compounding for `ln 2`, the tier shares retired, the bell's years and
  kelvin read on own rulers), with what stands, what needs a read and what
  does not close.
- docs/115 and EQUATIONS §8.4 (E40–E44): bonds from nucleons and electrons,
  derived from the electrons' count, the phase transitions, the fibre and
  transduction: a bond pairs a held lone end with a free one, whole
  clusters by integer count, the light of a bond, a bond parting by heat
  and torsion against its grip. Open: one count of open ends (two in the
  code, agreeing at 282 of 1495 weights); clusters that stay open.
- docs/114 and EQUATIONS §8.2 (E31–E35): the throat as a tube's connector
  (the face inversion as `x ↦ 1/x`, a tube between mouths, chained tubes
  composing by LAW-DILATION, the Kaluza–Klein passage on the R loop, an
  open tube), with `ring::{inverted, tube, tube_share, dilation, compose}`
  and their witnesses; the conflict with the compact sphere's passage
  stated (docs/114 §2).
- docs/113: the crossings at the universal core stated as Mycelium
  transitions for the court to rebuild (the throat's release and return as
  fluxes, the motes they carry as bodies); inner steps stay Netstratum's.
- `docs/PLAN.md`: the work in order under the v0.1.0 and v0.2.0
  milestones, each step an issue (#27–#38 filed for the consolidation, the
  Mycelium integration and the repository's organization).
- `docs/MYCELIUM.md`: what Netstratum takes from Mycelium, where each use
  runs and how complete it is, the defects (verification only as a
  diagnostic, unmeasured laws counted as agreeing, `tongue` half-wired), and
  the line between the two projects' physics. docs/109 A6 corrected to
  partial.
- `docs/PHYSICS.md`: the engines, the era-1 live path, the quantities
  computed in more than one place, the values not yet derived, the era-0
  bridges, and a ledger of what the engine's reads have established;
  EQUATIONS §8.1 adds the live-path laws E12–E30.
- The open-source layout: `AGENTS.md` (with a `CLAUDE.md` pointer),
  `MAINTAINERS.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, this changelog, and
  `scripts/check` (fmt, clippy `-D warnings`, tests, rustdoc `-D warnings`,
  `scripts/check-docs`).
- The founding with the whole table of elements in the universal core's
  throat (docs/112 F1), released inside the throat (F2), the band as an
  attractor (F3) and `τ` from the table's books (F5).
- Contract review: changes touching a shared contract are labelled
  `cross-project` and reviewed by Mycelium's maintainer, the coordinator
  (MAINTAINERS.md, CONTRIBUTING.md "Contracts").
- Getting involved: a README section, issue templates (defect, proposal,
  track) and a pull request template; open work tracked as GitHub issues
  under the v0.1.0 milestone.
- The universal core's band on the inverse faces (docs/104 §6).
- The firmament's collective: bonds, collections moving as one, its books
  and last phases saved (docs/111 W2, W3).
- The front's formulas: arrival, density, throat, surface share and
  strength (`conductor::ring`, docs/111 W1).
- Attachment of external agents through the hypha (docs/109) and the shell
  law (docs/110).

### Changed

- The torque turns each plane's pair on its own circle (docs/128 §8,
  EQUATIONS E63). Motes carry the orientation `G`; each plane's form
  (`strip::PlaneForm`) is its turning pair, its axis's poles and its reach
  and face. The plane turns `2·s_k′` a bell at its own share on its own face,
  resisted by the core's twist pressure, and its R partner turns with it.
  This retires the tilt's `2δ·t_k`. `flows::torque` takes the twist pressure.
- Heat recycled across the manifold's dissipation (#182, docs/127 §4,
  EQUATIONS E62). A mote's restored heat enters its enclosure's manifold,
  which recycles the held share `1 − δ` at its place to the motes whose own
  enclosures meet its own, by holding times held share, and radiates the
  rest as light. `flows::emit` takes the enclosure's holding unit.
- Heat travels with the matter that carries it (docs/126 §3a, docs/124 §3).
  Matter leaving a star (outburst, shed layers, dissolutions, infall), motes
  escaping a system and worlds crossing as motes carry their source's heat
  per holding (`formation::star_heat`, `held_heat`, `world_heat`), never 0.
  Infall mixes into the star by holding (`mixed_heat`).
- The vacuum as a Möbius strip (docs/126, EQUATIONS E37, E37a; operator
  2026-11-11). The pass carries the strip's half twist, each plane `½·t_k`
  by the mote's lead, in place of a flat ½ on every plane. The core's twist
  pressure (`torsion::core_twist`, one reading) releases on the continuum
  and consumes, reversed, on the vacuum face at the throat, where recycling
  deconstructs `E/(S + E) ⊕ (−P₁) ⊕ (−P₂) ⊕ (−P₃)`.
- A body's push oriented on the open pole, with its antipole (docs/125 §4.1,
  EQUATIONS E16): `p = Q·(1 − g)/w`, the protons' charge left open at the
  body's place, `p̄ = 1 − p` on the vacuum leg; no `½`, replacing `½·U/w`. The
  shear reads a vacuum mote's dipole on its inverted face, as the magnet does
  (E58).
- The nucleons weighted as they are (#176, docs/95 §0, EQUATIONS E58,
  register K15): `chem::Dimensions` is the nucleon dipole, the protons'
  open dimensions `3Q` and the neutrons' held dimensions `3Z` over `3w`, with
  the electrons' open dimensions `U` beside it ("frame" retired, mycelium
  #239). It replaces `(3·protons − U, U)`, which counted the
  protons as "3Z", put the electrons on the Q pole and dropped the neutrons.
  The dipole coupling's pole is `(Z − Q)/w` and its open share `U/3w`; the
  magnet and the throat's base frames read the same.
- Mycelium pinned at v0.4.0-rc.1 (#163; wire 0x0200, law version 5), a
  contract change. The uplink speaks the v0.4.0 SDK:
  - samples on the alphabet's section (`rule::SectionSample`, `TaskSection`)
    in place of rasters, admitted at the boundary;
  - the moved paths (`voice::VoiceKey`, `laws::`, `rule::Mv`,
    `house::membrane_at`, `Step::symmetry_motion`, `Coord::Cells`,
    `Symmetry::RotateE2E1By*`);
  - `TierFold::transitions`, and the court's `Boundary` refusals named;
  - the contract's step 0 signs the served clause register by reconstruction
    (ACT_LAWS mode 8) and reads the key's alignment (mode 9, a MUST act of
    the KERNEL profile), in place of the retired Concordance hash.
- The energy book (`conductor::energy`) sums its terms in a balanced tree:
  the same exact values, without the running sum whose denominator grew
  with every mote (amounts carry wide denominators). At bell 186 on the
  2^40 founding, the read fell from 15 s of the echo's 26 s to under 1 s.
- Motes carry the orientation `G` (#117 step 2b, #157; docs/117 §11.7):
  `Mote::phase` is retired, its phases now readings of `G`. The bell's two
  rings compose whole at `b = δ/(1 − δ)` a plane, and the swing is read pair
  by pair, so a plane passes through the centre alone. The mote record holds
  twelve values, and a record of per-plane phases is refused. Stars and
  worlds follow (#159).
- H's state is the generator (docs/117 §11.7, derived, not built; amends
  docs/120 §8d): a mote's and a body's orientation is held as the generator
  `G` over six elliptic planes with `K` per plane, its rate a ring the hold
  over the reach, `b = H/R`, and each plane's phase a reading. Register K6's
  split is retired: the composition makes it.
- Friction is booked (docs/122 law 2, step 4; EQUATIONS E20, E56): the
  torque's undelivered share is paid by the push (the bounce carries on
  `p − Σ m·|Δℓ| − heated`); an encounter's heat is the relative motion its
  grip ends, the pair's first-degree spread about their mean turning, and a
  merged star's heat its members' whole spread (the deficit contrast stays
  the stick test only); the ocean's torque heat is a named source.
  `breathe`'s energy line names every heat source and the heat still
  unaccounted.
- Bonds release into heat (docs/122 law 1, step 3; EQUATIONS E55): at the
  throat the peels draw on the bounce's work first and then on the press's
  release, and what the release has left warms the throat's matter (its light was spent on peels and shone
  whole, used twice); trapped matter carries its heat into the container.
  A star's burn warms the star, which emits from its system's throat, ½ a
  ring (`formation::star_glow`): one starlight law, `formation::burn`'s
  light retired (B9).
- Heat leaves by emission through the face (docs/122 law 3, step 2;
  EQUATIONS E54): a body keeps its held face `g` of its heat a ring and
  emits the rest as booked light, every kind by where it stands (hydrogen
  emits). The torque only makes heat; galaxy and ocean motes, a system's
  motes and its worlds emit, and the light is shone on the continuum face.
  Retired: `keep = (1 − open)(1 − δ)` and `formation::radiates`; the gas
  shell record moves from tag 6 (refused) to tag 31. Read (2^40, 580 bells,
  same founding): light made 0.0037 → 0.110 unit masses; stars born 26 →
  11, holding hydrogen 1 of 21 → 8 of 9; hydrogen burned 0 in both.
- The read binaries' line tags are lowercase (`energy at bell …`,
  `light — …`, `identity torque — …`), never capitals: a tag names a
  reading, and capitals read as a constant. Every line of `breathe`,
  `observe`, `pulse`, `life`, `atoms`, `witness` and the conductor follows;
  `observe`'s journal filter matches the conductor's tags after the beat
  (`beat N: breath`). STAMP, an acronym, and environment variable names
  keep their capitals.
- Names follow nature's terminology (register H12): "heart" is now the
  **star core** (astronomy's stellar core) in code and docs:
  `formation::heart_cut` → `formation::star_core_cut`; the `observe`
  commands `hearts` and `heartbeat` → `star-cores` and `burn`; the
  conductor's "heartbeat" → its `beat`. No law changes.
- Bounces are cheaper, the universe unchanged to every reading (#5):
  capture carries each star's mass forward instead of re-reading every
  holding per captured mote; the peel walks its rungs on whole numbers; the
  collective's bonds are screened with a certified floating-point margin
  before the exact test. Read (2^40, 580 bells): three bounces in 2 min 7 s
  (3 min 14 s before); at the third echo capture from 32 s to about 6 s,
  the peel out of the profile, the collections 1.9 s.
- Mycelium pinned at `v0.3.0` (the rc.3 commit), spoken through `mycelium-sdk` only
  (#10). The minds still perceive rasters; tasks and solve outputs cross
  the wire as samples, and a task of another rank or width is refused by
  name. Solve receipts carry μ as exact shares (`Mu`).
- The re-founding speaks Mycelium's KERNEL profile whole: step 0 reads the
  served statement and the Concordance's hash against the pinned SDK's and
  signs the hash; the universe is read back after its origin; the
  universe's manifest (its nuclei per Z, `pulse::nuclei_by_z`) is posted
  under the sitting reserve. `ns_hypha::SPOKEN` declares the acts spoken;
  Mycelium's in-process conformance read passes.
- No plane is favoured by its index (docs/78, no fill orders): a tilt's
  rounding grains return to the planes that lost most, not the last plane;
  a star's pull reads every plane by its tilt, not the first; a binding's
  arcs are read in every plane tied for the disc; a core with nothing
  turning keeps its tilt. Unchanged while the planes are alike (control
  read: the same universe to every count).
- The galaxy's band pairs read their sums certified: the strip's step and
  the crest's count of bindings each from narrow bounds on their sums,
  with the exact sum only where the bounds decide differently. The same
  outcomes (witnessed); the third bounce of a fresh 2^40 universe arrives
  at 187 s against 272 s. `breathe` reports each step's time per echo
  under `NETSTRATUM_PROFILE` (#5).
- The firmament's collective push (`flows::repel`) sums its pairs exactly on
  the phases' common 60-adic denominator instead of term by term in reduced
  rationals: the same values (witnessed), and a fresh 2^40 universe now
  reaches its third bounce (#5).
- A world's composition handed to era 0 is keyed by proton count (it was
  the nucleon count, which era 0 read as Z); clusters are split among
  their atoms' elements by nucleons instead of dropped (#27).
- docs/114 T4 and E34: the passage restated as an event on the orthogonal
  orbit (sheared across the axes, forward in time). The Kaluza–Klein
  reading is withdrawn: R is not compact.
- The workspace is formatted with rustfmt and clean under clippy
  `-D warnings`; `type_complexity`, `too_many_arguments` and
  `needless_range_loop` are allowed workspace-wide.

### API changes

- Mycelium pinned at `v0.3.0-rc.1` (was `v0.2.0`): `sdk::Kernel` is
  `sdk::Voice`; `HyphaKey::vk_bytes` is `verifying_bytes`;
  `forms::xylarium` is `forms::shares`. A seat's role is Mycelium's opaque
  tier, and Netstratum names its tiers in `ns_hypha::tiers` (progenitor 0,
  ancients 1, steward 2, sovereign 3, the bytes unchanged). Mycelium's task
  addresses change at this release; saved test data carrying old addresses
  is re-founded, never migrated.

### Layout changes

- `docs/STATUS.md` moved to `ROADMAP.md`, `docs/CONCEPTS.md` to
  `GLOSSARY.md`.
- The era-1 save gains tags 28 (the founded table's holding) and 29 (the
  firmament's last phases).
