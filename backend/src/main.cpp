#include <iostream>
#include <Luau/Common.h>

#include "backend/vm.h"

static int assertionHandler(const char* expr, const char* file, int line, const char* function)
{
    printf("%s(%d): ASSERTION FAILED: %s\n", file, line, expr);
    return 1;
}

int main() {
    Luau::assertHandler() = assertionHandler;

    for (Luau::FValue<bool>* flag = Luau::FValue<bool>::list; flag; flag = flag->next)
        if (strncmp(flag->name, "Luau", 4) == 0)
            flag->value = true;

    std::string s;
    std::cin >> s;
    std::cout << backend::run_luau_code(s) << std::endl;
}
