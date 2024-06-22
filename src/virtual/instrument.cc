#include "src/virtual/instrument.h"

#include "src/framework/config.h"
#include "src/framework/keyboard.h"
#include "src/framework/wav.h"

namespace Looper
{

bool valid_filename(const std::string &filename)
{
    /*
     * This is my most basic attempt to sanitize the path.
     */
    if (filename.find('/') != std::string::npos)
    {
        ABORT("Filename \"%s\" value must not include any slashes or spaces",
              filename.c_str());
    }

    return true;
}

Instrument::Instrument(const BlockConfig &_configs) : Source(_configs) {}

bool Instrument::init()
{
    std::string filename;
    ASSERT(configs.get_string("instrument", filename),
           "virtual_instrument must define an \"instrument\" parameter.");
    ASSERT(valid_filename(filename), "Invalid filename.");

    /*
     * Load the instrument file as json.
     */
    json data;
    const std::string path = "assets/instruments/" + filename + ".json";
    ASSERT(read_json_file(path, data), "Instrument init failed");

    /*
     * Load each of the audio clips into memory.
     */
    for (const auto &sound : data["sounds"])
    {
        const std::string key = sound["key"];
        const std::string clip_fname = sound["file"];
        ASSERT(valid_filename(clip_fname), "Invalid filename.");

        /*
         * Read in the audio file.
         */
        paudio_clip_t clip;
        const std::string clip_path = "assets/clips/" + clip_fname;
        ASSERT(read_wav_file(clip_path, clip),
               "Could not read clip \"%s\"",
               clip_fname.c_str());

        /*
         * Index the clip into the clip map.
         */
        clips[key] = clip;
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
            LOG(DEBUG, "Playing sample: %s", key.c_str());
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