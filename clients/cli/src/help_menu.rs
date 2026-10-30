//! Structured colored help menus for the CLI and trade desk.

use crate::style::Style;

/// Dispatch `help [topic]`.
pub fn help(style: Style, topic: Option<&str>) {
    match topic.map(|t| t.to_ascii_lowercase()).as_deref() {
        None | Some("") | Some("all") | Some("index") => index(style),
        Some("desk") | Some("trade-desk") | Some("t") => desk_help(style),
        Some("bench") | Some("workbench") | Some("make") => bench_help(style),
        Some("intel") | Some("intelligence") | Some("knowledge") | Some("kb") => intel_help(style),
        Some("trade-hub") | Some("trades") | Some("circulate") => trade_hub_help(style),
        Some("diplomatic")
        | Some("diplomacy")
        | Some("diplomatics")
        | Some("diplo") => diplo_help(style),
        Some("polity") | Some("line") | Some("identity") | Some("ap") => polity_help(style),
        Some("stamp") | Some("money") => stamp_help(style),
        Some("element") | Some("elements") | Some("matter") => element_help(style),
        Some("fleet") | Some("hull") | Some("ship") => fleet_help(style),
        Some("market") | Some("trade") | Some("offer") => market_help(style),
        Some("route") | Some("routes") | Some("charter") => routes_help(style),
        Some("colony") | Some("grow") | Some("build") => colony_help(style),
        Some("topics") | Some("list") => topics(style),
        Some(other) => {
            println!(
                "{} unknown help topic {} — try {}",
                style.bad("✗"),
                style.verb(other),
                style.key("help topics")
            );
            topics(style);
        }
    }
}

fn banner_line(style: Style, title: &str) {
    println!("{}", style.border("════════════════════════════════════════════════════════════"));
    println!("  {}", style.title(title));
    println!("{}", style.border("════════════════════════════════════════════════════════════"));
}

fn row(style: Style, verb: &str, desc: &str) {
    // Fixed width verb column with color on the verb only.
    let plain_w = 34usize;
    let pad = if verb.len() < plain_w {
        " ".repeat(plain_w - verb.len())
    } else {
        " ".into()
    };
    println!("  {}{}{}", style.verb(verb), pad, style.dim(desc));
}

fn index(style: Style) {
    banner_line(style, "NETSTRATUM — HELP INDEX");
    println!(
        "  {} for a topic · {} lists topics · full color ({} disables)",
        style.key("help <topic>"),
        style.key("help topics"),
        style.dim("--plain / NO_COLOR")
    );
    println!();
    println!("  {}", style.section("ROOT HUBS (peers)"));
    row(style, "bench", "BUILD components (+ craft leaves for every activity root)");
    row(style, "intel", "KNOWLEDGE activity — research · bases · ops · link");
    row(style, "trade", "CIRCULATE activity — books · stamp · element · fleet · rails");
    row(style, "diplomatic", "DIPLOMACY activity — local · stellar · mesh (alias: diplo)");
    row(style, "polity", "LINE IDENTITY — genesis · taxon · mint · AP (optional 5th · vent open)");
    row(style, "help bench|intel|trade-hub|diplomatic|polity", "hub maps · docs/HUBS.md");
    println!();
    println!("  {}", style.section("TRADE DESK (Phase 1–2)"));
    row(style, "desk · desk enter", "interactive trade / diplo / routes / arb");
    row(style, "help desk", "full desk keybinds and panes");
    row(style, "stamp · stamp mint · stamp aperture", "STAMP unit of account");
    row(style, "element aperture · element charter", "open-matter haul + lanes");
    row(style, "offer · trade <ids> with … · aperture", "barter clear + ROUTE-1");
    row(style, "treaty · treaties", "trade · defense · peace · war");
    println!();
    println!("  {}", style.section("COLONY LOOP"));
    row(style, "found · zone · tick · collect", "found and run a settlement");
    row(style, "star · site · thermal", "star + orbit (alignment feeds melt)");
    row(style, "prospect · assay · refine path", "elements and refinery");
    row(style, "map · status · dashboard", "board readings");
    println!();
    println!("  {}", style.section("META"));
    row(style, "save · resume <world>", "persist / reload chronicle");
    row(style, "verify · replay · chronicle", "SEAL B tools");
    row(style, "help topics", "every help topic name");
    println!();
    println!(
        "  {}",
        style.hint("Tip: `bench` makes · `trade` circulates · desk for interactive panes.")
    );
}

