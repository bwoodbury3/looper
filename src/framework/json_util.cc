#include "src/framework/json_util.h"

#include "src/framework/log.h"

namespace Looper
{

bool get_string(const json &data, const std::string &key, std::string &value)
{
    ASSERT(data.contains(key), "Missing parameter \"%s\"", key.c_str());

    json obj = data[key];
    ASSERT(obj.is_string(), "Parameter \"%s\" should be a string", key.c_str());

    value = obj.get<std::string>();
    return true;
}

bool get_string_default(const json &data,
                        const std::string &key,
                        const std::string &_default,
                        std::string &value)
{
    if (!data.contains(key))
    {
        value = _default;
    }
    return get_string(data, key, value);
}

bool get_int(const json &data, const std::string &key, int &value)
{
    ASSERT(data.contains(key), "Missing parameter \"%s\"", key.c_str());

    json obj = data[key];
    ASSERT(obj.is_number_integer(),
           "Parameter \"%s\" should be an int",
           key.c_str());

    value = obj.get<int>();
    return true;
}

bool get_int_default(const json &data,
                     const std::string &key,
                     const int _default,
                     int &value)
{
    if (!data.contains(key))
    {
        value = _default;
        return true;
    }
    return get_int(data, key, value);
}

bool get_float(const json &data, const std::string &key, float &value)
{
    ASSERT(data.contains(key), "Missing parameter \"%s\"", key.c_str());

    json obj = data[key];
    ASSERT(obj.is_number(), "Parameter \"%s\" should be a float", key.c_str());

    value = obj.get<float>();
    return true;
}

bool get_float_default(const json &data,
                       const std::string &key,
                       const float _default,
                       float &value)
{
    if (!data.contains(key))
    {
        value = _default;
        return true;
    }
    return get_float(data, key, value);
}

bool get_string_v(const json &data,
                  const std::string &key,
                  std::vector<std::string> &value)
{
    value.clear();
    ASSERT(data.contains(key), "Missing parameter \"%s\"", key.c_str());

    json obj = data[key];
    ASSERT(obj.is_array(), "Parameter \"%s\" should be an array", key.c_str());

    for (const auto &item : obj)
    {
        ASSERT(item.is_string(),
               "All list items of \"%s\" must be a string",
               key.c_str());
        value.push_back(item.get<std::string>());
    }
    return true;
}

bool get_array(const json &data, const std::string &key, json_v &value)
{
    value.clear();
    ASSERT(data.contains(key), "Missing parameter \"%s\"", key.c_str());

    const json obj = data[key];
    for (const auto &val : obj)
    {
        value.push_back(val);
    }
    return true;
}

}  // namespace Looper