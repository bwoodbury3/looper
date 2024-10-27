#include "src/transform/toggle.h"

#include "src/framework/tempo.h"

namespace Looper
{

Toggle::Toggle(const BlockConfig& _configs) : Transformer(_configs) {}

bool Toggle::init()
{
    ASSERT(input_streams.size() == 1, "Toggle must have 1 input stream");
    ASSERT(output_streams.size() == 1, "Toggle must have 1 output stream");

    /*
     * Sanity check all segments are outputs.
     */
    for (const auto& segment : segments)
    {
        if (segment.segment_type != segment_type_t::output)
        {
            ABORT("All Toggle segments must be outputs");
        }
    }

    return true;
}

bool Toggle::transform()
{
    const pstream_t input_stream = input_streams[0];
    pstream_t output_stream = output_streams[0];

    bool filled = false;
    for (const auto& segment : segments)
    {
        if (Tempo::in_measure(segment.start, segment.stop))
        {
            std::copy(input_stream->begin(),
                      input_stream->end(),
                      output_stream->begin());
            filled = true;
            break;
        }
    }

    if (!filled)
    {
        output_stream->fill(0);
    }

    return true;
}

}  // namespace Looper