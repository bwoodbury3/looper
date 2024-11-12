//! Thin wrapper around Hound: https://crates.io/crates/hound

extern crate hound;

extern crate log;
extern crate stream;

pub fn read_wav_file(filename: &str) -> Result<stream::Clip, ()> {
    let mut reader = log::unwrap_abort_msg!(
        hound::WavReader::open(filename),
        format!("Could not load {} as a wav", filename)
    );

    // Initialize an empty clip.
    let mut clip: Vec<stream::Sample> = Vec::with_capacity(reader.len() as usize);

    // Copy Wav file samples over to the clip.
    let samples = reader.samples::<i32>();
    for sample in samples {
        // match sample {
        //     Ok(_) => (),
        //     Err(e) => {
        //         println!("Error unwrapping sample: {}", e.to_string());
        //     }
        // };
        clip.push(log::unwrap_abort!(sample) as f32);
    }

    Ok(stream::Clip::new(clip.into()))
}
