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
    QASSERT(configs.get_float("start_measure", recording_interval.start));
    QASSERT(configs.get_float("stop_measure", recording_interval.stop));
    ASSERT(recording_interval.start < recording_interval.stop,
           "Invalid recording interval bounds: [%f, %f]",
           recording_interval.start,
           recording_interval.stop);
    const float record_interval_duration =
        recording_interval.stop - recording_interval.start;

    /*
     * Read in the replay intervals.
     */
    json_v raw_intervals;
    QASSERT(get_array(configs.base, "replay_intervals", raw_intervals));

    for (const json& raw_interval : raw_intervals)
    {
        replay_interval_t interval;
        QASSERT(get_float(raw_interval, "start_measure", interval.start));
        QASSERT(get_float(raw_interval, "stop_measure", interval.stop));
        QASSERT(get_float_default(
            raw_interval, "offset", 0.0, interval.measure_offset));

        /*
         * Sanity check the replay interval.
         */
        ASSERT(interval.start < interval.stop,
               "Invalid replay interval bounds: [%f, %f]",
               interval.start,
               interval.stop);
        ASSERT(interval.start >= recording_interval.stop,
               "Replay interval is before the end of the record interval");
        ASSERT(interval.measure_offset < record_interval_duration,
               "Measure offset is longer than the recording");

        replay_intervals.push_back(std::move(interval));

        LOG(DEBUG,
            "Added playback interval: [%f, %f]",
            interval.start,
            interval.stop);
    }

    return true;
}

bool Loop::transform()
{
    const auto& input_stream = input_streams[0];
    auto& output_stream = output_streams[0];

    if (Tempo::in_measure(recording_interval.start, recording_interval.stop))
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
    size_t skip_samples = 0;
    for (const auto& interval : replay_intervals)
    {
        if (Tempo::in_measure(interval.start, interval.stop))
        {
            should_play = true;
            skip_samples = Tempo::measures_to_samples(interval.measure_offset);
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
            sampler.skip(skip_samples);
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