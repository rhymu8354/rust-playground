# Library is in Rust
add_library(fibonacci STATIC IMPORTED)
set(FIBONACCI_PATH ${CMAKE_CURRENT_SOURCE_DIR}/../fibonacci/target/debug)
set_target_properties(
    fibonacci
    PROPERTIES
        IMPORTED_LOCATION ${FIBONACCI_PATH}/fibonacci.lib
)
# set_target_properties(
#     fibonacci
#     PROPERTIES
#         IMPORTED_IMPLIB ${FIBONACCI_PATH}/fibonacci.dll.lib
# )

# Main program is in C
add_executable(cmain src/main.c)
target_link_libraries(cmain fibonacci ws2_32 Userenv)
