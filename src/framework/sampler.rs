//! A simple module that can play an arbitrary audio clip into a stream.

extern crate log;
extern crate stream;

pub struct Sampler {
    /// The clip being played.
    clip: Option<stream::Clip>,

    /// The index into the current clip.
    clip_index: usize,

    /// Whether the clip is currently playing.
    is_playing: bool,

    /// Whether or not the Sampler is in loop mode.
    is_loop: bool,
}

impl Sampler {
    /// Construct a new empty sampler.
    pub fn new() -> Self {
        Sampler {
            clip: None,
            clip_index: 0,
            is_playing: false,
            is_loop: false,
        }
    }

    /// Play some samples.
    pub fn play(&mut self, clip: &stream::Clip, is_loop: bool) {
        self.clip = Some(clip.clone());
        self.is_loop = is_loop;

        self.is_playing = true;
        self.clip_index = 0;
    }

    /// Skip some samples.
    pub fn skip(&mut self, num_samples: usize) {
        let clip_rc = match &self.clip {
            Some(v) => v,
            None => {
                return;
            }
        };
        let clip = clip_rc.borrow();

        self.clip_index = std::cmp::min(self.clip_index + num_samples, clip.len());
    }

    /// Play some samples.
    pub fn is_playing(&self) -> bool {
        return self.is_playing;
    }

    /// Stop playing the current sample.
    pub fn stop(&mut self) {
        self.clip = None;
        self.is_playing = false;
        self.clip_index = 0;
    }

    /// Get the next bunch of samples.
    ///
    /// Sampler will *add* the playback to the current stream instead of overwriting the stream.
    pub fn next(&mut self, stream: &mut stream::RawStream) {
        // Not playing, nothing to do.
        if !self.is_playing {
            return;
        }

        let should_stop: bool;

        // Play the current clip.
        {
            let clip_rc = match &(self.clip) {
                Some(v) => v,
                None => {
                    panic!("Sampler was playing without a valid Clip");
                }
            };
            let clip = clip_rc.borrow();

            // Read enough samples to fill the stream, unless we run out of clip.
            let start_index = self.clip_index;
            let stop_index = std::cmp::min(start_index + stream.len(), clip.len());

            // Copy the clip slice to the output stream.
            for i in start_index..stop_index {
                stream[i - start_index] += clip[i];
            }

            self.clip_index = stop_index;
            should_stop = stop_index == clip.len();
        }

        // Stop the clip or restart the loop if we're at the end.
        if should_stop {
            self.clip_index = 0;
            if !self.is_loop {
                self.stop();
            }
        }
    }
}
