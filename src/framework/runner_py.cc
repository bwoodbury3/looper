/**
 * pybinds for interfacing with runner.h
 */

#include <pybind11/pybind11.h>

#include "src/audio/audio.h"
#include "src/framework/runner.h"
#include "src/modules/modules.h"

namespace py = pybind11;

PYBIND11_MODULE(looper, m)
{
    /*
     * Runner
     */
    py::class_<Looper::Runner>(m, "Runner")
        .def(py::init())
        .def("run",
             &Looper::Runner::run,
             py::call_guard<py::gil_scoped_release>())
        .def("stop", &Looper::Runner::stop);

    /*
     * One-time functions.
     */
    m.def("init_audio", &Looper::init_audio, "Initialize portaudio (required)");
    m.def("register_modules",
          &Looper::register_all_modules,
          "Initialize all modules");
}
