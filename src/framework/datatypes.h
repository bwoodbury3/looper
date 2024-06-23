#pragma once

#include <vector>

namespace Looper
{

/**
 * A single audio sample unit.
 */
typedef float sample_t;

/**
 * PI
 */
static const float PI = 3.141592;
static const float PI_2 = PI * 2.0;

/**
 * A clip of audio.
 */
typedef std::vector<sample_t> audio_clip_t;
typedef std::shared_ptr<audio_clip_t> paudio_clip_t;

/**
 * An interval between two measures [start, stop].
 */
struct measure_interval_t
{
    /**
     * The measure at which the interval starts.
     */
    float start;

    /**
     * The measure at which the interval stops.
     */
    float stop;
};

}