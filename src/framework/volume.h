#pragma once

#include "src/framework/datatypes.h"

namespace Looper
{

/**
 * Scale the volume of a clip by a float amount.
 *
 * @param clip The audio clip to modify.
 * @param volume The volume/amplitude scale factor.
 */
void scale_volume(paudio_clip_t clip, const float volume);

}  // namespace Looper