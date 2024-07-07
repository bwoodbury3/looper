#include "src/audio/audio.h"
#include "src/transform/combiner.h"
#include "src/transform/looper.h"
#include "src/virtual/instrument.h"
#include "src/virtual/metronome.h"

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
    BlockFactory::register_source<Metronome>("Metronome");

    /*
     * Sinks
     */
    BlockFactory::register_sink<OutputDevice>("AudioOutput");

    /*
     * Transformers
     */
    BlockFactory::register_transformer<Combiner>("Combiner");
    BlockFactory::register_transformer<Loop>("Looper");
}

}  // namespace Looper