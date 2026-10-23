//! RECYCLING (xylarium 11.0 (18), 3b95a08; operators 2026-10-19: "why does
//! it have to be split, instead of a deconstruction?"; "the bounce will be
//! contained in the heaviest of elements that move least by the force of
//! the bounce"; "depending on the amount of energy contained, the size of
//! the reverberation there should be a formula derived to decide what
//! happens to the incoming matter on the vacuum"; "derived recycling of
//! matter as it is consumed by the core on the vacuum"). The founding's
//! bounce run on what a core consumes. No core makes 3Z: holding and
//! magnetism only trade, and the 3Z ledger stays exact.
//! - TWO ANTIPOLES AT A THROAT. The CONTAINER `S` is the holding of the
//!   heaviest held kinds, the core's state and the kinds at or above its
//!   cut: the Z pole, coherence. The REVERBERATION `E = B·S`, with
//!   `B = p − Σ m·|Δℓ|` what the push leaves after its torques: the Q pole,
//!   decoherence.
//! - The matter in the throat (the vacuum's inbound leg, the remnants'
//!   drains) takes the bounce, shared by `H/(R + H)`. `S/(S + E)` is
//!   CONSTRUCTED: pressed like with like by the burn's rung, `z + z → 2z`
//!   where it runs (`2u(z) > u(2z)`), its light adding to E; where the rung
//!   does not run the pressed share stays whole. `E/(S + E)` is
//!   DECONSTRUCTED: each kind takes E by its holding share of that load and
//!   is PEELED loosest nucleon first, `z → (z − k) + kH` at
//!   `u(z − k) + k − u(z)`, as far as its share pays, in whole nuclei. Odd
//!   and even alike; nothing is chosen. Peeling to hydrogen costs `z − u(z)`
//!   whatever the path. On our atom a peel step costs 0 or 2 (it releases
//!   only at z 67 and 106): construction gives light, deconstruction takes
//!   it.
//! - THE GALAXY'S PUSH SORTS THE PRODUCTS (operator 2026-10-19: the
//!   galaxy's orbit fuels the core as "spinning pressure of torsion", its
//!   weight the core's pressure; "the vacuum has to have equal or greater
//!   pressure to consume than the continuum to release and push"; "the
//!   vacuum and the continuum must be mathematically inverted across all
//!   dimensional facets"): inversion through the throat swaps the pull and
//!   the push, fixed at the throat, so the core consumes while the weight of
//!   the rest of its galaxy is equal or greater than its own. The heaviest
//!   products are TRAPPED, joining the container, until the two balance;
//!   the rest leave on the continuum. (Read per product at the throat, where
//!   all stand deep in the pull, the balance trapped everything; the
//!   founding's cascade on the throat's stack alone took its spin from the
//!   count of kinds there; both swallowed the galaxies.)
use crate::flows::Mote;
use cfe::ratio::{ratio, Ratio};

fn u(z: u32) -> i64 {
    i64::from(cfe::almanac::atom::atom(z).map(|a| a.unpaired).unwrap_or(0))
}

/// The cost of peeling `k` nucleons from one nucleus of `z`, loosest first,
/// in open frames (holding units): `u(z − k) + k − u(z)` (hydrogen's open
/// frame each). Negative releases.
pub fn peel_cost(z: u32, k: u32) -> i64 {
    u(z - k) + i64::from(k) * u(1) - u(z)
}

/// The light one pair gives on the burn's rung, `2u(z) − u(2z)` (None where
/// the rung does not run).
pub fn rung_light(z: u32) -> Option<i64> {
    let l = 2 * u(z) - u(2 * z);
    (l > 0).then_some(l)
}

/// What a throat's recycling did, in holding units.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Recycled {
    /// Nuclei pressed into pairs (constructed).
    pub pressed: Ratio,
    /// The light construction gave to the reverberation.
    pub light: Ratio,
    /// Nuclei peeled (deconstructed), and the holding the peels opened.
    pub peeled: Ratio,
    pub spent: Ratio,
    /// Holding trapped at the throat (joining the container).
    pub trapped: Ratio,
    /// The throat's net hydrogen, in grains (made by peeling, less pressed
    /// into helium): the return as a rate.
    pub hydrogen: Ratio,
}

