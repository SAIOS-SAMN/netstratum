//! Root hubs — peers (see `docs/HUBS.md`).
//!
//! | Root | Role |
//! |------|------|
//! | **bench** | BUILD components (every activity root has craft leaves here) |
//! | **intel** | KNOWLEDGE activity |
//! | **trade** | CIRCULATE / market activity |
//! | **diplo** | DIPLOMACY activity (local + stellar) |
//!
//! Dual law: every standalone activity root **S** has craft surface **`bench S`**.
//! | Activity | Craft twin |
//! |----------|------------|
//! | `intel`  | `bench intel`  (tools / forge chain) |
//! | `trade`  | `bench trade`  (econ · fleet · offices umbrella) |
//! | `diplo`  | `bench diplo`  (councils · bonds · seats · routes) |

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
    leaf(style, "bench · intel · trade · diplo", "peer roots");
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

fn foot_diplo(style: &Style, parent: Option<&str>) {
    println!();
    println!("  {}", style.section("↩ BACK-OUT"));
    leaf(style, "diplo", "← DIPLOMACY root");
    if let Some(p) = parent {
        leaf(style, &format!("diplo {p}"), "← domain");
    }
    leaf(style, "back · .. · home", "diplo root");
    leaf(style, "bench diplo", "← craft diplomatic components");
    peers_line(style);
}

// ════════════════════════════════════════════════════════════════════
// BENCH — BUILD
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
        Some("chem") | Some("chemistry") | Some("science") | Some("bond") => {
            chem_bench(world, words, style, current)
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
        Some("diplo") | Some("diplomacy") | Some("diplomatics") | Some("pact") => {
            diplo_craft_bench(world, words, style, current)
        }
        Some("intel") | Some("intelligence") | Some("intel-craft") | Some("knowledge-craft") => {
            intel_craft_bench(world, words, style, current)
        }
        Some("trade") | Some("trades") | Some("circulate") | Some("trade-craft") => {
            trade_craft_bench(world, words, style, current)
        }
        // Activity-domain shortcuts still land on intel activity (not craft).
        Some("research") | Some("tech") | Some("practice") | Some("lab") => {
            println!("  {} → {}", style.dim("↳"), style.key("intel research"));
            let rest: Vec<String> = std::iter::once("research".into()).chain(words).collect();
            intel(world, rest.into_iter(), style, current);
        }
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
            "Craft only. Dual law: activity root S ⇒ craft surface `bench S` (intel · trade · diplo)."
        )
    );
    println!();
    println!("  {}", style.section("MATTER & PLACE"));
    row(style, "bench planetary", "bio · chem · terraform · medium · thermal · balance");
    row(style, "bench build", "zones · yard · refinery · regen");
    row(style, "bench star", "commission · site · age · rings");
    row(style, "bench estate", "chart · parcels · appraise · ladder");
    row(style, "bench medium", "ATM envelope · reinforce · tamper");
    println!();
    println!("  {}", style.section("CRAFT DOMAINS"));
    row(style, "bench chem", "gems/recipes → intel knowledge");
    row(style, "bench econ", "instruments · routes · rails (also under bench trade)");
    row(style, "bench fleet", "hulls · yard (also under bench trade)");
    row(style, "bench offices", "seats (trade vault/books · diplo)");
    row(style, "bench assay", "T7 court craft");
    row(style, "bench doctrine", "posture/KIN prep → trade fleet defend");
    println!();
    println!("  {}", style.section("DUAL LAW — activity roots as craft twins"));
    row(style, "bench intel", "forge chain · instruments · chem → root `intel`");
    row(style, "bench trade", "econ · fleet · offices umbrella → root `trade`");
    row(style, "bench diplo", "council · bond · forward · seats · routes → root `diplo`");
    println!();
    println!("  {}", style.section("ACTIVITY PEERS (run, not craft)"));
    row(style, "intel", "KNOWLEDGE activity");
    row(style, "trade", "CIRCULATE activity");
    row(style, "diplo", "DIPLOMACY activity (local · stellar)");
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
            row(style, "bench planetary terraform", "TERRA crown");
            row(style, "bench planetary medium", "→ medium domain");
            row(style, "bench planetary thermal", "heat · encase · cryo");
            row(style, "bench planetary balance", "entropy ∥ syntropy");
            leaf(style, "prospect · assay · refine", "veins");
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
        Some("terra") | Some("terraform") | Some("tf") => {
            banner(style, "BENCH › PLANETARY › TERRAFORM");
            println!("  can_terraform={}", world.can_terraform());
            let d = world.dual_use_atm();
            println!(
                "  dual-use tax={} · overt={} · covert={}",
                d.dual_use_tax, d.can_overt_terraform, d.can_covert_tamper
            );
            leaf(style, "terraform …", "crown deed");
            leaf(style, "bench medium", "ATM dual-use path");
            foot_bench(style, Some("planetary"));
        }
        Some("medium") | Some("atm") => medium_bench(world, words, style, current),
        Some("thermal") | Some("heat") => {
            banner(style, "BENCH › PLANETARY › THERMAL");
            leaf(style, "thermal", "band reading");
            leaf(style, "zone <tile> enc|cryo", "encasement · cryo array");
            leaf(style, "bench star", "site under sun");
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
            println!("{} planetary leaf {} — bio|chem|terraform|medium|thermal|balance|back", style.bad("✗"), other);
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
    if matches!(w.as_deref(), Some("age")) {
        println!(
            "  {}",
            style.hint("`age <class>` or thermal CLI — age_star takes StarClass")
        );
    }
    banner(style, "BENCH › STAR");
    match world.star() {
        Some(c) => println!("  sun · class {:?}", c),
        None => println!("  {}", style.warn("no sun — `star <class>`")),
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
    leaf(style, "star <class> · site · thermal · rings · sky", "deeds");
    leaf(style, "bench star age", "deep: age_star year voice");
    leaf(style, "terraform", "earned crown");
    println!("  {}", style.hint("Open: stellar capacity game (C1.7)."));
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
    leaf(style, "bench medium tamper", "deep: covert ATM (taxed if TERRA)");
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
        banner(style, "BENCH › CHEM — craft");
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
        leaf(style, "compound · iso · catalog", "deeds");
        leaf(style, "intel knowledge", "catalog appears in knowledge bases");
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

/// Craft surface for diplomacy — components the **diplo** root then runs.
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
    banner(style, "BENCH › DIPLO — diplomatic components (craft)");
    println!(
        "  {}",
        style.dim("Build the pieces. Run treaties/hauls on root `diplo` (not here).")
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
    leaf(style, "diplo local", "seal/renounce treaties");
    leaf(style, "diplo stellar", "war wall · aperture · license leap");
    leaf(style, "intel link", "license path readiness");
    foot_bench(style, Some("diplo"));
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
    leaf(style, "diplo", "treaties are their own root (bench diplo crafts them)");
    foot_bench(style, Some("trade"));
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
// INTEL — KNOWLEDGE ACTIVITY
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
    leaf(style, "diplo local · diplo stellar", "oath / aperture RUN");
    leaf(style, "bench diplo", "craft councils/bonds/seats first");
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
// TRADE — CIRCULATE / RUN
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
        Some("diplo") | Some("diplomacy") | Some("treaty") | Some("local") => {
            println!("  {} → {}", style.dim("↳"), style.key("diplo local"));
            let rest: Vec<String> = std::iter::once("local".into()).chain(words).collect();
            diplo(world, rest.into_iter(), style, current);
        }
        Some("stellar") | Some("interstellar") | Some("stars") | Some("union") => {
            println!("  {} → {}", style.dim("↳"), style.key("diplo stellar"));
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
        style.dim("Market/haul activity. Craft: `bench trade`. Diplomacy: root `diplo`.")
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
    row(style, "diplo local · diplo stellar", "treaties · war · license (not under trade)");
    row(style, "bench diplo", "craft councils · bonds · seats · routes");
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
    leaf(style, "diplo local · diplo stellar", "gates");
    leaf(style, "bench diplo", "craft components");
    foot_trade(style, Some("aperture"));
}

// ════════════════════════════════════════════════════════════════════
// DIPLO — DIPLOMACY ACTIVITY (local + stellar)
// ════════════════════════════════════════════════════════════════════

/// Root diplomacy activity. Craft components on `bench diplo`.
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
            diplo_local(world, style)
        }
        Some("stellar") | Some("interstellar") | Some("stars") | Some("union") | Some("cross") => {
            diplo_stellar(world, style)
        }
        Some("mesh") | Some("council") | Some("bond") => diplo_mesh(world, style, current),
        Some("craft") | Some("bench") => {
            println!("  {} → {}", style.dim("↳"), style.key("bench diplo"));
            diplo_craft_bench(world, words, style, current);
        }
        Some(other) => {
            println!(
                "{} unknown diplo domain {} — {}",
                style.bad("✗"),
                style.verb(other),
                style.key("diplo")
            );
            diplo_index(style);
        }
    }
}

