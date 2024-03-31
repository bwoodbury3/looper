/**
 * Go!
 */

#include <unistd.h>

#include "audio/audio.h"
#include "framework/log.h"

namespace Looper
{

/**
 * Go!
 *
 * @return True on success.
 */
bool go()
{
    ASSERT(Looper::init_audio(), "Could not initialize audio");

    Looper::InputDevice input("Microphone", "mic");
    ASSERT(input.init(), "Could not init mic");

    // Looper::OutputDevice output("Beats Fit Pro", "mic");
    Looper::OutputDevice output("Output", "mic");
    ASSERT(output.init(), "Could not init output");

    while (true)
    {
        stream_t stream;
        ASSERT(input.read(stream), "Could not read from the stream");
        ASSERT(output.write(stream), "Could not write to the stream");
    }

    return true;
}

}  // namespace Looper

int main(int argc, const char **argv)
{
    if (!Looper::go())
    {
        return -1;
    }

    return 0;
}