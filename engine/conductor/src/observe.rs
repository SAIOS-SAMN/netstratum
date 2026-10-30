//! **Observation** (docs/63) — a world is unrolled only while it is observed.
//!
//! Operator ruling 2026-10-09: the deterministic components of a world (its
//! orbit, its clock) are held in compact state until an observer is present
//! — a mind, a people, or the operator — and at that time the world is
//! unrolled and its stochastic life runs until no observer remains. A world
//! no one observes is its snapshot on disk and its spec in memory; it does
//! not advance (its proper time is its own, and stands while unwatched).
//!
//! Concordance 1.12 (operator, 2026-10-09): "A living people is always
//! observed (its own life reads its world); a world no one reads is carried
//! compact." Every living line is a living people, so every living line is
//! observed and runs. What compacts is a world with no people: one a mind,
//! the operator's watch, the progenitor line or contact names is observed
//! too, so a peopleless world they read is unrolled for them.

use crate::PolitySpec;
use cfe::ratio::Ratio;
use ledger::registry::Registry;
use std::collections::BTreeSet;
use std::path::Path;

/// The world names a takes log seats (`seq epoch seat world mind`).
pub fn seated(takes: &Path) -> BTreeSet<String> {
    std::fs::read_to_string(takes)
        .unwrap_or_default()
        .lines()
        .filter_map(|row| row.split(' ').nth(3).map(str::to_string))
        .filter(|world| world != "the")
        .collect()
}

/// The world names the operator watches: one per line of `watch`.
pub fn watched(watch: &Path) -> BTreeSet<String> {
    std::fs::read_to_string(watch)
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect()
}

/// The observed set among `lines` (every living line, awake or compact):
/// the seated, the watched, the progenitor candidate's, and the nearest
/// line to each of those.
pub fn observed(
    lines: &[&PolitySpec],
    registry: &Registry,
    takes: &Path,
    watch: &Path,
    span: &Ratio,
) -> BTreeSet<String> {
    let mut direct: BTreeSet<String> = seated(takes);
    direct.extend(watched(watch));
    let living: BTreeSet<String> = lines.iter().map(|s| s.civilization.sigil.clone()).collect();
    if let Some((sigil, _)) = crate::largest_lineage(registry, &living) {
        direct.extend(lines.iter().filter(|s| s.civilization.sigil == sigil).map(|s| s.world_name.clone()));
    }
    let direct: BTreeSet<String> = direct.into_iter().filter(|w| lines.iter().any(|s| &s.world_name == w)).collect();
    // A living people reads its own world: every living line is observed.
    let mut out = direct.clone();
    out.extend(lines.iter().map(|s| s.world_name.clone()));
    for line in lines.iter().filter(|s| direct.contains(&s.world_name)) {
        let near = lines
            .iter()
            .filter(|other| other.world_name != line.world_name)
            .map(|other| (crate::spatial_l1(&line.position, &other.position, span), &other.world_name))
            .min();
        if let Some((_, world)) = near {
            out.insert(world.clone());
        }
    }
    out
}
