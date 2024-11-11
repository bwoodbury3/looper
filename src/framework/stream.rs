use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;

/// The root sample type.
pub type Sample = f32;

/// The number of samples in a buffer. This is the number of samples sent to
/// sink blocks, and also the number of samples expected from source blocks.
pub const SAMPLES_PER_BUFFER: usize = 256;

/// The sample rate of the audio stream [samples/s].
pub const SAMPLE_RATE: i32 = 44100;

/// Stream type shorthand.
pub type Stream = Rc<RefCell<[Sample; SAMPLES_PER_BUFFER]>>;

/// A catalog of streams.
pub struct StreamCatalog {
    streams: HashMap<String, Stream>,
}

impl StreamCatalog {
    pub fn new() -> StreamCatalog {
        StreamCatalog {
            streams: HashMap::new(),
        }
    }

    /// Create a new stream source.
    pub fn create_source(&mut self, name: &str) -> Result<Stream, ()> {
        let stream: Stream = Rc::new(RefCell::new([0 as Sample; SAMPLES_PER_BUFFER]));
        match self.streams.insert(name.to_string(), stream.clone()) {
            None => Ok(stream.clone()),
            Some(_) => {
                println!("Cannot create duplicate stream: {}", name);
                Err(())
            }
        }
    }

    /// Bind to a sink's outputs.
    pub fn bind_sink(&self, name: &str) -> Result<Stream, ()> {
        match self.streams.get(name) {
            Some(stream) => Ok(stream.clone()),
            None => {
                println!("Could not find stream: {}", &name);
                Err(())
            }
        }
    }
}
