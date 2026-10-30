//! Root hubs — peers (see `docs/HUBS.md`).
//!
//! | Root | Role |
//! |------|------|
//! | **bench** | Build components (every activity root has craft leaves here) |
//! | **intel** | Knowledge activity |
//! | **trade** | Circulate / market activity |
//! | **diplomatic** | Diplomacy activity (local + stellar); short alias: `diplo` |
//!
//! Dual law: every standalone activity root **S** has craft surface **`bench S`**.
//! | Activity | Craft twin |
//! |----------|------------|
//! | `intel`       | `bench intel`       |
//! | `trade`       | `bench trade`       |
//! | `diplomatic`  | `bench diplomatic`  (`bench diplo` alias) |
//!
//! Science craft (under bench, not activity roots):
//! `bench chem` · `bench biology` · `bench physics` · `bench math` · `bench research`

use cfe::dop::{HullRole, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::world::World;

use crate::style::Style;

fn amount(value: &Ratio) -> String {
    format!("{}/{}", value.numer(), value.denom())
}

fn hex_prefix(bytes: &[u8; 32]) -> String {
    bytes.iter().take(4).map(|b| format!("{b:02x}")).collect()
}

/// Resolve a banked or prospected element by hex id prefix (deep hub deeds).
fn resolve_element_prefix(world: &World, orb: u64, prefix: &str) -> Option<[u8; 32]> {
    let pref = prefix.to_ascii_lowercase();
    if let Some(held) = world.orb_by_id(orb).or_else(|| world.orb()) {
        for (el, _) in &held.elements {
            if hex_prefix(el).starts_with(&pref) || hex_prefix(el).contains(&pref) {
                return Some(*el);
            }
            // Full hex start.
            let full: String = el.iter().map(|b| format!("{b:02x}")).collect();
            if full.starts_with(&pref) {
                return Some(*el);
            }
        }
    }
    for p in world.committed().prospects.iter().filter(|p| p.orb == orb) {
        let full: String = p.element.iter().map(|b| format!("{b:02x}")).collect();
        if full.starts_with(&pref) {
            return Some(p.element);
        }
    }
    None
}

fn banner(style: &Style, title: &str) {
    println!(
        "{}",
        style.border("════════════════════════════════════════════════════════════")
    );
    println!("  {}", style.title(title));
    println!(
        "{}",
        style.border("════════════════════════════════════════════════════════════")
    );
}

fn row(style: &Style, verb: &str, desc: &str) {
    let plain_w = 42usize;
    let pad = if verb.len() < plain_w {
        " ".repeat(plain_w - verb.len())
    } else {
        String::new()
    };
    println!("  {}{}{}", style.verb(verb), pad, style.dim(desc));
}

fn leaf(style: &Style, verb: &str, desc: &str) {
    println!("    {}  {}", style.key(verb), style.dim(desc));
}

fn is_back(word: Option<&str>) -> bool {
    matches!(
        word,
        Some("back")
            | Some("..")
            | Some("up")
            | Some("home")
            | Some("return")
            | Some("root")
            | Some("exit")
            | Some("q")
    )
}

fn peers_line(style: &Style) {
    leaf(
        style,
        "bench · intel · trade · diplomatic · polity",
        "peer roots (diplo alias · polity optional 5th)",
    );
}

fn foot_bench(style: &Style, parent: Option<&str>) {
    println!();
    println!("  {}", style.section("↩ BACK-OUT"));
    leaf(style, "bench", "← BUILD root");
    if let Some(p) = parent {
        leaf(style, &format!("bench {p}"), "← domain");
    }
    leaf(style, "back · .. · home", "bench root");
    peers_line(style);
}

fn foot_intel(style: &Style, parent: Option<&str>) {
    println!();
    println!("  {}", style.section("↩ BACK-OUT"));
    leaf(style, "intel", "← KNOWLEDGE root");
    if let Some(p) = parent {
        leaf(style, &format!("intel {p}"), "← domain");
    }
    leaf(style, "back · .. · home", "intel root");
    leaf(style, "bench intel", "← craft tools for this activity");
    peers_line(style);
}

fn foot_trade(style: &Style, parent: Option<&str>) {
    println!();
    println!("  {}", style.section("↩ BACK-OUT"));
    leaf(style, "trade", "← CIRCULATE root");
    if let Some(p) = parent {
        leaf(style, &format!("trade {p}"), "← domain");
    }
    leaf(style, "back · .. · home", "trade root");
    leaf(style, "bench trade", "← craft components for trade");
    peers_line(style);
}

fn foot_diplomatic(style: &Style, parent: Option<&str>) {
    println!();
    println!("  {}", style.section("↩ BACK-OUT"));
    leaf(style, "diplomatic", "← DIPLOMACY root (alias: diplo)");
    if let Some(p) = parent {
        leaf(style, &format!("diplomatic {p}"), "← domain");
    }
    leaf(style, "back · .. · home", "diplomatic root");
    leaf(style, "bench diplomatic", "← craft diplomatic components");
    peers_line(style);
}

/// Prefer the unambiguous name in UI copy; accept short `diplo` as alias.
fn diplomatic_key() -> &'static str {
    "diplomatic"
}

// ════════════════════════════════════════════════════════════════════
// bench — build
// ════════════════════════════════════════════════════════════════════

pub fn bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let domain = words.next();
    if is_back(domain.as_deref()) {
        bench_index(style);
        return;
    }
    match domain.as_deref() {
        None | Some("help") | Some("?") | Some("menu") | Some("index") => bench_index(style),
        Some("planetary") | Some("planet") | Some("world") => {
            planetary(world, words, style, current)
        }
        Some("build") | Some("infra") | Some("zones") => build_bench(world, words, style, current),
        Some("star") | Some("sky") | Some("system") => star_bench(world, words, style, current),
        Some("estate") | Some("survey") | Some("land") => {
            estate_bench(world, words, style, current)
        }
        // Science craft disciplines (components intel research then operates).
        Some("science") | Some("sci") | Some("disciplines") => {
            science_index(world, words, style, current)
        }
        Some("chem") | Some("chemistry") | Some("bond") | Some("alchemy") => {
            chem_bench(world, words, style, current)
        }
        Some("biology") | Some("bio") | Some("biologic") | Some("life") => {
            biology_bench(world, words, style, current)
        }
        Some("physics") | Some("phys") | Some("physical") => {
            physics_bench(world, words, style, current)
        }
        Some("math") | Some("maths") | Some("mathematics") | Some("ratios") => {
            math_bench(world, words, style, current)
        }
        Some("research") | Some("tech") | Some("practice") | Some("lab") | Some("constellation") => {
            research_craft_bench(world, words, style, current)
        }
        Some("econ") | Some("economics") | Some("craft") => {
            econ_bench(world, words, style, current)
        }
        Some("offices") | Some("office") | Some("seat") => {
            offices_bench(world, words, style, current)
        }
        Some("fleet") | Some("hull") | Some("ship") | Some("yard") => {
            fleet_craft_bench(world, words, style, current)
        }
        Some("doctrine") | Some("posture") | Some("defense") | Some("kin") => {
            doctrine_bench(world, words, style, current)
        }
        Some("medium") | Some("atm") | Some("envelope") => {
            medium_bench(world, words, style, current)
        }
        Some("assay") | Some("court") => assay_bench(world, words, style, current),
        // Dual law: activity root S ⇒ craft surface `bench S`.
        Some("diplomatic")
        | Some("diplomacy")
        | Some("diplomatics")
        | Some("diplo")
        | Some("pact") => diplo_craft_bench(world, words, style, current),
        Some("intel") | Some("intelligence") | Some("intel-craft") | Some("knowledge-craft") => {
            intel_craft_bench(world, words, style, current)
        }
        Some("trade") | Some("trades") | Some("circulate") | Some("trade-craft") => {
            trade_craft_bench(world, words, style, current)
        }
        // Dual law: polity identity craft twin (pol.2).
        Some("polity")
        | Some("line")
        | Some("identity")
        | Some("polity-craft") => {
            polity_craft_bench(world, words, style, current)
        }
        // Activity-domain shortcuts still land on intel activity (not craft).
        Some("spy") | Some("inf") | Some("ops") => {
            println!("  {} → {}", style.dim("↳"), style.key("intel ops"));
            let rest: Vec<String> = std::iter::once("ops".into()).chain(words).collect();
            intel(world, rest.into_iter(), style, current);
        }
        Some("link") | Some("cross") | Some("pollinate") | Some("coop") => {
            println!("  {} → {}", style.dim("↳"), style.key("intel link"));
            let rest: Vec<String> = std::iter::once("link".into()).chain(words).collect();
            intel(world, rest.into_iter(), style, current);
        }
        Some(other) => {
            println!(
                "{} unknown bench domain {} — {}",
                style.bad("✗"),
                style.verb(other),
                style.key("bench")
            );
            bench_index(style);
        }
    }
}

fn bench_index(style: &Style) {
    banner(style, "BENCH — BUILD COMPONENTS");
    println!(
        "  {}",
        style.dim(
            "Craft only. Dual law: activity root S ⇒ `bench S`. Science: chem · bio · phys · math · research."
        )
    );
    println!();
    println!("  {}", style.section("MATTER & PLACE"));
    row(style, "bench planetary", "bio · chem · worldshape · medium · thermal · balance");
    row(style, "bench build", "zones · yard · refinery · regen");
    row(style, "bench star", "commission · site · age · rings · capacity (C1.7)");
    row(style, "bench estate", "chart · parcels · appraise · ladder");
    row(style, "bench medium", "ATM envelope · reinforce · tamper");
    println!();
    println!("  {}", style.section("SCIENCE CRAFT"));
    row(style, "bench science", "index of all science disciplines");
    row(style, "bench chem", "chemistry · bonds · iso · catalog");
    row(style, "bench biology", "life · biomass · regen · habitat");
    row(style, "bench physics", "star · thermal · medium · site");
    row(style, "bench math", "ratios · balances · thresholds · books");
    row(style, "bench research", "constellation · forge · practice craft");
    println!();
    println!("  {}", style.section("CRAFT DOMAINS"));
    row(style, "bench econ", "instruments · routes · rails (also under bench trade)");
    row(style, "bench fleet", "hulls · yard (also under bench trade)");
    row(style, "bench offices", "seats (trade vault/books · diplomatic)");
    row(style, "bench assay", "T7 court craft");
    row(style, "bench doctrine", "posture/KIN prep → trade fleet defend");
    println!();
    println!("  {}", style.section("DUAL LAW — activity roots as craft twins"));
    row(style, "bench intel", "forge chain · instruments · chem → root `intel`");
    row(style, "bench trade", "econ · fleet · offices umbrella → root `trade`");
    row(
        style,
        "bench diplomatic",
        "council · bond · seats · routes → root `diplomatic` (alias: bench diplo)",
    );
    row(
        style,
        "bench polity",
        "axes · taxon · practice · offices · labor → root `polity` (POL.2)",
    );
    println!();
    println!("  {}", style.section("ACTIVITY PEERS (run, not craft)"));
    row(style, "intel", "KNOWLEDGE activity");
    row(style, "trade", "CIRCULATE activity");
    row(style, "diplomatic", "DIPLOMACY activity (alias: diplo)");
    foot_bench(style, None);
}

