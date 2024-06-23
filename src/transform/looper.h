#pragma once

#include "src/framework/block.h"
#include "src/framework/sampler.h"

namespace Looper
{

class Loop : public Transformer
{
   public:
    /**
     * Constructor
     */
    Loop(const BlockConfig& _configs);

    bool init() override;
    bool transform() override;

   private:
    /**
     * A loop replay interval.
     */
    struct replay_interval_t : public measure_interval_t
    {
        /**
         * Start the loop playback this many measures into the recording. For
         * example, if you had a 4 measure long loop recording and wanted to
         * play only the last 2.5 measures, you would set this to 1.5.
         *
         * Note that when the loop wraps around, it will start again from 0.
         */
        float measure_offset = 0.0;
    };

    /**
     * The interval during which the loop is recording audio.
     */
    measure_interval_t recording_interval;

    /**
     * The intervals during which to replay the loop.
     */
    std::vector<replay_interval_t> replay_intervals;

    /**
     * The recording clip that gets built over time.
     */
    paudio_clip_t recording;

    /**
     * Sampler for replaying the audio clip.
     */
    Sampler sampler;
};

}  // namespace Looper