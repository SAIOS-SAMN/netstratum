//! THE ALMANAC — the PERIODIC law of elements, the locality a star and
//! a planet give it, and the refinement pressure turns into knowledge.
//!
//! `14_ALMANAC.md` rules the element grammar; `dop::element_properties`
//! landed its first half — a family drawn from the star's spine bands,
//! a tier, a richness. What it has no notion of is PERIODICITY: two
//! elements of the same family are unrelated draws, so nothing recurs
//! and nothing can be predicted from position.
//!
//! **FIVE AXES** (operator ruling 2026-08-04: *increase to more axis*).
//! A table of period and group alone is a 2D construct — the same shape
//! struck twice already, at the Merkle root and at the census. An
//! element sits in a cell of FIVE, and each axis DECIDES something no
//! other one can:
//!
//!   LINE  (period)     DEPTH — the stratum, and so the PRESSURE the
//!                      element demands before it refines. 1..=7.
//!   LANE  (group)      CHARACTER — how much it BINDS. Properties RECUR
//!                      along a lane at every line. 1..=8.
//!   SHELL (geometry)   ARITY — how MANY bind at once. This is what
//!                      makes a compound n-ary instead of a pair, and
//!                      it is the axis that reaches the irreducibly
//!                      multi-party case pairs cannot express.
//!   PHASE (state)      THERMAL LOCALITY — which `SubstrateBand` the
//!                      element is workable in. A second locality,
//!                      independent of the star's depth: the star says
//!                      how DEEP, the orbit says how HOT.
//!   SPIN  (chirality)  HANDEDNESS — two compounds of identical
//!                      composition and opposite spin are DIFFERENT
//!                      gems. It distinguishes what nothing else can.
//!
//! Recurrence along the lane is still the load-bearing property — it
//! lets a civilization that has refined one element PREDICT the next,
//! which is what makes discovery cumulative rather than a lottery. The
//! other axes multiply the space that prediction ranges over: the cell
//! count is `LINES · LANES · SHELLS · PHASES · SPINS`, and a compound
//! must agree on shell and phase while saturating on lane, so the
//! reachable gem set grows with every axis rather than with size alone.
//!
//! **EVERYTHING HERE IS A DERIVATION.** Pure, total, exact-rational, no
//! I/O and no deed. It reads an element id, a star and an orb, and
//! returns what they imply. It changes no committed state and no
//! existing behaviour: `element_properties` is untouched and still
//! rules what a prospect yields. This is the reading that has to exist
//! before a refinement deed can be written against it.

use crate::ratio::{ratio, Ratio};
use crate::thermal::StarClass;

/// How many lanes the table carries. A lane is a CHARACTER class and
/// the count is the vocabulary's width, not a tuning knob.
pub const LANES: u8 = 8;
/// How many lines stand. The line is depth; the deepest is the most
/// pressured and the rarest.
pub const LINES: u8 = 7;
/// Binding GEOMETRIES. The shell fixes how many elements enter one
/// compound — arity `1 + shell`, so 2..=5 bodies.
pub const SHELLS: u8 = 4;
/// Workable STATES, one per `SubstrateBand` plus the refractory that
/// works in any of them.
pub const PHASES: u8 = 4;
/// Handedness. Two, and it is not a scale.
pub const SPINS: u8 = 2;

/// Every cell the table carries — the product of all five axes.
pub const CELLS: u32 = (LINES as u32) * (LANES as u32) * (SHELLS as u32) * (PHASES as u32) * (SPINS as u32);

/// WHERE AN ELEMENT SITS, and what that implies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Periodic {
    /// The period — depth. `1..=LINES`.
    pub line: u8,
    /// The group — character. `1..=LANES`. Properties recur here.
    pub lane: u8,
    /// The pressure this element needs before it will refine, rising
    /// with the line. Exact, never a float.
    pub refines_at: Ratio,
    /// How many of its own lane an element BINDS when it compounds —
    /// the lane's recurring valence, identical for every line.
    pub binds: u8,
    /// The binding GEOMETRY. `1..=SHELLS`.
    pub shell: u8,
    /// How many elements enter a compound of this geometry: `1 + shell`,
    /// so 2 bodies at shell 1 and 5 at shell 4. A compound above two is
    /// irreducibly not a pair.
    pub arity: u8,
    /// The workable STATE. `1..=PHASES`; `PHASES` is the refractory,
    /// workable in every band.
    pub phase: u8,
    /// Handedness, `0` or `1`. Not an ordering.
    pub spin: u8,
}

