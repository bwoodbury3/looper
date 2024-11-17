//! Looper Block.
//!
//! This block records some input streams for the specified input segment and loops it over all
//! subsequent output segments. If multiple input streams are provided, this block implicitly
//! combines them into the recording.
//!
//! Loop [Transformer]:
//!     Required parameters:
//!         name: Anything
//!         type: "Loop"
//!         segments: One input segment followed by 0+ output segments.
//!         input_channel: The input channel names.
//!         output_channel: The output channel name.

extern crate block;
extern crate config;
extern crate log;
extern crate sampler;
extern crate segment;
extern crate stream;

/// Looper Transformer block.
pub struct Looper {
    /// The block name.
    name: String,

    /// The input streams.
    input_streams: Vec<stream::Stream>,

    /// The output streams.
    output_stream: stream::Stream,

    /// The clip used for recording and playing back the stream.
    recording: stream::Clip,

    /// The sampler which controls the playback.
    sampler: sampler::Sampler,

    /// The recording segment.
    recording_segment: segment::Segment,

    /// The playback segments.
    playback_segments: Vec<segment::Segment>,
}

impl Looper {
    /// Construct a new Looper block.
    pub fn new(
        config: &config::BlockConfig,
        stream_catalog: &mut stream::StreamCatalog,
    ) -> Result<Self, ()> {
        // Read in parameters.
        let input_channels = config.get_str_list("input_channels")?;
        let output_channel = config.get_str("output_channel")?;
        let segments = config.get_segments()?;

        // Load streams. Load inputs first so that we don't accidentally bind to ourself.
        let mut input_streams = Vec::<stream::Stream>::with_capacity(input_channels.len());
        for channel in &input_channels {
            input_streams.push(stream_catalog.bind_sink(channel)?);
        }
        let output_stream = stream_catalog.create_source(output_channel)?;

        // Load segments and sort into their respective buckets.
        let mut recording_segment_opt: Option<segment::Segment> = None;
        let mut playback_segments = Vec::<segment::Segment>::with_capacity(segments.len() - 1);
        for segment in segments {
            if segment.segment_type == segment::SegmentType::Input {
                log::abort_if_msg!(
                    recording_segment_opt.is_some(),
                    "Looper blocks may only have one input segment"
                );
                recording_segment_opt = Some(segment);
            } else {
                playback_segments.push(segment);
            }
        }
        log::abort_if_msg!(
            recording_segment_opt.is_none(),
            "Looper must have one \"output\" [recording] segment"
        );
        let recording_segment = recording_segment_opt.unwrap();

        // Sanity check that the replay intervals come after the recording interval.
        for segment in &playback_segments {
            log::abort_if_msg!(
                segment.start < recording_segment.stop,
                "Looper output [playback] segments must be after the input [recording] segment"
            );
        }

        // Create the clip/sampler.
        let recording = stream::Clip::new(Vec::<stream::Sample>::new().into());
        let sampler = sampler::Sampler::new();

        Ok(Looper {
            name: config.name.to_owned(),
            input_streams: input_streams,
            output_stream: output_stream,
            recording: recording,
            sampler: sampler,
            recording_segment: recording_segment,
            playback_segments: playback_segments,
        })
    }
}

impl block::Transformer for Looper {
    fn transform(&mut self, state: &block::PlaybackState) {
        let tempo = state.tempo;

        // Record the input samples if we're in the recording segment.
        if tempo.in_measure(self.recording_segment.start, self.recording_segment.stop, 0.0) {
            let mut recording = self.recording.borrow_mut();
            if recording.is_empty() {
                println!("Loop started: {}", self.name);
            }

            // Resize the recording clip.
            let start_index = recording.len();
            recording.resize(start_index + stream::SAMPLES_PER_BUFFER, 0 as stream::Sample);

            // Add the input streams to the new extended portion of the recording.
            for input_stream in &self.input_streams {
                let stream = input_stream.borrow();
                for i in 0..stream::SAMPLES_PER_BUFFER {
                    recording[start_index + i] += stream[i];
                }
            }
        }

        // Detect whether we need to playback in this interval.
        let mut should_play = false;
        for segment in &self.playback_segments {
            if tempo.in_measure(segment.start, segment.stop, 1.0) {
                should_play = true;
                break;
            }
        }

        // If we're not playing and we should be, start the sampler. Otherwise turn it off.
        let mut output_stream = self.output_stream.borrow_mut();
        if should_play {
            if !self.sampler.is_playing() {
                println!("Playing loop: {}, len={}", self.name, self.recording.borrow().len() / stream::SAMPLES_PER_BUFFER);
                self.sampler.play(&self.recording, true);
            }
        } else {
            self.sampler.stop();
        }

        output_stream.fill(0 as stream::Sample);
        self.sampler.next(&mut output_stream);
    }
}
