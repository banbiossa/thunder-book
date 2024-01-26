load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")

cc_library(
    name="maze_state",
    srcs=[
        "maze_state.cc",
        "random_action.cc",
        "game.cc",
        "monte_carlo.cc",
    ],
    hdrs=[
        "maze_state.h",
        "random_action.h",
        "game.h",
        "monte_carlo.h",
    ],
    visibility=[
        "//src/ch06/plays:__pkg__"
    ],
)