fn planetary(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    match w.as_deref() {
        None | Some("help") | Some("?") => {
            banner(style, "BENCH › PLANETARY");
            row(style, "bench planetary biologic", "regen · biomass · balance");
            row(style, "bench planetary chem", "→ chem craft");
            row(style, "bench planetary worldshape", "WORLDSHAPE crown");
            row(style, "bench planetary medium", "→ medium domain");
            row(style, "bench planetary thermal", "heat · encase · cryo");
            row(style, "bench planetary balance", "entropy ∥ syntropy");
            row(style, "bench planetary prospect <n>", "deep: open stratum");
            row(style, "bench planetary assay <prefix>", "deep: draw richness");
            row(style, "bench planetary refine <prefix>", "deep: ElementRefinery");
            leaf(style, "prospect · assay · refine", "veins (also top-level CLI)");
            foot_bench(style, Some("planetary"));
        }
        // Deep deeds — fire prospect / assay / refine from the hub leaf.
        Some("prospect") | Some("vein") => {
            let Some(stratum) = words.next().and_then(|s| s.parse::<u8>().ok()) else {
                println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("bench planetary prospect <stratum>")
                );
                return;
            };
            match world.prospect(current, stratum) {
                Ok(p) => println!(
                    "{} stratum {} · {} · family {} · richness {}",
                    style.ok("◈"),
                    stratum,
                    style.cargo(&p.name),
                    p.family,
                    amount(&p.richness)
                ),
                Err(e) => println!("{} prospect refused: {e:?}", style.bad("✗")),
            }
            foot_bench(style, Some("planetary"));
        }
        Some("assay") => {
            let Some(prefix) = words.next() else {
                println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("bench planetary assay <element-prefix>")
                );
                return;
            };
            // Resolve banked / prospected element by hex prefix.
            let target = resolve_element_prefix(world, current, &prefix);
            match target {
                Some(el) => match world.assay(current, el) {
                    Ok(amt) => println!(
                        "{} assay {} · +{}",
                        style.ok("◈"),
                        &hex_prefix(&el),
                        amount(&amt)
                    ),
                    Err(e) => println!("{} assay refused: {e:?}", style.bad("✗")),
                },
                None => println!(
                    "{} no element matching prefix {}",
                    style.bad("✗"),
                    style.verb(&prefix)
                ),
            }
            foot_bench(style, Some("planetary"));
        }
        Some("refine") => {
            let Some(prefix) = words.next() else {
                println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("bench planetary refine <element-prefix>")
                );
                return;
            };
            let target = resolve_element_prefix(world, current, &prefix);
            match target {
                Some(el) => match world.refine(current, el) {
                    Ok(refined) => println!(
                        "{} refined → {}",
                        style.ok("◈"),
                        &hex_prefix(&refined)
                    ),
                    Err(e) => println!("{} refine refused: {e:?}", style.bad("✗")),
                },
                None => println!(
                    "{} no element matching prefix {}",
                    style.bad("✗"),
                    style.verb(&prefix)
                ),
            }
            foot_bench(style, Some("planetary"));
        }
        Some("bio") | Some("biologic") | Some("living") => {
            banner(style, "BENCH › PLANETARY › BIOLOGIC");
            if let Some(o) = world.orb_by_id(current).or_else(|| world.orb()) {
                let regen = o.zones.iter().filter(|z| z.kind == ZoneKind::Regenerator).count();
                println!(
                    "  biomass {} · regenerators {}",
                    amount(&o.banked_amount(Resource::Biomass)),
                    regen
                );
            }
            leaf(style, "zone <tile> regen|hab", "living infra");
            leaf(style, "collect", "settle");
            foot_bench(style, Some("planetary"));
        }
        Some("chem") | Some("chemistry") => chem_bench(world, words, style, current),
        Some("shape") | Some("worldshape") | Some("ws") => {
            banner(style, "BENCH › PLANETARY › WORLDSHAPE");
            println!("  can_worldshape={}", world.can_worldshape());
            let d = world.dual_use_atm();
            println!(
                "  dual-use tax={} · overt={} · covert={}",
                d.dual_use_tax, d.can_overt_worldshape, d.can_covert_tamper
            );
            leaf(style, "worldshape …", "crown deed");
            leaf(style, "bench medium", "ATM dual-use path");
            foot_bench(style, Some("planetary"));
        }
        Some("medium") | Some("atm") => medium_bench(world, words, style, current),
        Some("thermal") | Some("heat") => {
            banner(style, "BENCH › PLANETARY › THERMAL");
            leaf(style, "thermal", "band reading");
            leaf(style, "zone <tile> enc|cryo", "encasement · cryo array");
            leaf(style, "bench star", "site under star");
            foot_bench(style, Some("planetary"));
        }
        Some("balance") | Some("matter") => {
            banner(style, "BENCH › PLANETARY › BALANCE");
            println!(
                "  {}",
                style.dim("Order-work (refine/compound/hull) may refuse when entropy dominates (34).")
            );
            leaf(style, "zone … regen", "restore syntropy");
            leaf(style, "zone … enc", "encase heat");
            leaf(style, "collect", "seal vent books");
            foot_bench(style, Some("planetary"));
        }
        Some(other) => {
            println!("{} planetary leaf {} — bio|chem|worldshape|medium|thermal|balance|back", style.bad("✗"), other);
            foot_bench(style, Some("planetary"));
        }
    }
}

fn build_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    banner(style, "BENCH › BUILD — infra");
    if let Some(o) = world.orb_by_id(current).or_else(|| world.orb()) {
        println!("  zones: {}", o.zones.len());
        for z in o.zones.iter().take(16) {
            println!("    tile {} · {:?}", z.tile, z.kind);
        }
    }
    leaf(style, "zone <tile> ext|hab|smelt|synth|enc|cryo|res|regen|yard", "");
    leaf(style, "raze <tile>", "remove");
    leaf(style, "bench fleet", "hull craft after yard");
    foot_bench(style, Some("build"));
}

fn star_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    // C1.7 — stellar capacity game (lines_of as play).
    if matches!(
        w.as_deref(),
        Some("capacity") | Some("cap") | Some("lines") | Some("depth") | Some("game")
    ) {
        stellar_capacity_board(world, style, current);
        return;
    }
    if matches!(w.as_deref(), Some("age")) {
        println!(
            "  {}",
            style.hint("`age <class>` or thermal CLI — age_star takes StarClass")
        );
    }
    banner(style, "BENCH › STAR");
    match world.star() {
        Some(c) => println!("  star · class {:?}", c),
        None => println!("  {}", style.warn("no star — `star <class>`")),
    }
    if let Some(o) = world.orb_by_id(current).or_else(|| world.orb()) {
        if let Some(t) = o.thermal.as_ref() {
            println!(
                "  site albedo {} · r {}",
                amount(t.albedo.value()),
                amount(t.orbit_r.value())
            );
        } else {
            println!("  {}", style.dim("unsited — `site <A> <r>`"));
        }
    }
    // Capacity strip (always visible).
    let board = world.stellar_capacity_board(current);
    println!(
        "  {} star {}/{} · civ {} · open {} · local {} · refine-ready {}",
        style.section("capacity"),
        board.reach,
        board.table_lines,
        board.civ_reach,
        board.open_strata,
        board.local_veins,
        board.refinable_now
    );
    leaf(style, "bench star capacity", "C1.7 game — lines_of as play");
    leaf(style, "star <class> · site · thermal · rings · sky", "deeds");
    leaf(style, "bench star age", "deep: age_star year voice");
    leaf(style, "worldshape", "earned crown");
    leaf(style, "prospect <stratum> · assay · refine", "open & work veins");
    foot_bench(style, Some("star"));
}

/// C1.7 — full stellar capacity board.
fn stellar_capacity_board(world: &World, style: &Style, current: u64) {
    let b = world.stellar_capacity_board(current);
    banner(style, "BENCH › STAR › CAPACITY — lines_of as play");
    match b.star {
        Some(c) => println!(
            "  star {:?} · star forge {} of {} lines · civ_reach {} · pressure {}",
            c,
            b.reach,
            b.table_lines,
            b.civ_reach,
            amount(&b.pressure_now)
        ),
        None => println!("  {}", style.warn("no star — commission before capacity plays")),
    }
    println!(
        "  open strata {} · local veins {} · refinable now {} · deepest open {}",
        b.open_strata, b.local_veins, b.refinable_now, b.deepest_open
    );
    println!(
        "  {}",
        style.dim(
            "CIV-REACH-1: bronze digs lines 1–2; crystallize base-7 tech to mine deeper (capped by the star)"
        )
    );
    println!();
    println!("  {}", style.section("LINES (star depth)"));
    for row in &b.lines {
        let mark = if row.reachable {
            style.ok("●")
        } else {
            style.dim("○")
        };
        println!(
            "  {mark} line {} · pressure {} · veins {} · banked {}",
            row.line,
            amount(&row.pressure_at),
            row.open_veins,
            row.banked_kinds
        );
    }
    if !b.veins.is_empty() {
        println!();
        println!("  {}", style.section("OPEN VEINS"));
        for v in b.veins.iter().take(16) {
            let loc = if v.local {
                style.ok("local")
            } else {
                style.warn("deep")
            };
            let refn = if v.refines_now {
                style.ok("refines")
            } else {
                style.dim("raw")
            };
            println!(
                "  stratum {} · line {} · {} · {} · {} · richness {}",
                v.stratum,
                v.line,
                loc,
                refn,
                style.cargo(&v.name),
                amount(&v.richness)
            );
        }
        if b.veins.len() > 16 {
            println!("  {}", style.dim(&format!("… {} more", b.veins.len() - 16)));
        }
    }
    println!();
    println!("  {} {}", style.section("NEXT"), style.hint(&b.next_move));
    leaf(style, "prospect <stratum>", "open a depth reading");
    leaf(style, "assay <prefix> · refine <prefix>", "work local matter");
    leaf(style, "star <class> · age", "commission / evolve the forge depth");
    foot_bench(style, Some("star"));
}

fn medium_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    match w.as_deref() {
        Some("reinforce") => match world.medium_reinforce(current) {
            Ok(p) => println!("{} reinforced · pressure {}", style.ok("◈"), amount(&p)),
            Err(e) => println!("{} reinforce refused: {e:?}", style.bad("✗")),
        },
        Some("tamper") => match world.covert_tamper(current) {
            Ok(p) => println!("{} tamper pressure {}", style.warn("ATM"), amount(&p)),
            Err(e) => println!("{} tamper refused: {e:?}", style.bad("✗")),
        },
        _ => {}
    }
    let m = world.medium_integrity(current);
    let d = world.dual_use_atm();
    banner(style, "BENCH › MEDIUM — ATM envelope");
    println!(
        "  pressure {} / thr {} · compromised={} · reinforce {:?}",
        amount(&m.pressure),
        amount(&m.threshold),
        m.compromised,
        m.reinforce_resource
    );
    println!(
        "  voice {} · dual-use tax={} · {}",
        m.residual_voice, d.dual_use_tax, style.dim("metabolism-coloured")
    );
    leaf(style, "bench medium reinforce", "deep: lower pressure");
    leaf(style, "bench medium tamper", "deep: covert ATM (taxed if WORLDSHAPE)");
    leaf(style, "medium · tamper", "same deeds outside hub");
    foot_bench(style, Some("medium"));
}

fn offices_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    banner(style, "BENCH › OFFICES — seat craft");
    let deep = world.deep_rail_status();
    println!(
        "  Exchange={} · Assay={} · Vault={} · Anchor={}",
        world.exchange_seated(),
        world.assay_seated(),
        world.vault_seated(),
        deep.anchor_seated
    );
    println!();
    leaf(style, "seat exchange|assay|vault|anchor", "StationSeated craft");
    leaf(style, "bench assay", "T7 consilience court");
    leaf(style, "trade vault · trade books", "run after seat");
    foot_bench(style, Some("offices"));
    let _ = (w, current);
}

