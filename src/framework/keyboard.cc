#include "src/framework/keyboard.h"

#include <termios.h>
#include <unistd.h>

#include <mutex>

#include "src/framework/log.h"

namespace Looper::Keyboard
{

constexpr size_t BUFSIZE = 256;

/**
 * Snapshot of the keys that were pressed last cycle.
 */
std::vector<std::string> keys;

/**
 * The stdin settings on the terminal prior to starting the program.
 */
struct termios old_settings;

void _prep_stdin()
{
    struct termios new_settings;
    tcgetattr(fileno(stdin), &old_settings);
    new_settings = old_settings;

    /* Turn off canonical mode to read in character by character */
    new_settings.c_lflag &= ~ICANON;
    /* Echo characters to terminal */
    new_settings.c_lflag |= ECHO;

    tcsetattr(fileno(stdin), TCSANOW, &new_settings);
}

void _reset_stdin()
{
    /* Manually set echo in case this messed up from a previous run.*/
    old_settings.c_lflag |= ECHO;
    tcsetattr(fileno(stdin), TCSANOW, &old_settings);
}

bool init()
{
    _prep_stdin();
    return true;
}

const std::vector<std::string> &get_keys()
{
    return keys;
}

bool reset()
{
    keys.clear();

    fd_set set;
    FD_ZERO(&set);
    FD_SET(fileno(stdin), &set);

    /* nonblocking */
    struct timeval tv = {.tv_sec = 0, .tv_usec = 0};
    int res = select(fileno(stdin) + 1, &set, NULL, NULL, &tv);

    if (res > 0)
    {
        char buf[BUFSIZE];
        auto count = read(fileno(stdin), &buf, BUFSIZE);
        keys.reserve(count);

        /*
         * Read stdin char-by-char.
         */
        for (ssize_t i = 0; i < count; i++)
        {
            const char c = buf[i];
            if (c == 0x3 /* EOF */)
            {
                _reset_stdin();
                return false;
            }

            keys.push_back({c});
            LOG(DEBUG, "Keypress: %s", keys[i].c_str());
        }
    }

    return true;
}

}  // namespace Looper::Keyboard