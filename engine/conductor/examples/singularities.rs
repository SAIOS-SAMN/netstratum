//! Read every core's own table (docs/140 C2, #252): `singularities <var>`
//! reads a laid universe and founds, for each core, the table its own throat
//! makes from its own nucleons. Reads only; writes nothing.
fn main() {
    let var = std::env::args().nth(1).unwrap_or_else(|| "var-e1".into());
    let Some((_, u)) = conductor::pulse::read(std::path::Path::new(&var)) else {
        println!("no universe at {var}/pulse.bin");
        return;
    };
    for (path, nuclei, top) in conductor::cradle::singularity_tables(&u) {
        let digits = nuclei.to_string().len();
        println!(
            "core {path:?}: nuclei ~10^{} , its own table's frontier weight {top}",
            digits.saturating_sub(1)
        );
    }
}
