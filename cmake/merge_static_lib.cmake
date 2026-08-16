# merge_static_lib.cmake
# Usage: cmake -DSTAGE_DIR=... -DTARGET_LIB=... -DAR=... -DRANLIB=... -P merge_static_lib.cmake
#
# Merges all .o object files from STAGE_DIR into TARGET_LIB using deterministic
# CMake file glob instead of shell expansion (avoids sh/Make/Ninja/Xcode differences)

if(NOT STAGE_DIR OR NOT TARGET_LIB OR NOT CMAKE_AR)
    message(FATAL_ERROR "Required variables missing: STAGE_DIR=${STAGE_DIR}, TARGET_LIB=${TARGET_LIB}, CMAKE_AR=${CMAKE_AR}")
endif()

# Use CMake to glob, not shell, for cross-platform consistency
file(GLOB STAGED_OBJECTS "${STAGE_DIR}/*.o")

list(LENGTH STAGED_OBJECTS OBJ_COUNT)
if(OBJ_COUNT EQUAL 0)
    message(FATAL_ERROR "No .o files found in ${STAGE_DIR} — Rust archive extraction produced nothing to merge.")
endif()

message(STATUS "Merging ${OBJ_COUNT} object file(s) from ${STAGE_DIR} into ${TARGET_LIB}")

# Merge objects into the target archive
execute_process(
    COMMAND ${CMAKE_AR} rcs "${TARGET_LIB}" ${STAGED_OBJECTS}
    RESULT_VARIABLE AR_RESULT
)
if(NOT AR_RESULT EQUAL 0)
    message(FATAL_ERROR "ar rcs failed with code ${AR_RESULT} while merging ${OBJ_COUNT} objects into ${TARGET_LIB}")
endif()

# Rebuild archive index (important on some systems)
if(CMAKE_RANLIB)
    execute_process(COMMAND ${CMAKE_RANLIB} "${TARGET_LIB}")
endif()

message(STATUS "Successfully merged ${OBJ_COUNT} objects into ${TARGET_LIB}")
