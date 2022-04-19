#include <iostream>

int main() {
    while (!std::cin.eof()) {
        std::string s;
        std::cin >> s;
        std::cout << s << std::endl;
    }
}
