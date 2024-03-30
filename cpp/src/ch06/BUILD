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
        # "mcts_base.cc",
        # "mcts_node.cc",
    ],
    hdrs=[
        "maze_state.h",
        "random_action.h",
        "game.h",
        "monte_carlo.h",
        "mcts_alternate.h",
        "alternate_maze_state.h",
        "duct.h",
        # "mcts_base.h",
        # "mcts_node.h",
    ],
    visibility=[
        "//src/ch06/plays:__pkg__",
        "//test/ch06:__pkg__",
    ],
)

cc_library(
    name="try_template",
    hdrs=[
        "try_template.h",
    ],
    visibility=["//src/ch06/plays:__pkg__"],
)

cc_library(
    name="use_template",
    srcs=[
        "use_template.cc",
    ],
    hdrs=[
        "use_template.h",
    ],
    deps=[
        ":try_template",
    ],
    visibility=["//src/ch06/plays:__pkg__"],
)

cc_binary(
    name="template_play",
    srcs=[
        "template_play.cc",
    ],
    deps=[
        ":use_template",
    ],
)
