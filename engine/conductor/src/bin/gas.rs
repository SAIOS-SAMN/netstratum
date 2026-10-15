//! docs/83: read a live universe's gas, stretch by stretch, at a bell:
//! why it does or does not collapse. `gas <era1 var> <bell> [galaxy]`.
use conductor::{formation, pulse};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let var = args.get(1).map(String::as_str).unwrap_or("var-e1");
    let bell: u64 = args.get(2).and_then(|b| b.parse().ok()).unwrap_or(0);
    let only: Option<u32> = args.get(3).and_then(|g| g.parse().ok());
    let Some((_, u)) = pulse::read(Path::new(var)) else { return };
    let f = |r: &cfe::ratio::Ratio| r.numer().to_string().parse::<f64>().unwrap_or(0.0) / r.denom().to_string().parse::<f64>().unwrap_or(1.0);
    let universe = pulse::mass(&u.total());
    for core in u.cores.iter().filter(|c| only.is_none_or(|g| c.path[0] == g)) {
        let all = core.total();
        let share = pulse::mass(&all) / universe.clone();
        let (now, h) = formation::shells(&all, &core.kept, &share);
        let rings: Vec<formation::Shell> = now.into_iter().filter(|r| r.outer == r.inner).collect();
        let gas: Vec<formation::Shell> = core.gas.iter().cloned().chain(rings).collect();
        let stretches = formation::stretches(&gas, bell);
        let collapsing = stretches.iter().filter(|st| formation::collapse(st, &h).is_some()).count();
        let hot = stretches.iter().filter(|st| formation::ignition_share(&st.agitation).is_none()).count();
        println!("galaxy {:?}: H {:.3}, {} parcels, {} stretches, {} could collapse, {} too hot to ignite (a ≥ ½)", core.path, f(&h), core.gas.len(), stretches.len(), collapsing, hot);
        for st in stretches.iter().take(12) {
            println!("   [{:.3e}..{:.3e}] λ {:.3e} a {:.3e} λ/(1+λ) {:.3e} collapse {:?}", f(&st.inner), f(&st.outer), f(&st.lambda), f(&st.agitation), f(&st.lambda) / (1.0 + f(&st.lambda)), formation::collapse(st, &h).map(|(w, c)| (f(&w), c)));
        }
    }
}
