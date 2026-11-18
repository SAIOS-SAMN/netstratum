//! `same_universe <a> <b>`: whether two pulse.bin universes decode equal.
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let r = |p: &str| conductor::pulse::read(std::path::Path::new(p)).map(|(_, u)| u);
    match (r(&a[0]), r(&a[1])) {
        (Some(x), Some(y)) => println!("equal: {}", x == y),
        _ => println!("a universe did not read"),
    }
}
