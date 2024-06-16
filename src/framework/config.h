#pragma once

#include <nlohmann/json.hpp>
#include <string>
#include <vector>

#include "src/framework/block.h"

namespace Looper
{

class ConfigFile
{
   public:
    ConfigFile(const std::string &filename);
    bool get_blocks(std::vector<pSource> &sources,
                    std::vector<pSink> &sinks,
                    std::vector<pTransformer> &transformers) noexcept;

   private:
    const std::string filename;
};

}  // namespace Looper