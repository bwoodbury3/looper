#pragma once

#include <ctype.h>

#include <chrono>

typedef int64_t nano_t;

namespace Looper
{

/**
 * A monotonic clock.
 */
std::chrono::steady_clock monotonic_clock;

/**
 * Wrapper for getting the current monotonic time in nanoseconds, since chrono
 * is literally the ugliest library on earth.
 *
 * @return A monotonically increasing count of nanoseconds.
 */
inline nano_t monotonic_time()
{
    return std::chrono::duration_cast<std::chrono::nanoseconds>(
               monotonic_clock.now().time_since_epoch())
        .count();
}

}  // namespace Looper