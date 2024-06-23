/**
 * Trivial log framework.
 */

#pragma once

#include <stdio.h>

/**
 * Log level.
 */
#define DEBUG 0
#define INFO 1
#define WARN 2
#define ERROR 3

/**
 * The lowest log level that will be printed.
 */
#define LOG_LEVEL 0

/**
 * Log a message.
 *
 * @note newlines automatically added!
 *
 * @param level The log level (DEBUG, INFO, WARN, ERROR).
 * @param fmt printf style string format.
 */
#define LOG(level, fmt, ...)                 \
    do                                       \
    {                                        \
        if (level >= LOG_LEVEL)              \
        {                                    \
            fprintf(stderr,                  \
                    "%s:%d:%s(): " fmt "\n", \
                    __FILE__,                \
                    __LINE__,                \
                    __func__,                \
                    ##__VA_ARGS__);          \
        }                                    \
    } while (0)

/**
 * Assert that an op is true. Return false otherwise.
 *
 * @param result A bool result.
 * @param fmt String to print on failure.
 */
#define ASSERT(result, fmt, ...)                            \
    do                                                      \
    {                                                       \
        if (!(result))                                      \
        {                                                   \
            fprintf(stderr,                                 \
                    "%s:%d:%s(): ASSERT(false): " fmt "\n", \
                    __FILE__,                               \
                    __LINE__,                               \
                    __func__,                               \
                    ##__VA_ARGS__);                         \
            return false;                                   \
        }                                                   \
    } while (0)

/**
 * Assert that an op is true. Return false otherwise. No error message.
 *
 * @param result A bool result.
 */
#define QASSERT(result)                            \
    do                                             \
    {                                              \
        if (!(result))                             \
        {                                          \
            fprintf(stderr,                        \
                    "%s:%d:%s(): ASSERT(false)\n", \
                    __FILE__,                      \
                    __LINE__,                      \
                    __func__);                     \
            return false;                          \
        }                                          \
    } while (0)

/**
 * Abort.
 *
 * @param fmt String to print.
 */
#define ABORT(fmt, ...)                           \
    do                                            \
    {                                             \
        fprintf(stderr,                           \
                "%s:%d:%s(): ABORT(): " fmt "\n", \
                __FILE__,                         \
                __LINE__,                         \
                __func__,                         \
                ##__VA_ARGS__);                   \
        return false;                             \
    } while (0)

/**
 * Assert that an op is true. throw an exception otherwise.
 *
 * @param result A bool result.
 * @param fmt String to print on failure.
 */
#define EASSERT(result, fmt, ...)                                    \
    do                                                               \
    {                                                                \
        if (!(result))                                               \
        {                                                            \
            fprintf(stderr,                                          \
                    "%s:%d:%s(): ASSERT(false): " fmt "\n",          \
                    __FILE__,                                        \
                    __LINE__,                                        \
                    __func__,                                        \
                    ##__VA_ARGS__);                                  \
            throw std::runtime_error("Fatal error, see call stack"); \
        }                                                            \
    } while (0)
