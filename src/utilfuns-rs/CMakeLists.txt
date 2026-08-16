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

###############################################################################
# Determine the Rust target triple matching the current CMake cross-compile
# configuration, so `cargo build --target <triple>` produces object code
# compatible with whatever platform/arch CMake itself is targeting.
###############################################################################

set(RUST_TARGET_TRIPLE "")

if(CMAKE_SYSTEM_NAME STREQUAL "iOS")
    # iphonesimulator vs iphoneos sysroot tells us device vs simulator
    if(CMAKE_OSX_SYSROOT MATCHES "iphonesimulator")
        if(CMAKE_OSX_ARCHITECTURES MATCHES "x86_64")
            set(RUST_TARGET_TRIPLE "x86_64-apple-ios")
        else()
            set(RUST_TARGET_TRIPLE "aarch64-apple-ios-sim")
        endif()
    else()
        set(RUST_TARGET_TRIPLE "aarch64-apple-ios")
    endif()

elseif(CMAKE_SYSTEM_NAME STREQUAL "Android")
    # CMAKE_ANDROID_ARCH_ABI is set by the NDK toolchain file
    if(CMAKE_ANDROID_ARCH_ABI STREQUAL "arm64-v8a")
        set(RUST_TARGET_TRIPLE "aarch64-linux-android")
    elseif(CMAKE_ANDROID_ARCH_ABI STREQUAL "armeabi-v7a")
        set(RUST_TARGET_TRIPLE "armv7-linux-androideabi")
    elseif(CMAKE_ANDROID_ARCH_ABI STREQUAL "x86_64")
        set(RUST_TARGET_TRIPLE "x86_64-linux-android")
    elseif(CMAKE_ANDROID_ARCH_ABI STREQUAL "x86")
        set(RUST_TARGET_TRIPLE "i686-linux-android")
    else()
        message(FATAL_ERROR "Unrecognized CMAKE_ANDROID_ARCH_ABI: ${CMAKE_ANDROID_ARCH_ABI}")
    endif()

elseif(APPLE)
    # Plain macOS build (host or explicit -DCMAKE_OSX_ARCHITECTURES=)
    if(CMAKE_OSX_ARCHITECTURES MATCHES "x86_64" AND NOT CMAKE_OSX_ARCHITECTURES MATCHES "arm64")
        set(RUST_TARGET_TRIPLE "x86_64-apple-darwin")
    else()
        set(RUST_TARGET_TRIPLE "aarch64-apple-darwin")
    endif()

elseif(MINGW)
    set(RUST_TARGET_TRIPLE "x86_64-pc-windows-gnu")

elseif(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    if(CMAKE_SYSTEM_PROCESSOR MATCHES "aarch64|arm64")
        set(RUST_TARGET_TRIPLE "aarch64-unknown-linux-gnu")
    else()
        set(RUST_TARGET_TRIPLE "x86_64-unknown-linux-gnu")
    endif()

else()
    message(WARNING "Could not determine Rust target triple for this platform — falling back to host default")
endif()

if(RUST_TARGET_TRIPLE)
    message(STATUS "Building sword_utilfuns for Rust target: ${RUST_TARGET_TRIPLE}")
    set(RUST_UTILFUNS_TARGET_ARG --target ${RUST_TARGET_TRIPLE})
    set(RUST_UTILFUNS_TARGET_SUBDIR "${RUST_TARGET_TRIPLE}/")
    
    # Verify the target is installed
    execute_process(
        COMMAND ${CARGO_EXECUTABLE} --version
        OUTPUT_QUIET
    )
    execute_process(
        COMMAND rustup target list --installed
        OUTPUT_VARIABLE RUSTUP_INSTALLED_TARGETS
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    if(NOT RUSTUP_INSTALLED_TARGETS MATCHES "${RUST_TARGET_TRIPLE}")
        message(FATAL_ERROR "Rust target ${RUST_TARGET_TRIPLE} not installed. Run: rustup target add ${RUST_TARGET_TRIPLE}")
    endif()
else()
    set(RUST_UTILFUNS_TARGET_ARG "")
    set(RUST_UTILFUNS_TARGET_SUBDIR "")
endif()

set(RUST_UTILFUNS_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}/target")
set(RUST_UTILFUNS_LIB "${RUST_UTILFUNS_TARGET_DIR}/${RUST_UTILFUNS_TARGET_SUBDIR}release/libsword_utilfuns.a")

add_custom_command(
    OUTPUT "${RUST_UTILFUNS_LIB}"
    COMMAND ${CARGO_EXECUTABLE} build --release
            ${RUST_UTILFUNS_TARGET_ARG}
            --target-dir "${RUST_UTILFUNS_TARGET_DIR}"
    WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
    DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
            ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs
    COMMENT "Building Rust utilfuns library for ${RUST_TARGET_TRIPLE}"
)

add_custom_target(sword_rust_utilfuns ALL DEPENDS "${RUST_UTILFUNS_LIB}")

# Export path for use in parent CMakeLists
set(SWORD_RUST_UTILFUNS_LIB "${RUST_UTILFUNS_LIB}" PARENT_SCOPE)

