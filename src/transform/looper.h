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
     * The segment during which the device is recording.
     */
    Segment recording_segment;

    /**
     * The segment during which the device is replaying.
     */
    std::vector<Segment> replay_segments;

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