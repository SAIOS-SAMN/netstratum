//! Does the committed-state codec round-trip every live world exactly?
//! replay(chronicle) -> state_bytes -> from_state_bytes == the replay.
fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".into());
    let mut dirs: Vec<_> = std::fs::read_dir(format!("{root}/worlds"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.join("chronicle.tlv").exists())
        .collect();
    dirs.sort();
    let (mut ok, mut bad) = (0, 0);
    for d in dirs {
        let c = cfe::persist::load(&d.join("chronicle.tlv")).unwrap();
        let committed = cfe::world::replay(&c);
        let bytes = codec::state_bytes(&committed);
        match codec::from_state_bytes(&bytes) {
            Some(back) if back == committed => ok += 1,
            Some(back) => {
                bad += 1;
                let diff: Vec<&str> = [
                    ("orbs", back.orbs == committed.orbs),
                    ("entities", back.entities == committed.entities),
                    ("debts", back.debts == committed.debts),
                    ("techs", back.techs == committed.techs),
                    ("offers", back.offers == committed.offers),
                    ("practices", back.practices == committed.practices),
                    ("routes", back.routes == committed.routes),
                    ("notes", back.notes == committed.notes),
                    (
                        "treasury",
                        back.treasury_balances == committed.treasury_balances,
                    ),
                    ("leisure", back.leisure_banks == committed.leisure_banks),
                    ("pop_accrued", back.pop_accrued == committed.pop_accrued),
                    ("collapsed", back.collapsed == committed.collapsed),
                    ("prospects", back.prospects == committed.prospects),
                    ("stations", back.stations == committed.stations),
                ]
                .iter()
                .filter(|(_, same)| !same)
                .map(|(n, _)| *n)
                .collect();
                println!(
                    "{}: round-trip DIFFERS ({} bytes) in {:?}",
                    d.display(),
                    bytes.len(),
                    diff
                );
            }
            None => {
                bad += 1;
                println!("{}: does not decode", d.display());
            }
        }
    }
    println!("round-trip exact: {ok}, differs: {bad}");
}
