//! Chronometry inside A universe: one law with Xylarium (its
//! docs/XYLARIUM-KERNEL.md 11.4d, crates/xyl-kernel/src/chronometry.rs).
//! Time is kept by keepers chained from the epicenter outward, never one
//! scalar read everywhere.
//!
//! - The keepers. The epicenter (the universal core, its bounce), then for
//!   each galactic core three shells: its deep core (its innermost system's
//!   radius), INNER mantle (its half-mass radius) and outer rim (its outermost
//!   system's radius).
//! - A keeper's rate: the law of the wells (breath.rs), a layer at r with
//!   M(<r) inside runs at `r / (r + M(<r))` of the layer above; the galaxy's
//!   well against the cores nearer the epicenter, the shell's within it;
//!   nested wells multiply. Exact.
//! - Its delay: the bells the pulse takes to reach it: its galaxy's distance
//!   plus its shell's radius at the ring's light speed (a ring is the round
//!   trip to the home galaxy's rim and back: the home rim's radius is half a
//!   bell).
//! - Its reading at the epicenter's count N: local `rate (N − delay)`, slip
//!   `(1 − rate)(N − delay)` turns, coherence `1 − 2|slip − round slip|`,
//!   rung `⌊slip⌋`.
//! - The chain. At each committed moment (every pulse) every keeper appends an
//!   entry citing its own previous entry and the entry of the keeper inside
//!   it (rim → mantle → core → epicenter), hashed. Its slip against the
//!   keeper inside it is the entry's RESIDUAL, kept, never averaged. The
//!   epicenter's entry carries the last TUniversal final point heard (the
//!   bridge: rings per final-point advance, measured). `<var>/chrono.jrn`.
use crate::breath::unit_mass;

use crate::motion::galactic_year;
use crate::pulse::{mass, Universe};
use cfe::ratio::{ratio, Ratio};
use ledger::card;
use num_traits::Zero;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shell {
    Epicenter = 0,
    Core = 1,
    Mantle = 2,
    Rim = 3,
}

/// A keeper: its galaxy (none at the epicenter), shell, rate and delay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keeper {
    pub galaxy: Option<u32>,
    pub shell: Shell,
    pub rate: Ratio,
    pub delay: Ratio,
}

/// What a keeper reads at the epicenter's count.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    pub local: Ratio,
    pub slip: Ratio,
    pub coherence: Ratio,
    pub rung: u64,
}

impl Keeper {
    pub fn read(&self, n: &Ratio) -> Reading {
        let since = (n.clone() - self.delay.clone()).max(ratio(0, 1));
        let local = self.rate.clone() * since.clone();
        let slip = (ratio(1, 1) - self.rate.clone()) * since;
        let rung = slip.floor();
        let frac = slip.clone() - rung.clone();
        let off = if frac > ratio(1, 2) {
            ratio(1, 1) - frac
        } else {
            frac
        };
        Reading {
            local,
            slip,
            coherence: ratio(1, 1) - ratio(2, 1) * off,
            rung: rung.to_integer().try_into().unwrap_or(0),
        }
    }

    fn key(&self) -> (u32, u8) {
        (self.galaxy.unwrap_or(0), self.shell as u8)
    }
}

/// The universe's keepers now, each with the index of the keeper inside it.
pub fn keepers(u: &Universe) -> Vec<(Keeper, Option<usize>)> {
    let unit = unit_mass(u);
    let mut out = vec![(
        Keeper {
            galaxy: None,
            shell: Shell::Epicenter,
            rate: ratio(1, 1),
            delay: ratio(0, 1),
        },
        None,
    )];
    // The ring's light: the home galaxy's rim radius is half a bell.
    let home = u
        .cores
        .iter()
        .max_by(|a, b| mass(&a.kept).cmp(&mass(&b.kept)));
    let light = home
        .and_then(|c| c.satellites.last())
        .map(|s| ratio(2, 1) * s.radius.clone())
        .filter(|l| !l.is_zero())
        .unwrap_or_else(|| ratio(1, 1));
    for (i, core) in u.cores.iter().enumerate() {
        let half = galactic_year(core).map(|(k, _)| k).unwrap_or(0);
        let shells = [
            (Shell::Core, 0usize),
            (Shell::Mantle, half),
            (Shell::Rim, core.satellites.len().saturating_sub(1)),
        ];
        let mut inner = 0usize;
        for (shell, k) in shells {
            let Some(sys) = core.satellites.get(k) else {
                continue;
            };
            let keeper = Keeper {
                galaxy: core.path.first().map(|g| *g as u32),
                shell,
                rate: crate::breath::system_clock(u, i, k, &unit),
                delay: core.radius.clone() + sys.radius.clone() / light.clone(),
            };
            out.push((keeper, Some(inner)));
            inner = out.len() - 1;
        }
    }
    out
}

/// The chain's heads (each keeper's last entry hash), kept in its file.
pub struct Chain {
    path: PathBuf,
    pub heads: BTreeMap<(u32, u8), [u8; 32]>,
    pub entries: u64,
}

