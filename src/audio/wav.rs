//! Thin wrapper around Hound: https://crates.io/crates/hound

extern crate hound;

extern crate log;
extern crate stream;

use std::marker::PhantomData;

/// Read in a wav file as an audio clip.
pub fn read_wav_file(filename: &str) -> Result<stream::Clip, ()> {
    let mut reader = log::unwrap_abort_msg!(
        hound::WavReader::open(filename),
        format!("Could not load {} as a wav", filename)
    );
    // Initialize an empty clip.
    let mut clip: Vec<stream::Sample> = Vec::with_capacity(reader.len() as usize);

    // Read in and convert samples.
    let spec = reader.spec();
    match spec.sample_format {
        hound::SampleFormat::Int => {
            let samples = reader.samples::<i32>();
            for sample in samples {
                let val = log::unwrap_abort!(sample);
                clip.push(SampleConverter::<stream::Sample>::from_int(val, spec.bits_per_sample));
            }
        }
        hound::SampleFormat::Float => {
            let samples = reader.samples::<f32>();
            for sample in samples {
                let val = log::unwrap_abort!(sample);
                clip.push(SampleConverter::<stream::Sample>::from_float(val));
            }
        }
    }

    Ok(stream::Clip::new(clip.into()))
}

//=====================================

// Dead code here to allow changing of the Sample alias.

#[allow(dead_code)]
struct SampleConverter<T> {
    phantom: PhantomData<T>,
}

#[allow(dead_code)]
impl SampleConverter<f32> {
    fn from_int(num: i32, depth: u16) -> f32 {
        let max = (1 << (depth - 1) - 1) as f32;
        num as f32 / max
    }

    fn from_float(num: f32) -> f32 {
        num
    }
}

#[allow(dead_code)]
impl SampleConverter<i32> {
    fn from_int(num: i32, _depth: u16) -> i32 {
        num
    }

    fn from_float(num: f32) -> i32 {
        if num >= 1.0 {
            return i32::MAX;
        } else if num <= -1.0 {
            return i32::MIN + 1;
        }

        (num * (i32::MAX as f32)) as i32
    }
}
