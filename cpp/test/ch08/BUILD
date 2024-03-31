load("@rules_cc//cc:defs.bzl", "cc_test")

cc_test(
    name = "test",
    srcs = [
      "try_bits_test.cc",
      "maze_state_test.cc",
    ],
    copts = [
      "-Iexternal/gtest/googletest/include",
      "-Iexternal/gtest/googletest",
    ],
    deps = [
        "@gtest//:main",
        "//src/ch08:try_bits",
        "//src/ch08:maze_state",
    ],
)
