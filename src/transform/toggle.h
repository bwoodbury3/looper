#pragma once

#include "src/framework/block.h"

namespace Looper
{

class Toggle : public Transformer
{
   public:
    /**
     * Constructor.
     */
    Toggle(const BlockConfig &_configs);

    bool init() override;
    bool transform() override;
};

}  // namespace Looper