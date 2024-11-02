#include "src/framework/volume.h"

namespace Looper
{

void scale_volume(paudio_clip_t clip, const float volume)
{
    for (size_t i = 0; i < clip->size(); i++)
    {
        (*clip)[i] *= volume;
    }
}

}