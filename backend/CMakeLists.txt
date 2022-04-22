cmake_minimum_required(VERSION 3.16)

add_subdirectory(luau)

project(backend LANGUAGES CXX)

add_executable(backend)
add_dependencies(backend Luau.Analysis Luau.Compiler Luau.VM)
target_compile_features(backend PRIVATE cxx_std_17)
target_include_directories(backend PUBLIC include luau/Analysis/include luau/Compiler/include luau/VM/include)
target_link_libraries(backend PUBLIC Luau.Ast Luau.Analysis Luau.Compiler Luau.VM)
target_sources(backend PRIVATE
    include/backend/vm.h

    src/vm.cpp
    src/main.cpp
)
