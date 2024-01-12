load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")

cc_library(
    name="maze_state",
    srcs=[
        "auto_move_maze_state.cc",
        "random_action.cc",
        "hill_climb.cc",
        "simulated_annealing.cc",
    ],
    hdrs=[
        "auto_move_maze_state.h",
        "random_action.h",
        "hill_climb.h",
        "simulated_annealing.h",
    ],
    visibility=[
        "//test/ch04:__pkg__",
    ],
)


cc_binary(
    name="random_action",
    srcs=[
        "random_action.cc",
    ],
    deps=[
        ":maze_state",
    ],
)

cc_binary(
    name="hill_climb",
    srcs=[
        "hill_climb.cc",
    ],
    deps=[
        ":maze_state",
    ],
)

cc_binary(
    name="score_average",
    srcs=[
        "average_score.cc",
    ],
    deps=[
        ":maze_state",
    ],
)
