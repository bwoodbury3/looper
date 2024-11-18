# Run this script before you check in your code.

set -ex

bazel run //:rustfmt
bazel test //...
