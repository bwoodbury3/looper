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
    pub bpm: i32,

    /// The number of beats per measure (top of the time signature).
    pub beats_per_measure: i32,

    /// The duration of the beat (bottom of the time signature).
    pub beat_duration: i32,

    // Precomputed constants
    /// The number of seconds that pass in each step.
    pub seconds_per_step: f32,

    /// The number of seconds that pass for each beat.
    pub seconds_per_beat: f32,

    /// The number of beats that pass for each step.
    pub beats_per_step: f32,

    /// The number of measures that pass for each step.
    pub measures_per_step: f32,

    /// The number of samples per measure.
    pub samples_per_measure: f32,

    /// The largest number of beats that can pass in a single cycle.
    pub beat_epsilon: f32,

    /// The largest number of measures that can pass in a single cycle.
    pub measure_epsilon: f32,

    // Current state
    /// The current chunk.
    pub current_chunk: usize,

    /// The current beat.
    pub current_beat: f32,

    /// The current runtime in seconds.
    pub current_time_s: f32,
}

impl Tempo {
    /// Initialize a new tempo and reset the state to 0.
    pub fn new(project: &config::ProjectConfig) -> Result<Self, String> {
        let tempo = &project.tempo_config;
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

        let seconds_per_step = stream::SAMPLES_PER_BUFFER as f32 / stream::SAMPLE_RATE as f32;
        let seconds_per_beat = 60.0 / bpm as f32;
        let beats_per_step = seconds_per_step / seconds_per_beat;
        let measures_per_step = beats_per_step / beats_per_measure as f32;
        let samples_per_measure = stream::SAMPLES_PER_BUFFER as f32 / measures_per_step;
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
            current_time_s: 0.0,
        })
    }

    /// Step the tempo forward.
    ///
    /// DANGER: This should only be called by framework code, not by any blocks.
    /// Blocks should not have a mutable reference to tempo, so this shouldn't be possible.
    pub fn step(&mut self, num_chunks: i32) {
        self.current_chunk += num_chunks as usize;
        self.current_time_s += num_chunks as f32 * self.seconds_per_step;
        self.current_beat += num_chunks as f32 * self.beats_per_step;
    }

    /// Skip a number of measures forward.
    pub fn skip(&mut self, num_measures: f32) {
        let num_chunks = num_measures / self.measures_per_step;
        self.step(num_chunks.round() as i32);
    }

    /// The current measure.
    pub fn current_measure(&self) -> f32 {
        return self.current_beat / self.beats_per_measure as f32;
    }

    /// Whether the current time is in the provided measure.
    ///
    /// # Arguments
    ///
    /// * m1 - The beginning of the measure window
    /// * m2 - The end of the measure window
    /// * step_offset - A +/- offset to apply when calculating the window. This
    ///                 is typically used to compensate for latency.
    pub fn in_measure(&self, m1: f32, m2: f32, step_offset: f32) -> bool {
        let curr = self.current_measure() + self.measures_per_step * step_offset;

        return m1 - self.measure_epsilon <= curr && curr < m2;
    }

    /// Returns true if this chunk lands on the start of a beat.
    ///
    /// # Arguments
    ///
    /// * step_offset - A +/- offset to apply when calculating the window. This
    ///                 is typically used to compensate for latency.
    pub fn on_beat(&self, beat_offset: f32) -> bool {
        let beat: f32 = self.current_beat - beat_offset;
        let diff: f32 = beat - beat.floor();
        return 0.0 <= diff && diff < self.beat_epsilon;
    }
}
