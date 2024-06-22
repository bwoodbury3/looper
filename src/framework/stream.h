/**
 * Stream data structure.
 */

#pragma once

#include <ctype.h>

#include <array>
#include <string>

#include "src/framework/datatypes.h"

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
typedef std::array<sample_t, FRAMES_PER_BUFFER> stream_t;
typedef std::shared_ptr<stream_t> pstream_t;

/**
 * Sum two arrays.
 */
stream_t& operator+=(stream_t& first, const stream_t& second);
pstream_t& operator+=(pstream_t& first, const pstream_t& second);

/**
 * Return a string representing some of the data in the stream.
 *
 * @param stream The stream
 *
 * @return A string of text representing this stream.
 */
std::string print_stream(const stream_t& stream);

/**
 * Create a stream.
 *
 * @param name The name of the stream.
 * @param stream The stream.
 */
bool create_stream(const std::string& name, pstream_t& stream);

/**
 * Bind to an existing stream.
 *
 * @param name The name of the stream.
 * @param stream The stream.
 */
bool bind_stream(const std::string& name, pstream_t& stream);

}  // namespace Looper