#include "src/framework/config.h"

#include <fstream>
#include <sstream>

#include "src/framework/log.h"
#include "src/framework/tempo.h"

namespace Looper
{

bool read_json_file(const std::string &filename, json &data)
{
    std::ifstream stream(filename);
    ASSERT(stream.good(), "File not found: %s", filename.c_str());

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

ConfigFile::ConfigFile(const std::string &_filename) : filename(_filename) {}

bool ConfigFile::read_config(std::vector<pSource> &sources,
                             std::vector<pSink> &sinks,
                             std::vector<pTransformer> &transformers) noexcept
{
    sources.clear();
    sinks.clear();
    transformers.clear();

    json data;
    ASSERT(read_json_file(filename, data), "Error reading json file");
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
    for (const auto &[name, device] : devices.items())
    {
        ASSERT(device.contains("type"),
               "Device \"%s\" did not contain required \"type\" field",
               name.c_str());
        const std::string tname = device["type"];
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
    }

    return true;
}

}  // namespace Looper