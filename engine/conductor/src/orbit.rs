//! **Orbits** (docs/61 G1) — a line's place about its core, in closed form.
//!
//! The place at a beat is `centre + M q(s)`: `q(s)` the universe's own
//! circle (an L1 diamond of radius `r` in a two-axis parameter plane) at arc
//! `s = s0 + v * beat (mod 8r)`, and `M` a fixed rational map from that
//! plane into three dimensions: an in-plane shear `h` (the circle becomes an
//! ellipse) and a tilt `k` onto the perpendicular axis (the ellipse is
//! inclined). Everything is derived from the line's founding offset; every
//! coordinate is piecewise linear in the beat, so its denominators stay
//! bounded for ever.
//!
//! Era 1 (the refactor onto era 1's physics, 2026-10-15): the arc's pace is
//! the turning law's, one circuit in its system's galactic period
//! (`P = 1 + R / H`, `ring`), never a shared flat-curve speed.

use cfe::ratio::{ratio, Ratio};

/// A line's orbit about its core.
#[derive(Debug, Clone, PartialEq)]
pub struct Orbit {
    centre: [Ratio; 3],
    /// The parameter plane's two axes, then the perpendicular.
    axes: [usize; 3],
    /// In-plane shear: x_a = u + h w.
    h: Ratio,
    /// Tilt onto the perpendicular: x_c = k u + lift.
    k: Ratio,
    lift: Ratio,
    /// The diamond's L1 radius, and the founding arc.
    r: Ratio,
    s0: Ratio,
    /// The arc a beat: one circuit (8r) in the system's galactic period.
    per_beat: Ratio,
}

fn arr(p: &(Ratio, Ratio, Ratio)) -> [Ratio; 3] {
    [p.0.clone(), p.1.clone(), p.2.clone()]
}

/// The signed short way from `b` to `a` on one torus axis.
fn wrapped(a: &Ratio, b: &Ratio, span: &Ratio) -> Ratio {
    let raw = a - b;
    let d = &raw - (&raw / span).floor() * span;
    if d.clone() * ratio(2, 1) > *span {
        d - span
    } else {
        d
    }
}

/// `x` brought onto `[0, span)`.
fn onto(x: Ratio, span: &Ratio) -> Ratio {
    &x - (&x / span).floor() * span
}

fn abs(x: &Ratio) -> Ratio {
    if *x < ratio(0, 1) {
        -x.clone()
    } else {
        x.clone()
    }
}

/// The point of the diamond of radius `r` at L1 arc `s` (`0 <= s < 8r`),
/// starting at `(r, 0)` and turning toward `(0, r)`. Each edge has L1
/// length `2r`.
fn diamond(r: &Ratio, s: &Ratio) -> (Ratio, Ratio) {
    let two = ratio(2, 1);
    let edge = (s / (r * &two)).floor();
    let a = s - &edge * r * &two;
    let half = &a / &two;
    match edge.to_integer().try_into().unwrap_or(0u8) {
        0 => (r - &half, half),
        1 => (-half.clone(), r - &half),
        2 => (-r + &half, -half),
        _ => (half.clone(), -r + &half),
    }
}

/// The arc at which the diamond of radius `r` passes through `(u, w)`.
fn arc_of(r: &Ratio, u: &Ratio, w: &Ratio) -> Ratio {
    let zero = ratio(0, 1);
    let two = ratio(2, 1);
    if *u >= zero && *w >= zero && *u > zero {
        w * &two
    } else if *u <= zero && *w > zero {
        r * &two - u * &two
    } else if *u < zero && *w <= zero {
        r * ratio(4, 1) - w * &two
    } else {
        r * ratio(6, 1) + u * &two
    }
}

impl Orbit {
    /// The orbit of a line founded at `founded` about the core at `centre`,
    /// on a torus of side `span`; `None` for a line at its core.
    pub fn of(
        centre: &(Ratio, Ratio, Ratio),
        founded: &(Ratio, Ratio, Ratio),
        span: &Ratio,
        period: &Ratio,
    ) -> Option<Orbit> {
        if *period <= ratio(0, 1) {
            return None;
        }
        let centre = arr(centre);
        let founded = arr(founded);
        let d: Vec<Ratio> = (0..3)
            .map(|i| wrapped(&founded[i], &centre[i], span))
            .collect();
        let mut order = [0usize, 1, 2];
        order.sort_by(|x, y| abs(&d[*y]).cmp(&abs(&d[*x])).then(x.cmp(y)));
        let [a, b, c] = order;
        let radius = abs(&d[a]) + abs(&d[b]);
        if radius == ratio(0, 1) {
            return None;
        }
        // Derived from where the line was founded: the height over the
        // radius shears the circle; the height the first axis carries tilts it.
        let h = &d[c] / &radius;
        let w = d[b].clone();
        let u = &d[a] - &h * &w;
        let (k, lift) = if u == ratio(0, 1) {
            (ratio(0, 1), d[c].clone())
        } else {
            (&d[c] / &u, ratio(0, 1))
        };
        let r = abs(&u) + abs(&w);
        if r == ratio(0, 1) {
            return None;
        }
        let s0 = arc_of(&r, &u, &w);
        let per_beat = &r * ratio(8, 1) / period;
        Some(Orbit {
            centre,
            axes: [a, b, c],
            h,
            k,
            lift,
            r,
            s0,
            per_beat,
        })
    }

    /// The line's place at beat `beat` of its own clock, on the torus of side `span`.
    pub fn at(&self, beat: u64, span: &Ratio) -> (Ratio, Ratio, Ratio) {
        let perimeter = &self.r * ratio(8, 1);
        let s = &self.s0 + &self.per_beat * Ratio::from_integer(beat.into());
        let s = &s - (&s / &perimeter).floor() * &perimeter;
        let (u, w) = diamond(&self.r, &s);
        let mut x = [ratio(0, 1), ratio(0, 1), ratio(0, 1)];
        x[self.axes[0]] = &u + &self.h * &w;
        x[self.axes[1]] = w;
        x[self.axes[2]] = &self.k * &u + &self.lift;
        let p: Vec<Ratio> = (0..3)
            .map(|i| onto(&self.centre[i] + &x[i], span))
            .collect();
        (p[0].clone(), p[1].clone(), p[2].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_orbit_starts_where_the_line_was_founded_and_closes() {
        let span = ratio(2, 1);
        let centre = (ratio(1, 2), ratio(1, 2), ratio(1, 2));
        let founded = (
            ratio(1, 2) + ratio(3, 32),
            ratio(1, 2) - ratio(2, 32),
            ratio(1, 2) + ratio(1, 32),
        );
        let orbit = Orbit::of(&centre, &founded, &span, &ratio(360, 1)).expect("off its core");
        assert_eq!(orbit.at(0, &span), founded, "beat 0 is the founding place");
        // One circuit in its period: back at 360 beats.
        let beats: u64 = 360;
        assert_eq!(orbit.at(beats, &span), founded, "the orbit closes");
        assert_ne!(orbit.at(beats / 3, &span), founded, "and moves between");
    }
}
