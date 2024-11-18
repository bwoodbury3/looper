//! Metronome Block.
//!
//! The metronome provides a steady tick on every beat. The 'tick' sound is configurable.
//!
//! Metronome [Source]:
//!     Required parameters:
//!         name: Anything
//!         type: "Metronome"
//!         segments: A list of output segments for which the metronome is active. If none are
//!                   specified, the metronome will be active forever.
//!         output_channel: The output channel name
//!     Optional parameters:
//!         sound: The sound to play on each beat. Defaults to "hihat-closed1".
//!         volume: The volume of the metronome tick as a floating point multiplier.

extern crate block;
extern crate config;
extern crate log;
extern crate sampler;
extern crate segment;
extern crate stream;
extern crate tempo;
extern crate wav;

use stream::Scalable;

/// Metronome Source block.
pub struct Metronome {
    /// The output stream buffer.
    stream: stream::Stream,

    // The clip to play.
    clip: stream::Clip,

    /// The sampler.
    sampler: sampler::Sampler,

    /// The active segments.
    segments: Vec<segment::Segment>,
}

impl Metronome {
    /// Construct a new Metronome block.
    pub fn new(
        config: &config::BlockConfig,
        stream_catalog: &mut stream::StreamCatalog,
    ) -> Result<Self, ()> {
        // Read in parameters.
        let output_stream = config.get_str("output_channel")?;
        let sound = config.get_str_opt("sound", "hihat-closed1")?;
        let volume = config.get_f32_opt("volume", &1.0)?;
        let segments = config.get_segments()?;

        // Load streams.
        let stream = stream_catalog.create_source(output_stream)?;

        // Load in the clip to play.
        let filename = config::clip_path(sound);
        let clip = log::unwrap_abort_msg!(
            wav::read_wav_file(&filename),
            format!("Failed to find clip {} at {}", sound, filename)
        );
        clip.borrow_mut().scale(volume);

        // Load the sampler.
        let sampler = sampler::Sampler::new();

        // Validate the segments.
        for segment in &segments {
            log::abort_if_msg!(
                segment.segment_type == segment::SegmentType::Input,
                "Metronome only supports Output segment types."
            );
        }

        Ok(Metronome {
            stream: stream,
            clip: clip,
            sampler: sampler,
            segments: segments,
        })
    }
}

impl block::Source for Metronome {
    fn read(&mut self, state: &block::PlaybackState) {
        let tempo = state.tempo;

        if self.segments.is_empty() {
            // If no segments are present, assume the metronome is always on.
            if tempo.on_beat(0.0) {
                self.sampler.play(&self.clip, false);
            }
        } else {
            // Otherwise, play the metronome only in an active segment.
            for segment in &self.segments {
                if tempo.in_measure(segment.start, segment.stop, 0.0) {
                    if tempo.on_beat(0.0) {
                        self.sampler.play(&self.clip, false);
                        break;
                    }
                }
            }
        }

        let mut stream = self.stream.borrow_mut();
        stream.fill(stream::ZERO);
        self.sampler.next(&mut stream);
    }
}
