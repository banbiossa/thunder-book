load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")

cc_library(
    name="maze_state",
    srcs=[
        "maze_state.cc",
        "game.cc",
        "random_action.cc",
        "mcts.cc",
        "time_keeper.cc",
    ],
    hdrs=[
        "maze_state.h",
        "game.h",
        "random_action.h",
        "mcts.h",
        "time_keeper.h",
    ],
    visibility=["//src/ch08/plays:__pkg__"],
)
