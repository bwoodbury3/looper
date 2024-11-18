//! Thin wrapper around Hound: https://crates.io/crates/hound

extern crate hound;

extern crate log;
extern crate stream;

use std::{i64, marker::PhantomData, os::macos::raw};

use stream::SAMPLE_RATE;

/// Read in a wav file as an audio clip.
pub fn read_wav_file(filename: &str) -> Result<stream::Clip, ()> {
    let mut reader = log::unwrap_abort_msg!(
        hound::WavReader::open(filename),
        format!("Could not load {} as a wav", filename)
    );

    // Read in and convert samples.
    let spec = reader.spec();
    let num_channels = spec.channels as usize;
    let clip_size = (reader.len() as usize) / num_channels;

    // Initialize an empty clip.
    let mut clip: Vec<stream::Sample> = Vec::with_capacity(clip_size);

    match spec.sample_format {
        hound::SampleFormat::Int => {
            let samples = reader.samples::<i32>();
            for (i, sample) in samples.enumerate() {
                // TODO: Support multiple channels.
                // Channels are interleaved, so if we only want channel '0' we need to grab every
                // nth sample.
                if i % num_channels == 0 {
                    let val = log::unwrap_abort!(sample);
                    clip.push(SampleConverter::<stream::Sample>::from_int(
                        val,
                        spec.bits_per_sample,
                    ));
                }
            }
        }
        hound::SampleFormat::Float => {
            let samples = reader.samples::<f32>();
            for (i, sample) in samples.enumerate() {
                // TODO: Support multiple channels.
                if i % num_channels == 0 {
                    let val = log::unwrap_abort!(sample);
                    clip.push(SampleConverter::<stream::Sample>::from_float(val));
                }
            }
        }
    }

    Ok(stream::Clip::new(clip.into()))
}

/// Write a Clip to a file using the wav audio format.
pub fn write_wav_file(clip: &stream::Clip, filename: &str) -> Result<(), ()> {
    let depth: u16 = 16;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: depth,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = log::unwrap_abort_msg!(
        hound::WavWriter::create(filename, spec),
        "Failed to initialize the WavWriter"
    );

    let raw_clip = clip.borrow();
    for i in 0..raw_clip.len() {
        log::unwrap_abort_msg!(
            writer.write_sample(SampleConverter::<stream::Sample>::to_int(raw_clip[i], depth)),
            "Failed to write wav sample to WavWriter"
        );
    }

    Ok(())
}

//=====================================

// Dead code here to allow changing of the Sample alias.

/// Get the maximum value that an be stored with this many bits (signed).
fn int_max(depth: u16) -> i64 {
    i64::MAX >> (64 - depth)
}

fn int_min(depth: u16) -> i64 {
    -1 * (1 << (depth - 1))
}

#[allow(dead_code)]
struct SampleConverter<T> {
    phantom: PhantomData<T>,
}

#[allow(dead_code)]
impl SampleConverter<f32> {
    fn from_int(num: i32, depth: u16) -> f32 {
        let max = int_max(depth) as f32;
        num as f32 / max
    }

    fn from_float(num: f32) -> f32 {
        num
    }

    fn to_int(num: f32, depth: u16) -> i32 {
        let max = int_max(depth) as i32;
        let min = int_min(depth) as i32;
        if num >= 1.0 {
            return max;
        } else if num <= -1.0 {
            return min;
        }

        (num * (max as f32)) as i32
    }
}

#[allow(dead_code)]
impl SampleConverter<i32> {
    fn from_int(num: i32, _depth: u16) -> i32 {
        num
    }

    fn from_float(num: f32, depth: u16) -> i32 {
        let max = int_max(depth) as i32;
        let min = int_min(depth) as i32;
        if num >= 1.0 {
            return max;
        } else if num <= -1.0 {
            return min;
        }

        (num * (max as f32)) as i32
    }

    fn to_int(num: i32) -> i32 {
        num
    }
}
