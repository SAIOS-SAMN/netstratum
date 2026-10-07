//! A world's population against its physics ceilings, at its chronicle head.
fn main() {
    for dir in std::env::args().skip(1) {
        let c = cfe::persist::load(&std::path::Path::new(&dir).join("chronicle.tlv")).unwrap();
        let w = cfe::world::World::resume(c);
        let orb = cfe::world::FIRST_ORB;
        let f = |r: cfe::ratio::Ratio| format!("{:.3}", num_traits::ToPrimitive::to_f64(&r).unwrap_or(f64::NAN));
        println!(
            "{dir}: pop {} capacity(K) {} ceiling(K_pop) {} per_capita {} syntropic {}",
            w.population_of(orb),
            f(w.population_capacity(orb)),
            f(w.population_ceiling(orb)),
            f(w.per_capita_ration()),
            f(w.syntropic_flow(orb)),
        );
    }
}
