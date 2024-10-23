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

/**
 * Read configs into a list of sources/sinks/transformers, and initialize
 * everything.
 *
 * @param config_str The config as a json string
 * @param[out] sources The source blocks
 * @param[out] sinks The sink blocks
 * @param[out] transformers The transformer blocks
 */
bool read_config(const std::string &config_str,
                 std::vector<pSource> &sources,
                 std::vector<pSink> &sinks,
                 std::vector<pTransformer> &transformers) noexcept;

/**
 * Get the path to an instrument file on disk.
 *
 * @param name The name of the instrument.
 *
 * @return The path to the instrument config file.
 */
std::string instrument_path(const std::string &name) noexcept;

/**
 * Get the path to an audio clip on disk.
 *
 * @param name The name of the clip.
 *
 * @return The path to the clip audio file.
 */
std::string clip_path(const std::string &name) noexcept;

}  // namespace Looper