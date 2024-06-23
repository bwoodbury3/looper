Programmable loop pedal.

1. Connect instruments
2. Define a song format in json (see below)
3. `bazel run //src:looper <file.json>`

Example config file which includes a keyboard instrument, a built-in metronome, and a drum loop.
```json
{
    "config": {
        "tempo": {
            "bpm": 100,
            "beats_per_measure": 4,
            "beat_duration": 4
        }
    },
    "devices": [
        {
            "name": "drums",
            "type": "VirtualInstrument",
            "instrument": "drums1",
            "volume": 0.2,
            "output_channel": "drums"
        },
        {
            "name": "metronome",
            "type": "Metronome",
            "start_measure": 1,
            "stop_measure": 4,
            "freq": 440.0,
            "output_channel": "metronome"
        },
        {
            "name": "drum_loop",
            "type": "Loop",
            "start_measure": 2,
            "stop_measure": 3,
            "replay_intervals": [
                {
                    "start_measure": 3,
                    "stop_measure": 6,
                    "offset": 0
                }
            ],
            "input_channels": ["drums"],
            "output_channels": ["looped_drums"]
        },
        {
            "name": "combiner",
            "type": "Combiner",
            "input_channels": ["drums", "metronome", "looped_drums"],
            "output_channels": ["audio_out"]
        },
        {
            "name": "laptop_speaker",
            "type": "AudioOutput",
            "device": "Built-in Output",
            "input_channel": "audio_out"
        }
    ]
}
```
