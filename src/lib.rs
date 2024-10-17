//! Read Rich Transcription Time Marked (RTTM)
//!
//! See:
//! - <https://web.archive.org/web/20170119114252/http://www.itl.nist.gov/iad/mig/tests/rt/2009/docs/rt09-meeting-eval-plan-v2.pdf>
//! - <https://github.com/nryant/dscore?tab=readme-ov-file#rttm>
//!
//! Too tiny to really be a create...

pub mod rttm;
pub mod segment;
pub mod errors;

pub use rttm::Rttm;
pub use segment::RttmSegment;
pub use errors::RttmError;