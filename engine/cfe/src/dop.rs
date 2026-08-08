//! NS-1 DOP — dimensional orbs. The colony orb is the first consumer:
//! a self-contained game object whose committed state round-trips its
//! wire codec byte-exact.
//!
//! RULESET v1 (02_COLONY_LOOP §2 — LAW for replay; constants never
//! change silently under an existing world): founding reserves, zone
//! costs, yields, and upkeep live HERE, beside the types they rule.
//!
//! THE NETSTRATUM WIRE TAG REGISTRY (permanent — one tag, one meaning,
//! forever; retired codes are never reused):
//!   1  ClosureEvent::ColonyFounded
//!   2  ClosureEvent::ZoneBuilt
//!   3  ClosureEvent::YieldCollected
//!   4  ClosureEvent::UpkeepSettled
//!   5  ClosureEvent::FamineStruck
//!   6  ClosureEvent::FamineRelieved
//!   7  ClosureEvent::ZoneRazed
//!   8  ClosureEvent::WorldCharted (D3 — the terrain seed, the OPT-IN
//!         deed; 9 stays reserved for D3 milestones)
//!   10 ClosureEvent::EntityRegistered (N3)
//!   11 ClosureEvent::ScopeGranted (N3)
//!   12 ClosureEvent::ScopeRevoked (N3)
//!   13 ClosureEvent::ColonyCharted (P1)
//!   21 ClosureEvent::EraAnnexed (CONVERGE-1 — universe ledger only)
//!   22 ClosureEvent::OverheatStruck (THERMAL-1)
//!   23 ClosureEvent::OverheatCleared (THERMAL-1)
//!   24 ClosureEvent::StarCommissioned (THERMAL-1)
//!   25 ClosureEvent::PlanetSited (THERMAL-1)
//!   27 ClosureEvent::VentSettled (VENT-1)
//!   29 ClosureEvent::DemurrageSettled (SINK-1)
//!   30 ClosureEvent::ResourceTransferred (TRADE-1)
//!   31 ClosureEvent::OverflowAbsorbed (ABSORB-1)
//!   32 ClosureEvent::AbsorptionLapsed (ABSORB-1)
//!   33 ClosureEvent::BondSealed (BOND-1)
//!   34 ClosureEvent::BondSettled (BOND-1 — settlement AND severance:
//!      the severed flag + load ride the one settle receipt)
//!   35 ClosureEvent::ForwardSealed (FORWARD-1)
//!   36 ClosureEvent::ForwardReleased (FORWARD-1)
//!   37 ClosureEvent::ForwardSlashed (FORWARD-1)
//!   38 ClosureEvent::FrictionSettled (IDEO-1)
//!   16 Orb header (orb_id ‖ name)
//!   17 Orb zone entry (tile ‖ kind)
//!   18 Orb banked entry (resource ‖ ratio)
//!   19 Orb famine entry (resource)
//!   26 Orb thermal entry (albedo ‖ orbit_r ‖ overheat u8 ‖ θ?) —
//!      THERMAL-1; θ additive-trailing 2026-08-02, absent = unplaced

//!   20 Orb syntropy entry (syntropy_contained ratio) — THE INVERSE OF
//!         TAG 28. What the orb has CONTAINED across its whole life:
//!         every joule encased, whether the excess came from a star or
//!         from friction (both are the same currency, operator ruling
//!         2026-08-03). Emitted only when nonzero, so an orb that has
//!         contained nothing stays byte-identical to a pre-syntropy one.
//!   28 Orb vent entry (entropy_vented ratio ‖ LE64 last_touch) — VENT-1/SINK-1
//!   41 Orb cohesion entry (the eight books, IDEO-1) — emitted only
//!      when a book is nonzero; a zero-book orb encodes byte-identical
//!      to a pre-ideology one.
//!   39 ClosureEvent::TechDiscovered (TECH-1a — condition 0 the kernel
//!      solar leap; 1–5 the crystallized nodes, the TECH-1b slice)
//!   40 ClosureEvent::TechLicensed (PLANNER-1 — the issuing deed; the
//!      far world receipts TechDiscovered under the same deal; the
//!      solar leap is never licensed).
//!   42 ClosureEvent::GaugeShifted (AEON-1a — the key moves)
//!   43 ClosureEvent::Diverged (AEON-1a — the cleave, a peer branch)
//!   44 ClosureEvent::Confluent (AEON-1a — the recombination)
//!   45 ClosureEvent::StarAged (AEON-1a — the star moves in its region)
//!   46 ClosureEvent::EraFolded (AEON-1a — the fold; base G advances)
//!   47 ClosureEvent::ApertureSent (NS-4 APERTURE — the source leg)
//!   48 ClosureEvent::ApertureReceived (NS-4 APERTURE — the dest leg)
//!   49 ClosureEvent::OfferPosted (MARKET-1 — the resting intent)
//!   50 ClosureEvent::OfferCleared (MARKET-1 — the crossing filled it)
//!   51 RESERVED for the market board's third (BOARD-1 if it needs one)
//!   52 ClosureEvent::CouncilSealed (COUNCIL-1 — the multilateral pact)
//!   53 ClosureEvent::CouncilClosed (COUNCIL-1 — one member's reckoning)
//!   54 ClosureEvent::GalaxyObserved (GALAXY-1 — universe ledger only)
//!   55 ClosureEvent::CrystalDrawn (N4 / NS-2 §1.5 — the scanner's
//!         power draw, the ONE chronicle event of RQP: query, hit, and
//!         refusal never receipt — the lens law)
//!   56–58 NS-2 protocol records (query ‖ hit ‖ refusal — NS-2 §2,
//!         RQP's own framing, never world-chronicle events)
//!   59 ClosureEvent::LineSucceeded (NOVEL-1, 10 §3c — the succession
//!         deed: a successor line adopts a derelict world's chronicle
//!         as its own substrate; the living tree drops to the world's
//!         kernel, the record stands forever)
//!   60 ClosureEvent::GaugeShiftedAt (GAUGE-N, 11 §2.1 — the graded
//!         key: moves or opens plane `grade`; plane 0 is the landed
//!         scalar, tags 42–46 bind to it forever)
//!   61 ClosureEvent::GenesisSealed (G1, 04 §1/§5 — the three axes
//!         sealed as the world-chronicle receipt; the registry becomes
//!         a cache of this receipted truth, never ground truth)
//!   62 ClosureEvent::WantStruck (WANT-1 — the cognitive want: kind 1
//!      epistemological [board-side reading, reserved] · 2 resonance ·
//!      3 novelty)
//!   63 ClosureEvent::WantRelieved (WANT-1 — the mind fed)
//!   64 NS-5 mesh head (the announcement; MESH-1a, the mesh's own
//!         framing, never a world-chronicle event)
//!   65 RETIRED (Charter §11, 2026-07-31 — was the consilience
//!         receipt, a reconciling check between copies; retired codes
//!         are never reused)
//!   66 NS-5 mesh refusal (kinds 1 malformed · 2 forged · 5 usurped ·
//!         6 uncited stand — single-chain judgments; kinds 3–4
//!         drift/diverged RETIRED under §11 with tag 65)
//!   67 ClosureEvent::HandoffSealed (MESH-1b, NS-5 §7 — one writer
//!         per clock: the yielding hand seals, naming the successor;
//!         the taking hand's first append cites the seal; authority
//!         is a mesh WALK of the arbor, never a flag)
//!   68 ClosureEvent::PracticeAdopted (PRACTICE-1, 14 §4 — a line
//!         binds a receipted discipline; the derived creed never
//!         binds, the practice does)
//!   69 ClosureEvent::PracticeReleased (PRACTICE-1 — release is a
//!         REDISCOVERY: barred until new doctrine landed since the
//!         adoption; emits the condition-7 re-emergence node)
//!   70 ClosureEvent::ProspectStruck (ELEMENT-GRAMMAR-1, 14 §3 — the
//!         mint deed: identity read from what the genesis seed always
//!         held; properties hash-derived; the receipt carries them so
//!         replay reads, never re-rolls. Moved 68→70 at the live
//!         collision with PRACTICE-1: the later claimant moves.)
//!   71 ClosureEvent::AssayStruck (ELEMENT-GRAMMAR-1, 14 §3 — the
//!         extraction closure: richness × elapsed banked in ONE
//!         receipt, the bond-settle shape; the mint creates identity,
//!         the assay moves matter. WIDENED 2026-08-06, ECO-1 #40:
//!         a trailing ratio — the vein's richness AFTER the deed
//!         (wear and regeneration settle in the same receipt, the
//!         tag-101 additive pattern). The legacy 3-field form decodes
//!         with the field absent and replays with richness unmoved —
//!         sealed history exact.)
//!   72 Orb elements entry (element_id[32] ‖ ratio — banked element
//!         stores, element-id-ascending; SEAL A: old readers skip)
//!   74 ClosureEvent::ElderWarded (WARDEN-1a, 10 §3e — the reverse
//!         shield seats: the elder star refuses every aperture until
//!         its gate is solved)
//!   75 ClosureEvent::WardLifted (WARDEN-1a — the depth gate opened:
//!         the claiming line's recorded solves reached the ceiling)
//!   76 ClosureEvent::VeilDrawn (MASK-1a, 14 §4 — the lawful veil:
//!         clouds renders, never the chronicle)
//!   77 ClosureEvent::VeilLifted (MASK-1a — by deed, or STARVED when
//!         the standing joule toll goes unpaid; starved u8)
//!   78 ClosureEvent::InstrumentForged (MANU-1, 10 §3f — manufacturing:
//!         tech + understanding + industry into a held INSTRUMENT;
//!         grade = the crystallized condition; scan reach = base +
//!         grade, the global constant struck)
//!   79 ClosureEvent::RouteChartered (ROUTE-1, MARKET-2 sequence — a
//!         maintained lane suppresses the melt: φ_route = φ·(1−η);
//!         two-legged, the deal id pairs the books)
//!   80 ClosureEvent::RouteLapsed (ROUTE-1 — unpaid maintenance; the
//!         lane dies on THIS book, and a pair needs both sides live)
//!   81 ClosureEvent::IncursionStruck (INCURSION-1a, 07 §3.1 — the
//!         archived-sheet door: a sealed derelict wakes as an
//!         incursion line; universe ledger only, the archive never
//!         edited, the living never raided)
//!   82 ClosureEvent::VoidShare (MARKET-2 conform, ruled 2026-07-31 —
//!         the melt as a first-class receipt on the receiving book:
//!         sent = received + void share; every deal in the universe
//!         sums to zero under ONE rule; losses are receipted, never
//!         silent; the gate only sums, never recomputes φ·D.
//!         `orb_id` ADDED 2026-08-02 (SYN-6): the record named the
//!         DEAL and never the BEARER, so the melt was receipted and
//!         credited to NO ONE — visible in aggregate, unattributable
//!         to any book. Wire: LE64 orb ‖ LE64 deal ‖ resource ‖ ratio)
//!   83 The committed-state container (CODEC-1, §11 ladder #35 — the
//!         artifact digests bind: schema-versioned canonical bytes of
//!         the WHOLE committed state, engine/codec's monopoly; one
//!         state, one encoding; a schema bump is a kernel amendment,
//!         never a silent widening. Not a ClosureEvent — the arbor's
//!         framing when state rides a TLV stream)
//!   84 The arbor commitment record (SPINE-1, NS-5 §9 — one position
//!         on the one clock: coordinate {branch ‖ S ‖ G ‖ T} ‖ schema
//!         ‖ kernel version ‖ gated ‖ state digest ‖ parent ‖
//!         commit. The two maps derive from the append-only record
//!         stream; one value per address, always. Not a ClosureEvent —
//!         the mesh's own framing)
//!   85 ClosureEvent::TaxonSealed (KERNEL-TAXON, 15 §4 — class and
//!         clade sealed as identity: once per star, in-vocabulary or
//!         unlawful, no value moved. Coverage {containment, exactness}
//!         seals at coder-03's amendment; the vocabularies are 15 §2
//!         (class 1–7, 0 lawful-unclassed) and 15 §3 (clade 1–18,
//!         0 unrecorded), append-only by amendment)
//!   96 ClosureEvent::VocabularyMinted — VERSION NOTE, recorded at the
//!         d3-scope merge: this tag landed while KERNEL_VERSION read
//!         0.5.0 and its additive minor bump was MISSED (the lens
//!         lane's error, declared not absorbed). The 0.6.0 and 0.7.0
//!         bumps that followed on d3-scope (tags 97, 98) carried the
//!         symmetric version rule; the merge composes at 0.7.0, which
//!         SEALS all three additive deeds. No receipt was cut in the
//!         gap on any chronicle that reached main.
//!      (28_VOCABULARY §7, 2026-08-05 —
//!         a T0 progenitor adds an entry to a vocabulary: axis ‖ code ‖
//!         sigil ‖ word ‖ binding, where the binding is the entry's
//!         MECHANISM as exact rational factors over the closed channel
//!         set. The code is carried and VERIFIED against the fold, never
//!         chosen. Additive: a minor bump, never breaking)
//!  101 ClosureEvent::LineMinted (THE MINTED LINE, 2026-08-06 — the
//!         engine mints its peoples: seed alone rides the wire, and
//!         EVERYTHING derives from it through `line_synthesis` — name,
//!         species, motto, taxon, axes — one rule, one truth, nothing
//!         declarable to misdeclare. A minted line holds NO office:
//!         seats enter by tag 89's deed, never by a draw)
//!  100 ClosureEvent::ParcelConveyed (CONVEY, 2026-08-06 — the postal
//!         deed, the estate chain's last link: from ‖ to ‖ resource ‖
//!         amount ‖ postage ‖ survey ‖ seed ‖ covered. World-contained;
//!         both endpoints must be SURVEYED PARCELS; only the MONETARY
//!         portion of the sender's bank conveys — the free portion
//!         stays home; postage = postal_meter rate × amount, drawn from
//!         the sender and burned like upkeep, verified at the gate by
//!         the same rule)
//!   99 ClosureEvent::OfferCountered (THE COUNTER, 2026-08-06 — the
//!         third outcome of survey/appraise → refuse/accept/renegotiate:
//!         offer ‖ want_amount ‖ optional appraisal digest. Amends a
//!         RESTING offer's ask in committed state, so acceptance (the
//!         clear) honors the countered terms by construction. A STALE
//!         appraisal citation refuses at the emitter — staleness
//!         triggers renegotiation and the evidence must be re-taken;
//!         consent needs no such gate, so acceptance never re-checks)
//!   98 ClosureEvent::AppraisalStruck (THE APPRAISAL, 2026-08-06 — value
//!         derives from the survey at every scale: subject ‖ survey
//!         digest ‖ seed ‖ covered ‖ value. One rule dispatches on the
//!         subject (building = sealed cost, planetary = banks + buildings,
//!         estate = planets × the parcels' monetary break); the gate
//!         re-derives by calling the same rule. Staleness is decidable
//!         from the stamp and TRIGGERS RENEGOTIATION, never blocks
//!         acceptance — consent closes deals)
//!   97 ClosureEvent::EstateSurveyed (E5b THE ESTATE CHAIN, 2026-08-05 —
//!         the metered, witnessed observation of an estate: seed ‖ payer ‖
//!         covered (orb ‖ witness digest, one per orb surveyed) ‖ cost.
//!         Survey DERIVES from observation: parcels derive pure from the
//!         receipt's seed forever; the deed pays the meter — cost =
//!         survey_meter rate × volume — and carries one witness per orb,
//!         the digest of that orb's own founding receipt. An observation
//!         without witnesses cannot be built, a survey that cannot pay
//!         does not happen)
//!   86 ClosureEvent::DiscoveryMinted (U1 KERNEL-VERIFY,
//!         U_DIRECTIVES §2 — the PoUW mint: value created ONLY
//!         against verified work. orb ‖ resource ‖ amount ‖ work,
//!         where work is the OPAQUE proof payload — solver-verify's
//!         encoding monopoly (forge digest, difficulty, program,
//!         output, certificate); the receipt carries its proof whole,
//!         replay re-verifies forever. Gated by I4's bound arm:
//!         verify_work(work) = Ok(d) ∧ amount ≤ mint_bound(d);
//!         mint_bound is SEALED SHUT until Ruling Box 1)
//!   87 The arbor SEGMENT blob frame (SPINE-UPLINK, NS-5 §9 — a
//!         content-addressed geometry carried in a replication
//!         segment: state[32] ‖ state_bytes; the receiver re-hashes
//!         on arrival. Segment-only — NEVER stored in arbor.tlv).
//!   88 The arbor SEGMENT step frame (SPINE-UPLINK — the proof a
//!         JUDGED record's transition was lawful: commit[32] ‖
//!         actor[8] ‖ stamp[8] ‖ event_bytes; the receiver re-gates
//!         under its own kernel. Segment-only, carried only for
//!         gated records).
//!   89 ClosureEvent::StationSeated (THE VERTICAL AXIS, 2026-08-01 —
//!         the seat enters the chain. A station GATES AUTHORITY (the
//!         tier-0 verbs check it), and it lived only in
//!         `var/registry/<sigil>.json`, a file anyone with a shell can
//!         edit. Who holds which office is now a receipted deed and
//!         committed state; the registry becomes a CACHE from the
//!         receipt on — the G1 precedent, where GenesisSealed did
//!         exactly this for the axes. Station codes are the BAND-1
//!         vocabulary by index: 1 progenitor · 2 watcher · 3 observer
//!         · 4 sovereign.)
//!    102  ClosureEvent::DemurrageTouched (SINK-1, the atomic touch —
//!         the demurrage seam closed 2026-08-06)
//!    103  ClosureEvent::ParameterChartered (P2, the chartered
//!         parameters — the balance placeholders' mint path)
//!    104  ClosureEvent::GemCompounded (ELEMENT-CHEMISTRY, #37 — the
//!         almanac's first CONSUMER: banked elements bond into a gem
//!         by the lane law, the members spending their valence and the
//!         gem carrying away exactly the sum. The receipt names the
//!         members and the gem; every quantity re-derives from
//!         `almanac::recipe_of`)
//!    105  ClosureEvent::TreatySealed (#42 — the political spectrum:
//!         trade, defense, peace and war as one co-signed pairwise
//!         charter over the PAIR channels. The kind is the sign of what
//!         the terms do, and peace/war displace each other.)
//!    106  ClosureEvent::TreatyStruck (#42 — a pact ends: broken by the
//!         partner's contrary pact, or renounced. Expiry needs no
//!         receipt — a term runs out by the clock, like a bond's.)
//!    107  ClosureEvent::ElementRefined (WAVE-1 REFINE-1, 2026-08-07 —
//!         banked raw element spends into a refined-form identity.
//!         Rates re-derive from `refine_recipe`; the refined id from
//!         `refined_identity`. ZoneKind::ElementRefinery = 9 rides
//!         ZoneBuilt; the conversion itself needs this deed so SEAL B
//!         can replay element mass.)
//!    108  ClosureEvent::ElementApertureSent (WAVE-2 E-APT-1, 2026-08-07 —
//!         source leg of a cross-world ELEMENT haul: open matter leaves
//!         this book. Pair of 108/109; ROUTE-1 lanes suppress melt via
//!         the same pair_melt law as resource hauls. No demurrage.)
//!    109  ClosureEvent::ElementApertureReceived (WAVE-2 E-APT-1 —
//!         destination leg: open matter arrives. Delivered = sent ×
//!         (1−melt).)
//!    110  ClosureEvent::ElementRouteChartered (WAVE-2/3 E-ROUTE-1 —
//!         ROUTE-1 lane specialized to an ElementId cargo + rail altitude.)
//!    111  ClosureEvent::StampMinted (WAVE-4 STAMP-1 — crystal→STAMP
//!         convert at identity rate; source crystal spent receipted.)
//!    112  ClosureEvent::StampApertureSent (STAMP cross-world debit leg.)
//!    113  ClosureEvent::StampApertureReceived (STAMP cross-world credit.)
//!    114  ClosureEvent::ScrapSalvaged (FLEET-1 Phase-3 — iron→scrap
//!         identity convert at a ShipYard.)
//!    115  ClosureEvent::HullMinted (FLEET-1 — scrap spent, hull banks.)
//!    116  ClosureEvent::HullAssigned (FLEET-1 — hull ↔ route deal;
//!         deal 0 = idle pool.)
//!    117  ClosureEvent::HullScrapped (FLEET-1 — hull removed, scrap
//!         reverse half cost.)
//!    118  ClosureEvent::OrderWorked (34_MATTER_BALANCE — books both
//!         thermal arms after refine/compound/hull/salvage; no Resource
//!         move.)
//!    119  ClosureEvent::ElementSalvaged (34 — banked element → scrap
//!         at identity; yard required; recycling path.)
//!    120  ClosureEvent::NoteIssued (PHASE-4 NOTE-1 — STAMP debt instrument;
//!         collateral locked; ≠ BondSealed ideology stream.)
//!    121  ClosureEvent::NoteRedeemed (NOTE-1 — maturity pay or collateral seize.)
//!    122  ClosureEvent::FutureOpened (PHASE-4 FUT-1 — cargo×STAMP dated;
//!         margin both sides; mark = last clear only.)
//!    123  ClosureEvent::FutureSettled (FUT-1 — cash settle vs last clear.)
//!    124  ClosureEvent::LastClearMarked (PHASE-4 — receipted spot mark for
//!         instrument marks; Resource ‖ price_in_stamp.)
//!    125  ClosureEvent::ShareIssued (SHARE-1 — equity on production stream;
//!         zone class or resource stream; STAMP par optional.)
//!    126  ClosureEvent::ShareTransferred (SHARE-1 secondary transfer.)
//!    127  ClosureEvent::ShareDividend (SHARE-1 — exact fraction of yield window.)
//!    128  ClosureEvent::PerpOpened (PERP-1 — unending Future on last-clear index.)
//!    129  ClosureEvent::PerpFunded (PERP-1 — funding from premium vs last clear.)
//!    130  ClosureEvent::PerpClosed (PERP-1 — close or liquidation.)
//!    131  ClosureEvent::HullFinanced (P4.10 shipwright — Note paper with hull mint.)
//!    132  ClosureEvent::EscortOnDuty (P5-A KIN — escort hull assigned to
//!         corridor deal; activates convoy coverage.)
//!    133  ClosureEvent::InterdictionStruck (P5-A KIN — typed residual
//!         cargo loss on positive-melt haul without full escort cover.)
//!    134  ClosureEvent::ConvoyRiskMarked (P5-A KIN — optional receipted
//!         risk band reading for a deal; pure derive preferred.)
//!    135  ClosureEvent::MarketSealed (P5-M — circuit breaker; books freeze.)
//!    136  ClosureEvent::MarketResumed (P5-M — breaker lifts.)
//!    137  ClosureEvent::SpotClearRecorded (P5-M — honest clear depth for seal.)
//!    138  ClosureEvent::LiquidityRaidStruck (P5-M — typed economic attack residual.)
//!    139  ClosureEvent::ClearPoisonRefused (P5-M — mark without depth refused.)
//!    140  ClosureEvent::NoteCascadeStruck (P5-M2 — short-paper cascade pressure.)
//!    141  ClosureEvent::CovertTamperStruck (P5-D — ATM medium pressure up.)
//!    142  ClosureEvent::MediumReinforced (P5-D — ATM envelope defense spend.)
//!    143  ClosureEvent::TamperDetected (P5-D — unit pressure threshold unmask.)
//!    144  ClosureEvent::PostureReoriented (P5-B — doctrine/posture shift; genesis anatomy stays.)
//!    145  ClosureEvent::SpyProbed (P5-F — scout residual under INF/CYB.)
//!    146  ClosureEvent::TechExfiltrated (P5-F — hash copy; never free tech mint.)
//!    147  ClosureEvent::InstrumentSabotaged (P5-F — grade hit on held instrument.)
//!    148  ClosureEvent::RailPromoted (P5-G — element-route rail altitude lift.)
//!    149  ClosureEvent::StrandExported (X7 — STAMP/claim leaves to strand packet.)
//!    150  ClosureEvent::AnchorImported (X7 — Anchor credits STAMP on valid proof.)
//!    151  ClosureEvent::AnchorImportRefused (X7 — bad proof residual.)
//!    152  ClosureEvent::VaultDeposited (O6.5 — STAMP into Vault custody.)
//!    153  ClosureEvent::VaultWithdrawn (O6.5 — STAMP out of Vault custody.)
//!    154  ClosureEvent::AssayConsilience (O6.4 — Assay office re-proves vein.)
//!    155  ClosureEvent::AssaySlashed (O6.4 — false richness residual.)
//!    156  ClosureEvent::IsoUnionStruck (CHEM science — isomorphic gem union.)
//!    157  ClosureEvent::ChemCataloged (CHEM science — recipe sealed as known.)
//!    158  ClosureEvent::DarkPullStruck (CHEM science — dark-7 gravity residual.)
//!    159  ClosureEvent::ScienceSeated (LAB — paid science discipline unlock.)
//!    160  ClosureEvent::LabCreated (LAB — dual ± creation; may explode.)
//!    161  ClosureEvent::ScienceExploded (LAB — explosion residual slash.)
//!    162  ClosureEvent::RogueFactionStruck (LAB — synthetic matrix residual.)
//!    Phase-5 claim ledger: next free **163**. Seated science labs.
//!         Orb TLV: 73 STAMP, 74 scrap. ZoneKind::ShipYard = 10.
//!         (Historic free tags below top: 14, 15, 56, 57, 58, 64, 66,
//!         83, 84, 87; 9 and 51 reserved; 65 retired forever.)
//!   In-value practice codes (append-only): 1 the Austerity · 2 the Showpiece ·
//!   3 the Restraint · 4 the Hoard · 5 the Overdrive (deep-end: adoption
//!   gated on hardship ≥ 8 receipts). Placeholder names, operator
//!   renames anytime. Tech condition 7 = re-emergence (the released
//!   discipline's lesson; window-ease modifier, alignment-gated).
//!   In-value codes opened at N4 (they ride existing orb tags, no new
//!   TLV tag): Resource::Crystal = 7 (banked entry, tag 18);
//!   ZoneKind::Resonator = 7 (zone entry, tag 17).
//!   90 ClosureEvent::SovereignUnionSealed (D5 — THE SOVEREIGN
//!         UNION: the offices D(Ω) PROGENITOR and V(Ω) SOVEREIGN
//!         entered into union under a 25-year REVIEW HORIZON.
//!         Parties are OFFICES, never orb ids — seating an office as
//!         a colony reduces a global field to a local node. A bonded
//!         DYAD: neither party derives its legitimacy from the other
//!         (17 §3 — both are readings of Ω, neither defined through
//!         the other). Ruled 2026-08-02.
//!         NOT 0x40: tag 64 is the NS-5 mesh head, and a collision
//!         there would decode a mesh announcement as a union.)

use crate::books::{kind_of, BookAccount, BookKind, BookView};
use crate::literature::{Aspiration, WorkId};
use crate::ratio::{self, ratio, Ratio};
use crate::tlv;

/// **R1–R4 settlement altitude** (30_OFFICES_AND_RAILS). Receipted on
/// routes and instruments; never inferred from T-tier. Default **R1**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rail {
    Hearth = 1,
    Aperture = 2,
    Strand = 3,
    Court = 4,
}

impl Rail {
    pub fn from_wire(byte: u8) -> Option<Rail> {
        match byte {
            1 => Some(Rail::Hearth),
            2 => Some(Rail::Aperture),
            3 => Some(Rail::Strand),
            4 => Some(Rail::Court),
            _ => None,
        }
    }
    pub fn as_wire(self) -> u8 {
        self as u8
    }
    pub const fn hearth() -> Rail {
        Rail::Hearth
    }
    pub fn word(self) -> &'static str {
        match self {
            Rail::Hearth => "R1-hearth",
            Rail::Aperture => "R2-aperture",
            Rail::Strand => "R3-strand",
            Rail::Court => "R4-court",
        }
    }
}

/// THE TWO OFFICES (`EQUATIONS` D5, `18` §2c′) as a TYPE, so the office
/// can never be confused with the root ENTITY. `world::SOVEREIGN` is
/// `entity_id = 1` — the actor holding every grant under NS-6
/// containment — while `Office::Sovereign` is `V(Ω)`: what STANDS, not
/// who ACTS. The collision was undocumented until 2026-08-02; this type
/// is the disambiguation, so no reader has to infer which sovereign a
/// line means.
///
/// A BONDED DYAD, never a hierarchy: neither office derives its
/// legitimacy from the other (`17` §3 — both are readings of `Ω`,
/// neither defined through the other; D5 — both are PRIVILEGED).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Office {
    /// `D(Ω)` — the PROGENITOR. Makes the room; owes room that was
    /// EARNED (`PlaneUnearned` refuses otherwise).
    Progenitor = 0,
    /// `V(Ω)` — the SOVEREIGN. Holds the substance; owes work that was
    /// RECEIPTED (admission by `admits()`).
    Sovereign = 1,
}

impl Office {
    /// The wire byte, and the only lawful reading of one.
    pub fn from_wire(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::Progenitor),
            1 => Some(Self::Sovereign),
            _ => None,
        }
    }
}

/// **TWO DISTINCT ENDPOINTS, proven at construction** (29_CONSTRUCTION
/// C1). Every value-moving verb refused `from == to` separately and
/// anonymously — five arms of one shape. A `Between` cannot be a
/// self-loop; the door names the fault once and the arms die.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Between {
    from: u64,
    to: u64,
}

impl Between {
    pub fn new(from: u64, to: u64) -> Option<Between> {
        (from != to).then_some(Between { from, to })
    }
    pub fn from(&self) -> u64 {
        self.from
    }
    pub fn to(&self) -> u64 {
        self.to
    }
}

/// The literal door — panics on a self-loop literal, the `ratio(n, 0)`
/// class: a malformed literal is a build error in a runtime coat.
pub fn between(from: u64, to: u64) -> Between {
    Between::new(from, to).expect("a between literal must join two distinct endpoints")
}

/// **THREE OR MORE DISTINCT MEMBERS, proven at construction** (P1,
/// 29_CONSTRUCTION). A council is multilateral by definition — two
/// parties are a forward, and a duplicated member would be bound
/// twice under one lock. Both halves of `BadCouncil`'s membership
/// shape lived in one anonymous arm; an `Among` cannot be short and
/// cannot repeat, so the arm dies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Among {
    members: Vec<u64>,
}

