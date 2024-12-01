//! Library for managing tempo.
//!
//! Example
//! ```
//! if tempo.on_beat() {
//!     // Play metronome sound.
//! }
//! ```

extern crate config;
extern crate log;
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
    /// The number of steps in a single beat.
    pub steps_per_beat: i32,

    /// The number of steps in a single measure.
    pub steps_per_measure: i32,

    /// The number of samples in a measure.
    pub samples_per_measure: i32,

    // Current state
    /// The current step.
    pub current_step: i32,

    /// The current beat.
    pub current_beat_i32: i32,
    pub current_beat_f32: f32,
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

        // Calculate beat and measure widths.
        let steps_per_beat = (seconds_per_beat / seconds_per_step) as i32;
        let steps_per_measure = steps_per_beat * beats_per_measure;
        let samples_per_measure = steps_per_measure * stream::SAMPLES_PER_BUFFER as i32;

        Ok(Tempo {
            bpm: bpm,
            beats_per_measure: beats_per_measure,
            beat_duration: beat_duration,

            steps_per_beat: steps_per_beat,
            steps_per_measure: steps_per_measure,
            samples_per_measure: samples_per_measure,

            current_step: 0,
            current_beat_i32: 0,
            current_beat_f32: 0.0,
        })
    }

    /// Step the tempo forward.
    ///
    /// DANGER: This should only be called by framework code, not by any blocks.
    /// Blocks should not have a mutable reference to tempo, so this shouldn't be possible.
    pub fn step(&mut self, count: i32) {
        self.current_step += count as i32;
        self.current_beat_i32 = self.current_step / self.steps_per_beat;
        self.current_beat_f32 = self.current_step as f32 / self.steps_per_beat as f32;
    }

    /// Skip a number of measures forward.
    pub fn skip(&mut self, num_measures: i32) {
        let num_chunks = self.steps_per_measure * num_measures;
        self.step(num_chunks);
    }

    /// The current measure as a float since the beginning of the song.
    pub fn current_measure(&self) -> f32 {
        return self.current_beat_f32 / self.beats_per_measure as f32;
    }

    /// Whether the current time is in the provided measure. This method uses floating point
    /// arithmetic to determine the current measure, which may lead to minor imprecision on the
    /// exact timing of a measure.
    ///
    /// If being off by +/- 1 step is important, consider
    ///
    /// # Arguments
    ///
    /// * m1 - The beginning of the measure window
    /// * m2 - The end of the measure window
    pub fn in_measure(&self, m1: f32, m2: f32) -> bool {
        let m = self.current_measure();
        return m1 <= m && m < m2;
    }

    /// Returns true if this chunk lands on the start of a beat.
    ///
    /// # Arguments
    ///
    /// * step_offset - A +/- offset to apply when calculating the window. This
    ///                 is typically used to compensate for latency.
    pub fn on_beat(&self, step_offset: i32) -> bool {
        return (self.current_step + step_offset) % self.steps_per_beat == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let project = config::ProjectConfig::new("dat/tempo/tempo.json").unwrap();
        let mut tempo = Tempo::new(&project).unwrap();

        // Check configuration
        assert!(tempo.bpm == 100);
        assert!(tempo.beats_per_measure == 3);
        assert!(tempo.beat_duration == 4);

        // This will break if sample rate or samples per buffer ever change.
        let steps_per_beat: i32 = 103;
        let steps_per_measure: i32 = 103 * 3;
        assert!(tempo.steps_per_beat == steps_per_beat);
        assert!(tempo.steps_per_measure == steps_per_measure);
        assert!(tempo.current_beat_i32 == 0);
        assert!(tempo.current_beat_f32 == 0.0);

        // Step forward one beat.
        tempo.step(steps_per_beat);
        assert!(tempo.current_beat_i32 == 1);
        log::assert_approx_eq!(tempo.current_measure(), 0.3333333, 0.00001);

        // Step forward one beat.
        tempo.step(steps_per_beat);
        assert!(tempo.current_beat_i32 == 2);
        log::assert_approx_eq!(tempo.current_measure(), 0.6666666, 0.00001);

        // Step forward one beat.
        tempo.step(steps_per_beat);
        assert!(tempo.current_beat_i32 == 3);
        log::assert_approx_eq!(tempo.current_measure(), 1.0, 0.0000001);
    }

    #[test]
    fn test_in_measure() {
        let project = config::ProjectConfig::new("dat/tempo/tempo.json").unwrap();
        let mut tempo = Tempo::new(&project).unwrap();

        assert!(tempo.current_beat_i32 == 0);
        assert!(tempo.current_beat_f32 == 0.0);
        assert!(!tempo.in_measure(1.0, 2.0));

        let steps_per_measure: i32 = 103 * 3;

        // Now we're in the measure.
        tempo.step(steps_per_measure);
        assert!(tempo.current_beat_i32 == 3);
        assert!(tempo.in_measure(1.0, 2.0));

        // And now we're not.
        tempo.step(steps_per_measure);
        assert!(tempo.current_beat_i32 == 6);
        assert!(!tempo.in_measure(1.0, 2.0));
    }

    #[test]
    fn test_on_beat() {
        let project = config::ProjectConfig::new("dat/tempo/tempo.json").unwrap();
        let mut tempo = Tempo::new(&project).unwrap();

        // on_beat should fire on the first beat.
        assert!(tempo.on_beat(0));

        let mut beat_count: i32 = 0;
        while beat_count < 3 {
            // on_beat should return true whenever current_beat increments by 1.
            if tempo.current_beat_i32 == beat_count {
                assert!(tempo.on_beat(0));
                beat_count += 1;
            } else {
                assert!(!tempo.on_beat(0));
            }

            tempo.step(1);
        }
    }
}
