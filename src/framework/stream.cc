#include "src/framework/stream.h"

#include <map>

#include "src/framework/log.h"

namespace Looper
{

/**
 * Map of all streams, indexed by name.
 */
std::map<const std::string, pstream_t> streams;

stream_t& operator+=(stream_t& first, const stream_t& second)
{
    for (int i = 0; i < first.size(); ++i)
    {
        first[i] += second[i];
    }
    return first;
}

pstream_t& operator+=(pstream_t& first, const pstream_t& second)
{
    for (int i = 0; i < first->size(); ++i)
    {
        (*first)[i] += (*second)[i];
    }
    return first;
}

void clear_streams()
{
    streams.clear();
}

void print_stream(const stream_t& stream)
{
    std::string repr = "[";
    for (size_t i = 0; i < 5; i++)
    {
        repr += std::to_string(stream[i]) + ", ";
    }
    repr += "...]";
    LOG(DEBUG, "%s", repr.c_str());
}

bool create_stream(const std::string& name, pstream_t& stream)
{
    ASSERT(streams.count(name) == 0,
           "Attempted to create channel \"%s\" which was already created by "
           "another block",
           name.c_str());
    streams[name] = std::make_shared<stream_t>();
    stream = streams[name];
    return true;
}

bool bind_stream(const std::string& name, pstream_t& stream)
{
    ASSERT(streams.count(name) == 1,
           "Attempted to bind to channel \"%s\" which does not exist. (Are "
           "your blocks are out of order?)",
           name.c_str());
    stream = streams[name];
    return true;
}

}  // namespace Looper