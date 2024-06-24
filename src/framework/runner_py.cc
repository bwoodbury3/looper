/**
 * pybinds for interfacing with runner.h
 */

#include <pybind11/pybind11.h>

#include "src/audio/audio.h"
#include "src/framework/runner.h"
#include "src/modules/modules.h"

PYBIND11_MODULE(looper, m)
{
    /*
     * Runner
     */
    pybind11::class_<Looper::Runner>(m, "Runner")
        .def(pybind11::init())
        .def("run", &Looper::Runner::run)
        .def("stop", &Looper::Runner::stop);

    /*
     * One-time functions.
     */
    m.def("init_audio", &Looper::init_audio, "Initialize portaudio (required)");
    m.def("register_modules",
          &Looper::register_all_modules,
          "Initialize all modules");
}
