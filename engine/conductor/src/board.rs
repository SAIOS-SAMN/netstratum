//! THE BOARD (operator 2026-10-25: "lets just place this down, we have the
//! mechanics correctly you should basically have a fast forward button";
//! "maybe we should just fix static planets and stars"). The founding PLACES
//! each galaxy's first star systems and their worlds, where formation would
//! take far longer than the board can wait; everything after (orbits,
//! torques, burns, deaths, the ocean, the books) is the breath's physics.
//!
//! This reverses, for the board's founding only, the earlier rulings "worlds
//! are grown, never placed" and "never place matter" (2026-10-16/17), on the
//! operator's word. What is placed is EXISTENCE: every placed body's matter is
//! taken from its galaxy's own founding grains (3Z exact), and its orbit,
//! period and clock are the ring law's (`ℓ = R·H(<R)`), never set.
//!
//! The board's numbers are DEFINITIONS (polities.rs: "a definition COMPILES"):
//! constants here, tuned by editing them, never a data file.
use crate::flows::{Enclosed, Mote};
use crate::pulse::{mass, Body, Manifest, Universe};
use cfe::ratio::{ratio, Ratio};
use num_traits::Zero;

/// Stars at the founding across the whole universe, shared among the
/// galaxies by their share of its mass (every galaxy holding matter gets at
/// least one). Sexagesimal, a board choice.
pub const STARS: u32 = 60;
/// The share of a galaxy's founding grains gathered into its stars (the rest
/// stays its gas and disc, for the breath to form more).
pub const STAR_SHARE: (i64, i64) = (1, 4);
/// The worlds about each star, drawn from the seed in this range.
pub const WORLDS: (u32, u32) = (2, 8);
/// Of a star's matter, the share its worlds hold where its heart releases
/// nothing (the heart's own cut decides first; this is its floor).
pub const WORLD_SHARE: (i64, i64) = (1, 60);

fn frac_of(seed: &[u8; 32], path: &[u64], field: &str, j: u32) -> Ratio {
    let d = crate::pulse::draw(seed, path, field, j);
    crate::ring::grain(&Ratio::new(num_bigint::BigInt::from(d), num_bigint::BigInt::from(2u8).pow(64)))
}

fn take(m: &Manifest, share: &Ratio) -> Manifest {
    m.iter().map(|(k, a)| (k.clone(), crate::pulse::nucleus(&(a.clone() * share.clone())).min(a.clone()))).filter(|(_, a)| *a > ratio(0, 1)).collect()
}

fn less(m: &mut Manifest, by: &Manifest) {
    for (k, a) in by {
        if let Some(x) = m.get_mut(k) {
            *x -= a.clone();
        }
    }
    m.retain(|_, a| *a > ratio(0, 1));
}

fn body(path: Vec<u64>, radius: Ratio, period: Ratio, kept: Manifest, ell: Ratio, phase: Vec<Ratio>) -> Body {
    let state = mass(&kept);
    Body { ell_orth: ratio(0, 1), path, radius, period, kept, satellites: Vec::new(), medium: Manifest::new(), ell, medium_spin: ratio(0, 1), gas: Vec::new(), born: 0, state, flows: Vec::new(), reservoir: ratio(0, 1), turning: ratio(0, 1), bell: phase.clone(), phase, load: ratio(0, 1), remnant: false, heat: ratio(0, 1) }
}

