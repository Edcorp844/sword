if(NOT DEFINED stage_dir)
    message(FATAL_ERROR "merge_static_lib.cmake requires stage_dir")
endif()
if(NOT DEFINED archive)
    message(FATAL_ERROR "merge_static_lib.cmake requires archive")
endif()

file(REMOVE "${stage_dir}/__.SYMDEF")
file(GLOB rust_objects "${stage_dir}/*.o")
if(rust_objects STREQUAL "")
    message(FATAL_ERROR "No Rust utility object files were extracted into ${stage_dir}")
endif()

set(temp_archive "${archive}.rusttmp")
execute_process(
    COMMAND ${CMAKE_COMMAND} -E copy "${archive}" "${temp_archive}"
    RESULT_VARIABLE copy_result
    OUTPUT_VARIABLE copy_output
    ERROR_VARIABLE copy_error
)
if(NOT copy_result EQUAL 0)
    message(FATAL_ERROR "Failed to stage ${archive} for merge\n${copy_output}\n${copy_error}")
endif()

execute_process(
    COMMAND ${CMAKE_AR} rcs "${temp_archive}" ${rust_objects}
    RESULT_VARIABLE ar_result
    OUTPUT_VARIABLE ar_output
    ERROR_VARIABLE ar_error
)

if(NOT ar_result EQUAL 0)
    message(FATAL_ERROR "Failed to merge Rust utility objects into ${archive}\n${ar_output}\n${ar_error}")
endif()

execute_process(
    COMMAND ${CMAKE_COMMAND} -E copy "${temp_archive}" "${archive}"
    RESULT_VARIABLE move_result
    OUTPUT_VARIABLE move_output
    ERROR_VARIABLE move_error
)
if(NOT move_result EQUAL 0)
    message(FATAL_ERROR "Failed to install merged archive over ${archive}\n${move_output}\n${move_error}")
endif()

file(REMOVE "${temp_archive}")
