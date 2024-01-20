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
        "play_mini_max_vs_random.cc",
    ],
    deps=[
        ":maze_state",
    ],
)

cc_binary(
    name="alpha_beta_vs_mini_max",
    srcs=[
        "play_alpha_beta_vs_mini_max.cc",
    ],
    deps=[
        ":maze_state",
    ],
)


cc_binary(
    name="compare_speed_alpha_beta_vs_mini_max",
    srcs=[
        "compare_speed_alpha_beta_vs_mini_max.cc",
    ],
    deps=[
        ":maze_state",
    ],
)

cc_binary(
    name="play_compare_iterative_deepening",
    srcs=[
        "play_compare_iterative_deepening.cc",
    ],
    deps = [
        ":maze_state",
    ]
)


cc_binary(
    name="play_monte_carlo_vs_random",
    srcs=[
        "play_monte_carlo_vs_random.cc",
    ],
    deps=[
        ":maze_state",
    ],
)

cc_binary(
    name="play_compare_primitive_monte_carlo",
    srcs=[
        "play_compare_primitive_monte_carlo.cc",
    ],
    deps=[
        ":maze_state",
    ],
)
