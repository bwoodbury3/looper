/**
 * Go!
 */

#include <unistd.h>

#include "audio/audio.h"
#include "framework/config.h"
#include "framework/log.h"

namespace Looper
{

/**
 * Go!
 *
 * @return True on success.
 */
bool go()
{
    ASSERT(Looper::init_audio(), "Could not initialize audio");

    ConfigFile config("projects/test.json");
    std::vector<pSource> sources;
    std::vector<pSink> sinks;
    std::vector<pTransformer> transformers;
    ASSERT(config.get_blocks(sources, sinks, transformers),
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
    for (auto& block : sinks)
    {
        ASSERT(block->init_sink() && block->init(),
               "Unable to initialize \"%s\"",
               block->name().c_str());
    }
    for (auto& block : transformers)
    {
        ASSERT(block->init_transformer() && block->init(),
               "Unable to initialize \"%s\"",
               block->name().c_str());
    }

    /*
     * Run!
     */
    while (true)
    {
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
    }

    return true;
}

}  // namespace Looper

int main(int argc, const char** argv)
{
    if (!Looper::go())
    {
        return -1;
    }

    return 0;
}