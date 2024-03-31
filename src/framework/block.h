/**
 * Control flow framework.
 */

#pragma once

#include <string>

#include "src/framework/stream.h"

namespace Looper
{

/**
 * The root of all boilerplate. Blocks can implement any of the below functions
 * to run code at various control flow stages.
 */
class Block
{
   public:
    /**
     * Initialization function, guaranteed to be called before any audio i/o.
     */
    virtual bool init();
};

/**
 * Object which produces audio samples. Executed at the top of a dispatch
 * loop.
 */
class Source : public Block
{
   public:
    /**
     * Constructor.
     *
     * @param _output_channel The name of the output channel.
     */
    Source(const std::string &_output_channel);

    /**
     * Read from the source.
     *
     * @param[out] stream The stream.
     *
     * @return True on success.
     */
    virtual bool read(stream_t &stream) = 0;

    /**
     * The output channel name of this source.
     */
    const std::string output_channel;
};

/**
 * Object which receives audio samples for playback. Executed at the bottom of
 * a dispatch loop.
 */
class Sink : public Block
{
   public:
    /**
     * Constructor.
     *
     * @param _input_channel The name of the input channel.
     */
    Sink(const std::string &_input_channel);

    /**
     * Write to the sink.
     *
     * @param stream The stream.
     *
     * @return True on success.
     */
    virtual bool write(const stream_t &stream) = 0;

    /**
     * The input channel name for this sink.
     */
    const std::string input_channel;
};

/**
 * Object which receives samples and produces other samples. Like filters or
 * mixers. They may have N inputs and 1 output.
 */
template <size_t num_inputs, size_t num_outputs>
class Transformer : public Block
{
   public:
    typedef std::array<stream_t, num_inputs> input_streams_t;
    typedef std::array<stream_t, num_outputs> output_streams_t;

    /**
     * Constructor.
     *
     * @param _input_channels The names of the input channels.
     * @param _output_channels The names of the output channels.
     */
    Transformer(const std::array<std::string, num_inputs> &_input_channels,
                const std::array<std::string, num_outputs> &_output_channels)
        : input_channels(_input_channels), output_channels(_output_channels)
    {
    }

    /**
     * Transform a set of input streams into a set of output streams.
     *
     * @param inputs The input streams.
     * @param outputs The output streams.
     */
    virtual bool transform(const input_streams_t &inputs,
                           output_streams_t &outputs) = 0;

    const std::array<std::string, num_inputs> &input_channels;
    const std::array<std::string, num_outputs> &output_channels;
};

}  // namespace Looper