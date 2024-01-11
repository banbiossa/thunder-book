
load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")

cc_library(
    name="maze_state",
    srcs=[
        "auto_move_maze_state.cc",
        "random_action.cc",
    ],
    hdrs=[
        "auto_move_maze_state.h",
        "random_action.h",
    ],
    visibility=[
        "//test/ch04:__pkg__",
    ],
)

cc_binary(
    name = "play_random",
    srcs = [
        "random_action.cc",
    ],
    deps = [
        ":maze_state",
    ],
)
