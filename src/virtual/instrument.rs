//! VirtualInstrument Block.
//!
//! VirtualInstrument \[Source\]:
//!     Required parameters:
//!         name: Anything
//!         type: "VirtualInstrument"
//!         one of (see assets/instruments for examples):
//!             instrument: The name of the instrument without the json suffix.
//!                 OR
//!             sounds: A list of key/file pairs for the instrument. See Instrument Configuration
//!                     below.
//!     Optional parameters:
//!         volume: The volume of the instrument as a floating point multiplier.
//!
//! Instrument Configuration
//!
//! Instruments are configured a list of key/file pairs, where a key is a key on the keyboard and
//! the file is a wav file that is triggered (with the .wav suffix omitted). Example:
//!     "sounds": [
//!         {
//!             "key": "a",
//!             "file": "kick_drum"
//!         },
//!         {
//!             "key": "s",
//!             "file": "snare_drum"
//!         },
//!     ]

extern crate block;
extern crate config;
extern crate json;
extern crate log;
extern crate sampler;
extern crate segment;
extern crate stream;
extern crate tempo;
extern crate wav;

use std::collections::HashMap;

use stream::Scalable;

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
fn load_instrument_from_file(
    instrument_type: &str,
    volume: f32,
) -> Result<HashMap<char, ClipSamplerPair>, ()> {
    let filename = config::instrument_path(instrument_type);
    let config = log::unwrap_abort_msg!(
        config::read_json_file(filename.as_str()),
        format!("Invalid instrument \"{}\" (tried to load from: {})", instrument_type, filename)
    );
    log::abort_if!(config.is_null());

    let sounds = &config["sounds"];
    load_instrument(sounds, volume)
}

/// Load an instrument from JsonValue as a map of clips.
fn load_instrument(
    sounds: &json::JsonValue,
    volume: f32,
) -> Result<HashMap<char, ClipSamplerPair>, ()> {
    log::abort_if!(!sounds.is_array());

    // Load the audio clips into memory.
    let mut clips = HashMap::<char, ClipSamplerPair>::new();
    for sound in sounds.members() {
        let key = log::unwrap_abort!(sound["key"].as_str().ok_or(()));
        let clip_name = log::unwrap_abort!(sound["file"].as_str().ok_or(()));

        // Read in the key that plays this clip.
        log::abort_if_msg!(key.len() != 1, "Invalid instrument \"key\", must be of type char");
        let key_char = key.chars().next().unwrap();

        // Load the clip and the sampler.
        let clip_path = config::clip_path(clip_name);
        let clip = log::unwrap_abort!(wav::read_wav_file(clip_path.as_str()));
        let sampler = sampler::Sampler::new();

        // Scale the volume of the clip.
        clip.borrow_mut().scale(volume);

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
        let instrument_type = config.get_str_opt("instrument", "")?;
        let volume = config.get_f32_opt("volume", &1.0)?;

        // Load the stream.
        let stream = stream_catalog.create_source(output_channel)?;

        // Load the instrument configuration.
        let clips = match instrument_type {
            "" => {
                let sounds = log::unwrap_abort_msg!(
                    config.get_value("sounds"),
                    "Must specify either \"instrument\" or \"sounds\""
                );
                load_instrument(sounds, volume)?
            }
            name => load_instrument_from_file(name, volume)?,
        };

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
        stream.fill(stream::ZERO);
        for clip in self.clips.values_mut() {
            clip.sampler.next(&mut stream);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_instrument() {
        // This should unwrap.
        let clips = load_instrument_from_file("drums1", 1.0).unwrap();

        // Grab all of the keys/clips.
        for key in ['a', 's', 'd', 'f', 'g'] {
            assert!(clips.contains_key(&key));

            let clip = clips.get(&key).unwrap();
            assert!(clip.clip.borrow().len() > 0);
        }
    }

    #[test]
    fn test_load_instrument_fail() {
        // This should not unwrap.
        match load_instrument_from_file("invalid", 1.0) {
            Ok(_) => {
                panic!("Instrument should be invalid");
            }
            Err(_) => {}
        };
    }

    #[test]
    fn test_virtual_instrument() {
        // This should build with no problems.
        let project = config::ProjectConfig::new("dat/instrument/valid.json").unwrap();
        let mut stream_catalog = stream::StreamCatalog::new();
        let instrument = VirtualInstrument::new(&project.blocks[0], &mut stream_catalog).unwrap();

        // Validate all of the keys/clips.
        let clips = instrument.clips;
        for key in ['a', 's', 'd', 'f', 'g'] {
            assert!(clips.contains_key(&key));

            let clip = clips.get(&key).unwrap();
            assert!(clip.clip.borrow().len() > 0);
        }
    }

    #[test]
    fn test_virtual_instrument_inline() {
        // This should build with no problems.
        let project = config::ProjectConfig::new("dat/instrument/inline.json").unwrap();
        let mut stream_catalog = stream::StreamCatalog::new();
        let instrument = VirtualInstrument::new(&project.blocks[0], &mut stream_catalog).unwrap();

        // Validate all of the keys/clips.
        let clips = instrument.clips;
        for key in ['a', 's', 'd', 'f', 'g'] {
            assert!(clips.contains_key(&key));

            let clip = clips.get(&key).unwrap();
            assert!(clip.clip.borrow().len() > 0);
        }
    }

    #[test]
    fn test_no_instrument() {
        // This should build with no problems.
        let project = config::ProjectConfig::new("dat/instrument/no_instrument.json").unwrap();
        let mut stream_catalog = stream::StreamCatalog::new();
        match VirtualInstrument::new(&project.blocks[0], &mut stream_catalog) {
            Ok(_) => {
                panic!("Instrument should be invalid");
            }
            Err(_) => {}
        };
    }
}
