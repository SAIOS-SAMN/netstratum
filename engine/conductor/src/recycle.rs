//! RECYCLING (xylarium 11.0 (17), e555f1e/bc44bfc; operators 2026-10-19:
//! "as helium is recycled into a galactic core via vacuum or jetstream
//! mechanics: does it get resplit back into hydrogen?"). No core makes 3Z,
//! so a recycling event only moves the balance back, holding → magnetism:
//! the burn's rung run backward, `2z → z + z`, by DECOHERENCE ("coherence
//! holds the nucleus, decoherence splits it"), at the THROATS only (hearts
//! have no bounce).
//! - A split costs what the burn released, `u(a) + u(b) − u(z)` open frames
//!   a nucleus (equal halves today, `2u(z/2) − u(z)`): exactly the burn's
//!   light per pair, `w(2z) − 2w(z)`, so the split is its exact reverse. It
//!   is paid in the core's PULSE, off the 3Z ledger, which stays exact.
//! - The budget is what the regenerated push leaves after its kicks,
//!   `B = p − Σ m·|Δℓ|` (the pulse spent is what the matter absorbed, on
//!   either hemisphere), in universe units by the core's STATE. Splits run
//!   cheapest first and never overspend; a releasing split (negative cost)
//!   adds to the budget within the bounce. With no budget the throat does
//!   not decohere: the core stays within its metabolism.
//! - Splitting z all the way to hydrogen costs `z − u(z)` whatever the path
//!   (the steps telescope), so recycling to hydrogen always draws on a core.
//! - Odd kinds have no equal halves and molecules no single nucleus: they
//!   pass whole (the unequal partition is the operators' question).
use crate::flows::Mote;
use cfe::ratio::{ratio, Ratio};

fn u(z: u32) -> i64 {
    i64::from(cfe::almanac::atom::atom(z).map(|a| a.unpaired).unwrap_or(0))
}

/// The cost of splitting one nucleus of `z` into `a + (z − a)`, in open
/// frames (holding units): `u(a) + u(z − a) − u(z)`. Negative releases.
pub fn cost(z: u32, a: u32) -> i64 {
    u(a) + u(z - a) - u(z)
}

/// The split a kind takes today: equal halves of an even atom (None for
/// hydrogen, odd kinds and molecules). Returns (half, cost a nucleus).
pub fn split_of(key: &str) -> Option<(u32, i64)> {
    let z = crate::chem::atom_z(key)?;
    (z >= 2 && z % 2 == 0).then(|| (z / 2, cost(z, z / 2)))
}

/// What a throat's recycling did: the pulse spent, the release taken back,
/// the splits run.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Recycled {
    /// Holding opened (pulse spent on costly splits), in holding units.
    pub spent: Ratio,
    /// Holding closed by releasing splits (added to the budget).
    pub released: Ratio,
    /// Nuclei split.
    pub nuclei: Ratio,
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
        match crate::chem::components(&motes[i].key) {
            Some(ps) if ps.len() > 1 || ps.first().is_some_and(|(n, _)| *n > 1) => {
                let whole = motes[i].clone();
                for (j, (n, z)) in ps.into_iter().enumerate() {
                    let mut atom = whole.clone();
                    atom.key = format!("Z{z:03}");
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
            _ => out.push(i),
        }
    }
    out
}

