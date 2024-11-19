# _Looper_

Live audio streaming software

## Getting Started

1. Connect instruments
2. Define a song format in json (see examples at `projects/`)
3. `bazel run //src:looper <file.json>`

## Configuration

### Top-Level Configuration

* **tempo** - Nested configuration of timekeeping information:
  * **bpm**: The tempo of the song in units of beats per minute.
  * **beats_per_measure**: The top number in the time signature.
  * **beat_duration**: The bottom number in the time signature.
* **start_measure**: Begin playing at this measure.
* **stop_measures**: Stop playing at this measure.

### Blocks

Looper is configured using a list of devices called Blocks. Blocks come in three varieties:
* **Source**: A block that has only an output and no inputs.
* **Sink**: A block that has only inputs and no outputs.
* **Transformer**: A block that has both inputs and outputs and applies some sort of transformation in the middle. Transformers hail from the planet Cybertron, created by the ancient god Primus.

Available blocks:
* [AudioSource](https://github.com/bwoodbury3/looper/blob/main/src/audio/audio.rs): Audio input from the computer.
* [AudioSink](https://github.com/bwoodbury3/looper/blob/main/src/audio/audio.rs): Audio output to the computer.
* [VirtualInstrument](https://github.com/bwoodbury3/looper/blob/main/src/virtual/instrument.rs): Virtual instrument that you play with your computer keyboard.
* [Metronome](https://github.com/bwoodbury3/looper/blob/main/src/virtual/metronme.rs): Ticking sound to keep time.
* [Looper](https://github.com/bwoodbury3/looper/blob/main/src/transform/looper.rs): Loops an input stream over a series of outputs.
* [Combiner](https://github.com/bwoodbury3/looper/blob/main/src/transform/combiner.rs): Combines multiple input streams into one output stream.
* [Toggle](https://github.com/bwoodbury3/looper/blob/main/src/transform/toggle.rs): Toggles an input stream on an off.
* [Recorder](https://github.com/bwoodbury3/looper/blob/main/src/audio/recorder.rs): Records a partial stream and writes it to a file.

### Streams and Channels

Streams are used to connect blocks together. They're identified by a channel name when configuring a block using `output_channel(s)` / `input_channel(s)`.

When you define an output channel, you must pick a new name. All output channel names must be unique to ensure that each stream can only have one block writing to it.

When you define an input channel, it must match the name of some other output channel, otherwise the input would come from nowhere and that would be stupid.