fn assay_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    banner(style, "BENCH › ASSAY — T7 court craft");
    println!("  seated={}", world.assay_seated());
    println!(
        "  prospects {}",
        world.committed().prospects.iter().filter(|p| p.orb == current).count()
    );
    leaf(style, "seat assay", "open T7");
    leaf(style, "challenge <element-prefix>", "consilience / slash");
    leaf(style, "prospect · assay <prefix>", "open vein · extract");
    foot_bench(style, Some("assay"));
    let _ = w;
}

fn fleet_craft_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    // deep mint
    if matches!(w.as_deref(), Some("mint")) {
        let role = match words.next().as_deref() {
            Some("escort") => HullRole::Escort,
            Some("semi") => HullRole::Semi,
            Some("runner") => HullRole::Runner,
            _ => HullRole::Hauler,
        };
        let grade = words.next().and_then(|x| x.parse().ok()).unwrap_or(0);
        match world.mint_hull(current, role, grade) {
            Ok(id) => println!("{} hull {id} minted {:?}", style.ok("◈"), role),
            Err(e) => println!("{} mint refused: {e:?}", style.bad("✗")),
        }
    }
    banner(style, "BENCH › FLEET CRAFT");
    let yard = world.orb_by_id(current).is_some_and(|o| {
        o.zones.iter().any(|z| z.kind == ZoneKind::ShipYard)
    });
    println!(
        "  yard={} · hulls {} · free capacity {}",
        yard,
        world.hulls().len(),
        amount(&world.free_capacity(0))
    );
    leaf(style, "zone <tile> yard", "ShipYard component");
    leaf(style, "bench fleet mint [hauler|escort|semi|runner] [g]", "deep mint");
    leaf(style, "salvage · fleet mint … finance", "scrap + optional note");
    leaf(style, "trade fleet", "RUN capacity · escort · risk");
    foot_bench(style, Some("fleet"));
}

fn estate_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    if matches!(w.as_deref(), Some("chart") | Some("survey")) {
        match world.chart_terrain() {
            Ok(seed) => println!("{} terrain {:02x}{:02x}…", style.ok("◈"), seed[0], seed[1]),
            Err(e) => println!("{} chart refused: {e:?}", style.bad("✗")),
        }
    }
    if matches!(w.as_deref(), Some("ladder") | Some("estate")) {
        match world.survey_estate(current) {
            Ok(h) => println!("{} estate {:02x}{:02x}…", style.ok("◈"), h[0], h[1]),
            Err(e) => println!("{} estate refused: {e:?}", style.bad("✗")),
        }
    }
    banner(style, "BENCH › ESTATE");
    println!("  terrain charted={}", world.terrain_seed().is_some());
    if let Some(parcels) = world.parcels() {
        println!("  parcels {}", parcels.len());
        for p in parcels.iter().take(8) {
            println!(
                "    orb {} free {} mon {}",
                p.orb_id,
                amount(&p.free),
                amount(&p.monetary)
            );
        }
    } else {
        println!("  {}", style.dim("no parcels — estate ladder first"));
    }
    leaf(style, "bench estate chart|ladder", "deep deeds");
    leaf(style, "appraise … · estate parcels", "valuation");
    foot_bench(style, Some("estate"));
}

fn chem_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let mode = words.next();
    if is_back(mode.as_deref()) {
        bench_index(style);
        return;
    }
    let science = !matches!(mode.as_deref(), Some("bonds") | Some("list") | Some("standing"));
    if science {
        banner(style, "BENCH › CHEM — chemistry craft");
        if let Some(ax) = world.civ_axes() {
            println!(
                "  axes dis={} met={} per={}",
                ax.disposition, ax.metabolism, ax.perception
            );
        }
        for (p, w) in world.dark_pulls() {
            println!("  {} {} · {}", style.warn("DARK"), p.word(), amount(&w));
        }
        let rows = world.bench_science(current);
        if rows.is_empty() {
            println!("  {}", style.dim("nothing bonds"));
        }
        for (members, gem, k, standing, survival, dark, net) in &rows {
            let named: Vec<_> = members
                .iter()
                .map(|m| {
                    let h = hex_prefix(m);
                    h[..6.min(h.len())].to_string()
                })
                .collect();
            let gh = hex_prefix(gem);
            println!(
                "  {} → {} · k{} · net {} (s {} / d {})",
                style.strong(&named.join("+")),
                style.strong(&gh[..8.min(gh.len())]),
                k,
                amount(net),
                amount(survival),
                amount(dark)
            );
            if standing.is_none() {
                println!("      {}", style.ok("✓ workable"));
            } else {
                println!("      {}", style.warn("⚠ place gate"));
            }
        }
        if let Some((_, _, _, st, s, d, n)) = rows.first() {
            println!(
                "  {} top net {} · {}",
                style.warn("NEXT"),
                amount(n),
                if *d > *s { "dark leading" } else { "survival leading" }
            );
            let _ = st;
        }
        let rank = world.science_rank(current, cfe::almanac::ScienceDiscipline::Chem);
        println!(
            "  chem seat rank {} {}",
            rank,
            if rank == 0 {
                style.warn("(seat required for catalog/iso)")
            } else {
                style.ok("(seated)")
            }
        );
        leaf(style, "bench science seat chem", "pay crystal unlock");
        leaf(style, "compound · iso · catalog", "deeds (need seat R1)");
        leaf(style, "bench science create chem|biochem", "dual ± create (need R2)");
        leaf(style, "intel knowledge", "catalog appears in knowledge bases");
        leaf(style, "bench science", "all science disciplines");
        foot_bench(style, Some("chem"));
    } else {
        banner(style, "BENCH › CHEM — standing bonds");
        for (members, gem, k, standing) in world.bench_standing(current) {
            let named: Vec<_> = members
                .iter()
                .map(|m| {
                    let h = hex_prefix(m);
                    h[..6.min(h.len())].to_string()
                })
                .collect();
            let gh = hex_prefix(&gem);
            println!(
                "  {} → {} · k{}",
                named.join("+"),
                &gh[..8.min(gh.len())],
                k
            );
            let _ = standing;
        }
        leaf(style, "bench chem", "science view");
        foot_bench(style, Some("chem"));
    }
}

/// Science discipline index under bench — **seated** labs, dual ±, explosion.
fn science_index(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    match w.as_deref() {
        Some("seat") => {
            let dname = words.next().unwrap_or_default();
            match cfe::almanac::ScienceDiscipline::from_word(&dname) {
                Some(d) => match world.seat_science(current, d) {
                    Ok(rank) => {
                        println!(
                            "  {} seated {} rank {} — pay crystal, unlock capability",
                            style.ok("✓"),
                            d.word(),
                            rank
                        );
                    }
                    Err(e) => println!("  {} seat refused: {e:?}", style.bad("✗")),
                },
                None => {
                    println!(
                        "  {} usage: bench science seat <chem|biology|physics|math|research|synthetic|clone|skin>",
                        style.bad("✗")
                    );
                }
            }
            foot_bench(style, Some("science"));
        }
        Some("create") | Some("work") | Some("attempt") => {
            let pname = words.next().unwrap_or_default();
            match cfe::almanac::LabPath::from_word(&pname) {
                Some(path) => {
                    // Show poles first (known always; hidden conditional).
                    let card = world.lab_preview(current, path);
                    println!("  {}", style.section("OUTCOMES (before create)"));
                    println!(
                        "  {} + {}",
                        style.ok("KNOWN +"),
                        style.dim(card.known_pos.1)
                    );
                    println!(
                        "  {} − {}",
                        style.warn("KNOWN −"),
                        style.dim(card.known_neg.1)
                    );
                    match card.hidden_pos {
                        Some((_, g)) => println!(
                            "  {} + {}",
                            style.ok("HIDDEN +"),
                            style.dim(g)
                        ),
                        None => println!(
                            "  {} (seat math / empiricist to reveal)",
                            style.dim("HIDDEN +")
                        ),
                    }
                    match card.hidden_neg {
                        Some((_, g)) => println!(
                            "  {} − {}",
                            style.bad("HIDDEN −"),
                            style.dim(g)
                        ),
                        None => println!(
                            "  {} (revealed if dark-led or will explode)",
                            style.dim("HIDDEN −")
                        ),
                    }
                    println!(
                        "  risk {} · threshold 1/2 · will_explode={}",
                        amount(&card.risk),
                        card.will_explode
                    );
                    match world.lab_create(current, path) {
                        Ok(r) => {
                            if r.exploded {
                                println!(
                                    "  {} EXPLOSION {} — {}",
                                    style.bad("💥"),
                                    r.explosion_kind
                                        .map(|k| k.word())
                                        .unwrap_or("?"),
                                    r.explosion_kind
                                        .map(|k| k.gloss())
                                        .unwrap_or("")
                                );
                            } else {
                                println!(
                                    "  {} lab {} created · risk {}",
                                    style.ok("✓"),
                                    path.word(),
                                    amount(&r.risk)
                                );
                            }
                            if r.hidden_pos_landed {
                                println!("  {} hidden positive landed", style.ok("+"));
                            }
                            if r.hidden_neg_landed {
                                println!("  {} hidden negative landed", style.bad("−"));
                            }
                        }
                        Err(e) => println!("  {} create refused: {e:?}", style.bad("✗")),
                    }
                }
                None => {
                    println!(
                        "  {} usage: bench science create <chem|biochem|synth|clone|skin>",
                        style.bad("✗")
                    );
                }
            }
            foot_bench(style, Some("science"));
        }
        Some("preview") | Some("poles") | Some("risk") => {
            let pname = words.next().unwrap_or_else(|| "chem".into());
            if let Some(path) = cfe::almanac::LabPath::from_word(&pname) {
                let card = world.lab_preview(current, path);
                banner(style, "BENCH › SCIENCE › PREVIEW");
                println!("  path {} · risk {} · explode={}", path.word(), amount(&card.risk), card.will_explode);
                println!("  {} {}", style.ok("+ known"), card.known_pos.1);
                println!("  {} {}", style.warn("− known"), card.known_neg.1);
                if let Some((_, g)) = card.hidden_pos {
                    println!("  {} {}", style.ok("+ hidden"), g);
                } else {
                    println!("  {}", style.dim("+ hidden — seat math or empiricist perception"));
                }
                if let Some((_, g)) = card.hidden_neg {
                    println!("  {} {}", style.bad("− hidden"), g);
                } else {
                    println!("  {}", style.dim("− hidden — dark-led / explode reveals"));
                }
            } else {
                println!("{} unknown path {pname}", style.bad("✗"));
            }
            foot_bench(style, Some("science"));
        }
        Some("chem") | Some("chemistry") | Some("alchemy") => {
            chem_bench(world, words, style, current)
        }
        Some("bio") | Some("biology") | Some("life") => {
            biology_bench(world, words, style, current)
        }
        Some("phys") | Some("physics") => physics_bench(world, words, style, current),
        Some("math") | Some("maths") | Some("mathematics") => {
            math_bench(world, words, style, current)
        }
        Some("research") | Some("tech") | Some("lab") => {
            research_craft_bench(world, words, style, current)
        }
        Some("synth") | Some("synthetic") | Some("cyber") => {
            synth_bench(world, words, style, current)
        }
        Some("clone") | Some("clones") => clone_bench(world, words, style, current),
        Some("skin") | Some("face") => skin_bench(world, words, style, current),
        None | Some("help") | Some("?") | Some("index") | Some("menu") => {
            banner(style, "BENCH › SCIENCE — seated labs");
            println!(
                "  {}",
                style.dim(
                    "Pay crystal to seat. Rank 1 = act. Rank 2 = create (dual ± · explosion risk)."
                )
            );
            let board = world.science_lab_board(current);
            let met = match board.metabolism {
                1 => "mineral",
                3 => "plasmic",
                _ => "biotic",
            };
            let clade_w = board
                .clade
                .map(cfe::vocabulary::clade_word)
                .unwrap_or("unrecorded");
            println!(
                "  metabolism {} · clade {} · clones {} · synth g{} · skin g{} · rogue {}",
                met, clade_w, board.clones, board.synth_grade, board.skin_grade, board.rogue
            );
            println!(
                "  {}",
                style.dim(
                    "Costs: local resources · resonance (crystal, not STAMP) · off-planet for advanced R2. Taxon colours affinity."
                )
            );
            if world.off_planet_ready() {
                println!("  {}", style.ok("off-planet resourcing: ready (crossed/route/element route)"));
            } else {
                println!(
                    "  {}",
                    style.warn("off-planet resourcing: locked — need aperture/route before synth·clone·skin R2")
                );
            }
            println!();
            println!("  {}", style.section("DISCIPLINE SEATS (affinity · fee)"));
            for (d, rank, fee, aff) in &board.seats {
                let mark = if *rank >= 2 {
                    style.ok("R2")
                } else if *rank == 1 {
                    style.strong("R1")
                } else {
                    style.dim("--")
                };
                let aff_s = match aff {
                    cfe::almanac::SeatAffinity::Primary => style.ok("primary"),
                    cfe::almanac::SeatAffinity::Secondary => style.warn("secondary"),
                    cfe::almanac::SeatAffinity::Alien => style.bad("alien"),
                };
                println!(
                    "  {} {}  rank {} · {} · fee {} · {}",
                    mark,
                    style.verb(d.word()),
                    rank,
                    aff_s,
                    amount(fee),
                    style.dim(d.gloss())
                );
            }
            println!();
            println!("  {}", style.section("VERBS"));
            row(style, "bench science seat <discipline>", "pay crystal · unlock rank");
            row(style, "bench science preview <path>", "poles + explosion risk");
            row(
                style,
                "bench science create <path>",
                "chem|biochem|synth|clone|skin (needs R2 seats)",
            );
            row(style, "bench chem|biology|physics|math|research", "discipline craft");
            row(style, "bench science synth|clone|skin", "mod paths");
            leaf(style, "intel research · intel knowledge", "RUN knowledge activity");
            foot_bench(style, Some("science"));
        }
        Some(other) => {
            println!(
                "{} unknown science leaf {} — seat|create|preview|chem|…",
                style.bad("✗"),
                style.verb(other)
            );
            foot_bench(style, Some("science"));
        }
    }
}

