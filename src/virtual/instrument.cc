#include "src/virtual/instrument.h"

#include "src/framework/config.h"
#include "src/framework/keyboard.h"
#include "src/framework/tempo.h"
#include "src/framework/volume.h"
#include "src/framework/wav.h"

namespace Looper
{

Instrument::Instrument(const BlockConfig &_configs) : Source(_configs) {}

bool Instrument::init()
{
    std::string filename;
    ASSERT(configs.get_string("instrument", filename),
           "virtual_instrument must define an \"instrument\" parameter.");

    float volume = 1.0;
    QASSERT(configs.get_float_default("volume", volume, volume));

    /*
     * Load the instrument file as json.
     */
    json data;
    const std::string path = instrument_path(filename);
    ASSERT(read_json_file(path, data), "Instrument init failed");

    /*
     * Load each of the audio clips into memory.
     */
    for (const auto &sound : data["sounds"])
    {
        const std::string key = sound["key"];
        const std::string clip_name = sound["file"];

        /*
         * Read in the audio file.
         */
        paudio_clip_t clip;
        const std::string cpath = clip_path(clip_name);
        ASSERT(read_wav_file(cpath, clip),
               "Could not read clip \"%s\"",
               clip_name.c_str());

        /*
         * Scale the volume.
         */
        scale_volume(clip, volume);

        /*
         * Index the clip into the clip map.
         */
        clips[key] = clip;
    }

    /*
     * Sanity check that all of the segments are outputs.
     */
    for (const auto &segment : segments)
    {
        ASSERT(segment.segment_type == segment_type_t::output,
               "Instrument only accepts output segments");
    }

    return true;
}

bool Instrument::read()
{
    stream->fill(0);

    /*
     * Start playing whatever samples we need to play
     */
    const auto &keys = Keyboard::get_keys();
    for (const auto &key : keys)
    {
        if (clips.count(key) == 1)
        {
            samplers[key].play(clips[key], false);
        }
    }

    /*
     * Read off all of the streams.
     */
    for ([[maybe_unused]] auto &[key, sampler] : samplers)
    {
        pstream_t tmp = std::make_shared<stream_t>();
        sampler.next(tmp);

        stream += tmp;
    }

    return true;
}

}  // namespace Looper