load("@rules_cc//cc:defs.bzl", "cc_test")

cc_test(
    name = "test",
    srcs = [
      "maze_state_test.cc"
    ],
    copts = [
      "-Iexternal/gtest/googletest/include",
      "-Iexternal/gtest/googletest",
    ],
    deps = [
        "@gtest//:main",
        "//src/ch05:maze_state",
    ],
)
