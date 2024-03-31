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
     * @param _audio_device_name The name of the device given by the platform.
     * @param _output_channel See Looper::Source
     */
    InputDevice(const std::string &_audio_device_name,
                const std::string &_output_channel);

    virtual bool init() override;
    virtual bool read(stream_t &stream) override;

    /**
     * Stream holding the latest round of data.
     */
    stream_t buf;

    /**
     * Mutex for reading from the buffer.
     */
    std::mutex mutex;

   private:
    /**
     * The name of the audio device.
     */
    const std::string &audio_device_name;

    /**
     * Pointer to the portaudio stream.
     */
    PaStream *pa_stream;
};

}  // namespace Looper