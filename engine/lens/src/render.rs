//! THE RENDERERS (#37 — moved whole from the ledger at the split):
//! the state card and the receipt line, projections of a live World
//! into display JSON. One producer of every rendering — the ledger's
//! display half CALLS these; nothing ever parses them back.

use crate::json::{ratio_str, JsonWriter};
use cfe::ratio::Ratio;
use cfe::vrp::Receipt;
use cfe::world::World;

/// Hex of a digest — display form of the chain's bytes.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The six-field base-60 odometer: T of TUniversal, a pure function
/// of the tick count — never of any wall clock.
pub fn universal_clock(ticks: u64) -> String {
    let mut fields = [0u64; 6];
    let mut rest = ticks;
    for slot in (0..6).rev() {
        fields[slot] = rest % 60;
        rest /= 60;
    }
    fields
        .iter()
        .map(|f| format!("{f:02}"))
        .collect::<Vec<_>>()
        .join(":")
}

fn resource_name(r: cfe::dop::Resource) -> &'static str {
    match r {
        cfe::dop::Resource::Iron => "iron",
        cfe::dop::Resource::Biomass => "biomass",
        cfe::dop::Resource::Alloy => "alloy",
        cfe::dop::Resource::Diamond => "diamond",
        cfe::dop::Resource::Joule => "joule",
        cfe::dop::Resource::Sink => "sink",
        cfe::dop::Resource::Crystal => "crystal",
    }
}

fn zone_name(k: cfe::dop::ZoneKind) -> &'static str {
    match k {
        cfe::dop::ZoneKind::Extractor => "extractor",
        cfe::dop::ZoneKind::Habitat => "habitat",
        cfe::dop::ZoneKind::Smelter => "smelter",
        cfe::dop::ZoneKind::Synthesizer => "synthesizer",
        cfe::dop::ZoneKind::Encasement => "encasement",
        cfe::dop::ZoneKind::CryoArray => "cryo-array",
        cfe::dop::ZoneKind::Resonator => "resonator",
    }
}

/// The base-7 stack's concept words (10 §2: the condition fixes the
/// CONCEPT; the node hash draws the voice — the uplink carries the
/// concept, the chronicle carries the voice).
fn condition_word(condition: u8) -> &'static str {
    match condition {
        0 => "solar",
        1 => "insulation",
        2 => "discipline",
        3 => "deepening",
        4 => "doctrine",
        5 => "stores",
        6 => "terraforming",
        _ => "working",
    }
}

fn scope_name(s: cfe::ipsal::Scope) -> &'static str {
    match s {
        cfe::ipsal::Scope::Build => "build",
        cfe::ipsal::Scope::Raze => "raze",
        cfe::ipsal::Scope::Collect => "collect",
    }
}

/// WANT-1: the kind table's names (tag 62).
fn want_name(kind: u8) -> &'static str {
    match kind {
        1 => "epistemological",
        2 => "resonance",
        3 => "novelty",
        _ => "unnamed",
    }
}

