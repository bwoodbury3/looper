#include "src/framework/stream.h"

namespace Looper
{

std::string print_stream(const stream_t& stream)
{
    std::string repr = "[";
    for (size_t i = 0; i < 5; i++)
    {
        repr += std::to_string(stream[i]) + ", ";
    }
    repr += "...]";
    return repr;
}

}