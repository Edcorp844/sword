# Locate Rust Cargo build tool
set(CARGO_HOME "$ENV{HOME}/.cargo/bin")

# First check well-known locations
if(EXISTS "${CARGO_HOME}/cargo")
    set(CARGO_EXECUTABLE "${CARGO_HOME}/cargo")
elseif(EXISTS "/opt/homebrew/opt/rust/bin/cargo")
    set(CARGO_EXECUTABLE "/opt/homebrew/opt/rust/bin/cargo")
elseif(EXISTS "/usr/local/cargo")
    set(CARGO_EXECUTABLE "/usr/local/cargo")
else()
    # Fall back to searching PATH if direct checks fail
    find_program(CARGO_EXECUTABLE cargo)
    if(NOT CARGO_EXECUTABLE)
        message(FATAL_ERROR "Cargo not found. Checked: ${CARGO_HOME}/cargo, /opt/homebrew/opt/rust/bin/cargo, /usr/local/cargo. Please install Rust from https://rustup.rs/")
    endif()
endif()

if(NOT EXISTS "${CARGO_EXECUTABLE}")
    message(FATAL_ERROR "Cargo executable not found at: ${CARGO_EXECUTABLE}")
endif()

message(STATUS "Using Cargo: ${CARGO_EXECUTABLE}")

set(RUST_UTILFUNS_TARGET_DIR "${CMAKE_CURRENT_SOURCE_DIR}/target")
set(RUST_UTILFUNS_LIB "${RUST_UTILFUNS_TARGET_DIR}/release/libsword_utilfuns.a")

add_custom_command(
    OUTPUT "${RUST_UTILFUNS_LIB}"
    COMMAND ${CARGO_EXECUTABLE} build --release
    WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
    DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
            ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs
    COMMENT "Building Rust utilfuns library"
)

add_custom_target(sword_rust_utilfuns ALL DEPENDS "${RUST_UTILFUNS_LIB}")

# Export path for use in parent CMakeLists
set(SWORD_RUST_UTILFUNS_LIB "${RUST_UTILFUNS_LIB}" PARENT_SCOPE)

