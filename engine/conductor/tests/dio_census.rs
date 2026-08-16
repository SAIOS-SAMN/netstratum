//! The DIOPHANTINE CENSUS (diagnostic, 2026-08-16): dump the per-line siting
//! + thermal + settlement outcome at beat 15 so the realignment's effect on
//! K_pop and survival can be READ (not pinned) before the goldens re-measure.
//! `#[ignore]`d — it asserts nothing and adds ~30s, so it never runs in the
//! normal suite; invoke it explicitly (`--ignored`) when re-measuring a
//! thermal-base change. Kept through DIO-2 (species-relative η) for the same
//! re-measurement, then retire.

use conductor::run_era;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("dio-census-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
#[ignore = "diagnostic census, not a pin; run with --ignored when re-measuring a thermal-base change"]
fn census() {
    let root = scratch("beat15");
    let worlds = run_era(&root.to_string_lossy(), 15);
    let mut total = 0usize;
    let mut frozen: Vec<String> = Vec::new();
    println!("SIGIL          METAB     ZONES FAM  R            INSOLATION");
    for p in &worlds {
        let sigil = p.spec.civilization.sigil.clone();
        let meta = p.spec.civilization.metabolism.clone();
        let orb = p.world.orb().expect("orb");
        let zones = orb.zones.len();
        let fam = !orb.famine.is_empty();
        total += zones;
        if fam {
            frozen.push(sigil.clone());
        }
        let site = orb.thermal.clone();
        let (r, ins) = match (site, p.world.star()) {
            (Some(t), Some(class)) => {
                let r = t.orbit_r.value().clone();
                let ins = cfe::thermal::insolation(&class.luminosity(), t.albedo.value(), &r);
                (format!("{r}"), format!("{ins}"))
            }
            _ => ("none".into(), "none".into()),
        };
        println!("{sigil:14} {meta:9} {zones:5} {fam:5} {r:12} {ins}");
    }
    println!("TOTAL_ZONES={total} FAMISHED_COUNT={} FAMISHED={:?}", frozen.len(), frozen);
}
