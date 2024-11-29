//! Command line tool for modifying wav audio files.

extern crate log;
extern crate stream;
extern crate wav;

use std::env;
use std::process;
use std::str::FromStr;

fn help() -> process::ExitCode {
    println!("~ WAV TOOL ~");
    println!();
    println!("usage:");
    println!("$ wav_tool input_file.wav output_file.wav CMD");
    println!();
    println!("available commands:");
    println!("    trim <duration>      Trim the wav file to the specified duration");
    println!("    volume <amplitude>   Change the volume of the input file");
    println!("    fade <duration>      Apply a linear fade-out effect on the audio");

    return process::ExitCode::FAILURE;
}

fn trim(rc_clip: stream::Clip, duration: f32) -> stream::Clip {
    {
        let mut clip: std::cell::RefMut<'_, Vec<f32>> = rc_clip.borrow_mut();
        clip.truncate((duration * stream::SAMPLE_RATE as f32) as usize);
    }
    rc_clip
}

fn scale(rc_clip: stream::Clip, volume: f32) -> stream::Clip {
    {
        let mut clip = rc_clip.borrow_mut();
        for i in 0..clip.len() {
            clip[i] *= volume;
        }
    }
    rc_clip
}

fn fade(rc_clip: stream::Clip, duration: f32) -> stream::Clip {
    let fade_len = (duration * stream::SAMPLE_RATE as f32) as usize;
    {
        let mut clip = rc_clip.borrow_mut();
        assert!(fade_len < clip.len(), "The specified fade duration is longer than the clip");

        let begin = clip.len() - fade_len;
        for i in 0..fade_len {
            let ratio = (fade_len - i) as f32 / fade_len as f32;
            clip[begin + i] *= ratio;
        }
    }
    rc_clip
}

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        return help();
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let cmd = &args[3];

    let input_samples = wav::read_wav_file(&input_file).unwrap();

    let transformed: stream::Clip = match cmd.as_str() {
        "trim" => {
            let duration = f32::from_str(&args[4]).expect("duration must be a float");
            trim(input_samples, duration)
        }
        "volume" => {
            let amplitude = f32::from_str(&args[4]).expect("volume must be a float");
            scale(input_samples, amplitude)
        }
        "fade" => {
            let duration = f32::from_str(&args[4]).expect("duration must be a float");
            fade(input_samples, duration)
        }
        unknown_cmd => {
            println!("Command not recognized: {}", unknown_cmd);
            return help();
        }
    };

    wav::write_wav_file(&transformed, &output_file).expect("Failed to write to output file");

    process::ExitCode::SUCCESS
}
