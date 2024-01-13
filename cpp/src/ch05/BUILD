load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")


cc_library(
    name="maze_state",
    srcs=[
        "maze_state.cc",
        "random_action.cc",
        "mini_max.cc",
    ],
    hdrs=[
        "maze_state.h",
        "random_action.h",
        "mini_max.h",
    ],
    visibility=[
        "//test/ch05:__pkg__",
    ],
)

cc_binary(
    name="random",
    srcs=[
        "play_random.cc",
    ],
    deps=[
        ":maze_state",
    ],
)

cc_binary(
    name="mini_max",
    srcs=[
        "play_mini_max.cc",
    ],
    deps=[
        ":maze_state",
    ],
)