fn synth_bench(world: &mut World, mut words: impl Iterator<Item = String>, style: &Style, current: u64) {
    let w = words.next();
    if is_back(w.as_deref()) {
        science_index(world, std::iter::empty(), style, current);
        return;
    }
    banner(style, "BENCH › SYNTHETIC — cybernetic craft");
    println!(
        "  grade {} · rogue pressure {} · seat synthetic+math R2 to create",
        world.synth_grade_of(current),
        world.rogue_pressure_of(current)
    );
    let card = world.lab_preview(current, cfe::almanac::LabPath::SynthMod);
    println!("  risk {} · explode={}", amount(&card.risk), card.will_explode);
    println!("  {} {}", style.ok("+"), card.known_pos.1);
    println!("  {} {}", style.warn("−"), card.known_neg.1);
    if let Some((_, g)) = card.hidden_neg {
        println!("  {} {}", style.bad("hidden −"), g);
    }
    leaf(style, "bench science seat synthetic · seat math", "unlock");
    leaf(style, "bench science create synth", "attempt (may rogue)");
    foot_bench(style, Some("science"));
    let _ = w;
}

fn clone_bench(world: &mut World, mut words: impl Iterator<Item = String>, style: &Style, current: u64) {
    let w = words.next();
    if is_back(w.as_deref()) {
        science_index(world, std::iter::empty(), style, current);
        return;
    }
    banner(style, "BENCH › CLONE — population craft");
    println!(
        "  clones {} · seat clone+biology R2 · population control path",
        world.clone_count_of(current)
    );
    let card = world.lab_preview(current, cfe::almanac::LabPath::CloneWork);
    println!("  risk {} · explode={}", amount(&card.risk), card.will_explode);
    println!("  {} {}", style.ok("+"), card.known_pos.1);
    println!("  {} {}", style.warn("−"), card.known_neg.1);
    leaf(style, "bench science seat clone · seat biology", "unlock");
    leaf(style, "bench science create clone", "attempt");
    foot_bench(style, Some("science"));
    let _ = w;
}

fn skin_bench(world: &mut World, mut words: impl Iterator<Item = String>, style: &Style, current: u64) {
    let w = words.next();
    if is_back(w.as_deref()) {
        science_index(world, std::iter::empty(), style, current);
        return;
    }
    banner(style, "BENCH › SKIN — face/disguise craft (Tleilaxu-shaped)");
    println!(
        "  skin grade {} · seat skin+biology R2",
        world.skin_grade_of(current)
    );
    let card = world.lab_preview(current, cfe::almanac::LabPath::SkinWork);
    println!("  risk {} · explode={}", amount(&card.risk), card.will_explode);
    println!("  {} {}", style.ok("+"), card.known_pos.1);
    println!("  {} {}", style.warn("−"), card.known_neg.1);
    if let Some((_, g)) = card.hidden_pos {
        println!("  {} {}", style.ok("hidden +"), g);
    }
    if let Some((_, g)) = card.hidden_neg {
        println!("  {} {}", style.bad("hidden −"), g);
    }
    leaf(style, "bench science seat skin · seat biology", "unlock");
    leaf(style, "bench science create skin", "attempt");
    foot_bench(style, Some("science"));
    let _ = w;
}

/// Biology craft — living infrastructure and biomass path.
fn biology_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    banner(style, "BENCH › BIOLOGY — life craft");
    println!(
        "  {}",
        style.dim("Living components. Metabolism colour follows sealed axes.")
    );
    if let Some(ax) = world.civ_axes() {
        let met = match ax.metabolism {
            1 => "mineral",
            2 => "biotic",
            3 => "plasmic",
            _ => "?",
        };
        println!(
            "  metabolism {} ({}) · disposition {} · perception {}",
            ax.metabolism, met, ax.disposition, ax.perception
        );
    }
    if let Some(o) = world.orb_by_id(current).or_else(|| world.orb()) {
        let regen = o.zones.iter().filter(|z| z.kind == ZoneKind::Regenerator).count();
        let hab = o.zones.iter().filter(|z| z.kind == ZoneKind::Habitat).count();
        println!(
            "  biomass {} · regenerators {} · habitats {}",
            amount(&o.banked_amount(Resource::Biomass)),
            regen,
            hab
        );
        println!(
            "  crystal {} · iron {} · alloy {}",
            amount(&o.banked_amount(Resource::Crystal)),
            amount(&o.banked_amount(Resource::Iron)),
            amount(&o.banked_amount(Resource::Alloy))
        );
    }
    println!();
    println!("  {}", style.section("COMPONENTS TO CRAFT"));
    leaf(style, "zone <tile> regen|hab", "living infra");
    leaf(style, "collect", "settle biomass books");
    leaf(style, "bench chem", "biotic-scored bonds (metabolism axis)");
    leaf(style, "bench planetary biologic", "planetary bio leaf");
    println!();
    println!("  {}", style.section("THEN RUN / READ"));
    leaf(style, "intel knowledge", "bases after life work");
    leaf(style, "bench science", "all science disciplines");
    foot_bench(style, Some("biology"));
    let _ = w;
}

/// Physics craft — star, thermal, medium, site (call existing world APIs).
fn physics_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    match w.as_deref() {
        Some("star") | Some("sky") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench star"));
            star_bench(world, words, style, current);
            return;
        }
        Some("medium") | Some("atm") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench medium"));
            medium_bench(world, words, style, current);
            return;
        }
        Some("thermal") | Some("heat") => {
            banner(style, "BENCH › PHYSICS › THERMAL");
            leaf(style, "thermal", "band reading");
            leaf(style, "zone <tile> enc|cryo", "encasement · cryo array");
            leaf(style, "bench star", "site under star");
            foot_bench(style, Some("physics"));
            return;
        }
        _ => {}
    }
    banner(style, "BENCH › PHYSICS — physical craft");
    println!(
        "  {}",
        style.dim("Star · thermal · medium · site. No second physics — pure world readings.")
    );
    println!("  star={:?} · can_worldshape={}", world.star(), world.can_worldshape());
    let (heat, cold) = world.thermal_books_of(current);
    println!("  thermal books heat={} · cold={}", amount(&heat), amount(&cold));
    if let Some(bal) = world.thermal_balance_of(current) {
        println!("  thermal balance {}", amount(&bal));
    }
    let d = world.dual_use_atm();
    println!(
        "  dual-use tax={} · overt={} · covert={}",
        d.dual_use_tax, d.can_overt_worldshape, d.can_covert_tamper
    );
    let mi = world.medium_integrity(current);
    println!(
        "  medium pressure {} · thr {} · {}",
        amount(&mi.pressure),
        amount(&mi.threshold),
        style.dim(mi.residual_voice)
    );
    println!();
    println!("  {}", style.section("COMPONENTS TO CRAFT"));
    leaf(style, "commission · site · thermal", "star / orbit path");
    leaf(style, "zone <tile> enc|cryo", "thermal infra");
    leaf(style, "medium reinforce · tamper", "ATM envelope");
    leaf(style, "bench star · bench medium", "deep domains");
    leaf(style, "bench math", "ratio books for heat/balance");
    leaf(style, "bench science", "all science disciplines");
    foot_bench(style, Some("physics"));
    let _ = w;
}

/// Math craft — exact ratios, balances, thresholds (presentation over sealed math).
fn math_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    banner(style, "BENCH › MATH — ratio craft");
    println!(
        "  {}",
        style.dim("Exact rationals only. Thresholds and books — no free mint.")
    );
    if let Some(ax) = world.civ_axes() {
        println!(
            "  sealed axes dis={} met={} per={}",
            ax.disposition, ax.metabolism, ax.perception
        );
    }
    let (heat, cold) = world.thermal_books_of(current);
    println!("  thermal heat={} · cold={}", amount(&heat), amount(&cold));
    if let Some(bal) = world.thermal_balance_of(current) {
        println!("  thermal balance {}", amount(&bal));
    }
    let rb = world.research_board(current);
    println!(
        "  research held {} · forgeable {} · instruments {} · practices {}",
        rb.held_count, rb.forgeable_ready, rb.instruments, rb.practices
    );
    for n in rb.nodes.iter().filter(|n| n.threshold > 0).take(8) {
        println!(
            "  · cond {} · progress {}/{} · held={}",
            n.condition, n.progress, n.threshold, n.held
        );
    }
    let mi = world.market_integrity();
    println!(
        "  market crystal_depth {}/{} · sealed={}",
        mi.crystal_depth, mi.seal_need, mi.sealed
    );
    println!(
        "  free capacity {} · detection g{} · hulls {}",
        amount(&world.free_capacity(0)),
        world.detection_grade(),
        world.hulls().len()
    );
    println!();
    println!("  {}", style.section("READ / DERIVE"));
    leaf(style, "thermal · market · suite", "band / integrity readings");
    leaf(style, "bench research", "constellation thresholds as play");
    leaf(style, "bench physics · bench chem", "apply ratios in craft");
    leaf(style, "verify · replay", "SEAL B sealed math");
    leaf(style, "bench science", "all science disciplines");
    foot_bench(style, Some("math"));
    let _ = w;
}

