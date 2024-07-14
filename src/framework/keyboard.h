#include <vector>

namespace Looper::Keyboard
{

/**
 * Initialize the keyboard.
 */
bool init();

/**
 * Get all keys that were pressed last cycle.
 *
 * @return The keys.
 */
const std::vector<std::string> &get_keys();

/**
 * Called by framework code to start the next cycle and checkpoint all of the
 * keys that were pressed.
 *
 * @return True if the program receives EOF.
 */
bool reset();

/**
 * Out-of-band call to inject a keypress into the system via some mechanism
 * other than stdin.
 *
 * @param key The key that was pressed.
 */
void queue_keypress(const std::string &key);

}