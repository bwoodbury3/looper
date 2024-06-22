#include <vector>

namespace Looper
{

/**
 * A single audio sample unit.
 */
typedef float sample_t;

/**
 * A clip of audio.
 */
typedef std::vector<sample_t> audio_clip_t;
typedef std::shared_ptr<audio_clip_t> paudio_clip_t;

}