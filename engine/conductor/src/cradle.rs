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
//!
//! **Extend** (#250): a live universe receives matter at its universal
//! core's throat by a named deed, its receipt kept whole in `cradle.jrn`.
//!
//! **Advance** (#249): a founded universe breathed forward from its
//! checkpoint, the beat a ledger entry cites.
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

/// The founded universe in `var_root` advanced to bell `until`, bell by
/// bell as the conductor beats (`breath::breathe`), from its checkpoint
/// (`conductor.beat`), never from the founding. It is written back with its
/// beat. Returns the bell it stands at.
pub fn advance(var_root: &Path, until: u64) -> std::io::Result<u64> {
    let (seed, mut u) =
        pulse::read(var_root).ok_or_else(|| std::io::Error::other("no universe at pulse.bin"))?;
    let start: u64 = std::fs::read_to_string(var_root.join("conductor.beat"))
        .ok()
        .and_then(|t| t.trim().parse().ok())
        .unwrap_or(0);
    let t_u = u.t_u;
    for b in start..until {
        crate::breath::breathe(&mut u, &seed, t_u, b, b + 1);
    }
    let at = until.max(start);
    pulse::write(var_root, &seed, &u)?;
    std::fs::write(var_root.join("conductor.beat"), at.to_string())?;
    Ok(at)
}

/// Where an extension's matter comes from (docs/140 C3): every source is
/// named, so no matter enters a universe unaccounted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    /// A growth grant applied on the court, by its manifest hash.
    Growth([u8; 32]),
    /// An inbound crossing, by its crossing hash (#109; its keep formula
    /// derived there).
    Crossing([u8; 32]),
    /// A core's own bounce (V6), by the core's path.
    Bounce(Vec<u64>),
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex = |b: &[u8; 32]| b.iter().map(|x| format!("{x:02x}")).collect::<String>();
        match self {
            Source::Growth(m) => write!(f, "growth {}", hex(m)),
            Source::Crossing(c) => write!(f, "crossing {}", hex(c)),
            Source::Bounce(p) => write!(f, "bounce {p:?}"),
        }
    }
}

/// **Extend** (docs/140 C3, D2; #250): a live universe receives `parcel` at
/// its universal core's throat by a named deed. The parcel joins the throat's
/// mix, and from the next bell the release, the capture phases and the flows
/// read it; nothing before is re-run. The deed is written whole, the source
/// and every kind with its exact amount, to `cradle.jrn` (appended, never
/// rewritten), and the universe is written back. Nothing is edited: the
/// universe's total after is its total before plus the parcel. Returns the
/// receipt's line. Refused where the parcel is empty or holds a negative
/// amount, or the universe has no home core.
pub fn extend(var_root: &Path, parcel: &Manifest, source: &Source) -> std::io::Result<String> {
    use num_traits::{Signed, Zero};
    if parcel.is_empty() || parcel.values().any(|a| a.is_negative() || a.is_zero()) {
        return Err(std::io::Error::other(
            "an extension carries matter, every amount positive",
        ));
    }
    let (seed, mut u) =
        pulse::read(var_root).ok_or_else(|| std::io::Error::other("no universe at pulse.bin"))?;
    let home = u
        .cores
        .iter_mut()
        .find(|c| pulse::is_home(c))
        .ok_or_else(|| std::io::Error::other("the universe has no home core"))?;
    for (k, a) in parcel {
        *home
            .throat
            .entry(k.clone())
            .or_insert_with(|| cfe::ratio::ratio(0, 1)) += a.clone();
    }
    let bell: u64 = std::fs::read_to_string(var_root.join("conductor.beat"))
        .ok()
        .and_then(|t| t.trim().parse().ok())
        .unwrap_or(0);
    let parts: Vec<String> = parcel.iter().map(|(k, a)| format!("{k}={a}")).collect();
    let line = format!("extend at bell {bell} by {source}: {}", parts.join(" "));
    pulse::write(var_root, &seed, &u)?;
    use std::io::Write;
    let mut jrn = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(var_root.join("cradle.jrn"))?;
    writeln!(jrn, "{line}")?;
    Ok(line)
}

/// The parcels the cradle's journal records, in order (the receipts read
/// back exactly: each kind and its exact amount).
pub fn extensions(var_root: &Path) -> std::io::Result<Vec<Manifest>> {
    let text = match std::fs::read_to_string(var_root.join("cradle.jrn")) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    text.lines()
        .filter_map(|l| l.split_once(": ").map(|(_, p)| p))
        .map(|p| {
            p.split(' ')
                .map(|kv| {
                    let (k, a) = kv.split_once('=').ok_or_else(|| {
                        std::io::Error::other("a receipt part without its amount")
                    })?;
                    let kind = crate::chem::Kind::parse(k)
                        .ok_or_else(|| std::io::Error::other(format!("{k} is no kind")))?;
                    let amount = a
                        .parse::<Ratio>()
                        .map_err(|_| std::io::Error::other(format!("{a} is no exact amount")))?;
                    Ok((kind, amount))
                })
                .collect()
        })
        .collect()
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

    /// docs/140 §4, witness 3: an extension conserves. The universe's total
    /// after is its total before plus the parcel, and the receipt reads back
    /// the parcel exactly. An empty or negative parcel is refused.
    #[test]
    fn an_extension_adds_its_parcel_exactly_and_keeps_its_receipt() {
        let n = BigInt::from(10u32).pow(9);
        let g = crate::polarity::throat_held();
        let seed = *blake3::hash(b"cradle extend").as_bytes();
        let var = store("extend");
        std::fs::create_dir_all(&var).expect("a var");
        let f = found(None, &n, &g, &seed).expect("a founding");
        pulse::write(&var, &seed, &f.universe).expect("written");
        let before = pulse::read(&var).expect("read").1.total();
        let h = crate::chem::hydrogen();
        let he = crate::chem::Kind::atom(4).expect("weight 4");
        let parcel: Manifest = [
            (h.clone(), cfe::ratio::ratio(3, 7)),
            (he.clone(), cfe::ratio::ratio(1, 5)),
        ]
        .into_iter()
        .collect();
        let line = extend(&var, &parcel, &Source::Crossing([7; 32])).expect("extended");
        assert!(line.contains("crossing 0707"));
        let after = pulse::read(&var).expect("read").1.total();
        let mut want = before.clone();
        for (k, a) in &parcel {
            *want
                .entry(k.clone())
                .or_insert_with(|| cfe::ratio::ratio(0, 1)) += a.clone();
        }
        assert_eq!(after, want, "total after = before + parcel");
        assert_eq!(extensions(&var).expect("receipts"), vec![parcel.clone()]);
        let empty = Manifest::new();
        assert!(extend(&var, &empty, &Source::Growth([0; 32])).is_err());
        let neg: Manifest = [(h, cfe::ratio::ratio(-1, 2))].into_iter().collect();
        assert!(extend(&var, &neg, &Source::Growth([0; 32])).is_err());
        let _ = std::fs::remove_dir_all(&var);
    }
}
