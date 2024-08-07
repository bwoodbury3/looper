#pragma once

#include "src/framework/datatypes.h"
#include "src/framework/log.h"
#include "src/framework/stream.h"

namespace Looper
{

class Sampler
{
   public:
    Sampler();

    /**
     * Start playing an audio clip.
     *
     * @param _clip The clip to play.
     * @param _loop Whether to loop the clip on completion.
     */
    void play(paudio_clip_t _clip, bool _loop);

    /**
     * Skip the current playback index forward by num_samples. This has no
     * effect if nothing is playing.
     *
     * Skipping past the end of the clip will just skip to the end.
     *
     * @param num_samples The number of samples to skip.
     */
    void skip(size_t num_samples);

    /**
     * @return true if the sampler is currently playing audio.
     */
    bool playing();

    /**
     * Stop playing an audio clip.
     */
    void stop();

    /**
     * Write the next set of samples to the stream. If nothing is playing,
     * leaves the stream unchanged.
     *
     * @param[out] stream The output stream.
     */
    void next(pstream_t &stream);

   private:
    /**
     * Pointer to the currently playing audio clip.
     */
    paudio_clip_t clip;

    /**
     * The current playback index.
     */
    size_t clip_index;

    /**
     * Whether the audio clip is currently playing.
     */
    bool is_playing;

    /**
     * Whether to play back in loop mode.
     */
    bool loop;
};

}  // namespace Looper