/// PLACE the founding's first star systems and their worlds (see the module).
/// For each galaxy: up to `n` stars spaced by reach from the ring (1) within
/// the disc's span (`1 + span`), each at the balance of its place in the
/// galaxy's enclosure; each star's matter its equal part of the galaxy's
/// gathered share, cut by its own heart (the held its star, the released its
/// worlds, at least `WORLD_SHARE`); its worlds, heaviest inward, spaced by
/// reach from its extent (or its ring) within its breathing rim, each at the
/// balance of its place in its system's enclosure, the rest its belts.
/// Returns (stars, worlds) placed.
pub fn place(u: &mut Universe, seed: &[u8; 32]) -> (usize, usize) {
    let unit = u.unit.clone();
    let whole = mass(&u.total());
    if whole.is_zero() || unit.is_zero() {
        return (0, 0);
    }
    let (mut n_stars, mut n_worlds) = (0usize, 0usize);
    for core in u.cores.iter_mut() {
        let g = mass(&core.total());
        if g.is_zero() || core.flows.is_empty() {
            continue;
        }
        let home = crate::pulse::is_home(core);
        let span = if home { g.clone() / whole.clone() } else { ratio(1, 1) };
        let share_u = g.clone() / whole.clone();
        let n = ((share_u * ratio(i64::from(STARS), 1)).floor().to_integer().try_into().unwrap_or(0u32)).max(1);
        // The gathered share, taken from every mote alike.
        let f = ratio(STAR_SHARE.0, STAR_SHARE.1);
        let mut pool = Manifest::new();
        for x in core.flows.iter_mut() {
            let t = crate::pulse::nucleus(&(x.amount.clone() * f.clone())).min(x.amount.clone());
            x.amount -= t.clone();
            *pool.entry(x.key.clone()).or_insert_with(|| ratio(0, 1)) += t;
        }
        core.flows.retain(|x| x.amount > ratio(0, 1));
        // The galaxy's enclosure as its motes now stand (base 1, the ring).
        let g_now = g.clone();
        let enc = Enclosed::new(ratio(1, 1), core.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g_now.clone())).collect());
        let mut left = pool.clone();
        let mut siblings: Vec<u64> = core.satellites.iter().map(|s| s.path.get(1).copied().unwrap_or(0)).collect();
        // SPACING BY REACH (docs/78, the mass split): each star stands where
        // its reach just meets the last one's, `r_k (1 − m_k/H(<r_k)) =
        // r_(k−1) + r_d,(k−1)`, `r_d = r·m/H` its dominance reach, from the
        // ring outward (nothing orbits inside it). The spacing is the
        // enclosure's own, never an even ruler; what does not fit within the
        // disc's span stays the galaxy's.
        let mut bound = ratio(1, 1);
        for j in 0..n {
            let mine = if j + 1 == n { left.clone() } else { take(&pool, &ratio(1, i64::from(n))) };
            if mine.is_empty() {
                continue;
            }
            let m_s = mass(&mine) / g.clone();
            let h_b = enc.hold_within(&bound);
            if h_b <= m_s {
                break;
            }
            let r_s = crate::ring::grain(&(bound.clone() / (ratio(1, 1) - m_s.clone() / h_b)));
            if r_s > ratio(1, 1) + span.clone() {
                break;
            }
            less(&mut left, &mine);
            let h_gal = enc.hold_within(&r_s);
            bound = r_s.clone() + r_s.clone() * m_s / h_gal.clone();
            let ell = crate::ring::grain(&(r_s.clone() * h_gal.clone()));
            let radius = crate::ring::grain(&(r_s.clone() * g.clone() / unit.clone()));
            let period = crate::ring::period(&r_s, &h_gal);
            let key = crate::ring::birth_key(0, &r_s, &h_gal, &siblings);
            siblings.push(key);
            let path = vec![core.path[0], key];
            let phases = vec![frac_of(seed, &path, "phase-o", 0), frac_of(seed, &path, "phase-c", 0)];
            // Its heart's own cut: the held is the star, the released its worlds.
            let (mut kept, mut disc) = match crate::formation::heart_cut(&mine, &ratio(0, 1), &ratio(0, 1)) {
                Some((held, released, _)) => (held, released),
                None => (mine.clone(), Manifest::new()),
            };
            let floor = ratio(WORLD_SHARE.0, WORLD_SHARE.1);
            if mass(&disc) < mass(&mine) * floor.clone() {
                let more = take(&kept, &floor);
                less(&mut kept, &more);
                for (k, a) in more {
                    *disc.entry(k).or_insert_with(|| ratio(0, 1)) += a;
                }
            }
            if kept.is_empty() {
                // A heart that holds nothing is no star: its share stays the
                // galaxy's.
                for (k, a) in mine {
                    *left.entry(k).or_insert_with(|| ratio(0, 1)) += a;
                }
                continue;
            }
            let total_sys = mass(&kept) + mass(&disc);
            let h_s = mass(&kept) / total_sys.clone();
            // Its extent (the reactor's: its outermost held kind) and its
            // breathing rim against its galaxy (`breath::rim_of`).
            let extent = match crate::formation::heart_cut(&kept, &ratio(0, 1), &ratio(0, 1)) {
                Some((held, _, h_k)) if !h_k.is_zero() => h_s.clone() * crate::formation::carried(&kept, &ratio(0, 1), &ratio(0, 1)).into_iter().filter(|(k, _)| held.contains_key(k)).map(|(_, l)| l / h_k.clone()).max().unwrap_or_else(|| ratio(0, 1)),
                _ => ratio(0, 1),
            };
            let h_sys_gal = total_sys.clone() / g.clone();
            let rim = r_s.clone() / (h_sys_gal + h_gal.clone());
            let room_hi = rim;
            // Its worlds: the disc's kinds heaviest first, parted into k worlds
            // of equal holding, inner to outer.
            let k_lo = WORLDS.0;
            let k_span = WORLDS.1.saturating_sub(WORLDS.0) + 1;
            let k = k_lo + (crate::pulse::draw(seed, &path, "worlds", 0) % u64::from(k_span)) as u32;
            let mut kinds: Vec<(crate::chem::Kind, Ratio)> = disc.iter().map(|(k, a)| (k.clone(), a.clone())).collect();
            kinds.sort_by(|a, b| crate::chem::weight(&b.0).cmp(&crate::chem::weight(&a.0)).then(a.0.cmp(&b.0)));
            let disc_mass = mass(&disc);
            let mut worlds_m: Vec<Manifest> = vec![Manifest::new(); k as usize];
            let mut run = ratio(0, 1);
            for (kind, a) in kinds {
                let w = a.clone() * crate::flows::weight(&kind);
                let at = if disc_mass.is_zero() { 0 } else { ((run.clone() / disc_mass.clone()) * ratio(i64::from(k), 1)).floor().to_integer().try_into().unwrap_or(0usize).min(k as usize - 1) };
                *worlds_m[at].entry(kind).or_insert_with(|| ratio(0, 1)) += a;
                run += w;
            }
            let mut sats: Vec<Body> = Vec::new();
            let mut inner = h_s.clone();
            let mut w_sib: Vec<u64> = Vec::new();
            // Spacing by reach, one tier down (as the stars'): from the
            // reactor's extent or the system's ring (`R = H`, inside which
            // nothing orbits), whichever stands further out; what does not
            // fit within the rim joins the system's medium (its belts).
            let mut bound = if extent > h_s { extent.clone() } else { h_s.clone() };
            let mut belts = Manifest::new();
            for wm in worlds_m.into_iter() {
                if wm.is_empty() {
                    continue;
                }
                let m_w = mass(&wm) / total_sys.clone();
                let fits = inner > m_w && {
                    let r = bound.clone() / (ratio(1, 1) - m_w.clone() / inner.clone());
                    r <= room_hi
                };
                if !fits {
                    for (kind, a) in wm {
                        *belts.entry(kind).or_insert_with(|| ratio(0, 1)) += a;
                    }
                    continue;
                }
                let r = crate::ring::grain(&(bound.clone() / (ratio(1, 1) - m_w.clone() / inner.clone())));
                bound = r.clone() + r.clone() * m_w / inner.clone();
                let w_ell = crate::ring::grain(&(r.clone() * inner.clone()));
                let w_radius = crate::ring::grain(&(r.clone() * total_sys.clone() / unit.clone()));
                let w_period = crate::ring::period(&r, &inner);
                let w_key = crate::ring::birth_key(0, &r, &inner, &w_sib);
                w_sib.push(w_key);
                let w_path = vec![path[0], path[1], w_key];
                let w_phases = vec![frac_of(seed, &w_path, "phase-o", 0), frac_of(seed, &w_path, "phase-c", 0)];
                inner += mass(&wm) / total_sys.clone();
                sats.push(body(w_path, w_radius, w_period, wm, w_ell, w_phases));
                n_worlds += 1;
            }
            let mut sys = body(path, radius, period, kept, ell, phases);
            // Its own spin: the balance of its own hold, δ = ½ (its bell a
            // ring), kept extensive on the unit (`Body::turning`).
            sys.turning = crate::ring::grain(&(mass(&sys.kept) / unit.clone() * h_s.clone()));
            sys.reservoir = crate::formation::ignition_share(&ratio(0, 1)).unwrap_or_else(|| ratio(1, 1));
            sys.satellites = sats;
            sys.medium = belts;
            core.satellites.push(sys);
            n_stars += 1;
        }
        // Whatever the parting left over returns to the galaxy as it stood.
        if !left.is_empty() {
            for (k, a) in left {
                if let Some(x) = core.flows.iter_mut().find(|x| x.key == k) {
                    x.amount += a;
                } else {
                    core.flows.push(Mote { key: k, amount: a, ell_o: ratio(1, 1), ell_orth: ratio(0, 1), phase: vec![ratio(0, 1), ratio(1, 2)], agitation: ratio(0, 1), born: 0, vacuum: false, lever: Vec::new(), lever_place: ratio(0, 1) });
                }
            }
        }
        // The galaxy's held and free are READINGS of its motes: read again
        // now its stars have taken their share (the founding's own cut).
        let g_after = core.flows.iter().fold(ratio(0, 1), |s, x| s + x.amount.clone() * crate::flows::weight(&x.key));
        let flows_m = crate::flows::manifest(&core.flows);
        match crate::pulse::galaxy_push(&flows_m) {
            Some((p, _, _)) if !g_after.is_zero() => {
                let enc_c = Enclosed::new(ratio(1, 1), core.flows.iter().map(|x| (x.ell_o.clone(), x.amount.clone() * crate::flows::weight(&x.key) / g_after.clone())).collect());
                let (kept, free) = crate::flows::read_cut(&core.flows, &enc_c, &p);
                core.kept = kept;
                core.medium = free;
            }
            _ => {
                core.kept = Manifest::new();
                core.medium = flows_m;
            }
        }
    }
    (n_stars, n_worlds)
}
