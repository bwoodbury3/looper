/// Segments are a window of time (in units of measures) that specify when a
/// block should be operating or not operating.
///
/// Segments are configured under the block under the "segments" key.
///
/// The example below instantiates a looper which listens from measures 10-14
/// and then outputs from measures 14-20.
///
/// {
///     "name": "foo",
///     "type": "Looper",
///     "segments": {
///         {
///             "type": "input",
///             "start": 10,
///             "stop": 14,
///         },
///         {
///             "type": "output",
///             "start": 14,
///             "stop": 20,
///         },
///     }
/// }
extern crate log;

use std::convert::From;

/// The type of segment
#[derive(Clone, Debug, PartialEq)]
pub enum SegmentType {
    /// Input Segment
    Input,

    /// Output Segment
    Output,

    /// Invalid Segment (user configuration error)
    Invalid,
}

/// A segment of data
#[derive(Clone, Debug)]
pub struct Segment {
    /// The beginning measure of the segment.
    pub start: f32,

    /// The end measure of the segment.
    pub stop: f32,

    /// The type of segment.
    pub segment_type: SegmentType,

    /// The name of the segment (optional).
    pub name: Option<String>,
}

impl From<&str> for SegmentType {
    fn from(s: &str) -> Self {
        match s {
            "input" => SegmentType::Input,
            "output" => SegmentType::Output,
            _ => SegmentType::Invalid,
        }
    }
}
