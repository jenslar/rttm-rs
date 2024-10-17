use std::{path::Path, fs::File, io::{BufReader, BufRead, Write}, collections::HashSet};

use crate::{RttmError, RttmSegment};

/// Rich Transcription Time Marked (RTTM) file format.
/// 
/// References:
/// - <https://web.archive.org/web/20170119114252/http://www.itl.nist.gov/iad/mig/tests/rt/2009/docs/rt09-meeting-eval-plan-v2.pdf>
/// - <https://catalog.ldc.upenn.edu/docs/LDC2004T12/RTTM-format-v13.pdf>
/// - <https://stackoverflow.com/questions/30975084/rttm-file-format>
#[derive(Debug)]
pub struct Rttm(Vec<RttmSegment>);

impl Rttm {
    /// Read a RTTM plain-text file.
    pub fn read(path: &Path, continue_on_error: bool) -> Result<Self, RttmError> {
        let file = File::open(path)?;
        let lines = BufReader::new(file).lines();
        let mut segments: Vec<RttmSegment> = Vec::new();

        for line_result in lines {
            let line = if continue_on_error {
                if let Ok(line) = line_result {
                    line
                } else {
                    continue
                }
            } else {
                line_result?
            };

            let segment = RttmSegment::from_str(&line)?;

            segments.push(segment);
        };

        Ok(Self(segments))
    }

    /// Returns a reference to contained segments.
    pub fn segments(&self) -> &[RttmSegment] {
        &self.0
    }

    /// Returns a mutable reference to contained segments.
    pub fn segments_mut(&mut self) -> &mut [RttmSegment] {
        &mut self.0
    }

    /// Add a segment in last position.
    pub fn add(&mut self, segment: &RttmSegment) {
        self.0.push(segment.to_owned())
    }

    /// Remove the last segment.
    pub fn pop(&mut self) -> Option<RttmSegment> {
        self.0.pop()
    }

    /// Delete and return segment at `index`.
    /// Returns `None` if `index` is out of bounds.
    pub fn del(&mut self, index: usize) -> Option<RttmSegment> {
        if self.0.get(index).is_some() {
            return Some(self.0.remove(index))
        }
        None
    }

    /// Write to plain-text file that conforms to the standard.
    pub fn write(&self, path: &Path) -> std::io::Result<()> {
        let string = self.to_string();
        let mut file = File::create(path)?;
        file.write_all(string.as_bytes())?;

        Ok(())
    }

    /// Returns `Rttm` as a string that conforms to the standard,
    /// for writing to file.
    pub fn to_string(&self) -> String {
        self.iter()
            .map(|seg| seg.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Iterate over segments.
    pub fn iter(&self) -> impl Iterator<Item = &RttmSegment> {
        self.0.iter()
    }

    /// Returns sorted list of unique speakers.
    pub fn speakers(&self) -> Vec<&str> {
        let mut names = self.iter()
            .map(|seg| seg.speaker_name.as_str())
            .collect::<HashSet<&str>>()
            .into_iter()
            .collect::<Vec<_>>();
        names.sort();
        names
    }

    /// Returns number of unique speakers
    pub fn num_speakers(&self) -> usize {
        self.iter()
            .map(|seg| seg.speaker_name.as_str())
            .collect::<HashSet<&str>>()
            .len()
    }

    /// Returns first segment with `speaker`.
    pub fn find(&self, speaker: &str) -> Option<&RttmSegment> {
        self.iter().find(|seg| &seg.speaker_name == speaker)
    }

    /// Returns a new `Rttm` with only specified speaker present.
    pub fn filter(&self, speaker: &str) -> Self {
        let segments = self.iter()
            .filter(|seg| &seg.speaker_name == speaker)
            .cloned()
            .collect::<Vec<_>>();

        Self(segments)
    }

    /// Returns all timespans in seconds as tuples of `(start, end)`
    pub fn timespans(&self) -> Vec<(f64, f64)> {
        self.iter()
            .map(|seg| seg.timespan())
            .collect::<Vec<_>>()
    }

    /// Returns all timespans in millisecnds as tuples of `(start, end)`
    pub fn timespans_ms(&self) -> Vec<(i64, i64)> {
        self.iter()
            .map(|seg| seg.timespan_ms())
            .collect::<Vec<_>>()
    }

    /// Returns total duration for `speaker` in seconds.
    pub fn duration_speaker(&self, speaker: &str) -> f64 {
        self.iter()
            .filter_map(|seg| if &seg.speaker_name == speaker {
                Some(seg.turn_duration)
            } else {
                None
            })
            .sum()
    }

    /// Returns total duration of all segments in seconds.
    pub fn duration_total(&self) -> f64 {
        self.iter()
            .map(|seg| seg.turn_duration)
            .sum()
    }
}
