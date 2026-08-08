//! THE GAUGE — the frame, its graded shifts, divergence and
//! confluence, veils and shields.

use super::*;

impl World {
    /// THRESHOLD-1: has this world made the CROSSING — its first
    /// receipted aperture deed? Council eligibility reads here.
    pub fn has_crossed(&self) -> bool {
        self.crossed
    }
    /// THRESHOLD-1: a shielded world is one whose people have not made
    /// the solar leap — the Watchers' charge, reachable by no aperture.
    pub fn is_shielded(&self) -> bool {
        !self.knows_its_sun()
    }
    /// AEON-1a: grade 0 of the key — the landed scalar read; every
    /// existing caller and law binds here unchanged.
    pub fn gauge(&self) -> &Ratio {
        &self.gauge[0]
    }
    /// GAUGE-N: the WHOLE key — the graded vector of wrapped planes.
    /// Its length is the world's opened dimensionality (derived from
    /// receipts, never stored beyond them).
    pub fn key(&self) -> &[Ratio] {
        &self.gauge
    }
    /// GAUGE-N (11 §2.1): move — or OPEN — the key at a grade. A
    /// shift at an unopened grade GROWS the vector (zeros fill the
    /// gap: unopened planes rest at ground); the receipt carries the
    /// wrapped value. Grade 0 through this door is lawful and lands
    /// tag 55; the landed grade-0 deeds keep tag 42.
    pub fn shift_gauge_at(&mut self, grade: u16, gauge: Ratio) -> Result<(), ActionRefused> {
        let at = grade as usize;
        // GAUGE-N refinement (the growth law, 11 §2.1): OPENING a
        // plane is EARNED — one receipted mint per new dimension. A
        // mint is dimensional growth already receipted: a crystallized
        // or granted node (condition ≥ 1), a divergence, a confluence.
        // Moving an already-open plane stays free.
        if at >= self.gauge.len() {
            let mints = self
                .chronicle
                .receipts()
                .iter()
                .filter(|receipt| {
                    matches!(
                        &receipt.event,
                        ClosureEvent::TechDiscovered { condition: 1.., .. }
                            | ClosureEvent::Diverged { .. }
                            | ClosureEvent::Confluent { .. }
                    )
                })
                .count();
            if at > mints {
                return Err(ActionRefused::PlaneUnearned {
                    grade,
                    mints: mints as u64,
                });
            }
        }
        let wrapped = wrap_gauge(gauge);
        if self.gauge.len() <= at {
            self.gauge.resize(at + 1, ratio(0, 1));
        }
        self.gauge[at] = wrapped.clone();
        self.close(ClosureEvent::GaugeShiftedAt { grade, gauge: wrapped }, SOVEREIGN);
        Ok(())
    }
    /// AEON-1a (11 §2.1): move the key. The value wraps onto [0, 1) —
    /// the fiber is a circle, not a line; the receipted value is the
    /// WRAPPED one, so replay never re-derives the wrap.
    pub fn shift_gauge(&mut self, gauge: Ratio) {
        let wrapped = wrap_gauge(gauge);
        self.gauge[0] = wrapped.clone();
        self.close(ClosureEvent::GaugeShifted { gauge: wrapped }, SOVEREIGN);
    }
    /// AEON-1a (11 §3): DIVERGENCE — this line cleaves; a peer branch
    /// departs under `branch`, and this world's own key moves by
    /// `delta` (the branch takes the opposite phase in ITS chronicle —
    /// annex, never merge; the caller founds it).
    pub fn diverge(&mut self, branch: &str, delta: Ratio) {
        let gauge = wrap_gauge(self.gauge[0].clone() + delta);
        self.gauge[0] = gauge.clone();
        self.close(
            ClosureEvent::Diverged { branch: branch.into(), gauge },
            SOVEREIGN,
        );
    }
    /// AEON-1a (11 §3): CONFLUENCE — this line combines with a bonded
    /// partner; the key moves to the interference of both (the caller
    /// computes the torus midpoint from both chronicles and passes it;
    /// the receipt carries the wrapped result verbatim).
    pub fn conflue(&mut self, partner: &str, midpoint: Ratio) {
        let gauge = wrap_gauge(midpoint);
        self.gauge[0] = gauge.clone();
        self.close(
            ClosureEvent::Confluent { partner: partner.into(), gauge },
            SOVEREIGN,
        );
    }
    /// MASK-1a (14 §4) — draw the veil: renders cloud from here on
    /// (the chronicle never lies; the Watchers read through). A
    /// standing joule toll settles at every collect; an unpayable
    /// toll LIFTS the veil by receipt. Build scope.
    pub fn draw_veil(&mut self) -> Result<(), ActionRefused> {
        self.draw_veil_as(SOVEREIGN)
    }
    pub fn draw_veil_as(&mut self, actor: u64) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.veiled {
            return Err(ActionRefused::VeilOutOfOrder);
        }
        self.close(ClosureEvent::VeilDrawn {}, actor);
        self.veiled = true;
        Ok(())
    }
    /// MASK-1a — lift the veil by deed (starved false). Sovereign convenience.
    pub fn lift_veil(&mut self) -> Result<(), ActionRefused> {
        self.lift_veil_as(SOVEREIGN)
    }
    /// MASK-1a — lift the veil by deed (starved false).
    pub fn lift_veil_as(&mut self, actor: u64) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !self.veiled {
            return Err(ActionRefused::VeilOutOfOrder);
        }
        self.close(ClosureEvent::VeilLifted { starved: false }, actor);
        self.veiled = false;
        Ok(())
    }
    /// MASK-1a: the veil stands.
    pub fn is_veiled(&self) -> bool {
        self.veiled
    }
}
