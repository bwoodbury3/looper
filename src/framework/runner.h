#pragma once

#include "src/framework/block.h"

namespace Looper
{

class Runner
{
   public:
    Runner() = default;

    /**
     * Run.
     *
     * @param config_str The config json as a string.
     */
    bool run(const std::string &config_str);

    /**
     * Stop running.
     */
    bool stop();

   private:
    bool running = false;

    /**
     * The current list of sources, sinks, and transformers.
     */
    std::vector<pSource> sources;
    std::vector<pSink> sinks;
    std::vector<pTransformer> transformers;
};

}  // namespace Looper