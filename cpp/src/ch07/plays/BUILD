load("@rules_cc//cc:defs.bzl", "cc_binary")

cc_binary(
    name="random",
    srcs=[
        "play_random.cc",
    ],
    deps=[
        "//src/ch07:maze_state",
    ],
)

cc_binary(
    name="beam_search",
    srcs=[
        "play_beam_search.cc",
    ],
    deps=[
        "//src/ch07:maze_state",
    ],
)

cc_binary(
    name="time_beam_search",
    srcs=[
        "time_beam_search.cc",
    ],
    deps=[
        "//src/ch07:maze_state",
    ],
)

cc_binary(
    name="time_beam_search_bitset",
    srcs=[
        "time_beam_search_bitset.cc",
    ],
    deps=[
        "//src/ch07:maze_state",
        "//src:util",
    ],
)

cc_binary(
    name="bitset",
    srcs=[
        "play_bitset.cc",
    ],
)


cc_binary(
    name="poly",
    srcs=[
        "play_polymorphism.cc",
    ],
)

cc_binary(
    name="copy_count",
    srcs=[
        "copy_count.cc",
    ],
)
