#include "src/framework/block.h"

#include "src/framework/json_util.h"
#include "src/framework/log.h"

namespace Looper
{

BlockConfig::BlockConfig(const std::string &_name, const json &_base)
    : name(_name), base(_base)
{
}

bool BlockConfig::get_string(const std::string &key, std::string &value) const
{
    ASSERT(::Looper::get_string(base, key, value),
           "Error parsing block \"%s\"",
           name.c_str());
    return true;
}

bool BlockConfig::get_int(const std::string &key, int &value) const
{
    ASSERT(::Looper::get_int(base, key, value),
           "Error parsing block \"%s\"",
           name.c_str());
    return true;
}

bool BlockConfig::get_int_default(const std::string &key,
                                  const int _default,
                                  int &value) const
{
    ASSERT(::Looper::get_int_default(base, key, _default, value),
           "Error parsing block \"%s\"",
           name.c_str());
    return true;
}

bool BlockConfig::get_float(const std::string &key, float &value) const
{
    ASSERT(::Looper::get_float(base, key, value),
           "Error parsing block \"%s\"",
           name.c_str());
    return true;
}

bool BlockConfig::get_float_default(const std::string &key,
                                    const float _default,
                                    float &value) const
{
    ASSERT(::Looper::get_float_default(base, key, _default, value),
           "Error parsing block \"%s\"",
           name.c_str());
    return true;
}

bool BlockConfig::get_string_v(const std::string &key,
                               std::vector<std::string> &value) const
{
    ASSERT(::Looper::get_string_v(base, key, value),
           "Error parsing block \"%s\"",
           name.c_str());
    return true;
}

Block::Block(const BlockConfig &_configs) : configs(_configs) {}

bool Block::init()
{
    return true;
}

const std::string &Block::name() const
{
    return configs.name;
}

Source::Source(const BlockConfig &_configs) : Block(_configs) {}

bool Source::init_source()
{
    ASSERT(configs.get_string("output_channel", output_channel),
           "Missing output_channel parameter for \"%s\"",
           configs.name.c_str());
    ASSERT(create_stream(output_channel, stream),
           "Failed to create output stream for \"%s\"",
           output_channel.c_str());

    return true;
}

Sink::Sink(const BlockConfig &_configs) : Block(_configs) {}

bool Sink::init_sink()
{
    ASSERT(configs.get_string("input_channel", input_channel),
           "Missing input_channel parameter for \"%s\"",
           configs.name.c_str());
    ASSERT(bind_stream(input_channel, stream),
           "Failed to bind to input stream for \"%s\"",
           input_channel.c_str());

    return true;
}

Transformer::Transformer(const BlockConfig &_configs) : Block(_configs) {}

bool Transformer::init_transformer()
{
    ASSERT(configs.get_string_v("input_channels", input_channels),
           "Missing input_channels parameter for \"%s\"",
           configs.name.c_str());
    ASSERT(configs.get_string_v("output_channels", output_channels),
           "Missing output_channels parameter for \"%s\"",
           configs.name.c_str());

    for (const auto &output_channel : output_channels)
    {
        pstream_t pstream;
        ASSERT(create_stream(output_channel, pstream),
               "Failed to create output stream for \"%s\"",
               output_channel.c_str());
        output_streams.push_back(pstream);
    }
    for (const auto &input_channel : input_channels)
    {
        pstream_t pstream;
        ASSERT(bind_stream(input_channel, pstream),
               "Failed to bind to input stream for \"%s\"",
               input_channel.c_str());
        input_streams.push_back(pstream);
    }

    return true;
}

namespace BlockFactory
{

bool is_source(const std::string &tname)
{
    return source_builders.count(tname) > 0;
}

bool is_sink(const std::string &tname)
{
    /* Comment so that formatting doesn't try to one-line this */
    return sink_builders.count(tname) > 0;
}

bool is_transformer(const std::string &tname)
{
    return transformer_builders.count(tname) > 0;
}

bool build_source(const std::string &tname,
                  const BlockConfig &config,
                  pSource &source)
{
    ASSERT(is_source(tname), "Cannot build \"%s\" as a source.", tname.c_str());
    source = source_builders[tname](config);
    return true;
}

bool build_sink(const std::string &tname,
                const BlockConfig &config,
                pSink &sink)
{
    ASSERT(is_sink(tname), "Cannot build \"%s\" as a sink.", tname.c_str());
    sink = sink_builders[tname](config);
    return true;
}

bool build_transformer(const std::string &tname,
                       const BlockConfig &config,
                       pTransformer &transformer)
{
    ASSERT(is_transformer(tname),
           "Cannot build \"%s\" as a transformer.",
           tname.c_str());
    transformer = transformer_builders[tname](config);
    return true;
}

}  // namespace BlockFactory

}  // namespace Looper