fn diplo_index(style: &Style) {
    banner(style, "DIPLO — DIPLOMACY ACTIVITY");
    println!(
        "  {}",
        style.dim("Run treaties and stellar haul. Craft components: `bench diplo`.")
    );
    println!();
    row(style, "diplo local", "pair treaties · renounce · same-system play");
    row(style, "diplo stellar", "crossed · war walls · aperture · license leap");
    row(style, "diplo mesh", "councils · bonds · forwards standing");
    println!();
    println!("  {}", style.section("CRAFT (on bench)"));
    row(style, "bench diplo", "council · bond · forward · seats · routes · escorts");
    row(style, "bench offices · bench fleet · bench econ", "related components");
    foot_diplo(style, None);
}

fn diplo_local(world: &World, style: &Style) {
    banner(style, "DIPLO › LOCAL — pair treaties");
    println!(
        "  {}",
        style.dim("Same-system / pair play — does not require union crossing.")
    );
    let treaties = world.treaties();
    println!("  standing legs: {}", treaties.len());
    for t in treaties.iter().take(16) {
        println!("  · deal {} · {:?} · partner {}", t.deal, t.kind, t.partner);
    }
    leaf(style, "treaty trade|defense|peace|war with <world>", "seal");
    leaf(style, "treaties · renounce <deal> <world>", "list / drop");
    leaf(style, "diplo stellar", "interstellar wall / haul / license");
    leaf(style, "bench diplo", "craft components first");
    foot_diplo(style, Some("local"));
}

fn diplo_stellar(world: &World, style: &Style) {
    banner(style, "DIPLO › STELLAR — interstellar");
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
    leaf(style, "intel link · diplo mesh", "prepare co-op");
    leaf(style, "bench diplo", "craft councils/bonds/routes");
    foot_diplo(style, Some("stellar"));
}

fn diplo_mesh(world: &World, style: &Style, _current: u64) {
    banner(style, "DIPLO › MESH — standing accords");
    let now = world.committed().orbs.first().map(|_| world.now()).unwrap_or(0);
    // use link board + list councils/bonds
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
    leaf(style, "bench diplo", "craft path");
    foot_diplo(style, Some("mesh"));
}
