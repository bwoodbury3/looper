//! VirtualInstrument Block.
//!
//! VirtualInstrument [Source]:
//!     Required parameters:
//!         name: Anything
//!         type: "VirtualInstrument"
//!         instrument: The name of the instrument without the json suffix (see assets/instruments)
//!     Optional parameters:
//!         volume: The volume of the instrument.

extern crate block;
extern crate config;
extern crate log;
extern crate sampler;
extern crate segment;
extern crate stream;
extern crate tempo;
extern crate wav;

use std::collections::HashMap;

/// A clip paired to its sampler.
struct ClipSamplerPair {
    clip: stream::Clip,
    sampler: sampler::Sampler,
}

/// A virtual instrument that can be played using the keyboard.
pub struct VirtualInstrument {
    /// The output stream buffer.
    stream: stream::Stream,

    /// A mapping of keyboard keys to audio clips.
    clips: HashMap<char, ClipSamplerPair>,
}

/// Loads the instrument JSON file in as a map of clips.
fn load_instrument(instrument_type: &str) -> Result<HashMap<char, ClipSamplerPair>, ()> {
    let filename = config::instrument_path(instrument_type);
    let config = log::unwrap_abort_msg!(
        config::read_json_file(filename.as_str()),
        format!("Invalid instrument \"{}\" (tried to load from: {})", instrument_type, filename)
    );
    log::abort_if!(config.is_null());

    let sounds = &config["sounds"];
    log::abort_if!(!sounds.is_array());

    // Load the audio clips into memory.
    let mut clips = HashMap::<char, ClipSamplerPair>::new();
    for sound in sounds.members() {
        let key = log::unwrap_abort!(sound["key"].as_str().ok_or(()));
        let clip_name = log::unwrap_abort!(sound["file"].as_str().ok_or(()));

        log::abort_if_msg!(key.len() != 1, "Invalid instrument \"key\", must be of type char");
        let key_char = key.chars().next().unwrap();

        let clip_path = config::clip_path(clip_name);
        let clip = log::unwrap_abort!(wav::read_wav_file(clip_path.as_str()));
        let sampler = sampler::Sampler::new();

        clips.insert(key_char, ClipSamplerPair { clip, sampler });
    }

    Ok(clips)
}

impl VirtualInstrument {
    /// Construct a new VirtualInstrument block.
    pub fn new(
        config: &config::BlockConfig,
        stream_catalog: &mut stream::StreamCatalog,
    ) -> Result<Self, ()> {
        // Read in config parameters
        let output_channel = config.get_str("output_channel")?;
        let instrument_type = config.get_str("instrument")?;
        let _volume = config.get_f32_opt("volume", &1.0)?;

        // Load the stream.
        let stream = stream_catalog.create_source(output_channel)?;

        // Load the instrument configuration.
        let clips = load_instrument(instrument_type)?;

        Ok(VirtualInstrument {
            stream: stream,
            clips: clips,
        })
    }
}

impl block::Source for VirtualInstrument {
    fn read(&mut self, state: &block::PlaybackState) {
        // Play all of the samples for whichever keys were pressed.
        let keys = &state.keyboard.keys;
        for key in keys {
            match self.clips.get_mut(key) {
                Some(clip) => {
                    clip.sampler.play(&clip.clip, false);
                }
                None => {}
            }
        }

        // Read off all of the streams.
        let mut stream = self.stream.borrow_mut();
        stream.fill(0 as stream::Sample);
        for clip in self.clips.values_mut() {
            clip.sampler.next(&mut stream);
        }
    }
}
