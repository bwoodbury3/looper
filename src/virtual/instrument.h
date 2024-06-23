#pragma once

#include <map>

#include "src/framework/block.h"
#include "src/framework/sampler.h"

namespace Looper
{

class Instrument : public Source
{
   public:
    /**
     * Constructor
     */
    Instrument(const BlockConfig &_configs);

    bool init() override;
    bool read() override;

   private:
    /**
     * Audio clips, mapped by keyboard key.
     */
    std::map<std::string, paudio_clip_t> clips;

    /**
     * The samplers.
     */
    std::map<std::string, Sampler> samplers;
};

}  // namespace Looper