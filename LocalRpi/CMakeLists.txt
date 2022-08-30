cmake_minimum_required(VERSION 3.2)

project(LocalRpi)

set(CMAKE_CXX_STANDARD 20)

file(GLOB SRC "src/*")

add_executable(${CMAKE_PROJECT_NAME} ${SRC})
