//! Low Pass Filter Block.
//!
//! Filters out signals above a cutoff frequency.
//!
//! LowPass \[Transformer\]:
//!     Required parameters:
//!         name: Anything
//!         type: "LowPass"
//!         cutoff: The cutoff frequency.
//!         input_channel: The input channel name.
//!         output_channel: The output channel name.

extern crate block;
extern crate config;
extern crate log;
extern crate stream;

use std::str::FromStr;

use stream::Sample;

/// The LowPass Block.
pub struct LowPass {
    /// The input stream to toggle.
    input_stream: stream::Stream,

    /// The output stream.
    output_stream: stream::Stream,

    /// The numerator filter coefficients.
    numerator: Vec<f32>,

    /// The denominator filter coefficients.
    denominator: Vec<f32>,

    /// The last N input history, where N is the filter order.
    in_history: Vec<Sample>,

    /// The last N output history, where N is the filter order.
    out_history: Vec<Sample>,

    /// The ring buffer index into the history vector.
    ring_index: i32,
}

/// Consult a lookup table to get the filter coefficients.
fn get_filter_coefficients(freq: f32) -> Result<(Vec<f32>, Vec<f32>), ()> {
    let table_path = "assets/filters/low_pass.txt";
    let contents = log::unwrap_abort!(std::fs::read_to_string(table_path));

    let mut numerator: Vec<f32> = Vec::<f32>::new();
    let mut state: i32 = 0;
    for line in contents.lines() {
        if state == 0 {
            // Find the first frequency that's below the freq value.
            let freq_match: f32 = match line.find("freq") {
                Some(_) => log::unwrap_abort!(f32::from_str(&line[5..])),
                None => {
                    continue;
                }
            };
            if freq_match >= freq {
                state = 1;
            }
        } else if state == 1 {
            // Read the numerator coefficients.
            numerator = line.split(" ").map(|s| f32::from_str(s).unwrap()).collect();
            state = 2;
        } else if state == 2 {
            // Read the denominator coefficients and return.
            let denominator = line.split(" ").map(|s| f32::from_str(s).unwrap()).collect();
            return Ok((numerator, denominator));
        }
    }

    Err(())
}

impl LowPass {
    /// Create a new LowPass filter.
    pub fn new(
        config: &config::BlockConfig,
        stream_catalog: &mut stream::StreamCatalog,
    ) -> Result<Self, ()> {
        // Read in parameters.
        let input_channel = config.get_str("input_channel")?;
        let output_channel = config.get_str("output_channel")?;
        let freq = config.get_f32("cutoff")?;

        // Load streams.
        let input_stream = stream_catalog.bind_sink(input_channel)?;
        let output_stream = stream_catalog.create_source(output_channel)?;

        // Load the filter.
        let (numerator, denominator) = log::unwrap_abort_msg!(
            get_filter_coefficients(freq),
            format!("No filter found for freq={}", freq)
        );
        let order = numerator.len();

        Ok(LowPass {
            input_stream: input_stream,
            output_stream: output_stream,
            numerator: numerator,
            denominator: denominator,
            in_history: vec![0f32; order],
            out_history: vec![0f32; order],
            ring_index: 0i32,
        })
    }

    /// Apply the filter.
    fn filter(&mut self) {
        let input_stream = self.input_stream.borrow();
        let mut output_stream = self.output_stream.borrow_mut();
        let order = self.numerator.len() as i32;

        for m in 0..stream::SAMPLES_PER_BUFFER {
            // First term
            output_stream[m] = self.numerator[0] * input_stream[m];

            // Next N-1 terms
            for i in 1..order as usize {
                let prev_index = (self.ring_index - i as i32).rem_euclid(order) as usize;
                output_stream[m] += self.denominator[i] * self.out_history[prev_index]
                    + self.numerator[i] * self.in_history[prev_index];
            }

            // Update the historical ring buffer.
            self.in_history[self.ring_index as usize] = input_stream[m];
            self.out_history[self.ring_index as usize] = output_stream[m];
            self.ring_index = (self.ring_index + 1) % order;
        }
    }
}

impl block::Transformer for LowPass {
    fn transform(&mut self, _state: &block::PlaybackState) {
        self.filter();
    }
}

#[cfg(test)]
mod tests {
    use stream::SAMPLES_PER_BUFFER;

    use super::*;

    /// Load input and output signals
    fn load_signals(filename: &str) -> (Vec<f32>, Vec<f32>) {
        let contents = std::fs::read_to_string(filename).unwrap();

        let mut iter = contents.lines();
        let sig1 = iter
            .next()
            .unwrap()
            .split(" ")
            .map(|s| f32::from_str(s).unwrap())
            .collect();
        let sig2 = iter
            .next()
            .unwrap()
            .split(" ")
            .map(|s| f32::from_str(s).unwrap())
            .collect();

        return (sig1, sig2);
    }

    #[test]
    fn test_filter() {
        let project = config::ProjectConfig::new("dat/low_pass/low_pass1.yaml").unwrap();
        let mut stream_catalog = stream::StreamCatalog::new();
        let input_stream = stream_catalog.create_source("test").unwrap();

        let mut low_pass = LowPass::new(&project.blocks[0], &mut stream_catalog).unwrap();
        let output_stream = stream_catalog.bind_sink("test_filtered").unwrap();

        // Load unit test data
        let (input, output) = load_signals("dat/low_pass/low_pass_unit_test.txt");

        // Set the input.
        {
            let mut s = input_stream.borrow_mut();
            s.copy_from_slice(&input);
        }

        low_pass.filter();

        // Validate the output.
        {
            let s = output_stream.borrow();
            for i in 0..SAMPLES_PER_BUFFER {
                // This tolerance is pretty high but an iterative algorithm in a different
                // language is bound to diverge by a bit over 256 iterations.
                log::assert_approx_eq!(s[i], output[i], 0.1);
            }
        }
    }
}
