#include "src/audio/audio.h"

#include <vector>

#include "src/framework/log.h"
#include "src/framework/time.h"

namespace Looper
{

/**
 * Whether portaudio has been initialized.
 */
bool is_init = false;

bool init_audio()
{
    /*
     * Initialize portaudio if it hasn't been initialized yet.
     */
    auto err = Pa_Initialize();
    ASSERT(err == paNoError,
           "Failed to initialize PortAudio: %s",
           Pa_GetErrorText(err));
    is_init = true;
    return true;
}

/**
 * Get the device index matching the input name.
 *
 * @param[out] index The matching device index.
 * @param name The name of the device.
 * @param is_input Whether the device is an input.
 * @param is_output Whether the device is an output.
 */
bool get_device_index(PaDeviceIndex &index,
                      const std::string &name,
                      bool is_input,
                      bool is_output)
{
    ASSERT(is_init, "PortAudio was not initialized. Call init_audio()");

    /*
     * Get a list of devices.
     */
    const PaDeviceIndex numDevices = Pa_GetDeviceCount();
    ASSERT(numDevices >= 0,
           "Failed to get devices: %s",
           Pa_GetErrorText(numDevices));

    /*
     * Search for the device in the list of devices.
     */
    std::vector<std::string> available_devices;
    for (PaDeviceIndex i = 0; i < numDevices; i++)
    {
        /*
         * Return the first match.
         */
        const PaDeviceInfo *device_info = Pa_GetDeviceInfo(i);
        const std::string device_name = device_info->name;
        if (device_name.find(name) != std::string::npos)
        {
            if (is_input && device_info->maxInputChannels < 1)
            {
                LOG(INFO, "Device %s was not an input", device_info->name);
                continue;
            }
            else if (is_output && device_info->maxOutputChannels < 1)
            {
                LOG(INFO, "Device %s was not an output", device_info->name);
                continue;
            }

            index = i;
            return true;
        }

        /*
         * Keep track of this device in case we don't find the one we want.
         */
        available_devices.push_back(device_info->name);
    }

    /*
     * If we reach this point, we didn't find a device. Print out all of the
     * devices we found to help a user debug.
     */
    LOG(ERROR, "Could not find audio device \"%s\"", name.c_str());
    LOG(ERROR, "Available devices:");
    for (const std::string &device : available_devices)
    {
        LOG(ERROR, "  - %s", device.c_str());
    }

    return false;
}

/**
 * Callback for when we receive audio.
 *
 * @see PaStreamCallback
 */
int audio_input_callback(const void *input_buffer,
                         void *output_buffer,
                         unsigned long frames_per_buffer,
                         const PaStreamCallbackTimeInfo *time_info,
                         PaStreamCallbackFlags status_flags,
                         void *data)
{
    InputDevice *device = (InputDevice *)data;
    if (frames_per_buffer != device->buf.size())
    {
        LOG(ERROR, "Portaudio gave an unusual number of frames.");
        return paAbort;
    }

    /*
     * Wait until the previous data has been processed.
     */
    std::unique_lock<std::mutex> lock(device->mutex);
    while (device->buffer_full)
    {
        device->cv.wait(lock);
    }

    /*
     * Copy off the device buffer to our internal buffer.
     */
    float *in = (float *)input_buffer;
    for (size_t i = 0; i < device->buf.size(); i++)
    {
        device->buf[i] = *in;
        in++;
    }

    device->buffer_full = true;
    device->cv.notify_one();

    return paContinue;
}

/**
 * Callback for when we need to send audio.
 *
 * @see PaStreamCallback
 */
int audio_output_callback(const void *input_buffer,
                          void *output_buffer,
                          unsigned long frames_per_buffer,
                          const PaStreamCallbackTimeInfo *time_info,
                          PaStreamCallbackFlags status_flags,
                          void *data)
{
    OutputDevice *device = (OutputDevice *)data;
    if (frames_per_buffer != device->buf.size())
    {
        LOG(ERROR, "Portaudio gave an unusual number of frames.");
        return paAbort;
    }

    /*
     * Wait until the underlying stream is ready to send more data.
     */
    std::unique_lock<std::mutex> lock(device->mutex);
    while (!device->buffer_full)
    {
        device->cv.wait(lock);
    }

    /*
     * Copy off the device buffer to our internal buffer.
     */
    float *out = (float *)output_buffer;
    for (size_t i = 0; i < device->buf.size(); i++)
    {
        *out = device->buf[i];
        out++;
    }

    /*
     * Notify everyone that the buffer is empty.
     */
    device->buffer_full = false;
    device->cv.notify_one();

    return paContinue;
}

InputDevice::InputDevice(const BlockConfig _configs) : Source(_configs) {}

bool InputDevice::init()
{
    std::string audio_device_name;
    ASSERT(configs.get_string("device", audio_device_name),
           "Missing audio device name");

    /*
     * Get the device.
     */
    PaDeviceIndex index = -1;
    ASSERT(get_device_index(index, audio_device_name, true, false),
           "Error finding device.");
    const PaDeviceInfo *info = Pa_GetDeviceInfo(index);

    /*
     * Parameters.
     */
    const PaStreamParameters params = {
        .channelCount = 1,
        .device = index,
        .sampleFormat = SAMPLE_WIDTH,
        .suggestedLatency = info->defaultLowInputLatency,
        .hostApiSpecificStreamInfo = NULL};

    /*
     * Open an audio I/O stream.
     */
    PaError err = Pa_OpenStream(&pa_stream,
                                &params,
                                NULL,
                                SAMPLE_RATE,
                                SAMPLES_PER_BUFFER,
                                paNoFlag,
                                &audio_input_callback,
                                (void *)this);
    ASSERT(err == paNoError,
           "Failed to open input device: %s: %s",
           audio_device_name.c_str(),
           Pa_GetErrorText(err));

    /*
     * Read back the stream info.
     */
    const PaStreamInfo *stream_info = Pa_GetStreamInfo(pa_stream);
    LOG(DEBUG,
        "Input stream info: sample_rate=%f, latency=%f",
        stream_info->sampleRate,
        stream_info->inputLatency);

    err = Pa_StartStream(pa_stream);
    ASSERT(err == paNoError,
           "Failed to start device stream: %s: %s",
           audio_device_name.c_str(),
           Pa_GetErrorText(err));

    return true;
}

bool InputDevice::read()
{
    /*
     * Simple copy.
     */
    std::unique_lock<std::mutex> lock(mutex);
    while (!buffer_full)
    {
        cv.wait(lock);
    }

    std::copy(std::begin(buf), std::end(buf), std::begin(*stream));

    buffer_full = false;
    cv.notify_all();

    return true;
}

OutputDevice::OutputDevice(const BlockConfig _configs)
    : Sink(_configs), buffer_full(false)
{
}

bool OutputDevice::init()
{
    std::string audio_device_name;
    ASSERT(configs.get_string("device", audio_device_name),
           "Missing audio device name");

    /*
     * Get the device.
     */
    PaDeviceIndex index = -1;
    ASSERT(get_device_index(index, audio_device_name, false, true),
           "Error finding device.");
    const PaDeviceInfo *info = Pa_GetDeviceInfo(index);
    LOG(DEBUG, "API: %s", Pa_GetHostApiInfo(info->hostApi)->name);

    /*
     * Parameters.
     */
    const PaStreamParameters params = {
        .channelCount = 1,
        .device = index,
        .sampleFormat = SAMPLE_WIDTH,
        .suggestedLatency = info->defaultLowOutputLatency,
        .hostApiSpecificStreamInfo = NULL};

    /*
     * Open an audio I/O stream.
     */
    PaError err = Pa_OpenStream(&pa_stream,
                                NULL,
                                &params,
                                SAMPLE_RATE,
                                SAMPLES_PER_BUFFER,
                                paNoFlag,
                                &audio_output_callback,
                                (void *)this);
    ASSERT(err == paNoError,
           "Failed to open output device: %s: %s",
           audio_device_name.c_str(),
           Pa_GetErrorText(err));

    /*
     * Read back the stream info.
     */
    const PaStreamInfo *stream_info = Pa_GetStreamInfo(pa_stream);
    LOG(DEBUG,
        "Output stream info: sample_rate=%f, latency=%f",
        stream_info->sampleRate,
        stream_info->outputLatency);

    err = Pa_StartStream(pa_stream);
    ASSERT(err == paNoError,
           "Failed to start device stream: %s: %s",
           audio_device_name.c_str(),
           Pa_GetErrorText(err));

    return true;
}

bool OutputDevice::write()
{
    /*
     * Wait until the underlying stream is ready to receive more data.
     */
    std::unique_lock<std::mutex> lock(mutex);
    while (buffer_full)
    {
        cv.wait(lock);
    }

    /*
     * Simple copy.
     */
    std::copy(std::begin(*stream), std::end(*stream), std::begin(buf));

    /*
     * Notify the stream that there's more data available.
     */
    buffer_full = true;
    cv.notify_all();
    return true;
}

}  // namespace Looper