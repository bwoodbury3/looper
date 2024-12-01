load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_library", "rust_test")

# Macro for instantiating a rust_library and its associated tests/data.
def looper_library(name, srcs, deps = [], data = [], test_deps = [], test_data = []):
    rust_library(
        name = name,
        srcs = srcs,
        deps = deps,
        data = data,
    )

    # Not really used right now and slowing tests down.
    # rust_doc(
    #     name = "{}_doc".format(name),
    #     crate = ":{}".format(name),
    # )

    rust_test(
        name = "{}_test".format(name),
        crate = ":{}".format(name),
        deps = test_deps,
        data = test_data,
    )
