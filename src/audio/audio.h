#pragma once

#include <portaudio.h>

#include <mutex>
#include <string>

#include "src/framework/block.h"

namespace Looper
{

/**
 * Audio sample widths. Must match the type in stream.h.
 */
#define SAMPLE_WIDTH paFloat32

/**
 * Scan all available audio devices on the system. This must be called once at
 * the start.
 *
 * @return True on success.
 */
bool init_audio();

/**
 * Input audio device.
 */
class InputDevice : public Source
{
   public:
    /**
     * Initialize an input audio I/O device.
     *
     * @param _configs The configs.
     */
    InputDevice(const BlockConfig _configs);

    virtual bool init() override;
    virtual bool read() override;

    /**
     * Stream holding the latest round of data.
     */
    stream_t buf;

    /**
     * Mutex for interacting with the buffer.
     */
    std::mutex mutex;

   private:
    /**
     * Pointer to the portaudio stream.
     */
    PaStream *pa_stream;
};

/**
 * Output audio device.
 */
class OutputDevice : public Sink
{
   public:
    /**
     * Initialize an output audio I/O device.
     *
     * @param _configs The configs.
     */
    OutputDevice(const BlockConfig _configs);

    virtual bool init() override;
    virtual bool write() override;

    /**
     * Stream holding the latest round of data.
     */
    stream_t buf;

    /**
     * Mutex for interacting with the buffer.
     */
    std::mutex mutex;

    /**
     * Condition variable.
     */
    std::condition_variable cv;

    /**
     * Condition variable for buffer being full.
     */
    bool buffer_full;

   private:
    /**
     * Pointer to the portaudio stream.
     */
    PaStream *pa_stream;
};

}  // namespace Looper