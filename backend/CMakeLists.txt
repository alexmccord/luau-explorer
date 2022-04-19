cmake_minimum_required(VERSION 3.16)

add_subdirectory(luau)

project(backend LANGUAGES CXX)

add_executable(backend)
add_dependencies(backend Luau.Analysis Luau.Compiler Luau.VM)
target_compile_features(backend PRIVATE cxx_std_17)
target_include_directories(backend PUBLIC include Luau/Analysis/include Luau/Compiler/include Luau/VM/include)
target_link_libraries(backend PUBLIC Luau.Analysis Luau.Compiler Luau.VM)
target_sources(backend PRIVATE
    src/main.cpp
)