impl Among {
    pub fn new(members: Vec<u64>) -> Option<Among> {
        let mut distinct = members.clone();
        distinct.sort_unstable();
        distinct.dedup();
        (distinct.len() >= 3 && distinct.len() == members.len())
            .then_some(Among { members })
    }
    pub fn members(&self) -> &[u64] {
        &self.members
    }
}

/// The literal door for memberships.
pub fn among(members: &[u64]) -> Among {
    Among::new(members.to_vec())
        .expect("an among literal must hold three or more distinct members")
}

/// **TWO DISTINCT RESOURCES, proven at construction** (P1). An offer
/// that gives what it wants trades nothing — `give == want` was
/// `BadOffer`'s shape half. An `Exchange` cannot be reflexive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exchange {
    give: Resource,
    want: Resource,
}

impl Exchange {
    pub fn new(give: Resource, want: Resource) -> Option<Exchange> {
        (give != want).then_some(Exchange { give, want })
    }
    pub fn give(&self) -> Resource {
        self.give
    }
    pub fn want(&self) -> Resource {
        self.want
    }
}

/// The literal door for exchanges.
pub fn exchange(give: Resource, want: Resource) -> Exchange {
    Exchange::new(give, want).expect("an exchange literal must trade two distinct resources")
}

/// **A NAME, proven non-empty at construction** (P1). The nameless
/// line and the nameless word were each policed by their own arm
/// (`LineUnnamed`; the emptiness half of `MintNameTaken`); a `Name`
/// cannot be empty, so the arms die and only the collision FACTS
/// remain.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(String);

impl Name {
    pub fn new(text: &str) -> Option<Name> {
        (!text.is_empty()).then(|| Name(text.to_string()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for Name {
    fn fmt(&self, out: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        out.write_str(&self.0)
    }
}

/// The literal door for names.
pub fn name(text: &str) -> Name {
    Name::new(text).expect("a name literal must not be empty")
}

/// **A DELIVERY WINDOW — at least one tick.** `window == 0` was a shape
/// arm; a zero-tick obligation is not an obligation.
pub type Window = core::num::NonZeroU64;

/// The literal door for windows.
pub fn window(ticks: u64) -> Window {
    Window::new(ticks).expect("a window literal must be at least one tick")
}

/// **THE STATION, AS A TYPE** (BAND-1's closed vocabulary; NOT mintable
/// by 28_VOCABULARY §10 — offices change by amendment alone, which is
/// exactly why an enum is honest here where the minted vocabularies get
/// folds). The u8 range check in `seat_station_as` dies: a `Station`
/// IS in the band by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Station {
    Progenitor = 1,
    Watcher = 2,
    Observer = 3,
    Sovereign = 4,
    /// T4 Caravan — trade routes (30_OFFICES_AND_RAILS).
    Caravan = 5,
    /// T5 Crucible — elemental refineries.
    Crucible = 6,
    /// T6 Exchange — STAMP market-making.
    Exchange = 7,
    /// T7 Assay — element identity court.
    Assay = 8,
    /// T8 Vault — custody and collateral.
    Vault = 9,
    /// T9 Anchor — R3/R4 external settlement.
    Anchor = 10,
}

impl Station {
    /// The wire byte, and the only lawful reading of one.
    pub fn from_wire(byte: u8) -> Option<Station> {
        match byte {
            1 => Some(Station::Progenitor),
            2 => Some(Station::Watcher),
            3 => Some(Station::Observer),
            4 => Some(Station::Sovereign),
            5 => Some(Station::Caravan),
            6 => Some(Station::Crucible),
            7 => Some(Station::Exchange),
            8 => Some(Station::Assay),
            9 => Some(Station::Vault),
            10 => Some(Station::Anchor),
            _ => None,
        }
    }
    pub fn as_wire(self) -> u8 {
        self as u8
    }
    /// Registry / CLI word (append-only closed vocabulary).
    pub fn word(self) -> &'static str {
        match self {
            Station::Progenitor => "progenitor",
            Station::Watcher => "watcher",
            Station::Observer => "observer",
            Station::Sovereign => "sovereign",
            Station::Caravan => "caravan",
            Station::Crucible => "crucible",
            Station::Exchange => "exchange",
            Station::Assay => "assay",
            Station::Vault => "vault",
            Station::Anchor => "anchor",
        }
    }
    pub fn from_word(word: &str) -> Option<Station> {
        match word {
            "progenitor" => Some(Station::Progenitor),
            "watcher" => Some(Station::Watcher),
            "observer" => Some(Station::Observer),
            "sovereign" => Some(Station::Sovereign),
            "caravan" => Some(Station::Caravan),
            "crucible" => Some(Station::Crucible),
            "exchange" => Some(Station::Exchange),
            "assay" => Some(Station::Assay),
            "vault" => Some(Station::Vault),
            "anchor" => Some(Station::Anchor),
            _ => None,
        }
    }
}

/// THE RULED REVIEW HORIZON — 25 years at 60 ticks a year (`11` §1).
///
/// **A REVIEW, NEVER AN EXPIRY** (operator ruling 2026-08-02). A
/// `BOND-1` term SETTLES and ENDS at `sealed + term`; were the union
/// deeded that way, D5's breach clause would fire at stamp 1500 — *D
/// stops and V suffocates; V stops and D freezes* — and the foundation
/// would lapse on a timer. The union STANDS past this mark. What falls
/// due is the operator's re-approval, and nothing else.
pub const UNION_REVIEW_HORIZON: u64 = 1500;

/// The resources of the first slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Resource {
    Iron = 1,
    Biomass = 2,
    /// D2 (02 §5): smelted from iron; the tier-2 structural currency.
    Alloy = 3,
    /// D2 (02 §5): the ultra-refined tier — dense, trade-worthy.
    Diamond = 4,
    /// THERMAL-1 (05 §1.2): the commodity joule — minted ONLY by
    /// syntropic encasement of excess heat, never by fiat (01 §6.1).
    Joule = 5,
    /// SINK-1 (05 §1.4): bankable heat-absorption capacity — captured
    /// from vault-band headroom by cryo-lattice arrays, never passive.
    Sink = 6,
    /// N4 (NS-2 §1.5, 02 §5.1 — the deferral paid): the resonance
    /// crystal, the scanner's power source. Produced by the resonator,
    /// consumed by the lens.
    Crystal = 7,
}

impl Resource {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Iron),
            2 => Some(Self::Biomass),
            3 => Some(Self::Alloy),
            4 => Some(Self::Diamond),
            5 => Some(Self::Joule),
            6 => Some(Self::Sink),
            7 => Some(Self::Crystal),
            _ => None,
        }
    }

    pub const ALL: [Resource; 7] = [
        Self::Iron,
        Self::Biomass,
        Self::Alloy,
        Self::Diamond,
        Self::Joule,
        Self::Sink,
        Self::Crystal,
    ];
}

/// TECH-1b (10 §3): the condition table — the crystallized node's
/// modifier kind and magnitude per condition. MECHANICS only: the
/// NAME derives from the node hash (`tech_node_name` — the 10 §2
/// law-4 word tables), so two lines crystallizing the same condition
/// carry DIFFERENT names. Naming is never stacked on rungs (operator
/// ruling 2026-07-30): a line's technology is its own, seeded from
/// its own chronicle root.
pub fn tech_node_of(condition: u8) -> Option<(u8, Ratio)> {
    match condition {
        1 => Some((1, ratio(3, 4))),
        2 => Some((2, ratio(5, 4))),
        3 => Some((3, ratio(5, 4))),
        4 => Some((4, ratio(5, 4))),
        5 => Some((5, ratio(1, 2))),
        _ => None,
    }
}

/// TECH-1b (10 §2, law 4): the node's NAME — a deterministic
/// word-table composition from the node hash. The condition fixes the
/// CONCEPT (what the technology does); the hash draws the VOICE (how
/// this line speaks of it). Flavor at the surface, hash as the
/// identity underneath — replay re-derives the identical name because
/// the receipt carries the identical node.
/// **THE CONDITION VOCABULARY** — the bare concept a condition names,
/// separated from the voice that speaks it (2026-08-05).
///
/// It lived inline inside [`tech_node_name`], which returns
/// `"{voice} {concept}"`, so the only way to ask what a condition WAS
/// meant parsing a composed string — and a caller that tried
/// `starts_with` got the voice instead of the concept. The vocabulary is
/// now askable directly, which is what lets `survey::node_kind` classify
/// by MEANING rather than by number.
///
/// `None` is the honest answer outside the namespace: an unnamed
/// condition is not a node.
pub fn condition_concept(condition: u8) -> Option<&'static str> {
    match condition {
        0 => Some("solar leap"),
        1 => Some("insulation"),
        2 => Some("discipline"),
        3 => Some("deepening"),
        4 => Some("doctrine"),
        5 => Some("stores"),
        // TERRA-1: the gifted crown of the base stack — never
        // crystallized locally, only licensed down from tier 0.
        6 => Some("terraforming"),
        // PRACTICE-1's lesson: it rides the condition field and is NOT a
        // sky node.
        7 => Some("re-emergence"),
        _ => None,
    }
}

pub fn tech_node_name(condition: u8, node: &[u8; 32]) -> String {
    let concept = condition_concept(condition)
        .filter(|_| condition != 0)
        .unwrap_or("working");
    const VOICES: [&str; 12] = [
        "resonant", "patient", "vaulted", "keening", "tidal", "ember",
        "hollow", "woven", "still", "spiral", "murmured", "adamant",
    ];
    let voice = VOICES[(node[0] as usize) % VOICES.len()];
    format!("{voice} {concept}")
}

/// Founding banks the bootstrap reserves (02 §2.1): 20 iron, 10 biomass.
pub fn founding_reserves() -> Vec<(Resource, Ratio)> {
    vec![
        (Resource::Iron, ratio(20, 1)),    // APPROVED-SEED: founding endowment
        (Resource::Biomass, ratio(10, 1)), // APPROVED-SEED: founding endowment
    ]
}

/// THERMAL-1 (05 §1.2, placeholder): joules one active encasement
/// shell can capture per tick.
pub fn encasement_throughput() -> Ratio {
    crate::parameter::Parameter::EncasementThroughput.seed()
}

/// THERMAL-1 (05 §1.2, placeholder): the orb's base radiator capacity.
pub fn radiator_base() -> Ratio {
    crate::parameter::Parameter::RadiatorBase.seed()
}

/// THERMAL-1 (05 §1.2, placeholder): the per-tick leak on ACCRUED
/// joules outside the cryo vault — the D3 spoilage shape.
pub fn joule_leak_per_tick() -> Ratio {
    crate::parameter::Parameter::JouleLeakPerTick.seed()
}

/// ECON-J (05 §1.2, placeholder): joules one active zone burns per
/// tick for heating on a sub-mesic sited orb.
pub fn heating_per_zone() -> Ratio {
    crate::parameter::Parameter::HeatingPerZone.seed()
}

/// SINK-1 (05 §1.4, placeholder): sink capacity one active cryo-lattice
/// array captures per tick from vault-band headroom.
pub fn sink_throughput() -> Ratio {
    crate::parameter::Parameter::SinkThroughput.seed()
}

/// P2 (`21_KERNEL_PARAMETERS` §2): the per-elapsed-tick decay rate on
/// un-vaulted BANKED joules/sink, settled by receipt at touch.
///
/// EPOCHED (`cfe::epoch`). EPOCH 0 is the flat `1/32` every existing
/// receipt was written under; EPOCH 1 reads standing supply, so the
/// sink stiffens as the economy accumulates instead of holding still
/// while issuance grows.
pub fn demurrage_per_tick(supply: &Ratio, at: u64) -> Ratio {
    match crate::epoch::epoch_at(at) {
        crate::epoch::Epoch::Genesis => sink_flat(),
        crate::epoch::Epoch::Continuous => sink_by_supply(supply),
    }
}

/// EPOCH 0 — the landed flat rate. **Never edit this function.** Every
/// receipt stamped before `epoch::CONTINUOUS_FROM` is gated by it.
fn sink_flat() -> Ratio {
    ratio(1, 32) // SEALED-RATE: epoch-0 law — editing rewrites history
}

/// EPOCH 1 — the sink reads standing supply.
///
/// ```text
/// excess = max(0, supply − SINK_REFERENCE)
/// rate   = min(1/4, 1/32 · (1 + excess/(SINK_REFERENCE + 1)))
/// ```
///
/// **Supply, not rank.** Rank lives behind `bridge::read_entangled` in
/// the `kernel` crate, above this one, so `cfe` structurally cannot
/// read it — and a proposer computing a different rate from the gate
/// refuses honest deeds at I5 (`21_KERNEL_PARAMETERS` §9). Supply is
/// computable here, so one value serves both gates.
///
/// **Bounded above by `1/4`** (#16 requires a bounded function; an
/// uncapped rate above `1` would confiscate a whole bank each tick),
/// and still FLAT in the holding — supply moves the RATE, never the
/// shape of the decay (the 2^1100 denominator lesson).
fn sink_by_supply(supply: &Ratio) -> Ratio {
    let reference = ratio(SINK_REFERENCE, 1);
    let excess = (supply.clone() - reference.clone()).max(ratio(0, 1));
    let rate = ratio(1, 32) * (ratio(1, 1) + excess / (reference + ratio(1, 1)));
    rate.min(ratio(1, 4))
}

/// P2's reference supply — the banked holding a world carries before
/// the sink begins to stiffen. Derived from the landed rate rather than
/// picked: `32` is exactly the amount that fully decays in one tick at
/// `1/32` (#16 — a parameter with a stated reason, not a magic number).
const SINK_REFERENCE: i64 = 32;

/// TRADE-1 (09 §1, placeholder): the joule-commensurable weight of a
/// resource for cohesion-book postings — the commensuration table.
/// Weight 1 for every resource in v1; tunes at balance time.
pub fn commensurate_weight(_resource: Resource) -> Ratio {
    ratio(1, 1)
}

/// ABSORB-1 (09 §2.4, placeholder): the reciprocity window in ticks —
/// a cross-orb draw must be compensated within this span or the
/// remainder lapses to the debtor's extraction book.
pub fn reciprocity_window() -> u64 {
    8
}

/// BOND-1 (09 §2.2, placeholder): the notice window in ticks — a bond
/// cut without it (manual severance, or an insolvent default) posts
/// `rate × this` to the cutter's severance book. A bond run to its
/// term posts nothing.
pub fn bond_notice_window() -> u64 {
    8
}

/// FORWARD-1 (09 §3.1, placeholder curve): collateral scales by the
/// obligor's fidelity books — quota × (breached + 1)/(fulfilled + 1),
/// clamped to [1/4, 4]. The empty book is not a credential (full
/// collateral until history exists); the ceiling is the guardrail-6
/// law made literal: a zero-fidelity orb pays the maximum and is
/// NEVER barred.
pub fn collateral_required(fulfilled: &Ratio, breached: &Ratio, quota: &Ratio) -> Ratio {
    let one = ratio(1, 1);
    let multiplier = (breached.clone() + one.clone()) / (fulfilled.clone() + one);
    // The clamp IS the fourfold band — one home (P2).
    let (low, high) = crate::vocabulary::fourfold_band();
    let clamped = multiplier.max(low).min(high);
    quota.clone() * clamped
}

/// IDEO-1 (09 §3.2, placeholder): the ideological friction rate φ —
/// `ΔS_ideo = φ × abs(reciprocity_A − reciprocity_B) × volume` per
/// cross-orb transfer. Friction prices DIVERGENCE, not vice: aligned
/// partners (any alignment) pay exactly zero.
pub fn friction_rate() -> Ratio {
    crate::parameter::Parameter::FrictionRate.seed()
}

/// N4 (NS-2 §1.5, placeholder): crystals one scan draws from the
/// probing orb's bank — the lens's power, spent on the looking, hit or
/// miss. Gated pure before anything moves.
/// MASK-1a (14 §4, placeholder): the veil's standing joule toll,
/// settled at every collect — an unpayable toll lifts the veil by
/// receipt, never silently.
pub fn veil_toll() -> Ratio {
    crate::parameter::Parameter::VeilToll.seed()
}

/// KERNEL-TAXON (15 §2): the ratified class-vocabulary ceiling —
/// codes 1..=7, 0 lawful-unclassed; raised only by amendment.
pub fn class_ceiling() -> u8 {
    7
}

/// KERNEL-TAXON (15 §3): the ratified clade-vocabulary ceiling —
/// codes 1..=20, 0 unrecorded; raised only by amendment.
///
/// **RAISED 18 → 20 (2026-08-05), paying a landed ruling the ceiling had
/// outgrown.** `15_TAXON` §3 seated two more clades on 2026-08-01 by
/// operator ruling — **19 draconic** (a sub-archetype inside class 4,
/// *one archetype, two expressions*) and **20 metashifting** (the Darlok
/// seat, class 3, FILLED) — and this constant was not raised with them.
/// `seal_taxon` refuses anything above the ceiling, so both ruled clades
/// were unsealable: the rulings existed on paper and the deed that
/// records them could not be done. Found when the compiled roster
/// (THE DEFINITION LAW) began carrying `draconic` and `metashifting` as
/// live clade words.
pub fn clade_ceiling() -> u8 {
    20
}

/// **THE ARCHETYPE SIGILS** (operator ruling 2026-08-05: *sigil is
/// allowed because it is essentially a rune or symbol with a definition,
/// and each archetype needs a sigil as well as the vocabulary*).
///
/// One glyph per class of `15_TAXON` §2, each chosen to state the
/// class's MECHANISM rather than to decorate it. A sigil here is exactly
/// what the ruling says it is: a symbol that carries a definition, in
/// the same sense as `cfe::query::Sigil` (the frame-invariant canonical
/// form) and the registry's line identifier. Three uses of one honest
/// word — a mark that stands for a defined thing.
///
/// Compiled, never filed: the class partition is a DEFINITION, and
/// definitions compile (`conductor::polities`, THE DEFINITION LAW).
pub fn archetype_sigil(class: u8) -> &'static str {
    match class {
        0 => "∅",  // unclassed — the empty class, lawful and occupied by nothing
        1 => "⬡",  // biotic-swarm — the cell that tiles: many bodies, one lattice
        2 => "◈",  // geodic-lithovore — the faceted solid, eaten and rebuilt
        3 => "◐",  // metashifter — the half-lit disc: form unfixed
        4 => "≋",  // telepathic-resonant — phases in agreement
        5 => "⊚",  // pelagic-aerial — a body free inside a medium
        6 => "∿",  // plasma-field — the field's own oscillation
        7 => "▲",  // apex-terrestrial — the solitary peak
        _ => "?",
    }
}

/// The class WORD for a code — the vocabulary the sigil marks.
///
/// **`etherial-field` is renamed `plasma-field` (2026-08-05).** "Ether"
/// names the luminiferous aether, a medium nineteenth-century physics
/// proposed and Michelson–Morley measured out of existence. It is not
/// mysticism, which is why it survived earlier sweeps — it is worse for
/// this document's purposes: DISCREDITED PHYSICS carried in the class
/// vocabulary of an engine whose Article III admits only observables.
/// The mechanism is a plasma: current sheets, magnetohydrodynamics, the
/// `plasmic` metabolism the axis vocabulary already carries.
pub fn archetype_word(class: u8) -> &'static str {
    match class {
        0 => "unclassed",
        1 => "biotic-swarm",
        2 => "geodic-lithovore",
        3 => "metashifter",
        4 => "telepathic-resonant",
        5 => "pelagic-aerial",
        6 => "plasma-field",
        7 => "apex-terrestrial",
        _ => "unknown",
    }
}

/// **THE LINE WORD-TABLES** — the compiled base vocabulary a minted
/// civilization draws from (the standing ruling: the engine mints its
/// peoples). Sized against the ratified taxon vocabulary so EVERY
/// class has words and every disposition × perception has a motto —
/// totality pinned, not assumed. The minted-vocabulary system (tag 96)
/// extends these by deed; this is the base it extends.
const LINE_VOICES: [&str; 12] = [
    "adamant", "pale", "keening", "vaulted", "ember", "tidal",
    "hollow", "woven", "still", "spiral", "murmured", "resonant",
];
const LINE_HEADS_MINERAL: [&str; 6] = ["ember", "slag", "basalt", "vein", "cinder", "forge"];
const LINE_HEADS_BIOTIC: [&str; 6] = ["bloom", "grove", "reef", "moss", "fern", "spore"];
const LINE_HEADS_PLASMIC: [&str; 6] = ["arc", "corona", "flux", "aurora", "filament", "sheet"];
const LINE_TAILS: [[&str; 4]; 4] = [
    ["march", "reach", "field", "span"],     // planar
    ["vault", "deep", "well", "hollow"],     // volumetric
    ["choir", "chord", "hum", "call"],       // harmonic
    ["veil", "shade", "gloam", "shroud"],    // umbral
];
const MOTTO_STEMS: [&str; 3] = [
    "the frontier is",          // expansionist
    "what we measure is",       // empiricist
    "what we savor is",         // epicurean
];
const MOTTO_ENDS: [[&str; 4]; 4] = [
    ["the next horizon", "the drawn map", "the open plain", "the far marker"], // planar
    ["the whole depth", "the held volume", "the deep room", "the hollow's answer"], // volumetric
    ["the standing chord", "the shared tone", "the long resonance", "the answered call"], // harmonic
    ["the unlit path", "the kept shadow", "the quiet between", "the veiled turn"], // umbral
];

/// One minted line, whole — everything the registry card needs, drawn
/// from a digest. **No station field, structurally**: a draw mints a
/// PEOPLE (the common, lawful state); office enters only by the
/// receipted seat deed (tag 89). A draw that manufactured office would
/// mint authority from a hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineDraw {
    pub name: String,
    pub species: String,
    pub motto: String,
    pub class: u8,
    pub clade: u8,
    pub disposition: u8,
    pub metabolism: u8,
    pub perception: u8,
}

/// **LINE SYNTHESIS** — a civilization drawn from a digest: pure,
/// total, deterministic (CHARTER §9 — the caller seeds from the
/// chronicle, never a clock). Lawful BY CONSTRUCTION: every axis is
/// drawn modulo its closed vocabulary and every taxon code modulo its
/// ratified ceiling, so there is nothing left to check after.
pub fn line_synthesis(digest: &[u8; 32]) -> LineDraw {
    // The sealed pre-amendment derivation: compiled ceilings. Old
    // LineMinted receipts (ceilings: None) re-derive through THIS,
    // forever.
    line_synthesis_at(digest, class_ceiling(), clade_ceiling())
}

/// **THE FOLD-AWARE DRAW** (operator ruling 2026-08-06): the same
/// synthesis, under the vocabulary ceilings the CALLER proves — the
/// fold's at the mint stamp, so a minted class or clade is drawable
/// the moment its receipt stands. Pure and total; a zero ceiling is
/// lifted to one so the modulus is defined (the seed vocabularies are
/// never empty).
///
/// RECORDED DEBT: a draw wearing a MINTED class carries the right CODE
/// but its species word renders through the compiled table's fallback
/// ("unknown") — resolving the fold's own word needs the chronicle at
/// the render, which is the reader's seam, not this pure function's.
pub fn line_synthesis_at(digest: &[u8; 32], class_ceiling: u8, clade_ceiling: u8) -> LineDraw {
    let disposition = 1 + digest[0] % 3;
    let metabolism = 1 + digest[1] % 3;
    let perception = 1 + digest[2] % 4;
    let class = 1 + digest[3] % class_ceiling.max(1);
    let clade = 1 + digest[4] % clade_ceiling.max(1);
    let heads: &[&str; 6] = match metabolism {
        1 => &LINE_HEADS_MINERAL,
        3 => &LINE_HEADS_PLASMIC,
        _ => &LINE_HEADS_BIOTIC,
    };
    let head = heads[digest[5] as usize % heads.len()];
    let tail = LINE_TAILS[perception as usize - 1][digest[6] as usize % 4];
    let voice = LINE_VOICES[digest[7] as usize % LINE_VOICES.len()];
    let mut name_chars = head.chars();
    let name = match name_chars.next() {
        Some(first) => format!("{}{}{}", first.to_uppercase(), name_chars.as_str(), tail),
        None => tail.to_string(),
    };
    LineDraw {
        name,
        species: format!("{voice} {}", archetype_word(class)),
        motto: format!(
            "{} {}",
            MOTTO_STEMS[disposition as usize - 1],
            MOTTO_ENDS[perception as usize - 1][digest[8] as usize % 4]
        ),
        class,
        clade,
        disposition,
        metabolism,
        perception,
    }
}

/// **THE POSTAL METER** (CONVEY, 2026-08-06): what carrying one unit
/// through the post costs — postage = rate × amount, exact, linear,
/// never compounding. Iron, like the survey's stakes: stamps are
/// struck metal. A placeholder dial ruled at balance, and the gate
/// verifies a conveyance's declared postage by calling THIS rule.
pub fn postal_meter() -> (Resource, Ratio) {
    (Resource::Iron, ratio(1, 16))
}

/// **THE SURVEY METER** (E5b, 2026-08-05: *surveys must be metered*).
/// The sealed charging rule: what one survey costs, PER ORB COVERED —
/// `cost = rate × volume`, exact, flat, never compounding (the 2^1100
/// lesson binds charging exactly as it binds decay). The gate verifies
/// a survey's declared cost by calling THIS rule (call-not-copy), so
/// the meter cannot be quoted one way and charged another.
///
/// Iron is the carrier — survey stakes driven into ground — and the
/// rate is a placeholder parameter ruled at balance time, like every
/// dial in this file.
pub fn survey_meter() -> (Resource, Ratio) {
    (Resource::Iron, ratio(1, 8))
}

/// One parcel of a surveyed orb — the deed's unit of distribution
/// (E5b: *parcel breaks open into free/monetary values*).
///
/// THEOREM (exact break): `free + monetary == 1` of the parcel's whole,
/// BY CONSTRUCTION — the two are drawn as `k/12` and `(12−k)/12` from
/// one byte of the derivation digest, so no residue is truncated and
/// no residue is invented. What a break leaves over is the carry the
/// interest law accrues on (the residual carry law), and it is exact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parcel {
    pub orb_id: u64,
    /// The free portion — held without denomination.
    pub free: Ratio,
    /// The monetary portion — the denominated value the postal
    /// machinery conveys.
    pub monetary: Ratio,
}

/// **PARCELS DERIVE; THEY ARE NEVER STORED** (survey is a derivation of
/// observation). Pure and total over the survey receipt's seed and the
/// orbs it covered: the same receipt yields the same parcels forever,
/// and an orb the survey did not cover has no parcel — deriving one
/// would be an unmetered survey.
pub fn parcels_of(seed: &[u8; 32], covered: &[u64]) -> Vec<Parcel> {
    covered
        .iter()
        .map(|orb_id| {
            let mut hasher = blake3::Hasher::new();
            hasher.update(b"netstratum.estate.parcel.v1");
            hasher.update(seed);
            hasher.update(&orb_id.to_le_bytes());
            let draw = hasher.finalize().as_bytes()[0] % 13;
            let free = ratio(i64::from(draw), 12);
            Parcel {
                orb_id: *orb_id,
                monetary: ratio(1, 1) - free.clone(),
                free,
            }
        })
        .collect()
}

/// **THE APPRAISABLE SUBJECT — one law, every scale** (operator ruling
/// 2026-08-06: *planetary → estate → building → dwelling*; extended
/// 2026-08-07: *orbital place + prospected stratum* — 35_ORBIT_DYNAMO).
/// A DWELLING is a `Building` whose zone is `Habitat`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Holding {
    /// The surveyed whole — valued through its parcels.
    Estate,
    /// One colony orb — its banks plus its buildings.
    Planetary { orb: u64 },
    /// One zone on one orb. `Habitat` here is the dwelling.
    Building { orb: u64, tile: u16 },
    /// **Sited place under the dynamo** — planetary capital plus site
    /// premium (core grain × star magnetism). Requires thermal site.
    Orbital { orb: u64 },
    /// **Prospected exploration claim** — stratum on an orb (tile field
    /// carries the stratum code). Requires a standing prospect.
    Stratum { orb: u64, stratum: u8 },
}

impl Holding {
    /// The wire form: scale ‖ orb ‖ tile (zeros where unused).
    pub fn wire(&self) -> [u8; 11] {
        let (scale, orb, tile) = match self {
            Holding::Estate => (1u8, 0u64, 0u16),
            Holding::Planetary { orb } => (2, *orb, 0),
            Holding::Building { orb, tile } => (3, *orb, *tile),
            Holding::Orbital { orb } => (4, *orb, 0),
            Holding::Stratum { orb, stratum } => (5, *orb, u16::from(*stratum)),
        };
        let mut out = [0u8; 11];
        out[0] = scale;
        out[1..9].copy_from_slice(&orb.to_le_bytes());
        out[9..11].copy_from_slice(&tile.to_le_bytes());
        out
    }
    pub fn from_wire(value: &[u8]) -> Option<Self> {
        let orb = u64::from_le_bytes(value.get(1..9)?.try_into().ok()?);
        let tile = u16::from_le_bytes(value.get(9..11)?.try_into().ok()?);
        match value.first()? {
            1 => Some(Holding::Estate),
            2 => Some(Holding::Planetary { orb }),
            3 => Some(Holding::Building { orb, tile }),
            4 => Some(Holding::Orbital { orb }),
            5 => Some(Holding::Stratum {
                orb,
                stratum: u8::try_from(tile).ok()?,
            }),
            _ => None,
        }
    }
}

/// Grain steps of an orbit radius on [`crate::almanac::ORBIT_GRAIN`] —
/// the core-magnetism grade (35 §1). Floor 1.
pub fn core_magnetism_of(orbit_r: &Ratio) -> u64 {
    use num_bigint::BigInt;
    use num_traits::ToPrimitive;
    let grid = BigInt::from(crate::almanac::ORBIT_GRAIN);
    let stepped = orbit_r.numer() * &grid / orbit_r.denom();
    let at_least = if stepped < BigInt::from(1) {
        BigInt::from(1)
    } else {
        stepped
    };
    at_least.to_u64().unwrap_or(u64::MAX).max(1)
}

