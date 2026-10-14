//! docs/78 step 2: deploy era 1's stand-in universe from era 0's matter and
//! print its readings. `pulse <era0 var> <era1 var>`; the era 1 folder is
//! unfounded (no court writes).
use conductor::pulse;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (era0, era1) = (args.get(1).map(String::as_str).unwrap_or("var-g2"), args.get(2).map(String::as_str).unwrap_or("var-e1"));
    let (manifest, worlds) = pulse::stand_in(Path::new(era0));
    let total = manifest.values().fold(cfe::ratio::ratio(0, 1), |s, a| s + a.clone());
    println!("STAND-IN MANIFEST — {} elements from {worlds} worlds of era 0, {} grains in all", manifest.len(), total);
    // The stand-in seed until era 1's version-2 origin is founded.
    let seed = *blake3::hash(b"netstratum era 1, unfounded stand-in").as_bytes();
    let u = pulse::deploy(&seed, &manifest);
    let held = |m: &pulse::Manifest| m.values().fold(cfe::ratio::ratio(0, 1), |s, a| s + a.clone());
    let systems: usize = u.cores.iter().map(|c| c.satellites.len()).sum();
    let orbs: usize = u.cores.iter().flat_map(|c| &c.satellites).map(|s| s.satellites.len()).sum();
    println!("PULSE — {} cores, {systems} systems, {orbs} orbs; intergalactic medium {} grains", u.cores.len(), held(&u.medium));
    for c in &u.cores {
        let sys_orbs: Vec<usize> = c.satellites.iter().map(|s| s.satellites.len()).collect();
        let last = c.satellites.last().map(|s| (s.radius.clone(), s.period.clone()));
        println!(
            "  core {:?}: keeps {} grains of {}; {} systems (orbs each {:?}); medium {}; outermost radius/period {:?}",
            c.path, held(&c.kept), held(&c.total()), c.satellites.len(), sys_orbs, held(&c.medium), last.map(|(r, p)| format!("{r} / {p}"))
        );
    }
    let kept = u.total() == manifest;
    println!("CONSERVATION — every element deployed equals the manifest: {kept}");
    match pulse::write(Path::new(era1), &seed, &u) {
        Ok(()) => println!("written {era1}/pulse.bin"),
        Err(e) => println!("not written: {e}"),
    }
}