pub fn render_state(world_name: &str, planet: &str, civilization: &str, world: &World) -> String {
    let mut w = JsonWriter::new();
    w.open();
    w.field_str("civilization", civilization);
    w.field_str("world", world_name);
    w.field_str("planet", planet);
    w.field_u64("clock", world.now());
    w.field_u64("receipts", world.chronicle().receipts().len() as u64);

    if let Some(class) = world.star() {
        w.field_u64("star_class", class as u8 as u64);
    }
    // MARKET-1 / observer uplink (BOARD-1): the resting offers, data
    // fields the boards render — never prose to parse.
    if !world.offers().is_empty() {
        w.key("offers");
        w.raw_array(world.offers().iter().map(|offer| {
            let mut entry = JsonWriter::new();
            entry.open();
            entry.field_u64("offer", offer.offer);
            entry.field_str("give", resource_name(offer.give));
            entry.field_str("give_amount", &ratio_str(&offer.give_amount));
            entry.field_str("want", resource_name(offer.want));
            entry.field_str("want_amount", &ratio_str(&offer.want_amount));
            entry.close();
            entry.finish()
        }));
    }
    // CONSTEL-1 / observer uplink: the base-7 sky — a derived
    // reading (held/progress/threshold per condition), never stored.
    if let Some(orb) = world.orb() {
        w.key("constellation");
        w.raw_array(world.constellation(orb.orb_id).into_iter().map(
            |(condition, held, progress, threshold)| {
                let mut entry = JsonWriter::new();
                entry.open();
                entry.field_u64("condition", u64::from(condition));
                entry.field_u64("held", u64::from(held));
                entry.field_u64("progress", progress);
                entry.field_u64("threshold", threshold);
                entry.close();
                entry.finish()
            },
        ));
    }
    // MANU-1 / observer uplink: the forged instruments — the reach
    // law reads the best grade (reach = base + grade, 10 §3f).
    if !world.instruments().is_empty() {
        w.key("instruments");
        w.raw_array(world.instruments().iter().map(|(orb, node, grade)| {
            let mut entry = JsonWriter::new();
            entry.open();
            entry.field_u64("orb", *orb);
            entry.field_u64("grade", u64::from(*grade));
            entry.field_str(
                "node",
                &format!("{:02x}{:02x}{:02x}{:02x}", node[0], node[1], node[2], node[3]),
            );
            entry.close();
            entry.finish()
        }));
    }
    // WANT-1 / observer uplink: the struck cognitive wants.
    if !world.wants().is_empty() {
        w.key("wants");
        w.array(world.wants().iter().map(|(_, kind)| want_name(*kind).to_string()));
    }
    if let Some(orb) = world.orb() {
        w.field_str("colony", &orb.name);
        // VENT-1 / observer uplink: the dual ledger and the derived
        // (never stored) syntropy ratio.
        w.field_str("entropy_vented", &ratio_str(&orb.entropy_vented));
        let banked_j = orb.banked_amount(cfe::dop::Resource::Joule);
        let denominator = banked_j.clone() + orb.entropy_vented.clone();
        if denominator > Ratio::from(num_bigint::BigInt::from(0)) {
            w.field_str("syntropy_ratio", &format!(
                "{}/{}",
                ratio_str(&banked_j),
                ratio_str(&denominator)
            ));
        }
        // IDEO-1 / observer uplink: the eight committed cohesion books
        // and the four derived (never stored) axis ratios — emitted
        // only when a book is nonzero, like the wire entry.
        let axes: [(&str, &str, &Ratio, &str, &Ratio); 4] = [
            ("compassion", "given_in_crisis", &orb.given_in_crisis,
             "drawn_in_crisis", &orb.drawn_in_crisis),
            ("love", "bond_flow", &orb.bond_flow,
             "severance_load", &orb.severance_load),
            ("integrity", "covenants_fulfilled", &orb.covenants_fulfilled,
             "covenants_breached", &orb.covenants_breached),
            ("morality", "returned_across", &orb.returned_across,
             "extracted_across", &orb.extracted_across),
        ];
        let zero = Ratio::from(num_bigint::BigInt::from(0));
        if axes.iter().any(|(_, _, syn, _, ent)| **syn != zero || **ent != zero) {
            w.key("cohesion");
            w.open();
            for (axis, syn_name, syn, ent_name, ent) in axes {
                w.field_str(syn_name, &ratio_str(syn));
                w.field_str(ent_name, &ratio_str(ent));
                if let Some(reading) = cfe::dop::cohesion_ratio(syn, ent) {
                    w.field_str(&format!("{axis}_ratio"), &ratio_str(&reading));
                }
            }
            w.close();
        }
        // CONSTEL-1 / observer uplink (the deferred export, paid):
        // the base-7 stack per orb — condition · held · progress ·
        // threshold, the crystallizer's OWN deposit-sheet reading
        // (never a second bookkeeping). Counters up, ratios derived
        // surface-side; emitted only when something has begun.
        let constellation = world.constellation(orb.orb_id);
        if constellation.iter().any(|(_, held, progress, _)| *held || *progress > 0) {
            w.key("constellation");
            w.open_array();
            for (condition, held, progress, threshold) in constellation {
                w.open();
                w.field_str("condition", condition_word(condition));
                w.field_u64("held", u64::from(held));
                w.field_u64("progress", progress);
                w.field_u64("threshold", threshold);
                w.close();
            }
            w.close_array();
        }
        w.key("banked");
        w.open();
        for (resource, amount) in &orb.banked {
            w.field_str(resource_name(*resource), &ratio_str(amount));
        }
        w.close();
        w.key("famine");
        w.array(orb.famine.iter().map(|r| resource_name(*r).to_string()));
        w.key("zones");
        w.open_array();
        for zone in &orb.zones {
            w.open();
            w.field_u64("tile", zone.tile as u64);
            w.field_str("kind", zone_name(zone.kind));
            w.close();
        }
        w.close_array();
    }

    w.key("accrued");
    w.open();
    for (resource, amount) in &world.accrued() {
        w.field_str(resource_name(*resource), &ratio_str(amount));
    }
    w.close();

    w.key("entities");
    w.open_array();
    for entity in world.entities() {
        w.open();
        w.field_u64("id", entity.entity_id);
        w.field_str("handle", &entity.handle);
        match entity.principal {
            None => w.field_raw("principal", "null"),
            Some(p) => w.field_u64("principal", p),
        }
        w.key("grants");
        w.array(entity.grants.iter().map(|s| scope_name(*s).to_string()));
        w.close();
    }
    w.close_array();
    w.close();
    w.finish()
}

