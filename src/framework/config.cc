#include "src/framework/config.h"

#include <fstream>
#include <sstream>

#include "src/framework/log.h"
#include "src/framework/tempo.h"

namespace Looper
{

bool _safe_parse(const std::string &str, json &data)
{
    try
    {
        data = json::parse(str.begin(), str.end());
    }
    catch (const json::parse_error &e)
    {
        ABORT("%s", e.what());
    }

    return true;
}

bool _safe_parse(std::ifstream &stream, json &data)
{
    try
    {
        data = json::parse(stream);
    }
    catch (const json::parse_error &e)
    {
        ABORT("%s", e.what());
    }

    return true;
}

bool read_json_file(const std::string &filename, json &data)
{
    std::ifstream stream(filename);
    ASSERT(stream.good(), "File not found: %s", filename.c_str());
    QASSERT(_safe_parse(stream, data));
    return true;
}

bool read_config(const std::string &config_str,
                 std::vector<pSource> &sources,
                 std::vector<pSink> &sinks,
                 std::vector<pTransformer> &transformers) noexcept
{
    sources.clear();
    sinks.clear();
    transformers.clear();

    json data;
    QASSERT(_safe_parse(config_str, data));

    ASSERT(data.contains("config"), "Must define \"config\" key");
    const json config = data["config"];

    /*
     * Initialize the tempo system.
     */
    ASSERT(config.contains("tempo"), "Must define \"tempo\" key");
    ASSERT(Tempo::init(config["tempo"]),
           "Failed to initialize tempo framework");

    /*
     * Get all audio devices.
     */
    ASSERT(data.contains("devices"),
           "Config file did not contain any audio devices");
    json devices = data["devices"];
    for (const auto &device : devices)
    {
        std::string name, tname;
        ASSERT(get_string(device, "name", name), "Unnamed block!");
        ASSERT(get_string(device, "type", tname),
               "Invalid type for block: %s",
               name.c_str());
        const BlockConfig config(name, device);

        if (BlockFactory::is_source(tname))
        {
            pSource source;
            ASSERT(BlockFactory::build_source(tname, config, source),
                   "Unable to build %s",
                   name.c_str());
            sources.push_back(source);
        }
        else if (BlockFactory::is_sink(tname))
        {
            pSink sink;
            ASSERT(BlockFactory::build_sink(tname, config, sink),
                   "Unable to build %s",
                   name.c_str());
            sinks.push_back(sink);
        }
        else if (BlockFactory::is_transformer(tname))
        {
            pTransformer transformer;
            ASSERT(BlockFactory::build_transformer(tname, config, transformer),
                   "Unable to build %s",
                   name.c_str());
            transformers.push_back(transformer);
        }
        else
        {
            ABORT("Unknown device=\"%s\" with type=\"%s\"",
                  name.c_str(),
                  tname.c_str());
        }

        LOG(DEBUG, "Added block %s(%s)", tname.c_str(), name.c_str());
    }

    return true;
}

std::string instrument_path(const std::string &name) noexcept
{
    return "assets/instruments/" + name + ".json";
}

std::string clip_path(const std::string &name) noexcept
{
    return "assets/clips/" + name + ".wav";
}

}  // namespace Looper