fn topics(style: Style) {
    banner_line(style, "HELP TOPICS");
    for (name, blurb) in [
        ("bench", "BUILD hub — craft (+ science · dual twins)"),
        ("intel", "KNOWLEDGE activity — research · bases · ops · link"),
        ("trade-hub", "CIRCULATE activity — market · haul · vault"),
        ("diplomatic", "DIPLOMACY activity — local · stellar · mesh (alias: diplo)"),
        ("polity", "LINE IDENTITY — genesis · taxon · mint · AP (optional 5th)"),
        ("desk", "trade desk panes and keybinds"),
        ("stamp", "STAMP mint and aperture"),
        ("element", "element haul and charter"),
        ("fleet", "scrap, yard, hulls, capacity"),
        ("market", "offers and trade clears"),
        ("routes", "ROUTE-1 and element lanes"),
        ("colony", "found / zone / tick / collect"),
        ("topics", "this list"),
    ] {
        row(style, name, blurb);
    }
    println!(
        "\n  {} {}",
        style.label("usage:"),
        style.verb("help <topic>")
    );
}

fn bench_help(style: Style) {
    banner_line(style, "BENCH — BUILD COMPONENTS");
    println!(
        "  {}",
        style.dim("Craft only. Dual law: activity root S ⇒ bench S. docs/HUBS.md")
    );
    println!();
    println!("  {}", style.section("MATTER & PLACE"));
    row(style, "bench planetary …", "bio · chem · worldshape · medium · thermal · balance");
    row(style, "bench build", "zones · yard · refinery · regen");
    row(style, "bench star", "commission · site · age · rings");
    row(style, "bench estate", "chart · ladder · parcels · appraise");
    row(style, "bench medium", "ATM reinforce · tamper");
    println!();
    println!("  {}", style.section("SCIENCE LABS (seated · dual ± · explosion)"));
    row(style, "bench science", "seats board · clones · synth · skin · rogue");
    row(style, "bench science seat <d>", "pay crystal · unlock rank 1/2");
    row(style, "bench science preview <path>", "known/hidden poles + risk");
    row(style, "bench science create <path>", "chem|biochem|synth|clone|skin");
    row(style, "bench chem", "chemistry (needs chem seat for catalog/iso)");
    row(style, "bench biology · physics · math · research", "discipline craft");
    row(style, "bench science synth|clone|skin", "mod paths (matrix / pop / face)");
    println!();
    println!("  {}", style.section("CRAFT DOMAINS"));
    row(style, "bench econ", "instruments · routes · rails");
    row(style, "bench offices", "seat exchange|vault|assay|anchor");
    row(style, "bench fleet", "yard · mint hull · scrap");
    row(style, "bench assay", "T7 court · challenge");
    row(style, "bench doctrine", "posture · KIN prep");
    println!();
    println!("  {}", style.section("DUAL LAW — activity roots as craft twins"));
    row(style, "bench intel", "knowledge tools → root intel");
    row(style, "bench trade", "econ · fleet · offices → root trade");
    row(style, "bench diplomatic", "councils · bonds · seats → root diplomatic");
    println!();
    println!("  {}", style.section("ACTIVITY PEERS"));
    row(style, "intel · trade · diplomatic", "run (not craft; diplo = short alias)");
    row(style, "bench · back · home", "← bench root");
}

fn diplo_help(style: Style) {
    banner_line(style, "DIPLOMATIC — DIPLOMACY ACTIVITY");
    println!(
        "  {}",
        style.dim("Preferred: `diplomatic`. Alias: `diplo`. Craft: `bench diplomatic`. docs/HUBS.md")
    );
    println!();
    row(style, "diplomatic local", "pair treaties · renounce");
    row(style, "diplomatic stellar", "crossed · war · aperture · license");
    row(style, "diplomatic mesh", "councils · bonds · forwards");
    row(style, "bench diplomatic", "CRAFT components for diplomacy");
    println!();
    row(style, "diplomatic · back · home", "← diplomatic root");
    row(style, "diplo …", "short alias still accepted");
    row(style, "bench · intel · trade · polity", "peer roots");
}