pub fn render_receipt(receipt: &Receipt) -> String {
    use cfe::dop::ClosureEvent as E;
    let mut w = JsonWriter::new();
    w.open();
    w.field_u64("seq", receipt.sequence);
    w.field_u64("stamp", receipt.stamp);
    w.field_str("parent", &hex(&receipt.parent.0));
    w.field_str("digest", &hex(&receipt.own.0));
    w.key("by");
    w.array(receipt.provenance.iter().map(|p| p.to_string()));
    let (name, detail): (&str, String) = match &receipt.event {
        E::ColonyFounded { name, .. } => ("colony_founded", name.clone()),
        // BOOK-1 (19_LENS §16) — the open registry's three deeds. The
        // kind is shown truncated: it is an identity, not a reading, and
        // a full digest in a display line teaches nobody anything.
        E::BookMinted { name, kind, .. } => (
            "book_minted",
            format!("{name} ({})", &hex(&kind.0)[..8]),
        ),
        E::BookPosted { kind, kept, lost, .. } => (
            "book_posted",
            format!(
                "{} kept / {} lost on {}",
                ratio_str(kept),
                ratio_str(lost),
                &hex(&kind.0)[..8]
            ),
        ),
        // Closed to WRITING only — the display says so, because the
        // word "archived" alone reads as gone (§16c).
        E::BookArchived { kind, .. } => (
            "book_archived",
            format!("{} closed to writing", &hex(&kind.0)[..8]),
        ),
        E::ZoneBuilt { tile, kind, .. } => ("zone_built", format!("{} @ {tile}", zone_name(*kind))),
        E::YieldCollected { resource, amount, .. } => (
            "yield_collected",
            format!("{} {}", ratio_str(amount), resource_name(*resource)),
        ),
        E::DiscoveryMinted { resource, amount, work, .. } => (
            "discovery_minted",
            format!(
                "{} {} against {} bytes of verified work",
                ratio_str(amount),
                resource_name(*resource),
                work.len()
            ),
        ),
        E::UpkeepSettled { resource, amount, .. } => (
            "upkeep_settled",
            format!("{} {}", ratio_str(amount), resource_name(*resource)),
        ),
        E::FamineStruck { resource, .. } => ("famine_struck", resource_name(*resource).into()),
        E::FamineRelieved { resource, .. } => ("famine_relieved", resource_name(*resource).into()),
        E::ZoneRazed { tile, .. } => ("zone_razed", format!("tile {tile}")),
        E::EntityRegistered { handle, .. } => ("entity_registered", handle.clone()),
        E::ScopeGranted { agent, scope } => {
            ("scope_granted", format!("{} to #{agent}", scope_name(*scope)))
        }
        E::ScopeRevoked { agent, scope } => {
            ("scope_revoked", format!("{} from #{agent}", scope_name(*scope)))
        }
        E::ColonyCharted { orb_id, source_orb, name } => (
            "colony_charted",
            format!("{name} (orb {orb_id}) from orb {source_orb}"),
        ),
        E::EraAnnexed { era, world, offset, .. } => (
            "era_annexed",
            format!("{world} of {era} at offset {offset}"),
        ),
        E::GalaxyObserved { observer, name, .. } => (
            "galaxy_observed",
            format!("{name} observed by {observer}"),
        ),
        E::IncursionStruck { era, world, line } => (
            "incursion_struck",
            format!("{world} of {era} wakes as {line}"),
        ),
        E::TaxonSealed { class, clade } => (
            "taxon_sealed",
            format!("class {class} · clade {clade} — identity sealed"),
        ),
        E::StationSeated { line, station } => (
            "station_seated",
            format!("{line} takes station {station} — the seat enters the chain"),
        ),
        E::ElderWarded {} => ("elder_warded", "the reverse shield seats".into()),
        E::WardLifted { claimer } => (
            "ward_lifted",
            format!("the depth gate opened by {claimer}"),
        ),
        E::VeilDrawn {} => ("veil_drawn", "the renders cloud; the chronicle never lies".into()),
        E::VeilLifted { starved } => (
            "veil_lifted",
            if *starved { "STARVED — the toll went unpaid".into() } else { "by deed".into() },
        ),
        E::PracticeAdopted { practice } => (
            "practice_adopted",
            format!("discipline {practice} bound by receipt"),
        ),
        E::PracticeReleased { practice } => (
            "practice_released",
            format!("discipline {practice} released — the lesson re-emerges"),
        ),
        E::InstrumentForged { orb_id, grade, .. } => (
            "instrument_forged",
            format!("orb {orb_id} forges its grade-{grade} instrument — the horizon grows"),
        ),
        E::VoidShare { deal, amount, .. } => (
            "void_share",
            format!("deal {deal}: the void's exact share — {} melted in transit", ratio_str(amount)),
        ),
        E::RouteChartered { deal, eta } => (
            "route_chartered",
            format!("lane {deal} chartered at η {eta} — the path is paid for"),
        ),
        E::RouteLapsed { deal } => (
            "route_lapsed",
            format!("lane {deal} LAPSES — raw space takes the path back"),
        ),
        E::WorldCharted { seed } => (
            "world_charted",
            format!(
                "terrain seed {:02x}{:02x}{:02x}{:02x}",
                seed[0], seed[1], seed[2], seed[3]
            ),
        ),
        E::OverheatStruck { overflow, .. } => {
            ("overheat_struck", format!("overflow {}", ratio_str(overflow)))
        }
        E::OverheatCleared { .. } => ("overheat_cleared", String::new()),
        E::StarCommissioned { class } => ("star_commissioned", format!("class {class}")),
        E::PlanetSited { orb_id, albedo, orbit_r } => (
            "planet_sited",
            format!("orb {orb_id} A={} r={}", ratio_str(albedo), ratio_str(orbit_r)),
        ),
        E::VentSettled { amount, .. } => ("vent_settled", ratio_str(amount)),
        E::DemurrageSettled { resource, amount, .. } => (
            "demurrage_settled",
            format!("{} {}", ratio_str(amount), resource_name(*resource)),
        ),
        E::ResourceTransferred { from_orb, to_orb, resource, amount } => (
            "resource_transferred",
            format!(
                "{} {} orb {from_orb} → orb {to_orb}",
                ratio_str(amount),
                resource_name(*resource)
            ),
        ),
        E::TechDiscovered { orb_id, condition, name, .. } => (
            "tech_discovered",
            format!("{name} — orb {orb_id} condition {condition}"),
        ),
        E::TechLicensed { deal, from_orb, council, condition, name, .. } => (
            "tech_licensed",
            format!(
                "{name} licensed out — orb {from_orb}, deal {deal}, council {council}, condition {condition}"
            ),
        ),
        E::CouncilSealed { council, resource, quota, window, collateral, members } => (
            "council_sealed",
            format!(
                "council {council}: {} members bound alike — {} {} quota, {} collateral, {window} ticks",
                members.len(),
                ratio_str(quota),
                resource_name(*resource),
                ratio_str(collateral)
            ),
        ),
        E::CouncilClosed { council, member, fulfilled, banked } => (
            "council_closed",
            format!(
                "council {council}: orb {member} {} — {} banked home",
                match fulfilled {
                    true => "kept faith",
                    false => "breached",
                },
                ratio_str(banked)
            ),
        ),
        E::WantStruck { orb_id, kind, reading } => (
            "want_struck",
            format!(
                "the {} want strikes orb {orb_id} — {} ticks starved",
                want_name(*kind),
                ratio_str(reading)
            ),
        ),
        E::WantRelieved { orb_id, kind } => (
            "want_relieved",
            format!("the {} want relieved — orb {orb_id}'s mind is fed", want_name(*kind)),
        ),
        E::OfferPosted { offer, give, give_amount, want, want_amount, .. } => (
            "offer_posted",
            format!(
                "offer {offer} rests: {} {} for {} {}",
                ratio_str(give_amount),
                resource_name(*give),
                ratio_str(want_amount),
                resource_name(*want)
            ),
        ),
        E::OfferCleared { offer, deal } => (
            "offer_cleared",
            format!("offer {offer} cleared — the crossing filled it (deal {deal})"),
        ),
        E::GaugeShiftedAt { grade, gauge } => (
            "gauge_shifted_at",
            format!("the key moves at grade {grade} — {}", ratio_str(gauge)),
        ),
        E::GaugeShifted { gauge } => {
            ("gauge_shifted", format!("the key moves to {}", ratio_str(gauge)))
        }
        E::Diverged { branch, gauge } => (
            "diverged",
            format!("cleaved — branch {branch} departs; the key moves to {}", ratio_str(gauge)),
        ),
        E::Confluent { partner, gauge } => (
            "confluent",
            format!("combined with {partner}; the key settles at {}", ratio_str(gauge)),
        ),
        E::StarAged { class } => ("star_aged", format!("the star moves — class {class}")),
        E::EraFolded { era, gauge } => (
            "era_folded",
            format!("aeon {era} folds; the new sheet seats at {}", ratio_str(gauge)),
        ),
        E::ApertureSent { deal, from_orb, resource, amount } => (
            "aperture_sent",
            format!(
                "{} {} orb {from_orb} → deal {deal}",
                ratio_str(amount),
                resource_name(*resource)
            ),
        ),
        E::ApertureReceived { deal, to_orb, resource, amount } => (
            "aperture_received",
            format!(
                "{} {} deal {deal} → orb {to_orb}",
                ratio_str(amount),
                resource_name(*resource)
            ),
        ),
        E::OverflowAbsorbed { source_orb, absorber_orb, amount } => (
            "overflow_absorbed",
            format!(
                "{} heat orb {source_orb} → orb {absorber_orb}'s sink",
                ratio_str(amount)
            ),
        ),
        E::AbsorptionLapsed { debtor_orb, creditor_orb, amount } => (
            "absorption_lapsed",
            format!(
                "{} unpaid orb {debtor_orb} → orb {creditor_orb}",
                ratio_str(amount)
            ),
        ),
        E::BondSealed { source_orb, target_orb, resource, rate, term } => (
            "bond_sealed",
            format!(
                "{} {}/tick orb {source_orb} → orb {target_orb} (term {term})",
                ratio_str(rate),
                resource_name(*resource)
            ),
        ),
        E::BondSettled { source_orb, target_orb, resource, amount, severed, load } => (
            "bond_settled",
            format!(
                "{} {} orb {source_orb} → orb {target_orb}{}",
                ratio_str(amount),
                resource_name(*resource),
                match severed {
                    true => format!(" SEVERED (load {})", ratio_str(load)),
                    false => String::new(),
                }
            ),
        ),
        E::CovenantSealed { obligor_orb, beneficiary_orb, resource, quota, window, collateral } => (
            "covenant_sealed",
            format!(
                "{} {} orb {obligor_orb} → orb {beneficiary_orb} in {window} (collateral {})",
                ratio_str(quota),
                resource_name(*resource),
                ratio_str(collateral)
            ),
        ),
        E::CovenantReleased { obligor_orb, beneficiary_orb, resource, collateral } => (
            "covenant_released",
            format!(
                "{} {} home to orb {obligor_orb} (pact with orb {beneficiary_orb})",
                ratio_str(collateral),
                resource_name(*resource)
            ),
        ),
        E::CovenantSlashed { obligor_orb, beneficiary_orb, resource, collateral } => (
            "covenant_slashed",
            format!(
                "{} {} forfeit orb {obligor_orb} → orb {beneficiary_orb}",
                ratio_str(collateral),
                resource_name(*resource)
            ),
        ),
        E::FrictionSettled { orb_id, amount } => (
            "friction_settled",
            format!("{} entropy on orb {orb_id}", ratio_str(amount)),
        ),
        E::CrystalDrawn { orb_id, amount } => (
            "crystal_drawn",
            format!("{} crystal held the lens open on orb {orb_id}", ratio_str(amount)),
        ),
        E::LineSucceeded { successor } => (
            "line_succeeded",
            format!("{successor} adopts this chronicle as its own substrate"),
        ),
        E::GenesisSealed { civilization, disposition, metabolism, perception } => (
            "genesis_sealed",
            format!(
                "{civilization} seals its axes ({disposition}/{metabolism}/{perception}) — the registry is a cache from here on"
            ),
        ),
        E::HandoffSealed { from, to } => (
            "handoff_sealed",
            format!("hand {from} yields the pen to hand {to}"),
        ),
        E::ProspectStruck { orb_id, stratum, name, family, tier, .. } => (
            "prospect_struck",
            format!(
                "{name} read from stratum {stratum} on orb {orb_id} (family {family}, tier {tier}) — what the seed always held"
            ),
        ),
        E::AssayStruck { orb_id, amount, .. } => (
            "assay_struck",
            format!("{} drawn from the opened vein on orb {orb_id}", ratio_str(amount)),
        ),
    };
    w.field_str("event", name);
    w.field_str("detail", &detail);
    w.field_str("tlv", &hex(&receipt.event.wire_bytes()));
    w.close();
    w.finish()
}
