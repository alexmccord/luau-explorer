#include <iostream>
#include <cstring>

#include <Luau/Common.h>

#include "backend/vm.h"

static int assertionHandler(const char* expr, const char* file, int line, const char* function)
{
    std::cerr << file << "(" << line << "): ASSERTION FAILED: " << expr << std::endl;
    return 1;
}

static std::string read_stdin() {
    std::string length_s;
    length_s.reserve(sizeof(size_t));
    fgets(length_s.data(), sizeof(size_t) + 1, stdin);
    size_t length = std::stoull(length_s);

    std::string s;
    char buf[4096];
    while (fgets(buf, length + 1, stdin) && s.size() <= length)
        s.append(buf);
    return s;
}

int main() {
    Luau::assertHandler() = assertionHandler;

    for (Luau::FValue<bool>* flag = Luau::FValue<bool>::list; flag; flag = flag->next)
        if (strncmp(flag->name, "Luau", 4) == 0)
            flag->value = true;

    char opcode = fgetc(stdin);

    if (opcode == 1) {
        std::string code = read_stdin();

        if (auto error = backend::run(code)) {
            std::cerr << *error << std::endl;
            return 1;
        }
    } else {
        std::cerr << "Unknown opcode " << int(opcode) << std::endl;
        return 1;
    }
}
