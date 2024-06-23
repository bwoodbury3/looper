#include "src/framework/runner.h"

#include "src/audio/audio.h"
#include "src/framework/config.h"
#include "src/framework/keyboard.h"
#include "src/framework/log.h"
#include "src/framework/tempo.h"
#include "src/modules/modules.h"

namespace Looper
{

bool Runner::run(const std::string& config_str)
{
    ASSERT(read_config(config_str, sources, sinks, transformers),
           "Config file parsing failed");

    LOG(DEBUG, "Num sources: %lu", sources.size());
    LOG(DEBUG, "Num sinks: %lu", sinks.size());
    LOG(DEBUG, "Num transformers: %lu", transformers.size());

    /*
     * Initialize all of the blocks.
     */
    for (auto& block : sources)
    {
        ASSERT(block->init_source() && block->init(),
               "Unable to initialize \"%s\"",
               block->name().c_str());
    }
    for (auto& block : transformers)
    {
        ASSERT(block->init_transformer() && block->init(),
               "Unable to initialize \"%s\"",
               block->name().c_str());
    }
    for (auto& block : sinks)
    {
        ASSERT(block->init_sink() && block->init(),
               "Unable to initialize \"%s\"",
               block->name().c_str());
    }

    /*
     * Do this last so that ASSERT errors don't fail to clean up the keyboard.
     */
    ASSERT(Keyboard::init(), "Could not initialize the keyboard");

    /*
     * Run!
     */
    while (true)
    {
        if (!Keyboard::reset())
        {
            LOG(INFO, "Terminating program.");
            break;
        }

        for (auto& source : sources)
        {
            source->read();
        }
        for (auto& transformer : transformers)
        {
            transformer->transform();
        }
        for (auto& sink : sinks)
        {
            sink->write();
        }

        Tempo::step();
    }

    return true;
}

bool Runner::stop()
{
    if (!running)
    {
        return true;
    }
    running = false;

    /*
     * Delete all sources/sinks/transformers.
     */
    sources.clear();
    sinks.clear();
    transformers.clear();

    /*
     * Clear all streams.
     */
    clear_streams();

    return true;
}

}  // namespace Looper