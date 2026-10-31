//! The universe journal: the conductor's own readings, kept the way every
//! world keeps its journal (`scribe::record_journal`): framed TLV appended to
//! `<var>/universe.jrn`, never rewritten.
//!
//! The worlds' chronicles, annals and journals outlive any one process; the
//! conductor's readings (the hubs, decide, the fleet, the year's close) went
//! only to stdout, and the system journal keeps about five hours of them.
//! Here they live: one frame a beat, and one at each year's close. The relay
//! is handed off by the file alone: a conductor resuming at beat b reads the
//! last beat kept and writes only after it, so no frame is kept twice and
//! none is held in memory.

use ledger::card;
use std::path::{Path, PathBuf};

/// Frame records (this file's own register).
pub mod tag {
    pub const BEAT: u8 = 1;
    /// 1 a beat's readings, 2 a year's close.
    pub const KIND: u8 = 2;
    pub const CLEARED: u8 = 10;
    pub const ALONE: u8 = 11;
    pub const CLOSED: u8 = 12;
    pub const UNMET: u8 = 13;
    pub const PARTIAL: u8 = 14;
    /// `name=count`, one record a refusal.
    pub const REFUSED: u8 = 15;
    pub const ROAD_TOLLS: u8 = 16;
    pub const GATE_TOLLS: u8 = 17;
    pub const DECIDE_ROWS: u8 = 20;
    pub const DECIDE_LOST: u8 = 21;
    pub const DECIDE_CORRECT: u8 = 22;
    pub const DECIDE_SCORED: u8 = 23;
    pub const PERSIST_CORRECT: u8 = 24;
    pub const PERSIST_ROWS: u8 = 25;
    pub const BOOKS: u8 = 26;
    pub const TRIPLES: u8 = 27;
    pub const HULLS_RELEASED: u8 = 30;
    /// The breath (docs/78): echoes arriving this beat, systems born, the
    /// mass returned to the media (deepest-hole units, as text), stars
    /// dissolved; and the clocks: galaxy terms, galaxy epochs and worlds'
    /// local years come due this beat.
    pub const ECHOES: u8 = 50;
    pub const BIRTHS: u8 = 51;
    pub const RETURNED: u8 = 52;
    pub const DISSOLVED: u8 = 53;
    pub const TERMS: u8 = 54;
    pub const EPOCHS: u8 = 55;
    pub const LOCAL_YEARS: u8 = 56;
    /// Minds that woke this beat (docs/82), each founding a line.
    pub const WAKINGS: u8 = 57;
    /// Measurements this beat (docs/79: any recorded interaction).
    pub const MEASUREMENTS: u8 = 58;
    /// A year's close: one record a line read at it.
    pub const LINE: u8 = 40;
}

pub const BEAT_FRAME: u64 = 1;
pub const YEAR_FRAME: u64 = 2;

/// One beat's breath and clocks, for the journal.
#[derive(Debug, Clone, Default)]
pub struct BreathReading {
    pub echoes: u64,
    pub births: u64,
    pub returned: cfe::ratio::Ratio,
    pub dissolved: u64,
    pub terms: u64,
    pub epochs: u64,
    pub local_years: u64,
    pub wakings: u64,
    pub measurements: u64,
}

/// The journal at `var_root`, and the last beat it keeps of each kind.
pub struct Annal {
    path: PathBuf,
    last_beat: Option<u64>,
    last_year: Option<u64>,
}

impl Annal {
    /// Open (or begin) the journal, reading the last beat kept.
    pub fn open(var_root: &Path) -> Self {
        let path = var_root.join("universe.jrn");
        let (mut last_beat, mut last_year) = (None, None);
        if let Ok(data) = std::fs::read(&path) {
            for frame in card::frames(&data) {
                let (mut beat, mut kind) = (None, None);
                for (t, v) in card::records(frame) {
                    match t {
                        tag::BEAT => beat = card::u64_of(v),
                        tag::KIND => kind = card::u64_of(v),
                        _ => {}
                    }
                }
                match (kind, beat) {
                    (Some(BEAT_FRAME), Some(b)) => last_beat = Some(b),
                    (Some(YEAR_FRAME), Some(b)) => last_year = Some(b),
                    _ => {}
                }
            }
        }
        Self {
            path,
            last_beat,
            last_year,
        }
    }

    /// The last beat this journal keeps a beat frame for.
    pub fn last_beat(&self) -> Option<u64> {
        self.last_beat
    }

