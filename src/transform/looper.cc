#include "src/transform/looper.h"

#include "src/framework/tempo.h"

namespace Looper
{

Loop::Loop(const BlockConfig& _configs)
    : Transformer(_configs), recording(std::make_shared<audio_clip_t>())
{
}

bool Loop::init()
{
    /*
     * Looper can only operate on one input/output stream.
     */
    ASSERT(input_streams.size() == 1, "Must have exactly one input stream");
    ASSERT(output_streams.size() == 1, "Must have exactly one output stream");

    /*
     * Read in configs.
     */
    bool has_recording_segment = false;
    for (const auto& segment : segments)
    {
        if (segment.segment_type == segment_type_t::input)
        {
            has_recording_segment = true;
            recording_segment = segment;
        }
        else if (segment.segment_type == segment_type_t::output)
        {
            replay_segments.push_back(segment);
        }
        else
        {
            ABORT("Unsupported segment type. Only support inputs/outputs");
        }
    }

    /*
     * Sanity check the recording interval.
     */
    ASSERT(has_recording_segment, "Didn't find an input segment!");
    ASSERT(recording_segment.start < recording_segment.stop,
           "Invalid recording interval bounds: [%f, %f]",
           recording_segment.start,
           recording_segment.stop);

    /*
     * Sanity check the replay intervals.
     */
    for (const auto& interval : replay_segments)
    {
        ASSERT(interval.start < interval.stop,
               "Invalid replay interval bounds: [%f, %f]",
               interval.start,
               interval.stop);
        ASSERT(interval.start >= recording_segment.stop,
               "Replay interval is before the end of the record interval");
    }

    return true;
}

bool Loop::transform()
{
    const auto& input_stream = input_streams[0];
    auto& output_stream = output_streams[0];

    if (Tempo::in_measure(recording_segment.start, recording_segment.stop))
    {
        if (recording->size() == 0)
        {
            LOG(DEBUG, "Loop started!");
        }

        /*
         * Concatenate the recording with the new input stream.
         */
        size_t cur_size = recording->size();
        recording->resize(cur_size + input_stream->size());
        std::copy(input_stream->begin(),
                  input_stream->end(),
                  recording->begin() + cur_size);
    }

    /*
     * Figure out if we're in a replay interval and should be playing.
     */
    bool should_play = false;
    for (const auto& interval : replay_segments)
    {
        if (Tempo::in_measure(interval.start, interval.stop, 1.0))
        {
            should_play = true;
            break;
        }
    }

    /*
     * If we're not playing and we should be, start the sampler. Otherwise turn
     * it off.
     */
    if (should_play)
    {
        if (!sampler.playing())
        {
            LOG(DEBUG, "Playing loop");
            sampler.play(recording, true);
        }
    }
    else
    {
        sampler.stop();
        output_stream->fill(0);
    }

    sampler.next(output_stream);
    return true;
}

}  // namespace Looper