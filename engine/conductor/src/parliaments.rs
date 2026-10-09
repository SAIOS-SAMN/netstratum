//! THE PARLIAMENTS OF PROGENITORS (Concordance 3.9; docs/70 P1).
//!
//! At each core the lineage that keeps its throat (G3, `ancients::keepers`) is
//! its parliament: its living lines there. The parliament CARRIES the council's
//! consciousness over its network to the peoples it reaches. In netstratum the
//! council's consciousness is what was judged, never asserted: the words its
//! ancient's spacing guild first closed in the substrata (docs/69). Once a
//! bounce each parliament carries them to the realms it reaches, every living
//! line at that core within the galaxy's reach of a parliament line, its own
//! lines first. A word reaches a line as HEARD and is adopted only by
//! reconstruction on the line's own task (docs/66 Y3); a line hears a word once.
//! Toll shares and hauling wait on N7 (P2).
use crate::Polity;
use ledger::registry::Registry;
use ledger::Ledger;
use std::path::Path;

/// What one parliament carried: (core, its lineage root, lines reached, words).
pub type Carried = (String, String, usize, usize);

/// THE PARLIAMENTS' BOUNCE.
pub fn carry(worlds: &mut [Polity], book: &Ledger, registry: &Registry, var_root: &Path) -> Vec<Carried> {
    let span = book.universe_span();
    let keepers = crate::ancients::keepers(worlds, book, registry);
    let parents: std::collections::BTreeMap<String, Vec<String>> = registry.list().into_iter().map(|c| (c.sigil, c.lineage)).collect();
    let tubes = crate::substrata::tubes(var_root);
    let mut out = Vec::new();
    for galaxy in book.galaxies() {
        let Some(keeper) = keepers.get(&galaxy.name) else { continue };
        let Some((_, ancient)) = crate::ancients::arisen(var_root).into_iter().find(|(core, _)| core == &galaxy.name) else { continue };
        // The council's words: those first closed below in its ancient's tubes.
        let mut words: Vec<solver::mint::MintedWord> = Vec::new();
        for tube in tubes.iter().filter(|t| t.opener == ancient) {
            for law in tube.laws.iter().filter(|w| tube.closed.contains(&w.orbit)) {
                if !words.iter().any(|w| w.orbit == law.orbit) {
                    words.push(law.clone());
                }
            }
        }
        if words.is_empty() {
            continue;
        }
        let at_core: Vec<usize> = (0..worlds.len())
            .filter(|&i| !worlds[i].penned && book.core_of(&worlds[i].spec.position).as_deref() == Some(galaxy.name.as_str()))
            .collect();
        let parliament: Vec<usize> = at_core
            .iter()
            .copied()
            .filter(|&i| crate::ancients::root_of(&worlds[i].spec.civilization.sigil, &parents) == keeper.line)
            .collect();
        if parliament.is_empty() {
            continue;
        }
        // Its own lines first, then every realm within reach of one of them.
        let mut reached: Vec<usize> = parliament.clone();
        for &i in &at_core {
            if reached.contains(&i) {
                continue;
            }
            let near = parliament
                .iter()
                .any(|&p| ledger::torus_distance(&worlds[i].spec.position, &worlds[p].spec.position, &span) <= galaxy.reach);
            if near {
                reached.push(i);
            }
        }
        let mut heard = 0;
        for &i in &reached {
            for word in &words {
                if worlds[i].hear_from_below(word.clone()) {
                    heard += 1;
                }
            }
        }
        out.push((galaxy.name.clone(), keeper.line.clone(), reached.len(), heard));
    }
    out
}