/// Research craft — constellation / forge components (activity twin: intel research).
fn research_craft_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    // Deep forge path stays on intel research deed routing when verb is forge/practice.
    if matches!(w.as_deref(), Some("forge") | Some("practice") | Some("run")) {
        println!("  {} → {}", style.dim("↳"), style.key("intel research"));
        let rest: Vec<String> = std::iter::once("research".into())
            .chain(w.into_iter())
            .chain(words)
            .collect();
        intel(world, rest.into_iter(), style, current);
        return;
    }
    banner(style, "BENCH › RESEARCH — constellation craft");
    println!(
        "  {}",
        style.dim("Build research components. Operate forge/practice on `intel research`.")
    );
    let board = world.research_board(current);
    println!(
        "  held {} · forgeable {} · instruments {} · practices {} · wants {}",
        board.held_count,
        board.forgeable_ready,
        board.instruments,
        board.practices,
        board.wants
    );
    if board.novelty_blocked {
        println!(
            "  {}",
            style.warn("novelty blocks crystallize — license / discovery")
        );
    }
    println!("  {} {}", style.warn("NEXT"), board.next_move);
    for n in board.nodes.iter().take(12) {
        let mark = if n.forgeable {
            style.ok("forge")
        } else if n.held {
            style.strong("held")
        } else {
            style.dim("open")
        };
        println!(
            "  · c{} {:?} {} · {}/{} · {}",
            n.condition, n.kind, mark, n.progress, n.threshold, n.next_hint
        );
    }
    println!();
    println!("  {}", style.section("COMPONENTS TO CRAFT"));
    leaf(style, "zone <tile> smelt|synth|res", "forge chain zones");
    leaf(style, "forge <condition>", "instrument (deed)");
    leaf(style, "bench intel", "tool craft umbrella");
    leaf(style, "bench chem · bench math", "matter / threshold support");
    println!();
    println!("  {}", style.section("THEN RUN ON"));
    leaf(style, "intel research [forge|practice]", "knowledge activity");
    leaf(style, "bench research forge <c>", "shortcut → intel research forge");
    leaf(style, "bench science", "all science disciplines");
    foot_bench(style, Some("research"));
}

fn econ_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    _current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    let deep = world.deep_rail_status();
    banner(style, "BENCH › ECON — craft trade components");
    println!(
        "  Exchange={} Vault={} Anchor={} · notes/fut/share/perp {}/{}/{}/{}",
        world.exchange_seated(),
        world.vault_seated(),
        deep.anchor_seated,
        world.notes().iter().filter(|n| n.open).count(),
        world.futures().iter().filter(|f| f.open).count(),
        world.shares().iter().filter(|s| s.open).count(),
        world.perps().iter().filter(|p| p.open).count(),
    );
    leaf(style, "seat exchange|vault|anchor", "or bench offices");
    leaf(style, "forge · note issue · future · share · perp", "MM listing");
    leaf(style, "charter · element charter · rail promote", "lanes");
    leaf(style, "stamp mint", "unit craft (run under trade stamp)");
    leaf(style, "trade books|stamp|element|rails", "RUN activity");
    foot_bench(style, Some("econ"));
}

/// Craft surface for diplomacy — components the **diplomatic** root then runs.
fn diplo_craft_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    let board = world.link_board();
    banner(style, "BENCH › DIPLOMATIC — components (craft)");
    println!(
        "  {}",
        style.dim("Build the pieces. Run treaties/hauls on root `diplomatic` (alias: diplo).")
    );
    println!(
        "  councils {} (open {}) · bonds {} · forwards {} · treaties {}",
        board.councils, board.council_open, board.bonds, board.forwards, board.treaties
    );
    println!(
        "  Exchange={} Vault={} Anchor={} · crossed={}",
        world.exchange_seated(),
        world.vault_seated(),
        world.deep_rail_status().anchor_seated,
        world.has_crossed()
    );
    println!();
    println!("  {}", style.section("COMPONENTS TO CRAFT"));
    leaf(style, "seat … · bench offices", "diplomatic standing / economic seats");
    leaf(style, "council …", "multilateral forward (accord craft)");
    leaf(style, "bond · forward", "standing orders / vitrified pacts");
    leaf(style, "charter · element charter", "lanes for pair trade");
    leaf(style, "forge <c>", "instruments (trust / detection for talks)");
    leaf(style, "bench fleet mint escort", "KIN for convoy diplomacy");
    println!();
    println!("  {}", style.section("THEN RUN ON"));
    leaf(style, "diplomatic local", "seal/renounce treaties");
    leaf(style, "diplomatic stellar", "war wall · aperture · license leap");
    leaf(style, "intel link", "license path readiness");
    foot_bench(style, Some("diplomatic"));
    let _ = (w, current);
}

/// Craft surface for intel tools — twin of root `intel` activity.
fn intel_craft_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    // Nested craft shortcuts under bench intel.
    match w.as_deref() {
        Some("chem") | Some("catalog") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench chem"));
            chem_bench(world, words, style, current);
            return;
        }
        Some("build") | Some("zone") | Some("zones") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench build"));
            build_bench(world, words, style, current);
            return;
        }
        Some("doctrine") | Some("posture") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench doctrine"));
            doctrine_bench(world, words, style, current);
            return;
        }
        _ => {}
    }
    banner(style, "BENCH › INTEL — knowledge components (craft)");
    println!(
        "  {}",
        style.dim("Build the tools. Run research/ops/bases on root `intel` (not here).")
    );
    let smelt = world.orb_by_id(current).is_some_and(|o| {
        o.zones.iter().any(|z| z.kind == ZoneKind::Smelter)
    });
    let synth = world.orb_by_id(current).is_some_and(|o| {
        o.zones.iter().any(|z| z.kind == ZoneKind::Synthesizer)
    });
    println!(
        "  smelter={} · synthesizer={} · instruments {} · detection g{}",
        smelt,
        synth,
        world.instrument_count(),
        world.detection_grade()
    );
    println!();
    println!("  {}", style.section("COMPONENTS TO CRAFT"));
    leaf(style, "zone <tile> smelt|synth|res", "forge chain components");
    leaf(style, "forge <condition>", "instrument (or intel research forge)");
    leaf(style, "bench chem · catalog", "recipes → intel knowledge");
    leaf(style, "bench build", "smelter / synthesizer / research zones");
    leaf(style, "bench doctrine", "posture prep for ops");
    println!();
    println!("  {}", style.section("THEN RUN ON"));
    leaf(style, "intel research", "constellation · forge · practice");
    leaf(style, "intel knowledge · intel ops · intel link", "bases / spy / co-op");
    foot_bench(style, Some("intel"));
    let _ = w;
}

/// Craft surface for trade components — twin of root `trade` activity.
fn trade_craft_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    // Nested craft shortcuts under bench trade.
    match w.as_deref() {
        Some("econ") | Some("instruments") | Some("routes") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench econ"));
            econ_bench(world, words, style, current);
            return;
        }
        Some("fleet") | Some("hull") | Some("ship") | Some("yard") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench fleet"));
            fleet_craft_bench(world, words, style, current);
            return;
        }
        Some("offices") | Some("office") | Some("seat") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench offices"));
            offices_bench(world, words, style, current);
            return;
        }
        Some("assay") | Some("court") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench assay"));
            assay_bench(world, words, style, current);
            return;
        }
        _ => {}
    }
    banner(style, "BENCH › TRADE — market/haul components (craft)");
    println!(
        "  {}",
        style.dim("Build the pieces. Run books/haul/vault on root `trade` (not here).")
    );
    println!(
        "  Exchange={} Vault={} Anchor={} · hulls {} · instruments {}",
        world.exchange_seated(),
        world.vault_seated(),
        world.deep_rail_status().anchor_seated,
        world.hulls().len(),
        world.instrument_count()
    );
    println!();
    println!("  {}", style.section("COMPONENTS TO CRAFT"));
    leaf(style, "bench econ", "instruments · routes · rails");
    leaf(style, "bench fleet", "yard · mint hull · scrap");
    leaf(style, "bench offices", "seat exchange|vault|assay|anchor");
    leaf(style, "bench assay", "T7 court craft");
    leaf(style, "bench doctrine", "KIN / posture for escort run");
    println!();
    println!("  {}", style.section("THEN RUN ON"));
    leaf(style, "trade books · trade stamp · trade element", "market / unit / matter");
    leaf(style, "trade fleet · trade vault · trade rails", "haul / custody / rails");
    leaf(style, "diplomatic", "treaties are their own root (bench diplomatic crafts them)");
    foot_bench(style, Some("trade"));
    let _ = w;
}

/// Craft surface for polity identity components — twin of root `polity` (Pol.2).
fn polity_craft_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    match w.as_deref() {
        Some("offices") | Some("office") | Some("seat") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench offices"));
            offices_bench(world, words, style, current);
            return;
        }
        Some("doctrine") | Some("posture") | Some("kin") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench doctrine"));
            doctrine_bench(world, words, style, current);
            return;
        }
        Some("practice") | Some("research") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench research"));
            research_craft_bench(world, words, style, current);
            return;
        }
        Some("labor") | Some("hab") | Some("population") => {
            // Deep deed: show labor + allow habitat build hint.
            let labor = world.labor_board(current);
            banner(style, "BENCH › POLITY › LABOR — craft population seats");
            println!(
                "  force {} · habitats {} · clones {} · slots {}",
                labor.force, labor.habitats, labor.clones, labor.work_slots
            );
            println!("  {}", style.hint(&labor.next_hint));
            leaf(style, "zone <tile> hab", "seat a habitat (labor +1)");
            leaf(style, "bench science create clone", "clone residual population");
            leaf(style, "polity ap", "shared AP budget (P8.1)");
            foot_bench(style, Some("polity"));
            return;
        }
        _ => {}
    }
    banner(style, "BENCH › POLITY — identity components (craft · POL.2)");
    println!(
        "  {}",
        style.dim("Build the pieces that root `polity` runs. Dual law: activity S ⇒ bench S.")
    );
    match world.genesis() {
        Some(g) => println!(
            "  genesis {} · dis {} · met {} · per {}",
            g.civilization, g.disposition, g.metabolism, g.perception
        ),
        None => println!("  {}", style.warn("genesis unsealed — seal on activity root first")),
    }
    let labor = world.labor_board(current);
    let ap = world.ap_board();
    println!(
        "  labor {} (hab {} + clone {}) · AP {}/{} · rail {}",
        labor.force,
        labor.habitats,
        labor.clones,
        ap.available,
        ap.capacity,
        world.max_rail_word()
    );
    println!();
    println!("  {}", style.section("COMPONENTS TO CRAFT"));
    leaf(style, "bench polity labor", "habitats · clones · work slots");
    leaf(style, "bench offices", "seat exchange|vault|assay|anchor (identity ladder)");
    leaf(style, "bench doctrine", "posture / KIN prep for the line");
    leaf(style, "bench research", "practice adopt (discipline binds the line)");
    leaf(style, "bench build", "zone habitat — labor seats");
    println!();
    println!("  {}", style.section("THEN RUN ON"));
    leaf(style, "polity genesis · polity taxon", "axes / class-clade");
    leaf(style, "polity mint · polity ap", "discovery mint / AP board");
    foot_bench(style, Some("polity"));
    let _ = w;
}

fn doctrine_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    _current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        bench_index(style);
        return;
    }
    let board = world.doctrine_board();
    banner(style, "BENCH › DOCTRINE");
    println!(
        "  {} / {} · reoriented={} · kin {}",
        board.primary, board.secondary, board.reoriented, board.kin_voice
    );
    println!(
        "  met={} dis={} per={}",
        board.metabolism, board.disposition, board.perception
    );
    let names = ["CEL", "ATM", "COG", "CYB", "INF", "KIN", "MKT"];
    for (i, r) in board.ranks.iter().enumerate() {
        let bar = "█".repeat(*r as usize) + &"░".repeat(3usize.saturating_sub(*r as usize));
        println!("  {} {bar} ({r})", names[i]);
    }
    println!("  {} {}", style.warn("NEXT"), board.next_hint);
    leaf(style, "posture set …", "reorient");
    leaf(style, "trade fleet · defend escort", "KIN run");
    foot_bench(style, Some("doctrine"));
    let _ = w;
}

