//! Thin wrapper around Hound: <https://crates.io/crates/hound>

extern crate hound;

extern crate log;
extern crate stream;

use std::{i32, i64, marker::PhantomData};

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

// WAV<->stream::Sample conversion code follows. Conversion code has been implemented for f32 and
// i32 types only, so if the stream::Sample typedef is ever changed to something different you'll
// have to go sort that out yourself.

/// Get the maximum value that an be stored with this many bits (signed).
fn int_max(depth: u16) -> i64 {
    i64::MAX >> (64 - depth)
}

struct SampleConverter<T> {
    phantom: PhantomData<T>,
}

impl SampleConverter<f32> {
    // Takes a wav fixed-point sample and converts it to a floating point value between [-1, 1].
    fn from_int(num: i32, depth: u16) -> f32 {
        let max = int_max(depth) as f32;
        let num = num as f32 / max;
        num.max(-1.0).min(1.0)
    }

    // Does nothing because the wav sample is already the correct format.
    fn from_float(num: f32) -> f32 {
        num
    }

    // Takes an f32 sample and converts it to a fixed-point value in the range of the bit depth.
    fn to_int(num: f32, depth: u16) -> i32 {
        let max = int_max(depth) as i32;
        let num = num.max(-1.0).min(1.0);

        (num * (max as f32)) as i32
    }
}

#[allow(dead_code)] // This code is dead because Sample == f32
impl SampleConverter<i32> {
    // Scales a wav fixed-point sample at the provided depth up to 32-bit fixed-point.
    fn from_int(num: i32, depth: u16) -> i32 {
        num << (32 - depth)
    }

    // Converts a floating point sample to 32-bit fixed point.
    fn from_float(num: f32) -> i32 {
        let num = num.max(-1.0).min(1.0);
        (num * (i32::MAX as f32)) as i32
    }

    // Scales a 32-bit fixed point sample down to a fixed-point sample of the provided depth.
    fn to_int(num: i32, depth: u16) -> i32 {
        num >> (32 - depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::assert_approx_eq;

    #[test]
    fn test_convert_f32() {
        // Convert from i32->f32 at two different bit depths.
        assert_approx_eq!(SampleConverter::<f32>::from_int(7, 4), 1.0, 1e-5f32);
        assert_approx_eq!(SampleConverter::<f32>::from_int(0, 4), 0.0, 1e-5f32);
        assert_approx_eq!(SampleConverter::<f32>::from_int(-7, 4), -1.0, 1e-5f32);

        assert_approx_eq!(SampleConverter::<f32>::from_int(32767, 16), 1.0, 1e-3f32);
        assert_approx_eq!(SampleConverter::<f32>::from_int(16383, 16), 0.5, 1e-3f32);
        assert_approx_eq!(SampleConverter::<f32>::from_int(0, 16), 0.0, 1e-3f32);
        assert_approx_eq!(SampleConverter::<f32>::from_int(-16383, 16), -0.5, 1e-3f32);
        assert_approx_eq!(SampleConverter::<f32>::from_int(-32767, 16), -1.0, 1e-3f32);

        // f32->f32 is a simple passthrough.
        assert_eq!(SampleConverter::<f32>::from_float(3.14159), 3.14159);

        // Convert from f32->i32 at two different bit depths.
        assert_eq!(SampleConverter::<f32>::to_int(1.0, 16), 32767);
        assert_eq!(SampleConverter::<f32>::to_int(0.5, 16), 16383);
        assert_eq!(SampleConverter::<f32>::to_int(0.0, 16), 0);
        assert_eq!(SampleConverter::<f32>::to_int(-0.5, 16), -16383);
        assert_eq!(SampleConverter::<f32>::to_int(-1.0, 16), -32767);

        assert_eq!(SampleConverter::<f32>::to_int(1.0, 4), 7);
        assert_eq!(SampleConverter::<f32>::to_int(0.0, 4), 0);
        assert_eq!(SampleConverter::<f32>::to_int(-1.0, 4), -7);
    }

    #[test]
    fn test_convert_i32() {
        // i32->i32 is a scale by the bit depth ratio
        assert_eq!(SampleConverter::<i32>::from_int(7, 8), 117440512);
        assert_eq!(SampleConverter::<i32>::from_int(3, 16), 196608);

        // Convert f32->i32 at two different bit depths.
        assert_eq!(SampleConverter::<i32>::from_float(1.0), i32::MAX);
        assert_eq!(SampleConverter::<i32>::from_float(0.5), i32::MAX / 2 + 1);
        assert_eq!(SampleConverter::<i32>::from_float(0.0), 0);
        assert_eq!(SampleConverter::<i32>::from_float(-0.5), i32::MIN / 2);
        assert_eq!(SampleConverter::<i32>::from_float(-1.0), i32::MIN);

        // i32->i32 is a simple passthrough.
        assert_eq!(SampleConverter::<i32>::to_int(118365240, 8), 7);
        assert_eq!(SampleConverter::<i32>::to_int(196614, 16), 3);
    }
}
