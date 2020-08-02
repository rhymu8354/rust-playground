add_subdirectory("accidental_move")
add_subdirectory("cfib")

# `cmain` is commented out because it depeonds on `fibonacci` which is
# a Rust library, and this workspace isn't smart enough yet to build that
# library and place it where `cmain` can consume it.
#
# add_subdirectory("cmain")
