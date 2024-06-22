#include "src/audio/audio.h"
#include "src/virtual/instrument.h"

namespace Looper
{

/**
 * Register all modules with the framework.
 */
inline void register_all_modules()
{
    /*
     * Sources
     */
    BlockFactory::register_source<InputDevice>("AudioInput");
    BlockFactory::register_source<Instrument>("VirtualInstrument");

    /*
     * Sinks
     */
    BlockFactory::register_sink<OutputDevice>("AudioOutput");

    /*
     * Transformers
     */
}

}