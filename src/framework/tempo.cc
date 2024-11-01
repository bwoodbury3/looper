#include "src/framework/tempo.h"

#include "src/framework/json_util.h"
#include "src/framework/log.h"
#include "src/framework/stream.h"

namespace Looper::Tempo
{

/*
 * Required config parameters.
 */

/**
 * Beats per minute (tempo).
 */
float bpm = -1;

/**
 * Beats per measure.
 */
int beats_per_measure = 1;

/**
 * Beat duration.
 */
int beat_duration = -1;

/*
 * Constants.
 */

/**
 * The number of seconds that pass in each step.
 */
float seconds_per_step = -1;

/**
 * The number of seconds that pass for each beat.
 */
float seconds_per_beat = -1;

/**
 * The number of beats that pass for each step.
 */
float beats_per_step = -1;

/**
 * The number of measures that pass for each step.
 */
float measures_per_step = -1;

/**
 * The number of samples per measure.
 */
float samples_per_measure = -1;

/**
 * Epsilon value.
 */
float beat_epsilon = -1;
float measure_epsilon = -1;

/*
 * Current state.
 */

/**
 * The current chunk.
 */
size_t current_chunk = 0;

/**
 * The current beat.
 */
float current_beat = 0.0;

/**
 * The current runtime in seconds.
 */
float current_time_s = 0.0;

bool init(const json& data)
{
    /*
     * Get configs.
     */
    ASSERT(get_float(data, "bpm", bpm), "Bad parameter");
    ASSERT(get_int(data, "beats_per_measure", beats_per_measure),
           "Bad parameter");
    ASSERT(get_int(data, "beat_duration", beat_duration), "Bad parameter");

    /*
     * Write constants from config.
     */
    seconds_per_step = static_cast<float>(SAMPLES_PER_BUFFER) /
                       static_cast<float>(SAMPLE_RATE);
    seconds_per_beat = 60.0 / bpm;
    beats_per_step = seconds_per_step / seconds_per_beat;
    measures_per_step = beats_per_step / beats_per_measure;
    samples_per_measure =
        static_cast<float>(SAMPLES_PER_BUFFER) / measures_per_step;
    beat_epsilon = beats_per_step / 2.0;
    measure_epsilon = measures_per_step / 2.0;

    /*
     * Reset the stateful variables.
     */
    current_chunk = 0;
    current_time_s = 0;
    current_beat = 0;

    return true;
}

void step()
{
    current_chunk++;
    current_time_s += seconds_per_step;
    current_beat += beats_per_step;

    /*
     * Debug print for the current measure.
     */
    const float measure = current_measure();
    const float measure_rounded = std::round(measure);
    if (std::abs(measure - measure_rounded) < measure_epsilon)
    {
        LOG(DEBUG, "Measure %d", (int)measure_rounded);
    }
}

float current_measure()
{
    return current_beat / beats_per_measure;
}

bool in_measure(const float m1, const float m2, const float step_offset)
{
    float curr = current_measure();
    curr += measures_per_step * step_offset;

    /*
     * HACK: Epsilon is used in the beginning of the measure but not the end
     * for the metronome specifically.
     */
    return m1 - measure_epsilon <= curr && curr < m2;
}

bool on_beat(float beat_offset)
{
    /*
     * Subtracting beat_offset introduces a delay.
     */
    const float beat = current_beat - beat_offset;
    return abs(std::round(beat) - beat) < beat_epsilon;
}

size_t measures_to_samples(float measures)
{
    return static_cast<size_t>(measures * samples_per_measure);
}

}  // namespace Looper::Tempo