fn polity_help(style: Style) {
    banner_line(style, "POLITY — LINE IDENTITY (5th root · dual law)");
    println!(
        "  {}",
        style.dim("Genesis · taxon · mint · AP · labor. Craft: `bench polity`. docs/HUBS.md")
    );
    println!();
    row(style, "polity", "index · live genesis/taxon/AP/labor strip");
    row(style, "polity genesis", "sealed axes · seal path");
    row(style, "polity taxon", "class · clade · economy mode · planet fit");
    row(style, "polity mint", "discovery mint (no free mint)");
    row(style, "polity ap", "P8.1 shared AP budget board");
    row(style, "polity labor", "P8.3 force from habitats + clones");
    row(style, "bench polity", "CRAFT offices · doctrine · labor · practice");
    println!();
    row(style, "polity · back · home", "← polity root");
    row(style, "bench · intel · trade · diplomatic", "peer roots");
}

fn intel_help(style: Style) {
    banner_line(style, "INTEL — KNOWLEDGE ACTIVITY");
    println!(
        "  {}",
        style.dim("Operate knowledge. Craft tools: `bench intel`. docs/HUBS.md")
    );
    println!();
    row(style, "intel research [forge|practice]", "constellation game");
    row(style, "intel knowledge", "tech · chem · stash · lit · practices");
    row(style, "intel culture", "literature aspire / claim");
    row(style, "intel books", "BOOK-1 orb accounts");
    row(style, "intel ops [probe|exfil|sabotage]", "spy pipeline");
    row(style, "intel opsec [veil|lift|ward]", "veil · ward · detection");
    row(style, "intel link", "cross-pollination · license");
    row(style, "intel wants", "cognitive wants");
    println!();
    row(style, "bench intel", "CRAFT components for knowledge");
    row(style, "bench research · bench science", "science craft twins");
    row(style, "intel · back · home", "← intel root");
    row(style, "bench · trade · diplomatic", "peer roots");
}

fn trade_hub_help(style: Style) {
    banner_line(style, "TRADE — CIRCULATE ACTIVITY");
    println!(
        "  {}",
        style.dim("Run components. Craft: `bench trade`. Knowledge: intel. docs/HUBS.md")
    );
    println!();
    row(style, "trade books", "offers · instruments · market war");
    row(style, "trade stamp", "mint · aperture unit run");
    row(style, "trade element", "element aperture · charter");
    row(style, "trade fleet", "capacity · escort · risk");
    row(style, "trade rails", "R1–R4 · Anchor");
    row(style, "trade vault", "custody · margin");
    row(style, "trade aperture", "cross-world haul");
    row(style, "trade diplo", "→ diplomatic local (own root)");
    row(style, "trade stellar", "→ diplomatic stellar (own root)");
    row(style, "trade desk", "interactive suite");
    println!();
    row(style, "bench trade", "CRAFT econ · fleet · offices");
    println!();
    row(
        style,
        "trade <my> with <world> <their> deal <n>",
        "barter clear",
    );
    row(style, "trade · back · home", "← trade root");
    row(style, "bench · intel · diplomatic", "peer roots");
}

