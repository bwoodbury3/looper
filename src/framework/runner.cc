#include "src/framework/runner.h"

#include "src/audio/audio.h"
#include "src/framework/config.h"
#include "src/framework/keyboard.h"
#include "src/framework/log.h"
#include "src/framework/tempo.h"
#include "src/modules/modules.h"

namespace Looper
{

bool Runner::run(const std::string config_str)
{
    {
        std::lock_guard<std::mutex> lock(mu);
        ASSERT(!running, "Already running; refusing to re-run");
    }

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
     * Mark ourselves running after all assertions are complete.
     */
    {
        std::lock_guard<std::mutex> lock(mu);
        running = true;
        request_stop = false;
    }

    /*
     * Flush the input and output buffers. Otherwise we will read garbage input
     * which produces a huge audio snap at the beginning.
     */
    for (size_t i = 0; i < 5; i++)
    {
        for (auto& source : sources)
        {
            source->read();
        }

        for (auto& sink : sinks)
        {
            sink->stream->fill(0);
            sink->write();
        }
    }

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

        {
            std::lock_guard<std::mutex> lock(mu);
            if (request_stop)
            {
                break;
            }
        }
    }

    /*
     * Stop
     */
    {
        std::lock_guard<std::mutex> lock(mu);
        running = false;
        cv.notify_all();
    }

    return true;
}

bool Runner::stop()
{
    /*
     * Request main thread to stop.
     */
    {
        std::unique_lock<std::mutex> lock(mu);
        if (running)
        {
            request_stop = true;
            cv.wait_for(lock, std::chrono::seconds(1));
            ASSERT(!running, "Failed to stop the main thread!");
        }
    }

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

void Runner::queue_keypress(const std::string& key)
{
    /*
     * Only queue keypress when it's running.
     */
    if (running)
    {
        Keyboard::queue_keypress(key);
    }
}

bool Runner::is_running()
{
    std::lock_guard<std::mutex> lock(mu);
    return running;
}

float Runner::current_measure()
{
    return Tempo::current_measure();
}

}  // namespace Looper