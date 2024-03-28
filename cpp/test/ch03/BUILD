load("@rules_cc//cc:defs.bzl", "cc_test")

cc_test(
    name = "maze-state-test",
    srcs = [
      "maze-state-test.cc",
    ],
    copts = [
      "-Iexternal/gtest/googletest/include",
      "-Iexternal/gtest/googletest",
    ],
    deps = [
        "@gtest//:main",
        "//src/ch03:maze-state",
    ],
)

# these can be the same suite but separting
# because maze-state-test was here
# before the params were added
cc_test(
    name = "test_algorithm",
    srcs = [
      "test_greedy.cc",
      "test_beam_search.cc",
      "test_time_keeper.cc",
      "test_chokudai_search.cc",
    ],
    copts = [
      "-Iexternal/gtest/googletest/include",
      "-Iexternal/gtest/googletest",
    ],
    deps = [
        "@gtest//:main",
        "//src/ch03:maze-state",
    ],
)
