#pragma once

#include <nlohmann/json.hpp>

/**
 * Utility functions for blocks for getting the current time/beat/measure.
 */
namespace Looper::Tempo
{

using json = nlohmann::json;

/**
 * Initialize the tempo framework.
 *
 * This is called by framework code.
 *
 * @param data Config data.
 */
bool init(const json &data);

/**
 * Step the tempo system forward.
 *
 * This is called by framework code.
 */
void step();

/**
 * Get the current measure, 1-indexed. A fractional value that also tells you
 * how far into the measure you are, if you care.
 */
float current_measure();

/**
 * Return true if the current measure is between [m1, m2) (1-indexed). Accepts
 * partial measures as well.
 *
 *  In the following examples, we're currently on beat 1 (4/4), measure 1:
 *      in_measure(1.0, 2.0) -> True
 *      in_measure(1.0, 1.5) -> True
 *      in_measure(1.5, 2.0) -> False
 *
 * @param m1 The beginning of the measure interval
 * @param m2 The end of the measure interval
 */
bool in_measure(const float m1, const float m2);

/**
 * Returns true on the rising edge of the next beat. This is useful if you want
 * to trigger something to happen on a particular beat.
 *
 * @param beat_offset Offset the trigger by beat_offset. For instance, if you
 *                    want to trigger a quarter-beat earlier, pass in -0.25.
 */
bool on_beat(float beat_offset = 0.0);

/**
 * Convert a decimal number of measures to a count of samples.
 *
 * @param measures The number of measures
 *
 * @return The count of samples.
 */
size_t measures_to_samples(float measures);

}  // namespace Looper::Tempo