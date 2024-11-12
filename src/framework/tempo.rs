//! Library for managing tempo.
//!
//! Example
//! ```
//! if tempo.on_beat() {
//!     // Play metronome sound.
//! }
//! ```

extern crate config;
extern crate stream;

/// Struct for managing the tempo of the project and the current state.
pub struct Tempo {
    // Configuration parameters

    /// Beats per minute.
    bpm: i32,

    /// The number of beats per measure (top of the time signature).
    beats_per_measure: i32,

    /// The duration of the beat (bottom of the time signature).
    beat_duration: i32,

    // Precomputed constants

    /// The number of seconds that pass in each step.
    seconds_per_step: f64,

    /// The number of seconds that pass for each beat.
    seconds_per_beat: f64,

    /// The number of beats that pass for each step.
    beats_per_step: f64,

    /// The number of measures that pass for each step.
    measures_per_step: f64,

    /// The number of samples per measure.
    samples_per_measure: f64,

    /// The largest number of beats that can pass in a single cycle.
    beat_epsilon: f64,

    /// The largest number of measures that can pass in a single cycle.
    measure_epsilon: f64,

    // Current state

    /// The current chunk.
    current_chunk: usize,

    /// The current beat.
    current_beat: f64,

    /// The current runtime in seconds.
    current_time_s: f64
}

impl Tempo {
    /// Initialize a new tempo and reset the state to 0.
    pub fn new(project: &config::ProjectConfig) -> Result<Self, String> {
        let tempo = &project.global_config["tempo"];
        let bpm = match tempo["bpm"].as_i32() {
            Some(v) => v,
            None => {
                println!("\"bpm\" not specified, using default '120'");
                120
            }
        };
        let beats_per_measure = match tempo["beats_per_measure"].as_i32() {
            Some(v) => v,
            None => {
                println!("\"beats_per_measure\" not specified, using default '4'");
                4
            }
        };
        let beat_duration = match tempo["beat_duration"].as_i32() {
            Some(v) => v,
            None => {
                println!("\"beat_duration\" not specified, using default '4'");
                4
            }
        };

        let seconds_per_step = stream::SAMPLES_PER_BUFFER as f64 /
                               stream::SAMPLE_RATE as f64;
        let seconds_per_beat = 60.0 / bpm as f64;
        let beats_per_step = seconds_per_step / seconds_per_beat;
        let measures_per_step = beats_per_step / beats_per_measure as f64;
        let samples_per_measure = stream::SAMPLES_PER_BUFFER as f64 / measures_per_step;
        let beat_epsilon = beats_per_step;
        let measure_epsilon = measures_per_step;

        Ok(Tempo {
            bpm: bpm,
            beats_per_measure: beats_per_measure,
            beat_duration: beat_duration,

            seconds_per_step: seconds_per_step,
            seconds_per_beat: seconds_per_beat,
            beats_per_step: beats_per_step,
            measures_per_step: measures_per_step,
            samples_per_measure: samples_per_measure,
            beat_epsilon: beat_epsilon,
            measure_epsilon: measure_epsilon,

            current_chunk: 0,
            current_beat: 0.0,
            current_time_s: 0.0
        })
    }

    /// Step the tempo forward.
    ///
    /// DANGER: This should only be called by framework code, not by any blocks.
    /// Blocks should not have a mutable reference to tempo, so this shouldn't be possible.
    pub fn step(&mut self) {
        self.current_chunk += 1;
        self.current_time_s += self.seconds_per_step;
        self.current_beat += self.beats_per_step;
    }

    /// The current measure.
    pub fn current_measure(&self) -> f64 {
        return self.current_beat / self.beats_per_measure as f64;
    }

    /// Whether the current time is in the provided measure.
    pub fn in_measure(&self, m1: f64, m2: f64, step_offset: f64) -> bool {
        let curr = self.current_measure() + self.measures_per_step * step_offset;

        return m1 - self.measure_epsilon <= curr && curr < m2;
    }

    /// Returns true if this chunk lands on the start of a beat.
    pub fn on_beat(&self, beat_offset: f64) -> bool {
        let beat: f64 = self.current_beat - beat_offset;
        let diff: f64 = beat - beat.floor();
        return 0.0 <= diff && diff < self.beat_epsilon;
    }
}