/// Read an element's position. Pure: the same id always reads the same
/// cell, so the table is stable across every world that meets it.
pub fn periodic_of(element: &[u8; 32]) -> Periodic {
    // Bytes 3..=7 are unused by `dop::element_properties` (which reads
    // 1 and 2), so the table is independent of the landed family/tier
    // draw rather than a re-slicing of it. One byte per axis, so no two
    // axes are correlated by construction.
    let line = 1 + (element[3] % LINES);
    let lane = 1 + (element[4] % LANES);
    let shell = 1 + (element[5] % SHELLS);
    let phase = 1 + (element[6] % PHASES);
    let spin = element[7] % SPINS;
    Periodic {
        line,
        lane,
        shell,
        arity: 1 + shell,
        phase,
        spin,
        // Pressure rises with depth: line 1 refines at 1/7, line 7 at 1.
        refines_at: ratio(i64::from(line), i64::from(LINES)),
        // The lane's valence recurs: 1..=4, and lanes 5..=8 mirror
        // 4..=1 — the fold that makes a table periodic rather than a
        // list.
        binds: match lane {
            l if l <= 4 => l,
            l => LANES + 1 - l,
        },
    }
}

/// **LOCALITY** — which lines a WORLD can reach at all.
///
/// A star's class fixes how deep its planets' strata run: a heavy,
/// bright class forges and disperses down to the deep lines; a dim one
/// never reaches them. So the same element id is REACHABLE in one
/// system and absent from another, which is what makes a resource a
/// place rather than a number.
pub fn lines_of(class: StarClass) -> u8 {
    match class {
        // The heavy forges reach every line.
        StarClass::O | StarClass::B => LINES,
        StarClass::A | StarClass::F => 6,
        StarClass::G => 5,
        StarClass::K => 4,
        StarClass::M => 3,
        // The remnants: a collapsed star is the deepest forge there is,
        // and a dwarf the shallowest.
        StarClass::NeutronStar | StarClass::WolfRayet => LINES,
        StarClass::RedGiant | StarClass::WhiteDwarf => 5,
        StarClass::BrownDwarf => 2,
        StarClass::LuminalWhiteHole => LINES,
    }
}

/// Is this element PRESENT in a system of this class? Locality as a
/// predicate: deep lines simply do not occur under a shallow star.
pub fn local_to(element: &[u8; 32], class: StarClass) -> bool {
    periodic_of(element).line <= lines_of(class)
}

/// **THE PRESSURE OF A SYSTEM** — what an orb can bring to bear.
///
/// Pressure is the orb's own depth of working: the stratum it has
/// opened, against the deepest line its star offers. An orb that has
/// opened nothing brings nothing; one working the deepest stratum of a
/// heavy star brings full pressure.
pub fn pressure(stratum: u8, class: StarClass) -> Ratio {
    let reach = lines_of(class);
    match reach {
        0 => ratio(0, 1),
        _ => ratio(i64::from(stratum.min(reach)), i64::from(reach)),
    }
}

/// **REFINEMENT** — does this pressure release this element?
///
/// The element refines when the system brings at least the pressure its
/// line demands. Exact comparison; no tolerance, no float.
pub fn refines(element: &[u8; 32], stratum: u8, class: StarClass) -> bool {
    local_to(element, class) && pressure(stratum, class) >= periodic_of(element).refines_at
}

/// **WORKABLE** — is this element's phase reachable in that band?
///
/// The star says how DEEP a system runs; the orbit says how HOT the orb
/// is. They are independent, and an element needs both: present by line
/// and workable by phase. The refractory phase (`PHASES`) works in every
/// band, which is what keeps a shallow cold world from being barren.
pub fn workable_in(element: &[u8; 32], band: crate::thermal::SubstrateBand) -> bool {
    let phase = periodic_of(element).phase;
    if phase == PHASES {
        return true;
    }
    let wants = match band {
        crate::thermal::SubstrateBand::Cryogenic => 1,
        crate::thermal::SubstrateBand::Mesic => 2,
        crate::thermal::SubstrateBand::HyperThermal => 3,
    };
    phase == wants
}

