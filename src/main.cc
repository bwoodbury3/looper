/**
 * Go!
 */

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

    Looper::InputDevice input("sldas", "mic");
    ASSERT(input.init(), "Could not start program");

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