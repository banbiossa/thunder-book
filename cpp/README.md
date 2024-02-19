# thunder-book

- thunder book の実装

## setup

- Bazel で build + gtest で test
    - `brew install bazel` 等でbazelさえあればOK
- `bazel build //src/test/ch03:play-maze -c opt` で build
- `./bazel-bin/src/ch03/play-maze` で run
- `bazel test --test_output=errors test/ch03:maze-state-test` で test


### debug
- `bazel build --config debug //src/ch08/plays:bitstate`
- `lldb ../../bazel-bin/src/ch08/plays/bitstate`
- `breakpoint set -f <file> -l <lineno>`
- `run`
    - n
    - s
    - frame variable <var>
    - c