/// Site premium for a sited place: `(core/grain) × star_magnetism × iron weight`.
pub fn orbital_site_premium(orbit_r: &Ratio, star_magnetism: u8) -> Ratio {
    let core = core_magnetism_of(orbit_r);
    let grain = ratio(crate::almanac::ORBIT_GRAIN, 1);
    let star_m = ratio(i64::from(star_magnetism.max(1)), 1);
    (ratio(core as i64, 1) / grain) * star_m * commensurate_weight(Resource::Iron)
}

/// Prospect richness row for pure appraisal (orb, stratum, richness).
pub type ProspectRow = (u64, u8, Ratio);

/// **THE VALUATION RULE — sealed, pure, one dispatch** (T2/T3 of the
/// appraisal). Value derives from surveyed content and nothing else:
///
///   Building   Σ its kind's sealed build cost × commensurate weight
///   Planetary  Σ banked × weight  +  Σ its buildings' values
///   Orbital    planetary + site premium (must be sited)
///   Stratum    prospect richness × iron weight (must be prospected)
///   Estate     Σ planetary(orb) × the parcel's MONETARY portion —
///              the appraisal is what DENOMINATES the parcel break
///
/// `None` off the estate: a zone that does not stand, an orb that does
/// not exist, an empty parcel set — absence is never a zero. The gate
/// re-derives an appraisal by calling THIS rule (call-not-copy), so a
/// value cannot be quoted one way and receipted another.
pub fn appraised_value(
    holding: &Holding,
    orbs: &[ColonyOrb],
    parcels: &[Parcel],
    star_magnetism: u8,
    prospects: &[ProspectRow],
) -> Option<Ratio> {
    let weight_of = |costs: Vec<(Resource, Ratio)>| {
        costs
            .into_iter()
            .fold(ratio(0, 1), |sum, (resource, amount)| {
                sum + amount * commensurate_weight(resource)
            })
    };
    match holding {
        Holding::Building { orb, tile } => {
            let held = orbs.iter().find(|o| o.orb_id == *orb)?;
            let zone = held.zones.iter().find(|z| z.tile == *tile)?;
            Some(weight_of(zone.kind.cost()))
        }
        Holding::Planetary { orb } => {
            let held = orbs.iter().find(|o| o.orb_id == *orb)?;
            let banked = held
                .banked
                .iter()
                .fold(ratio(0, 1), |sum, (resource, amount)| {
                    sum + amount.clone() * commensurate_weight(*resource)
                });
            let elements = held
                .elements
                .iter()
                .fold(ratio(0, 1), |sum, (_, amount)| {
                    sum + amount.clone() * commensurate_weight(Resource::Iron)
                });
            let built = held
                .zones
                .iter()
                .fold(ratio(0, 1), |sum, zone| sum + weight_of(zone.kind.cost()));
            Some(banked + elements + built)
        }
        Holding::Orbital { orb } => {
            let held = orbs.iter().find(|o| o.orb_id == *orb)?;
            let site = held.thermal.as_ref()?;
            let base = appraised_value(
                &Holding::Planetary { orb: *orb },
                orbs,
                &[],
                star_magnetism,
                prospects,
            )?;
            let premium = orbital_site_premium(site.orbit_r.value(), star_magnetism);
            Some(base + premium)
        }
        Holding::Stratum { orb, stratum } => {
            let richness = prospects
                .iter()
                .find(|(o, s, _)| *o == *orb && *s == *stratum)
                .map(|(_, _, r)| r.clone())?;
            Some(richness * commensurate_weight(Resource::Iron))
        }
        Holding::Estate => {
            if parcels.is_empty() {
                return None;
            }
            let mut total = ratio(0, 1);
            for parcel in parcels {
                // Estate book uses Orbital when sited (place + capital),
                // else planetary — exploration expansion premium on survey.
                let rung = if orbs
                    .iter()
                    .find(|o| o.orb_id == parcel.orb_id)
                    .is_some_and(|o| o.thermal.is_some())
                {
                    Holding::Orbital {
                        orb: parcel.orb_id,
                    }
                } else {
                    Holding::Planetary {
                        orb: parcel.orb_id,
                    }
                };
                let planetary = appraised_value(&rung, orbs, &[], star_magnetism, prospects)?;
                total += planetary * parcel.monetary.clone();
            }
            Some(total)
        }
    }
}

pub fn scan_draw() -> Ratio {
    ratio(1, 1)
}

/// G1 (04 §1): the axis vocabulary, closed and append-only (the
/// registry lists the words; these are their wire codes). Axis 1
/// disposition (1 expansionist ‖ 2 empiricist ‖ 3 epicurean); axis 2
/// metabolism (1 mineral ‖ 2 biotic ‖ 3 plasmic); axis 3 perception
/// (1 planar ‖ 2 volumetric ‖ 3 harmonic ‖ 4 umbral).
pub fn axis_valid(axis: u8, code: u8) -> bool {
    match axis {
        1 | 2 => (1..=3).contains(&code),
        3 => (1..=4).contains(&code),
        _ => false,
    }
}

/// G2 (04 §1.2): the ration law — the resource filling the FOOD slot
/// in every flow, keyed by the SEALED metabolism (biotic is the
/// ruleset-v1 default). Mineral eats iron; plasmic eats joules — and
/// the fiat law binds (05 §1.2, 01 §6.1): joules enter only by
/// encasement, so plasmic agriculture IS the thermal loop.
pub fn metabolic_ration(metabolism: u8) -> Resource {
    match metabolism {
        1 => Resource::Iron,
        3 => Resource::Joule,
        _ => Resource::Biomass,
    }
}

/// G2: the habitat's harvest under the sealed metabolism — what the
/// line farms. Biotic farms biomass (v1 verbatim); mineral farms
/// iron; plasmic farms NOTHING (rate 0 — its food is captured heat,
/// never a yield line).
pub fn metabolic_harvest(metabolism: u8) -> (Resource, Ratio) {
    match metabolism {
        1 => (Resource::Iron, ratio(1, 1)),
        3 => (Resource::Joule, ratio(0, 1)),
        _ => (Resource::Biomass, ratio(1, 1)),
    }
}

/// G2: one zone's yield under the sealed metabolism — the habitat's
/// biomass line is the FARM slot and reads the line's harvest; every
/// other yield is extraction or conversion and never substitutes.
pub fn zone_yield(kind: ZoneKind, metabolism: u8) -> (Resource, Ratio) {
    match (kind, kind.yield_per_tick()) {
        (ZoneKind::Habitat, (Resource::Biomass, _)) => metabolic_harvest(metabolism),
        // REGEN-1: the regenerator restores the line's OWN ration —
        // the harvest map's resource at the extractor's rate.
        (ZoneKind::Regenerator, (_, rate)) => {
            let (ration, _) = metabolic_harvest(metabolism);
            (ration, rate)
        }
        (_, held) => held,
    }
}

/// G2: one zone's upkeep under the sealed metabolism — every biomass
/// component is the FOOD slot and reads the line's ration; structural
/// components (iron maintenance, conversion inputs in other
/// resources) never substitute.
pub fn zone_upkeep(kind: ZoneKind, metabolism: u8) -> Vec<(Resource, Ratio)> {
    kind.upkeep_per_tick()
        .into_iter()
        .map(|(resource, amount)| match resource {
            Resource::Biomass => (metabolic_ration(metabolism), amount),
            held => (held, amount),
        })
        .collect()
}

/// G3 (04 §1.3, §4; NS-2 §4, placeholder table): the fraction of
/// structure surviving a cross-frame read — exact rationals, symmetric
/// in v1 (the loss prices the DISTANCE between frames). Same basis 1;
/// the 04 §4 example verbatim (harmonic through a planar eye: 3/4);
/// umbral reads by occlusion and keeps 1/2 of any other frame; basis
/// 0 (unsealed) carries no frame claim and reads whole. The loss is
/// SHOWN, never silent — and never keys a bucket: frames quotient out
/// of the sigil (the H¹ class IS what survives translation).
pub fn translation_survival(ours: u8, theirs: u8) -> Ratio {
    match (ours, theirs) {
        (0, _) | (_, 0) => ratio(1, 1),
        (a, b) if a == b => ratio(1, 1),
        (4, _) | (_, 4) => ratio(1, 2),
        _ => ratio(3, 4),
    }
}

/// ELEMENT-GRAMMAR-1 (14 §2): one minted element, as the deed returns
/// it — identity and the hash-derived properties the receipt carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prospect {
    pub element: [u8; 32],
    pub family: u8,
    pub tier: u8,
    pub richness: Ratio,
    pub name: String,
}

/// ELEMENT-GRAMMAR-1 (14 §4, placeholder weights): the abundance
/// arbor — integer weights per (StarClass, family); families 1 light ·
/// 2 heavy · 3 volatile · 4 noble. A zero weight NEVER yields (the
/// arbor law, measured). Values are placeholder parameters; the
/// operator tunes at balance.
fn spine_bands(class: crate::thermal::StarClass) -> [u32; 4] {
    use crate::thermal::StarClass as S;
    match class {
        S::O | S::B | S::A => [1, 2, 0, 1],
        S::F | S::G | S::K => [2, 1, 1, 1],
        S::M | S::BrownDwarf => [1, 0, 2, 1],
        S::WhiteDwarf | S::RedGiant | S::NeutronStar | S::WolfRayet | S::LuminalWhiteHole => {
            [0, 2, 1, 2]
        }
    }
}

/// The arbor weight as the exact rational the tables speak.
pub fn spine_weight(class: crate::thermal::StarClass, family: u8) -> Ratio {
    match (1..=4).contains(&family) {
        true => ratio(i64::from(spine_bands(class)[family as usize - 1]), 1),
        false => ratio(0, 1),
    }
}

/// ELEMENT-GRAMMAR-1 (14 §2): properties read from the identity hash
/// by exact modular arithmetic (CHARTER §9 — the draw is a reading of
/// the seed, never an oracle): family by arbor band, tier 1–3,
/// richness = the family's base over the tier.
pub fn element_properties(
    element: &[u8; 32],
    class: crate::thermal::StarClass,
) -> (u8, u8, Ratio) {
    let bands = spine_bands(class);
    let total: u32 = bands.iter().sum();
    let mut pick = u32::from(element[1]) % total.max(1);
    let mut family = 4u8;
    for (at, weight) in bands.iter().enumerate() {
        if pick < *weight {
            family = at as u8 + 1;
            break;
        }
        pick -= weight;
    }
    let tier = 1 + (element[2] % 3);
    let base: i64 = match family {
        1 => 3,
        2 => 2,
        3 => 1,
        _ => 1,
    };
    let richness = ratio(base, i64::from(tier) * 2);
    (family, tier, richness)
}

/// **ECO-1 (#40) — THE VEIN'S WEAR LAW, the one home.** The verb
/// settles it, the kernel re-derives it; both call HERE (the
/// `fourfold_band` discipline — one function, two readers).
///
/// The vein is a SPRING. Its natural flow is hash-derived
/// ([`element_properties`]) and never moves; its standing flow is
/// committed state that the draw tires and the regenerators restore.
/// Every quantity below already rides the receipt or prior committed
/// state, so the law holds NO new dial:
///
/// 1. WEAR — the draw priced against the spring's own nature:
///    `worn = standing × natural / (natural + drawn)`. A draw the size
///    of the natural flow halves the vein; a light draw barely marks
///    it; a hard draw guts it. Exact-rational throughout.
/// 2. RECOVERY — the Regenerator's second arm, THE SAME FORM RUN ON
///    THE GAP. Each regenerator standing on the orb returns matter at
///    its own working rate ([`ZoneKind::Regenerator`]'s yield rate —
///    what extraction takes, regeneration returns) across the settled
///    span, and that returned quantity closes the distance to natural
///    by exactly the fraction the drawn quantity opened it by:
///    `after = natural − (natural − worn) × natural / (natural + restored)`.
///
/// **The cap is a THEOREM, not a check.** Recovery shrinks the gap by
/// a factor in `(0, 1]`, so `worn ≤ after ≤ natural` for every input:
/// no regenerator can lift a vein past its nature, and no draw can
/// push one below zero. Both bounds hold by construction, which is
/// why neither is written as a clamp. A first cut ADDED the recovery
/// as a flat `rate × elapsed` and clamped at the top — that summed a
/// quantity into a flow, and made one regenerator saturate the cap in
/// a single tick, so the whole ecology collapsed to a binary.
///
/// `natural` is strictly positive for every drawn element (the family
/// bases are 1..=3 over tiers 1..=3), so the divisors never vanish.
pub fn vein_wear(
    standing: &Ratio,
    natural: &Ratio,
    drawn: &Ratio,
    regenerators: u64,
    elapsed: u64,
) -> Ratio {
    let worn = standing.clone() * natural.clone() / (natural.clone() + drawn.clone());
    let (_, rate) = ZoneKind::Regenerator.yield_per_tick();
    let restored = ratio(regenerators as i64, 1) * rate * ratio(elapsed as i64, 1);
    let gap = (natural.clone() - worn) * natural.clone() / (natural.clone() + restored);
    natural.clone() - gap
}

/// The element's name — the naming law (10 §2 law 4) applied to
/// matter: the family fixes the FORM, the hash draws the voice.
pub fn element_name(family: u8, element: &[u8; 32]) -> String {
    let form = match family {
        1 => "lode",
        2 => "marrow",
        3 => "rime",
        _ => "veil",
    };
    const VOICES: [&str; 12] = [
        "adamant", "pale", "keening", "vaulted", "ember", "tidal",
        "hollow", "woven", "still", "spiral", "murmured", "resonant",
    ];
    let voice = VOICES[(element[0] as usize) % VOICES.len()];
    format!("{voice} {form}")
}

/// **REFINE-1 — THE REFINED IDENTITY** (31_ELEMENT_COMMERCE §3).
/// Content-addressed from the raw element; the gate re-derives by
/// calling the same function (call-not-copy). One home for emitter
/// and admits.
pub fn refined_identity(raw: &[u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"netstratum.element.refine.v1");
    hasher.update(raw);
    *hasher.finalize().as_bytes()
}

/// **REFINE-1 — THE RECIPE** (14 §5 / 31 §3): spend and yield DERIVE
/// from the raw element's tier via the standing refinement law
/// (`refinement_input` / `refinement_yield`) — no new dial. The
/// refined identity is content-addressed. Gate re-derives both.
pub fn refine_recipe(
    raw: &[u8; 32],
    class: crate::thermal::StarClass,
) -> (Ratio, [u8; 32], Ratio) {
    let (_family, tier, _) = element_properties(raw, class);
    let spend = refinement_input(u32::from(tier));
    let yielded = refinement_yield(u32::from(tier));
    let refined = refined_identity(raw);
    (spend, refined, yielded)
}

/// G1 (04 §1.1, placeholder weights): the disposition's value
/// weighting over the three v1 counters — territory closures, verified
/// discoveries, consumed yield — exact rationals summing to 1. The
/// physics never differs; only which history counts as winning.
pub fn disposition_weights(disposition: u8) -> Option<[Ratio; 3]> {
    match disposition {
        1 => Some([ratio(3, 5), ratio(1, 5), ratio(1, 5)]),
        2 => Some([ratio(1, 5), ratio(3, 5), ratio(1, 5)]),
        3 => Some([ratio(1, 5), ratio(1, 5), ratio(3, 5)]),
        _ => None,
    }
}

/// IDEO-1 (09 §2.5): the derived axis reading — syntropic /
/// (syntropic + entropic), or None when no deed was ever posted. The
/// ratio DERIVES at read; it is never stored (the syntropy-ratio
/// precedent).
pub fn cohesion_ratio(syntropic: &Ratio, entropic: &Ratio) -> Option<Ratio> {
    let denominator = syntropic.clone() + entropic.clone();
    match denominator > ratio(0, 1) {
        true => Some(syntropic.clone() / denominator),
        false => None,
    }
}

/// **HOW MANY KINDS THE TABLE COMPILES (#41)** — the last compiled
/// wire code, and so the boundary a minted kind begins past. Counted
/// from the table by [`ZoneKind::from_wire`] rather than restated, so
/// landing a ninth compiled kind moves this by itself.
pub const ZONE_KINDS: u8 = {
    let mut code = 0u8;
    while ZoneKind::from_wire_const(code + 1).is_some() {
        code += 1;
    }
    code
};

/// **THE ZONE'S OWN WORD** — the naming law applied to buildings, and
/// the ONE home for it (#41). The zone axis's seed is built from this,
/// so a kind's word is vocabulary rather than decoration; the lens and
/// the CLI read it here instead of keeping copies, which is how the
/// three spellings that used to exist became one.
pub fn zone_word(kind: ZoneKind) -> &'static str {
    match kind {
        ZoneKind::Extractor => "extractor",
        ZoneKind::Habitat => "habitat",
        ZoneKind::Smelter => "smelter",
        ZoneKind::Synthesizer => "synthesizer",
        ZoneKind::Encasement => "encasement",
        ZoneKind::CryoArray => "cryo-array",
        ZoneKind::Resonator => "resonator",
        ZoneKind::Regenerator => "regenerator",
        ZoneKind::ElementRefinery => "element-refinery",
        ZoneKind::ShipYard => "shipyard",
        // #41: a minted kind's WORD lives in the fold, not the table.
        // The chronicle names it; this answers only that it is one.
        ZoneKind::Minted(_) => "minted",
    }
}

/// The zone kinds, with their ruleset-v1 economics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ZoneKind {
    Extractor = 1,
    Habitat = 2,
    /// D2 converter (02 §5.2): its upkeep is its INPUT, its yield its
    /// OUTPUT — conversion is a net flow like every other flow.
    Smelter = 3,
    Synthesizer = 4,
    /// THERMAL-1 (05 §1.2): the syntropic shell — captures excess heat
    /// into joules BEFORE anything vents. Its "yield" is computed by
    /// the heat law at tick time, never by `yield_per_tick`.
    Encasement = 5,
    /// SINK-1 (05 §1.4): the cryo-lattice array — captures vault-band
    /// radiator headroom as bankable sink. Heat-law yield, like the
    /// shell. The infrastructure gate; research/throttle gates deferred.
    CryoArray = 6,
    /// N4 converter (NS-2 §1.5, the 02 §5.2 pattern): refines diamond
    /// into resonance crystals. One synthesizer feeds one resonator on
    /// the knife edge — diamond nets exactly 0.
    Resonator = 7,
    /// **REGEN-1 — THE REGENERATOR** (operator directive 2026-08-06):
    /// ecological restoration as infrastructure. What extraction
    /// takes, regeneration returns: it yields the metabolism's RATION
    /// resource at the extractor's own rate — every number DERIVES
    /// from the standing table, no new dial. Placing one requires the
    /// DEEPENING crystallized on the orb (condition 3 — the lattice
    /// understood): regeneration is a technology, not a birthright.
    Regenerator = 8,
    /// **REFINE-1 — THE ELEMENT REFINERY** (31_ELEMENT_COMMERCE §3,
    /// Wave-1 2026-08-07): converts banked raw elements into refined
    /// forms via `refine_as` / tag 107. Resource yield is zero — the
    /// conversion is the element ledger. DEEPENING (condition 3)
    /// gates placement, same technology door as the regenerator.
    /// Q-T2: T5 Crucible is advantage only; T3 may refine with the zone.
    ElementRefinery = 9,
    /// **FLEET-1 — THE SHIP YARD** (33 Phase 3): converts scrap into
    /// hulls via `mint_hull`. Resource yield zero. Cost mirrors
    /// smelter iron bill (call-not-copy). Q-F4: yard required to mint.
    ShipYard = 10,
    /// **A MINTED BUILDING (#41)** — the fold code of a zone-axis entry
    /// past the compiled table. Its economics are its BASE's, modulated
    /// by its binding (`vocabulary::zone_base`), so they are read
    /// through the World (which holds the chronicle) and never from the
    /// compiled methods below.
    Minted(u8),
}

/// FLEET-1 hull role (universal physics; civ voice is separate).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum HullRole {
    Hauler = 1,
    Semi = 2,
    Runner = 3,
    Escort = 4,
}

impl HullRole {
    pub fn from_wire(byte: u8) -> Option<HullRole> {
        match byte {
            1 => Some(HullRole::Hauler),
            2 => Some(HullRole::Semi),
            3 => Some(HullRole::Runner),
            4 => Some(HullRole::Escort),
            _ => None,
        }
    }
    pub fn as_wire(self) -> u8 {
        self as u8
    }
    pub fn word(self) -> &'static str {
        match self {
            HullRole::Hauler => "hauler",
            HullRole::Semi => "semi",
            HullRole::Runner => "runner",
            HullRole::Escort => "escort",
        }
    }
}

/// Receipted hull asset (capacity/η/upkeep derive at read).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hull {
    pub hull_id: u64,
    pub role: u8,
    pub grade: u8,
    pub home_orb: u64,
    /// 0 = idle pool; else ROUTE-1 / E-ROUTE deal id.
    pub assigned_route: u64,
}

/// NOTE-1 — market STAMP debt (≠ ideology BondSealed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketNote {
    pub note_id: u64,
    pub orb_id: u64,
    pub issuer: u64,
    pub holder: u64,
    pub notional: Ratio,
    pub collateral: Ratio,
    pub maturity: u64,
    pub open: bool,
}

/// FUT-1 — dated cargo×STAMP; mark = last clear only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketFuture {
    pub fut_id: u64,
    pub orb_id: u64,
    pub long: u64,
    pub short: u64,
    pub resource: Resource,
    pub notional: Ratio,
    pub price: Ratio,
    pub margin: Ratio,
    pub expiry: u64,
    pub open: bool,
}

/// SHARE-1 — equity claim on a production stream (resource yield class).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketShare {
    pub share_id: u64,
    pub orb_id: u64,
    pub holder: u64,
    /// Stream resource (e.g. Iron from extractors).
    pub stream: Resource,
    /// Units of claim outstanding.
    pub units: Ratio,
    /// Par STAMP per unit at issue (exact).
    pub par: Ratio,
    pub open: bool,
}

/// PERP-1 — unending Future on a last-clear index (resource).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketPerp {
    pub perp_id: u64,
    pub orb_id: u64,
    pub long: u64,
    pub short: u64,
    pub resource: Resource,
    pub notional: Ratio,
    /// Entry mark (STAMP per unit).
    pub entry: Ratio,
    pub margin: Ratio,
    pub open: bool,
}

/// Capacity units a hull can haul (derive: role base × (1+grade)).
/// Bases are powers of two built from `1+1` — no free dials.
pub fn hull_capacity(role: u8, grade: u8) -> Ratio {
    let one = ratio(1, 1);
    let two = one.clone() + one.clone();
    let four = two.clone() + two.clone();
    let eight = four.clone() + four.clone();
    let base = match HullRole::from_wire(role).unwrap_or(HullRole::Hauler) {
        HullRole::Hauler => four,
        HullRole::Semi => eight,
        HullRole::Runner => one,
        HullRole::Escort => two,
    };
    base * ratio(1 + i64::from(grade.min(7)), 1)
}

/// Hull η in [0, 1) — grade × Resonator yield (1/8 ladder, landed).
pub fn hull_eta(role: u8, grade: u8) -> Ratio {
    let g = i64::from(grade.min(7));
    let boost = match HullRole::from_wire(role).unwrap_or(HullRole::Hauler) {
        HullRole::Runner => g + 1, // couriers grade one step better
        _ => g,
    };
    let eighth = ZoneKind::Resonator.yield_per_tick().1;
    ratio(boost.min(7), 1) * eighth
}

/// Scrap spent to mint (derive: capacity at role/grade).
pub fn hull_scrap_cost(role: HullRole, grade: u8) -> Ratio {
    hull_capacity(role.as_wire(), grade)
}
/// **THE REFINEMENT LAW** (2026-08-05, on the operator's ruling that
/// this codebase carries *too many constants — you aren't building
/// algorithms or theorems, you're building static content*).
///
/// The structural economy was a hand-tuned table. It is not: it is a
/// LADDER, and the ladder has a law. Every structural resource sits at a
/// REFINEMENT TIER — how many conversion steps it stands from raw — and
/// a converter at tier `t` consumes tier `t−1` and emits tier `t`:
///
/// ```text
///     input per tick   =  2^(t−1)          doubling with each step
///     yield per tick   =  4^(2−t)          quartering with each step
/// ```
///
/// **This REPRODUCES the landed table exactly** (pinned in
/// `refinement_pins`), so nothing is repriced by stating it — the
/// numbers were always this law, written out longhand. The Smelter's
/// `2 Iron → 1 Alloy` and the Synthesizer's `4 Alloy → 1/4 Diamond` are
/// the same two formulas at `t = 2` and `t = 3`.
///
/// **Why the two rates differ.** Input doubles and yield quarters, so
/// the cost per unit of refined output rises by EIGHT per step. That is
/// the whole reason a refined tier is worth more than what it is made
/// of, and it is now one number rather than six.
///
/// **AND IT NAMES ITS OWN BOUNDARY.** Only the structural chain
/// `Iron → Alloy → Diamond` obeys it. Joule, Sink and Crystal are OFF
/// the chain and have their own laws — the heat law (05 §1.2) mints
/// joules, the sink law banks cold, and Crystal is the SURVEY currency
/// rather than a refinement of anything. A law that claimed them would
/// be overreach, so it declines them by returning `None`.
pub fn refinement_tier(resource: Resource) -> Option<u32> {
    match resource {
        // Raw: taken from the world, refined from nothing.
        Resource::Iron | Resource::Biomass => Some(1),
        // Smelted from iron — the tier-2 structural currency.
        Resource::Alloy => Some(2),
        // The ultra-refined tier, from alloy.
        Resource::Diamond => Some(3),
        // OFF THE CHAIN, each with its own law.
        Resource::Joule | Resource::Sink | Resource::Crystal => None,
        // NO CATCH-ALL, deliberately. The match is exhaustive, so a new
        // resource breaks the build until someone RULES whether it
        // stands on the refinement chain — the `layer_of` discipline,
        // where adding a case fails compilation rather than defaulting
        // into the safest-looking answer.
    }
}

/// The input a tier-`t` conversion consumes: `2^(t−1)`.
pub fn refinement_input(tier: u32) -> Ratio {
    ratio(1i64 << tier.saturating_sub(1).min(30), 1)
}

/// The output a tier-`t` conversion emits: `4^(2−t)`.
pub fn refinement_yield(tier: u32) -> Ratio {
    match tier <= 2 {
        true => ratio(1i64 << (2 * (2 - tier.min(2))), 1),
        false => ratio(1, 1i64 << (2 * (tier - 2)).min(30)),
    }
}


impl ZoneKind {
    /// The wire reading, WHOLE: a compiled code names its kind, and
    /// anything past the table is a MINTED kind carrying its own fold
    /// code (#41). Validity of a minted code is the FOLD's business,
    /// exactly as a practice's is — the wire only says which entry is
    /// meant.
    pub fn from_wire(code: u8) -> Option<Self> {
        match Self::from_wire_const(code) {
            Some(compiled) => Some(compiled),
            None if code > ZONE_KINDS => Some(Self::Minted(code)),
            None => None,
        }
    }

    /// The code this kind rides as — one byte, compiled or minted.
    /// Replaces the `as u8` cast the compiled-only enum allowed.
    pub fn wire(self) -> u8 {
        match self {
            Self::Extractor => 1,
            Self::Habitat => 2,
            Self::Smelter => 3,
            Self::Synthesizer => 4,
            Self::Encasement => 5,
            Self::CryoArray => 6,
            Self::Resonator => 7,
            Self::Regenerator => 8,
            Self::ElementRefinery => 9,
            Self::ShipYard => 10,
            Self::Minted(code) => code,
        }
    }

    /// Is this a kind the table compiles, rather than one the chain
    /// minted?
    pub fn compiled(self) -> bool {
        !matches!(self, Self::Minted(_))
    }

