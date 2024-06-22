#include "src/framework/datatypes.h"

namespace Looper
{

/**
 * Read a wave file in as an audio clip.
 *
 * @param filename The filename
 * @param[out] clip The clip
 */
bool read_wav_file(const std::string& filename, paudio_clip_t& clip);

}