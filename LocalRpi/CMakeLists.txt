cmake_minimum_required(VERSION 3.16) # Lower versions should also be supported
project(LocalRpi CXX)

# Program name and sources
set (TARGET ${PROJECT_NAME})
file (GLOB SOURCES "src/*.cc")
file (GLOB MODULES "src/*.cpp")

# Add target to build iostream module
add_custom_target(std_modules ALL
    COMMAND ${CMAKE_COMMAND} -E echo "Building standard library modules"
    COMMAND g++ -fmodules-ts -std=c++20 -c -x c++-system-header iostream
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
)

# Function to set up modules in GCC
function (prepare_for_module TGT)
    target_compile_options(${TGT} PUBLIC -fmodules-ts)
    set_property(TARGET ${TGT} PROPERTY CXX_STANDARD 20)
    set_property(TARGET ${TGT} PROPERTY CXX_EXTENSIONS OFF)
    add_dependencies(${TGT} std_modules)
endfunction()

# Setup program modules object library
set (MODULE_TARGET prog-modules)
add_library(${MODULE_TARGET} OBJECT ${MODULES})
prepare_for_module(${MODULE_TARGET})

# Setup executable
add_executable(${TARGET} ${SOURCES})
prepare_for_module(${TARGET})

# Add modules to application using object library
target_link_libraries(${TARGET} PRIVATE ${MODULE_TARGET})
