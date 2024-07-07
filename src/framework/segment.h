#pragma once

#include <string>

namespace Looper
{

/**
 * The type of segment.
 */
enum segment_type_t
{
    input,
    output,
};

/**
 * A segment of data.
 */
struct Segment
{
    /**
     * The starting measure.
     */
    float start;

    /**
     * The stop measure.
     */
    float stop;

    /**
     * The type of segment.
     */
    segment_type_t segment_type;

    /**
     * Get a segment type from a raw string.
     *
     * @param type_str The string type of segment.
     * @param[out] segment_type The segment type output.
     *
     * @return True on success.
     */
    static bool to_segment_type(const std::string& type_str,
                                segment_type_t& segment_type);
};

}  // namespace Looper