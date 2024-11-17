# Run this script if you change rust dependencies

set -ex

CARGO_BAZEL_REPIN=true bazel build //...
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