    /// **THE COMPILED TABLE ONLY** — and it must stay that way:
    /// [`ZONE_KINDS`] counts the table by walking this until it
    /// answers `None`, so a version that admitted minted codes would
    /// never terminate. One match, two callers; the codes are never
    /// written down twice.
    pub const fn from_wire_const(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Extractor),
            2 => Some(Self::Habitat),
            3 => Some(Self::Smelter),
            4 => Some(Self::Synthesizer),
            5 => Some(Self::Encasement),
            6 => Some(Self::CryoArray),
            7 => Some(Self::Resonator),
            8 => Some(Self::Regenerator),
            9 => Some(Self::ElementRefinery),
            10 => Some(Self::ShipYard),
            _ => None,
        }
    }

    /// The build cost, spent from banked at build time (02 §2.2, §5.2).
    pub fn cost(&self) -> Vec<(Resource, Ratio)> {
        match self {
            Self::Extractor => vec![(Resource::Iron, ratio(10, 1))],
            Self::Habitat => vec![(Resource::Biomass, ratio(5, 1))],
            Self::Smelter => vec![
                (Resource::Iron, ratio(15, 1)),
                (Resource::Biomass, ratio(10, 1)),
            ],
            Self::Synthesizer => vec![
                (Resource::Alloy, ratio(20, 1)),
                (Resource::Biomass, ratio(15, 1)),
            ],
            Self::Encasement => vec![(Resource::Iron, ratio(8, 1))],
            Self::CryoArray => vec![(Resource::Iron, ratio(10, 1))],
            Self::Resonator => vec![
                (Resource::Alloy, ratio(10, 1)),
                (Resource::Diamond, ratio(5, 1)),
            ],
            // #41: THE COMPILED TABLE HAS NO ANSWER FOR A MINTED KIND.
            // Its economics are its base's, modulated by its binding,
            // and the base lives in the FOLD — so the authority is
            // `World::zone_economics`, which holds the chronicle. This
            // answers the ground state so the method stays total; a
            // caller reaching a minted kind here has asked the wrong
            // object, and the build gate refuses before that can pay.
            Self::Minted(_) => Vec::new(),
            // DERIVED: the shell's frame plus a double hearth — the
            // restoration engine is built from what the table already
            // prices.
            Self::Regenerator => {
                let mut cost = Self::Encasement.cost();
                // Two hearths' worth of the habitat's own line, summed
                // — the table's numbers, twice, no factor of its own.
                for _ in 0..2 {
                    for (resource, held) in Self::Habitat.cost() {
                        match cost.iter_mut().find(|(kind, _)| *kind == resource) {
                            Some((_, sum)) => *sum += held,
                            None => cost.push((resource, held)),
                        }
                    }
                }
                cost
            }
            // DERIVED: the habitat's bill. The refinery's feedstock is
            // elements (tag 72); pricing it on the smelter's iron line
            // raided the research→industry forge (smelter gate 30 iron)
            // and left mineral pure-NPC lines unable to smelt. Habitat
            // biomass is the founding ration every metabolism can meet
            // once, no new dial (31 §3).
            Self::ElementRefinery => Self::Habitat.cost(),
            // Yard: habitat biomass bill — leaves iron free for scrap
            // salvage (measured: smelter cost left founding iron too
            // thin to mint a hauler in the same era).
            Self::ShipYard => Self::Habitat.cost(),
        }
    }

    /// The exact yield rate — a ratio, never a float (RFA). The
    /// encasement shell's joules come from the heat law (05 §1.2), not
    /// from here: its listed rate is zero.
    pub fn yield_per_tick(&self) -> (Resource, Ratio) {
        match self {
            Self::Extractor => (Resource::Iron, ratio(3, 2)),
            Self::Habitat => (Resource::Biomass, ratio(1, 1)),
            // DERIVED (THE REFINEMENT LAW): `4^(2−t)`. These read
            // `ratio(1,1)` and `ratio(1,4)` until 2026-08-05, which was
            // the law written out longhand in two places.
            Self::Smelter => (
                Resource::Alloy,
                refinement_yield(refinement_tier(Resource::Alloy).unwrap_or(1)),
            ),
            Self::Synthesizer => (
                Resource::Diamond,
                refinement_yield(refinement_tier(Resource::Diamond).unwrap_or(1)),
            ),
            Self::Encasement => (Resource::Joule, ratio(0, 1)),
            Self::CryoArray => (Resource::Sink, ratio(0, 1)),
            Self::Resonator => (Resource::Crystal, ratio(1, 8)),
            // DERIVED: the ration resource (the habitat's own yield
            // map) at the EXTRACTOR's rate — what extraction takes,
            // regeneration returns.
            Self::Regenerator => {
                let (ration, _) = Self::Habitat.yield_per_tick();
                let (_, rate) = Self::Extractor.yield_per_tick();
                (ration, rate)
            }
            // Element conversion is the element ledger (`refine_as`),
            // not a Resource flow — zero Resource yield.
            Self::ElementRefinery => (Resource::Iron, ratio(0, 1)),
            Self::ShipYard => (Resource::Iron, ratio(0, 1)),
            // #41: the ground state — see `cost`. A minted kind yields
            // through `World::zone_economics`, never here.
            Self::Minted(_) => (Resource::Iron, ratio(0, 1)),
        }
    }

    /// The per-tick upkeep drains (02 §2.2, §5.2): rations, life
    /// support, and — for converters — the conversion inputs.
    pub fn upkeep_per_tick(&self) -> Vec<(Resource, Ratio)> {
        match self {
            Self::Extractor => vec![(Resource::Biomass, ratio(1, 2))],
            Self::Habitat => vec![(Resource::Iron, ratio(1, 4))],
            // DERIVED (THE REFINEMENT LAW): the structural input is
            // `2^(t−1)` of the tier below. The biomass line is the FOOD
            // slot and rides the ration law, which is a different one.
            Self::Smelter => vec![
                (
                    Resource::Iron,
                    refinement_input(refinement_tier(Resource::Alloy).unwrap_or(1)),
                ),
                (Resource::Biomass, ratio(1, 2)),
            ],
            Self::Synthesizer => vec![
                (
                    Resource::Alloy,
                    refinement_input(refinement_tier(Resource::Diamond).unwrap_or(1)),
                ),
                (Resource::Biomass, ratio(1, 1)),
            ],
            Self::Encasement => vec![(Resource::Iron, ratio(1, 4))],
            Self::CryoArray => vec![(Resource::Iron, ratio(1, 4))],
            Self::Resonator => vec![(Resource::Diamond, ratio(1, 4))],
            // DERIVED: the habitat's own life-support line.
            Self::Regenerator => Self::Habitat.upkeep_per_tick(),
            // DERIVED: the encasement's iron maintenance — elements are
            // the feedstock, so the food slot must not drain biomass.
            // Measured: biomass upkeep kept mineral lines forge-starved
            // forever (smelter needs 10 bio; a bio-upkeep refinery
            // spent the triangle-trade ration before industry could).
            Self::ElementRefinery => Self::Encasement.upkeep_per_tick(),
            Self::ShipYard => Self::Encasement.upkeep_per_tick(),
            // #41: the ground state — see `cost`.
            Self::Minted(_) => Vec::new(),
        }
    }

    /// THERMAL-1 (05 §1.2): each zone's per-tick heat contribution to
    /// the net-heat ledger (placeholder parameters — converters run
    /// hot, the shell itself adds nothing).
    pub fn factory_heat(&self) -> Ratio {
        match self {
            Self::Extractor | Self::Habitat | Self::Regenerator => ratio(1, 4),
            Self::Smelter
            | Self::Synthesizer
            | Self::Resonator
            | Self::ElementRefinery
            | Self::ShipYard => {
                ratio(1, 1)
            }
            Self::Encasement | Self::CryoArray => ratio(0, 1),
            // #41: the ground state — see `cost`.
            Self::Minted(_) => ratio(0, 1),
        }
    }

    /// Razing refunds half of EACH cost component (02 §2.4).
    pub fn raze_refund(&self) -> Vec<(Resource, Ratio)> {
        self.cost()
            .into_iter()
            .map(|(resource, cost)| {
                let two = ratio(1, 1) + ratio(1, 1);
                (resource, cost / two)
            })
            .collect()
    }
}

/// A zoned tile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zone {
    pub tile: u16,
    pub kind: ZoneKind,
    /// **#41: THE BASE A MINTED BUILDING WAS RAISED ON.** `None` for
    /// every compiled kind, which is its own base.
    ///
    /// Committed state, and it has to be: the raze refunds half of what
    /// the build spent, and the gate reads the PRIOR zone to price it.
    /// A minted kind's base lives in the FOLD, which `admits` cannot
    /// read — so the base is FIXED at build time, judged then, and read
    /// from committed state ever after. Wire: a trailing byte on the
    /// zone's own record (SEAL A — old readers stop at the kind).
    pub base: Option<ZoneKind>,
}

impl Zone {
    /// A compiled kind stands on itself.
    pub fn compiled(tile: u16, kind: ZoneKind) -> Zone {
        Zone { tile, kind, base: None }
    }
    /// **WHAT THIS ZONE COSTS AND REFUNDS** — the base's numbers where
    /// one was raised, its own where it is compiled. One home for both
    /// the emitter and the gate, so a build and its raze cannot price
    /// the same building differently.
    pub fn priced_as(&self) -> ZoneKind {
        self.base.unwrap_or(self.kind)
    }
}

/// The closures of the colony loop — completed transitions, the only
/// things that chronicle (NS-3 §2.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosureEvent {
    /// BOOK-1 (19_LENS §16) — a civilization declared a new kind of
    /// account. The kind is content-addressed over the normalized name,
    /// so re-declaring an existing book is not a second book.
    BookMinted { orb_id: u64, kind: BookKind, name: String },
    /// BOOK-1 — a DUAL entry: both sides of one account, together.
    /// There is no single-sided posting deed, by construction.
    BookPosted { orb_id: u64, kind: BookKind, kept: Ratio, lost: Ratio },
    /// BOOK-1 — an account closed to WRITING. It keeps every entry it
    /// took and stays readable forever (19_LENS §16c).
    BookArchived { orb_id: u64, kind: BookKind },
    /// LIT-1 — a literate line declares a work it reaches for. Proves
    /// nothing; costs nothing but the receipt.
    WorkAspired { orb_id: u64, work: WorkId, name: String },
    /// LIT-1 — the line CLAIMS the work and cites chain receipts for
    /// it. The witnesses are receipt digests; an unwitnessed claim
    /// cannot be made.
    WorkClaimed { orb_id: u64, work: WorkId, witnesses: Vec<[u8; 32]> },
    ColonyFounded { orb_id: u64, name: String },
    ZoneBuilt {
        orb_id: u64,
        tile: u16,
        kind: ZoneKind,
        /// #41: the compiled kind this build was PRICED against — the
        /// base a minted kind modulates. `None` for a compiled kind,
        /// which is its own base. The gate reads it because `admits`
        /// judges from committed state and cannot resolve the fold.
        base: Option<ZoneKind>,
    },
    YieldCollected { orb_id: u64, resource: Resource, amount: Ratio },
    /// Collect settled a negative net from banked (02 §2.3).
    UpkeepSettled { orb_id: u64, resource: Resource, amount: Ratio },
    /// banked+accrued fell below 0: the stores emptied (`drawn` = what
    /// banked held at the strike), consumers went dormant (02 §2.4).
    FamineStruck { orb_id: u64, resource: Resource, drawn: Ratio },
    /// The resource recovered to one-tick upkeep; consumers woke.
    FamineRelieved { orb_id: u64, resource: Resource },
    /// A zone demolished for half its cost back.
    ZoneRazed { orb_id: u64, tile: u16 },
    /// D3 (02 §2.5) — the star's terrain charted: the seed its tile
    /// classes derive from, pure and forever. The OPT-IN deed (the
    /// THERMAL-1 precedent): a star without this receipt reads all
    /// Standard; the seed is the FOUNDING receipt's digest
    /// (chronicle-seeded, CHARTER §9 — no wall clock, no dice).
    WorldCharted { seed: [u8; 32] },
    /// N3 — an entity joined this internet: principal 0 = a sovereign
    /// operator; nonzero = an agent contained by that principal.
    EntityRegistered { entity_id: u64, principal: u64, handle: String },
    /// N3 — a principal granted an agent one action scope.
    ScopeGranted { agent: u64, scope: crate::ipsal::Scope },
    /// N3 — a grant revoked; the agent's next such action refuses.
    ScopeRevoked { agent: u64, scope: crate::ipsal::Scope },
    /// P1 — a charted colony charted on a new planet: the founding
    /// reserves MOVE from the source orb's bank (conservation — a
    /// charter mints nothing).
    ColonyCharted { orb_id: u64, source_orb: u64, name: String },
    /// CONVERGE-1 (07 §1) — an archived world adopted onto the one
    /// timeline: its era, name, terminal digest, and epoch offset.
    /// Lives ONLY in the universe ledger (`convergence.tlv`), never in
    /// a world chronicle; world replay ignores it.
    EraAnnexed { era: String, world: String, terminal: [u8; 32], offset: u64 },
    /// GALAXY-1 (13_GALAXY §3–§4) — the observation deed: a crossing
    /// anchored past the frontier (wrapped-L1 to EVERY charted center
    /// exceeding its reach — a far region, never an outside) CREATES
    /// the galaxy it observes. Lives ONLY in the universe ledger
    /// (`convergence.tlv`), never in a world chronicle; world replay
    /// ignores it.
    GalaxyObserved {
        observer: String,
        creator: String,
        name: String,
        center: (Ratio, Ratio, Ratio),
        reach: Ratio,
    },
    /// THERMAL-1 (05 §1.2) — overflow exceeded encasement: the sector
    /// overheats; yields degrade by 1/(1+overflow) until cleared.
    /// World-driven (no entity forged it), the famine-strike shape.
    OverheatStruck { orb_id: u64, overflow: Ratio },
    /// THERMAL-1 — the overflow returned to zero; yields recover.
    OverheatCleared { orb_id: u64 },
    /// THERMAL-1 (NS-6 §7) — the container's sun commissioned with its
    /// StarClass wire code. Once per world; fixes every orbit's L.
    StarCommissioned { class: u8 },
    /// THERMAL-1 (05 §1.1) — a colony sited on its orbit: albedo A and
    /// orbit radius r, the orb's terms in the equilibrium baseline.
    ///
    /// Extended 2026-08-02 with `theta` (longitude) and `psi`
    /// (inclination), each a fraction of a TURN, `[0, 1) ⊂ ℚ`. Both are
    /// ADDITIVE-TRAILING on tag 25: the decoder reads three fields and a
    /// receipt without the rest yields `None`, so no rehash and no new
    /// tag. (`0x41` was proposed and is RETIRED under Charter §11 —
    /// `recurrence_guard` fails the build on reuse.)
    PlanetSited {
        orb_id: u64,
        albedo: Unit,
        orbit_r: Positive,
        theta: Option<Slot>,
        psi: Option<Slot>,
    },
    /// VENT-1 (05 §1.4) — accumulated vented entropy settled into the
    /// orb's cumulative ledger at collect (the yield pattern: one
    /// receipt, no per-tick bloat).
    VentSettled { orb_id: u64, amount: Ratio },
    /// SINK-1 (Q2) — receipted demurrage: un-vaulted banked stores
    /// decayed since last touch; settled here, never silently.
    ///
    /// SEALED (2026-08-06): new touches emit [`Self::DemurrageTouched`]
    /// — the per-row form judged each row against a prior the previous
    /// row had already anchored, so a dual-resource touch's second row
    /// always read elapsed 0 (the demurrage seam). Old receipts stay
    /// decodable and replayable forever.
    DemurrageSettled { orb_id: u64, resource: Resource, amount: Ratio },
    /// SINK-1 (tag 102, the demurrage seam closed 2026-08-06) — **ONE
    /// TOUCH, ONE RECEIPT**: every resource the touch settles rides
    /// together, because the TOUCH is the deed and the anchor moves
    /// once. The judge reads the whole settlement against the anchor
    /// that stood before it — the per-row order sensitivity that
    /// refused lawful second rows is unrepresentable here.
    DemurrageTouched { orb_id: u64, decays: Vec<(Resource, Ratio)> },
    /// **P2 (tag 103) — A BALANCE PLACEHOLDER CHARTERED**: the value
    /// the named parameter holds from this receipt on, for this world.
    /// The fold reads seed ∪ charters (`crate::parameter::of`); the
    /// band is proven at the door (`crate::parameter::Chartered` — the
    /// fourfold band of the seed, anchored so charters cannot walk);
    /// WHO may charter is the T0 standing gate, the mint's own law.
    ParameterChartered { key: u8, value: Ratio },
    /// **TREATIES (tag 105, #42) — THE POLITICAL SPECTRUM.** One
    /// world's leg of a co-signed pairwise charter: the kind is the
    /// politics (trade · defense · peace · war), the terms bind the
    /// PAIR channels, and `term` is how long it stands (0 = until
    /// broken, the bond's shape). Both books carry their own leg, the
    /// aperture's deal-id pattern — neither world owns the pact.
    TreatySealed {
        deal: u64,
        kind: u8,
        partner: String,
        terms: Vec<(u8, Ratio)>,
        term: u64,
    },
    /// **TREATIES (tag 106, #42)** — a standing pact ENDS. `broken`
    /// separates the two ways: a pact struck down by the partner's
    /// contrary pact (peace over war, war over peace) is BROKEN and
    /// posts against whoever did it; one renounced by its own holder
    /// is not.
    TreatyStruck { deal: u64, partner: String, broken: bool },
    /// **ELEMENT-CHEMISTRY (tag 104, #37) — MATTER BONDS.** The
    /// almanac's first consumer: a set of banked elements whose lanes
    /// saturate bonds into a GEM, each member spending its lane's
    /// valence and the gem carrying away exactly the sum.
    ///
    /// The receipt names only the ORB, the MEMBERS and the GEM.
    /// Everything else — what each member spends, what the gem yields,
    /// the knowledge won — re-derives from
    /// [`crate::almanac::recipe_of`] over the members alone, so the
    /// gate re-computes rather than trusts (call-not-copy). Member
    /// order is the offered order; the gem is order-free by its own
    /// law.
    GemCompounded { orb_id: u64, members: Vec<[u8; 32]>, gem: [u8; 32] },
    /// REFINE-1 (tag 107, 31_ELEMENT_COMMERCE §3) — banked raw element
    /// spends into a refined-form identity. Spend and yield re-derive
    /// from [`refine_recipe`]; the refined id from [`refined_identity`].
    /// One ElementRefinery on the orb is the technology gate (Q-T2:
    /// T3 may refine; T5 is advantage only, not a hard gate).
    ElementRefined {
        orb_id: u64,
        raw: [u8; 32],
        refined: [u8; 32],
        spent: Ratio,
        yielded: Ratio,
    },
    /// TRADE-1 (09 §2.1) — BANKED value moved orb-to-orb within one
    /// world: the chartering conservation law generalized (nothing
    /// minted, nothing lost). The cohesion-book postings derive from
    /// this receipt and the counterparty's receipted crisis state — a
    /// replay fact, never a judgment call.
    ResourceTransferred { from_orb: u64, to_orb: u64, resource: Resource, amount: Ratio },
    /// ABSORB-1 (05 §1.4, 09 §2.4) — un-sealed overflow routed into a
    /// partner's banked sink instead of the void: sink_consumed =
    /// heat_absorbed = vented_after_shed, the absorber's ledger
    /// carrying the shed. Opens a reciprocity window owed by the
    /// source to the absorber.
    OverflowAbsorbed { source_orb: u64, absorber_orb: u64, amount: Ratio },
    /// ABSORB-1 (09 §2.4) — a reciprocity window expired unpaid: the
    /// remainder posts to the debtor's extraction book. World-driven
    /// at the exact stamp (the famine-strike shape).
    AbsorptionLapsed { debtor_orb: u64, creditor_orb: u64, amount: Ratio },
    /// BOND-1 (09 §2.2) — a standing order sealed: source streams
    /// `rate` of `resource` per tick to target; `term` in ticks, 0 =
    /// open-ended. One bond per (source, target, resource).
    BondSealed { source_orb: u64, target_orb: u64, resource: Resource, rate: Ratio, term: u64 },
    /// BOND-1 — a bond's accumulated span settled by ONE receipt at
    /// the touch (collect / sever / term expiry). `severed` ends the
    /// bond; `load` is the severance posting (zero for an orderly
    /// end). Its own tag — a bond settle must stay distinguishable
    /// from a manual transfer, or replay could not re-anchor the bond.
    BondSettled {
        source_orb: u64,
        target_orb: u64,
        resource: Resource,
        amount: Ratio,
        severed: bool,
        load: Ratio,
    },
    /// FORWARD-1 (01 §7, 09 §2.3) — a vitrified delivery pact sealed:
    /// the obligor locks `collateral` (out of its bank, the vault by
    /// construction) against delivering ≥ `quota` of `resource` to the
    /// beneficiary within `window` ticks. The collateral rides the
    /// receipt so replay never re-derives the fidelity curve.
    ForwardSealed {
        obligor_orb: u64,
        beneficiary_orb: u64,
        resource: Resource,
        quota: Ratio,
        window: u64,
        collateral: Ratio,
    },
    /// FORWARD-1 — the quota was met: the collateral returns home and
    /// the stake posts `forwards_fulfilled`. World-driven at the tick
    /// boundary (fulfilment wins the deadline tie).
    ForwardReleased {
        obligor_orb: u64,
        beneficiary_orb: u64,
        resource: Resource,
        collateral: Ratio,
    },
    /// FORWARD-1 — the deadline passed unmet: BREACH_AND_SLASH — the
    /// collateral forfeits to the aggrieved and the stake posts
    /// `forwards_breached`.
    ForwardSlashed {
        obligor_orb: u64,
        beneficiary_orb: u64,
        resource: Resource,
        collateral: Ratio,
    },
    /// IDEO-1 (09 §3.2) — accrued ideological friction sealed into the
    /// mover's vented ledger at its collect (the VentSettled shape):
    /// real entropy, the handling cost of divergent boundary policies.
    FrictionSettled { orb_id: u64, amount: Ratio },
    /// WANT-1 (12_UNION §5b) — a COGNITIVE want strikes: a famine of
    /// the mind, derived from the chronicle and the SEALED identity
    /// (G1 — the entity type is the receipted axes, never the registry
    /// cache). Kinds: 1 epistemological (truth-starvation — reading
    /// reserved to the board/lens side), 2 resonance (isolation — a
    /// harmonic mind starved of links), 3 novelty (stagnation — an
    /// empiricist mind starved of signal; crystallization HALTS while
    /// struck, so relief arrives by licensed leap — the co-op pressure
    /// by design). `reading` is the measured drought, in ticks.
    WantStruck { orb_id: u64, kind: u8, reading: Ratio },
    /// WANT-1 — the want relieved: the mind fed (a new link, a fresh
    /// discovery). World-driven at the year voice, the famine shape.
    WantRelieved { orb_id: u64, kind: u8 },
    /// MESH-1b (NS-5 §7) — the receipted handoff: the yielding hand
    /// `from` seals, naming successor `to`. One writer per clock;
    /// authority derives by the mesh's walk of the arbor (never a
    /// flag, never consulted by any game rule — the world's own
    /// mechanics are blind to it).
    HandoffSealed { from: u64, to: u64 },
    /// MARKET-1 (BUILD_ORDER §3a) — an OFFER posts: this orb stands
    /// ready to GIVE `give_amount` of `give` for `want_amount` of
    /// `want`. The price IS the ratio (no absolute price exists —
    /// guardrail 6); the offer RESTS as committed state until a
    /// crossing clears it or it is never met. No escrow: affordability
    /// is gated at the fill, and a stale offer refuses then.
    OfferPosted {
        offer: u64,
        orb_id: u64,
        give: Resource,
        give_amount: Ratio,
        want: Resource,
        want_amount: Ratio,
    },
    /// MARKET-1 — the offer CLEARED: a crossing filled it. The fill's
    /// amounts live in the aperture legs the `deal` links (deal and
    /// deal+1 — the two directions); this receipt closes the resting
    /// intent. Replay removes the offer.
    OfferCleared { offer: u64, deal: u64 },
    /// GAUGE-N (11 §2.1, wired 2026-07-30) — the key at a GRADE: the
    /// fiber is a graded vector of wrapped keys, not one dial, and
    /// this deed moves (or OPENS — a shift at an unopened grade grows
    /// the key, the tonal dimension the growth law promises) one
    /// plane. Grade 0 is the landed scalar; tag 42 stays its own
    /// grade-0 shorthand forever (tags permanent).
    GaugeShiftedAt { grade: u16, gauge: Ratio },
    /// AEON-1a (11 §2.1) — the world's KEY moved: its gauge phase G,
    /// an exact rational wrapped on the torus fiber. Progression,
    /// elevation (12_UNION §6), and crystallized tech all move in G;
    /// replay takes the receipted value verbatim (already wrapped).
    GaugeShifted { gauge: Ratio },
    /// AEON-1a (11 §3) — DIVERGENCE: this line cleaved; a peer branch
    /// (no parent, no child) departs under `branch`, and THIS world's
    /// own key moves to `gauge` (the cleave IS a key change). The
    /// branch founds its own chronicle — annex, never merge.
    Diverged { branch: String, gauge: Ratio },
    /// AEON-1a (11 §3) — CONFLUENCE: this line combined with a bonded
    /// partner line; the key moves to `gauge` (the interference of
    /// both — the midpoint on the torus). Earned via a sustained bond,
    /// never free; the hybrid's own chronicle founds separately.
    Confluent { partner: String, gauge: Ratio },
    /// AEON-1a (11 §6) — the star MOVED in its region (mass ×
    /// composition × age): the class steps, luminosity recomputes,
    /// every bound orbit's T_eq⁴ shifts at the next tick. Movement in
    /// a region, never a rung on a track.
    StarAged { class: u8 },
    /// AEON-1a (11 §5) — the FOLD: the era sealed and rotated into the
    /// archive sequence; the receipt carries the era ordinal and the
    /// NEW base gauge — the next aeon seats on its own G-sheet, a
    /// parallel degree of the torus that cannot recur into this one.
    /// Nothing is destroyed; the fold is archival by construction.
    EraFolded { era: u64, gauge: Ratio },
    /// PLANNER-1 (12_UNION §2, 10 §2.2) — a technology LICENSED out of
    /// this world into another: the planner's issuing deed, the source
    /// leg of a two-world hand-off (the aperture shape — the far world
    /// receipts `TechDiscovered` under the same deal). Bound to an
    /// OPEN council in this world (`council` — delegation settles
    /// inside diplomacy, 12_UNION §7); the solar leap (condition 0) is
    /// NEVER licensed — the young meet their sun themselves.
    TechLicensed {
        deal: u64,
        from_orb: u64,
        council: u64,
        condition: u8,
        node: [u8; 32],
        name: String,
    },
    /// COUNCIL-1 (12_UNION §7) — a COUNCIL seals: a multilateral
    /// forward among three or more orbs, every member bound alike —
    /// each locks the same collateral against delivering ≥ quota to
    /// fellow members within the window. The group overtakes the
    /// individual: the reckoning gates all members together at the
    /// boundary, never one at a time.
    CouncilSealed {
        council: u64,
        resource: Resource,
        quota: Ratio,
        window: u64,
        collateral: Ratio,
        members: Vec<u64>,
    },
    /// COUNCIL-1 — one member's reckoning at the council boundary:
    /// `banked` is exactly what returned to this member's bank (its
    /// own collateral if faithful, plus its share of forfeits; a
    /// breacher amid faithful members banks zero; if EVERY member
    /// breached, each banks its collateral home — no aggrieved party,
    /// the pact dissolves, the books still post). Replay banks the
    /// carried amount verbatim; the integrity stake reads from the
    /// sealed council.
    CouncilClosed { council: u64, member: u64, fulfilled: bool, banked: Ratio },
    /// TECH-1a (10 §2.2) — a technology node enters the chronicle.
    /// `condition` 0 is the KERNEL solar leap (12_UNION §5a): a world's
    /// first thermal opt-in receipts that its people now know their
    /// sun — the opt-in IS the leap's receipt. Conditions 1–5 are the
    /// crystallized emergent nodes (10 §3, the TECH-1b slice); the
    /// node id is the seeded-synthesis hash (SEAL B — replay
    /// re-derives it), `modifier`/`magnitude` the applied parameter
    /// (0 ‖ 1/1 for the kernel leap: knowing is not yet a bonus).
    TechDiscovered {
        orb_id: u64,
        node: [u8; 32],
        condition: u8,
        modifier: u8,
        magnitude: Ratio,
        name: String,
    },
    /// NS-4 APERTURE (BUILD_ORDER §3a) — the SOURCE leg of a cross-world
    /// value hand-off: banked value LEFT this world's orb for another,
    /// bound to `deal` (the id the destination's `ApertureReceived`
    /// carries). Conservation is a property of the PAIR, not of this
    /// chronicle alone — replay of this world sees only the debit.
    ApertureSent { deal: u64, from_orb: u64, resource: Resource, amount: Ratio },
    /// NS-4 APERTURE (BUILD_ORDER §3a) — the DESTINATION leg: banked
    /// value ARRIVED in this world's orb from another, bound to the same
    /// `deal`. Replay of this world sees only the credit.
    ApertureReceived { deal: u64, to_orb: u64, resource: Resource, amount: Ratio },
    /// E-APT-1 (tag 108, WAVE-2) — source leg of a cross-world ELEMENT
    /// haul. Open matter (tag 72 ledger) leaves this orb; the pair's
    /// melt is applied on the receive leg. No resource demurrage.
    ElementApertureSent {
        deal: u64,
        from_orb: u64,
        element: [u8; 32],
        amount: Ratio,
    },
    /// E-APT-1 (tag 109) — destination leg: element mass arrives
    /// (already melt-reduced). Bound to the same `deal`.
    ElementApertureReceived {
        deal: u64,
        to_orb: u64,
        element: [u8; 32],
        amount: Ratio,
    },
    /// E-ROUTE-1 (tag 110) — a ROUTE-1 lane specialized to one element
    /// cargo identity and a settlement rail (1..=4). Alloy charter cost
    /// matches ROUTE-1; melt suppression still rides `pair_melt`.
    ElementRouteChartered {
        deal: u64,
        element: [u8; 32],
        eta: Unit,
        rail: u8,
    },
    /// STAMP-1 (tag 111) — mint STAMP against spent crystal (identity
    /// rate: stamp == crystal_spent). No free faucet.
    StampMinted {
        orb_id: u64,
        amount: Ratio,
        crystal_spent: Ratio,
    },
    /// STAMP aperture source leg (tag 112) — STAMP leaves this book.
    StampApertureSent {
        deal: u64,
        from_orb: u64,
        amount: Ratio,
    },
    /// STAMP aperture destination leg (tag 113) — STAMP arrives.
    StampApertureReceived {
        deal: u64,
        to_orb: u64,
        amount: Ratio,
    },
    /// FLEET-1 (tag 114) — iron salvaged into scrap at a ShipYard.
    ScrapSalvaged {
        orb_id: u64,
        amount: Ratio,
        iron_spent: Ratio,
    },
    /// FLEET-1 (tag 115) — hull minted; scrap spent.
    HullMinted {
        hull_id: u64,
        orb_id: u64,
        role: u8,
        grade: u8,
        scrap_spent: Ratio,
    },
    /// FLEET-1 (tag 116) — hull assigned to route deal (0 = idle).
    HullAssigned { hull_id: u64, deal: u64 },
    /// FLEET-1 (tag 117) — hull scrapped; scrap returned.
    HullScrapped {
        hull_id: u64,
        orb_id: u64,
        scrap_returned: Ratio,
    },
    /// MATTER-BALANCE-1 (tag 118) — order-work books both thermal arms.
    /// `kind`: 1 refine · 2 compound · 3 hull mint · 4 salvage.
    /// Neither arm is Resource; both replay onto orb books.
    OrderWorked {
        orb_id: u64,
        kind: u8,
        entropy: Ratio,
        syntropy: Ratio,
    },
    /// MATTER-BALANCE-1 (tag 119) — element → scrap at identity amount.
    ElementSalvaged {
        orb_id: u64,
        element: [u8; 32],
        amount: Ratio,
    },
    /// NOTE-1 (tag 120) — market Note (STAMP debt); collateral locked.
    NoteIssued {
        note_id: u64,
        orb_id: u64,
        issuer: u64,
        holder: u64,
        notional: Ratio,
        collateral: Ratio,
        maturity: u64,
    },
    /// NOTE-1 (tag 121) — redeem at maturity (pay notional) or seize collateral.
    NoteRedeemed {
        note_id: u64,
        orb_id: u64,
        paid: Ratio,
        seized: Ratio,
    },
    /// FUT-1 (tag 122) — dated cargo delivery for STAMP; margin both sides.
    FutureOpened {
        fut_id: u64,
        orb_id: u64,
        long: u64,
        short: u64,
        resource: Resource,
        notional: Ratio,
        price: Ratio,
        margin: Ratio,
        expiry: u64,
    },
    /// FUT-1 (tag 123) — cash settle vs receipted last-clear mark.
    FutureSettled {
        fut_id: u64,
        orb_id: u64,
        mark: Ratio,
        long_pnl: Ratio,
        short_pnl: Ratio,
    },
    /// PHASE-4 (tag 124) — last-clear spot mark (STAMP per unit cargo).
    LastClearMarked {
        orb_id: u64,
        resource: Resource,
        price: Ratio,
    },
    /// SHARE-1 (tag 125) — equity issue on a production stream.
    ShareIssued {
        share_id: u64,
        orb_id: u64,
        holder: u64,
        stream: Resource,
        units: Ratio,
        par: Ratio,
    },
    /// SHARE-1 (tag 126) — transfer units to a new holder.
    ShareTransferred {
        share_id: u64,
        from: u64,
        to: u64,
        units: Ratio,
    },
    /// SHARE-1 (tag 127) — dividend from receipted yield window.
    ShareDividend {
        share_id: u64,
        orb_id: u64,
        stream: Resource,
        resource_cut: Ratio,
        amount: Ratio,
    },
    /// PERP-1 (tag 128) — open perpetual on last-clear index.
    PerpOpened {
        perp_id: u64,
        orb_id: u64,
        long: u64,
        short: u64,
        resource: Resource,
        notional: Ratio,
        entry: Ratio,
        margin: Ratio,
    },
    /// PERP-1 (tag 129) — funding payment (premium vs last clear).
    PerpFunded {
        perp_id: u64,
        orb_id: u64,
        funding: Ratio,
        mark: Ratio,
    },
    /// PERP-1 (tag 130) — close or liquidate.
    PerpClosed {
        perp_id: u64,
        orb_id: u64,
        mark: Ratio,
        long_pnl: Ratio,
        liquidated: bool,
    },
    /// P4.10 (tag 131) — shipwright finance: Note issued with hull mint.
    HullFinanced {
        hull_id: u64,
        note_id: u64,
        orb_id: u64,
        scrap_spent: Ratio,
        note_face: Ratio,
    },
    /// P5-A KIN (tag 132) — escort hull on corridor duty.
    EscortOnDuty {
        hull_id: u64,
        deal: u64,
        grade: u8,
    },
    /// P5-A KIN (tag 133) — typed interdiction residual (cargo lost to risk).
    InterdictionStruck {
        deal: u64,
        orb_id: u64,
        resource: Resource,
        amount: Ratio,
        risk: Ratio,
    },
    /// P5-A KIN (tag 134) — receipted convoy risk band for a deal (optional).
    ConvoyRiskMarked {
        deal: u64,
        risk: Ratio,
        escort_cover: Ratio,
    },
    /// P5-M MKT (tag 135) — circuit breaker; instrument books freeze until resume_at.
    MarketSealed { resume_at: u64 },
    /// P5-M MKT (tag 136) — breaker lifted.
    MarketResumed {},
    /// P5-M MKT (tag 137) — honest spot clear depth for last-clear seal.
    SpotClearRecorded {
        orb_id: u64,
        resource: Resource,
        price: Ratio,
        depth: u64,
    },
    /// P5-M MKT (tag 138) — liquidity raid residual (fee burned; optional offer void).
    LiquidityRaidStruck {
        orb_id: u64,
        fee: Ratio,
        offers_voided: u8,
    },
    /// P5-M MKT (tag 139) — attempted mark without clear depth (refused audit).
    ClearPoisonRefused {
        orb_id: u64,
        resource: Resource,
        price: Ratio,
        depth: u64,
        need: u64,
    },
    /// P5-M2 (tag 140) — note cascade: fee burned; cascade pressure rises.
    NoteCascadeStruck {
        orb_id: u64,
        fee: Ratio,
        pressure: u64,
    },
    /// P5-D ATM (tag 141) — covert medium tamper; pressure rises by eighth.
    CovertTamperStruck {
        orb_id: u64,
        cost: Ratio,
        pressure: Ratio,
    },
    /// P5-D / P5.19b ATM (tag 142) — medium envelope reinforce; pressure falls.
    /// `resource` is the spend arm (species-coloured: iron/biomass/crystal).
    MediumReinforced {
        orb_id: u64,
        cost: Ratio,
        pressure: Ratio,
        resource: Resource,
    },
    /// P5-D ATM (tag 143) — unit pressure threshold crossed (unmask residual).
    TamperDetected {
        orb_id: u64,
        pressure: Ratio,
    },
    /// P5-B (tag 144) — in-game defense doctrine reorientation.
    /// Genesis metabolism (anatomy) is immutable; disposition/perception
    /// posture weights and primary domain emphasis may shift under receipt.
    /// Codes: disposition/perception use axis vocabulary; primary 0 = clear
    /// override, 1..=7 = CEL..MKT domain.
    PostureReoriented {
        disposition: u8,
        perception: u8,
        primary: u8,
    },
    /// P5-F (tag 145) — scout probe residual (crystal fee; detection gated).
    SpyProbed {
        orb_id: u64,
        depth: u8,
        fee: Ratio,
    },
    /// P5-F (tag 146) — tech hash exfil into intel stash (not techs ledger).
    TechExfiltrated {
        condition: u8,
        tech_hash: [u8; 32],
        fee: Ratio,
    },
    /// P5-F (tag 147) — sabotage: remove one instrument grade (typed residual).
    InstrumentSabotaged {
        orb_id: u64,
        grade: u8,
        tech_hash: [u8; 32],
    },
    /// P5-G (tag 148) — promote element route rail R1→R2 (R3/R4 feature-gated).
    RailPromoted {
        deal: u64,
        from_rail: u8,
        to_rail: u8,
    },
    /// X7 (tag 149) — STAMP (or instrument claim) exported to strand wire.
    StrandExported {
        orb_id: u64,
        amount: Ratio,
        claim_hash: [u8; 32],
        kind: u8,
    },
    /// X7 (tag 150) — Anchor credits STAMP after valid strand/court proof.
    AnchorImported {
        orb_id: u64,
        amount: Ratio,
        claim_hash: [u8; 32],
        kind: u8,
    },
    /// X7 (tag 151) — Anchor refused a proof (typed residual).
    AnchorImportRefused { orb_id: u64, reason: u8 },
    /// O6.5 (tag 152) — STAMP deposited into Vault custody.
    VaultDeposited { orb_id: u64, amount: Ratio },
    /// O6.5 (tag 153) — STAMP withdrawn from Vault (not margin-held).
    VaultWithdrawn { orb_id: u64, amount: Ratio },
    /// O6.4 (tag 154) — Assay office consilience re-prove of a vein.
    AssayConsilience {
        orb_id: u64,
        element: [u8; 32],
        matched: bool,
    },
    /// O6.4 (tag 155) — false assay residual: richness corrected + crystal slash.
    AssaySlashed {
        orb_id: u64,
        element: [u8; 32],
        slash: Ratio,
        richness_after: Ratio,
    },
    /// CHEM science (tag 156) — isomorphic union of two banked gems.
    IsoUnionStruck {
        orb_id: u64,
        a: [u8; 32],
        b: [u8; 32],
        product: [u8; 32],
        knowledge: u16,
    },
    /// CHEM science (tag 157) — recipe/gem sealed into the chemistry catalog.
    ChemCataloged {
        orb_id: u64,
        gem: [u8; 32],
        knowledge: u16,
        survival: Ratio,
    },
    /// CHEM science (tag 158) — dark-7 gravity residual on a bond/union.
    DarkPullStruck {
        orb_id: u64,
        pull: u8,
        weight: Ratio,
        dark_score: Ratio,
    },
    /// LAB (tag 159) — science discipline seated / elevated (paid crystal).
    ScienceSeated {
        orb_id: u64,
        discipline: u8,
        rank: u8,
        fee: Ratio,
    },
    /// LAB (tag 160) — lab creation with dual ± outcomes; may explode.
    LabCreated {
        orb_id: u64,
        path: u8,
        risk: Ratio,
        exploded: bool,
        explosion_kind: u8,
        hidden_pos: bool,
        hidden_neg: bool,
        fee: Ratio,
    },
    /// LAB (tag 161) — explosion residual (slash resource).
    ScienceExploded {
        orb_id: u64,
        path: u8,
        kind: u8,
        resource: u8,
        slash: Ratio,
    },
    /// LAB (tag 162) — synthetic rogue faction pressure (matrix residual).
    RogueFactionStruck {
        orb_id: u64,
        pressure: u8,
    },
    /// N4 (NS-2 §1.5) — the scanner's power draw, the ONE chronicle
    /// event of RQP: crystals left the probing orb's bank to hold the
    /// lens open. Deliberately blind to what was asked and what was
    /// seen — query, hit, and refusal never receipt (the lens law).
    CrystalDrawn { orb_id: u64, amount: Ratio },
    /// NOVEL-1 (10 §3c) — the succession deed: a successor line
    /// adopted this world's chronicle as its own substrate (the
    /// derelict door, 07 §3). The living tree drops to the world's
    /// kernel; every discovered node stands in the record forever —
    /// the pre-existing is changed, never replaced.
    LineSucceeded { successor: String },
    /// G1 (04 §1/§5) — the three axes sealed as THE binding copy: from
    /// this receipt on, the registry entry is a cache of receipted
    /// truth and no mechanic reads it where this receipt exists.
    /// Genesis is entry in the bands (04 §6a) — never a tier-0 seat.
    GenesisSealed { civilization: String, disposition: u8, metabolism: u8, perception: u8 },
    /// PRACTICE-1 (14 §4) — a receipted discipline binds the line:
    /// one benefit, one friction, exact-rational, on landed hooks.
    PracticeAdopted { practice: u8 },
    /// PRACTICE-1 — the rediscovery: lawful only after new doctrine
    /// landed since adoption; the re-emergence node rides separately
    /// as TechDiscovered condition 7.
    PracticeReleased { practice: u8 },
    /// ELEMENT-GRAMMAR-1 (14 §3) — the mint deed: an element's
    /// identity read from what the genesis seed always held. The
    /// receipt CARRIES the hash-derived properties, so replay reads
    /// and never re-rolls. The mint creates identity, never matter.
    ProspectStruck {
        orb_id: u64,
        stratum: u8,
        element: [u8; 32],
        family: u8,
        tier: u8,
        richness: Ratio,
        name: String,
    },
    /// ELEMENT-GRAMMAR-1 (14 §3) — the extraction closure: richness ×
    /// elapsed banked in ONE receipt (the bond-settle shape).
    /// ECO-1 (#40): `richness` is the vein's flow AFTER this deed —
    /// wear from the draw and recovery from the orb's regenerators
    /// settle in the same receipt ([`vein_wear`], the one home).
    /// `None` is the sealed pre-ecology form: richness holds still.
    AssayStruck { orb_id: u64, element: [u8; 32], amount: Ratio, richness: Option<Ratio> },
    /// WARDEN-1a (10 §3e) — the reverse shield: this star is the
    /// elder; every aperture refuses until the ward lifts.
    ElderWarded {},
    /// WARDEN-1a — the depth gate opened by `claimer`; the elder
    /// trades from here on (the template grant is WARDEN-1b).
    WardLifted { claimer: String },
    /// MASK-1a (14 §4) — the lawful veil: renders cloud, the
    /// chronicle never lies; a standing joule toll settles at every
    /// collect.
    VeilDrawn {},
    /// MASK-1a — the veil ends: by deed (starved 0) or STARVED by an
    /// unpayable toll (starved 1), receipted either way.
    VeilLifted { starved: bool },
    /// INCURSION-1a (07 §3.1) — the door: a sealed derelict continued
    /// live under a woken line. Universe ledger ONLY; world replay
    /// ignores it (the EraAnnexed precedent).
    IncursionStruck { era: String, world: String, line: String },
    /// KERNEL-TAXON (15 §4) — class and clade sealed as identity,
    /// once per star, beside the G1 seal and never inside it (a new
    /// deed is additive; extending GenesisSealed would be breaking).
    /// Codes must stand in the ratified vocabularies; names stay
    /// register-side forever.
    TaxonSealed { class: u8, clade: u8 },
    /// **THE VOCABULARY MINT** (tag 90, `28_VOCABULARY` §7,
    /// 2026-08-05): a T0 progenitor adds an entry to a vocabulary.
    ///
    /// This is what makes the class, clade, axis and practice tables
    /// GROWABLE without a coder. The definition law bars a FILE from
    /// saying what exists; it never barred the CHAIN, because a minting
    /// is something that HAPPENED and every node replays it identically.
    ///
    /// `code` is CARRIED AND VERIFIED, never chosen — the lawful value
    /// is `ceiling(axis) + 1` read from the fold, so two nodes cannot
    /// mint colliding codes and a reader who has not folded the chain
    /// can still see which entry this is.
    ///
    /// `binding` is the entry's MECHANISM (§4): exact rational factors
    /// over the closed channel set. Data, not code — no sandbox, no
    /// interpreter, no new capability class. An entry that computes
    /// nothing is refused, which is `27_EVIDENCE` LAW 8 enforced at the
    /// door instead of discovered later.
    VocabularyMinted {
        axis: u8,
        code: u8,
        sigil: String,
        word: String,
        binding: Vec<(u8, Ratio)>,
        /// **THE DECLARED BASE (#41)** — for a ZONE mint, the compiled
        /// kind this entry's binding modulates. `None` on every other
        /// axis, and on a zone mint that leaves the base to be inferred
        /// from its channels (`vocabulary::zone_base`). Rides as a
        /// trailing byte by the additive pattern, so every sealed
        /// pre-zone mint decodes with it absent.
        base: Option<u8>,
    },
    /// THE VERTICAL AXIS (tag 89, 2026-08-01): a line takes an office.
    ///
    /// The station GATES AUTHORITY — the tier-0 verbs check it — and it
    /// lived only in `var/registry/<sigil>.json`, editable by anyone
    /// with a shell. `KERNEL_RELAY` §1.5 named this exactly: *the game
    /// is constitutional; the universe is not*. Now the seat is a deed.
    ///
    /// `station` is the closed office vocabulary by index (1 progenitor
    /// … 10 anchor; T4–T9 Wave-1); 0 and anything outside 1..=10 are
    /// not offices and the wire refuses them via [`Station::from_wire`].
    StationSeated { line: String, station: u8 },
    /// U1 KERNEL-VERIFY (U_DIRECTIVES §2): the discovery mint — value
    /// created ONLY against verified work (the PoUW anchor, I4's
    /// bound arm). `work` is the OPAQUE proof payload — encoding is
    /// solver-verify's monopoly (THE OPAQUE-PAYLOAD LAW): cfe carries
    /// it, never reads it; the kernel hands it to the sealed verifier
    /// whole. The receipt carries its proof — replay re-verifies.
    DiscoveryMinted { orb_id: u64, resource: Resource, amount: Ratio, work: Vec<u8> },
    /// MANU-1 (10 §3f): an instrument forged — technology +
    /// understanding + industry overlaid; held per-orb like techs.
    InstrumentForged { orb_id: u64, tech_hash: [u8; 32], grade: u8 },
    /// MARKET-2 (conform ruling): the void's exact share of a hauled
    /// deal — receipted on the receiving book by the same transition
    /// that computes the trade. Banks never move on it; it is the
    /// loss made visible.
    ///
    /// **`orb_id` ADDED 2026-08-02 (SYN-6, operator approved).** The
    /// record named the DEAL and never the BEARER, so the melt was
    /// receipted and credited to no one: a loss visible in the
    /// aggregate and unattributable to any book. Every other
    /// value-bearing receipt in this codec carries the orb that bore
    /// it — `DemurrageSettled`, `UpkeepSettled`, `AssayStruck` — and
    /// this one alone did not, which is why `World::value_lost_of`
    /// could account decay and upkeep per orb and had to leave the
    /// melt out entirely. The field is FIRST, matching the
    /// `LE64 ‖ …` layout every orb-bearing record already uses.
    VoidShare { orb_id: u64, deal: u64, resource: Resource, amount: Ratio },
    /// D5 — **THE SOVEREIGN UNION**, struck. The offices `D(Ω)`
    /// PROGENITOR and `V(Ω)` SOVEREIGN enter their union under a
    /// REVIEW HORIZON.
    ///
    /// **PARTIES ARE OFFICES, NEVER ORB IDS.** `BondSealed` carries
    /// `source: u64, target: u64` — colony ids — and seating an office
    /// there would reduce a global field to a local node. Both offices
    /// are named explicitly so the deed is self-describing and so a
    /// malformed union (two progenitors, two sovereigns) is REFUSABLE
    /// rather than merely unlikely.
    ///
    /// **NO `approved_by_operator` FLAG.** An earlier design carried
    /// one. A deed is a CLAIM, and a claim that asserts its own
    /// approval is forgeable by whoever constructs it — the kernel
    /// would have nothing to check the bool against. Approval is
    /// established by WHO ACTS: I2 containment already binds the actor
    /// to the root entity, unforgeably, and that is the signature.
    ///
    /// `term_stamps` is a REVIEW horizon, never an expiry: nothing
    /// settles and nothing lapses when it falls due.
    SovereignUnionSealed { progenitor: Office, sovereign: Office, term_stamps: u64 },
    /// ROUTE-1: a lane chartered — this book's leg of the pair.
    RouteChartered { deal: u64, eta: Unit },
    /// ROUTE-1: the lane lapsed on this book (unpaid maintenance).
    RouteLapsed { deal: u64 },
    /// **THE ESTATE SURVEY** (tag 97, E5b — operator rulings 2026-08-05:
    /// *survey is a derivation of observation; observations must be
    /// witnessed; surveys must be metered*).
    ///
    /// THEOREM (witnessed): `covered` carries one entry per orb the
    /// survey stood over — the orb's id and the DIGEST OF ITS OWN
    /// FOUNDING RECEIPT, gathered from the chronicle at the deed. The
    /// emitter refuses an orb it cannot witness, and the gate refuses
    /// an empty covering, so an unwitnessed observation is not a state
    /// that can enter the record.
    ///
    /// THEOREM (metered): `cost` equals the sealed meter over the
    /// volume — `survey_meter().1 × covered.len()` — checked by the
    /// gate calling the SAME rule (call-not-copy), and drawn from
    /// `payer`'s bank before the receipt closes. A survey that cannot
    /// pay refuses pure: no receipt, no debit.
    ///
    /// THEOREM (derivation): `seed` is the chronicle's terminal digest
    /// at the survey (CHARTER §9). Parcels are not state — they DERIVE
    /// from this receipt via [`parcels_of`], the same receipt yielding
    /// the same parcels forever. Nothing here is stored that replay
    /// cannot re-check.
    EstateSurveyed {
        seed: [u8; 32],
        payer: u64,
        covered: Vec<(u64, [u8; 32])>,
        cost: Ratio,
    },
    /// **THE APPRAISAL** (tag 98): value derived from the survey, at
    /// one scale of the ladder, self-contained for the gate — `seed`
    /// and `covered` ride along so the gate re-derives the parcels and
    /// the value from the prior state alone (the covenant-collateral
    /// precedent: what verification needs rides the receipt). Moves no
    /// value; the stamp is what makes staleness decidable, and
    /// STALENESS TRIGGERS RENEGOTIATION, NEVER BLOCKS ACCEPTANCE.
    AppraisalStruck {
        subject: Holding,
        survey: [u8; 32],
        seed: [u8; 32],
        covered: Vec<u64>,
        value: Ratio,
    },
    /// **THE COUNTER-OFFER** (tag 99): renegotiation as a receipt. It
    /// amends the resting offer's ask — the negotiation stays STANDING
    /// (resting books are lawful) and every round is a receipt. The
    /// optional `appraisal` digest is the round's evidence; the emitter
    /// refuses a STALE citation, because staleness is the trigger that
    /// demands fresh evidence — and deliberately gates nothing else.
    OfferCountered {
        offer: u64,
        want_amount: Ratio,
        appraisal: Option<[u8; 32]>,
    },
    /// **CONVEY** (tag 100): the postal deed. The parcel's monetary
    /// break finally MEANS something mechanically: only the monetary
    /// portion of the sender's bank is postal-addressable, the free
    /// portion stays home, and both endpoints must be surveyed parcels.
    /// `seed` and `covered` ride along so the gate re-derives the
    /// parcels, the monetary cap and the postage from the prior state
    /// alone (the appraisal's self-containment pattern). Postage burns
    /// — the amount arrives whole.
    ParcelConveyed {
        from_orb: u64,
        to_orb: u64,
        resource: Resource,
        amount: Ratio,
        postage: Ratio,
        survey: [u8; 32],
        seed: [u8; 32],
        covered: Vec<u64>,
    },
    /// **THE MINTED LINE** (tag 101): a civilization enters by receipt.
    /// The seed is the whole receipt — every field of the line derives
    /// from it through [`line_synthesis`], so the wire carries ONE
    /// truth and the registry card is a render of it. Banked-neutral:
    /// a mint creates IDENTITY, never matter (the element grammar's
    /// law, applied to peoples).
    ///
    /// **FOLD-AWARE (operator ruling 2026-08-06, receipt authority):**
    /// `ceilings` declares the (class, clade) vocabulary ceilings the
    /// draw was taken under — the FOLD's ceilings at the mint stamp, so
    /// minted classes and clades are drawable the moment their receipts
    /// stand. `None` is the sealed pre-amendment form: those receipts
    /// drew under the compiled ceilings, and decode to exactly that,
    /// byte-faithful both ways.
    LineMinted { seed: [u8; 32], ceilings: Option<(u8, u8)> },
}

