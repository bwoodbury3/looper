load("@bazel_skylib//rules:copy_file.bzl", "copy_file")

PYBIND_COPTS = ["-fexceptions"]

PYBIND_FEATURES = [
    "-use_header_modules",  # Required for pybind11.
    "-parse_headers",
]

PYBIND_DEPS = [
    Label("@pybind11//:pybind11"),
    "@python_3_11//:python_headers",
]

# Builds a Python extension module using pybind11.
# This can be directly used in Python with the import statement.
# Assuming the name NAME, the following targets will be defined:
#   1. NAME.so - the shared/dynamic library for the extension module
#   3. NAME - an alias pointing to NAME.so
# Generally, the user will "depend" on this extension module via the
# data attribute of their py_* target; specifying NAME is preferred.
def pybind_extension(
        name,
        copts = [],
        features = [],
        linkopts = [],
        tags = [],
        deps = [],
        **kwargs):
    # Mark common dependencies as required for build_cleaner.
    tags = tags + ["req_dep=%s" % dep for dep in PYBIND_DEPS]

    native.cc_binary(
        name = name + ".so",
        copts = copts + PYBIND_COPTS + ["-fvisibility=hidden"],
        features = features + PYBIND_FEATURES,
        linkopts = linkopts + ["-undefined", "dynamic_lookup"],
        linkshared = 1,
        tags = tags,
        deps = deps + PYBIND_DEPS,
        **kwargs
    )
