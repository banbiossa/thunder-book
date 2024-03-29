load("@rules_cc//cc:defs.bzl", "cc_test")

cc_test(
    name = "util_test",
    srcs = [
      "util_test.cc",
    ],
    copts = [
      "-Iexternal/gtest/googletest/include",
      "-Iexternal/gtest/googletest",
    ],
    deps = [
        "@gtest//:main",
        "//src:util",
    ],
)
