load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library")

cc_library(
    name="maze-state",
    srcs=[
        "maze-state.cc",
        "greedy.cc",
        "beam_search.cc",
        "time_keeper.cc",
        "chokudai_search.cc",
    ],
    hdrs=[
        "maze-state.h",
        "greedy.h",
        "beam_search.h",
        "time_keeper.h",
        "chokudai_search.h",
    ],
    visibility=[
        "//test/ch03:__pkg__",
    ],
)


cc_binary(
    name="score-average",
    srcs=[
        "average_score.cc",
    ],
    deps=[
        ":maze-state",
    ],
)
