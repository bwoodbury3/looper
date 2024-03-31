/**
 * Stream data structure.
 */

#pragma once

#include <ctype.h>

#include <array>
#include <string>

namespace Looper
{

/*
 * All constants in this file are int64s so that downstream code can do basic
 * integer math on them.
 */

/**
 * The number of frames in a single stream buffer.
 */
static const int64_t FRAMES_PER_BUFFER = 512;

/**
 * The sample rate of the audio.
 */
static const int64_t SAMPLE_RATE = 44100;

/**
 * An audio stream.
 */
typedef std::array<float, FRAMES_PER_BUFFER> stream_t;

/**
 * Return a string representing some of the data in the stream.
 *
 * @param stream The stream
 *
 * @return A string of text representing this stream.
 */
std::string print_stream(const stream_t& stream);

}  // namespace Looper