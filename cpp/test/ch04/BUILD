load("@rules_cc//cc:defs.bzl", "cc_test")

cc_test(
    name = "maze_state_test",
    srcs = [
      "maze_state_test.cc",
      "random_action_test.cc",
    ],
    copts = [
      "-Iexternal/gtest/googletest/include",
      "-Iexternal/gtest/googletest",
    ],
    deps = [
        "@gtest//:main",
        "//src/ch04:maze_state",
    ],
)
