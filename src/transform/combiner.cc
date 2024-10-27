#include "src/transform/combiner.h"

namespace Looper
{

Combiner::Combiner(const BlockConfig& _configs) : Transformer(_configs) {}

bool Combiner::init()
{
    ASSERT(output_streams.size() == 1,
           "Combiner may only have 1 output stream");
    return true;
}

bool Combiner::transform()
{
    pstream_t& stream = output_streams[0];
    stream->fill(0);

    for (const pstream_t& input : input_streams)
    {
        stream += input;
    }

    return true;
}

}  // namespace Looper