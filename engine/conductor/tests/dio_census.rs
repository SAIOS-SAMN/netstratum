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
    use num_traits::ToPrimitive;
    let f = |x: &cfe::ratio::Ratio| x.to_f64().unwrap_or(f64::NAN);
    let mut total = 0usize;
    let mut frozen: Vec<String> = Vec::new();
    println!("SIGIL          CLADE PREF          INS    ETA    SYNTR  CULT   ZONES FAM");
    for p in &worlds {
        let sigil = p.spec.civilization.sigil.clone();
        let orb = p.world.orb().expect("orb");
        let orb_id = cfe::world::FIRST_ORB;
        let zones = orb.zones.len();
        let fam = !orb.famine.is_empty();
        total += zones;
        if fam {
            frozen.push(sigil.clone());
        }
        let clade = p.world.rejuvenation_clade();
        let pref = cfe::almanac::clade_preferred_region(clade);
        let sited = orb.thermal.is_some();
        let (ins, eta) = if sited {
            let raw = p.world.insolation_of(orb_id);
            let t_eq4 = raw.clone() * cfe::thermal::thermal_k();
            let eta = cfe::almanac::habitability_flux(clade, &t_eq4);
            (f(&raw), f(&eta))
        } else {
            (f64::NAN, f64::NAN)
        };
        let syn = f(&p.world.syntropic_flow(orb_id));
        let cult = f(&p.world.cultivated_yield(orb_id));
        println!(
            "{sigil:14} {clade:5} {pref:13?} {ins:6.3} {eta:6.3} {syn:6.2} {cult:6.2} {zones:5} {fam}"
        );
    }
    println!("TOTAL_ZONES={total} FAMISHED_COUNT={} FAMISHED={:?}", frozen.len(), frozen);
}

#[test]
#[ignore = "diagnostic: 6-seat industry trajectory under DIO-3a"]
fn industry_probe() {
    let root = scratch("industry");
    let worlds = conductor::run_era_until_among(&root.to_string_lossy(), 90, 6, |_| false);
    println!("SIGIL          METAB     CLADE LEVEL ZONES SMELT FAM");
    for p in &worlds {
        let orb = p.world.orb();
        use cfe::dop::ZoneKind::*;
        let count = |k: cfe::dop::ZoneKind| orb.map(|o| o.zones.iter().filter(|z| z.kind == k).count()).unwrap_or(0);
        let zones = orb.map(|o| o.zones.len()).unwrap_or(0);
        println!("{:14} {:9} clade={:3} lvl={} zones={} | Smelt={} Synth={} Refin={} Extract={}",
            p.spec.civilization.sigil, p.spec.civilization.metabolism,
            p.world.rejuvenation_clade(), p.level, zones,
            count(Smelter), count(Synthesizer), count(ElementRefinery), count(Extractor));
    }
}
