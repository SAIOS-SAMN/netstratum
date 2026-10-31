//! The axes — where an element sits.
//!
//! Five, and each decides something no other can. An axis that changes
//! no outcome is decoration.
//!
//!   LINE  (period)     DEPTH      -> the pressure it demands to refine
//!   lane  (group)      character  -> how much it binds; recurs at
//!                                    every line
//!   SHELL (geometry)   ARITY      -> how many bind at once
//!   phase (state)      thermal    -> which region it is workable in
//!   spin  (chirality)  handedness -> which gem a composition names
//!
//! Recurrence along the lane is the load-bearing property: a lane's
//! character depends on the lane alone, so a civilization that has
//! refined one element can predict the next. That is what makes
//! discovery cumulative rather than a lottery.

use crate::ratio::{ratio, Ratio};

/// **The LANES — widened from 8 to 10** (operator ruling 2026-08-04:
/// *widen the lanes*), and the width is derived rather than preferred.
///
/// Saturation for an n-body compound is `arity · (LANES/2 + 1) / 2`, so
/// it is a whole number for every arity exactly when `LANES/2 + 1` is
/// even — that is, when **`LANES ≡ 2 (mod 4)`**. At 8 the figure is
/// `arity · 5 / 2`, whole only for even arities, which is why the
/// previous landing had to record that no ternary compound could
/// saturate. At 10 it is `arity · 3`, whole for all of them.
///
/// ```text
/// LANES   a=2    a=3     a=4    a=5
///     8     5   15/2      10   25/2      odd arities impossible
///    10     6      9      12     15      every arity reachable
///    12     7   21/2      14   35/2      the same hole returns
/// ```
///
/// 10 is the smallest widening that opens the odd arities, and the
/// three-body compound is the one that matters: it is the irreducibly
/// multi-party case a pairwise projection cannot express.
pub const LANES: u8 = 10;
/// How many lines stand. The line is depth; the deepest is the most
/// pressured and the rarest.
pub const LINES: u8 = 7;
/// Binding geometries. The shell fixes how many elements enter one
/// compound — arity `1 + shell`, so 2..=5 bodies.
pub const SHELLS: u8 = 4;
/// Workable states, one per thermal region family plus the refractory
/// that works in any of them.
pub const PHASES: u8 = 4;
/// Handedness. Two, and it is not a scale.
pub const SPINS: u8 = 2;

/// Every cell the table carries — the product of all five axes.
pub const CELLS: u32 =
    (LINES as u32) * (LANES as u32) * (SHELLS as u32) * (PHASES as u32) * (SPINS as u32);

/// Where an element sits, and what that implies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Periodic {
    /// The period — depth. `1..=LINES`.
    pub line: u8,
    /// The group — character. `1..=LANES`. Properties recur here.
    pub lane: u8,
    /// The pressure this element needs before it will refine, rising
    /// with the line. Exact, never a float.
    pub refines_at: Ratio,
    /// How many of its own lane an element binds when it compounds —
    /// the lane's recurring valence, identical for every line.
    pub binds: u8,
    /// The binding geometry. `1..=SHELLS`.
    pub shell: u8,
    /// How many elements enter a compound of this geometry: `1 + shell`.
    /// A compound above two is irreducibly not a pair.
    pub arity: u8,
    /// The workable STATE. `1..=PHASES`; `PHASES` is the refractory.
    pub phase: u8,
    /// Handedness, `0` or `1`. Not an ordering.
    pub spin: u8,
}

/// Read an element's position. Pure: the same id always reads the same
/// cell, so the table is stable across every world that meets it.
pub fn periodic_of(element: &[u8; 32]) -> Periodic {
    // Era 1 (2026-10-15): a canonical element's cell is READ from its atom
    // (docs/78, the atom tier), never drawn from its bytes.
    if let Some(z) = super::atom::z_of(element) {
        if let Some(cell) = atom_cell(z) {
            return cell;
        }
    }
    // Bytes 3..=7, one per axis, so no two axes are correlated by
    // construction. Bytes 1 and 2 belong to `dop::element_properties`
    // and are untouched — this table is independent of that draw rather
    // than a re-slicing of it.
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
        refines_at: ratio(i64::from(line), i64::from(LINES)),
        // The valence folds at the half: 1..=LANES/2 rising, then back
        // down. The fold is what makes the table periodic rather than a
        // list.
        binds: match lane {
            l if l <= LANES / 2 => l,
            l => LANES + 1 - l,
        },
    }
}

/// An atom's cell on the five axes: line its depth (the deepest lines folded
/// into the last), lane its unpaired count (the bonding half of the fold),
/// shell its outer band's electrons (one to four), phase where its contact
/// grip (`τ · 2,760 K`, life's contact anchor) holds against the thermal
/// bands, spin the parity of Z.
fn atom_cell(z: u32) -> Option<Periodic> {
    let a = super::atom::atom(z)?;
    let line = a.depth.clamp(1, u32::from(LINES)) as u8;
    let lane = a.unpaired.clamp(1, u32::from(LANES / 2)) as u8;
    let shell = a.outer.clamp(1, u32::from(SHELLS)) as u8;
    let grip = a.tightness.clone() * ratio(2760, 1);
    let phase = if grip < ratio(250, 1) {
        1
    } else if grip < ratio(600, 1) {
        2
    } else if grip <= ratio(2273, 1) {
        3
    } else {
        PHASES
    };
    Some(Periodic {
        line,
        lane,
        shell,
        arity: 1 + shell,
        phase,
        spin: (z % 2) as u8,
        refines_at: ratio(i64::from(line), i64::from(LINES)),
        binds: lane,
    })
}

/// **The MATTER law's digest** (docs/58 §3) — what a universe's Big Bang
/// records of its matter: the periodic table's five axes, the resource
/// ledger's grain, the base resources by wire code and the star classes by
/// wire code. Two universes carry the same digest exactly when their matter
/// obeys the same law; a change to any axis is a different physics.
pub fn matter_law_digest() -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key("netstratum matter law v1");
    h.update(&[LINES, LANES, SHELLS, PHASES, SPINS]);
    h.update(&crate::world::RESOURCE_LEDGER_GRAIN.to_le_bytes());
    for code in 1u8..=u8::MAX {
        if crate::dop::Resource::from_wire(code).is_some() {
            h.update(&[b'R', code]);
        }
        if crate::thermal::StarClass::from_wire(code).is_some() {
            h.update(&[b'S', code]);
        }
    }
    *h.finalize().as_bytes()
}
