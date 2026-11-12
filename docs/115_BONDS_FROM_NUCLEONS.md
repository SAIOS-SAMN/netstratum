# 115 — Bonds from nucleons and electrons

Matter bonds through its electrons, and the manifold structure (filaments,
bodies, worlds) emerges from what bonds. This document derives the bond
from laws already in the record: the electrons' count on S^11 (docs/104
§1.1), the sub-atomic phase transitions (docs/100 §9), the fibre and its
grip (docs/97 §2, §13), the closing cluster (`chem::closing`), and
transduction (the burn's law, E25: open frames closed are magnetism made
holding, booked as light). No new constant is introduced.

**Status:** derived (B1–B6); two points open (§7). Not built.

## 0. Where bonds stand now

- Clusters form only inside a world already formed
  (`formation::layer_chemistry`, `formation.rs:2060`).
- The firmament's `flows::bonded` marks collections that move together; it
  makes no cluster.
- Every other mote stays one kind from release to the end of a run.

Structure is therefore read before chemistry, and worlds form one kind
each (the read of 2026-11-03: three worlds, each a single kind). This
document reverses the order: bonds wherever matter meets, structure from
them.

## 1. B1: a bond is a fibre between two atoms

On S^11 a nucleus binds a lone end into a fibre: two counter-turning ends
sharing one whole turn, so exactly one end of a fibre stands on the held
hemisphere whatever the phase (docs/97 §2, docs/104 §1.1). The electrons of
an atom converge the same way: `F_e = Z(Q)` fibres and `L_e = Q(Q) − Z(Q)`
lone ends (`nuclides::electron_ends`, on the branch `law/electron-band`).

A bond is the same fibre made between two atoms: a lone electron end of A
and a lone end of B, counter-turning, one on each hemisphere. So:

- **A bond pairs a held end with a free end.** It never pairs two held or
  two free ends (those are not counter-turning).
- **Atoms bond most in opposite phases:** one with its lone ends on the held
  face, the other with its lone ends on the free face (one gives, one
  takes). Two atoms in the same phase do not bond.

This replaces the product of shares proposed before (`o_A·o_B·in_step`):
the pairing is a count, end for end, and the in-step question is answered by
the fibre itself.

## 2. B2: the pulse phase and the hemisphere

A lone end's hemisphere is read at its mote's pulse phase `p` against the
held face (`φ_held` = 0 forward, ½ on the vacuum leg, docs/104 §1.1). The
pulse phase is the orthogonal orbit's phase against the echo at the mote's
place (docs/97 §13). The stored phases are already read against the bell:
each plane's phase advances by its own turning over the bell's two rings
and the bell's own turn is whole (`flows::torque`), so `p = φ_⊥`.

A lone end stands on the held hemisphere where `in_step(p, φ_held) ≥ ½`
(the hemisphere law: a state crossing its boundary, docs/100 §9, never a
threshold set). All of a mote's nuclei share its phase, so a mote's lone
ends are all held or all free: its held lone ends number `a·L` or 0, `a` its
nuclei.

## 3. B3: how much bonds, in whole clusters

Where a mote A whose lone ends are held meets a mote B whose lone ends are
free (B1), the ends pair one for one into the closing cluster
`chem::closing(w_A, L_A, w_B, L_B)`: `n_A = L_B/g` atoms of A with
`n_B = L_A/g` of B, `g = gcd(L_A, L_B)`, every end paired. The clusters
formed are

    k = min(⌊a_A / n_A⌋, ⌊a_B / n_B⌋)

whole clusters: an integer division of whole nuclei, never a share floored.
What does not fill a whole cluster stays as it was, in its own mote and its
own state: nothing is assigned as a remainder. Nucleons are conserved (each
cluster holds `n_A·w_A + n_B·w_B`).

## 4. B4: the light of a bond (transduction)

The burn's law (E25) reads a reaction by the magnetism it transduces: open
frames closed become holding, `|Δm|`, and the light is the transduced flux;
3Z (holding + magnetism) is conserved. A bond is that transduction at the
electrons (docs/100 §9.1: each frame bound is a quantum of light given):

    light given = k · (n_A·L_A + n_B·L_B) = 2·k·n_A·L_A

