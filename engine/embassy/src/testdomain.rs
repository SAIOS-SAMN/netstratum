//! A test domain as one deed (docs/140 D3, #249): the cradle founds a
//! universe, the embassy puts it on the court, and its grant is applied.
//! Never for a live domain: the home must hold no tier key, and every key is
//! minted here, fresh ("a new key is a new domain").
use mycelium_sdk::session::Session;
use std::io;
use std::path::Path;

/// The tier keys a domain signs with (`fold::TIERS` and the domain).
const KEYS: [&str; 3] = ["tier-domain.key", "tier-colonies.key", "tier-union.key"];

/// The founding's size, `2^N` or a decimal, in grains.
fn grains(size: &str) -> io::Result<num_bigint::BigInt> {
    let g = match size.split_once('^') {
        Some((b, e)) => b
            .parse::<num_bigint::BigInt>()
            .ok()
            .zip(e.parse::<u32>().ok())
            .map(|(b, e)| b.pow(e)),
        None => size.parse::<num_bigint::BigInt>().ok(),
    };
    g.filter(|g| *g > num_bigint::BigInt::from(0))
        .ok_or_else(|| io::Error::other(format!("{size} is not a positive count of grains")))
}

/// Fresh keys in `home`, refused where any tier key stands already.
fn mint(home: &Path) -> io::Result<()> {
    use std::io::Write;
    if KEYS.iter().any(|k| home.join(k).exists()) {
        return Err(io::Error::other(
            "the home already holds a tier key: a live domain is never re-keyed",
        ));
    }
    std::fs::create_dir_all(home)?;
    for k in KEYS {
        // 32 bytes of OS entropy, as every nonce here (`refound::fresh`).
        let key = crate::refound::fresh()?;
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(home.join(k))?;
        f.write_all(&key)?;
    }
    Ok(())
}

/// Found a test domain: keys, the cradle's founding at the universal core's
/// throat, the Big Bang on its universe ledger, the refound on the court,
/// one bell, and the grant applied. Returns what was said.
pub fn found(
    session: &Session,
    home: &Path,
    var: &Path,
    size: &str,
    phrase: &str,
) -> io::Result<Vec<String>> {
    if var.join("pulse.bin").exists() {
        return Err(io::Error::other("the var already holds a universe"));
    }
    let n = grains(size)? * conductor::pulse::nuclei_per_grain() * num_bigint::BigInt::from(1024);
    mint(home)?;
    let mut said = vec![format!("keys minted in {}", home.display())];
    let seed = *blake3::hash(phrase.as_bytes()).as_bytes();
    let store = std::env::var("CRADLE_STORE").unwrap_or_else(|_| "var-cradle".into());
    let f = conductor::cradle::found(
        Some(Path::new(&store)),
        &n,
        &conductor::polarity::throat_held(),
        &seed,
    )
    .ok_or_else(|| io::Error::other("the founding holds no disc, or its echo carries no ring"))?;
    conductor::pulse::write(var, &seed, &f.universe)?;
    said.push(format!(
        "founded: {} kinds, the deployment {}",
        f.table.len(),
        if f.reused { "reused" } else { "made and held" }
    ));
    let (_, fresh) = ledger::Ledger::at(var.join("worlds")).found_universe(
        conductor::roster_digest(),
        cfe::almanac::matter_law_digest(),
        0,
    )?;
    said.push(format!("Big Bang recorded: {fresh}"));
    let (sealed, lines) = crate::refound::refound(session, home, var)?;
    said.extend(lines);
    if !sealed {
        said.push("the refound did not seal; the grant is not applied".into());
        return Ok(said);
    }
    let at = conductor::cradle::advance(var, 1)?;
    said.push(format!("advanced to bell {at}"));
    said.extend(crate::refound::apply(session, home, var)?);
    Ok(said)
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_size_reads_and_a_keyed_home_is_refused() {
        assert_eq!(
            super::grains("2^4").expect("a size"),
            num_bigint::BigInt::from(16)
        );
        assert_eq!(
            super::grains("12").expect("a size"),
            num_bigint::BigInt::from(12)
        );
        assert!(super::grains("0").is_err() && super::grains("x").is_err());
        let root = std::env::var("TMPDIR").unwrap_or_else(|_| ".testtmp".into());
        let home =
            std::path::PathBuf::from(root).join(format!("testdomain-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        super::mint(&home).expect("fresh keys");
        for k in super::KEYS {
            assert_eq!(std::fs::read(home.join(k)).expect("a key").len(), 32);
        }
        assert!(
            super::mint(&home).is_err(),
            "a keyed home is never re-keyed"
        );
        let _ = std::fs::remove_dir_all(&home);
    }
}
