extern crate audio;
extern crate block;
extern crate config;
extern crate log;

pub struct Runner {
    project: config::ProjectConfig,
}

impl Runner {
    /// Create a new runner.
    pub fn new(filename: &str) -> Result<Self, String> {
        // Read in configuration
        let project = config::ProjectConfig::new(filename)?;

        // Initialize the runner.
        Ok(Runner { project: project })
    }

    /// Run!
    pub fn run(&self) -> Result<(), ()> {
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

                // TRANSFORMERS

                // SINKS
                "AudioSink" => {
                    let sink = audio::AudioSink::new(block_config, &mut stream_catalog, &pa)?;
                    sinks.push(Box::new(sink));
                }

                _ => {
                    println!("TODO REMOVE: Unknown block: {}", block_config.block_type);
                }
            }
        }

        // Flush all of the input buffers.
        for _ in 0..3 {
            for source in &mut sources {
                source.read();
            }
        }

        // Run all of the blocks.
        loop {
            for source in &mut sources {
                source.read();
            }
            for transformer in &mut transformers {
                transformer.transform();
            }
            for sink in &mut sinks {
                sink.write();
            }
        }
    }
}
