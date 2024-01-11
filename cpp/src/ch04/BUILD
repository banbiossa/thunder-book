load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")

cc_library(
    name="maze_state",
    srcs=[
        "auto_move_maze_state.cc",
        # "random_action.cc",
        # "hill_climb.cc",
    ],
    hdrs=[
        "auto_move_maze_state.h",
        # "random_action.h",
        # "hill_climb.h",
    ],
    visibility=[
        "//test/ch04:__pkg__",
    ],
)

cc_library(
    name="random_action",
    srcs=[
        # "auto_move_maze_state.cc",
        "random_action.cc",
        # "hill_climb.cc",
    ],
    hdrs=[
        # "auto_move_maze_state.h",
        "random_action.h",
        # "hill_climb.h",
    ],
    visibility=[
        "//test/ch04:__pkg__",
    ],
    deps=[
        ":maze_state",
    ],
)

cc_library(
    name="hill_climb",
    srcs=[
        # "auto_move_maze_state.cc",
        # "random_action.cc",
        "hill_climb.cc",
    ],
    hdrs=[
        # "auto_move_maze_state.h",
        # "random_action.h",
        "hill_climb.h",
    ],
    visibility=[
        "//test/ch04:__pkg__",
    ],
    deps=[
        ":maze_state",
    ],
)

# cc_library(
#     name="maze_state",
#     srcs=[
#         "auto_move_maze_state.cc",
#         "random_action.cc",
#         "hill_climb.cc",
#     ],
#     hdrs=[
#         "auto_move_maze_state.h",
#         "random_action.h",
#         "hill_climb.h",
#     ],
#     visibility=[
#         "//test/ch04:__pkg__",
#     ],
# )


cc_binary(
    name="play_random",
    srcs=[
        "play_random.cc",
    ],
    deps=[
        ":maze_state",
        ":random_action",
    ],
)

# cc_binary(
#     name="hill_climb",
#     srcs=[
#         "hill_climb.cc",
#     ],
#     deps=[
#         ":maze_state",
#     ],
# )
