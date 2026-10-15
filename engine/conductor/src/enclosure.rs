//! docs/78 "The singularities": THE ENCLOSURES (operator, 2026-10-15). Each
//! galaxy is its own closed S^360 and each star system its own closed S^60,
//! hung inside the sphere above at its core: the core is the THROAT joining
//! the two. Galactic cores are HUBS (uplink to the universal core, downlinks
//! to what they enclose); only the universal core is uplinked to the hypha.
//!
//! - **The rim** is where the core's pulse refocuses, on the enclosure's far
//!   side: held as the sphere's HALF CIRCUMFERENCE, in light-rings, so its
//!   radius (rim / π) is never formed (a reading, like a sine). Transitional:
//!   the outermost member's reach, until formation reads the refocus.
//! - **The enclosure's own ring** is the pulse's round trip, core to rim and
//!   back: `2 · rim` light-rings. Each tier counts its turns on its own ring,
//!   so the same turning reads few turns on a large enclosure's ring and many
//!   on a small one's: galaxies read slow and worlds fast, nothing fitted.
//! - **A place** on an enclosure is its polar share (its reach over the rim,
//!   a share of a half turn from the core) and its phase (turns): exact.
//! - **A route** between two bodies runs on their spheres to their cores and
//!   through the throats, up the parent links to the enclosure they share and
//!   down again: galaxies interlink only through the universal core (the hub
//!   ruling), so the route between galaxies runs through it.
use crate::pulse::{Body, Universe};
use cfe::ratio::{ratio, Ratio};

/// An enclosed sphere: a galaxy's S^360 or a star system's S^60.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enclosure {
    /// Its core's path: `[g]` a galaxy, `[g, s]` a star system.
    pub path: Vec<u32>,
    /// 360 for a galaxy, 60 for a star system.
    pub dim: u32,
    /// The half circumference, in light-rings (where the pulse refocuses).
    pub rim: Ratio,
    /// Its throat's reach on the sphere above: a galaxy's distance from the
    /// universal core, a system's orbit about its galaxy's core.
    pub throat: Ratio,
}

impl Enclosure {
    /// Its own ring: the pulse's round trip, `2 · rim`.
    pub fn ring(&self) -> Ratio {
        ratio(2, 1) * self.rim.clone()
    }

    /// The enclosure above (empty: the universe's).
    pub fn parent(&self) -> &[u32] {
        &self.path[..self.path.len() - 1]
    }
}

/// The rim of a centre's enclosure: its outermost member's reach.
fn rim_of(centre: &Body) -> Option<Ratio> {
    centre.satellites.iter().map(|s| s.radius.clone()).max().filter(|r| *r > ratio(0, 1))
}

/// Every enclosure of the universe: each galaxy with a member, and each
/// star system with a world.
pub fn enclosures(u: &Universe) -> Vec<Enclosure> {
    let mut out = Vec::new();
    for core in &u.cores {
        if let Some(rim) = rim_of(core) {
            out.push(Enclosure { path: core.path.clone(), dim: 360, rim, throat: core.radius.clone() });
        }
        for sys in &core.satellites {
            if let Some(rim) = rim_of(sys) {
                out.push(Enclosure { path: sys.path.clone(), dim: 60, rim, throat: sys.radius.clone() });
            }
        }
    }
    out
}

/// A body's place on its enclosure: its polar share of a half turn from the
/// core (its reach over the rim) and its phase at `bell`, in turns.
pub fn place(body: &Body, enclosure: &Enclosure, phase0: &Ratio, bell: u64) -> (Ratio, Ratio) {
    let polar = body.radius.clone() / enclosure.rim.clone();
    let turn = if body.period > ratio(0, 1) { ratio(bell as i64, 1) / body.period.clone() } else { ratio(0, 1) };
    let phase = phase0.clone() + turn;
    (polar, phase.clone() - phase.floor())
}

/// The reach of the body at `path` up to its enclosure's core, then each
/// throat up to the universal core: the arcs it climbs, innermost first.
fn climb(u: &Universe, path: &[u32]) -> Option<Vec<Ratio>> {
    let core = u.cores.iter().find(|c| c.path.first() == path.first())?;
    let mut arcs = Vec::new();
    match path.len() {
        1 => arcs.push(core.radius.clone()),
        2 => {
            let sys = core.satellites.iter().find(|s| s.path == path)?;
            arcs.extend([sys.radius.clone(), core.radius.clone()]);
        }
        3 => {
            let sys = core.satellites.iter().find(|s| s.path[..] == path[..2])?;
            let orb = sys.satellites.iter().find(|o| o.path == path)?;
            arcs.extend([orb.radius.clone(), sys.radius.clone(), core.radius.clone()]);
        }
        _ => return None,
    }
    Some(arcs)
}

/// THE ROUTE between two bodies through the throats, in light-rings: each
/// climbs to the deepest enclosure they share (the universal core for two
/// galaxies) and the arcs are summed; within a shared enclosure the arcs to
/// its core are added (the hub route through the throat). Exact.
pub fn route(u: &Universe, a: &[u32], b: &[u32]) -> Option<Ratio> {
    let shared = a.iter().zip(b).take_while(|(x, y)| x == y).count();
    let (up_a, up_b) = (climb(u, a)?, climb(u, b)?);
    // Climb only to the shared enclosure: drop the arcs above it.
    let keep = |arcs: &[Ratio], len: usize| arcs[..len - shared].iter().fold(ratio(0, 1), |s, x| s + x.clone());
    Some(keep(&up_a, a.len()) + keep(&up_b, b.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn galaxies_and_systems_enclose_and_routes_run_through_the_throats() {
        let manifest: crate::pulse::Manifest = [("Z001".to_string(), ratio(4_000_000, 1)), ("Z008".to_string(), ratio(1_000_000, 1))].into();
        let u = crate::pulse::deploy(&[3; 32], &manifest);
        let encl = enclosures(&u);
        assert!(encl.iter().any(|e| e.dim == 360) && encl.iter().any(|e| e.dim == 60));
        for e in &encl {
            assert_eq!(e.ring(), ratio(2, 1) * e.rim.clone());
        }
        // Two worlds of one system meet at their star: the sum of their reaches.
        let sys = u.cores[0].satellites.iter().find(|s| s.satellites.len() >= 2).expect("a system with two worlds");
        let (w1, w2) = (&sys.satellites[0], &sys.satellites[1]);
        assert_eq!(route(&u, &w1.path, &w2.path), Some(w1.radius.clone() + w2.radius.clone()));
        // Two galaxies meet only through the universal core.
        let (g1, g2) = (&u.cores[0], &u.cores[1]);
        assert_eq!(route(&u, &g1.path, &g2.path), Some(g1.radius.clone() + g2.radius.clone()));
        // A world reaches another galaxy up its system's throat, its galaxy's,
        // the universal core, and down.
        let r = route(&u, &w1.path, &g2.path).unwrap();
        assert_eq!(r, w1.radius.clone() + sys.radius.clone() + g1.radius.clone() + g2.radius.clone());
        // A place: polar share at most one half turn.
        let e = encl.iter().find(|e| e.path == sys.path).unwrap();
        let (polar, phase) = place(w2, e, &ratio(0, 1), 1000);
        assert!(polar <= ratio(1, 1) && phase < ratio(1, 1));
    }
}
