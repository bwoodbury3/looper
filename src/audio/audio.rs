extern crate portaudio;

extern crate block;
extern crate config;
extern crate stream;

use portaudio::{stream_flags::Flags, Blocking, DeviceIndex, DeviceInfo, Flow, PortAudio};

type PaStreamInput = portaudio::Input<stream::Sample>;
type PaStreamOutput = portaudio::Output<stream::Sample>;

/// Get a new PortAudio instance.
pub fn pa_get() -> Result<PortAudio, ()> {
    let pa = log::unwrap_abort!(PortAudio::new());
    println!("PortAudio initialized.");

    Ok(pa)
}

/// Get the audio device with the associated name.
fn get_device_index<'a>(
    pa: &'a PortAudio,
    name: &str,
    is_input: bool,
    is_output: bool,
) -> Result<(DeviceIndex, DeviceInfo<'a>), ()> {
    for device in log::unwrap_abort!(pa.devices()) {
        let (device_id, device_info) = log::unwrap_abort!(device);

        // Found the matching device -- assert that it meets our criteria.
        // If it doesn't, that's ok, let's try the next one.
        if device_info.name.contains(name) {
            if is_input && device_info.max_input_channels < 1 {
                println!("Not an input device! {}", device_info.name);
                continue;
            }
            if is_output && device_info.max_output_channels < 1 {
                println!("Not an output device! {}", device_info.name);
                continue;
            }

            return Ok((device_id, device_info));
        }
    }

    // If we reach this point, we didn't find a device.
    println!("Could not find audio device: \"{}\"", name);
    println!("Available devices:");
    for device in log::unwrap_abort!(pa.devices()) {
        let (_, device_info) = log::unwrap_abort!(device);
        println!("  - \"{}\"", device_info.name);
    }

    Err(())
}

// Audio Blocks

/// An audio source.
pub struct AudioSource {
    /// The block name.
    name: String,

    /// The input [portaudio] stream.
    pa_stream: portaudio::Stream<Blocking<<PaStreamInput as Flow>::Buffer>, PaStreamInput>,

    /// The output [looper] stream buffer.
    stream: stream::Stream,
}

/// An audio sink.
pub struct AudioSink {
    /// The block name.
    name: String,

    /// The output [portaudio] stream.
    pa_stream: portaudio::Stream<Blocking<<PaStreamOutput as Flow>::Buffer>, PaStreamOutput>,

    /// The input [looper] stream buffer.
    stream: stream::Stream,
}

/// A valid Source Block which derives input from system audio hardware. This could be an external
/// USB microphone, an audio jack, or a built-in microphone.
impl AudioSource {
    /// Initialize a PortAudio AudioSource block.
    pub fn new(
        config: &config::BlockConfig,
        stream_catalog: &mut stream::StreamCatalog,
        pa: &PortAudio,
    ) -> Result<Self, ()> {
        // Read in args.
        let output_streams = config.get_str_list("output_channels")?;
        let device_name = config.get_str("device")?;

        // Load streams.
        log::abort_if_msg!(output_streams.len() != 1, "AudioSource must have one output channel");
        let stream = stream_catalog.create_source(output_streams[0])?;

        // PortAudio wizardry follows...

        // Get the audio device.
        let (device_index, device_info) =
            log::unwrap_abort!(get_device_index(pa, device_name, true, false));

        // PortAudio stream parameters.
        let input_params = portaudio::StreamParameters::<stream::Sample>::new(
            device_index,
            1,    /* channel count */
            true, /* interleaved */
            device_info.default_low_output_latency,
        );

        // Open the audio stream.
        let settings = portaudio::InputStreamSettings {
            params: input_params,
            sample_rate: 44100.0,
            frames_per_buffer: stream::SAMPLES_PER_BUFFER as u32,
            flags: Flags::NO_FLAG,
        };
        let mut pa_stream = log::unwrap_abort_msg!(
            pa.open_blocking_stream(settings),
            format!("Error opening audio channel \"{}\"", device_name)
        );
        pa_stream.start();

        Ok(AudioSource {
            name: config.name.to_owned(),
            pa_stream: pa_stream,
            stream: stream,
        })
    }

