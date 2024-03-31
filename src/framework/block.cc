#include "src/framework/block.h"

namespace Looper
{

bool Block::init() { return true; }

Source::Source(const std::string &_output_channel)
    : output_channel(_output_channel)
{
}

Sink::Sink(const std::string &_input_channel) : input_channel(_input_channel) {}

}  // namespace Looper