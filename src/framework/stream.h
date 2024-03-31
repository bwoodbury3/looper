/**
 * Stream data structure.
 */

#pragma once

#include <ctype.h>

#include <array>

namespace Looper
{

/*
 * All constants in this file are int64s so that downstream code can do basic
 * integer math on them.
 */

/**
 * The number of frames in a single stream buffer.
 */
static const int64_t FRAMES_PER_BUFFER = 256;

/**
 * The sample rate of the audio.
 */
static const int64_t SAMPLE_RATE = 44100;

typedef std::array<float, FRAMES_PER_BUFFER> stream_t;

}  // namespace Looper