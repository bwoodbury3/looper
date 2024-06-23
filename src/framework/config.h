#pragma once

#include <nlohmann/json.hpp>
#include <string>
#include <vector>

#include "src/framework/block.h"

namespace Looper
{

/**
 * Read a file as json.
 *
 * @param filename The filename
 * @param[out] data The json data
 */
bool read_json_file(const std::string &filename, json &data);

class ConfigFile
{
   public:
    ConfigFile(const std::string &filename);
    bool read_config(std::vector<pSource> &sources,
                     std::vector<pSink> &sinks,
                     std::vector<pTransformer> &transformers) noexcept;

   private:
    const std::string filename;
};

}  // namespace Looper