    /// Block until some number of samples is available to read.
    fn get_available_samples(&self) -> usize {
        match self.pa_stream.read_available() {
            Ok(available) => match available {
                portaudio::StreamAvailable::Frames(frames) => return frames as usize,
                _ => {
                    println!("Stream {} has overflowed", self.name);
                    return 0;
                }
            },
            Err(e) => {
                println!("Input stream error!! {}", e.to_string());
                return 0;
            }
        };
    }
}

/// A valid Sink Block which writes a stream to a hardware speaker.
impl AudioSink {
    /// Initialize a PortAudio AudioSink block.
    pub fn new(
        config: &config::BlockConfig,
        stream_catalog: &mut stream::StreamCatalog,
        pa: &PortAudio,
    ) -> Result<Self, ()> {
        // Read in args.
        let input_streams = config.get_str_list("input_channels")?;
        let device_name = config.get_str("device")?;

        // Load streams.
        log::abort_if_msg!(input_streams.len() != 1, "AudioSink must have one input channel");
        let stream = stream_catalog.bind_sink(input_streams[0])?;

        // PortAudio wizardry follows...

        // Get the audio device.
        let (device_index, device_info) =
            log::unwrap_abort!(get_device_index(pa, device_name, false, true));

        // PortAudio stream parameters.
        let output_params = portaudio::StreamParameters::<stream::Sample>::new(
            device_index,
            1,    /* channel count */
            true, /* interleaved */
            device_info.default_low_output_latency,
        );

        // Open the audio stream.
        let settings = portaudio::OutputStreamSettings {
            params: output_params,
            sample_rate: 44100.0,
            frames_per_buffer: stream::SAMPLES_PER_BUFFER as u32,
            flags: Flags::NO_FLAG,
        };
        let mut pa_stream = log::unwrap_abort_msg!(
            pa.open_blocking_stream(settings),
            format!("Error opening audio channel \"{}\"", device_name)
        );
        pa_stream.start();

        Ok(AudioSink {
            name: config.name.to_owned(),
            pa_stream: pa_stream,
            stream: stream,
        })
    }

    /// Block until some number of samples is available to write.
    fn get_available_samples(&self) -> usize {
        match self.pa_stream.write_available() {
            Ok(available) => match available {
                portaudio::StreamAvailable::Frames(frames) => return frames as usize,
                _ => {
                    println!("Stream {} has underflowed", self.name);
                    return 0;
                }
            },
            Err(e) => {
                println!("Output stream error!! {}", e.to_string());
                return 0;
            }
        };
    }
}

impl block::Source for AudioSource {
    fn read(&mut self) {
        let mut stream = (*self.stream).borrow_mut();

        let mut index: usize = 0;
        while index < stream::SAMPLES_PER_BUFFER {
            // Get the number of available samples and read them in from PA.
            let samples_remaining = stream::SAMPLES_PER_BUFFER - index;
            let num_samples = std::cmp::min(self.get_available_samples(), samples_remaining);

            // Read samples from the buffer.
            match self.pa_stream.read(num_samples as u32) {
                Ok(samples) => {
                    for i in 0..samples.len() {
                        stream[i + index] = samples[i];
                    }
                }
                Err(e) => {
                    println!("Failed to read audio input: {}", e.to_string());
                    break;
                }
            };

            index += num_samples;
        }
    }
}

impl block::Sink for AudioSink {
    fn write(&mut self) {
        let stream = (*self.stream).borrow();

        let mut index: usize = 0;
        while index < stream::SAMPLES_PER_BUFFER {
            // Get the number of available samples and read them in from PA.
            let samples_remaining = stream::SAMPLES_PER_BUFFER - index;
            let num_samples = std::cmp::min(self.get_available_samples(), samples_remaining);

            // Write samples out to the buffer.
            match self.pa_stream.write(num_samples as u32, |output| {
                for i in 0..num_samples {
                    output[i] = stream[i + index];
                }
            }) {
                Ok(_) => (),
                Err(e) => {
                    println!("Failed to write audio output: {}", e.to_string());
                    break;
                }
            }

            index += num_samples;
        }
    }
}