/// The clusters at `at` PART in the throat's heat (1: every cluster's
/// loosest winding is within it), each into its atoms, keeping the mote's
/// state. Returns the throat's motes after parting (new ones appended).
pub fn part(motes: &mut Vec<Mote>, at: &[usize]) -> Vec<usize> {
    let mut out = Vec::new();
    for &i in at {
        if i >= motes.len() {
            continue;
        }
        let ps: Vec<(u32, u32)> = crate::chem::components(&motes[i].key).to_vec();
        match ps.as_slice() {
            [(1, _)] => out.push(i),
            _ => {
                let whole = motes[i].clone();
                for (j, (n, z)) in ps.into_iter().enumerate() {
                    let Some(kind) = crate::chem::Kind::atom(z) else { continue };
                    let mut atom = whole.clone();
                    atom.key = kind;
                    atom.amount = whole.amount.clone() * ratio(i64::from(n), 1);
                    if j == 0 {
                        motes[i] = atom;
                        out.push(i);
                    } else {
                        motes.push(atom);
                        out.push(motes.len() - 1);
                    }
                }
            }
        }
    }
    out
}

/// Recycle the (parted) motes at `at`, the matter standing in the throat,
/// under the bounce `b = p − Σ m·|Δℓ|` contained in `s` (holding units).
/// Products keep their mote's state; the heaviest are set at the throat
/// (reach 0) as of `bell` while the container stays within `g`, the weight
/// of the rest of its galaxy (holding units); the rest leave.
pub fn recycle(motes: &mut Vec<Mote>, at: &[usize], b: &Ratio, s: &Ratio, g: &Ratio, bell: u64) -> Recycled {
    let mut out = Recycled::default();
    if *b <= ratio(0, 1) || *s <= ratio(0, 1) {
        return out;
    }
    let h_in = at.iter().filter(|&&i| i < motes.len() && motes[i].key == crate::chem::hydrogen()).fold(ratio(0, 1), |acc, &i| acc + motes[i].amount.clone());
    let e = b.clone() * s.clone();
    let built = s.clone() / (s.clone() + e.clone());
    let mut stack: Vec<usize> = at.iter().copied().filter(|&i| i < motes.len()).collect();
    // CONSTRUCTED: each kind's share pressed like with like on the rung.
    let mut load: Vec<(usize, u32, Ratio)> = Vec::new();
    let mut light = ratio(0, 1);
    for &i in &stack.clone() {
        let Some(z) = crate::chem::atom_z(&motes[i].key) else { continue };
        let a = motes[i].amount.clone();
        let pressed = crate::pulse::nucleus(&(a.clone() * built.clone())).min(a.clone());
        if let Some(l) = rung_light(z) {
            let pairs = crate::pulse::nucleus(&(pressed.clone() / ratio(2, 1)));
            if pairs > ratio(0, 1) {
                motes[i].amount -= pairs.clone() * ratio(2, 1);
                let mut fused = motes[i].clone();
                let Some(product) = crate::chem::Kind::atom(2 * z) else { continue };
                fused.key = product;
                fused.amount = pairs.clone();
                motes.push(fused);
                stack.push(motes.len() - 1);
                light += pairs.clone() * ratio(l, 1);
                out.pressed += pairs * ratio(2, 1);
            }
        }
        // The deconstructed share: what was not pressed.
        let free = (a - pressed).min(motes[i].amount.clone());
        if z >= 2 && free > ratio(0, 1) {
            load.push((i, z, free));
        }
    }
    out.light = light.clone();
    // DECONSTRUCTED: the reverberation with construction's light, shared by
    // holding over the load, each kind peeled loosest first.
    let pool = e + light;
    let whole = load.iter().fold(ratio(0, 1), |acc, (_, z, n)| acc + n.clone() * ratio(i64::from(cfe::almanac::atom::hold(*z)), 1));
    if whole > ratio(0, 1) {
        for (i, z, n) in load {
            let mut share = pool.clone() * n.clone() * ratio(i64::from(cfe::almanac::atom::hold(z)), 1) / whole.clone();
            // As deep as it pays, every nucleus alike; a releasing step is
            // always taken.
            let mut k = 0u32;
            while k + 1 < z {
                let step = n.clone() * ratio(peel_cost(z, k + 1) - peel_cost(z, k), 1);
                if step > share {
                    break;
                }
                share -= step;
                k += 1;
            }
            // Whole nuclei one step further with what is left.
            let next = if k + 1 < z { peel_cost(z, k + 1) - peel_cost(z, k) } else { 0 };
            let deeper = if next > 0 { crate::pulse::nucleus(&(share.clone() / ratio(next, 1))).min(n.clone()) } else { ratio(0, 1) };
            if k == 0 && deeper <= ratio(0, 1) {
                continue;
            }
            out.spent += n.clone() * ratio(peel_cost(z, k), 1) + deeper.clone() * ratio(next, 1);
            out.peeled += n.clone();
            motes[i].amount -= n.clone();
            let emit = |key: Option<crate::chem::Kind>, amount: Ratio, motes: &mut Vec<Mote>, stack: &mut Vec<usize>| {
                if let (Some(key), true) = (key, amount > ratio(0, 1)) {
                    let mut m = motes[i].clone();
                    m.key = key;
                    m.amount = amount;
                    motes.push(m);
                    stack.push(motes.len() - 1);
                }
            };
            let shallow = n.clone() - deeper.clone();
            let hydrogen = shallow.clone() * ratio(i64::from(k), 1) + deeper.clone() * ratio(i64::from(k + 1), 1);
            let (rest, rest_deep) = (z - k, z - k - 1);
            let mut h_total = hydrogen;
            if rest == 1 { h_total += shallow.clone() } else { emit(crate::chem::Kind::atom(rest), shallow, motes, &mut stack) }
            if deeper > ratio(0, 1) {
                if rest_deep == 1 { h_total += deeper.clone() } else { emit(crate::chem::Kind::atom(rest_deep), deeper, motes, &mut stack) }
            }
            emit(Some(crate::chem::hydrogen()), h_total, motes, &mut stack);
        }
    }
    // THE GALAXY'S PUSH SORTS THE PRODUCTS (operator 2026-10-19: "build it
    // with the galaxy's push ... the orbit of the galaxy is the fuel for the
    // cores as spinning pressure of torsion - where the weight of the galaxy
    // is equal to the amount of pressure in the core"; and "the vacuum has to
    // have equal or greater pressure to consume than the continuum to release
    // and push - this is how you get the equilibrium on magnetism of the
    // heavier elements"). Each product stands at its OWN reach, `R = ℓ/H`
    // (its state, carried), and is TRAPPED where the vacuum's pull there, the
    // deficit `H/(R + H)`, is equal to or greater than the continuum's push
    // `p`: the core's own hold law (`pulse::push_cut`). The rest leaves on
    // the continuum.
    // THE INVERSION (operator 2026-10-19: "the vacuum and the continuum must
    // be mathematically inverted across all dimensional facets"): inversion
    // through the throat, `R → H²/R` (any dimension), turns the vacuum's pull
    // `H/(R + H)` into `R/(R + H)`, the continuum's push, and back; its one
    // fixed point is the throat, `R = H`, both ½. Across the throat the two
    // pressures are the galaxy's weight pressing in and the core's own
    // pushing out, and the core CONSUMES where the vacuum's is equal or
    // greater (operator: "the vacuum has to have equal or greater pressure to
    // consume than the continuum to release and push - this is how you get
    // the equilibrium on magnetism of the heavier elements"; "the weight of
    // the galaxy is equal to the amount of pressure in the core"). So the
    // throat TRAPS, heaviest first, only while the container stays within the
    // weight of the rest of its galaxy: `S + Δ ≤ G − Δ`, room `(G − S)/2`.
    // Past the balance it releases everything; the rest leaves on the
    // continuum.
    let mut room = ((g.clone() - s.clone()) / ratio(2, 1)).max(ratio(0, 1));
    let mut order: Vec<usize> = stack.iter().copied().filter(|&i| motes[i].amount > ratio(0, 1)).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(crate::chem::hold(&motes[i].key)));
    let mut held: Vec<(crate::chem::Kind, Ratio)> = Vec::new();
    for i in order {
        if room <= ratio(0, 1) {
            break;
        }
        let w = ratio(i64::from(crate::chem::hold(&motes[i].key)), 1);
        let t = crate::pulse::nucleus(&(room.clone() / w.clone())).min(motes[i].amount.clone());
        if t > ratio(0, 1) {
            room -= t.clone() * w;
            held.push((motes[i].key.clone(), t));
        }
    }
    for (k, mut t) in held {
        out.trapped += t.clone() * ratio(i64::from(crate::chem::hold(&k)), 1);
        let mut at_throat: Option<Mote> = None;
        for &i in &stack {
            if t <= ratio(0, 1) {
                break;
            }
            if motes[i].key == k && motes[i].amount > ratio(0, 1) {
                let x = motes[i].amount.clone().min(t.clone());
                motes[i].amount -= x.clone();
                t -= x.clone();
                if at_throat.is_none() {
                    let mut m = motes[i].clone();
                    m.amount = ratio(0, 1);
                    m.ell_o = ratio(0, 1);
                    m.ell_orth = ratio(0, 1);
                    m.agitation = ratio(0, 1);
                    m.born = bell;
                    at_throat = Some(m);
                }
                if let Some(m) = at_throat.as_mut() {
                    m.amount += x;
                }
            }
        }
        if let Some(mut m) = at_throat {
            m.vacuum = false;
            motes.push(m);
        }
    }
    out.hydrogen = stack.iter().filter(|&&i| motes[i].key == crate::chem::hydrogen()).fold(ratio(0, 1), |acc, &i| acc + motes[i].amount.clone()) - h_in;
    // Everything that passed leaves on the continuum (operator 2026-10-19:
    // "redistributed back via the continuum"); the trapped stand at the
    // throat, continuum matter too.
    for &i in &stack {
        motes[i].vacuum = false;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mote(key: &str, amount: Ratio) -> Mote {
        Mote { key: crate::chem::Kind::parse(key).expect("a kind"), amount, ell_o: ratio(1, 2), ell_orth: ratio(0, 1), phase: vec![ratio(0, 1), ratio(1, 2)], agitation: ratio(0, 1), born: 0, vacuum: true, lever: Vec::new(), lever_place: ratio(0, 1) }
    }

    fn three_z(ms: &[Mote]) -> Ratio {
        ms.iter().fold(ratio(0, 1), |s, m| s + m.amount.clone() * ratio(3 * i64::from(crate::chem::charge(&m.key)), 1))
    }

    #[test]
    fn the_throat_presses_and_peels_and_3z_holds() {
        // Peeling telescopes: z → zH costs z − u(z).
        for z in [2u32, 8, 17, 28, 67, 106] {
            assert_eq!(peel_cost(z, z - 1), i64::from(z) - u(z), "z = {z}");
        }
        // Every single peel costs 0 or 2, releasing only at z 67 and 106.
        assert!((2..=240).all(|z| [0, 2].contains(&(peel_cost(z, 1))) || [67, 106].contains(&z)));
        // The inversion through the throat swaps the pull and the push:
        // δ(H²/R) = 1 − δ(R), fixed at R = H (both ½).
        let d = |r: Ratio, h: Ratio| h.clone() / (r + h);
        for (r, h) in [(ratio(1, 3), ratio(3, 2)), (ratio(7, 2), ratio(1, 1)), (ratio(2, 1), ratio(2, 1))] {
            assert_eq!(d(h.clone() * h.clone() / r.clone(), h.clone()), ratio(1, 1) - d(r, h));
        }
        // The rung: H + H gives light, Be + Be does not.
        assert_eq!(rung_light(1), Some(2));
        assert_eq!(rung_light(4), None);
        // Water parts in the throat's heat.
        let mut ms = vec![mote("Z008·4Z001", ratio(1, 1))];
        let at = part(&mut ms, &[0]);
        assert_eq!(at.len(), 2);
        // A bounce B = ½ on S = 4: a third is peeled, two thirds pressed.
        let before = three_z(&ms);
        let r = recycle(&mut ms, &at, &ratio(1, 2), &ratio(4, 1), &ratio(4, 1), 1);
        assert_eq!(three_z(&ms), before);
        assert!(r.pressed > ratio(0, 1) && r.light > ratio(0, 1));
        assert!(r.peeled > ratio(0, 1));
        // No bounce, nothing recycled.
        let mut ms = vec![mote("Z008", ratio(1, 1))];
        assert_eq!(recycle(&mut ms, &[0], &ratio(0, 1), &ratio(4, 1), &ratio(4, 1), 1), Recycled::default());
        assert_eq!(ms[0].key, crate::chem::Kind::parse("Z008").expect("a kind"));
    }
}