/// Recycle the motes at `at` (the matter standing in the throat) against
/// `budget` holding units: cheapest split first, each in whole nuclei, the
/// products keeping the mote's state on the continuum (outbound). Motes
/// split partly are divided; new motes are appended.
pub fn recycle(motes: &mut Vec<Mote>, at: &[usize], budget: &Ratio) -> Recycled {
    let mut out = Recycled::default();
    if *budget <= ratio(0, 1) {
        return out;
    }
    let mut left = budget.clone();
    let mut open: Vec<usize> = at.iter().copied().filter(|&i| i < motes.len()).collect();
    loop {
        // The cheapest split standing (ties: the heavier kind first).
        let pick = open
            .iter()
            .filter_map(|&i| split_of(&motes[i].key).map(|(half, c)| (c, std::cmp::Reverse(half), i)))
            .filter(|(c, _, i)| motes[*i].amount > ratio(0, 1) && (*c <= 0 || left > ratio(0, 1)))
            .min();
        let Some((c, std::cmp::Reverse(half), i)) = pick else { break };
        let amount = motes[i].amount.clone();
        // As many whole nuclei as the budget covers (all, where it is free
        // or releases).
        let n = if c <= 0 { amount.clone() } else { crate::pulse::nucleus(&(left.clone() / ratio(c, 1))).min(amount.clone()) };
        if n <= ratio(0, 1) {
            break;
        }
        let frames = n.clone() * ratio(c, 1);
        left -= frames.clone();
        if c > 0 {
            out.spent += frames;
        } else {
            out.released -= frames;
        }
        out.nuclei += n.clone();
        let mut product = motes[i].clone();
        product.key = format!("Z{half:03}");
        product.amount = n.clone() * ratio(2, 1);
        // REDISTRIBUTED VIA THE CONTINUUM (operator 2026-10-19: "fall back
        // into the cores in the vacuum, have high pressure activity in the
        // core and then are redistributed back via the continuum"): the
        // products leave on the outbound leg as continuum matter.
        product.vacuum = false;
        if n == amount {
            motes[i] = product;
        } else {
            motes[i].amount -= n;
            motes.push(product);
            open.push(motes.len() - 1);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mote(key: &str, amount: Ratio) -> Mote {
        Mote { key: key.into(), amount, ell_o: ratio(1, 2), ell_c: ratio(0, 1), phase: vec![ratio(0, 1), ratio(1, 2)], agitation: ratio(0, 1), born: 0, vacuum: true }
    }

    fn three_z(ms: &[Mote]) -> Ratio {
        ms.iter().fold(ratio(0, 1), |s, m| s + m.amount.clone() * ratio(3 * i64::from(crate::chem::charge(&m.key)), 1))
    }

    #[test]
    fn helium_splits_to_hydrogen_at_the_burns_light_and_3z_holds() {
        // He → 2H costs 2 frames a nucleus, the light H + H gave.
        assert_eq!(split_of("Z002"), Some((1, 2)));
        assert_eq!(cost(2, 1), i64::from(cfe::almanac::atom::hold(2)) - 2 * i64::from(cfe::almanac::atom::hold(1)));
        // Odd kinds and molecules pass whole.
        assert_eq!(split_of("Z003"), None);
        assert_eq!(split_of("Z008·4Z001"), None);
        // Recycling to hydrogen telescopes: z − u(z) whatever the path.
        for z in [4u32, 8, 16, 28, 32] {
            let mut total = 0i64;
            let mut stack = vec![z];
            while let Some(k) = stack.pop() {
                if k >= 2 && k % 2 == 0 {
                    total += cost(k, k / 2);
                    stack.extend([k / 2, k / 2]);
                } else if k > 1 {
                    // An odd part is taken apart one hydrogen at a time.
                    total += cost(k, 1);
                    stack.extend([1, k - 1]);
                }
            }
            assert_eq!(total, i64::from(z) - u(z), "z = {z}");
        }
        // A budget of 2 frames splits one nucleus of He, never more.
        let mut ms = vec![mote("Z002", ratio(1, 1))];
        let before = three_z(&ms);
        let r = recycle(&mut ms, &[0], &ratio(2, 1));
        assert_eq!(three_z(&ms), before);
        assert_eq!(r.spent, ratio(2, 1));
        assert!(ms.iter().any(|m| m.key == "Z001" && m.amount == ratio(2, 1)));
        // Oxygen releases into the budget and cascades: O → 2Be (−4) →
        // 4He (0) → 8H (2 each), 4 frames in all.
        let mut ms = vec![mote("Z008", ratio(1, 1))];
        let r = recycle(&mut ms, &[0], &ratio(4, 1));
        assert_eq!(r.spent - r.released, ratio(4, 1));
        assert!(ms.iter().all(|m| m.key == "Z001"));
        // Water parts in the throat's heat before anything splits.
        let mut ms = vec![mote("Z008·4Z001", ratio(1, 1))];
        let before = three_z(&ms);
        let at = part(&mut ms, &[0]);
        assert_eq!(at.len(), 2);
        assert_eq!(three_z(&ms), before);
        assert!(ms.iter().any(|m| m.key == "Z001" && m.amount == ratio(4, 1)));
        // No budget, no decoherence.
        let mut ms = vec![mote("Z008", ratio(1, 1))];
        assert_eq!(recycle(&mut ms, &[0], &ratio(0, 1)), Recycled::default());
        assert_eq!(ms[0].key, "Z008");
    }
}
