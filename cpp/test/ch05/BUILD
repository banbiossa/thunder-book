load("@rules_cc//cc:defs.bzl", "cc_test")

cc_test(
    name = "test",
    srcs = [
      "maze_state_test.cc",
      "mini_max_test.cc",
      "random_action_test.cc",
      "alpha_beta_test.cc",
      "monte_carlo_test.cc",
      "iterative_deepening_test.cc",
      "monte_carlo_tree_search_test.cc",
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
