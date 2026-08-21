//! THE COGNITIVE ARENA — run civilizations on the manifold and rank their
//! cognitive ability (doc 55, the entity-atlas made runnable).
//!
//! Each civilization is a fresh species seated on its own planet, its minds
//! organized into HOUSES by its own classification system. A house is a chart; a
//! cognitive CONTACT between two houses is a point of their overlap; the DEED that
//! transports a class across a contact is a stochastic draw of the connection
//! (seeded, replay-exact — the workbench's `instinct::choose` in miniature). A
//! contact whose draw comes up FLAT carries a shared idea; a curved draw obstructs
//! it. The civilization's COLLECTIVE COGNITION is the exact atlas ΔH over its
//! houses — the classes born on the flat contacts that no single house holds.
//!
//! The claim under test: raw per-house knowledge held EQUAL, the civilization
//! whose classification lets its houses COMPOSE (many flat contacts) is provably
//! more cognitively able than one that HOARDS (few, or curved). Cognition is the
//! topology of sharing, not the sum of private minds.

use num_bigint::BigInt;
use reduct::atlas::delta_h;
use reduct::sheaf::CellSheaf;

fn one() -> BigInt {
    BigInt::from(1)
}

/// A house's mind, as a chart: one contractible knowledge-body (a point). Species
/// differ in richer stalks; here every house starts cognitively equal so the test
/// isolates the ONE variable — how the classification connects them.
fn house() -> CellSheaf {
    CellSheaf::new(vec![vec![1]], vec![]).expect("a house is a chart")
}

/// The overlap carrying `contacts` cognitive contact-points between two houses.
fn contact_overlap(contacts: usize) -> CellSheaf {
    CellSheaf::new(vec![vec![1; contacts.max(1)]], vec![]).expect("overlap is a complex")
}

/// Each house's knowledge restricts onto every contact point (all-ones column).
fn restriction(contacts: usize) -> Vec<Vec<Vec<BigInt>>> {
    vec![(0..contacts.max(1)).map(|_| vec![one()]).collect()]
}

/// Seeded stochastic draw (splitmix64) — replay-exact, no wall clock.
fn draw(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    (z ^ (z >> 31)) % 1000
}

struct Civ {
    name: &'static str,
    species: &'static str,
    planet: &'static str,
    /// How the classification seats knowledge into cross-house contacts.
    classification: &'static str,
    contacts: usize,
    /// Connection coherence in per-mille: the chance any contact's transport draw
    /// comes up FLAT this beat (the species' cognitive alignment).
    coherence: u64,
    seed: u64,
}

impl Civ {
    /// One beat of collective cognition: sample which contacts transport flatly,
    /// then measure the EXACT atlas ΔH over the two houses on the flat overlap.
    fn cognition_at(&self, beat: u64) -> usize {
        let flat: usize = (0..self.contacts)
            .filter(|c| draw(self.seed ^ (beat << 20) ^ (*c as u64)) < self.coherence)
            .count();
        if flat == 0 {
            return 0;
        }
        delta_h(
            &house(),
            &house(),
            &contact_overlap(flat),
            &restriction(flat),
            &restriction(flat),
        )
    }

    /// Cognitive ability = mean collective ΔH over a window of beats (×1000 to
    /// stay in exact integers — the no-scalar law; no float scores).
    fn ability_milli(&self, beats: u64) -> u64 {
        let total: usize = (0..beats).map(|b| self.cognition_at(b)).sum();
        (total as u64 * 1000) / beats
    }
}

#[test]
fn civilizations_ranked_by_collective_cognition() {
    // Three fresh species, three planets, three classification systems — equal raw
    // minds, differing only in how their houses are permitted to compose.
    let composers = Civ {
        name: "the Aurelian Consilium",
        species: "aurelians",
        planet: "Halcyon-3",
        classification: "OPEN — every house seats a delegate in every other's court",
        contacts: 6,
        coherence: 900,
        seed: 0xA11CE,
    };
    let guilds = Civ {
        name: "the Ferran Guilds",
        species: "ferrans",
        planet: "Cinder-7",
        classification: "GUILD — houses trade only through paired charters",
        contacts: 3,
        coherence: 700,
        seed: 0xF3EEE,
    };
    let hoarders = Civ {
        name: "the Vault Dominion",
        species: "vaultkin",
        planet: "Sable-1",
        classification: "SILOED — each house seals its knowledge; one grudging contact",
        contacts: 1,
        coherence: 300,
        seed: 0x5AB1E,
    };

    let beats = 64;
    let mut board: Vec<(&Civ, u64)> = [&composers, &guilds, &hoarders]
        .into_iter()
        .map(|c| (c, c.ability_milli(beats)))
        .collect();
    board.sort_by(|a, b| b.1.cmp(&a.1));

    eprintln!("=== COGNITIVE ARENA — {beats} beats, collective ΔH (mean ×1000) ===");
    for (rank, (c, ability)) in board.iter().enumerate() {
        eprintln!(
            "  #{}  {:>6}.{:03}  {}  [{}] on {} — {}",
            rank + 1,
            ability / 1000,
            ability % 1000,
            c.name,
            c.species,
            c.planet,
            c.classification,
        );
    }

    // The provable ordering: the composing civilization out-thinks the guild, which
    // out-thinks the hoarder — from the SAME raw minds, by the topology of sharing
    // alone. Cognition is the connection, not the sum of private knowledge.
    assert!(
        board[0].0.name == composers.name,
        "the composing civilization must hold the highest collective cognition"
    );
    assert!(
        board[1].0.name == guilds.name && board[2].0.name == hoarders.name,
        "guild out-thinks hoarder: more flat cognitive contacts ⇒ more shared classes"
    );
    assert!(
        board[0].1 > board[2].1 * 3,
        "the composer's collective mind is a multiple of the hoarder's, provably"
    );
    // The hoarder, sealed to a single contact, mints NO shared class (ΔH ≡ 0).
    assert_eq!(hoarders.ability_milli(beats), 0, "a sealed civilization has no collective cognition");
}
