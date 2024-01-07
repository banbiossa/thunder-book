load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")

cc_library(
    name="maze-state",
    srcs=["maze-state.cc"],
    hdrs=["maze-state.h"],
    visibility=[
        "//test/ch03:__pkg__",
    ],
)

cc_binary(
    name="play-maze",
    srcs=["play-maze.cc"],
    deps=[
        ":maze-state",
    ],
)

cc_library(
    name="greedy",
    srcs=["greedy.cc"],
    hdrs=["greedy.h"],
    deps=[
        ":maze-state",
    ],
    visibility=[
        "//test/ch03:__pkg__",
    ],
)

cc_binary(
    name="play-greedy",
    srcs=["play-greedy.cc"],
    deps=[
        ":maze-state",
        ":greedy",
    ],
)
