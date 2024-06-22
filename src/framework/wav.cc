#include "src/framework/wav.h"

#include "AudioFile.h"
#include "src/framework/log.h"

namespace Looper
{

bool read_wav_file(const std::string& filename, paudio_clip_t& clip)
{
    clip = std::make_shared<audio_clip_t>();

    AudioFile<sample_t> audio_file;
    ASSERT(audio_file.load(filename), "Error loading AudioFile as wav");

    /*
     * TODO: Support stereo.
     */
    ASSERT(audio_file.getNumSamplesPerChannel() > 0,
           "Audio clip must have at least 1 channel");
    const auto& channel = audio_file.samples[0];
    ASSERT(channel.size() > 0, "Audio file is empty!");

    /*
     * Copy over the samples to the clip.
     */
    clip->resize(channel.size());
    std::copy(channel.begin(), channel.end(), clip->begin());
    ASSERT(clip->size() > 0, "Failed to copy audio to clip!");

    return true;
}

}  // namespace Looper