load("@rules_cc//cc:defs.bzl", "cc_test")

cc_test(
    name = "test",
    srcs = [
      "maze_state_test.cc",
      "random_action_test.cc",
      "monte_carlo_test.cc",
      "duct_test.cc",
      "alternate_maze_state_test.cc",
    ],
    copts = [
      "-Iexternal/gtest/googletest/include",
      "-Iexternal/gtest/googletest",
    ],
    deps = [
        "@gtest//:main",
        "//src/ch06:maze_state",
    ],
)
