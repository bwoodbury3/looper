#pragma once

#include "src/framework/block.h"

namespace Looper
{

class Combiner : public Transformer
{
   public:
    /**
     * Constructor.
     */
    Combiner(const BlockConfig &_configs);

    bool init() override;
    bool transform() override;
};

}  // namespace Looper