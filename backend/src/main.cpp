#include <iostream>
#include <cstring>

#include <Luau/Common.h>

#include "backend/vm.h"

static int assertionHandler(const char* expr, const char* file, int line, const char* function)
{
    std::cerr << file << "(" << line << "): ASSERTION FAILED: " << expr << std::endl;
    return 1;
}

int main() {
    Luau::assertHandler() = assertionHandler;

    for (Luau::FValue<bool>* flag = Luau::FValue<bool>::list; flag; flag = flag->next)
        if (strncmp(flag->name, "Luau", 4) == 0)
            flag->value = true;

    std::string s;
    std::cin >> s;
    if (auto error = backend::run_luau_code(s)) {
        std::cerr << *error << std::endl;
    }
}
