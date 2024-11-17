# Run this script if you change rust dependencies

bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
CARGO_BAZEL_REPIN=true bazel build //...
