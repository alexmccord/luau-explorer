#pragma once

#include <string>
#include <lua.h>
#include <luacode.h>

namespace backend {

std::string run_luau_code(const std::string& code);

} // namespace backend
