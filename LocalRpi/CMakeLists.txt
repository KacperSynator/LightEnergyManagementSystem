cmake_minimum_required(VERSION 3.16) # Lower versions should also be supported
project(LocalRpi CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library (libble++ SHARED IMPORTED)
set_property(TARGET libble++ PROPERTY IMPORTED_LOCATION "/usr/local/lib/libble++.so") 

# Program name and sources
file (GLOB SOURCES "src/*.cc")
file (GLOB MODULES "src/*.cxx")

function(add_module name)
    # using GCC
    add_custom_target(${name}.pcm
            COMMAND
                ${CMAKE_CXX_COMPILER}
                -std=c++20
                -fmodules-ts
                -c
                ${CMAKE_CURRENT_SOURCE_DIR}/${ARGN}
                -o ${name}.pcm
                )
endfunction()

# using GCC
add_compile_options(-fmodules-ts)
add_module(hello src/hello.cxx)
add_executable(${PROJECT_NAME} ${SOURCES})

target_link_options(${PROJECT_NAME} PUBLIC "LINKER:hello.pcm")
add_dependencies(${PROJECT_NAME} hello.pcm)
target_link_libraries(${PROJECT_NAME} PRIVATE bluetooth libble++ )

# Add modules to application using object library

