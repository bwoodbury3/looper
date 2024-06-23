#include <nlohmann/json.hpp>

namespace Looper
{

using json = nlohmann::json;
using json_v = std::vector<json>;

/**
 * Get a string from a key.
 *
 * @param data The data
 * @param key The key
 * @param[out] value The value
 *
 * @return True on success.
 */
bool get_string(const json &data, const std::string &key, std::string &value);

/**
 * Get an int from a key.
 *
 * @param data The data
 * @param key The key
 * @param[out] value The value
 *
 * @return True on success.
 */
bool get_int(const json &data, const std::string &key, int &value);

/**
 * Get an int from a key with a default value.
 *
 * @param data The data
 * @param key The key
 * @param _default The default value
 * @param[out] value The value
 *
 * @return True on success.
 */
bool get_int_default(const json &data,
                     const std::string &key,
                     const int _default,
                     int &value);

/**
 * Get a float from a key.
 *
 * @param data The data
 * @param key The key
 * @param[out] value The value
 *
 * @return True on success.
 */
bool get_float(const json &data, const std::string &key, float &value);

/**
 * Get a float from a key with a default value.
 *
 * @param data The data
 * @param key The key
 * @param _default The default value
 * @param[out] value The value
 *
 * @return True on success.
 */
bool get_float_default(const json &data,
                       const std::string &key,
                       const float _default,
                       float &value);

/**
 * Get a list of strings from a key.
 *
 * @param data The data
 * @param key The key
 * @param[out] value The value
 *
 * @return True on success.
 */
bool get_string_v(const json &data,
                  const std::string &key,
                  std::vector<std::string> &value);

/**
 * Get a json array.
 *
 * @param data The data
 * @param key The key
 * @param[out] value The value
 *
 * @return True on success.
 */
bool get_array(const json &data, const std::string &key, json_v &value);

}  // namespace Looper