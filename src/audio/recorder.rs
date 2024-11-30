//! Recorder Block
//!
//! The recorder block listens and saves a .wav file for every input segment. The file is saved in
//! memory while the song is playing and is written to disk on cleanup. For this reason, the
//! Looper configuration must be COMPLETE in order for the recording to work. In other words, if
//! you Ctrl+C, you won't get your audio!
//!
//! Recorder \[Sink\]:
//!     Required parameters:
//!         name: Anything. Note that this will be the name of the audio file.
//!         type: "Recorder"
//!         directory: The path to the directory on the disk where the audio file will be saved.
//!         input_chanel: The channel to record.
//!         segments: A list of exactly one "input" segment to record.

extern crate block;
extern crate config;
extern crate log;
extern crate segment;
extern crate stream;
extern crate tempo;
extern crate wav;

use std::fs;
use std::path;

pub struct Recorder {
    /// The name of the block. This gets used in the name of the recorded file.
    name: String,

    /// The input stream to record.
    stream: stream::Stream,

    /// The segment to record.
    segment: segment::Segment,

    /// The .wav filename on disk to store the file.
    filename: String,

    /// The clip recording the segment.
    clip: stream::Clip,

    /// Whether the stream is finished recording.
    complete: bool,

    /// WHether the stream is disabled.
    disabled: bool,
}

impl Recorder {
    pub fn new(
        config: &config::BlockConfig,
        stream_catalog: &stream::StreamCatalog,
    ) -> Result<Self, ()> {
        // Read in parameters.
        let input_channel = config.get_str("input_channel")?;
        let directory = config.get_str("directory")?;
        let disabled = config.get_bool_opt("disabled", false)?;
        let mut segments = config.get_segments()?;

        // Load stream.
        let stream = stream_catalog.bind_sink(input_channel)?;

        // Validate the filename and create it on disk.
        let filename = format!("{}.wav", config.name);
        let pathbuf = path::Path::new(directory).join(filename);
        let path = match pathbuf.to_str() {
            Some(v) => v,
            None => {
                log::abort_msg!(format!("Invalid path: {}", pathbuf.display()));
            }
        };
        log::unwrap_abort_msg!(
            fs::OpenOptions::new().write(true).create(true).open(path),
            format!("Error creating file: {}", path)
        );

        // Check segments.
        log::abort_if_msg!(segments.len() != 1, "Recorder requires exactly 1 segment.");
        let segment = segments.remove(0);
        log::abort_if_msg!(
            segment.segment_type == segment::SegmentType::Output,
            "Recorder only accepts \"input\" segments"
        );

        // Create the clip.
        let clip = stream::Clip::new(stream::RawClip::new().into());

        Ok(Recorder {
            name: config.name.to_owned(),
            stream: stream,
            segment: segment,
            filename: path.to_owned(),
            clip: clip,
            complete: false,
            disabled: disabled,
        })
    }
}

impl block::Sink for Recorder {
    fn write(&mut self, state: &block::PlaybackState) {
        let tempo = state.tempo;

        // We've already recorded. Nothing to do.
        if self.complete || self.disabled {
            return;
        }

        // Record the input samples if we're in the recording segment.
        if tempo.in_measure(self.segment.start, self.segment.stop, 0.0) {
            let mut clip = self.clip.borrow_mut();
            if clip.is_empty() {
                println!("Recording started: {}", self.name);
            }

            // Resize the recording clip.
            let start_index = clip.len();
            clip.resize(start_index + stream::SAMPLES_PER_BUFFER, stream::ZERO);

            // Add the input streams to the new extended portion of the recording.
            let stream = self.stream.borrow();
            for i in 0..stream::SAMPLES_PER_BUFFER {
                clip[start_index + i] = stream[i];
            }
        } else if tempo.current_measure() > self.segment.stop {
            println!("Recording complete: {}", self.name);
            self.complete = true;
        }
    }

    fn cleanup(&mut self) {
        if !self.complete {
            println!("Abandoning recording \"{}\" because the segment wasn't complete.", self.name);
            return;
        }
        if !self.disabled {
            println!("Recorder {} is disabled, nothing to do.", self.name);
            return;
        }

        match wav::write_wav_file(&self.clip, &self.filename) {
            Ok(_) => {}
            Err(_) => {
                println!(
                    "Could not write recording to disk \"{}\" => {}",
                    self.name, self.filename
                );
            }
        }

        println!("Saved \"{}\" recording to => {}", self.name, self.filename);
    }
}
