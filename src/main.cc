/**
 * Go!
 */

#include <unistd.h>

#include <fstream>
#include <sstream>

#include "src/audio/audio.h"
#include "src/framework/config.h"
#include "src/framework/runner.h"
#include "src/modules/modules.h"

namespace Looper
{

/**
 * Go!
 *
 * @return True on success.
 */
bool go(const std::string& filename)
{
    ASSERT(init_audio(), "Could not initialize audio");
    register_all_modules();

    Runner runner;

    /*
     * Read json file.
     */
    std::ifstream stream(filename);
    ASSERT(stream.good(), "File not found: %s", filename.c_str());
    std::stringstream buffer;
    buffer << stream.rdbuf();
    const std::string config_str = buffer.str();

    /*
     * Run the runner.
     */
    runner.run(config_str);
    runner.stop();

    return true;
}

}  // namespace Looper

void help()
{
    fprintf(stderr, "~ L 0 0 P E R ~\n");
    fprintf(stderr, "Usage:\n");
    fprintf(stderr, "\tbazel run //src:looper <file.json>\n");
}

int main(int argc, const char** argv)
{
    if (argc < 2)
    {
        help();
        return -1;
    }

    const std::string filename(argv[1]);
    if (!Looper::go(filename))
    {
        return -1;
    }

    return 0;
}