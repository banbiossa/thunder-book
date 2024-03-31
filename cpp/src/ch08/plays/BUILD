load("@rules_cc//cc:defs.bzl", "cc_binary")

cc_binary(
    name="random",
    srcs=[
        "play_random.cc",
    ],
    deps=[
        "//src/ch08:maze_state",
    ],
)

cc_binary(
    name="mcts_vs_random",
    srcs=[
        "mcts_vs_random.cc",
    ],
    deps=[
        "//src/ch08:maze_state",
    ],
)


cc_binary(
    name="mcts_timebound_vs_random",
    srcs=[
        "mcts_timebound_vs_random.cc",
    ],
    deps=[
        "//src/ch08:maze_state",
    ],
)

cc_binary(
    name="compare",
    srcs=[
        "compare.cc",
    ],
    deps=[
        "//src/ch08:maze_state",
        "//src:util",
    ],
)
