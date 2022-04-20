#pragma once

#include <optional>
#include <string>
#include <vector>

namespace backend {

std::optional<std::string> run_luau_code(const std::string& code);

} // namespace backend