impl ClosureEvent {
    /// The normative TLV event encoding (NS-3 §2.2): one record, tag =
    /// the event's permanent code, value = the event's field layout.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        match self {
            Self::ColonyFounded { orb_id, name } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(name.as_bytes());
                tlv::put(1, &value, &mut out);
            }
            Self::ZoneBuilt { orb_id, tile, kind, base } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&tile.to_le_bytes());
                value.push(kind.wire());
                // #41: the base this build was priced against, trailing
                // and optional — the gate reads it because it cannot
                // read the fold.
                if let Some(base) = base {
                    value.push(base.wire());
                }
                tlv::put(2, &value, &mut out);
            }
            Self::YieldCollected { orb_id, resource, amount } => {
                tlv::put(3, &amount_layout(*orb_id, *resource, amount), &mut out);
            }
            Self::UpkeepSettled { orb_id, resource, amount } => {
                tlv::put(4, &amount_layout(*orb_id, *resource, amount), &mut out);
            }
            Self::FamineStruck { orb_id, resource, drawn } => {
                tlv::put(5, &amount_layout(*orb_id, *resource, drawn), &mut out);
            }
            Self::FamineRelieved { orb_id, resource } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*resource as u8);
                tlv::put(6, &value, &mut out);
            }
            Self::ZoneRazed { orb_id, tile } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&tile.to_le_bytes());
                tlv::put(7, &value, &mut out);
            }
            Self::WorldCharted { seed } => {
                tlv::put(8, seed, &mut out);
            }
            Self::EntityRegistered { entity_id, principal, handle } => {
                let mut value = entity_id.to_le_bytes().to_vec();
                value.extend_from_slice(&principal.to_le_bytes());
                value.extend_from_slice(handle.as_bytes());
                tlv::put(10, &value, &mut out);
            }
            Self::ScopeGranted { agent, scope } => {
                let mut value = agent.to_le_bytes().to_vec();
                value.push(*scope as u8);
                tlv::put(11, &value, &mut out);
            }
            Self::ScopeRevoked { agent, scope } => {
                let mut value = agent.to_le_bytes().to_vec();
                value.push(*scope as u8);
                tlv::put(12, &value, &mut out);
            }
            Self::ColonyCharted { orb_id, source_orb, name } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&source_orb.to_le_bytes());
                value.extend_from_slice(name.as_bytes());
                tlv::put(13, &value, &mut out);
            }
            Self::EraAnnexed { era, world, terminal, offset } => {
                let mut value = offset.to_le_bytes().to_vec();
                value.extend_from_slice(terminal);
                value.extend_from_slice(&(era.len() as u16).to_le_bytes());
                value.extend_from_slice(era.as_bytes());
                value.extend_from_slice(world.as_bytes());
                tlv::put(21, &value, &mut out);
            }
            Self::GalaxyObserved { observer, creator, name, center, reach } => {
                let mut value = Vec::new();
                for text in [observer, creator, name] {
                    value.extend_from_slice(&(text.len() as u16).to_le_bytes());
                    value.extend_from_slice(text.as_bytes());
                }
                ratio::encode(&center.0, &mut value);
                ratio::encode(&center.1, &mut value);
                ratio::encode(&center.2, &mut value);
                ratio::encode(reach, &mut value);
                tlv::put(54, &value, &mut out);
            }
            Self::OverheatStruck { orb_id, overflow } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(overflow, &mut value);
                tlv::put(22, &value, &mut out);
            }
            Self::OverheatCleared { orb_id } => {
                tlv::put(23, &orb_id.to_le_bytes(), &mut out);
            }
            Self::StarCommissioned { class } => {
                tlv::put(24, &[*class], &mut out);
            }
            Self::PlanetSited { orb_id, albedo, orbit_r, theta, psi } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(albedo.value(), &mut value);
                ratio::encode(orbit_r.value(), &mut value);
                // TRAILING and OPTIONAL: an unsited angle writes no
                // bytes at all, so a receipt that never had one is
                // byte-identical to what it was before the amendment.
                if let Some(turn) = theta {
                    ratio::encode(turn.angle(), &mut value);
                    if let Some(tilt) = psi {
                        ratio::encode(tilt.angle(), &mut value);
                    }
                }
                tlv::put(25, &value, &mut out);
            }
            Self::VentSettled { orb_id, amount } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                tlv::put(27, &value, &mut out);
            }
            Self::DemurrageSettled { orb_id, resource, amount } => {
                tlv::put(29, &amount_layout(*orb_id, *resource, amount), &mut out);
            }
            Self::DemurrageTouched { orb_id, decays } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(decays.len() as u8);
                for (resource, amount) in decays {
                    value.push(*resource as u8);
                    ratio::encode(amount, &mut value);
                }
                tlv::put(102, &value, &mut out);
            }
            Self::ParameterChartered { key, value } => {
                let mut bytes = vec![*key];
                ratio::encode(value, &mut bytes);
                tlv::put(103, &bytes, &mut out);
            }
            Self::TreatySealed { deal, kind, partner, terms, term } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.push(*kind);
                value.extend_from_slice(&term.to_le_bytes());
                value.push(terms.len() as u8);
                for (channel, factor) in terms {
                    value.push(*channel);
                    ratio::encode(factor, &mut value);
                }
                value.extend_from_slice(partner.as_bytes());
                tlv::put(105, &value, &mut out);
            }
            Self::TreatyStruck { deal, partner, broken } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.push(u8::from(*broken));
                value.extend_from_slice(partner.as_bytes());
                tlv::put(106, &value, &mut out);
            }
            Self::GemCompounded { orb_id, members, gem } => {
                // orb ‖ gem ‖ members — the member count is the
                // remainder over 32, so no length prefix is needed and
                // a truncated value cannot decode as a shorter set.
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(gem);
                for member in members {
                    value.extend_from_slice(member);
                }
                tlv::put(104, &value, &mut out);
            }
            Self::ElementRefined { orb_id, raw, refined, spent, yielded } => {
                // orb ‖ raw ‖ refined ‖ spent ‖ yielded
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(raw);
                value.extend_from_slice(refined);
                ratio::encode(spent, &mut value);
                ratio::encode(yielded, &mut value);
                tlv::put(107, &value, &mut out);
            }
            Self::ResourceTransferred { from_orb, to_orb, resource, amount } => {
                let mut value = from_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&to_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                tlv::put(30, &value, &mut out);
            }
            Self::OverflowAbsorbed { source_orb, absorber_orb, amount } => {
                let mut value = source_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&absorber_orb.to_le_bytes());
                ratio::encode(amount, &mut value);
                tlv::put(31, &value, &mut out);
            }
            Self::AbsorptionLapsed { debtor_orb, creditor_orb, amount } => {
                let mut value = debtor_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&creditor_orb.to_le_bytes());
                ratio::encode(amount, &mut value);
                tlv::put(32, &value, &mut out);
            }
            Self::BondSealed { source_orb, target_orb, resource, rate, term } => {
                let mut value = source_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&target_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(rate, &mut value);
                value.extend_from_slice(&term.to_le_bytes());
                tlv::put(33, &value, &mut out);
            }
            Self::BondSettled { source_orb, target_orb, resource, amount, severed, load } => {
                let mut value = source_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&target_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                value.push(*severed as u8);
                ratio::encode(load, &mut value);
                tlv::put(34, &value, &mut out);
            }
            Self::ForwardSealed {
                obligor_orb,
                beneficiary_orb,
                resource,
                quota,
                window,
                collateral,
            } => {
                let mut value = obligor_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&beneficiary_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(quota, &mut value);
                value.extend_from_slice(&window.to_le_bytes());
                ratio::encode(collateral, &mut value);
                tlv::put(35, &value, &mut out);
            }
            Self::ForwardReleased { obligor_orb, beneficiary_orb, resource, collateral } => {
                tlv::put(
                    36,
                    &pact_layout(*obligor_orb, *beneficiary_orb, *resource, collateral),
                    &mut out,
                );
            }
            Self::ForwardSlashed { obligor_orb, beneficiary_orb, resource, collateral } => {
                tlv::put(
                    37,
                    &pact_layout(*obligor_orb, *beneficiary_orb, *resource, collateral),
                    &mut out,
                );
            }
            Self::FrictionSettled { orb_id, amount } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                tlv::put(38, &value, &mut out);
            }
            Self::TechLicensed { deal, from_orb, council, condition, node, name } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&from_orb.to_le_bytes());
                value.extend_from_slice(&council.to_le_bytes());
                value.push(*condition);
                value.extend_from_slice(node);
                value.extend_from_slice(name.as_bytes());
                tlv::put(40, &value, &mut out);
            }
            Self::CouncilSealed { council, resource, quota, window, collateral, members } => {
                let mut value = council.to_le_bytes().to_vec();
                value.push(*resource as u8);
                ratio::encode(quota, &mut value);
                value.extend_from_slice(&window.to_le_bytes());
                ratio::encode(collateral, &mut value);
                for member in members {
                    value.extend_from_slice(&member.to_le_bytes());
                }
                tlv::put(52, &value, &mut out);
            }
            Self::CouncilClosed { council, member, fulfilled, banked } => {
                let mut value = council.to_le_bytes().to_vec();
                value.extend_from_slice(&member.to_le_bytes());
                value.push(*fulfilled as u8);
                ratio::encode(banked, &mut value);
                tlv::put(53, &value, &mut out);
            }
            Self::GaugeShifted { gauge } => {
                let mut value = Vec::new();
                ratio::encode(gauge, &mut value);
                tlv::put(42, &value, &mut out);
            }
            Self::GaugeShiftedAt { grade, gauge } => {
                let mut value = grade.to_le_bytes().to_vec();
                ratio::encode(gauge, &mut value);
                tlv::put(60, &value, &mut out);
            }
            Self::WantStruck { orb_id, kind, reading } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*kind);
                ratio::encode(reading, &mut value);
                tlv::put(62, &value, &mut out);
            }
            Self::WantRelieved { orb_id, kind } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*kind);
                tlv::put(63, &value, &mut out);
            }
            Self::HandoffSealed { from, to } => {
                let mut value = from.to_le_bytes().to_vec();
                value.extend_from_slice(&to.to_le_bytes());
                tlv::put(67, &value, &mut out);
            }
            Self::OfferPosted { offer, orb_id, give, give_amount, want, want_amount } => {
                let mut value = offer.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                value.push(*give as u8);
                ratio::encode(give_amount, &mut value);
                value.push(*want as u8);
                ratio::encode(want_amount, &mut value);
                tlv::put(49, &value, &mut out);
            }
            Self::OfferCleared { offer, deal } => {
                let mut value = offer.to_le_bytes().to_vec();
                value.extend_from_slice(&deal.to_le_bytes());
                tlv::put(50, &value, &mut out);
            }
            Self::Diverged { branch, gauge } => {
                let mut value = Vec::new();
                ratio::encode(gauge, &mut value);
                value.extend_from_slice(branch.as_bytes());
                tlv::put(43, &value, &mut out);
            }
            Self::Confluent { partner, gauge } => {
                let mut value = Vec::new();
                ratio::encode(gauge, &mut value);
                value.extend_from_slice(partner.as_bytes());
                tlv::put(44, &value, &mut out);
            }
            Self::StarAged { class } => {
                tlv::put(45, &[*class], &mut out);
            }
            Self::EraFolded { era, gauge } => {
                let mut value = era.to_le_bytes().to_vec();
                ratio::encode(gauge, &mut value);
                tlv::put(46, &value, &mut out);
            }
            Self::TechDiscovered { orb_id, node, condition, modifier, magnitude, name } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(node);
                value.push(*condition);
                value.push(*modifier);
                ratio::encode(magnitude, &mut value);
                value.extend_from_slice(name.as_bytes());
                tlv::put(39, &value, &mut out);
            }
            Self::ApertureSent { deal, from_orb, resource, amount } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&from_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                tlv::put(47, &value, &mut out);
            }
            Self::ApertureReceived { deal, to_orb, resource, amount } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&to_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                tlv::put(48, &value, &mut out);
            }
            Self::ElementApertureSent { deal, from_orb, element, amount } => {
                // deal ‖ from_orb ‖ element[32] ‖ amount
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&from_orb.to_le_bytes());
                value.extend_from_slice(element);
                ratio::encode(amount, &mut value);
                tlv::put(108, &value, &mut out);
            }
            Self::ElementApertureReceived { deal, to_orb, element, amount } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&to_orb.to_le_bytes());
                value.extend_from_slice(element);
                ratio::encode(amount, &mut value);
                tlv::put(109, &value, &mut out);
            }
            Self::ElementRouteChartered { deal, element, eta, rail } => {
                // deal ‖ element[32] ‖ eta ‖ rail
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(element);
                ratio::encode(eta.value(), &mut value);
                value.push(*rail);
                tlv::put(110, &value, &mut out);
            }
            Self::StampMinted { orb_id, amount, crystal_spent } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                ratio::encode(crystal_spent, &mut value);
                tlv::put(111, &value, &mut out);
            }
            Self::StampApertureSent { deal, from_orb, amount } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&from_orb.to_le_bytes());
                ratio::encode(amount, &mut value);
                tlv::put(112, &value, &mut out);
            }
            Self::StampApertureReceived { deal, to_orb, amount } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&to_orb.to_le_bytes());
                ratio::encode(amount, &mut value);
                tlv::put(113, &value, &mut out);
            }
            Self::ScrapSalvaged { orb_id, amount, iron_spent } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                ratio::encode(iron_spent, &mut value);
                tlv::put(114, &value, &mut out);
            }
            Self::HullMinted {
                hull_id,
                orb_id,
                role,
                grade,
                scrap_spent,
            } => {
                let mut value = hull_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                value.push(*role);
                value.push(*grade);
                ratio::encode(scrap_spent, &mut value);
                tlv::put(115, &value, &mut out);
            }
            Self::HullAssigned { hull_id, deal } => {
                let mut value = hull_id.to_le_bytes().to_vec();
                value.extend_from_slice(&deal.to_le_bytes());
                tlv::put(116, &value, &mut out);
            }
            Self::HullScrapped {
                hull_id,
                orb_id,
                scrap_returned,
            } => {
                let mut value = hull_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                ratio::encode(scrap_returned, &mut value);
                tlv::put(117, &value, &mut out);
            }
            Self::OrderWorked {
                orb_id,
                kind,
                entropy,
                syntropy,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*kind);
                ratio::encode(entropy, &mut value);
                ratio::encode(syntropy, &mut value);
                tlv::put(118, &value, &mut out);
            }
            Self::ElementSalvaged {
                orb_id,
                element,
                amount,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(element);
                ratio::encode(amount, &mut value);
                tlv::put(119, &value, &mut out);
            }
            Self::NoteIssued {
                note_id,
                orb_id,
                issuer,
                holder,
                notional,
                collateral,
                maturity,
            } => {
                let mut value = note_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                value.extend_from_slice(&issuer.to_le_bytes());
                value.extend_from_slice(&holder.to_le_bytes());
                ratio::encode(notional, &mut value);
                ratio::encode(collateral, &mut value);
                value.extend_from_slice(&maturity.to_le_bytes());
                tlv::put(120, &value, &mut out);
            }
            Self::NoteRedeemed {
                note_id,
                orb_id,
                paid,
                seized,
            } => {
                let mut value = note_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                ratio::encode(paid, &mut value);
                ratio::encode(seized, &mut value);
                tlv::put(121, &value, &mut out);
            }
            Self::FutureOpened {
                fut_id,
                orb_id,
                long,
                short,
                resource,
                notional,
                price,
                margin,
                expiry,
            } => {
                let mut value = fut_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                value.extend_from_slice(&long.to_le_bytes());
                value.extend_from_slice(&short.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(notional, &mut value);
                ratio::encode(price, &mut value);
                ratio::encode(margin, &mut value);
                value.extend_from_slice(&expiry.to_le_bytes());
                tlv::put(122, &value, &mut out);
            }
            Self::FutureSettled {
                fut_id,
                orb_id,
                mark,
                long_pnl,
                short_pnl,
            } => {
                let mut value = fut_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                ratio::encode(mark, &mut value);
                ratio::encode(long_pnl, &mut value);
                ratio::encode(short_pnl, &mut value);
                tlv::put(123, &value, &mut out);
            }
            Self::LastClearMarked {
                orb_id,
                resource,
                price,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*resource as u8);
                ratio::encode(price, &mut value);
                tlv::put(124, &value, &mut out);
            }
            Self::ShareIssued {
                share_id,
                orb_id,
                holder,
                stream,
                units,
                par,
            } => {
                let mut value = share_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                value.extend_from_slice(&holder.to_le_bytes());
                value.push(*stream as u8);
                ratio::encode(units, &mut value);
                ratio::encode(par, &mut value);
                tlv::put(125, &value, &mut out);
            }
            Self::ShareTransferred {
                share_id,
                from,
                to,
                units,
            } => {
                let mut value = share_id.to_le_bytes().to_vec();
                value.extend_from_slice(&from.to_le_bytes());
                value.extend_from_slice(&to.to_le_bytes());
                ratio::encode(units, &mut value);
                tlv::put(126, &value, &mut out);
            }
            Self::ShareDividend {
                share_id,
                orb_id,
                stream,
                resource_cut,
                amount,
            } => {
                let mut value = share_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                value.push(*stream as u8);
                ratio::encode(resource_cut, &mut value);
                ratio::encode(amount, &mut value);
                tlv::put(127, &value, &mut out);
            }
            Self::PerpOpened {
                perp_id,
                orb_id,
                long,
                short,
                resource,
                notional,
                entry,
                margin,
            } => {
                let mut value = perp_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                value.extend_from_slice(&long.to_le_bytes());
                value.extend_from_slice(&short.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(notional, &mut value);
                ratio::encode(entry, &mut value);
                ratio::encode(margin, &mut value);
                tlv::put(128, &value, &mut out);
            }
            Self::PerpFunded {
                perp_id,
                orb_id,
                funding,
                mark,
            } => {
                let mut value = perp_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                ratio::encode(funding, &mut value);
                ratio::encode(mark, &mut value);
                tlv::put(129, &value, &mut out);
            }
            Self::PerpClosed {
                perp_id,
                orb_id,
                mark,
                long_pnl,
                liquidated,
            } => {
                let mut value = perp_id.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                ratio::encode(mark, &mut value);
                ratio::encode(long_pnl, &mut value);
                value.push(u8::from(*liquidated));
                tlv::put(130, &value, &mut out);
            }
            Self::HullFinanced {
                hull_id,
                note_id,
                orb_id,
                scrap_spent,
                note_face,
            } => {
                let mut value = hull_id.to_le_bytes().to_vec();
                value.extend_from_slice(&note_id.to_le_bytes());
                value.extend_from_slice(&orb_id.to_le_bytes());
                ratio::encode(scrap_spent, &mut value);
                ratio::encode(note_face, &mut value);
                tlv::put(131, &value, &mut out);
            }
            Self::EscortOnDuty {
                hull_id,
                deal,
                grade,
            } => {
                let mut value = hull_id.to_le_bytes().to_vec();
                value.extend_from_slice(&deal.to_le_bytes());
                value.push(*grade);
                tlv::put(132, &value, &mut out);
            }
            Self::InterdictionStruck {
                deal,
                orb_id,
                resource,
                amount,
                risk,
            } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.extend_from_slice(&orb_id.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                ratio::encode(risk, &mut value);
                tlv::put(133, &value, &mut out);
            }
            Self::ConvoyRiskMarked {
                deal,
                risk,
                escort_cover,
            } => {
                let mut value = deal.to_le_bytes().to_vec();
                ratio::encode(risk, &mut value);
                ratio::encode(escort_cover, &mut value);
                tlv::put(134, &value, &mut out);
            }
            Self::MarketSealed { resume_at } => {
                tlv::put(135, &resume_at.to_le_bytes(), &mut out);
            }
            Self::MarketResumed {} => {
                tlv::put(136, &[], &mut out);
            }
            Self::SpotClearRecorded {
                orb_id,
                resource,
                price,
                depth,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*resource as u8);
                ratio::encode(price, &mut value);
                value.extend_from_slice(&depth.to_le_bytes());
                tlv::put(137, &value, &mut out);
            }
            Self::LiquidityRaidStruck {
                orb_id,
                fee,
                offers_voided,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(fee, &mut value);
                value.push(*offers_voided);
                tlv::put(138, &value, &mut out);
            }
            Self::ClearPoisonRefused {
                orb_id,
                resource,
                price,
                depth,
                need,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*resource as u8);
                ratio::encode(price, &mut value);
                value.extend_from_slice(&depth.to_le_bytes());
                value.extend_from_slice(&need.to_le_bytes());
                tlv::put(139, &value, &mut out);
            }
            Self::NoteCascadeStruck {
                orb_id,
                fee,
                pressure,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(fee, &mut value);
                value.extend_from_slice(&pressure.to_le_bytes());
                tlv::put(140, &value, &mut out);
            }
            Self::CovertTamperStruck {
                orb_id,
                cost,
                pressure,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(cost, &mut value);
                ratio::encode(pressure, &mut value);
                tlv::put(141, &value, &mut out);
            }
            Self::MediumReinforced {
                orb_id,
                cost,
                pressure,
                resource,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(cost, &mut value);
                ratio::encode(pressure, &mut value);
                value.push(*resource as u8);
                tlv::put(142, &value, &mut out);
            }
            Self::TamperDetected { orb_id, pressure } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(pressure, &mut value);
                tlv::put(143, &value, &mut out);
            }
            Self::PostureReoriented {
                disposition,
                perception,
                primary,
            } => {
                tlv::put(144, &[*disposition, *perception, *primary], &mut out);
            }
            Self::SpyProbed {
                orb_id,
                depth,
                fee,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*depth);
                ratio::encode(fee, &mut value);
                tlv::put(145, &value, &mut out);
            }
            Self::TechExfiltrated {
                condition,
                tech_hash,
                fee,
            } => {
                let mut value = vec![*condition];
                value.extend_from_slice(tech_hash);
                ratio::encode(fee, &mut value);
                tlv::put(146, &value, &mut out);
            }
            Self::InstrumentSabotaged {
                orb_id,
                grade,
                tech_hash,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*grade);
                value.extend_from_slice(tech_hash);
                tlv::put(147, &value, &mut out);
            }
            Self::RailPromoted {
                deal,
                from_rail,
                to_rail,
            } => {
                let mut value = deal.to_le_bytes().to_vec();
                value.push(*from_rail);
                value.push(*to_rail);
                tlv::put(148, &value, &mut out);
            }
            Self::StrandExported {
                orb_id,
                amount,
                claim_hash,
                kind,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                value.extend_from_slice(claim_hash);
                value.push(*kind);
                tlv::put(149, &value, &mut out);
            }
            Self::AnchorImported {
                orb_id,
                amount,
                claim_hash,
                kind,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                value.extend_from_slice(claim_hash);
                value.push(*kind);
                tlv::put(150, &value, &mut out);
            }
            Self::AnchorImportRefused { orb_id, reason } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*reason);
                tlv::put(151, &value, &mut out);
            }
            Self::VaultDeposited { orb_id, amount } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                tlv::put(152, &value, &mut out);
            }
            Self::VaultWithdrawn { orb_id, amount } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                tlv::put(153, &value, &mut out);
            }
            Self::AssayConsilience {
                orb_id,
                element,
                matched,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(element);
                value.push(u8::from(*matched));
                tlv::put(154, &value, &mut out);
            }
            Self::AssaySlashed {
                orb_id,
                element,
                slash,
                richness_after,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(element);
                ratio::encode(slash, &mut value);
                ratio::encode(richness_after, &mut value);
                tlv::put(155, &value, &mut out);
            }
            Self::IsoUnionStruck {
                orb_id,
                a,
                b,
                product,
                knowledge,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(a);
                value.extend_from_slice(b);
                value.extend_from_slice(product);
                value.extend_from_slice(&knowledge.to_le_bytes());
                tlv::put(156, &value, &mut out);
            }
            Self::ChemCataloged {
                orb_id,
                gem,
                knowledge,
                survival,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(gem);
                value.extend_from_slice(&knowledge.to_le_bytes());
                ratio::encode(survival, &mut value);
                tlv::put(157, &value, &mut out);
            }
            Self::DarkPullStruck {
                orb_id,
                pull,
                weight,
                dark_score,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*pull);
                ratio::encode(weight, &mut value);
                ratio::encode(dark_score, &mut value);
                tlv::put(158, &value, &mut out);
            }
            Self::ScienceSeated {
                orb_id,
                discipline,
                rank,
                fee,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*discipline);
                value.push(*rank);
                ratio::encode(fee, &mut value);
                tlv::put(159, &value, &mut out);
            }
            Self::LabCreated {
                orb_id,
                path,
                risk,
                exploded,
                explosion_kind,
                hidden_pos,
                hidden_neg,
                fee,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*path);
                ratio::encode(risk, &mut value);
                value.push(u8::from(*exploded));
                value.push(*explosion_kind);
                value.push(u8::from(*hidden_pos));
                value.push(u8::from(*hidden_neg));
                ratio::encode(fee, &mut value);
                tlv::put(160, &value, &mut out);
            }
            Self::ScienceExploded {
                orb_id,
                path,
                kind,
                resource,
                slash,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*path);
                value.push(*kind);
                value.push(*resource);
                ratio::encode(slash, &mut value);
                tlv::put(161, &value, &mut out);
            }
            Self::RogueFactionStruck { orb_id, pressure } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*pressure);
                tlv::put(162, &value, &mut out);
            }
            Self::CrystalDrawn { orb_id, amount } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                ratio::encode(amount, &mut value);
                tlv::put(55, &value, &mut out);
            }
            Self::LineSucceeded { successor } => {
                tlv::put(59, successor.as_bytes(), &mut out);
            }
            Self::GenesisSealed { civilization, disposition, metabolism, perception } => {
                let mut value = vec![*disposition, *metabolism, *perception];
                value.extend_from_slice(civilization.as_bytes());
                tlv::put(61, &value, &mut out);
            }
            Self::PracticeAdopted { practice } => {
                tlv::put(68, &[*practice], &mut out);
            }
            Self::PracticeReleased { practice } => {
                tlv::put(69, &[*practice], &mut out);
            }
            Self::ProspectStruck { orb_id, stratum, element, family, tier, richness, name } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*stratum);
                value.extend_from_slice(element);
                value.push(*family);
                value.push(*tier);
                ratio::encode(richness, &mut value);
                value.extend_from_slice(name.as_bytes());
                tlv::put(70, &value, &mut out);
            }
            Self::AssayStruck { orb_id, element, amount, richness } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(element);
                ratio::encode(amount, &mut value);
                // ECO-1: the trailing after-richness, additive — its
                // absence IS the sealed legacy form, byte-exact.
                if let Some(after) = richness {
                    ratio::encode(after, &mut value);
                }
                tlv::put(71, &value, &mut out);
            }
            Self::BookMinted { orb_id, kind, name } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&kind.0);
                value.extend_from_slice(name.as_bytes());
                tlv::put(91, &value, &mut out);
            }
            Self::BookPosted { orb_id, kind, kept, lost } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&kind.0);
                ratio::encode(kept, &mut value);
                ratio::encode(lost, &mut value);
                tlv::put(92, &value, &mut out);
            }
            Self::BookArchived { orb_id, kind } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&kind.0);
                tlv::put(93, &value, &mut out);
            }
            Self::WorkAspired { orb_id, work, name } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&work.0);
                value.extend_from_slice(name.as_bytes());
                tlv::put(94, &value, &mut out);
            }
            Self::WorkClaimed { orb_id, work, witnesses } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&work.0);
                for witness in witnesses {
                    value.extend_from_slice(witness);
                }
                tlv::put(95, &value, &mut out);
            }
            Self::ElderWarded {} => {
                tlv::put(74, &[], &mut out);
            }
            Self::WardLifted { claimer } => {
                tlv::put(75, claimer.as_bytes(), &mut out);
            }
            Self::VeilDrawn {} => {
                tlv::put(76, &[], &mut out);
            }
            Self::VeilLifted { starved } => {
                tlv::put(77, &[u8::from(*starved)], &mut out);
            }
            Self::IncursionStruck { era, world, line } => {
                let mut value = Vec::new();
                for text in [era, world, line] {
                    value.extend_from_slice(&(text.len() as u16).to_le_bytes());
                    value.extend_from_slice(text.as_bytes());
                }
                tlv::put(81, &value, &mut out);
            }
            Self::TaxonSealed { class, clade } => {
                tlv::put(85, &[*class, *clade], &mut out);
            }
            Self::VocabularyMinted { axis, code, sigil, word, binding, base } => {
                let mut value = vec![*axis, *code];
                let mut text = |s: &String| {
                    let bytes = s.as_bytes();
                    value.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                    value.extend_from_slice(bytes);
                };
                text(sigil);
                text(word);
                value.push(binding.len() as u8);
                for (channel, factor) in binding {
                    value.push(*channel);
                    ratio::encode(factor, &mut value);
                }
                // #41: the declared base, trailing and optional — its
                // ABSENCE is the sealed pre-zone form, byte-exact.
                if let Some(base) = base {
                    value.push(*base);
                }
                tlv::put(96, &value, &mut out);
            }
            Self::EstateSurveyed { seed, payer, covered, cost } => {
                let mut value = seed.to_vec();
                value.extend_from_slice(&payer.to_le_bytes());
                value.extend_from_slice(&(covered.len() as u32).to_le_bytes());
                for (orb, witness) in covered {
                    value.extend_from_slice(&orb.to_le_bytes());
                    value.extend_from_slice(witness);
                }
                ratio::encode(cost, &mut value);
                tlv::put(97, &value, &mut out);
            }
            Self::OfferCountered { offer, want_amount, appraisal } => {
                let mut value = offer.to_le_bytes().to_vec();
                match appraisal {
                    Some(digest) => {
                        value.push(1);
                        value.extend_from_slice(digest);
                    }
                    None => value.push(0),
                }
                ratio::encode(want_amount, &mut value);
                tlv::put(99, &value, &mut out);
            }
            Self::LineMinted { seed, ceilings } => {
                let mut value = seed.to_vec();
                if let Some((class, clade)) = ceilings {
                    value.push(*class);
                    value.push(*clade);
                }
                tlv::put(101, &value, &mut out);
            }
            Self::ParcelConveyed {
                from_orb,
                to_orb,
                resource,
                amount,
                postage,
                survey,
                seed,
                covered,
            } => {
                let mut value = from_orb.to_le_bytes().to_vec();
                value.extend_from_slice(&to_orb.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                ratio::encode(postage, &mut value);
                value.extend_from_slice(survey);
                value.extend_from_slice(seed);
                value.extend_from_slice(&(covered.len() as u32).to_le_bytes());
                for orb in covered {
                    value.extend_from_slice(&orb.to_le_bytes());
                }
                tlv::put(100, &value, &mut out);
            }
            Self::AppraisalStruck { subject, survey, seed, covered, value: worth } => {
                let mut value = subject.wire().to_vec();
                value.extend_from_slice(survey);
                value.extend_from_slice(seed);
                value.extend_from_slice(&(covered.len() as u32).to_le_bytes());
                for orb in covered {
                    value.extend_from_slice(&orb.to_le_bytes());
                }
                ratio::encode(worth, &mut value);
                tlv::put(98, &value, &mut out);
            }
            Self::StationSeated { line, station } => {
                let mut value = vec![*station];
                value.extend_from_slice(line.as_bytes());
                tlv::put(89, &value, &mut out);
            }
            Self::DiscoveryMinted { orb_id, resource, amount, work } => {
                let mut value = amount_layout(*orb_id, *resource, amount);
                value.extend_from_slice(work);
                tlv::put(86, &value, &mut out);
            }
            Self::InstrumentForged { orb_id, tech_hash, grade } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(tech_hash);
                value.push(*grade);
                tlv::put(78, &value, &mut out);
            }
            Self::RouteChartered { deal, eta } => {
                let mut value = deal.to_le_bytes().to_vec();
                ratio::encode(eta.value(), &mut value);
                tlv::put(79, &value, &mut out);
            }
            Self::VoidShare { orb_id, deal, resource, amount } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&deal.to_le_bytes());
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                tlv::put(82, &value, &mut out);
            }
            Self::SovereignUnionSealed { progenitor, sovereign, term_stamps } => {
                let mut value = vec![*progenitor as u8, *sovereign as u8];
                value.extend_from_slice(&term_stamps.to_le_bytes());
                tlv::put(90, &value, &mut out);
            }
            Self::RouteLapsed { deal } => {
                tlv::put(80, &deal.to_le_bytes(), &mut out);
            }
        }
        out
    }

    /// Decode one event from its wire record; unknown tags are not an
    /// event (the caller skips them — SEAL A).
    pub fn from_wire(tag: u8, value: &[u8]) -> Option<Self> {
        match tag {
            1 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let name = String::from_utf8(value.get(8..)?.to_vec()).ok()?;
                Some(Self::ColonyFounded { orb_id, name })
            }
            2 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let tile = u16::from_le_bytes(value.get(8..10)?.try_into().ok()?);
                let kind = ZoneKind::from_wire(*value.get(10)?)?;
                // #41: a byte past the kind is the base this build was
                // priced against; the sealed form ends here.
                let base = match value.get(11) {
                    Some(code) => Some(ZoneKind::from_wire(*code)?),
                    None => None,
                };
                Some(Self::ZoneBuilt { orb_id, tile, kind, base })
            }
            3 => {
                let (orb_id, resource, amount) = amount_fields(value)?;
                Some(Self::YieldCollected { orb_id, resource, amount })
            }
            4 => {
                let (orb_id, resource, amount) = amount_fields(value)?;
                Some(Self::UpkeepSettled { orb_id, resource, amount })
            }
            5 => {
                let (orb_id, resource, drawn) = amount_fields(value)?;
                Some(Self::FamineStruck { orb_id, resource, drawn })
            }
            6 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(8)?)?;
                Some(Self::FamineRelieved { orb_id, resource })
            }
            7 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let tile = u16::from_le_bytes(value.get(8..10)?.try_into().ok()?);
                Some(Self::ZoneRazed { orb_id, tile })
            }
            8 => {
                let seed: [u8; 32] = value.get(..32)?.try_into().ok()?;
                Some(Self::WorldCharted { seed })
            }
            10 => {
                let entity_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let principal = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let handle = String::from_utf8(value.get(16..)?.to_vec()).ok()?;
                Some(Self::EntityRegistered { entity_id, principal, handle })
            }
            11 => {
                let agent = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let scope = crate::ipsal::Scope::from_wire(*value.get(8)?)?;
                Some(Self::ScopeGranted { agent, scope })
            }
            12 => {
                let agent = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let scope = crate::ipsal::Scope::from_wire(*value.get(8)?)?;
                Some(Self::ScopeRevoked { agent, scope })
            }
            13 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let source_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let name = String::from_utf8(value.get(16..)?.to_vec()).ok()?;
                Some(Self::ColonyCharted { orb_id, source_orb, name })
            }
            21 => {
                let offset = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let terminal: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let era_len = u16::from_le_bytes(value.get(40..42)?.try_into().ok()?) as usize;
                let era = String::from_utf8(value.get(42..42 + era_len)?.to_vec()).ok()?;
                let world = String::from_utf8(value.get(42 + era_len..)?.to_vec()).ok()?;
                Some(Self::EraAnnexed { era, world, terminal, offset })
            }
            54 => {
                let mut at = 0usize;
                let mut take_text = || -> Option<String> {
                    let len =
                        u16::from_le_bytes(value.get(at..at + 2)?.try_into().ok()?) as usize;
                    at += 2;
                    let text = String::from_utf8(value.get(at..at + len)?.to_vec()).ok()?;
                    at += len;
                    Some(text)
                };
                let observer = take_text()?;
                let creator = take_text()?;
                let name = take_text()?;
                let (x, used) = ratio::decode(value.get(at..)?)?;
                at += used;
                let (y, used) = ratio::decode(value.get(at..)?)?;
                at += used;
                let (z, used) = ratio::decode(value.get(at..)?)?;
                at += used;
                let (reach, _) = ratio::decode(value.get(at..)?)?;
                Some(Self::GalaxyObserved { observer, creator, name, center: (x, y, z), reach })
            }
            22 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (overflow, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::OverheatStruck { orb_id, overflow })
            }
            23 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                Some(Self::OverheatCleared { orb_id })
            }
            24 => Some(Self::StarCommissioned { class: *value.first()? }),
            25 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (albedo, used) = ratio::decode(value.get(8..)?)?;
                let albedo = Unit::new(albedo)?;
                let (orbit_r, used_r) = ratio::decode(value.get(8 + used..)?)?;
                let orbit_r = Positive::new(orbit_r)?;
                // The trailing angle, ABSENT on every receipt written
                // before the amendment. `None` is not `0`: zero would
                // put all of history on one ray.
                let tail = value.get(8 + used + used_r..).unwrap_or(&[]);
                // FORM AT THE DOOR (29_CONSTRUCTION C1): an off-slot
                // angle refuses at DECODE — the codec fixes form — so a
                // `Slot` past this point is proven and nothing
                // downstream re-checks it. `OffSlot` is dead.
                let (theta, used_t) = match ratio::decode(tail) {
                    Some((turn, n)) => (Some(Slot::from_angle(turn)?), n),
                    None => (None, 0),
                };
                let psi = match tail.get(used_t..).filter(|rest| !rest.is_empty()) {
                    Some(rest) => {
                        let (turn, _) = ratio::decode(rest)?;
                        Some(Slot::from_angle(turn)?)
                    }
                    None => None,
                };
                Some(Self::PlanetSited { orb_id, albedo, orbit_r, theta, psi })
            }
            27 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::VentSettled { orb_id, amount })
            }
            29 => {
                let (orb_id, resource, amount) = amount_fields(value)?;
                Some(Self::DemurrageSettled { orb_id, resource, amount })
            }
            102 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let count = *value.get(8)? as usize;
                let mut at = 9usize;
                let mut decays = Vec::with_capacity(count);
                for _ in 0..count {
                    let resource = Resource::from_wire(*value.get(at)?)?;
                    at += 1;
                    let (amount, used) = ratio::decode(value.get(at..)?)?;
                    at += used;
                    decays.push((resource, amount));
                }
                // Refuse trailing bytes — never a guess.
                (at == value.len()).then_some(Self::DemurrageTouched { orb_id, decays })
            }
            103 => {
                let key = *value.first()?;
                // The key must NAME a parameter — refuse, never guess.
                crate::parameter::Parameter::from_code(key)?;
                let (chartered, used) = ratio::decode(value.get(1..)?)?;
                (1 + used == value.len())
                    .then_some(Self::ParameterChartered { key, value: chartered })
            }
            105 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let kind = *value.get(8)?;
                // The kind must NAME a pact — refuse, never guess.
                crate::treaty::TreatyKind::from_wire(kind)?;
                let term = u64::from_le_bytes(value.get(9..17)?.try_into().ok()?);
                let count = *value.get(17)? as usize;
                let mut at = 18usize;
                let mut terms = Vec::with_capacity(count);
                for _ in 0..count {
                    let channel = *value.get(at)?;
                    at += 1;
                    let (factor, used) = ratio::decode(value.get(at..)?)?;
                    at += used;
                    terms.push((channel, factor));
                }
                let partner = String::from_utf8(value.get(at..)?.to_vec()).ok()?;
                Some(Self::TreatySealed { deal, kind, partner, terms, term })
            }
            106 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let broken = *value.get(8)? != 0;
                let partner = String::from_utf8(value.get(9..)?.to_vec()).ok()?;
                Some(Self::TreatyStruck { deal, partner, broken })
            }
            104 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let gem: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let rest = value.get(40..)?;
                // A partial identity is not a member — refuse, never
                // guess (the DemurrageTouched discipline).
                if rest.is_empty() || rest.len() % 32 != 0 {
                    return None;
                }
                let members: Vec<[u8; 32]> = rest
                    .chunks_exact(32)
                    .map(|chunk| chunk.try_into().expect("chunks_exact(32)"))
                    .collect();
                Some(Self::GemCompounded { orb_id, members, gem })
            }
            107 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let raw: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let refined: [u8; 32] = value.get(40..72)?.try_into().ok()?;
                let (spent, used) = ratio::decode(value.get(72..)?)?;
                let (yielded, _) = ratio::decode(value.get(72 + used..)?)?;
                Some(Self::ElementRefined {
                    orb_id,
                    raw,
                    refined,
                    spent,
                    yielded,
                })
            }
            30 => {
                let from_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let to_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (amount, _) = ratio::decode(value.get(17..)?)?;
                Some(Self::ResourceTransferred { from_orb, to_orb, resource, amount })
            }
            31 => {
                let source_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let absorber_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(16..)?)?;
                Some(Self::OverflowAbsorbed { source_orb, absorber_orb, amount })
            }
            32 => {
                let debtor_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let creditor_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(16..)?)?;
                Some(Self::AbsorptionLapsed { debtor_orb, creditor_orb, amount })
            }
            33 => {
                let source_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let target_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (rate, used) = ratio::decode(value.get(17..)?)?;
                let term =
                    u64::from_le_bytes(value.get(17 + used..17 + used + 8)?.try_into().ok()?);
                Some(Self::BondSealed { source_orb, target_orb, resource, rate, term })
            }
            34 => {
                let source_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let target_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (amount, used) = ratio::decode(value.get(17..)?)?;
                let severed = *value.get(17 + used)? != 0;
                let (load, _) = ratio::decode(value.get(18 + used..)?)?;
                Some(Self::BondSettled { source_orb, target_orb, resource, amount, severed, load })
            }
            35 => {
                let obligor_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let beneficiary_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (quota, used) = ratio::decode(value.get(17..)?)?;
                let window =
                    u64::from_le_bytes(value.get(17 + used..25 + used)?.try_into().ok()?);
                let (collateral, _) = ratio::decode(value.get(25 + used..)?)?;
                Some(Self::ForwardSealed {
                    obligor_orb,
                    beneficiary_orb,
                    resource,
                    quota,
                    window,
                    collateral,
                })
            }
            36 => {
                let (obligor_orb, beneficiary_orb, resource, collateral) = pact_fields(value)?;
                Some(Self::ForwardReleased { obligor_orb, beneficiary_orb, resource, collateral })
            }
            37 => {
                let (obligor_orb, beneficiary_orb, resource, collateral) = pact_fields(value)?;
                Some(Self::ForwardSlashed { obligor_orb, beneficiary_orb, resource, collateral })
            }
            38 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::FrictionSettled { orb_id, amount })
            }
            40 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let from_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let council = u64::from_le_bytes(value.get(16..24)?.try_into().ok()?);
                let condition = *value.get(24)?;
                let node: [u8; 32] = value.get(25..57)?.try_into().ok()?;
                let name = String::from_utf8(value.get(57..)?.to_vec()).ok()?;
                Some(Self::TechLicensed { deal, from_orb, council, condition, node, name })
            }
            52 => {
                let council = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(8)?)?;
                let (quota, used) = ratio::decode(value.get(9..)?)?;
                let mut at = 9 + used;
                let window = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
                at += 8;
                let (collateral, used) = ratio::decode(value.get(at..)?)?;
                at += used;
                let rest = value.get(at..)?;
                if rest.len() % 8 != 0 {
                    return None;
                }
                let members = rest
                    .chunks_exact(8)
                    .map(|chunk| u64::from_le_bytes(chunk.try_into().expect("8 bytes")))
                    .collect();
                Some(Self::CouncilSealed { council, resource, quota, window, collateral, members })
            }
            53 => {
                let council = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let member = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let fulfilled = *value.get(16)? != 0;
                let (banked, _) = ratio::decode(value.get(17..)?)?;
                Some(Self::CouncilClosed { council, member, fulfilled, banked })
            }
            42 => {
                let (gauge, _) = ratio::decode(value)?;
                Some(Self::GaugeShifted { gauge })
            }
            60 => {
                let grade = u16::from_le_bytes(value.get(..2)?.try_into().ok()?);
                let (gauge, _) = ratio::decode(value.get(2..)?)?;
                Some(Self::GaugeShiftedAt { grade, gauge })
            }
            62 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let kind = *value.get(8)?;
                let (reading, _) = ratio::decode(value.get(9..)?)?;
                Some(Self::WantStruck { orb_id, kind, reading })
            }
            63 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let kind = *value.get(8)?;
                Some(Self::WantRelieved { orb_id, kind })
            }
            67 => {
                let from = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let to = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                Some(Self::HandoffSealed { from, to })
            }
            49 => {
                let offer = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let give = Resource::from_wire(*value.get(16)?)?;
                let (give_amount, used) = ratio::decode(value.get(17..)?)?;
                let at = 17 + used;
                let want = Resource::from_wire(*value.get(at)?)?;
                let (want_amount, _) = ratio::decode(value.get(at + 1..)?)?;
                Some(Self::OfferPosted { offer, orb_id, give, give_amount, want, want_amount })
            }
            50 => {
                let offer = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let deal = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                Some(Self::OfferCleared { offer, deal })
            }
            43 => {
                let (gauge, used) = ratio::decode(value)?;
                let branch = String::from_utf8(value.get(used..)?.to_vec()).ok()?;
                Some(Self::Diverged { branch, gauge })
            }
            44 => {
                let (gauge, used) = ratio::decode(value)?;
                let partner = String::from_utf8(value.get(used..)?.to_vec()).ok()?;
                Some(Self::Confluent { partner, gauge })
            }
            45 => Some(Self::StarAged { class: *value.first()? }),
            46 => {
                let era = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (gauge, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::EraFolded { era, gauge })
            }
            39 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let node: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let condition = *value.get(40)?;
                let modifier = *value.get(41)?;
                let (magnitude, used) = ratio::decode(value.get(42..)?)?;
                let name = String::from_utf8(value.get(42 + used..)?.to_vec()).ok()?;
                Some(Self::TechDiscovered { orb_id, node, condition, modifier, magnitude, name })
            }
            47 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let from_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (amount, _) = ratio::decode(value.get(17..)?)?;
                Some(Self::ApertureSent { deal, from_orb, resource, amount })
            }
            48 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let to_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (amount, _) = ratio::decode(value.get(17..)?)?;
                Some(Self::ApertureReceived { deal, to_orb, resource, amount })
            }
            108 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let from_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let element: [u8; 32] = value.get(16..48)?.try_into().ok()?;
                let (amount, _) = ratio::decode(value.get(48..)?)?;
                Some(Self::ElementApertureSent {
                    deal,
                    from_orb,
                    element,
                    amount,
                })
            }
            109 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let to_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let element: [u8; 32] = value.get(16..48)?.try_into().ok()?;
                let (amount, _) = ratio::decode(value.get(48..)?)?;
                Some(Self::ElementApertureReceived {
                    deal,
                    to_orb,
                    element,
                    amount,
                })
            }
            110 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let element: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let (eta_r, used) = ratio::decode(value.get(40..)?)?;
                let eta = Unit::new(eta_r)?;
                let rail = *value.get(40 + used)?;
                Rail::from_wire(rail)?;
                Some(Self::ElementRouteChartered {
                    deal,
                    element,
                    eta,
                    rail,
                })
            }
            111 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, used) = ratio::decode(value.get(8..)?)?;
                let (crystal_spent, _) = ratio::decode(value.get(8 + used..)?)?;
                Some(Self::StampMinted {
                    orb_id,
                    amount,
                    crystal_spent,
                })
            }
            112 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let from_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(16..)?)?;
                Some(Self::StampApertureSent {
                    deal,
                    from_orb,
                    amount,
                })
            }
            113 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let to_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(16..)?)?;
                Some(Self::StampApertureReceived {
                    deal,
                    to_orb,
                    amount,
                })
            }
            114 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, used) = ratio::decode(value.get(8..)?)?;
                let (iron_spent, _) = ratio::decode(value.get(8 + used..)?)?;
                Some(Self::ScrapSalvaged {
                    orb_id,
                    amount,
                    iron_spent,
                })
            }
            115 => {
                let hull_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let role = *value.get(16)?;
                let grade = *value.get(17)?;
                let (scrap_spent, _) = ratio::decode(value.get(18..)?)?;
                Some(Self::HullMinted {
                    hull_id,
                    orb_id,
                    role,
                    grade,
                    scrap_spent,
                })
            }
            116 => {
                let hull_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let deal = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                Some(Self::HullAssigned { hull_id, deal })
            }
            117 => {
                let hull_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (scrap_returned, _) = ratio::decode(value.get(16..)?)?;
                Some(Self::HullScrapped {
                    hull_id,
                    orb_id,
                    scrap_returned,
                })
            }
            118 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let kind = *value.get(8)?;
                let (entropy, used) = ratio::decode(value.get(9..)?)?;
                let (syntropy, _) = ratio::decode(value.get(9 + used..)?)?;
                Some(Self::OrderWorked {
                    orb_id,
                    kind,
                    entropy,
                    syntropy,
                })
            }
            119 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let element: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let (amount, _) = ratio::decode(value.get(40..)?)?;
                Some(Self::ElementSalvaged {
                    orb_id,
                    element,
                    amount,
                })
            }
            120 => {
                let note_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let issuer = u64::from_le_bytes(value.get(16..24)?.try_into().ok()?);
                let holder = u64::from_le_bytes(value.get(24..32)?.try_into().ok()?);
                let (notional, u1) = ratio::decode(value.get(32..)?)?;
                let (collateral, u2) = ratio::decode(value.get(32 + u1..)?)?;
                let at = 32 + u1 + u2;
                let maturity = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
                Some(Self::NoteIssued {
                    note_id,
                    orb_id,
                    issuer,
                    holder,
                    notional,
                    collateral,
                    maturity,
                })
            }
            121 => {
                let note_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (paid, u1) = ratio::decode(value.get(16..)?)?;
                let (seized, _) = ratio::decode(value.get(16 + u1..)?)?;
                Some(Self::NoteRedeemed {
                    note_id,
                    orb_id,
                    paid,
                    seized,
                })
            }
            122 => {
                let fut_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let long = u64::from_le_bytes(value.get(16..24)?.try_into().ok()?);
                let short = u64::from_le_bytes(value.get(24..32)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(32)?)?;
                let (notional, u1) = ratio::decode(value.get(33..)?)?;
                let (price, u2) = ratio::decode(value.get(33 + u1..)?)?;
                let (margin, u3) = ratio::decode(value.get(33 + u1 + u2..)?)?;
                let at = 33 + u1 + u2 + u3;
                let expiry = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
                Some(Self::FutureOpened {
                    fut_id,
                    orb_id,
                    long,
                    short,
                    resource,
                    notional,
                    price,
                    margin,
                    expiry,
                })
            }
            123 => {
                let fut_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (mark, u1) = ratio::decode(value.get(16..)?)?;
                let (long_pnl, u2) = ratio::decode(value.get(16 + u1..)?)?;
                let (short_pnl, _) = ratio::decode(value.get(16 + u1 + u2..)?)?;
                Some(Self::FutureSettled {
                    fut_id,
                    orb_id,
                    mark,
                    long_pnl,
                    short_pnl,
                })
            }
            124 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(8)?)?;
                let (price, _) = ratio::decode(value.get(9..)?)?;
                Some(Self::LastClearMarked {
                    orb_id,
                    resource,
                    price,
                })
            }
            125 => {
                let share_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let holder = u64::from_le_bytes(value.get(16..24)?.try_into().ok()?);
                let stream = Resource::from_wire(*value.get(24)?)?;
                let (units, u1) = ratio::decode(value.get(25..)?)?;
                let (par, _) = ratio::decode(value.get(25 + u1..)?)?;
                Some(Self::ShareIssued {
                    share_id,
                    orb_id,
                    holder,
                    stream,
                    units,
                    par,
                })
            }
            126 => {
                let share_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let from = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let to = u64::from_le_bytes(value.get(16..24)?.try_into().ok()?);
                let (units, _) = ratio::decode(value.get(24..)?)?;
                Some(Self::ShareTransferred {
                    share_id,
                    from,
                    to,
                    units,
                })
            }
            127 => {
                let share_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let stream = Resource::from_wire(*value.get(16)?)?;
                let (resource_cut, u1) = ratio::decode(value.get(17..)?)?;
                let (amount, _) = ratio::decode(value.get(17 + u1..)?)?;
                Some(Self::ShareDividend {
                    share_id,
                    orb_id,
                    stream,
                    resource_cut,
                    amount,
                })
            }
            128 => {
                let perp_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let long = u64::from_le_bytes(value.get(16..24)?.try_into().ok()?);
                let short = u64::from_le_bytes(value.get(24..32)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(32)?)?;
                let (notional, u1) = ratio::decode(value.get(33..)?)?;
                let (entry, u2) = ratio::decode(value.get(33 + u1..)?)?;
                let (margin, _) = ratio::decode(value.get(33 + u1 + u2..)?)?;
                Some(Self::PerpOpened {
                    perp_id,
                    orb_id,
                    long,
                    short,
                    resource,
                    notional,
                    entry,
                    margin,
                })
            }
            129 => {
                let perp_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (funding, u1) = ratio::decode(value.get(16..)?)?;
                let (mark, _) = ratio::decode(value.get(16 + u1..)?)?;
                Some(Self::PerpFunded {
                    perp_id,
                    orb_id,
                    funding,
                    mark,
                })
            }
            130 => {
                let perp_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let (mark, u1) = ratio::decode(value.get(16..)?)?;
                let (long_pnl, u2) = ratio::decode(value.get(16 + u1..)?)?;
                let liquidated = *value.get(16 + u1 + u2)? != 0;
                Some(Self::PerpClosed {
                    perp_id,
                    orb_id,
                    mark,
                    long_pnl,
                    liquidated,
                })
            }
            131 => {
                let hull_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let note_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(16..24)?.try_into().ok()?);
                let (scrap_spent, u1) = ratio::decode(value.get(24..)?)?;
                let (note_face, _) = ratio::decode(value.get(24 + u1..)?)?;
                Some(Self::HullFinanced {
                    hull_id,
                    note_id,
                    orb_id,
                    scrap_spent,
                    note_face,
                })
            }
            132 => {
                let hull_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let deal = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let grade = *value.get(16)?;
                Some(Self::EscortOnDuty {
                    hull_id,
                    deal,
                    grade,
                })
            }
            133 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let orb_id = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (amount, u1) = ratio::decode(value.get(17..)?)?;
                let (risk, _) = ratio::decode(value.get(17 + u1..)?)?;
                Some(Self::InterdictionStruck {
                    deal,
                    orb_id,
                    resource,
                    amount,
                    risk,
                })
            }
            134 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (risk, u1) = ratio::decode(value.get(8..)?)?;
                let (escort_cover, _) = ratio::decode(value.get(8 + u1..)?)?;
                Some(Self::ConvoyRiskMarked {
                    deal,
                    risk,
                    escort_cover,
                })
            }
            135 => {
                let resume_at = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                Some(Self::MarketSealed { resume_at })
            }
            136 => Some(Self::MarketResumed {}),
            137 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(8)?)?;
                let (price, u1) = ratio::decode(value.get(9..)?)?;
                let at = 9 + u1;
                let depth = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
                Some(Self::SpotClearRecorded {
                    orb_id,
                    resource,
                    price,
                    depth,
                })
            }
            138 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (fee, u1) = ratio::decode(value.get(8..)?)?;
                let offers_voided = *value.get(8 + u1)?;
                Some(Self::LiquidityRaidStruck {
                    orb_id,
                    fee,
                    offers_voided,
                })
            }
            139 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(8)?)?;
                let (price, u1) = ratio::decode(value.get(9..)?)?;
                let at = 9 + u1;
                let depth = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
                let need = u64::from_le_bytes(value.get(at + 8..at + 16)?.try_into().ok()?);
                Some(Self::ClearPoisonRefused {
                    orb_id,
                    resource,
                    price,
                    depth,
                    need,
                })
            }
            140 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (fee, u1) = ratio::decode(value.get(8..)?)?;
                let at = 8 + u1;
                let pressure = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
                Some(Self::NoteCascadeStruck {
                    orb_id,
                    fee,
                    pressure,
                })
            }
            141 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (cost, u1) = ratio::decode(value.get(8..)?)?;
                let (pressure, _) = ratio::decode(value.get(8 + u1..)?)?;
                Some(Self::CovertTamperStruck {
                    orb_id,
                    cost,
                    pressure,
                })
            }
            142 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (cost, u1) = ratio::decode(value.get(8..)?)?;
                let at = 8 + u1;
                let (pressure, u2) = ratio::decode(value.get(at..)?)?;
                let rest = at + u2;
                // Additive: older frames omit resource → Iron (P5-D default).
                let resource = value
                    .get(rest)
                    .copied()
                    .and_then(Resource::from_wire)
                    .unwrap_or(Resource::Iron);
                Some(Self::MediumReinforced {
                    orb_id,
                    cost,
                    pressure,
                    resource,
                })
            }
            143 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (pressure, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::TamperDetected { orb_id, pressure })
            }
            144 => {
                let disposition = *value.first()?;
                let perception = *value.get(1)?;
                let primary = *value.get(2)?;
                Some(Self::PostureReoriented {
                    disposition,
                    perception,
                    primary,
                })
            }
            145 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let depth = *value.get(8)?;
                let (fee, _) = ratio::decode(value.get(9..)?)?;
                Some(Self::SpyProbed {
                    orb_id,
                    depth,
                    fee,
                })
            }
            146 => {
                let condition = *value.first()?;
                let tech_hash: [u8; 32] = value.get(1..33)?.try_into().ok()?;
                let (fee, _) = ratio::decode(value.get(33..)?)?;
                Some(Self::TechExfiltrated {
                    condition,
                    tech_hash,
                    fee,
                })
            }
            147 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let grade = *value.get(8)?;
                let tech_hash: [u8; 32] = value.get(9..41)?.try_into().ok()?;
                Some(Self::InstrumentSabotaged {
                    orb_id,
                    grade,
                    tech_hash,
                })
            }
            148 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let from_rail = *value.get(8)?;
                let to_rail = *value.get(9)?;
                Rail::from_wire(from_rail)?;
                Rail::from_wire(to_rail)?;
                Some(Self::RailPromoted {
                    deal,
                    from_rail,
                    to_rail,
                })
            }
            149 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, u1) = ratio::decode(value.get(8..)?)?;
                let at = 8 + u1;
                let claim_hash: [u8; 32] = value.get(at..at + 32)?.try_into().ok()?;
                let kind = *value.get(at + 32)?;
                Some(Self::StrandExported {
                    orb_id,
                    amount,
                    claim_hash,
                    kind,
                })
            }
            150 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, u1) = ratio::decode(value.get(8..)?)?;
                let at = 8 + u1;
                let claim_hash: [u8; 32] = value.get(at..at + 32)?.try_into().ok()?;
                let kind = *value.get(at + 32)?;
                Some(Self::AnchorImported {
                    orb_id,
                    amount,
                    claim_hash,
                    kind,
                })
            }
            151 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let reason = *value.get(8)?;
                Some(Self::AnchorImportRefused { orb_id, reason })
            }
            152 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::VaultDeposited { orb_id, amount })
            }
            153 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::VaultWithdrawn { orb_id, amount })
            }
            154 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let element: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let matched = *value.get(40)? != 0;
                Some(Self::AssayConsilience {
                    orb_id,
                    element,
                    matched,
                })
            }
            155 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let element: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let (slash, u1) = ratio::decode(value.get(40..)?)?;
                let (richness_after, _) = ratio::decode(value.get(40 + u1..)?)?;
                Some(Self::AssaySlashed {
                    orb_id,
                    element,
                    slash,
                    richness_after,
                })
            }
            156 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let a: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let b: [u8; 32] = value.get(40..72)?.try_into().ok()?;
                let product: [u8; 32] = value.get(72..104)?.try_into().ok()?;
                let knowledge = u16::from_le_bytes(value.get(104..106)?.try_into().ok()?);
                Some(Self::IsoUnionStruck {
                    orb_id,
                    a,
                    b,
                    product,
                    knowledge,
                })
            }
            157 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let gem: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let knowledge = u16::from_le_bytes(value.get(40..42)?.try_into().ok()?);
                let (survival, _) = ratio::decode(value.get(42..)?)?;
                Some(Self::ChemCataloged {
                    orb_id,
                    gem,
                    knowledge,
                    survival,
                })
            }
            158 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let pull = *value.get(8)?;
                let (weight, u1) = ratio::decode(value.get(9..)?)?;
                let (dark_score, _) = ratio::decode(value.get(9 + u1..)?)?;
                Some(Self::DarkPullStruck {
                    orb_id,
                    pull,
                    weight,
                    dark_score,
                })
            }
            159 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let discipline = *value.get(8)?;
                let rank = *value.get(9)?;
                let (fee, _) = ratio::decode(value.get(10..)?)?;
                Some(Self::ScienceSeated {
                    orb_id,
                    discipline,
                    rank,
                    fee,
                })
            }
            160 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let path = *value.get(8)?;
                let (risk, u1) = ratio::decode(value.get(9..)?)?;
                let base = 9 + u1;
                let exploded = *value.get(base)? != 0;
                let explosion_kind = *value.get(base + 1)?;
                let hidden_pos = *value.get(base + 2)? != 0;
                let hidden_neg = *value.get(base + 3)? != 0;
                let (fee, _) = ratio::decode(value.get(base + 4..)?)?;
                Some(Self::LabCreated {
                    orb_id,
                    path,
                    risk,
                    exploded,
                    explosion_kind,
                    hidden_pos,
                    hidden_neg,
                    fee,
                })
            }
            161 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let path = *value.get(8)?;
                let kind = *value.get(9)?;
                let resource = *value.get(10)?;
                let (slash, _) = ratio::decode(value.get(11..)?)?;
                Some(Self::ScienceExploded {
                    orb_id,
                    path,
                    kind,
                    resource,
                    slash,
                })
            }
            162 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let pressure = *value.get(8)?;
                Some(Self::RogueFactionStruck { orb_id, pressure })
            }
            55 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (amount, _) = ratio::decode(value.get(8..)?)?;
                Some(Self::CrystalDrawn { orb_id, amount })
            }
            59 => {
                let successor = String::from_utf8(value.to_vec()).ok()?;
                Some(Self::LineSucceeded { successor })
            }
            61 => {
                let disposition = *value.first()?;
                let metabolism = *value.get(1)?;
                let perception = *value.get(2)?;
                let civilization = String::from_utf8(value.get(3..)?.to_vec()).ok()?;
                Some(Self::GenesisSealed { civilization, disposition, metabolism, perception })
            }
            68 => Some(Self::PracticeAdopted { practice: *value.first()? }),
            69 => Some(Self::PracticeReleased { practice: *value.first()? }),
            70 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let stratum = *value.get(8)?;
                let element: [u8; 32] = value.get(9..41)?.try_into().ok()?;
                let family = *value.get(41)?;
                let tier = *value.get(42)?;
                let (richness, used) = ratio::decode(value.get(43..)?)?;
                let name = String::from_utf8(value.get(43 + used..)?.to_vec()).ok()?;
                Some(Self::ProspectStruck { orb_id, stratum, element, family, tier, richness, name })
            }
            71 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let element: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let (amount, used) = ratio::decode(value.get(40..)?)?;
                // ECO-1: bytes past the amount are the after-richness;
                // the sealed 3-field form ends exactly here.
                let richness = match value.len() > 40 + used {
                    true => Some(ratio::decode(value.get(40 + used..)?)?.0),
                    false => None,
                };
                Some(Self::AssayStruck { orb_id, element, amount, richness })
            }
            91 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let kind: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let name = String::from_utf8(value.get(40..)?.to_vec()).ok()?;
                Some(Self::BookMinted { orb_id, kind: BookKind(kind), name })
            }
            92 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let kind: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let (kept, used) = ratio::decode(value.get(40..)?)?;
                let (lost, _) = ratio::decode(value.get(40 + used..)?)?;
                Some(Self::BookPosted { orb_id, kind: BookKind(kind), kept, lost })
            }
            93 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let kind: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                Some(Self::BookArchived { orb_id, kind: BookKind(kind) })
            }
            94 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let work: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let name = String::from_utf8(value.get(40..)?.to_vec()).ok()?;
                Some(Self::WorkAspired { orb_id, work: WorkId(work), name })
            }
            95 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let work: [u8; 32] = value.get(8..40)?.try_into().ok()?;
                let rest = value.get(40..)?;
                if rest.len() % 32 != 0 {
                    return None;
                }
                let witnesses =
                    rest.chunks(32).map(|c| c.try_into().ok()).collect::<Option<Vec<_>>>()?;
                Some(Self::WorkClaimed { orb_id, work: WorkId(work), witnesses })
            }
            74 => Some(Self::ElderWarded {}),
            75 => Some(Self::WardLifted {
                claimer: String::from_utf8(value.to_vec()).ok()?,
            }),
            76 => Some(Self::VeilDrawn {}),
            77 => Some(Self::VeilLifted { starved: *value.first()? != 0 }),
            81 => {
                let mut at = 0usize;
                let mut take_text = || -> Option<String> {
                    let len =
                        u16::from_le_bytes(value.get(at..at + 2)?.try_into().ok()?) as usize;
                    at += 2;
                    let text = String::from_utf8(value.get(at..at + len)?.to_vec()).ok()?;
                    at += len;
                    Some(text)
                };
                let era = take_text()?;
                let world = take_text()?;
                let line = take_text()?;
                Some(Self::IncursionStruck { era, world, line })
            }
            85 => Some(Self::TaxonSealed {
                class: *value.first()?,
                clade: *value.get(1)?,
            }),
            96 => {
                let axis = *value.first()?;
                let code = *value.get(1)?;
                let mut at = 2usize;
                let take_text = |at: &mut usize| -> Option<String> {
                    let len =
                        u16::from_le_bytes(value.get(*at..*at + 2)?.try_into().ok()?) as usize;
                    *at += 2;
                    let text = String::from_utf8(value.get(*at..*at + len)?.to_vec()).ok()?;
                    *at += len;
                    Some(text)
                };
                let sigil = take_text(&mut at)?;
                let word = take_text(&mut at)?;
                let count = *value.get(at)? as usize;
                at += 1;
                let mut binding = Vec::with_capacity(count);
                for _ in 0..count {
                    let channel = *value.get(at)?;
                    at += 1;
                    let (factor, used) = ratio::decode(value.get(at..)?)?;
                    at += used;
                    binding.push((channel, factor));
                }
                // #41: a byte past the binding is the declared base;
                // the sealed pre-zone form ends exactly here.
                let base = value.get(at).copied();
                Some(Self::VocabularyMinted { axis, code, sigil, word, binding, base })
            }
            97 => {
                let seed: [u8; 32] = value.get(..32)?.try_into().ok()?;
                let payer = u64::from_le_bytes(value.get(32..40)?.try_into().ok()?);
                let count =
                    u32::from_le_bytes(value.get(40..44)?.try_into().ok()?) as usize;
                let mut at = 44usize;
                let mut covered = Vec::with_capacity(count);
                for _ in 0..count {
                    let orb = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
                    at += 8;
                    let witness: [u8; 32] = value.get(at..at + 32)?.try_into().ok()?;
                    at += 32;
                    covered.push((orb, witness));
                }
                let (cost, _) = ratio::decode(value.get(at..)?)?;
                Some(Self::EstateSurveyed { seed, payer, covered, cost })
            }
            98 => {
                let subject = Holding::from_wire(value.get(..11)?)?;
                let survey: [u8; 32] = value.get(11..43)?.try_into().ok()?;
                let seed: [u8; 32] = value.get(43..75)?.try_into().ok()?;
                let count =
                    u32::from_le_bytes(value.get(75..79)?.try_into().ok()?) as usize;
                let mut at = 79usize;
                let mut covered = Vec::with_capacity(count);
                for _ in 0..count {
                    covered.push(u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?));
                    at += 8;
                }
                let (worth, _) = ratio::decode(value.get(at..)?)?;
                Some(Self::AppraisalStruck { subject, survey, seed, covered, value: worth })
            }
            99 => {
                let offer = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (appraisal, at) = match value.get(8)? {
                    0 => (None, 9usize),
                    1 => (Some(value.get(9..41)?.try_into().ok()?), 41usize),
                    _ => return None,
                };
                let (want_amount, _) = ratio::decode(value.get(at..)?)?;
                Some(Self::OfferCountered { offer, want_amount, appraisal })
            }
            100 => {
                let from_orb = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let to_orb = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let mut at = 17usize;
                let (amount, used) = ratio::decode(value.get(at..)?)?;
                at += used;
                let (postage, used) = ratio::decode(value.get(at..)?)?;
                at += used;
                let survey: [u8; 32] = value.get(at..at + 32)?.try_into().ok()?;
                at += 32;
                let seed: [u8; 32] = value.get(at..at + 32)?.try_into().ok()?;
                at += 32;
                let count =
                    u32::from_le_bytes(value.get(at..at + 4)?.try_into().ok()?) as usize;
                at += 4;
                let mut covered = Vec::with_capacity(count);
                for _ in 0..count {
                    covered.push(u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?));
                    at += 8;
                }
                Some(Self::ParcelConveyed {
                    from_orb,
                    to_orb,
                    resource,
                    amount,
                    postage,
                    survey,
                    seed,
                    covered,
                })
            }
            101 => {
                let seed: [u8; 32] = value.get(..32)?.try_into().ok()?;
                let ceilings = match value.len() {
                    32 => None,
                    34 => Some((*value.get(32)?, *value.get(33)?)),
                    _ => return None, // refuse, never guess
                };
                Some(Self::LineMinted { seed, ceilings })
            }
            89 => {
                let station = *value.first()?;
                // Wave-1: offices 1..=10 (progenitor…anchor). Refuse
                // unknown rather than guess.
                Station::from_wire(station)?;
                Some(Self::StationSeated {
                    line: String::from_utf8(value.get(1..)?.to_vec()).ok()?,
                    station,
                })
            }
            86 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(8)?)?;
                let (amount, used) = ratio::decode(value.get(9..)?)?;
                // The remainder is the work payload, opaque here by
                // law — solver-verify owns its form and refuses its
                // breaks; the TLV length bounds it.
                let work = value.get(9 + used..)?.to_vec();
                Some(Self::DiscoveryMinted { orb_id, resource, amount, work })
            }
            78 => Some(Self::InstrumentForged {
                orb_id: u64::from_le_bytes(value.get(..8)?.try_into().ok()?),
                tech_hash: value.get(8..40)?.try_into().ok()?,
                grade: *value.get(40)?,
            }),
            79 => {
                let deal = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let (eta, _) = ratio::decode(value.get(8..)?)?;
                let eta = Unit::new(eta)?;
                Some(Self::RouteChartered { deal, eta })
            }
            80 => Some(Self::RouteLapsed {
                deal: u64::from_le_bytes(value.get(..8)?.try_into().ok()?),
            }),
            90 => {
                let progenitor = Office::from_wire(*value.first()?)?;
                let sovereign = Office::from_wire(*value.get(1)?)?;
                let term_stamps = u64::from_le_bytes(value.get(2..10)?.try_into().ok()?);
                Some(Self::SovereignUnionSealed { progenitor, sovereign, term_stamps })
            }
            82 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let deal = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(16)?)?;
                let (amount, _) = ratio::decode(value.get(17..)?)?;
                Some(Self::VoidShare { orb_id, deal, resource, amount })
            }
            _ => None,
        }
    }
}

