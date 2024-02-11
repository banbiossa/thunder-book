load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")

cc_library(
    name="maze_state",
    srcs=[
        "maze_state.cc",
        "beam_search.cc",
        "random_action.cc",
        "game.cc",
        "bitset_matrix.cc",
        "bitset_single.cc",
        "globals.cc",
    ],
    hdrs=[
        "maze_state.h",
        "beam_search.h",
        "random_action.h",
        "game.h",
        "bitset_matrix.h",
        "bitset_single.h",
        "globals.h",
    ],
    visibility=["//src/ch07/plays:__pkg__"],
)
