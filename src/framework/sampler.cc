#include "src/framework/sampler.h"

namespace Looper
{

Sampler::Sampler() : clip(), clip_index(0), is_playing(false), loop(false) {}

void Sampler::play(paudio_clip_t _clip, bool _loop)
{
    clip = _clip;
    loop = _loop;

    is_playing = true;
    clip_index = 0;
}

void Sampler::skip(const size_t num_samples)
{
    clip_index = std::min(clip_index + num_samples, clip->size());
}

bool Sampler::playing()
{
    return is_playing;
}

void Sampler::stop()
{
    clip = nullptr;
    is_playing = false;
    clip_index = 0;
}

void Sampler::next(pstream_t &stream)
{
    if (!is_playing)
    {
        return;
    }

    /*
     * Read enough samples to fill the stream, unless we run out of clip.
     */
    const size_t start_index = clip_index;
    const size_t stop_index =
        std::min(start_index + stream->size(), clip->size());

    /*
     * Copy the clip slice to the output stream.
     */
    std::copy(clip->begin() + start_index,
              clip->begin() + stop_index,
              stream->begin());

    /*
     * Stop the clip or restart the loop if we're at the end of the clip.
     */
    if (stop_index == clip->size())
    {
        clip_index = 0;
        if (!loop)
        {
            stop();
        }
    }
    else
    {
        clip_index = stop_index;
    }
}

}  // namespace Looper