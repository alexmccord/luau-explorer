cmake_minimum_required(VERSION 3.16)

include(FetchContent)

FetchContent_Declare(luau
    GIT_REPOSITORY https://github.com/Roblox/luau.git
    GIT_TAG        0.523
)
FetchContent_MakeAvailable(luau)

# if(NOT MSVC)
#     # Works around gcc 9.x bug where locals are sometimes unused in constexpr context.
#     add_compile_options(-Wno-unused-variable -Wno-unused-but-set-variable)
# endif()

project(backend LANGUAGES CXX)

add_executable(backend)
add_dependencies(backend Luau.Analysis Luau.Compiler)
target_compile_features(backend PRIVATE cxx_std_17)
target_link_libraries(backend Luau.Analysis Luau.Compiler)
target_sources(backend PRIVATE
    src/main.cpp
)