fn desk_help(style: Style) {
    banner_line(style, "TRADE DESK — INTERACTIVE SUITE");
    println!(
        "  {}              open TRADE pane once",
        style.verb("desk")
    );
    println!(
        "  {}       open a pane once",
        style.verb("desk t|y|r|a|f|x|g|d|e")
    );
    println!(
        "  {}        interactive loop (desk» prompt)",
        style.verb("desk enter")
    );
    println!();
    println!("  {}", style.section("PANES (keys in desk»)"));
    row(style, "t  TRADE", "peers · D_chart · D_eff · melt · alignment · door");
    row(style, "y  DIPLOMATIC", "standing treaties · renounce hint");
    row(style, "r  ROUTES", "carrier + element lanes · melt windows");
    row(style, "a  ARB", "read-only merchant triangles (walled + bridge)");
    row(style, "f  FLEET", "hulls · scrap · free cap");
    row(style, "x  EXCH", "STAMP · books · instruments");
    row(style, "g  GROW", "colony strip / zones");
    row(style, "d  DEFEND", "posture · escort · veil/ward");
    row(style, "e  EXPLORE", "banked elements / prospects");
    row(style, "q  quit", "leave desk loop");
    row(style, "?  help", "this reminder inside desk");
    println!();
    println!("  {}", style.section("HUB DEEP-LINKS (H1.3)"));
    row(style, "h · hub", "open primary hub domain for this pane");
    row(style, "hc · craft", "open craft twin (bench) for this pane");
    row(style, "t → trade books · bench trade", "circulate + dual craft");
    row(style, "y → diplomatic local · bench diplomatic", "diplomacy activity + craft");
    row(style, "r → trade rails · bench econ", "lanes + route craft");
    row(style, "a → trade books · bench trade", "triangles → books");
    row(style, "f → trade fleet · bench fleet", "run hulls + craft hulls");
    row(style, "x → trade books · bench offices", "instruments + seats");
    row(style, "g → bench planetary · bench build", "colony craft");
    row(style, "d → intel opsec · bench doctrine", "knowledge + KIN craft");
    row(style, "e → intel research · bench research", "knowledge + research craft");
    println!();
    println!("  {}", style.section("DEEDS (type any world verb inside desk»)"));
    row(style, "offer / trade / aperture / charter", "carrier market + lanes");
    row(style, "stamp mint|aperture", "money path");
    row(style, "element aperture|charter", "open matter path");
    row(style, "treaty …", "diplomacy");
    row(style, "bench · intel · trade · diplomatic …", "any hub verb (same as root CLI)");
    println!();
    println!(
        "  {}",
        style.hint("Melt = pair_melt(D_eff); D_eff = D_chart + D_orbit (Phase-2 alignment).")
    );
    println!(
        "  {}",
        style.hint("No desk ledger — every action is a chronicle deed (SEAL B). Hubs: docs/HUBS.md.")
    );
}

fn fleet_help(style: Style) {
    banner_line(style, "FLEET — SCRAP · YARD · HULLS");
    row(style, "zone <tile> yard", "build ShipYard (smelter cost)");
    row(style, "fleet · fleet list", "scrap, free capacity, hull roster");
    row(style, "fleet salvage <amt>", "iron → scrap 1:1 at yard");
    row(
        style,
        "fleet salvage element <amt> <prefix>",
        "element → scrap (alchemy surplus → ships)",
    );
    row(style, "fleet mint <role> [grade]", "hauler|semi|runner|escort");
    row(style, "fleet assign <id> <deal|0>", "bind hull to route (0=idle)");
    row(style, "fleet scrap <id>", "dismantle hull, recover half scrap");
    println!();
    println!(
        "  {}",
        style.hint("Positive-melt hauls need free hull capacity. Metabolism filters roles.")
    );
    println!(
        "  {}",
        style.hint("Entropy-dominant balance gates hull mint — build regenerator (34_MATTER_BALANCE).")
    );
    println!(
        "  {}",
        style.hint("desk f — FLEET pane. NPC fleet_step salvages/mints under pure era.")
    );
}

fn stamp_help(style: Style) {
    banner_line(style, "STAMP — UNIT OF ACCOUNT");
    row(style, "stamp · stamp balance", "show STAMP and crystal stock");
    row(style, "stamp mint <amt>", "crystal → STAMP at 1:1 identity");
    row(
        style,
        "stamp aperture <amt> to <world> deal <n>",
        "cross-world STAMP (pays melt on D_eff)",
    );
    println!();
    println!(
        "  {}",
        style.hint("Mint is not a faucet — crystal must be banked (discovery / resonator).")
    );
}

fn element_help(style: Style) {
    banner_line(style, "ELEMENTS — OPEN MATTER");
    row(
        style,
        "element aperture <amt> <prefix> to <world> deal <n>",
        "haul banked element under melt",
    );
    row(
        style,
        "element charter <world> <prefix> deal <n> [rail 1-4]",
        "element lane (default rail R2)",
    );
    row(style, "elements · prospect · assay", "list / open / draw veins");
    println!();
    println!(
        "  {}",
        style.hint("<prefix> matches the hex id prefix from `elements`.")
    );
}

