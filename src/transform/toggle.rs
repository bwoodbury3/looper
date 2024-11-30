//! Toggle Block.
//!
//! Toggle takes an input stream and toggles it on during the provided segments.
//!
//! Toggle \[Transformer\]:
//!     Required parameters:
//!         name: Anything
//!         type: "Toggle"
//!         segments: A list of output segments.
//!         input_channel: The input channel name.
//!         output_channel: The output channel name.

extern crate block;
extern crate config;
extern crate log;
extern crate segment;
extern crate stream;

/// The Toggle Block.
pub struct Toggle {
    /// The input stream to toggle.
    input_stream: stream::Stream,

    /// The output stream.
    output_stream: stream::Stream,

    /// The output stream segments for which the audio is toggled.
    segments: Vec<segment::Segment>,
}

impl Toggle {
    /// Create a new Toggle.
    pub fn new(
        config: &config::BlockConfig,
        stream_catalog: &mut stream::StreamCatalog,
    ) -> Result<Self, ()> {
        // Read in parameters.
        let input_channel = config.get_str("input_channel")?;
        let output_channel = config.get_str("output_channel")?;
        let segments = config.get_segments()?;

        // Load streams.
        let input_stream = stream_catalog.bind_sink(input_channel)?;
        let output_stream = stream_catalog.create_source(output_channel)?;

        // Validate segments.
        for segment in &segments {
            log::abort_if_msg!(
                segment.segment_type == segment::SegmentType::Input,
                "All Toggle segments must have \"type\"=\"output\""
            );
        }

        Ok(Toggle {
            input_stream: input_stream,
            output_stream: output_stream,
            segments: segments,
        })
    }
}

impl block::Transformer for Toggle {
    fn transform(&mut self, state: &block::PlaybackState) {
        let tempo = state.tempo;

        let mut output_stream = self.output_stream.borrow_mut();
        for segment in &self.segments {
            if tempo.in_measure(segment.start, segment.stop, stream::ZERO) {
                let input_stream = self.input_stream.borrow();
                output_stream.clone_from_slice(&input_stream[..]);
                return;
            }
        }

        output_stream.fill(stream::ZERO);
    }
}