// ════════════════════════════════════════════════════════════════════
// intel — knowledge activity
// ════════════════════════════════════════════════════════════════════

pub fn intel(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let domain = words.next();
    if is_back(domain.as_deref()) {
        intel_index(style);
        return;
    }
    match domain.as_deref() {
        None | Some("help") | Some("?") | Some("menu") | Some("index") => intel_index(style),
        Some("research") | Some("tech") | Some("lab") | Some("constellation") => {
            intel_research(world, words, style, current)
        }
        Some("knowledge") | Some("base") | Some("kb") | Some("bases") | Some("library") => {
            intel_knowledge(world, style, current)
        }
        Some("ops") | Some("spy") | Some("pipeline") | Some("espionage") => {
            intel_ops(world, words, style, current)
        }
        Some("opsec") | Some("veil") | Some("ward") | Some("mask") => {
            intel_opsec(world, words, style, current)
        }
        Some("link") | Some("cross") | Some("pollinate") | Some("coop") => {
            intel_link(world, words, style, current)
        }
        Some("culture") | Some("lit") | Some("literature") | Some("works") => {
            intel_culture(world, words, style, current)
        }
        Some("books") | Some("accounts") => intel_books(world, words, style, current),
        Some("wants") | Some("want") | Some("mind") => intel_wants(world, style, current),
        Some("craft") | Some("bench") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench intel"));
            intel_craft_bench(world, words, style, current);
        }
        Some(other) => {
            println!(
                "{} unknown intel domain {} — {}",
                style.bad("✗"),
                style.verb(other),
                style.key("intel")
            );
            intel_index(style);
        }
    }
}

fn intel_index(style: &Style) {
    banner(style, "INTEL — KNOWLEDGE ACTIVITY");
    println!(
        "  {}",
        style.dim("Operate knowledge. Craft tools: `bench intel` (dual law).")
    );
    println!();
    row(style, "intel research", "constellation · forge · practice");
    row(style, "intel knowledge", "all bases: tech · chem · stash · lit · books");
    row(style, "intel culture", "literature aspire / claim");
    row(style, "intel books", "orb account books (BOOK-1)");
    row(style, "intel ops", "spy pipeline probe · exfil · sabotage · scan");
    row(style, "intel opsec", "veil · ward · detection");
    row(style, "intel link", "cross-pollination · license · council");
    row(style, "intel wants", "cognitive wants (novelty / resonance)");
    println!();
    println!("  {}", style.section("CRAFT (on bench)"));
    row(style, "bench intel", "forge chain · instruments · chem · zones");
    row(style, "bench chem · bench build · bench doctrine", "related components");
    foot_intel(style, None);
}

fn intel_research(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        intel_index(style);
        return;
    }
    if matches!(leaf_w.as_deref(), Some("forge")) {
        let cond = words.next().and_then(|w| w.parse::<u8>().ok());
        match cond.and_then(cfe::world::survey::ForgeableGrade::new) {
            Some(g) => match world.forge_instrument(current, g) {
                Ok(()) => println!(
                    "{} forged g{} · detection {}",
                    style.ok("◈"),
                    g.condition(),
                    world.detection_grade()
                ),
                Err(e) => println!("{} forge refused: {e:?}", style.bad("✗")),
            },
            None => println!(
                "{} {}",
                style.bad("✗"),
                style.strong("intel research forge <condition>")
            ),
        }
    }
    if matches!(leaf_w.as_deref(), Some("practice")) {
        if let Some(p) = words.next().and_then(|w| w.parse().ok()) {
            match world.adopt_practice(p) {
                Ok(()) => println!("{} practice {p}", style.ok("◈")),
                Err(e) => println!("{} {e:?}", style.bad("✗")),
            }
        }
    }
    let board = world.research_board(current);
    banner(style, "INTEL › RESEARCH");
    println!(
        "  held {} · forge-ready {} · instruments {} · practices {} · wants {}",
        board.held_count, board.forgeable_ready, board.instruments, board.practices, board.wants
    );
    if board.novelty_blocked {
        println!("  {}", style.warn("NOVELTY WANT — crystallize blocked"));
    }
    println!("  {} {}", style.warn("NEXT"), board.next_move);
    for n in &board.nodes {
        if n.threshold == 0 {
            continue;
        }
        let mark = if n.instrumented {
            "⚙"
        } else if n.held {
            "●"
        } else if n.progress > 0 {
            "○"
        } else {
            "·"
        };
        let bar = if n.threshold > 1 {
            format!(" {}/{}", n.progress, n.threshold)
        } else {
            String::new()
        };
        let forge = if n.forgeable { " ←FORGE" } else { "" };
        println!("  {mark} c{:<2} {:?}{bar}{forge}", n.condition, n.kind);
    }
    leaf(style, "intel research forge <c> · practice <n>", "deep deeds");
    leaf(style, "bench build", "smelter/synth components");
    foot_intel(style, Some("research"));
}

fn intel_knowledge(world: &World, style: &Style, current: u64) {
    banner(style, "INTEL › KNOWLEDGE BASES");
    // Tech
    println!("  {}", style.section("TECH"));
    let techs: Vec<_> = world.techs().iter().filter(|(o, _, _, _)| *o == current).collect();
    if techs.is_empty() {
        println!("  {}", style.dim("(empty)"));
    } else {
        for (_, c, h, mag) in &techs {
            println!("  · c{} mag {} {:02x}{:02x}…", c, amount(mag), h[0], h[1]);
        }
    }
    // Chem
    println!("  {}", style.section("CHEM CATALOG"));
    let cat = world.chem_catalog();
    if cat.is_empty() {
        println!("  {}", style.dim("(empty — bench chem · catalog)"));
    } else {
        for g in cat {
            let h = hex_prefix(g);
            println!("  · {}", &h[..8.min(h.len())]);
        }
    }
    // Stash
    println!("  {}", style.section("INTEL STASH"));
    let stash = world.intel_hashes();
    if stash.is_empty() {
        println!("  {}", style.dim("(empty — intel ops exfil)"));
    } else {
        for h in stash.iter().take(12) {
            println!("  · {:02x}{:02x}{:02x}{:02x}…", h[0], h[1], h[2], h[3]);
        }
    }
    // Literature
    println!("  {}", style.section("LITERATURE"));
    if world.literate(current) {
        for v in world.literature(current) {
            let st = if v.claimed {
                "CLAIMED"
            } else if v.witnesses > 0 {
                "aspired"
            } else {
                "—"
            };
            println!("  · {} · {st} · witnesses {}", v.name, v.witnesses);
        }
    } else {
        println!("  {}", style.dim("not literate — keep books first"));
    }
    // Practices + instruments
    println!(
        "  practices {:?} · instruments {}",
        world.committed().practices,
        world.instruments().iter().filter(|(o, _, _)| *o == current).count()
    );
    leaf(style, "intel culture · intel books · intel research · intel ops", "grow bases");
    foot_intel(style, Some("knowledge"));
}

fn intel_culture(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        intel_index(style);
        return;
    }
    if matches!(w.as_deref(), Some("aspire")) {
        if let Some(name) = words.next() {
            match world.aspire(current, &name) {
                Ok(_) => println!("{} aspired {name}", style.ok("◈")),
                Err(e) => println!("{} {e:?}", style.bad("✗")),
            }
        } else {
            println!("{} aspire <work-name>", style.bad("✗"));
        }
    }
    banner(style, "INTEL › CULTURE — literature");
    println!("  literate={}", world.literate(current));
    for v in world.literature(current) {
        println!(
            "  · {} · claimed={} · witnesses {}",
            v.name, v.claimed, v.witnesses
        );
    }
    leaf(style, "intel culture aspire <work>", "deep");
    leaf(style, "aspire · claim (CLI)", "full witness path");
    foot_intel(style, Some("culture"));
}

fn intel_books(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        intel_index(style);
        return;
    }
    banner(style, "INTEL › BOOKS — orb accounts (BOOK-1)");
    if let Some(o) = world.orb_by_id(current) {
        if o.books.is_empty() {
            println!("  {}", style.dim("no minted books — mint_book via books CLI"));
        } else {
            for b in &o.books {
                println!("  · {} · archived={}", b.name, b.archived);
            }
        }
    }
    leaf(style, "books mint|post|archive …", "deeds (CLI)");
    leaf(style, "intel knowledge", "see all bases");
    foot_intel(style, Some("books"));
    let _ = w;
}

fn intel_ops(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        intel_index(style);
        return;
    }
    match leaf_w.as_deref() {
        Some("probe") => match world.spy_probe(current) {
            Ok(d) => println!("{} probe depth {d}", style.ok("◈")),
            Err(e) => println!("{} {e:?}", style.bad("✗")),
        },
        Some("exfil") => {
            if let Some(c) = words.next().and_then(|w| w.parse().ok()) {
                match world.tech_exfil(current, c) {
                    Ok(h) => println!("{} exfil {:02x}{:02x}…", style.warn("EXFIL"), h[0], h[1]),
                    Err(e) => println!("{} {e:?}", style.bad("✗")),
                }
            } else {
                println!("{} intel ops exfil <condition>", style.bad("✗"));
            }
        }
        Some("sabotage") => match world.sabotage_instrument(current) {
            Ok(g) => println!("{} sabotaged g{g}", style.warn("SABOTAGE")),
            Err(e) => println!("{} {e:?}", style.bad("✗")),
        },
        Some("scan") => {
            println!(
                "  {}",
                style.hint("scan <exemplar-orb> — instrumented read (CLI verb)")
            );
        }
        _ => {}
    }
    let board = world.intel_board(current);
    banner(style, "INTEL › OPS");
    println!(
        "  g{} · inst {} · probes {} · stash {} · veiled={}",
        board.detection, board.instruments, board.probe_marks, board.intel_stash, board.veiled
    );
    println!("  {} {}", style.warn("NEXT"), board.next_hint);
    for (need, label) in [(0u8, "surface"), (1, "probe"), (2, "stratum"), (3, "optics")] {
        println!(
            "  {} g≥{need} {label}",
            if board.detection >= need {
                style.ok("✓")
            } else {
                style.dim("·")
            }
        );
    }
    leaf(style, "intel ops probe|exfil <c>|sabotage", "deep");
    leaf(style, "intel opsec · intel research forge", "ladder / veil");
    foot_intel(style, Some("ops"));
}

fn intel_opsec(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        intel_index(style);
        return;
    }
    match w.as_deref() {
        Some("veil") | Some("draw") => match world.draw_veil() {
            Ok(()) => println!("{} veil drawn", style.ok("◈")),
            Err(e) => println!("{} {e:?}", style.bad("✗")),
        },
        Some("lift") => match world.lift_veil() {
            Ok(()) => println!("{} veil lifted", style.ok("◈")),
            Err(e) => println!("{} {e:?}", style.bad("✗")),
        },
        Some("ward") => match world.ward_elder() {
            Ok(()) => println!("{} ward seated", style.ok("◈")),
            Err(e) => println!("{} {e:?}", style.bad("✗")),
        },
        Some("unward") => {
            println!("  {}", style.hint("ward lift <claimer> <depth> — CLI verb"));
        }
        _ => {}
    }
    let inf = world.inf_integrity();
    banner(style, "INTEL › OPSEC");
    println!(
        "  veiled={} · ward={} · detection g{} · instruments {} · toll {}",
        inf.veiled,
        inf.ward_state.word(),
        inf.detection_grade,
        inf.instrument_count,
        amount(&inf.veil_toll)
    );
    leaf(style, "intel opsec veil|lift|ward", "deep");
    leaf(style, "veil · ward · forge", "same deeds");
    foot_intel(style, Some("opsec"));
    let _ = current;
}

