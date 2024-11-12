//! Metronome Blocks.
//!
//! Metronome [Source]:
//!     Required parameters:
//!         name: Anything
//!         type: "Metronome"
//!         segments: A list of output segments for which the metronome is active. If none are
//!                   specified, the metronome will be active forever.
//!     Optional parameters:
//!         sound: The sound to play. Defaults to "hihat-closed1".

extern crate block;
extern crate config;
extern crate log;
extern crate sampler;
extern crate stream;
extern crate tempo;
extern crate wav;

/// Metronome Source block.
pub struct Metronome {
    /// The block name.
    name: String,

    /// The output stream buffer.
    stream: stream::Stream,

    // The clip to play.
    clip: stream::Clip,

    /// The sampler.
    sampler: sampler::Sampler,
}

impl Metronome {
    /// Construct a new Metronome block.
    pub fn new(config: &config::BlockConfig, stream_catalog: &mut stream::StreamCatalog) -> Result<Self, ()> {
        // Read in parameters.
        let output_stream = config.get_str("output_channel")?;
        // TODO: Segments here
        let sound = config.get_str_opt("sound", "hihat-closed1")?;

        // Load streams.
        let stream = stream_catalog.create_source(output_stream)?;

        // Load in the clip to play.
        let filename = config::clip_path(sound);
        let clip = log::unwrap_abort_msg!(
            wav::read_wav_file(&filename),
            format!("Failed to find clip {} at {}", sound, filename)
        );

        // Load the sampler.
        let sampler = sampler::Sampler::new();

        Ok(Metronome {
            name: config.name.to_owned(),
            stream: stream,
            clip: clip,
            sampler: sampler
        })
    }
}

impl block::Source for Metronome {
    fn read(&mut self, tempo: &tempo::Tempo) {
        let mut stream = self.stream.borrow_mut();

        stream.fill(0.0);

        // TODO segments.
        if tempo.on_beat(0.0) {
            self.sampler.play(&self.clip, false);
        }

        self.sampler.next(&mut stream);
    }
}
