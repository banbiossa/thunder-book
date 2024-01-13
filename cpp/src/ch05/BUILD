load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")


cc_library(
    name="maze_state",
    srcs=[
        "maze_state.cc",
        "random_action.cc",
    ],
    hdrs=[
        "maze_state.h",
        "random_action.h",
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
