#pragma once

#include <atomic>

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
     * Deliberate copy to avoid GIL
     *
     * @param config_str The config json as a string.
     */
    bool run(const std::string config_str);

    /**
     * Stop running. Returns when fully stopped.
     */
    bool stop();

   private:
    /**
     * Runtime orchestration.
     */
    std::condition_variable cv;
    std::mutex mu;
    bool request_stop = false;
    bool running = false;

    /**
     * The current list of sources, sinks, and transformers.
     */
    std::vector<pSource> sources;
    std::vector<pSink> sinks;
    std::vector<pTransformer> transformers;
};

}  // namespace Looper