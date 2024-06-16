/**
 * Control flow framework.
 */

#pragma once

#include <functional>
#include <map>
#include <memory>
#include <nlohmann/json.hpp>
#include <string>

#include "src/framework/log.h"
#include "src/framework/stream.h"

namespace Looper
{

using json = nlohmann::json;

/**
 * BlockConfig object that can be passed to objects to contain arbitrary
 * user-defined configuration.
 */
class BlockConfig
{
   public:
    /**
     * Constructor
     */
    BlockConfig(const std::string &_name, json &_base);

    /**
     * Get a string from a key.
     */
    bool get_string(const std::string &key, std::string &value) const;

    /**
     * Get a list of strings from a key.
     */
    bool get_string_v(const std::string &key,
                      std::vector<std::string> &value) const;

    /**
     * The name of the block. Helpful for debug prints.
     */
    const std::string name;

   private:
    /**
     * The backing json config object.
     */
    json base;
};

/**
 * The root of all boilerplate. Blocks can implement any of the below functions
 * to run code at various control flow stages.
 */
class Block
{
   public:
    /**
     * Constructor
     */
    Block(const BlockConfig &_configs);

    virtual ~Block() = default;

    /**
     * Initialization function, guaranteed to be called before any audio i/o.
     */
    virtual bool init();

    /**
     * The block's name.
     */
    const std::string &name() const;

   protected:
    const BlockConfig configs;
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
     */
    Source(const BlockConfig &_configs);

    /**
     * Initialize a source. This is called by framework code.
     */
    bool init_source();

    /**
     * Read from the source.
     *
     * @return True on success.
     */
    virtual bool read() = 0;

    /**
     * The output channel name of this source.
     */
    std::string output_channel;

    /**
     * The output channel.
     */
    pstream_t stream;
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
     */
    Sink(const BlockConfig &_configs);

    /**
     * Initialize a sink. This is called by framework code.
     */
    bool init_sink();

    /**
     * Write to the sink.
     *
     * @return True on success.
     */
    virtual bool write() = 0;

    /**
     * The input channel name for this sink.
     */
    std::string input_channel;

    /**
     * The input stream.
     */
    pstream_t stream;
};

/**
 * Object which receives samples and produces other samples. Like filters or
 * mixers. They may have N inputs and 1 output.
 */
class Transformer : public Block
{
   public:
    /**
     * Constructor.
     */
    Transformer(const BlockConfig &_configs);

    /**
     * Initialize a transformer. This is called by framework code.
     */
    bool init_transformer();

    /**
     * Transform a set of input streams into a set of output streams.
     *
     * @return True on success.
     */
    virtual bool transform() = 0;

    /**
     * The input channel list.
     */
    std::vector<std::string> input_channels;

    /**
     * The output channel list.
     */
    std::vector<std::string> output_channels;

    /**
     * The input streams.
     */
    std::vector<pstream_t> input_streams;

    /**
     * The output streams.
     */
    std::vector<pstream_t> output_streams;
};

/**
 * Convenience wrappers for shared_ptrs.
 */
using pSource = std::shared_ptr<Source>;
using pSink = std::shared_ptr<Sink>;
using pTransformer = std::shared_ptr<Transformer>;

/**
 * Factory which builds blocks.
 */
namespace BlockFactory
{

/**
 * Define factories that create each type of block.
 */
using SourceBuilder = std::function<pSource(const BlockConfig &)>;
using SinkBuilder = std::function<pSink(const BlockConfig &)>;
using TransformerBuilder = std::function<pTransformer(const BlockConfig &)>;

/**
 * Factories for types registered with the block API.
 */
inline std::map<std::string, SourceBuilder> source_builders{};
inline std::map<std::string, SinkBuilder> sink_builders{};
inline std::map<std::string, TransformerBuilder> transformer_builders{};

/**
 * Register a new Source block with the framework.
 *
 * @param tname The tname of the block.
 */
template <typename T>
static bool register_source(const std::string &tname)
{
    static_assert(std::is_base_of<Source, T>());
    ASSERT(source_builders.count(tname) == 0,
           "Duplicate \"%s\" already registered with the factory",
           tname.c_str());

    source_builders[tname] = [](const BlockConfig &configs)
    { return std::make_shared<T>(configs); };

    LOG(DEBUG, "Registered source \"%s\"", tname.c_str());

    return true;
}

/**
 * Register a new Source block with the framework.
 *
 * @param tname The tname of the block.
 */
template <typename T>
static bool register_sink(const std::string &tname)
{
    static_assert(std::is_base_of<Sink, T>());
    ASSERT(sink_builders.count(tname) == 0,
           "Duplicate \"%s\" already registered with the factory",
           tname.c_str());

    sink_builders[tname] = [](const BlockConfig &configs)
    { return std::make_shared<T>(configs); };

    LOG(DEBUG, "Registered sink \"%s\"", tname.c_str());

    return true;
}

/**
 * Register a new Source block with the framework.
 *
 * @param tname The tname of the block.
 */
template <typename T>
static bool register_transformer(const std::string &tname)
{
    static_assert(std::is_base_of<Transformer, T>());
    ASSERT(transformer_builders.count(tname) == 0,
           "Duplicate \"%s\" already registered with the factory",
           tname.c_str());

    transformer_builders[tname] = [](const BlockConfig &configs)
    { return std::make_shared<T>(configs); };

    LOG(DEBUG, "Registered transformer \"%s\"", tname.c_str());

    return true;
}

/**
 * Whether a block tname is a source.
 */
bool is_source(const std::string &tname);

/**
 * Whether a block tname is a sink.
 */
bool is_sink(const std::string &tname);

/**
 * Whether a block tname is a transformer.
 */
bool is_transformer(const std::string &tname);

/**
 * Build a source.
 *
 * @param tname The tname of the source type.
 * @param config The config.
 * @param[out] source The output.
 */
bool build_source(const std::string &tname,
                  const BlockConfig &config,
                  pSource &source);

/**
 * Build a sink.
 *
 * @param tname The tname of the sink type.
 * @param config The config.
 * @param[out] sink The output.
 */
bool build_sink(const std::string &tname,
                const BlockConfig &config,
                pSink &sink);

/**
 * Build a transformer.
 *
 * @param tname The tname of the transformer type.
 * @param config The config.
 * @param[out] transformer The output.
 */
bool build_transformer(const std::string &tname,
                       const BlockConfig &config,
                       pTransformer &transformer);

}  // namespace BlockFactory

}  // namespace Looper