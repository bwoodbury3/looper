//! Combiner Block.
//!
//! Combiner [Transformer]:
//!     Required parameters:
//!         name: Anything
//!         type: "Combiner"
//!         input_channels: The input channel names
//!         output_channel: The output channel name

extern crate block;
extern crate config;
extern crate log;
extern crate stream;

/// The Combiner Block.
pub struct Combiner {
    /// The input streams to combine.
    input_streams: Vec<stream::Stream>,

    /// The output streams.
    output_stream: stream::Stream,
}

impl Combiner {
    /// Create a new combiner.
    pub fn new(
        config: &config::BlockConfig,
        stream_catalog: &mut stream::StreamCatalog,
    ) -> Result<Self, ()> {
        // Read in parameters.
        let input_channels = config.get_str_list("input_channels")?;
        let output_channel = config.get_str("output_channel")?;

        // Load streams.
        let mut input_streams = Vec::<stream::Stream>::with_capacity(input_channels.len());
        for channel in input_channels {
            input_streams.push(stream_catalog.bind_sink(channel)?);
        }
        let output_stream = stream_catalog.create_source(output_channel)?;

        Ok(Combiner {
            input_streams: input_streams,
            output_stream: output_stream,
        })
    }
}

impl block::Transformer for Combiner {
    fn transform(&mut self, _: &block::PlaybackState) {
        let mut output_stream = self.output_stream.borrow_mut();
        output_stream.fill(0 as stream::Sample);

        for stream in &self.input_streams {
            let input_stream = stream.borrow();
            for i in 0..stream::SAMPLES_PER_BUFFER {
                output_stream[i] += input_stream[i];
            }
        }
    }
}