/// The shared `LE64 ‖ LE64 ‖ resource u8 ‖ ratio` pact layout (36–37).
fn pact_layout(obligor: u64, beneficiary: u64, resource: Resource, collateral: &Ratio) -> Vec<u8> {
    let mut value = obligor.to_le_bytes().to_vec();
    value.extend_from_slice(&beneficiary.to_le_bytes());
    value.push(resource as u8);
    ratio::encode(collateral, &mut value);
    value
}

fn pact_fields(value: &[u8]) -> Option<(u64, u64, Resource, Ratio)> {
    let obligor = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
    let beneficiary = u64::from_le_bytes(value.get(8..16)?.try_into().ok()?);
    let resource = Resource::from_wire(*value.get(16)?)?;
    let (collateral, _) = ratio::decode(value.get(17..)?)?;
    Some((obligor, beneficiary, resource, collateral))
}

/// The shared `LE64(orb_id) ‖ resource u8 ‖ ratio` layout (tags 3–5).
fn amount_layout(orb_id: u64, resource: Resource, amount: &Ratio) -> Vec<u8> {
    let mut value = orb_id.to_le_bytes().to_vec();
    value.push(resource as u8);
    ratio::encode(amount, &mut value);
    value
}

fn amount_fields(value: &[u8]) -> Option<(u64, Resource, Ratio)> {
    let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
    let resource = Resource::from_wire(*value.get(8)?)?;
    let (amount, _) = ratio::decode(value.get(9..)?)?;
    Some((orb_id, resource, amount))
}

