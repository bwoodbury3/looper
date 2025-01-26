# _Looper_

Live audio streaming software

## Getting Started

1. Connect instruments
2. Define a song format in yaml (see examples at `projects/`)
3. `bazel run //src:looper <file.yaml>`

## Configuration

### Top-Level Configuration

* **tempo** - Nested configuration of timekeeping information:
  * **bpm**: The tempo of the song in units of beats per minute.
  * **beats_per_measure**: The top number in the time signature.
  * **beat_duration**: The bottom number in the time signature.
* **start_measure**: Begin playing at this measure.
* **stop_measure**: Stop playing at this measure.
* **variables**: Named variables that can be substituted for numbers elsewhere in the project.

### Blocks

Looper is configured using a list of devices called Blocks. Blocks come in three varieties:
* **Source**: A block that has only an output and no inputs.
* **Sink**: A block that has only inputs and no outputs.
* **Transformer**: A block that has both inputs and outputs and applies some sort of transformation in the middle. Transformers hail from the planet Cybertron, created by the ancient god Primus.

Available blocks:
* [AudioSource](https://github.com/bwoodbury3/looper/blob/main/src/audio/audio.rs): External audio input (an amplifier or a USB microphone).
* [AudioSink](https://github.com/bwoodbury3/looper/blob/main/src/audio/audio.rs): Audio output from the program (a speaker or a file).
* [VirtualInstrument](https://github.com/bwoodbury3/looper/blob/main/src/virtual/instrument.rs): Virtual instrument that you play with your computer keyboard.
* [Metronome](https://github.com/bwoodbury3/looper/blob/main/src/virtual/metronme.rs): Ticking sound to keep time.
* [Looper](https://github.com/bwoodbury3/looper/blob/main/src/transform/looper.rs): Loops an input stream over a series of outputs.
* [Combiner](https://github.com/bwoodbury3/looper/blob/main/src/transform/combiner.rs): Combines multiple input streams into one output stream.
* [Toggle](https://github.com/bwoodbury3/looper/blob/main/src/transform/toggle.rs): Toggles an input stream on an off.
* [Recorder](https://github.com/bwoodbury3/looper/blob/main/src/audio/recorder.rs): Records a partial stream and writes it to a file.
* [LowPass](https://github.com/bwoodbury3/looper/blob/main/src/transform/low_pass.rs): Adds a low pass filter at a configurable frequency.

### Streams and Channels

Streams are used to connect blocks together. They're identified by a channel name when configuring a block using `output_channel(s)` / `input_channel(s)`.

When you define an output channel, you must pick a new name. All output channel names must be unique to ensure that each stream can only have one block writing to it.

When you define an input channel, it must match the name of some other output channel, otherwise the input would come from nowhere and that would be stupid.

### Variables

As mentioned above, named variables can be defined at the top-level configuration and substituted for arbitrary values elsewhere in the project. For example, take this common case:

```yaml
devices:
-   name: Drumloop Chorus
    type: Loop
    input_channels:
    -   drums
    output_channel: drumloop-chorus
    segments:
    -   start: 1
        stop: 3
        type: input
    -   start: 15 # Chorus 1
        stop: 21
        type: output
    -   start: 33 # Chorus 2
        stop: 39
        type: output
```

Variables can be used to organize the project better and keep blocks in sync if the song structure changes:

```yaml
config:
    variables:
        # Define a top-level song structure
        CHORUS_1_START: 15
        CHORUS_1_END: 21
        CHORUS_2_START: 33
        CHORUS_2_END: 39

devices:
-   name: Drumloop Chorus
    type: Loop
    input_channels:
    -   drums
    output_channel: drumloop-chorus
    segments:
    -   start: 1
        stop: 3
        type: input
    -   start: CHORUS_1_START
        stop: CHORUS_1_END
        type: output
    -   start: CHORUS_2_START
        stop: CHORUS_2_END
        type: output
```
