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
//!     Optional parameters:
//!         clip_override: A wav file that can be swapped for the input channel. This is useful for
//!                        when you want to practice one section of a song without playing all of
//!                        the other sections.

extern crate block;
extern crate config;
extern crate log;
extern crate sampler;
extern crate segment;
extern crate stream;
extern crate wav;

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

    /// Whether or not the recording is complete.
    recording_complete: bool,

    /// The current interval index.
    cur_interval: usize,

    /// Whether we are currently playing something.
    is_playing: bool,
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
        let clip_override = config.get_str_opt("clip_override", "")?;

        // Load streams. Load inputs first so that we don't accidentally bind to ourself.
        let mut input_streams = Vec::<stream::Stream>::with_capacity(input_channels.len());
        for channel in &input_channels {
            input_streams.push(stream_catalog.bind_sink(channel)?);
        }
        let output_stream = stream_catalog.create_source(output_channel)?;
        output_stream.borrow_mut().fill(stream::ZERO);

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
            "Looper blocks may only have one input segment"
        );
        let recording_segment = recording_segment_opt.unwrap();

        // Sort the segments to build the state machine.
        playback_segments.sort_by(|a, b| a.start.total_cmp(&b.start));

        // Sanity check that the replay intervals come after the recording interval.
        for segment in &playback_segments {
            log::abort_if_msg!(
                segment.start < recording_segment.stop,
                "Looper output [playback] segments must be after the input [recording] segment"
            );
        }

        // Create the clip/sampler.
        let recording: stream::Clip;
        if clip_override != "" {
            recording = wav::read_wav_file(clip_override)?;
        } else {
            recording = stream::empty_clip();
        }
        let sampler = sampler::Sampler::new();

        // If clip_override is provided, treat the input segment as an output segment.
        if clip_override != "" {
            playback_segments.push(recording_segment.clone());
        }

        Ok(Looper {
            name: config.name.to_owned(),
            input_streams: input_streams,
            output_stream: output_stream,
            recording: recording,
            sampler: sampler,
            recording_segment: recording_segment,
            playback_segments: playback_segments,
            recording_complete: clip_override != "",
            cur_interval: 0,
            is_playing: false,
        })
    }
}

impl block::Transformer for Looper {
    fn transform(&mut self, state: &block::PlaybackState) {
        let tempo = state.tempo;
        let cur_measure = tempo.current_measure();

        //  -- RECORDING PHASE -- //
        if !self.recording_complete {
            // Determine whether we're in the recording interval.
            if self.recording_segment.start <= cur_measure
                && cur_measure < self.recording_segment.stop
            {
                let mut recording = self.recording.borrow_mut();
                if recording.is_empty() {
                    println!("Loop recording started: {}", self.name);
                }

                // Resize the recording clip.
                let start_index = recording.len();
                recording.resize(start_index + stream::SAMPLES_PER_BUFFER, stream::ZERO);

                // Add the input streams to the new extended portion of the recording.
                for input_stream in &self.input_streams {
                    let stream = input_stream.borrow();
                    for i in 0..stream::SAMPLES_PER_BUFFER {
                        recording[start_index + i] += stream[i];
                    }
                }
            }

            // Mark the recording complete if we're past the recording interval.
            if cur_measure >= self.recording_segment.stop {
                println!("Loop recording complete: {}", self.name);
                self.recording_complete = true;
            }
        }

        //  -- PLAYBACK PHASE -- //
        if self.recording_complete {
            let mut should_play = false;
            let mut next_interval = self.cur_interval;
            while next_interval < self.playback_segments.len() {
                let segment = &self.playback_segments[next_interval];

                // We're waiting for the next segment. Break without playing.
                if cur_measure < segment.start {
                    break;
                }

                // We're in the current segment. Break and play.
                if cur_measure < segment.stop {
                    should_play = true;
                    break;
                }

                // Otherwise, check the next segment.
                next_interval += 1;
            }

            // We have entered a new interval. Start the sample from the beginning.
            if should_play && (next_interval != self.cur_interval || !self.is_playing) {
                println!("Playing loop: {}", self.name);
                self.sampler.play(&self.recording, true);
            }
            // If we shouldn't play at all, stop the sampler.
            else if !should_play {
                self.sampler.stop();
            }

            let mut output_stream = self.output_stream.borrow_mut();

            // If we were playing something on the previous cycle, zero out the output stream.
            if self.is_playing {
                output_stream.fill(stream::ZERO);
            }
            self.sampler.next(&mut output_stream);

            // Save off internal state.
            self.cur_interval = next_interval;
            self.is_playing = should_play;
        }
    }
}
