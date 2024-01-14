load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")


cc_library(
    name="maze_state",
    srcs=[
        "maze_state.cc",
        "random_action.cc",
        "mini_max.cc",
        "alpha_beta.cc",
    ],
    hdrs=[
        "maze_state.h",
        "random_action.h",
        "mini_max.h",
        "alpha_beta.h",
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
    name="mini_max_vs_random",
    srcs=[
        "play_mini_max.cc",
    ],
    deps=[
        ":maze_state",
    ],
)

cc_binary(
    name="mini_max_win_rate",
    srcs=[
        "mini_max_win_rate.cc",
    ],
    deps=[
        ":maze_state",
    ],
)