mod tag {
    pub const GALAXY: u8 = 1;
    pub const SHELL: u8 = 2;
    pub const AT: u8 = 3;
    pub const LOCAL: u8 = 4;
    pub const SLIP: u8 = 5;
    pub const COHERENCE: u8 = 6;
    pub const RUNG: u8 = 7;
    pub const RESIDUAL: u8 = 8;
    pub const PARENT: u8 = 9;
    pub const INNER: u8 = 10;
    pub const HASH: u8 = 11;
    pub const POINT: u8 = 12;
}

impl Chain {
    pub fn open(var_root: &Path) -> Chain {
        let path = var_root.join("chrono.jrn");
        let mut heads = BTreeMap::new();
        let mut entries = 0;
        if let Ok(data) = std::fs::read(&path) {
            for frame in card::frames(&data) {
                let (mut g, mut s, mut h) = (None, None, None);
                for (t, v) in card::records(frame) {
                    match t {
                        tag::GALAXY => g = card::u64_of(v),
                        tag::SHELL => s = card::u64_of(v),
                        tag::HASH => h = <[u8; 32]>::try_from(v).ok(),
                        _ => {}
                    }
                }
                if let (Some(g), Some(s), Some(h)) = (g, s, h) {
                    heads.insert((g as u32, s as u8), h);
                    entries += 1;
                }
            }
        }
        Chain {
            path,
            heads,
            entries,
        }
    }

    /// Commit a moment at the epicenter's count `n`; `point` the last final
    /// point's height the house heard (None: unlinked). Returns each rim's
    /// residual against its mantle.
    pub fn commit(
        &mut self,
        u: &Universe,
        n: u64,
        point: Option<u64>,
    ) -> std::io::Result<Vec<(u32, Ratio)>> {
        let keepers = keepers(u);
        let at = ratio(n as i64, 1);
        let mut slips: Vec<Ratio> = Vec::with_capacity(keepers.len());
        let mut hashes: Vec<[u8; 32]> = Vec::with_capacity(keepers.len());
        let mut rims = Vec::new();
        let mut out = Vec::new();
        for (keeper, inner) in &keepers {
            let r = keeper.read(&at);
            let residual = inner
                .map(|i| r.slip.clone() - slips[i].clone())
                .unwrap_or_else(|| ratio(0, 1));
            let parent = self.heads.get(&keeper.key()).copied().unwrap_or([0; 32]);
            let inner_hash = inner.map(|i| hashes[i]).unwrap_or([0; 32]);
            let mut h = blake3::Hasher::new_derive_key("netstratum chronometry entry v1");
            h.update(&keeper.key().0.to_le_bytes());
            h.update(&[keeper.key().1]);
            for part in [
                at.to_string(),
                r.local.to_string(),
                r.slip.to_string(),
                residual.to_string(),
            ] {
                h.update(part.as_bytes());
                h.update(&[0]);
            }
            h.update(&parent);
            h.update(&inner_hash);
            if keeper.shell == Shell::Epicenter {
                h.update(&point.map(|p| p.to_le_bytes()).unwrap_or([0xff; 8]));
            }
            let hash: [u8; 32] = *h.finalize().as_bytes();
            card::put_u64(tag::GALAXY, u64::from(keeper.key().0), &mut out);
            card::put_u64(tag::SHELL, u64::from(keeper.key().1), &mut out);
            card::put_u64(tag::AT, n, &mut out);
            card::put_str(tag::LOCAL, &r.local.to_string(), &mut out);
            card::put_str(tag::SLIP, &r.slip.to_string(), &mut out);
            card::put_str(tag::COHERENCE, &r.coherence.to_string(), &mut out);
            card::put_u64(tag::RUNG, r.rung, &mut out);
            card::put_str(tag::RESIDUAL, &residual.to_string(), &mut out);
            card::put(tag::PARENT, &parent, &mut out);
            card::put(tag::INNER, &inner_hash, &mut out);
            card::put(tag::HASH, &hash, &mut out);
            if keeper.shell == Shell::Epicenter {
                if let Some(p) = point {
                    card::put_u64(tag::POINT, p, &mut out);
                }
            }
            card::append_frame(&self.path, &out)?;
            out.clear();
            self.heads.insert(keeper.key(), hash);
            self.entries += 1;
            if keeper.shell == Shell::Rim {
                rims.push((keeper.key().0, residual));
            }
            slips.push(r.slip);
            hashes.push(hash);
        }
        Ok(rims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_keeper_reads_its_slip_and_rungs() {
        let k = Keeper {
            galaxy: Some(1),
            shell: Shell::Rim,
            rate: ratio(3, 4),
            delay: ratio(20, 1),
        };
        let r = k.read(&ratio(28, 1));
        assert_eq!(
            (r.local, r.slip.clone(), r.rung),
            (ratio(6, 1), ratio(2, 1), 2)
        );
        assert_eq!(r.coherence, ratio(1, 1), "back in phase at two whole slips");
        assert_eq!(
            k.read(&ratio(10, 1)).slip,
            ratio(0, 1),
            "before the echo arrives, nothing"
        );
    }
}