/// **COMPOSITION** — n elements COMPOUND into a gem.
///
/// Three conditions, one per axis that participates, and the arity is
/// what makes this more than a pair:
///
///   SHELL  every element shares the geometry, and the count entering
///          equals its `arity`. A shell-2 compound takes THREE bodies
///          and is irreducibly not a pair — the multi-party case the
///          pairwise projection cannot express.
///   PHASE  every element is workable in the same state. Matter that
///          cannot coexist does not bond.
///   LANE   the binds SATURATE across the whole set:
///          `Σ binds == arity · (LANES/2 + 1) / 2`, the n-body form of
///          the two-body saturation.
///
/// The rule stays the LANES', not the members' — any set drawn from the
/// same lanes compounds alike whatever their lines, so one recipe names
/// a family of gems.
pub fn compounds(elements: &[[u8; 32]]) -> bool {
    let Some(first) = elements.first().map(periodic_of) else {
        return false;
    };
    if elements.len() != usize::from(first.arity) {
        return false;
    }
    let read: Vec<Periodic> = elements.iter().map(periodic_of).collect();
    if read.iter().any(|p| p.shell != first.shell || p.phase != first.phase) {
        return false;
    }
    let sum: u32 = read.iter().map(|p| u32::from(p.binds)).sum();
    let saturation = u32::from(first.arity) * u32::from(LANES / 2 + 1);
    sum * 2 == saturation
}

/// The gem a set makes — content-addressed, so the SAME set always
/// names the same gem and a "new" gem is new by its identity rather
/// than by anyone's say-so.
///
/// Order-free: the members are sorted before hashing, because a
/// compound is not a sequence. **Spin is NOT order-free** — the set's
/// spin parity enters the digest, so two compounds of identical
/// composition and opposite handedness are different gems. That is the
/// axis earning its place.
pub fn gem_of(elements: &[[u8; 32]]) -> Option<[u8; 32]> {
    if !compounds(elements) {
        return None;
    }
    let mut sorted: Vec<[u8; 32]> = elements.to_vec();
    sorted.sort_unstable();
    let parity: u8 = elements
        .iter()
        .map(|e| periodic_of(e).spin)
        .fold(0u8, |a, b| a ^ b);
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"netstratum.almanac.gem.v2");
    hasher.update(&[parity]);
    for member in &sorted {
        hasher.update(member);
    }
    Some(*hasher.finalize().as_bytes())
}

/// **KNOWLEDGE** — the depth a gem is worth.
///
/// The sum of the members' lines, so a compound of deep elements
/// carries more than a shallow one, and a WIDER compound carries more
/// than a narrow one at the same depth — arity is earned too. No
/// compound, no knowledge: `None`, never a zero.
pub fn knowledge_of(elements: &[[u8; 32]]) -> Option<u16> {
    compounds(elements)
        .then(|| elements.iter().map(|e| u16::from(periodic_of(e).line)).sum())
}

// ─────────────────────────────────────────────────────────────────────
// THE THERMAL CLOSURE
// ─────────────────────────────────────────────────────────────────────

/// **THE THERMAL LINE, CLOSED** (operator ruling 2026-08-04: *thermal
/// is open ended — needs closure*).
///
/// `thermal::band_of` returns `Option<SubstrateBand>` and answers NONE
/// in four places, which the tree's own witnesses already record
/// (`blue_o_star_at_one_unit_exceeds_every_band`,
/// `band_boundaries_are_exact_and_inclusive`): below the cryogenic
/// floor, between cryogenic and mesic, between mesic and hyperthermal,
/// and above hyperthermal with no ceiling at all.
///
/// A `None` is not a temperature. Every orb HAS one, so a reading that
/// declines to name it is a hole in the vocabulary rather than a
/// refusal — and the phase axis above needs a total answer, since an
/// element must be workable SOMEWHERE.
///
/// This closes the line by NAMING the uncovered stretches. It changes
/// no admission: `SubstrateBand` and `band_of` are untouched and still
/// rule siting. The three bands appear here unchanged, with the gaps
/// between them given names instead of silence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThermalRegion {
    /// Below the cryogenic floor. The deepest cold, and a place — not
    /// an absence.
    Quenched,
    /// 10 K – 100 K.
    Cryogenic,
    /// 100 K – 250 K. The gap `band_of` answered NONE for.
    Sublime,
    /// 250 K – 320 K.
    Mesic,
    /// 320 K – 600 K. The second silent gap.
    Torrid,
    /// 600 K – 1500 K.
    HyperThermal,
    /// Above 1500 K and up to the ceiling.
    Incandescent,
    /// **ABOVE THE CEILING — the open end that closure forbids.**
    /// Reachable today because `site_planet` admits ANY positive orbit
    /// and `t_eq4 = L(1-a)k / r²` grows without bound as `r` shrinks.
    /// Naming it does not close it; only an orbit floor does. See
    /// [`ceiling4`].
    Beyond,
}

