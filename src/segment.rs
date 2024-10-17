use std::time::Duration;

use crate::RttmError;

/// A single row in an RTTM file. Delimiter is a single space.
#[derive(Debug, Default, Clone)]
pub struct RttmSegment {
    /// Type, segment type; should always be SPEAKER
    pub segment_type: String,
    /// File ID, file name; basename of the recording minus extension (e.g., `rec1_a`)
    pub file_id: String,
    /// Channel ID, channel (1-indexed) that turn is on; should always be 1
    pub channel_id: usize,
    /// Turn Onset, onset of turn in seconds from beginning of recording
    pub turn_onset: f64,
    /// Turn Duration, duration of turn in seconds
    pub turn_duration: f64,
    /// Orthography Field, should always be `<NA>`
    pub orthography_field: String,
    /// Speaker Type, should always be `<NA>`
    pub speaker_type: String,
    /// Speaker Name, name of speaker of turn; should be unique within scope of each file
    pub speaker_name: String,
    /// Confidence Score, system confidence (probability) that information is correct; should always be `<NA>`
    pub confidence_score: String,
    /// Signal Lookahead Time, should always be `<NA>`
    pub signal_lookahead_time: String,
}

impl RttmSegment {
    /// Parse a string into `RttmSegment`.
    pub fn from_str(value: &str) -> Result<Self, RttmError> {
        let mut segment = Self::default();

        let split = value.split(' ');
        for (i, value) in split.enumerate() {
            match i {
                0 => segment.segment_type = value.to_owned(),
                1 => segment.file_id = value.to_owned(),
                2 => segment.channel_id = value.parse::<usize>()?,
                3 => segment.turn_onset = value.parse::<f64>()?,
                4 => segment.turn_duration = value.parse::<f64>()?,
                5 => segment.orthography_field = value.to_owned(),
                6 => segment.speaker_type = value.to_owned(),
                7 => segment.speaker_name = value.to_owned(),
                8 => segment.confidence_score = value.to_owned(),
                9 => segment.signal_lookahead_time = value.to_owned(),
                idx => return Err(RttmError::SegmentAlignmentError(idx + 1))
            }
        }

        Ok(segment)
    }

    /// Returns `RttmSegment` as a string that conforms to the standard
    /// for writing to file.
    pub fn to_string(&self) -> String {
        format!("{} {} {} {} {} {} {} {} {} {}",
            self.segment_type,
            self.file_id,
            self.channel_id,
            self.turn_onset,
            self.turn_duration,
            self.orthography_field,
            self.speaker_type,
            self.speaker_name,
            self.confidence_score,
            self.signal_lookahead_time,
        )
    }

    /// Returns start and end time in seconds.
    pub fn timespan(&self) -> (f64, f64) {
        (self.turn_onset, self.turn_onset + self.turn_duration)
    }

    /// Returns start and end time in milliseconds.
    pub fn timespan_ms(&self) -> (i64, i64) {
        (
            (1000. * self.turn_onset).round() as i64,
            (1000. * (self.turn_onset + self.turn_duration)).round() as i64
        )
    }

    /// Returns duration as `std::time::Duration`.
    pub fn duration(&self) -> Duration {
        Duration::from_secs_f64(self.turn_duration)
    }

    /// Returns duration in milliseconds.
    pub fn milliseconds(&self) -> u128 {
        self.duration().as_millis()
    }
}