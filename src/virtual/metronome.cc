#include "src/virtual/metronome.h"

#include "src/framework/config.h"
#include "src/framework/tempo.h"
#include "src/framework/volume.h"
#include "src/framework/wav.h"

namespace Looper
{

Metronome::Metronome(const BlockConfig& _configs) : Source(_configs) {}

bool Metronome::init()
{
    QASSERT(configs.get_float_default("freq", freq, freq));
    QASSERT(configs.get_float_default("volume", volume, volume));

    std::string clip_name;
    QASSERT(configs.get_string_default("sound", "", clip_name));

    const size_t num_samples = 0.05 * SAMPLE_RATE;
    const float step = 1.0 / SAMPLE_RATE;
    const float PI_2_freq = PI_2 * freq;

    /*
     * Sanity check that all of the segments are outputs.
     */
    for (const auto& segment : segments)
    {
        ASSERT(segment.segment_type == segment_type_t::output,
               "Metronome only accepts output segments");
    }

    if (clip_name == "")
    {
        /*
         * If the user didn't specify a sample to play, create a little beep
         * tone that plays when the metronome fires.
         */
        clip = std::make_shared<audio_clip_t>();
        clip->resize(num_samples);
        for (size_t i = 0; i < num_samples; i++)
        {
            (*clip)[i] = volume * std::sin(PI_2_freq * i * step);
        }
    }
    else
    {
        /*
         * Otherwise load that audio sample in.
         */
        const std::string cpath = clip_path(clip_name);
        ASSERT(read_wav_file(cpath, clip),
               "Could not read clip \"%s\"",
               clip_name.c_str());

        /*
         * Scale the volume.
         */
        scale_volume(clip, volume);
    }

    return true;
}

bool Metronome::read()
{
    stream->fill(0);

    for (const auto& segment : segments)
    {
        if (Tempo::in_measure(segment.start, segment.stop))
        {
            if (Tempo::on_beat())
            {
                sampler.play(clip, false);
            }
        }
    }

    sampler.next(stream);
    return true;
}

}  // namespace Looper