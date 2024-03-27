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

cc_test(
    name = "greedy",
    srcs = [
      "test_greedy.cc",
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
