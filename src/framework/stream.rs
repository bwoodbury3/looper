//! Trivial streaming library
//!
//! A Stream is a Rc<RefCell<?>> wrapper around a Sample array with runtime checks for
//! simultaneous borrows between blocks.
//!
//! Streams should be queried at Block initialization (::new) with a reference saved off for use
//! at runtime.
//!
//! All output/source streams are considered read/write and must be *created* using create_source()
//! while input/sink streams should only use bind_sink(). This assumption is baked into the runtime
//! control flow and if not followed will cause projects to fail to load.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;

/// The root sample type.
pub type Sample = f32;

/// Zero constant of type Sample.
pub const ZERO: Sample = 0 as Sample;

/// The number of samples in a buffer. This is the number of samples sent to
/// sink blocks, and also the number of samples expected from source blocks.
pub const SAMPLES_PER_BUFFER: usize = 256;

/// The sample rate of the audio stream [samples/s].
pub const SAMPLE_RATE: i32 = 44100;

/// Stream type shorthand.
pub type RawStream = [Sample; SAMPLES_PER_BUFFER];
pub type Stream = Rc<RefCell<RawStream>>;

/// A variable-sized audio clip.
pub type RawClip = Vec<Sample>;
pub type Clip = Rc<RefCell<RawClip>>;

pub trait Scalable {
    /// Scale the volume of an audio unit.
    fn scale(&mut self, volume: f32);
}

impl Scalable for RawStream {
    fn scale(&mut self, volume: f32) {
        for i in 0..self.len() {
            self[i] *= volume;
        }
    }
}

impl Scalable for RawClip {
    fn scale(&mut self, volume: f32) {
        for i in 0..self.len() {
            self[i] *= volume;
        }
    }
}

/// A catalog of streams. Blocks use this at creation to create or bind to output or input streams
/// respectively. Streams must only be borrowed by blocks at runtime and cleaned up before their
/// relevant read()/write()/transform() is returned. Saving off a mutable reference at runtime is
/// prohibited, as a mutable stream reference will cause other blocks which depend on the stream to
/// panic.
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
        let stream: Stream = Rc::new(RefCell::new([ZERO; SAMPLES_PER_BUFFER]));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_catalog() {
        let mut sc = StreamCatalog::new();

        // Create a new stream, bind to it, and verify that the two streams point to the same thing
        let name = "stream";
        let stream_create = sc.create_source(name).unwrap();
        let stream_bind = sc.bind_sink(name).unwrap();
        stream_create.borrow_mut()[7] = 12 as Sample;
        assert_eq!(stream_bind.borrow()[0], ZERO);
        assert_eq!(stream_bind.borrow()[7], 12 as Sample);
    }

    #[test]
    fn test_bind_without_create() {
        let sc = StreamCatalog::new();

        // Verify that binding to a stream that doesn't exist raises an Err
        let name = "stream";
        match sc.bind_sink(name) {
            Ok(_) => {
                panic!("Stream bind should have failed.")
            }
            Err(_) => {}
        };
    }

    #[test]
    fn test_double_create() {
        let mut sc = StreamCatalog::new();

        // Verify that binding to a stream that doesn't exist raises an Err
        let name = "stream";
        sc.create_source(name).unwrap();
        match sc.create_source(name) {
            Ok(_) => {
                panic!("Stream should not have been created twice.")
            }
            Err(_) => {}
        };
    }

    #[test]
    fn test_scale_stream() {
        let mut stream: RawStream = [ZERO; SAMPLES_PER_BUFFER];
        for i in 0..SAMPLES_PER_BUFFER {
            stream[i] = i as Sample;
        }
        stream.scale(0.5);

        // Assert all the values are twice as big now.
        for i in 0..SAMPLES_PER_BUFFER {
            assert_eq!(stream[i], i as Sample * 0.5);
        }
    }

    #[test]
    fn test_scale_clip() {
        let clip_size = 20;
        let mut clip: RawClip = RawClip::with_capacity(clip_size);
        for i in 0..clip_size {
            clip.push(i as Sample);
        }
        clip.scale(0.5);

        // Assert all the values are twice as big now.
        for i in 0..clip_size {
            assert_eq!(clip[i], i as Sample * 0.5);
        }
    }
}
