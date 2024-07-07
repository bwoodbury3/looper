#include "src/framework/segment.h"

#include "src/framework/log.h"

namespace Looper
{

bool Segment::to_segment_type(const std::string& type_str,
                              segment_type_t& segment_type)
{
    if (type_str == "input")
    {
        segment_type = segment_type_t::input;
    }
    else if (type_str == "output")
    {
        segment_type = segment_type_t::output;
    }
    else
    {
        ABORT("Unrecognized segment type: %s", type_str.c_str());
    }

    return true;
}

}  // namespace Looper