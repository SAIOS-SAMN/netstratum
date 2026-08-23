//! PERCEPTION CENSUS — count the registry's civilizations by `perception`
//! axis. Read-only: lists registry cards, no mutation.
//!
//! ```text
//! cargo run --release -p conductor --example perception_census
//! ```

fn main() {
    let registry = ledger::registry::Registry::at("var/registry");
    let cards = registry.list();
    let mut counts: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for card in &cards {
        *counts.entry(card.perception.clone()).or_default() += 1;
    }
    println!("total registered civilizations: {}", cards.len());
    for (perception, n) in &counts {
        println!("  {perception}: {n}");
    }
}
