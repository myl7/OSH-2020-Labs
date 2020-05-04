cmake_minimum_required(VERSION 3.15)
project(mchat CXX)

set(CMAKE_CXX_STANDARD 20)

add_executable(
    mchat_mt
    src/mt/main.cpp
    src/comm/accsoc.hpp
    src/mt/srvop.hpp
)
target_include_directories(mchat_mt PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/src>)
target_link_libraries(mchat_mt PRIVATE pthread)

add_executable(
    mchat_mp
    src/mp/main.cpp
    src/mp/srvsoc.hpp
)
target_include_directories(mchat_mp PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/src>)