fn market_help(style: Style) {
    banner_line(style, "MARKET — CARRIER BARTER");
    row(
        style,
        "offer <g_amt> <give> for <w_amt> <want> id <n>",
        "resting intent (no escrow)",
    );
    row(
        style,
        "trade <my_id> with <world> <their_id> deal <n>",
        "clear complementary offers",
    );
    row(
        style,
        "aperture <amt> <res> to <world> deal <n>",
        "one-way resource haul",
    );
    println!();
    println!(
        "  {}",
        style.hint("Price is want/give. STAMP/element cargo use stamp/element verbs (P1.8 widen later).")
    );
}

fn routes_help(style: Style) {
    banner_line(style, "ROUTES — LANES");
    row(
        style,
        "charter <world> deal <n>",
        "ROUTE-1 carrier lane (alloy cost, η from instruments)",
    );
    row(
        style,
        "element charter <world> <prefix> deal <n> [rail]",
        "element-specialized lane",
    );
    row(style, "routes", "list carrier + element lanes");
    row(style, "desk r", "ROUTES pane with alignment windows");
    println!();
    println!(
        "  {}",
        style.hint("Upkeep 1/4 ember per lane at year voice — unpaid lanes lapse.")
    );
    println!(
        "  {}",
        style.hint(
            "Alignment CHEAP/FAIR/DEAR: each orbit advances at its own period \
             (core grain × polity ÷ star magnetism) — not lockstep.",
        ),
    );
}

fn colony_help(style: Style) {
    banner_line(style, "COLONY LOOP");
    row(style, "found <name>", "genesis colony");
    row(style, "zone <tile> <ext|hab|smelt|yard|regen|…>", "build");
    row(style, "tick <n> · collect", "time + settle workspace");
    row(style, "star <class> · site <A> <r>", "commission star + site orbit");
    row(style, "rings / sky", "star field · core · orbit period/slot");
    row(style, "thermal · map · status", "readings + thermal balance");
    row(style, "estate survey · parcels · appraise …", "real estate ladder (35)");
    row(style, "note list|issue|redeem", "NOTE-1 STAMP debt (needs T6 Exchange)");
    row(style, "future list|mark|open|settle", "FUT-1 cargo×STAMP (Exchange MM)");
    row(style, "share list|issue|dividend|transfer", "SHARE-1 equity stream (Exchange MM)");
    row(style, "perp list|open|fund|close|liq", "PERP-1 last-clear index (Exchange MM)");
    row(style, "vault status|deposit|withdraw", "O6.5 T8 Vault custody + margin reserve");
    row(style, "challenge <element-prefix>", "O6.4 T7 Assay consilience / slash");
    row(style, "fleet mint hauler|escort [g] finance", "hull + optional yard Note");
    row(style, "defend status|escort|risk", "P5-A KIN convoy + posture (36)");
    row(style, "posture status|set", "P5-B doctrine reorient (anatomy sealed)");
    row(style, "veil status|draw|lift · ward status|elder|lift", "P5-C INF MASK/WARDEN");
    row(style, "market status|seal|resume|clear|raid|cascade", "P5-M/M2 MKT war");
    row(style, "medium status|reinforce · tamper", "P5-D ATM + dual-use WORLDSHAPE tax");
    row(style, "spy status|probe|exfil|sabotage", "P5-F espionage (intel stash, no free tech)");
    row(style, "rail status|promote", "P5-G R1→R2; R3/R4 feature-gated");
    row(style, "anchor export-stamp|import-strand|import-court", "X7 T9 Anchor (feature-gated)");
    row(style, "suite market|convoy|cyber|spy", "UI over deeds — same chronicle");
    row(style, "prospect · assay · compound · refine", "elements + alchemy");
    row(style, "bench · trade", "MAKE hub · CIRCULATE hub (primary workbenches)");
    row(style, "bench chem · iso · catalog", "chemistry science path");
    println!();
    println!(
        "  {}",
        style.hint("`site` places θ on the lattice — alignment uses it for D_orbit.")
    );
    println!(
        "  {}",
        style.hint(
            "Loop: extract → refine/compound → scrap → hulls → trade; entropy∥syntropy caps order-work (34)."
        )
    );
}