fn intel_link(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    _current: u64,
) {
    let w = words.next();
    if is_back(w.as_deref()) {
        intel_index(style);
        return;
    }
    let board = world.link_board();
    banner(style, "INTEL › LINK — cross-pollination");
    println!(
        "  score {} · crossed={} · councils {} open {} · bonds {} · fwd {}",
        amount(&board.score),
        board.crossed,
        board.councils,
        board.council_open,
        board.bonds,
        board.forwards
    );
    println!(
        "  treaties {} · war {} · licensable {:?}",
        board.treaties, board.at_war_count, board.techs_licensable
    );
    println!("  {} {}", style.warn("NEXT"), board.next_hint);
    leaf(style, "council · license … via …", "tech leap");
    leaf(style, "absorb · bond · forward", "mesh");
    leaf(style, "diplomatic local · diplomatic stellar", "oath / aperture RUN");
    leaf(style, "bench diplomatic", "craft councils/bonds/seats first");
    foot_intel(style, Some("link"));
    let _ = w;
}

fn intel_wants(world: &World, style: &Style, current: u64) {
    banner(style, "INTEL › WANTS — mind famine");
    let wants: Vec<_> = world.wants().iter().filter(|(o, _)| *o == current).collect();
    if wants.is_empty() {
        println!("  {}", style.dim("no cognitive wants struck"));
    } else {
        for (_, k) in wants {
            let name = match *k {
                1 => "epistemological",
                2 => "resonance",
                3 => "novelty",
                _ => "unknown",
            };
            println!("  · kind {k} ({name})");
        }
    }
    let board = world.research_board(current);
    if board.novelty_blocked {
        println!("  {}", style.warn("novelty blocks crystallize — license / discovery"));
    }
    leaf(style, "intel link · license", "feed novelty");
    leaf(style, "intel research", "see constellation");
    foot_intel(style, Some("wants"));
}

// ════════════════════════════════════════════════════════════════════
// trade — circulate / run
// ════════════════════════════════════════════════════════════════════

pub fn trade(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let domain = words.next();
    if is_back(domain.as_deref()) {
        trade_index(style);
        return;
    }
    match domain.as_deref() {
        None | Some("help") | Some("?") | Some("menu") | Some("index") => trade_index(style),
        Some("books") | Some("market") | Some("mkt") | Some("instruments") => {
            trade_books(world, style)
        }
        Some("stamp") | Some("money") | Some("unit") => trade_stamp(world, style, current),
        Some("element") | Some("matter") | Some("elem") => trade_element(world, style, current),
        Some("fleet") | Some("hull") | Some("ships") => trade_fleet(world, style, current),
        Some("rails") | Some("rail") | Some("anchor") => trade_rails(world, style),
        Some("vault") | Some("custody") => trade_vault(world, style, current),
        Some("aperture") | Some("haul") | Some("cross") => trade_aperture(style),
        // Diplomacy is its own root — redirect.
        Some("diplo")
        | Some("diplomatic")
        | Some("diplomacy")
        | Some("treaty")
        | Some("local") => {
            println!("  {} → {}", style.dim("↳"), style.key("diplomatic local"));
            let rest: Vec<String> = std::iter::once("local".into()).chain(words).collect();
            diplo(world, rest.into_iter(), style, current);
        }
        Some("stellar") | Some("interstellar") | Some("stars") | Some("union") => {
            println!("  {} → {}", style.dim("↳"), style.key("diplomatic stellar"));
            let rest: Vec<String> = std::iter::once("stellar".into()).chain(words).collect();
            diplo(world, rest.into_iter(), style, current);
        }
        Some("desk") => {
            println!("  {}", style.hint("desk enter · panes t|y|r|a|f"));
            foot_trade(style, None);
        }
        Some("craft") | Some("bench") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench trade"));
            trade_craft_bench(world, words, style, current);
        }
        Some("barter") | Some("clear") => {
            println!(
                "{} {}",
                style.dim("↳"),
                style.strong("trade <my> with <world> <their> deal <n>")
            );
            foot_trade(style, None);
        }
        Some(other) => {
            println!(
                "{} unknown trade domain {} — {}",
                style.bad("✗"),
                style.verb(other),
                style.key("trade")
            );
            trade_index(style);
        }
    }
}

fn trade_index(style: &Style) {
    banner(style, "TRADE — CIRCULATE / RUN (activity)");
    println!(
        "  {}",
        style.dim("Market/haul activity. Craft: `bench trade`. Diplomacy: root `diplomatic`.")
    );
    println!();
    row(style, "trade books", "offers · instruments · market war");
    row(style, "trade stamp", "mint · aperture (unit of account run)");
    row(style, "trade element", "element aperture · charter · refine path");
    row(style, "trade fleet", "capacity · assign · escort · risk");
    row(style, "trade rails", "R1–R4 · Anchor import/export");
    row(style, "trade vault", "custody · margin");
    row(style, "trade aperture", "resource/element/stamp haul");
    row(style, "trade desk", "interactive suite");
    println!();
    println!("  {}", style.section("CRAFT (on bench)"));
    row(style, "bench trade", "econ · fleet · offices · assay umbrella");
    row(style, "bench econ · bench fleet · bench offices", "direct craft domains");
    println!();
    println!("  {}", style.section("DIPLOMACY (own root)"));
    row(
        style,
        "diplomatic local · diplomatic stellar",
        "treaties · war · license (not under trade)",
    );
    row(style, "bench diplomatic", "craft councils · bonds · seats · routes");
    leaf(style, "trade <id> with <world> <id> deal <n>", "barter clear");
    foot_trade(style, None);
}

fn trade_books(world: &World, style: &Style) {
    banner(style, "TRADE › BOOKS");
    let mi = world.market_integrity();
    println!(
        "  sealed={} · Exchange={} · note/fut/share/perp={}/{}/{}/{} · offers {}",
        mi.sealed,
        world.exchange_seated(),
        mi.notes_open,
        mi.futures_open,
        mi.shares_open,
        mi.perps_open,
        world.offers().len()
    );
    println!(
        "  clear depth crystal {}/{}",
        mi.crystal_depth, mi.seal_need
    );
    leaf(style, "offer · note|future|share|perp · market · suite market", "");
    leaf(style, "trade stamp", "unit of account");
    foot_trade(style, Some("books"));
}

fn trade_stamp(world: &World, style: &Style, current: u64) {
    banner(style, "TRADE › STAMP — unit activity");
    let stamp = world
        .orb_by_id(current)
        .map(|o| o.stamp_amount())
        .unwrap_or_else(|| ratio(0, 1));
    println!("  free STAMP {} · vault {}", amount(&stamp), amount(&world.vault_balance_of(current)));
    leaf(style, "stamp mint · stamp aperture", "run unit");
    leaf(style, "trade vault", "custody");
    leaf(style, "bench econ", "craft side");
    foot_trade(style, Some("stamp"));
}

fn trade_element(world: &World, style: &Style, current: u64) {
    banner(style, "TRADE › ELEMENT — open-matter commerce");
    let n = world
        .orb_by_id(current)
        .map(|o| o.elements.len())
        .unwrap_or(0);
    println!(
        "  banked element kinds {} · element routes {}",
        n,
        world.element_routes().len()
    );
    leaf(style, "element aperture · element charter", "haul / lanes");
    leaf(style, "refine · e (list)", "path + inventory");
    leaf(style, "bench chem · planetary", "craft matter first");
    foot_trade(style, Some("element"));
}

fn trade_fleet(world: &World, style: &Style, current: u64) {
    banner(style, "TRADE › FLEET — run capacity");
    println!(
        "  hulls {} · free capacity {}",
        world.hulls().len(),
        amount(&world.free_capacity(0))
    );
    // KIN sample if any routes
    if let Some((deal, _)) = world.routes().first() {
        let k = world.kin_integrity(*deal);
        println!(
            "  sample deal {deal} · risk band {:?} · escorts…",
            world.convoy_risk_band(*deal)
        );
        let _ = k;
    }
    leaf(style, "fleet assign · defend escort|risk", "KIN run");
    leaf(style, "bench fleet", "craft hulls / yard");
    foot_trade(style, Some("fleet"));
    let _ = current;
}

fn trade_rails(world: &World, style: &Style) {
    banner(style, "TRADE › RAILS");
    let d = world.deep_rail_status();
    println!(
        "  Anchor={} strand={} court={} · element routes {}",
        d.anchor_seated,
        d.strand_enabled,
        d.court_enabled,
        world.element_routes().len()
    );
    leaf(style, "rail promote · anchor export|import", "");
    foot_trade(style, Some("rails"));
}

fn trade_vault(world: &World, style: &Style, current: u64) {
    banner(style, "TRADE › VAULT");
    let bal = world.vault_balance_of(current);
    let res = world.vault_reserved(current);
    let free = if bal > res {
        bal.clone() - res.clone()
    } else {
        ratio(0, 1)
    };
    println!(
        "  seated={} · custody {} · reserved {} · free {}",
        world.vault_seated(),
        amount(&bal),
        amount(&res),
        amount(&free)
    );
    leaf(style, "vault deposit|withdraw", "");
    leaf(style, "bench offices", "seat vault");
    foot_trade(style, Some("vault"));
}

fn trade_aperture(style: &Style) {
    banner(style, "TRADE › APERTURE");
    leaf(style, "aperture · element aperture · stamp aperture", "haul");
    leaf(style, "diplomatic local · diplomatic stellar", "gates");
    leaf(style, "bench diplomatic", "craft components");
    foot_trade(style, Some("aperture"));
}

// ════════════════════════════════════════════════════════════════════
// diplomatic — diplomacy activity (local + stellar)
// Preferred name: `diplomatic`. Short alias: `diplo` (still accepted).
// ════════════════════════════════════════════════════════════════════

/// Root diplomacy activity. Craft components on `bench diplomatic`.
pub fn diplo(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let domain = words.next();
    if is_back(domain.as_deref()) {
        diplo_index(style);
        return;
    }
    match domain.as_deref() {
        None | Some("help") | Some("?") | Some("menu") | Some("index") => diplo_index(style),
        Some("local") | Some("treaty") | Some("treaties") | Some("pair") => {
            // Deep deeds: further words fire the treaty grammar itself
            // (call-not-copy — the one home in `treaty_command`);
            // bare `diplomatic local` reads the board.
            let mut rest = words.peekable();
            match rest.peek().is_some() {
                true => crate::treaty_command(world, rest, style),
                false => diplo_local(world, style),
            }
        }
        Some("stellar") | Some("interstellar") | Some("stars") | Some("union") | Some("cross") => {
            diplo_stellar(world, style)
        }
        Some("mesh") | Some("council") | Some("bond") => diplo_mesh(world, style, current),
        Some("craft") | Some("bench") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench diplomatic"));
            diplo_craft_bench(world, words, style, current);
        }
        Some(other) => {
            println!(
                "{} unknown diplomatic domain {} — {}",
                style.bad("✗"),
                style.verb(other),
                style.key(diplomatic_key())
            );
            diplo_index(style);
        }
    }
}

