
load("@rules_cc//cc:defs.bzl", "cc_test")

cc_test(
    name = "test",
    srcs = ["test.cc"],
    copts = [
      "-Iexternal/gtest/googletest/include",
      "-Iexternal/gtest/googletest",
    ],
    deps = [
        "@gtest//:main",
        "//src/ch08:try_bits",
    ],
)
