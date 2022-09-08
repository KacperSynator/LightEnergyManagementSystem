cmake_minimum_required(VERSION 3.16) # Lower versions should also be supported
project(LocalRpi CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library (libble++ SHARED IMPORTED)
set_property(TARGET libble++ PROPERTY IMPORTED_LOCATION "/usr/local/lib/libble++.so") 

file (GLOB SOURCES "src/*.cc")

add_executable(${PROJECT_NAME} ${SOURCES})
target_include_directories(${PROJECT_NAME} PUBLIC "${PROJECT_SOURCE_DIR}/inc")
target_link_libraries(${PROJECT_NAME} PRIVATE bluetooth libble++)