/// **THE CEILING** — the hottest a lawful siting may reach, given a
/// floor on the orbit.
///
/// There is no ceiling today because there is no orbit floor: the
/// siting law refuses `orbit_r <= 0` and admits everything above it, so
/// `1/10^k` is lawful for any `k` and the temperature it implies has no
/// bound. The T∞ purge barred the reciprocal pole — *infinity is
/// disallowed, the torus wraps finitely* — and this axis still runs to
/// it.
///
/// A second cost rides the same hole, and it is the one this tree has
/// already been burned by: `t_eq4` divides by `r²`, so an orbit of
/// `1/10^k` yields an exact rational with a denominator around
/// `10^(2k)`. That is the DENOMINATOR BLOWUP that killed proportional
/// demurrage at ~2^1100 and forced the flat law.
///
/// Given a floor `r_min`, the ceiling is the brightest class at zero
/// albedo sitting there. Stated as a function rather than a constant
/// because **choosing `r_min` is a ruling, not a derivation.**
pub fn ceiling4(r_min: &Ratio) -> Ratio {
    crate::thermal::t_eq4(
        &crate::thermal::StarClass::O.luminosity(),
        &ratio(0, 1),
        r_min,
    )
}

/// Read the region of a temperature. **TOTAL** — every `t_eq4` names a
/// region, and that is the closure.
pub fn region_of(t_eq4: &Ratio, r_min: &Ratio) -> ThermalRegion {
    use crate::thermal::SubstrateBand as B;
    let (cryo_lo, cryo_hi) = B::Cryogenic.bounds4();
    let (mesic_lo, mesic_hi) = B::Mesic.bounds4();
    let (hyper_lo, hyper_hi) = B::HyperThermal.bounds4();
    match t_eq4 {
        t if *t < cryo_lo => ThermalRegion::Quenched,
        t if *t <= cryo_hi => ThermalRegion::Cryogenic,
        t if *t < mesic_lo => ThermalRegion::Sublime,
        t if *t <= mesic_hi => ThermalRegion::Mesic,
        t if *t < hyper_lo => ThermalRegion::Torrid,
        t if *t <= hyper_hi => ThermalRegion::HyperThermal,
        t if *t <= ceiling4(r_min) => ThermalRegion::Incandescent,
        _ => ThermalRegion::Beyond,
    }
}

/// Is an element workable at this temperature? The total form of
/// [`workable_in`], answering for every orb rather than only for those
/// that happen to land inside a band.
///
/// The refractory phase works in any NAMED region. Nothing is workable
/// `Beyond` — matter above the ceiling is not a place to do chemistry,
/// and that arm exists to be UNREACHABLE once an orbit floor lands.
pub fn workable_at(element: &[u8; 32], t_eq4: &Ratio, r_min: &Ratio) -> bool {
    let region = region_of(t_eq4, r_min);
    if region == ThermalRegion::Beyond {
        return false;
    }
    let phase = periodic_of(element).phase;
    if phase == PHASES {
        return true;
    }
    matches!(
        (phase, region),
        (1, ThermalRegion::Quenched | ThermalRegion::Cryogenic | ThermalRegion::Sublime)
            | (2, ThermalRegion::Mesic)
            | (3, ThermalRegion::Torrid | ThermalRegion::HyperThermal | ThermalRegion::Incandescent)
    )
}
