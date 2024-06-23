#include "src/virtual/metronome.h"

#include "src/framework/tempo.h"

namespace Looper
{

Metronome::Metronome(const BlockConfig &_configs) : Source(_configs) {}

bool Metronome::init()
{
    QASSERT(configs.get_float("start_measure", start_measure));
    QASSERT(configs.get_float("stop_measure", stop_measure));
    QASSERT(configs.get_float_default("freq", freq, freq));
    QASSERT(configs.get_float_default("volume", volume, volume));

    const size_t num_samples = 0.05 * SAMPLE_RATE;
    const float step = 1.0 / SAMPLE_RATE;
    const float PI_2_freq = PI_2 * freq;

    /*
     * Initialize the little beep tone that plays when the metronome fires.
     */
    clip = std::make_shared<audio_clip_t>();
    clip->resize(num_samples);
    for (size_t i = 0; i < num_samples; i++)
    {
        (*clip)[i] = volume * std::sin(PI_2_freq * i * step);
    }

    return true;
}

bool Metronome::read()
{
    stream->fill(0);

    if (Tempo::in_measure(start_measure, stop_measure))
    {
        if (Tempo::on_beat())
        {
            sampler.play(clip, false);
        }
    }

    sampler.next(stream);
    return true;
}

}  // namespace Looper