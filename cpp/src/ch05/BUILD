load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")


cc_library(
    name="maze_state",
    srcs=[
        "maze_state.cc",
        "random_action.cc",
        "mini_max.cc",
        "alpha_beta.cc",
        "win_rate.cc",
        "test_speed.cc",
        "iterative_deepening.cc",
        "time_keeper.cc",
        "monte_carlo.cc",
        "monte_carlo_tree_search.cc",
        "thunder_search.cc",
    ],
    hdrs=[
        "maze_state.h",
        "random_action.h",
        "mini_max.h",
        "alpha_beta.h",
        "win_rate.h",
        "test_speed.h",
        "iterative_deepening.h",
        "time_keeper.h",
        "monte_carlo.h",
        "monte_carlo_tree_search.h",
        "thunder_search.h",
    ],
    visibility=[
        "//test/ch05:__pkg__",
        "//src/ch05/plays:__pkg__",
    ],
)
