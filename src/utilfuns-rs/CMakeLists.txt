find_program(CARGO_EXECUTABLE cargo HINTS "$ENV{HOME}/.cargo/bin" REQUIRED)

set(RUST_UTILFUNS_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}/target")
set(RUST_UTILFUNS_LIB "${RUST_UTILFUNS_TARGET_DIR}/release/libsword_utilfuns.a")

add_custom_command(
    OUTPUT "${RUST_UTILFUNS_LIB}"
    COMMAND ${CARGO_EXECUTABLE} build --release --target-dir "${RUST_UTILFUNS_TARGET_DIR}"
    WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
    DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
            ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs
    COMMENT "Building Rust utilfuns library"
)

add_custom_target(sword_rust_utilfuns ALL DEPENDS "${RUST_UTILFUNS_LIB}")

add_library(rust_utilfuns STATIC IMPORTED GLOBAL)
set_target_properties(rust_utilfuns PROPERTIES
    IMPORTED_LOCATION "${RUST_UTILFUNS_LIB}"
)

add_dependencies(sword sword_rust_utilfuns)
add_dependencies(sword_static sword_rust_utilfuns)

target_link_libraries(sword rust_utilfuns)
target_link_libraries(sword_static rust_utilfuns)