each cluster closing every end it pairs. It is booked on the existing light
ledger (`made = received + drained`) at the bell it forms. The cluster's
agitation is its members' holding-weighted mean; it takes none of the light
(the light leaves the bond).

## 5. B5: a bond parts (transmutation, both directions)

A fibre's ends stay one on each hemisphere whatever the phase (B1), so a
bond never parts by phase alone. It parts as a nucleus's fibres part
(docs/100 §9.2): where its heat and its torsion exceed its grip,

    a + τ > κ_c·h_c

`a` the cluster's agitation, `τ` its torsion (E36), `κ_c` its bound share
(its paired ends over all its ends, `2F/ends`; 1 for a closed cluster) and
`h_c` its held count (docs/104 §1.1). It parts whole into its atoms, each
taking the cluster's state; it takes back the light it gave (B4), drawn
from its agitation. One crossing, both directions: below it the bond
stands, past it the atoms are free to bond again.

## 6. B6: what emerges

- A cluster is heavier than its atoms and closed (no lone ends), so its
  push `Q/(A + Q)` falls (docs/104 §1.2) and the existing laws carry it to
  another reach: bonding sorts matter without a sort.
- Contact is the existing chain (motes whose bands overlap on one face,
  `flows::chains`, walked along their sorted bands): bonded matter is the
  filament's filler; collections and bodies form from mixed matter by the
  existing collapse law.
- `formation::layer_chemistry` becomes the same law read inside a world,
  not a separate step.

Measured by a bounded read: clusters formed and of which kinds; each
world's kind mix (`composition_read`); nucleons and 3Z conserved; the light
ledger closing; the identities; the carrier's widths.

## 7. Open

1. **One count of an atom's open ends.** The code holds two:
   `chem::unpaired` (era 0's atom windings, `cfe::almanac::atom`), which sets
   magnetism and the push everywhere, and the electrons' lone ends `L_e` from
   the S^11 convergence (docs/104 §1.1). They agree at 282 of the 1495
   weights of a 2^40 founding (w 80: 5 and 1; w 102: 19 and 21; w 1000:
   130 and 158). docs/97 §3 reads the open frames from the electrons, which
   is `L_e`; changing magnetism to it moves every push. To decide before the
   build.

   **Derived 2026-11-07 from docs/97 §3–§4 and docs/104 §1.1.** The two
   counts carry two roles the docs already assign:
   - **The open frames** (magnetism, the push, the dipole's open share, and
     the ends a bond pairs; restated 2026-11-11 in docs/95 §0: the dipole's
     poles are the nucleons' held and open dimensions `(3Z, 3Q)`, and these
     open dimensions stand beside it over `3w`) are `chem::unpaired`, read from
     `atom(Q)`. docs/97 §4: "the atom, unpaired frames, hold and tightness:
     read from `atom(Q)` as now". `atom(Q)` is the electrons' own harmonic
     atom on S¹¹ (operators 2026-10-18, `cfe::almanac::atom`), so this is
     the reading from the electrons that §3 asks for. It is not era 0's.
   - **The lone ends `L_e`** of the convergence count the electrons' bound
     share, `h = (F_e + L_e·in_step)/Q` (docs/104 §1.1), with R's amplitude
     `κ_e` (docs/97 §3). They are the bound/free state, not the frames.
   
   Bonds therefore pair open frames `U`, and nothing in the code changes.
   Open (issue #128): one set of electrons carries two lone counts that
   differ at 1213 of 1495 weights; whether the two S¹¹ constructions must
   agree is not decided by the docs.
2. **Clusters that stay open.** `chem::closing` pairs every end, so every
   cluster is closed and bonds no further; larger matter would grow only by
   collections, not chemistry. Bonding a cluster that keeps open ends needs
   a kind that records its fibres (`chem::Kind` records parts only).

## 8. Build

1. Decide §7.1.
2. `chem::bond`: B1–B3 for two motes in contact, returning the clusters and
   the light; `chem::parts_where` for B5.
3. In `breath`, along each galaxy's and each system's chains, each bell:
   bonds formed, bonds parted, the light booked.
4. `layer_chemistry` read through the same law.
5. The read of §6.