/// THERMAL-1 (05 §1.1): a colony's terms in the equilibrium baseline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThermalSite {
    /// Albedo A, 0 ≤ A < 1.
    pub albedo: Unit,
    /// Orbit radius r > 0, in the star's units.
    pub orbit_r: Positive,
    /// THE ANGLE — a fraction of a TURN, `θ ∈ [0, 1) ⊂ ℚ`.
    ///
    /// **Turns, never radians.** A full turn in radians is `2π`, which
    /// is irrational, so a rational angle in radians cannot express a
    /// wrap. In turns the wrap is exact: `θ ≡ θ (mod 1)`.
    ///
    /// **`None` on every receipt written before this amendment.**
    /// Defaulting to `0` would place every historical orb on one ray —
    /// all collinear — which is a spatial claim nobody made. An orb
    /// without an angle has a radius and no place.
    ///
    /// Adjacency is WRAPPED L1 on `(r, θ, ψ)`, never Euclidean: plane
    /// distance needs `cos` and a square root and both are barred
    /// (`24_FORM` §2b). See [`separation`].
    pub theta: Option<Slot>,
    /// **ψ — THE INCLINATION**, the second turn, also `[0, 1) ⊂ ℚ`.
    ///
    /// `θ` alone confines every orb to ONE PLANE: a disc around the
    /// star. `ψ` lifts it out, and `(r, θ, ψ)` is a place in a VOLUME
    /// rather than on a sheet. That is what lets `Ω` range over the
    /// galaxy instead of a single orbital plane.
    ///
    /// Three wrapped coordinates is also the shape the chain has
    /// carried all along — `arbor::Coordinate` holds `s: [Ratio; 3]`
    /// (`19_KERNEL` §3b measured it as an address with no place under
    /// it). With `ψ` the orb finally inhabits the triple the
    /// commitment already addresses.
    ///
    /// `None` on every frame written before this amendment, and `None`
    /// means UNINCLINED — not the equator, which `Some(0)` means.
    pub psi: Option<Slot>,
}

