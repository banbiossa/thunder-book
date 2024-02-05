load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")

cc_library(
    name="maze_state",
    srcs=[
        "maze_state.cc",
        "random_action.cc",
        "game.cc",
        "monte_carlo.cc",
        "mcts_alternate.cc",
        "alternate_maze_state.cc",
        "duct.cc",
        "mcts.cc",
    ],
    hdrs=[
        "maze_state.h",
        "random_action.h",
        "game.h",
        "monte_carlo.h",
        "mcts_alternate.h",
        "alternate_maze_state.h",
        "duct.h",
        "mcts.h",
    ],
    visibility=[
        "//src/ch06/plays:__pkg__"
    ],
)
