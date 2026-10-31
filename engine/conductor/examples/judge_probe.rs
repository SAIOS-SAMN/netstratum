//! Judge probe (task #7) — re-judge live chronicles deed by deed and
//! print each refusal with its context, so the three live tallies
//! (NoSuchEntity · ConservationBroken · DecayExceedsBound) resolve to
//! named receipts instead of counts. Opt-in, appends nothing, judges
//! nothing into the chain — the cost_probe discipline.

fn main() {
    let root = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "var/worlds".into());
    let limit: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(24);
    let mut shown = 0usize;
    let mut worlds = 0usize;
    let Ok(entries) = std::fs::read_dir(&root) else {
        println!("no such root: {root}");
        return;
    };
    let mut names: Vec<_> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.join("chronicle.tlv").exists())
        .collect();
    names.sort();
    for dir in names {
        if shown >= limit {
            break;
        }
        let Ok(chronicle) = cfe::persist::load(&dir.join("chronicle.tlv")) else {
            continue;
        };
        worlds += 1;
        let receipts = chronicle.receipts().to_vec();
        let mut prior =
            cfe::world::replay(&cfe::vrp::Chronicle::from_receipts(Vec::new()).expect("empty"));
        for at in 0..receipts.len() {
            if shown >= limit {
                break;
            }
            let next = cfe::world::replay(
                &cfe::vrp::Chronicle::from_receipts(receipts[..=at].to_vec()).expect("prefix"),
            );
            let receipt = &receipts[at];
            let step = kernel::Step {
                event: receipt.event().clone(),
                actor: receipt
                    .provenance()
                    .first()
                    .copied()
                    .unwrap_or(cfe::world::SOVEREIGN),
                stamp: receipt.stamp(),
            };
            if let Err(unlawful) = kernel::admits(&prior, &next, &step) {
                shown += 1;
                let world_name = dir.file_name().and_then(|s| s.to_str()).unwrap_or("?");
                println!(
                    "{world_name} · seq {} · stamp {} · {:?}\n    deed: {:?}\n    prior: entities={} orbs={} star={:?}",
                    receipt.sequence(),
                    receipt.stamp(),
                    unlawful,
                    brief(receipt.event()),
                    prior.entities.len(),
                    prior.orbs.len(),
                    prior.star,
                );
                if let cfe::dop::ClosureEvent::DemurrageTouched { orb_id, .. }
                | cfe::dop::ClosureEvent::DemurrageSettled { orb_id, .. } = receipt.event()
                {
                    if let Some(orb) = prior.orbs.iter().find(|o| o.orb_id == *orb_id) {
                        println!(
                            "    orb: last_touch={} banked={:?}",
                            orb.last_touch,
                            orb.banked
                                .iter()
                                .map(|(r, a)| format!("{r:?}:{a}"))
                                .collect::<Vec<_>>()
                        );
                    }
                }
            }
            prior = next;
        }
    }
    println!("---\nprobed {worlds} worlds · showed {shown} refusals (limit {limit})");
}

fn brief(event: &cfe::dop::ClosureEvent) -> String {
    let text = format!("{event:?}");
    text.chars().take(220).collect()
}
