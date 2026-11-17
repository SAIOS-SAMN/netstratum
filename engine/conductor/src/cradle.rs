//! The cradle (docs/140, #246): the progenitor's machine for the deeds on a
//! universe. Tier 0, the Board-Makers (docs/12 §2a): it makes and extends the
//! board, and governs nothing on it.
//!
//! **Found** (C2, D1; #248): `found(count, g, seed)`. The bounce's nucleons
//! found a table at a place of held share `g` (`pulse::founding_at`, the
//! climb of docs/97 §14–§15). The table is deployed (`pulse::deploy`), then
//! oriented by the seed (`pulse::orient`). The deployed universe depends only
//! on `(count, g)`, never on the seed, so it is computed once and held per
//! `(count, g)` in a store, and a second founding there is a load and an
//! orientation. The held universe reads back exactly (`pulse::read`). Each
//! founding says whether it reused a held one: the work counted, never timed.
use crate::pulse::{self, Manifest, Universe};
use cfe::ratio::Ratio;
use std::path::{Path, PathBuf};

/// A founding: its table, its universe oriented by the seed, and whether the
/// deployed universe was held already (the work saved).
#[derive(Debug, Clone)]
pub struct Founding {
    pub table: Manifest,
    pub universe: Universe,
    pub reused: bool,
}

/// The held deployment for `(count, g)` in `store`: its own folder, named by
/// the hash of the count and the place (exact, in decimal).
fn held_at(store: &Path, nuclei: &num_bigint::BigInt, g: &Ratio) -> PathBuf {
    let key = blake3::hash(format!("cradle founding v1 {nuclei} {g}").as_bytes());
    store.join(key.to_hex().as_str())
}

/// Found a universe of `nuclei` at a place of held share `g`, oriented by
/// `seed`. With a `store`, the deployed universe is held there per
/// `(count, g)` and reused. None where the founding holds no disc or its
/// echo carries no ring (`pulse::deploy`), or a held deployment does not read.
pub fn found(
    store: Option<&Path>,
    nuclei: &num_bigint::BigInt,
    g: &Ratio,
    seed: &[u8; 32],
) -> Option<Founding> {
    let table = pulse::founding_at(nuclei, g);
    let held = store.map(|s| held_at(s, nuclei, g));
    let (mut universe, reused) = match held.as_ref().and_then(|h| pulse::read(h)) {
        // The held deployment is the one this table makes: its total is the
        // table (conservation), or it is not used.
        Some((_, u)) if u.total() == table => (u, true),
        _ => {
            let u = pulse::deploy(&table)?;
            if let Some(h) = &held {
                if std::fs::create_dir_all(h).is_ok() {
                    // Held un-oriented: the seed orients each founding apart.
                    let _ = pulse::write(h, &[0u8; 32], &u);
                }
            }
            (u, false)
        }
    };
    pulse::orient(&mut universe, seed);
    Some(Founding {
        table,
        universe,
        reused,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    fn store(name: &str) -> PathBuf {
        let root = std::env::var("TMPDIR").unwrap_or_else(|_| ".testtmp".into());
        let p = PathBuf::from(root).join(format!("cradle-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        p
    }

    /// docs/140 §4, witness 1: a held founding equals a fresh one; the
    /// second founding at the same `(count, g)` reuses the held deployment;
    /// nucleons conserved; the throat's founding is `pulse::founding`.
    #[test]
    fn a_held_founding_equals_a_fresh_one_and_is_reused() {
        let n = BigInt::from(10u32).pow(9);
        let g = crate::polarity::throat_held();
        let seed = *blake3::hash(b"cradle witness").as_bytes();
        let s = store("held");
        let fresh = found(None, &n, &g, &seed).expect("a founding");
        let first = found(Some(&s), &n, &g, &seed).expect("a founding");
        let second = found(Some(&s), &n, &g, &seed).expect("a founding");
        assert!(!fresh.reused && !first.reused && second.reused);
        assert_eq!(first.universe, fresh.universe);
        assert_eq!(second.universe, fresh.universe, "held equals fresh");
        assert_eq!(
            fresh.table,
            pulse::founding(&n),
            "the throat is the universal core's"
        );
        assert_eq!(fresh.universe.total(), fresh.table, "nucleons conserved");
        let other = *blake3::hash(b"another seed").as_bytes();
        let oriented = found(Some(&s), &n, &g, &other).expect("a founding");
        assert!(oriented.reused, "the seed does not key the held deployment");
        let _ = std::fs::remove_dir_all(&s);
    }
}
