//! The main control loop for running Looper.
//!
//! ```
//! let runner = Runner::new("my_project.json")?;
//! let _ = runner.run();
//! ```

extern crate audio;
extern crate block;
extern crate combiner;
extern crate config;
extern crate instrument;
extern crate keyboard;
extern crate log;
extern crate looper;
extern crate metronome;
extern crate tempo;

/// Top level looper runner.
pub struct Runner {
    /// The project configuration.
    project: config::ProjectConfig,

    /// The tempo state.
    tempo: tempo::Tempo,

    /// Keyboard I/O.
    keyboard: keyboard::Keyboard,
}

impl Runner {
    /// Create a new runner.
    pub fn new(filename: &str) -> Result<Self, String> {
        // Read in configuration
        let project = config::ProjectConfig::new(filename)?;
        let tempo = log::unwrap_abort_str!(tempo::Tempo::new(&project));
        let keyboard = log::unwrap_abort_str!(keyboard::Keyboard::new());

        // Initialize the runner.
        Ok(Runner {
            project: project,
            tempo: tempo,
            keyboard: keyboard,
        })
    }

    /// Run!
    pub fn run(&mut self) -> Result<(), ()> {
        // Initialize portaudio.
        let pa = log::unwrap_abort!(audio::pa_get());

        // Initialize streams.
        let mut stream_catalog = stream::StreamCatalog::new();

        // Populate all of the sources/sinks/transformers.
        let mut sources: Vec<Box<dyn block::Source>> = Vec::new();
        let mut sinks: Vec<Box<dyn block::Sink>> = Vec::new();
        let mut transformers: Vec<Box<dyn block::Transformer>> = Vec::new();

        // Create all of the blocks.
        for block_config in &self.project.blocks {
            match block_config.block_type.as_str() {
                // SOURCES
                "AudioSource" => {
                    let source = audio::AudioSource::new(block_config, &mut stream_catalog, &pa)?;
                    sources.push(Box::new(source));
                }
                "VirtualInstrument" => {
                    let source =
                        instrument::VirtualInstrument::new(block_config, &mut stream_catalog)?;
                    sources.push(Box::new(source));
                }
                "Metronome" => {
                    let source = metronome::Metronome::new(block_config, &mut stream_catalog)?;
                    sources.push(Box::new(source));
                }

                // TRANSFORMERS
                "Loop" => {
                    let tform = looper::Looper::new(block_config, &mut stream_catalog)?;
                    transformers.push(Box::new(tform));
                }
                "Combiner" => {
                    let tform = combiner::Combiner::new(block_config, &mut stream_catalog)?;
                    transformers.push(Box::new(tform));
                }

                // SINKS
                // Sinks do not get mutable references to StreamCatalog because
                // they should not be allowed to create streams.
                "AudioSink" => {
                    let sink = audio::AudioSink::new(block_config, &stream_catalog, &pa)?;
                    sinks.push(Box::new(sink));
                }

                _ => {
                    println!("TODO REMOVE: Unknown block: {}", block_config.block_type);
                }
            }
        }

        // Flush all of the input buffers.
        for _ in 0..3 {
            let state = block::PlaybackState {
                tempo: &self.tempo,
                keyboard: &self.keyboard,
            };

            for source in &mut sources {
                source.read(&state);
            }
        }

        // Run all of the blocks.
        loop {
            {
                let state = block::PlaybackState {
                    tempo: &self.tempo,
                    keyboard: &self.keyboard,
                };

                for source in &mut sources {
                    source.read(&state);
                }
                for transformer in &mut transformers {
                    transformer.transform(&state);
                }
                for sink in &mut sinks {
                    sink.write(&state);
                }
            }

            self.tempo.step();
            self.keyboard.reset();
        }
    }
}
