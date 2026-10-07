//! How each world opened: star, orbit, first overheat, collapse tick, and the
//! line's axes — read from the chronicle and the registry.
use cfe::dop::ClosureEvent as E;
fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".to_string());
    let mut dirs: Vec<_> = std::fs::read_dir(format!("{root}/worlds")).unwrap().flatten().map(|e| e.path()).filter(|p| p.join("chronicle.tlv").exists()).collect();
    dirs.sort();
    for d in dirs {
        let c = cfe::persist::load(&d.join("chronicle.tlv")).unwrap();
        let (mut star, mut orbit, mut albedo, mut heat, mut fell, mut vented) = (None, String::new(), String::new(), None, None, 0u32);
        for r in c.receipts() {
            match r.event() {
                E::StarCommissioned { class } => star = Some(*class),
                E::PlanetSited { albedo: a, orbit_r, .. } => {
                    orbit = format!("{}", orbit_r.value());
                    albedo = format!("{}", a.value());
                }
                E::OverheatStruck { overflow, .. } if heat.is_none() => heat = Some((r.stamp(), num_traits::ToPrimitive::to_f64(overflow).unwrap_or(0.0))),
                E::VentSettled { .. } => vented += 1,
                E::CivilizationCollapsed { .. } if fell.is_none() => fell = Some(r.stamp()),
                _ => {}
            }
        }
        let name = d.file_name().unwrap().to_str().unwrap().to_string();
        println!("{name:<18} star={:<5} orbit={orbit:<6} albedo={albedo:<5} overheat={:<22} vents={vented:<5} collapsed={}",
            star.map(|s| s.to_string()).unwrap_or("-".into()),
            heat.map(|(t, o)| format!("t{t} {o:.0}")).unwrap_or("-".into()),
            fell.map(|t| format!("t{t}")).unwrap_or("-".into()));
    }
}
