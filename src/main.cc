/**
 * Go!
 */

#include <unistd.h>

#include "src/audio/audio.h"
#include "src/framework/config.h"
#include "src/framework/keyboard.h"
#include "src/framework/log.h"
#include "src/framework/tempo.h"
#include "src/modules/modules.h"

namespace Looper
{

/**
 * Go!
 *
 * @return True on success.
 */
bool go(const std::string& filename)
{
    register_all_modules();
    ASSERT(init_audio(), "Could not initialize audio");

    ConfigFile config(filename);
    std::vector<pSource> sources;
    std::vector<pSink> sinks;
    std::vector<pTransformer> transformers;
    ASSERT(config.read_config(sources, sinks, transformers),
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

}  // namespace Looper

void help()
{
    fprintf(stderr, "~ L 0 0 P E R ~\n");
    fprintf(stderr, "Usage:\n");
    fprintf(stderr, "\tbazel run //src:looper <file.json>\n");
}

int main(int argc, const char** argv)
{
    if (argc < 2)
    {
        help();
        return -1;
    }

    const std::string filename(argv[1]);
    if (!Looper::go(filename))
    {
        return -1;
    }

    return 0;
}