fn diplo_index(style: &Style) {
    banner(style, "DIPLOMATIC — DIPLOMACY ACTIVITY");
    println!(
        "  {}",
        style.dim("Run treaties and stellar haul. Craft: `bench diplomatic`. Alias: diplo.")
    );
    println!();
    row(style, "diplomatic local", "pair treaties · renounce · same-system play");
    row(style, "diplomatic stellar", "crossed · war walls · aperture · license leap");
    row(style, "diplomatic mesh", "councils · bonds · forwards standing");
    println!();
    println!("  {}", style.section("CRAFT (on bench)"));
    row(
        style,
        "bench diplomatic",
        "council · bond · forward · seats · routes · escorts",
    );
    row(style, "bench offices · bench fleet · bench econ", "related components");
    leaf(style, "diplo …", "short alias for diplomatic …");
    foot_diplomatic(style, None);
}

fn diplo_local(world: &World, style: &Style) {
    banner(style, "DIPLOMATIC › LOCAL — pair treaties");
    println!(
        "  {}",
        style.dim("Same-system / pair play — does not require union crossing.")
    );
    let treaties = world.treaties();
    println!("  standing legs: {}", treaties.len());
    for t in treaties.iter().take(16) {
        println!("  · deal {} · {:?} · partner {}", t.deal, t.kind, t.partner);
    }
    leaf(style, "diplomatic local trade|defense|peace|war with <world>", "seal (deep deed)");
    leaf(style, "diplomatic local renounce <deal> <world>", "drop (deep deed)");
    leaf(style, "diplomatic local elements <deal> <world> <id>...", "fence (deep deed, E3.9)");
    leaf(style, "diplomatic stellar", "interstellar wall / haul / license");
    leaf(style, "bench diplomatic", "craft components first");
    foot_diplomatic(style, Some("local"));
}

fn diplo_stellar(world: &World, style: &Style) {
    banner(style, "DIPLOMATIC › STELLAR — interstellar");
    println!(
        "  crossed(union)={} · shielded={} · star={:?}",
        world.has_crossed(),
        world.is_shielded(),
        world.star()
    );
    let treaties = world.treaties();
    let wars: Vec<_> = treaties
        .iter()
        .filter(|t| t.kind == cfe::treaty::TreatyKind::War)
        .collect();
    println!("  war legs: {}", wars.len());
    for t in &wars {
        println!("  · WAR deal {} · {}", t.deal, t.partner);
    }
    let board = world.link_board();
    println!(
        "  link score {} · licensable {:?} · open councils {}",
        amount(&board.score),
        board.techs_licensable,
        board.council_open
    );
    println!("  {} {}", style.warn("NEXT"), board.next_hint);
    leaf(style, "aperture · element aperture · stamp aperture", "cross haul");
    leaf(style, "treaty war|peace with <world>", "stellar wall");
    leaf(style, "license <c> to <world> via <council> deal <n>", "tech leap");
    leaf(style, "intel link · diplomatic mesh", "prepare co-op");
    leaf(style, "bench diplomatic", "craft councils/bonds/routes");
    foot_diplomatic(style, Some("stellar"));
}

fn diplo_mesh(world: &World, style: &Style, _current: u64) {
    banner(style, "DIPLOMATIC › MESH — standing accords");
    let now = world.committed().orbs.first().map(|_| world.now()).unwrap_or(0);
    let board = world.link_board();
    println!(
        "  councils {} open {} · bonds {} · forwards {} · score {}",
        board.councils,
        board.council_open,
        board.bonds,
        board.forwards,
        amount(&board.score)
    );
    for c in world.councils().iter().take(8) {
        let open = now < c.sealed.saturating_add(c.window);
        println!(
            "  · council {} · members {} · open={}",
            c.council,
            c.members.len(),
            open
        );
    }
    for b in world.bonds().iter().take(8) {
        println!(
            "  · bond {}→{} · {:?}",
            b.source, b.target, b.resource
        );
    }
    leaf(style, "council · bond · forward · absorb", "deeds (CLI)");
    leaf(style, "bench diplomatic", "craft path");
    foot_diplomatic(style, Some("mesh"));
}

// ════════════════════════════════════════════════════════════════════
// polity — optional 5th root (vent open)
// ════════════════════════════════════════════════════════════════════
//
// Genesis · taxon · mint · AP as the LINE identity surface. Not dual-law
// craft (no `bench polity` yet) — activity root for who the player is and
// what the player may become. Deeds stay existing verbs; this hub routes + reads.

fn foot_polity(style: &Style, parent: Option<&str>) {
    println!();
    println!("  {}", style.section("↩ BACK-OUT"));
    leaf(style, "polity", "← POLITY root (optional 5th)");
    if let Some(p) = parent {
        leaf(style, &format!("polity {p}"), "← domain");
    }
    leaf(style, "back · .. · home", "polity root");
    peers_line(style);
    leaf(style, "polity · bench · intel · trade · diplomatic", "five roots (polity optional)");
}

/// Optional 5th root — line identity (genesis · taxon · mint · AP).
pub fn polity(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let domain = words.next();
    if is_back(domain.as_deref()) {
        polity_index(world, style, current);
        return;
    }
    match domain.as_deref() {
        None | Some("help") | Some("?") | Some("menu") | Some("index") => {
            polity_index(world, style, current)
        }
        Some("genesis") | Some("axes") | Some("civ") => polity_genesis(world, style),
        Some("taxon") | Some("species") | Some("clade") | Some("imperium") => {
            polity_taxon(world, style, current)
        }
        Some("mint") | Some("discovery") | Some("warrant") => polity_mint(world, style, current),
        Some("ap") | Some("budget") | Some("action") => polity_ap(world, style),
        Some("craft") | Some("bench") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench polity"));
            polity_craft_bench(world, words, style, current);
        }
        Some("labor") | Some("population") | Some("hab") => {
            let labor = world.labor_board(current);
            banner(style, "POLITY › LABOR — habitats + clones (P8.3)");
            println!(
                "  force {} · habitats {} · clones {} · work slots {}",
                labor.force, labor.habitats, labor.clones, labor.work_slots
            );
            println!("  {}", style.hint(&labor.next_hint));
            leaf(style, "bench polity labor", "craft path");
            leaf(style, "zone <tile> hab", "seat habitat");
            foot_polity(style, Some("labor"));
        }
        Some(other) => {
            println!(
                "{} unknown polity domain {} — {}",
                style.bad("✗"),
                style.verb(other),
                style.key("polity")
            );
            polity_index(world, style, current);
        }
    }
}

fn polity_index(world: &World, style: &Style, current: u64) {
    banner(style, "POLITY — LINE IDENTITY (optional 5th root · vent open)");
    println!(
        "  {}",
        style.dim(
            "Who you are. Genesis axes · taxon economy · discovery mint · shared AP. No bench twin yet."
        )
    );
    println!();
    // Live strip.
    match world.genesis() {
        Some(g) => println!(
            "  {} {} · dis {} · met {} · per {}",
            style.section("genesis"),
            style.accent(&g.civilization),
            g.disposition,
            g.metabolism,
            g.perception
        ),
        None => println!("  {} {}", style.section("genesis"), style.warn("unsealed — `genesis …`")),
    }
    let eco = world.taxon_economy_of(current);
    println!(
        "  {} mode {:?} · fit {:?} · flatlander={}",
        style.section("taxon"),
        eco.mode,
        eco.fit,
        eco.flatlander
    );
    let ap = world.ap_board();
    println!(
        "  {} {}/{} · habitats {} · {}",
        style.section("AP"),
        ap.available,
        ap.capacity,
        ap.habitats,
        style.dim(&format!("window t{}–t{}", ap.window_start, ap.window_end))
    );
    println!();
    row(style, "polity genesis", "seal / read disposition · metabolism · perception");
    row(style, "polity taxon", "class · clade · economy mode · planet fit");
    row(style, "polity mint", "discovery mint path (warrant · crystal · no free mint)");
    row(style, "polity ap", "P8.1 shared action budget board");
    row(style, "polity labor", "P8.3 force from habitats + clones");
    println!();
    println!("  {}", style.section("CRAFT (on bench)"));
    row(style, "bench polity", "axes · offices · doctrine · labor · practice (POL.2 dual law)");
    leaf(style, "law", "no free mint · call-not-copy · SEAL B — same as peer roots");
    foot_polity(style, None);
}

fn polity_genesis(world: &World, style: &Style) {
    banner(style, "POLITY › GENESIS — sealed axes");
    match world.genesis() {
        Some(g) => {
            println!("  civilization {}", style.accent(&g.civilization));
            println!(
                "  disposition {} · metabolism {} · perception {}",
                g.disposition, g.metabolism, g.perception
            );
            if let Some(axes) = world.civ_axes() {
                println!(
                    "  civ_axes dis={} met={} per={}",
                    axes.disposition, axes.metabolism, axes.perception
                );
            }
        }
        None => {
            println!("  {}", style.warn("genesis unsealed"));
            leaf(style, "genesis <name> <dis> <met> <per>", "seal axes (CLI)");
        }
    }
    leaf(style, "status", "full colony strip");
    foot_polity(style, Some("genesis"));
}

fn polity_taxon(world: &World, style: &Style, current: u64) {
    banner(style, "POLITY › TAXON — class · clade · economy");
    match world.committed().taxon {
        Some((class, clade)) => println!(
            "  sealed class {} · clade {}",
            style.strong(&class.to_string()),
            style.strong(&clade.to_string())
        ),
        None => println!(
            "  {}",
            style.dim("taxon unsealed — flatlander baseline for economy reads")
        ),
    }
    let eco = world.taxon_economy_of(current);
    println!(
        "  mode {:?} · planet fit {:?} · flatlander={}",
        eco.mode, eco.fit, eco.flatlander
    );
    println!(
        "  regenerator tech need {} · extractor soft-cap {}",
        world.regenerator_tech_need(),
        world.extractor_soft_cap()
    );
    leaf(style, "taxon …", "seal class/clade (CLI — see help taxon)");
    leaf(style, "docs/37_IMPERIUM_TAXON.md", "imperium words · open clade");
    foot_polity(style, Some("taxon"));
}

fn polity_mint(world: &World, style: &Style, current: u64) {
    banner(style, "POLITY › MINT — discovery (no free mint)");
    let crystal = world
        .orb_by_id(current)
        .or_else(|| world.orb())
        .map(|o| amount(&o.banked_amount(Resource::Crystal)))
        .unwrap_or_else(|| "0/1".into());
    println!("  crystal bank {}", style.money(&crystal));
    println!(
        "  {}",
        style.dim("Discovery mint requires sealed work warrant — never free crystal.")
    );
    leaf(style, "mint discovery …", "warranted mint path (CLI)");
    leaf(style, "stamp mint <amt>", "1:1 crystal→STAMP convert (not discovery)");
    leaf(style, "polity ap", "AP gates build/prospect/research/quote");
    foot_polity(style, Some("mint"));
}

fn polity_ap(world: &World, style: &Style) {
    banner(style, "POLITY › AP — shared action budget (P8.1)");
    let b = world.ap_board();
    println!(
        "  available {} / capacity {} · spent {} · habitats {}",
        style.strong(&b.available.to_string()),
        b.capacity,
        b.spent,
        b.habitats
    );
    println!(
        "  window t{}–t{} · now t{}",
        b.window_start, b.window_end, b.now
    );
    println!("  {}", style.hint(&b.next_hint));
    println!();
    println!("  {}", style.section("GATED FAMILIES (1 AP each)"));
    leaf(style, "build", "zone …");
    leaf(style, "prospect", "prospect <stratum>");
    leaf(style, "research", "forge instrument · practice adopt");
    leaf(style, "quote", "offer / post_offer");
    leaf(style, "refill", "year-voice boundary (60 ticks)");
    foot_polity(style, Some("ap"));
}
