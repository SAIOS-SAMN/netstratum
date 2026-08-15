//! **THE THERMODYNAMIC CARRYING CAPACITY** (COG Slice 2b) — the population
//! ceiling K is derived from the star's continuous insolation `I = L(1−A)/r²`
//! over the planet's arable area, gated by the sitting clade's biological
//! efficiency `η = habitability`. This pins the physics the operator ruled:
//! the star's raw energy sets the food ceiling, the species' fit to the
//! thermal band decides how much of it becomes food, and the "habitable zone"
//! is therefore RELATIONAL — a star×species intersection, not a spatial ring.

use cfe::dop::{positive, unit};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::World;

const FIRST_ORB: u64 = cfe::world::FIRST_ORB;

/// A founded, star-lit, sited, taxon-sealed world — the minimal seat a
/// population ceiling reads.
fn seat(name: &str, star: StarClass, albedo: (i64, i64), orbit: (i64, i64), clade: u16) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(star).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(albedo.0, albedo.1), positive(orbit.0, orbit.1), None, None)
        .expect("sites");
    world.seal_taxon(2, clade).expect("seals taxon");
    world
}

#[test]
fn insolation_ranks_by_star_and_orbit() {
    // Raw physics, no bucket: a luminous star delivers more flux than a dim
    // one at the same orbit, and a closer orbit more than a far one.
    let g = seat("g", StarClass::G, (0, 1), (1, 1), 5);
    let o = seat("o", StarClass::O, (0, 1), (1, 1), 5);
    let m = seat("m", StarClass::M, (0, 1), (1, 1), 5);
    assert!(
        o.insolation_of(FIRST_ORB) > g.insolation_of(FIRST_ORB),
        "an O-giant outshines the Sun: {} vs {}",
        o.insolation_of(FIRST_ORB),
        g.insolation_of(FIRST_ORB)
    );
    assert!(
        g.insolation_of(FIRST_ORB) > m.insolation_of(FIRST_ORB),
        "the Sun outshines an M-dwarf"
    );
    // The G-star unit seat normalizes to ≈1 (albedo 0, r=1).
    assert_eq!(g.insolation_norm(FIRST_ORB), ratio(1, 1), "G at r=1, A=0 is the reference unit");

    // A close orbit beats a far one for the same star.
    let g_far = seat("gf", StarClass::G, (0, 1), (4, 1), 5);
    assert!(
        g.insolation_of(FIRST_ORB) > g_far.insolation_of(FIRST_ORB),
        "a close orbit catches more light than a far one"
    );
}

#[test]
fn the_habitable_zone_is_relational_not_a_spatial_ring() {
    // The SAME planet+star yields a living ceiling for a matched clade and a
    // DEAD one for a mismatched clade — the star shines the same; only the
    // species' efficiency differs. This is the whole point: a "good planet"
    // exists only relative to who is trying to eat its light.
    let matched = seat("temperate-matched", StarClass::G, (0, 1), (1, 1), 5); // Mesic clade on a Mesic world
    let k_matched = matched.population_ceiling(FIRST_ORB);
    assert!(k_matched > ratio(0, 1), "a matched clade has a living ceiling: {k_matched}");

    // A HyperThermal-preferred clade (13) on the same temperate world: its
    // band is wrong, η collapses, the ceiling dies — it freezes under a sun
    // that feeds the flatlander fine.
    let mismatched = seat("temperate-mismatched", StarClass::G, (0, 1), (1, 1), 13);
    let k_mismatched = mismatched.population_ceiling(FIRST_ORB);
    assert!(
        k_mismatched < k_matched,
        "a mismatched clade starves on the same world: {k_mismatched} < {k_matched}"
    );
}

#[test]
fn a_bright_star_hosts_a_high_energy_niche_a_dim_dwarf_cannot() {
    // The operator's law made physical: K is the star's raw energy filtered
    // through the species' efficiency. A hot-preferred clade (13) MATCHED to
    // its HyperThermal band (η=1) around a luminous O-giant supports an
    // enormous ceiling — the brilliant star delivers a torrent of flux to a
    // species built to drink it. An M-dwarf simply cannot make a hot world at
    // any orbit ≥ 1: it is too dim to reach the band at all, so the same hot
    // niche is UNREACHABLE around it (the "M-dwarf bottleneck"). Both seats
    // share a founding name, so their hash-derived arable area is identical
    // and only the star differs.
    let bright = probe_matched_ceiling("niche", StarClass::O)
        .expect("a luminous O-giant reaches the hot band");
    assert!(bright > ratio(0, 1), "the matched hot niche has a vast ceiling: {bright}");
    assert!(
        probe_matched_ceiling("niche", StarClass::M).is_none(),
        "a dim M-dwarf is too faint to host a hot-preferred species at any orbit"
    );

    // And where the hot clade is STRANDED around the dim dwarf (forced to the
    // closest orbit, still the wrong band), its ceiling is a fraction of the
    // bright-star niche it was built for — energy starvation AND thermal
    // mismatch compound.
    let stranded = seat("niche", StarClass::M, (0, 1), (1, 1), 13).population_ceiling(FIRST_ORB);
    assert!(
        bright > stranded,
        "the bright, matched niche dwarfs the dim, stranded one: {bright} > {stranded}"
    );
}

/// Seat a hot-preferred clade (13, HyperThermal) MATCHED to the band its own
/// star+orbit produces, and read the resulting ceiling. Sweeps orbit until
/// the planet lands in the clade's preferred band (η=1). Returns `None` when
/// the star is too dim to reach that band at any orbit ≥ 1.
fn probe_matched_ceiling(name: &str, star: StarClass) -> Option<cfe::ratio::Ratio> {
    use cfe::almanac::clade_preferred_region;
    let want = clade_preferred_region(13);
    for r in 1..=256i64 {
        let world = seat(name, star, (0, 1), (r, 1), 13);
        if world.orb_region(FIRST_ORB) == Some(want) {
            return Some(world.population_ceiling(FIRST_ORB));
        }
    }
    None
}
