#pragma once

#include "src/framework/block.h"
#include "src/framework/sampler.h"

namespace Looper
{

class Metronome : public Source
{
   public:
    /**
     * Constructor
     */
    Metronome(const BlockConfig &_configs);

    bool init() override;
    bool read() override;

   private:
    /**
     * The audio clip to play on each beat.
     */
    paudio_clip_t clip;

    /**
     * The sampler to play the audio clip.
     */
    Sampler sampler;

    /**
     * The start measure.
     */
    float start_measure;

    /**
     * The stop measure.
     */
    float stop_measure;

    /**
     * The frequency.
     */
    float freq = 440.0;

    /**
     * The volume.
     */
    float volume = 0.5;
};

}  // namespace Looper