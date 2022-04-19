#include <lualib.h>
#include <memory>

#include "backend/vm.h"

namespace backend {

std::string run_luau_code(const std::string& code) {
    std::unique_ptr<lua_State, void(*)(lua_State*)> state{luaL_newstate(), lua_close};
    lua_State* L = state.get();

    luaL_openlibs(L);
    luaL_sandbox(L);
    luaL_sandboxthread(L);

    size_t bytecodeSize = 0;
    char* bytecode = luau_compile(code.data(), code.size(), nullptr, &bytecodeSize);
    int result = luau_load(L, "chunk", bytecode, bytecodeSize, 0);
    free(bytecode);

    if (result != 0) {
        size_t len;
        const char* msg = lua_tolstring(L, -1, &len);

        std::string error(msg, len);
        lua_pop(L, 1);

        return error;
    }

    result = lua_resume(L, nullptr, 0);

    if (result != 0) {
        std::string error;

        if (result == LUA_YIELD) {
            error = "thread yielded unexpectedly";
        } else if (const char* str = lua_tostring(L, -1)) {
            error = str;
        }

        error += "\nstacktrace:\n";
        error += lua_debugtrace(L);
        return error;
    }

    return "";
}

} // namespace backend