    fn append(&mut self, kind: u64, beat: u64, body: Vec<u8>) -> std::io::Result<bool> {
        let last = if kind == YEAR_FRAME {
            self.last_year
        } else {
            self.last_beat
        };
        if last.is_some_and(|l| beat <= l) {
            return Ok(false);
        }
        let mut out = Vec::new();
        card::put_u64(tag::BEAT, beat, &mut out);
        card::put_u64(tag::KIND, kind, &mut out);
        out.extend(body);
        card::append_frame(&self.path, &out)?;
        if kind == YEAR_FRAME {
            self.last_year = Some(beat)
        } else {
            self.last_beat = Some(beat)
        }
        Ok(true)
    }

    /// A beat's readings.
    #[allow(clippy::too_many_arguments)]
    pub fn beat(
        &mut self,
        beat: u64,
        hubs: &crate::HubCensus,
        cleared: usize,
        road: &cfe::ratio::Ratio,
        gate: &cfe::ratio::Ratio,
        released: usize,
        breath: &BreathReading,
    ) -> std::io::Result<bool> {
        let mut b = Vec::new();
        card::put_u64(tag::ECHOES, breath.echoes, &mut b);
        card::put_u64(tag::BIRTHS, breath.births, &mut b);
        card::put_str(tag::RETURNED, &breath.returned.to_string(), &mut b);
        card::put_u64(tag::DISSOLVED, breath.dissolved, &mut b);
        card::put_u64(tag::TERMS, breath.terms, &mut b);
        card::put_u64(tag::EPOCHS, breath.epochs, &mut b);
        card::put_u64(tag::LOCAL_YEARS, breath.local_years, &mut b);
        card::put_u64(tag::WAKINGS, breath.wakings, &mut b);
        card::put_u64(tag::MEASUREMENTS, breath.measurements, &mut b);
        card::put_u64(tag::CLEARED, cleared as u64, &mut b);
        card::put_u64(tag::ALONE, hubs.alone as u64, &mut b);
        card::put_u64(tag::CLOSED, hubs.closed as u64, &mut b);
        card::put_u64(tag::UNMET, hubs.unmet as u64, &mut b);
        card::put_u64(tag::PARTIAL, hubs.partial as u64, &mut b);
        for (name, n) in &hubs.refused {
            card::put_str(tag::REFUSED, &format!("{name}={n}"), &mut b);
        }
        card::put_str(tag::ROAD_TOLLS, &road.to_string(), &mut b);
        card::put_str(tag::GATE_TOLLS, &gate.to_string(), &mut b);
        let d = &hubs.decided;
        card::put_u64(tag::DECIDE_ROWS, u64::from(d.rows), &mut b);
        card::put_u64(tag::DECIDE_LOST, u64::from(d.lost), &mut b);
        card::put_u64(tag::DECIDE_CORRECT, u64::from(d.correct), &mut b);
        card::put_u64(tag::DECIDE_SCORED, u64::from(d.scored), &mut b);
        card::put_u64(tag::PERSIST_CORRECT, u64::from(hubs.persistence.0), &mut b);
        card::put_u64(tag::PERSIST_ROWS, u64::from(hubs.persistence.1), &mut b);
        card::put_u64(tag::BOOKS, hubs.books.0 as u64, &mut b);
        card::put_u64(tag::TRIPLES, hubs.books.1 as u64, &mut b);
        card::put_u64(tag::HULLS_RELEASED, released as u64, &mut b);
        self.append(BEAT_FRAME, beat, b)
    }

    /// A year's close: every line read at it.
    pub fn year(&mut self, beat: u64, lines: &[String]) -> std::io::Result<bool> {
        let mut b = Vec::new();
        for line in lines {
            card::put_str(tag::LINE, line, &mut b);
        }
        self.append(YEAR_FRAME, beat, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_relay_hands_off_by_the_file() {
        let dir = std::env::temp_dir().join(format!("annal-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let census = crate::HubCensus::default();
        let zero = cfe::ratio::ratio(0, 1);
        let mut a = Annal::open(&dir);
        assert!(a
            .beat(5, &census, 0, &zero, &zero, 0, &BreathReading::default())
            .unwrap());
        assert!(a.year(5, &["a line".into()]).unwrap());
        // A resumed witness keeps nothing twice and carries on.
        let mut b = Annal::open(&dir);
        assert_eq!(b.last_beat(), Some(5));
        assert!(!b
            .beat(5, &census, 0, &zero, &zero, 0, &BreathReading::default())
            .unwrap());
        assert!(b
            .beat(6, &census, 0, &zero, &zero, 0, &BreathReading::default())
            .unwrap());
        let _ = std::fs::remove_dir_all(dir);
    }
}
