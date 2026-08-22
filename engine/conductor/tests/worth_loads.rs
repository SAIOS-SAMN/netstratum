// ADOPT BY WORTH loads from the mesh file (if present) and ranks by measured
// utility. Off-wire (no file) it is silently empty and callers fall back.
#[test]
fn worth_loads_and_ranks() {
    scribe::membrane::load_worth();
    let n = scribe::membrane::worth_loaded();
    eprintln!("WORTH loaded from mesh file: {n} orbits carry measured utility");
    // prime a synthetic high-worth orbit and confirm the reader returns it
    let orbit = [7u8; 32];
    scribe::membrane::prime_worth(vec![(orbit, 42)]);
    assert_eq!(scribe::membrane::word_worth(&orbit), 42, "primed worth reads back");
    assert_eq!(scribe::membrane::word_worth(&[0u8; 32]), 0, "unknown orbit is worth 0");
}