/// The colony orb — COMMITTED state only. Unreceipted accrual is
/// workspace (it lives on the world, not the orb) and never encodes:
/// what has no receipt does not persist.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColonyOrb {
    pub orb_id: u64,
    pub name: String,
    /// Zones, tile-ascending (canonical order).
    pub zones: Vec<Zone>,
    /// Banked yields, resource-ascending (canonical order); zero
    /// entries are dropped (one state, one encoding).
    pub banked: Vec<(Resource, Ratio)>,
    /// Resources under famine, ascending — zones consuming these are
    /// DORMANT (no yield, no drain).
    pub famine: Vec<Resource>,
    /// THERMAL-1: the orbit siting; ABSENT until `PlanetSited` — an
    /// unsited orb is thermally dormant (the opt-in guard).
    pub thermal: Option<ThermalSite>,
    /// THERMAL-1: currently overheating (overflow > 0)?
    pub overheat: bool,
    /// VENT-1: cumulative vented entropy, settled by receipt.
    /// **THE ENTROPIC BOOK** — what this orb has SHED to the void
    /// across its whole life. Cumulative, never pruned.
    pub entropy_vented: Ratio,
    /// **THE SYNTROPIC BOOK — the exact inverse of
    /// [`Self::entropy_vented`]** (operator ruling 2026-08-03).
    ///
    /// What this orb has CONTAINED: every joule encased, whether the
    /// excess arrived from a star or from friction — the same currency
    /// either way. Cumulative and never pruned, so the two books are
    /// finally symmetric.
    ///
    /// **Why it had to be a FIELD and not only a reading.** The loss
    /// arm has always been a stored orb field, replayed and public,
    /// while the capture arm lived either in a pruned window on the
    /// crystal sheet or in a derived reader. An orb could always be
    /// asked what it had lost forever and never what it had kept. That
    /// asymmetry is the negentropic shape this work exists to refuse,
    /// and no amount of deriving fixes it while one side is stored and
    /// the other is not.
    ///
    /// It must equal `World::heat_encased_of` for the same orb — the
    /// stored book and the derived reading are pinned against each
    /// other, so drift between them is a test failure rather than a
    /// silent divergence.
    pub syntropy_contained: Ratio,
    /// SINK-1: the stamp of the last settle (demurrage anchor).
    pub last_touch: u64,
    /// TRADE-1 (09 §2.1): the compassion books — cumulative
    /// joule-commensurable value given TO counterparties standing in
    /// receipted crisis, and drawn FROM them. Committed state rebuilt
    /// from receipts alone on replay; wire encoding DEFERRED to
    /// IDEO-1 (the reserved orb cohesion entry, tag 38 — recorded on
    /// the board).
    pub given_in_crisis: Ratio,
    /// The entropic book of the pair (see `given_in_crisis`).
    pub drawn_in_crisis: Ratio,
    /// ABSORB-1 (09 §2.4): the morality books — cumulative value
    /// returned against cross-orb draws inside the reciprocity window,
    /// and the lapsed remainders. Same deferral as the compassion
    /// books: committed, replay-rebuilt, wire at IDEO-1.
    pub returned_across: Ratio,
    /// The entropic book of the pair (see `returned_across`).
    pub extracted_across: Ratio,
    /// BOND-1 (09 §2.2): the love books — cumulative value streamed
    /// over this orb's standing bonds (as source), and the severance
    /// loads posted when it cut a bond without notice (or defaulted).
    /// Same deferral: committed, replay-rebuilt, wire at IDEO-1.
    pub bond_flow: Ratio,
    /// The entropic book of the pair (see `bond_flow`).
    pub severance_load: Ratio,
    /// FORWARD-1 (09 §2.3): the integrity books — VALUE-weighted
    /// stakes of forwards released and slashed (a thousand toy
    /// promises never wash one giant breach). Same deferral:
    /// committed, replay-rebuilt, wire at IDEO-1.
    pub forwards_fulfilled: Ratio,
    /// The entropic book of the pair (see `forwards_fulfilled`).
    pub forwards_breached: Ratio,
    /// ELEMENT-GRAMMAR-1 (14 §2): banked element stores,
    /// element-id-ascending (canonical order); zero entries dropped.
    /// Wire: orb entry tag 70 (SEAL A — old readers skip).
    pub elements: Vec<([u8; 32], Ratio)>,
    /// STAMP-1 (32_STAMP_MARKET): unit-of-account balance on this orb.
    /// Exact rational; zero encodes as ABSENT (one state, one encoding).
    /// Wire: orb entry tag 73 (SEAL A — old readers skip).
    pub stamp: Ratio,
    /// FLEET-1 scrap pool (tag 74). One kind (Q-F3).
    pub scrap: Ratio,
    /// BOOK-1 (19_LENS §16): the books this orb MINTED, kind-ascending
    /// (canonical order). The charter's four are NOT here — they keep
    /// their landed fields and their landed wire entry; `books()`
    /// presents both through one view. Wire: orb entry tag 90 (SEAL A
    /// — old readers skip an orb's minted books rather than fail).
    pub books: Vec<BookAccount>,
    /// LIT-1: the works this line reaches for, work-ascending. A work
    /// carrying witnesses has been CLAIMED. Wire: orb entry tag 91
    /// (SEAL A — old readers skip).
    pub aspirations: Vec<Aspiration>,
}

impl ColonyOrb {
    /// The canonical orb codec: header, zones tile-ascending, banked
    /// resource-ascending, famines ascending.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut header = self.orb_id.to_le_bytes().to_vec();
        header.extend_from_slice(self.name.as_bytes());
        tlv::put(16, &header, &mut out);
        for zone in &self.zones {
            let mut value = zone.tile.to_le_bytes().to_vec();
            value.push(zone.kind.wire());
            // #41: the base a minted building was raised on, trailing
            // and optional — its ABSENCE is the sealed pre-zone form.
            if let Some(base) = zone.base {
                value.push(base.wire());
            }
            tlv::put(17, &value, &mut out);
        }
        for (resource, amount) in &self.banked {
            let mut value = vec![*resource as u8];
            ratio::encode(amount, &mut value);
            tlv::put(18, &value, &mut out);
        }
        for resource in &self.famine {
            tlv::put(19, &[*resource as u8], &mut out);
        }
        if let Some(site) = &self.thermal {
            let mut value = Vec::new();
            ratio::encode(site.albedo.value(), &mut value);
            ratio::encode(site.orbit_r.value(), &mut value);
            value.push(self.overheat as u8);
            // THE ANGLE, additive-trailing on tag 26 (2026-08-02). An
            // unsited angle writes nothing, so an orb that never had a
            // place is byte-identical to its pre-amendment frame — and
            // one that HAS a place keeps it across the state container,
            // which is what the committed snapshot lost before this.
            if let Some(turn) = &site.theta {
                ratio::encode(turn.angle(), &mut value);
                // ψ only after θ — an inclination without a longitude
                // would be a place with no place to be inclined from.
                if let Some(tilt) = &site.psi {
                    ratio::encode(tilt.angle(), &mut value);
                }
            }
            tlv::put(26, &value, &mut out);
        }
        if self.syntropy_contained != ratio(0, 1) {
            let mut value = Vec::new();
            ratio::encode(&self.syntropy_contained, &mut value);
            tlv::put(20, &value, &mut out);
        }
        if self.entropy_vented != ratio(0, 1) || self.last_touch != 0 {
            let mut value = Vec::new();
            ratio::encode(&self.entropy_vented, &mut value);
            value.extend_from_slice(&self.last_touch.to_le_bytes());
            tlv::put(28, &value, &mut out);
        }
        // IDEO-1: the cohesion entry — the eight books in fixed order,
        // emitted only when a book is nonzero (one state, one
        // encoding; a zero-book orb stays byte-identical to a
        // pre-ideology one).
        let books = self.cohesion_books();
        if books.iter().any(|book| **book != ratio(0, 1)) {
            let mut value = Vec::new();
            for book in books {
                ratio::encode(book, &mut value);
            }
            tlv::put(41, &value, &mut out);
        }
        // ELEMENT-GRAMMAR-1: banked element stores, id-ascending;
        // zero entries never encode (one state, one encoding) — an
        // element-less orb stays byte-identical to a pre-almanac one.
        for (element, amount) in &self.elements {
            let mut value = element.to_vec();
            ratio::encode(amount, &mut value);
            tlv::put(72, &value, &mut out);
        }
        // STAMP-1: unit-of-account balance — only when nonzero.
        if self.stamp != ratio(0, 1) {
            let mut value = Vec::new();
            ratio::encode(&self.stamp, &mut value);
            tlv::put(73, &value, &mut out);
        }
        // FLEET-1 scrap pool — only when nonzero.
        if self.scrap != ratio(0, 1) {
            let mut value = Vec::new();
            ratio::encode(&self.scrap, &mut value);
            tlv::put(74, &value, &mut out);
        }
        // BOOK-1: minted books, kind-ascending. An orb that minted none
        // stays byte-identical to a pre-BOOK-1 one (one state, one
        // encoding). The name trails because it is variable-length.
        for account in &self.books {
            let mut value = account.kind.0.to_vec();
            value.extend_from_slice(&account.minted_by.to_le_bytes());
            value.push(account.archived as u8);
            ratio::encode(&account.kept, &mut value);
            ratio::encode(&account.lost, &mut value);
            value.extend_from_slice(account.name.as_bytes());
            tlv::put(90, &value, &mut out);
        }
        // LIT-1: aspirations, work-ascending. A line that reaches for
        // nothing emits no entry and stays byte-identical to a
        // pre-LIT-1 orb.
        for held in &self.aspirations {
            let mut value = held.work.0.to_vec();
            value.extend_from_slice(&(held.witnesses.len() as u32).to_le_bytes());
            for witness in &held.witnesses {
                value.extend_from_slice(witness);
            }
            value.extend_from_slice(held.name.as_bytes());
            tlv::put(91, &value, &mut out);
        }
        out
    }

    /// **EVERY BOOK THIS ORB KEEPS — charter and minted alike.**
    ///
    /// The charter four first in charter order, then minted books
    /// kind-ascending. A reader never learns which storage a book came
    /// from, and nothing downstream may special-case the four.
    pub fn books(&self) -> Vec<BookView> {
        let chartered = [
            ("TRADE-1", &self.given_in_crisis, &self.drawn_in_crisis),
            ("ABSORB-1", &self.returned_across, &self.extracted_across),
            ("BOND-1", &self.bond_flow, &self.severance_load),
            ("COVENANT-1", &self.forwards_fulfilled, &self.forwards_breached),
        ];
        let mut out: Vec<BookView> = chartered
            .iter()
            .map(|(name, kept, lost)| BookView {
                kind: kind_of(name),
                name: (*name).to_string(),
                chartered: true,
                kept: (*kept).clone(),
                lost: (*lost).clone(),
                // A charter book is closeable by nobody (19_LENS §16c).
                archived: false,
            })
            .collect();
        out.extend(self.books.iter().map(|account| BookView {
            kind: account.kind,
            name: account.name.clone(),
            chartered: false,
            kept: account.kept.clone(),
            lost: account.lost.clone(),
            archived: account.archived,
        }));
        out
    }

    /// The minted account for a kind, if this orb keeps one.
    pub fn book(&self, kind: BookKind) -> Option<&BookAccount> {
        self.books.iter().find(|held| held.kind == kind)
    }

    /// IDEO-1: the eight cohesion books in their canonical wire order.
    fn cohesion_books(&self) -> [&Ratio; 8] {
        [
            &self.given_in_crisis,
            &self.drawn_in_crisis,
            &self.bond_flow,
            &self.severance_load,
            &self.forwards_fulfilled,
            &self.forwards_breached,
            &self.returned_across,
            &self.extracted_across,
        ]
    }

    /// Decode an orb; unknown tags are SKIPPED (SEAL A).
    pub fn from_wire(bytes: &[u8]) -> Option<Self> {
        let mut orb = ColonyOrb::default();
        let mut seen_header = false;
        for (tag, value) in tlv::records(bytes) {
            match tag {
                16 => {
                    orb.orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                    orb.name = String::from_utf8(value.get(8..)?.to_vec()).ok()?;
                    seen_header = true;
                }
                17 => {
                    let tile = u16::from_le_bytes(value.get(..2)?.try_into().ok()?);
                    let kind = ZoneKind::from_wire(*value.get(2)?)?;
                    // #41: a byte past the kind is the raised base; the
                    // sealed form ends exactly here and reads None.
                    let base = match value.get(3) {
                        Some(code) => Some(ZoneKind::from_wire(*code)?),
                        None => None,
                    };
                    orb.zones.push(Zone { tile, kind, base });
                }
                18 => {
                    let resource = Resource::from_wire(*value.first()?)?;
                    let (amount, _) = ratio::decode(value.get(1..)?)?;
                    orb.banked.push((resource, amount));
                }
                19 => {
                    orb.famine.push(Resource::from_wire(*value.first()?)?);
                }
                72 => {
                    let element: [u8; 32] = value.get(..32)?.try_into().ok()?;
                    let (amount, _) = ratio::decode(value.get(32..)?)?;
                    orb.elements.push((element, amount));
                }
                73 => {
                    let (amount, _) = ratio::decode(value)?;
                    orb.stamp = amount;
                }
                74 => {
                    let (amount, _) = ratio::decode(value)?;
                    orb.scrap = amount;
                }
                91 => {
                    let work: [u8; 32] = value.get(..32)?.try_into().ok()?;
                    let count =
                        u32::from_le_bytes(value.get(32..36)?.try_into().ok()?) as usize;
                    let mut witnesses = Vec::with_capacity(count);
                    for at in 0..count {
                        let from = 36 + at * 32;
                        witnesses.push(value.get(from..from + 32)?.try_into().ok()?);
                    }
                    let name =
                        String::from_utf8(value.get(36 + count * 32..)?.to_vec()).ok()?;
                    orb.aspirations.push(Aspiration { work: WorkId(work), name, witnesses });
                }
                90 => {
                    let kind: [u8; 32] = value.get(..32)?.try_into().ok()?;
                    let minted_by = u64::from_le_bytes(value.get(32..40)?.try_into().ok()?);
                    let archived = *value.get(40)? != 0;
                    let (kept, used_k) = ratio::decode(value.get(41..)?)?;
                    let (lost, used_l) = ratio::decode(value.get(41 + used_k..)?)?;
                    let name =
                        String::from_utf8(value.get(41 + used_k + used_l..)?.to_vec()).ok()?;
                    orb.books.push(BookAccount {
                        kind: BookKind(kind),
                        name,
                        minted_by,
                        kept,
                        lost,
                        archived,
                    });
                }
                26 => {
                    let (albedo, used) = ratio::decode(value)?;
                    let albedo = Unit::new(albedo)?;
                    let (orbit_r, used_r) = ratio::decode(value.get(used..)?)?;
                    let orbit_r = Positive::new(orbit_r)?;
                    orb.overheat = *value.get(used + used_r)? != 0;
                    // The trailing angle: absent on every frame written
                    // before the amendment, and absent means UNPLACED,
                    // never zero.
                    // The two trailing turns, each absent on frames
                    // written before its amendment. Absent is UNPLACED
                    // and UNINCLINED, never zero.
                    let tail = value.get(used + used_r + 1..).unwrap_or(&[]);
                    let (theta, used_t) = match ratio::decode(tail) {
                        Some((turn, n)) => (Some(Slot::from_angle(turn)?), n),
                        None => (None, 0),
                    };
                    // FORM AT THE DOOR: same law as tag 25 — a Slot
                    // past decode is proven, nothing downstream checks.
                    let psi = match tail.get(used_t..).filter(|rest| !rest.is_empty()) {
                        Some(rest) => {
                            let (turn, _) = ratio::decode(rest)?;
                            Some(Slot::from_angle(turn)?)
                        }
                        None => None,
                    };
                    orb.thermal = Some(ThermalSite { albedo, orbit_r, theta, psi });
                }
                20 => {
                    let (contained, _) = ratio::decode(value)?;
                    orb.syntropy_contained = contained;
                }
                28 => {
                    let (vented, used) = ratio::decode(value)?;
                    orb.entropy_vented = vented;
                    orb.last_touch =
                        u64::from_le_bytes(value.get(used..used + 8)?.try_into().ok()?);
                }
                41 => {
                    let mut at = 0usize;
                    let mut books = Vec::with_capacity(8);
                    for _ in 0..8 {
                        let (book, used) = ratio::decode(value.get(at..)?)?;
                        books.push(book);
                        at += used;
                    }
                    let mut drain = books.into_iter();
                    orb.given_in_crisis = drain.next()?;
                    orb.drawn_in_crisis = drain.next()?;
                    orb.bond_flow = drain.next()?;
                    orb.severance_load = drain.next()?;
                    orb.forwards_fulfilled = drain.next()?;
                    orb.forwards_breached = drain.next()?;
                    orb.returned_across = drain.next()?;
                    orb.extracted_across = drain.next()?;
                }
                _ => {}
            }
        }
        seen_header.then_some(orb)
    }

    /// The banked amount of a resource (zero when absent).
    pub fn banked_amount(&self, resource: Resource) -> Ratio {
        self.banked
            .iter()
            .find(|(held, _)| *held == resource)
            .map(|(_, amount)| amount.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// Add (or, negative, draw) an amount in the canonical ledger; zero
    /// entries drop so equal states share one encoding.
    pub fn bank(&mut self, resource: Resource, amount: Ratio) {
        match self.banked.iter_mut().find(|(held, _)| *held == resource) {
            Some((_, held)) => *held += amount,
            None => {
                self.banked.push((resource, amount));
                self.banked.sort_by_key(|(resource, _)| *resource);
            }
        }
        self.banked.retain(|(_, amount)| amount != &ratio(0, 1));
    }

    /// ELEMENT-GRAMMAR-1: the banked element read.
    pub fn element_amount(&self, element: &[u8; 32]) -> Ratio {
        self.elements
            .iter()
            .find(|(held, _)| held == element)
            .map(|(_, amount)| amount.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// ELEMENT-GRAMMAR-1: the element ledger, the bank's own laws —
    /// id-ascending canonical order, zero entries dropped.
    pub fn bank_element(&mut self, element: [u8; 32], amount: Ratio) {
        match self.elements.iter_mut().find(|(held, _)| *held == element) {
            Some((_, held)) => *held += amount,
            None => {
                self.elements.push((element, amount));
                self.elements.sort_by_key(|(element, _)| *element);
            }
        }
        self.elements.retain(|(_, amount)| amount != &ratio(0, 1));
    }

    /// STAMP-1: the unit-of-account balance on this orb.
    pub fn stamp_amount(&self) -> Ratio {
        self.stamp.clone()
    }

    /// STAMP-1: move STAMP (signed delta). Zero balances encode as absent.
    pub fn bank_stamp(&mut self, amount: Ratio) {
        self.stamp += amount;
        if self.stamp == ratio(0, 1) {
            self.stamp = ratio(0, 1);
        }
    }

    /// FLEET-1: scrap pool on this orb.
    pub fn scrap_amount(&self) -> Ratio {
        self.scrap.clone()
    }

    /// FLEET-1: move scrap (signed delta).
    pub fn bank_scrap(&mut self, amount: Ratio) {
        self.scrap += amount;
        if self.scrap < ratio(0, 1) {
            self.scrap = ratio(0, 1);
        }
    }

    /// How much of one input stands, as a share in `[0, 1]`.
    ///
    /// **Binary TODAY** because `famine` is a membership set: a
    /// resource is famined or it is not. When scarcity carries a degree
    /// (20_FOUNDING §6f, task #14/#16) this becomes the fraction held
    /// against the fraction needed, and every reader below turns
    /// continuous without changing a line.
    fn availability(&self, resource: &Resource) -> Ratio {
        match self.famine.contains(resource) {
            true => ratio(0, 1),
            false => ratio(1, 1),
        }
    }

    /// A zone's VIGOUR — the rate it can run at, in `[0, 1]`.
    ///
    /// **THE LAW OF THE MINIMUM.** A process runs at the rate of its
    /// SCARCEST input, never at the average of them. The obvious
    /// alternative — the share of inputs that stand — is wrong and
    /// measurably so: a Synthesizer consumes alloy 4 and biomass 1, so
    /// an averaged reading would run it at 1/2 with NO ALLOY AT ALL and
    /// mint diamond out of feedstock it does not hold. **Averaging
    /// inputs creates matter.** The minimum cannot.
    ///
    /// A zone with no upkeep reads full vigour: nothing is scarce.
    ///
    /// This reads 0 or 1 today because [`Self::availability`] is
    /// binary, so it is exactly the old all-or-nothing behaviour — but
    /// the LAW is now the right one, and it turns fractional the moment
    /// scarcity does.
    pub fn zone_vigour(&self, zone: &Zone) -> Ratio {
        zone.kind
            .upkeep_per_tick()
            .iter()
            .map(|(consumed, _)| self.availability(consumed))
            .min()
            .unwrap_or_else(|| ratio(1, 1))
    }

    /// Does a zone run at all? The boolean face of [`zone_vigour`],
    /// kept for callers that genuinely want a yes/no — the journal
    /// names dormant zones.
    pub fn zone_active(&self, zone: &Zone) -> bool {
        self.zone_vigour(zone) > ratio(0, 1)
    }
}

/// D3 (02 §2.5): the tile classes and their yield multipliers. Upkeep
/// never scales with terrain — rich tiles are strictly better, barren
/// ones a cost you zone over only when the map forces you.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileClass {
    Standard,
    Rich,
    Barren,
}

impl TileClass {
    pub fn multiplier(&self) -> Ratio {
        match self {
            Self::Standard => ratio(1, 1),
            Self::Rich => ratio(3, 2),
            Self::Barren => ratio(1, 2),
        }
    }

    /// The map glyph (02 §2.5): · standard, ▲ rich, ░ barren.
    pub fn glyph(&self) -> char {
        match self {
            Self::Standard => '·',
            Self::Rich => '▲',
            Self::Barren => '░',
        }
    }
}

/// The pure derivation: BLAKE3(seed ‖ LE64 orb ‖ LE16 tile), first
/// byte mod 64 → ~40 Standard, ~12 Rich, ~12 Barren (02 §2.5 — the
/// frequencies are approximate by construction, the derivation exact).
pub fn tile_class(seed: &[u8; 32], orb_id: u64, tile: u16) -> TileClass {
    let mut hasher = blake3::Hasher::new();
    hasher.update(seed);
    hasher.update(&orb_id.to_le_bytes());
    hasher.update(&tile.to_le_bytes());
    match hasher.finalize().as_bytes()[0] % 64 {
        0..=39 => TileClass::Standard,
        40..=51 => TileClass::Rich,
        _ => TileClass::Barren,
    }
}



/// THE TURN LAW — relocated to `ratio` (2026-08-05) so the solver can
/// address a SPHERE in the same frame the universe sites its orbs in,
/// and re-exported here so every existing citation resolves unchanged.
/// One truth, two readers: `cfe::dop::TURN_SLOTS` and
/// `ratio::TURN_SLOTS` are the same constant, not two copies.
pub use ratio::{non_negative, on_slot, positive, slot_angle, slot_of, turn_gap, unit, NonNegative, Positive, Slot, Unit, TURN_SLOTS};


/// WRAPPED L1 SEPARATION between two sited orbs — the only adjacency
/// this system can compute exactly.
///
/// ```text
/// d = |r₁ − r₂| + min(|θ₁ − θ₂|, 1 − |θ₁ − θ₂|)
/// ```
///
/// The angular term takes the SHORT WAY ROUND, which is what makes the
/// space a circle rather than a segment: `θ = 9/10` and `θ = 1/10` are
/// `1/5` apart, not `4/5`.
///
/// `None` means the orb has a radius and no place, so no separation is
/// defined — an unsited orb is not "far", it is UNPLACED, and reporting
/// a distance would invent one.
pub fn separation(a: &ThermalSite, b: &ThermalSite) -> Option<Ratio> {
    let (Some(theta_a), Some(theta_b)) = (&a.theta, &b.theta) else {
        return None;
    };
    let radial = if a.orbit_r.value() >= b.orbit_r.value() {
        a.orbit_r.value().clone() - b.orbit_r.value().clone()
    } else {
        b.orbit_r.value().clone() - a.orbit_r.value().clone()
    };
    let mut total = radial + turn_gap(theta_a.angle(), theta_b.angle());
    // ψ ADDS ITS OWN WRAPPED TERM. Two orbs at one radius and one
    // longitude are still apart if they are inclined differently —
    // without this the volume collapses back to a disc.
    if let (Some(psi_a), Some(psi_b)) = (&a.psi, &b.psi) {
        total += turn_gap(psi_a.angle(), psi_b.angle());
    }
    Some(total)
}


